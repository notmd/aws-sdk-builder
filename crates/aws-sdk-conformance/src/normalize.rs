use crate::manifest::Exclusions;
use proc_macro2::LineColumn;
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

/// Record source-only reference normalizations without changing the checked-in
/// upstream snapshot. The patch is applied in memory by the comparator.
pub fn write_reference_patches(
    root: &Path,
    patches_root: &Path,
    exclusions: &Exclusions,
) -> Result<usize, String> {
    if !root.is_dir() {
        return Err(format!(
            "reference root is not a directory: {}",
            root.display()
        ));
    }
    fs::create_dir_all(patches_root)
        .map_err(|error| format!("{}: {error}", patches_root.display()))?;

    let mut files = BTreeMap::new();
    collect_files(root, root, Path::new(""), exclusions, &mut files)?;
    let mut patches = 0;
    for (relative, source_path) in files {
        if relative
            .extension()
            .and_then(|extension| extension.to_str())
            != Some("rs")
        {
            continue;
        }
        let source = fs::read_to_string(&source_path)
            .map_err(|error| format!("{}: {error}", source_path.display()))?;
        let normalized = normalize_reference_source(&source, &relative)?;
        if normalized == source {
            continue;
        }
        let patch = diffy::create_patch(&source, &normalized).to_string();
        let patch_path = patch_path(patches_root, &relative);
        if let Some(parent) = patch_path.parent() {
            fs::create_dir_all(parent).map_err(|error| format!("{}: {error}", parent.display()))?;
        }
        fs::write(&patch_path, patch)
            .map_err(|error| format!("{}: {error}", patch_path.display()))?;
        patches += 1;
    }
    Ok(patches)
}

pub fn patch_path(patches_root: &Path, relative: &Path) -> PathBuf {
    PathBuf::from(format!("{}.patch", patches_root.join(relative).display()))
}

/// Apply all source-only normalizations used for checked-in reference patches.
/// Parsing is deliberately part of the normalization contract so a malformed
/// upstream Rust file fails the update instead of receiving a textual
/// best-effort rewrite.
pub fn normalize_reference_source(source: &str, relative: &Path) -> Result<String, String> {
    let normalized = normalize_crate_paths(source, relative)?;
    drop_inline_unit_tests(&normalized, relative)
}

/// Rewrite every parsed Rust path rooted at `crate` to the relative `super`
/// path. This transformation is stored in a checked-in patch and applied to
/// the reference at comparison time.
pub fn normalize_crate_paths(source: &str, relative: &Path) -> Result<String, String> {
    let file = syn::parse_file(source).map_err(|error| {
        format!(
            "cannot parse reference Rust source {}: {error}",
            relative.display()
        )
    })?;
    let mut visitor = CratePathVisitor::default();
    syn::visit::visit_file(&mut visitor, &file);
    if visitor.spans.is_empty() {
        return Ok(source.to_owned());
    }

    let mut normalized = source.to_owned();
    let mut ranges = visitor
        .spans
        .into_iter()
        .map(|span| -> Result<(usize, usize), String> {
            let start = source_offset(source, span.start())?;
            let end = source_offset(source, span.end())?;
            Ok((start, end))
        })
        .collect::<Result<Vec<_>, _>>()?;
    ranges.sort_by_key(|(start, _)| std::cmp::Reverse(*start));
    ranges.dedup();
    for (start, _end) in ranges {
        if source.get(start..start + "crate".len()) != Some("crate") {
            return Err(format!(
                "parsed crate path has no crate token in {}",
                relative.display()
            ));
        }
        normalized.replace_range(start..start + "crate".len(), "super");
    }
    syn::parse_file(&normalized).map_err(|error| {
        format!(
            "normalized reference Rust source {} no longer parses: {error}",
            relative.display()
        )
    })?;
    Ok(normalized)
}

/// Remove inline `#[cfg(test)] mod ...` units from a Rust source file.
///
/// Generated SDK snapshots intentionally omit these upstream-only test
/// modules. Their attributes are part of the module span, so attributes such
/// as `#[allow(unreachable_code, unused_variables)]` are removed together
/// with the module.
pub fn drop_inline_unit_tests(source: &str, relative: &Path) -> Result<String, String> {
    if !source.contains("#[cfg") {
        return Ok(source.to_owned());
    }
    let file = syn::parse_file(source).map_err(|error| {
        format!(
            "cannot parse reference Rust source {}: {error}",
            relative.display()
        )
    })?;
    let mut visitor = InlineUnitTestVisitor::default();
    syn::visit::visit_file(&mut visitor, &file);
    if visitor.spans.is_empty() {
        return Ok(source.to_owned());
    }

    let mut normalized = source.to_owned();
    let mut ranges = visitor
        .spans
        .into_iter()
        .map(|span| -> Result<(usize, usize), String> {
            let mut start = source_offset(source, span.start())?;
            let mut end = source_offset(source, span.end())?;
            loop {
                while start > 0 && matches!(source.as_bytes()[start - 1], b' ' | b'\t') {
                    start -= 1;
                }
                if start > 0 && source.as_bytes()[start - 1] == b'\n' {
                    start -= 1;
                }
                let line_start = source[..start].rfind('\n').map_or(0, |index| index + 1);
                let previous_line = source[line_start..start].trim();
                if previous_line.starts_with("#[") {
                    start = line_start;
                } else {
                    break;
                }
            }
            if end < source.len() && source.as_bytes()[end] == b'\n' {
                end += 1;
            }
            Ok((start, end))
        })
        .collect::<Result<Vec<_>, _>>()?;
    ranges.sort_by_key(|(start, _)| std::cmp::Reverse(*start));
    for (start, end) in ranges {
        normalized.replace_range(start..end, "");
    }
    syn::parse_file(&normalized).map_err(|error| {
        format!(
            "normalized reference Rust source {} no longer parses: {error}",
            relative.display()
        )
    })?;
    Ok(normalized)
}

#[derive(Default)]
struct CratePathVisitor {
    spans: Vec<proc_macro2::Span>,
}

impl<'ast> syn::visit::Visit<'ast> for CratePathVisitor {
    fn visit_item_use(&mut self, item: &'ast syn::ItemUse) {
        if use_tree_starts_at_crate(&item.tree) {
            use syn::spanned::Spanned;
            self.spans.push(item.tree.span());
        }
    }

    fn visit_path(&mut self, path: &'ast syn::Path) {
        if path_starts_at_crate(path) {
            use syn::spanned::Spanned;
            self.spans.push(path.span());
        }
        syn::visit::visit_path(self, path);
    }
}

#[derive(Default)]
struct InlineUnitTestVisitor {
    spans: Vec<proc_macro2::Span>,
}

impl<'ast> syn::visit::Visit<'ast> for InlineUnitTestVisitor {
    fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {
        if item.attrs.iter().any(is_cfg_test) {
            use syn::spanned::Spanned;
            self.spans.push(item.span());
            return;
        }
        syn::visit::visit_item_mod(self, item);
    }
}

fn is_cfg_test(attribute: &syn::Attribute) -> bool {
    let syn::Meta::List(meta) = &attribute.meta else {
        return false;
    };
    if !meta.path.is_ident("cfg") {
        return false;
    }
    syn::parse2::<syn::Meta>(meta.tokens.clone())
        .map(|meta| cfg_meta_contains_test(&meta))
        .unwrap_or(false)
}

fn cfg_meta_contains_test(meta: &syn::Meta) -> bool {
    use syn::parse::Parser;

    match meta {
        syn::Meta::Path(path) => path.is_ident("test"),
        syn::Meta::List(list) if list.path.is_ident("all") || list.path.is_ident("any") => {
            let parser = syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated;
            parser
                .parse2(list.tokens.clone())
                .map(|metas| metas.iter().any(cfg_meta_contains_test))
                .unwrap_or(false)
        }
        syn::Meta::List(_) | syn::Meta::NameValue(_) => false,
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

fn path_starts_at_crate(path: &syn::Path) -> bool {
    path.segments.len() > 1
        && path
            .segments
            .first()
            .is_some_and(|segment| segment.ident == "crate")
}

fn source_offset(source: &str, location: LineColumn) -> Result<usize, String> {
    let line = location
        .line
        .checked_sub(1)
        .ok_or_else(|| format!("invalid zero line in parsed source location: {location:?}"))?;
    let line_start = source
        .split_inclusive('\n')
        .take(line)
        .map(str::len)
        .sum::<usize>();
    let offset = line_start + location.column;
    if offset > source.len() || !source.is_char_boundary(offset) {
        Err(format!("invalid parsed source location: {location:?}"))
    } else {
        Ok(offset)
    }
}

pub fn copy_filtered_tree(
    source: &Path,
    destination: &Path,
    exclusions: &Exclusions,
) -> Result<usize, String> {
    if !source.is_dir() {
        return Err(format!(
            "source service is not a directory: {}",
            source.display()
        ));
    }
    fs::create_dir_all(destination)
        .map_err(|error| format!("{}: {error}", destination.display()))?;
    copy_directory(source, destination, Path::new(""), exclusions)
}

pub fn strip_excluded(root: &Path, exclusions: &Exclusions) -> Result<usize, String> {
    if !root.is_dir() {
        return Err(format!(
            "snapshot root is not a directory: {}",
            root.display()
        ));
    }
    strip_directory(root, Path::new(""), exclusions)
}

pub fn count_files(root: &Path, exclusions: &Exclusions) -> Result<usize, String> {
    if !root.is_dir() {
        return Ok(0);
    }
    let mut files = BTreeMap::new();
    collect_files(root, root, Path::new(""), exclusions, &mut files)?;
    Ok(files.len())
}

fn copy_directory(
    source: &Path,
    destination: &Path,
    relative_root: &Path,
    exclusions: &Exclusions,
) -> Result<usize, String> {
    let mut count = 0;
    let entries = fs::read_dir(source).map_err(|error| format!("{}: {error}", source.display()))?;
    for entry in entries {
        let entry = entry.map_err(|error| format!("{}: {error}", source.display()))?;
        let source_path = entry.path();
        let relative = relative_root.join(entry.file_name());
        let file_type = entry
            .file_type()
            .map_err(|error| format!("{}: {error}", source_path.display()))?;
        reject_symlink(&source_path, &file_type)?;
        if exclusions.excludes(&relative) {
            continue;
        }
        let destination_path = destination.join(entry.file_name());
        if file_type.is_dir() {
            fs::create_dir_all(&destination_path)
                .map_err(|error| format!("{}: {error}", destination_path.display()))?;
            count += copy_directory(&source_path, &destination_path, &relative, exclusions)?;
        } else if file_type.is_file() {
            fs::copy(&source_path, &destination_path).map_err(|error| {
                format!(
                    "{} -> {}: {error}",
                    source_path.display(),
                    destination_path.display()
                )
            })?;
            count += 1;
        } else {
            return Err(format!("unsupported file type: {}", source_path.display()));
        }
    }
    Ok(count)
}

fn strip_directory(
    current: &Path,
    relative_root: &Path,
    exclusions: &Exclusions,
) -> Result<usize, String> {
    let mut removed = 0;
    let entries =
        fs::read_dir(current).map_err(|error| format!("{}: {error}", current.display()))?;
    for entry in entries {
        let entry = entry.map_err(|error| format!("{}: {error}", current.display()))?;
        let path = entry.path();
        let relative = relative_root.join(entry.file_name());
        let file_type = entry
            .file_type()
            .map_err(|error| format!("{}: {error}", path.display()))?;
        reject_symlink(&path, &file_type)?;
        if exclusions.excludes(&relative) {
            if file_type.is_dir() {
                fs::remove_dir_all(&path)
                    .map_err(|error| format!("{}: {error}", path.display()))?;
            } else if file_type.is_file() {
                fs::remove_file(&path).map_err(|error| format!("{}: {error}", path.display()))?;
            } else {
                return Err(format!("unsupported file type: {}", path.display()));
            }
            removed += 1;
        } else if file_type.is_dir() {
            removed += strip_directory(&path, &relative, exclusions)?;
        }
    }
    Ok(removed)
}

fn collect_files(
    root: &Path,
    current: &Path,
    relative_root: &Path,
    exclusions: &Exclusions,
    files: &mut BTreeMap<PathBuf, PathBuf>,
) -> Result<(), String> {
    let entries =
        fs::read_dir(current).map_err(|error| format!("{}: {error}", current.display()))?;
    for entry in entries {
        let entry = entry.map_err(|error| format!("{}: {error}", current.display()))?;
        let path = entry.path();
        let relative = relative_root.join(entry.file_name());
        let file_type = entry
            .file_type()
            .map_err(|error| format!("{}: {error}", path.display()))?;
        reject_symlink(&path, &file_type)?;
        if exclusions.excludes(&relative) {
            continue;
        }
        if file_type.is_dir() {
            collect_files(root, &path, &relative, exclusions, files)?;
        } else if file_type.is_file() {
            let root_relative = path
                .strip_prefix(root)
                .map_err(|error| format!("{}: {error}", path.display()))?
                .to_owned();
            files.insert(root_relative, path);
        } else {
            return Err(format!("unsupported file type: {}", path.display()));
        }
    }
    Ok(())
}

fn reject_symlink(path: &Path, file_type: &fs::FileType) -> Result<(), String> {
    if file_type.is_symlink() {
        Err(format!(
            "symlinks are not allowed in conformance data: {}",
            path.display()
        ))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn exclusions() -> Exclusions {
        Exclusions {
            files: vec!["Cargo.toml".to_owned(), "README.md".to_owned()],
            directories: vec!["tests".to_owned(), "benches".to_owned()],
        }
    }

    #[test]
    fn strips_only_configured_snapshot_artifacts() {
        let root = tempdir().unwrap();
        fs::create_dir_all(root.path().join("src")).unwrap();
        fs::create_dir_all(root.path().join("tests/data")).unwrap();
        fs::create_dir_all(root.path().join("benches")).unwrap();
        fs::write(root.path().join("Cargo.toml"), "package").unwrap();
        fs::write(root.path().join("README.md"), "readme").unwrap();
        fs::write(root.path().join("LICENSE"), "license").unwrap();
        fs::write(root.path().join("src/lib.rs"), "source").unwrap();
        fs::write(root.path().join("tests/data/input.json"), "test").unwrap();
        fs::write(root.path().join("benches/bench.rs"), "bench").unwrap();

        strip_excluded(root.path(), &exclusions()).unwrap();

        assert!(!root.path().join("Cargo.toml").exists());
        assert!(!root.path().join("README.md").exists());
        assert!(!root.path().join("tests").exists());
        assert!(!root.path().join("benches").exists());
        assert!(root.path().join("LICENSE").exists());
        assert!(root.path().join("src/lib.rs").exists());
        assert_eq!(count_files(root.path(), &exclusions()).unwrap(), 2);
    }

    #[test]
    fn copies_filtered_tree_without_symlinks() {
        let source = tempdir().unwrap();
        let destination = tempdir().unwrap();
        fs::create_dir_all(source.path().join("src")).unwrap();
        fs::create_dir_all(source.path().join("tests")).unwrap();
        fs::write(source.path().join("src/lib.rs"), "source").unwrap();
        fs::write(source.path().join("tests/test.rs"), "test").unwrap();

        assert_eq!(
            copy_filtered_tree(
                source.path(),
                destination.path().join("service").as_path(),
                &exclusions()
            )
            .unwrap(),
            1
        );
        assert!(destination.path().join("service/src/lib.rs").exists());
        assert!(!destination.path().join("service/tests").exists());
    }

    #[test]
    fn normalizes_all_crate_paths_with_syn() {
        let source = "use crate::types::Thing;\n\nmod nested {\n    use crate::error::Error;\n    fn value() { let _ = crate::runtime::value(); }\n}\n";
        let normalized = normalize_crate_paths(source, Path::new("src/lib.rs")).unwrap();
        assert!(normalized.contains("use super::types::Thing;"));
        assert!(normalized.contains("use super::error::Error;"));
        assert!(normalized.contains("super::runtime::value()"));
        assert!(!normalized.contains("crate::"));
    }

    #[test]
    fn drops_inline_unit_tests_with_their_attributes() {
        let source = "pub struct Client;\n\n#[allow(unreachable_code, unused_variables)]\n#[cfg(all(test, feature = \"gated-tests\"))]\nmod client_test {\n    #[test]\n    fn works() {}\n}\n\npub fn client() {}\n";
        let normalized = drop_inline_unit_tests(source, Path::new("src/lib.rs")).unwrap();
        assert!(normalized.contains("pub struct Client;"));
        assert!(normalized.contains("pub fn client() {}"));
        assert!(!normalized.contains("unreachable_code"));
        assert!(!normalized.contains("cfg(all(test"));
        assert!(!normalized.contains("mod client_test"));
        syn::parse_file(&normalized).unwrap();
    }

    #[test]
    fn writes_a_source_preserving_patch_for_each_normalized_file() {
        let root = tempdir().unwrap();
        let patches = tempdir().unwrap();
        fs::create_dir_all(root.path().join("src")).unwrap();
        fs::write(
            root.path().join("src/lib.rs"),
            "use crate::types::Thing;\nfn value() { crate::runtime::value(); }\n",
        )
        .unwrap();

        assert_eq!(
            write_reference_patches(root.path(), patches.path(), &Exclusions::default()).unwrap(),
            1
        );
        let patch =
            fs::read_to_string(patch_path(patches.path(), Path::new("src/lib.rs"))).unwrap();
        assert!(patch.contains("-use crate::types::Thing;"));
        assert!(patch.contains("+use super::types::Thing;"));
        assert!(patch.contains("-fn value() { crate::runtime::value(); }"));
        assert!(patch.contains("+fn value() { super::runtime::value(); }"));
        assert_eq!(
            fs::read_to_string(root.path().join("src/lib.rs")).unwrap(),
            "use crate::types::Thing;\nfn value() { crate::runtime::value(); }\n"
        );
    }
}
