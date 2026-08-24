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
    validate_file_plan(files)?;
    let body = compose_module("src/lib.rs", files, &mut BTreeSet::new())?;
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

fn validate_file_plan(files: &Files) -> Result<(), BuildError> {
    let paths = files.keys().cloned().collect::<BTreeSet<_>>();
    validate_split_plan(&paths).map_err(|message| BuildError::GeneratedSourceParse {
        path: PathBuf::from(ORIGINAL_FILE),
        message,
    })?;
    for (relative, source) in files {
        syn::parse_file(source).map_err(|error| BuildError::GeneratedSourceParse {
            path: PathBuf::from(relative),
            message: error.to_string(),
        })?;
    }
    Ok(())
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
    files: &Files,
    visiting: &mut BTreeSet<String>,
) -> Result<String, BuildError> {
    if !visiting.insert(relative.to_owned()) {
        return Err(generated_error(relative, "module tree contains a cycle"));
    }
    let mut source = files
        .get(relative)
        .cloned()
        .ok_or_else(|| generated_error(relative, "module plan is missing a source file"))?;

    let depth = module_depth(relative);
    let base_depth = if relative == "src/lib.rs" { 0 } else { depth };
    source = rewrite_crate_paths(&source, base_depth, Path::new(relative))?;

    for child in child_files(relative, &files.keys().cloned().collect())
        .into_iter()
        .rev()
    {
        let module = module_name(&child)?;
        let bounds = external_module_bounds(&source, &module, relative)?;
        let child_source = compose_module(&child, files, visiting)?;
        let declaration = source[bounds.item_start..bounds.item_end]
            .strip_suffix(';')
            .ok_or_else(|| {
                generated_error(relative, format!("module `{module}` has no semicolon"))
            })?;
        let replacement = format!("{declaration} {{\n{child_source}}}");
        source.replace_range(bounds.item_start..bounds.item_end, &replacement);
    }

    visiting.remove(relative);
    Ok(normalize_source(&source))
}

fn child_files(relative: &str, files: &BTreeSet<String>) -> Vec<String> {
    let path = Path::new(relative);
    let Some(filename) = path.file_name().and_then(|name| name.to_str()) else {
        return Vec::new();
    };
    let module_directory = if filename == "lib.rs" {
        path.parent().unwrap_or_else(|| Path::new("")).to_owned()
    } else {
        path.parent()
            .unwrap_or_else(|| Path::new(""))
            .join(filename.strip_suffix(".rs").unwrap_or(filename))
    };
    files
        .iter()
        .filter(|candidate| {
            *candidate != relative
                && Path::new(candidate).parent() == Some(module_directory.as_path())
        })
        .cloned()
        .collect()
}

struct ModuleBounds {
    item_start: usize,
    item_end: usize,
}

fn external_module_bounds(
    source: &str,
    module: &str,
    relative: &str,
) -> Result<ModuleBounds, BuildError> {
    let file =
        syn::parse_file(source).map_err(|error| generated_error(relative, error.to_string()))?;
    let item = top_level_module(&file, module).ok_or_else(|| {
        generated_error(
            relative,
            format!("no external declaration for module `{module}`"),
        )
    })?;
    if item.content.is_some() {
        return Err(generated_error(
            relative,
            format!("module `{module}` is already inline"),
        ));
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
    Ok(ModuleBounds {
        item_start,
        item_end,
    })
}

fn top_level_module<'ast>(file: &'ast syn::File, module: &str) -> Option<&'ast syn::ItemMod> {
    let mut found = None;
    for item in &file.items {
        if let syn::Item::Mod(item) = item
            && item.ident == module
        {
            if found.is_some() {
                return None;
            }
            found = Some(item);
        }
    }
    found
}

fn rewrite_crate_paths(
    source: &str,
    base_depth: usize,
    relative: &Path,
) -> Result<String, BuildError> {
    let file =
        syn::parse_file(source).map_err(|error| generated_error(relative, error.to_string()))?;
    let mut visitor = CratePathVisitor {
        base_depth,
        ..Default::default()
    };
    syn::visit::visit_file(&mut visitor, &file);
    let starts = visitor
        .spans
        .into_iter()
        .map(|(span, replacement)| {
            let start = source_offset_build(source, span.start(), relative)?;
            if source.get(start..start + "crate".len()) != Some("crate") {
                return Err(generated_error(
                    relative,
                    format!("parsed crate path does not start at byte {start}"),
                ));
            }
            Ok((start, replacement))
        })
        .collect::<Result<BTreeMap<_, _>, BuildError>>()?;
    if starts.is_empty() {
        return Ok(source.to_owned());
    }
    let mut rewritten = source.to_owned();
    for (start, replacement) in starts.into_iter().rev() {
        rewritten.replace_range(start..start + "crate".len(), &replacement);
    }
    syn::parse_file(&rewritten).map_err(|error| {
        generated_error(
            relative,
            format!("rewritten canonical module does not parse: {error}"),
        )
    })?;
    Ok(rewritten)
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
    let column = source[line_start..line_end]
        .char_indices()
        .nth(location.column)
        .map_or(line_end - line_start, |(offset, _)| offset);
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
}
