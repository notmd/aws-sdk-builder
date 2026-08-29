use crate::{
    diff,
    manifest::{self, Manifest, ServiceManifest},
    model::{Model, Operation},
    transform::{self, Coverage, TransformError},
};
use std::{
    collections::{BTreeMap, BTreeSet},
    ffi::OsString,
    fmt::Write,
    fs,
    path::{Component, Path, PathBuf},
    process::Command,
};
use tempfile::TempDir;
use thiserror::Error;
use toml_edit::{Array, DocumentMut, Item as TomlItem, Table, Value};

#[derive(Debug, Error)]
pub enum ConformanceError {
    #[error("{0}")]
    Message(String),
    #[error(transparent)]
    Manifest(#[from] manifest::ManifestError),
    #[error(transparent)]
    Model(#[from] crate::model::ModelError),
    #[error(transparent)]
    Transform(#[from] TransformError),
    #[error(transparent)]
    Diff(#[from] diff::DiffError),
    #[error("{path}: {source}")]
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
}

pub fn run_cli(arguments: Vec<OsString>) -> Result<(), ConformanceError> {
    let conformance = arguments
        .first()
        .is_some_and(|argument| argument == "conformance");
    let manifest_path = argument_path(&arguments, "--manifest")
        .unwrap_or_else(|| PathBuf::from(manifest::DEFAULT_PATH));
    let manifest = Manifest::load(&manifest_path)?;
    let root = Manifest::repository_root(&manifest_path)
        .canonicalize()
        .map_err(|source| ConformanceError::Io {
            path: manifest_path.clone(),
            source,
        })?;
    if conformance {
        run_conformance(&manifest, &root)
    } else {
        run_generation(&manifest, &root)
    }
}

fn run_generation(manifest: &Manifest, root: &Path) -> Result<(), ConformanceError> {
    let workspace = TempDir::new().map_err(|source| ConformanceError::Io {
        path: root.to_owned(),
        source,
    })?;
    let upstream = prepare_upstreams(manifest, workspace.path())?;
    for service in &manifest.services {
        let staged = stage_service(service, root, &upstream, workspace.path())?;
        let model = Model::load(&root.join(&service.model_path))?;
        let operations = model.operations()?;
        write_stage_artifacts(service, &staged, &upstream, &operations)?;
        install_atomically(&staged, &root.join(&service.output_dir), workspace.path())?;
        println!(
            "{}: generated {} operations at {}",
            service.key,
            operations.len(),
            service.output_dir
        );
    }
    Ok(())
}

fn run_conformance(manifest: &Manifest, root: &Path) -> Result<(), ConformanceError> {
    let workspace = TempDir::new().map_err(|source| ConformanceError::Io {
        path: root.to_owned(),
        source,
    })?;
    let upstream = prepare_upstreams(manifest, workspace.path())?;
    let previous = fs::read_to_string(root.join("conformance/summary.md")).ok();
    let mut reports = Vec::new();
    for service in &manifest.services {
        let staged = stage_service(service, root, &upstream, workspace.path())?;
        let model = Model::load(&root.join(&service.model_path))?;
        let operations = model.operations()?;
        verify_service(service, &staged, &operations)?;
        let changed_files = write_stage_artifacts(service, &staged, &upstream, &operations)?;
        verify_diff_artifacts(&staged)?;
        check_feature_matrix(service, &staged, &model, &operations, workspace.path())?;
        check_public_api(service, &staged, &operations, workspace.path())?;
        reports.push((
            service.key.clone(),
            Coverage {
                total: operations.len(),
                transformed: operations.len(),
                missing: 0,
                ambiguous: 0,
            },
            changed_files,
        ));
    }
    let summary = render_summary(&reports, previous.as_deref())?;
    let summary_path = root.join("conformance/summary.md");
    if let Some(parent) = summary_path.parent() {
        fs::create_dir_all(parent).map_err(|source| ConformanceError::Io {
            path: parent.to_owned(),
            source,
        })?;
    }
    fs::write(&summary_path, summary).map_err(|source| ConformanceError::Io {
        path: summary_path,
        source,
    })?;
    println!("conformance: {} services passed", reports.len());
    Ok(())
}

fn write_stage_artifacts(
    service: &ServiceManifest,
    staged: &Path,
    upstreams: &BTreeMap<(String, String), PathBuf>,
    operations: &[Operation],
) -> Result<Vec<String>, ConformanceError> {
    let upstream = upstreams
        .get(&(service.repository.clone(), service.revision.clone()))
        .expect("prepared upstream");
    let before = diff::snapshot(&upstream.join(&service.upstream_path))?;
    let after = diff::snapshot(staged)?;
    let changed_files = diff::changed_files(&before, &after);
    let patch = diff::unified_patch(&before, &after);
    write_diff_artifacts(service, staged, operations, &changed_files, &patch)?;
    Ok(changed_files)
}

fn stage_service(
    service: &ServiceManifest,
    root: &Path,
    upstreams: &BTreeMap<(String, String), PathBuf>,
    workspace: &Path,
) -> Result<PathBuf, ConformanceError> {
    let upstream = upstreams
        .get(&(service.repository.clone(), service.revision.clone()))
        .ok_or_else(|| {
            ConformanceError::Message(format!("no downloaded source for {}", service.key))
        })?;
    let source = safe_join(upstream, &service.upstream_path)?;
    if !source.join("src").is_dir() {
        return Err(ConformanceError::Message(format!(
            "{} has no src directory",
            source.display()
        )));
    }
    let stage = workspace.join("staged").join(&service.key);
    copy_without_tests(&source, &stage)?;
    let model = Model::load(&root.join(&service.model_path))?;
    let operations = model.operations()?;
    transform::transform_tree(
        &stage,
        &service.package_name,
        &service.library_name,
        &operations,
    )?;
    Ok(stage)
}

fn prepare_upstreams(
    manifest: &Manifest,
    workspace: &Path,
) -> Result<BTreeMap<(String, String), PathBuf>, ConformanceError> {
    let mut result = BTreeMap::new();
    let mut keys = BTreeSet::new();
    for service in &manifest.services {
        keys.insert((service.repository.clone(), service.revision.clone()));
    }
    for (repository, revision) in keys {
        let archive = workspace.join(format!("{}.tar.gz", revision));
        let unpacked = workspace.join(format!("unpacked-{revision}"));
        fs::create_dir_all(&unpacked).map_err(|source| ConformanceError::Io {
            path: unpacked.clone(),
            source,
        })?;
        if let Some(local_archive) = std::env::var_os("AWS_SDK_MODULARIZER_ARCHIVE") {
            fs::copy(&local_archive, &archive).map_err(|source| ConformanceError::Io {
                path: archive.clone(),
                source,
            })?;
        } else {
            run_process(
                "curl",
                &[
                    "-L",
                    "--fail",
                    "--silent",
                    "--show-error",
                    &archive_url(&repository, &revision)?,
                    "-o",
                    archive.to_str().ok_or_else(|| {
                        ConformanceError::Message("archive path is not UTF-8".to_owned())
                    })?,
                ],
                workspace,
            )?;
        }
        run_process(
            "tar",
            &[
                "-xzf",
                archive.to_str().ok_or_else(|| {
                    ConformanceError::Message("archive path is not UTF-8".to_owned())
                })?,
                "-C",
                unpacked.to_str().ok_or_else(|| {
                    ConformanceError::Message("unpack path is not UTF-8".to_owned())
                })?,
            ],
            workspace,
        )?;
        let root = single_directory(&unpacked)?;
        result.insert((repository, revision), root);
    }
    Ok(result)
}

fn archive_url(repository: &str, revision: &str) -> Result<String, ConformanceError> {
    let value = repository
        .strip_prefix("https://github.com/")
        .unwrap_or_default()
        .trim_end_matches('/')
        .trim_end_matches(".git");
    if value.split('/').count() != 2 {
        return Err(ConformanceError::Message(format!(
            "unsupported GitHub repository URL: {repository}"
        )));
    }
    Ok(format!(
        "https://github.com/{value}/archive/{revision}.tar.gz"
    ))
}

fn single_directory(path: &Path) -> Result<PathBuf, ConformanceError> {
    let mut directories = Vec::new();
    for entry in fs::read_dir(path).map_err(|source| ConformanceError::Io {
        path: path.to_owned(),
        source,
    })? {
        let entry = entry.map_err(|source| ConformanceError::Io {
            path: path.to_owned(),
            source,
        })?;
        if entry
            .file_type()
            .map_err(|source| ConformanceError::Io {
                path: entry.path(),
                source,
            })?
            .is_dir()
        {
            directories.push(entry.path());
        }
    }
    match directories.as_slice() {
        [root] => Ok(root.clone()),
        _ => Err(ConformanceError::Message(format!(
            "archive must contain exactly one root directory: {}",
            path.display()
        ))),
    }
}

fn copy_without_tests(source: &Path, destination: &Path) -> Result<(), ConformanceError> {
    if destination.exists() {
        return Err(ConformanceError::Message(format!(
            "staging directory already exists: {}",
            destination.display()
        )));
    }
    fs::create_dir_all(destination).map_err(|source_error| ConformanceError::Io {
        path: destination.to_owned(),
        source: source_error,
    })?;
    copy_directory_contents(source, destination)
}

fn copy_directory_contents(source: &Path, destination: &Path) -> Result<(), ConformanceError> {
    for entry in fs::read_dir(source).map_err(|source_error| ConformanceError::Io {
        path: source.to_owned(),
        source: source_error,
    })? {
        let entry = entry.map_err(|source_error| ConformanceError::Io {
            path: source.to_owned(),
            source: source_error,
        })?;
        let source_path = entry.path();
        let name = entry.file_name();
        if name == "tests" {
            continue;
        }
        let destination_path = destination.join(name);
        let file_type = entry
            .file_type()
            .map_err(|source_error| ConformanceError::Io {
                path: source_path.clone(),
                source: source_error,
            })?;
        if file_type.is_symlink() {
            return Err(ConformanceError::Message(format!(
                "symlink is not allowed in upstream crate: {}",
                source_path.display()
            )));
        }
        if file_type.is_dir() {
            fs::create_dir_all(&destination_path).map_err(|source_error| ConformanceError::Io {
                path: destination_path.clone(),
                source: source_error,
            })?;
            copy_directory_contents(&source_path, &destination_path)?;
        } else if file_type.is_file() {
            fs::copy(&source_path, &destination_path).map_err(|source_error| {
                ConformanceError::Io {
                    path: destination_path,
                    source: source_error,
                }
            })?;
        }
    }
    Ok(())
}

fn safe_join(root: &Path, relative: &str) -> Result<PathBuf, ConformanceError> {
    let path = Path::new(relative);
    if path.is_absolute()
        || path.components().any(|component| {
            matches!(
                component,
                Component::ParentDir | Component::CurDir | Component::RootDir
            )
        })
    {
        return Err(ConformanceError::Message(format!(
            "unsafe relative path: {relative}"
        )));
    }
    let joined = root.join(path);
    if !joined.starts_with(root) {
        return Err(ConformanceError::Message(format!(
            "path escapes root: {relative}"
        )));
    }
    Ok(joined)
}

fn install_atomically(
    staged: &Path,
    target: &Path,
    workspace: &Path,
) -> Result<(), ConformanceError> {
    let parent = target.parent().ok_or_else(|| {
        ConformanceError::Message(format!("output has no parent: {}", target.display()))
    })?;
    fs::create_dir_all(parent).map_err(|source| ConformanceError::Io {
        path: parent.to_owned(),
        source,
    })?;
    let backup_directory = workspace.join("backups");
    fs::create_dir_all(&backup_directory).map_err(|source| ConformanceError::Io {
        path: backup_directory.clone(),
        source,
    })?;
    let backup = backup_directory.join(format!(
        "{}-{}",
        target
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("service"),
        std::process::id()
    ));
    if backup.exists() {
        return Err(ConformanceError::Message(format!(
            "backup path already exists: {}",
            backup.display()
        )));
    }
    let existed = target.exists();
    if existed {
        fs::rename(target, &backup).map_err(|source| ConformanceError::Io {
            path: target.to_owned(),
            source,
        })?;
    }
    if let Err(source) = fs::rename(staged, target) {
        if existed {
            let _ = fs::rename(&backup, target);
        }
        return Err(ConformanceError::Io {
            path: target.to_owned(),
            source,
        });
    }
    Ok(())
}

fn verify_service(
    service: &ServiceManifest,
    stage: &Path,
    operations: &[Operation],
) -> Result<(), ConformanceError> {
    if stage.join("tests").exists() {
        return Err(ConformanceError::Message(format!(
            "{} retained tests/",
            service.key
        )));
    }
    let cargo =
        fs::read_to_string(stage.join("Cargo.toml")).map_err(|source| ConformanceError::Io {
            path: stage.join("Cargo.toml"),
            source,
        })?;
    let document = cargo.parse::<toml_edit::DocumentMut>().map_err(|error| {
        ConformanceError::Message(format!("{} Cargo.toml: {error}", service.key))
    })?;
    let package = document
        .get("package")
        .and_then(toml_edit::Item::as_table)
        .and_then(|table| table.get("name"))
        .and_then(toml_edit::Item::as_str);
    let library = document
        .get("lib")
        .and_then(toml_edit::Item::as_table)
        .and_then(|table| table.get("name"))
        .and_then(toml_edit::Item::as_str);
    if package != Some(service.package_name.as_str())
        || library != Some(service.library_name.as_str())
    {
        return Err(ConformanceError::Message(format!(
            "{} manifest names do not match manifest",
            service.key
        )));
    }
    let features = document
        .get("features")
        .and_then(toml_edit::Item::as_table)
        .ok_or_else(|| {
            ConformanceError::Message(format!("{} has no features table", service.key))
        })?;
    let default = features.get("default").and_then(toml_edit::Item::as_array);
    for operation in operations {
        if features.get(&operation.feature).is_none()
            || default.is_some_and(|default| {
                default
                    .iter()
                    .any(|value| value.as_str() == Some(operation.feature.as_str()))
            })
        {
            return Err(ConformanceError::Message(format!(
                "{} has an invalid {} feature",
                service.key, operation.feature
            )));
        }
    }
    for relative in diff::snapshot(stage)?.keys() {
        if relative.starts_with("tests/") {
            return Err(ConformanceError::Message(format!(
                "{} snapshot includes {relative}",
                service.key
            )));
        }
    }
    Ok(())
}

fn write_diff_artifacts(
    service: &ServiceManifest,
    stage: &Path,
    operations: &[Operation],
    changed_files: &[String],
    patch: &str,
) -> Result<(), ConformanceError> {
    let mut report = String::new();
    report.push_str("# AWS SDK modularizer diff\n\n");
    report.push_str(&format!("- repository: `{}`\n- revision: `{}`\n- service: `{}`\n- model: `{}`\n- package: `{}`\n- library: `{}`\n- tests exclusion: `tests/**` is excluded from the comparison.\n\n", service.repository, service.revision, service.upstream_path, service.model_path, service.package_name, service.library_name));
    report.push_str("## Operations\n\n");
    for operation in operations {
        report.push_str(&format!(
            "- `{}`: `{}`\n",
            operation.feature, operation.name
        ));
    }
    report.push_str("\n## Changed files\n\n");
    for file in changed_files {
        report.push_str(&format!("- `{file}`\n"));
    }
    report.push_str("\n## Customizations\n\n- Add one non-default Cargo feature and matching cfg gates for each model operation.\n- Rename the Cargo package/library from manifest data.\n- Remove upstream `tests/` from generated output.\n\n## Reproduction\n\n```text\ncargo run -p aws-sdk-modularizer -- --manifest services-manifest.json\n```\n");
    fs::write(stage.join("DIFF.MD"), report).map_err(|source| ConformanceError::Io {
        path: stage.join("DIFF.MD"),
        source,
    })?;
    fs::write(stage.join("DIFF.diff"), patch).map_err(|source| ConformanceError::Io {
        path: stage.join("DIFF.diff"),
        source,
    })?;
    Ok(())
}

fn verify_diff_artifacts(stage: &Path) -> Result<(), ConformanceError> {
    let report_path = stage.join("DIFF.MD");
    let report = fs::read_to_string(&report_path).map_err(|source| ConformanceError::Io {
        path: report_path.clone(),
        source,
    })?;
    if !report.contains("tests/**") {
        return Err(ConformanceError::Message(format!(
            "{} does not document tests/** exclusion",
            report_path.display()
        )));
    }
    let patch_path = stage.join("DIFF.diff");
    let patch = fs::read_to_string(&patch_path).map_err(|source| ConformanceError::Io {
        path: patch_path.clone(),
        source,
    })?;
    if patch.contains("tests/") || patch.contains("tests\\") {
        return Err(ConformanceError::Message(format!(
            "{} contains tests/ hunks",
            patch_path.display()
        )));
    }
    Ok(())
}

fn check_feature_matrix(
    service: &ServiceManifest,
    stage: &Path,
    model: &Model,
    operations: &[Operation],
    workspace: &Path,
) -> Result<(), ConformanceError> {
    let target_base = workspace.join("cargo-target").join(&service.key);
    run_cargo_check(stage, &target_base, &[])?;
    for operation in operations {
        run_cargo_check(
            stage,
            &target_base,
            std::slice::from_ref(&operation.feature),
        )?;
    }
    let all = operations
        .iter()
        .map(|operation| operation.feature.clone())
        .collect::<Vec<_>>();
    run_cargo_check(stage, &target_base, &all)?;
    for group in model.shared_operation_groups(operations) {
        if group.len() > 1 {
            run_cargo_check(stage, &target_base, &group)?;
        }
    }
    Ok(())
}

fn check_public_api(
    service: &ServiceManifest,
    stage: &Path,
    operations: &[Operation],
    workspace: &Path,
) -> Result<(), ConformanceError> {
    let enabled_probe = TempDir::new().map_err(|source| ConformanceError::Io {
        path: workspace.to_owned(),
        source,
    })?;
    write_probe_manifest(
        service,
        stage,
        enabled_probe.path(),
        operations
            .iter()
            .map(|operation| operation.feature.as_str()),
    )?;
    let enabled_source = enabled_probe.path().join("src/main.rs");
    fs::create_dir_all(enabled_source.parent().expect("probe source has parent")).map_err(
        |source| ConformanceError::Io {
            path: enabled_source.clone(),
            source,
        },
    )?;
    fs::write(&enabled_source, enabled_probe_source(operations)).map_err(|source| {
        ConformanceError::Io {
            path: enabled_source,
            source,
        }
    })?;
    run_public_probe(
        service,
        enabled_probe.path(),
        workspace
            .join("public-api-target")
            .join(&service.key)
            .join("enabled"),
        true,
        &[],
    )?;

    let disabled_probe = TempDir::new().map_err(|source| ConformanceError::Io {
        path: workspace.to_owned(),
        source,
    })?;
    write_probe_manifest(service, stage, disabled_probe.path(), std::iter::empty())?;
    let bins = disabled_probe.path().join("src/bin");
    fs::create_dir_all(&bins).map_err(|source| ConformanceError::Io {
        path: bins.clone(),
        source,
    })?;
    for operation in operations {
        let path = bins.join(format!("{}.rs", operation.module));
        fs::write(&path, disabled_probe_source(&operation.module))
            .map_err(|source| ConformanceError::Io { path, source })?;
    }
    run_public_probe(
        service,
        disabled_probe.path(),
        workspace
            .join("public-api-target")
            .join(&service.key)
            .join("disabled"),
        false,
        operations,
    )
}

fn write_probe_manifest<'a, I>(
    service: &ServiceManifest,
    stage: &Path,
    probe: &Path,
    features: I,
) -> Result<(), ConformanceError>
where
    I: IntoIterator<Item = &'a str>,
{
    let mut package = Table::new();
    package["name"] = toml_edit::value(format!("aws-sdk-modularizer-probe-{}", service.key));
    package["version"] = toml_edit::value("0.0.0");
    package["edition"] = toml_edit::value("2021");

    let mut dependency = Table::new();
    dependency["package"] = toml_edit::value(service.package_name.clone());
    dependency["path"] = toml_edit::value(stage.to_string_lossy().into_owned());
    let mut feature_array = Array::new();
    for feature in features {
        feature_array.push(feature);
    }
    if !feature_array.is_empty() {
        dependency["features"] = TomlItem::Value(Value::Array(feature_array));
    }

    let mut dependencies = Table::new();
    dependencies["service"] = TomlItem::Table(dependency);
    let mut document = DocumentMut::new();
    document["package"] = TomlItem::Table(package);
    document["dependencies"] = TomlItem::Table(dependencies);
    let path = probe.join("Cargo.toml");
    fs::create_dir_all(probe).map_err(|source| ConformanceError::Io {
        path: probe.to_owned(),
        source,
    })?;
    fs::write(&path, document.to_string()).map_err(|source| ConformanceError::Io { path, source })
}

fn enabled_probe_source(operations: &[Operation]) -> String {
    let mut source = String::from("#![allow(unused_imports)]\n");
    for operation in operations {
        writeln!(source, "use service::operation::{};", operation.module)
            .expect("writing to a String cannot fail");
    }
    writeln!(source, "\nfn probe(client: &service::Client) {{")
        .expect("writing to a String cannot fail");
    for operation in operations {
        writeln!(source, "    let _ = client.{}();", operation.module)
            .expect("writing to a String cannot fail");
    }
    source.push_str("}\n\nfn main() {}\n");
    source
}

fn disabled_probe_source(module: &str) -> String {
    format!(
        "use service::operation::{module};\n\nfn main() {{\n    let client: &service::Client = unreachable!();\n    let _ = client.{module}();\n}}\n"
    )
}

fn run_public_probe(
    service: &ServiceManifest,
    probe: &Path,
    target: PathBuf,
    expect_success: bool,
    operations: &[Operation],
) -> Result<(), ConformanceError> {
    fs::create_dir_all(&target).map_err(|source| ConformanceError::Io {
        path: target.clone(),
        source,
    })?;
    let manifest_path = probe.join("Cargo.toml");
    let manifest_path = manifest_path
        .to_str()
        .ok_or_else(|| ConformanceError::Message("probe manifest path is not UTF-8".to_owned()))?;
    let mut arguments = vec!["check"];
    if !expect_success {
        arguments.push("--bins");
    }
    arguments.extend(["--manifest-path", manifest_path]);
    let output = Command::new("cargo")
        .args(arguments)
        .env("CARGO_TARGET_DIR", &target)
        .output()
        .map_err(|source| {
            ConformanceError::Message(format!(
                "failed to run public API probe for {}: {source}",
                service.key
            ))
        })?;
    if output.status.success() != expect_success {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(ConformanceError::Message(format!(
            "public API probe {} for {} had unexpected result: {}",
            if expect_success {
                "failed"
            } else {
                "succeeded"
            },
            service.key,
            stderr
        )));
    }
    if !expect_success {
        let stderr = String::from_utf8_lossy(&output.stderr);
        for operation in operations {
            let source_path = format!("src/bin/{}.rs", operation.module);
            if !stderr.contains(&source_path) {
                return Err(ConformanceError::Message(format!(
                    "disabled public API probe for {} did not reject {}",
                    service.key, operation.module
                )));
            }
        }
    }
    Ok(())
}

fn run_cargo_check(
    crate_root: &Path,
    target: &Path,
    features: &[String],
) -> Result<(), ConformanceError> {
    fs::create_dir_all(target).map_err(|source| ConformanceError::Io {
        path: target.to_owned(),
        source,
    })?;
    let manifest_path = crate_root.join("Cargo.toml");
    let manifest_path = manifest_path
        .to_str()
        .ok_or_else(|| ConformanceError::Message("manifest path is not UTF-8".to_owned()))?;
    let mut arguments = vec!["check", "--manifest-path", manifest_path];
    let feature_argument;
    if !features.is_empty() {
        feature_argument = features.join(",");
        arguments.extend(["--features", &feature_argument]);
    }
    let output = Command::new("cargo")
        .args(arguments)
        .env("CARGO_TARGET_DIR", target)
        .output()
        .map_err(|source| {
            ConformanceError::Message(format!("failed to run cargo check: {source}"))
        })?;
    if !output.status.success() {
        return Err(ConformanceError::Message(format!(
            "cargo check failed for {} with features [{}]: {}",
            crate_root.display(),
            features.join(","),
            String::from_utf8_lossy(&output.stderr)
        )));
    }
    Ok(())
}

fn render_summary(
    reports: &[(String, Coverage, Vec<String>)],
    previous: Option<&str>,
) -> Result<String, ConformanceError> {
    let mut output = String::from("# Operation coverage\n\n");
    for (key, coverage, changed_files) in reports {
        let old = previous.and_then(|text| prior_transformed(text, key));
        let delta = old.map_or(
            format!("+{} (no previous report)", coverage.transformed),
            |old| format_signed(coverage.transformed as isize - old as isize),
        );
        output.push_str(&format!("## {key}\n\n- total: {}\n- transformed: {}\n- missing: {}\n- ambiguous: {}\n- coverage delta: {}\n- changed files: {}\n\n", coverage.total, coverage.transformed, coverage.missing, coverage.ambiguous, delta, changed_files.len()));
    }
    Ok(output)
}

fn prior_transformed(summary: &str, service: &str) -> Option<usize> {
    let mut in_service = false;
    for line in summary.lines() {
        if let Some(name) = line.strip_prefix("## ") {
            in_service = name == service;
            continue;
        }
        if in_service {
            if let Some(value) = line.strip_prefix("- transformed: ") {
                return value.trim().parse().ok();
            }
        }
    }
    None
}

fn format_signed(value: isize) -> String {
    if value >= 0 {
        format!("+{value}")
    } else {
        value.to_string()
    }
}

fn run_process(
    program: &str,
    arguments: &[&str],
    current_dir: &Path,
) -> Result<(), ConformanceError> {
    let output = Command::new(program)
        .args(arguments)
        .current_dir(current_dir)
        .output()
        .map_err(|source| {
            ConformanceError::Message(format!("failed to run {program}: {source}"))
        })?;
    if !output.status.success() {
        return Err(ConformanceError::Message(format!(
            "{program} failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }
    Ok(())
}

fn argument_path(arguments: &[OsString], flag: &str) -> Option<PathBuf> {
    arguments
        .windows(2)
        .find(|pair| pair[0] == flag)
        .map(|pair| PathBuf::from(&pair[1]))
}
