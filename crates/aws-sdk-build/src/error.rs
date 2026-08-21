use std::{io, path::PathBuf};

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
    #[error("failed to read Smithy model {path}: {source}")]
    ModelRead {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("failed to parse Smithy model {path}: {source}")]
    ModelParse {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },
    #[error("invalid Smithy model {path}: {message}")]
    InvalidModel { path: PathBuf, message: String },
    #[error("shape {shape} is defined more than once while loading {path}")]
    DuplicateShape { path: PathBuf, shape: String },
    #[error("failed to write pruned Smithy model {path}: {source}")]
    ModelWrite {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("Smithy service was not found: {service}")]
    ServiceNotFound { service: String },
    #[error("operation {operation} was not found on service {service}")]
    OperationNotFound { service: String, operation: String },
    #[error("shape {shape} referenced from {referenced_from} was not found in the model")]
    MissingShapeReference {
        referenced_from: String,
        shape: String,
    },
    #[error("Smithy executable was not found; searched: {searched:?}")]
    SmithyExecutableNotFound { searched: Vec<String> },
    #[error("failed to start Smithy command {command}: {source}")]
    SmithySpawn {
        command: String,
        #[source]
        source: io::Error,
    },
    #[error("Smithy command failed ({status}): {command}\nstdout:\n{stdout}\nstderr:\n{stderr}")]
    SmithyToolFailed {
        command: String,
        status: String,
        stdout: String,
        stderr: String,
    },
}
