use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{CompileReport, error::BuildError};

pub(crate) fn install(stage: &Path, out_dir: &Path) -> Result<CompileReport, BuildError> {
    let generated = stage.join("generated");
    let manifest = stage.join("aws_sdk_build_manifest.json");
    validate_tree(&generated)?;
    validate_rust_file(&stage.join("aws_sdk.rs"))?;
    let staged_manifest = fs::read(&manifest).map_err(|source| BuildError::SourceRead {
        path: manifest.clone(),
        source,
    })?;
    serde_json::from_slice::<serde_json::Value>(&staged_manifest).map_err(|source| {
        BuildError::InvalidGeneratedRust {
            path: manifest.clone(),
            message: source.to_string(),
        }
    })?;

    fs::create_dir_all(out_dir).map_err(|source| BuildError::Install {
        path: out_dir.to_owned(),
        source,
    })?;
    let final_root = out_dir.join("generated");
    let final_include = out_dir.join("aws_sdk.rs");
    let final_manifest = out_dir.join("aws_sdk_build_manifest.json");
    let install_root = out_dir.join(format!(".aws-sdk-build-install-{}", std::process::id()));
    if install_root.exists() {
        fs::remove_dir_all(&install_root).map_err(|source| BuildError::Install {
            path: install_root.clone(),
            source,
        })?;
    }
    fs::create_dir_all(&install_root).map_err(|source| BuildError::Install {
        path: install_root.clone(),
        source,
    })?;
    copy_tree(&generated, &install_root.join("generated"))?;
    copy_file(&stage.join("aws_sdk.rs"), &install_root.join("aws_sdk.rs"))?;
    copy_file(&manifest, &install_root.join("aws_sdk_build_manifest.json"))?;

    let backup = out_dir.join(format!(".aws-sdk-build-backup-{}", std::process::id()));
    if backup.exists() {
        fs::remove_dir_all(&backup).map_err(|source| BuildError::Install {
            path: backup.clone(),
            source,
        })?;
    }
    fs::create_dir_all(&backup).map_err(|source| BuildError::Install {
        path: backup.clone(),
        source,
    })?;
    let finals: [&Path; 3] = [&final_root, &final_include, &final_manifest];
    for (index, path) in finals.iter().enumerate() {
        if path.exists()
            && let Err(source) = fs::rename(path, backup.join(index.to_string()))
        {
            restore(&backup, &finals);
            let _ = fs::remove_dir_all(&install_root);
            let _ = fs::remove_dir_all(&backup);
            return Err(BuildError::Install {
                path: (*path).to_owned(),
                source,
            });
        }
    }
    let staged = [
        install_root.join("generated"),
        install_root.join("aws_sdk.rs"),
        install_root.join("aws_sdk_build_manifest.json"),
    ];
    for (index, (source, destination)) in staged.iter().zip(finals.iter()).enumerate() {
        if let Err(source_error) = fs::rename(source, destination) {
            for installed in finals.iter().take(index) {
                remove_path(installed);
            }
            restore(&backup, &finals);
            let _ = fs::remove_dir_all(&install_root);
            let _ = fs::remove_dir_all(&backup);
            return Err(BuildError::Install {
                path: (*destination).to_owned(),
                source: source_error,
            });
        }
    }
    let _ = fs::remove_dir_all(&install_root);
    let _ = fs::remove_dir_all(&backup);

    let manifest_value: serde_json::Value = serde_json::from_slice(&staged_manifest)
        .map_err(|source| BuildError::ManifestSerialize { source })?;
    let consumer_crate_name = manifest_value["consumer_crate_name"]
        .as_str()
        .unwrap_or_default()
        .to_owned();
    let operations = manifest_value["selected_operations"]
        .as_array()
        .map(|items| {
            items
                .iter()
                .filter_map(|item| item.as_str().map(ToOwned::to_owned))
                .collect()
        })
        .unwrap_or_default();
    Ok(CompileReport {
        generated_root: final_root,
        manifest: final_manifest,
        consumer_crate_name,
        operations,
    })
}

pub(crate) fn validate_tree(root: &Path) -> Result<(), BuildError> {
    let mut files = Vec::new();
    collect_files(root, &mut files)?;
    for path in files {
        if path.extension().and_then(|extension| extension.to_str()) == Some("rs") {
            validate_rust_file(&path)?;
        }
    }
    Ok(())
}

pub(crate) fn install_snapshot(stage: &Path, output_dir: &Path) -> Result<(), BuildError> {
    let staged_generated = stage.join("generated");
    validate_tree(&staged_generated)?;
    let parent = output_dir.parent().unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent).map_err(|source| BuildError::Install {
        path: parent.to_owned(),
        source,
    })?;

    let backup = parent.join(format!(".aws-sdk-snapshot-backup-{}", std::process::id()));
    if backup.exists() {
        return Err(BuildError::Install {
            path: backup,
            source: std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "snapshot backup path already exists",
            ),
        });
    }

    let had_existing = output_dir.exists();
    if had_existing {
        fs::rename(output_dir, &backup).map_err(|source| BuildError::Install {
            path: output_dir.to_owned(),
            source,
        })?;
    }

    if let Err(source) = fs::rename(&staged_generated, output_dir) {
        if had_existing {
            let _ = fs::rename(&backup, output_dir);
        }
        return Err(BuildError::Install {
            path: output_dir.to_owned(),
            source,
        });
    }

    if had_existing {
        fs::remove_dir_all(&backup).map_err(|source| BuildError::Install {
            path: backup,
            source,
        })?;
    }
    Ok(())
}

fn validate_rust_file(path: &Path) -> Result<(), BuildError> {
    let source = fs::read_to_string(path).map_err(|source| BuildError::SourceRead {
        path: path.to_owned(),
        source,
    })?;
    syn::parse_file(&source).map_err(|error| BuildError::InvalidGeneratedRust {
        path: path.to_owned(),
        message: error.to_string(),
    })?;
    Ok(())
}

fn collect_files(root: &Path, files: &mut Vec<PathBuf>) -> Result<(), BuildError> {
    let entries = fs::read_dir(root).map_err(|source| BuildError::SourceRead {
        path: root.to_owned(),
        source,
    })?;
    for entry in entries {
        let path = entry
            .map_err(|source| BuildError::SourceRead {
                path: root.to_owned(),
                source,
            })?
            .path();
        if path.is_dir() {
            collect_files(&path, files)?;
        } else {
            files.push(path);
        }
    }
    Ok(())
}

fn copy_tree(source: &Path, destination: &Path) -> Result<(), BuildError> {
    fs::create_dir_all(destination).map_err(|source| BuildError::OutputWrite {
        path: destination.to_owned(),
        source,
    })?;
    let entries = fs::read_dir(source).map_err(|source_error| BuildError::SourceRead {
        path: source.to_owned(),
        source: source_error,
    })?;
    for entry in entries {
        let source_path = entry
            .map_err(|source_error| BuildError::SourceRead {
                path: source.to_owned(),
                source: source_error,
            })?
            .path();
        let destination_path =
            destination.join(source_path.file_name().expect("directory entry has a name"));
        if source_path.is_dir() {
            copy_tree(&source_path, &destination_path)?;
        } else {
            copy_file(&source_path, &destination_path)?;
        }
    }
    Ok(())
}

fn copy_file(source: &Path, destination: &Path) -> Result<(), BuildError> {
    fs::copy(source, destination)
        .map(|_| ())
        .map_err(|source_error| BuildError::OutputWrite {
            path: destination.to_owned(),
            source: source_error,
        })
}

fn restore(backup: &Path, finals: &[&Path]) {
    for (index, destination) in finals.iter().enumerate() {
        let saved = backup.join(index.to_string());
        if saved.exists() {
            let _ = fs::rename(saved, destination);
        }
    }
}

fn remove_path(path: &Path) {
    if path.is_dir() {
        let _ = fs::remove_dir_all(path);
    } else if path.exists() {
        let _ = fs::remove_file(path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn invalid_stage_does_not_touch_existing_output() {
        let stage = tempdir().unwrap();
        let output = tempdir().unwrap();
        let include = output.path().join("aws_sdk.rs");
        let manifest = output.path().join("aws_sdk_build_manifest.json");
        let generated = output.path().join("generated");
        fs::create_dir_all(&generated).unwrap();
        fs::write(&include, "old include\n").unwrap();
        fs::write(&manifest, "old manifest\n").unwrap();
        fs::write(generated.join("old.rs"), "old source\n").unwrap();

        assert!(install(stage.path(), output.path()).is_err());
        assert_eq!(fs::read(include).unwrap(), b"old include\n");
        assert_eq!(fs::read(manifest).unwrap(), b"old manifest\n");
        assert_eq!(fs::read(generated.join("old.rs")).unwrap(), b"old source\n");
    }
}
