use crate::model::Operation;
use proc_macro2::LineColumn;
use quote::ToTokens;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};
use syn::{
    spanned::Spanned,
    visit::{self, Visit},
    Attribute, File, Item, Meta, UseTree,
};
use thiserror::Error;
use toml_edit::{Array, DocumentMut, Item as TomlItem, Table, Value};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Coverage {
    pub total: usize,
    pub transformed: usize,
    pub missing: usize,
    pub ambiguous: usize,
}

impl Coverage {
    pub fn is_complete(self) -> bool {
        self.total == self.transformed + self.missing + self.ambiguous && self.missing == 0 && self.ambiguous == 0
    }
}

#[derive(Debug, Clone)]
pub struct TransformOutput {
    pub coverage: Coverage,
    pub changed_files: Vec<String>,
}

#[derive(Debug, Error)]
pub enum TransformError {
    #[error("{path}: {source}")]
    Io { path: PathBuf, source: std::io::Error },
    #[error("{path}: invalid Rust: {message}")]
    Rust { path: PathBuf, message: String },
    #[error("{path}: invalid Cargo.toml: {message}")]
    Cargo { path: PathBuf, message: String },
    #[error("{path}: {message}")]
    Invalid { path: PathBuf, message: String },
    #[error("operation mapping is incomplete: {coverage:?}")]
    IncompleteMapping { coverage: Coverage },
}

#[derive(Clone)]
struct SourceFile {
    relative: String,
    source: String,
    syntax: File,
}

#[derive(Debug)]
struct OperationPathVisitor<'a> {
    operations: &'a [Operation],
    waiter_owners: &'a BTreeMap<String, BTreeSet<String>>,
    symbol_owners: &'a BTreeMap<String, BTreeSet<String>>,
    owners: BTreeSet<String>,
    method_calls: BTreeSet<String>,
    references: BTreeSet<String>,
}

#[derive(Default)]
struct SuperModuleReferenceVisitor {
    names: BTreeSet<String>,
}

#[derive(Default)]
struct TypeReferenceVisitor {
    references: BTreeSet<String>,
}

#[derive(Default)]
struct OperationModuleReferenceVisitor {
    modules: BTreeSet<String>,
}

struct DocumentationVisitor<'a> {
    source: &'a str,
    old_library_name: &'a str,
    new_library_name: &'a str,
    edits: Vec<(usize, usize, String)>,
}

impl<'ast> Visit<'ast> for SuperModuleReferenceVisitor {
    fn visit_path(&mut self, path: &'ast syn::Path) {
        let mut segments = path.segments.iter();
        if segments.next().is_some_and(|segment| segment.ident == "super") {
            if let Some(segment) = segments.next() {
                self.names.insert(segment.ident.to_string());
            }
        }
        visit::visit_path(self, path);
    }
}

impl<'ast> Visit<'ast> for TypeReferenceVisitor {
    fn visit_path(&mut self, path: &'ast syn::Path) {
        let segments = path
            .segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect::<Vec<_>>();
        for index in 0..segments.len() {
            if segments.get(index).map(String::as_str) != Some("types") {
                continue;
            }
            for end in (index + 1)..=segments.len() {
                self.references.insert(segments[index..end].join("::"));
            }
            if let Some(key) = normalized_type_key(&segments, index) {
                self.references.insert(key);
            }
        }
        visit::visit_path(self, path);
    }
}

fn normalized_type_key(segments: &[String], types_index: usize) -> Option<String> {
    let suffix = segments.get(types_index + 1..)?;
    let (namespace, name) = match suffix {
        [module, name] if module.starts_with('_') => ("types", name.as_str()),
        [module, name] if module == "builders" => ("types", name.as_str()),
        [error, module, name] if error == "error" && module.starts_with('_') => ("types::error", name.as_str()),
        [error, module, name] if error == "error" && module == "builders" => ("types::error", name.as_str()),
        _ => return None,
    };
    let name = name.strip_suffix("Builder").unwrap_or(name);
    (!name.is_empty()).then(|| format!("{namespace}::{name}"))
}

impl<'ast> Visit<'ast> for OperationModuleReferenceVisitor {
    fn visit_path(&mut self, path: &'ast syn::Path) {
        let mut segments = path.segments.iter();
        while let Some(segment) = segments.next() {
            if segment.ident == "operation" {
                if let Some(module) = segments.next() {
                    self.modules.insert(module.ident.to_string());
                }
            }
        }
        visit::visit_path(self, path);
    }
}

impl<'ast> Visit<'ast> for DocumentationVisitor<'_> {
    fn visit_attribute(&mut self, attribute: &'ast Attribute) {
        if attribute.path().is_ident("doc") {
            if let (Ok(start), Ok(end)) = (
                source_offset(self.source, attribute.span().start()),
                source_offset(self.source, attribute.span().end()),
            ) {
                let source_attribute = &self.source[start..end];
                let replaced = replace_identifier(source_attribute, self.old_library_name, self.new_library_name);
                if replaced != source_attribute {
                    self.edits.push((start, end, replaced));
                }
            }
        }
        visit::visit_attribute(self, attribute);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct MethodKey {
    self_type: String,
    trait_type: Option<String>,
    name: String,
}

type MethodOwnership = BTreeMap<MethodKey, BTreeSet<String>>;
type StatementEdits = BTreeMap<String, Vec<(usize, usize, BTreeSet<String>)>>;

fn impl_context(item_impl: &syn::ItemImpl) -> (String, Option<String>) {
    (
        item_impl.self_ty.to_token_stream().to_string(),
        item_impl
            .trait_
            .as_ref()
            .map(|(path, _)| path.to_token_stream().to_string()),
    )
}

fn method_key(context: &(String, Option<String>), name: &str) -> MethodKey {
    MethodKey {
        self_type: context.0.clone(),
        trait_type: context.1.clone(),
        name: name.to_owned(),
    }
}

impl<'ast> Visit<'ast> for OperationPathVisitor<'_> {
    fn visit_item_mod(&mut self, _item: &'ast syn::ItemMod) {
        // A module declaration owns its child file, not every operation-owned
        // item nested inside that file. Child ownership is resolved from the
        // module graph separately.
    }

    fn visit_path(&mut self, path: &'ast syn::Path) {
        let segments = path
            .segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect::<Vec<_>>();
        for index in 0..segments.len() {
            if segments.get(index).map(String::as_str) == Some("operation") {
                if let Some(module) = segments.get(index + 1) {
                    if let Some(operation) = self.operations.iter().find(|operation| &operation.module == module) {
                        self.owners.insert(operation.feature.clone());
                    }
                }
            }
            if segments.get(index).map(String::as_str) == Some("waiters") {
                if let Some(waiter) = segments.get(index + 1) {
                    if let Some(owners) = self.waiter_owners.get(waiter) {
                        self.owners.extend(owners.iter().cloned());
                    }
                }
            }
            for end in (index + 1)..=segments.len() {
                let symbol = segments[index..end].join("::");
                if self.symbol_owners.contains_key(&symbol) {
                    self.references.insert(symbol);
                }
            }
        }
        visit::visit_path(self, path);
    }

    fn visit_expr_method_call(&mut self, call: &'ast syn::ExprMethodCall) {
        let method = call.method.to_string();
        self.method_calls.insert(method.clone());
        if is_client_receiver(&call.receiver) {
            if let Some(operation) = self.operations.iter().find(|operation| operation.module == method) {
                self.owners.insert(operation.feature.clone());
            }
        }
        visit::visit_expr_method_call(self, call);
    }

    fn visit_expr_call(&mut self, call: &'ast syn::ExprCall) {
        if let syn::Expr::Path(path) = call.func.as_ref() {
            if let Some(segment) = path.path.segments.last() {
                self.method_calls.insert(segment.ident.to_string());
            }
        }
        visit::visit_expr_call(self, call);
    }
}

fn is_client_receiver(expression: &syn::Expr) -> bool {
    match expression {
        syn::Expr::Path(path) => path
            .path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == "client" || segment.ident == "self"),
        syn::Expr::Field(field) => is_client_receiver(&field.base),
        syn::Expr::Paren(paren) => is_client_receiver(&paren.expr),
        _ => false,
    }
}

/// Rewrites a copied service crate in place.
pub fn transform_tree(
    crate_root: &Path,
    package_name: &str,
    library_name: &str,
    operations: &[Operation],
) -> Result<TransformOutput, TransformError> {
    let source_root = crate_root.join("src");
    if !source_root.is_dir() {
        return Err(TransformError::Invalid {
            path: crate_root.to_owned(),
            message: "service crate has no src directory".to_owned(),
        });
    }
    let mut source_files = collect_source_files(&source_root, crate_root)?;
    let mut coverage = mapping_coverage(&source_files, operations, crate_root)?;
    if !coverage.is_complete() {
        return Err(TransformError::IncompleteMapping { coverage });
    }
    let mut changed_files = prune_unselected_operation_surface(crate_root, &source_files, operations)?;
    if !changed_files.is_empty() {
        source_files = collect_source_files(&source_root, crate_root)?;
    }
    let redundant_cfg_files = remove_redundant_operation_child_cfgs(crate_root, &source_files, operations)?;
    changed_files.extend(redundant_cfg_files);
    if !changed_files.is_empty() {
        source_files = collect_source_files(&source_root, crate_root)?;
    }

    let old_library_name = cargo_library_name(&crate_root.join("Cargo.toml"))?;
    let documentation_files = rewrite_documentation(crate_root, &source_files, &old_library_name, library_name)?;
    changed_files.extend(documentation_files);
    if !changed_files.is_empty() {
        source_files = collect_source_files(&source_root, crate_root)?;
    }

    let waiter_owners = waiter_owners(&source_files, operations);
    let type_owners = build_type_owners(&source_files, operations);
    let operation_error_names = operation_error_names(&source_files);
    let error_variant_owners = error_variant_owners(&source_files, &type_owners);
    let symbol_owners = operation_symbols(&source_files, operations, &waiter_owners, &type_owners);
    let local_module_roots = local_module_roots(&source_files);
    let mut edits = BTreeMap::<String, BTreeMap<usize, BTreeSet<String>>>::new();
    let mut statement_edits = StatementEdits::new();
    for source_file in &source_files {
        let child_of_parent_gated_module = is_parent_gated_child_path(&source_file.relative, operations)
            || is_type_definition_child_path(&source_file.relative);
        let mut direct_owners = path_owners(&source_file.relative, operations);
        if is_protocol_serde_path(&source_file.relative) {
            if let Some(module_owners) = symbol_owners.get(&source_module_path(&source_file.relative)) {
                direct_owners.extend(module_owners.iter().cloned());
            }
        }
        let method_owners =
            collect_method_owners(&source_file.syntax.items, operations, &waiter_owners, &symbol_owners);
        for item in &source_file.syntax.items {
            let mut owners = direct_owners.clone();
            if is_type_root_path(&source_file.relative) {
                owners.extend(type_root_item_owners(
                    &source_file.relative,
                    item,
                    &source_files,
                    &type_owners,
                ));
            }
            let mut item_visitor = OperationPathVisitor {
                operations,
                waiter_owners: &waiter_owners,
                symbol_owners: &symbol_owners,
                owners: BTreeSet::new(),
                method_calls: BTreeSet::new(),
                references: BTreeSet::new(),
            };
            item_visitor.visit_item(item);
            if direct_owners.is_empty() {
                owners.extend(item_visitor.owners);
                if owners.is_empty() {
                    owners.extend(operation_error_owners(&source_file.relative, item, &type_owners));
                }
                if owners.is_empty() && is_type_ownership_path(&source_file.relative) {
                    owners.extend(type_reference_owners(item, &type_owners));
                }
                if owners.is_empty() {
                    owners.extend(operation_error_reference_owners(
                        item,
                        &type_owners,
                        &operation_error_names,
                    ));
                }
                if owners.is_empty() && source_file.relative == "src/protocol_serde.rs" {
                    if let Item::Fn(function) = item {
                        if let Some(helper_owners) =
                            symbol_owners.get(&format!("protocol_serde::{}", function.sig.ident))
                        {
                            owners.extend(helper_owners.iter().cloned());
                        }
                    }
                }
                if owners.is_empty() {
                    owners.extend(protocol_reference_owners(&item_visitor.references, &symbol_owners));
                }
            }
            if source_file.relative == "src/error_meta.rs" {
                collect_error_variant_edits(
                    &source_file.relative,
                    &source_file.source,
                    item,
                    &error_variant_owners,
                    &mut edits,
                )?;
                if let Item::Impl(item_impl) = item {
                    collect_error_match_arm_edits(
                        &source_file.relative,
                        &source_file.source,
                        item_impl,
                        &error_variant_owners,
                        &mut edits,
                    )?;
                }
            }
            if !child_of_parent_gated_module {
                if let Item::Impl(item_impl) = item {
                    let trait_method_owners = impl_method_owners(item_impl, &method_owners);
                    let header_owners = impl_header_owners(item_impl, operations, &waiter_owners, &symbol_owners);
                    if direct_owners.is_empty() && !header_owners.is_empty() {
                        if !has_matching_cfg(item_attrs(item), &header_owners) {
                            add_edit(
                                &mut edits,
                                &source_file.relative,
                                item_start(item, &source_file.source)?,
                                header_owners,
                            );
                        }
                    } else if direct_owners.is_empty()
                        && item_impl.trait_.is_some()
                        && !trait_method_owners.is_empty()
                        && item_impl.items.iter().all(|item| matches!(item, syn::ImplItem::Fn(_)))
                    {
                        collect_impl_method_edits(
                            &source_file.relative,
                            &source_file.source,
                            item_impl,
                            &method_owners,
                            &mut edits,
                        )?;
                    } else if direct_owners.is_empty() && !owners.is_empty() {
                        if !has_matching_cfg(item_attrs(item), &owners) {
                            add_edit(
                                &mut edits,
                                &source_file.relative,
                                item_start(item, &source_file.source)?,
                                owners,
                            );
                        }
                    } else if direct_owners.is_empty() {
                        collect_impl_method_edits(
                            &source_file.relative,
                            &source_file.source,
                            item_impl,
                            &method_owners,
                            &mut edits,
                        )?;
                    } else if !owners.is_empty() && !has_matching_cfg(item_attrs(item), &owners) {
                        add_edit(
                            &mut edits,
                            &source_file.relative,
                            item_start(item, &source_file.source)?,
                            owners,
                        );
                    }
                } else if !owners.is_empty()
                    && !matches!(item, Item::Trait(_))
                    && !has_matching_cfg(item_attrs(item), &owners)
                {
                    add_edit(
                        &mut edits,
                        &source_file.relative,
                        item_start(item, &source_file.source)?,
                        owners,
                    );
                }
            }
            if !child_of_parent_gated_module {
                if let Item::Trait(item_trait) = item {
                    let trait_method_owners =
                        trait_method_owners(item_trait, operations, &waiter_owners, &symbol_owners);
                    if !trait_method_owners.is_empty() {
                        collect_trait_method_edits(
                            &source_file.relative,
                            &source_file.source,
                            item_trait,
                            operations,
                            &waiter_owners,
                            &symbol_owners,
                            &mut edits,
                        )?;
                    }
                }
            }
            if !child_of_parent_gated_module {
                collect_module_edits(
                    &source_file.relative,
                    &source_file.source,
                    item,
                    &source_files,
                    operations,
                    &waiter_owners,
                    &symbol_owners,
                    &method_owners,
                    &mut edits,
                )?;
                if matches!(item, Item::Fn(_)) {
                    collect_statement_edits(
                        &source_file.relative,
                        &source_file.source,
                        item,
                        operations,
                        &waiter_owners,
                        &symbol_owners,
                        &local_module_roots,
                        &mut statement_edits,
                    )?;
                }
            }
        }
    }

    for source_file in &source_files {
        let file_edits = edits.get(&source_file.relative);
        let file_statement_edits = statement_edits.get(&source_file.relative);
        if file_edits.is_none() && file_statement_edits.is_none() {
            continue;
        }
        let updated = apply_edits(
            &source_file.source,
            file_edits,
            file_statement_edits,
            &source_file.relative,
        )?;
        syn::parse_file(&updated).map_err(|error| TransformError::Rust {
            path: crate_root.join(&source_file.relative),
            message: error.to_string(),
        })?;
        fs::write(crate_root.join(&source_file.relative), updated).map_err(|source| TransformError::Io {
            path: crate_root.join(&source_file.relative),
            source,
        })?;
        changed_files.push(source_file.relative.clone());
    }
    rewrite_cargo_named(&crate_root.join("Cargo.toml"), package_name, library_name, operations)?;
    changed_files.push("Cargo.toml".to_owned());
    source_files = collect_source_files(&source_root, crate_root)?;
    for source_file in &source_files {
        syn::parse_file(&source_file.source).map_err(|error| TransformError::Rust {
            path: crate_root.join(&source_file.relative),
            message: error.to_string(),
        })?;
    }
    coverage.transformed = operations.len();
    Ok(TransformOutput {
        coverage,
        changed_files,
    })
}

fn mapping_coverage(
    files: &[SourceFile],
    operations: &[Operation],
    crate_root: &Path,
) -> Result<Coverage, TransformError> {
    let declarations = module_declarations(files, "src/operation.rs", crate_root)?;
    let client_declarations = module_declarations(files, "src/client.rs", crate_root)?;
    let mut coverage = Coverage {
        total: operations.len(),
        ..Coverage::default()
    };
    for operation in operations {
        let operation_count = declarations.iter().filter(|name| *name == &operation.module).count();
        let client_count = client_declarations
            .iter()
            .filter(|name| *name == &operation.module)
            .count();
        match (operation_count, client_count) {
            (1, 1) => coverage.transformed += 1,
            (0, 0) => coverage.missing += 1,
            _ => coverage.ambiguous += 1,
        }
    }
    Ok(coverage)
}

fn collect_source_files(source_root: &Path, crate_root: &Path) -> Result<Vec<SourceFile>, TransformError> {
    let mut paths = Vec::new();
    collect_paths(source_root, &mut paths).map_err(|source| TransformError::Io {
        path: source_root.to_owned(),
        source,
    })?;
    paths.sort();
    let mut files = Vec::new();
    for path in paths {
        let relative = path
            .strip_prefix(crate_root)
            .expect("source is below crate root")
            .to_string_lossy()
            .replace('\\', "/");
        let source = fs::read_to_string(&path).map_err(|source| TransformError::Io {
            path: path.clone(),
            source,
        })?;
        let syntax = syn::parse_file(&source).map_err(|error| TransformError::Rust {
            path,
            message: error.to_string(),
        })?;
        files.push(SourceFile {
            relative,
            source,
            syntax,
        });
    }
    Ok(files)
}

fn collect_paths(root: &Path, paths: &mut Vec<PathBuf>) -> Result<(), std::io::Error> {
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_paths(&path, paths)?;
        } else if path.extension().and_then(|extension| extension.to_str()) == Some("rs") {
            paths.push(path);
        }
    }
    Ok(())
}

fn prune_unselected_operation_surface(
    crate_root: &Path,
    files: &[SourceFile],
    operations: &[Operation],
) -> Result<Vec<String>, TransformError> {
    let selected = operations
        .iter()
        .map(|operation| operation.module.clone())
        .collect::<BTreeSet<_>>();
    let operation_modules = module_names(files, "src/operation.rs");
    let unselected = operation_modules
        .difference(&selected)
        .map(|module| (*module).to_owned())
        .collect::<BTreeSet<_>>();
    if unselected.is_empty() {
        return Ok(Vec::new());
    }

    let mut removals = BTreeMap::<String, Vec<(usize, usize)>>::new();
    let mut removed_files = BTreeSet::new();
    for file in files {
        if operation_or_client_module(file.relative.as_str()).is_some_and(|module| unselected.contains(module)) {
            removed_files.insert(file.relative.clone());
            continue;
        }
        let operation_references = operation_module_references(&file.syntax);
        let unselected_references = operation_references
            .intersection(&unselected)
            .cloned()
            .collect::<BTreeSet<_>>();
        if unselected_references.is_empty() && file.relative != "src/protocol_serde.rs" {
            continue;
        }

        match file.relative.as_str() {
            "src/client.rs" => {
                collect_client_method_removals(&file.source, &file.syntax.items, &unselected, &mut removals)?;
            }
            "src/protocol_serde.rs" | "src/waiters.rs" => {
                for item in &file.syntax.items {
                    let Item::Mod(module) = item else {
                        continue;
                    };
                    let Some(child_relative) =
                        resolve_external_module(&file.relative, &module.ident.to_string(), files)
                    else {
                        continue;
                    };
                    let Some(child) = files.iter().find(|child| {
                        child.relative == child_relative
                            || child.relative == format!("{child_relative}.rs")
                            || child.relative == format!("{child_relative}/mod.rs")
                    }) else {
                        continue;
                    };
                    let child_references = operation_module_references(&child.syntax);
                    let stale_operation_shape = is_unselected_protocol_shape(&module.ident.to_string(), &unselected);
                    if child_references.is_disjoint(&selected) && stale_operation_shape {
                        removed_files.insert(child.relative.clone());
                        add_removal(&mut removals, &file.relative, item_range(item, &file.source)?);
                    }
                }
            }
            _ => {
                for item in &file.syntax.items {
                    if !operation_module_references(item).is_disjoint(&unselected) {
                        add_removal(&mut removals, &file.relative, item_range(item, &file.source)?);
                    }
                }
            }
        }
    }

    for relative in ["src/operation.rs", "src/client.rs"] {
        let Some(file) = files.iter().find(|file| file.relative == relative) else {
            continue;
        };
        for item in &file.syntax.items {
            let Item::Mod(module) = item else {
                continue;
            };
            if unselected.contains(module.ident.to_string().as_str()) {
                add_removal(&mut removals, relative, item_range(item, &file.source)?);
            }
        }
    }

    let mut changed = removed_files.clone();
    for file in files {
        let Some(ranges) = removals.get(&file.relative) else {
            continue;
        };
        if removed_files.contains(&file.relative) {
            continue;
        }
        let updated = apply_removals(&file.source, ranges, &file.relative)?;
        if updated != file.source {
            fs::write(crate_root.join(&file.relative), updated).map_err(|source| TransformError::Io {
                path: crate_root.join(&file.relative),
                source,
            })?;
            changed.insert(file.relative.clone());
        }
    }
    for relative in &removed_files {
        fs::remove_file(crate_root.join(relative)).map_err(|source| TransformError::Io {
            path: crate_root.join(relative),
            source,
        })?;
    }
    Ok(changed.into_iter().collect())
}

fn is_unselected_protocol_shape(module: &str, unselected: &BTreeSet<String>) -> bool {
    let Some(shape) = module.strip_prefix("shape_") else {
        return false;
    };
    unselected.iter().any(|operation| {
        shape == operation || shape == format!("{operation}_input") || shape == format!("{operation}_output")
    })
}

fn module_names(files: &[SourceFile], relative: &str) -> BTreeSet<String> {
    files
        .iter()
        .find(|file| file.relative == relative)
        .into_iter()
        .flat_map(|file| file.syntax.items.iter())
        .filter_map(|item| match item {
            Item::Mod(item) => Some(item.ident.to_string()),
            _ => None,
        })
        .collect()
}

fn operation_or_client_module(relative: &str) -> Option<&str> {
    let path = relative.strip_prefix("src/")?;
    let mut components = path.split('/');
    if !matches!(components.next(), Some("operation" | "client")) {
        return None;
    }
    let module = components.next()?;
    Some(module.strip_suffix(".rs").unwrap_or(module))
}

fn operation_module_references<T: VisitOperationModules>(node: &T) -> BTreeSet<String> {
    let mut visitor = OperationModuleReferenceVisitor::default();
    node.visit_operation_modules(&mut visitor);
    visitor.modules
}

trait VisitOperationModules {
    fn visit_operation_modules(&self, visitor: &mut OperationModuleReferenceVisitor);
}

impl VisitOperationModules for File {
    fn visit_operation_modules(&self, visitor: &mut OperationModuleReferenceVisitor) {
        visitor.visit_file(self);
    }
}

impl VisitOperationModules for Item {
    fn visit_operation_modules(&self, visitor: &mut OperationModuleReferenceVisitor) {
        visitor.visit_item(self);
    }
}

impl VisitOperationModules for syn::TraitItemFn {
    fn visit_operation_modules(&self, visitor: &mut OperationModuleReferenceVisitor) {
        visitor.visit_trait_item_fn(self);
    }
}

impl VisitOperationModules for syn::ImplItemFn {
    fn visit_operation_modules(&self, visitor: &mut OperationModuleReferenceVisitor) {
        visitor.visit_impl_item_fn(self);
    }
}

fn collect_client_method_removals(
    source: &str,
    items: &[Item],
    unselected: &BTreeSet<String>,
    removals: &mut BTreeMap<String, Vec<(usize, usize)>>,
) -> Result<(), TransformError> {
    for item in items {
        match item {
            Item::Trait(item_trait) => {
                for item in &item_trait.items {
                    let syn::TraitItem::Fn(method) = item else {
                        continue;
                    };
                    if operation_module_references(method).is_disjoint(unselected) {
                        continue;
                    }
                    add_removal(removals, "src/client.rs", trait_item_range(method, source)?);
                }
            }
            Item::Impl(item_impl) if item_impl.trait_.is_some() => {
                for item in &item_impl.items {
                    let syn::ImplItem::Fn(method) = item else {
                        continue;
                    };
                    if operation_module_references(method).is_disjoint(unselected) {
                        continue;
                    }
                    add_removal(removals, "src/client.rs", impl_item_range(method, source)?);
                }
            }
            _ => {}
        }
    }
    Ok(())
}

fn module_declarations(files: &[SourceFile], relative: &str, crate_root: &Path) -> Result<Vec<String>, TransformError> {
    let Some(file) = files.iter().find(|file| file.relative == relative) else {
        return Err(TransformError::Invalid {
            path: crate_root.join(relative),
            message: "service source is missing the module declaration file".to_owned(),
        });
    };
    Ok(file
        .syntax
        .items
        .iter()
        .filter_map(|item| match item {
            Item::Mod(item) => Some(item.ident.to_string()),
            _ => None,
        })
        .collect())
}

fn operation_symbols(
    files: &[SourceFile],
    operations: &[Operation],
    waiter_owners: &BTreeMap<String, BTreeSet<String>>,
    type_owners: &BTreeMap<String, BTreeSet<String>>,
) -> BTreeMap<String, BTreeSet<String>> {
    let mut symbols = BTreeMap::<String, BTreeSet<String>>::new();
    for file in files {
        let module_path = source_module_path(&file.relative);
        if module_path.contains("::") {
            symbols.entry(module_path.clone()).or_default();
        }
        if file.relative == "src/protocol_serde.rs" {
            for item in &file.syntax.items {
                if let Item::Fn(function) = item {
                    symbols
                        .entry(join_symbol_path(&module_path, &function.sig.ident.to_string()))
                        .or_default();
                }
            }
        }
    }
    let mut super_module_references = SuperModuleReferenceVisitor::default();
    for file in files {
        super_module_references.visit_file(&file.syntax);
    }
    loop {
        let previous = symbols.clone();
        for file in files {
            let mut file_owners = path_owners(&file.relative, operations);
            if is_protocol_serde_path(&file.relative) {
                if let Some(module_owners) = previous.get(&source_module_path(&file.relative)) {
                    file_owners.extend(module_owners.iter().cloned());
                }
            }
            if !file_owners.is_empty() {
                add_symbol(&mut symbols, &source_module_path(&file.relative), &file_owners);
            }
            collect_operation_symbols(
                &file.syntax.items,
                &source_module_path(&file.relative),
                operations,
                waiter_owners,
                &previous,
                &super_module_references.names,
                &file_owners,
                type_owners,
                &mut symbols,
            );
        }
        if symbols == previous {
            break;
        }
    }
    symbols
}

fn collect_operation_symbols(
    items: &[Item],
    parent_path: &str,
    operations: &[Operation],
    waiter_owners: &BTreeMap<String, BTreeSet<String>>,
    known_symbols: &BTreeMap<String, BTreeSet<String>>,
    super_module_references: &BTreeSet<String>,
    inherited_owners: &BTreeSet<String>,
    type_owners: &BTreeMap<String, BTreeSet<String>>,
    symbols: &mut BTreeMap<String, BTreeSet<String>>,
) {
    for item in items {
        let mut visitor = OperationPathVisitor {
            operations,
            waiter_owners,
            symbol_owners: known_symbols,
            owners: BTreeSet::new(),
            method_calls: BTreeSet::new(),
            references: BTreeSet::new(),
        };
        visitor.visit_item(item);
        let item_owners = if inherited_owners.is_empty() {
            inferred_item_owners(parent_path, item, &visitor, type_owners, known_symbols)
        } else {
            inherited_owners.clone()
        };
        for reference in &visitor.references {
            if !is_operation_or_client_symbol(reference) {
                add_symbol(symbols, reference, &item_owners);
            }
        }
        match item {
            Item::Fn(function) => {
                if !item_owners.is_empty() {
                    add_symbol(
                        symbols,
                        &join_symbol_path(parent_path, &function.sig.ident.to_string()),
                        &item_owners,
                    );
                }
            }
            Item::Mod(module) => {
                let Some((_, nested)) = &module.content else {
                    continue;
                };
                let module_name = module.ident.to_string();
                let module_path = join_symbol_path(parent_path, &module_name);
                let mut module_owners = item_owners;
                for child in nested {
                    let mut child_visitor = OperationPathVisitor {
                        operations,
                        waiter_owners,
                        symbol_owners: known_symbols,
                        owners: BTreeSet::new(),
                        method_calls: BTreeSet::new(),
                        references: BTreeSet::new(),
                    };
                    child_visitor.visit_item(child);
                    let child_owners = if module_owners.is_empty() {
                        inferred_item_owners(&module_path, child, &child_visitor, type_owners, known_symbols)
                    } else {
                        module_owners.clone()
                    };
                    for reference in &child_visitor.references {
                        if !is_operation_or_client_symbol(reference) {
                            add_symbol(symbols, reference, &child_owners);
                        }
                    }
                    module_owners.extend(child_owners);
                    if let Item::Mod(child_module) = child {
                        if let Some(owners) = known_symbols.get(&child_module.ident.to_string()) {
                            module_owners.extend(owners.iter().cloned());
                        }
                    }
                }
                if !module_owners.is_empty() {
                    add_symbol(symbols, &module_path, &module_owners);
                    if super_module_references.contains(&module_name) {
                        add_symbol(symbols, &module_name, &module_owners);
                    }
                }
                collect_operation_symbols(
                    nested,
                    &module_path,
                    operations,
                    waiter_owners,
                    known_symbols,
                    super_module_references,
                    &module_owners,
                    type_owners,
                    symbols,
                );
            }
            _ => {}
        }
    }
}

fn build_type_owners(files: &[SourceFile], operations: &[Operation]) -> BTreeMap<String, BTreeSet<String>> {
    let mut owners = BTreeMap::<String, BTreeSet<String>>::new();
    for file in files {
        if !file.relative.starts_with("src/operation/") {
            continue;
        }
        let file_owners = path_owners(&file.relative, operations);
        if file_owners.is_empty() {
            continue;
        }
        let mut visitor = TypeReferenceVisitor::default();
        visitor.visit_file(&file.syntax);
        for reference in visitor.references {
            owners.entry(reference).or_default().extend(file_owners.iter().cloned());
        }
    }
    let type_modules = type_module_keys(files);
    loop {
        let previous = owners.clone();
        for file in files {
            let module = source_module_path(&file.relative);
            let Some(type_keys) = type_modules.get(&module) else {
                continue;
            };
            let type_owners = type_keys
                .iter()
                .filter_map(|key| owners.get(key))
                .flat_map(|owners| owners.iter().cloned())
                .collect::<BTreeSet<_>>();
            if type_owners.is_empty() {
                continue;
            }
            let mut visitor = TypeReferenceVisitor::default();
            visitor.visit_file(&file.syntax);
            for reference in visitor.references {
                if reference.starts_with("types::") {
                    owners.entry(reference).or_default().extend(type_owners.iter().cloned());
                }
            }
        }
        if owners == previous {
            break;
        }
    }
    owners
}

fn type_module_keys(files: &[SourceFile]) -> BTreeMap<String, BTreeSet<String>> {
    let mut modules = BTreeMap::new();
    for relative in ["src/types.rs", "src/types/error.rs"] {
        let Some(file) = files.iter().find(|file| file.relative == relative) else {
            continue;
        };
        for item in &file.syntax.items {
            let Item::Use(item_use) = item else {
                continue;
            };
            let mut paths = Vec::new();
            collect_use_paths(&item_use.tree, &mut Vec::new(), &mut paths);
            for path in paths {
                let Some((module, key)) = type_import_parts(&path, false) else {
                    continue;
                };
                modules.entry(module).or_insert_with(BTreeSet::new).insert(key);
            }
        }
    }
    modules
}

fn collect_use_paths(tree: &UseTree, prefix: &mut Vec<String>, paths: &mut Vec<Vec<String>>) {
    match tree {
        UseTree::Path(path) => {
            prefix.push(path.ident.to_string());
            collect_use_paths(&path.tree, prefix, paths);
            prefix.pop();
        }
        UseTree::Name(name) => {
            prefix.push(name.ident.to_string());
            paths.push(prefix.clone());
            prefix.pop();
        }
        UseTree::Rename(rename) => {
            prefix.push(rename.ident.to_string());
            paths.push(prefix.clone());
            prefix.pop();
        }
        UseTree::Group(group) => {
            for tree in &group.items {
                collect_use_paths(tree, prefix, paths);
            }
        }
        UseTree::Glob(_) => {}
    }
}

fn type_import_parts(path: &[String], builder: bool) -> Option<(String, String)> {
    let types_index = path.iter().position(|segment| segment == "types")?;
    let imported = path.last()?.as_str();
    let imported = if builder {
        imported.strip_suffix("Builder")?
    } else {
        imported
    };
    if imported.is_empty() || types_index + 2 >= path.len() {
        return None;
    }
    let module = path[types_index..path.len() - 1].join("::");
    let namespace = if path.get(types_index + 1).map(String::as_str) == Some("error") {
        "types::error"
    } else {
        "types"
    };
    Some((module, format!("{namespace}::{imported}")))
}

fn type_root_item_owners(
    relative: &str,
    item: &Item,
    files: &[SourceFile],
    type_owners: &BTreeMap<String, BTreeSet<String>>,
) -> BTreeSet<String> {
    let builder = matches!(relative, "src/types/builders.rs" | "src/types/error/builders.rs");
    match item {
        Item::Use(item_use) => {
            let mut paths = Vec::new();
            collect_use_paths(&item_use.tree, &mut Vec::new(), &mut paths);
            paths
                .iter()
                .filter_map(|path| type_import_parts(path, builder))
                .filter_map(|(_, key)| type_owners.get(&key))
                .flat_map(|owners| owners.iter().cloned())
                .collect()
        }
        Item::Mod(module) => {
            let namespace = match relative {
                "src/types.rs" | "src/types/builders.rs" => "types",
                "src/types/error.rs" | "src/types/error/builders.rs" => "types::error",
                _ => return BTreeSet::new(),
            };
            let module = format!("{namespace}::{}", module.ident);
            type_module_keys(files)
                .get(&module)
                .into_iter()
                .flatten()
                .filter_map(|key| type_owners.get(key))
                .flat_map(|owners| owners.iter().cloned())
                .collect()
        }
        _ => BTreeSet::new(),
    }
}

fn error_variant_owners(
    files: &[SourceFile],
    type_owners: &BTreeMap<String, BTreeSet<String>>,
) -> BTreeMap<String, BTreeSet<String>> {
    let mut owners = BTreeMap::new();
    let Some(file) = files.iter().find(|file| file.relative == "src/error_meta.rs") else {
        return owners;
    };
    for item in &file.syntax.items {
        let Item::Enum(item_enum) = item else {
            continue;
        };
        if item_enum.ident != "Error" {
            continue;
        }
        for variant in &item_enum.variants {
            let mut visitor = TypeReferenceVisitor::default();
            visitor.visit_variant(variant);
            let variant_owners = type_reference_owner_set(&visitor.references, type_owners);
            if !variant_owners.is_empty() {
                owners.insert(variant.ident.to_string(), variant_owners);
            }
        }
    }
    owners
}

fn type_reference_owner_set(
    references: &BTreeSet<String>,
    type_owners: &BTreeMap<String, BTreeSet<String>>,
) -> BTreeSet<String> {
    intersect_owner_sets(references.iter().filter_map(|reference| type_owners.get(reference)))
}

fn collect_error_variant_edits(
    relative: &str,
    source: &str,
    item: &Item,
    variant_owners: &BTreeMap<String, BTreeSet<String>>,
    edits: &mut BTreeMap<String, BTreeMap<usize, BTreeSet<String>>>,
) -> Result<(), TransformError> {
    let Item::Enum(item_enum) = item else {
        return Ok(());
    };
    if item_enum.ident != "Error" {
        return Ok(());
    }
    for variant in &item_enum.variants {
        let Some(owners) = variant_owners.get(&variant.ident.to_string()) else {
            continue;
        };
        if has_matching_cfg(&variant.attrs, owners) {
            continue;
        }
        let first = variant
            .to_token_stream()
            .into_iter()
            .next()
            .ok_or_else(|| TransformError::Rust {
                path: PathBuf::from(relative),
                message: "empty Rust enum variant".to_owned(),
            })?;
        let start_span = variant.attrs.first().map(Spanned::span).unwrap_or_else(|| first.span());
        add_edit(
            edits,
            relative,
            source_offset(source, start_span.start())?,
            owners.clone(),
        );
    }
    Ok(())
}

struct ErrorMatchArmVisitor<'a> {
    relative: &'a str,
    source: &'a str,
    variant_owners: &'a BTreeMap<String, BTreeSet<String>>,
    edits: &'a mut BTreeMap<String, BTreeMap<usize, BTreeSet<String>>>,
    error: Option<TransformError>,
}

impl<'ast> Visit<'ast> for ErrorMatchArmVisitor<'_> {
    fn visit_expr_match(&mut self, expression: &'ast syn::ExprMatch) {
        for arm in &expression.arms {
            let Some(variant) = match_arm_variant(&arm.pat) else {
                continue;
            };
            let Some(owners) = self.variant_owners.get(&variant) else {
                continue;
            };
            if has_matching_cfg(&arm.attrs, owners) {
                continue;
            }
            let Some(first) = arm.to_token_stream().into_iter().next() else {
                self.error = Some(TransformError::Rust {
                    path: PathBuf::from(self.relative),
                    message: "empty Rust match arm".to_owned(),
                });
                return;
            };
            match source_offset(self.source, first.span().start()) {
                Ok(offset) => add_edit(self.edits, self.relative, offset, owners.clone()),
                Err(error) => {
                    self.error = Some(error);
                    return;
                }
            }
        }
        visit::visit_expr_match(self, expression);
    }
}

fn match_arm_variant(pattern: &syn::Pat) -> Option<String> {
    let path = match pattern {
        syn::Pat::Path(path) => &path.path,
        syn::Pat::Struct(pattern) => &pattern.path,
        syn::Pat::TupleStruct(pattern) => &pattern.path,
        syn::Pat::Reference(pattern) => return match_arm_variant(&pattern.pat),
        _ => return None,
    };
    path.segments.last().map(|segment| segment.ident.to_string())
}

fn collect_error_match_arm_edits(
    relative: &str,
    source: &str,
    item_impl: &syn::ItemImpl,
    variant_owners: &BTreeMap<String, BTreeSet<String>>,
    edits: &mut BTreeMap<String, BTreeMap<usize, BTreeSet<String>>>,
) -> Result<(), TransformError> {
    let self_type = item_impl.self_ty.to_token_stream().to_string();
    let trait_name = item_impl
        .trait_
        .as_ref()
        .and_then(|(path, _)| path.segments.last())
        .map(|segment| segment.ident.to_string());
    if self_type != "Error"
        || !matches!(
            trait_name.as_deref(),
            Some("Display" | "ProvideErrorMetadata" | "Error" | "RequestId" | "RequestIdExt")
        )
    {
        return Ok(());
    }
    let mut visitor = ErrorMatchArmVisitor {
        relative,
        source,
        variant_owners,
        edits,
        error: None,
    };
    visitor.visit_item_impl(item_impl);
    if let Some(error) = visitor.error {
        return Err(error);
    }
    Ok(())
}

fn inferred_item_owners(
    parent_path: &str,
    item: &Item,
    visitor: &OperationPathVisitor<'_>,
    type_owners: &BTreeMap<String, BTreeSet<String>>,
    known_symbols: &BTreeMap<String, BTreeSet<String>>,
) -> BTreeSet<String> {
    if !visitor.owners.is_empty() {
        return visitor.owners.clone();
    }
    let declared_owners = operation_error_owners_for_parent(parent_path, item, type_owners);
    if !declared_owners.is_empty() {
        return declared_owners;
    }
    if parent_path == "protocol_serde" {
        if let Item::Fn(function) = item {
            if let Some(owners) = known_symbols.get(&join_symbol_path(parent_path, &function.sig.ident.to_string())) {
                if !owners.is_empty() {
                    return owners.clone();
                }
            }
        }
    }
    if is_type_ownership_module(parent_path) {
        let type_reference_owners = type_reference_owners(item, type_owners);
        if !type_reference_owners.is_empty() {
            return type_reference_owners;
        }
    }
    let mut owners = protocol_reference_owners(&visitor.references, known_symbols);
    let mut super_references = SuperModuleReferenceVisitor::default();
    super_references.visit_item(item);
    owners.extend(
        super_references
            .names
            .iter()
            .filter_map(|reference| known_symbols.get(reference))
            .flat_map(|owners| owners.iter().cloned()),
    );
    owners
}

fn type_reference_owners(item: &Item, type_owners: &BTreeMap<String, BTreeSet<String>>) -> BTreeSet<String> {
    let mut visitor = TypeReferenceVisitor::default();
    visitor.visit_item(item);
    type_reference_owner_set(&visitor.references, type_owners)
}

fn intersect_owner_sets<'a>(owner_sets: impl IntoIterator<Item = &'a BTreeSet<String>>) -> BTreeSet<String> {
    let mut owner_sets = owner_sets.into_iter();
    let Some(first) = owner_sets.next() else {
        return BTreeSet::new();
    };
    let mut result = first.clone();
    for owners in owner_sets {
        result.retain(|owner| owners.contains(owner));
    }
    result
}

fn operation_error_owners(
    relative: &str,
    item: &Item,
    type_owners: &BTreeMap<String, BTreeSet<String>>,
) -> BTreeSet<String> {
    if relative != "src/types/error.rs" {
        return BTreeSet::new();
    }
    operation_error_owners_for_name(item_type_name(item), type_owners)
}

fn operation_error_owners_for_parent(
    parent_path: &str,
    item: &Item,
    type_owners: &BTreeMap<String, BTreeSet<String>>,
) -> BTreeSet<String> {
    if parent_path != "types::error" {
        return BTreeSet::new();
    }
    operation_error_owners_for_name(item_type_name(item), type_owners)
}

fn operation_error_owners_for_name(
    name: Option<String>,
    type_owners: &BTreeMap<String, BTreeSet<String>>,
) -> BTreeSet<String> {
    let Some(name) = name.filter(|name| name != "Error" && name.ends_with("Error")) else {
        return BTreeSet::new();
    };
    type_owners
        .get(&format!("types::error::{name}"))
        .cloned()
        .unwrap_or_default()
}

fn operation_error_reference_owners(
    item: &Item,
    type_owners: &BTreeMap<String, BTreeSet<String>>,
    operation_error_names: &BTreeSet<String>,
) -> BTreeSet<String> {
    if matches!(item, Item::Enum(item) if item.ident == "Error") {
        return BTreeSet::new();
    }
    let mut visitor = TypeReferenceVisitor::default();
    visitor.visit_item(item);
    intersect_owner_sets(visitor.references.iter().filter_map(|reference| {
        let name = reference.strip_prefix("types::error::")?;
        if name.contains("::") || !name.ends_with("Error") {
            return None;
        }
        if !operation_error_names.contains(name) {
            return None;
        }
        type_owners.get(reference)
    }))
}

fn operation_error_names(files: &[SourceFile]) -> BTreeSet<String> {
    files
        .iter()
        .find(|file| file.relative == "src/types/error.rs")
        .into_iter()
        .flat_map(|file| file.syntax.items.iter())
        .filter_map(|item| match item {
            Item::Enum(item) if item.ident != "Error" && item.ident.to_string().ends_with("Error") => {
                Some(item.ident.to_string())
            }
            _ => None,
        })
        .collect()
}

fn item_type_name(item: &Item) -> Option<String> {
    match item {
        Item::Enum(item) => Some(item.ident.to_string()),
        Item::Impl(item) => match item.self_ty.as_ref() {
            syn::Type::Path(path) => path.path.segments.last().map(|segment| segment.ident.to_string()),
            _ => None,
        },
        _ => None,
    }
}

fn is_type_ownership_path(relative: &str) -> bool {
    relative.strip_prefix("src/") == Some("event_stream_serde.rs")
        || relative.strip_prefix("src/") == Some("serde_util.rs")
        || is_protocol_serde_path(relative)
}

fn is_type_ownership_module(module: &str) -> bool {
    module == "event_stream_serde" || module == "protocol_serde" || module.starts_with("protocol_serde::")
}

fn add_symbol(symbols: &mut BTreeMap<String, BTreeSet<String>>, name: &str, owners: &BTreeSet<String>) {
    symbols
        .entry(name.to_owned())
        .or_default()
        .extend(owners.iter().cloned());
}

fn is_operation_or_client_symbol(symbol: &str) -> bool {
    matches!(symbol.split("::").next(), Some("operation" | "client"))
}

fn protocol_reference_owners(
    references: &BTreeSet<String>,
    symbol_owners: &BTreeMap<String, BTreeSet<String>>,
) -> BTreeSet<String> {
    references
        .iter()
        .filter(|reference| is_protocol_shape_symbol(reference))
        .filter_map(|reference| symbol_owners.get(reference))
        .flat_map(|owners| owners.iter().cloned())
        .collect()
}

fn is_protocol_shape_symbol(symbol: &str) -> bool {
    let mut segments = symbol.split("::");
    segments.next() == Some("protocol_serde") && segments.next().is_some_and(|segment| segment.starts_with("shape_"))
}

fn operation_symbol_reference_owners(
    references: &BTreeSet<String>,
    symbol_owners: &BTreeMap<String, BTreeSet<String>>,
    local_module_roots: &BTreeSet<String>,
) -> BTreeSet<String> {
    references
        .iter()
        .filter(|reference| is_local_symbol_reference(reference, local_module_roots))
        .filter(|reference| {
            !is_operation_or_client_symbol(reference)
                && !matches!(
                    reference.split("::").next(),
                    Some("config" | "error" | "error_meta" | "types")
                )
        })
        .filter_map(|reference| symbol_owners.get(reference))
        .flat_map(|owners| owners.iter().cloned())
        .collect()
}

fn is_local_symbol_reference(reference: &str, local_module_roots: &BTreeSet<String>) -> bool {
    let mut segments = reference.split("::");
    match segments.next() {
        Some("crate" | "self" | "super") => true,
        Some(root) => local_module_roots.contains(root),
        None => false,
    }
}

fn join_symbol_path(parent: &str, name: &str) -> String {
    if parent.is_empty() {
        name.to_owned()
    } else {
        format!("{parent}::{name}")
    }
}

fn source_module_path(relative: &str) -> String {
    let path = relative.strip_prefix("src/").unwrap_or(relative);
    let mut components = path.split('/').map(str::to_owned).collect::<Vec<_>>();
    if let Some(last) = components.last_mut() {
        if let Some(stem) = last.strip_suffix(".rs") {
            *last = stem.to_owned();
        }
    }
    if components
        .last()
        .is_some_and(|component| component == "lib" || component == "mod")
    {
        components.pop();
    }
    components.join("::")
}

fn is_protocol_serde_path(relative: &str) -> bool {
    let path = relative.strip_prefix("src/").unwrap_or(relative);
    path == "protocol_serde.rs" || path.starts_with("protocol_serde/")
}

fn is_type_root_path(relative: &str) -> bool {
    matches!(
        relative,
        "src/types.rs" | "src/types/builders.rs" | "src/types/error.rs" | "src/types/error/builders.rs"
    )
}

fn is_type_definition_child_path(relative: &str) -> bool {
    let path = relative.strip_prefix("src/").unwrap_or(relative);
    (path.starts_with("types/_") || path.starts_with("types/error/_")) && path.ends_with(".rs")
}

fn local_module_roots(files: &[SourceFile]) -> BTreeSet<String> {
    files
        .iter()
        .filter_map(|file| source_module_path(&file.relative).split("::").next().map(str::to_owned))
        .filter(|root| !root.is_empty())
        .collect()
}

fn waiter_owners(files: &[SourceFile], operations: &[Operation]) -> BTreeMap<String, BTreeSet<String>> {
    let mut result = BTreeMap::new();
    for file in files {
        let Some(relative) = file.relative.strip_prefix("src/waiters/") else {
            continue;
        };
        let Some(waiter) = relative.strip_suffix(".rs") else {
            continue;
        };
        let mut visitor = OperationPathVisitor {
            operations,
            waiter_owners: &BTreeMap::new(),
            symbol_owners: &BTreeMap::new(),
            owners: BTreeSet::new(),
            method_calls: BTreeSet::new(),
            references: BTreeSet::new(),
        };
        visitor.visit_file(&file.syntax);
        if !visitor.owners.is_empty() {
            result.insert(waiter.to_owned(), visitor.owners);
        }
    }
    result
}

fn path_owners(relative: &str, operations: &[Operation]) -> BTreeSet<String> {
    let mut owners = BTreeSet::new();
    let path = relative.strip_prefix("src/").unwrap_or(relative);
    let components = path.split('/').collect::<Vec<_>>();
    if components.len() >= 2 && (components[0] == "operation" || components[0] == "client") {
        if let Some(operation) = operations
            .iter()
            .find(|operation| components[1].trim_end_matches(".rs") == operation.module)
        {
            owners.insert(operation.feature.clone());
        }
    }
    if components.first() == Some(&"protocol_serde") {
        let filename = components
            .last()
            .and_then(|filename| filename.strip_suffix(".rs"))
            .unwrap_or_default();
        if let Some(rest) = filename.strip_prefix("shape_") {
            let operation = operations.iter().find(|operation| rest == operation.module);
            if let Some(operation) = operation {
                owners.insert(operation.feature.clone());
            }
        }
    }
    owners
}

fn is_operation_or_client_child_path(relative: &str, operations: &[Operation]) -> bool {
    let path = relative.strip_prefix("src/").unwrap_or(relative);
    let mut components = path.split('/');
    matches!(components.next(), Some("operation" | "client"))
        && components
            .next()
            .map(|module| module.trim_end_matches(".rs"))
            .is_some_and(|module| operations.iter().any(|operation| operation.module == module))
}

fn parent_operation_feature<'a>(relative: &str, operations: &'a [Operation]) -> Option<&'a str> {
    let path = relative.strip_prefix("src/").unwrap_or(relative);
    let mut components = path.split('/');
    if !matches!(components.next(), Some("operation")) {
        return None;
    }
    let module = components.next()?.trim_end_matches(".rs");
    operations
        .iter()
        .find(|operation| operation.module == module)
        .map(|operation| operation.feature.as_str())
}

struct RedundantCfgVisitor<'a> {
    source: &'a str,
    feature: Option<&'a str>,
    operation_features: &'a BTreeSet<String>,
    removals: Vec<(usize, usize)>,
}

impl<'ast> Visit<'ast> for RedundantCfgVisitor<'_> {
    fn visit_attribute(&mut self, attribute: &'ast Attribute) {
        let matches_parent = self
            .feature
            .is_some_and(|feature| is_exact_feature_cfg(attribute, feature));
        let matches_protocol_parent = self.feature.is_none()
            && self
                .operation_features
                .iter()
                .any(|feature| is_exact_feature_cfg(attribute, feature));
        if matches_parent || matches_protocol_parent {
            if let (Ok(start), Ok(mut end)) = (
                source_offset(self.source, attribute.span().start()),
                source_offset(self.source, attribute.span().end()),
            ) {
                if self.source.as_bytes().get(end) == Some(&b'\r') {
                    end += 1;
                }
                if self.source.as_bytes().get(end) == Some(&b'\n') {
                    end += 1;
                }
                self.removals.push((start, end));
            }
        }
        visit::visit_attribute(self, attribute);
    }
}

fn is_exact_feature_cfg(attribute: &Attribute, feature: &str) -> bool {
    if !attribute.path().is_ident("cfg") {
        return false;
    }
    let Meta::List(list) = &attribute.meta else {
        return false;
    };
    list.tokens.to_string() == format!("feature = {feature:?}")
}

fn remove_redundant_operation_child_cfgs(
    crate_root: &Path,
    files: &[SourceFile],
    operations: &[Operation],
) -> Result<Vec<String>, TransformError> {
    let mut changed = Vec::new();
    let operation_features = operations
        .iter()
        .map(|operation| operation.feature.clone())
        .collect::<BTreeSet<_>>();
    for file in files {
        let feature = parent_operation_feature(&file.relative, operations);
        let protocol_child = is_protocol_serde_child_path(&file.relative);
        if feature.is_none() && !protocol_child {
            continue;
        }
        let mut visitor = RedundantCfgVisitor {
            source: &file.source,
            feature,
            operation_features: &operation_features,
            removals: Vec::new(),
        };
        visitor.visit_file(&file.syntax);
        if visitor.removals.is_empty() {
            continue;
        }
        let updated = apply_removals(&file.source, &visitor.removals, &file.relative)?;
        if updated == file.source {
            continue;
        }
        fs::write(crate_root.join(&file.relative), updated).map_err(|source| TransformError::Io {
            path: crate_root.join(&file.relative),
            source,
        })?;
        changed.push(file.relative.clone());
    }
    Ok(changed)
}

fn is_parent_gated_child_path(relative: &str, operations: &[Operation]) -> bool {
    is_operation_or_client_child_path(relative, operations)
        || is_protocol_serde_child_path(relative)
        || waiter_module_name(relative).is_some_and(|name| name != "matchers")
}

fn is_protocol_serde_child_path(relative: &str) -> bool {
    relative
        .strip_prefix("src/protocol_serde/")
        .is_some_and(|path| path.ends_with(".rs"))
}

fn waiter_module_name(relative: &str) -> Option<&str> {
    let path = relative.strip_prefix("src/waiters/")?;
    let name = path.strip_suffix(".rs")?;
    (!name.contains('/')).then_some(name)
}

fn collect_module_edits(
    relative: &str,
    source: &str,
    item: &Item,
    files: &[SourceFile],
    operations: &[Operation],
    waiter_owners: &BTreeMap<String, BTreeSet<String>>,
    symbol_owners: &BTreeMap<String, BTreeSet<String>>,
    method_owners: &MethodOwnership,
    edits: &mut BTreeMap<String, BTreeMap<usize, BTreeSet<String>>>,
) -> Result<(), TransformError> {
    let Item::Mod(module) = item else {
        return Ok(());
    };
    let current_virtual = module_virtual_path(relative, &module.ident.to_string());
    let child_relative = if module.content.is_some() {
        current_virtual.clone()
    } else {
        resolve_external_module(relative, &module.ident.to_string(), files).ok_or_else(|| TransformError::Invalid {
            path: PathBuf::from(relative),
            message: format!("module `{}` has no source file", module.ident),
        })?
    };
    let child_path = if child_relative.ends_with(".rs") {
        child_relative.clone()
    } else {
        format!("{child_relative}.rs")
    };
    let mut owners = path_owners(&child_path, operations);
    if let Some(file) = files.iter().find(|file| {
        file.relative == child_relative
            || file.relative == format!("{child_relative}.rs")
            || file.relative == format!("{child_relative}/mod.rs")
    }) {
        if is_protocol_serde_path(&child_path) || !owners.is_empty() {
            let module_path = source_module_path(&file.relative);
            if let Some(module_owners) = symbol_owners.get(&module_path) {
                owners.extend(module_owners.iter().cloned());
            }
        }
        if let Some(waiter) = waiter_module_name(&child_relative) {
            owners.extend(waiter_owners.get(waiter).into_iter().flatten().cloned());
        } else if child_relative.starts_with("src/waiters/") {
            let mut visitor = OperationPathVisitor {
                operations,
                waiter_owners,
                symbol_owners,
                owners: BTreeSet::new(),
                method_calls: BTreeSet::new(),
                references: BTreeSet::new(),
            };
            visitor.visit_file(&file.syntax);
            owners.extend(visitor.owners);
        }
    }
    if module.content.is_some() {
        let module_name = module.ident.to_string();
        let module_path = join_symbol_path(&source_module_path(relative), &module_name);
        for symbol in [module_name, module_path] {
            if let Some(symbol_module_owners) = symbol_owners.get(&symbol) {
                owners.extend(symbol_module_owners.iter().cloned());
            }
        }
    }
    if !owners.is_empty() && !has_matching_cfg(&module.attrs, &owners) {
        add_edit(edits, relative, item_start(item, source)?, owners);
    }
    if let Some((_, items)) = &module.content {
        for child in items {
            collect_module_edits(
                relative,
                source,
                child,
                files,
                operations,
                waiter_owners,
                symbol_owners,
                method_owners,
                edits,
            )?;
        }
        collect_inline_item_edits(
            relative,
            source,
            items,
            operations,
            waiter_owners,
            symbol_owners,
            method_owners,
            edits,
        )?;
    }
    Ok(())
}

struct StatementCfgVisitor<'a> {
    relative: &'a str,
    source: &'a str,
    operations: &'a [Operation],
    waiter_owners: &'a BTreeMap<String, BTreeSet<String>>,
    symbol_owners: &'a BTreeMap<String, BTreeSet<String>>,
    local_module_roots: &'a BTreeSet<String>,
    edits: &'a mut StatementEdits,
    error: Option<TransformError>,
}

impl<'ast> Visit<'ast> for StatementCfgVisitor<'_> {
    fn visit_stmt(&mut self, statement: &'ast syn::Stmt) {
        let mut visitor = OperationPathVisitor {
            operations: self.operations,
            waiter_owners: self.waiter_owners,
            symbol_owners: self.symbol_owners,
            owners: BTreeSet::new(),
            method_calls: BTreeSet::new(),
            references: BTreeSet::new(),
        };
        visitor.visit_stmt(statement);
        let owners =
            operation_symbol_reference_owners(&visitor.references, self.symbol_owners, self.local_module_roots);
        if !owners.is_empty() && !has_matching_cfg(statement_attrs(statement), &owners) {
            match statement_range(statement, self.source) {
                Ok((start, end)) => self
                    .edits
                    .entry(self.relative.to_owned())
                    .or_default()
                    .push((start, end, owners)),
                Err(error) => {
                    self.error = Some(error);
                    return;
                }
            }
            return;
        }
        visit::visit_stmt(self, statement);
    }
}

fn collect_statement_edits(
    relative: &str,
    source: &str,
    item: &Item,
    operations: &[Operation],
    waiter_owners: &BTreeMap<String, BTreeSet<String>>,
    symbol_owners: &BTreeMap<String, BTreeSet<String>>,
    local_module_roots: &BTreeSet<String>,
    statement_edits: &mut StatementEdits,
) -> Result<(), TransformError> {
    let mut visitor = StatementCfgVisitor {
        relative,
        source,
        operations,
        waiter_owners,
        symbol_owners,
        local_module_roots,
        edits: statement_edits,
        error: None,
    };
    visitor.visit_item(item);
    if let Some(error) = visitor.error {
        return Err(error);
    }
    Ok(())
}

fn statement_attrs(statement: &syn::Stmt) -> &[Attribute] {
    match statement {
        syn::Stmt::Local(local) => &local.attrs,
        syn::Stmt::Item(item) => item_attrs(item),
        syn::Stmt::Expr(_, _) => &[],
        syn::Stmt::Macro(statement_macro) => &statement_macro.attrs,
    }
}

fn statement_range(statement: &syn::Stmt, source: &str) -> Result<(usize, usize), TransformError> {
    token_range(statement.to_token_stream(), statement_attrs(statement), source)
}

fn collect_inline_item_edits(
    relative: &str,
    source: &str,
    items: &[Item],
    operations: &[Operation],
    waiter_owners: &BTreeMap<String, BTreeSet<String>>,
    symbol_owners: &BTreeMap<String, BTreeSet<String>>,
    method_owners: &MethodOwnership,
    edits: &mut BTreeMap<String, BTreeMap<usize, BTreeSet<String>>>,
) -> Result<(), TransformError> {
    for item in items {
        if let Item::Impl(item_impl) = item {
            let header_owners = impl_header_owners(item_impl, operations, waiter_owners, symbol_owners);
            let method_impl_owners = impl_method_owners(item_impl, method_owners);
            if !header_owners.is_empty() {
                if !has_matching_cfg(item_attrs(item), &header_owners) {
                    add_edit(edits, relative, item_start(item, source)?, header_owners);
                }
            } else if item_impl.trait_.is_some() && !method_impl_owners.is_empty() {
                if !has_matching_cfg(item_attrs(item), &method_impl_owners) {
                    add_edit(edits, relative, item_start(item, source)?, method_impl_owners);
                }
            } else {
                collect_impl_method_edits(relative, source, item_impl, method_owners, edits)?;
            }
        } else {
            let mut visitor = OperationPathVisitor {
                operations,
                waiter_owners,
                symbol_owners,
                owners: BTreeSet::new(),
                method_calls: BTreeSet::new(),
                references: BTreeSet::new(),
            };
            visitor.visit_item(item);
            if !visitor.owners.is_empty() && !has_matching_cfg(item_attrs(item), &visitor.owners) {
                add_edit(edits, relative, item_start(item, source)?, visitor.owners);
            }
        }
        if let Item::Mod(module) = item {
            if let Some((_, nested)) = &module.content {
                collect_inline_item_edits(
                    relative,
                    source,
                    nested,
                    operations,
                    waiter_owners,
                    symbol_owners,
                    method_owners,
                    edits,
                )?;
            }
        }
    }
    Ok(())
}

fn collect_impl_method_edits(
    relative: &str,
    source: &str,
    item_impl: &syn::ItemImpl,
    method_owners: &MethodOwnership,
    edits: &mut BTreeMap<String, BTreeMap<usize, BTreeSet<String>>>,
) -> Result<(), TransformError> {
    let context = impl_context(item_impl);
    for item in &item_impl.items {
        let syn::ImplItem::Fn(method) = item else {
            continue;
        };
        let key = method_key(&context, &method.sig.ident.to_string());
        let owners = method_owners.get(&key).cloned().unwrap_or_default();
        if !owners.is_empty() && !has_matching_cfg(&method.attrs, &owners) {
            let first = method
                .to_token_stream()
                .into_iter()
                .next()
                .ok_or_else(|| TransformError::Rust {
                    path: PathBuf::from(relative),
                    message: "empty Rust impl item".to_owned(),
                })?;
            add_edit(edits, relative, source_offset(source, first.span().start())?, owners);
        }
    }
    Ok(())
}

fn trait_method_owners(
    item_trait: &syn::ItemTrait,
    operations: &[Operation],
    waiter_owners: &BTreeMap<String, BTreeSet<String>>,
    symbol_owners: &BTreeMap<String, BTreeSet<String>>,
) -> BTreeSet<String> {
    item_trait
        .items
        .iter()
        .filter_map(|item| {
            let syn::TraitItem::Fn(method) = item else {
                return None;
            };
            let mut visitor = OperationPathVisitor {
                operations,
                waiter_owners,
                symbol_owners,
                owners: BTreeSet::new(),
                method_calls: BTreeSet::new(),
                references: BTreeSet::new(),
            };
            visitor.visit_trait_item_fn(method);
            Some(visitor.owners)
        })
        .flatten()
        .collect()
}

fn collect_trait_method_edits(
    relative: &str,
    source: &str,
    item_trait: &syn::ItemTrait,
    operations: &[Operation],
    waiter_owners: &BTreeMap<String, BTreeSet<String>>,
    symbol_owners: &BTreeMap<String, BTreeSet<String>>,
    edits: &mut BTreeMap<String, BTreeMap<usize, BTreeSet<String>>>,
) -> Result<(), TransformError> {
    for item in &item_trait.items {
        let syn::TraitItem::Fn(method) = item else {
            continue;
        };
        let mut visitor = OperationPathVisitor {
            operations,
            waiter_owners,
            symbol_owners,
            owners: BTreeSet::new(),
            method_calls: BTreeSet::new(),
            references: BTreeSet::new(),
        };
        visitor.visit_trait_item_fn(method);
        if visitor.owners.is_empty() || has_matching_cfg(&method.attrs, &visitor.owners) {
            continue;
        }
        let first = method
            .to_token_stream()
            .into_iter()
            .next()
            .ok_or_else(|| TransformError::Rust {
                path: PathBuf::from(relative),
                message: "empty Rust trait item".to_owned(),
            })?;
        add_edit(
            edits,
            relative,
            source_offset(source, first.span().start())?,
            visitor.owners,
        );
    }
    Ok(())
}

fn collect_method_owners(
    items: &[Item],
    operations: &[Operation],
    waiter_owners: &BTreeMap<String, BTreeSet<String>>,
    symbol_owners: &BTreeMap<String, BTreeSet<String>>,
) -> MethodOwnership {
    let mut methods = BTreeMap::<MethodKey, (BTreeSet<String>, BTreeSet<String>)>::new();
    collect_method_info(items, operations, waiter_owners, symbol_owners, &mut methods);
    loop {
        let mut changed = false;
        let current = methods.clone();
        for (key, (owners, calls)) in methods.iter_mut() {
            for call in calls.iter() {
                let inherent_key = MethodKey {
                    self_type: key.self_type.clone(),
                    trait_type: None,
                    name: call.clone(),
                };
                let trait_key = MethodKey {
                    self_type: key.self_type.clone(),
                    trait_type: key.trait_type.clone(),
                    name: call.clone(),
                };
                if let Some((called_owners, _)) = current.get(&inherent_key).or_else(|| current.get(&trait_key)) {
                    let before = owners.len();
                    owners.extend(called_owners.iter().cloned());
                    changed |= before != owners.len();
                }
            }
        }
        if !changed {
            break;
        }
    }
    methods.into_iter().map(|(key, (owners, _))| (key, owners)).collect()
}

fn impl_header_owners(
    item_impl: &syn::ItemImpl,
    operations: &[Operation],
    waiter_owners: &BTreeMap<String, BTreeSet<String>>,
    symbol_owners: &BTreeMap<String, BTreeSet<String>>,
) -> BTreeSet<String> {
    let mut visitor = OperationPathVisitor {
        operations,
        waiter_owners,
        symbol_owners,
        owners: BTreeSet::new(),
        method_calls: BTreeSet::new(),
        references: BTreeSet::new(),
    };
    visitor.visit_generics(&item_impl.generics);
    visitor.visit_type(&item_impl.self_ty);
    if let Some((trait_path, _)) = &item_impl.trait_ {
        visitor.visit_path(trait_path);
    }
    visitor.owners
}

fn impl_method_owners(item_impl: &syn::ItemImpl, method_owners: &MethodOwnership) -> BTreeSet<String> {
    let context = impl_context(item_impl);
    item_impl
        .items
        .iter()
        .filter_map(|item| match item {
            syn::ImplItem::Fn(method) => method_owners.get(&method_key(&context, &method.sig.ident.to_string())),
            _ => None,
        })
        .flat_map(|owners| owners.iter().cloned())
        .collect()
}

fn collect_method_info(
    items: &[Item],
    operations: &[Operation],
    waiter_owners: &BTreeMap<String, BTreeSet<String>>,
    symbol_owners: &BTreeMap<String, BTreeSet<String>>,
    methods: &mut BTreeMap<MethodKey, (BTreeSet<String>, BTreeSet<String>)>,
) {
    for item in items {
        match item {
            Item::Impl(item_impl) => {
                let context = impl_context(item_impl);
                for item in &item_impl.items {
                    let syn::ImplItem::Fn(method) = item else {
                        continue;
                    };
                    let mut visitor = OperationPathVisitor {
                        operations,
                        waiter_owners,
                        symbol_owners,
                        owners: BTreeSet::new(),
                        method_calls: BTreeSet::new(),
                        references: BTreeSet::new(),
                    };
                    visitor.visit_impl_item_fn(method);
                    let key = method_key(&context, &method.sig.ident.to_string());
                    let entry = methods.entry(key).or_default();
                    entry.0.extend(visitor.owners);
                    entry.1.extend(visitor.method_calls);
                }
            }
            Item::Mod(module) => {
                if let Some((_, nested)) = &module.content {
                    collect_method_info(nested, operations, waiter_owners, symbol_owners, methods);
                }
            }
            _ => {}
        }
    }
}

fn resolve_external_module(relative: &str, name: &str, files: &[SourceFile]) -> Option<String> {
    let virtual_path = module_virtual_path(relative, name);
    let candidates = [format!("{virtual_path}.rs"), format!("{virtual_path}/mod.rs")];
    let matches = candidates
        .iter()
        .filter(|candidate| files.iter().any(|file| &file.relative == *candidate))
        .collect::<Vec<_>>();
    (matches.len() == 1).then(|| matches[0].to_string())
}

fn module_virtual_path(relative: &str, name: &str) -> String {
    let path = Path::new(relative);
    let parent = if path.file_name().and_then(|name| name.to_str()) == Some("lib.rs")
        || path.file_name().and_then(|name| name.to_str()) == Some("mod.rs")
    {
        path.parent().unwrap_or_else(|| Path::new("src")).to_owned()
    } else {
        path.with_extension("")
    };
    format!("{}/{}", parent.to_string_lossy().replace('\\', "/"), name)
}

fn item_start(item: &Item, source: &str) -> Result<usize, TransformError> {
    let first = item
        .to_token_stream()
        .into_iter()
        .next()
        .ok_or_else(|| TransformError::Rust {
            path: PathBuf::from("<source>"),
            message: "empty Rust item".to_owned(),
        })?;
    source_offset(source, first.span().start())
}

fn source_offset(source: &str, location: LineColumn) -> Result<usize, TransformError> {
    let mut offset = 0;
    for (line_index, line) in source.split_inclusive('\n').enumerate() {
        if line_index + 1 == location.line {
            return line
                .as_bytes()
                .get(..location.column)
                .map(|_| offset + location.column)
                .ok_or_else(|| TransformError::Rust {
                    path: PathBuf::from("<source>"),
                    message: "token span is outside source".to_owned(),
                });
        }
        offset += line.len();
    }
    Err(TransformError::Rust {
        path: PathBuf::from("<source>"),
        message: "token span has no source line".to_owned(),
    })
}

fn add_edit(
    edits: &mut BTreeMap<String, BTreeMap<usize, BTreeSet<String>>>,
    relative: &str,
    offset: usize,
    owners: BTreeSet<String>,
) {
    edits
        .entry(relative.to_owned())
        .or_default()
        .entry(offset)
        .or_default()
        .extend(owners);
}

fn add_removal(removals: &mut BTreeMap<String, Vec<(usize, usize)>>, relative: &str, range: (usize, usize)) {
    removals.entry(relative.to_owned()).or_default().push(range);
}

fn item_range(item: &Item, source: &str) -> Result<(usize, usize), TransformError> {
    token_range(item.to_token_stream(), item_attrs(item), source)
}

fn trait_item_range(item: &syn::TraitItemFn, source: &str) -> Result<(usize, usize), TransformError> {
    token_range(item.to_token_stream(), &item.attrs, source)
}

fn impl_item_range(item: &syn::ImplItemFn, source: &str) -> Result<(usize, usize), TransformError> {
    token_range(item.to_token_stream(), &item.attrs, source)
}

fn token_range(
    tokens: proc_macro2::TokenStream,
    attributes: &[Attribute],
    source: &str,
) -> Result<(usize, usize), TransformError> {
    let first = tokens.clone().into_iter().next().ok_or_else(|| TransformError::Rust {
        path: PathBuf::from("<source>"),
        message: "empty Rust item".to_owned(),
    })?;
    let last = tokens.into_iter().last().ok_or_else(|| TransformError::Rust {
        path: PathBuf::from("<source>"),
        message: "empty Rust item".to_owned(),
    })?;
    let start_span = attributes.first().map(Spanned::span).unwrap_or_else(|| first.span());
    let start = source_offset(source, start_span.start())?;
    let mut end = source_offset(source, last.span().end())?;
    if source.as_bytes().get(end) == Some(&b'\r') {
        end += 1;
    }
    if source.as_bytes().get(end) == Some(&b'\n') {
        end += 1;
    }
    Ok((start, end))
}

fn apply_removals(source: &str, ranges: &[(usize, usize)], relative: &str) -> Result<String, TransformError> {
    let mut ranges = ranges.to_owned();
    ranges.sort_unstable();
    let mut output = source.to_owned();
    let mut previous_start = usize::MAX;
    for (start, end) in ranges.into_iter().rev() {
        if start > end || end > output.len() {
            return Err(TransformError::Invalid {
                path: PathBuf::from(relative),
                message: "Rust removal is outside source".to_owned(),
            });
        }
        if end > previous_start {
            continue;
        }
        output.replace_range(start..end, "");
        previous_start = start;
    }
    Ok(output)
}

fn item_attrs(item: &Item) -> &[Attribute] {
    match item {
        Item::Const(item) => &item.attrs,
        Item::Enum(item) => &item.attrs,
        Item::ExternCrate(item) => &item.attrs,
        Item::Fn(item) => &item.attrs,
        Item::ForeignMod(item) => &item.attrs,
        Item::Impl(item) => &item.attrs,
        Item::Macro(item) => &item.attrs,
        Item::Mod(item) => &item.attrs,
        Item::Static(item) => &item.attrs,
        Item::Struct(item) => &item.attrs,
        Item::Trait(item) => &item.attrs,
        Item::TraitAlias(item) => &item.attrs,
        Item::Type(item) => &item.attrs,
        Item::Union(item) => &item.attrs,
        Item::Use(item) => &item.attrs,
        Item::Verbatim(_) => &[],
        _ => &[],
    }
}

fn apply_edits(
    source: &str,
    edits: Option<&BTreeMap<usize, BTreeSet<String>>>,
    statement_edits: Option<&Vec<(usize, usize, BTreeSet<String>)>>,
    relative: &str,
) -> Result<String, TransformError> {
    let mut output = source.to_owned();
    let mut insertions = BTreeMap::<usize, String>::new();
    if let Some(edits) = edits {
        for (offset, owners) in edits {
            insertions.entry(*offset).or_default().push_str(&cfg_attribute(owners));
        }
    }
    if let Some(statement_edits) = statement_edits {
        for (start, end, owners) in statement_edits {
            insertions
                .entry(*start)
                .or_default()
                .push_str(&format!("{}{{\n", cfg_attribute(owners)));
            insertions.entry(*end).or_default().push_str("\n}\n");
        }
    }
    for (offset, text) in insertions.iter().rev() {
        if *offset > output.len() {
            return Err(TransformError::Invalid {
                path: PathBuf::from(relative),
                message: "Rust edit is outside source".to_owned(),
            });
        }
        output.insert_str(*offset, &text);
    }
    Ok(output)
}

fn cfg_attribute(owners: &BTreeSet<String>) -> String {
    let features = owners
        .iter()
        .map(|owner| format!("feature = {owner:?}"))
        .collect::<Vec<_>>();
    if features.len() == 1 {
        format!("#[cfg({})]\n", features[0])
    } else {
        format!("#[cfg(any({}))]\n", features.join(", "))
    }
}

fn has_matching_cfg(attributes: &[Attribute], owners: &BTreeSet<String>) -> bool {
    attributes.iter().any(|attribute| {
        if !attribute.path().is_ident("cfg") {
            return false;
        }
        let Meta::List(list) = &attribute.meta else {
            return false;
        };
        let tokens = list.tokens.to_string();
        owners
            .iter()
            .all(|owner| tokens.contains(&format!("feature = {owner:?}")))
    })
}

fn rewrite_documentation(
    crate_root: &Path,
    source_files: &[SourceFile],
    old_library_name: &str,
    new_library_name: &str,
) -> Result<Vec<String>, TransformError> {
    if old_library_name == new_library_name {
        return Ok(Vec::new());
    }

    let mut changed_files = Vec::new();
    for source_file in source_files {
        let mut visitor = DocumentationVisitor {
            source: &source_file.source,
            old_library_name,
            new_library_name,
            edits: Vec::new(),
        };
        visitor.visit_file(&source_file.syntax);
        if visitor.edits.is_empty() {
            continue;
        }
        let updated = apply_text_edits(&source_file.source, &visitor.edits, &source_file.relative)?;
        syn::parse_file(&updated).map_err(|error| TransformError::Rust {
            path: crate_root.join(&source_file.relative),
            message: error.to_string(),
        })?;
        fs::write(crate_root.join(&source_file.relative), updated).map_err(|source| TransformError::Io {
            path: crate_root.join(&source_file.relative),
            source,
        })?;
        changed_files.push(source_file.relative.clone());
    }
    Ok(changed_files)
}

fn replace_identifier(value: &str, old: &str, new: &str) -> String {
    let mut output = String::with_capacity(value.len());
    let mut cursor = 0;
    while let Some(relative_start) = value[cursor..].find(old) {
        let start = cursor + relative_start;
        let end = start + old.len();
        let previous_is_identifier = value[..start].chars().next_back().is_some_and(is_identifier_character);
        let next_is_identifier = value[end..].chars().next().is_some_and(is_identifier_character);
        output.push_str(&value[cursor..start]);
        if previous_is_identifier || next_is_identifier {
            output.push_str(old);
        } else {
            output.push_str(new);
        }
        cursor = end;
    }
    output.push_str(&value[cursor..]);
    output
}

fn is_identifier_character(character: char) -> bool {
    character.is_ascii_alphanumeric() || character == '_'
}

fn apply_text_edits(source: &str, edits: &[(usize, usize, String)], relative: &str) -> Result<String, TransformError> {
    let mut edits = edits.to_owned();
    edits.sort_by_key(|(start, end, _)| (*start, *end));
    let mut output = source.to_owned();
    let mut previous_start = usize::MAX;
    for (start, end, replacement) in edits.into_iter().rev() {
        if start > end || end > output.len() {
            return Err(TransformError::Invalid {
                path: PathBuf::from(relative),
                message: "Rust text edit is outside source".to_owned(),
            });
        }
        if end > previous_start {
            return Err(TransformError::Invalid {
                path: PathBuf::from(relative),
                message: "overlapping Rust text edits".to_owned(),
            });
        }
        output.replace_range(start..end, &replacement);
        previous_start = start;
    }
    Ok(output)
}

fn cargo_library_name(path: &Path) -> Result<String, TransformError> {
    let source = fs::read_to_string(path).map_err(|source| TransformError::Io {
        path: path.to_owned(),
        source,
    })?;
    let document = source.parse::<DocumentMut>().map_err(|error| TransformError::Cargo {
        path: path.to_owned(),
        message: error.to_string(),
    })?;
    if let Some(name) = document
        .get("lib")
        .and_then(TomlItem::as_table)
        .and_then(|table| table.get("name"))
        .and_then(TomlItem::as_str)
    {
        return Ok(name.to_owned());
    }
    document
        .get("package")
        .and_then(TomlItem::as_table)
        .and_then(|table| table.get("name"))
        .and_then(TomlItem::as_str)
        .map(|name| name.replace('-', "_"))
        .ok_or_else(|| TransformError::Cargo {
            path: path.to_owned(),
            message: "manifest has no library or package name".to_owned(),
        })
}

pub fn rewrite_cargo_named(
    path: &Path,
    package_name: &str,
    library_name: &str,
    operations: &[Operation],
) -> Result<(), TransformError> {
    let source = fs::read_to_string(path).map_err(|source| TransformError::Io {
        path: path.to_owned(),
        source,
    })?;
    let mut document = source.parse::<DocumentMut>().map_err(|error| TransformError::Cargo {
        path: path.to_owned(),
        message: error.to_string(),
    })?;
    let package = document
        .get_mut("package")
        .and_then(TomlItem::as_table_mut)
        .ok_or_else(|| TransformError::Cargo {
            path: path.to_owned(),
            message: "manifest has no [package] table".to_owned(),
        })?;
    package["name"] = toml_edit::value(package_name);
    let lib = document
        .entry("lib")
        .or_insert(TomlItem::Table(Table::new()))
        .as_table_mut()
        .ok_or_else(|| TransformError::Cargo {
            path: path.to_owned(),
            message: "[lib] is not a table".to_owned(),
        })?;
    lib["name"] = toml_edit::value(library_name);
    {
        let features = document
            .entry("features")
            .or_insert(TomlItem::Table(Table::new()))
            .as_table_mut()
            .ok_or_else(|| TransformError::Cargo {
                path: path.to_owned(),
                message: "[features] is not a table".to_owned(),
            })?;
        for operation in operations {
            if features.get(&operation.feature).is_none() {
                features.insert(&operation.feature, TomlItem::Value(Value::Array(Array::new())));
            }
        }
        if let Some(default) = features.get("default").and_then(TomlItem::as_array) {
            if operations
                .iter()
                .any(|operation| default.iter().any(|value| value.as_str() == Some(&operation.feature)))
            {
                return Err(TransformError::Cargo {
                    path: path.to_owned(),
                    message: "an operation feature is enabled by default".to_owned(),
                });
            }
        }
    }
    strip_local_dependency_paths(&mut document);
    fs::write(path, document.to_string()).map_err(|source| TransformError::Io {
        path: path.to_owned(),
        source,
    })
}

fn strip_local_dependency_paths(document: &mut DocumentMut) {
    for section in ["dependencies", "dev-dependencies", "build-dependencies"] {
        let Some(table) = document.get_mut(section).and_then(TomlItem::as_table_mut) else {
            continue;
        };
        for (_, item) in table.iter_mut() {
            match item {
                TomlItem::Table(table) => {
                    table.remove("path");
                }
                TomlItem::Value(Value::InlineTable(table)) => {
                    table.remove("path");
                }
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn operations() -> Vec<Operation> {
        vec![Operation {
            shape_id: "example#GetThing".to_owned(),
            name: "GetThing".to_owned(),
            module: "get_thing".to_owned(),
            feature: "op_get_thing".to_owned(),
        }]
    }

    #[test]
    fn rewrites_inline_and_external_modules_with_exact_cfg() {
        let directory = tempdir().unwrap();
        fs::create_dir(directory.path().join("src")).unwrap();
        fs::create_dir(directory.path().join("src/operation")).unwrap();
        fs::create_dir(directory.path().join("src/client")).unwrap();
        fs::write(
            directory.path().join("src/lib.rs"),
            "pub mod operation;\npub mod client;\n",
        )
        .unwrap();
        fs::write(directory.path().join("src/operation.rs"), "pub mod get_thing;\n").unwrap();
        fs::write(directory.path().join("src/client.rs"), "mod get_thing;\n").unwrap();
        fs::write(
            directory.path().join("src/operation/get_thing.rs"),
            "pub struct GetThing;\n",
        )
        .unwrap();
        fs::write(
            directory.path().join("src/client/get_thing.rs"),
            "impl super::Client {}\n",
        )
        .unwrap();
        fs::write(
            directory.path().join("Cargo.toml"),
            "[package]\nname = \"old\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
        )
        .unwrap();
        transform_tree(directory.path(), "new", "new", &operations()).unwrap();
        let operation_source = fs::read_to_string(directory.path().join("src/operation.rs")).unwrap();
        assert!(operation_source.contains("#[cfg(feature = \"op_get_thing\")]\npub mod get_thing;"));
        let client_source = fs::read_to_string(directory.path().join("src/client.rs")).unwrap();
        assert!(client_source.contains("#[cfg(feature = \"op_get_thing\")]\nmod get_thing;"));
    }

    #[test]
    fn rewrites_old_library_names_only_in_documentation() {
        let directory = tempdir().unwrap();
        fs::create_dir(directory.path().join("src")).unwrap();
        fs::create_dir(directory.path().join("src/operation")).unwrap();
        fs::create_dir(directory.path().join("src/client")).unwrap();
        fs::write(
            directory.path().join("src/lib.rs"),
            r#"//! use old_lib::Client;
//! old_library stays unchanged.
//! let _ = "old_lib";
pub fn marker() { let _ = "old_lib"; }
pub mod operation;
pub mod client;
"#,
        )
        .unwrap();
        fs::write(directory.path().join("src/operation.rs"), "pub mod get_thing;\n").unwrap();
        fs::write(directory.path().join("src/client.rs"), "mod get_thing;\n").unwrap();
        fs::write(
            directory.path().join("src/operation/get_thing.rs"),
            "pub struct GetThing;\n",
        )
        .unwrap();
        fs::write(
            directory.path().join("src/client/get_thing.rs"),
            "impl super::Client {}\n",
        )
        .unwrap();
        fs::write(
            directory.path().join("Cargo.toml"),
            "[package]\nname = \"old-package\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[lib]\nname = \"old_lib\"\n",
        )
        .unwrap();

        transform_tree(directory.path(), "new-package", "new_lib", &operations()).unwrap();

        let source = fs::read_to_string(directory.path().join("src/lib.rs")).unwrap();
        assert!(source.contains("//! use new_lib::Client;"));
        assert!(source.contains("//! old_library stays unchanged."));
        assert!(source.contains("//! let _ = \"new_lib\";"));
        assert!(source.contains("pub fn marker() { let _ = \"old_lib\"; }"));
    }

    #[test]
    fn gates_protocol_modules_instead_of_child_items() {
        let directory = tempdir().unwrap();
        fs::create_dir(directory.path().join("src")).unwrap();
        fs::create_dir(directory.path().join("src/operation")).unwrap();
        fs::create_dir(directory.path().join("src/client")).unwrap();
        fs::create_dir(directory.path().join("src/protocol_serde")).unwrap();
        fs::write(
            directory.path().join("src/lib.rs"),
            "pub mod operation;\npub mod client;\npub(crate) mod protocol_serde;\n",
        )
        .unwrap();
        fs::write(directory.path().join("src/operation.rs"), "pub mod get_thing;\n").unwrap();
        fs::write(directory.path().join("src/client.rs"), "mod get_thing;\n").unwrap();
        fs::write(
            directory.path().join("src/operation/get_thing.rs"),
            "pub struct GetThing;\nfn serialize() { crate::protocol_serde::type_erase_result(); crate::protocol_serde::shape_get_thing::serialize(); }\n",
        )
        .unwrap();
        fs::write(
            directory.path().join("src/client/get_thing.rs"),
            "impl super::Client {}\n",
        )
        .unwrap();
        fs::write(
            directory.path().join("src/protocol_serde.rs"),
            "pub(crate) fn type_erase_result() {}\npub(crate) mod shape_get_thing;\n",
        )
        .unwrap();
        fs::write(
            directory.path().join("src/protocol_serde/shape_get_thing.rs"),
            "#[cfg(feature = \"op_get_thing\")]\npub(crate) fn serialize() {}\n",
        )
        .unwrap();
        fs::write(
            directory.path().join("Cargo.toml"),
            "[package]\nname = \"old\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
        )
        .unwrap();

        transform_tree(directory.path(), "new", "new", &operations()).unwrap();

        let protocol_source = fs::read_to_string(directory.path().join("src/protocol_serde.rs")).unwrap();
        assert!(protocol_source.contains("#[cfg(feature = \"op_get_thing\")]\npub(crate) mod shape_get_thing;"));
        assert!(protocol_source.contains("#[cfg(feature = \"op_get_thing\")]\npub(crate) fn type_erase_result() {}"));
        let child_source = fs::read_to_string(directory.path().join("src/protocol_serde/shape_get_thing.rs")).unwrap();
        assert!(!child_source.contains("#[cfg"));
    }

    #[test]
    fn removes_protocol_modules_for_unselected_operations() {
        let directory = tempdir().unwrap();
        fs::create_dir(directory.path().join("src")).unwrap();
        fs::create_dir(directory.path().join("src/operation")).unwrap();
        fs::create_dir(directory.path().join("src/client")).unwrap();
        fs::create_dir(directory.path().join("src/protocol_serde")).unwrap();
        fs::write(
            directory.path().join("src/lib.rs"),
            "pub mod operation;\npub mod client;\npub(crate) mod protocol_serde;\n",
        )
        .unwrap();
        fs::write(
            directory.path().join("src/operation.rs"),
            "pub mod get_thing;\npub mod old_thing;\n",
        )
        .unwrap();
        fs::write(
            directory.path().join("src/client.rs"),
            "mod get_thing;\nmod old_thing;\n",
        )
        .unwrap();
        fs::write(
            directory.path().join("src/operation/get_thing.rs"),
            "pub struct GetThing;\n",
        )
        .unwrap();
        fs::write(
            directory.path().join("src/operation/old_thing.rs"),
            "pub struct OldThing;\n",
        )
        .unwrap();
        fs::write(
            directory.path().join("src/client/get_thing.rs"),
            "impl super::Client {}\n",
        )
        .unwrap();
        fs::write(
            directory.path().join("src/client/old_thing.rs"),
            "impl super::Client {}\n",
        )
        .unwrap();
        fs::write(
            directory.path().join("src/protocol_serde.rs"),
            "pub(crate) mod shape_get_thing;\npub(crate) mod shape_old_thing;\n",
        )
        .unwrap();
        fs::write(
            directory.path().join("src/protocol_serde/shape_get_thing.rs"),
            "pub(crate) fn serialize() {}\n",
        )
        .unwrap();
        fs::write(
            directory.path().join("src/protocol_serde/shape_old_thing.rs"),
            "// stale operation serializer\n",
        )
        .unwrap();
        fs::write(
            directory.path().join("Cargo.toml"),
            "[package]\nname = \"old\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
        )
        .unwrap();

        transform_tree(directory.path(), "new", "new", &operations()).unwrap();

        let protocol_source = fs::read_to_string(directory.path().join("src/protocol_serde.rs")).unwrap();
        assert!(protocol_source.contains("shape_get_thing;"));
        assert!(!protocol_source.contains("shape_old_thing;"));
        assert!(!directory.path().join("src/protocol_serde/shape_old_thing.rs").exists());
    }

    #[test]
    fn only_operation_named_client_children_inherit_parent_cfg() {
        let operations = operations();
        assert!(is_parent_gated_child_path("src/client/get_thing.rs", &operations));
        assert!(is_parent_gated_child_path(
            "src/operation/get_thing/builders.rs",
            &operations
        ));
        assert!(is_parent_gated_child_path(
            "src/client/get_thing/internal.rs",
            &operations
        ));
        assert!(!is_parent_gated_child_path("src/client/customize.rs", &operations));
        assert!(!is_parent_gated_child_path(
            "src/client/customize/internal.rs",
            &operations
        ));
    }

    #[test]
    fn statement_ownership_skips_shared_runtime_symbols() {
        let owners = BTreeMap::from([
            (
                "error::sealed_unhandled".to_owned(),
                BTreeSet::from(["op_get_thing".to_owned()]),
            ),
            (
                "s3_express::runtime_plugin".to_owned(),
                BTreeSet::from(["op_get_thing".to_owned()]),
            ),
        ]);
        assert!(operation_symbol_reference_owners(
            &BTreeSet::from(["error::sealed_unhandled".to_owned()]),
            &owners,
            &BTreeSet::from(["error".to_owned()]),
        )
        .is_empty());
        assert_eq!(
            operation_symbol_reference_owners(
                &BTreeSet::from(["s3_express::runtime_plugin".to_owned()]),
                &owners,
                &BTreeSet::from(["s3_express".to_owned()]),
            ),
            BTreeSet::from(["op_get_thing".to_owned()]),
        );
        assert!(operation_symbol_reference_owners(
            &BTreeSet::from(["aws_types::service_config::ServiceConfigKey".to_owned()]),
            &owners,
            &BTreeSet::from(["s3_express".to_owned()]),
        )
        .is_empty());
    }

    #[test]
    fn protocol_shape_exact_match_wins_over_operation_prefix() {
        let operations = vec![
            Operation {
                shape_id: "example#ListJobs".to_owned(),
                name: "ListJobs".to_owned(),
                module: "list_jobs".to_owned(),
                feature: "op_list_jobs".to_owned(),
            },
            Operation {
                shape_id: "example#ListJobsByConsumableResource".to_owned(),
                name: "ListJobsByConsumableResource".to_owned(),
                module: "list_jobs_by_consumable_resource".to_owned(),
                feature: "op_list_jobs_by_consumable_resource".to_owned(),
            },
        ];

        assert_eq!(
            path_owners(
                "src/protocol_serde/shape_list_jobs_by_consumable_resource.rs",
                &operations,
            ),
            BTreeSet::from(["op_list_jobs_by_consumable_resource".to_owned()])
        );
        assert!(path_owners(
            "src/protocol_serde/shape_list_jobs_by_consumable_resource_summary.rs",
            &operations,
        )
        .is_empty());
    }

    #[test]
    fn gates_error_match_arms_for_request_id_impl() {
        let source = "impl crate::s3_request_id::RequestIdExt for Error {\n    fn extended_request_id(&self) -> Option<&str> {\n        match self {\n            Self::AccessDenied(e) => e.extended_request_id(),\n            Self::Unhandled(e) => e.meta.extended_request_id(),\n        }\n    }\n}\n";
        let file = syn::parse_file(source).unwrap();
        let Item::Impl(item_impl) = &file.items[0] else {
            panic!("expected an impl item");
        };
        let owners = BTreeMap::from([("AccessDenied".to_owned(), BTreeSet::from(["op_get_thing".to_owned()]))]);
        let mut edits = BTreeMap::new();

        collect_error_match_arm_edits("src/error_meta.rs", source, item_impl, &owners, &mut edits).unwrap();

        assert_eq!(edits.len(), 1);
        assert_eq!(edits["src/error_meta.rs"].len(), 1);
    }
}
