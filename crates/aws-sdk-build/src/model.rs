use std::collections::{BTreeMap, BTreeSet, VecDeque};

use serde_json::{Map, Value};
use sha2::{Digest, Sha256};

use crate::{error::BuildError, registry::ModelEntry};

#[derive(Debug, Clone)]
pub(crate) struct Model {
    pub(crate) entry: ModelEntry,
    pub(crate) root: Value,
    pub(crate) shapes: BTreeMap<String, Value>,
}

#[derive(Debug, Clone)]
pub(crate) struct SelectedModel {
    pub(crate) model: Model,
    pub(crate) operations: Vec<String>,
}

/// Protocols understood by the generated client layer.
///
/// The model advertises protocols as service traits. Keeping this value in the
/// model layer means renderers can consume a protocol plan instead of making
/// service-name decisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ProtocolKind {
    RestXml,
    RestJson1,
    AwsJson1_0,
    AwsJson1_1,
    AwsQuery,
    AwsQueryCompatible,
    Ec2Query,
}

impl ProtocolKind {
    pub(crate) const fn trait_id(self) -> &'static str {
        match self {
            Self::RestXml => "aws.protocols#restXml",
            Self::RestJson1 => "aws.protocols#restJson1",
            Self::AwsJson1_0 => "aws.protocols#awsJson1_0",
            Self::AwsJson1_1 => "aws.protocols#awsJson1_1",
            Self::AwsQuery => "aws.protocols#awsQuery",
            Self::AwsQueryCompatible => "aws.protocols#awsQueryCompatible",
            Self::Ec2Query => "aws.protocols#ec2Query",
        }
    }
}

impl Model {
    pub(crate) fn load(entry: ModelEntry) -> Result<Self, BuildError> {
        if let Some(expected) = crate::registry::checksum(entry.key) {
            let actual = format!("{:x}", Sha256::digest(entry.bytes));
            if actual != expected {
                return Err(BuildError::InvalidModel {
                    model: entry.filename.to_owned(),
                    message: format!("SHA-256 mismatch: expected {expected}, got {actual}"),
                });
            }
        }
        let root = serde_json::from_slice::<Value>(entry.bytes).map_err(|source| {
            BuildError::ModelParse {
                model: entry.filename.to_owned(),
                source,
            }
        })?;
        let shapes = root
            .get("shapes")
            .and_then(Value::as_object)
            .ok_or_else(|| BuildError::InvalidModel {
                model: entry.filename.to_owned(),
                message: "model must contain an object-valued shapes member".to_owned(),
            })?
            .iter()
            .map(|(id, shape)| (id.clone(), shape.clone()))
            .collect();
        Ok(Self {
            entry,
            root,
            shapes,
        })
    }

    pub(crate) fn select(
        &self,
        requested: &[String],
        all_operations: bool,
    ) -> Result<SelectedModel, BuildError> {
        let declared = self.declared_operations()?;
        let selected_ids = if all_operations {
            declared.clone()
        } else {
            requested
                .iter()
                .map(|name| {
                    declared
                        .iter()
                        .find(|id| {
                            id.as_str() == name.as_str() || terminal_name(id) == name.as_str()
                        })
                        .filter(|id| self.is_operation(id))
                        .cloned()
                        .ok_or_else(|| BuildError::UnknownOperation {
                            service: self.entry.key.to_owned(),
                            operation: name.clone(),
                            model: self.entry.filename.to_owned(),
                        })
                })
                .collect::<Result<Vec<_>, _>>()?
        };

        let mut queue = VecDeque::from_iter(
            std::iter::once(self.entry.service_shape_id.to_owned()).chain(selected_ids.clone()),
        );
        let mut retained = BTreeSet::new();
        while let Some(id) = queue.pop_front() {
            if !retained.insert(id.clone()) {
                continue;
            }
            let Some(shape) = self.shapes.get(&id) else {
                continue;
            };
            let mut references = BTreeSet::new();
            collect_shape_references(shape, &self.shapes, &mut references);
            for reference in references {
                if self.shapes.contains_key(&reference) {
                    queue.push_back(reference);
                } else if !reference.starts_with("smithy.api#") {
                    return Err(BuildError::MissingShapeReference {
                        model: self.entry.filename.to_owned(),
                        referenced_from: id,
                        shape: reference,
                    });
                }
            }
        }

        let mut shapes = Map::new();
        for id in retained {
            if let Some(shape) = self.shapes.get(&id) {
                shapes.insert(id, shape.clone());
            }
        }
        if let Some(service) = shapes
            .get_mut(self.entry.service_shape_id)
            .and_then(Value::as_object_mut)
        {
            service.insert(
                "operations".to_owned(),
                Value::Array(selected_ids.iter().cloned().map(operation_value).collect()),
            );
        }
        let mut root = self.root.clone();
        root["shapes"] = Value::Object(shapes);
        let selected_shape_map = root_shape_map(&root);
        let operations = selected_ids
            .iter()
            .map(|id| terminal_name(id).to_owned())
            .collect();
        Ok(SelectedModel {
            model: Self {
                entry: self.entry,
                root,
                shapes: selected_shape_map,
            },
            operations,
        })
    }

    pub(crate) fn protocol(&self) -> Result<ProtocolKind, BuildError> {
        let service = self
            .shapes
            .get(self.entry.service_shape_id)
            .ok_or_else(|| BuildError::InvalidModel {
                model: self.entry.filename.to_owned(),
                message: format!("service {} is missing", self.entry.service_shape_id),
            })?;
        let traits = service
            .get("traits")
            .and_then(Value::as_object)
            .ok_or_else(|| BuildError::InvalidModel {
                model: self.entry.filename.to_owned(),
                message: format!("service {} has no traits", self.entry.service_shape_id),
            })?;

        // This order is the supported-protocol order used by the client
        // generator. It is deliberately independent of service identity and
        // of JSON object iteration order.
        [
            ProtocolKind::RestXml,
            ProtocolKind::RestJson1,
            ProtocolKind::AwsJson1_0,
            ProtocolKind::AwsJson1_1,
            ProtocolKind::AwsQuery,
            ProtocolKind::AwsQueryCompatible,
            ProtocolKind::Ec2Query,
        ]
        .into_iter()
        .find(|protocol| traits.contains_key(protocol.trait_id()))
        .ok_or_else(|| BuildError::InvalidModel {
            model: self.entry.filename.to_owned(),
            message: format!(
                "service {} does not advertise a supported AWS protocol",
                self.entry.service_shape_id
            ),
        })
    }

    fn is_operation(&self, id: &str) -> bool {
        self.shapes
            .get(id)
            .and_then(|shape| shape.get("type"))
            .and_then(Value::as_str)
            == Some("operation")
    }

    fn declared_operations(&self) -> Result<Vec<String>, BuildError> {
        let service = self
            .shapes
            .get(self.entry.service_shape_id)
            .ok_or_else(|| BuildError::InvalidModel {
                model: self.entry.filename.to_owned(),
                message: format!("service {} is missing", self.entry.service_shape_id),
            })?;
        if let Some(operations) = service.get("operations").and_then(Value::as_array) {
            return Ok(operations.iter().filter_map(operation_target).collect());
        }
        let mut resources = VecDeque::new();
        if let Some(service_resources) = service.get("resources").and_then(Value::as_array) {
            resources.extend(service_resources.iter().filter_map(operation_target));
        }
        let mut seen = BTreeSet::new();
        let mut operations = Vec::new();
        while let Some(resource_id) = resources.pop_front() {
            if !seen.insert(resource_id.clone()) {
                continue;
            }
            let Some(resource) = self.shapes.get(&resource_id) else {
                continue;
            };
            for key in [
                "operations",
                "collectionOperations",
                "create",
                "put",
                "read",
                "update",
                "delete",
                "list",
            ] {
                if let Some(values) = resource.get(key) {
                    match values {
                        Value::Array(values) => {
                            operations.extend(values.iter().filter_map(operation_target));
                        }
                        value => {
                            if let Some(target) = operation_target(value) {
                                operations.push(target);
                            }
                        }
                    }
                }
            }
            if let Some(nested) = resource.get("resources").and_then(Value::as_array) {
                resources.extend(nested.iter().filter_map(operation_target));
            }
        }
        operations.retain(|id| self.is_operation(id));
        operations.sort();
        operations.dedup();
        if operations.is_empty() {
            return Err(BuildError::InvalidModel {
                model: self.entry.filename.to_owned(),
                message: format!(
                    "service {} has neither operations nor resource operations",
                    self.entry.service_shape_id
                ),
            });
        }
        Ok(operations)
    }
}

fn root_shape_map(root: &Value) -> BTreeMap<String, Value> {
    root.get("shapes")
        .and_then(Value::as_object)
        .map(|shapes| {
            shapes
                .iter()
                .map(|(id, value)| (id.clone(), value.clone()))
                .collect()
        })
        .unwrap_or_default()
}

fn operation_target(value: &Value) -> Option<String> {
    value.as_str().map(ToOwned::to_owned).or_else(|| {
        value
            .get("target")
            .and_then(Value::as_str)
            .map(ToOwned::to_owned)
    })
}

fn operation_value(id: String) -> Value {
    Value::Object(Map::from_iter([("target".to_owned(), Value::String(id))]))
}

fn terminal_name(id: &str) -> &str {
    id.rsplit('#').next().unwrap_or(id)
}

fn collect_shape_references(
    value: &Value,
    shapes: &BTreeMap<String, Value>,
    output: &mut BTreeSet<String>,
) {
    match value {
        Value::Array(values) => {
            values
                .iter()
                .for_each(|value| collect_shape_references(value, shapes, output));
        }
        Value::Object(object) => {
            if let Some(traits) = object.get("traits").and_then(Value::as_object) {
                output.extend(traits.keys().filter(|id| shapes.contains_key(*id)).cloned());
            }
            for (key, value) in object {
                if key == "members" {
                    if let Some(members) = value.as_object() {
                        for member in members.values() {
                            if let Some(target) = member_target(member)
                                && (shapes.contains_key(target)
                                    || !target.starts_with("smithy.api#"))
                            {
                                output.insert(target.to_owned());
                            }
                        }
                    }
                    continue;
                }
                let is_reference = matches!(
                    key.as_str(),
                    "target"
                        | "input"
                        | "output"
                        | "errors"
                        | "members"
                        | "resource"
                        | "resources"
                        | "key"
                        | "value"
                        | "identifiers"
                        | "eventStream"
                );
                if is_reference {
                    collect_reference_values(value, shapes, output);
                } else if key != "traits" {
                    collect_shape_references(value, shapes, output);
                }
            }
        }
        _ => {}
    }
}

fn member_target(value: &Value) -> Option<&str> {
    value
        .as_str()
        .or_else(|| value.get("target").and_then(Value::as_str))
}

fn collect_reference_values(
    value: &Value,
    shapes: &BTreeMap<String, Value>,
    output: &mut BTreeSet<String>,
) {
    match value {
        Value::String(id) => {
            if shapes.contains_key(id) || !id.starts_with("smithy.api#") {
                output.insert(id.clone());
            }
        }
        Value::Array(values) => values
            .iter()
            .for_each(|value| collect_reference_values(value, shapes, output)),
        Value::Object(object) => {
            if let Some(target) = object.get("target").and_then(Value::as_str) {
                if shapes.contains_key(target) || !target.starts_with("smithy.api#") {
                    output.insert(target.to_owned());
                }
            } else {
                object
                    .values()
                    .for_each(|value| collect_reference_values(value, shapes, output));
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_packaged_service_loads_and_selects_all_operations() {
        for entry in crate::registry::entries() {
            let model = Model::load(*entry).unwrap_or_else(|error| {
                panic!("{} failed to load: {error}", entry.key);
            });
            let selected = model.select(&[], true).unwrap_or_else(|error| {
                panic!("{} failed closure: {error}", entry.key);
            });
            assert!(
                !selected.operations.is_empty(),
                "{} has no operations",
                entry.key
            );
        }
    }

    #[test]
    fn s3_nested_members_remain_in_the_operation_closure() {
        let entry = crate::registry::lookup("s3").unwrap();
        let model = Model::load(entry).unwrap();
        let selected = model.select(&[], true).unwrap();
        assert!(
            selected
                .model
                .shapes
                .contains_key("com.amazonaws.s3#NotificationConfiguration")
        );
    }

    #[test]
    fn packaged_services_select_protocols_from_service_traits() {
        for entry in crate::registry::entries() {
            let model = Model::load(*entry).unwrap();
            let protocol = model
                .protocol()
                .unwrap_or_else(|error| panic!("{} has no supported protocol: {error}", entry.key));
            assert!(protocol.trait_id().starts_with("aws.protocols#"));
        }
    }
}
