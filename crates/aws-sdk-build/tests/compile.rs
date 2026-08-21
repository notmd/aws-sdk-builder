#![cfg(unix)]

use std::{
    fs,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
};

use aws_sdk_build::configure;
use tempfile::tempdir;

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures")
        .join(name)
}

#[test]
fn compile_prunes_model_invokes_explicit_smithy_and_installs_output() {
    let tool_directory = tempdir().unwrap();
    let tool = tool_directory.path().join("fake-smithy");
    fs::write(
        &tool,
        r##"#!/bin/sh
if grep -q 'DeleteThing' model.json; then
  echo 'unselected operation leaked into pruned model' >&2
  exit 12
fi
mkdir -p output/aws-sdk/src
printf '%s\n' 'pub struct Client;' > output/aws-sdk/src/lib.rs
printf '%s\n' 'pub struct GetThing;' > output/aws-sdk/src/get_thing.rs
"##,
    )
    .unwrap();
    let mut permissions = fs::metadata(&tool).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&tool, permissions).unwrap();

    let output = tempdir().unwrap();
    let report = configure()
        .model(fixture("selection-model.json"))
        .service("example#Service")
        .operations(["GetThing"])
        .out_dir(output.path())
        .smithy(&tool)
        .compile()
        .unwrap();

    assert_eq!(report.generated_root, output.path().join("generated"));
    assert_eq!(
        fs::read_to_string(output.path().join("generated/src/get_thing.rs")).unwrap(),
        "pub struct GetThing;\n"
    );
    let manifest = fs::read_to_string(output.path().join("aws_sdk_build_manifest.json")).unwrap();
    assert!(manifest.contains("GetThing"));
    assert!(!manifest.contains("DeleteThing"));
}
