use serde::Deserialize;
use std::{
    collections::BTreeSet,
    fs,
    path::{Component, Path, PathBuf},
};
use thiserror::Error;

pub const DEFAULT_PATH: &str = "services-manifest.json";

#[derive(Debug, Clone, Deserialize)]
pub struct Manifest {
    pub schema_version: u32,
    pub services: Vec<ServiceManifest>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServiceManifest {
    pub key: String,
    pub repository: String,
    pub revision: String,
    pub upstream_path: String,
    pub model_path: String,
    pub output_dir: String,
    pub package_name: String,
    pub library_name: String,
}

#[derive(Debug, Error)]
pub enum ManifestError {
    #[error("{path}: {source}")]
    Read {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("{path}: invalid JSON: {source}")]
    Parse {
        path: PathBuf,
        source: serde_json::Error,
    },
    #[error("invalid services manifest: {0}")]
    Invalid(String),
}

impl Manifest {
    pub fn load(path: &Path) -> Result<Self, ManifestError> {
        let bytes = fs::read(path).map_err(|source| ManifestError::Read {
            path: path.to_owned(),
            source,
        })?;
        let manifest =
            serde_json::from_slice::<Self>(&bytes).map_err(|source| ManifestError::Parse {
                path: path.to_owned(),
                source,
            })?;
        manifest.validate()?;
        Ok(manifest)
    }

    pub fn validate(&self) -> Result<(), ManifestError> {
        if self.schema_version != 2 {
            return Err(ManifestError::Invalid(format!(
                "unsupported schema version {}; expected 2",
                self.schema_version
            )));
        }
        if self.services.is_empty() {
            return Err(ManifestError::Invalid(
                "services must not be empty".to_owned(),
            ));
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
            if !service.repository.starts_with("https://github.com/") {
                return Err(ManifestError::Invalid(format!(
                    "{} repository must be an HTTPS GitHub URL",
                    service.key
                )));
            }
            validate_revision(&service.revision)?;
            for (label, value) in [
                ("upstream_path", &service.upstream_path),
                ("model_path", &service.model_path),
                ("output_dir", &service.output_dir),
            ] {
                validate_relative_path(&format!("{} {label}", service.key), value)?;
            }
            if service.package_name.is_empty() || service.library_name.is_empty() {
                return Err(ManifestError::Invalid(format!(
                    "{} package_name and library_name must not be empty",
                    service.key
                )));
            }
            if !outputs.insert(&service.output_dir) {
                return Err(ManifestError::Invalid(format!(
                    "duplicate output directory: {}",
                    service.output_dir
                )));
            }
        }
        if self
            .services
            .windows(2)
            .any(|pair| pair[0].key >= pair[1].key)
        {
            return Err(ManifestError::Invalid(
                "services must be sorted by key".to_owned(),
            ));
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
        return Err(ManifestError::Invalid(format!(
            "invalid service key: {key:?}"
        )));
    }
    Ok(())
}

fn validate_relative_path(label: &str, value: &str) -> Result<(), ManifestError> {
    let path = Path::new(value);
    if value.is_empty()
        || path.is_absolute()
        || path.components().any(|component| {
            matches!(
                component,
                Component::ParentDir | Component::CurDir | Component::RootDir
            )
        })
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
            repository: "https://github.com/example/aws-sdk-rust".to_owned(),
            revision: "0123456789abcdef0123456789abcdef01234567".to_owned(),
            upstream_path: format!("sdk/{key}"),
            model_path: format!("models/{key}.json"),
            output_dir: format!("crates/{key}"),
            package_name: format!("aws-sdk-{key}"),
            library_name: format!("aws_sdk_{key}"),
        }
    }

    #[test]
    fn rejects_unsafe_or_ambiguous_manifest_entries() {
        let mut manifest = Manifest {
            schema_version: 2,
            services: vec![service("s3")],
        };
        manifest.validate().unwrap();
        manifest.services[0].revision = "short".to_owned();
        assert!(manifest.validate().is_err());
        manifest.services[0].revision = "0123456789abcdef0123456789abcdef01234567".to_owned();
        manifest.services[0].output_dir = "../outside".to_owned();
        assert!(manifest.validate().is_err());
    }
}
