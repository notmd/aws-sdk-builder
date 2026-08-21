use std::collections::{BTreeSet, VecDeque};

use serde_json::Value;

use crate::error::BuildError;

pub(crate) fn closure(
    shapes: &std::collections::BTreeMap<String, Value>,
    service_id: &str,
    operation_ids: &[String],
) -> Result<BTreeSet<String>, BuildError> {
    let mut queue = VecDeque::from_iter(
        std::iter::once(service_id.to_owned()).chain(operation_ids.iter().cloned()),
    );
    let mut retained = BTreeSet::new();

    while let Some(shape_id) = queue.pop_front() {
        if !retained.insert(shape_id.clone()) {
            continue;
        }
        let Some(shape) = shapes.get(&shape_id) else {
            continue;
        };
        let service_shape = shape
            .get("type")
            .and_then(Value::as_str)
            .is_some_and(|shape_type| shape_type == "service");
        let mut references = BTreeSet::new();
        collect_references(shape, shapes, service_shape, false, &mut references);
        for reference in references {
            if shapes.contains_key(&reference) {
                if !retained.contains(&reference) {
                    queue.push_back(reference);
                }
            } else if !reference.starts_with("smithy.api#") {
                return Err(BuildError::MissingShapeReference {
                    referenced_from: shape_id.clone(),
                    shape: reference,
                });
            }
        }
    }

    Ok(retained)
}

fn collect_references(
    value: &Value,
    shapes: &std::collections::BTreeMap<String, Value>,
    service_shape: bool,
    expected_shape_reference: bool,
    references: &mut BTreeSet<String>,
) {
    match value {
        Value::Array(values) => {
            for value in values {
                collect_references(
                    value,
                    shapes,
                    service_shape,
                    expected_shape_reference,
                    references,
                );
            }
        }
        Value::Object(object) => {
            for (key, value) in object {
                if service_shape && key == "operations" {
                    continue;
                }
                if key == "traits" {
                    if let Value::Object(traits) = value {
                        references.extend(
                            traits
                                .keys()
                                .filter(|trait_id| shapes.contains_key(*trait_id))
                                .cloned(),
                        );
                    }
                }
                let expected_shape_reference = matches!(
                    key.as_str(),
                    "target"
                        | "input"
                        | "output"
                        | "errors"
                        | "resource"
                        | "resources"
                        | "key"
                        | "value"
                        | "identifiers"
                );
                collect_references(
                    value,
                    shapes,
                    service_shape,
                    expected_shape_reference,
                    references,
                );
            }
        }
        Value::String(string) if expected_shape_reference => {
            references.insert(string.clone());
        }
        Value::Bool(_) | Value::Number(_) | Value::Null | Value::String(_) => {}
    }
}
