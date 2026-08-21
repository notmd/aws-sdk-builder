use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum BuildError {
    #[error("no Smithy model was configured")]
    MissingModelConfiguration,
    #[error("Smithy model path does not exist: {path}")]
    MissingModel { path: PathBuf },
    #[error("no Smithy service was configured")]
    MissingService,
    #[error("the operation selection cannot be empty")]
    EmptyOperations,
    #[error("no output directory was configured")]
    MissingOutputDirectory,
    #[error("code generation is not available yet")]
    GenerationUnavailable,
}
