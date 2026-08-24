use std::{fs, path::Path};

use crate::{CompileReport, error::BuildError};

pub(crate) fn install(
    stage: &Path,
    out_dir: &Path,
    operations: Vec<String>,
) -> Result<CompileReport, BuildError> {
    install_generated(stage, out_dir, operations, None)
}

pub(crate) fn install_service(
    stage: &Path,
    out_dir: &Path,
    operations: Vec<String>,
    state: &str,
) -> Result<CompileReport, BuildError> {
    install_generated(stage, out_dir, operations, Some(state))
}

fn install_generated(
    stage: &Path,
    out_dir: &Path,
    operations: Vec<String>,
    state: Option<&str>,
) -> Result<CompileReport, BuildError> {
    let generated = stage.join("generated");
    fs::create_dir_all(out_dir).map_err(|source| BuildError::Install {
        path: out_dir.to_owned(),
        source,
    })?;
    let final_root = out_dir.join("generated");
    let final_state = out_dir.join(".aws-sdk-builder-state.json");
    let install_root = out_dir.join(format!(".aws-sdk-builder-install-{}", std::process::id()));
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
    write_consumer_loaders(&install_root.join("generated"))?;
    if let Some(state) = state {
        fs::write(install_root.join(".aws-sdk-builder-state.json"), state).map_err(|source| {
            BuildError::OutputWrite {
                path: install_root.join(".aws-sdk-builder-state.json"),
                source,
            }
        })?;
    }

    let backup = out_dir.join(format!(".aws-sdk-builder-backup-{}", std::process::id()));
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
    let obsolete = [
        out_dir.join("aws_sdk.rs"),
        out_dir.join("aws_sdk_builder_manifest.json"),
    ];
    for (index, path) in obsolete.iter().enumerate() {
        if path.exists()
            && let Err(source) = fs::rename(path, backup.join(format!("obsolete-{index}")))
        {
            restore_obsolete(&backup, &obsolete);
            let _ = fs::remove_dir_all(&install_root);
            let _ = fs::remove_dir_all(&backup);
            return Err(BuildError::Install {
                path: path.clone(),
                source,
            });
        }
    }
    let mut finals = vec![final_root.clone()];
    if state.is_some() {
        finals.push(final_state.clone());
    }
    for (index, path) in finals.iter().enumerate() {
        if path.exists()
            && let Err(source) = fs::rename(path, backup.join(index.to_string()))
        {
            restore(&backup, &finals);
            restore_obsolete(&backup, &obsolete);
            let _ = fs::remove_dir_all(&install_root);
            let _ = fs::remove_dir_all(&backup);
            return Err(BuildError::Install {
                path: (*path).to_owned(),
                source,
            });
        }
    }
    let mut staged = vec![install_root.join("generated")];
    if state.is_some() {
        staged.push(install_root.join(".aws-sdk-builder-state.json"));
    }
    for (index, (source, destination)) in staged.iter().zip(finals.iter()).enumerate() {
        if let Err(source_error) = fs::rename(source, destination) {
            for installed in finals.iter().take(index) {
                remove_path(installed);
            }
            restore(&backup, &finals);
            restore_obsolete(&backup, &obsolete);
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

    Ok(CompileReport {
        generated_root: final_root,
        operations,
    })
}

fn write_consumer_loaders(generated: &Path) -> Result<(), BuildError> {
    let entries = fs::read_dir(generated).map_err(|source| BuildError::Install {
        path: generated.to_owned(),
        source,
    })?;
    for entry in entries {
        let entry = entry.map_err(|source| BuildError::Install {
            path: generated.to_owned(),
            source,
        })?;
        let service = entry.path();
        if !entry
            .file_type()
            .map_err(|source| BuildError::Install {
                path: service.clone(),
                source,
            })?
            .is_dir()
        {
            continue;
        }
        let Some(service_name) = service.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        if !service.join(crate::ORIGINAL_FILE).is_file() {
            continue;
        }
        let module = format!("__aws_sdk_builder_{service_name}_generated");
        let loader = format!("#[path = \"original.rs\"]\nmod {module};\npub use {module}::*;\n");
        fs::write(service.join("consumer.rs"), loader).map_err(|source| {
            BuildError::OutputWrite {
                path: service.join("consumer.rs"),
                source,
            }
        })?;
    }
    Ok(())
}

pub(crate) fn install_snapshot(stage: &Path, output_dir: &Path) -> Result<(), BuildError> {
    let staged_generated = stage.join("generated");
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

pub(crate) fn copy_tree(source: &Path, destination: &Path) -> Result<(), BuildError> {
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

fn restore(backup: &Path, finals: &[std::path::PathBuf]) {
    for (index, destination) in finals.iter().enumerate() {
        let saved = backup.join(index.to_string());
        if saved.exists() {
            let _ = fs::rename(saved, destination);
        }
    }
}

fn restore_obsolete(backup: &Path, paths: &[std::path::PathBuf]) {
    for (index, path) in paths.iter().enumerate() {
        let saved = backup.join(format!("obsolete-{index}"));
        if saved.exists() {
            let _ = fs::rename(saved, path);
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
        let generated = output.path().join("generated");
        fs::create_dir_all(&generated).unwrap();
        fs::write(&include, "old include\n").unwrap();
        fs::write(generated.join("old.rs"), "old source\n").unwrap();

        assert!(install(stage.path(), output.path(), Vec::new()).is_err());
        assert_eq!(fs::read(include).unwrap(), b"old include\n");
        assert_eq!(fs::read(generated.join("old.rs")).unwrap(), b"old source\n");
    }

    #[test]
    fn install_removes_legacy_manifest_and_returns_rust_metadata() {
        let stage = tempdir().unwrap();
        let output = tempdir().unwrap();
        let generated = stage.path().join("generated");
        fs::create_dir_all(&generated).unwrap();
        fs::write(generated.join("lib.rs"), "pub struct Client;\n").unwrap();
        fs::write(
            output.path().join("aws_sdk_builder_manifest.json"),
            "legacy manifest\n",
        )
        .unwrap();

        let report = install(
            stage.path(),
            output.path(),
            vec!["s3::ListBuckets".to_owned()],
        )
        .unwrap();

        assert!(!output.path().join("aws_sdk_builder_manifest.json").exists());
        assert!(!output.path().join("aws_sdk.rs").exists());
        assert_eq!(report.operations, ["s3::ListBuckets"]);
        assert!(report.generated_root.join("lib.rs").is_file());
    }

    #[test]
    fn failed_service_install_preserves_all_existing_outputs() {
        let stage = tempdir().unwrap();
        let output = tempdir().unwrap();
        let generated = output.path().join("generated");
        fs::create_dir_all(&generated).unwrap();
        fs::write(output.path().join("aws_sdk.rs"), "old facade\n").unwrap();
        fs::write(generated.join("old.rs"), "old source\n").unwrap();
        fs::write(
            output.path().join(".aws-sdk-builder-state.json"),
            "old state\n",
        )
        .unwrap();

        let result = install_service(stage.path(), output.path(), Vec::new(), "new state\n");

        assert!(result.is_err());
        assert_eq!(
            fs::read(output.path().join("aws_sdk.rs")).unwrap(),
            b"old facade\n"
        );
        assert_eq!(fs::read(generated.join("old.rs")).unwrap(), b"old source\n");
        assert_eq!(
            fs::read(output.path().join(".aws-sdk-builder-state.json")).unwrap(),
            b"old state\n"
        );
    }

    #[test]
    fn stale_facade_is_removed_after_a_successful_install() {
        let stage = tempdir().unwrap();
        let output = tempdir().unwrap();
        fs::create_dir_all(stage.path().join("generated")).unwrap();
        fs::write(
            stage.path().join("generated/lib.rs"),
            "pub struct Client;\n",
        )
        .unwrap();
        fs::write(output.path().join("aws_sdk.rs"), "old facade\n").unwrap();

        install(stage.path(), output.path(), Vec::new()).unwrap();

        assert!(!output.path().join("aws_sdk.rs").exists());
    }
}
