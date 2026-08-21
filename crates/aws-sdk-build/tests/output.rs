use std::{fs, path::Path};

use aws_sdk_build::output::install;
use serde_json::Value;
use tempfile::tempdir;

fn generated_projection(root: &Path) {
    let source = root.join("output/aws-sdk/src");
    fs::create_dir_all(&source).unwrap();
    fs::write(
        source.join("lib.rs"),
        "#![allow(deprecated)]\n//! generated docs\npub mod client;\n",
    )
    .unwrap();
    fs::write(source.join("client.rs"), "pub struct Client;\n").unwrap();
    fs::write(
        root.join("output/aws-sdk/Cargo.toml"),
        "[package]\nname = \"generated-sdk\"\n",
    )
    .unwrap();
}

#[test]
fn install_writes_include_root_manifest_and_rust_sources() {
    let generated = tempdir().unwrap();
    generated_projection(generated.path());
    let output = tempdir().unwrap();

    let report = install(
        &generated.path().join("output"),
        output.path(),
        "example#Service",
        &["GetThing".to_owned()],
    )
    .unwrap();

    assert_eq!(report.generated_root, output.path().join("generated"));
    assert_eq!(
        report.manifest,
        output.path().join("aws_sdk_build_manifest.json")
    );
    assert_eq!(
        fs::read_to_string(output.path().join("aws_sdk.rs")).unwrap(),
        "include!(concat!(env!(\"OUT_DIR\"), \"/generated/src/lib.rs\"));\n"
    );
    assert_eq!(
        fs::read_to_string(output.path().join("generated/src/client.rs")).unwrap(),
        "pub struct Client;\n"
    );
    assert_eq!(
        fs::read_to_string(output.path().join("generated/src/lib.rs")).unwrap(),
        "pub mod client;\n"
    );

    let manifest: Value = serde_json::from_slice(
        &fs::read(output.path().join("aws_sdk_build_manifest.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(manifest["service"], "example#Service");
    assert_eq!(manifest["operations"][0], "GetThing");
    assert_eq!(manifest["files"][0], "src/client.rs");
}

#[test]
fn failed_install_does_not_replace_existing_include_root() {
    let generated = tempdir().unwrap();
    let output = tempdir().unwrap();
    let include_root = output.path().join("aws_sdk.rs");
    fs::write(&include_root, "old generated output\n").unwrap();

    let error = install(
        &generated.path().join("missing-output"),
        output.path(),
        "example#Service",
        &["GetThing".to_owned()],
    )
    .unwrap_err();

    assert!(error.to_string().contains("generated Rust projection"));
    assert_eq!(
        fs::read_to_string(include_root).unwrap(),
        "old generated output\n"
    );
}
