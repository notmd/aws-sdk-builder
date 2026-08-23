use crate::{
    manifest::{self, ServicesManifest},
    normalize,
};
use std::{
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
    process::Command,
};
use tempfile::TempDir;

pub fn run(arguments: &[OsString]) -> Result<(), String> {
    let manifest_path = optional_path(arguments, "--manifest")
        .unwrap_or_else(|| PathBuf::from(manifest::DEFAULT_PATH));
    let dry_run = arguments.iter().any(|argument| argument == "--dry-run");
    let manifest = ServicesManifest::load(&manifest_path)?;
    manifest::validate_registered_services(&manifest)?;

    let repository_root = manifest_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .canonicalize()
        .map_err(|error| format!("{}: {error}", manifest_path.display()))?;
    let workspace = TempDir::new_in(&repository_root)
        .map_err(|error| format!("{}: {error}", repository_root.display()))?;
    let upstream = prepare_upstream(&manifest, workspace.path())?;

    let staged_reference = workspace.path().join("reference");
    let staged_models = workspace.path().join("models");
    fs::create_dir_all(&staged_reference)
        .map_err(|error| format!("{}: {error}", staged_reference.display()))?;
    fs::create_dir_all(&staged_models)
        .map_err(|error| format!("{}: {error}", staged_models.display()))?;

    let mut model_targets = Vec::with_capacity(manifest.services.len());
    for service in &manifest.services {
        let source = upstream.join(&service.upstream_path);
        reject_symlink(&source)?;
        let source = source
            .canonicalize()
            .map_err(|error| format!("{}: {error}", source.display()))?;
        if !source.starts_with(&upstream) {
            return Err(format!(
                "service path escapes upstream: {}",
                service.upstream_path
            ));
        }
        if !source.join("src").is_dir() {
            return Err(format!(
                "service source has no src directory: {}",
                source.display()
            ));
        }
        let destination = staged_reference.join(&service.reference_path);
        let count =
            normalize::copy_filtered_tree(&source, &destination, &manifest.comparison.exclude)?;
        if normalize::count_files(&destination, &manifest.comparison.exclude)? != count {
            return Err(format!(
                "file count changed while staging service {}",
                service.key
            ));
        }

        let model_source = upstream.join(&service.model_path);
        reject_symlink(&model_source)?;
        let model_source = model_source
            .canonicalize()
            .map_err(|error| format!("{}: {error}", model_source.display()))?;
        if !model_source.starts_with(&upstream) || !model_source.is_file() {
            return Err(format!(
                "model source is not a file inside upstream: {}",
                service.model_path
            ));
        }
        let staged_model = staged_models.join(&service.model_destination);
        if let Some(parent) = staged_model.parent() {
            fs::create_dir_all(parent).map_err(|error| format!("{}: {error}", parent.display()))?;
        }
        fs::copy(&model_source, &staged_model).map_err(|error| {
            format!(
                "{} -> {}: {error}",
                model_source.display(),
                staged_model.display()
            )
        })?;
        model_targets.push((
            staged_model,
            repository_root.join(&service.model_destination),
        ));
        println!(
            "{}: staged {count} reference files and model.json",
            service.key
        );
    }

    if dry_run {
        println!(
            "dry run: upstream {} at {}",
            manifest.upstream.repository, manifest.upstream.commit
        );
        return Ok(());
    }

    let reference_target = repository_root.join(&manifest.roots.reference);
    let mut installs = vec![(staged_reference, reference_target)];
    installs.extend(model_targets);
    atomic_install(&installs, workspace.path())?;
    println!("updated conformance reference and service models");
    Ok(())
}

fn prepare_upstream(manifest: &ServicesManifest, workspace: &Path) -> Result<PathBuf, String> {
    let archive_url = github_archive_url(&manifest.upstream.repository, &manifest.upstream.commit)
        .ok_or_else(|| {
            format!(
                "upstream repository must be an HTTPS GitHub repository for archive download: {}",
                manifest.upstream.repository
            )
        })?;
    let archive = workspace.join("upstream.zip");
    let unpacked = workspace.join("unpacked");
    fs::create_dir_all(&unpacked).map_err(|error| format!("{}: {error}", unpacked.display()))?;
    run_process(
        "curl",
        &[
            "-L".to_owned(),
            "--fail".to_owned(),
            "--silent".to_owned(),
            "--show-error".to_owned(),
            archive_url,
            "-o".to_owned(),
            archive.display().to_string(),
        ],
        None,
    )?;
    run_process(
        "unzip",
        &[
            "-q".to_owned(),
            archive.display().to_string(),
            "-d".to_owned(),
            unpacked.display().to_string(),
        ],
        None,
    )?;
    let root = single_directory(&unpacked)?;
    let root_name = root
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| format!("invalid upstream archive root: {}", root.display()))?;
    if !root_name.ends_with(&format!("-{}", manifest.upstream.commit)) {
        return Err(format!(
            "upstream archive root {root_name:?} does not contain pinned commit {}",
            manifest.upstream.commit
        ));
    }
    Ok(root)
}

fn github_archive_url(repository: &str, commit: &str) -> Option<String> {
    let repository = repository.strip_prefix("https://github.com/")?;
    let repository = repository
        .trim_end_matches('/')
        .strip_suffix(".git")
        .unwrap_or(repository.trim_end_matches('/'));
    if repository.split('/').count() != 2 {
        return None;
    }
    Some(format!(
        "https://github.com/{repository}/archive/{commit}.zip"
    ))
}

fn single_directory(root: &Path) -> Result<PathBuf, String> {
    let mut directories = Vec::new();
    for entry in fs::read_dir(root).map_err(|error| format!("{}: {error}", root.display()))? {
        let entry = entry.map_err(|error| format!("{}: {error}", root.display()))?;
        let file_type = entry
            .file_type()
            .map_err(|error| format!("{}: {error}", entry.path().display()))?;
        if !file_type.is_dir() {
            return Err(format!(
                "upstream archive contains unexpected file: {}",
                entry.path().display()
            ));
        }
        directories.push(entry.path());
    }
    match directories.as_slice() {
        [directory] => Ok(directory.clone()),
        _ => Err(format!(
            "upstream archive must contain one root directory: {}",
            root.display()
        )),
    }
}

fn optional_path(arguments: &[OsString], flag: &str) -> Option<PathBuf> {
    arguments
        .windows(2)
        .find(|pair| pair[0] == flag)
        .map(|pair| PathBuf::from(&pair[1]))
}

fn run_process(
    program: &str,
    arguments: &[String],
    current_dir: Option<&Path>,
) -> Result<String, String> {
    let mut command = Command::new(program);
    command.args(arguments);
    if let Some(current_dir) = current_dir {
        command.current_dir(current_dir);
    }
    let output = command
        .output()
        .map_err(|error| format!("failed to run {program}: {error}"))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let detail = stderr.trim();
        return Err(if detail.is_empty() {
            format!("{program} exited with {}", output.status)
        } else {
            format!("{program} failed: {detail}")
        });
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

fn atomic_install(entries: &[(PathBuf, PathBuf)], workspace: &Path) -> Result<(), String> {
    let backup_root = workspace.join("backups");
    fs::create_dir_all(&backup_root)
        .map_err(|error| format!("{}: {error}", backup_root.display()))?;
    let mut backups = Vec::new();
    let mut installed = Vec::new();

    for (index, (_, target)) in entries.iter().enumerate() {
        if fs::symlink_metadata(target).is_ok() {
            let backup = backup_root.join(index.to_string());
            fs::rename(target, &backup)
                .map_err(|error| format!("cannot stage existing {}: {error}", target.display()))?;
            backups.push((target.clone(), backup));
        }
    }

    for (staged, target) in entries {
        if let Some(parent) = target.parent()
            && let Err(error) = fs::create_dir_all(parent)
        {
            rollback(&installed, &backups);
            return Err(format!("{}: {error}", parent.display()));
        }
        if let Err(error) = fs::rename(staged, target) {
            rollback(&installed, &backups);
            return Err(format!(
                "{} -> {}: {error}",
                staged.display(),
                target.display()
            ));
        }
        installed.push(target.clone());
    }

    for (_, backup) in backups {
        if let Err(error) = remove_path(&backup) {
            return Err(format!("{}: {error}", backup.display()));
        }
    }
    Ok(())
}

fn rollback(installed: &[PathBuf], backups: &[(PathBuf, PathBuf)]) {
    for path in installed.iter().rev() {
        let _ = remove_path(path);
    }
    for (target, backup) in backups.iter().rev() {
        let _ = fs::rename(backup, target);
    }
}

fn reject_symlink(path: &Path) -> Result<(), String> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() => {
            Err(format!("symlinks are not allowed: {}", path.display()))
        }
        Ok(_) => Ok(()),
        Err(error) => Err(format!("{}: {error}", path.display())),
    }
}

fn remove_path(path: &Path) -> std::io::Result<()> {
    let metadata = fs::symlink_metadata(path)?;
    if metadata.is_dir() {
        fs::remove_dir_all(path)
    } else {
        fs::remove_file(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn builds_exact_github_archive_url() {
        assert_eq!(
            github_archive_url(
                "https://github.com/awslabs/aws-sdk-rust",
                "0123456789abcdef0123456789abcdef01234567"
            ),
            Some("https://github.com/awslabs/aws-sdk-rust/archive/0123456789abcdef0123456789abcdef01234567.zip".to_owned())
        );
        assert!(github_archive_url("/tmp/local-repository", "abc").is_none());
    }

    #[test]
    fn accepts_one_unpacked_archive_root() {
        let root = tempdir().unwrap();
        fs::create_dir(root.path().join("aws-sdk-rust-commit")).unwrap();
        assert_eq!(
            single_directory(root.path()).unwrap(),
            root.path().join("aws-sdk-rust-commit")
        );
    }
}
