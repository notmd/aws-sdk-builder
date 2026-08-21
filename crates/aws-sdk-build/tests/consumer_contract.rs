use std::{fs, path::Path};

#[test]
fn consumer_build_script_uses_only_service_operation_selection() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/generated-consumer");
    let build_script = fs::read_to_string(root.join("build.rs")).unwrap();

    assert!(build_script.contains("configure"));
    assert!(build_script.contains(".add(\"s3\""));
    assert!(build_script.contains(".compile"));
    for obsolete in [
        ".model(",
        ".service(",
        ".operations(",
        ".out_dir(",
        ".smithy(",
    ] {
        assert!(
            !build_script.contains(obsolete),
            "obsolete API in {obsolete}"
        );
    }
}

#[test]
fn consumer_uses_the_include_facade() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/generated-consumer");
    let source = fs::read_to_string(root.join("src/lib.rs")).unwrap();
    assert!(source.contains("aws_sdk_build::include_sdk!();"));
    assert!(source.contains("aws_sdk_s3::operation::abort_multipart_upload::AbortMultipartUpload"));
}
