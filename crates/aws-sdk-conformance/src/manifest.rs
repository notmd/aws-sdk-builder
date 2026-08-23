use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeSet,
    fs,
    path::{Component, Path, PathBuf},
};

pub const DEFAULT_PATH: &str = "services-manifest.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServicesManifest {
    pub schema_version: u32,
    pub upstream: Upstream,
    pub roots: Roots,
    pub comparison: Comparison,
    pub services: Vec<ServiceManifest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Upstream {
    pub repository: String,
    pub commit: String,
    pub sdk_root: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Roots {
    pub reference: String,
    pub generated: String,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comparison {
    pub exclude: Exclusions,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Exclusions {
    pub files: Vec<String>,
    pub directories: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceManifest {
    pub key: String,
    pub upstream_path: String,
    pub model_path: String,
    pub model_destination: String,
    pub reference_path: String,
    pub generated_path: String,
    pub crate_name: String,
    pub module_name: String,
    pub sdk_version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference_files: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generated_files: Option<usize>,
}

impl ServicesManifest {
    pub fn load(path: &Path) -> Result<Self, String> {
        let bytes = fs::read(path).map_err(|error| format!("{}: {error}", path.display()))?;
        let manifest = serde_json::from_slice::<Self>(&bytes)
            .map_err(|error| format!("{}: invalid JSON: {error}", path.display()))?;
        manifest.validate()?;
        Ok(manifest)
    }

    pub fn write_atomic(&self, path: &Path) -> Result<(), String> {
        let parent = path.parent().unwrap_or_else(|| Path::new("."));
        fs::create_dir_all(parent).map_err(|error| format!("{}: {error}", parent.display()))?;
        let temporary = path.with_extension("json.tmp");
        let contents = serde_json::to_string_pretty(self)
            .map_err(|error| format!("{}: cannot serialize manifest: {error}", path.display()))?;
        fs::write(&temporary, format!("{contents}\n"))
            .map_err(|error| format!("{}: {error}", temporary.display()))?;
        if let Err(error) = fs::rename(&temporary, path) {
            let _ = fs::remove_file(&temporary);
            return Err(format!("{}: {error}", path.display()));
        }
        Ok(())
    }

    pub fn root_path(&self, manifest_path: &Path, root: &str) -> PathBuf {
        manifest_path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join(root)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.schema_version != 1 {
            return Err(format!(
                "unsupported services manifest schema version: {}",
                self.schema_version
            ));
        }
        if self.upstream.repository.trim().is_empty() {
            return Err("upstream repository must not be empty".to_owned());
        }
        validate_commit(&self.upstream.commit)?;
        validate_relative_path("upstream.sdk_root", &self.upstream.sdk_root)?;
        validate_relative_path("roots.reference", &self.roots.reference)?;
        validate_relative_path("roots.generated", &self.roots.generated)?;
        validate_relative_path("roots.summary", &self.roots.summary)?;
        self.comparison.exclude.validate()?;

        if self.services.is_empty() {
            return Err("services manifest must list at least one service".to_owned());
        }
        let mut keys = BTreeSet::new();
        let mut references = BTreeSet::new();
        let mut generated = BTreeSet::new();
        let sdk_prefix = format!("{}/", self.upstream.sdk_root.trim_end_matches('/'));
        for service in &self.services {
            if !keys.insert(service.key.clone()) {
                return Err(format!("duplicate service key: {}", service.key));
            }
            validate_service_key(&service.key)?;
            validate_relative_path("service.upstream_path", &service.upstream_path)?;
            if !service.upstream_path.starts_with(&sdk_prefix) {
                return Err(format!(
                    "service upstream path is outside sdk root: {}",
                    service.upstream_path
                ));
            }
            validate_relative_path("service.model_path", &service.model_path)?;
            validate_relative_path("service.model_destination", &service.model_destination)?;
            validate_relative_path("service.reference_path", &service.reference_path)?;
            validate_relative_path("service.generated_path", &service.generated_path)?;
            if !references.insert(service.reference_path.clone()) {
                return Err(format!(
                    "duplicate reference path: {}",
                    service.reference_path
                ));
            }
            if !generated.insert(service.generated_path.clone()) {
                return Err(format!(
                    "duplicate generated path: {}",
                    service.generated_path
                ));
            }
            if service.crate_name.trim().is_empty()
                || service.module_name.trim().is_empty()
                || service.sdk_version.trim().is_empty()
            {
                return Err(format!("service metadata is incomplete: {}", service.key));
            }
        }
        if self
            .services
            .windows(2)
            .any(|services| services[0].key >= services[1].key)
        {
            return Err("services must be sorted by key".to_owned());
        }
        Ok(())
    }
}

impl Exclusions {
    pub fn validate(&self) -> Result<(), String> {
        for file in &self.files {
            if file.is_empty() || file.contains('/') || file.contains('\\') {
                return Err(format!("invalid excluded file name: {file:?}"));
            }
        }
        for directory in &self.directories {
            if directory.is_empty() || directory.contains('/') || directory.contains('\\') {
                return Err(format!("invalid excluded directory name: {directory:?}"));
            }
        }
        Ok(())
    }

    pub fn excludes(&self, relative: &Path) -> bool {
        let components = relative
            .components()
            .filter_map(|component| match component {
                Component::Normal(value) => value.to_str(),
                _ => None,
            })
            .collect::<Vec<_>>();
        if components
            .last()
            .is_some_and(|file| self.files.iter().any(|item| item == file))
        {
            return true;
        }
        components
            .iter()
            .any(|directory| self.directories.iter().any(|item| item == directory))
    }
}

pub fn validate_registered_services(manifest: &ServicesManifest) -> Result<(), String> {
    for service in &manifest.services {
        let metadata = aws_sdk_builder::registry::lookup(&service.key)
            .map_err(|error| format!("{}: {error}", service.key))?;
        if metadata.crate_name != service.crate_name
            || metadata.module_name != service.module_name
            || metadata.sdk_version != Some(service.sdk_version.as_str())
        {
            return Err(format!(
                "manifest metadata does not match registered service: {}",
                service.key
            ));
        }
    }
    Ok(())
}

pub fn validate_commit(commit: &str) -> Result<(), String> {
    if commit.len() != 40 || !commit.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(format!(
            "upstream commit must be a full 40-character hexadecimal SHA: {commit}"
        ));
    }
    Ok(())
}

fn validate_service_key(key: &str) -> Result<(), String> {
    if key.is_empty() || key == "." || key == ".." || key.contains('/') || key.contains('\\') {
        return Err(format!("invalid service key: {key:?}"));
    }
    Ok(())
}

fn validate_relative_path(label: &str, value: &str) -> Result<(), String> {
    let path = Path::new(value);
    if value.is_empty() || path.is_absolute() {
        return Err(format!(
            "{label} must be a non-empty relative path: {value:?}"
        ));
    }
    for component in path.components() {
        if matches!(
            component,
            Component::ParentDir | Component::CurDir | Component::RootDir
        ) {
            return Err(format!("{label} contains unsafe path component: {value:?}"));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_manifest() -> ServicesManifest {
        ServicesManifest {
            schema_version: 1,
            upstream: Upstream {
                repository: "https://example.invalid/aws-sdk-rust".to_owned(),
                commit: "0123456789abcdef0123456789abcdef01234567".to_owned(),
                sdk_root: "sdk".to_owned(),
            },
            roots: Roots {
                reference: "conformance/reference".to_owned(),
                generated: "conformance/generated".to_owned(),
                summary: "conformance/summary.md".to_owned(),
            },
            comparison: Comparison {
                exclude: Exclusions {
                    files: vec!["Cargo.toml".to_owned(), "README.md".to_owned()],
                    directories: vec!["tests".to_owned(), "benches".to_owned()],
                },
            },
            services: vec![ServiceManifest {
                key: "s3".to_owned(),
                upstream_path: "sdk/s3".to_owned(),
                model_path: "aws-models/s3.json".to_owned(),
                model_destination: "crates/aws-sdk-builder-s3/model.json".to_owned(),
                reference_path: "s3".to_owned(),
                generated_path: "s3".to_owned(),
                crate_name: "aws-sdk-s3".to_owned(),
                module_name: "aws_sdk_s3".to_owned(),
                sdk_version: "1.0.0".to_owned(),
                reference_files: None,
                generated_files: None,
            }],
        }
    }

    #[test]
    fn validates_pinned_manifest_and_exclusions() {
        let manifest = sample_manifest();
        manifest.validate().unwrap();
        assert!(
            manifest
                .comparison
                .exclude
                .excludes(Path::new("Cargo.toml"))
        );
        assert!(
            manifest
                .comparison
                .exclude
                .excludes(Path::new("tests/data.json"))
        );
        assert!(
            !manifest
                .comparison
                .exclude
                .excludes(Path::new("src/lib.rs"))
        );
    }

    #[test]
    fn rejects_short_or_unsorted_metadata() {
        let mut manifest = sample_manifest();
        manifest.upstream.commit = "short".to_owned();
        assert!(manifest.validate().is_err());

        let mut manifest = sample_manifest();
        manifest.services.push(ServiceManifest {
            key: "dynamodb".to_owned(),
            upstream_path: "sdk/dynamodb".to_owned(),
            model_path: "aws-models/dynamodb.json".to_owned(),
            model_destination: "crates/aws-sdk-builder-dynamodb/model.json".to_owned(),
            reference_path: "dynamodb".to_owned(),
            generated_path: "dynamodb".to_owned(),
            crate_name: "aws-sdk-dynamodb".to_owned(),
            module_name: "aws_sdk_dynamodb".to_owned(),
            sdk_version: "1.0.0".to_owned(),
            reference_files: None,
            generated_files: None,
        });
        assert!(manifest.validate().is_err());
    }
}
