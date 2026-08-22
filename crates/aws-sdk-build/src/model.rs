use std::collections::{BTreeMap, BTreeSet, VecDeque};

use serde_json::{Map, Value};
use sha2::{Digest, Sha256};

use crate::{error::BuildError, registry::ModelEntry};

#[derive(Debug, Clone)]
pub(crate) struct Model {
    pub(crate) entry: ModelEntry,
    pub(crate) root: Value,
    pub(crate) shapes: BTreeMap<String, Value>,
    pub(crate) protocol_tests: Vec<Value>,
}

#[derive(Debug, Clone)]
pub(crate) struct SelectedModel {
    pub(crate) model: Model,
    pub(crate) operations: Vec<String>,
    /// Operation order from the source service shape, restricted to the selection.
    ///
    /// Smithy uses the model's service operation order while discovering shared
    /// shapes for the public `types` re-exports. The selected operation list is
    /// intentionally kept separate because callers may request operations in a
    /// different order.
    pub(crate) operation_order: Vec<String>,
    pub(crate) protocol_tests: Vec<Value>,
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
        let protocol_tests = entry
            .protocol_tests
            .map(serde_json::from_slice::<Value>)
            .transpose()
            .map_err(|source| BuildError::ModelParse {
                model: format!("{} protocol tests", entry.filename),
                source,
            })?
            .and_then(|value| value.get("tests").cloned())
            .and_then(|value| value.as_array().cloned())
            .unwrap_or_default();
        Ok(Self {
            entry,
            root,
            shapes,
            protocol_tests,
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

        let selected_set = selected_ids.iter().cloned().collect::<BTreeSet<_>>();
        let mut operation_order = self
            .root
            .get("shapes")
            .and_then(Value::as_object)
            .and_then(|shapes| shapes.get(self.entry.service_shape_id))
            .and_then(|service| service.get("operations"))
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(member_target)
            .filter(|id| selected_set.contains(*id))
            .map(terminal_name)
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>();
        for operation_id in &selected_ids {
            let operation_name = terminal_name(operation_id);
            if !operation_order.iter().any(|name| name == operation_name) {
                operation_order.push(operation_name.to_owned());
            }
        }

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
        apply_model_customizations(&mut shapes);
        normalize_operation_shapes(
            &mut shapes,
            &selected_ids,
            self.entry.service_shape_id,
            self.entry.filename,
        )?;
        let selected_protocol_tests = self
            .protocol_tests
            .iter()
            .filter(|test| {
                test.get("operation")
                    .and_then(Value::as_str)
                    .is_none_or(|operation| {
                        selected_ids
                            .iter()
                            .any(|selected| terminal_name(selected) == operation)
                    })
                    || test
                        .get("shape")
                        .and_then(Value::as_str)
                        .is_some_and(|shape| {
                            selected_ids.iter().any(|operation_id| {
                                shapes
                                    .get(operation_id)
                                    .and_then(|operation| operation.get("errors"))
                                    .and_then(Value::as_array)
                                    .is_some_and(|errors| {
                                        errors
                                            .iter()
                                            .filter_map(member_target)
                                            .any(|error| error == shape)
                                    })
                            })
                        })
            })
            .cloned()
            .collect();
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
                protocol_tests: self.protocol_tests.clone(),
            },
            operations,
            operation_order,
            protocol_tests: selected_protocol_tests,
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
        if !self.shapes.contains_key(self.entry.service_shape_id) {
            return Err(BuildError::InvalidModel {
                model: self.entry.filename.to_owned(),
                message: format!("service {} is missing", self.entry.service_shape_id),
            });
        }

        // Match Smithy's TopDownIndex.getContainedOperations: a service owns
        // both operations listed directly on the service and operations
        // attached to any resource in the service closure. Some AWS models
        // intentionally set `disableDefaultOperations`, leaving the latter
        // out of the service's explicit `operations` array.
        let mut queue = VecDeque::from_iter([self.entry.service_shape_id.to_owned()]);
        let mut visited = BTreeSet::new();
        let mut operations = BTreeSet::new();
        while let Some(id) = queue.pop_front() {
            if !visited.insert(id.clone()) {
                continue;
            }
            let Some(shape) = self.shapes.get(&id) else {
                continue;
            };
            if self.is_operation(&id) {
                operations.insert(id);
            }
            let mut references = BTreeSet::new();
            collect_shape_references(shape, &self.shapes, &mut references);
            queue.extend(references);
        }

        if operations.is_empty() {
            return Err(BuildError::InvalidModel {
                model: self.entry.filename.to_owned(),
                message: format!(
                    "service {} has neither operations nor resource operations",
                    self.entry.service_shape_id
                ),
            });
        }
        Ok(operations.into_iter().collect())
    }
}

/// Port Smithy Rust's OperationNormalizer: every selected operation gets a
/// private, operation-specific input and output structure while the original
/// modeled structures remain in the model for reuse by other shapes.
///
/// Keeping this transform in the model layer is important. Renderers can then
/// treat operation I/O uniformly, without accidentally classifying a shared
/// modeled structure as an operation-only shape.
fn normalize_operation_shapes(
    shapes: &mut Map<String, Value>,
    operation_ids: &[String],
    service_id: &str,
    model: &str,
) -> Result<(), BuildError> {
    let mut synthetic_shapes = Vec::with_capacity(operation_ids.len() * 2);
    let mut rewritten_operations = Vec::with_capacity(operation_ids.len());

    for operation_id in operation_ids {
        let Some(operation) = shapes.get(operation_id).cloned() else {
            continue;
        };
        let namespace = operation_id
            .split_once('#')
            .map(|(namespace, _)| namespace)
            .unwrap_or_default();
        let operation_name = terminal_name(operation_id);
        let input_id = format!("{namespace}.synthetic#{operation_name}Input");
        let output_id = format!("{namespace}.synthetic#{operation_name}Output");

        let input_original = operation
            .get("input")
            .and_then(member_target)
            .filter(|id| *id != "smithy.api#Unit");
        let output_original = operation
            .get("output")
            .and_then(member_target)
            .filter(|id| *id != "smithy.api#Unit");
        let input_shape = synthetic_operation_shape(
            shapes,
            input_original,
            &input_id,
            "smithy.api.internal#syntheticInput",
            true,
            operation_id,
            model,
        )?;
        let output_shape = synthetic_operation_shape(
            shapes,
            output_original,
            &output_id,
            "smithy.api.internal#syntheticOutput",
            false,
            operation_id,
            model,
        )?;
        synthetic_shapes.push((input_id.clone(), input_shape));
        synthetic_shapes.push((output_id.clone(), output_shape));

        let mut rewritten = operation;
        let object = rewritten
            .as_object_mut()
            .expect("operation shapes must be JSON objects");
        object.insert("input".to_owned(), operation_value(input_id));
        object.insert("output".to_owned(), operation_value(output_id));
        rewritten_operations.push((operation_id.clone(), rewritten));
    }

    for (id, shape) in synthetic_shapes {
        if shapes.contains_key(&id) {
            return Err(BuildError::InvalidModel {
                model: model.to_owned(),
                message: format!("synthetic operation shape {id} conflicts with an existing shape"),
            });
        }
        shapes.insert(id, shape);
    }
    for (id, operation) in rewritten_operations {
        shapes.insert(id, operation);
    }
    prune_to_directed_closure(shapes, service_id, operation_ids);
    Ok(())
}

fn prune_to_directed_closure(
    shapes: &mut Map<String, Value>,
    service_id: &str,
    operation_ids: &[String],
) {
    let mut queue = VecDeque::from_iter(
        std::iter::once(service_id.to_owned()).chain(operation_ids.iter().cloned()),
    );
    let mut retained = BTreeSet::new();
    let all_shapes = shapes
        .iter()
        .map(|(id, value)| (id.clone(), value.clone()))
        .collect::<BTreeMap<_, _>>();
    while let Some(id) = queue.pop_front() {
        if !retained.insert(id.clone()) {
            continue;
        }
        let Some(shape) = shapes.get(&id) else {
            continue;
        };
        let mut references = BTreeSet::new();
        collect_shape_references(shape, &all_shapes, &mut references);
        queue.extend(references);
    }
    let retained = retained
        .into_iter()
        .filter_map(|id| shapes.remove(&id).map(|shape| (id, shape)))
        .collect::<Map<_, _>>();
    *shapes = retained;
}

fn synthetic_operation_shape(
    shapes: &Map<String, Value>,
    original_id: Option<&str>,
    synthetic_id: &str,
    synthetic_trait: &str,
    is_input: bool,
    operation_id: &str,
    model: &str,
) -> Result<Value, BuildError> {
    let mut shape =
        match original_id {
            Some(original_id) => shapes.get(original_id).cloned().ok_or_else(|| {
                BuildError::MissingShapeReference {
                    model: model.to_owned(),
                    referenced_from: operation_id.to_owned(),
                    shape: original_id.to_owned(),
                }
            })?,
            None => serde_json::json!({"type": "structure"}),
        };
    let object = shape
        .as_object_mut()
        .ok_or_else(|| BuildError::InvalidModel {
            model: model.to_owned(),
            message: format!(
                "operation {operation_id} references a non-object {synthetic_id} shape"
            ),
        })?;
    if object.get("type").and_then(Value::as_str) != Some("structure") {
        return Err(BuildError::InvalidModel {
            model: model.to_owned(),
            message: format!(
                "operation {operation_id} references non-structure shape {synthetic_id}"
            ),
        });
    }
    let traits = object
        .entry("traits".to_owned())
        .or_insert_with(|| Value::Object(Map::new()))
        .as_object_mut()
        .expect("shape traits must be an object");
    let mut synthetic_metadata = Map::new();
    synthetic_metadata.insert(
        "operation".to_owned(),
        Value::String(operation_id.to_owned()),
    );
    if let Some(original_id) = original_id {
        synthetic_metadata.insert(
            "originalId".to_owned(),
            Value::String(original_id.to_owned()),
        );
    }
    traits.insert(
        synthetic_trait.to_owned(),
        Value::Object(synthetic_metadata),
    );
    if is_input {
        traits
            .entry("smithy.api#input".to_owned())
            .or_insert_with(|| Value::Object(Map::new()));
    }
    Ok(shape)
}

/// Applies model-driven AWS customizations that cannot be expressed in the
/// packaged Smithy model alone. The predicates intentionally inspect shape
/// relationships and traits instead of service or operation names.
fn apply_model_customizations(shapes: &mut Map<String, Value>) {
    // Some AWS models use a `Credentials` aggregate whose member-level
    // sensitivity is promoted to the aggregate by an AWS decorator. Express
    // that transform from the shape graph so the renderer can keep Smithy-RS's
    // non-recursive structure redaction predicate.
    let sensitive_targets = shapes
        .iter()
        .filter(|(_, shape)| {
            shape
                .get("traits")
                .and_then(Value::as_object)
                .is_some_and(|traits| traits.contains_key("smithy.api#sensitive"))
        })
        .map(|(id, _)| id.clone())
        .collect::<BTreeSet<_>>();
    let credential_shapes = shapes
        .iter()
        .filter(|(id, shape)| {
            terminal_name(id) == "Credentials"
                && shape.get("type").and_then(Value::as_str) == Some("structure")
                && shape
                    .get("members")
                    .and_then(Value::as_object)
                    .is_some_and(|members| {
                        members
                            .values()
                            .filter_map(member_target)
                            .any(|target| sensitive_targets.contains(target))
                    })
        })
        .map(|(id, _)| id.clone())
        .collect::<Vec<_>>();
    for id in credential_shapes {
        let shape = shapes
            .get_mut(&id)
            .and_then(Value::as_object_mut)
            .expect("credential shape must remain an object");
        shape
            .entry("traits".to_owned())
            .or_insert_with(|| Value::Object(Map::new()))
            .as_object_mut()
            .expect("shape traits must be an object")
            .insert("smithy.api#sensitive".to_owned(), Value::Object(Map::new()));
    }

    // Smithy-RS's AWS decorator marks CopyObject as incompatible with stalled
    // stream protection because its response can legitimately pause while the
    // service performs the copy.
    if let Some(shape) = shapes
        .get_mut("com.amazonaws.s3#CopyObject")
        .and_then(Value::as_object_mut)
    {
        shape
            .entry("traits".to_owned())
            .or_insert_with(|| Value::Object(Map::new()))
            .as_object_mut()
            .expect("operation traits must be an object")
            .insert(
                "software.amazon.smithy.rust.codegen.client.smithy.traits#incompatibleWithStalledStreamProtectionTrait"
                    .to_owned(),
                Value::Object(Map::new()),
            );
    }

    // The AWS Smithy-RS decorator marks this response so the paginator uses
    // `is_truncated` instead of the final numeric marker to detect exhaustion.
    // Keep the customization in the model transform, where the upstream
    // decorator applies it, rather than teaching the generic paginator about
    // an operation name.
    if let Some(shape) = shapes
        .get_mut("com.amazonaws.s3#ListPartsOutput")
        .and_then(Value::as_object_mut)
    {
        shape
            .entry("traits".to_owned())
            .or_insert_with(|| Value::Object(Map::new()))
            .as_object_mut()
            .expect("shape traits must be an object")
            .insert(
                "software.amazon.smithy.rust.codegen.client.smithy.traits#isTruncatedPaginatorTrait"
                    .to_owned(),
                Value::Object(Map::new()),
            );
    }

    // The S3 service decorator in smithy-rs permits these responses to use
    // wire roots that do not match their modeled output shape names.
    for shape_id in [
        "com.amazonaws.s3#CreateSessionOutput",
        "com.amazonaws.s3#GetObjectAttributesOutput",
        "com.amazonaws.s3#ListDirectoryBucketsOutput",
    ] {
        if let Some(shape) = shapes.get_mut(shape_id).and_then(Value::as_object_mut) {
            shape
                .entry("traits".to_owned())
                .or_insert_with(|| Value::Object(Map::new()))
                .as_object_mut()
                .expect("shape traits must be an object")
                .insert(
                    "smithy.api.internal#allowInvalidXmlRoot".to_owned(),
                    Value::Object(Map::new()),
                );
        }
    }

    // The Smithy Rust client transform adds an optional `Message` member to
    // every modeled error that does not already have a case-insensitive
    // `message`/`Message` member. This keeps error accessors and protocol
    // deserializers uniform even when the service model omits the field.
    for shape in shapes.values_mut() {
        let Some(shape_object) = shape.as_object_mut() else {
            continue;
        };
        let is_error = shape_object
            .get("traits")
            .and_then(Value::as_object)
            .is_some_and(|traits| traits.contains_key("smithy.api#error"));
        if !is_error || shape_object.get("type").and_then(Value::as_str) != Some("structure") {
            continue;
        }
        let members = shape_object
            .entry("members".to_owned())
            .or_insert_with(|| Value::Object(Map::new()))
            .as_object_mut()
            .expect("error structure members must be an object");
        if !members
            .keys()
            .any(|name| name.eq_ignore_ascii_case("message"))
        {
            members.insert(
                "Message".to_owned(),
                serde_json::json!({"target": "smithy.api#String"}),
            );
        }
    }

    let expires_targets = shapes
        .values()
        .filter_map(Value::as_object)
        .filter_map(|shape| shape.get("members").and_then(Value::as_object))
        .flat_map(|members| members.iter())
        .filter(|(name, member)| {
            name.eq_ignore_ascii_case("Expires")
                && member
                    .get("traits")
                    .and_then(Value::as_object)
                    .and_then(|traits| traits.get("smithy.api#httpHeader"))
                    .and_then(Value::as_str)
                    .is_some_and(|header| header.eq_ignore_ascii_case("Expires"))
        })
        .filter_map(|(_, member)| member_target(member).map(ToOwned::to_owned))
        .filter(|target| {
            shapes
                .get(target)
                .and_then(|shape| shape.get("type"))
                .and_then(Value::as_str)
                == Some("string")
        })
        .collect::<BTreeSet<_>>();

    if expires_targets.is_empty() {
        return;
    }

    for target in &expires_targets {
        if let Some(shape) = shapes.get_mut(target).and_then(Value::as_object_mut) {
            shape.insert("type".to_owned(), Value::String("timestamp".to_owned()));
        }
    }

    let target = expires_targets
        .first()
        .expect("expires target set is non-empty");
    let namespace = target
        .split('#')
        .next()
        .and_then(|namespace| namespace.rsplit('.').next())
        .unwrap_or("service");
    let synthetic_target = format!("aws.sdk.rust.{namespace}.synthetic#ExpiresString");
    shapes.insert(
        synthetic_target.clone(),
        serde_json::json!({ "type": "string" }),
    );

    for shape in shapes.values_mut() {
        let Some(shape_object) = shape.as_object_mut() else {
            continue;
        };
        let is_output = shape_object
            .get("traits")
            .and_then(Value::as_object)
            .is_some_and(|traits| traits.contains_key("smithy.api#output"));
        if !is_output {
            continue;
        }
        let Some(members) = shape_object
            .get_mut("members")
            .and_then(Value::as_object_mut)
        else {
            continue;
        };
        let Some((expires_name, expires_member)) = members.iter_mut().find(|(name, member)| {
            name.eq_ignore_ascii_case("Expires")
                && member_target(member).is_some_and(|target| expires_targets.contains(target))
        }) else {
            continue;
        };

        let expires_name = expires_name.clone();
        let expires_member = expires_member.clone();
        let deprecated = expires_member
            .get("traits")
            .and_then(Value::as_object)
            .and_then(|traits| traits.get("smithy.api#deprecated"))
            .cloned()
            .unwrap_or_else(|| {
                serde_json::json!({
                    "message": "Please use `expires_string` which contains the raw, unparsed value of this field."
                })
            });
        let mut updated_member = expires_member;
        updated_member
            .as_object_mut()
            .expect("expires member is an object")
            .entry("traits".to_owned())
            .or_insert_with(|| Value::Object(Map::new()));
        updated_member
            .get_mut("traits")
            .and_then(Value::as_object_mut)
            .expect("expires member traits are an object")
            .insert("smithy.api#deprecated".to_owned(), deprecated);
        let documentation = updated_member
            .get("traits")
            .and_then(Value::as_object)
            .and_then(|traits| traits.get("smithy.api#documentation"))
            .cloned();
        let mut traits = Map::new();
        if let Some(documentation) = documentation {
            traits.insert("smithy.api#documentation".to_owned(), documentation);
        }
        traits.insert(
            "smithy.api#httpHeader".to_owned(),
            Value::String("ExpiresString".to_owned()),
        );
        let synthetic_member = Value::Object(Map::from_iter([
            ("target".to_owned(), Value::String(synthetic_target.clone())),
            ("traits".to_owned(), Value::Object(traits)),
        ]));
        let old_members = std::mem::take(members);
        let mut new_members = Map::new();
        for (name, member) in old_members {
            if name != expires_name {
                new_members.insert(name, member);
            }
        }
        // Smithy's S3 decorator removes Expires and appends the transformed
        // Expires/ExpiresString pair after the original output members.
        new_members.insert(expires_name.clone(), updated_member);
        new_members.insert(format!("{expires_name}String"), synthetic_member);
        *members = new_members;
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
    fn operation_normalization_preserves_shared_s3_shapes() {
        let entry = crate::registry::lookup("s3").unwrap();
        let model = Model::load(entry).unwrap();
        let selected = model.select(&[], true).unwrap();
        let operation = selected
            .model
            .shapes
            .get("com.amazonaws.s3#GetBucketNotificationConfiguration")
            .and_then(Value::as_object)
            .unwrap();
        let output_id = operation.get("output").and_then(member_target).unwrap();
        assert_eq!(
            output_id,
            "com.amazonaws.s3.synthetic#GetBucketNotificationConfigurationOutput"
        );
        assert!(
            selected
                .model
                .shapes
                .get(output_id)
                .and_then(|shape| shape.get("traits"))
                .and_then(Value::as_object)
                .is_some_and(|traits| {
                    traits.contains_key("smithy.api.internal#syntheticOutput")
                })
        );
        assert!(
            selected
                .model
                .shapes
                .contains_key("com.amazonaws.s3#NotificationConfiguration")
        );
        assert!(
            !selected
                .model
                .shapes
                .contains_key("com.amazonaws.s3#GetBucketNotificationConfigurationOutput")
        );
        let synthetic_output = selected
            .model
            .shapes
            .get(output_id)
            .and_then(|shape| shape.get("traits"))
            .and_then(Value::as_object)
            .and_then(|traits| traits.get("smithy.api.internal#syntheticOutput"))
            .and_then(Value::as_object)
            .and_then(|metadata| metadata.get("originalId"))
            .and_then(Value::as_str);
        assert_eq!(
            synthetic_output,
            Some("com.amazonaws.s3#NotificationConfiguration")
        );
    }

    #[test]
    fn credential_aggregates_inherit_member_sensitivity() {
        let entry = crate::registry::lookup("sts").unwrap();
        let model = Model::load(entry).unwrap();
        let selected = model.select(&[], true).unwrap();
        assert!(
            selected
                .model
                .shapes
                .get("com.amazonaws.sts#Credentials")
                .and_then(|shape| shape.get("traits"))
                .and_then(Value::as_object)
                .is_some_and(|traits| traits.contains_key("smithy.api#sensitive"))
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
