use proc_macro2::LineColumn;
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

pub const ORIGINAL_FILE: &str = "original.rs";
const CANONICAL_MODULE: &str = "__aws_sdk_builder_generated";

/// Materialize the physical module tree represented by a canonical artifact.
///
/// The transform operates on parsed `syn` item spans. It never searches Rust
/// source with regular expressions or balances braces itself; comments,
/// strings, attributes, docs, and nested items therefore remain token data.
pub fn split(source: &str, files: &BTreeSet<String>) -> Result<BTreeMap<String, String>, String> {
    validate_split_plan(files)?;
    let file = syn::parse_file(source)
        .map_err(|error| format!("canonical original.rs does not parse: {error}"))?;

    let mut output = BTreeMap::new();
    let body = unwrap_canonical_module(source, &file)?;
    let children = child_file_index(files);
    project_module(&body, "src/lib.rs", &children, &mut output)?;
    if output.len() != files.len() {
        return Err(format!(
            "canonical projection materialized {} of {} planned files",
            output.len(),
            files.len()
        ));
    }
    Ok(output)
}

fn unwrap_canonical_module(source: &str, file: &syn::File) -> Result<String, String> {
    let item = top_level_module(file, CANONICAL_MODULE).ok_or_else(|| {
        format!("canonical original.rs has no module declaration for {CANONICAL_MODULE}")
    })?;
    let Some((brace, _)) = &item.content else {
        return Err(format!(
            "canonical module `{CANONICAL_MODULE}` is not inline"
        ));
    };
    let body_start = source_offset(source, brace.span.open().end(), ORIGINAL_FILE)?;
    let body_end = source_offset(source, brace.span.close().start(), ORIGINAL_FILE)?;
    Ok(normalize_source(strip_leading_newline(
        &source[body_start..body_end],
    )))
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

fn project_module(
    source: &str,
    relative: &str,
    children: &BTreeMap<std::path::PathBuf, Vec<String>>,
    output: &mut BTreeMap<String, String>,
) -> Result<(), String> {
    let mut body = normalize_source(source);
    let child_paths = child_files(relative, children);
    if child_paths.is_empty() {
        if output.insert(relative.to_owned(), body).is_some() {
            return Err(format!("duplicate projected module path: {relative}"));
        }
        return Ok(());
    }
    let file = syn::parse_file(&body)
        .map_err(|error| format!("cannot parse module {relative}: {error}"))?;
    let mut child_projections = Vec::new();
    for child in child_paths {
        let module = module_name(&child)?;
        let bounds = inline_module_bounds(&body, &file, &module, relative)?;
        let child_source = normalize_source(strip_leading_newline(
            &body[bounds.open_end..bounds.close_start],
        ));
        child_projections.push((child, bounds, child_source));
    }
    child_projections.sort_by_key(|(_, bounds, _)| std::cmp::Reverse(bounds.item_start));

    for (child, bounds, child_source) in child_projections {
        let declaration = format!("{};", body[bounds.item_start..bounds.open_start].trim_end());
        body.replace_range(bounds.item_start..bounds.item_end, &declaration);
        project_module(&child_source, &child, children, output)?;
    }

    if output.insert(relative.to_owned(), body).is_some() {
        return Err(format!("duplicate projected module path: {relative}"));
    }
    Ok(())
}

fn child_file_index(files: &BTreeSet<String>) -> BTreeMap<std::path::PathBuf, Vec<String>> {
    let mut index = BTreeMap::new();
    for candidate in files {
        if let Some(directory) = Path::new(candidate)
            .parent()
            .map(|parent| parent.to_owned())
        {
            index
                .entry(directory)
                .or_insert_with(Vec::new)
                .push(candidate.clone());
        }
    }
    index
}

fn child_files(
    relative: &str,
    children: &BTreeMap<std::path::PathBuf, Vec<String>>,
) -> Vec<String> {
    let Some(directory) = module_directory(relative) else {
        return Vec::new();
    };
    children
        .get(&directory)
        .into_iter()
        .flatten()
        .filter(|candidate| candidate.as_str() != relative)
        .cloned()
        .collect()
}

fn module_directory(relative: &str) -> Option<std::path::PathBuf> {
    let path = Path::new(relative);
    let filename = path.file_name()?.to_str()?;
    Some(if filename == "lib.rs" {
        path.parent().unwrap_or_else(|| Path::new("")).to_owned()
    } else {
        path.parent()
            .unwrap_or_else(|| Path::new(""))
            .join(filename.strip_suffix(".rs").unwrap_or(filename))
    })
}

struct ModuleBounds {
    item_start: usize,
    item_end: usize,
    open_start: usize,
    open_end: usize,
    close_start: usize,
}

fn inline_module_bounds(
    source: &str,
    file: &syn::File,
    module: &str,
    relative: &str,
) -> Result<ModuleBounds, String> {
    let item = top_level_module(file, module)
        .ok_or_else(|| format!("canonical original.rs has no module declaration for {module}"))?;
    let Some((brace, _)) = &item.content else {
        return Err(format!(
            "canonical module `{module}` is not inline in {relative}"
        ));
    };
    let open_span = brace.span.open();
    let close_span = brace.span.close();
    let ident_start = item.ident.span().start();
    let item_start_span = item
        .attrs
        .first()
        .filter(|attribute| attribute.pound_token.span.start() <= ident_start)
        .map(|attribute| attribute.pound_token.span)
        .unwrap_or_else(|| item.ident.span());
    Ok(ModuleBounds {
        item_start: source_offset(source, item_start_span.start(), relative)?,
        item_end: source_offset(source, close_span.end(), relative)?,
        open_start: source_offset(source, open_span.start(), relative)?,
        open_end: source_offset(source, open_span.end(), relative)?,
        close_start: source_offset(source, close_span.start(), relative)?,
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

fn module_name(relative: &str) -> Result<String, String> {
    let name = Path::new(relative)
        .file_stem()
        .and_then(|name| name.to_str())
        .ok_or_else(|| format!("module path has no valid file stem: {relative}"))?;
    if name == "lib" || syn::parse_str::<syn::Ident>(name).is_err() {
        return Err(format!("invalid module name `{name}` in {relative}"));
    }
    Ok(name.to_owned())
}

fn strip_leading_newline(source: &str) -> &str {
    source.strip_prefix('\n').unwrap_or(source)
}

fn normalize_source(source: &str) -> String {
    format!("{}\n", source.trim_end_matches('\n'))
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

    const ORIGINAL: &str = r#"mod __aws_sdk_builder_generated {
#![allow(dead_code)]
mod outer {
/// Outer: café.
pub mod inner {
pub use super::super::outer::Value;
}
pub use super::outer::Value;

mod private {
    pub const X: u8 = 1;
}
}
}
pub use __aws_sdk_builder_generated::*;
"#;

    #[test]
    fn splits_declared_files_from_the_canonical_artifact() {
        let files = BTreeSet::from([
            "src/lib.rs".to_owned(),
            "src/outer.rs".to_owned(),
            "src/outer/inner.rs".to_owned(),
        ]);
        let split = split(ORIGINAL, &files).unwrap();
        assert_eq!(split["src/lib.rs"], "#![allow(dead_code)]\nmod outer;\n");
        assert_eq!(
            split["src/outer.rs"],
            "/// Outer: café.\npub mod inner;\npub use super::outer::Value;\n\nmod private {\n    pub const X: u8 = 1;\n}\n"
        );
        assert_eq!(
            split["src/outer/inner.rs"],
            "pub use super::super::outer::Value;\n"
        );
    }

    #[test]
    fn preserves_attributes_attached_to_materialized_modules() {
        let original = "mod __aws_sdk_builder_generated {\n#[cfg(feature = \"x\")]\npub mod outer {\n#![allow(dead_code)]\npub struct Value;\n}\n}\npub use __aws_sdk_builder_generated::*;\n";
        let files = BTreeSet::from(["src/lib.rs".to_owned(), "src/outer.rs".to_owned()]);
        let split = split(original, &files).unwrap();
        assert_eq!(
            split["src/lib.rs"],
            "#[cfg(feature = \"x\")]\npub mod outer;\n"
        );
        assert_eq!(
            split["src/outer.rs"],
            "#![allow(dead_code)]\npub struct Value;\n"
        );
    }
}
