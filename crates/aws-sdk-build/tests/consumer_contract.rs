use std::{fs, path::Path};

#[test]
fn consumer_build_script_only_selects_operations() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/generated-consumer");
    let build_script = fs::read_to_string(root.join("build.rs")).unwrap();

    for call in ["configure", "model", "service", "operations", "compile"] {
        assert!(build_script.contains(call), "build.rs is missing {call}");
    }
    assert!(!build_script.contains("struct Client"));
    assert!(!build_script.contains("http::Request"));
}

#[test]
fn consumer_includes_generated_root() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/generated-consumer");
    let source = fs::read_to_string(root.join("src/lib.rs")).unwrap();

    assert!(source.contains("OUT_DIR"));
    assert!(source.contains("aws_sdk.rs"));
}
