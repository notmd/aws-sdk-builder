mod config;
mod error;
pub mod model;
pub mod output;
pub(crate) mod prune;
pub mod runner;
pub mod smithy;

pub use config::Builder;
pub use error::BuildError;

#[derive(Debug, Clone)]
pub struct CompileReport {
    pub generated_root: std::path::PathBuf,
    pub manifest: std::path::PathBuf,
    pub operations: Vec<String>,
}

pub fn configure() -> Builder {
    Builder::default()
}

impl Builder {
    pub fn compile(self) -> Result<CompileReport, BuildError> {
        let config::ValidatedConfig {
            model_path,
            service,
            selection,
        } = self.validate()?;
        let out_dir = self
            .out_dir
            .as_deref()
            .ok_or(BuildError::MissingOutputDirectory)?;
        println!("cargo:rerun-if-changed={}", model_path.display());
        println!("cargo:rerun-if-env-changed=SMITHY_CLI");
        if let Some(smithy) = self.smithy.as_deref() {
            println!("cargo:rerun-if-changed={}", smithy.display());
        }

        let workspace = tempfile::tempdir().map_err(|source| BuildError::OutputWrite {
            path: out_dir.to_path_buf(),
            source,
        })?;
        let model_path_in_workspace = workspace.path().join("model.json");
        selection.write_json(&model_path_in_workspace)?;
        let smithy_config = smithy::BuildConfig::new(
            std::path::Path::new("model.json"),
            &service,
            std::path::Path::new("output"),
            &self.rust_client_codegen,
        )
        .to_json();
        let smithy_config_path = workspace.path().join("smithy-build.json");
        let smithy_config_text =
            serde_json::to_string_pretty(&smithy_config).map_err(|source| {
                BuildError::OutputWrite {
                    path: smithy_config_path.clone(),
                    source: std::io::Error::other(source),
                }
            })?;
        std::fs::write(&smithy_config_path, smithy_config_text).map_err(|source| {
            BuildError::OutputWrite {
                path: smithy_config_path.clone(),
                source,
            }
        })?;

        let executable = runner::resolve_from_environment(self.smithy.as_deref())?;
        runner::run(&executable, workspace.path())?;
        output::install(
            &workspace.path().join("output"),
            out_dir,
            &service,
            selection.operations(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_rejects_missing_model_and_empty_operation_selection() {
        let missing = configure()
            .model("does-not-exist.json")
            .service("example#Service")
            .operations(["GetThing"])
            .validate()
            .expect_err("missing model must fail before tool invocation");
        assert!(matches!(missing, BuildError::MissingModel { .. }));

        let empty = configure()
            .model("model.json")
            .service("example#Service")
            .operations(std::iter::empty::<&str>())
            .validate()
            .expect_err("empty operation selection must be rejected");
        assert!(matches!(empty, BuildError::EmptyOperations));
    }

    #[test]
    fn builder_validation_resolves_the_selected_model_operations() {
        let model = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures/selection-model.json");
        let validated = configure()
            .model(model)
            .service("example#Service")
            .operations(["GetThing"])
            .validate()
            .unwrap();

        assert_eq!(validated.selection.operations(), &["GetThing"]);
    }
}
