use proc_macro2::{LineColumn, Span};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
};

use crate::error::BuildError;

pub(crate) const ORIGINAL_FILE: &str = "original.rs";
const CANONICAL_MODULE: &str = "__aws_sdk_builder_generated";

type Files = BTreeMap<String, String>;

/// Compose the generated module tree into one inline-module Rust artifact.
pub(crate) fn compose(files: &Files) -> Result<String, BuildError> {
    let paths = files.keys().cloned().collect::<BTreeSet<_>>();
    validate_split_plan(&paths).map_err(|message| generated_error(ORIGINAL_FILE, message))?;
    let plans = files
        .iter()
        .map(|(relative, source)| {
            SourcePlan::parse(relative, source).map(|plan| (relative.clone(), plan))
        })
        .collect::<Result<BTreeMap<_, _>, _>>()?;
    let children_by_parent = build_children_by_parent(&plans);
    let body = compose_module(
        "src/lib.rs",
        &plans,
        &children_by_parent,
        &mut BTreeSet::new(),
    )?;
    // `include!` expands at the caller's item position. Keeping the generated
    // crate attributes and crate documentation inside a real module makes the
    // same canonical file valid both at a crate root and in a caller wrapper.
    let source = format!("mod {CANONICAL_MODULE} {{\n{body}}}\npub use {CANONICAL_MODULE}::*;\n");
    syn::parse_file(&source).map_err(|error| BuildError::GeneratedSourceParse {
        path: PathBuf::from(ORIGINAL_FILE),
        message: format!("canonical original.rs does not parse: {error}"),
    })?;
    Ok(source)
}

fn validate_split_plan(files: &BTreeSet<String>) -> Result<(), String> {
    if !files.contains("src/lib.rs") {
        return Err("module plan has no src/lib.rs".to_owned());
    }
    for relative in files {
        let _ = module_segments(relative)?;
    }
    Ok(())
}

fn compose_module(
    relative: &str,
    plans: &BTreeMap<String, SourcePlan>,
    children_by_parent: &BTreeMap<String, Vec<String>>,
    visiting: &mut BTreeSet<String>,
) -> Result<String, BuildError> {
    if !visiting.insert(relative.to_owned()) {
        return Err(generated_error(relative, "module tree contains a cycle"));
    }
    let plan = plans
        .get(relative)
        .ok_or_else(|| generated_error(relative, "module plan is missing a source file"))?;
    let children = children_by_parent
        .get(relative)
        .map(Vec::as_slice)
        .unwrap_or_default();
    let mut replacements = Vec::with_capacity(plan.crate_paths.len() + children.len());
    let mut module_ranges = Vec::with_capacity(children.len());
    for child in children {
        let module = module_name(child)?;
        let bounds = plan.module_bounds(&module, relative)?;
        let child_source = compose_module(child, plans, children_by_parent, visiting)?;
        let declaration_source = plan.rewrite_paths_in(bounds.item_start..bounds.item_end);
        let declaration = declaration_source.strip_suffix(';').ok_or_else(|| {
            generated_error(relative, format!("module `{module}` has no semicolon"))
        })?;
        let replacement = format!("{declaration} {{\n{child_source}}}");
        replacements.push(Replacement {
            start: bounds.item_start,
            end: bounds.item_end,
            text: replacement,
        });
        module_ranges.push((bounds.item_start, bounds.item_end));
    }

    replacements.extend(
        plan.crate_paths
            .iter()
            .filter(|path| {
                !module_ranges
                    .iter()
                    .any(|(start, end)| *start <= path.start && path.end <= *end)
            })
            .map(|path| Replacement {
                start: path.start,
                end: path.end,
                text: path.replacement.clone(),
            }),
    );
    let source = apply_replacements(&plan.source, replacements, relative)?;
    visiting.remove(relative);
    Ok(normalize_source(&source))
}

fn build_children_by_parent(plans: &BTreeMap<String, SourcePlan>) -> BTreeMap<String, Vec<String>> {
    let mut children_by_parent = BTreeMap::<String, Vec<String>>::new();
    for relative in plans.keys().filter(|relative| *relative != "src/lib.rs") {
        let path = Path::new(relative);
        let Some(parent) = path.parent() else {
            continue;
        };
        let parent = if parent == Path::new("src") {
            PathBuf::from("src/lib.rs")
        } else {
            parent.with_extension("rs")
        };
        children_by_parent
            .entry(parent.to_string_lossy().into_owned())
            .or_default()
            .push(relative.clone());
    }
    children_by_parent
}

struct SourcePlan {
    source: String,
    modules: BTreeMap<String, ModuleDeclaration>,
    crate_paths: Vec<CratePath>,
}

impl SourcePlan {
    fn parse(relative: &str, source: &str) -> Result<Self, BuildError> {
        let file = syn::parse_file(source)
            .map_err(|error| generated_error(relative, error.to_string()))?;
        let mut modules = BTreeMap::new();
        for item in &file.items {
            let syn::Item::Mod(item) = item else {
                continue;
            };
            let declaration = ModuleDeclaration::from_item(item, source, relative)?;
            let name = item.ident.to_string();
            if modules.insert(name.clone(), declaration).is_some() {
                modules.insert(name, ModuleDeclaration::Duplicate);
            }
        }

        let depth = module_depth(relative);
        let base_depth = if relative == "src/lib.rs" { 0 } else { depth };
        let mut visitor = CratePathVisitor {
            base_depth,
            ..Default::default()
        };
        syn::visit::visit_file(&mut visitor, &file);
        let mut crate_paths = BTreeMap::new();
        for (span, replacement) in visitor.spans {
            let start = source_offset_build(source, span.start(), relative)?;
            let end = start.checked_add("crate".len()).ok_or_else(|| {
                generated_error(relative, "crate path span exceeds source length")
            })?;
            if source.get(start..end) != Some("crate") {
                return Err(generated_error(
                    relative,
                    format!("parsed crate path does not start at byte {start}"),
                ));
            }
            crate_paths.insert(
                start,
                CratePath {
                    start,
                    end,
                    replacement,
                },
            );
        }

        Ok(Self {
            source: source.to_owned(),
            modules,
            crate_paths: crate_paths.into_values().collect(),
        })
    }

    fn module_bounds(&self, module: &str, relative: &str) -> Result<&ModuleBounds, BuildError> {
        match self.modules.get(module) {
            Some(ModuleDeclaration::External(bounds)) => Ok(bounds),
            Some(ModuleDeclaration::Inline) => Err(generated_error(
                relative,
                format!("module `{module}` is already inline"),
            )),
            Some(ModuleDeclaration::Duplicate) | None => Err(generated_error(
                relative,
                format!("no external declaration for module `{module}`"),
            )),
        }
    }

    fn rewrite_paths_in(&self, range: std::ops::Range<usize>) -> String {
        let local = self
            .crate_paths
            .iter()
            .filter_map(|path| {
                if range.start <= path.start && path.end <= range.end {
                    Some(Replacement {
                        start: path.start - range.start,
                        end: path.end - range.start,
                        text: path.replacement.clone(),
                    })
                } else {
                    None
                }
            })
            .collect();
        apply_replacements_unchecked(&self.source[range], local)
    }
}

enum ModuleDeclaration {
    External(ModuleBounds),
    Inline,
    Duplicate,
}

impl ModuleDeclaration {
    fn from_item(item: &syn::ItemMod, source: &str, relative: &str) -> Result<Self, BuildError> {
        if item.content.is_some() {
            return Ok(Self::Inline);
        }
        let ident_start = item.ident.span().start();
        let item_start_span = item
            .attrs
            .first()
            .filter(|attribute| attribute.pound_token.span.start() <= ident_start)
            .map(|attribute| attribute.pound_token.span)
            .unwrap_or_else(|| item.ident.span());
        let item_end_span = item
            .semi
            .as_ref()
            .map(|semi| semi.span)
            .unwrap_or_else(|| item.ident.span());
        let item_start = source_offset_build(source, item_start_span.start(), relative)?;
        let item_end = source_offset_build(source, item_end_span.end(), relative)?;
        Ok(Self::External(ModuleBounds {
            item_start,
            item_end,
        }))
    }
}

struct ModuleBounds {
    item_start: usize,
    item_end: usize,
}

struct CratePath {
    start: usize,
    end: usize,
    replacement: String,
}

struct Replacement {
    start: usize,
    end: usize,
    text: String,
}

fn apply_replacements(
    source: &str,
    mut replacements: Vec<Replacement>,
    relative: &str,
) -> Result<String, BuildError> {
    replacements.sort_by_key(|replacement| std::cmp::Reverse(replacement.start));
    for pair in replacements.windows(2) {
        if pair[0].start < pair[1].end {
            return Err(generated_error(relative, "source replacements overlap"));
        }
    }
    let mut rewritten = source.to_owned();
    for replacement in replacements {
        rewritten.replace_range(replacement.start..replacement.end, &replacement.text);
    }
    Ok(rewritten)
}

fn apply_replacements_unchecked(source: &str, mut replacements: Vec<Replacement>) -> String {
    replacements.sort_by_key(|replacement| std::cmp::Reverse(replacement.start));
    let mut rewritten = source.to_owned();
    for replacement in replacements {
        rewritten.replace_range(replacement.start..replacement.end, &replacement.text);
    }
    rewritten
}

#[derive(Default)]
struct CratePathVisitor {
    base_depth: usize,
    module_depth: usize,
    spans: Vec<(Span, String)>,
}

impl<'ast> syn::visit::Visit<'ast> for CratePathVisitor {
    fn visit_item_use(&mut self, item: &'ast syn::ItemUse) {
        if use_tree_starts_at_crate(&item.tree) {
            use syn::spanned::Spanned;
            self.spans
                .push((item.tree.span(), self.replacement().to_owned()));
        }
    }

    fn visit_path(&mut self, path: &'ast syn::Path) {
        if path.segments.len() > 1
            && path
                .segments
                .first()
                .is_some_and(|segment| segment.ident == "crate")
        {
            use syn::spanned::Spanned;
            self.spans
                .push((path.span(), self.replacement().to_owned()));
        }
        syn::visit::visit_path(self, path);
    }

    fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {
        for attribute in &item.attrs {
            self.visit_attribute(attribute);
        }
        self.visit_visibility(&item.vis);
        if let Some((_, items)) = &item.content {
            self.module_depth += 1;
            for nested in items {
                self.visit_item(nested);
            }
            self.module_depth -= 1;
        }
    }

    fn visit_macro(&mut self, mac: &'ast syn::Macro) {
        if mac.path.segments.len() > 1
            && mac
                .path
                .segments
                .first()
                .is_some_and(|segment| segment.ident == "crate")
        {
            use syn::spanned::Spanned;
            self.spans
                .push((mac.path.span(), self.replacement().to_owned()));
        }
        collect_macro_crate_paths(mac.tokens.clone(), self.replacement(), &mut self.spans);
    }
}

impl CratePathVisitor {
    fn replacement(&self) -> String {
        let depth = self.base_depth + self.module_depth;
        if depth == 0 {
            "self".to_owned()
        } else {
            std::iter::repeat_n("super", depth)
                .collect::<Vec<_>>()
                .join("::")
        }
    }
}

fn collect_macro_crate_paths(
    tokens: proc_macro2::TokenStream,
    replacement: String,
    spans: &mut Vec<(Span, String)>,
) {
    let tokens = tokens.into_iter().collect::<Vec<_>>();
    for index in 0..tokens.len().saturating_sub(2) {
        let proc_macro2::TokenTree::Ident(ident) = &tokens[index] else {
            continue;
        };
        if ident != "crate"
            || (index > 0
                && matches!(
                    &tokens[index - 1],
                    proc_macro2::TokenTree::Punct(punct) if punct.as_char() == '$'
                ))
            || !matches!(
                (&tokens[index + 1], &tokens[index + 2]),
                (
                    proc_macro2::TokenTree::Punct(first),
                    proc_macro2::TokenTree::Punct(second)
                ) if first.as_char() == ':' && second.as_char() == ':'
            )
        {
            continue;
        }
        spans.push((ident.span(), replacement.clone()));
    }
    for token in tokens {
        if let proc_macro2::TokenTree::Group(group) = token {
            collect_macro_crate_paths(group.stream(), replacement.clone(), spans);
        }
    }
}

fn use_tree_starts_at_crate(tree: &syn::UseTree) -> bool {
    match tree {
        syn::UseTree::Path(path) => path.ident == "crate",
        syn::UseTree::Name(name) => name.ident == "crate",
        syn::UseTree::Rename(rename) => rename.ident == "crate",
        syn::UseTree::Glob(_) | syn::UseTree::Group(_) => false,
    }
}

fn module_segments(relative: &str) -> Result<Vec<&str>, String> {
    let path = Path::new(relative);
    let components = path
        .components()
        .filter_map(|component| component.as_os_str().to_str())
        .collect::<Vec<_>>();
    if components.first() != Some(&"src") || components.len() < 2 {
        return Err(format!("module path is outside src: {relative}"));
    }
    let filename = components
        .last()
        .and_then(|name| name.strip_suffix(".rs"))
        .ok_or_else(|| format!("module path is not Rust source: {relative}"))?;
    if filename == "mod" {
        return Err(format!("mod.rs paths are not supported: {relative}"));
    }
    let mut segments = components[1..components.len() - 1].to_vec();
    if filename != "lib" {
        segments.push(filename);
    }
    if segments
        .iter()
        .any(|segment| syn::parse_str::<syn::Ident>(segment).is_err())
    {
        return Err(format!(
            "module path contains an invalid Rust identifier: {relative}"
        ));
    }
    Ok(segments)
}

fn module_name(relative: &str) -> Result<String, BuildError> {
    let name = Path::new(relative)
        .file_stem()
        .and_then(|name| name.to_str())
        .ok_or_else(|| generated_error(relative, "module path has no valid file stem"))?;
    if name == "lib" || syn::parse_str::<syn::Ident>(name).is_err() {
        return Err(generated_error(
            relative,
            format!("invalid module name `{name}`"),
        ));
    }
    Ok(name.to_owned())
}

fn module_depth(relative: &str) -> usize {
    Path::new(relative)
        .components()
        .count()
        .saturating_sub(1)
        .max(1)
}

fn normalize_source(source: &str) -> String {
    format!("{}\n", source.trim_end_matches('\n'))
}

fn generated_error(path: impl AsRef<Path>, message: impl Into<String>) -> BuildError {
    BuildError::GeneratedSourceParse {
        path: path.as_ref().to_owned(),
        message: message.into(),
    }
}

fn source_offset_build(
    source: &str,
    location: LineColumn,
    relative: impl AsRef<Path>,
) -> Result<usize, BuildError> {
    let relative = relative.as_ref();
    source_offset(source, location, &relative.display().to_string())
        .map_err(|message| generated_error(relative, message))
}

fn source_offset(source: &str, location: LineColumn, relative: &str) -> Result<usize, String> {
    let line = location
        .line
        .checked_sub(1)
        .ok_or_else(|| format!("invalid zero line in parsed source location: {location:?}"))?;
    let line_start = source
        .split_inclusive('\n')
        .take(line)
        .map(str::len)
        .sum::<usize>();
    let line_end = source[line_start..]
        .find('\n')
        .map_or(source.len(), |offset| line_start + offset);
    let line_source = &source[line_start..line_end];
    let Some(column) = line_source
        .char_indices()
        .nth(location.column)
        .map(|(offset, _)| offset)
        .or_else(|| (location.column == line_source.chars().count()).then_some(line_source.len()))
    else {
        return Err(format!(
            "invalid parsed source location in {relative}: {location:?}"
        ));
    };
    let offset = line_start + column;
    if offset > source.len() || !source.is_char_boundary(offset) {
        Err(format!(
            "invalid parsed source location in {relative}: {location:?}"
        ))
    } else {
        Ok(offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composes_declared_files() {
        let files = Files::from([
            (
                "src/lib.rs".to_owned(),
                "#![allow(dead_code)]\nmod outer;\n".to_owned(),
            ),
            (
                "src/outer.rs".to_owned(),
                "/// Outer: café.\npub mod inner;\npub use crate::outer::Value;\n\nmod private {\n    pub const X: u8 = 1;\n}\n".to_owned(),
            ),
            (
                "src/outer/inner.rs".to_owned(),
                "pub use crate::outer::Value;\n".to_owned(),
            ),
        ]);
        let original = compose(&files).unwrap();
        assert!(original.contains("pub mod inner {"));
        assert!(original.contains("pub use super::outer::Value;"));
        syn::parse_file(&original).unwrap();
    }

    #[test]
    fn preserves_attributes_attached_to_materialized_modules() {
        let files = Files::from([
            (
                "src/lib.rs".to_owned(),
                "#[cfg(feature = \"x\")]\npub mod outer;\n".to_owned(),
            ),
            (
                "src/outer.rs".to_owned(),
                "#![allow(dead_code)]\npub struct Value;\n".to_owned(),
            ),
        ]);
        let original = compose(&files).unwrap();
        assert!(original.contains("#[cfg(feature = \"x\")]\npub mod outer {"));
        assert!(original.contains("#![allow(dead_code)]\npub struct Value;"));
    }

    #[test]
    fn composes_multiple_children_bottom_up_in_source_order() {
        let files = Files::from([
            (
                "src/lib.rs".to_owned(),
                "mod zeta;\nmod alpha;\nmod beta;\n".to_owned(),
            ),
            (
                "src/zeta.rs".to_owned(),
                "pub mod deep;\npub const ZETA: u8 = 1;\n".to_owned(),
            ),
            (
                "src/zeta/deep.rs".to_owned(),
                "pub const DEEP: u8 = 2;\npub use crate::zeta::ZETA;\n".to_owned(),
            ),
            (
                "src/alpha.rs".to_owned(),
                "pub const ALPHA: u8 = 3;\n".to_owned(),
            ),
            (
                "src/beta.rs".to_owned(),
                "pub const BETA: u8 = 4;\n".to_owned(),
            ),
        ]);

        let original = compose(&files).unwrap();
        let alpha = original.find("mod alpha {").unwrap();
        let beta = original.find("mod beta {").unwrap();
        let zeta = original.find("mod zeta {").unwrap();
        assert!(zeta < alpha && alpha < beta);
        assert!(original.contains("pub mod deep {\npub const DEEP: u8 = 2;"));
        assert!(original.contains("pub use super::super::zeta::ZETA;"));
        syn::parse_file(&original).unwrap();
    }

    #[test]
    fn rewrites_crate_paths_after_unicode_without_reparsing() {
        let files = Files::from([
            (
                "src/lib.rs".to_owned(),
                "const LABEL: &str = \"café\"; pub use crate::outer::Value;\nmod outer;\n"
                    .to_owned(),
            ),
            ("src/outer.rs".to_owned(), "pub struct Value;\n".to_owned()),
        ]);

        let original = compose(&files).unwrap();
        assert!(original.contains("pub use self::outer::Value;"));
        syn::parse_file(&original).unwrap();
    }

    #[test]
    fn planning_parse_reports_invalid_physical_source() {
        let files = Files::from([
            ("src/lib.rs".to_owned(), "mod child;\n".to_owned()),
            ("src/child.rs".to_owned(), "pub fn broken( {\n".to_owned()),
        ]);

        let error = compose(&files).unwrap_err();
        match error {
            BuildError::GeneratedSourceParse { path, .. } => {
                assert_eq!(path, PathBuf::from("src/child.rs"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }
}
