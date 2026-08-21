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
        let generated = codegen::generate(stage.path(), &crate_name, &selections)?;
        output::install(stage.path(), &out_dir).map(|mut report| {
            report.operations = generated.operations;
            report
        })
    }
}
