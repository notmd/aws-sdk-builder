use std::{
    fs,
    path::{Path, PathBuf},
};

use serde_json::json;
use tempfile::Builder;

use crate::{error::BuildError, CompileReport};

pub fn install(
    generated_dir: &Path,
    out_dir: &Path,
    service: &str,
    operations: &[String],
) -> Result<CompileReport, BuildError> {
    fs::create_dir_all(out_dir).map_err(|source| BuildError::OutputWrite {
        path: out_dir.to_path_buf(),
        source,
    })?;
    let projection =
        find_projection(generated_dir)?.ok_or_else(|| BuildError::GeneratedOutputNotFound {
            path: generated_dir.to_path_buf(),
        })?;
    let stage = Builder::new()
        .prefix(".aws-sdk-build-")
        .tempdir_in(out_dir)
        .map_err(|source| BuildError::OutputWrite {
            path: out_dir.to_path_buf(),
            source,
        })?;
    let staged_generated = stage.path().join("generated");
    copy_directory(&projection.join("src"), &staged_generated.join("src"))?;

    let files = rust_files(&staged_generated)?;
    let manifest = json!({
        "service": service,
        "operations": operations,
        "files": files,
    });
    write_file(
        &stage.path().join("aws_sdk.rs"),
        "include!(concat!(env!(\"OUT_DIR\"), \"/generated/src/lib.rs\"));\n",
    )?;
    write_file(
        &stage.path().join("aws_sdk_build_manifest.json"),
        &serde_json::to_string_pretty(&manifest).map_err(|source| BuildError::OutputWrite {
            path: stage.path().join("aws_sdk_build_manifest.json"),
            source: std::io::Error::other(source),
        })?,
    )?;

    let generated_root = out_dir.join("generated");
    let include_root = out_dir.join("aws_sdk.rs");
    let manifest_path = out_dir.join("aws_sdk_build_manifest.json");
    let final_paths = [
        generated_root.clone(),
        include_root.clone(),
        manifest_path.clone(),
    ];
    let backup = Builder::new()
        .prefix(".aws-sdk-build-backup-")
        .tempdir_in(out_dir)
        .map_err(|source| BuildError::OutputWrite {
            path: out_dir.to_path_buf(),
            source,
        })?;
    let mut backed_up = Vec::new();
    for (index, final_path) in final_paths.iter().enumerate() {
        if final_path.exists() {
            let backup_path = backup.path().join(index.to_string());
            if let Err(source) = fs::rename(final_path, &backup_path) {
                for (original, saved) in backed_up.iter().rev() {
                    let _ = fs::rename(saved, original);
                }
                return Err(BuildError::OutputWrite {
                    path: final_path.clone(),
                    source,
                });
            }
            backed_up.push((final_path.clone(), backup_path));
        }
    }

    let staged_paths = [
        stage.path().join("generated"),
        stage.path().join("aws_sdk.rs"),
        stage.path().join("aws_sdk_build_manifest.json"),
    ];
    let mut installed: Vec<PathBuf> = Vec::new();
    for (staged_path, final_path) in staged_paths.iter().zip(final_paths.iter()) {
        if let Err(source) = fs::rename(staged_path, final_path) {
            for installed_path in installed {
                remove_path(&installed_path);
            }
            for (original, saved) in backed_up.iter().rev() {
                let _ = fs::rename(saved, original);
            }
            return Err(BuildError::OutputWrite {
                path: final_path.clone(),
                source,
            });
        }
        installed.push(final_path.clone());
    }

    Ok(CompileReport {
        generated_root,
        manifest: manifest_path,
        operations: operations.to_vec(),
    })
}

fn find_projection(root: &Path) -> Result<Option<PathBuf>, BuildError> {
    if !root.is_dir() {
        return Ok(None);
    }
    if root.join("src/lib.rs").is_file() {
        return Ok(Some(root.to_path_buf()));
    }
    let entries = fs::read_dir(root).map_err(|source| BuildError::OutputRead {
        path: root.to_path_buf(),
        source,
    })?;
    let mut children = entries
        .map(|entry| {
            entry
                .map(|entry| entry.path())
                .map_err(|source| BuildError::OutputRead {
                    path: root.to_path_buf(),
                    source,
                })
        })
        .collect::<Result<Vec<_>, _>>()?;
    children.sort();
    for child in children {
        if child.is_dir() {
            if let Some(projection) = find_projection(&child)? {
                return Ok(Some(projection));
            }
        }
    }
    Ok(None)
}

fn copy_directory(source: &Path, destination: &Path) -> Result<(), BuildError> {
    fs::create_dir_all(destination).map_err(|source_error| BuildError::OutputWrite {
        path: destination.to_path_buf(),
        source: source_error,
    })?;
    let mut entries = fs::read_dir(source)
        .map_err(|source_error| BuildError::OutputRead {
            path: source.to_path_buf(),
            source: source_error,
        })?
        .map(|entry| {
            entry
                .map(|entry| entry.path())
                .map_err(|source_error| BuildError::OutputRead {
                    path: source.to_path_buf(),
                    source: source_error,
                })
        })
        .collect::<Result<Vec<_>, _>>()?;
    entries.sort();
    for entry in entries {
        let target = destination.join(entry.file_name().expect("directory entries have names"));
        if entry.is_dir() {
            copy_directory(&entry, &target)?;
        } else {
            fs::copy(&entry, &target).map_err(|source_error| BuildError::OutputCopy {
                path: entry.clone(),
                destination: target,
                source: source_error,
            })?;
        }
    }
    Ok(())
}

fn rust_files(root: &Path) -> Result<Vec<String>, BuildError> {
    let mut files = Vec::new();
    collect_rust_files(root, root, &mut files)?;
    files.sort();
    Ok(files)
}

fn collect_rust_files(
    root: &Path,
    directory: &Path,
    files: &mut Vec<String>,
) -> Result<(), BuildError> {
    let entries = fs::read_dir(directory).map_err(|source| BuildError::OutputRead {
        path: directory.to_path_buf(),
        source,
    })?;
    for entry in entries {
        let path = entry
            .map_err(|source| BuildError::OutputRead {
                path: directory.to_path_buf(),
                source,
            })?
            .path();
        if path.is_dir() {
            collect_rust_files(root, &path, files)?;
        } else if path.extension().and_then(|extension| extension.to_str()) == Some("rs") {
            let relative = path
                .strip_prefix(root)
                .expect("collected file is below the root")
                .to_string_lossy()
                .replace(std::path::MAIN_SEPARATOR, "/");
            files.push(relative);
        }
    }
    Ok(())
}

fn write_file(path: &Path, contents: &str) -> Result<(), BuildError> {
    fs::write(path, contents).map_err(|source| BuildError::OutputWrite {
        path: path.to_path_buf(),
        source,
    })
}

fn remove_path(path: &Path) {
    if path.is_dir() {
        let _ = fs::remove_dir_all(path);
    } else {
        let _ = fs::remove_file(path);
    }
}
