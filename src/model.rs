use crate::names;
use serde_json::Value;
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    fs,
    path::{Path, PathBuf},
};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Model {
    pub path: PathBuf,
    pub service_shape: String,
    pub shapes: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Operation {
    pub shape_id: String,
    pub name: String,
    pub module: String,
    pub feature: String,
}

#[derive(Debug, Error)]
pub enum ModelError {
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
    #[error("{path}: {message}")]
    Invalid { path: PathBuf, message: String },
}

impl Model {
    pub fn load(path: &Path) -> Result<Self, ModelError> {
        let bytes = fs::read(path).map_err(|source| ModelError::Read {
            path: path.to_owned(),
            source,
        })?;
        Self::from_bytes(path, &bytes)
    }

    pub fn from_bytes(path: &Path, bytes: &[u8]) -> Result<Self, ModelError> {
        let root = serde_json::from_slice::<Value>(bytes).map_err(|source| ModelError::Parse {
            path: path.to_owned(),
            source,
        })?;
        let shapes = root
            .get("shapes")
            .and_then(Value::as_object)
            .ok_or_else(|| ModelError::Invalid {
                path: path.to_owned(),
                message: "model must contain an object-valued shapes member".to_owned(),
            })?;
        let service_shapes = shapes
            .iter()
            .filter_map(|(id, shape)| {
                (shape.get("type").and_then(Value::as_str) == Some("service")).then_some(id.clone())
            })
            .collect::<Vec<_>>();
        let service_shape = match service_shapes.as_slice() {
            [service] => service.clone(),
            [] => {
                return Err(ModelError::Invalid {
                    path: path.to_owned(),
                    message: "model must contain exactly one service shape; found none".to_owned(),
                });
            }
            many => {
                return Err(ModelError::Invalid {
                    path: path.to_owned(),
                    message: format!(
                        "model must contain exactly one service shape; found {}",
                        many.len()
                    ),
                });
            }
        };
        Ok(Self {
            path: path.to_owned(),
            service_shape,
            shapes: shapes.clone().into_iter().collect(),
        })
    }

    pub fn operations(&self) -> Result<Vec<Operation>, ModelError> {
        let mut queue = VecDeque::from([self.service_shape.clone()]);
        let mut visited = BTreeSet::new();
        let mut operation_ids = Vec::new();
        while let Some(shape_id) = queue.pop_front() {
            if !visited.insert(shape_id.clone()) {
                continue;
            }
            let Some(shape) = self.shapes.get(&shape_id) else {
                continue;
            };
            if shape.get("type").and_then(Value::as_str) == Some("operation") {
                operation_ids.push(shape_id.clone());
            }
            queue.extend(model_relationship_targets(shape));
        }
        if operation_ids.is_empty() {
            return Err(ModelError::Invalid {
                path: self.path.clone(),
                message: format!("service {} contains no operations", self.service_shape),
            });
        }
        let mut modules = BTreeMap::new();
        let mut operations = Vec::with_capacity(operation_ids.len());
        for shape_id in operation_ids {
            let name = terminal_name(&shape_id).to_owned();
            let module = names::rust_module_name(&name);
            if let Some(previous) = modules.insert(module.clone(), shape_id.clone()) {
                return Err(ModelError::Invalid {
                    path: self.path.clone(),
                    message: format!(
                        "operation names {previous} and {shape_id} map to the same Rust module {module:?}"
                    ),
                });
            }
            operations.push(Operation {
                shape_id,
                name,
                feature: format!("op_{module}"),
                module,
            });
        }
        Ok(operations)
    }

    /// Returns deterministic groups whose operations share a modeled input or
    /// output shape. These groups exercise shared generated helpers in
    /// conformance without naming a service or operation in the codemod.
    pub fn shared_operation_groups(&self, operations: &[Operation]) -> Vec<Vec<String>> {
        let mut by_shape = BTreeMap::<String, BTreeSet<String>>::new();
        for operation in operations {
            let Some(shape) = self
                .shapes
                .get(&operation.shape_id)
                .and_then(Value::as_object)
            else {
                continue;
            };
            for field in ["input", "output"] {
                let Some(target) = shape.get(field).and_then(target_id) else {
                    continue;
                };
                by_shape
                    .entry(target.to_owned())
                    .or_default()
                    .insert(operation.feature.clone());
            }
        }
        by_shape
            .into_values()
            .filter(|group| group.len() > 1)
            .map(|group| group.into_iter().collect())
            .collect()
    }
}

fn model_relationship_targets(shape: &Value) -> Vec<String> {
    let Some(object) = shape.as_object() else {
        return Vec::new();
    };
    let mut result = Vec::new();
    for (key, value) in object {
        let follows_shape_relationship = matches!(
            key.as_str(),
            "operations"
                | "resources"
                | "collectionOperations"
                | "put"
                | "read"
                | "update"
                | "delete"
                | "list"
                | "input"
                | "output"
                | "errors"
                | "identifiers"
                | "members"
                | "member"
                | "key"
                | "value"
                | "eventStream"
        );
        if follows_shape_relationship {
            collect_targets(value, &mut result);
        }
    }
    result
}

fn collect_targets(value: &Value, result: &mut Vec<String>) {
    match value {
        Value::String(value) if value.contains('#') => result.push(value.clone()),
        Value::Array(values) => values
            .iter()
            .for_each(|value| collect_targets(value, result)),
        Value::Object(object) => {
            if let Some(target) = object.get("target") {
                collect_targets(target, result);
            }
            if object.get("type").is_some() {
                result.extend(model_relationship_targets(value));
            }
        }
        _ => {}
    }
}

fn target_id(value: &Value) -> Option<&str> {
    value.as_str().or_else(|| {
        value
            .as_object()
            .and_then(|object| object.get("target"))
            .and_then(Value::as_str)
    })
}

fn terminal_name(shape_id: &str) -> &str {
    shape_id.rsplit_once('#').map_or(shape_id, |(_, name)| name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discovers_object_operation_targets_in_model_order() {
        let model = Model::load(Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/object-operation-model.json"
        )))
        .unwrap();
        let operations = model.operations().unwrap();
        assert_eq!(
            operations
                .iter()
                .map(|operation| operation.name.as_str())
                .collect::<Vec<_>>(),
            ["GetThing", "DeleteThing"]
        );
        assert_eq!(operations[0].feature, "op_get_thing");
    }

    #[test]
    fn rejects_multiple_service_shapes() {
        let error = Model::from_bytes(
            Path::new("bad.json"),
            br#"{"shapes":{"a#A":{"type":"service"},"a#B":{"type":"service"}}}"#,
        )
        .unwrap_err();
        assert!(error.to_string().contains("exactly one service shape"));
    }
}
