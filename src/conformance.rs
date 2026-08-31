use crate::{
    diff,
    manifest::{self, Manifest, ServiceManifest},
    model::{Model, Operation},
    transform::{self, Coverage, TransformError},
};
use std::{
    collections::BTreeSet,
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
    Io { path: PathBuf, source: std::io::Error },
}

#[derive(Debug, Clone)]
struct FeatureMatrix {
    singletons: Vec<Vec<String>>,
    all: Vec<String>,
    shared_groups: Vec<Vec<String>>,
}

pub fn run_cli(arguments: Vec<OsString>) -> Result<(), ConformanceError> {
    let conformance = arguments.first().is_some_and(|argument| argument == "conformance");
    let manifest_path =
        argument_path(&arguments, "--manifest").unwrap_or_else(|| PathBuf::from(manifest::DEFAULT_PATH));
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
    let upstream = prepare_upstream(manifest, workspace.path())?;
    for service in &manifest.services {
        let model = load_model(service, &upstream)?;
        let operations = model.operations()?;
        let staged = stage_service(service, &upstream, workspace.path(), &operations)?;
        write_stage_artifacts(manifest, service, &staged, &upstream, workspace.path(), &operations)?;
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
    let upstream = prepare_upstream(manifest, workspace.path())?;
    let previous = fs::read_to_string(root.join("conformance/summary.md")).ok();
    let mut reports = Vec::new();
    for service in &manifest.services {
        let model = load_model(service, &upstream)?;
        let operations = model.operations()?;
        let staged = stage_service(service, &upstream, workspace.path(), &operations)?;
        verify_service(service, &staged, &operations)?;
        let changed_files =
            write_stage_artifacts(manifest, service, &staged, &upstream, workspace.path(), &operations)?;
        verify_diff_artifacts(&staged, &changed_files)?;
        let feature_matrix = check_feature_matrix(service, &staged, &model, &operations, workspace.path())?;
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
            feature_matrix,
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
    manifest: &Manifest,
    service: &ServiceManifest,
    staged: &Path,
    upstream: &Path,
    workspace: &Path,
    operations: &[Operation],
) -> Result<Vec<String>, ConformanceError> {
    let formatted_upstream = workspace.join("formatted-upstream").join(&service.key);
    let source = safe_join(upstream, &service.upstream_path)?;
    copy_without_tests(&source, &formatted_upstream)?;
    format_crate(&formatted_upstream)?;
    let before = diff::snapshot(&formatted_upstream)?;
    let after = diff::snapshot(staged)?;
    let changed_files = diff::changed_files(&before, &after);
    let file_patches = diff::file_patches(&before, &after);
    let patch = diff::unified_patch(&before, &after);
    write_diff_artifacts(
        manifest,
        service,
        staged,
        operations,
        &changed_files,
        &file_patches,
        &patch,
    )?;
    Ok(changed_files)
}

fn stage_service(
    service: &ServiceManifest,
    upstream: &Path,
    workspace: &Path,
    operations: &[Operation],
) -> Result<PathBuf, ConformanceError> {
    let source = safe_join(upstream, &service.upstream_path)?;
    if !source.join("src").is_dir() {
        return Err(ConformanceError::Message(format!(
            "{} has no src directory",
            source.display()
        )));
    }
    let stage = workspace.join("staged").join(&service.key);
    copy_without_tests(&source, &stage)?;
    let package_name = service.package_name();
    let library_name = service.library_name();
    transform::transform_tree(&stage, &package_name, &library_name, operations)?;
    format_crate(&stage)?;
    Ok(stage)
}

fn format_crate(crate_root: &Path) -> Result<(), ConformanceError> {
    let manifest_path = crate_root.join("Cargo.toml");
    let manifest_path = manifest_path
        .to_str()
        .ok_or_else(|| ConformanceError::Message("staged manifest path is not UTF-8".to_owned()))?;
    run_process(
        "cargo",
        &[
            "fmt",
            "--manifest-path",
            manifest_path,
            "--all",
            "--",
            "--config",
            "edition=2021",
            "--config",
            "max_width=120",
            "--config",
            "newline_style=Unix",
        ],
        crate_root,
    )?;
    Ok(())
}

fn load_model(service: &ServiceManifest, upstream: &Path) -> Result<Model, ConformanceError> {
    let path = safe_join(upstream, &service.model_path)?;
    Ok(Model::load(&path)?)
}

fn prepare_upstream(manifest: &Manifest, workspace: &Path) -> Result<PathBuf, ConformanceError> {
    let archive = workspace.join(format!("{}.tar.gz", manifest.revision));
    let unpacked = workspace.join(format!("unpacked-{}", manifest.revision));
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
        let url = archive_url(&manifest.repository, &manifest.revision)?;
        run_process(
            "curl",
            &[
                "-L",
                "--fail",
                "--silent",
                "--show-error",
                &url,
                "-o",
                archive
                    .to_str()
                    .ok_or_else(|| ConformanceError::Message("archive path is not UTF-8".to_owned()))?,
            ],
            workspace,
        )?;
    }
    run_process(
        "tar",
        &[
            "-xzf",
            archive
                .to_str()
                .ok_or_else(|| ConformanceError::Message("archive path is not UTF-8".to_owned()))?,
            "-C",
            unpacked
                .to_str()
                .ok_or_else(|| ConformanceError::Message("unpack path is not UTF-8".to_owned()))?,
        ],
        workspace,
    )?;
    single_directory(&unpacked)
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
    Ok(format!("https://github.com/{value}/archive/{revision}.tar.gz"))
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
        let file_type = entry.file_type().map_err(|source_error| ConformanceError::Io {
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
            fs::copy(&source_path, &destination_path).map_err(|source_error| ConformanceError::Io {
                path: destination_path,
                source: source_error,
            })?;
        }
    }
    Ok(())
}

fn safe_join(root: &Path, relative: &str) -> Result<PathBuf, ConformanceError> {
    let path = Path::new(relative);
    if path.is_absolute()
        || path
            .components()
            .any(|component| matches!(component, Component::ParentDir | Component::CurDir | Component::RootDir))
    {
        return Err(ConformanceError::Message(format!("unsafe relative path: {relative}")));
    }
    let joined = root.join(path);
    if !joined.starts_with(root) {
        return Err(ConformanceError::Message(format!("path escapes root: {relative}")));
    }
    Ok(joined)
}

fn install_atomically(staged: &Path, target: &Path, workspace: &Path) -> Result<(), ConformanceError> {
    let parent = target
        .parent()
        .ok_or_else(|| ConformanceError::Message(format!("output has no parent: {}", target.display())))?;
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
        target.file_name().and_then(|name| name.to_str()).unwrap_or("service"),
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

fn verify_service(service: &ServiceManifest, stage: &Path, operations: &[Operation]) -> Result<(), ConformanceError> {
    if stage.join("tests").exists() {
        return Err(ConformanceError::Message(format!("{} retained tests/", service.key)));
    }
    let cargo = fs::read_to_string(stage.join("Cargo.toml")).map_err(|source| ConformanceError::Io {
        path: stage.join("Cargo.toml"),
        source,
    })?;
    let document = cargo
        .parse::<toml_edit::DocumentMut>()
        .map_err(|error| ConformanceError::Message(format!("{} Cargo.toml: {error}", service.key)))?;
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
    let package_name = service.package_name();
    let library_name = service.library_name();
    let library_matches =
        library == Some(library_name.as_str()) || (library.is_none() && package_name.replace('-', "_") == library_name);
    if package != Some(package_name.as_str()) || !library_matches {
        return Err(ConformanceError::Message(format!(
            "{} manifest names do not match manifest",
            service.key
        )));
    }
    let features = document
        .get("features")
        .and_then(toml_edit::Item::as_table)
        .ok_or_else(|| ConformanceError::Message(format!("{} has no features table", service.key)))?;
    let expected_operation_features = operations
        .iter()
        .map(|operation| operation.feature.clone())
        .collect::<BTreeSet<_>>();
    if !operation_features_match(features, &expected_operation_features) {
        return Err(ConformanceError::Message(format!(
            "{} operation features do not match model: expected {:?}, found {:?}",
            service.key,
            expected_operation_features,
            operation_feature_names(features)
        )));
    }
    let default = features.get("default").and_then(toml_edit::Item::as_array);
    for operation in operations {
        if default.is_some_and(|default| {
            default
                .iter()
                .any(|value| value.as_str() == Some(operation.feature.as_str()))
        }) {
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

fn operation_feature_names(features: &Table) -> BTreeSet<String> {
    features
        .iter()
        .filter_map(|(name, _)| name.starts_with("op_").then(|| name.to_owned()))
        .collect()
}

fn operation_features_match(features: &Table, expected: &BTreeSet<String>) -> bool {
    operation_feature_names(features) == *expected
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operation_feature_set_rejects_unknown_features() {
        let document = r#"
            [features]
            op_alpha = []
            op_unknown = []
            default = []
        "#
        .parse::<DocumentMut>()
        .expect("valid Cargo manifest");
        let features = document["features"].as_table().expect("features table");
        let expected = BTreeSet::from(["op_alpha".to_owned()]);
        assert!(!operation_features_match(features, &expected));
        assert_eq!(
            operation_feature_names(features),
            BTreeSet::from(["op_alpha".to_owned(), "op_unknown".to_owned()])
        );
    }

    #[test]
    fn formats_feature_matrix_selections_exactly() {
        let selections = vec![
            vec!["op_alpha".to_owned()],
            vec!["op_beta".to_owned(), "op_gamma".to_owned()],
        ];

        assert_eq!(
            format_feature_selections(&selections),
            "[op_alpha], [op_beta, op_gamma]"
        );
    }
}

fn write_diff_artifacts(
    manifest: &Manifest,
    service: &ServiceManifest,
    stage: &Path,
    operations: &[Operation],
    changed_files: &[String],
    file_patches: &[(String, String)],
    patch: &str,
) -> Result<(), ConformanceError> {
    let mut report = String::new();
    report.push_str("# AWS SDK modularizer diff\n\n");
    let package_name = service.package_name();
    let library_name = service.library_name();
    report.push_str(&format!("- repository: `{}`\n- revision: `{}`\n- service: `{}`\n- model: `{}`\n- package: `{}`\n- library: `{}`\n- tests exclusion: `tests/**` is excluded from the comparison.\n\n", manifest.repository, manifest.revision, service.upstream_path, service.model_path, package_name, library_name));
    report.push_str("## Operations\n\n");
    for operation in operations {
        report.push_str(&format!("- `{}`: `{}`\n", operation.feature, operation.name));
    }
    report.push_str("\n## Changed files\n\n");
    for file in changed_files {
        report.push_str(&format!("- `{file}`\n"));
    }
    report.push_str("\n## File diffs\n\n");
    for (file, file_patch) in file_patches {
        report.push_str(&format!("### `{file}`\n\n```diff\n"));
        report.push_str(file_patch);
        if !file_patch.ends_with('\n') {
            report.push('\n');
        }
        report.push_str("```\n\n");
    }
    if file_patches.is_empty() {
        report.push_str("No textual changes.\n\n");
    }
    report.push_str("## Unified diff\n\n```diff\n");
    report.push_str(patch);
    if !patch.ends_with('\n') {
        report.push('\n');
    }
    report.push_str("```\n\n");
    report.push_str("\n## Customizations\n\n- Add one non-default Cargo feature and matching cfg gates for each model operation.\n- Derive the Cargo package/library names from the service key.\n- Remove upstream `tests/` from generated output.\n\n## Reproduction\n\n```text\ncargo run -p aws-sdk-modularizer -- --manifest services-manifest.json\n```\n");
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

fn verify_diff_artifacts(stage: &Path, changed_files: &[String]) -> Result<(), ConformanceError> {
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
    if !report.contains("## File diffs") {
        return Err(ConformanceError::Message(format!(
            "{} does not contain per-file diffs",
            report_path.display()
        )));
    }
    for file in changed_files {
        let heading = format!("### `{file}`");
        let marker = format!("diff --git a/{file} b/{file}");
        if !report.contains(&heading) || !report.contains(&marker) {
            return Err(ConformanceError::Message(format!(
                "{} does not embed the diff for {file}",
                report_path.display()
            )));
        }
    }
    if !report.contains("## Unified diff") {
        return Err(ConformanceError::Message(format!(
            "{} does not contain the unified diff",
            report_path.display()
        )));
    }
    if !patch.is_empty() && !report.contains(&patch) {
        return Err(ConformanceError::Message(format!(
            "{} does not embed the unified diff",
            report_path.display()
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
) -> Result<FeatureMatrix, ConformanceError> {
    let target_base = workspace.join("cargo-target").join(&service.key);
    run_cargo_check(stage, &target_base, &[])?;
    let mut singletons = Vec::with_capacity(operations.len());
    for operation in operations {
        run_cargo_check(stage, &target_base, std::slice::from_ref(&operation.feature))?;
        singletons.push(vec![operation.feature.clone()]);
    }
    let all = operations
        .iter()
        .map(|operation| operation.feature.clone())
        .collect::<Vec<_>>();
    run_cargo_check(stage, &target_base, &all)?;
    let shared_groups = model.shared_operation_groups(operations);
    for group in &shared_groups {
        run_cargo_check(stage, &target_base, group)?;
    }
    Ok(FeatureMatrix {
        singletons,
        all,
        shared_groups,
    })
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
        operations.iter().map(|operation| operation.feature.as_str()),
    )?;
    let enabled_source = enabled_probe.path().join("src/main.rs");
    fs::create_dir_all(enabled_source.parent().expect("probe source has parent")).map_err(|source| {
        ConformanceError::Io {
            path: enabled_source.clone(),
            source,
        }
    })?;
    fs::write(&enabled_source, enabled_probe_source(operations)).map_err(|source| ConformanceError::Io {
        path: enabled_source,
        source,
    })?;
    run_public_probe(
        service,
        enabled_probe.path(),
        workspace.join("public-api-target").join(&service.key).join("enabled"),
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
        workspace.join("public-api-target").join(&service.key).join("disabled"),
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
    dependency["package"] = toml_edit::value(service.package_name());
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
        writeln!(source, "use service::operation::{};", operation.module).expect("writing to a String cannot fail");
    }
    writeln!(source, "\nfn probe(client: &service::Client) {{").expect("writing to a String cannot fail");
    for operation in operations {
        writeln!(source, "    let _ = client.{}();", operation.module).expect("writing to a String cannot fail");
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
    if expect_success {
        let arguments = ["check", "--manifest-path", manifest_path];
        let output = Command::new("cargo")
            .args(arguments)
            .env("CARGO_TARGET_DIR", &target)
            .output()
            .map_err(|source| {
                ConformanceError::Message(format!("failed to run public API probe for {}: {source}", service.key))
            })?;
        if !output.status.success() {
            return Err(ConformanceError::Message(format!(
                "public API probe failed for {}: {}",
                service.key,
                String::from_utf8_lossy(&output.stderr)
            )));
        }
        return Ok(());
    }

    for operation in operations {
        let mut arguments = vec!["check", "--bin", operation.module.as_str()];
        arguments.extend(["--manifest-path", manifest_path]);
        let output = Command::new("cargo")
            .args(&arguments)
            .env("CARGO_TARGET_DIR", &target)
            .output()
            .map_err(|source| {
                ConformanceError::Message(format!(
                    "failed to run disabled public API probe for {}: {source}",
                    service.key
                ))
            })?;
        let diagnostics = format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        if output.status.success() {
            return Err(ConformanceError::Message(format!(
                "disabled public API probe for {} unexpectedly accepted {}",
                service.key, operation.module,
            )));
        }
        let source_path = format!("src/bin/{}.rs", operation.module);
        if !diagnostics.contains(&source_path) {
            let diagnostic = diagnostics.chars().take(4_000).collect::<String>();
            return Err(ConformanceError::Message(format!(
                "disabled public API probe for {} did not reject {}: {diagnostic}",
                service.key, operation.module,
            )));
        }
    }
    Ok(())
}

fn run_cargo_check(crate_root: &Path, target: &Path, features: &[String]) -> Result<(), ConformanceError> {
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
        .map_err(|source| ConformanceError::Message(format!("failed to run cargo check: {source}")))?;
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
    reports: &[(String, Coverage, Vec<String>, FeatureMatrix)],
    previous: Option<&str>,
) -> Result<String, ConformanceError> {
    let mut output = String::from("# Operation coverage\n\n");
    for (key, coverage, changed_files, feature_matrix) in reports {
        let old = previous.and_then(|text| prior_transformed(text, key));
        let delta = old.map_or(format!("+{} (no previous report)", coverage.transformed), |old| {
            format_signed(coverage.transformed as isize - old as isize)
        });
        output.push_str(&format!(
            "## {key}\n\n- total: {}\n- transformed: {}\n- missing: {}\n- ambiguous: {}\n- coverage delta: {}\n- changed files: {}\n- feature selections:\n  - zero: `[]`\n  - singleton: `{}`\n  - all: `{}`\n  - shared groups: `{}`\n\n",
            coverage.total,
            coverage.transformed,
            coverage.missing,
            coverage.ambiguous,
            delta,
            changed_files.len(),
            format_feature_selections(&feature_matrix.singletons),
            format_feature_selection(&feature_matrix.all),
            format_feature_selections(&feature_matrix.shared_groups),
        ));
    }
    Ok(output)
}

fn format_feature_selection(features: &[String]) -> String {
    format!("[{}]", features.join(", "))
}

fn format_feature_selections(selections: &[Vec<String>]) -> String {
    if selections.is_empty() {
        "none".to_owned()
    } else {
        selections
            .iter()
            .map(|selection| format_feature_selection(selection))
            .collect::<Vec<_>>()
            .join(", ")
    }
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

fn run_process(program: &str, arguments: &[&str], current_dir: &Path) -> Result<(), ConformanceError> {
    let output = Command::new(program)
        .args(arguments)
        .current_dir(current_dir)
        .output()
        .map_err(|source| ConformanceError::Message(format!("failed to run {program}: {source}")))?;
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
