use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

use serde_json::{Map, Value};

use crate::{error::BuildError, prune};

#[derive(Debug, Clone)]
pub struct Model {
    root: Value,
    shapes: BTreeMap<String, Value>,
}

#[derive(Debug, Clone)]
pub struct Selection {
    document: Value,
    operations: Vec<String>,
}

pub fn load(path: &Path) -> Result<Model, BuildError> {
    if path.is_dir() {
        load_directory(path)
    } else {
        load_file(path)
    }
}

impl Model {
    pub fn select(
        &self,
        service_id: &str,
        operations: Option<&[String]>,
    ) -> Result<Selection, BuildError> {
        let service = self
            .shapes
            .get(service_id)
            .filter(|shape| shape.get("type").and_then(Value::as_str) == Some("service"))
            .ok_or_else(|| BuildError::ServiceNotFound {
                service: service_id.to_owned(),
            })?;
        let declared_operations = service
            .get("operations")
            .and_then(Value::as_array)
            .ok_or_else(|| BuildError::InvalidModel {
                path: PathBuf::from("<loaded model>"),
                message: format!("service {service_id} has no operations array"),
            })?;
        let declared_operations = declared_operations
            .iter()
            .filter_map(Value::as_str)
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>();

        let selected_ids = match operations {
            None => declared_operations,
            Some([]) => return Err(BuildError::EmptyOperations),
            Some(requested) => requested
                .iter()
                .map(|operation| {
                    find_operation(&self.shapes, &declared_operations, operation).ok_or_else(|| {
                        BuildError::OperationNotFound {
                            service: service_id.to_owned(),
                            operation: operation.clone(),
                        }
                    })
                })
                .collect::<Result<Vec<_>, _>>()?,
        };

        let selected_names = selected_ids
            .iter()
            .filter_map(|operation_id| operation_id.rsplit('#').next())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>();
        let retained = prune::closure(&self.shapes, service_id, &selected_ids)?;
        let shapes = retained
            .into_iter()
            .filter_map(|shape_id| {
                self.shapes
                    .get(&shape_id)
                    .cloned()
                    .map(|shape| (shape_id, shape))
            })
            .collect::<Map<_, _>>();

        let mut document = self.root.clone();
        document["shapes"] = Value::Object(shapes);
        Ok(Selection {
            document,
            operations: selected_names,
        })
    }
}

impl Selection {
    pub fn document(&self) -> &Value {
        &self.document
    }

    pub fn operations(&self) -> &[String] {
        &self.operations
    }

    pub fn write_json(&self, path: &Path) -> Result<(), BuildError> {
        let file = fs::File::create(path).map_err(|source| BuildError::ModelWrite {
            path: path.to_path_buf(),
            source,
        })?;
        serde_json::to_writer_pretty(file, &self.document).map_err(|source| {
            BuildError::ModelWrite {
                path: path.to_path_buf(),
                source: std::io::Error::other(source),
            }
        })
    }
}

fn load_file(path: &Path) -> Result<Model, BuildError> {
    let bytes = fs::read(path).map_err(|source| BuildError::ModelRead {
        path: path.to_path_buf(),
        source,
    })?;
    let root =
        serde_json::from_slice::<Value>(&bytes).map_err(|source| BuildError::ModelParse {
            path: path.to_path_buf(),
            source,
        })?;
    from_documents(path, vec![root])
}

fn load_directory(path: &Path) -> Result<Model, BuildError> {
    let mut files = Vec::new();
    collect_json_files(path, &mut files)?;
    files.sort();
    if files.is_empty() {
        return Err(BuildError::InvalidModel {
            path: path.to_path_buf(),
            message: "directory contains no .json model files".to_owned(),
        });
    }
    let documents = files
        .iter()
        .map(|file| {
            let bytes = fs::read(file).map_err(|source| BuildError::ModelRead {
                path: file.clone(),
                source,
            })?;
            serde_json::from_slice::<Value>(&bytes).map_err(|source| BuildError::ModelParse {
                path: file.clone(),
                source,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    from_documents(path, documents)
}

fn collect_json_files(path: &Path, files: &mut Vec<PathBuf>) -> Result<(), BuildError> {
    for entry in fs::read_dir(path).map_err(|source| BuildError::ModelRead {
        path: path.to_path_buf(),
        source,
    })? {
        let entry = entry.map_err(|source| BuildError::ModelRead {
            path: path.to_path_buf(),
            source,
        })?;
        let entry_path = entry.path();
        if entry_path.is_dir() {
            collect_json_files(&entry_path, files)?;
        } else if entry_path
            .extension()
            .and_then(|extension| extension.to_str())
            == Some("json")
        {
            files.push(entry_path);
        }
    }
    Ok(())
}

fn from_documents(path: &Path, documents: Vec<Value>) -> Result<Model, BuildError> {
    let mut root = Map::new();
    let mut shapes = BTreeMap::new();
    for document in documents {
        let mut object = document
            .as_object()
            .cloned()
            .ok_or_else(|| BuildError::InvalidModel {
                path: path.to_path_buf(),
                message: "model root must be a JSON object".to_owned(),
            })?;
        let document_shapes = object
            .remove("shapes")
            .and_then(|value| value.as_object().cloned())
            .ok_or_else(|| BuildError::InvalidModel {
                path: path.to_path_buf(),
                message: "model must contain an object-valued shapes member".to_owned(),
            })?;
        for (shape_id, shape) in document_shapes {
            if shapes.insert(shape_id.clone(), shape).is_some() {
                return Err(BuildError::DuplicateShape {
                    path: path.to_path_buf(),
                    shape: shape_id,
                });
            }
        }
        for (key, value) in object {
            root.entry(key).or_insert(value);
        }
    }
    root.insert("shapes".to_owned(), Value::Object(Map::new()));
    Ok(Model {
        root: Value::Object(root),
        shapes,
    })
}

fn find_operation(
    shapes: &BTreeMap<String, Value>,
    declared_operations: &[String],
    requested: &str,
) -> Option<String> {
    declared_operations
        .iter()
        .find_map(|operation_id| {
            let terminal_name = operation_id.rsplit('#').next()?;
            if operation_id == requested || terminal_name == requested {
                return Some(operation_id.clone());
            }
            None
        })
        .filter(|operation_id| {
            shapes
                .get(operation_id)
                .and_then(|shape| shape.get("type"))
                .and_then(Value::as_str)
                == Some("operation")
        })
}
