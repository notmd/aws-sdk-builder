use std::path::PathBuf;

use crate::error::BuildError;

pub(crate) const DEFAULT_RUST_CLIENT_CODEGEN: &str =
    "software.amazon.smithy.rust:codegen-aws-sdk:0.1.25";

#[derive(Debug, Clone)]
pub struct Builder {
    pub(crate) model: Option<PathBuf>,
    pub(crate) service: Option<String>,
    pub(crate) operations: Option<Vec<String>>,
    pub(crate) out_dir: Option<PathBuf>,
    pub(crate) smithy: Option<PathBuf>,
    pub(crate) rust_client_codegen: String,
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            model: None,
            service: None,
            operations: None,
            out_dir: None,
            smithy: None,
            rust_client_codegen: DEFAULT_RUST_CLIENT_CODEGEN.to_owned(),
        }
    }
}

impl Builder {
    pub fn model<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.model = Some(path.into());
        self
    }

    pub fn service(mut self, service: impl Into<String>) -> Self {
        self.service = Some(service.into());
        self
    }

    pub fn operations<I, S>(mut self, operations: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.operations = Some(operations.into_iter().map(Into::into).collect());
        self
    }

    pub fn out_dir<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.out_dir = Some(path.into());
        self
    }

    pub fn smithy<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.smithy = Some(path.into());
        self
    }

    pub fn rust_client_codegen(mut self, coordinate: impl Into<String>) -> Self {
        self.rust_client_codegen = coordinate.into();
        self
    }

    pub(crate) fn validate(&self) -> Result<(), BuildError> {
        if self.service.is_none() {
            return Err(BuildError::MissingService);
        }
        if matches!(self.operations.as_deref(), Some([])) {
            return Err(BuildError::EmptyOperations);
        }
        let path = self
            .model
            .as_deref()
            .ok_or(BuildError::MissingModelConfiguration)?;
        if !path.exists() {
            return Err(BuildError::MissingModel {
                path: path.to_path_buf(),
            });
        }
        Ok(())
    }
}
