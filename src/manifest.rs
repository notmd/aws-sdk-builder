use serde::Deserialize;
use std::{
    collections::BTreeSet,
    fs,
    path::{Component, Path, PathBuf},
};
use thiserror::Error;

pub const DEFAULT_PATH: &str = "services-manifest.json";

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub repository: String,
    pub revision: String,
    pub services: Vec<ServiceManifest>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServiceManifest {
    pub key: String,
    pub upstream_path: String,
    pub model_path: String,
    pub output_dir: String,
}

impl ServiceManifest {
    pub fn package_name(&self) -> String {
        format!("aws-sdk-{}", self.key)
    }

    pub fn library_name(&self) -> String {
        format!("aws_sdk_{}", self.key.replace('-', "_"))
    }
}

#[derive(Debug, Error)]
pub enum ManifestError {
    #[error("{path}: {source}")]
    Read { path: PathBuf, source: std::io::Error },
    #[error("{path}: invalid JSON: {source}")]
    Parse { path: PathBuf, source: serde_json::Error },
    #[error("invalid services manifest: {0}")]
    Invalid(String),
}

impl Manifest {
    pub fn load(path: &Path) -> Result<Self, ManifestError> {
        let bytes = fs::read(path).map_err(|source| ManifestError::Read {
            path: path.to_owned(),
            source,
        })?;
        let manifest = serde_json::from_slice::<Self>(&bytes).map_err(|source| ManifestError::Parse {
            path: path.to_owned(),
            source,
        })?;
        manifest.validate()?;
        Ok(manifest)
    }

    pub fn validate(&self) -> Result<(), ManifestError> {
        if !self.repository.starts_with("https://github.com/") {
            return Err(ManifestError::Invalid(
                "repository must be an HTTPS GitHub URL".to_owned(),
            ));
        }
        validate_revision(&self.revision)?;
        if self.services.is_empty() {
            return Err(ManifestError::Invalid("services must not be empty".to_owned()));
        }
        let mut keys = BTreeSet::new();
        let mut outputs = BTreeSet::new();
        for service in &self.services {
            validate_key(&service.key)?;
            if !keys.insert(&service.key) {
                return Err(ManifestError::Invalid(format!(
                    "duplicate service key: {}",
                    service.key
                )));
            }
            for (label, value) in [
                ("upstream_path", &service.upstream_path),
                ("model_path", &service.model_path),
                ("output_dir", &service.output_dir),
            ] {
                validate_relative_path(&format!("{} {label}", service.key), value)?;
            }
            if !outputs.insert(&service.output_dir) {
                return Err(ManifestError::Invalid(format!(
                    "duplicate output directory: {}",
                    service.output_dir
                )));
            }
        }
        if self.services.windows(2).any(|pair| pair[0].key >= pair[1].key) {
            return Err(ManifestError::Invalid("services must be sorted by key".to_owned()));
        }
        Ok(())
    }

    pub fn repository_root(manifest_path: &Path) -> PathBuf {
        manifest_path
            .parent()
            .filter(|path| !path.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new("."))
            .to_owned()
    }
}

pub fn validate_revision(revision: &str) -> Result<(), ManifestError> {
    if revision.len() != 40 || !revision.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(ManifestError::Invalid(format!(
            "revision must be a full 40-character hexadecimal SHA: {revision}"
        )));
    }
    Ok(())
}

fn validate_key(key: &str) -> Result<(), ManifestError> {
    if key.is_empty() || key == "." || key == ".." || key.contains('/') || key.contains('\\') {
        return Err(ManifestError::Invalid(format!("invalid service key: {key:?}")));
    }
    Ok(())
}

fn validate_relative_path(label: &str, value: &str) -> Result<(), ManifestError> {
    let path = Path::new(value);
    if value.is_empty()
        || path.is_absolute()
        || path
            .components()
            .any(|component| matches!(component, Component::ParentDir | Component::CurDir | Component::RootDir))
    {
        return Err(ManifestError::Invalid(format!(
            "{label} must be a safe relative path: {value:?}"
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn service(key: &str) -> ServiceManifest {
        ServiceManifest {
            key: key.to_owned(),
            upstream_path: format!("sdk/{key}"),
            model_path: format!("aws-models/{key}.json"),
            output_dir: format!("crates/{key}"),
        }
    }

    #[test]
    fn rejects_unsafe_or_ambiguous_manifest_entries() {
        let mut manifest = Manifest {
            repository: "https://github.com/example/aws-sdk-rust".to_owned(),
            revision: "0123456789abcdef0123456789abcdef01234567".to_owned(),
            services: vec![service("s3")],
        };
        manifest.validate().unwrap();
        manifest.revision = "short".to_owned();
        assert!(manifest.validate().is_err());
        manifest.revision = "0123456789abcdef0123456789abcdef01234567".to_owned();
        manifest.services[0].output_dir = "../outside".to_owned();
        assert!(manifest.validate().is_err());
    }

    #[test]
    fn derives_cargo_names_from_service_key() {
        let service = service("example-service");
        assert_eq!(service.package_name(), "aws-sdk-example-service");
        assert_eq!(service.library_name(), "aws_sdk_example_service");
    }
}
