use std::{collections::BTreeMap, fs, path::Path};

use crate::{CompileReport, error::BuildError};

pub(crate) fn install(
    stage: &Path,
    out_dir: &Path,
    consumer_crate_name: String,
    operations: Vec<String>,
) -> Result<CompileReport, BuildError> {
    let generated = stage.join("generated");

    fs::create_dir_all(out_dir).map_err(|source| BuildError::Install {
        path: out_dir.to_owned(),
        source,
    })?;
    let final_root = out_dir.join("generated");
    let final_include = out_dir.join("aws_sdk.rs");
    let legacy_manifest = out_dir.join("aws_sdk_builder_manifest.json");
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
    copy_file(&stage.join("aws_sdk.rs"), &install_root.join("aws_sdk.rs"))?;

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
    let finals: [&Path; 2] = [&final_root, &final_include];
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
    if legacy_manifest.exists()
        && let Err(source) = fs::rename(&legacy_manifest, backup.join("legacy_manifest"))
    {
        restore(&backup, &finals);
        let _ = fs::remove_dir_all(&install_root);
        let _ = fs::remove_dir_all(&backup);
        return Err(BuildError::Install {
            path: legacy_manifest,
            source,
        });
    }
    let staged = [
        install_root.join("generated"),
        install_root.join("aws_sdk.rs"),
    ];
    for (index, (source, destination)) in staged.iter().zip(finals.iter()).enumerate() {
        if let Err(source_error) = fs::rename(source, destination) {
            for installed in finals.iter().take(index) {
                remove_path(installed);
            }
            restore(&backup, &finals);
            restore_legacy_manifest(&backup, &legacy_manifest);
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
        consumer_crate_name,
        operations,
    })
}

pub(crate) fn install_service(
    stage: &Path,
    out_dir: &Path,
    consumer_crate_name: String,
    operations: Vec<String>,
    state: &str,
) -> Result<CompileReport, BuildError> {
    let generated = stage.join("generated");
    fs::create_dir_all(out_dir).map_err(|source| BuildError::Install {
        path: out_dir.to_owned(),
        source,
    })?;
    let final_root = out_dir.join("generated");
    let final_include = out_dir.join("aws_sdk.rs");
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
    copy_file(&stage.join("aws_sdk.rs"), &install_root.join("aws_sdk.rs"))?;
    fs::write(install_root.join(".aws-sdk-builder-state.json"), state).map_err(|source| {
        BuildError::OutputWrite {
            path: install_root.join(".aws-sdk-builder-state.json"),
            source,
        }
    })?;

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
    let finals: [&Path; 3] = [&final_root, &final_include, &final_state];
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
        install_root.join(".aws-sdk-builder-state.json"),
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

    Ok(CompileReport {
        generated_root: final_root,
        consumer_crate_name,
        operations,
    })
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

pub(crate) fn merge_facade(existing: Option<&str>, current: &str) -> String {
    let mut blocks = BTreeMap::new();
    if let Some(existing) = existing {
        blocks.extend(facade_blocks(existing));
    }
    blocks.extend(facade_blocks(current));
    let header_end = facade_blocks_start(current).unwrap_or(current.len());
    let mut output = current[..header_end].to_owned();
    for (_, block) in blocks {
        output.push_str(&block);
    }
    output
}

fn facade_blocks(source: &str) -> BTreeMap<String, String> {
    let mut blocks = BTreeMap::new();
    let mut offset = 0;
    while let Some(relative) = source[offset..].find("pub mod ") {
        let start_line = source[..offset + relative]
            .rfind("\n")
            .map_or(0, |index| index + 1);
        let module_start = offset + relative + "pub mod ".len();
        let Some(open_relative) = source[module_start..].find(" {") else {
            break;
        };
        let module = &source[module_start..module_start + open_relative];
        if module == "meta" {
            offset = module_start + open_relative + 2;
            continue;
        }
        let open = module_start + open_relative + 1;
        let mut depth = 0_i32;
        let mut end = None;
        for (index, character) in source[open..].char_indices() {
            match character {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        let end_index = open + index + 1;
                        end = Some(
                            source[end_index..]
                                .find('\n')
                                .map_or(end_index, |next| end_index + next + 1),
                        );
                        break;
                    }
                }
                _ => {}
            }
        }
        let Some(end) = end else { break };
        blocks.insert(module.to_owned(), source[start_line..end].to_owned());
        offset = end;
    }
    blocks
}

fn facade_blocks_start(source: &str) -> Option<usize> {
    source.find("#[allow(")
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

fn restore_legacy_manifest(backup: &Path, path: &Path) {
    let saved = backup.join("legacy_manifest");
    if saved.exists() {
        let _ = fs::rename(saved, path);
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

        assert!(
            install(
                stage.path(),
                output.path(),
                "consumer".to_owned(),
                Vec::new()
            )
            .is_err()
        );
        assert_eq!(fs::read(include).unwrap(), b"old include\n");
        assert_eq!(fs::read(generated.join("old.rs")).unwrap(), b"old source\n");
    }

    #[test]
    fn install_removes_legacy_manifest_and_returns_rust_metadata() {
        let stage = tempdir().unwrap();
        let output = tempdir().unwrap();
        let generated = stage.path().join("generated");
        fs::create_dir_all(&generated).unwrap();
        fs::write(stage.path().join("aws_sdk.rs"), "pub mod s3 {}\n").unwrap();
        fs::write(generated.join("lib.rs"), "pub struct Client;\n").unwrap();
        fs::write(
            output.path().join("aws_sdk_builder_manifest.json"),
            "legacy manifest\n",
        )
        .unwrap();

        let report = install(
            stage.path(),
            output.path(),
            "consumer".to_owned(),
            vec!["s3::ListBuckets".to_owned()],
        )
        .unwrap();

        assert!(!output.path().join("aws_sdk_builder_manifest.json").exists());
        assert_eq!(report.consumer_crate_name, "consumer");
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

        let result = install_service(
            stage.path(),
            output.path(),
            "consumer".to_owned(),
            Vec::new(),
            "new state\n",
        );

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
    fn aggregate_facade_merges_each_service_once() {
        let existing = "#[allow(dead_code)]\npub mod aws_sdk_s3 {\n    pub struct Client;\n}\n";
        let current = "#[allow(dead_code)]\npub mod aws_sdk_sqs {\n    pub struct Client;\n}\n";
        let merged = merge_facade(Some(existing), current);
        assert_eq!(merged.matches("pub mod aws_sdk_s3").count(), 1);
        assert_eq!(merged.matches("pub mod aws_sdk_sqs").count(), 1);
        assert!(merged.find("aws_sdk_s3").unwrap() < merged.find("aws_sdk_sqs").unwrap());
    }
}
