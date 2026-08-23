use std::{
    fs,
    path::{Path, PathBuf},
};

use aws_sdk_builder::registry;
use sha2::{Digest, Sha256};

const SERVICES: &[&str] = &[
    "dynamodb", "iam", "kms", "lambda", "s3", "sns", "sqs", "sts",
];

fn files_below(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let mut pending = vec![root.to_owned()];
    while let Some(directory) = pending.pop() {
        for entry in fs::read_dir(directory).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                pending.push(path);
            } else {
                files.push(path);
            }
        }
    }
    files.sort();
    files
}

#[test]
fn core_has_no_service_models() {
    let core = Path::new(env!("CARGO_MANIFEST_DIR"));
    assert!(!core.join("models").exists());
    assert!(!core.join("models-manifest.json").exists());
}

#[test]
fn every_service_package_contains_only_one_model_and_glue() {
    let core = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace = core.parent().unwrap();
    for service in SERVICES {
        let package = workspace.join(format!("aws-sdk-builder-{service}"));
        let relative_files = files_below(&package)
            .into_iter()
            .map(|path| path.strip_prefix(&package).unwrap().to_owned())
            .collect::<Vec<_>>();
        assert_eq!(
            relative_files,
            vec![
                PathBuf::from("Cargo.toml"),
                PathBuf::from("model.json"),
                PathBuf::from("src/lib.rs"),
            ],
            "unexpected files in {service} service package"
        );

        let model_path = package.join("model.json");
        let model = fs::read(&model_path).unwrap();
        let json: serde_json::Value = serde_json::from_slice(&model).unwrap();
        assert!(json.get("shapes").is_some_and(serde_json::Value::is_object));
        let metadata = registry::lookup(service).unwrap();
        let checksum = format!("{:x}", Sha256::digest(&model));
        assert_eq!(checksum, metadata.model_sha256, "checksum for {service}");
    }
}
