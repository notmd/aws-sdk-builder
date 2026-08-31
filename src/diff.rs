use std::{
    collections::BTreeMap,
    fs,
    path::{Component, Path, PathBuf},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DiffError {
    #[error("cannot read {path}: {source}")]
    Read {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("cannot walk {path}: {source}")]
    Walk {
        path: PathBuf,
        source: std::io::Error,
    },
}

pub fn snapshot(root: &Path) -> Result<BTreeMap<String, Vec<u8>>, DiffError> {
    let mut files = BTreeMap::new();
    walk(root, root, &mut files)?;
    Ok(files)
}

fn walk(
    root: &Path,
    current: &Path,
    files: &mut BTreeMap<String, Vec<u8>>,
) -> Result<(), DiffError> {
    let entries = fs::read_dir(current).map_err(|source| DiffError::Walk {
        path: current.to_owned(),
        source,
    })?;
    for entry in entries {
        let entry = entry.map_err(|source| DiffError::Walk {
            path: current.to_owned(),
            source,
        })?;
        let path = entry.path();
        let relative = path.strip_prefix(root).unwrap_or(&path);
        if excluded(relative) {
            continue;
        }
        if path.is_dir() {
            walk(root, &path, files)?;
        } else if path.is_file() {
            let name = relative.to_string_lossy().replace('\\', "/");
            let bytes = fs::read(&path).map_err(|source| DiffError::Read {
                path: path.clone(),
                source,
            })?;
            files.insert(name, bytes);
        }
    }
    Ok(())
}

pub fn excluded(relative: &Path) -> bool {
    relative
        .components()
        .any(|component| component == Component::Normal("tests".as_ref()))
        || relative == Path::new("DIFF.MD")
        || relative == Path::new("DIFF.diff")
}

pub fn unified_patch(
    before: &BTreeMap<String, Vec<u8>>,
    after: &BTreeMap<String, Vec<u8>>,
) -> String {
    let mut result = String::new();
    let keys = before
        .keys()
        .chain(after.keys())
        .cloned()
        .collect::<std::collections::BTreeSet<_>>();
    for key in keys {
        let old = before.get(&key).map(Vec::as_slice).unwrap_or_default();
        let new = after.get(&key).map(Vec::as_slice).unwrap_or_default();
        if old == new {
            continue;
        }
        let old_text = String::from_utf8_lossy(old);
        let new_text = String::from_utf8_lossy(new);
        let patch = diffy::create_patch(&old_text, &new_text).to_string();
        if patch.trim().is_empty() {
            continue;
        }
        result.push_str(&format!(
            "diff --git a/{key} b/{key}\n--- a/{key}\n+++ b/{key}\n"
        ));
        let mut lines = patch.lines();
        let _ = lines.next();
        let _ = lines.next();
        for line in lines {
            result.push_str(line);
            result.push('\n');
        }
    }
    result
}

pub fn changed_files(
    before: &BTreeMap<String, Vec<u8>>,
    after: &BTreeMap<String, Vec<u8>>,
) -> Vec<String> {
    before
        .keys()
        .chain(after.keys())
        .cloned()
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .filter(|key| before.get(key) != after.get(key))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn tests_are_excluded_from_snapshot_and_patch() {
        let mut before = BTreeMap::new();
        before.insert("tests/example.rs".to_owned(), b"old".to_vec());
        before.insert("src/lib.rs".to_owned(), b"old".to_vec());
        let mut after = before.clone();
        after.insert("tests/example.rs".to_owned(), b"new".to_vec());
        after.insert("src/lib.rs".to_owned(), b"new".to_vec());
        before.remove("tests/example.rs");
        after.remove("tests/example.rs");
        let patch = unified_patch(&before, &after);
        assert!(!patch.contains("tests/"));
        assert!(patch.contains("src/lib.rs"));
    }
}
