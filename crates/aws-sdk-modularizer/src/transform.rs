use crate::model::Operation;
use proc_macro2::LineColumn;
use quote::ToTokens;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};
use syn::{
    Attribute, File, Item, Meta,
    visit::{self, Visit},
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
        self.total == self.transformed + self.missing + self.ambiguous
            && self.missing == 0
            && self.ambiguous == 0
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
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
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

impl<'ast> Visit<'ast> for SuperModuleReferenceVisitor {
    fn visit_path(&mut self, path: &'ast syn::Path) {
        let mut segments = path.segments.iter();
        if segments
            .next()
            .is_some_and(|segment| segment.ident == "super")
        {
            if let Some(segment) = segments.next() {
                self.names.insert(segment.ident.to_string());
            }
        }
        visit::visit_path(self, path);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct MethodKey {
    self_type: String,
    trait_type: Option<String>,
    name: String,
}

type MethodOwnership = BTreeMap<MethodKey, BTreeSet<String>>;

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
                    if let Some(operation) = self
                        .operations
                        .iter()
                        .find(|operation| &operation.module == module)
                    {
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
            if let Some(operation) = self
                .operations
                .iter()
                .find(|operation| operation.module == method)
            {
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

    let waiter_owners = waiter_owners(&source_files, operations);
    let symbol_owners = operation_symbols(&source_files, operations, &waiter_owners);
    let mut edits = BTreeMap::<String, BTreeMap<usize, BTreeSet<String>>>::new();
    for source_file in &source_files {
        let mut direct_owners = path_owners(&source_file.relative, operations);
        if is_protocol_serde_path(&source_file.relative) {
            if let Some(module_owners) =
                symbol_owners.get(&source_module_path(&source_file.relative))
            {
                direct_owners.extend(module_owners.iter().cloned());
            }
        }
        let method_owners = collect_method_owners(
            &source_file.syntax.items,
            operations,
            &waiter_owners,
            &symbol_owners,
        );
        for item in &source_file.syntax.items {
            let mut owners = direct_owners.clone();
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
            }
            if let Item::Impl(item_impl) = item {
                let header_owners =
                    impl_header_owners(item_impl, operations, &waiter_owners, &symbol_owners);
                let method_impl_owners = impl_method_owners(item_impl, &method_owners);
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
                    && !method_impl_owners.is_empty()
                {
                    if !has_matching_cfg(item_attrs(item), &method_impl_owners) {
                        add_edit(
                            &mut edits,
                            &source_file.relative,
                            item_start(item, &source_file.source)?,
                            method_impl_owners,
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
            } else if !owners.is_empty() && !has_matching_cfg(item_attrs(item), &owners) {
                add_edit(
                    &mut edits,
                    &source_file.relative,
                    item_start(item, &source_file.source)?,
                    owners,
                );
            }
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
        }
    }

    let mut changed_files = Vec::new();
    for source_file in &source_files {
        let Some(file_edits) = edits.get(&source_file.relative) else {
            continue;
        };
        let updated = apply_edits(&source_file.source, file_edits, &source_file.relative)?;
        syn::parse_file(&updated).map_err(|error| TransformError::Rust {
            path: crate_root.join(&source_file.relative),
            message: error.to_string(),
        })?;
        fs::write(crate_root.join(&source_file.relative), updated).map_err(|source| {
            TransformError::Io {
                path: crate_root.join(&source_file.relative),
                source,
            }
        })?;
        changed_files.push(source_file.relative.clone());
    }
    rewrite_cargo_named(
        &crate_root.join("Cargo.toml"),
        package_name,
        library_name,
        operations,
    )?;
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
        let operation_count = declarations
            .iter()
            .filter(|name| *name == &operation.module)
            .count();
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

fn collect_source_files(
    source_root: &Path,
    crate_root: &Path,
) -> Result<Vec<SourceFile>, TransformError> {
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

fn module_declarations(
    files: &[SourceFile],
    relative: &str,
    crate_root: &Path,
) -> Result<Vec<String>, TransformError> {
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
) -> BTreeMap<String, BTreeSet<String>> {
    let mut symbols = BTreeMap::new();
    for file in files {
        let module_path = source_module_path(&file.relative);
        if module_path.contains("::") {
            symbols.entry(module_path).or_default();
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
                if let Some(module_owners) =
                    previous.get(&source_module_path(&file.relative))
                {
                    file_owners.extend(module_owners.iter().cloned());
                }
            }
            if !file_owners.is_empty() {
                add_symbol(
                    &mut symbols,
                    &source_module_path(&file.relative),
                    &file_owners,
                );
            }
            collect_operation_symbols(
                &file.syntax.items,
                &source_module_path(&file.relative),
                operations,
                waiter_owners,
                &previous,
                &super_module_references.names,
                &file_owners,
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
            visitor.owners.clone()
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
                        child_visitor.owners.clone()
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
                    symbols,
                );
            }
            _ => {}
        }
    }
}

fn add_symbol(
    symbols: &mut BTreeMap<String, BTreeSet<String>>,
    name: &str,
    owners: &BTreeSet<String>,
) {
    symbols
        .entry(name.to_owned())
        .or_default()
        .extend(owners.iter().cloned());
}

fn is_operation_or_client_symbol(symbol: &str) -> bool {
    matches!(symbol.split("::").next(), Some("operation" | "client"))
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

fn waiter_owners(
    files: &[SourceFile],
    operations: &[Operation],
) -> BTreeMap<String, BTreeSet<String>> {
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
            let operation = operations
                .iter()
                .find(|operation| rest == operation.module)
                .or_else(|| {
                    operations
                        .iter()
                        .filter(|operation| rest.starts_with(&format!("{}_", operation.module)))
                        .max_by_key(|operation| operation.module.len())
                });
            if let Some(operation) = operation {
                owners.insert(operation.feature.clone());
            }
        }
    }
    owners
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
        resolve_external_module(relative, &module.ident.to_string(), files).ok_or_else(|| {
            TransformError::Invalid {
                path: PathBuf::from(relative),
                message: format!("module `{}` has no source file", module.ident),
            }
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
        if child_relative.starts_with("src/waiters/") {
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
            let header_owners =
                impl_header_owners(item_impl, operations, waiter_owners, symbol_owners);
            let method_impl_owners = impl_method_owners(item_impl, method_owners);
            if !header_owners.is_empty() {
                if !has_matching_cfg(item_attrs(item), &header_owners) {
                    add_edit(edits, relative, item_start(item, source)?, header_owners);
                }
            } else if item_impl.trait_.is_some() && !method_impl_owners.is_empty() {
                if !has_matching_cfg(item_attrs(item), &method_impl_owners) {
                    add_edit(
                        edits,
                        relative,
                        item_start(item, source)?,
                        method_impl_owners,
                    );
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
            let first = method.to_token_stream().into_iter().next().ok_or_else(|| {
                TransformError::Rust {
                    path: PathBuf::from(relative),
                    message: "empty Rust impl item".to_owned(),
                }
            })?;
            add_edit(
                edits,
                relative,
                source_offset(source, first.span().start())?,
                owners,
            );
        }
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
    collect_method_info(
        items,
        operations,
        waiter_owners,
        symbol_owners,
        &mut methods,
    );
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
                if let Some((called_owners, _)) = current
                    .get(&inherent_key)
                    .or_else(|| current.get(&trait_key))
                {
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
    methods
        .into_iter()
        .map(|(key, (owners, _))| (key, owners))
        .collect()
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

fn impl_method_owners(
    item_impl: &syn::ItemImpl,
    method_owners: &MethodOwnership,
) -> BTreeSet<String> {
    let context = impl_context(item_impl);
    item_impl
        .items
        .iter()
        .filter_map(|item| match item {
            syn::ImplItem::Fn(method) => {
                method_owners.get(&method_key(&context, &method.sig.ident.to_string()))
            }
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
    let candidates = [
        format!("{virtual_path}.rs"),
        format!("{virtual_path}/mod.rs"),
    ];
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
    edits: &BTreeMap<usize, BTreeSet<String>>,
    relative: &str,
) -> Result<String, TransformError> {
    let mut output = source.to_owned();
    for (offset, owners) in edits.iter().rev() {
        let text = cfg_attribute(owners);
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
    let mut document = source
        .parse::<DocumentMut>()
        .map_err(|error| TransformError::Cargo {
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
                features.insert(
                    &operation.feature,
                    TomlItem::Value(Value::Array(Array::new())),
                );
            }
        }
        if let Some(default) = features.get("default").and_then(TomlItem::as_array) {
            if operations.iter().any(|operation| {
                default
                    .iter()
                    .any(|value| value.as_str() == Some(&operation.feature))
            }) {
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
        fs::write(
            directory.path().join("src/operation.rs"),
            "pub mod get_thing;\n",
        )
        .unwrap();
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
        let operation_source =
            fs::read_to_string(directory.path().join("src/operation.rs")).unwrap();
        assert!(
            operation_source.contains("#[cfg(feature = \"op_get_thing\")]\npub mod get_thing;")
        );
        let client_source = fs::read_to_string(directory.path().join("src/client.rs")).unwrap();
        assert!(client_source.contains("#[cfg(feature = \"op_get_thing\")]\nmod get_thing;"));
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
    }
}
