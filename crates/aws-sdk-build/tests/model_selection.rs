use std::{
    fs,
    path::{Path, PathBuf},
};

use aws_sdk_build::model::load;
use aws_sdk_build::BuildError;
use tempfile::tempdir;

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures")
        .join(name)
}

#[test]
fn selection_keeps_transitive_shapes_but_excludes_unselected_operations() {
    let model = load(&fixture("selection-model.json")).unwrap();
    let selection = model
        .select("example#Service", Some(&["GetThing".into()]))
        .unwrap();
    let shapes = selection.document()["shapes"].as_object().unwrap();

    assert!(shapes.contains_key("example#Service"));
    assert!(shapes.contains_key("example#GetThing"));
    assert!(shapes.contains_key("example#GetThingInput"));
    assert!(shapes.contains_key("example#GetThingOutput"));
    assert!(shapes.contains_key("example#SharedError"));
    assert!(shapes.contains_key("smithy.api#String"));
    assert!(!shapes.contains_key("example#DeleteThing"));
    assert!(!shapes.contains_key("example#DeleteThingInput"));
    assert!(!shapes.contains_key("example#DeleteThingOutput"));
    assert!(!shapes.contains_key("example#UnreachableShape"));
}

#[test]
fn omitted_operations_selects_every_service_operation() {
    let model = load(&fixture("selection-model.json")).unwrap();
    let selection = model.select("example#Service", None).unwrap();

    assert_eq!(selection.operations(), &["DeleteThing", "GetThing"]);
    let shapes = selection.document()["shapes"].as_object().unwrap();
    assert!(shapes.contains_key("example#DeleteThing"));
    assert!(shapes.contains_key("example#GetThing"));
}

#[test]
fn unknown_service_reports_the_requested_id() {
    let model = load(&fixture("selection-model.json")).unwrap();
    let error = model.select("example#Missing", None).unwrap_err();

    assert!(
        matches!(error, BuildError::ServiceNotFound { service } if service == "example#Missing")
    );
}

#[test]
fn unknown_operation_reports_the_requested_name() {
    let model = load(&fixture("selection-model.json")).unwrap();
    let error = model
        .select("example#Service", Some(&["Missing".into()]))
        .unwrap_err();

    assert!(
        matches!(error, BuildError::OperationNotFound { operation, .. } if operation == "Missing")
    );
}

#[test]
fn missing_non_prelude_reference_reports_the_reference_chain_start() {
    let model = load(&fixture("missing-reference-model.json")).unwrap();
    let error = model
        .select("example#Service", Some(&["GetThing".into()]))
        .unwrap_err();

    assert!(matches!(
        error,
        BuildError::MissingShapeReference {
            referenced_from,
            shape
        } if referenced_from == "example#GetThing" && shape == "example#MissingInput"
    ));
}

#[test]
fn directory_input_merges_files_and_serializes_deterministically() {
    let directory = tempdir().unwrap();
    fs::write(
        directory.path().join("a.json"),
        r#"{
            "smithy": "1.0",
            "shapes": {
                "example#Service": {
                    "type": "service",
                    "version": "2026-01-01",
                    "operations": ["example#GetThing"]
                }
            }
        }"#,
    )
    .unwrap();
    fs::write(
        directory.path().join("z.json"),
        r#"{
            "shapes": {
                "example#GetThing": {"type": "operation"}
            }
        }"#,
    )
    .unwrap();

    let model = load(directory.path()).unwrap();
    let first = model.select("example#Service", None).unwrap();
    let second = load(directory.path())
        .unwrap()
        .select("example#Service", None)
        .unwrap();

    assert!(first.document()["shapes"].get("example#GetThing").is_some());
    assert_eq!(
        serde_json::to_vec(first.document()).unwrap(),
        serde_json::to_vec(second.document()).unwrap()
    );
}

#[test]
fn directory_input_rejects_duplicate_shape_ids() {
    let directory = tempdir().unwrap();
    let document = r#"{"shapes":{"example#Duplicate":{"type":"string"}}}"#;
    fs::write(directory.path().join("a.json"), document).unwrap();
    fs::write(directory.path().join("b.json"), document).unwrap();

    let error = load(directory.path()).unwrap_err();
    assert!(
        matches!(error, BuildError::DuplicateShape { shape, .. } if shape == "example#Duplicate")
    );
}
