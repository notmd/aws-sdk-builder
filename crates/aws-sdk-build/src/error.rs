use std::{io, path::PathBuf};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum BuildError {
    #[error("no AWS services were selected; call add(service, operations) before compile()")]
    NoServices,
    #[error("Cargo did not provide {variable}; compile() must run from a Cargo build script")]
    MissingCargoEnvironment { variable: &'static str },
    #[error("unknown AWS service `{service}`; lookup used packaged registry {registry}")]
    UnknownService { service: String, registry: String },
    #[error(
        "unknown operation `{operation}` for service `{service}`; lookup used packaged model {model}"
    )]
    UnknownOperation {
        service: String,
        operation: String,
        model: String,
    },
    #[error("invalid packaged model {model}: {message}")]
    InvalidModel { model: String, message: String },
    #[error("failed to parse packaged model {model}: {source}")]
    ModelParse {
        model: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("shape {shape} referenced from {referenced_from} is not present in model {model}")]
    MissingShapeReference {
        model: String,
        referenced_from: String,
        shape: String,
    },
    #[error("failed to create generator staging directory below {path}: {source}")]
    StageCreate { path: PathBuf, source: io::Error },
    #[error("failed to read generated source {path}: {source}")]
    SourceRead { path: PathBuf, source: io::Error },
    #[error("failed to write generated output {path}: {source}")]
    OutputWrite { path: PathBuf, source: io::Error },
    #[error("generated Rust source {path} is invalid: {message}")]
    InvalidGeneratedRust { path: PathBuf, message: String },
    #[error("failed to install generated output {path}: {source}")]
    Install { path: PathBuf, source: io::Error },
}
