use crate::manifest::Exclusions;
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

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
}
