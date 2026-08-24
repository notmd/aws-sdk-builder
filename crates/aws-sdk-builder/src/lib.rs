mod artifact;
mod codegen;
mod config;
mod endpoint_codegen;
mod error;
pub mod model;
mod names;
mod output;
pub mod registry;

pub use config::{Builder, OperationNames};
pub use error::BuildError;
pub use registry::{ServiceMetadata, ServiceSource};

#[derive(Debug, Clone)]
pub struct CompileReport {
    pub generated_root: std::path::PathBuf,
    pub operations: Vec<String>,
}

/// The file plan needed to project a generated canonical artifact for review.
#[derive(Debug, Clone)]
pub struct ConformanceSnapshot {
    pub operation_count: usize,
    pub service_files: std::collections::BTreeMap<String, std::collections::BTreeSet<String>>,
}

pub fn configure() -> Builder {
    Builder::default()
}

/// Validates a service model before a build script asks the generator to use it.
pub fn validate_model(metadata: ServiceMetadata, model: &'static [u8]) -> Result<(), BuildError> {
    model::Model::load(ServiceSource::new(metadata, model)).map(|_| ())
}

/// Compiles one service model into the consumer shared generated SDK tree.
///
/// Service builder crates expose this function through their own compile
/// wrapper. An empty operation collection selects every operation.
pub fn compile<O: OperationNames>(
    metadata: ServiceMetadata,
    model: &'static [u8],
    operations: O,
) -> Result<CompileReport, BuildError> {
    compile_source(ServiceSource::new(metadata, model), operations)
}

/// Compiles a service source.
pub fn compile_source<O: OperationNames>(
    source: ServiceSource,
    operations: O,
) -> Result<CompileReport, BuildError> {
    let out_dir = std::env::var_os("OUT_DIR")
        .map(std::path::PathBuf::from)
        .ok_or(BuildError::MissingCargoEnvironment {
            variable: "OUT_DIR",
        })?;
    let requested = config::selection(source, operations);
    let state_path = out_dir.join(".aws-sdk-builder-state.json");
    let state = read_state(&state_path)?;
    let old = state
        .get("services")
        .and_then(serde_json::Value::as_object)
        .and_then(|services| services.get(source.metadata.key))
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}));
    let old_all = old
        .get("all_operations")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let mut merged_operations = old
        .get("operations")
        .and_then(serde_json::Value::as_array)
        .map(|operations| {
            operations
                .iter()
                .filter_map(serde_json::Value::as_str)
                .map(ToOwned::to_owned)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    if requested.all_operations {
        merged_operations.clear();
    } else if !old_all {
        merged_operations.extend(requested.operations);
        merged_operations.sort();
        merged_operations.dedup();
    } else {
        merged_operations.clear();
    }
    let merged_all = old_all || requested.all_operations;
    let merged = config::ServiceSelection {
        source,
        operations: merged_operations.clone(),
        all_operations: merged_all,
    };

    let stage = tempfile::Builder::new()
        .prefix("aws-sdk-builder-")
        .tempdir_in(&out_dir)
        .map_err(|source| BuildError::StageCreate {
            path: out_dir.clone(),
            source,
        })?;
    if out_dir.join("generated").is_dir() {
        output::copy_tree(&out_dir.join("generated"), &stage.path().join("generated"))?;
    }
    let generated = codegen::generate(stage.path(), &[merged])?;
    let mut next_state = state;
    next_state["services"][source.metadata.key] = serde_json::json!({
        "all_operations": merged_all,
        "operations": merged_operations,
    });
    let state_text = serde_json::to_string_pretty(&next_state).expect("state is serializable");
    output::install_service(stage.path(), &out_dir, generated.operations, &state_text)
}

fn read_state(path: &std::path::Path) -> Result<serde_json::Value, BuildError> {
    if !path.exists() {
        return Ok(serde_json::json!({"services": {}}));
    }
    let bytes = std::fs::read(path).map_err(|source| BuildError::SourceRead {
        path: path.to_owned(),
        source,
    })?;
    let mut state = serde_json::from_slice::<serde_json::Value>(&bytes).map_err(|source| {
        BuildError::StateParse {
            path: path.to_owned(),
            source,
        }
    })?;
    if !state
        .get("services")
        .is_some_and(serde_json::Value::is_object)
    {
        state["services"] = serde_json::json!({});
    }
    Ok(state)
}

/// Generate an all-operation source snapshot for conformance or inspection.
pub fn generate_all<I>(
    output_dir: impl AsRef<std::path::Path>,
    services: I,
) -> Result<ConformanceSnapshot, BuildError>
where
    I: IntoIterator<Item = ServiceSource>,
{
    let output_dir = output_dir.as_ref();
    let parent = output_dir
        .parent()
        .unwrap_or_else(|| std::path::Path::new("."));
    std::fs::create_dir_all(parent).map_err(|source| BuildError::StageCreate {
        path: parent.to_owned(),
        source,
    })?;
    let selections = services
        .into_iter()
        .map(|source| config::selection(source, std::iter::empty::<&'static str>()))
        .collect::<Vec<_>>();
    if selections.is_empty() {
        return Err(BuildError::NoServices);
    }

    let stage = tempfile::Builder::new()
        .prefix("aws-sdk-builder-snapshot-")
        .tempdir_in(parent)
        .map_err(|source| BuildError::StageCreate {
            path: parent.to_owned(),
            source,
        })?;
    let generated = codegen::generate(stage.path(), &selections)?;
    output::install_snapshot(stage.path(), output_dir)?;
    Ok(ConformanceSnapshot {
        operation_count: generated.operations.len(),
        service_files: generated
            .files
            .into_iter()
            .map(|(service, files)| (service, files.into_keys().collect()))
            .collect(),
    })
}

impl Builder {
    pub fn compile(self) -> Result<CompileReport, BuildError> {
        let out_dir = std::env::var_os("OUT_DIR")
            .map(std::path::PathBuf::from)
            .ok_or(BuildError::MissingCargoEnvironment {
                variable: "OUT_DIR",
            })?;
        let selections = self.resolve()?;
        let stage = tempfile::Builder::new()
            .prefix("aws-sdk-builder-")
            .tempdir_in(&out_dir)
            .map_err(|source| BuildError::StageCreate {
                path: out_dir.clone(),
                source,
            })?;
        let generated = codegen::generate(stage.path(), &selections)?;
        output::install(stage.path(), &out_dir, generated.operations)
    }
}
