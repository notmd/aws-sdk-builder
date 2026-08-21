mod codegen;
mod config;
mod error;
pub mod model;
mod names;
mod output;
pub mod registry;

pub use config::Builder;
pub use error::BuildError;

#[derive(Debug, Clone)]
pub struct CompileReport {
    pub generated_root: std::path::PathBuf,
    pub manifest: std::path::PathBuf,
    pub consumer_crate_name: String,
    pub operations: Vec<String>,
}

pub fn configure() -> Builder {
    Builder::default()
}

/// Generate an all-operation source snapshot for an external comparison or
/// inspection tool.
///
/// This is intentionally separate from [`Builder::compile`]: consumers generate
/// into Cargo's `OUT_DIR`, while tooling can own a source snapshot directory.
pub fn generate_all<I, S>(
    output_dir: impl AsRef<std::path::Path>,
    services: I,
) -> Result<usize, BuildError>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
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
        .map(|service| config::ServiceSelection {
            key: service.as_ref().to_owned(),
            operations: Vec::new(),
            all_operations: true,
        })
        .collect::<Vec<_>>();
    if selections.is_empty() {
        return Err(BuildError::NoServices);
    }

    let stage = tempfile::Builder::new()
        .prefix("aws-sdk-snapshot-")
        .tempdir_in(parent)
        .map_err(|source| BuildError::StageCreate {
            path: parent.to_owned(),
            source,
        })?;
    let generated = codegen::generate(stage.path(), "generated_snapshot", false, &selections)?;
    output::validate_tree(&stage.path().join("generated"))?;
    output::install_snapshot(stage.path(), output_dir)?;
    Ok(generated.operations.len())
}

/// Includes the stable generated `OUT_DIR/aws_sdk.rs` facade.
#[macro_export]
macro_rules! include_sdk {
    () => {
        include!(concat!(env!("OUT_DIR"), "/aws_sdk.rs"));
    };
}

impl Builder {
    pub fn compile(self) -> Result<CompileReport, BuildError> {
        let out_dir = std::env::var_os("OUT_DIR")
            .map(std::path::PathBuf::from)
            .ok_or(BuildError::MissingCargoEnvironment {
                variable: "OUT_DIR",
            })?;
        let package_name =
            std::env::var("CARGO_PKG_NAME").map_err(|_| BuildError::MissingCargoEnvironment {
                variable: "CARGO_PKG_NAME",
            })?;
        let crate_name = names::rust_crate_name(&package_name);
        let selections = self.resolve()?;
        let stage = tempfile::Builder::new()
            .prefix("aws-sdk-build-")
            .tempdir_in(&out_dir)
            .map_err(|source| BuildError::StageCreate {
                path: out_dir.clone(),
                source,
            })?;
        let generated = codegen::generate(stage.path(), &crate_name, true, &selections)?;
        output::install(stage.path(), &out_dir).map(|mut report| {
            report.operations = generated.operations;
            report
        })
    }
}

#[cfg(test)]
mod tests {
    use super::generate_all;

    #[test]
    fn generate_all_installs_an_all_operation_service_snapshot() {
        let output_root = tempfile::tempdir().unwrap();
        let generated = output_root.path().join("generated");
        let operation_count = generate_all(&generated, ["s3"]).unwrap();

        assert!(operation_count > 2);
        assert!(
            generated
                .join("s3/src/operation/create_bucket.rs")
                .is_file()
        );
        assert!(generated.join("s3/src/operation/head_bucket.rs").is_file());
    }
}
