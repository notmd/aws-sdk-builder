mod config;
mod error;

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
        self.validate()?;
        Err(BuildError::GenerationUnavailable)
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
}
