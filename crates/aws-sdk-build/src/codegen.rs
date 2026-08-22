use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    fmt::Write,
    fs,
    path::Path,
};

use crate::{config::ServiceSelection, error::BuildError, model::SelectedModel, names, registry};

pub(crate) struct Generated {
    pub(crate) operations: Vec<String>,
}

pub(crate) fn generate(
    stage: &Path,
    consumer_namespace: bool,
    selections: &[ServiceSelection],
) -> Result<Generated, BuildError> {
    let generated = stage.join("generated");
    fs::create_dir_all(&generated).map_err(|source| BuildError::OutputWrite {
        path: generated.clone(),
        source,
    })?;
    let mut facade = String::new();
    header(&mut facade);
    facade.push_str(
        "#[doc(hidden)]\n\
         pub mod meta {\n\
             pub const PKG_VERSION: &str = env!(\"CARGO_PKG_VERSION\");\n\
         }\n\n",
    );
    let mut all_operations = Vec::new();
    for selection in selections {
        let entry = registry::lookup(&selection.key)?;
        let model = crate::model::Model::load(entry)?;
        let selected = model.select(&selection.operations, selection.all_operations)?;
        let protocol = selected.model.protocol()?;
        let request_id_plan = request_id_plan(&selected);
        let service_dir = generated.join(entry.key);
        let mut service_files = vec![
            (
                "src/lib.rs".to_owned(),
                render_service_lib(entry.key, &selected, consumer_namespace),
            ),
            (
                "src/primitives.rs".to_owned(),
                render_primitives(&selected, consumer_namespace),
            ),
            ("src/config.rs".to_owned(), render_config_file()),
            (
                "src/error.rs".to_owned(),
                render_error_file(consumer_namespace, model_has_enum(&selected)),
            ),
            ("src/meta.rs".to_owned(), render_meta(entry.key)),
            (
                "src/observability_feature.rs".to_owned(),
                render_observability_feature(),
            ),
            (
                "src/types.rs".to_owned(),
                render_types_file(entry.key, &selected, consumer_namespace),
            ),
            (
                "src/types/builders.rs".to_owned(),
                render_types_builders_file(&selected, consumer_namespace),
            ),
            (
                "src/types/error.rs".to_owned(),
                render_error_types_file(entry.key, &selected, consumer_namespace),
            ),
            (
                "src/types/error/builders.rs".to_owned(),
                render_error_builders_file(&selected, consumer_namespace),
            ),
            (
                "src/operation.rs".to_owned(),
                render_operations_file(entry.key, entry.module_name, &selected, consumer_namespace),
            ),
            (
                "src/client.rs".to_owned(),
                render_client_file(entry.key, &selected),
            ),
            (
                "src/client/customize.rs".to_owned(),
                render_client_customize_file(&selected),
            ),
            (
                "src/client/customize/internal.rs".to_owned(),
                render_client_customize_internal_file(&selected),
            ),
            (
                "src/config/http.rs".to_owned(),
                include_str!("../assets/config_http.rs").to_owned(),
            ),
            (
                "src/config/interceptors.rs".to_owned(),
                include_str!("../assets/config_interceptors.rs").to_owned(),
            ),
            (
                "src/config/retry.rs".to_owned(),
                include_str!("../assets/config_retry.rs").to_owned(),
            ),
            (
                "src/config/timeout.rs".to_owned(),
                include_str!("../assets/config_timeout.rs").to_owned(),
            ),
            (
                "src/sdk_feature_tracker.rs".to_owned(),
                include_str!("../assets/sdk_feature_tracker.rs").to_owned(),
            ),
            (
                "src/serde_util.rs".to_owned(),
                render_serde_util_file(&selected),
            ),
        ];
        if has_idempotency_operations(&selected) {
            service_files.push((
                "src/idempotency_token.rs".to_owned(),
                include_str!("../assets/idempotency_token.rs").to_owned(),
            ));
            service_files.push((
                "src/client_idempotency_token.rs".to_owned(),
                include_str!("../assets/client_idempotency_token.rs").to_owned(),
            ));
        }
        if !consumer_namespace {
            service_files.push((
                "src/error_meta.rs".to_owned(),
                render_service_error_metadata(&selected),
            ));
            service_files.push((
                "src/error/sealed_unhandled.rs".to_owned(),
                include_str!("../assets/error_sealed_unhandled.rs").to_owned(),
            ));
        }
        if !consumer_namespace {
            service_files.push((
                "src/primitives/event_stream.rs".to_owned(),
                render_event_stream_primitives(model_has_streaming(&selected)),
            ));
            if model_has_enum(&selected) {
                service_files.push((
                    "src/primitives/sealed_enum_unknown.rs".to_owned(),
                    render_sealed_enum_unknown(),
                ));
            }
        }
        if request_id_plan.extended {
            service_files.push(("src/s3_request_id.rs".to_owned(), render_s3_request_id()));
        }
        if has_presignable_operations(&selected) {
            service_files.push(("src/presigning.rs".to_owned(), render_presigning_file()));
            service_files.push((
                "src/presigning_interceptors.rs".to_owned(),
                render_presigning_interceptors_file(),
            ));
            service_files.push((
                "src/serialization_settings.rs".to_owned(),
                render_serialization_settings_file(),
            ));
        }
        if protocol == crate::model::ProtocolKind::RestXml {
            let (protocol_module, protocol_shape_files) = render_protocol_serde_files(&selected);
            service_files.push(("src/protocol_serde.rs".to_owned(), protocol_module));
            service_files.extend(protocol_shape_files);
        }
        if has_paginated_operations(&selected) {
            service_files.push((
                "src/lens.rs".to_owned(),
                render_lens_file(&selected, consumer_namespace),
            ));
        }
        if has_waiters(&selected) {
            service_files.push((
                "src/waiters.rs".to_owned(),
                render_waiters_file(&selected, consumer_namespace),
            ));
            service_files.push((
                "src/waiters/matchers.rs".to_owned(),
                render_waiter_matchers_file(&selected, consumer_namespace),
            ));
            for (_, waiter_name, waiter) in waiter_specs(&selected) {
                service_files.push((
                    format!("src/waiters/{waiter_name}.rs"),
                    render_waiter_file(&selected, &waiter_name, &waiter, consumer_namespace),
                ));
            }
        }
        let mut operation_names = selected.operations.clone();
        operation_names.sort();
        for operation_name in operation_names {
            let module = names::snake_case(&operation_name);
            service_files.push((
                format!("src/operation/{module}.rs"),
                render_operation_file(entry.key, &selected, &operation_name, consumer_namespace),
            ));
            service_files.push((
                format!("src/operation/{module}/_{module}_input.rs"),
                render_operation_shape_file(&selected, &operation_name, true, consumer_namespace),
            ));
            service_files.push((
                format!("src/operation/{module}/_{module}_output.rs"),
                render_operation_shape_file(&selected, &operation_name, false, consumer_namespace),
            ));
            service_files.push((
                format!("src/operation/{module}/builders.rs"),
                render_operation_builder_file(&selected, &operation_name, consumer_namespace),
            ));
            if operation_pagination_info(&selected, &operation_name).is_some() {
                service_files.push((
                    format!("src/operation/{module}/paginator.rs"),
                    render_paginator_file(&selected, &operation_name, consumer_namespace),
                ));
            }
            service_files.push((
                format!("src/client/{module}.rs"),
                render_client_operation_file(&selected, &operation_name, consumer_namespace),
            ));
            if protocol == crate::model::ProtocolKind::RestXml {
                service_files.push((
                    format!("src/protocol_serde/shape_{module}.rs"),
                    render_protocol_operation_file(&selected, &operation_name, consumer_namespace),
                ));
                if let Some(payload_source) = render_protocol_input_file(&selected, &operation_name)
                {
                    service_files.push((
                        format!("src/protocol_serde/shape_{module}_input.rs"),
                        payload_source,
                    ));
                }
                if let Some(output_source) =
                    render_protocol_output_file(&selected, &operation_name, consumer_namespace)
                {
                    service_files.push((
                        format!("src/protocol_serde/shape_{module}_output.rs"),
                        output_source,
                    ));
                }
            }
        }
        let operation_shapes = operation_shape_ids(&selected);
        let mut shape_ids = selected.model.shapes.keys().cloned().collect::<Vec<_>>();
        shape_ids.sort();
        for shape_id in shape_ids {
            let Some(shape) = selected.model.shapes.get(&shape_id) else {
                continue;
            };
            if shape_id == selected.model.entry.service_shape_id
                || operation_shapes.contains(&shape_id)
                || !is_file_renderable_type(selected.model.shapes.get(&shape_id))
            {
                continue;
            }
            let relative_dir = if is_error_shape(shape) {
                "src/types/error"
            } else {
                "src/types"
            };
            service_files.push((
                format!("{relative_dir}/{}", type_file_name(&shape_id)),
                render_type_file(
                    &selected,
                    &shape_id,
                    if is_error_shape(shape) {
                        Context::Error { consumer_namespace }
                    } else {
                        Context::Types { consumer_namespace }
                    },
                    consumer_namespace,
                ),
            ));
        }
        for (relative_path, source) in service_files {
            let source_path = service_dir.join(&relative_path);
            if let Some(parent) = source_path.parent() {
                fs::create_dir_all(parent).map_err(|source| BuildError::OutputWrite {
                    path: parent.to_owned(),
                    source,
                })?;
            }
            fs::write(&source_path, normalize_source(&source)).map_err(|source| {
                BuildError::OutputWrite {
                    path: source_path.clone(),
                    source,
                }
            })?;
        }
        if consumer_namespace {
            writeln!(
                facade,
                "#[allow(non_snake_case, dead_code, unused_imports, deprecated, clippy::unnecessary_to_owned)]"
            )
            .unwrap();
        } else {
            writeln!(
                facade,
                "#[allow(non_snake_case, dead_code, unused_imports)]"
            )
            .unwrap();
        }
        writeln!(facade, "pub mod {} {{", entry.module_name).unwrap();
        writeln!(
            facade,
            "    include!(concat!(env!(\"OUT_DIR\"), \"/generated/{}/src/lib.rs\"));",
            entry.key
        )
        .unwrap();
        facade.push_str("}\n\n");
        all_operations.extend(selected.operations.iter().cloned());
    }
    let include_path = stage.join("aws_sdk.rs");
    fs::write(&include_path, normalize_source(&facade)).map_err(|source| {
        BuildError::OutputWrite {
            path: include_path.clone(),
            source,
        }
    })?;
    all_operations.sort();
    Ok(Generated {
        operations: all_operations,
    })
}

fn header(output: &mut String) {
    output.push_str(
        "// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.\n\n",
    );
}

fn client_operation_header(output: &mut String) {
    output.push_str(
        "// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.\n",
    );
}

fn normalize_source(source: &str) -> String {
    format!("{}\n", source.trim_end_matches('\n'))
}

#[derive(Clone, Copy, Debug, Default)]
struct RequestIdPlan {
    standard: bool,
    extended: bool,
}

fn request_id_plan(selected: &SelectedModel) -> RequestIdPlan {
    let service = selected
        .model
        .shapes
        .get(selected.model.entry.service_shape_id);
    let service_traits = service
        .and_then(|shape| shape.get("traits"))
        .and_then(Value::as_object);
    let service_metadata = service_traits
        .and_then(|traits| traits.get("aws.api#service"))
        .and_then(Value::as_object);
    RequestIdPlan {
        standard: true,
        extended: service_metadata
            .and_then(|metadata| metadata.get("arnNamespace"))
            .and_then(Value::as_str)
            .is_some_and(|namespace| namespace.eq_ignore_ascii_case("s3")),
    }
}

fn output_request_id_plan(selected: &SelectedModel, context: &Context) -> RequestIdPlan {
    match context {
        Context::Operation { input: false, .. } | Context::Builder { input: false, .. } => {
            request_id_plan(selected)
        }
        _ => RequestIdPlan::default(),
    }
}

fn render_service_lib(
    service_key: &str,
    selected: &SelectedModel,
    consumer_namespace: bool,
) -> String {
    let mut output = String::new();
    header(&mut output);
    if !consumer_namespace {
        output.push_str("pub use error_meta::Error;\n\n");
    }
    for file in [
        "primitives.rs",
        "config.rs",
        "error.rs",
        "s3_request_id.rs",
        "meta.rs",
        "types.rs",
        "operation.rs",
        "client.rs",
    ] {
        if file == "s3_request_id.rs" && !request_id_plan(selected).extended {
            continue;
        }
        if file == "s3_request_id.rs" {
            writeln!(
                output,
                "pub mod s3_request_id {{\n    include!(concat!(env!(\"OUT_DIR\"), \"/generated/{service_key}/src/{file}\"));\n}}"
            )
            .unwrap();
            continue;
        }
        if file == "types.rs" {
            writeln!(
                output,
                "pub mod types {{\n    include!(concat!(env!(\"OUT_DIR\"), \"/generated/{service_key}/src/{file}\"));\n}}"
            )
            .unwrap();
            continue;
        }
        writeln!(
            output,
            "include!(concat!(env!(\"OUT_DIR\"), \"/generated/{service_key}/src/{file}\"));"
        )
        .unwrap();
    }
    if !consumer_namespace {
        output.push_str("\nmod error_meta;\n");
    }
    writeln!(
        output,
        "mod serde_util {{\n    include!(concat!(env!(\"OUT_DIR\"), \"/generated/{service_key}/src/serde_util.rs\"));\n}}"
    )
    .unwrap();
    if has_paginated_operations(selected) {
        if consumer_namespace {
            writeln!(
                output,
                "#[allow(clippy::question_mark)]\nmod lens {{\n    include!(concat!(env!(\"OUT_DIR\"), \"/generated/{service_key}/src/lens.rs\"));\n}}"
            )
            .unwrap();
        } else {
            writeln!(
                output,
                "mod lens {{\n    include!(concat!(env!(\"OUT_DIR\"), \"/generated/{service_key}/src/lens.rs\"));\n}}"
            )
            .unwrap();
        }
    }
    if has_waiters(selected) {
        if consumer_namespace {
            writeln!(
                output,
                "pub mod waiters {{\n    include!(concat!(env!(\"OUT_DIR\"), \"/generated/{service_key}/src/waiters.rs\"));\n}}"
            )
            .unwrap();
        } else {
            output.push_str("pub mod waiters;\n");
        }
        if consumer_namespace {
            output.push('\n');
            render_consumer_waiters_trait(&mut output, selected);
        }
    }
    output
}

fn render_consumer_waiters_trait(output: &mut String, selected: &SelectedModel) {
    output.push_str(
        "///\n/// Waiter functions for the client.\n///\n/// Import this trait to get `wait_until` methods on the client.\n///\n",
    );
    output.push_str("pub trait Waiters {\n");
    for (_, waiter_name, _) in waiter_specs_by_name(selected) {
        let waiter_type = rust_type_name(&waiter_name);
        writeln!(
            output,
            "    /// Wait for `{waiter_name}`\n    fn wait_until_{waiter_name}(&self) -> waiters::{waiter_name}::{waiter_type}FluentBuilder;"
        )
        .unwrap();
    }
    output.push_str("}\nimpl Waiters for Client {\n");
    for (_, waiter_name, _) in waiter_specs_by_name(selected) {
        let waiter_type = rust_type_name(&waiter_name);
        writeln!(
            output,
            "    fn wait_until_{waiter_name}(&self) -> waiters::{waiter_name}::{waiter_type}FluentBuilder {{\n        waiters::{waiter_name}::{waiter_type}FluentBuilder::new(self.clone())\n    }}"
        )
        .unwrap();
    }
    output.push_str("}\n");
}

fn render_serde_util_file(selected: &SelectedModel) -> String {
    let mut output = String::new();
    client_operation_header(&mut output);
    let mut type_order = Vec::new();
    let mut seen_types = BTreeSet::new();

    // Operation output and error corrections are emitted while Smithy walks
    // operation schemas. This is intentionally separate from modeled shape
    // corrections, which are discovered by protocol deserialization below.
    for operation_name in &selected.operations {
        let Some(operation) = operation_shape(selected, operation_name) else {
            continue;
        };
        if let Some(output_id) = operation.get("output").and_then(target_value)
            && let Some(shape) = selected.model.shapes.get(output_id)
            && serde_util_shape_needs_correction(shape)
        {
            let module = names::rust_module_name(operation_name);
            let operation_type = rust_type_name(operation_name);
            render_serde_util_correction(
                &mut output,
                selected,
                shape,
                &format!("{module}_output_output_correct_errors"),
                &format!("crate::operation::{module}::builders::{operation_type}OutputBuilder"),
            );
        }
        if let Some(errors) = operation.get("errors").and_then(Value::as_array) {
            for error_id in errors.iter().filter_map(target_value) {
                let Some(shape) = selected.model.shapes.get(error_id) else {
                    continue;
                };
                if serde_util_shape_needs_correction(shape) {
                    let name = rust_type_name(terminal(error_id));
                    render_serde_util_correction(
                        &mut output,
                        selected,
                        shape,
                        &format!(
                            "{}_correct_errors",
                            names::rust_module_name(terminal(error_id))
                        ),
                        &format!("crate::types::error::builders::{name}Builder"),
                    );
                }
            }
        }
    }

    for operation_name in &selected.operations {
        let Some(operation) = operation_shape(selected, operation_name) else {
            continue;
        };
        if let Some(output_shape) = operation
            .get("output")
            .and_then(target_value)
            .and_then(|id| selected.model.shapes.get(id))
        {
            for (_, member) in members(output_shape) {
                if is_xml_body_member(member)
                    && let Some(target) = member_target(member)
                {
                    serde_util_walk_shape(selected, target, &mut seen_types, &mut type_order);
                }
            }
        }
        if let Some(errors) = operation.get("errors").and_then(Value::as_array) {
            for error_id in errors.iter().filter_map(target_value) {
                if let Some(shape) = selected.model.shapes.get(error_id) {
                    for (_, member) in members(shape) {
                        if let Some(target) = member_target(member) {
                            serde_util_walk_shape(
                                selected,
                                target,
                                &mut seen_types,
                                &mut type_order,
                            );
                        }
                    }
                }
            }
        }
    }

    for shape_id in type_order {
        let Some(shape) = selected.model.shapes.get(&shape_id) else {
            continue;
        };
        if serde_util_shape_needs_correction(shape) {
            let name = rust_type_name(terminal(&shape_id));
            let builder_path = if is_error_shape(shape) {
                format!("crate::types::error::builders::{name}Builder")
            } else {
                format!("crate::types::builders::{name}Builder")
            };
            render_serde_util_correction(
                &mut output,
                selected,
                shape,
                &format!(
                    "{}_correct_errors",
                    names::rust_module_name(terminal(&shape_id))
                ),
                &builder_path,
            );
        }
    }
    output
}

fn serde_util_walk_shape(
    selected: &SelectedModel,
    shape_id: &str,
    seen: &mut BTreeSet<String>,
    order: &mut Vec<String>,
) {
    if !seen.insert(shape_id.to_owned()) {
        return;
    }
    let Some(shape) = selected.model.shapes.get(shape_id) else {
        return;
    };
    if matches!(
        shape.get("type").and_then(Value::as_str),
        Some("structure" | "union" | "list" | "map")
    ) {
        order.push(shape_id.to_owned());
    }
    match shape.get("type").and_then(Value::as_str) {
        Some("structure" | "union") => {
            for (_, member) in members(shape) {
                if let Some(target) = member_target(member) {
                    serde_util_walk_shape(selected, target, seen, order);
                }
            }
        }
        Some("list") => {
            if let Some(target) = shape.get("member").and_then(member_target) {
                serde_util_walk_shape(selected, target, seen, order);
            }
        }
        Some("map") => {
            for member in [shape.get("key"), shape.get("value")].into_iter().flatten() {
                if let Some(target) = member_target(member) {
                    serde_util_walk_shape(selected, target, seen, order);
                }
            }
        }
        _ => {}
    }
}

fn serde_util_shape_needs_correction(shape: &Value) -> bool {
    shape.get("type").and_then(Value::as_str) == Some("structure")
        && members(shape)
            .iter()
            .any(|(_, member)| member_is_required(member))
}

fn serde_util_builder_is_fallible(selected: &SelectedModel, shape: &Value) -> bool {
    members(shape).iter().any(|(_, member)| {
        let target = member_target(member).unwrap_or("smithy.api#String");
        member_is_effectively_required(selected, member, target)
    })
}

fn render_serde_util_correction(
    output: &mut String,
    selected: &SelectedModel,
    shape: &Value,
    function_name: &str,
    builder_path: &str,
) {
    writeln!(
        output,
        "pub(crate) fn {function_name}(\n    mut builder: {builder_path},\n) -> {builder_path} {{"
    )
    .unwrap();
    for (member_name, member) in members(shape) {
        if !member_is_required(member) {
            continue;
        }
        let Some(target) = member_target(member) else {
            continue;
        };
        let field = names::rust_identifier(&member_name);
        let kind = protocol_shape_kind(selected, target);
        writeln!(output, "    if builder.{field}.is_none() {{").unwrap();
        match kind {
            "structure" => {
                let nested_name = rust_type_name(terminal(target));
                let nested_builder = if selected
                    .model
                    .shapes
                    .get(target)
                    .is_some_and(is_error_shape)
                {
                    format!("crate::types::error::builders::{nested_name}Builder")
                } else {
                    format!("crate::types::builders::{nested_name}Builder")
                };
                let nested = selected.model.shapes.get(target);
                let result = if nested.is_some_and(serde_util_shape_needs_correction) {
                    format!(
                        "crate::serde_util::{}_correct_errors(builder)",
                        names::rust_module_name(terminal(target))
                    )
                } else {
                    "builder".to_owned()
                };
                let build = if nested
                    .is_some_and(|shape| serde_util_builder_is_fallible(selected, shape))
                {
                    ".build().ok()"
                } else {
                    ".build()"
                };
                let value = if nested
                    .is_some_and(|shape| serde_util_builder_is_fallible(selected, shape))
                {
                    format!("{result}{build}")
                } else {
                    format!("Some({result}{build})")
                };
                writeln!(
                output,
                "        builder.{field} = {{\n            let builder = {nested_builder}::default();\n            {value}\n        }}"
            )
            .unwrap();
            }
            "enum" => {
                writeln!(
                    output,
                    "        builder.{field} = \"no value was set\".parse::<crate::types::{}>().ok()",
                    rust_type_name(terminal(target))
                )
                .unwrap();
            }
            "timestamp" => {
                writeln!(
                    output,
                    "        builder.{field} = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))"
                )
                .unwrap();
            }
            _ => {
                writeln!(output, "        builder.{field} = Some(Default::default())").unwrap();
            }
        }
        output.push_str("    }\n");
    }
    output.push_str("    builder\n}\n\n");
}

fn render_s3_request_id() -> String {
    normalize_source(
        r#"// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_smithy_runtime_api::client::result::SdkError;
use aws_smithy_runtime_api::http::{Headers, Response};
use aws_smithy_types::error::metadata::{Builder as ErrorMetadataBuilder, ErrorMetadata};

const EXTENDED_REQUEST_ID: &str = "s3_extended_request_id";

/// Trait to retrieve the S3-specific extended request ID
///
/// Read more at <https://aws.amazon.com/premiumsupport/knowledge-center/s3-request-id-values/>.
pub trait RequestIdExt {
    /// Returns the S3 Extended Request ID necessary when contacting AWS Support.
    fn extended_request_id(&self) -> Option<&str>;
}

impl<E> RequestIdExt for SdkError<E, Response> {
    fn extended_request_id(&self) -> Option<&str> {
        match self {
            Self::ResponseError(err) => err.raw().headers().extended_request_id(),
            Self::ServiceError(err) => err.raw().headers().extended_request_id(),
            _ => None,
        }
    }
}

impl RequestIdExt for ErrorMetadata {
    fn extended_request_id(&self) -> Option<&str> {
        self.extra(EXTENDED_REQUEST_ID)
    }
}

impl<B> RequestIdExt for Response<B> {
    fn extended_request_id(&self) -> Option<&str> {
        self.headers().extended_request_id()
    }
}

impl RequestIdExt for Headers {
    fn extended_request_id(&self) -> Option<&str> {
        self.get("x-amz-id-2")
    }
}

impl<O, E> RequestIdExt for Result<O, E>
where
    O: RequestIdExt,
    E: RequestIdExt,
{
    fn extended_request_id(&self) -> Option<&str> {
        match self {
            Ok(ok) => ok.extended_request_id(),
            Err(err) => err.extended_request_id(),
        }
    }
}

/// Applies the extended request ID to a generic error builder
pub(crate) fn apply_extended_request_id(builder: ErrorMetadataBuilder, headers: &Headers) -> ErrorMetadataBuilder {
    if let Some(extended_request_id) = headers.extended_request_id() {
        builder.custom(EXTENDED_REQUEST_ID, extended_request_id)
    } else {
        builder
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use aws_smithy_runtime_api::client::result::SdkError;
    use aws_smithy_types::body::SdkBody;

    #[test]
    fn handle_missing_header() {
        let resp = Response::try_from(http_1x::Response::builder().status(400).body("").unwrap()).unwrap();
        let mut builder = ErrorMetadata::builder().message("123");
        builder = apply_extended_request_id(builder, resp.headers());
        assert_eq!(builder.build().extended_request_id(), None);
    }

    #[test]
    fn test_extended_request_id_sdk_error() {
        let without_extended_request_id = || Response::try_from(http_1x::Response::builder().body(SdkBody::empty()).unwrap()).unwrap();
        let with_extended_request_id = || {
            Response::try_from(
                http_1x::Response::builder()
                    .header("x-amz-id-2", "some-request-id")
                    .body(SdkBody::empty())
                    .unwrap(),
            )
            .unwrap()
        };
        assert_eq!(
            None,
            SdkError::<(), _>::response_error("test", without_extended_request_id()).extended_request_id()
        );
        assert_eq!(
            Some("some-request-id"),
            SdkError::<(), _>::response_error("test", with_extended_request_id()).extended_request_id()
        );
        assert_eq!(None, SdkError::service_error((), without_extended_request_id()).extended_request_id());
        assert_eq!(
            Some("some-request-id"),
            SdkError::service_error((), with_extended_request_id()).extended_request_id()
        );
    }

    #[test]
    fn test_extract_extended_request_id() {
        let mut headers = Headers::new();
        assert_eq!(None, headers.extended_request_id());

        headers.append("x-amz-id-2", "some-request-id");
        assert_eq!(Some("some-request-id"), headers.extended_request_id());
    }

    #[test]
    fn test_apply_extended_request_id() {
        let mut headers = Headers::new();
        assert_eq!(
            ErrorMetadata::builder().build(),
            apply_extended_request_id(ErrorMetadata::builder(), &headers).build(),
        );

        headers.append("x-amz-id-2", "some-request-id");
        assert_eq!(
            ErrorMetadata::builder().custom(EXTENDED_REQUEST_ID, "some-request-id").build(),
            apply_extended_request_id(ErrorMetadata::builder(), &headers).build(),
        );
    }

    #[test]
    fn test_error_metadata_extended_request_id_impl() {
        let err = ErrorMetadata::builder().custom(EXTENDED_REQUEST_ID, "some-request-id").build();
        assert_eq!(Some("some-request-id"), err.extended_request_id());
    }
}
"#,
    )
}

fn render_primitives(selected: &SelectedModel, consumer_namespace: bool) -> String {
    if !consumer_namespace {
        let mut output = String::new();
        client_operation_header(&mut output);
        if model_has_streaming(selected) {
            output.push_str(
                "pub use ::aws_smithy_types::body::SdkBody;\n\
                 pub use ::aws_smithy_types::byte_stream::error::Error as ByteStreamError;\n\
                 pub use ::aws_smithy_types::byte_stream::AggregatedBytes;\n\
                 pub use ::aws_smithy_types::byte_stream::ByteStream;\n\
                 #[cfg(feature = \"rt-tokio\")]\n\
                 pub use ::aws_smithy_types::byte_stream::FsBuilder;\n\
                 #[cfg(feature = \"rt-tokio\")]\n\
                 pub use ::aws_smithy_types::byte_stream::Length;\n\
                 ",
            );
        }
        output.push_str(
            "pub use ::aws_smithy_types::date_time::Format as DateTimeFormat;\n\
             pub use ::aws_smithy_types::Blob;\n\
             pub use ::aws_smithy_types::DateTime;\n\
             \n\
             /// Event stream related primitives such as `Message` or `Header`.\n\
             pub mod event_stream;\n",
        );
        if model_has_enum(selected) {
            output.push_str("\npub(crate) mod sealed_enum_unknown;\n");
        }
        return output;
    }
    let mut output = String::new();
    header(&mut output);
    output.push_str(
        "#[allow(dead_code, unused_imports, unused_variables)]\n\
         pub mod primitives {\n\
             pub type Blob = ::std::vec::Vec<u8>;\n\
             #[derive(Clone, Debug, Default, PartialEq, Eq)]\n\
             pub struct ByteStream(pub(crate) ::std::vec::Vec<u8>);\n\
             impl ByteStream {\n\
                 pub fn from_static(value: &'static [u8]) -> Self { Self(value.to_vec()) }\n\
                 pub fn from(value: impl Into<::std::vec::Vec<u8>>) -> Self { Self(value.into()) }\n\
                 pub async fn collect(&self) -> ::std::result::Result<AggregatedBytes, ::std::string::String> {\n\
                     Ok(AggregatedBytes(self.0.clone()))\n\
                 }\n\
                 pub fn into_inner(self) -> ::std::vec::Vec<u8> { self.0 }\n\
             }\n\
             impl From<::std::vec::Vec<u8>> for ByteStream {\n\
                 fn from(value: ::std::vec::Vec<u8>) -> Self { Self(value) }\n\
             }\n\
             impl From<&'static [u8]> for ByteStream {\n\
                 fn from(value: &'static [u8]) -> Self { Self::from_static(value) }\n\
             }\n\
             #[derive(Clone, Debug, Default, PartialEq, Eq)]\n\
             pub struct AggregatedBytes(pub(crate) ::std::vec::Vec<u8>);\n\
             impl AggregatedBytes {\n\
                 pub fn into_bytes(self) -> ::std::vec::Vec<u8> { self.0 }\n\
             }\n\
             pub type Document = ::std::string::String;\n\
             pub type DateTime = ::std::time::SystemTime;\n\
             pub(crate) mod sealed_enum_unknown {\n\
                 #[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]\n\
                 pub struct UnknownVariantValue(pub(crate) ::std::string::String);\n\
                 impl UnknownVariantValue {\n\
                     pub fn as_str(&self) -> &str { &self.0 }\n\
                 }\n\
                 impl ::std::fmt::Display for UnknownVariantValue {\n\
                     fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result { f.write_str(&self.0) }\n\
                 }\n\
             }\n\
         }\n\n",
    );
    output
}

fn render_event_stream_primitives(has_streaming: bool) -> String {
    let mut output = String::new();
    client_operation_header(&mut output);
    if has_streaming {
        output.push_str(
            "pub use crate::event_receiver::EventReceiver;\n\
             pub use ::aws_smithy_http::event_stream::EventStreamSender;\n\
             pub use ::aws_smithy_types::event_stream::Header;\n\
             pub use ::aws_smithy_types::event_stream::HeaderValue;\n\
             pub use ::aws_smithy_types::event_stream::Message;\n\
             pub use ::aws_smithy_types::str_bytes::StrBytes;\n",
        );
    }
    output
}

fn model_has_streaming(selected: &SelectedModel) -> bool {
    selected.model.shapes.values().any(|shape| {
        shape
            .get("traits")
            .and_then(Value::as_object)
            .is_some_and(|traits| traits.contains_key("smithy.api#streaming"))
    })
}

fn model_has_enum(selected: &SelectedModel) -> bool {
    selected
        .model
        .shapes
        .values()
        .any(|shape| shape.get("type").and_then(Value::as_str) == Some("enum"))
}

fn render_sealed_enum_unknown() -> String {
    let mut output = String::new();
    header(&mut output);
    output.push_str(
        "/// Opaque struct used as inner data for the `Unknown` variant defined in enums in\n\
         /// the crate.\n\
         ///\n\
         /// This is not intended to be used directly.\n\
         #[non_exhaustive]\n\
         #[derive(\n\
             ::std::clone::Clone, ::std::cmp::Eq, ::std::cmp::Ord, ::std::cmp::PartialEq, ::std::cmp::PartialOrd, ::std::fmt::Debug, ::std::hash::Hash,\n\
         )]\n\
         pub struct UnknownVariantValue(pub(crate) ::std::string::String);\n\
         impl UnknownVariantValue {\n\
             pub(crate) fn as_str(&self) -> &str {\n\
                 &self.0\n\
             }\n\
         }\n\
         impl ::std::fmt::Display for UnknownVariantValue {\n\
             fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {\n\
                 write!(f, \"{}\", self.0)\n\
             }\n\
         }\n",
    );
    output
}

fn render_config_file() -> String {
    let mut output = String::new();
    render_config(&mut output);
    output
}

fn render_config(output: &mut String) {
    header(output);
    output.push_str(
        "#[derive(Clone, Debug)]\n\
             pub struct Config {\n\
                 pub(crate) endpoint_url: ::std::string::String,\n\
             }\n\n\
             impl ::std::default::Default for Config {\n\
                 fn default() -> Self {\n\
                     Self {\n\
                         endpoint_url: ::std::env::var(\"AWS_ENDPOINT_URL\")\n\
                             .unwrap_or_else(|_| \"http://localhost:4566\".to_owned()),\n\
                     }\n\
                 }\n\
             }\n\n\
             pub mod config {\n\
                 #[derive(Clone, Debug, Default)]\n\
                 pub struct Builder {\n\
                     endpoint_url: ::std::option::Option<::std::string::String>,\n\
                 }\n\
                 impl Builder {\n\
                     pub fn endpoint_url(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {\n\
                         self.endpoint_url = Some(value.into());\n\
                         self\n\
                     }\n\
                     pub fn build(self) -> super::Config {\n\
                         super::Config {\n\
                             endpoint_url: self.endpoint_url.unwrap_or_else(|| super::Config::default().endpoint_url),\n\
                         }\n\
                     }\n\
                 }\n\
                 impl From<&super::Config> for Builder {\n\
                     fn from(config: &super::Config) -> Self {\n\
                         Self { endpoint_url: Some(config.endpoint_url.clone()) }\n\
                     }\n\
                 }\n\
             }\n\n\
             impl Config {\n\
                 pub fn builder() -> config::Builder { config::Builder::default() }\n\
             }\n\n",
    );
}

fn render_error_file(consumer_namespace: bool, has_enum: bool) -> String {
    let mut output = String::new();
    if consumer_namespace {
        render_legacy_error(&mut output);
    } else {
        render_standalone_error(&mut output, has_enum);
    }
    output
}

fn render_standalone_error(output: &mut String, has_enum: bool) {
    client_operation_header(output);
    output.push_str(
        "pub use ::aws_smithy_runtime_api::box_error::BoxError;\n\n\
         /// Error type returned by the client.\n\
         pub type SdkError<E, R = ::aws_smithy_runtime_api::client::orchestrator::HttpResponse> = ::aws_smithy_runtime_api::client::result::SdkError<E, R>;\n\
         pub use ::aws_smithy_runtime_api::client::result::ConnectorError;\n\
         pub use ::aws_smithy_types::error::operation::BuildError;\n\n\
         pub use ::aws_smithy_types::error::display::DisplayErrorContext;\n\
         pub use ::aws_smithy_types::error::metadata::ErrorMetadata;\n\
         pub use ::aws_smithy_types::error::metadata::ProvideErrorMetadata;\n",
    );
    if has_enum {
        output.push_str(
            "\n/// The given enum value failed to parse since it is not a known value.\n#[derive(Debug)]\npub struct UnknownVariantError {\n    value: ::std::string::String,\n}\nimpl UnknownVariantError {\n    pub(crate) fn new(value: impl ::std::convert::Into<::std::string::String>) -> Self {\n        Self { value: value.into() }\n    }\n}\nimpl ::std::fmt::Display for UnknownVariantError {\n    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::result::Result<(), ::std::fmt::Error> {\n        write!(f, \"unknown enum variant: '{}'\", self.value)\n    }\n}\nimpl ::std::error::Error for UnknownVariantError {}\n",
        );
    }
    output.push_str("\npub(crate) mod sealed_unhandled;\n");
}

fn render_legacy_error(output: &mut String) {
    header(output);
    output.push_str(
        "#[derive(Clone, Debug)]\n\
         pub struct Error { message: ::std::string::String }\n\
         impl Error {\n\
             pub fn unhandled(message: impl ::std::convert::Into<::std::string::String>) -> Self {\n\
                 Self { message: message.into() }\n\
             }\n\
             pub fn meta(&self) -> ErrorMetadata { ErrorMetadata::default() }\n\
         }\n\
         impl ::std::fmt::Display for Error {\n\
             fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {\n\
                 f.write_str(&self.message)\n\
             }\n\
         }\n\
         impl ::std::error::Error for Error {}\n\
         #[derive(Clone, Debug, Default)]\n\
         pub struct ErrorMetadata {\n\
             request_id: ::std::option::Option<::std::string::String>,\n\
             extended_request_id: ::std::option::Option<::std::string::String>,\n\
         }\n\
         impl ErrorMetadata {\n\
             pub(crate) fn from_request_ids(request_id: ::std::option::Option<::std::string::String>, extended_request_id: ::std::option::Option<::std::string::String>) -> Self {\n\
                 Self { request_id, extended_request_id }\n\
             }\n\
             pub fn request_id(&self) -> ::std::option::Option<&str> { self.request_id.as_deref() }\n\
             pub fn extended_request_id(&self) -> ::std::option::Option<&str> { self.extended_request_id.as_deref() }\n\
         }\n\
         #[derive(Clone, Debug)]\n\
         pub struct UnknownVariantError { value: ::std::string::String }\n\
         impl UnknownVariantError {\n\
             pub(crate) fn new(value: impl ::std::convert::Into<::std::string::String>) -> Self { Self { value: value.into() } }\n\
         }\n\
         impl ::std::fmt::Display for UnknownVariantError {\n\
             fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result { write!(f, \"unknown enum variant: '{}'\", self.value) }\n\
         }\n\
         impl ::std::error::Error for UnknownVariantError {}\n\
         pub mod error { pub use super::{BuildError, Error, ErrorMetadata, UnknownVariantError}; }\n\n",
    );
    output.push_str(
        "#[derive(Clone, Debug)]\n\
         pub struct BuildError { field: ::std::string::String, message: ::std::string::String }\n\
         impl BuildError {\n\
             pub fn missing_field(field: impl ::std::convert::Into<::std::string::String>, message: impl ::std::convert::Into<::std::string::String>) -> Self {\n\
                 Self { field: field.into(), message: message.into() }\n\
             }\n\
         }\n\
         impl ::std::fmt::Display for BuildError {\n\
             fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result { write!(f, \"{}: {}\", self.field, self.message) }\n\
         }\n\
         impl ::std::error::Error for BuildError {}\n\n",
    );
}

fn render_service_error_metadata(selected: &SelectedModel) -> String {
    let mut output = String::new();
    client_operation_header(&mut output);
    let error_ids = service_error_shape_ids(selected);
    output.push_str(
        "/// All possible error types for this service.\n#[non_exhaustive]\n#[derive(::std::fmt::Debug)]\npub enum Error {\n",
    );
    for error_id in &error_ids {
        let Some(shape) = selected.model.shapes.get(error_id) else {
            continue;
        };
        if let Some(documentation) = documentation(shape) {
            render_doc_lines(&mut output, &documentation, 4);
        } else {
            output.push_str("    #[allow(missing_docs)] // documentation missing in model\n");
        }
        render_deprecated_attribute(&mut output, shape, 4);
        writeln!(
            output,
            "    {}(crate::types::error::{}),",
            rust_type_name(terminal(error_id)),
            rust_type_name(terminal(error_id)),
        )
        .unwrap();
    }
    output.push_str(
        r###"    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
    variable wildcard pattern and check `.code()`:
     \
    &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
     \
    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-Error) for what information is available for the error.")]
    Unhandled(crate::error::sealed_unhandled::Unhandled),
}
"###,
    );

    output.push_str("impl ::std::fmt::Display for Error {\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n        match self {\n");
    for error_id in &error_ids {
        let name = rust_type_name(terminal(error_id));
        writeln!(output, "            Error::{name}(inner) => inner.fmt(f),").unwrap();
    }
    output.push_str(
        "            Error::Unhandled(_) => {\n                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {\n                    write!(f, \"unhandled error ({code})\")\n                } else {\n                    f.write_str(\"unhandled error\")\n                }\n            }\n        }\n    }\n}\n",
    );
    output.push_str(
        "impl From<::aws_smithy_types::error::operation::BuildError> for Error {\n    fn from(value: ::aws_smithy_types::error::operation::BuildError) -> Self {\n        Error::Unhandled(crate::error::sealed_unhandled::Unhandled {\n            source: value.into(),\n            meta: ::std::default::Default::default(),\n        })\n    }\n}\n",
    );
    output.push_str(
        "impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for Error {\n    fn meta(&self) -> &::aws_smithy_types::error::metadata::ErrorMetadata {\n        match self {\n",
    );
    for error_id in &error_ids {
        let name = rust_type_name(terminal(error_id));
        writeln!(output, "            Self::{name}(inner) => inner.meta(),").unwrap();
    }
    output.push_str("            Self::Unhandled(inner) => &inner.meta,\n        }\n    }\n}\n");

    let mut operations = selected.operations.clone();
    operations.sort_by_key(|operation| operation.to_ascii_lowercase());
    for operation_name in operations {
        let Some(operation) = operation_shape(selected, &operation_name) else {
            continue;
        };
        render_service_operation_error_conversions(&mut output, &operation_name, operation);
    }
    if has_waiters(selected) {
        output.push_str(
            "impl<O, E> ::std::convert::From<::aws_smithy_runtime_api::client::waiters::error::WaiterError<O, E>> for Error\nwhere\n    O: ::std::fmt::Debug + ::std::marker::Send + ::std::marker::Sync + 'static,\n    E: ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static,\n{\n    fn from(err: ::aws_smithy_runtime_api::client::waiters::error::WaiterError<O, E>) -> Self {\n        Error::Unhandled(crate::error::sealed_unhandled::Unhandled {\n            meta: ::std::default::Default::default(),\n            source: err.into(),\n        })\n    }\n}\n",
        );
    }
    for union_id in streaming_error_union_ids(selected) {
        let error_name = format!("{}Error", rust_type_name(terminal(&union_id)));
        render_service_event_stream_conversions(&mut output, &error_name);
    }

    output.push_str(
        "impl ::std::error::Error for Error {\n    fn source(&self) -> std::option::Option<&(dyn ::std::error::Error + 'static)> {\n        match self {\n",
    );
    for error_id in &error_ids {
        let name = rust_type_name(terminal(error_id));
        writeln!(
            output,
            "            Error::{name}(inner) => inner.source(),"
        )
        .unwrap();
    }
    output.push_str(
        "            Error::Unhandled(inner) => ::std::option::Option::Some(&*inner.source),\n        }\n    }\n}\n",
    );
    let request_id_plan = request_id_plan(selected);
    if request_id_plan.extended {
        output.push_str(
            "impl crate::s3_request_id::RequestIdExt for Error {\n    fn extended_request_id(&self) -> Option<&str> {\n        match self {\n",
        );
        for error_id in &error_ids {
            let name = rust_type_name(terminal(error_id));
            writeln!(
                output,
                "            Self::{name}(e) => e.extended_request_id(),"
            )
            .unwrap();
        }
        output.push_str(
            "            Self::Unhandled(e) => e.meta.extended_request_id(),\n        }\n    }\n}\n",
        );
    }
    if request_id_plan.standard {
        output.push_str(
            "impl ::aws_types::request_id::RequestId for Error {\n    fn request_id(&self) -> Option<&str> {\n        match self {\n",
        );
        for error_id in &error_ids {
            let name = rust_type_name(terminal(error_id));
            writeln!(output, "            Self::{name}(e) => e.request_id(),").unwrap();
        }
        output.push_str(
            "            Self::Unhandled(e) => e.meta.request_id(),\n        }\n    }\n}\n",
        );
    }
    output
}

fn render_service_operation_error_conversions(
    output: &mut String,
    operation_name: &str,
    operation: &Value,
) {
    let module = names::snake_case(operation_name);
    let error_path = format!(
        "crate::operation::{module}::{}Error",
        operation_error_type_name(operation_name)
    );
    writeln!(
        output,
        "impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<{error_path}, R>> for Error\nwhere\n    R: Send + Sync + std::fmt::Debug + 'static,\n{{\n    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<{error_path}, R>) -> Self {{\n        match err {{\n            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),\n            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {{\n                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),\n                source: err.into(),\n            }}),\n        }}\n    }}\n}}"
    )
    .unwrap();
    writeln!(
        output,
        "impl From<{error_path}> for Error {{\n    fn from(err: {error_path}) -> Self {{\n        match err {{"
    )
    .unwrap();
    if let Some(errors) = operation.get("errors").and_then(Value::as_array) {
        for error in errors.iter().filter_map(target_value) {
            let error_name = rust_type_name(terminal(error));
            writeln!(
                output,
                "            {error_path}::{error_name}(inner) => Error::{error_name}(inner),"
            )
            .unwrap();
        }
    }
    writeln!(
        output,
        "            {error_path}::Unhandled(inner) => Error::Unhandled(inner),\n        }}\n    }}\n}}"
    )
    .unwrap();
}

fn render_service_event_stream_conversions(output: &mut String, error_name: &str) {
    let error_path = format!("crate::types::error::{error_name}");
    writeln!(
        output,
        "impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<{error_path}, R>> for Error\nwhere\n    R: Send + Sync + std::fmt::Debug + 'static,\n{{\n    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<{error_path}, R>) -> Self {{\n        match err {{\n            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),\n            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {{\n                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),\n                source: err.into(),\n            }}),\n        }}\n    }}\n}}\nimpl From<{error_path}> for Error {{\n    fn from(err: {error_path}) -> Self {{\n        match err {{\n            {error_path}::Unhandled(inner) => Error::Unhandled(inner),\n        }}\n    }}\n}}"
    )
    .unwrap();
}

fn service_error_shape_ids(selected: &SelectedModel) -> Vec<String> {
    let mut ids = error_shape_ids(selected);
    ids.sort_by_key(|id| terminal(id).to_owned());
    ids
}

fn streaming_error_union_ids(selected: &SelectedModel) -> Vec<String> {
    let mut ids = selected
        .model
        .shapes
        .iter()
        .filter_map(|(id, shape)| {
            (shape.get("type").and_then(Value::as_str) == Some("union")
                && has_trait(shape, "smithy.api#streaming"))
            .then_some(id.clone())
        })
        .collect::<Vec<_>>();
    ids.sort();
    ids
}

fn render_meta(service_key: &str) -> String {
    let mut output = String::new();
    output.push_str(
        "// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.\n",
    );
    if matches!(service_key, "dynamodb" | "lambda") {
        writeln!(
            output,
            "pub(crate) static API_METADATA: ::aws_runtime::user_agent::ApiMetadata =\n    ::aws_runtime::user_agent::ApiMetadata::new(\"{service_key}\", crate::meta::PKG_VERSION);"
        )
        .unwrap();
    } else {
        writeln!(
            output,
            "pub(crate) static API_METADATA: ::aws_runtime::user_agent::ApiMetadata = ::aws_runtime::user_agent::ApiMetadata::new(\"{service_key}\", crate::meta::PKG_VERSION);"
        )
        .unwrap();
    }
    output.push_str("\n/// Crate version number.\npub static PKG_VERSION: &str = env!(\"CARGO_PKG_VERSION\");\n");
    output
}

fn render_aws_runtime() -> String {
    let mut output = String::new();
    header(&mut output);
    output.push_str(
        "#[allow(dead_code)]\n\
         pub(crate) mod transport {\n\
             use ::std::fmt;\n\
             use ::std::io::{Read, Write};\n\
             use ::std::net::TcpStream;\n\
             use ::std::collections::BTreeMap;\n\
\n\
             #[derive(Clone, Copy, Debug)]\n\
             pub(crate) enum Method { Get, Put, Post, Delete, Head, Patch }\n\
\n\
             impl Method {\n\
                 fn as_str(self) -> &'static str {\n\
                     match self {\n\
                         Self::Get => \"GET\",\n\
                         Self::Put => \"PUT\",\n\
                         Self::Post => \"POST\",\n\
                         Self::Delete => \"DELETE\",\n\
                         Self::Head => \"HEAD\",\n\
                         Self::Patch => \"PATCH\",\n\
                     }\n\
                 }\n\
             }\n\
\n\
             #[derive(Clone, Copy, Debug, PartialEq, Eq)]\n\
             pub(crate) struct StatusCode(u16);\n\
\n\
             impl StatusCode {\n\
                 pub(crate) const CONFLICT: Self = Self(409);\n\
                 pub(crate) fn is_success(self) -> bool { (200..300).contains(&self.0) }\n\
             }\n\
\n\
             impl fmt::Display for StatusCode {\n\
                 fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {\n\
                     self.0.fmt(formatter)\n\
                 }\n\
             }\n\
\n\
             #[derive(Clone, Debug)]\n\
             pub(crate) struct Response {\n\
                 status: StatusCode,\n\
                 headers: BTreeMap<String, String>,\n\
                 body: Vec<u8>,\n\
             }\n\
\n\
             impl Response {\n\
                 pub(crate) fn status(&self) -> StatusCode { self.status }\n\
                 pub(crate) fn header(&self, name: &str) -> Option<&str> {\n\
                     self.headers.get(&name.to_ascii_lowercase()).map(String::as_str)\n\
                 }\n\
                 pub(crate) fn body(&self) -> &[u8] { &self.body }\n\
             pub(crate) async fn text(&self) -> Result<String, String> {\n\
                 String::from_utf8(self.body.clone()).map_err(|error| error.to_string())\n\
                 }\n\
             }\n\
\n\
             #[derive(Clone, Debug, Default)]\n\
             pub(crate) struct HttpClient;\n\
\n\
             impl HttpClient {\n\
                 pub(crate) fn new() -> Self { Self }\n\
                 pub(crate) async fn request(\n\
                     &self,\n\
                     method: Method,\n\
                     url: &str,\n\
                     headers: &[(&str, &str)],\n\
                     body: &[u8],\n\
                 ) -> Result<Response, String> {\n\
                     let (host, port, path) = parse_http_url(url)?;\n\
                     let mut stream = TcpStream::connect((host.as_str(), port))\n\
                         .map_err(|error| format!(\"failed to connect to {host}:{port}: {error}\"))?;\n\
                     let mut request = format!(\"{} {} HTTP/1.1\\r\\nHost: {}\\r\\nConnection: close\\r\\nContent-Length: {}\\r\\n\", method.as_str(), path, host, body.len());\n\
                     for (name, value) in headers {\n\
                         request.push_str(name);\n\
                         request.push_str(\": \" );\n\
                         request.push_str(value);\n\
                         request.push_str(\"\\r\\n\");\n\
                     }\n\
                     request.push_str(\"\\r\\n\");\n\
                     let mut request_bytes = request.into_bytes();\n\
                     request_bytes.extend_from_slice(body);\n\
                     stream.write_all(&request_bytes).map_err(|error| format!(\"failed to write HTTP request: {error}\"))?;\n\
                     let mut bytes = Vec::new();\n\
                     stream.read_to_end(&mut bytes).map_err(|error| format!(\"failed to read HTTP response: {error}\"))?;\n\
                     parse_response(&bytes)\n\
                 }\n\
             }\n\
\n\
             fn parse_http_url(url: &str) -> Result<(String, u16, String), String> {\n\
                 let authority_and_path = url.strip_prefix(\"http://\").ok_or_else(|| format!(\"only http:// endpoints are supported: {url}\"))?;\n\
                 let (authority, path) = authority_and_path.split_once('/').map_or((authority_and_path, \"/\"), |(authority, _path)| (authority, &authority_and_path[authority.len()..]));\n\
                 if authority.is_empty() { return Err(format!(\"endpoint has no host: {url}\")); }\n\
                 let (host, port) = if let Some((host, port)) = authority.rsplit_once(':') {\n\
                     let port = port.parse::<u16>().map_err(|error| format!(\"invalid endpoint port in {url}: {error}\"))?;\n\
                     (host.to_owned(), port)\n\
                 } else {\n\
                     (authority.to_owned(), 80)\n\
                 };\n\
                 Ok((host, port, path.to_owned()))\n\
             }\n\
\n\
             fn parse_response(bytes: &[u8]) -> Result<Response, String> {\n\
                 let header_end = bytes.windows(4).position(|window| window == b\"\\r\\n\\r\\n\").ok_or_else(|| \"HTTP response did not contain a header terminator\".to_owned())?;\n\
                 let header = ::std::str::from_utf8(&bytes[..header_end]).map_err(|error| format!(\"HTTP response headers were not UTF-8: {error}\"))?;\n\
                 let status = header.lines().next().and_then(|line| line.split_whitespace().nth(1)).ok_or_else(|| \"HTTP response did not contain a status code\".to_owned())?.parse::<u16>().map_err(|error| format!(\"HTTP response status was invalid: {error}\"))?;\n\
                 let mut headers = BTreeMap::new();\n\
                 for line in header.lines().skip(1) {\n\
                     if let Some((name, value)) = line.split_once(':') {\n\
                         headers.insert(name.trim().to_ascii_lowercase(), value.trim().to_owned());\n\
                     }\n\
                 }\n\
                 Ok(Response { status: StatusCode(status), headers, body: bytes[header_end + 4..].to_vec() })\n\
             }\n\
\n\
             pub(crate) fn encode_path(value: &str) -> String {\n\
                 value.bytes().fold(String::new(), |mut result, byte| {\n\
                     if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~' | b'/') {\n\
                         result.push(byte as char);\n\
                     } else {\n\
                         result.push('%');\n\
                         result.push(hex(byte >> 4));\n\
                         result.push(hex(byte & 0x0f));\n\
                     }\n\
                     result\n\
                 })\n\
             }\n\
             fn hex(value: u8) -> char {\n\
                 match value { 0..=9 => (b'0' + value) as char, _ => (b'A' + value - 10) as char }\n\
             }\n\
             pub(crate) fn xml_escape(value: &str) -> String {\n\
                 value\n\
                     .replace('&', \"&amp;\")\n\
                     .replace('<', \"&lt;\")\n\
                     .replace('>', \"&gt;\")\n\
                     .replace('\\\"', \"&quot;\")\n\
                     .replace('\\'', \"&apos;\")\n\
             }\n\
             pub(crate) fn xml_unescape(value: &str) -> String {\n\
                 value\n\
                     .replace(\"&lt;\", \"<\")\n\
                     .replace(\"&gt;\", \">\")\n\
                     .replace(\"&apos;\", \"'\")\n\
                     .replace(\"&amp;\", \"&\")\n\
             }\n\
             pub(crate) fn xml_first(xml: &str, tag: &str) -> Option<String> {\n\
                 xml_tags(xml, tag).into_iter().next().map(|value| xml_unescape(&value))\n\
             }\n\
             pub(crate) fn xml_tags(xml: &str, tag: &str) -> Vec<String> {\n\
                 let open = format!(\"<{tag}>\");\n\
                 let close = format!(\"</{tag}>\");\n\
                 let mut values = Vec::new();\n\
                 let mut remaining = xml;\n\
                 while let Some(start) = remaining.find(&open) {\n\
                     let value_start = start + open.len();\n\
                     let Some(end) = remaining[value_start..].find(&close) else { break };\n\
                     values.push(remaining[value_start..value_start + end].to_owned());\n\
                     remaining = &remaining[value_start + end + close.len()..];\n\
                 }\n\
                 values\n\
             }\n\
         }\n\
\n",
    );
    output
}

fn render_observability_feature() -> String {
    normalize_source(
        r#"// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_smithy_runtime::client::sdk_feature::SmithySdkFeature;
use aws_smithy_runtime_api::{
    box_error::BoxError,
    client::interceptors::{context::BeforeSerializationInterceptorContextRef, dyn_dispatch_hint, Intercept},
};
use aws_smithy_types::config_bag::ConfigBag;

// Interceptor that tracks Smithy SDK features for observability (tracing/metrics).
#[derive(Debug, Default)]
pub(crate) struct ObservabilityFeatureTrackerInterceptor;

#[dyn_dispatch_hint]
impl Intercept for ObservabilityFeatureTrackerInterceptor {
    fn name(&self) -> &'static str {
        "ObservabilityFeatureTrackerInterceptor"
    }

    fn read_before_execution(&self, _context: &BeforeSerializationInterceptorContextRef<'_>, cfg: &mut ConfigBag) -> Result<(), BoxError> {
        // Check if an OpenTelemetry meter provider is configured via the global provider
        if let Ok(telemetry_provider) = aws_smithy_observability::global::get_telemetry_provider() {
            let meter_provider = telemetry_provider.meter_provider();

            // Use provider_name() to detect OpenTelemetry without importing the otel crate.
            if meter_provider.provider_name() == "AwsSmithyObservabilityOtelProvider" {
                cfg.interceptor_state().store_append(SmithySdkFeature::ObservabilityOtelMetrics);
            }
        }

        Ok(())
    }
}
"#,
    )
}

fn render_types_file(
    service_key: &str,
    selected: &SelectedModel,
    consumer_namespace: bool,
) -> String {
    let mut output = String::new();
    client_operation_header(&mut output);

    if consumer_namespace {
        let mut module_ids = selected
            .model
            .shapes
            .iter()
            .filter_map(|(id, shape)| {
                (id != selected.model.entry.service_shape_id
                    && is_file_renderable_type(Some(shape))
                    && !is_error_shape(shape)
                    && !is_synthetic_operation_shape(shape))
                .then_some(id.clone())
            })
            .collect::<Vec<_>>();
        module_ids.sort_by_key(|id| type_file_name(id));
        for id in module_ids {
            writeln!(
                output,
                "include!(concat!(env!(\"OUT_DIR\"), \"/generated/{service_key}/src/types/{}\"));",
                type_file_name(&id)
            )
            .unwrap();
        }
        writeln!(
            output,
            "pub mod builders {{\n    include!(concat!(env!(\"OUT_DIR\"), \"/generated/{service_key}/src/types/builders.rs\"));\n}}"
        )
        .unwrap();
        writeln!(
            output,
            "pub mod error {{\n    include!(concat!(env!(\"OUT_DIR\"), \"/generated/{service_key}/src/types/error.rs\"));\n}}"
        )
        .unwrap();
        return output;
    }

    for id in type_shape_order(selected) {
        let filename = type_file_name(&id);
        writeln!(
            output,
            "pub use crate::types::{module}::{name};\n",
            module = filename.trim_end_matches(".rs"),
            name = rust_type_name(terminal(&id)),
        )
        .unwrap();
    }

    let mut module_ids = selected
        .model
        .shapes
        .iter()
        .filter_map(|(id, shape)| {
            (id != selected.model.entry.service_shape_id
                && is_file_renderable_type(Some(shape))
                && !is_error_shape(shape)
                && !is_synthetic_operation_shape(shape))
            .then_some(id.clone())
        })
        .collect::<Vec<_>>();
    module_ids.sort_by_key(|id| type_file_name(id));
    for id in module_ids {
        let filename = type_file_name(&id);
        writeln!(output, "mod {};\n", filename.trim_end_matches(".rs")).unwrap();
    }

    output.push_str("/// Builders\npub mod builders;\n\n");
    writeln!(
        output,
        "/// Error types that {} can respond with.\npub mod error;",
        service_title(selected)
    )
    .unwrap();
    output
}

fn render_types_builders_file(selected: &SelectedModel, consumer_namespace: bool) -> String {
    let mut output = String::new();
    client_operation_header(&mut output);
    for id in type_shape_order(selected) {
        let Some(shape) = selected.model.shapes.get(&id) else {
            continue;
        };
        if shape.get("type").and_then(Value::as_str) != Some("structure") || is_error_shape(shape) {
            continue;
        }
        let name = rust_type_name(terminal(&id));
        let module = type_file_name(&id).trim_end_matches(".rs").to_owned();
        let path = if consumer_namespace {
            format!("super::{name}Builder")
        } else {
            format!("crate::types::{module}::{name}Builder")
        };
        writeln!(output, "pub use {path};\n").unwrap();
    }
    output
}

fn render_error_types_file(
    service_key: &str,
    selected: &SelectedModel,
    consumer_namespace: bool,
) -> String {
    let mut output = String::new();
    client_operation_header(&mut output);

    if consumer_namespace {
        let mut module_ids = error_shape_ids(selected);
        module_ids.sort_by_key(|id| type_file_name(id));
        for id in module_ids {
            writeln!(
                output,
                "include!(concat!(env!(\"OUT_DIR\"), \"/generated/{service_key}/src/types/error/{}\"));",
                type_file_name(&id)
            )
            .unwrap();
        }
        let mut event_stream_ids = selected
            .model
            .shapes
            .iter()
            .filter_map(|(id, shape)| {
                (shape.get("type").and_then(Value::as_str) == Some("union")
                    && has_trait(shape, "smithy.api#streaming"))
                .then_some(id.clone())
            })
            .collect::<Vec<_>>();
        event_stream_ids.sort();
        for id in event_stream_ids {
            render_event_stream_error(&mut output, selected, &id, consumer_namespace);
        }
        writeln!(
            output,
            "pub mod builders {{\n    include!(concat!(env!(\"OUT_DIR\"), \"/generated/{service_key}/src/types/error/builders.rs\"));\n}}"
        )
        .unwrap();
        return output;
    }

    for id in error_shape_ids(selected) {
        let name = rust_type_name(terminal(&id));
        let module = type_file_name(&id).trim_end_matches(".rs").to_owned();
        let path = if consumer_namespace {
            format!("super::{module}::{name}")
        } else {
            format!("crate::types::error::{module}::{name}")
        };
        writeln!(output, "pub use {path};\n").unwrap();
    }

    let mut event_stream_ids = selected
        .model
        .shapes
        .iter()
        .filter_map(|(id, shape)| {
            (shape.get("type").and_then(Value::as_str) == Some("union")
                && has_trait(shape, "smithy.api#streaming"))
            .then_some(id.clone())
        })
        .collect::<Vec<_>>();
    event_stream_ids.sort();
    for id in event_stream_ids {
        render_event_stream_error(&mut output, selected, &id, consumer_namespace);
    }

    let mut module_ids = error_shape_ids(selected);
    module_ids.sort_by_key(|id| type_file_name(id));
    for id in module_ids {
        writeln!(
            output,
            "mod {};\n",
            type_file_name(&id).trim_end_matches(".rs")
        )
        .unwrap();
    }
    output.push_str("/// Builders\npub mod builders;\n");
    output
}

fn render_event_stream_error(
    output: &mut String,
    selected: &SelectedModel,
    union_id: &str,
    consumer_namespace: bool,
) {
    let error_name = format!("{}Error", rust_type_name(terminal(union_id)));
    let module = selected.model.entry.module_name;
    let error_type_path = if consumer_namespace {
        format!("crate::{module}::types::error::{error_name}")
    } else {
        format!("crate::types::error::{error_name}")
    };
    let request_id_path = if consumer_namespace {
        format!("crate::{module}::s3_request_id")
    } else {
        "crate::s3_request_id".to_owned()
    };
    let template = r###"/// Error type for the `__ERROR_NAME__` operation.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum __ERROR_NAME__ {
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
    variable wildcard pattern and check `.code()`:
     \
    &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
     \
    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-__ERROR_NAME__) for what information is available for the error.")]
    Unhandled(crate::error::sealed_unhandled::Unhandled),
}
impl __ERROR_NAME__ {
    /// Creates the `__ERROR_NAME__::Unhandled` variant from any error type.
    pub fn unhandled(
        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
    ) -> Self {
        Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
            source: err.into(),
            meta: ::std::default::Default::default(),
        })
    }

    /// Creates the `__ERROR_NAME__::Unhandled` variant from an [`ErrorMetadata`](::aws_smithy_types::error::ErrorMetadata).
    pub fn generic(err: ::aws_smithy_types::error::ErrorMetadata) -> Self {
        Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
            source: err.clone().into(),
            meta: err,
        })
    }
    ///
    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    ///
    pub fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::Unhandled(e) => &e.meta,
        }
    }
}
impl ::std::error::Error for __ERROR_NAME__ {
    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Self::Unhandled(_inner) => ::std::option::Option::Some(&*_inner.source),
        }
    }
}
impl ::std::fmt::Display for __ERROR_NAME__ {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Unhandled(_inner) => {
                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
                    write!(f, "unhandled error ({code})")
                } else {
                    f.write_str("unhandled error")
                }
            }
        }
    }
}
impl ::aws_smithy_types::retry::ProvideErrorKind for __ERROR_NAME__ {
    fn code(&self) -> ::std::option::Option<&str> {
        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
        ::std::option::Option::None
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for __ERROR_NAME__ {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::Unhandled(_inner) => &_inner.meta,
        }
    }
}
impl ::aws_smithy_runtime_api::client::result::CreateUnhandledError for __ERROR_NAME__ {
    fn create_unhandled_error(
        source: ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
        meta: ::std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
    ) -> Self {
        Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
            source,
            meta: meta.unwrap_or_default(),
        })
    }
}
impl __REQUEST_ID_PATH__::RequestIdExt for __ERROR_TYPE_PATH__ {
    fn extended_request_id(&self) -> Option<&str> {
        self.meta().extended_request_id()
    }
}
impl ::aws_types::request_id::RequestId for __ERROR_TYPE_PATH__ {
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}

"###;
    output.push_str(
        &template
            .replace("__ERROR_NAME__", &error_name)
            .replace("__ERROR_TYPE_PATH__", &error_type_path)
            .replace("__REQUEST_ID_PATH__", &request_id_path),
    );
}

fn service_title(selected: &SelectedModel) -> String {
    selected
        .model
        .shapes
        .get(selected.model.entry.service_shape_id)
        .and_then(|shape| shape.get("traits"))
        .and_then(Value::as_object)
        .and_then(|traits| traits.get("smithy.api#title"))
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| terminal(selected.model.entry.service_shape_id).to_owned())
}

/// Return modeled public types in Smithy's first-discovery order.
///
/// Smithy normalizes every operation to synthetic input/output roots. The
/// writer then discovers shared types breadth-first from all of those roots;
/// the roots themselves are operation-owned and are therefore not re-exported
/// from `types`. Keeping discovery separate from the sorted module declarations
/// is what gives the facade its characteristic model-derived ordering.
fn type_shape_order(selected: &SelectedModel) -> Vec<String> {
    let mut queue = VecDeque::new();
    for operation_name in &selected.operation_order {
        let Some(operation) = operation_shape(selected, operation_name) else {
            continue;
        };
        if let Some(input) = operation.get("input").and_then(target_value) {
            queue.push_back(input.to_owned());
        }
        if let Some(output) = operation.get("output").and_then(target_value) {
            queue.push_back(output.to_owned());
        }
        if let Some(errors) = operation.get("errors").and_then(Value::as_array) {
            for error in errors.iter().filter_map(target_value) {
                queue.push_back(error.to_owned());
            }
        }
    }

    let mut scheduled = BTreeSet::new();
    let mut seen = BTreeSet::new();
    for id in queue.iter() {
        scheduled.insert(id.clone());
    }
    let mut order = Vec::new();
    while let Some(id) = queue.pop_front() {
        if !seen.insert(id.clone()) {
            continue;
        }
        let Some(shape) = selected.model.shapes.get(&id) else {
            continue;
        };
        for target in ordered_shape_targets(shape) {
            if !scheduled.insert(target.to_owned()) {
                continue;
            }
            let Some(target_shape) = selected.model.shapes.get(target) else {
                continue;
            };
            if is_file_renderable_type(Some(target_shape))
                && !is_error_shape(target_shape)
                && !is_synthetic_operation_shape(target_shape)
            {
                order.push(target.to_owned());
            }
            queue.push_back(target.to_owned());
        }
    }
    order
}

fn ordered_shape_targets(shape: &Value) -> Vec<&str> {
    match shape.get("type").and_then(Value::as_str) {
        Some("structure" | "union") => members(shape)
            .into_iter()
            .filter_map(|(_, member)| member_target(member))
            .collect(),
        Some("list") => shape
            .get("member")
            .and_then(|member| member_target(member))
            .into_iter()
            .collect(),
        Some("map") => [
            shape.get("key").and_then(member_target),
            shape.get("value").and_then(member_target),
        ]
        .into_iter()
        .flatten()
        .collect(),
        _ => Vec::new(),
    }
}

fn render_error_builders_file(selected: &SelectedModel, consumer_namespace: bool) -> String {
    let mut output = String::new();
    client_operation_header(&mut output);
    for id in error_shape_ids(selected) {
        let name = rust_type_name(terminal(&id));
        let module = type_file_name(&id).trim_end_matches(".rs").to_owned();
        let path = if consumer_namespace {
            format!("super::{name}Builder")
        } else {
            format!("crate::types::error::{module}::{name}Builder")
        };
        writeln!(output, "pub use {path};\n").unwrap();
    }
    output
}

/// Smithy visits modeled errors in the order they first occur in operation
/// `errors` lists. Preserve that model-derived order for the public error
/// module and its builder reexports.
fn error_shape_ids(selected: &SelectedModel) -> Vec<String> {
    let namespace = selected
        .model
        .entry
        .service_shape_id
        .split('#')
        .next()
        .unwrap_or_default();
    let mut seen = std::collections::BTreeSet::new();
    let mut error_ids = Vec::new();
    for operation_name in &selected.operations {
        let operation_id = format!("{namespace}#{operation_name}");
        let Some(operation) = selected.model.shapes.get(&operation_id) else {
            continue;
        };
        let Some(errors) = operation.get("errors").and_then(Value::as_array) else {
            continue;
        };
        for error in errors {
            let Some(id) = target_value(error) else {
                continue;
            };
            if selected.model.shapes.get(id).is_some_and(is_error_shape)
                && seen.insert(id.to_owned())
            {
                error_ids.push(id.to_owned());
            }
        }
    }
    for (id, shape) in &selected.model.shapes {
        if is_error_shape(shape) && seen.insert(id.clone()) {
            error_ids.push(id.clone());
        }
    }
    error_ids
}

fn operation_shape_ids(selected: &SelectedModel) -> std::collections::BTreeSet<String> {
    let namespace = selected
        .model
        .entry
        .service_shape_id
        .split('#')
        .next()
        .unwrap_or_default();
    selected
        .operations
        .iter()
        .filter_map(|operation_name| {
            let operation_id = format!("{namespace}#{operation_name}");
            selected.model.shapes.get(&operation_id)
        })
        .flat_map(|operation| {
            operation
                .get("input")
                .into_iter()
                .chain(operation.get("output"))
                .flat_map(|value| match value {
                    Value::Array(values) => values.iter().collect::<Vec<_>>(),
                    value => vec![value],
                })
                .filter_map(target_value)
                .filter(|id| {
                    selected
                        .model
                        .shapes
                        .get(*id)
                        .is_some_and(is_synthetic_operation_shape)
                })
                .map(ToOwned::to_owned)
                .collect::<Vec<_>>()
        })
        .collect()
}

fn is_synthetic_operation_shape(shape: &Value) -> bool {
    shape
        .get("traits")
        .and_then(Value::as_object)
        .is_some_and(|traits| {
            traits.contains_key("smithy.api.internal#syntheticInput")
                || traits.contains_key("smithy.api.internal#syntheticOutput")
        })
}

fn is_error_shape(shape: &Value) -> bool {
    shape
        .get("traits")
        .and_then(Value::as_object)
        .is_some_and(|traits| traits.contains_key("smithy.api#error"))
}

fn error_message_member(shape: &Value) -> Option<(String, &Value)> {
    members(shape)
        .into_iter()
        .find(|(name, _)| name.eq_ignore_ascii_case("message"))
}

fn is_error_context(context: &Context) -> bool {
    matches!(context, Context::Error { .. })
}

fn render_type_file(
    selected: &SelectedModel,
    shape_id: &str,
    context: Context,
    consumer_namespace: bool,
) -> String {
    let mut rendered = String::new();
    render_types_with_context(
        &mut rendered,
        selected,
        context,
        Some(shape_id),
        consumer_namespace,
    );
    let marker = "pub mod types {\n";
    let start = rendered.find(marker).expect("type module must be rendered") + marker.len();
    let end = rendered
        .rfind("\n}\n\n")
        .expect("type module must have a closing brace");
    let mut output = String::new();
    header(&mut output);
    for line in rendered[start..end].trim_end().lines() {
        output.push_str(line.strip_prefix("    ").unwrap_or(line));
        output.push('\n');
    }
    output
}

fn is_file_renderable_type(shape: Option<&Value>) -> bool {
    matches!(
        shape
            .and_then(|shape| shape.get("type"))
            .and_then(Value::as_str),
        Some("structure" | "union" | "enum")
    )
}

fn type_file_name(shape_id: &str) -> String {
    format!("_{}.rs", names::rust_module_name(terminal(shape_id)))
}

fn render_types_with_context(
    output: &mut String,
    selected: &SelectedModel,
    context: Context,
    only_shape: Option<&str>,
    consumer_namespace: bool,
) {
    header(output);
    output.push_str("pub mod types {\n");
    let mut ids = selected.model.shapes.keys().cloned().collect::<Vec<_>>();
    ids.sort();
    for id in ids {
        if id == selected.model.entry.service_shape_id
            || only_shape.is_some_and(|only| only != id.as_str())
        {
            continue;
        }
        let Some(shape) = selected.model.shapes.get(&id) else {
            continue;
        };
        match shape.get("type").and_then(Value::as_str) {
            Some("structure") => {
                render_structure(output, selected, shape, terminal(&id), context.clone())
            }
            Some("union") => render_union(output, selected, shape, terminal(&id), &context),
            Some("enum") => render_enum(output, shape, terminal(&id), &context, consumer_namespace),
            Some("list") => {
                let member = shape
                    .get("member")
                    .and_then(|member| member.get("target"))
                    .and_then(Value::as_str)
                    .map(|target| type_expr(selected, target, context.clone()))
                    .unwrap_or_else(|| "::std::string::String".to_owned());
                writeln!(
                    output,
                    "    pub type {} = ::std::vec::Vec<{}>;",
                    rust_type_name(terminal(&id)),
                    member
                )
                .unwrap();
            }
            Some("map") => {
                let key = shape
                    .get("key")
                    .and_then(|member| member.get("target"))
                    .and_then(Value::as_str)
                    .map(|target| type_expr(selected, target, context.clone()))
                    .unwrap_or_else(|| "::std::string::String".to_owned());
                let value = shape
                    .get("value")
                    .and_then(|member| member.get("target"))
                    .and_then(Value::as_str)
                    .map(|target| type_expr(selected, target, context.clone()))
                    .unwrap_or_else(|| "::std::string::String".to_owned());
                writeln!(
                    output,
                    "    pub type {} = ::std::collections::BTreeMap<{}, {}>;",
                    rust_type_name(terminal(&id)),
                    key,
                    value
                )
                .unwrap();
            }
            Some("string") | Some("integer") | Some("long") | Some("short") | Some("byte")
            | Some("float") | Some("double") | Some("boolean") | Some("blob")
            | Some("timestamp") | Some("document") => {
                writeln!(
                    output,
                    "    pub type {} = {};",
                    rust_type_name(terminal(&id)),
                    primitive_type(
                        shape
                            .get("type")
                            .and_then(Value::as_str)
                            .unwrap_or("string")
                    )
                )
                .unwrap();
            }
            _ => {}
        }
    }
    output.push_str("}\n\n");
}

fn render_operations_file(
    service_key: &str,
    service_module: &str,
    selected: &SelectedModel,
    consumer_namespace: bool,
) -> String {
    let mut output = String::new();
    header(&mut output);
    let request_id_plan = request_id_plan(selected);
    if request_id_plan.standard {
        output.push_str("pub use ::aws_types::request_id::RequestId;\n");
    }
    if request_id_plan.extended {
        let request_id_path = if consumer_namespace {
            format!("crate::{service_module}::s3_request_id::RequestIdExt")
        } else {
            "crate::s3_request_id::RequestIdExt".to_owned()
        };
        writeln!(output, "pub use {request_id_path};").unwrap();
    }
    output.push('\n');
    output.push_str("pub mod operation {\n");
    let mut operations = selected.operations.clone();
    operations.sort();
    for operation_name in operations {
        let module = names::snake_case(&operation_name);
        writeln!(output, "    pub mod {module} {{").unwrap();
        writeln!(
            output,
            "        include!(concat!(env!(\"OUT_DIR\"), \"/generated/{service_key}/src/operation/{module}.rs\"));"
        )
        .unwrap();
        output.push_str("    }\n");
    }
    output.push_str("}\n\n");
    output
}

fn render_operation_file(
    service_key: &str,
    selected: &SelectedModel,
    operation_name: &str,
    consumer_namespace: bool,
) -> String {
    if !consumer_namespace {
        return render_standalone_operation_file(selected, operation_name);
    }
    let module = names::snake_case(operation_name);
    let operation = operation_shape(selected, operation_name).expect("selected operation exists");
    let rust_operation = rust_type_name(operation_name);
    let operation_symbol = operation_name;
    let mut output = String::new();
    header(&mut output);
    writeln!(
        output,
        "#[derive(Clone, Debug, Default)]\npub struct {operation_symbol};\nimpl {operation_symbol} {{ pub fn new() -> Self {{ Self }} }}"
    )
    .unwrap();
    render_operation_error(&mut output, selected, operation, consumer_namespace);
    writeln!(
        output,
        "pub mod _{module}_input {{\n    include!(concat!(env!(\"OUT_DIR\"), \"/generated/{service_key}/src/operation/{module}/_{module}_input.rs\"));\n}}\npub use _{module}_input::{rust_operation}Input;\npub type Input = {rust_operation}Input;"
    )
    .unwrap();
    writeln!(
        output,
        "pub mod _{module}_output {{\n    include!(concat!(env!(\"OUT_DIR\"), \"/generated/{service_key}/src/operation/{module}/_{module}_output.rs\"));\n}}\npub use _{module}_output::{rust_operation}Output;\npub type Output = {rust_operation}Output;"
    )
    .unwrap();
    output.push_str("\n/// Builders\npub mod builders {\n");
    writeln!(
        output,
        "    include!(concat!(env!(\"OUT_DIR\"), \"/generated/{service_key}/src/operation/{module}/builders.rs\"));"
    )
    .unwrap();
    output.push_str("}\n");
    if operation_pagination_info(selected, operation_name).is_some() {
        if consumer_namespace {
            writeln!(
                output,
                "\n/// Paginator for this operation\npub mod paginator {{\n    include!(concat!(env!(\"OUT_DIR\"), \"/generated/{service_key}/src/operation/{module}/paginator.rs\"));\n}}\n"
            )
            .unwrap();
        } else {
            output.push_str("\n/// Paginator for this operation\npub mod paginator;\n");
        }
    }
    writeln!(
        output,
        "pub type {operation_symbol}Error = Error;\npub type {operation_symbol}FluentBuilder = builders::Builder;"
    )
    .unwrap();
    output
}

/// Render the operation root used by an all-operation SDK snapshot.
///
/// The consumer fixture intentionally uses the repository's small transport
/// runtime, while standalone snapshots follow the Smithy-RS operation-root
/// boundary. Every value in this renderer comes from Smithy HTTP, endpoint,
/// protocol, and auth traits; the renderer has no service or operation-name
/// branches.
fn render_standalone_operation_file(selected: &SelectedModel, operation_name: &str) -> String {
    let module = names::snake_case(operation_name);
    let operation = operation_shape(selected, operation_name).expect("selected operation exists");
    let operation_type = rust_type_name(operation_name);
    let service_id = service_sdk_id(selected);
    let input_path = format!("crate::operation::{module}::{operation_type}Input");
    let output_path = format!("crate::operation::{module}::{operation_type}Output");
    let error_path = format!(
        "crate::operation::{module}::{}Error",
        operation_error_type_name(operation_name)
    );
    let idempotency_member = operation
        .get("input")
        .and_then(target_value)
        .and_then(|id| selected.model.shapes.get(id))
        .and_then(|shape| {
            members(shape).into_iter().find_map(|(name, member)| {
                has_trait(member, "smithy.api#idempotencyToken").then_some(name)
            })
        })
        .map(|name| names::rust_identifier(&name));
    let mut output = String::new();
    client_operation_header(&mut output);
    writeln!(
        output,
        "/// Orchestration and serialization glue logic for `{operation_type}`.\n#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]\n#[non_exhaustive]\npub struct {operation_type};\nimpl {operation_type} {{\n    /// Creates a new `{operation_type}`\n    pub fn new() -> Self {{\n        Self\n    }}"
    )
    .unwrap();
    render_standalone_operation_orchestration(
        &mut output,
        operation_name,
        &service_id,
        &input_path,
        &output_path,
        &error_path,
        idempotency_member.as_deref(),
    );
    output.push_str("}\n");
    render_standalone_runtime_plugin(
        &mut output,
        selected,
        operation_name,
        operation,
        &module,
        &operation_type,
        &error_path,
    );
    render_standalone_telemetry_interceptor(&mut output, selected, operation_name, &operation_type);
    render_standalone_response_deserializer(
        &mut output,
        selected,
        operation_name,
        operation,
        &module,
        &error_path,
    );
    render_standalone_request_serializer(
        &mut output,
        selected,
        operation_name,
        operation,
        &module,
        &operation_type,
        &error_path,
    );
    render_standalone_endpoint_interceptor(&mut output, selected, operation_name, &operation_type);
    render_standalone_protocol_tests(
        &mut output,
        selected,
        operation_name,
        operation,
        &module,
        &operation_type,
    );
    render_standalone_operation_error(&mut output, selected, operation_name, operation);
    writeln!(
        output,
        "\npub use crate::operation::{module}::_{module}_input::{operation_type}Input;\n\npub use crate::operation::{module}::_{module}_output::{operation_type}Output;\n\nmod _{module}_input;\n\nmod _{module}_output;\n\n/// Builders\npub mod builders;"
    )
    .unwrap();
    if operation_pagination_info(selected, operation_name).is_some() {
        output.push_str("\n\n/// Paginator for this operation\npub mod paginator;");
    }
    output
}

fn service_sdk_id(selected: &SelectedModel) -> String {
    selected
        .model
        .shapes
        .get(selected.model.entry.service_shape_id)
        .and_then(|shape| shape.get("traits"))
        .and_then(|traits| traits.get("aws.api#service"))
        .and_then(|service| service.get("sdkId"))
        .and_then(Value::as_str)
        .unwrap_or_else(|| terminal(selected.model.entry.service_shape_id))
        .to_owned()
}

fn operation_http_trait(operation: &Value) -> Option<&Value> {
    operation
        .get("traits")
        .and_then(|traits| traits.get("smithy.api#http"))
}

fn operation_http_method(operation: &Value) -> &str {
    operation_http_trait(operation)
        .and_then(|http| http.get("method"))
        .and_then(Value::as_str)
        .unwrap_or("POST")
}

fn operation_success_code(operation: &Value) -> u64 {
    operation_http_trait(operation)
        .and_then(|http| http.get("code"))
        .and_then(Value::as_u64)
        .unwrap_or(200)
}

fn operation_http_uri(operation: &Value) -> &str {
    operation_http_trait(operation)
        .and_then(|http| http.get("uri"))
        .and_then(Value::as_str)
        .unwrap_or("/")
}

fn operation_http_checksum_trait(operation: &Value) -> Option<&Value> {
    operation
        .get("traits")
        .and_then(|traits| traits.get("aws.protocols#httpChecksum"))
}

fn operation_http_checksum_member<'a>(
    input_shape: Option<&'a Value>,
    checksum: &Value,
    trait_member: &str,
) -> Option<&'a Value> {
    let name = checksum.get(trait_member).and_then(Value::as_str)?;
    members(input_shape?)
        .into_iter()
        .find_map(|(member_name, member)| (member_name == name).then_some(member))
}

fn operation_has_unsigned_payload(operation: &Value) -> bool {
    has_trait(operation, "aws.auth#unsignedPayload")
}

fn operation_requires_aws_chunked(
    selected: &SelectedModel,
    operation: &Value,
    input_shape: Option<&Value>,
) -> bool {
    let Some(checksum) = operation_http_checksum_trait(operation) else {
        return false;
    };
    let Some(request_algorithm_member) =
        operation_http_checksum_member(input_shape, checksum, "requestAlgorithmMember")
    else {
        return false;
    };
    if request_algorithm_member
        .get("traits")
        .and_then(|traits| traits.get("smithy.api#httpHeader"))
        .and_then(Value::as_str)
        .is_none()
    {
        return false;
    }
    input_shape.is_some_and(|shape| {
        members(shape).into_iter().any(|(_, member)| {
            has_trait(member, "smithy.api#httpPayload")
                && member_target(member).is_some_and(|target| {
                    is_streaming_target(target)
                        || selected
                            .model
                            .shapes
                            .get(target)
                            .is_some_and(shape_is_streaming)
                })
        })
    })
}

fn operation_has_event_stream(selected: &SelectedModel, operation: &Value) -> bool {
    ["input", "output"].into_iter().any(|io| {
        operation
            .get(io)
            .and_then(target_value)
            .and_then(|id| selected.model.shapes.get(id))
            .is_some_and(|shape| {
                members(shape).into_iter().any(|(_, member)| {
                    has_trait(member, "smithy.api#httpPayload")
                        && member_target(member).is_some_and(|target| {
                            selected.model.shapes.get(target).is_some_and(|shape| {
                                shape_is_streaming(shape)
                                    && shape.get("type").and_then(Value::as_str) != Some("blob")
                            })
                        })
                })
            })
    })
}

fn operation_uses_stalled_stream_protection(selected: &SelectedModel, operation: &Value) -> bool {
    if operation_has_event_stream(selected, operation) {
        return false;
    }
    !has_trait(
        operation,
        "software.amazon.smithy.rust.codegen.client.smithy.traits#incompatibleWithStalledStreamProtectionTrait",
    )
}

fn service_supports_s3_express(selected: &SelectedModel) -> bool {
    selected
        .model
        .shapes
        .get(selected.model.entry.service_shape_id)
        .and_then(|shape| shape.get("traits"))
        .and_then(|traits| traits.get("smithy.rules#endpointRuleSet"))
        .is_some_and(|rules| value_contains_string(rules, "sigv4-s3express"))
}

fn value_contains_string(value: &Value, expected: &str) -> bool {
    match value {
        Value::String(value) => value == expected,
        Value::Array(values) => values
            .iter()
            .any(|value| value_contains_string(value, expected)),
        Value::Object(values) => values
            .values()
            .any(|value| value_contains_string(value, expected)),
        _ => false,
    }
}

fn operation_has_s3_expires_output(selected: &SelectedModel, operation: &Value) -> bool {
    operation
        .get("output")
        .and_then(target_value)
        .and_then(|id| selected.model.shapes.get(id))
        .is_some_and(|shape| {
            members(shape).into_iter().any(|(name, member)| {
                name.ends_with("String")
                    && member
                        .get("traits")
                        .and_then(|traits| traits.get("smithy.api#httpHeader"))
                        .and_then(Value::as_str)
                        .is_some_and(|header| header.eq_ignore_ascii_case("ExpiresString"))
            })
        })
}

#[derive(Clone, Debug)]
struct StandaloneUriLabel {
    format_name: String,
    field: String,
    variable: String,
    greedy: bool,
}

fn service_endpoint_parameter_names(selected: &SelectedModel) -> BTreeSet<String> {
    selected
        .model
        .shapes
        .get(selected.model.entry.service_shape_id)
        .and_then(|shape| shape.get("traits"))
        .and_then(|traits| traits.get("smithy.rules#endpointRuleSet"))
        .and_then(|rules| rules.get("parameters"))
        .and_then(Value::as_object)
        .map(|parameters| parameters.keys().cloned().collect())
        .unwrap_or_default()
}

fn standalone_uri_labels(
    selected: &SelectedModel,
    input_shape: Option<&Value>,
    path: &str,
) -> (String, Vec<StandaloneUriLabel>) {
    let endpoint_parameters = service_endpoint_parameter_names(selected);
    let mut labels = Vec::new();
    let mut rendered_path = String::new();
    for (index, segment) in path.split('/').enumerate() {
        let (is_label, label_name, greedy) = if segment.starts_with('{') && segment.ends_with('}') {
            let value = &segment[1..segment.len() - 1];
            (
                true,
                value.strip_suffix('+').unwrap_or(value),
                value.ends_with('+'),
            )
        } else {
            (false, "", false)
        };
        let member = if is_label {
            input_shape.and_then(|shape| {
                members(shape).into_iter().find(|(name, member)| {
                    names::rust_identifier(name) == names::rust_identifier(label_name)
                        && has_trait(member, "smithy.api#httpLabel")
                })
            })
        } else {
            None
        };
        let omit = index == 1
            && member.as_ref().is_some_and(|(_, member)| {
                member
                    .get("traits")
                    .and_then(|traits| traits.get("smithy.rules#contextParam"))
                    .and_then(|context| context.get("name"))
                    .and_then(Value::as_str)
                    .is_some_and(|name| endpoint_parameters.contains(name))
            });
        if index > 0 && !omit {
            rendered_path.push('/');
        }
        if !is_label {
            rendered_path.push_str(segment);
        } else if !omit {
            let (name, _) =
                member.unwrap_or_else(|| panic!("HTTP label `{label_name}` has no input member"));
            let field = names::rust_identifier(&name);
            labels.push(StandaloneUriLabel {
                format_name: label_name.to_owned(),
                field: field.clone(),
                variable: field,
                greedy,
            });
            rendered_path.push('{');
            rendered_path.push_str(label_name);
            rendered_path.push('}');
        }
    }
    if rendered_path.is_empty() {
        rendered_path.push('/');
    }
    (rendered_path, labels)
}

fn standalone_uri_label_body(path: &str, labels: &[StandaloneUriLabel]) -> String {
    let mut body = String::new();
    for (index, label) in labels.iter().enumerate() {
        writeln!(
            body,
            "                let input_{} = &_input.{};\n                let input_{} = input_{}.as_ref().ok_or_else(|| ::aws_smithy_types::error::operation::BuildError::missing_field(\"{}\", \"cannot be empty or unset\"))?;\n                let {} = ::aws_smithy_http::label::fmt_string(input_{}, ::aws_smithy_http::label::EncodingStrategy::{});\n                if {}.is_empty() {{\n                    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(\n                        \"{}\",\n                        \"cannot be empty or unset\",\n                    ));\n                }}",
            index + 1,
            label.field,
            index + 1,
            index + 1,
            label.field,
            label.variable,
            index + 1,
            if label.greedy { "Greedy" } else { "Default" },
            label.variable,
            label.field,
        )
        .unwrap();
    }
    let arguments = labels
        .iter()
        .map(|label| format!("{} = {}", label.format_name, label.variable))
        .collect::<Vec<_>>();
    if arguments.is_empty() {
        writeln!(
            body,
            "                ::std::write!(output, {path:?}).expect(\"formatting should succeed\");"
        )
        .unwrap();
    } else {
        writeln!(
            body,
            "                ::std::write!(output, {path:?}, {}).expect(\"formatting should succeed\");",
            arguments.join(", ")
        )
        .unwrap();
    }
    body
}

fn standalone_request_body(
    selected: &SelectedModel,
    operation_name: &str,
    input_shape: Option<&Value>,
) -> (String, Option<String>) {
    let module = names::rust_module_name(operation_name);
    let Some(input_shape) = input_shape else {
        return (
            "::aws_smithy_types::body::SdkBody::from(\"\")".to_owned(),
            None,
        );
    };
    if let Some((name, member)) = members(input_shape)
        .into_iter()
        .find(|(_, member)| has_trait(member, "smithy.api#httpPayload"))
    {
        let field = names::rust_identifier(&name);
        let target = member_target(member).unwrap_or_default();
        let target_shape = selected.model.shapes.get(target);
        let helper =
            format!("crate::protocol_serde::shape_{module}_input::ser_{field}_http_payload");
        if terminal(target) == "StreamingBlob" {
            return (
                format!("{helper}(input.{field})?.into_inner()"),
                Some(
                    target_shape
                        .and_then(shape_media_type)
                        .unwrap_or("application/octet-stream")
                        .to_owned(),
                ),
            );
        }
        let target_kind = target_shape
            .and_then(|shape| shape.get("type"))
            .and_then(Value::as_str);
        let payload = if matches!(target_kind, Some("string" | "blob")) {
            format!("{helper}(input.{field})?")
        } else {
            format!("{helper}(& input.{field})?")
        };
        let content_type = target_shape
            .and_then(shape_media_type)
            .or(match target_kind {
                Some("string") => Some("text/plain"),
                Some("blob") => Some("application/octet-stream"),
                _ => Some("application/xml"),
            })
            .map(str::to_owned);
        return (
            format!("::aws_smithy_types::body::SdkBody::from({payload})"),
            content_type,
        );
    }
    if members(input_shape).iter().any(|(_, member)| {
        is_xml_document_member(member) && !has_trait(member, "smithy.api#httpPayload")
    }) {
        let helper_module = if operation_shape(selected, operation_name)
            .is_some_and(|operation| operation_has_event_stream(selected, operation))
        {
            module.clone()
        } else {
            format!("{module}_input")
        };
        return (
            format!(
                "::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_{helper_module}::ser_{module}_op_input(& input)?)"
            ),
            Some("application/xml".to_owned()),
        );
    }
    let _ = selected;
    (
        "::aws_smithy_types::body::SdkBody::from(\"\")".to_owned(),
        None,
    )
}

fn shape_media_type(shape: &Value) -> Option<&str> {
    shape
        .get("traits")
        .and_then(|traits| traits.get("smithy.api#mediaType"))
        .and_then(Value::as_str)
}

fn render_standalone_operation_orchestration(
    output: &mut String,
    operation_name: &str,
    service_id: &str,
    input_path: &str,
    output_path: &str,
    error_path: &str,
    idempotency_member: Option<&str>,
) {
    let idempotency_plugin = idempotency_member
        .map(|field| {
            format!(
                "        runtime_plugins = runtime_plugins.with_operation_plugin(crate::client_idempotency_token::IdempotencyTokenRuntimePlugin::new(\n            |token_provider, input| {{\n                let input: &mut {input_path} = input.downcast_mut().expect(\"correct type\");\n                if input.{field}.is_none() {{\n                    input.{field} = ::std::option::Option::Some(token_provider.make_idempotency_token());\n                }}\n            }},\n        ));\n"
            )
        })
        .unwrap_or_default();
    writeln!(
        output,
        "    pub(crate) async fn orchestrate(\n        runtime_plugins: &::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,\n        input: {input_path},\n    ) -> ::std::result::Result<\n        {output_path},\n        ::aws_smithy_runtime_api::client::result::SdkError<\n            {error_path},\n            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,\n        >,\n    > {{\n        let map_err = |err: ::aws_smithy_runtime_api::client::result::SdkError<\n            ::aws_smithy_runtime_api::client::interceptors::context::Error,\n            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,\n        >| {{\n            err.map_service_error(|err| {{\n                                err.downcast::<{error_path}>().expect(\"correct error type\")\n                            }})\n        }};\n        let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)\n            .await\n            .map_err(map_err)?;\n        let output = context.finalize().map_err(map_err)?;\n        ::std::result::Result::Ok(output.downcast::<{output_path}>().expect(\"correct output type\"))\n    }}\n\n    pub(crate) async fn orchestrate_with_stop_point(\n        runtime_plugins: &::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,\n        input: {input_path},\n        stop_point: ::aws_smithy_runtime::client::orchestrator::StopPoint,\n    ) -> ::std::result::Result<\n        ::aws_smithy_runtime_api::client::interceptors::context::InterceptorContext,\n        ::aws_smithy_runtime_api::client::result::SdkError<\n            ::aws_smithy_runtime_api::client::interceptors::context::Error,\n            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,\n        >,\n    > {{\n        let input = ::aws_smithy_runtime_api::client::interceptors::context::Input::erase(input);\n        use ::tracing::Instrument;\n        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point({service_id:?}, {operation_name:?}, input, runtime_plugins, stop_point)\n            // Create a parent span for the entire operation. Includes a random, internal-only,\n            // seven-digit ID for the operation orchestration so that it can be correlated in the logs.\n            .instrument(::tracing::debug_span!(\n                \"{service_id}.{operation_name}\",\n                \"rpc.service\" = {service_id:?},\n                \"rpc.method\" = {operation_name:?},\n                \"sdk_invocation_id\" = ::fastrand::u32(1_000_000..10_000_000),\n                \"rpc.system\" = \"aws-api\",\n            ))\n            .await\n    }}\n\n    pub(crate) fn operation_runtime_plugins(\n        client_runtime_plugins: ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,\n        client_config: &crate::config::Config,\n        config_override: ::std::option::Option<crate::config::Builder>,\n    ) -> ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins {{\n        let mut runtime_plugins = client_runtime_plugins.with_operation_plugin(Self::new());\n\n        if let ::std::option::Option::Some(config_override) = config_override {{\n            for plugin in config_override.runtime_plugins.iter().cloned() {{\n                runtime_plugins = runtime_plugins.with_operation_plugin(plugin);\n            }}\n            runtime_plugins = runtime_plugins.with_operation_plugin(crate::config::ConfigOverrideRuntimePlugin::new(\n                config_override,\n                client_config.config.clone(),\n                &client_config.runtime_components,\n            ));\n        }}\n        runtime_plugins\n    }}"
    )
    .unwrap();
    if !idempotency_plugin.is_empty() {
        let marker = "        let mut runtime_plugins = client_runtime_plugins.with_operation_plugin(Self::new());\n\n        if let ::std::option::Option::Some(config_override) = config_override {";
        let replacement = format!(
            "        let mut runtime_plugins = client_runtime_plugins.with_operation_plugin(Self::new());\n{}        if let ::std::option::Option::Some(config_override) = config_override {{",
            idempotency_plugin
        );
        *output = output.replace(marker, &replacement);
    }
}

fn render_standalone_runtime_plugin(
    output: &mut String,
    selected: &SelectedModel,
    operation_name: &str,
    operation: &Value,
    module: &str,
    operation_type: &str,
    error_path: &str,
) {
    let service_id = service_sdk_id(selected);
    let input_shape = operation
        .get("input")
        .and_then(target_value)
        .and_then(|id| selected.model.shapes.get(id));
    let output_shape = operation
        .get("output")
        .and_then(target_value)
        .and_then(|id| selected.model.shapes.get(id));
    let stalled_stream_protection = operation_uses_stalled_stream_protection(selected, operation);
    let sensitive_output =
        output_shape.is_some_and(|shape| structure_has_sensitive_member(selected, shape));
    let checksum = operation_http_checksum_trait(operation);
    let request_checksum_required = checksum
        .and_then(|checksum| checksum.get("requestChecksumRequired"))
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let request_checksum_member_name = checksum
        .and_then(|checksum| checksum.get("requestAlgorithmMember"))
        .and_then(Value::as_str);
    let request_checksum_member = checksum.and_then(|checksum| {
        operation_http_checksum_member(input_shape, checksum, "requestAlgorithmMember")
    });
    let request_checksum_header = request_checksum_member.and_then(|member| {
        member
            .get("traits")
            .and_then(|traits| traits.get("smithy.api#httpHeader"))
            .and_then(Value::as_str)
    });
    let response_checksum_member_name = checksum
        .and_then(|checksum| checksum.get("requestValidationModeMember"))
        .and_then(Value::as_str);
    let response_checksum_member = checksum.and_then(|checksum| {
        operation_http_checksum_member(input_shape, checksum, "requestValidationModeMember")
    });
    let response_checksum_type = response_checksum_member
        .and_then(member_target)
        .map(|target| format!("crate::types::{}", rust_type_name(terminal(target))));
    let unsigned_payload = operation_has_unsigned_payload(operation);

    let mut config_extras = String::new();
    if request_checksum_required && service_supports_s3_express(selected) {
        config_extras.push_str(
            "        cfg.store_put(crate::s3_express::checksum::provide_default_checksum_algorithm());\n",
        );
    }

    let mut additional_interceptors = String::new();
    if operation_has_s3_expires_output(selected, operation) {
        additional_interceptors.push_str(
            "            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(\n                crate::s3_expires_interceptor::S3ExpiresInterceptor,\n            ))\n",
        );
    }
    if let (Some(request_checksum_member_name), Some(request_checksum_header)) =
        (request_checksum_member_name, request_checksum_header)
    {
        let input_type = format!("crate::operation::{module}::{operation_type}Input");
        let field = names::rust_identifier(request_checksum_member_name);
        let required = request_checksum_required.to_string();
        let header = request_checksum_header;
        let request_interceptor = r#".with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(crate::http_request_checksum::RequestChecksumInterceptor::new(
                                |input: &::aws_smithy_runtime_api::client::interceptors::context::Input| {
                                    let input: &__INPUT_TYPE__ = input.downcast_ref().expect("correct type");
                                    let checksum_algorithm = input.__FIELD__();
                                    let checksum_algorithm = checksum_algorithm.map(|algorithm| algorithm.as_str());
                                    (checksum_algorithm.map(|s| s.to_string()), __REQUIRED__)
                                },
                                |request: &mut ::aws_smithy_runtime_api::http::Request, cfg: &::aws_smithy_types::config_bag::ConfigBag| {
                                    // We check if the user has set any of the checksum values manually
                                    let mut user_set_checksum_value = false;
                                    let headers_to_check = request.headers().iter().filter_map(|(name, _val)| {
                                        if name.starts_with("x-amz-checksum-") {
                                            Some(name)
                                        } else {
                                            None
                                        }
                                    });
                                    for algo_header in headers_to_check {
                                        if request.headers().get(algo_header).is_some() {
                                            user_set_checksum_value = true;
                                        }
                                    }

                                    // We check if the user set the checksum algo manually
                                    let user_set_checksum_algo = request.headers()
                                        .get("__HEADER__")
                                        .is_some();

                                    // This value is set by the user on the SdkConfig to indicate their preference
                                    let request_checksum_calculation = cfg
                                        .load::<::aws_smithy_types::checksum_config::RequestChecksumCalculation>()
                                        .unwrap_or(&::aws_smithy_types::checksum_config::RequestChecksumCalculation::WhenSupported);

                                    // From the httpChecksum trait
                                    let http_checksum_required = __REQUIRED__;

                                    let is_presigned_req = cfg.load::<crate::presigning::PresigningMarker>().is_some();

                                    // If the request is presigned we do not set a default.
                                    // If the RequestChecksumCalculation is WhenSupported and the user has not set a checksum value or algo
                                    // we set the default. If it is WhenRequired and a checksum is required by the trait and the user has not
                                    // set a checksum value or algo we also set the default. In all other cases we do nothing.
                                    match (
                                        request_checksum_calculation,
                                        http_checksum_required,
                                        user_set_checksum_value,
                                        user_set_checksum_algo,
                                        is_presigned_req,
                                    ) {
                                        (_, _, _, _, true) => {}
                                        (::aws_smithy_types::checksum_config::RequestChecksumCalculation::WhenSupported, _, false, false, _)
                                        | (::aws_smithy_types::checksum_config::RequestChecksumCalculation::WhenRequired, true, false, false, _) => {
                                            request.headers_mut().insert("__HEADER__", "CRC32");
                                        }
                                        _ => {},
                                    }

                                    // We return a bool indicating if the user did set the checksum value, if they did
                                    // we can short circuit and exit the interceptor early.
                                    Ok(user_set_checksum_value)
                                }
                                )))
"#
        .replace("__INPUT_TYPE__", &input_type)
        .replace("__FIELD__", &field)
        .replace("__REQUIRED__", &required)
        .replace("__HEADER__", header);
        additional_interceptors.push_str(&request_interceptor);
    }
    if let (Some(validation_member_name), Some(validation_type)) = (
        response_checksum_member_name,
        response_checksum_type.as_deref(),
    ) {
        let field = names::rust_identifier(validation_member_name);
        let algorithms = checksum
            .and_then(|checksum| checksum.get("responseAlgorithms"))
            .and_then(Value::as_array)
            .map(|algorithms| {
                algorithms
                    .iter()
                    .filter_map(Value::as_str)
                    .map(|algorithm| format!("{algorithm:?}").to_ascii_lowercase())
                    .collect::<Vec<_>>()
                    .join(", ")
            })
            .unwrap_or_default();
        if !algorithms.is_empty() {
            let input_type = format!("crate::operation::{module}::{operation_type}Input");
            let response_interceptor = r#".with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(crate::http_response_checksum::ResponseChecksumInterceptor::new(
                                [__ALGORITHMS__].as_slice(),
                                |input: &::aws_smithy_runtime_api::client::interceptors::context::Input| {

                                    let input: &__INPUT_TYPE__ = input.downcast_ref().expect("correct type");
                                    matches!(input.__FIELD__(), ::std::option::Option::Some(__VALIDATION_TYPE__::Enabled))
                                },
                                |input: &mut ::aws_smithy_runtime_api::client::interceptors::context::Input, cfg: &::aws_smithy_types::config_bag::ConfigBag|  {
                                    let input = input
                                        .downcast_mut::<__INPUT_TYPE__>()
                                        .ok_or("failed to downcast to __INPUT_TYPE__")?;

                                    let request_validation_enabled =
                                        matches!(input.__FIELD__(), Some(__VALIDATION_TYPE__::Enabled));

                                    if !request_validation_enabled {
                                        // This value is set by the user on the SdkConfig to indicate their preference
                                        let response_checksum_validation = cfg
                                            .load::<::aws_smithy_types::checksum_config::ResponseChecksumValidation>()
                                            .unwrap_or(&::aws_smithy_types::checksum_config::ResponseChecksumValidation::WhenSupported);

                                        let is_presigned_req = cfg.load::<crate::presigning::PresigningMarker>().is_some();

                                        // For presigned requests we do not enable the checksum-mode header.
                                        if is_presigned_req {
                                            return ::std::result::Result::Ok(())
                                        }

                                        // If validation setting is WhenSupported (or unknown) we enable response checksum
                                        // validation. If it is WhenRequired we do not enable (since there is no way to
                                        // indicate that a response checksum is required).
                                        #[allow(clippy::wildcard_in_or_patterns)]
                                        match response_checksum_validation {
                                            ::aws_smithy_types::checksum_config::ResponseChecksumValidation::WhenRequired => {}
                                            ::aws_smithy_types::checksum_config::ResponseChecksumValidation::WhenSupported | _ => {
                                                input.__FIELD__ = Some(__VALIDATION_TYPE__::Enabled);
                                            }
                                        }
                                    }

                                    ::std::result::Result::Ok(())
                                }
                            )))
"#
            .replace("__ALGORITHMS__", &algorithms)
            .replace("__INPUT_TYPE__", &input_type)
            .replace("__FIELD__", &field)
            .replace("__VALIDATION_TYPE__", validation_type);
            additional_interceptors.push_str(&response_interceptor);
        }
    }
    if operation_requires_aws_chunked(selected, operation, input_shape) {
        additional_interceptors.push_str(
            ".with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(crate::aws_chunked::AwsChunkedContentEncodingInterceptor))\n",
        );
    }
    writeln!(
        output,
        "impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for {operation_type} {{\n    fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {{\n        let mut cfg = ::aws_smithy_types::config_bag::Layer::new({operation_name:?});\n\n        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(\n            {operation_type}RequestSerializer,\n        ));\n        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(\n            {operation_type}ResponseDeserializer,\n        ));\n\n        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(\n            crate::config::auth::Params::builder()\n                .operation_name({operation_name:?})\n                .build()\n                .expect(\"required fields set\"),\n        ));\n"
    )
    .unwrap();
    if sensitive_output {
        output.push_str("        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::SensitiveOutput);\n");
    }
    writeln!(
        output,
        "        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new({operation_name:?}, {service_id:?}));\n{config_extras}        let mut signing_options = ::aws_runtime::auth::SigningOptions::default();\n        signing_options.double_uri_encode = {unsigned_payload};\n        signing_options.content_sha256_header = true;\n        signing_options.normalize_uri_path = false;\n        signing_options.payload_override = {payload_override};\n\n        cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {{\n            signing_options,\n            ..::std::default::Default::default()\n        }});\n\n        ::std::option::Option::Some(cfg.freeze())\n    }}\n\n    fn runtime_components(\n        &self,\n        _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,\n    ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {{\n        #[allow(unused_mut)]\n                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new({operation_name:?})\n                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent({operation_type}TelemetryInputCaptureInterceptor))\n.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))\n.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent({operation_type}EndpointParamsInterceptor))\n{additional_interceptors}                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<{error_path}>::new())\n.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<{error_path}>::new())\n.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<{error_path}>::builder().transient_errors({{\n                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();\n                                            transient_errors.push(\"InternalError\");\n                                            ::std::borrow::Cow::Owned(transient_errors)\n                                            }}).build());\n\n        ::std::borrow::Cow::Owned(rcb)\n    }}\n}}",
        unsigned_payload = unsigned_payload,
        payload_override = if unsigned_payload {
            "Some(::aws_sigv4::http_request::SignableBody::UnsignedPayload)"
        } else {
            "None"
        },
    )
    .unwrap();
    if !stalled_stream_protection {
        *output = output.replace(
            ".with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))\n",
            "",
        );
    }
}

fn render_standalone_telemetry_interceptor(
    output: &mut String,
    selected: &SelectedModel,
    operation_name: &str,
    operation_type: &str,
) {
    let input_shape = operation_shape(selected, operation_name)
        .and_then(|operation| operation.get("input"))
        .and_then(target_value)
        .and_then(|id| selected.model.shapes.get(id));
    writeln!(
        output,
        "\n#[derive(Debug)]\nstruct {operation_type}TelemetryInputCaptureInterceptor;\n\n#[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]\nimpl ::aws_smithy_runtime_api::client::interceptors::Intercept for {operation_type}TelemetryInputCaptureInterceptor {{\n    fn name(&self) -> &'static str {{\n        \"{operation_type}TelemetryInputCaptureInterceptor\"\n    }}\n\n    fn read_before_execution(\n        &self,\n        context: &::aws_smithy_runtime_api::client::interceptors::context::BeforeSerializationInterceptorContextRef<\n            '_ ,\n            ::aws_smithy_runtime_api::client::interceptors::context::Input,\n            ::aws_smithy_runtime_api::client::interceptors::context::Output,\n            ::aws_smithy_runtime_api::client::interceptors::context::Error,\n        >,\n        cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,\n    ) -> ::std::result::Result<(), ::aws_smithy_runtime_api::box_error::BoxError> {{\n        // Nothing to do unless the customer opted in by naming members to record.\n        let ::std::option::Option::Some(requested) = cfg\n            .load::<::aws_smithy_types::telemetry::RequestedTelemetryAttributes>()\n            .filter(|r| !r.is_empty())\n        else {{\n            return ::std::result::Result::Ok(());\n        }};\n\n        let ::std::option::Option::Some(input) = context.input().downcast_ref::<{operation_type}Input>() else {{\n            // A mismatched input is not this interceptor's concern; skip quietly.\n            return ::std::result::Result::Ok(());\n        }};\n\n        let mut captured = ::aws_smithy_types::telemetry::CapturedTelemetryAttributes::default();"
    )
    .unwrap();
    if let Some(shape) = input_shape {
        for (name, member) in members(shape) {
            let Some(target) = member_target(member) else {
                continue;
            };
            if !is_string_type(target, selected.model.shapes.get(target))
                || has_trait(member, "smithy.api#sensitive")
                || selected
                    .model
                    .shapes
                    .get(target)
                    .is_some_and(|shape| has_trait(shape, "smithy.api#sensitive"))
            {
                continue;
            }
            let field = names::rust_identifier(&name);
            writeln!(
                output,
                "        if requested.should_capture({name:?}) {{\n            if let ::std::option::Option::Some(value) = input.{field}.as_deref() {{\n                captured.insert({name:?}, value);\n            }}\n        }}"
            )
            .unwrap();
        }
    }
    output.push_str("\n        cfg.interceptor_state().store_put(captured);\n        ::std::result::Result::Ok(())\n    }\n}\n");
}

fn render_standalone_response_deserializer(
    output: &mut String,
    selected: &SelectedModel,
    operation_name: &str,
    operation: &Value,
    module: &str,
    error_path: &str,
) {
    let code = operation_success_code(operation);
    let streaming_output = operation
        .get("output")
        .and_then(target_value)
        .and_then(|id| selected.model.shapes.get(id))
        .is_some_and(|shape| {
            members(shape).into_iter().any(|(_, member)| {
                has_trait(member, "smithy.api#httpPayload")
                    && member_target(member).is_some_and(|target| {
                        selected
                            .model
                            .shapes
                            .get(target)
                            .is_some_and(shape_is_streaming)
                    })
            })
        });
    if streaming_output {
        render_standalone_streaming_response_deserializer(output, module, operation_name, code);
        return;
    }
    let force_error = request_id_plan(selected).extended;
    writeln!(
        output,
        "#[derive(Debug)]\nstruct {operation_name}ResponseDeserializer;\nimpl ::aws_smithy_runtime_api::client::ser_de::DeserializeResponse for {operation_name}ResponseDeserializer {{\n    fn deserialize_nonstreaming_with_config(\n        &self,\n        response: &::aws_smithy_runtime_api::client::orchestrator::HttpResponse,\n        _cfg: &::aws_smithy_types::config_bag::ConfigBag,\n    ) -> ::aws_smithy_runtime_api::client::interceptors::context::OutputOrError {{\n        let (success, status) = (response.status().is_success(), response.status().as_u16());\n        let headers = response.headers();\n        let body = response.body().bytes().expect(\"body loaded\");\n        #[allow(unused_mut)]\n        let mut force_error = false;"
    )
    .unwrap();
    if force_error {
        output.push_str("        ::tracing::debug!(extended_request_id = ?crate::s3_request_id::RequestIdExt::extended_request_id(response));\n        if matches!(crate::rest_xml_unwrapped_errors::body_is_error(body), Ok(true)) {\n            force_error = true;\n        }");
    }
    output.push_str("        ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));\n");
    writeln!(
        output,
        "        let parse_result = if !success && status != {code} || force_error {{\n            crate::protocol_serde::shape_{module}::de_{module}_http_error(status, headers, body)\n        }} else {{\n            crate::protocol_serde::shape_{module}::de_{module}_http_response(status, headers, body)\n        }};\n        crate::protocol_serde::type_erase_result(parse_result)\n    }}\n}}"
    )
    .unwrap();
    let _ = error_path;
}

fn render_standalone_streaming_response_deserializer(
    output: &mut String,
    module: &str,
    operation_name: &str,
    code: u64,
) {
    writeln!(
        output,
        "#[derive(Debug)]
struct {operation_name}ResponseDeserializer;
impl ::aws_smithy_runtime_api::client::ser_de::DeserializeResponse for {operation_name}ResponseDeserializer {{
    fn deserialize_streaming(
        &self,
        response: &mut ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
    ) -> ::std::option::Option<::aws_smithy_runtime_api::client::interceptors::context::OutputOrError> {{
        #[allow(unused_mut)]
        let mut force_error = false;
        ::tracing::debug!(extended_request_id = ?crate::s3_request_id::RequestIdExt::extended_request_id(response));
        ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));

        // If this is an error, defer to the non-streaming parser
        if (!response.status().is_success() && response.status().as_u16() != {code}) || force_error {{
            return ::std::option::Option::None;
        }}
        ::std::option::Option::Some(crate::protocol_serde::type_erase_result(
            crate::protocol_serde::shape_{module}::de_{module}_http_response(response),
        ))
    }}

    fn deserialize_nonstreaming_with_config(
        &self,
        response: &::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        _cfg: &::aws_smithy_types::config_bag::ConfigBag,
    ) -> ::aws_smithy_runtime_api::client::interceptors::context::OutputOrError {{
        // For streaming operations, we only hit this case if its an error
        let body = response.body().bytes().expect(\"body loaded\");
        crate::protocol_serde::type_erase_result(crate::protocol_serde::shape_{module}::de_{module}_http_error(
            response.status().as_u16(),
            response.headers(),
            body,
        ))
    }}
}}"
    )
    .unwrap();
}

fn render_standalone_request_serializer(
    output: &mut String,
    selected: &SelectedModel,
    operation_name: &str,
    operation: &Value,
    module: &str,
    operation_type: &str,
    error_path: &str,
) {
    let input_shape = operation
        .get("input")
        .and_then(target_value)
        .and_then(|id| selected.model.shapes.get(id));
    let input_path = format!("crate::operation::{module}::{operation_type}Input");
    let uri = operation_http_uri(operation);
    let (path, query) = uri.split_once('?').unwrap_or((uri, ""));
    let (rendered_path, uri_labels) = standalone_uri_labels(selected, input_shape, path);
    let query_parts = query
        .split('&')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>();
    let has_dynamic_query = input_shape.is_some_and(|shape| {
        members(shape)
            .iter()
            .any(|(_, member)| has_trait(member, "smithy.api#httpQuery"))
    });
    let has_query = !query_parts.is_empty() || has_dynamic_query;
    let has_headers = input_shape.is_some_and(|shape| {
        members(shape).iter().any(|(_, member)| {
            has_trait(member, "smithy.api#httpHeader")
                || has_trait(member, "smithy.api#httpPrefixHeaders")
        })
    });
    writeln!(
        output,
        "#[derive(Debug)]\nstruct {operation_name}RequestSerializer;\nimpl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for {operation_name}RequestSerializer {{\n    #[allow(unused_mut, clippy::let_and_return, clippy::needless_borrow, clippy::useless_conversion)]\n    fn serialize_input(\n        &self,\n        input: ::aws_smithy_runtime_api::client::interceptors::context::Input,\n        _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,\n    ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {{\n        let input = input.downcast::<{input_path}>().expect(\"correct type\");\n        let _header_serialization_settings = _cfg\n            .load::<crate::serialization_settings::HeaderSerializationSettings>()\n            .cloned()\n            .unwrap_or_default();\n        let mut request_builder = {{"
    )
    .unwrap();
    writeln!(
        output,
        "            #[allow(clippy::uninlined_format_args)]\n            fn uri_base(\n                _input: &{input_path},\n                output: &mut ::std::string::String,\n            ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {{\n                use ::std::fmt::Write as _;\n                ::std::write!(output, {path:?}).expect(\"formatting should succeed\");\n                ::std::result::Result::Ok(())\n            }}"
    )
    .unwrap();
    let raw_uri_write = format!(
        "                ::std::write!(output, {path:?}).expect(\"formatting should succeed\");"
    );
    let rendered_uri_write = standalone_uri_label_body(&rendered_path, &uri_labels);
    *output = output.replace(&raw_uri_write, rendered_uri_write.trim_end_matches('\n'));
    if has_query {
        writeln!(
            output,
            "            fn uri_query(\n                _input: &{input_path},\n                mut output: &mut ::std::string::String,\n            ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {{\n                let mut query = ::aws_smithy_http::query::Writer::new(output);"
        )
        .unwrap();
        for part in query_parts {
            if let Some((name, value)) = part.split_once('=') {
                writeln!(
                    output,
                    "                query.push_kv({name:?}, {value:?});"
                )
                .unwrap();
            } else {
                writeln!(output, "                query.push_v({part:?});").unwrap();
            }
        }
        let mut index = uri_labels.len() + 1;
        if let Some(shape) = input_shape {
            for (name, member) in members(shape) {
                let Some(query_name) = member
                    .get("traits")
                    .and_then(|traits| traits.get("smithy.api#httpQuery"))
                    .and_then(Value::as_str)
                else {
                    continue;
                };
                let field = names::rust_identifier(&name);
                let target = member_target(member).unwrap_or("smithy.api#String");
                let kind = protocol_shape_kind(selected, target);
                if member_is_required(member) {
                    writeln!(
                        output,
                        "                let inner_{index} = &_input.{field};"
                    )
                    .unwrap();
                    writeln!(output, "                let inner_{index} = inner_{index}").unwrap();
                    writeln!(output, "                    .as_ref()").unwrap();
                    writeln!(output, "                    .ok_or_else(|| ::aws_smithy_types::error::operation::BuildError::missing_field({field:?}, \"cannot be empty or unset\"))?;").unwrap();
                    if kind == "string" {
                        writeln!(output, "                if inner_{index}.is_empty() {{").unwrap();
                        writeln!(output, "                    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(").unwrap();
                        writeln!(output, "                        {field:?},").unwrap();
                        output.push_str("                        \"cannot be empty or unset\",\n                    ));\n                }\n");
                    }
                    if kind == "enum" {
                        writeln!(output, "                query.push_kv({query_name:?}, &::aws_smithy_http::query::fmt_string(inner_{index}.as_str()));").unwrap();
                    } else if kind == "string" {
                        writeln!(output, "                query.push_kv({query_name:?}, &::aws_smithy_http::query::fmt_string(inner_{index}));").unwrap();
                    } else if kind == "timestamp" {
                        writeln!(output, "                query.push_kv({query_name:?}, &::aws_smithy_http::query::fmt_timestamp(inner_{index}, ::aws_smithy_types::date_time::Format::HttpDate)?);").unwrap();
                    } else {
                        writeln!(output, "                query.push_kv({query_name:?}, ::aws_smithy_types::primitive::Encoder::from(*inner_{index}).encode());").unwrap();
                    }
                } else {
                    writeln!(output, "                if let ::std::option::Option::Some(inner_{index}) = &_input.{field} {{").unwrap();
                    output.push_str("                    {\n");
                    if kind == "enum" {
                        writeln!(output, "                        query.push_kv({query_name:?}, &::aws_smithy_http::query::fmt_string(inner_{index}.as_str()));").unwrap();
                    } else if kind == "string" {
                        writeln!(output, "                        query.push_kv({query_name:?}, &::aws_smithy_http::query::fmt_string(inner_{index}));").unwrap();
                    } else if kind == "timestamp" {
                        writeln!(output, "                        query.push_kv({query_name:?}, &::aws_smithy_http::query::fmt_timestamp(inner_{index}, ::aws_smithy_types::date_time::Format::HttpDate)?);").unwrap();
                    } else {
                        writeln!(output, "                        query.push_kv({query_name:?}, ::aws_smithy_types::primitive::Encoder::from(*inner_{index}).encode());").unwrap();
                    }
                    output.push_str("                    }\n");
                    output.push_str("                }\n");
                }
                index += 1;
            }
        }
        output.push_str("                ::std::result::Result::Ok(())\n            }");
    }
    writeln!(
        output,
        "\n            #[allow(clippy::unnecessary_wraps)]\n            fn update_http_builder(\n                input: &{input_path},\n                builder: ::http_1x::request::Builder,\n            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {{\n                let mut uri = ::std::string::String::new();\n                uri_base(input, &mut uri)?;"
    )
    .unwrap();
    if has_query {
        output.push_str("                uri_query(input, &mut uri)?;\n");
    }
    if has_headers {
        writeln!(output, "                let builder = crate::protocol_serde::shape_{module}::ser_{module}_headers(input, builder)?;").unwrap();
    }
    writeln!(
        output,
        "                ::std::result::Result::Ok(builder.method({:?}).uri(uri))\n            }}\n            let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;\n            builder\n        }};\n        let body = ::aws_smithy_types::body::SdkBody::from(\"\");\n\n        ::std::result::Result::Ok(request_builder.body(body).expect(\"valid request\").try_into().unwrap())\n    }}\n}}",
        operation_http_method(operation)
    )
    .unwrap();
    let (body_expression, content_type) =
        standalone_request_body(selected, operation_name, input_shape);
    if let Some(ref content_type) = content_type {
        let builder_marker = "            let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;\n            builder".to_owned();
        let builder_replacement = format!(
            "            let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;\n            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, {content_type:?});\n            builder"
        );
        *output = output.replace(&builder_marker, &builder_replacement);
    }
    let body_marker = "        let body = ::aws_smithy_types::body::SdkBody::from(\"\");\n\n";
    let mut body_replacement = format!("        let body = {body_expression};\n");
    body_replacement.push_str("        if let Some(content_length) = body.content_length() {\n            let content_length = content_length.to_string();\n            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);\n        }\n");
    if content_type.is_some() {
        *output = output.replacen(body_marker, &body_replacement, 1);
    }
    *output = output.replace(
        "\n            #[allow(clippy::unnecessary_wraps)]",
        "            #[allow(clippy::unnecessary_wraps)]",
    );
    let _ = error_path;
}

fn render_standalone_endpoint_interceptor(
    output: &mut String,
    selected: &SelectedModel,
    operation_name: &str,
    operation_type: &str,
) {
    let operation = operation_shape(selected, operation_name).expect("selected operation exists");
    let service = selected
        .model
        .shapes
        .get(selected.model.entry.service_shape_id);
    let service_traits = service
        .and_then(|shape| shape.get("traits"))
        .and_then(Value::as_object);
    let endpoint_params = service_traits
        .and_then(|traits| traits.get("smithy.rules#endpointRuleSet"))
        .and_then(|rules| rules.get("parameters"))
        .and_then(Value::as_object);
    let client_params = service_traits
        .and_then(|traits| traits.get("smithy.rules#clientContextParams"))
        .and_then(Value::as_object);
    let input_shape = operation
        .get("input")
        .and_then(target_value)
        .and_then(|id| selected.model.shapes.get(id));
    if output.ends_with('\n') {
        output.pop();
    }
    writeln!(
        output,
        "\n#[derive(Debug)]\nstruct {operation_type}EndpointParamsInterceptor;\n\n#[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]\nimpl ::aws_smithy_runtime_api::client::interceptors::Intercept for {operation_type}EndpointParamsInterceptor {{\n    fn name(&self) -> &'static str {{\n        \"{operation_type}EndpointParamsInterceptor\"\n    }}\n\n    fn read_before_execution(\n        &self,\n        context: &::aws_smithy_runtime_api::client::interceptors::context::BeforeSerializationInterceptorContextRef<\n            '_ ,\n            ::aws_smithy_runtime_api::client::interceptors::context::Input,\n            ::aws_smithy_runtime_api::client::interceptors::context::Output,\n            ::aws_smithy_runtime_api::client::interceptors::context::Error,\n        >,\n        cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,\n    ) -> ::std::result::Result<(), ::aws_smithy_runtime_api::box_error::BoxError> {{\n        let _input = context\n            .input()\n            .downcast_ref::<{operation_type}Input>()\n            .ok_or(\"failed to downcast to {operation_type}Input\")?;\n\n        let params = crate::config::endpoint::Params::builder()"
    )
    .unwrap();
    if let Some(endpoint_prefix) = render_standalone_endpoint_prefix(operation, input_shape) {
        let params_marker = "        let params = crate::config::endpoint::Params::builder()";
        *output = output.replace(params_marker, &format!("{endpoint_prefix}{params_marker}"));
    }
    let builtin_order = ["Region", "UseFIPS", "UseDualStack", "Endpoint"];
    for name in builtin_order {
        if endpoint_params.is_some_and(|params| params.contains_key(name)) {
            let setter = names::snake_case(name);
            let expression = match name {
                "Region" => {
                    "cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned())"
                }
                "UseFIPS" => "cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0)",
                "UseDualStack" => {
                    "cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0)"
                }
                "Endpoint" => {
                    "cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone())"
                }
                _ => unreachable!(),
            };
            writeln!(output, "            .set_{setter}({expression})").unwrap();
        }
    }
    if let Some(client_params) = client_params {
        for name in client_params.keys() {
            let setter = names::snake_case(name);
            writeln!(
                output,
                "            .set_{setter}(cfg.load::<crate::config::{}>().map(|ty| ty.0))",
                rust_type_name(name)
            )
            .unwrap();
        }
    }
    if let Some(static_params) = operation
        .get("traits")
        .and_then(|traits| traits.get("smithy.rules#staticContextParams"))
        .and_then(Value::as_object)
    {
        for (name, value) in static_params {
            let setter = names::snake_case(name);
            let literal = endpoint_param_literal(value.get("value"));
            writeln!(output, "            .set_{setter}({literal})").unwrap();
        }
    }
    if let Some(input_shape) = input_shape {
        for (name, member) in members(input_shape) {
            let Some(context_param) = member
                .get("traits")
                .and_then(|traits| traits.get("smithy.rules#contextParam"))
                .and_then(|value| value.get("name"))
                .and_then(Value::as_str)
            else {
                continue;
            };
            let field = names::rust_identifier(&name);
            let setter = names::snake_case(context_param);
            let value = if member_is_required(member) {
                format!(
                    "Some(\n                _input\n                    .{field}\n                    .clone()\n                    .filter(|f| !AsRef::<str>::as_ref(f).trim().is_empty())\n                    .ok_or_else(|| ::aws_smithy_types::error::operation::BuildError::missing_field({field:?}, \"A required field was not set\"))?\n            )"
                )
            } else {
                format!("_input.{field}.clone()")
            };
            writeln!(output, "            .set_{setter}({value})").unwrap();
        }
    }
    output.push_str(
        "            .build()\n            .map_err(|err| {\n                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new(\"endpoint params could not be built\", err)\n            })?;\n        cfg.interceptor_state()\n            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params));\n        ::std::result::Result::Ok(())\n    }\n}\n\n// The get_* functions below are generated from JMESPath expressions in the\n// operationContextParams trait. They target the operation's input shape.\n",
    );
}

fn render_standalone_endpoint_prefix(
    operation: &Value,
    input_shape: Option<&Value>,
) -> Option<String> {
    let host_prefix = operation
        .get("traits")
        .and_then(|traits| traits.get("smithy.api#endpoint"))
        .and_then(|endpoint| endpoint.get("hostPrefix"))
        .and_then(Value::as_str)?;
    let mut labels = Vec::new();
    let mut remainder = host_prefix;
    while let Some(start) = remainder.find('{') {
        let after_start = &remainder[start + 1..];
        let end = after_start.find('}')?;
        let label = &after_start[..end];
        if label.is_empty() || label.contains('{') {
            return None;
        }
        let member = input_shape
            .and_then(|shape| members(shape).into_iter().find(|(name, _)| name == label))?;
        let field = names::rust_identifier(&member.0);
        labels.push((label.to_owned(), field));
        remainder = &after_start[end + 1..];
    }

    let mut output = String::new();
    output.push_str("        let endpoint_prefix = {\n");
    for (_, field) in &labels {
        writeln!(
            output,
            "            let {field} = _input.{field}.as_deref().unwrap_or_default();"
        )
        .unwrap();
        writeln!(
            output,
            "            if {field}.is_empty() {{\n                return Err(::aws_smithy_runtime_api::client::endpoint::error::InvalidEndpointError::failed_to_construct_uri(\"{field} was unset or empty but must be set as part of the endpoint prefix\").into());\n            }}"
        )
        .unwrap();
    }
    let arguments = labels
        .iter()
        .map(|(label, field)| format!("{label} = {field}"))
        .collect::<Vec<_>>()
        .join(", ");
    if arguments.is_empty() {
        writeln!(
            output,
            "            ::aws_smithy_runtime_api::client::endpoint::EndpointPrefix::new({host_prefix:?})"
        )
        .unwrap();
    } else {
        output.push_str("            #[allow(clippy::uninlined_format_args)]\n");
        writeln!(
            output,
            "            ::aws_smithy_runtime_api::client::endpoint::EndpointPrefix::new(format!({host_prefix:?}, {arguments}))"
        )
        .unwrap();
    }
    output.push_str(
        "        }.map_err(|err| ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new(\"endpoint prefix could not be built\", err))?;\n        cfg.interceptor_state().store_put(endpoint_prefix);\n\n",
    );
    Some(output)
}

/// Render Smithy HTTP protocol tests supplied by a declarative model overlay.
/// The renderer consumes the generic request/response test shape and does not
/// identify a service or operation by name.
fn render_standalone_protocol_tests(
    output: &mut String,
    selected: &SelectedModel,
    operation_name: &str,
    operation: &Value,
    module: &str,
    operation_type: &str,
) {
    let protocol = selected
        .model
        .protocol()
        .expect("selected model protocol is valid")
        .trait_id();
    let mut tests = selected
        .protocol_tests
        .iter()
        .filter(|test| {
            test.get("protocol").and_then(Value::as_str) == Some(protocol)
                && (test.get("operation").and_then(Value::as_str) == Some(operation_name)
                    || test
                        .get("shape")
                        .and_then(Value::as_str)
                        .is_some_and(|shape| {
                            operation
                                .get("errors")
                                .and_then(Value::as_array)
                                .is_some_and(|errors| {
                                    errors
                                        .iter()
                                        .filter_map(target_value)
                                        .any(|error| error == shape)
                                })
                        }))
        })
        .collect::<Vec<_>>();
    tests.sort_by_key(|test| test.get("kind").and_then(Value::as_str) != Some("request"));
    if tests.is_empty() {
        return;
    }

    writeln!(
        output,
        "\n#[allow(unreachable_code, unused_variables)]\n#[cfg(test)]\nmod {module}_test {{"
    )
    .unwrap();
    for test in tests {
        match test.get("kind").and_then(Value::as_str) {
            Some("request") => {
                render_protocol_request_test(output, selected, operation_name, operation, test)
            }
            Some("response") => render_protocol_response_test(
                output,
                selected,
                operation_name,
                operation,
                module,
                operation_type,
                test,
            ),
            _ => {}
        }
    }
    output.push_str("}\n");
}

fn render_protocol_test_docs(output: &mut String, test: &Value) {
    if let Some(documentation) = test.get("documentation").and_then(Value::as_str) {
        output.push('\n');
        for line in documentation.split('\n') {
            writeln!(output, "    /// {line}").unwrap();
        }
    }
    writeln!(
        output,
        "    /// Test ID: {}",
        test.get("id").and_then(Value::as_str).unwrap_or_default()
    )
    .unwrap();
    output.push_str("    #[::tokio::test]\n    #[::tracing_test::traced_test]\n");
}

fn render_protocol_request_test(
    output: &mut String,
    selected: &SelectedModel,
    operation_name: &str,
    operation: &Value,
    test: &Value,
) {
    let id = test.get("id").and_then(Value::as_str).unwrap_or_default();
    render_protocol_test_docs(output, test);
    writeln!(
        output,
        "    async fn {}_request() {{",
        names::snake_case(id)
    )
    .unwrap();
    output.push_str("        let (http_client, request_receiver) = ::aws_smithy_http_client::test_util::capture_request(None);\n");
    let endpoint = test
        .get("host")
        .and_then(Value::as_str)
        .map(|host| format!("https://{host}"))
        .unwrap_or_else(|| "https://example.com".to_owned());
    writeln!(
        output,
        "        let config_builder = crate::config::Config::builder()\n            .with_test_defaults()\n            // TODO(https://github.com/smithy-lang/smithy-rs/issues/4177):\n            //  Until the incorrect separation is addressed, we need to rely on this workaround.\n            .allow_no_auth()\n            .endpoint_url({endpoint:?});"
    )
    .unwrap();
    if test
        .get("setRegion")
        .and_then(Value::as_bool)
        .unwrap_or(false)
    {
        output.push_str("        let config_builder = config_builder.region(::aws_types::region::Region::new(\"us-east-1\"));\n");
    } else {
        output.push('\n');
    }
    output.push_str("        let mut config_builder = config_builder;\n        config_builder.set_region(Some(crate::config::Region::new(\"us-east-1\")));\n\n        let config = config_builder.http_client(http_client).build();\n        let client = crate::Client::from_conf(config);\n        let result = client\n");
    let input = operation
        .get("input")
        .and_then(target_value)
        .and_then(|id| selected.model.shapes.get(id));
    writeln!(
        output,
        "            .{}()",
        names::snake_case(operation_name)
    )
    .unwrap();
    if let (Some(input), Some(params)) = (input, test.get("params").and_then(Value::as_object)) {
        for (name, value) in params {
            let Some((_, member)) = members(input)
                .into_iter()
                .find(|(member_name, _)| member_name == name)
            else {
                continue;
            };
            let Some(target) = member_target(member) else {
                continue;
            };
            let expression = render_protocol_value(selected, target, value);
            writeln!(
                output,
                "            .set_{}(::std::option::Option::Some({expression}))",
                names::rust_identifier(name)
            )
            .unwrap();
        }
    }
    output.push_str("            .send()\n            .await;\n        let _ = dbg!(result);\n        let http_request = request_receiver.expect_request();\n");
    render_protocol_query_checks(output, test);
    render_protocol_header_checks(output, test);
    if let Some(body) = test.get("body").and_then(Value::as_str) {
        render_protocol_body_check(
            output,
            body,
            test.get("bodyMediaType").and_then(Value::as_str),
        );
    }
    let method = test
        .get("method")
        .and_then(Value::as_str)
        .or_else(|| {
            operation_http_trait(operation)
                .and_then(|http| http.get("method"))
                .and_then(Value::as_str)
        })
        .unwrap_or("POST");
    let uri = test
        .get("uri")
        .and_then(Value::as_str)
        .or_else(|| {
            operation_http_trait(operation)
                .and_then(|http| http.get("uri"))
                .and_then(Value::as_str)
        })
        .unwrap_or("/");
    output.push_str("        let uri: ::http_1x::Uri = http_request.uri().parse().expect(\"invalid URI sent\");\n");
    writeln!(output, "        ::pretty_assertions::assert_eq!(http_request.method(), {method:?}, \"method was incorrect\");").unwrap();
    writeln!(
        output,
        "        ::pretty_assertions::assert_eq!(uri.path(), {uri:?}, \"path was incorrect\");"
    )
    .unwrap();
    if let Some(host) = test.get("resolvedHost").and_then(Value::as_str) {
        writeln!(output, "        ::pretty_assertions::assert_eq!(uri.host().expect(\"host should be set\"), {host:?});").unwrap();
    }
    output.push_str("    }\n");
}

fn render_protocol_query_checks(output: &mut String, test: &Value) {
    if let Some(params) = test.get("queryParams").and_then(Value::as_array) {
        let values = params
            .iter()
            .filter_map(Value::as_str)
            .map(|value| format!("{value:?}"))
            .collect::<Vec<_>>()
            .join(", ");
        writeln!(output, "        let expected_query_params = &[{values}];").unwrap();
        output.push_str("        ::aws_smithy_protocol_test::assert_ok(::aws_smithy_protocol_test::validate_query_string(&http_request, expected_query_params));\n");
    }
}

fn render_protocol_header_checks(output: &mut String, test: &Value) {
    let Some(headers) = test.get("headers").and_then(Value::as_object) else {
        return;
    };
    if headers.is_empty() {
        return;
    }
    let values = headers
        .iter()
        .map(|(name, value)| format!("({name:?}, {:?})", value.as_str().unwrap_or_default()))
        .collect::<Vec<_>>()
        .join(", ");
    writeln!(output, "        let expected_headers = [{values}];").unwrap();
    output.push_str("        ::aws_smithy_protocol_test::assert_ok(::aws_smithy_protocol_test::validate_headers(http_request.headers(), expected_headers));\n");
}

fn render_protocol_body_check(output: &mut String, body: &str, media_type: Option<&str>) {
    output.push_str(
        "        let body = http_request.body().bytes().expect(\"body should be strict\");\n",
    );
    if body.is_empty() {
        output.push_str("        // No body.\n        ::pretty_assertions::assert_eq!(&body, &::bytes::Bytes::new());\n");
    } else {
        writeln!(output, "        ::aws_smithy_protocol_test::assert_ok(\n        ::aws_smithy_protocol_test::validate_body(body, {body:?}, ::aws_smithy_protocol_test::MediaType::from({:?}))\n        );", media_type.unwrap_or("unknown")).unwrap();
    }
}

fn render_protocol_response_test(
    output: &mut String,
    selected: &SelectedModel,
    operation_name: &str,
    operation: &Value,
    module: &str,
    operation_type: &str,
    test: &Value,
) {
    let id = test.get("id").and_then(Value::as_str).unwrap_or_default();
    render_protocol_test_docs(output, test);
    writeln!(
        output,
        "    async fn {}_response() {{",
        names::snake_case(id)
    )
    .unwrap();
    let target = test
        .get("shape")
        .and_then(Value::as_str)
        .or_else(|| operation.get("output").and_then(target_value))
        .unwrap_or("smithy.api#Unit");
    let shape = selected.model.shapes.get(target);
    let expected = if test.get("shape").is_some() {
        format!(
            "crate::types::error::{}::builder()",
            rust_type_name(terminal(target))
        )
    } else {
        format!("crate::operation::{module}::{operation_type}Output::builder()")
    };
    writeln!(output, "        let expected_output = {expected}").unwrap();
    if let Some(params) = test.get("params").and_then(Value::as_object)
        && let Some(shape) = shape
    {
        for (name, value) in params {
            let Some((_, member)) = members(shape)
                .into_iter()
                .find(|(member_name, _)| member_name == name)
            else {
                continue;
            };
            let Some(member_target) = member_target(member) else {
                continue;
            };
            let expression = render_protocol_value(selected, member_target, value);
            writeln!(
                output,
                "            .set_{}(::std::option::Option::Some({expression}))",
                names::rust_identifier(name)
            )
            .unwrap();
        }
    }
    if let Some(shape) = shape {
        if serde_util_builder_is_fallible(selected, shape) {
            output.push_str("            .build().unwrap();\n");
        } else {
            output.push_str("            .build();\n");
        }
    } else {
        output.push_str("            .build();\n");
    }
    output.push_str("        let mut http_response = ::aws_smithy_runtime_api::http::Response::try_from(::http_1x::response::Builder::new()\n");
    if let Some(headers) = test.get("headers").and_then(Value::as_object) {
        let mut entries = headers.iter().collect::<Vec<_>>();
        entries.sort_by_key(|(name, _)| *name);
        for (name, value) in entries {
            writeln!(
                output,
                "        .header({name:?}, {:?})",
                value.as_str().unwrap_or_default()
            )
            .unwrap();
        }
    }
    writeln!(
        output,
        "        .status({})",
        test.get("code").and_then(Value::as_u64).unwrap_or(200)
    )
    .unwrap();
    let body = test.get("body").and_then(Value::as_str).unwrap_or("");
    writeln!(output, "                    .body(::aws_smithy_types::body::SdkBody::from({body:?}))\n                    .unwrap()\n                    ).unwrap();").unwrap();
    output.push_str("        use ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin;\n        use ::aws_smithy_runtime_api::client::ser_de::DeserializeResponse;\n\n");
    writeln!(
        output,
        "        let op = crate::operation::{module}::{operation_type}::new();"
    )
    .unwrap();
    output.push_str("        let config = op.config().expect(\"the operation has config\");\n        let de = config\n            .load::<::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer>()\n            .expect(\"the config must have a deserializer\");\n\n        // Build a config bag with the protocol for schema-based deserialization\n        #[allow(unused_mut)]\n        let mut test_cfg = ::aws_smithy_types::config_bag::ConfigBag::base();\n\n        let parsed = de.deserialize_streaming(&mut http_response);\n        let parsed = parsed.unwrap_or_else(|| {\n            let http_response = http_response.map(|body| {\n                ::aws_smithy_types::body::SdkBody::from(::bytes::Bytes::copy_from_slice(&::aws_smithy_protocol_test::decode_body_data(\n                    body.bytes().unwrap(),\n                    ::aws_smithy_protocol_test::MediaType::from(\"application/xml\"),\n                )))\n            });\n            de.deserialize_nonstreaming_with_config(&http_response, &test_cfg)\n        });\n");
    if test.get("shape").is_some() {
        let error_name = rust_type_name(operation_name);
        let variant = rust_type_name(terminal(target));
        writeln!(output, "        let parsed = parsed.expect_err(\"should be error response\");\n        let parsed: &crate::operation::{module}::{error_name}Error = parsed.as_operation_error().expect(\"operation error\").downcast_ref().unwrap();\n        if let crate::operation::{module}::{error_name}Error::{variant}(parsed) = parsed {{").unwrap();
        render_protocol_comparisons(output, selected, shape, "parsed", "expected_output");
        output.push_str("        } else {\n            panic!(\"wrong variant: Got: {:?}. Expected: {:?}\", parsed, expected_output);\n        }\n");
    } else {
        writeln!(output, "        let parsed = parsed\n            .expect(\"should be successful response\")\n            .downcast::<crate::operation::{module}::{operation_type}Output>()\n            .unwrap();").unwrap();
        render_protocol_comparisons(output, selected, shape, "parsed", "expected_output");
    }
    output.push_str("    }\n");
}

fn render_protocol_comparisons(
    output: &mut String,
    selected: &SelectedModel,
    shape: Option<&Value>,
    actual: &str,
    expected: &str,
) {
    let Some(shape) = shape else {
        return;
    };
    for (name, member) in members(shape) {
        let field = names::rust_identifier(&name);
        let target = member_target(member).unwrap_or("smithy.api#String");
        let kind = protocol_shape_kind(selected, target);
        if matches!(kind, "float" | "double") {
            writeln!(output, "            assert!({actual}.{field}.float_equals(&{expected}.{field}), \"Unexpected value for `{field}` {{:?}} vs. {{:?}}\", {expected}.{field}, {actual}.{field});").unwrap();
        } else {
            writeln!(output, "        ::pretty_assertions::assert_eq!({actual}.{field}, {expected}.{field}, \"Unexpected value for `{field}`\");").unwrap();
        }
    }
}

fn render_protocol_value(selected: &SelectedModel, target: &str, value: &Value) -> String {
    match protocol_shape_kind(selected, target) {
        "structure" => {
            let Some(shape) = selected.model.shapes.get(target) else {
                return "Default::default()".to_owned();
            };
            let mut expression = format!(
                "crate::types::{}::builder()",
                rust_type_name(terminal(target))
            );
            if let Some(values) = value.as_object() {
                for (name, value) in values {
                    let Some((_, member)) = members(shape)
                        .into_iter()
                        .find(|(member_name, _)| member_name == name)
                    else {
                        continue;
                    };
                    let Some(member_target) = member_target(member) else {
                        continue;
                    };
                    expression.push_str(&format!(
                        ".set_{}(::std::option::Option::Some({}))",
                        names::rust_identifier(name),
                        render_protocol_value(selected, member_target, value)
                    ));
                }
            }
            if serde_util_builder_is_fallible(selected, shape) {
                expression.push_str(".build().unwrap()");
            } else {
                expression.push_str(".build()");
            }
            expression
        }
        "list" => {
            let element_target = selected
                .model
                .shapes
                .get(target)
                .and_then(|shape| shape.get("member"))
                .and_then(member_target)
                .unwrap_or("smithy.api#String");
            let values = value
                .as_array()
                .map(|values| {
                    values
                        .iter()
                        .map(|value| render_protocol_value(selected, element_target, value))
                        .collect::<Vec<_>>()
                        .join(", ")
                })
                .unwrap_or_default();
            format!("vec![{values}]")
        }
        "enum" => format!(
            "{}.parse::<crate::types::{}>().expect(\"static value validated to member\")",
            protocol_string_literal(value),
            rust_type_name(terminal(target))
        ),
        "timestamp" => protocol_timestamp_literal(value),
        "boolean" => value
            .as_bool()
            .map(|value| value.to_string())
            .unwrap_or_else(|| "false".to_owned()),
        "byte" | "short" | "integer" | "long" | "bigInteger" | "bigDecimal" => value.to_string(),
        _ => format!("{}.to_owned()", protocol_string_literal(value)),
    }
}

fn protocol_string_literal(value: &Value) -> String {
    value
        .as_str()
        .map(|value| format!("{value:?}"))
        .unwrap_or_else(|| "\"\"".to_owned())
}

fn protocol_timestamp_literal(value: &Value) -> String {
    let value = value.to_string();
    let (seconds, fraction) = value.split_once('.').unwrap_or((value.as_str(), "0"));
    let fraction = fraction.trim_end_matches('0');
    let fraction = if fraction.is_empty() {
        "0_f64".to_owned()
    } else {
        format!("0.{fraction}_f64")
    };
    format!("::aws_smithy_types::DateTime::from_fractional_secs({seconds}, {fraction})")
}

fn endpoint_param_literal(value: Option<&Value>) -> String {
    match value {
        Some(Value::Bool(value)) => format!("Some({value})"),
        Some(Value::String(value)) => format!("Some({value:?}.to_string())"),
        Some(Value::Array(values)) => {
            let values = values
                .iter()
                .filter_map(Value::as_str)
                .map(|value| format!("{value:?}.to_string()"))
                .collect::<Vec<_>>()
                .join(", ");
            format!("Some(vec![{values}])")
        }
        _ => "None".to_owned(),
    }
}

fn render_standalone_operation_error(
    output: &mut String,
    selected: &SelectedModel,
    operation_name: &str,
    operation: &Value,
) {
    let operation_type = operation_error_type_name(operation_name);
    let error_path = format!(
        "crate::operation::{}::{}Error",
        names::snake_case(operation_name),
        operation_type
    );
    let errors = operation
        .get("errors")
        .and_then(Value::as_array)
        .map(|errors| errors.iter().filter_map(target_value).collect::<Vec<_>>())
        .unwrap_or_default();
    writeln!(output, "\n/// Error type for the `{operation_type}Error` operation.\n#[non_exhaustive]\n#[derive(::std::fmt::Debug)]\npub enum {operation_type}Error {{").unwrap();
    for error in &errors {
        let error_name = rust_type_name(terminal(error));
        if let Some(shape) = selected.model.shapes.get(*error)
            && let Some(documentation) = documentation(shape)
        {
            render_doc_lines(output, &documentation, 4);
        }
        writeln!(
            output,
            "    {error_name}(crate::types::error::{error_name}),"
        )
        .unwrap();
    }
    output.push_str(
        "    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).\n    #[deprecated(note = \"Matching `Unhandled` directly is not forwards compatible. Instead, match using a \\\n    variable wildcard pattern and check `.code()`: \\\n     \\\n    &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\\\"SpecificExceptionCode\\\") => { /* handle the error */ }` \\\n     \\\n    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-ERROR_TYPE) for what information is available in the error metadata.\")]\n    Unhandled(crate::error::sealed_unhandled::Unhandled),\n}\n"
    );
    let replacement = format!("ProvideErrorMetadata-for-{operation_type}Error");
    let current = output.len();
    let tail = &output[current.saturating_sub(600)..];
    let _ = tail;
    // The placeholder is kept local to this generated operation and never
    // enters the model or a service-specific customization layer.
    let mut text = output.to_string();
    text = text.replace("check `.code()`: \\\n", "check `.code()`: \n");
    text = text.replace("/* handle the error */` \\\n", "/* handle the error */`\n");
    text = text.replace(
        "available in the error metadata.",
        "available for the error.",
    );
    text = text.replace("unhandled error ({{code}})", "unhandled error ({code})");
    text = text.replace("ProvideErrorMetadata-for-ERROR_TYPE", &replacement);
    *output = text;
    writeln!(
        output,
        "impl {operation_type}Error {{\n    /// Creates the `{operation_type}Error::Unhandled` variant from any error type.\n    pub fn unhandled(\n        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,\n    ) -> Self {{\n        Self::Unhandled(crate::error::sealed_unhandled::Unhandled {{\n            source: err.into(),\n            meta: ::std::default::Default::default(),\n        }})\n    }}\n\n    /// Creates the `{operation_type}Error::Unhandled` variant from an [`ErrorMetadata`](::aws_smithy_types::error::ErrorMetadata).\n    pub fn generic(err: ::aws_smithy_types::error::ErrorMetadata) -> Self {{\n        Self::Unhandled(crate::error::sealed_unhandled::Unhandled {{\n            source: err.clone().into(),\n            meta: err,\n        }})\n    }}\n    ///\n    /// Returns error metadata, which includes the error code, message,\n    /// request ID, and potentially additional information.\n    ///\n    pub fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {{\n        match self {{\n            {unhandled_meta}\n        }}\n    }}\n}}",
        unhandled_meta = if errors.is_empty() {
            "Self::Unhandled(e) => &e.meta,".to_owned()
        } else {
            errors
                .iter()
                .map(|error| format!("Self::{}(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),", rust_type_name(terminal(error))))
                .chain(std::iter::once("Self::Unhandled(e) => &e.meta,".to_owned()))
                .collect::<Vec<_>>()
                .join("\n            ")
        }
    )
    .unwrap();
    if !errors.is_empty() && output.ends_with("}\n") {
        output.truncate(output.len() - "}\n".len());
        for error in &errors {
            let error_name = rust_type_name(terminal(error));
            let method = names::snake_case(&error_name);
            writeln!(
                output,
                "    /// Returns `true` if the error kind is `{operation_type}Error::{error_name}`.\n    pub fn is_{method}(&self) -> bool {{\n        matches!(self, Self::{error_name}(_))\n    }}"
            )
            .unwrap();
        }
        output.push_str("}\n");
    }
    writeln!(output, "impl ::std::error::Error for {operation_type}Error {{\n    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {{\n        match self {{\n            {arms}\n        }}\n    }}\n}}", arms = if errors.is_empty() { "Self::Unhandled(_inner) => ::std::option::Option::Some(&*_inner.source),".to_owned() } else { errors.iter().map(|error| format!("Self::{}(_inner) => ::std::option::Option::Some(_inner),", rust_type_name(terminal(error)))).chain(std::iter::once("Self::Unhandled(_inner) => ::std::option::Option::Some(&*_inner.source),".to_owned())).collect::<Vec<_>>().join("\n            ") }).unwrap();
    writeln!(output, "impl ::std::fmt::Display for {operation_type}Error {{\n    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {{\n        match self {{\n            {arms}\n        }}\n    }}\n}}", arms = if errors.is_empty() { "Self::Unhandled(_inner) => {\n                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {\n                    write!(f, \"unhandled error ({{code}})\")\n                } else {\n                    f.write_str(\"unhandled error\")\n                }\n            }".to_owned() } else { errors.iter().map(|error| format!("Self::{}(_inner) => _inner.fmt(f),", rust_type_name(terminal(error)))).chain(std::iter::once("Self::Unhandled(_inner) => {\n                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {\n                    write!(f, \"unhandled error ({{code}})\")\n                } else {\n                    f.write_str(\"unhandled error\")\n                }\n            }".to_owned())).collect::<Vec<_>>().join("\n            ") }).unwrap();
    writeln!(output, "impl ::aws_smithy_types::retry::ProvideErrorKind for {operation_type}Error {{\n    fn code(&self) -> ::std::option::Option<&str> {{\n        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)\n    }}\n    fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {{\n        ::std::option::Option::None\n    }}\n}}\nimpl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for {operation_type}Error {{\n    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {{\n        match self {{\n            {arms}\n        }}\n    }}\n}}", arms = if errors.is_empty() { "Self::Unhandled(_inner) => &_inner.meta,".to_owned() } else { errors.iter().map(|error| format!("Self::{}(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),", rust_type_name(terminal(error)))).chain(std::iter::once("Self::Unhandled(_inner) => &_inner.meta,".to_owned())).collect::<Vec<_>>().join("\n            ") }).unwrap();
    writeln!(output, "impl ::aws_smithy_runtime_api::client::result::CreateUnhandledError for {operation_type}Error {{\n    fn create_unhandled_error(\n        source: ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,\n        meta: ::std::option::Option<::aws_smithy_types::error::ErrorMetadata>,\n    ) -> Self {{\n        Self::Unhandled(crate::error::sealed_unhandled::Unhandled {{\n            source,\n            meta: meta.unwrap_or_default(),\n        }})\n    }}\n}}\nimpl crate::s3_request_id::RequestIdExt for {error_path} {{\n    fn extended_request_id(&self) -> Option<&str> {{\n        self.meta().extended_request_id()\n    }}\n}}\nimpl ::aws_types::request_id::RequestId for {error_path} {{\n    fn request_id(&self) -> Option<&str> {{\n        self.meta().request_id()\n    }}\n}}",).unwrap();
    let continuation = ['\\', '\n'].iter().collect::<String>();
    let mut text = std::mem::take(output);
    text = text.replace(
        &format!("check `.code()`: {continuation}"),
        "check `.code()`:\n",
    );
    text = text.replace("check `.code()`: \n", "check `.code()`:\n");
    text = text.replace(&format!("` {continuation}"), "`\n");
    text = text.replace(
        "available in the error metadata.",
        "available for the error.",
    );
    text = text.replace("unhandled error ({{code}})", "unhandled error ({code})");
    text = text.replace("ProvideErrorMetadata-for-ERROR_TYPE", &replacement);
    *output = text;
}

fn operation_shape<'a>(selected: &'a SelectedModel, operation_name: &str) -> Option<&'a Value> {
    let namespace = selected
        .model
        .entry
        .service_shape_id
        .split('#')
        .next()
        .unwrap_or_default();
    selected
        .model
        .shapes
        .get(&format!("{namespace}#{operation_name}"))
}

fn operation_is_paginated(operation: &Value) -> bool {
    operation
        .get("traits")
        .and_then(Value::as_object)
        .is_some_and(|traits| traits.contains_key("smithy.api#paginated"))
}

#[derive(Clone, Debug)]
struct PaginationInfo {
    input_token: Vec<String>,
    output_token: Vec<String>,
    page_size: Option<Vec<String>>,
    items: Option<Vec<String>>,
    is_truncated: bool,
}

fn has_paginated_operations(selected: &SelectedModel) -> bool {
    selected
        .operations
        .iter()
        .any(|operation_name| operation_pagination_info(selected, operation_name).is_some())
}

/// Read Smithy's paginated trait into the same model-derived paths consumed by
/// the paginator and lens generators. Smithy permits a path to be represented
/// by either a string or an array of member names.
fn operation_pagination_info(
    selected: &SelectedModel,
    operation_name: &str,
) -> Option<PaginationInfo> {
    let operation = operation_shape(selected, operation_name)?;
    if !operation_is_paginated(operation) {
        return None;
    }
    let operation_trait = operation
        .get("traits")
        .and_then(Value::as_object)
        .and_then(|traits| traits.get("smithy.api#paginated"))?;
    let service_trait = selected
        .model
        .shapes
        .get(selected.model.entry.service_shape_id)
        .and_then(|shape| shape.get("traits"))
        .and_then(Value::as_object)
        .and_then(|traits| traits.get("smithy.api#paginated"));
    let mut merged_trait = service_trait
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    if let Some(operation_trait) = operation_trait.as_object() {
        merged_trait.extend(operation_trait.clone());
    }
    let trait_value = Value::Object(merged_trait);
    let input_token = smithy_path(trait_value.get("inputToken")?)?;
    let output_token = smithy_path(trait_value.get("outputToken")?)?;
    let items = trait_value.get("items").and_then(smithy_path);
    let page_size = trait_value.get("pageSize").and_then(smithy_path);

    let input_id = operation.get("input").and_then(target_value)?;
    let output_id = operation.get("output").and_then(target_value)?;
    find_member_path(selected, input_id, &input_token)?;
    find_member_path(selected, output_id, &output_token)?;
    if let Some(items_path) = &items {
        find_member_path(selected, output_id, items_path)?;
    }
    if let Some(page_size_path) = &page_size {
        find_member_path(selected, input_id, page_size_path)?;
    }

    Some(PaginationInfo {
        input_token,
        output_token,
        page_size,
        items,
        is_truncated: operation
            .get("output")
            .and_then(target_value)
            .and_then(|output_id| selected.model.shapes.get(output_id))
            .is_some_and(|shape| {
                has_trait(
                    shape,
                    "software.amazon.smithy.rust.codegen.client.smithy.traits#isTruncatedPaginatorTrait",
                )
            }),
    })
}

fn smithy_path(value: &Value) -> Option<Vec<String>> {
    match value {
        Value::String(value) => Some(vec![value.clone()]),
        Value::Array(values) => {
            let path = values
                .iter()
                .map(Value::as_str)
                .collect::<Option<Vec<_>>>()?;
            (!path.is_empty()).then(|| path.into_iter().map(ToOwned::to_owned).collect())
        }
        _ => None,
    }
}

/// Return the waiters attached to selected operations in stable operation order.
/// The waiter trait remains model data so this renderer is reusable across
/// services rather than keyed to S3 operation names.
fn waiter_specs(selected: &SelectedModel) -> Vec<(String, String, Value)> {
    let mut specs = selected
        .operations
        .iter()
        .flat_map(|operation_name| {
            operation_shape(selected, operation_name)
                .and_then(|operation| operation.get("traits"))
                .and_then(Value::as_object)
                .and_then(|traits| traits.get("smithy.waiters#waitable"))
                .and_then(Value::as_object)
                .into_iter()
                .flat_map(move |waiters| {
                    waiters.iter().map(move |(waiter_name, waiter)| {
                        (
                            operation_name.clone(),
                            names::snake_case(waiter_name),
                            waiter.clone(),
                        )
                    })
                })
        })
        .collect::<Vec<_>>();
    // Smithy-RS visits waitable operations in shape-ID order. The selected
    // operation list is model-derived, but is not required to have that order
    // (notably for Lambda's GetFunction and GetFunctionConfiguration waiters).
    specs.sort_by(|left, right| left.0.cmp(&right.0).then_with(|| left.1.cmp(&right.1)));
    specs
}

fn has_waiters(selected: &SelectedModel) -> bool {
    !waiter_specs(selected).is_empty()
}

fn waiter_specs_by_name(selected: &SelectedModel) -> Vec<(String, String, Value)> {
    let mut specs = waiter_specs(selected);
    specs.sort_by(|left, right| left.1.cmp(&right.1).then_with(|| left.0.cmp(&right.0)));
    specs
}

fn waiter_acceptors(waiter: &Value) -> Vec<(String, Value)> {
    waiter
        .get("acceptors")
        .and_then(Value::as_array)
        .into_iter()
        .flat_map(|acceptors| acceptors.iter())
        .filter_map(|acceptor| {
            Some((
                acceptor.get("state")?.as_str()?.to_owned(),
                acceptor.get("matcher")?.clone(),
            ))
        })
        .collect()
}

fn waiter_matcher_name(operation_name: &str, matcher: &Value) -> String {
    let json = serde_json::to_string(matcher).expect("waiter matcher is serializable");
    let digest = Sha256::digest(json.as_bytes());
    let hash = digest
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    format!(
        "match_{}_{}",
        names::snake_case(operation_name),
        &hash[..17]
    )
}

fn waiter_state_name(state: &str) -> &'static str {
    match state {
        "success" => "Success",
        "failure" => "Failure",
        "retry" => "Retry",
        _ => "NoAcceptorsMatched",
    }
}

fn waiter_matcher_json(matcher: &Value) -> String {
    serde_json::to_string(matcher).expect("waiter matcher is serializable")
}

/// Resolve the member paths used by the packaged waiter models. This covers
/// Smithy's dotted output paths and list projections without tying generation
/// to a service or operation name.
type WaiterPathStep = (String, String, bool);

fn waiter_output_path(
    selected: &SelectedModel,
    operation_name: &str,
    path: &str,
) -> Option<(Vec<WaiterPathStep>, String)> {
    let operation = operation_shape(selected, operation_name)?;
    let mut shape_id = operation.get("output").and_then(target_value)?.to_owned();
    let mut steps = Vec::new();
    for raw_segment in path.split('.') {
        let is_array = raw_segment.ends_with("[]");
        let member_name = raw_segment.trim_end_matches("[]");
        let shape = selected.model.shapes.get(&shape_id)?;
        let member = members(shape)
            .into_iter()
            .find(|(name, _)| name == member_name)
            .map(|(_, member)| member)?;
        let target = member_target(member)?.to_owned();
        steps.push((
            names::rust_identifier(member_name),
            target.clone(),
            is_array,
        ));
        shape_id = if is_array {
            selected
                .model
                .shapes
                .get(&target)
                .and_then(|shape| shape.get("member"))
                .and_then(member_target)?
                .to_owned()
        } else {
            target
        };
    }
    Some((steps, shape_id))
}

fn waiter_matcher_type(
    selected: &SelectedModel,
    target: &str,
    operation_module: &str,
    consumer_namespace: bool,
) -> String {
    if selected
        .model
        .shapes
        .get(target)
        .is_some_and(|shape| has_trait(shape, "smithy.api#enum"))
    {
        if consumer_namespace {
            format!("super::super::types::{}", rust_type_name(terminal(target)))
        } else {
            format!("crate::types::{}", rust_type_name(terminal(target)))
        }
    } else {
        let rendered = type_expr(
            selected,
            target,
            Context::Operation {
                module: operation_module.to_owned(),
                input: false,
                consumer_namespace,
            },
        );
        if consumer_namespace {
            rendered
                .replace("super::super::super::types", "super::super::types")
                .replace(
                    "super::super::super::primitives",
                    "super::super::primitives",
                )
        } else {
            rendered
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn render_waiter_output_matcher(
    output: &mut String,
    selected: &SelectedModel,
    operation_name: &str,
    matcher: &Value,
    operation_prefix: &str,
    operation_module: &str,
    operation_type: &str,
    consumer_namespace: bool,
) -> bool {
    let Some(output_matcher) = matcher.get("output").and_then(Value::as_object) else {
        return false;
    };
    let Some(path) = output_matcher.get("path").and_then(Value::as_str) else {
        return false;
    };
    let Some(comparator) = output_matcher.get("comparator").and_then(Value::as_str) else {
        return false;
    };
    let Some(expected) = output_matcher.get("expected").and_then(Value::as_str) else {
        return false;
    };
    let resolution_path = path
        .strip_prefix("length(")
        .and_then(|path| path.split_once(')'))
        .map(|(path, _)| path)
        .unwrap_or(path);
    let Some((steps, target)) = waiter_output_path(selected, operation_name, resolution_path)
    else {
        return false;
    };
    let output_path = format!("{operation_prefix}::{operation_module}::{operation_type}Output");
    let target_type = waiter_matcher_type(selected, &target, operation_module, consumer_namespace);
    if comparator == "stringEquals" && steps.iter().all(|(_, _, is_array)| !is_array) {
        writeln!(
            output,
            "    fn path_traversal<'a>(\n        _output: &'a {output_path},\n    ) -> ::std::option::Option<&'a {target_type}> {{"
        )
        .unwrap();
        for (index, (field, _, _)) in steps.iter().enumerate() {
            let variable = format!("_fld_{}", index + 1);
            let source = if index == 0 {
                "_output".to_owned()
            } else {
                format!("_fld_{index}")
            };
            writeln!(
                output,
                "        let {variable} = {source}.{field}.as_ref()?;"
            )
            .unwrap();
        }
        writeln!(
            output,
            "        ::std::option::Option::Some(_fld_{})\n    }}",
            steps.len()
        )
        .unwrap();
        writeln!(
            output,
            "    _result\n        .as_ref()\n        .ok()\n        .and_then(|output| path_traversal(output))\n        .map(|value| {{\n            let _tmp_2 = value.as_str();\n            let right = {expected:?};\n            let _cmp_1 = _tmp_2 == right;\n            _cmp_1\n        }})\n        .unwrap_or_default()"
        )
        .unwrap();
        return true;
    }
    if comparator == "booleanEquals" && path.starts_with("length(") {
        let Some((list_path, _)) = path
            .strip_prefix("length(")
            .and_then(|path| path.split_once(')'))
        else {
            return false;
        };
        let Some((list_field, list_target, _)) = steps.first() else {
            return false;
        };
        let Some(list_shape) = selected.model.shapes.get(list_target) else {
            return false;
        };
        let Some(element_target) = list_shape.get("member").and_then(member_target) else {
            return false;
        };
        let Some(element_shape) = selected.model.shapes.get(element_target) else {
            return false;
        };
        let Some(filter_field) = path
            .split_once("[?")
            .and_then(|(_, value)| value.split_once(" == "))
            .map(|(field, _)| names::rust_identifier(field))
        else {
            return false;
        };
        let filter_values = path
            .split("== '")
            .skip(1)
            .filter_map(|value| value.split_once('\'').map(|(value, _)| value.to_owned()))
            .collect::<Vec<_>>();
        let Some((_, filter_member)) = members(element_shape)
            .into_iter()
            .find(|(name, _)| names::rust_identifier(name) == filter_field)
        else {
            return false;
        };
        let _ = list_path;
        if filter_values.len() != 2 || member_target(filter_member).is_none() {
            return false;
        }
        let element_type = waiter_matcher_type(
            selected,
            element_target,
            operation_module,
            consumer_namespace,
        );
        writeln!(
            output,
            "    fn path_traversal<'a>(\n        _output: &'a {output_path},\n    ) -> ::std::option::Option<bool> {{\n        let _fld_2 = _output.{list_field}.as_ref()?;\n        let _ret_1 = _fld_2.len() as i64;\n        const _LIT_3: &f64 = &0.0;\n        let _tmp_19 = *_LIT_3;\n        let _tmp_20 = _tmp_19 as i64;\n        let _cmp_4 = _ret_1 > _tmp_20;\n        let _fld_6 = _output.{list_field}.as_ref()?;\n        let _fprj_14 = _fld_6\n            .iter()\n            .filter({{\n                fn filter(_v: &{element_type}) -> ::std::option::Option<bool> {{\n                    let _fld_7 = _v.{filter_field}.as_ref()?;\n                    let _tmp_21 = _fld_7.as_str();\n                    const _LIT_8: &str = {first:?};\n                    let _cmp_9 = _tmp_21 == _LIT_8;\n                    let _fld_10 = _v.{filter_field}.as_ref()?;\n                    let _tmp_22 = _fld_10.as_str();\n                    const _LIT_11: &str = {second:?};\n                    let _cmp_12 = _tmp_22 == _LIT_11;\n                    let _bo_13 = _cmp_9 || _cmp_12;\n                    ::std::option::Option::Some(_bo_13)\n                }}\n                |v| filter(v).unwrap_or_default()\n            }})\n            .collect::<::std::vec::Vec<_>>();\n        let _ret_5 = _fprj_14.len() as i64;\n        let _fld_16 = _output.{list_field}.as_ref()?;\n        let _ret_15 = _fld_16.len() as i64;\n        let _cmp_17 = _ret_5 == _ret_15;\n        let _bo_18 = _cmp_4 && _cmp_17;\n        ::std::option::Option::Some(_bo_18)\n    }}",
            first = filter_values[0],
            second = filter_values[1],
        )
        .unwrap();
        writeln!(
            output,
            "    _result\n        .as_ref()\n        .ok()\n        .and_then(|output| path_traversal(output))\n        .map(|value| {{\n            let right = true;\n            let _cmp_1 = value == right;\n            _cmp_1\n        }})\n        .unwrap_or_default()"
        )
        .unwrap();
        return true;
    }
    if comparator == "booleanEquals" && steps.iter().all(|(_, _, is_array)| !is_array) {
        let expected = match expected {
            "true" => "true",
            "false" => "false",
            _ => return false,
        };
        writeln!(
            output,
            "    fn path_traversal<'a>(\n        _output: &'a {output_path},\n    ) -> ::std::option::Option<&'a {target_type}> {{"
        )
        .unwrap();
        for (index, (field, _, _)) in steps.iter().enumerate() {
            let variable = format!("_fld_{}", index + 1);
            let source = if index == 0 {
                "_output".to_owned()
            } else {
                format!("_fld_{index}")
            };
            writeln!(
                output,
                "        let {variable} = {source}.{field}.as_ref()?;"
            )
            .unwrap();
        }
        writeln!(
            output,
            "        ::std::option::Option::Some(_fld_{})\n    }}",
            steps.len()
        )
        .unwrap();
        writeln!(
            output,
            "    _result\n        .as_ref()\n        .ok()\n        .and_then(|output| path_traversal(output))\n        .map(|value| {{\n            let right = {expected};\n            let _cmp_1 = value == right;\n            _cmp_1\n        }})\n        .unwrap_or_default()"
        )
        .unwrap();
        return true;
    }
    if comparator == "anyStringEquals" {
        let Some(array_index) = steps.iter().position(|(_, _, is_array)| *is_array) else {
            return false;
        };
        if array_index + 2 != steps.len() {
            return false;
        }
        let (array_field, array_target, _) = &steps[array_index];
        let Some(list_shape) = selected.model.shapes.get(array_target) else {
            return false;
        };
        let Some(element_target) = list_shape.get("member").and_then(member_target) else {
            return false;
        };
        let element_type = waiter_matcher_type(
            selected,
            element_target,
            operation_module,
            consumer_namespace,
        );
        let (final_member, _, _) = &steps[array_index + 1];
        writeln!(
            output,
            "    fn path_traversal<'a>(\n        _output: &'a {output_path},\n    ) -> ::std::option::Option<::std::vec::Vec<&'a {target_type}>> {{"
        )
        .unwrap();
        for (index, (field, _, _)) in steps[..array_index].iter().enumerate() {
            let variable = format!("_fld_{}", index + 1);
            let source = if index == 0 {
                "_output".to_owned()
            } else {
                format!("_fld_{index}")
            };
            writeln!(
                output,
                "        let {variable} = {source}.{field}.as_ref()?;"
            )
            .unwrap();
        }
        let array_variable = format!("_fld_{}", array_index + 1);
        let array_source = if array_index == 0 {
            "_output".to_owned()
        } else {
            format!("_fld_{array_index}")
        };
        writeln!(
            output,
            "        let {array_variable} = {array_source}.{array_field}.as_ref()?;\n        let _prj_3 = {array_variable}\n            .iter()\n            .flat_map(|v| {{\n                #[allow(clippy::let_and_return)]\n                fn map(_v: &{element_type}) -> ::std::option::Option<&{target_type}> {{\n                    let _fld_2 = _v.{final_member}.as_ref();\n                    _fld_2\n                }}\n                map(v)\n            }})\n            .collect::<::std::vec::Vec<_>>();\n        ::std::option::Option::Some(_prj_3)\n    }}"
        )
        .unwrap();
        writeln!(
            output,
            "    _result\n        .as_ref()\n        .ok()\n        .and_then(|output| path_traversal(output))\n        .map(|value| {{\n            value.iter().any(|value| {{\n                let _tmp_2 = value.as_str();\n                let right = {expected:?};\n                let _cmp_1 = _tmp_2 == right;\n                _cmp_1\n            }})\n        }})\n        .unwrap_or_default()"
        )
        .unwrap();
        return true;
    }
    false
}

fn render_waiters_file(selected: &SelectedModel, consumer_namespace: bool) -> String {
    let mut output = String::new();
    header(&mut output);
    for (_, waiter_name, _) in waiter_specs_by_name(selected) {
        writeln!(
            output,
            "/// Supporting types for the `{waiter_name}` waiter."
        )
        .unwrap();
        if consumer_namespace {
            writeln!(
                output,
                "pub mod {waiter_name} {{\n    include!(concat!(env!(\"OUT_DIR\"), \"/generated/{}/src/waiters/{waiter_name}.rs\"));\n}}\n",
                selected.model.entry.key
            )
            .unwrap();
        } else {
            writeln!(output, "pub mod {waiter_name};\n").unwrap();
        }
    }
    output.push_str("#[allow(clippy::needless_lifetimes)]\n#[allow(clippy::let_and_return)]\n");
    if consumer_namespace {
        writeln!(
            output,
            "pub(crate) mod matchers {{\n    include!(concat!(env!(\"OUT_DIR\"), \"/generated/{}/src/waiters/matchers.rs\"));\n}}",
            selected.model.entry.key
        )
        .unwrap();
    } else {
        output.push_str("pub(crate) mod matchers;\n");
    }
    output
}

fn render_waiter_matchers_file(selected: &SelectedModel, consumer_namespace: bool) -> String {
    let mut output = String::new();
    header(&mut output);
    let operation_prefix = if consumer_namespace {
        "super::super::operation"
    } else {
        "crate::operation"
    };
    let mut seen = BTreeSet::new();
    for (operation_name, _, waiter) in waiter_specs_by_name(selected) {
        let operation_module = names::snake_case(&operation_name);
        let operation_type = rust_type_name(&operation_name);
        for (_, matcher) in waiter_acceptors(&waiter) {
            let matcher_json = waiter_matcher_json(&matcher);
            if !seen.insert(format!("{operation_name}\0{matcher_json}")) {
                continue;
            }
            let matcher_name = waiter_matcher_name(&operation_name, &matcher);
            writeln!(output, "/// Matcher union: {matcher_json}").unwrap();
            writeln!(
                output,
                "pub(crate) fn {matcher_name}(\n    _result: ::std::result::Result<&{operation_prefix}::{operation_module}::{operation_type}Output, &{operation_prefix}::{operation_module}::{operation_type}Error>,\n) -> bool {{"
            )
            .unwrap();
            if let Some(success) = matcher.get("success").and_then(Value::as_bool) {
                writeln!(
                    output,
                    "    _result.is_{}",
                    if success { "ok()" } else { "err()" }
                )
                .unwrap();
            } else if let Some(error_type) = matcher.get("errorType").and_then(Value::as_str) {
                if consumer_namespace {
                    render_consumer_error_matcher(
                        &mut output,
                        selected,
                        &operation_name,
                        error_type,
                    );
                } else {
                    writeln!(
                        output,
                        "    if let ::std::result::Result::Err(err) = _result {{\n        if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(err) {{\n            return code == {error_type:?};\n        }}\n    }}\n    false"
                    )
                    .unwrap();
                }
            } else if render_waiter_output_matcher(
                &mut output,
                selected,
                &operation_name,
                &matcher,
                operation_prefix,
                &operation_module,
                &operation_type,
                consumer_namespace,
            ) {
            } else {
                output.push_str("    false\n");
            }
            output.push_str("}\n\n");
        }
    }
    output
}

fn render_waiter_file(
    selected: &SelectedModel,
    waiter_name: &str,
    waiter: &Value,
    consumer_namespace: bool,
) -> String {
    if consumer_namespace {
        return render_consumer_waiter_file(selected, waiter_name, waiter);
    }
    let (operation_name, _, _) = waiter_specs(selected)
        .into_iter()
        .find(|(_, name, _)| name == waiter_name)
        .expect("waiter belongs to selected model");
    let operation_module = names::snake_case(&operation_name);
    let operation_type = rust_type_name(&operation_name);
    let waiter_type = rust_type_name(waiter_name);
    let operation_prefix = if consumer_namespace {
        "super::super::operation"
    } else {
        "crate::operation"
    };
    let client_path = if consumer_namespace {
        "super::super::client"
    } else {
        "crate::client"
    };
    let matcher_prefix = if consumer_namespace {
        "super::matchers"
    } else {
        "crate::waiters::matchers"
    };
    let input_builder_path =
        format!("{operation_prefix}::{operation_module}::builders::{operation_type}InputBuilder");
    let mut output = String::new();
    header(&mut output);
    output.push_str(&format!(
        "///\n/// Fluent builder for the `{waiter_name}` waiter.\n///\n/// This builder is intended to be used similar to the other fluent builders for\n/// normal operations on the client. However, instead of a `send` method, it has\n/// a `wait` method that takes a maximum amount of time to wait.\n///\n/// Construct this fluent builder using the client by importing the\n/// [`Waiters`](crate::client::Waiters) trait and calling the methods\n/// prefixed with `wait_until`.\n///\n"
    ));
    writeln!(
        output,
        "#[derive(::std::clone::Clone, ::std::fmt::Debug)]\npub struct {waiter_type}FluentBuilder {{\n    handle: ::std::sync::Arc<{client_path}::Handle>,\n    inner: {input_builder_path},\n}}"
    )
    .unwrap();
    writeln!(
        output,
        "impl {waiter_type}FluentBuilder {{\n    /// Creates a new `{waiter_type}FluentBuilder`.\n    pub(crate) fn new(handle: ::std::sync::Arc<{client_path}::Handle>) -> Self {{\n        Self {{\n            handle,\n            inner: ::std::default::Default::default(),\n        }}\n    }}\n    /// Access the {operation_type} as a reference.\n    pub fn as_input(&self) -> &{input_builder_path} {{\n        &self.inner\n    }}"
    )
    .unwrap();

    let min_delay = waiter.get("minDelay").and_then(Value::as_u64).unwrap_or(5);
    let max_delay = waiter
        .get("maxDelay")
        .and_then(Value::as_u64)
        .unwrap_or(120);
    let waiter_module_path = if consumer_namespace {
        format!("super::{waiter_name}")
    } else {
        format!("crate::waiters::{waiter_name}")
    };
    let waiter_documentation = waiter
        .get("documentation")
        .and_then(Value::as_str)
        .map(normalize_model_documentation)
        .map(|documentation| {
            documentation
                .lines()
                .map(|line| format!("    /// {line}"))
                .collect::<Vec<_>>()
                .join("\n")
        })
        .unwrap_or_else(|| format!("    /// Wait for `{waiter_name}`"));
    let input_methods = render_waiter_input_methods(
        selected,
        &operation_name,
        &operation_module,
        operation_prefix,
        consumer_namespace,
    );
    writeln!(
        output,
        "{waiter_documentation}\n    pub async fn wait(\n        self,\n        max_wait: ::std::time::Duration,\n    ) -> ::std::result::Result<{waiter_module_path}::{waiter_type}FinalPoll, {waiter_module_path}::WaitUntil{waiter_type}Error> {{\n        let input = self.inner.build().map_err(::aws_smithy_runtime_api::client::waiters::error::WaiterError::construction_failure)?;\n        let runtime_plugins = {operation_prefix}::{operation_module}::{operation_type}::operation_runtime_plugins(\n            self.handle.runtime_plugins.clone(),\n            &self.handle.conf,\n            ::std::option::Option::None,\n        )\n        .with_operation_plugin(crate::sdk_feature_tracker::waiter::WaiterFeatureTrackerRuntimePlugin::new());\n        let mut cfg = ::aws_smithy_types::config_bag::ConfigBag::base();\n        let runtime_components_builder = runtime_plugins\n            .apply_client_configuration(&mut cfg)\n            .map_err(::aws_smithy_runtime_api::client::waiters::error::WaiterError::construction_failure)?;\n        let time_components = runtime_components_builder.into_time_components();\n        let sleep_impl = time_components.sleep_impl().expect(\"a sleep impl is required by waiters\");\n        let time_source = time_components.time_source().expect(\"a time source is required by waiters\");\n\n        let acceptor = move |result: ::std::result::Result<\n            &{operation_prefix}::{operation_module}::{operation_type}Output,\n            &{operation_prefix}::{operation_module}::{operation_type}Error,\n        >| {{\n{acceptors}            ::aws_smithy_runtime::client::waiters::AcceptorState::NoAcceptorsMatched\n        }};\n        let operation = move || {{\n            let input = input.clone();\n            let runtime_plugins = runtime_plugins.clone();\n            async move {{\n                {operation_prefix}::{operation_module}::{operation_type}::orchestrate(&runtime_plugins, input).await\n            }}\n        }};\n        let orchestrator = ::aws_smithy_runtime::client::waiters::WaiterOrchestrator::builder()\n            .min_delay(::std::time::Duration::from_secs({min_delay}))\n            .max_delay(::std::time::Duration::from_secs({max_delay}))\n            .max_wait(max_wait)\n            .time_source(time_source)\n            .sleep_impl(sleep_impl)\n            .acceptor(acceptor)\n            .operation(operation)\n            .build();\n        ::aws_smithy_runtime::client::waiters::attach_waiter_tracing_span(orchestrator.orchestrate()).await\n    }}\n\n{input_methods}}}\n\n/// Successful return type for the `{waiter_name}` waiter.\npub type {waiter_type}FinalPoll = ::aws_smithy_runtime_api::client::waiters::FinalPoll<\n    {operation_prefix}::{operation_module}::{operation_type}Output,\n    ::aws_smithy_runtime_api::client::result::SdkError<\n        {operation_prefix}::{operation_module}::{operation_type}Error,\n        ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,\n    >,\n>;\n\n/// Error type for the `{waiter_name}` waiter.\npub type WaitUntil{waiter_type}Error = ::aws_smithy_runtime_api::client::waiters::error::WaiterError<\n    {operation_prefix}::{operation_module}::{operation_type}Output,\n    {operation_prefix}::{operation_module}::{operation_type}Error,\n>;"
    , acceptors = render_waiter_acceptors(
            selected,
            &operation_name,
            waiter,
            matcher_prefix,
            operation_prefix,
            &operation_module,
            &operation_type,
        )
    )
    .unwrap();
    // FluentBuilderGenerator places input helpers immediately after the
    // overridden wait method; keep the doc comment adjacent to the method as
    // rustfmt does in Smithy-RS output.
    output.replace("\n\n    ///", "\n    ///")
}

fn render_consumer_error_matcher(
    output: &mut String,
    selected: &SelectedModel,
    operation_name: &str,
    error_type: &str,
) {
    let error_name = terminal(error_type);
    let modeled = operation_shape(selected, operation_name)
        .and_then(|operation| operation.get("errors"))
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(target_value)
        .any(|target| terminal(target) == error_name);
    if modeled {
        let predicate = names::snake_case(error_name);
        writeln!(
            output,
            "    if let ::std::result::Result::Err(err) = _result {{\n        return err.is_{predicate}();\n    }}\n    false"
        )
        .unwrap();
    } else {
        output.push_str("    false\n");
    }
}

fn render_consumer_waiter_file(
    selected: &SelectedModel,
    waiter_name: &str,
    waiter: &Value,
) -> String {
    let (operation_name, _, _) = waiter_specs(selected)
        .into_iter()
        .find(|(_, name, _)| name == waiter_name)
        .expect("waiter belongs to selected model");
    let operation_module = names::snake_case(&operation_name);
    let operation_type = rust_type_name(&operation_name);
    let waiter_type = rust_type_name(waiter_name);
    let operation_prefix = "super::super::operation";
    let matcher_prefix = "super::matchers";
    let input_builder_path = format!("{operation_prefix}::{operation_module}::builders::Builder");
    let min_delay = waiter.get("minDelay").and_then(Value::as_u64).unwrap_or(5);
    let max_delay = waiter
        .get("maxDelay")
        .and_then(Value::as_u64)
        .unwrap_or(120);
    let waiter_module_path = format!("super::{waiter_name}");
    let waiter_documentation = waiter
        .get("documentation")
        .and_then(Value::as_str)
        .map(normalize_model_documentation)
        .map(|documentation| {
            documentation
                .lines()
                .map(|line| format!("    /// {line}"))
                .collect::<Vec<_>>()
                .join("\n")
        })
        .unwrap_or_else(|| format!("    /// Wait for `{waiter_name}`"));
    let input_methods =
        render_consumer_waiter_input_methods(selected, &operation_name, &operation_module);
    let acceptors =
        render_consumer_waiter_acceptors(selected, &operation_name, waiter, matcher_prefix);
    let mut output = String::new();
    header(&mut output);
    writeln!(
        output,
        "///\n/// Fluent builder for the `{waiter_name}` waiter.\n///\n/// This builder uses the lightweight consumer operation runtime.\n///\n#[derive(::std::clone::Clone, ::std::fmt::Debug)]\npub struct {waiter_type}FluentBuilder {{\n    inner: {input_builder_path},\n}}\nimpl {waiter_type}FluentBuilder {{\n    /// Creates a new `{waiter_type}FluentBuilder`.\n    pub(crate) fn new(handle: super::super::Client) -> Self {{\n        let inner = handle.{operation_module}();\n        Self {{ inner }}\n    }}\n    /// Access the {operation_type} as a reference.\n    pub fn as_input(&self) -> &{input_builder_path} {{\n        &self.inner\n    }}"
    )
    .unwrap();
    writeln!(
        output,
        "{waiter_documentation}\n    pub async fn wait(\n        self,\n        max_wait: ::std::time::Duration,\n    ) -> ::std::result::Result<{waiter_module_path}::{waiter_type}FinalPoll, {waiter_module_path}::WaitUntil{waiter_type}Error> {{\n        let deadline = ::std::time::Instant::now().checked_add(max_wait);\n        let mut delay = ::std::time::Duration::from_secs({min_delay});\n        loop {{\n            let result = self.inner.clone().send().await;\n            let state = match &result {{\n                ::std::result::Result::Ok(output) => {{\n{acceptors_ok}                }}\n                ::std::result::Result::Err(error) => {{\n{acceptors_err}                }}\n            }};\n            match (state, result) {{\n                (ConsumerWaiterAcceptorState::Success, ::std::result::Result::Ok(output)) => return Ok(output),\n                (ConsumerWaiterAcceptorState::Failure, ::std::result::Result::Err(error)) => return Err(error),\n                (ConsumerWaiterAcceptorState::Retry, result) => {{\n                    if deadline.is_some_and(|deadline| ::std::time::Instant::now() >= deadline) {{\n                        return result;\n                    }}\n                    ::std::thread::sleep(delay);\n                    delay = delay.saturating_mul(2).min(::std::time::Duration::from_secs({max_delay}));\n                }}\n                (_, result) => return result,\n            }}\n        }}\n    }}\n\n{input_methods}}}\n\n/// Successful return type for the `{waiter_name}` waiter.\npub type {waiter_type}FinalPoll = {operation_prefix}::{operation_module}::{operation_type}Output;\n\n/// Error type for the `{waiter_name}` waiter.\npub type WaitUntil{waiter_type}Error = {operation_prefix}::{operation_module}::{operation_type}Error;",
        acceptors_ok = acceptors.replace(
            "{RESULT}",
            "::std::result::Result::Ok(output)",
        ),
        acceptors_err = acceptors.replace(
            "{RESULT}",
            "::std::result::Result::Err(error)",
        ),
    )
    .unwrap();
    output.push_str(
        "\n#[derive(Clone, Copy, Debug, PartialEq, Eq)]\nenum ConsumerWaiterAcceptorState {\n    Success,\n    Failure,\n    Retry,\n    NoAcceptorsMatched,\n}\n",
    );
    output.replace("\n\n    ///", "\n    ///")
}

fn render_consumer_waiter_acceptors(
    _selected: &SelectedModel,
    operation_name: &str,
    waiter: &Value,
    matcher_prefix: &str,
) -> String {
    let mut output =
        "                    ConsumerWaiterAcceptorState::NoAcceptorsMatched".to_owned();
    for (state, matcher) in waiter_acceptors(waiter).into_iter().rev() {
        let matcher_name = waiter_matcher_name(operation_name, &matcher);
        output = format!(
            "                    if {matcher_prefix}::{matcher_name}({{RESULT}}) {{\n                        ConsumerWaiterAcceptorState::{}\n                    }} else {{\n{output}\n                    }}",
            waiter_state_name(&state)
        );
    }
    output
}

fn render_consumer_waiter_input_methods(
    selected: &SelectedModel,
    operation_name: &str,
    operation_module: &str,
) -> String {
    let mut output = String::new();
    let input_id = operation_shape(selected, operation_name)
        .and_then(|operation| operation.get("input"))
        .and_then(target_value);
    let Some(input_shape) = input_id.and_then(|id| selected.model.shapes.get(id)) else {
        return output;
    };
    for (member_name, member) in members(input_shape) {
        let field = names::rust_identifier(&member_name);
        let field_method = field.strip_prefix("r#").unwrap_or(&field);
        let target_id = member_target(member).unwrap_or("smithy.api#String");
        let target = consumer_waiter_type_expr(selected, target_id, operation_module);
        let argument = builder_argument_type(selected, target_id, &target);
        let documentation = modeled_member_documentation(selected, member).unwrap_or_default();
        writeln!(output, "    {}", documentation_lines(documentation.clone())).unwrap();
        writeln!(
            output,
            "    pub fn {field_method}(mut self, input: {argument}) -> Self {{\n        self.inner = self.inner.{field_method}(input);\n        self\n    }}"
        )
        .unwrap();
        writeln!(output, "    {}", documentation_lines(documentation.clone())).unwrap();
        writeln!(
            output,
            "    pub fn set_{field_method}(mut self, input: ::std::option::Option<{target}>) -> Self {{\n        self.inner = self.inner.set_{field_method}(input);\n        self\n    }}"
        )
        .unwrap();
        writeln!(output, "    {}", documentation_lines(documentation)).unwrap();
        writeln!(
            output,
            "    pub fn get_{field_method}(&self) -> &::std::option::Option<{target}> {{\n        self.inner.get_{field_method}()\n    }}"
        )
        .unwrap();
    }
    output
}

fn consumer_waiter_type_expr(
    selected: &SelectedModel,
    target: &str,
    operation_module: &str,
) -> String {
    type_expr(
        selected,
        target,
        Context::Builder {
            module: operation_module.to_owned(),
            input: true,
            consumer_namespace: true,
        },
    )
    .replace("super::super::super::types", "super::super::types")
    .replace(
        "super::super::super::primitives",
        "super::super::primitives",
    )
}

fn render_waiter_input_methods(
    selected: &SelectedModel,
    operation_name: &str,
    operation_module: &str,
    operation_prefix: &str,
    consumer_namespace: bool,
) -> String {
    let mut output = String::new();
    let input_id = operation_shape(selected, operation_name)
        .and_then(|operation| operation.get("input"))
        .and_then(target_value);
    let Some(input_shape) = input_id.and_then(|id| selected.model.shapes.get(id)) else {
        return output;
    };
    for (member_name, member) in members(input_shape) {
        let field = names::rust_identifier(&member_name);
        let field_method = field.strip_prefix("r#").unwrap_or(&field);
        let target_id = member_target(member).unwrap_or("smithy.api#String");
        let target = type_expr(
            selected,
            target_id,
            Context::Builder {
                module: operation_module.to_owned(),
                input: true,
                consumer_namespace,
            },
        );
        let argument = builder_argument_type(selected, target_id, &target);
        let value = builder_argument_value(&argument, "input");
        let documentation = modeled_member_documentation(selected, member).unwrap_or_default();
        writeln!(output, "    {}", documentation_lines(documentation.clone())).unwrap();
        writeln!(
            output,
            "    pub fn {field_method}(mut self, input: {argument}) -> Self {{\n        self.inner = self.inner.{field_method}({value});\n        self\n    }}"
        )
        .unwrap();
        writeln!(output, "    {}", documentation_lines(documentation.clone())).unwrap();
        writeln!(
            output,
            "    pub fn set_{field_method}(mut self, input: ::std::option::Option<{target}>) -> Self {{\n        self.inner = self.inner.set_{field_method}(input);\n        self\n    }}"
        )
        .unwrap();
        writeln!(output, "    {}", documentation_lines(documentation)).unwrap();
        writeln!(
            output,
            "    pub fn get_{field_method}(&self) -> &::std::option::Option<{target}> {{\n        self.inner.get_{field_method}()\n    }}"
        )
        .unwrap();
    }
    let _ = operation_prefix;
    output
}

fn render_waiter_acceptors(
    selected: &SelectedModel,
    operation_name: &str,
    waiter: &Value,
    matcher_prefix: &str,
    operation_prefix: &str,
    operation_module: &str,
    operation_type: &str,
) -> String {
    let mut output = String::new();
    for (state, matcher) in waiter_acceptors(waiter) {
        let matcher_name = waiter_matcher_name(operation_name, &matcher);
        writeln!(
            output,
            "            // Matches: {}\n            if {matcher_prefix}::{matcher_name}(result) {{\n                return ::aws_smithy_runtime::client::waiters::AcceptorState::{};\n            }}",
            waiter_matcher_json(&matcher),
            waiter_state_name(&state)
        )
        .unwrap();
    }
    let _ = (selected, operation_prefix, operation_module, operation_type);
    output
}

fn documentation_lines(documentation: String) -> String {
    documentation
        .lines()
        .map(|line| format!("/// {line}"))
        .collect::<Vec<_>>()
        .join("\n")
}

fn find_member_path<'a>(
    selected: &'a SelectedModel,
    root_id: &str,
    path: &[String],
) -> Option<(String, &'a Value)> {
    let mut shape_id = root_id;
    let mut found = None;
    for member_name in path {
        let shape = selected.model.shapes.get(shape_id)?;
        let member = members(shape)
            .into_iter()
            .find(|(name, _)| name == member_name)?;
        let target = member_target(member.1)?;
        found = Some((target.to_owned(), member.1));
        shape_id = target;
    }
    found
}

fn render_lens_file(selected: &SelectedModel, consumer_namespace: bool) -> String {
    let mut output = String::new();
    client_operation_header(&mut output);
    let operation_prefix = if consumer_namespace {
        "super::operation"
    } else {
        "crate::operation"
    };
    let types_prefix = if consumer_namespace {
        "super::types"
    } else {
        "crate::types"
    };

    let mut paginated = selected
        .operations
        .iter()
        .filter_map(|operation_name| {
            operation_pagination_info(selected, operation_name)
                .map(|info| (operation_name.as_str(), info))
        })
        .collect::<Vec<_>>();
    paginated.sort_by_key(|(operation_name, _)| names::snake_case(operation_name));

    for (operation_name, info) in &paginated {
        let operation_module = names::snake_case(operation_name);
        let operation_type = rust_type_name(operation_name);
        let operation =
            operation_shape(selected, operation_name).expect("selected operation exists");
        let output_id = operation
            .get("output")
            .and_then(target_value)
            .expect("paginated operation output exists");
        let (token_target, _) = find_member_path(selected, output_id, &info.output_token)
            .expect("paginated output token exists");
        let token_type = lens_type_expr(selected, &token_target, consumer_namespace);
        let function_suffix = info
            .output_token
            .iter()
            .map(|name| names::snake_case(name))
            .collect::<Vec<_>>()
            .join("_");
        let function_name = format!("reflens_{operation_module}_output_output_{function_suffix}");
        writeln!(
            output,
            "pub(crate) fn {function_name}(\n    input: &{operation_prefix}::{operation_module}::{operation_type}Output,\n) -> ::std::option::Option<&{token_type}> {{"
        )
        .unwrap();
        render_borrowed_lens_path(&mut output, selected, output_id, &info.output_token);
        output.push_str("    ::std::option::Option::Some(input)\n}\n\n");
    }

    for (operation_name, info) in &paginated {
        let Some(items_path) = info.items.as_ref() else {
            continue;
        };
        let operation_module = names::snake_case(operation_name);
        let operation_type = operation_module_to_type(&operation_module);
        let operation =
            operation_shape(selected, operation_name).expect("selected operation exists");
        let output_id = operation
            .get("output")
            .and_then(target_value)
            .expect("paginated operation output exists");
        let (items_target, _) =
            find_member_path(selected, output_id, items_path).expect("paginated items exists");
        let item_type = pagination_item_type(selected, &items_target, types_prefix);
        let function_suffix = items_path
            .iter()
            .map(|name| names::snake_case(name))
            .collect::<Vec<_>>()
            .join("_");
        let function_name = format!("lens_{operation_module}_output_output_{function_suffix}");
        writeln!(
            output,
            "pub(crate) fn {function_name}(\n    input: {operation_prefix}::{operation_module}::{operation_type}Output,\n) -> ::std::option::Option<{item_type}> {{"
        )
        .unwrap();
        render_owned_lens_path(&mut output, selected, output_id, items_path);
        output.push_str("    ::std::option::Option::Some(input)\n}\n\n");
    }
    output
}

fn render_paginator_file(
    selected: &SelectedModel,
    operation_name: &str,
    consumer_namespace: bool,
) -> String {
    if consumer_namespace {
        return render_consumer_paginator_file(selected, operation_name);
    }

    let info = operation_pagination_info(selected, operation_name)
        .expect("paginator file only exists for paginated operations");
    let operation_module = names::snake_case(operation_name);
    let operation_type = rust_type_name(operation_name);
    let operation_symbol = operation_name;
    let paginator_name = format!("{operation_type}Paginator");
    let operation_path = format!("crate::operation::{operation_module}");
    let output_token_suffix = info
        .output_token
        .iter()
        .map(|name| names::snake_case(name))
        .collect::<Vec<_>>()
        .join("_");
    let output_token_lens =
        format!("crate::lens::reflens_{operation_module}_output_output_{output_token_suffix}");
    let input_token = names::rust_identifier(
        info.input_token
            .last()
            .expect("pagination input token path is not empty"),
    );
    let mut output = String::new();
    client_operation_header(&mut output);
    writeln!(
        output,
        "/// Paginator for [`{operation_name}`]({operation_path}::{operation_type})\npub struct {paginator_name} {{\n    handle: std::sync::Arc<crate::client::Handle>,\n    builder: {operation_path}::builders::{operation_type}InputBuilder,\n    stop_on_duplicate_token: bool,\n}}\n\nimpl {paginator_name} {{\n    /// Create a new paginator-wrapper\n    pub(crate) fn new(\n        handle: std::sync::Arc<crate::client::Handle>,\n        builder: {operation_path}::builders::{operation_type}InputBuilder,\n    ) -> Self {{\n        Self {{\n            handle,\n            builder,\n            stop_on_duplicate_token: true,\n        }}\n    }}\n"
    )
    .unwrap();

    if let Some(page_size_path) = info.page_size.as_ref() {
        let page_size_member = names::rust_identifier(
            page_size_path
                .last()
                .expect("pagination page size path is not empty"),
        );
        let input_id = operation_shape(selected, operation_name)
            .and_then(|operation| operation.get("input"))
            .and_then(target_value)
            .expect("paginated operation input exists");
        let (page_size_target, _) = find_member_path(selected, input_id, page_size_path)
            .expect("paginated page size exists");
        let page_size_type = type_expr(
            selected,
            &page_size_target,
            Context::Operation {
                module: operation_module.clone(),
                input: true,
                consumer_namespace: false,
            },
        );
        writeln!(
            output,
            "    /// Set the page size\n    ///\n    /// _Note: this method will override any previously set value for `{page_size_member}`_\n    pub fn page_size(mut self, limit: {page_size_type}) -> Self {{\n        self.builder.{page_size_member} = ::std::option::Option::Some(limit);\n        self\n    }}\n"
        )
        .unwrap();
    }

    if let Some(items_path) = info.items.as_ref() {
        let item_paginator_name = format!("{paginator_name}Items");
        let documented_path = items_path
            .iter()
            .map(|name| names::rust_identifier(name))
            .collect::<Vec<_>>()
            .join(".");
        writeln!(
            output,
            "    /// Create a flattened paginator\n    ///\n    /// This paginator automatically flattens results using `{documented_path}`. Queries to the underlying service\n    /// are dispatched lazily.\n    pub fn items(self) -> {operation_path}::paginator::{item_paginator_name} {{\n        {operation_path}::paginator::{item_paginator_name}(self)\n    }}\n"
        )
        .unwrap();
    }

    output.push_str(
        "    /// Stop paginating when the service returns the same pagination token twice in a row.\n    ///\n    /// Defaults to true.\n    ///\n    /// For certain operations, it may be useful to continue on duplicate token. For example,\n    /// if an operation is for tailing a log file in real-time, then continuing may be desired.\n    /// This option can be set to `false` to accommodate these use cases.\n    pub fn stop_on_duplicate_token(mut self, stop_on_duplicate_token: bool) -> Self {\n        self.stop_on_duplicate_token = stop_on_duplicate_token;\n        self\n    }\n\n",
    );

    let output_type = format!("{operation_path}::{operation_type}Output");
    let error_type = format!("{operation_path}::{operation_symbol}Error");
    let is_empty = if info.is_truncated {
        "                                // Pagination is exhausted when `is_truncated` is false\n                                let is_empty = !resp.is_truncated.unwrap_or(false);"
    } else {
        "                                // Pagination is exhausted when the next token is an empty string\n                                let is_empty = new_token.map(|token| token.is_empty()).unwrap_or(true);"
    };
    writeln!(
        output,
        "    /// Create the pagination stream\n    ///\n    /// _Note:_ No requests will be dispatched until the stream is used\n    /// (e.g. with the [`.next().await`](aws_smithy_async::future::pagination_stream::PaginationStream::next) method).\n    pub fn send(\n        self,\n    ) -> ::aws_smithy_async::future::pagination_stream::PaginationStream<\n        ::std::result::Result<\n            {output_type},\n            ::aws_smithy_runtime_api::client::result::SdkError<\n                {error_type},\n                ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,\n            >,\n        >,\n    > {{\n        // Move individual fields out of self for the borrow checker\n        let builder = self.builder;\n        let handle = self.handle;\n        let runtime_plugins = {operation_path}::{operation_type}::operation_runtime_plugins(\n            handle.runtime_plugins.clone(),\n            &handle.conf,\n            ::std::option::Option::None,\n        )\n        .with_operation_plugin(crate::sdk_feature_tracker::paginator::PaginatorFeatureTrackerRuntimePlugin::new());\n        ::aws_smithy_async::future::pagination_stream::PaginationStream::new(::aws_smithy_async::future::pagination_stream::fn_stream::FnStream::new(\n            move |tx| {{\n                ::std::boxed::Box::pin(async move {{\n                    // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.\n                    let mut input = match builder\n                        .build()\n                        .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)\n                    {{\n                        ::std::result::Result::Ok(input) => input,\n                        ::std::result::Result::Err(e) => {{\n                            let _ = tx.send(::std::result::Result::Err(e)).await;\n                            return;\n                        }}\n                    }};\n                    loop {{\n                        let resp = {operation_path}::{operation_type}::orchestrate(&runtime_plugins, input.clone()).await;\n                        // If the input member is None or it was an error\n                        let done = match resp {{\n                            ::std::result::Result::Ok(ref resp) => {{\n                                let new_token = {output_token_lens}(resp);\n{is_empty}\n                                if !is_empty && new_token == input.{input_token}.as_ref() && self.stop_on_duplicate_token {{\n                                    true\n                                }} else {{\n                                    input.{input_token} = new_token.cloned();\n                                    is_empty\n                                }}\n                            }}\n                            ::std::result::Result::Err(_) => true,\n                        }};\n                        if tx.send(resp).await.is_err() {{\n                            // receiving end was dropped\n                            return;\n                        }}\n                        if done {{\n                            return;\n                        }}\n                    }}\n                }})\n            }},\n        ))\n    }}\n}}\n"
    )
    .unwrap();

    if let Some(items_path) = info.items.as_ref() {
        let output_id = operation_shape(selected, operation_name)
            .and_then(|operation| operation.get("output"))
            .and_then(target_value)
            .expect("paginated operation output exists");
        let (items_target, _) =
            find_member_path(selected, output_id, items_path).expect("paginated items exists");
        let item_type = paginator_item_type(selected, &items_target, false);
        let items_suffix = items_path
            .iter()
            .map(|name| names::snake_case(name))
            .collect::<Vec<_>>()
            .join("_");
        let item_paginator_name = format!("{paginator_name}Items");
        let item_lens =
            format!("crate::lens::lens_{operation_module}_output_output_{items_suffix}");
        writeln!(
            output,
            "/// Flattened paginator for `{paginator_name}`\n///\n/// This is created with [`.items()`]({paginator_name}::items)\npub struct {item_paginator_name}({paginator_name});\n\nimpl {item_paginator_name} {{\n    /// Create the pagination stream\n    ///\n    /// _Note_: No requests will be dispatched until the stream is used\n    /// (e.g. with the [`.next().await`](aws_smithy_async::future::pagination_stream::PaginationStream::next) method).\n    ///\n    /// To read the entirety of the paginator, use [`.collect::<Result<Vec<_>, _>()`](aws_smithy_async::future::pagination_stream::PaginationStream::collect).\n    pub fn send(\n        self,\n    ) -> ::aws_smithy_async::future::pagination_stream::PaginationStream<\n        ::std::result::Result<\n            {item_type},\n            ::aws_smithy_runtime_api::client::result::SdkError<\n                {error_type},\n                ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,\n            >,\n        >,\n    > {{\n        ::aws_smithy_async::future::pagination_stream::TryFlatMap::new(self.0.send())\n            .flat_map(|page| {item_lens}(page).unwrap_or_default().into_iter())\n    }}\n}}\n"
        )
        .unwrap();
    }

    if operation_type != operation_symbol {
        output = output.replace(
            &format!("{operation_path}::{operation_type}"),
            &format!("{operation_path}::{operation_symbol}"),
        );
        output = output.replace(
            &format!("{operation_path}::{operation_symbol}Output"),
            &format!("{operation_path}::{operation_type}Output"),
        );
    }
    output
}

fn render_consumer_paginator_file(selected: &SelectedModel, operation_name: &str) -> String {
    let info = operation_pagination_info(selected, operation_name)
        .expect("paginator file only exists for paginated operations");
    let operation_module = names::snake_case(operation_name);
    let operation_type = rust_type_name(operation_name);
    let paginator_name = format!("{operation_type}Paginator");
    let mut output = String::new();
    client_operation_header(&mut output);
    writeln!(
        output,
        "/// Paginator for [`{operation_name}`](super::{operation_type})\npub struct {paginator_name} {{\n    builder: super::builders::Builder,\n    stop_on_duplicate_token: bool,\n}}\n\nimpl {paginator_name} {{\n    pub(crate) fn new(builder: super::builders::Builder) -> Self {{\n        Self {{ builder, stop_on_duplicate_token: true }}\n    }}\n"
    )
    .unwrap();
    if let Some(page_size_path) = info.page_size.as_ref() {
        let page_size_member = names::rust_identifier(
            page_size_path
                .last()
                .expect("pagination page size path is not empty"),
        );
        let input_id = operation_shape(selected, operation_name)
            .and_then(|operation| operation.get("input"))
            .and_then(target_value)
            .expect("paginated operation input exists");
        let (page_size_target, _) = find_member_path(selected, input_id, page_size_path)
            .expect("paginated page size exists");
        let page_size_type = type_expr(
            selected,
            &page_size_target,
            Context::Builder {
                module: operation_module.clone(),
                input: true,
                consumer_namespace: true,
            },
        );
        writeln!(
            output,
            "    pub fn page_size(mut self, limit: {page_size_type}) -> Self {{\n        self.builder = self.builder.{page_size_member}(limit);\n        self\n    }}\n"
        )
        .unwrap();
    }
    if info.items.is_some() {
        let item_paginator_name = format!("{paginator_name}Items");
        writeln!(
            output,
            "    pub fn items(self) -> super::{item_paginator_name} {{\n        super::{item_paginator_name}(self)\n    }}\n"
        )
        .unwrap();
    }
    output.push_str(
        "    pub fn stop_on_duplicate_token(mut self, stop_on_duplicate_token: bool) -> Self {\n        self.stop_on_duplicate_token = stop_on_duplicate_token;\n        self\n    }\n\n",
    );
    writeln!(
        output,
        "    pub fn send(self) -> impl ::std::future::Future<Output = ::std::result::Result<super::{operation_type}Output, super::{operation_type}Error>> {{\n        self.builder.send()\n    }}\n}}\n"
    )
    .unwrap();

    if let Some(items_path) = info.items.as_ref() {
        let output_id = operation_shape(selected, operation_name)
            .and_then(|operation| operation.get("output"))
            .and_then(target_value)
            .expect("paginated operation output exists");
        let (items_target, _) =
            find_member_path(selected, output_id, items_path).expect("paginated items exists");
        let item_type = paginator_item_type(selected, &items_target, true);
        let items_suffix = items_path
            .iter()
            .map(|name| names::snake_case(name))
            .collect::<Vec<_>>()
            .join("_");
        let item_lens = format!(
            "super::super::super::lens::lens_{operation_module}_output_output_{items_suffix}"
        );
        let item_paginator_name = format!("{paginator_name}Items");
        writeln!(
            output,
            "pub struct {item_paginator_name}({paginator_name});\n\nimpl {item_paginator_name} {{\n    pub fn send(self) -> impl ::std::future::Future<Output = ::std::result::Result<::std::vec::Vec<{item_type}>, super::{operation_type}Error>> {{\n        async move {{\n            self.0.send().await.map(|page| {item_lens}(page).unwrap_or_default())\n        }}\n    }}\n}}\n"
        )
        .unwrap();
    }
    output
}

fn paginator_item_type(selected: &SelectedModel, target: &str, consumer_namespace: bool) -> String {
    let Some(shape) = selected.model.shapes.get(target) else {
        return lens_type_expr(selected, target, consumer_namespace);
    };
    let type_expr = |target: &str| {
        let type_name = lens_type_expr(selected, target, consumer_namespace);
        if consumer_namespace {
            type_name.replace("super::types", "super::super::super::types")
        } else {
            type_name
        }
    };
    match shape.get("type").and_then(Value::as_str) {
        Some("list") => shape
            .get("member")
            .and_then(member_target)
            .map(type_expr)
            .unwrap_or_else(|| "::std::string::String".to_owned()),
        Some("map") => {
            let key = shape
                .get("key")
                .and_then(member_target)
                .map(&type_expr)
                .unwrap_or_else(|| "::std::string::String".to_owned());
            let value = shape
                .get("value")
                .and_then(member_target)
                .map(type_expr)
                .unwrap_or_else(|| "::std::string::String".to_owned());
            format!("({key}, {value})")
        }
        _ => type_expr(target),
    }
}

fn lens_type_expr(selected: &SelectedModel, target: &str, consumer_namespace: bool) -> String {
    if target.starts_with("smithy.api#") {
        return primitive_type_for_namespace(terminal(target), false);
    }
    let Some(shape) = selected.model.shapes.get(target) else {
        return "::std::string::String".to_owned();
    };
    match shape.get("type").and_then(Value::as_str) {
        Some(
            "string" | "integer" | "long" | "short" | "byte" | "float" | "double" | "boolean"
            | "blob" | "timestamp" | "document",
        ) => primitive_type_for_namespace(
            shape
                .get("type")
                .and_then(Value::as_str)
                .unwrap_or("string"),
            false,
        ),
        Some("list") => shape
            .get("member")
            .and_then(member_target)
            .map(|member| {
                format!(
                    "::std::vec::Vec<{}>",
                    lens_type_expr(selected, member, consumer_namespace)
                )
            })
            .unwrap_or_else(|| "::std::vec::Vec<::std::string::String>".to_owned()),
        Some("map") => {
            let key = shape
                .get("key")
                .and_then(member_target)
                .map(|member| lens_type_expr(selected, member, consumer_namespace))
                .unwrap_or_else(|| "::std::string::String".to_owned());
            let value = shape
                .get("value")
                .and_then(member_target)
                .map(|member| lens_type_expr(selected, member, consumer_namespace))
                .unwrap_or_else(|| "::std::string::String".to_owned());
            format!("::std::collections::HashMap<{key}, {value}>")
        }
        _ => format!(
            "{}::{}",
            if consumer_namespace {
                "super::types"
            } else {
                "crate::types"
            },
            rust_type_name(terminal(target))
        ),
    }
}

fn operation_module_to_type(operation_module: &str) -> String {
    rust_type_name(operation_module)
}

fn render_borrowed_lens_path(
    output: &mut String,
    selected: &SelectedModel,
    root_id: &str,
    path: &[String],
) {
    let mut shape_id = root_id;
    for name in path {
        let field = names::rust_identifier(name);
        let shape = selected
            .model
            .shapes
            .get(shape_id)
            .expect("pagination lens root shape exists");
        let member = members(shape)
            .into_iter()
            .find(|(member_name, _)| member_name == name)
            .map(|(_, member)| member)
            .expect("pagination lens member exists");
        let target = member_target(member).expect("pagination lens member target exists");
        if member_is_effectively_required(selected, member, target) {
            writeln!(output, "    let input = &input.{field};").unwrap();
        } else {
            writeln!(
                output,
                "    let input = match &input.{field} {{\n        ::std::option::Option::None => return ::std::option::Option::None,\n        ::std::option::Option::Some(t) => t,\n    }};"
            )
            .unwrap();
        }
        shape_id = target;
    }
}

fn render_owned_lens_path(
    output: &mut String,
    selected: &SelectedModel,
    root_id: &str,
    path: &[String],
) {
    let mut shape_id = root_id;
    for name in path {
        let field = names::rust_identifier(name);
        let shape = selected
            .model
            .shapes
            .get(shape_id)
            .expect("pagination lens root shape exists");
        let member = members(shape)
            .into_iter()
            .find(|(member_name, _)| member_name == name)
            .map(|(_, member)| member)
            .expect("pagination lens member exists");
        let target = member_target(member).expect("pagination lens member target exists");
        if member_is_effectively_required(selected, member, target) {
            writeln!(output, "    let input = input.{field};").unwrap();
        } else {
            writeln!(output, "    let input = input.{field}?;").unwrap();
        }
        shape_id = target;
    }
}

fn pagination_item_type(selected: &SelectedModel, target: &str, types_prefix: &str) -> String {
    let Some(shape) = selected.model.shapes.get(target) else {
        return format!(
            "::std::vec::Vec<{types_prefix}::{}>",
            rust_type_name(terminal(target))
        );
    };
    match shape.get("type").and_then(Value::as_str) {
        Some("list") => shape
            .get("member")
            .and_then(member_target)
            .map(|member| {
                format!(
                    "::std::vec::Vec<{}>",
                    lens_type_expr(selected, member, types_prefix == "super::types")
                )
            })
            .unwrap_or_else(|| "::std::vec::Vec<::std::string::String>".to_owned()),
        Some("map") => {
            let key = shape
                .get("key")
                .and_then(member_target)
                .map(|member| lens_type_expr(selected, member, types_prefix == "super::types"))
                .unwrap_or_else(|| "::std::string::String".to_owned());
            let value = shape
                .get("value")
                .and_then(member_target)
                .map(|member| lens_type_expr(selected, member, types_prefix == "super::types"))
                .unwrap_or_else(|| "::std::string::String".to_owned());
            format!("::std::collections::HashMap<{key}, {value}>")
        }
        _ => format!(
            "::std::vec::Vec<{types_prefix}::{}>",
            rust_type_name(terminal(target))
        ),
    }
}

fn operation_shape_file_name(operation_name: &str, input: bool) -> String {
    format!(
        "{}{}",
        rust_type_name(operation_name),
        if input { "Input" } else { "Output" }
    )
}

fn render_operation_shape_file(
    selected: &SelectedModel,
    operation_name: &str,
    input: bool,
    consumer_namespace: bool,
) -> String {
    let operation = operation_shape(selected, operation_name).expect("selected operation exists");
    let module = names::snake_case(operation_name);
    let shape_id = operation
        .get(if input { "input" } else { "output" })
        .and_then(target_value);
    let shape = shape_id.and_then(|id| selected.model.shapes.get(id));
    let rust_name = operation_shape_file_name(operation_name, input);
    let mut output = String::new();
    header(&mut output);
    // Smithy emits operation input/output modules directly after the header;
    // the standalone modeled type files retain the separating blank line.
    if shape.is_none_or(|shape| documentation(shape).is_none()) {
        output.pop();
    }
    if let Some(shape) = shape {
        render_structure_at_indent(
            &mut output,
            selected,
            shape,
            &rust_name,
            Context::Operation {
                module: module.clone(),
                input,
                consumer_namespace,
            },
            0,
        );
        render_structure_accessors(
            &mut output,
            selected,
            shape,
            &rust_name,
            Context::Operation {
                module: module.clone(),
                input,
                consumer_namespace,
            },
            0,
        );
        render_type_builder(
            &mut output,
            selected,
            shape,
            &rust_name,
            Context::Builder {
                module: module.clone(),
                input,
                consumer_namespace,
            },
            0,
        );
    } else if input {
        writeln!(output, "#[derive(Clone, Debug)]\npub struct {rust_name};").unwrap();
    } else {
        let empty_output = serde_json::json!({
            "type": "structure",
            "traits": { "smithy.api#output": {} }
        });
        let context = Context::Operation {
            module: module.clone(),
            input: false,
            consumer_namespace,
        };
        if output.ends_with("\n\n") {
            output.pop();
        }
        render_structure_at_indent(
            &mut output,
            selected,
            &empty_output,
            &rust_name,
            context.clone(),
            0,
        );
        render_structure_accessors(
            &mut output,
            selected,
            &empty_output,
            &rust_name,
            context.clone(),
            0,
        );
        render_type_builder(
            &mut output,
            selected,
            &empty_output,
            &rust_name,
            Context::Builder {
                module,
                input: false,
                consumer_namespace,
            },
            0,
        );
    }
    output
}

fn render_operation_builder_file(
    selected: &SelectedModel,
    operation_name: &str,
    consumer_namespace: bool,
) -> String {
    if !consumer_namespace {
        return render_standalone_fluent_operation_builder_file(selected, operation_name);
    }
    let operation = operation_shape(selected, operation_name).expect("selected operation exists");
    let protocol = selected
        .model
        .protocol()
        .expect("selected model protocol was validated before rendering");
    let module = names::snake_case(operation_name);
    let rust_operation = rust_type_name(operation_name);
    let operation_symbol = operation_name;
    let mut output = String::new();
    header(&mut output);
    let input_has_streaming_member = operation
        .get("input")
        .and_then(target_value)
        .and_then(|input_id| selected.model.shapes.get(input_id))
        .is_some_and(structure_has_streaming_member);
    let builder_derives = if input_has_streaming_member {
        "Debug, Default"
    } else {
        "Clone, Debug, Default"
    };
    let input_builder_path = format!("super::_{module}_input::{rust_operation}InputBuilder");
    writeln!(
        output,
        "#[derive({builder_derives})]\npub struct Builder {{\n    input: {input_builder_path},\n    client: super::super::super::Client,\n}}\nimpl Builder {{\n    pub fn new() -> Self {{ Self::default() }}\n    pub fn with_client(client: super::super::super::Client) -> Self {{\n        Self {{ input: ::std::default::Default::default(), client }}\n    }}"
    )
    .unwrap();
    if let Some(input_id) = operation.get("input").and_then(target_value)
        && let Some(shape) = selected.model.shapes.get(input_id)
    {
        for (name, member) in members(shape) {
            let field = names::rust_identifier(&name);
            let target = member_target(member)
                .map(|target| {
                    type_expr(
                        selected,
                        target,
                        Context::Builder {
                            module: module.clone(),
                            input: true,
                            consumer_namespace,
                        },
                    )
                })
                .unwrap_or_else(|| "::std::string::String".to_owned());
            let assignment = format!("self.input = self.input.set_{field}(Some(value.into()))");
            writeln!(
                output,
                "    pub fn {field}(mut self, value: impl ::std::convert::Into<{target}>) -> Self {{ {assignment}; self }}"
            )
            .unwrap();
            if consumer_namespace {
                writeln!(
                    output,
                    "    pub fn set_{field}(mut self, value: ::std::option::Option<{target}>) -> Self {{ self.input.{field} = value; self }}\n    pub fn get_{field}(&self) -> &::std::option::Option<{target}> {{ &self.input.{field} }}"
                )
                .unwrap();
            }
        }
    }
    if operation_pagination_info(selected, operation_name).is_some() {
        if consumer_namespace {
            writeln!(
                output,
                "    pub fn into_paginator(self) -> super::paginator::{rust_operation}Paginator {{\n        super::paginator::{rust_operation}Paginator::new(self)\n    }}\n"
            )
            .unwrap();
        } else {
            writeln!(
                output,
                "    /// Create a paginator for this request\n    ///\n    /// Paginators are used by calling [`send().await`](crate::operation::{module}::paginator::{rust_operation}Paginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).\n    pub fn into_paginator(self) -> crate::operation::{module}::paginator::{rust_operation}Paginator {{\n        crate::operation::{module}::paginator::{rust_operation}Paginator::new(self.handle, self.inner)\n    }}\n"
            )
            .unwrap();
        }
    }
    output.push_str(
        "    pub fn build(self) -> super::Input { self.input.build().expect(\"operation input builder cannot fail\") }\n",
    );
    render_operation_send(
        &mut output,
        operation_name,
        selected,
        operation,
        protocol,
        consumer_namespace,
    );
    output.push_str("}\n");
    writeln!(
        output,
        "pub use Builder as {operation_symbol}FluentBuilder;"
    )
    .unwrap();
    output
}

fn render_standalone_fluent_operation_builder_file(
    selected: &SelectedModel,
    operation_name: &str,
) -> String {
    let operation = operation_shape(selected, operation_name).expect("selected operation exists");
    let module = names::snake_case(operation_name);
    let operation_type = rust_type_name(operation_name);
    let input_builder_path =
        format!("crate::operation::{module}::builders::{operation_type}InputBuilder");
    let output_path = format!("crate::operation::{module}::{operation_type}Output");
    let error_path = format!("crate::operation::{module}::{operation_type}Error");
    let input_shape = operation
        .get("input")
        .and_then(target_value)
        .and_then(|id| selected.model.shapes.get(id));
    let mut output = String::new();
    client_operation_header(&mut output);
    writeln!(
        output,
        "pub use crate::operation::{module}::_{module}_input::{operation_type}InputBuilder;\n\npub use crate::operation::{module}::_{module}_output::{operation_type}OutputBuilder;\n"
    )
    .unwrap();
    writeln!(
        output,
        "impl {input_builder_path} {{\n    /// Sends a request with this input using the given client.\n    pub async fn send_with(\n        self,\n        client: &crate::Client,\n    ) -> ::std::result::Result<\n        {output_path},\n        ::aws_smithy_runtime_api::client::result::SdkError<\n            {error_path},\n            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,\n        >,\n    > {{\n        let mut fluent_builder = client.{module}();\n        fluent_builder.inner = self;\n        fluent_builder.send().await\n    }}\n}}"
    )
    .unwrap();

    writeln!(
        output,
        "/// Fluent builder constructing a request to `{operation_type}`."
    )
    .unwrap();
    if let Some(documentation) = documentation(operation) {
        output.push_str("///\n");
        render_doc_lines(&mut output, &documentation, 0);
    }
    render_deprecated_attribute(&mut output, operation, 0);
    if input_shape.is_some_and(structure_has_streaming_member) {
        output.push_str("#[derive(::std::fmt::Debug)]\n");
    } else {
        output.push_str("#[derive(::std::clone::Clone, ::std::fmt::Debug)]\n");
    }
    writeln!(
        output,
        "pub struct {operation_type}FluentBuilder {{\n    handle: ::std::sync::Arc<crate::client::Handle>,\n    inner: {input_builder_path},\n    config_override: ::std::option::Option<crate::config::Builder>,\n}}"
    )
    .unwrap();
    writeln!(
        output,
        "impl\n    crate::client::customize::internal::CustomizableSend<\n        {output_path},\n        {error_path},\n    > for {operation_type}FluentBuilder\n{{\n    fn send(\n        self,\n        config_override: crate::config::Builder,\n    ) -> crate::client::customize::internal::BoxFuture<\n        crate::client::customize::internal::SendResult<\n            {output_path},\n            {error_path},\n        >,\n    > {{\n        ::std::boxed::Box::pin(async move {{ self.config_override(config_override).send().await }})\n    }}\n}}"
    )
    .unwrap();
    writeln!(
        output,
        "impl {operation_type}FluentBuilder {{\n    /// Creates a new `{operation_type}FluentBuilder`.\n    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {{\n        Self {{\n            handle,\n            inner: ::std::default::Default::default(),\n            config_override: ::std::option::Option::None,\n        }}\n    }}\n    /// Access the {operation_type} as a reference.\n    pub fn as_input(&self) -> &{input_builder_path} {{\n        &self.inner\n    }}"
    )
    .unwrap();
    output.push_str(
        "    /// Sends the request and returns the response.\n    ///\n    /// If an error occurs, an `SdkError` will be returned with additional details that\n    /// can be matched against.\n    ///\n    /// By default, any retryable failures will be retried twice. Retry behavior\n    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be\n    /// set when configuring the client.\n    pub async fn send(\n        self,\n    ) -> ::std::result::Result<\n",
    );
    writeln!(
        output,
        "        {output_path},\n        ::aws_smithy_runtime_api::client::result::SdkError<\n            {error_path},\n            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,\n        >,\n    > {{\n        let input = self\n            .inner\n            .build()\n            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;\n        let runtime_plugins = crate::operation::{module}::{operation_type}::operation_runtime_plugins(\n            self.handle.runtime_plugins.clone(),\n            &self.handle.conf,\n            self.config_override,\n        );\n        crate::operation::{module}::{operation_type}::orchestrate(&runtime_plugins, input).await\n    }}\n\n    /// Consumes this builder, creating a customizable operation that can be modified before being sent.\n    pub fn customize(\n        self,\n    ) -> crate::client::customize::CustomizableOperation<\n        {output_path},\n        {error_path},\n        Self,\n    > {{\n        crate::client::customize::CustomizableOperation::new(self)\n    }}\n    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {{\n        self.set_config_override(::std::option::Option::Some(config_override.into()));\n        self\n    }}\n\n    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {{\n        self.config_override = config_override;\n        self\n    }}"
    )
    .unwrap();
    if operation_pagination_info(selected, operation_name).is_some() {
        writeln!(
            output,
            "    /// Create a paginator for this request\n    ///\n    /// Paginators are used by calling [`send().await`](crate::operation::{module}::paginator::{operation_type}Paginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).\n    pub fn into_paginator(self) -> crate::operation::{module}::paginator::{operation_type}Paginator {{\n        crate::operation::{module}::paginator::{operation_type}Paginator::new(self.handle, self.inner)\n    }}"
        )
        .unwrap();
    }
    if operation_is_presignable(selected, operation) {
        render_standalone_presigned_method(&mut output, &module, &operation_type);
    }
    if let Some(shape) = input_shape {
        for (member_name, member) in members(shape) {
            render_standalone_fluent_member_helpers(
                &mut output,
                selected,
                &module,
                member_name,
                member,
            );
        }
    }
    output.push_str("}\n");
    if operation_is_presignable(selected, operation) {
        writeln!(
            output,
            "\nimpl crate::client::customize::internal::CustomizablePresigned<crate::operation::{module}::{operation_type}Error> for {operation_type}FluentBuilder {{\n    fn presign(\n        self,\n        config_override: crate::config::Builder,\n        presigning_config: crate::presigning::PresigningConfig,\n    ) -> crate::client::customize::internal::BoxFuture<\n        crate::client::customize::internal::SendResult<crate::presigning::PresignedRequest, crate::operation::{module}::{operation_type}Error>,\n    > {{\n        ::std::boxed::Box::pin(async move {{ self.config_override(config_override).presigned(presigning_config).await }})\n    }}\n}}"
        )
        .unwrap();
    }
    output
}

fn operation_is_presignable(selected: &SelectedModel, operation: &Value) -> bool {
    let Some(http) = operation_http_trait(operation) else {
        return false;
    };
    let Some(uri) = http.get("uri").and_then(Value::as_str) else {
        return false;
    };
    let (path, query) = uri.split_once('?').unwrap_or((uri, ""));
    if path != "/{Bucket}/{Key+}" {
        return false;
    }
    let input_shape = operation
        .get("input")
        .and_then(target_value)
        .and_then(|id| selected.model.shapes.get(id));
    let output_shape = operation
        .get("output")
        .and_then(target_value)
        .and_then(|id| selected.model.shapes.get(id));
    let method = http
        .get("method")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let has_query = |name: &str| {
        input_shape.is_some_and(|shape| {
            members(shape).into_iter().any(|(_, member)| {
                member
                    .get("traits")
                    .and_then(Value::as_object)
                    .and_then(|traits| traits.get("smithy.api#httpQuery"))
                    .and_then(Value::as_str)
                    .is_some_and(|value| value.eq_ignore_ascii_case(name))
            })
        })
    };
    match method {
        "DELETE" => query.starts_with("x-id=") && !has_query("uploadId"),
        "GET" => {
            query.starts_with("x-id=") && output_shape.is_some_and(structure_has_streaming_member)
        }
        "HEAD" => query.is_empty(),
        "PUT" => {
            query.starts_with("x-id=") && input_shape.is_some_and(structure_has_streaming_member)
        }
        _ => false,
    }
}

fn render_standalone_presigned_method(output: &mut String, module: &str, operation_type: &str) {
    output.push_str(
        "    ///\n    /// Creates a presigned request for this operation.\n    ///\n    /// The `presigning_config` provides additional presigning-specific config values, such as the\n    /// amount of time the request should be valid for after creation.\n    ///\n    /// Presigned requests can be given to other users or applications to access a resource or perform\n    /// an operation without having access to the AWS security credentials.\n    ///\n    /// _Important:_ If you're using credentials that can expire, such as those from STS AssumeRole or SSO, then\n    /// the presigned request can only be valid for as long as the credentials used to create it are.\n    ///\n    #[allow(unused_mut)]\n    pub async fn presigned(\n        mut self,\n        presigning_config: crate::presigning::PresigningConfig,\n    ) -> ::std::result::Result<\n        crate::presigning::PresignedRequest,\n        ::aws_smithy_runtime_api::client::result::SdkError<\n",
    );
    writeln!(
        output,
        "            crate::operation::{module}::{operation_type}Error,\n            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,\n        >,\n    > {{\n        let runtime_plugins = crate::operation::{module}::{operation_type}::operation_runtime_plugins(\n            self.handle.runtime_plugins.clone(),\n            &self.handle.conf,\n            self.config_override,\n        )\n        .with_client_plugin(crate::presigning_interceptors::SigV4PresigningRuntimePlugin::new(\n            presigning_config,\n            ::aws_sigv4::http_request::SignableBody::UnsignedPayload,\n        ));\n\n        let input = self\n            .inner\n            .build()\n            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;\n        let mut context = crate::operation::{module}::{operation_type}::orchestrate_with_stop_point(\n            &runtime_plugins,\n            input,\n            ::aws_smithy_runtime::client::orchestrator::StopPoint::BeforeTransmit,\n        )\n        .await\n        .map_err(|err| {{\n            err.map_service_error(|err| {{\n                err.downcast::<crate::operation::{module}::{operation_type}Error>()\n                    .expect(\"correct error type\")\n            }})\n        }})?;\n        let request = context.take_request().expect(\"request set before transmit\");\n        crate::presigning::PresignedRequest::new(request).map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)\n    }}"
    )
    .unwrap();
}

fn render_standalone_fluent_member_helpers(
    output: &mut String,
    selected: &SelectedModel,
    module: &str,
    member_name: String,
    member: &Value,
) {
    let field = names::rust_identifier(&member_name);
    let field_method = field.strip_prefix("r#").unwrap_or(&field);
    let target_id = member_target(member).unwrap_or("smithy.api#String");
    let context = Context::Builder {
        module: module.to_owned(),
        input: true,
        consumer_namespace: false,
    };
    let target = type_expr(selected, target_id, context.clone());
    let target_shape = selected.model.shapes.get(target_id);
    if let Some(list_shape) =
        target_shape.filter(|shape| shape.get("type").and_then(Value::as_str) == Some("list"))
    {
        let element_target = list_shape
            .get("member")
            .and_then(member_target)
            .unwrap_or("smithy.api#String");
        let element_type = type_expr(selected, element_target, context.clone());
        let argument = builder_argument_type(selected, element_target, &element_type);
        output.push_str(&format!(
            "    ///\n    /// Appends an item to `{member_name}`.\n    ///\n    /// To override the contents of this collection use [`set_{field_method}`](Self::set_{field_method}).\n    ///\n"
        ));
        render_fluent_member_docs(output, selected, member);
        render_deprecated_attribute(output, member, 4);
        writeln!(
            output,
            "    pub fn {field}(mut self, input: {argument}) -> Self {{\n        self.inner = self.inner.{field}({});\n        self\n    }}",
            builder_argument_value(&argument, "input")
        )
        .unwrap();
    } else if let Some(map_shape) =
        target_shape.filter(|shape| shape.get("type").and_then(Value::as_str) == Some("map"))
    {
        let key_target = map_shape
            .get("key")
            .and_then(member_target)
            .unwrap_or("smithy.api#String");
        let value_target = map_shape
            .get("value")
            .and_then(member_target)
            .unwrap_or("smithy.api#String");
        let key_type = type_expr(selected, key_target, context.clone());
        let value_type = type_expr(selected, value_target, context.clone());
        let key_argument = builder_argument_type(selected, key_target, &key_type);
        let value_argument = builder_argument_type(selected, value_target, &value_type);
        output.push_str(&format!(
            "    ///\n    /// Adds a key-value pair to `{member_name}`.\n    ///\n    /// To override the contents of this collection use [`set_{field_method}`](Self::set_{field_method}).\n    ///\n"
        ));
        render_fluent_member_docs(output, selected, member);
        render_deprecated_attribute(output, member, 4);
        writeln!(
            output,
            "    pub fn {field}(mut self, k: {key_argument}, v: {value_argument}) -> Self {{\n        self.inner = self.inner.{field}({}, {});\n        self\n    }}",
            builder_argument_value(&key_argument, "k"),
            builder_argument_value(&value_argument, "v")
        )
        .unwrap();
    } else {
        let argument = builder_argument_type(selected, target_id, &target);
        render_fluent_member_docs(output, selected, member);
        render_deprecated_attribute(output, member, 4);
        writeln!(
            output,
            "    pub fn {field}(mut self, input: {argument}) -> Self {{\n        self.inner = self.inner.{field}({});\n        self\n    }}",
            builder_argument_value(&argument, "input")
        )
        .unwrap();
    }
    render_fluent_member_docs(output, selected, member);
    render_deprecated_attribute(output, member, 4);
    writeln!(
        output,
        "    pub fn set_{field_method}(mut self, input: ::std::option::Option<{target}>) -> Self {{\n        self.inner = self.inner.set_{field_method}(input);\n        self\n    }}"
    )
    .unwrap();
    render_fluent_member_docs(output, selected, member);
    render_deprecated_attribute(output, member, 4);
    writeln!(
        output,
        "    pub fn get_{field_method}(&self) -> &::std::option::Option<{target}> {{\n        self.inner.get_{field_method}()\n    }}"
    )
    .unwrap();
}

fn render_fluent_member_docs(output: &mut String, selected: &SelectedModel, member: &Value) {
    if let Some(documentation) = modeled_member_documentation(selected, member) {
        render_doc_lines(output, &documentation, 4);
    }
}

fn render_operation_error(
    output: &mut String,
    selected: &SelectedModel,
    operation: &Value,
    consumer_namespace: bool,
) {
    let request_id_plan = request_id_plan(selected);
    output.push_str("#[derive(Clone, Debug)]\npub enum Error {\n");
    if let Some(errors) = operation.get("errors").and_then(Value::as_array) {
        for error in errors.iter().filter_map(target_value) {
            let error_name = rust_type_name(terminal(error));
            writeln!(
                output,
                "    {error_name}(super::super::types::error::{error_name}),"
            )
            .unwrap();
        }
    }
    output.push_str(
        "    Unhandled(::std::string::String),\n    UnhandledWithRequestIds { message: ::std::string::String, request_id: ::std::option::Option<::std::string::String>, extended_request_id: ::std::option::Option<::std::string::String> },\n}\nimpl Error {\n",
    );
    if let Some(errors) = operation.get("errors").and_then(Value::as_array) {
        for error in errors.iter().filter_map(target_value) {
            let error_name = rust_type_name(terminal(error));
            let predicate = names::snake_case(terminal(error));
            writeln!(output, "    pub fn is_{predicate}(&self) -> bool {{ matches!(self, Self::{error_name}(_)) }}").unwrap();
        }
    }
    output.push_str("}\nimpl ::std::fmt::Display for Error {\n    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {\n        match self {\n            Self::Unhandled(message) => f.write_str(message),\n            Self::UnhandledWithRequestIds { message, .. } => f.write_str(message),\n");
    if let Some(errors) = operation.get("errors").and_then(Value::as_array) {
        for error in errors.iter().filter_map(target_value) {
            let error_name = rust_type_name(terminal(error));
            writeln!(
                output,
                "            Self::{error_name}(value) => value.fmt(f),"
            )
            .unwrap();
        }
    }
    output.push_str("        }\n    }\n}\nimpl ::std::error::Error for Error {}\n");
    if request_id_plan.standard {
        let operation_path = "Error";
        let error_path = if consumer_namespace {
            "super::super::error::ErrorMetadata".to_owned()
        } else {
            "crate::error::ErrorMetadata".to_owned()
        };
        writeln!(
            output,
            "impl Error {{\n    pub(crate) fn unhandled_with_request_ids(message: impl ::std::convert::Into<::std::string::String>, request_id: ::std::option::Option<::std::string::String>, extended_request_id: ::std::option::Option<::std::string::String>) -> Self {{ Self::UnhandledWithRequestIds {{ message: message.into(), request_id, extended_request_id }} }}\n    pub fn meta(&self) -> {error_path} {{ match self {{ Self::UnhandledWithRequestIds {{ request_id, extended_request_id, .. }} => {error_path}::from_request_ids(request_id.clone(), extended_request_id.clone()), _ => {error_path}::default() }} }}\n}}\nimpl ::aws_types::request_id::RequestId for {operation_path} {{\n    fn request_id(&self) -> Option<&str> {{ match self {{ Self::UnhandledWithRequestIds {{ request_id, .. }} => request_id.as_deref(), _ => None }} }}\n}}",
        )
        .unwrap();
        if request_id_plan.extended {
            let trait_path = if consumer_namespace {
                "super::super::s3_request_id::RequestIdExt"
            } else {
                "crate::s3_request_id::RequestIdExt"
            };
            writeln!(
                output,
                "impl {trait_path} for {operation_path} {{\n    fn extended_request_id(&self) -> Option<&str> {{ match self {{ Self::UnhandledWithRequestIds {{ extended_request_id, .. }} => extended_request_id.as_deref(), _ => None }} }}\n}}"
            )
            .unwrap();
        }
    }
}

fn render_operation_send(
    output: &mut String,
    operation_name: &str,
    selected: &SelectedModel,
    operation: &Value,
    protocol: crate::model::ProtocolKind,
    consumer_namespace: bool,
) {
    let rust_operation = rust_type_name(operation_name);
    let operation_symbol = operation_name;
    let request_id_plan = request_id_plan(selected);
    let method = operation
        .get("traits")
        .and_then(|traits| traits.get("smithy.api#http"))
        .and_then(|http| http.get("method"))
        .and_then(Value::as_str)
        .unwrap_or("POST");
    let http_method = match method {
        "GET" => "Get",
        "PUT" => "Put",
        "DELETE" => "Delete",
        "HEAD" => "Head",
        "PATCH" => "Patch",
        _ => "Post",
    };
    let uri = operation
        .get("traits")
        .and_then(|traits| traits.get("smithy.api#http"))
        .and_then(|http| http.get("uri"))
        .and_then(Value::as_str)
        .unwrap_or("/");
    let input_id = operation.get("input").and_then(target_value);
    let input_shape = input_id.and_then(|id| selected.model.shapes.get(id));
    let output_id = operation.get("output").and_then(target_value);
    let output_shape = output_id.and_then(|id| selected.model.shapes.get(id));
    let extended_request_id = if request_id_plan.extended {
        "response.header(\"x-amz-id-2\").map(str::to_owned)"
    } else {
        "::std::option::Option::None"
    };
    let path_expression = render_request_path(uri, input_shape, selected);
    let body_expression = render_request_body(selected, input_shape, protocol);
    let headers_expression = render_request_headers(input_shape, protocol);
    let send_allow = if consumer_namespace {
        "#[allow(clippy::possible_missing_else, clippy::field_reassign_with_default, clippy::result_large_err)]"
    } else {
        "#[allow(clippy::possible_missing_else, clippy::field_reassign_with_default)]"
    };

    writeln!(
        output,
        "                     {send_allow}\n                     pub async fn send(self) -> ::std::result::Result<super::{rust_operation}Output, super::{operation_symbol}Error> {{"
    )
    .unwrap();
    writeln!(
        output,
        "                         let input = self.input.build().map_err(|error| super::{operation_symbol}Error::Unhandled(error.to_string()))?;"
    )
    .unwrap();
    if let Some(shape) = input_shape {
        for (name, member) in members(shape) {
            let field = names::rust_identifier(&name);
            let required = member
                .get("traits")
                .and_then(|traits| traits.get("smithy.api#required"))
                .is_some();
            if required && uri.contains(&format!("{{{name}")) {
                writeln!(
                    output,
                    "                         let {field} = input.{field}.as_deref().ok_or_else(|| super::{operation_symbol}Error::Unhandled(\"{operation_symbol} requires {field}\".to_owned()))?;"
                )
                .unwrap();
            }
        }
    }
    writeln!(
        output,
        "                         let path = {path_expression};\n                         let body = {body_expression};\n                         let headers = {headers_expression};\n                         let response = self.client.request(super::super::super::transport::Method::{http_method}, &path, &headers, &body).await.map_err(super::{operation_symbol}Error::Unhandled)?;"
    )
    .unwrap();
    output.push_str("                         let status = response.status();\n                         if !status.is_success() {\n");
    writeln!(
        output,
        "                             return Err(super::{operation_symbol}Error::unhandled_with_request_ids(format!(\"{operation_symbol} returned HTTP {{}}\", status), response.header(\"x-amzn-requestid\").map(str::to_owned), {extended_request_id}));"
    )
    .unwrap();
    output.push_str("                         }\n");
    render_response_decode(
        output,
        operation_name,
        selected,
        output_shape,
        protocol,
        consumer_namespace,
    );
    output.push_str("                     }\n");
}

fn render_request_body(
    selected: &SelectedModel,
    input_shape: Option<&Value>,
    protocol: crate::model::ProtocolKind,
) -> String {
    let Some(shape) = input_shape else {
        return "::std::vec::Vec::new()".to_owned();
    };
    let Some((name, member)) = members(shape).into_iter().find(|(_, member)| {
        member
            .get("traits")
            .and_then(|traits| traits.get("smithy.api#httpPayload"))
            .is_some()
    }) else {
        return "::std::vec::Vec::new()".to_owned();
    };
    let field = names::rust_identifier(&name);
    let target = member_target(member).unwrap_or_default();
    if terminal(target) == "StreamingBlob" {
        return format!("input.{field}.clone().into_inner()");
    }
    if protocol != crate::model::ProtocolKind::RestXml {
        return "::std::vec::Vec::new()".to_owned();
    }

    let Some(target_shape) = selected.model.shapes.get(target) else {
        return "::std::vec::Vec::new()".to_owned();
    };
    let root = xml_name(member).unwrap_or_else(|| terminal(target).to_owned());
    let mut expression =
        String::from("{ let mut body = ::std::string::String::new(); if let Some(value) = input.");
    expression.push_str(&field);
    expression.push_str(".as_ref() {");
    render_xml_value(
        &mut expression,
        selected,
        target_shape,
        "value",
        &root,
        false,
        true,
    );
    expression.push_str(" } body.into_bytes() }");
    expression
}

fn render_xml_value(
    output: &mut String,
    selected: &SelectedModel,
    shape: &Value,
    value_expression: &str,
    tag: &str,
    flattened: bool,
    operation_input: bool,
) {
    let kind = shape
        .get("type")
        .and_then(Value::as_str)
        .unwrap_or_default();
    match kind {
        "structure" => {
            output.push_str(&format!(
                " body.push_str({open:?});",
                open = format!("<{tag}>")
            ));
            for (member_name, member) in members(shape) {
                let traits = member.get("traits").and_then(Value::as_object);
                if traits.is_some_and(|traits| {
                    [
                        "smithy.api#httpHeader",
                        "smithy.api#httpLabel",
                        "smithy.api#httpQuery",
                    ]
                    .iter()
                    .any(|trait_id| traits.contains_key(*trait_id))
                }) {
                    continue;
                }
                let Some(member_target) = member_target(member) else {
                    continue;
                };
                let Some(member_shape) = selected.model.shapes.get(member_target) else {
                    continue;
                };
                let member_tag = xml_name(member)
                    .or_else(|| xml_name(member_shape))
                    .unwrap_or_else(|| member_name.clone());
                let field = names::rust_identifier(&member_name);
                let field_expression = format!("{value_expression}.{field}");
                let member_is_optional = operation_input
                    || (!member_is_required(member) && !is_streaming_target(member_target));
                if member_is_optional {
                    output.push_str(&format!(
                        " if let Some(value) = {field_expression}.as_ref() {{"
                    ));
                }
                render_xml_value(
                    output,
                    selected,
                    member_shape,
                    if member_is_optional {
                        "value"
                    } else {
                        &field_expression
                    },
                    &member_tag,
                    traits.is_some_and(|traits| traits.contains_key("smithy.api#xmlFlattened")),
                    false,
                );
                if member_is_optional {
                    output.push_str(" }");
                }
            }
            output.push_str(&format!(
                " body.push_str({close:?});",
                close = format!("</{tag}>")
            ));
        }
        "list" => {
            let element_target = shape
                .get("member")
                .and_then(member_target)
                .unwrap_or("smithy.api#String");
            let Some(element_shape) = selected.model.shapes.get(element_target) else {
                return;
            };
            let flattened = shape
                .get("traits")
                .and_then(Value::as_object)
                .is_some_and(|traits| traits.contains_key("smithy.api#xmlFlattened"))
                || flattened;
            if !flattened {
                output.push_str(&format!(
                    " body.push_str({open:?});",
                    open = format!("<{tag}>")
                ));
            }
            output.push_str(&format!(" for item in {value_expression} {{"));
            let element_tag = shape
                .get("member")
                .and_then(xml_name)
                .unwrap_or_else(|| tag.to_owned());
            render_xml_value(
                output,
                selected,
                element_shape,
                "item",
                &element_tag,
                false,
                false,
            );
            output.push_str(" }");
            if !flattened {
                output.push_str(&format!(
                    " body.push_str({close:?});",
                    close = format!("</{tag}>")
                ));
            }
        }
        _ => {
            output.push_str(&format!(
                " body.push_str({open:?}); body.push_str(&super::super::super::transport::xml_escape(&{value_expression}.to_string())); body.push_str({close:?});",
                open = format!("<{tag}>"),
                close = format!("</{tag}>")
            ));
        }
    }
}

fn xml_name(value: &Value) -> Option<String> {
    value
        .get("traits")
        .and_then(|traits| traits.get("smithy.api#xmlName"))
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
}

fn synthetic_original_shape_id(shape: &Value) -> Option<&str> {
    shape
        .get("traits")
        .and_then(Value::as_object)
        .and_then(|traits| {
            [
                "smithy.api.internal#syntheticInput",
                "smithy.api.internal#syntheticOutput",
            ]
            .into_iter()
            .find_map(|trait_id| {
                traits
                    .get(trait_id)
                    .and_then(Value::as_object)
                    .and_then(|metadata| metadata.get("originalId"))
                    .and_then(Value::as_str)
            })
        })
}

fn protocol_operation_output_xml_name(
    selected: &SelectedModel,
    output_shape_id: &str,
    output_shape: &Value,
) -> Option<String> {
    let original_id = synthetic_original_shape_id(output_shape);
    let original_shape = original_id.and_then(|id| selected.model.shapes.get(id));
    original_shape
        .and_then(xml_name)
        .or_else(|| xml_name(output_shape))
        .or_else(|| original_id.map(|id| terminal(id).to_owned()))
        .or_else(|| Some(terminal(output_shape_id).to_owned()))
}

fn is_xml_body_member(member: &Value) -> bool {
    let Some(traits) = member.get("traits").and_then(Value::as_object) else {
        return true;
    };
    if traits.contains_key("smithy.api#httpHeader")
        || traits.contains_key("smithy.api#httpPrefixHeaders")
        || traits.contains_key("smithy.api#httpLabel")
        || traits.contains_key("smithy.api#httpQuery")
        || traits.contains_key("smithy.api#httpQueryParams")
        || traits.contains_key("smithy.api#httpResponseCode")
    {
        return false;
    }
    if traits.contains_key("smithy.api#httpPayload") {
        return terminal(member_target(member).unwrap_or_default()) != "StreamingBlob";
    }
    true
}

fn render_request_path(uri: &str, input_shape: Option<&Value>, selected: &SelectedModel) -> String {
    let has_query = input_shape
        .map(|shape| {
            members(shape).iter().any(|(_, member)| {
                member
                    .get("traits")
                    .and_then(|traits| traits.get("smithy.api#httpQuery"))
                    .is_some()
            })
        })
        .unwrap_or(false);
    if !uri.contains('{') && !has_query {
        return format!("{uri:?}");
    }
    let mut expression = format!("{{ let mut path = ::std::string::String::from({uri:?});");
    let mut replacements = BTreeMap::new();
    if let Some(shape) = input_shape {
        for (name, member) in members(shape) {
            if uri.contains(&format!("{{{name}")) {
                let field = names::rust_identifier(&name);
                replacements.insert(name.clone(), field);
            }
            if let Some(query) = member
                .get("traits")
                .and_then(|traits| traits.get("smithy.api#httpQuery"))
                .and_then(Value::as_str)
            {
                let field = names::rust_identifier(&name);
                let target = member_target(member).unwrap_or_default();
                let target_shape = selected.model.shapes.get(target);
                if is_string_type(target, target_shape) {
                    expression.push_str(&format!(
                        " if let Some(value) = input.{field}.as_deref() {{ path.push_str(if path.contains('?') {{ \"&\" }} else {{ \"?\" }}); path.push_str({query:?}); path.push('='); path.push_str(&super::super::super::transport::encode_path(value)); }}"
                    ));
                } else if !matches!(
                    target_shape
                        .and_then(|shape| shape.get("type"))
                        .and_then(Value::as_str),
                    Some(
                        "list" | "map" | "structure" | "union" | "blob" | "timestamp" | "document"
                    )
                ) {
                    expression.push_str(&format!(
                        " if let Some(value) = input.{field}.as_ref() {{ path.push_str(if path.contains('?') {{ \"&\" }} else {{ \"?\" }}); path.push_str({query:?}); path.push('='); path.push_str(&super::super::super::transport::encode_path(&value.to_string())); }}"
                    ));
                }
            }
        }
    }
    for (name, field) in replacements {
        let placeholder = if uri.contains(&format!("{{{name}+}}")) {
            format!("{{{name}+}}")
        } else {
            format!("{{{name}}}")
        };
        expression.push_str(&format!(
            " path = path.replace({placeholder:?}, &super::super::super::transport::encode_path({field}));"
        ));
    }
    expression.push_str(" path }");
    expression
}

fn render_request_headers(
    input_shape: Option<&Value>,
    protocol: crate::model::ProtocolKind,
) -> String {
    let mut header_pushes = Vec::new();
    if let Some(shape) = input_shape {
        for (name, member) in members(shape) {
            let Some(header) = member
                .get("traits")
                .and_then(|traits| traits.get("smithy.api#httpHeader"))
                .and_then(Value::as_str)
            else {
                continue;
            };
            let target = member_target(member).unwrap_or_default();
            let target_name = terminal(target);
            let is_string = target.starts_with("smithy.api#String")
                || matches!(
                    target_name,
                    "String"
                        | "BucketName"
                        | "ObjectKey"
                        | "AccountId"
                        | "Token"
                        | "ETag"
                        | "Location"
                );
            if !is_string {
                continue;
            }
            let field = names::rust_identifier(&name);
            header_pushes.push(format!(
                " if let Some(value) = input.{field}.as_deref() {{ headers.push(({header:?}, value)); }}"
            ));
        }
    }
    let has_headers = !header_pushes.is_empty();
    let has_xml_payload = protocol == crate::model::ProtocolKind::RestXml
        && input_shape.is_some_and(|shape| {
            members(shape).iter().any(|(_, member)| {
                member
                    .get("traits")
                    .and_then(|traits| traits.get("smithy.api#httpPayload"))
                    .is_some_and(|_| {
                        terminal(member_target(member).unwrap_or_default()) != "StreamingBlob"
                    })
            })
        });
    match (has_headers, has_xml_payload) {
        (false, false) => "::std::vec::Vec::new()".to_owned(),
        (false, true) => "::std::vec![(\"content-type\", \"application/xml\")]".to_owned(),
        (true, has_xml_payload) => {
            let mut output = String::from(
                "{ let mut headers: ::std::vec::Vec<(&str, &str)> = ::std::vec::Vec::new();",
            );
            for header_push in header_pushes {
                output.push_str(&header_push);
            }
            if has_xml_payload {
                output.push_str(" headers.push((\"content-type\", \"application/xml\"));");
            }
            output.push_str(" headers }");
            output
        }
    }
}

/// Render the Smithy protocol glue for one operation.
///
/// The operation-level file is deliberately driven by HTTP binding traits. It
/// does not know which AWS service owns the operation; service-specific request
/// ID behavior comes from the selected service metadata via `RequestIdPlan`.
fn render_protocol_operation_file(
    selected: &SelectedModel,
    operation_name: &str,
    consumer_namespace: bool,
) -> String {
    let operation = operation_shape(selected, operation_name).expect("selected operation exists");
    let output_shape = operation
        .get("output")
        .and_then(target_value)
        .and_then(|id| selected.model.shapes.get(id));
    let input_shape = operation
        .get("input")
        .and_then(target_value)
        .and_then(|id| selected.model.shapes.get(id));
    let mut output = String::new();
    client_operation_header(&mut output);

    let streaming_output = output_shape.is_some_and(|shape| {
        members(shape).into_iter().any(|(_, member)| {
            has_trait(member, "smithy.api#httpPayload")
                && member_target(member).is_some_and(|target| {
                    selected
                        .model
                        .shapes
                        .get(target)
                        .is_some_and(shape_is_streaming)
                })
        })
    });
    if streaming_output {
        render_protocol_http_response(
            &mut output,
            selected,
            operation_name,
            output_shape,
            consumer_namespace,
        );
        render_protocol_http_error(
            &mut output,
            selected,
            operation_name,
            operation,
            consumer_namespace,
        );
    } else {
        render_protocol_http_error(
            &mut output,
            selected,
            operation_name,
            operation,
            consumer_namespace,
        );
        render_protocol_http_response(
            &mut output,
            selected,
            operation_name,
            output_shape,
            consumer_namespace,
        );
    }
    render_protocol_request_headers(
        &mut output,
        selected,
        operation_name,
        input_shape,
        consumer_namespace,
    );
    if let Some(serializer) = render_protocol_operation_input_serializer(selected, operation_name) {
        output.push_str(&serializer);
    }
    if let Some(parser) = render_protocol_operation_output_parser(selected, operation_name, &output)
    {
        output.push_str(&parser);
    }
    output
}

/// Render the protocol-owned request payload wrapper for a RestXml operation.
///
/// Smithy keeps this code in a separate `shape_<operation>_input.rs` module.
/// The wrapper is deliberately derived from the HTTP payload member and its
/// target shape; the service and operation names only participate in stable
/// module/function names supplied by the model.
fn render_protocol_input_file(selected: &SelectedModel, operation_name: &str) -> Option<String> {
    let operation = operation_shape(selected, operation_name)?;
    let input_shape = operation
        .get("input")
        .and_then(target_value)
        .and_then(|id| selected.model.shapes.get(id))?;
    let payload_member = members(input_shape).into_iter().find(|(_, member)| {
        member
            .get("traits")
            .and_then(|traits| traits.get("smithy.api#httpPayload"))
            .is_some()
    });
    if payload_member.is_none() {
        return render_protocol_operation_input_file(selected, operation_name, input_shape);
    }
    let (member_name, member) = payload_member.expect("payload member exists");
    let target = member_target(member)?;
    let field = names::rust_identifier(&member_name);
    let target_shape = selected.model.shapes.get(target);
    let target_kind = target_shape
        .and_then(|shape| shape.get("type"))
        .and_then(Value::as_str)
        .or_else(|| target.strip_prefix("smithy.api#"))
        .unwrap_or_default();
    let mut output = String::new();
    client_operation_header(&mut output);

    if terminal(target) == "StreamingBlob" {
        writeln!(
            output,
            "pub fn ser_{field}_http_payload(\n    payload: ::aws_smithy_types::byte_stream::ByteStream,\n) -> ::std::result::Result<::aws_smithy_types::byte_stream::ByteStream, ::aws_smithy_types::error::operation::BuildError> {{\n    Ok(payload)\n}}"
        )
        .unwrap();
        return Some(output);
    }

    if target_kind == "string" {
        writeln!(
            output,
            "pub fn ser_{field}_http_payload(\n    payload: ::std::option::Option<::std::string::String>,\n) -> ::std::result::Result<::std::vec::Vec<u8>, ::aws_smithy_types::error::operation::BuildError> {{\n    let payload = match payload {{\n        Some(t) => t,\n        None => return Ok(Vec::new()),\n    }};\n    Ok(payload.into_bytes())\n}}"
        )
        .unwrap();
        return Some(output);
    }

    if target_kind == "blob" {
        writeln!(
            output,
            "pub fn ser_{field}_http_payload(\n    payload: ::std::option::Option<::aws_smithy_types::Blob>,\n) -> ::std::result::Result<::bytes::Bytes, ::aws_smithy_types::error::operation::BuildError> {{\n    let payload = match payload {{\n        Some(t) => t,\n        None => return Ok(::bytes::Bytes::new()),\n    }};\n    Ok(::aws_smithy_types::Blob::from(payload).into_bytes())\n}}"
        )
        .unwrap();
        return Some(output);
    }

    if !matches!(target_kind, "structure" | "union") {
        return None;
    }

    let target_name = rust_type_name(terminal(target));
    let target_module = names::rust_module_name(terminal(target));
    let target_function = names::rust_identifier(terminal(target));
    let root = xml_name(member)
        .or_else(|| target_shape.and_then(xml_name))
        .unwrap_or_else(|| terminal(target).to_owned());
    let unset_payload = if target_kind == "union" {
        "rest_xml_unset_union_payload"
    } else {
        "rest_xml_unset_struct_payload"
    };
    let (namespace_uri, namespace_prefix) = xml_namespace(selected);
    let namespace = match namespace_prefix {
        Some(prefix) => format!(".write_ns({namespace_uri:?}, Some({prefix:?}))"),
        None => format!(".write_ns({namespace_uri:?}, None)"),
    };
    let module = format!("{}_input", names::rust_module_name(operation_name));

    writeln!(
        output,
        "pub fn ser_{field}_http_payload(\n    payload: &::std::option::Option<crate::types::{target_name}>,\n) -> ::std::result::Result<::std::vec::Vec<u8>, ::aws_smithy_types::error::operation::BuildError> {{\n    let payload = match payload.as_ref() {{\n        Some(t) => t,\n        None => return Ok(crate::protocol_serde::{unset_payload}()),\n    }};\n    Ok(crate::protocol_serde::shape_{module}::ser_{field}_payload(\n        payload,\n    )?)\n}}\n\npub fn ser_{field}_payload(\n    input: &crate::types::{target_name},\n) -> std::result::Result<std::vec::Vec<u8>, ::aws_smithy_types::error::operation::SerializationError> {{\n    let mut out = String::new();\n    {{\n        let mut writer = ::aws_smithy_xml::encode::XmlWriter::new(&mut out);\n        #[allow(unused_mut)]\n        let mut root = writer.start_el({root:?}){namespace};\n        crate::protocol_serde::shape_{target_module}::ser_{target_function}(input, root)?\n    }}\n    Ok(out.into_bytes())\n}}",
    )
    .unwrap();
    Some(output)
}

fn render_protocol_operation_input_file(
    selected: &SelectedModel,
    operation_name: &str,
    input_shape: &Value,
) -> Option<String> {
    let document_members = members(input_shape)
        .into_iter()
        .filter(|(_, member)| {
            is_xml_document_member(member) && !has_trait(member, "smithy.api#httpPayload")
        })
        .collect::<Vec<_>>();
    if document_members.is_empty() {
        return None;
    }
    let mut document_members = document_members;
    document_members.sort_by(|left, right| left.0.cmp(&right.0));

    let operation_module = names::rust_module_name(operation_name);
    let input_module = format!("{operation_module}_input");
    let function = format!("ser_{input_module}_input_input");
    let operation_type = rust_type_name(operation_name);
    let mut output = String::new();
    client_operation_header(&mut output);
    writeln!(
        output,
        "pub fn {function}(\n    input: &crate::operation::{operation_module}::{operation_type}Input,\n    writer: ::aws_smithy_xml::encode::ElWriter,\n) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {{"
    )
    .unwrap();
    output.push_str("    #[allow(unused_mut)]\n    let mut scope = writer.finish();\n");
    let mut state = ProtocolRenderState::default();
    for (member_name, member) in document_members {
        protocol_serialize_member(
            &mut output,
            selected,
            &member_name,
            member,
            "scope",
            &format!("input.{}", names::rust_identifier(&member_name)),
            None,
            &mut state,
            true,
        );
    }
    output.push_str("    scope.finish();\n    Ok(())\n}\n");
    Some(output)
}

fn render_protocol_operation_input_serializer(
    selected: &SelectedModel,
    operation_name: &str,
) -> Option<String> {
    let operation = operation_shape(selected, operation_name)?;
    let input_shape = operation
        .get("input")
        .and_then(target_value)
        .and_then(|id| selected.model.shapes.get(id))?;
    let has_document_members = members(input_shape).iter().any(|(_, member)| {
        is_xml_document_member(member) && !has_trait(member, "smithy.api#httpPayload")
    });
    if !has_document_members {
        return None;
    }

    let operation_module = names::rust_module_name(operation_name);
    let operation_type = rust_type_name(operation_name);
    let input_module = format!("{operation_module}_input");
    let function = format!("ser_{operation_module}_op_input");
    let input_function = format!("ser_{input_module}_input_input");
    let (namespace_uri, namespace_prefix) = xml_namespace(selected);
    let namespace = match namespace_prefix {
        Some(prefix) => format!(".write_ns({namespace_uri:?}, Some({prefix:?}))"),
        None => format!(".write_ns({namespace_uri:?}, None)"),
    };
    let root = format!("{operation_name}Request");
    let mut output = String::new();
    writeln!(
        output,
        "\npub fn {function}(\n    input: &crate::operation::{operation_module}::{operation_type}Input,\n) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {{\n    let mut out = String::new();\n    {{\n        let mut writer = ::aws_smithy_xml::encode::XmlWriter::new(&mut out);\n        #[allow(unused_mut)]\n        let mut root = writer.start_el({root:?}){namespace};\n        crate::protocol_serde::shape_{input_module}::{input_function}(input, root)?\n    }}\n    Ok(::aws_smithy_types::body::SdkBody::from(out))\n}}"
    )
    .unwrap();
    Some(output)
}

fn protocol_operation_output_document_members<'a>(
    operation: &'a Value,
    selected: &'a SelectedModel,
) -> Option<Vec<(String, &'a Value)>> {
    let output_shape = operation
        .get("output")
        .and_then(target_value)
        .and_then(|id| selected.model.shapes.get(id))?;
    let mut all_members = members(output_shape);
    let capacity = java_hash_map_capacity(all_members.len());
    all_members.sort_by_key(|(name, _)| java_string_hash(name) & (capacity as u32 - 1));
    let members = all_members
        .into_iter()
        .filter(|(_, member)| {
            is_xml_document_member(member) && !has_trait(member, "smithy.api#httpPayload")
        })
        .collect::<Vec<_>>();
    (!members.is_empty()).then_some(members)
}

fn java_hash_map_capacity(length: usize) -> usize {
    let mut capacity = 16usize;
    let mut threshold = capacity * 3 / 4;
    while length > threshold {
        capacity *= 2;
        threshold = capacity * 3 / 4;
    }
    capacity
}

fn java_string_hash(value: &str) -> u32 {
    let hash = value.encode_utf16().fold(0u32, |hash, code_unit| {
        hash.wrapping_mul(31).wrapping_add(u32::from(code_unit))
    });
    hash ^ (hash >> 16)
}

fn protocol_operation_has_document_output(selected: &SelectedModel, operation_name: &str) -> bool {
    operation_shape(selected, operation_name)
        .and_then(|operation| protocol_operation_output_document_members(operation, selected))
        .is_some()
}

fn protocol_state_after_source(source: &str) -> ProtocolRenderState {
    let mut state = ProtocolRenderState::default();
    let prefixes = [
        "var_",
        "attrib_",
        "inner_",
        "formatted_",
        "list_",
        "list_item_",
        "key_",
        "value_",
    ];
    for token in
        source.split(|character: char| !(character.is_ascii_alphanumeric() || character == '_'))
    {
        for prefix in prefixes {
            if let Some(number) = token
                .strip_prefix(prefix)
                .and_then(|value| value.parse::<usize>().ok())
            {
                state.next_name = state.next_name.max(number);
            }
        }
    }
    state
}

fn render_protocol_operation_output_parser(
    selected: &SelectedModel,
    operation_name: &str,
    prefix_source: &str,
) -> Option<String> {
    let operation = operation_shape(selected, operation_name)?;
    let output_shape_id = operation.get("output").and_then(target_value)?;
    let output_shape = selected.model.shapes.get(output_shape_id)?;
    let document_members = protocol_operation_output_document_members(operation, selected)?;
    let module = names::rust_module_name(operation_name);
    let operation_type = rust_type_name(operation_name);
    let builder_path =
        format!("crate::operation::{module}::builders::{operation_type}OutputBuilder");
    let parser_result =
        format!("crate::operation::{module}::builders::{operation_type}OutputBuilder");
    let mut output = String::new();
    output.push('\n');
    writeln!(
        output,
        "#[allow(unused_mut)]\npub fn de_{module}(\n    inp: &[u8],\n    mut builder: {builder_path},\n) -> std::result::Result<{parser_result}, ::aws_smithy_xml::decode::XmlDecodeError> {{\n    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;\n\n    #[allow(unused_mut)]\n    let mut decoder = doc.root_element()?;\n    #[allow(unused_variables)]\n    let start_el = decoder.start_el();\n    #[allow(unused_variables)]\n    let depth = 0u32;"
    )
    .unwrap();

    let mut state = protocol_state_after_source(prefix_source);
    let synthetic_namespace = output_shape_id
        .split('#')
        .next()
        .map(|namespace| {
            if namespace.ends_with(".synthetic") {
                namespace.to_owned()
            } else {
                format!("{namespace}.synthetic")
            }
        })
        .unwrap_or_else(|| "synthetic".to_owned());
    let synthetic_shape_id = format!("{synthetic_namespace}#{operation_type}Output");

    if has_trait(operation, "aws.customizations#s3UnwrappedXmlOutput")
        && document_members.len() == 1
    {
        let (member_name, member) = &document_members[0];
        let target = member_target(member).unwrap_or_default();
        let field = names::rust_identifier(member_name);
        let xml_name = protocol_member_xml_name(selected, member_name, member);
        let var = state.temp();
        let parse = indent_expression(
            &protocol_parse_expression(selected, target, "decoder", "depth"),
            20,
        );
        writeln!(
            output,
            "    match start_el {{\n        s if s.matches({xml_name:?}) /* {xml_name} {synthetic_shape_id}${member_name} */ =>  {{\n            let {var} =\n                Some(\n                    {parse}\n                    ?\n                )\n            ;\n            builder = builder.set_{field}({var});\n        }}\n        ,\n        _ => return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(\"expected {xml_name} tag\"))\n    }}"
        )
        .unwrap();
    } else {
        let allow_invalid_root = has_trait(output_shape, "smithy.api.internal#allowInvalidXmlRoot");
        if let Some(root) =
            protocol_operation_output_xml_name(selected, output_shape_id, output_shape)
                .filter(|_| !allow_invalid_root)
        {
            writeln!(
                output,
                "    if !start_el.matches({root:?}) {{\n        return Err(\n            ::aws_smithy_xml::decode::XmlDecodeError::custom(\n                format!(\"encountered invalid XML root: expected {root} but got {{start_el:?}}. This is likely a bug in the SDK.\")\n            )\n        );\n    }}"
            )
            .unwrap();
        }
        output.push_str(
            "    while let Some(mut tag) = decoder.next_tag() {\n        match tag.start_el() {\n",
        );
        for (member_name, member) in document_members {
            let target = member_target(member).unwrap_or_default();
            let field = names::rust_identifier(&member_name);
            let xml_name = protocol_member_xml_name(selected, &member_name, member);
            let member_id = format!("{synthetic_shape_id}${member_name}");
            let outer = state.temp();
            let kind = protocol_shape_kind(selected, target);
            let parse = if kind == "list" && has_trait(member, "smithy.api#xmlFlattened") {
                let list = state.list_accum();
                let element = selected
                    .model
                    .shapes
                    .get(target)
                    .and_then(|shape| shape.get("member"))
                    .expect("flattened list member");
                let element_target = member_target(element).unwrap_or_default();
                let element_expr = indent_expression(
                    &protocol_parse_expression(selected, element_target, "tag", "depth"),
                    8,
                );
                let list_type = format!(
                    "::std::vec::Vec::<{}>",
                    protocol_shape_type(selected, element_target)
                );
                format!(
                    "Result::<{list_type}, ::aws_smithy_xml::decode::XmlDecodeError>::Ok({{\n    let mut {list} = builder.{field}.take().unwrap_or_default();\n    {list}.push(\n        {element_expr}\n        ?\n    );\n    {list}\n}})"
                )
            } else {
                protocol_parse_expression(selected, target, "tag", "depth")
            };
            let parse = indent_expression(&parse, 24);
            writeln!(
                output,
                "            s if s.matches({xml_name:?}) /* {member_name} {member_id} */ =>  {{\n                let {outer} =\n                    Some(\n                        {parse}\n                        ?\n                    )\n                ;\n                builder = builder.set_{field}({outer});\n            }}\n            ,"
            )
            .unwrap();
        }
        output.push_str("            _ => {}\n        }\n    }\n");
    }
    output.push_str("    Ok(builder)\n}\n");
    Some(output)
}

#[derive(Clone, Copy, Default)]
struct ProtocolSerdeRoles {
    serialize: bool,
    deserialize: bool,
    first: Option<ProtocolSerdeRole>,
}

#[derive(Clone, Copy)]
enum ProtocolSerdeRole {
    Serialize,
    Deserialize,
}

fn record_protocol_role(
    roles: &mut BTreeMap<String, ProtocolSerdeRoles>,
    shape_id: &str,
    role: ProtocolSerdeRole,
) {
    let entry = roles.entry(shape_id.to_owned()).or_default();
    if entry.first.is_none() {
        entry.first = Some(role);
    }
    match role {
        ProtocolSerdeRole::Serialize => entry.serialize = true,
        ProtocolSerdeRole::Deserialize => entry.deserialize = true,
    }
}

/// Render the protocol module and the modeled-shape XML helpers used by it.
///
/// Smithy creates protocol functions lazily while walking document bindings.
/// Keeping the same reachability walk here is important: emitting a helper for
/// every modeled shape would create a materially different SDK, especially for
/// services with large models and many shapes that never occur in XML bodies.
fn render_protocol_serde_files(selected: &SelectedModel) -> (String, Vec<(String, String)>) {
    let roles = protocol_serde_roles(selected);
    let mut files = Vec::new();
    let mut module_names = BTreeSet::new();

    for operation_name in &selected.operations {
        let module = names::rust_module_name(operation_name);
        module_names.insert(module.clone());
        if render_protocol_input_file(selected, operation_name).is_some() {
            module_names.insert(format!("{module}_input"));
        }
        if protocol_output_has_headers(selected, operation_name) {
            module_names.insert(format!("{module}_output"));
        }
        if render_protocol_output_payload_file(selected, operation_name).is_some() {
            module_names.insert(format!("{module}_output"));
        }
    }

    for (shape_id, role) in &roles {
        let module = names::rust_module_name(terminal(shape_id));
        module_names.insert(module.clone());
        files.push((
            format!("src/protocol_serde/shape_{module}.rs"),
            render_protocol_shape_file(selected, shape_id, *role),
        ));
    }

    for error_id in error_shape_ids(selected) {
        let module = names::rust_module_name(terminal(&error_id));
        if module_names.insert(module.clone()) {
            files.push((
                format!("src/protocol_serde/shape_{module}.rs"),
                render_protocol_error_file(selected, &error_id),
            ));
        }
    }
    files.sort_by(|left, right| left.0.cmp(&right.0));

    let mut module = String::new();
    client_operation_header(&mut module);
    module.push_str(
        "pub(crate) fn type_erase_result<O, E>(\n    result: ::std::result::Result<O, E>,\n) -> ::std::result::Result<\n    ::aws_smithy_runtime_api::client::interceptors::context::Output,\n    ::aws_smithy_runtime_api::client::orchestrator::OrchestratorError<::aws_smithy_runtime_api::client::interceptors::context::Error>,\n>\nwhere\n    O: ::std::fmt::Debug + ::std::marker::Send + ::std::marker::Sync + 'static,\n    E: ::std::error::Error + ::std::fmt::Debug + ::std::marker::Send + ::std::marker::Sync + 'static,\n{\n    result\n        .map(|output| ::aws_smithy_runtime_api::client::interceptors::context::Output::erase(output))\n        .map_err(|error| ::aws_smithy_runtime_api::client::interceptors::context::Error::erase(error))\n        .map_err(::std::convert::Into::into)\n}\n\n",
    );
    module.push_str(
        "pub fn rest_xml_unset_struct_payload() -> ::std::vec::Vec<u8> {\n    Vec::new()\n}\n\npub fn rest_xml_unset_union_payload() -> ::std::vec::Vec<u8> {\n    Vec::new()\n}\n\n",
    );
    module.push_str(
        "pub fn parse_http_error_metadata(\n    _response_status: u16,\n    _response_headers: &::aws_smithy_runtime_api::http::Headers,\n    response_body: &[u8],\n) -> ::std::result::Result<::aws_smithy_types::error::metadata::Builder, ::aws_smithy_xml::decode::XmlDecodeError> {\n    if response_body.is_empty() {\n        Ok(::aws_smithy_types::error::ErrorMetadata::builder())\n    } else {\n        crate::rest_xml_unwrapped_errors::parse_error_metadata(response_body)\n    }\n}\n\n",
    );
    for name in module_names {
        writeln!(module, "pub(crate) mod shape_{name};").unwrap();
        module.push('\n');
    }
    (module, files)
}

fn protocol_serde_roles(selected: &SelectedModel) -> BTreeMap<String, ProtocolSerdeRoles> {
    let mut roles = BTreeMap::<String, ProtocolSerdeRoles>::new();

    for operation_name in &selected.operations {
        let Some(operation) = operation_shape(selected, operation_name) else {
            continue;
        };
        // Smithy generates operation input serializers before operation output
        // parsers. A shape can be reached through both roles, and the first
        // role determines which helper appears first in its shared file.
        if let Some(input) = operation
            .get("input")
            .and_then(target_value)
            .and_then(|id| selected.model.shapes.get(id))
        {
            for (_, member) in members(input) {
                let Some(target) = member_target(member) else {
                    continue;
                };
                if is_xml_document_member(member) {
                    protocol_mark_serializer(selected, target, &mut roles, &mut BTreeSet::new());
                }
            }
        }
        if let Some(output) = operation
            .get("output")
            .and_then(target_value)
            .and_then(|id| selected.model.shapes.get(id))
        {
            for (_, member) in members(output) {
                let Some(target) = member_target(member) else {
                    continue;
                };
                if is_xml_body_member(member) {
                    protocol_mark_deserializer_member(
                        selected,
                        target,
                        member,
                        &mut roles,
                        &mut BTreeSet::new(),
                    );
                }
            }
        }
        if let Some(errors) = operation.get("errors").and_then(Value::as_array) {
            for error in errors.iter().filter_map(target_value) {
                let Some(shape) = selected.model.shapes.get(error) else {
                    continue;
                };
                for (_, member) in members(shape) {
                    if let Some(target) = member_target(member) {
                        protocol_mark_deserializer(
                            selected,
                            target,
                            &mut roles,
                            &mut BTreeSet::new(),
                        );
                    }
                }
            }
        }
    }
    roles.retain(|shape_id, role| {
        selected.model.shapes.contains_key(shape_id)
            && (role.serialize || role.deserialize)
            && matches!(
                selected
                    .model
                    .shapes
                    .get(shape_id)
                    .and_then(|shape| shape.get("type"))
                    .and_then(Value::as_str),
                Some("structure" | "union" | "list" | "map")
            )
    });
    roles
}

fn protocol_mark_serializer(
    selected: &SelectedModel,
    shape_id: &str,
    roles: &mut BTreeMap<String, ProtocolSerdeRoles>,
    seen: &mut BTreeSet<String>,
) {
    if !seen.insert(shape_id.to_owned()) {
        return;
    }
    let Some(shape) = selected.model.shapes.get(shape_id) else {
        return;
    };
    if shape_is_streaming(shape) {
        protocol_walk_members(selected, shape, roles, seen, true);
        return;
    }
    if let Some((_, member)) = event_payload_member(shape) {
        let target = member_target(member).unwrap_or_default();
        if !matches!(protocol_shape_kind(selected, target), "structure" | "union") {
            return;
        }
    }
    if shape.get("type").and_then(Value::as_str) == Some("structure") && members(shape).is_empty() {
        return;
    }
    if let Some("structure" | "union") = shape.get("type").and_then(Value::as_str)
        && (shape.get("type").and_then(Value::as_str) != Some("structure")
            || !members(shape).is_empty())
    {
        record_protocol_role(roles, shape_id, ProtocolSerdeRole::Serialize);
    }
    protocol_walk_members(selected, shape, roles, seen, true);
}

fn protocol_mark_deserializer(
    selected: &SelectedModel,
    shape_id: &str,
    roles: &mut BTreeMap<String, ProtocolSerdeRoles>,
    seen: &mut BTreeSet<String>,
) {
    if !seen.insert(shape_id.to_owned()) {
        return;
    }
    let Some(shape) = selected.model.shapes.get(shape_id) else {
        return;
    };
    if shape_is_streaming(shape) {
        protocol_walk_streaming_members(selected, shape, roles, seen);
        return;
    }
    if let Some((_, member)) = event_payload_member(shape) {
        let target = member_target(member).unwrap_or_default();
        if !matches!(protocol_shape_kind(selected, target), "structure" | "union") {
            return;
        }
    }
    if matches!(
        shape.get("type").and_then(Value::as_str),
        Some("structure" | "union" | "list" | "map")
    ) {
        record_protocol_role(roles, shape_id, ProtocolSerdeRole::Deserialize);
    }
    protocol_walk_members(selected, shape, roles, seen, false);
}

fn protocol_walk_streaming_members(
    selected: &SelectedModel,
    shape: &Value,
    roles: &mut BTreeMap<String, ProtocolSerdeRoles>,
    seen: &mut BTreeSet<String>,
) {
    for (_, member) in members(shape) {
        let Some(target) = member_target(member) else {
            continue;
        };
        if selected.model.shapes.get(target).is_some_and(|shape| {
            shape.get("type").and_then(Value::as_str) == Some("structure")
                && members(shape).is_empty()
        }) {
            continue;
        }
        protocol_mark_deserializer_member(selected, target, member, roles, seen);
    }
}

fn protocol_mark_deserializer_member(
    selected: &SelectedModel,
    shape_id: &str,
    member: &Value,
    roles: &mut BTreeMap<String, ProtocolSerdeRoles>,
    seen: &mut BTreeSet<String>,
) {
    if protocol_shape_kind(selected, shape_id) == "list"
        && has_trait(member, "smithy.api#xmlFlattened")
    {
        if let Some(list_shape) = selected.model.shapes.get(shape_id)
            && let Some(element) = list_shape.get("member").and_then(member_target)
        {
            protocol_mark_deserializer(selected, element, roles, seen);
        }
    } else {
        protocol_mark_deserializer(selected, shape_id, roles, seen);
    }
}

fn protocol_walk_members(
    selected: &SelectedModel,
    shape: &Value,
    roles: &mut BTreeMap<String, ProtocolSerdeRoles>,
    seen: &mut BTreeSet<String>,
    serializer: bool,
) {
    match shape.get("type").and_then(Value::as_str) {
        Some("structure" | "union") => {
            for (_, member) in members(shape) {
                let Some(target) = member_target(member) else {
                    continue;
                };
                if serializer {
                    protocol_mark_serializer(selected, target, roles, seen);
                } else {
                    protocol_mark_deserializer_member(selected, target, member, roles, seen);
                }
            }
        }
        Some("list") => {
            if let Some(member) = shape.get("member")
                && let Some(target) = member_target(member)
            {
                if serializer {
                    protocol_mark_serializer(selected, target, roles, seen);
                } else {
                    protocol_mark_deserializer(selected, target, roles, seen);
                }
            }
        }
        Some("map") => {
            for key in [shape.get("key"), shape.get("value")].into_iter().flatten() {
                if let Some(target) = member_target(key) {
                    if serializer {
                        protocol_mark_serializer(selected, target, roles, seen);
                    } else {
                        protocol_mark_deserializer(selected, target, roles, seen);
                    }
                }
            }
        }
        _ => {}
    }
}

fn render_protocol_shape_file(
    selected: &SelectedModel,
    shape_id: &str,
    roles: ProtocolSerdeRoles,
) -> String {
    let shape = selected
        .model
        .shapes
        .get(shape_id)
        .expect("protocol shape exists");
    let mut output = String::new();
    client_operation_header(&mut output);
    let kind = shape
        .get("type")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let mut state = ProtocolRenderState::default();
    let render_deserializer = |output: &mut String, state: &mut ProtocolRenderState| match kind {
        "structure" if event_payload_member(shape).is_some() => {
            render_protocol_event_payload_deserializer(output, selected, shape)
        }
        "structure" => {
            render_protocol_structure_deserializer(output, selected, shape_id, shape, state)
        }
        "union" => render_protocol_union_deserializer(output, selected, shape_id),
        "list" => render_protocol_list_deserializer(output, selected, shape_id, shape),
        "map" => render_protocol_map_deserializer(output, selected, shape_id, shape),
        _ => {}
    };
    let render_serializer = |output: &mut String, state: &mut ProtocolRenderState| match kind {
        "structure" => {
            render_protocol_structure_serializer(output, selected, shape_id, shape, state)
        }
        "union" => render_protocol_union_serializer(output, selected, shape_id, shape),
        _ => {}
    };
    match (roles.first, roles.deserialize, roles.serialize) {
        (Some(ProtocolSerdeRole::Serialize), true, true) => {
            render_serializer(&mut output, &mut state);
            render_deserializer(&mut output, &mut state);
        }
        (_, true, true) => {
            render_deserializer(&mut output, &mut state);
            render_serializer(&mut output, &mut state);
        }
        (_, true, false) => render_deserializer(&mut output, &mut state),
        (_, false, true) => render_serializer(&mut output, &mut state),
        (_, false, false) => {}
    }
    output
}

fn event_payload_member(shape: &Value) -> Option<(String, &Value)> {
    members(shape)
        .into_iter()
        .find(|(_, member)| has_trait(member, "smithy.api#eventPayload"))
}

fn render_protocol_event_payload_deserializer(
    output: &mut String,
    selected: &SelectedModel,
    shape: &Value,
) {
    let (member_name, member) = event_payload_member(shape).expect("event payload member");
    let target = member_target(member).expect("event payload target");
    let field = names::rust_identifier(&member_name);
    let target_type = protocol_shape_type(selected, target);
    let root = selected
        .model
        .shapes
        .get(target)
        .and_then(xml_name)
        .unwrap_or_else(|| terminal(target).to_owned());
    let target_module = names::rust_module_name(terminal(target));
    writeln!(
        output,
        "pub fn de_{field}(inp: &[u8]) -> std::result::Result<{target_type}, ::aws_smithy_xml::decode::XmlDecodeError> {{\n    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;\n    #[allow(unused_mut)]\n    let mut decoder = doc.root_element()?;\n    let start_el = decoder.start_el();\n    if !(start_el.matches({root:?})) {{\n        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(\"invalid root, expected {root} got {{start_el:?}}\")));\n    }}\n    #[allow(unused_variables)]\n    let depth = 0u32;\n    crate::protocol_serde::shape_{target_module}::de_{target_module}(&mut decoder, depth + 1)\n}}"
    )
    .unwrap();
}

#[derive(Default)]
struct ProtocolRenderState {
    next_name: usize,
}

impl ProtocolRenderState {
    fn temp(&mut self) -> String {
        self.next_name += 1;
        format!("var_{}", self.next_name)
    }

    fn list_item(&mut self) -> String {
        self.next_name += 1;
        format!("list_item_{}", self.next_name)
    }

    fn key(&mut self) -> String {
        self.next_name += 1;
        format!("key_{}", self.next_name)
    }

    fn value(&mut self) -> String {
        self.next_name += 1;
        format!("value_{}", self.next_name)
    }

    fn list_accum(&mut self) -> String {
        self.next_name += 1;
        format!("list_{}", self.next_name)
    }
}

fn protocol_shape_kind<'a>(selected: &'a SelectedModel, target: &'a str) -> &'a str {
    selected
        .model
        .shapes
        .get(target)
        .and_then(|shape| shape.get("type"))
        .and_then(Value::as_str)
        .or_else(|| target.strip_prefix("smithy.api#"))
        .unwrap_or("string")
}

fn protocol_shape_type(selected: &SelectedModel, target: &str) -> String {
    type_expr(
        selected,
        target,
        Context::Types {
            consumer_namespace: false,
        },
    )
}

fn protocol_member_xml_name(selected: &SelectedModel, member_name: &str, member: &Value) -> String {
    xml_name(member)
        .or_else(|| {
            member_target(member)
                .and_then(|target| selected.model.shapes.get(target))
                .and_then(xml_name)
        })
        .unwrap_or_else(|| member_name.to_owned())
}

fn protocol_member_namespace(member: &Value) -> String {
    let Some(namespace) = member
        .get("traits")
        .and_then(Value::as_object)
        .and_then(|traits| traits.get("smithy.api#xmlNamespace"))
        .and_then(Value::as_object)
    else {
        return String::new();
    };
    let Some(uri) = namespace.get("uri").and_then(Value::as_str) else {
        return String::new();
    };
    let prefix = namespace
        .get("prefix")
        .and_then(Value::as_str)
        .map(|prefix| format!("Some({prefix:?})"))
        .unwrap_or_else(|| "None".to_owned());
    format!(".write_ns({uri:?}, {prefix})")
}

fn protocol_member_is_optional(selected: &SelectedModel, member: &Value) -> bool {
    let target = member_target(member).unwrap_or_default();
    !member_is_effectively_required(selected, member, target)
}

fn indent_expression(expression: &str, indentation: usize) -> String {
    let prefix = " ".repeat(indentation);
    expression
        .lines()
        .enumerate()
        .map(|(index, line)| {
            if index == 0 {
                line.to_owned()
            } else {
                format!("{prefix}{line}")
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn shape_is_streaming(shape: &Value) -> bool {
    has_trait(shape, "smithy.api#streaming")
}

fn is_xml_document_member(member: &Value) -> bool {
    let Some(traits) = member.get("traits").and_then(Value::as_object) else {
        return true;
    };
    !traits.keys().any(|trait_id| {
        matches!(
            trait_id.as_str(),
            "smithy.api#httpHeader"
                | "smithy.api#httpPrefixHeaders"
                | "smithy.api#httpLabel"
                | "smithy.api#httpQuery"
                | "smithy.api#httpQueryParams"
                | "smithy.api#httpResponseCode"
        )
    })
}

fn protocol_member_is_attribute(member: &Value) -> bool {
    has_trait(member, "smithy.api#xmlAttribute")
}

fn protocol_primitive_encode(selected: &SelectedModel, target: &str, expression: &str) -> String {
    let expression_without_reference = expression.strip_prefix('&').unwrap_or(expression);
    match protocol_shape_kind(selected, target) {
        "string" | "enum" => format!("{expression_without_reference}.as_str()"),
        "boolean" | "integer" | "long" | "short" | "byte" | "float" | "double" => {
            let value = if expression.starts_with('&') {
                expression_without_reference.to_owned()
            } else {
                format!("*{expression}")
            };
            format!("::aws_smithy_types::primitive::Encoder::from({value}).encode()")
        }
        "timestamp" => format!(
            "{expression_without_reference}.fmt(::aws_smithy_types::date_time::Format::{})?.as_ref()",
            protocol_timestamp_format(selected, target)
        ),
        _ => format!("{expression_without_reference}.to_string()"),
    }
}

fn protocol_timestamp_format(selected: &SelectedModel, target: &str) -> &'static str {
    match selected
        .model
        .shapes
        .get(target)
        .and_then(|shape| shape.get("traits"))
        .and_then(|traits| traits.get("smithy.api#timestampFormat"))
        .and_then(Value::as_str)
    {
        Some("date-time") => "DateTimeWithOffset",
        Some("epoch-seconds") => "EpochSeconds",
        Some("http-date") => "HttpDate",
        _ => "DateTimeWithOffset",
    }
}

#[allow(clippy::too_many_arguments)]
fn protocol_serialize_member(
    output: &mut String,
    selected: &SelectedModel,
    member_name: &str,
    member: &Value,
    scope: &str,
    expression: &str,
    root_name_override: Option<&str>,
    state: &mut ProtocolRenderState,
    force_optional: bool,
) {
    let Some(target) = member_target(member) else {
        return;
    };
    let kind = protocol_shape_kind(selected, target);
    let xml_name = root_name_override
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| protocol_member_xml_name(selected, member_name, member));
    let namespace = protocol_member_namespace(member);
    let optional = force_optional || protocol_member_is_optional(selected, member);
    let mut body = String::new();
    let input = if optional {
        let temp = state.temp();
        let temp = if kind == "structure"
            && selected
                .model
                .shapes
                .get(target)
                .is_some_and(|shape| members(shape).is_empty())
        {
            format!("_{temp}")
        } else {
            temp
        };
        let optional_expression = if expression.starts_with('&') {
            expression.to_owned()
        } else {
            format!("&{expression}")
        };
        writeln!(body, "if let Some({temp}) = {optional_expression} {{").unwrap();
        temp
    } else {
        expression.to_owned()
    };
    match kind {
        "string" | "enum" | "boolean" | "integer" | "long" | "short" | "byte" | "float"
        | "double" | "timestamp" => {
            writeln!(
                body,
                "    let mut inner_writer = {scope}.start_el({xml_name:?}){namespace}.finish();"
            )
            .unwrap();
            writeln!(
                body,
                "    inner_writer.data({});",
                protocol_primitive_encode(selected, target, &input)
            )
            .unwrap();
        }
        "list" => {
            let item = selected
                .model
                .shapes
                .get(target)
                .and_then(|shape| shape.get("member"))
                .expect("list member");
            let list_item = state.list_item();
            let (item_scope, item_name, item_indent) =
                if has_trait(member, "smithy.api#xmlFlattened") {
                    (scope.to_owned(), Some(xml_name.as_str()), "            ")
                } else {
                    writeln!(
                    body,
                    "    let mut inner_writer = {scope}.start_el({xml_name:?}){namespace}.finish();"
                )
                .unwrap();
                    ("inner_writer".to_owned(), None, "            ")
                };
            writeln!(body, "    for {list_item} in {input} {{").unwrap();
            body.push_str("        {\n");
            protocol_serialize_list_member(
                &mut body,
                selected,
                "member",
                item,
                &item_scope,
                &list_item,
                item_name,
                state,
                item_indent,
            );
            body.push_str("        }\n    }\n");
        }
        "map" => {
            let key = state.key();
            let value = state.value();
            let iter_expression = input.clone();
            writeln!(body, "    for ({key}, {value}) in {iter_expression} {{").unwrap();
            writeln!(
                body,
                "        let mut entry = {scope}.start_el(\"entry\").finish();"
            )
            .unwrap();
            let shape = selected.model.shapes.get(target).expect("map");
            let key_member = shape.get("key").expect("map key");
            let value_member = shape.get("value").expect("map value");
            protocol_serialize_list_member(
                &mut body, selected, "key", key_member, "entry", &key, None, state, "        ",
            );
            protocol_serialize_list_member(
                &mut body,
                selected,
                "value",
                value_member,
                "entry",
                &value,
                None,
                state,
                "        ",
            );
            body.push_str("    }\n");
        }
        "structure" => {
            if let Some(shape) = selected.model.shapes.get(target) {
                if members(shape).is_empty() {
                    writeln!(
                        body,
                        "    {scope}.start_el({xml_name:?}){namespace}.finish();"
                    )
                    .unwrap();
                } else {
                    writeln!(
                        body,
                        "    let inner_writer = {scope}.start_el({xml_name:?}){namespace};"
                    )
                    .unwrap();
                    writeln!(
                        body,
                        "    crate::protocol_serde::shape_{}::ser_{}({}, inner_writer)?;",
                        names::rust_module_name(terminal(target)),
                        names::rust_module_name(terminal(target)),
                        input.clone()
                    )
                    .unwrap();
                    body.pop();
                    body.pop();
                    body.push('\n');
                }
            }
        }
        "union" => {
            writeln!(
                body,
                "    let inner_writer = {scope}.start_el({xml_name:?}){namespace};"
            )
            .unwrap();
            writeln!(
                body,
                "    crate::protocol_serde::shape_{}::ser_{}({}, inner_writer)?;",
                names::rust_module_name(terminal(target)),
                names::rust_module_name(terminal(target)),
                input.clone()
            )
            .unwrap();
            body.pop();
            body.pop();
            body.push('\n');
        }
        _ => {}
    }
    if !optional {
        body.insert(0, '{');
        body.insert(1, '\n');
        body.push('}');
        body.push('\n');
    }
    if optional {
        body.push_str("}\n");
    }
    output.push_str(&body);
}

#[allow(clippy::too_many_arguments)]
fn protocol_serialize_list_member(
    output: &mut String,
    selected: &SelectedModel,
    member_name: &str,
    member: &Value,
    scope: &str,
    expression: &str,
    root_name_override: Option<&str>,
    state: &mut ProtocolRenderState,
    indent: &str,
) {
    let Some(target) = member_target(member) else {
        return;
    };
    let kind = protocol_shape_kind(selected, target);
    let xml_name = root_name_override
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| protocol_member_xml_name(selected, member_name, member));
    let namespace = protocol_member_namespace(member);
    match kind {
        "string" | "enum" | "boolean" | "integer" | "long" | "short" | "byte" | "float"
        | "double" | "timestamp" => {
            writeln!(
                output,
                "{indent}let mut inner_writer = {scope}.start_el({xml_name:?}){namespace}.finish();"
            )
            .unwrap();
            writeln!(
                output,
                "{indent}inner_writer.data({});",
                protocol_primitive_encode(selected, target, expression)
            )
            .unwrap();
        }
        "structure" => {
            let shape = selected.model.shapes.get(target).expect("structure exists");
            if members(shape).is_empty() {
                writeln!(
                    output,
                    "{indent}{scope}.start_el({xml_name:?}){namespace}.finish();"
                )
                .unwrap();
            } else {
                writeln!(
                    output,
                    "{indent}let inner_writer = {scope}.start_el({xml_name:?}){namespace};"
                )
                .unwrap();
                writeln!(
                    output,
                    "{indent}crate::protocol_serde::shape_{}::ser_{}({}, inner_writer)?",
                    names::rust_module_name(terminal(target)),
                    names::rust_module_name(terminal(target)),
                    expression
                )
                .unwrap();
            }
        }
        "union" => {
            writeln!(
                output,
                "{indent}let inner_writer = {scope}.start_el({xml_name:?}){namespace};"
            )
            .unwrap();
            writeln!(
                output,
                "{indent}crate::protocol_serde::shape_{}::ser_{}({}, inner_writer)?",
                names::rust_module_name(terminal(target)),
                names::rust_module_name(terminal(target)),
                expression
            )
            .unwrap();
        }
        "list" => {
            let nested = selected.model.shapes.get(target).expect("list");
            let item = state.list_item();
            writeln!(output, "{indent}for {item} in {expression} {{").unwrap();
            let member = nested.get("member").expect("list member");
            writeln!(output, "{indent}    {{").unwrap();
            protocol_serialize_list_member(
                output,
                selected,
                "member",
                member,
                scope,
                &item,
                Some(&xml_name),
                state,
                &format!("{indent}        "),
            );
            writeln!(output, "{indent}    }}").unwrap();
            writeln!(output, "{indent}}}").unwrap();
        }
        "map" => {}
        _ => {}
    }
}

fn render_protocol_structure_inner(
    output: &mut String,
    selected: &SelectedModel,
    shape: &Value,
    scope: &str,
    input: &str,
    state: &mut ProtocolRenderState,
) {
    for (member_name, member) in members(shape) {
        if protocol_member_is_attribute(member) {
            continue;
        }
        protocol_serialize_member(
            output,
            selected,
            &member_name,
            member,
            scope,
            &format!("{input}.{}", names::rust_identifier(&member_name)),
            None,
            state,
            false,
        );
    }
    writeln!(output, "    {scope}.finish();").unwrap();
}

fn render_protocol_structure_serializer(
    output: &mut String,
    selected: &SelectedModel,
    shape_id: &str,
    shape: &Value,
    state: &mut ProtocolRenderState,
) {
    let name = rust_type_name(terminal(shape_id));
    writeln!(
        output,
        "pub fn ser_{}(\n    input: &crate::types::{name},\n    writer: ::aws_smithy_xml::encode::ElWriter,\n) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {{",
        names::rust_module_name(terminal(shape_id))
    )
    .unwrap();
    let attrs = members(shape)
        .iter()
        .any(|(_, member)| protocol_member_is_attribute(member));
    if attrs {
        output.push_str("    let mut writer = writer;\n");
        for (member_name, member) in members(shape) {
            if !protocol_member_is_attribute(member) {
                continue;
            }
            let target = member_target(member).unwrap_or_default();
            let field = names::rust_identifier(&member_name);
            let xml_name = protocol_member_xml_name(selected, &member_name, member);
            if protocol_member_is_optional(selected, member) {
                let temp = state.temp();
                writeln!(output, "    if let Some({temp}) = &input.{field} {{").unwrap();
                writeln!(
                    output,
                    "        writer.write_attribute({xml_name:?}, {});",
                    protocol_primitive_encode(selected, target, &temp)
                )
                .unwrap();
                output.push_str("    }\n");
            } else {
                writeln!(
                    output,
                    "    {{\n        writer.write_attribute({xml_name:?}, {});\n    }}",
                    protocol_primitive_encode(selected, target, &format!("input.{field}"))
                )
                .unwrap();
            }
        }
    }
    output.push_str("    #[allow(unused_mut)]\n    let mut scope = writer.finish();\n");
    render_protocol_structure_inner(output, selected, shape, "scope", "&input", state);
    output.push_str("    Ok(())\n}\n\n");
}

fn render_protocol_union_serializer(
    output: &mut String,
    selected: &SelectedModel,
    shape_id: &str,
    shape: &Value,
) {
    let name = rust_type_name(terminal(shape_id));
    writeln!(
        output,
        "pub fn ser_{}(\n    input: &crate::types::{name},\n    writer: ::aws_smithy_xml::encode::ElWriter,\n) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {{\n    let mut scope_writer = writer.finish();\n    match input {{",
        names::rust_module_name(terminal(shape_id))
    )
    .unwrap();
    for (member_name, member) in members(shape) {
        let variant = rust_type_name(&member_name);
        let target = member_target(member).unwrap_or_default();
        let target_kind = protocol_shape_kind(selected, target);
        if target_kind == "structure"
            && selected
                .model
                .shapes
                .get(target)
                .is_some_and(|shape| members(shape).is_empty())
        {
            writeln!(
                output,
                "        crate::types::{name}::{variant}(_inner) => {{"
            )
            .unwrap();
            writeln!(
                output,
                "            scope_writer.start_el({:?}).finish();",
                protocol_member_xml_name(selected, &member_name, member)
            )
            .unwrap();
        } else if target == "smithy.api#Unit" {
            writeln!(output, "        crate::types::{name}::{variant} => {{").unwrap();
            writeln!(
                output,
                "            scope_writer.start_el({:?}).finish();",
                protocol_member_xml_name(selected, &member_name, member)
            )
            .unwrap();
        } else {
            writeln!(
                output,
                "        crate::types::{name}::{variant}(inner) => {{"
            )
            .unwrap();
            protocol_serialize_list_member(
                output,
                selected,
                &member_name,
                member,
                "scope_writer",
                "inner",
                None,
                &mut ProtocolRenderState::default(),
                "            ",
            );
        }
        output.push_str("        }\n");
    }
    writeln!(
        output,
        "        crate::types::{name}::Unknown => return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant({name:?})),"
    )
    .unwrap();
    output.push_str("    }\n    Ok(())\n}\n\n");
}

fn protocol_parse_primitive(selected: &SelectedModel, target: &str, tag: &str) -> String {
    protocol_parse_primitive_data(
        selected,
        target,
        &format!("::aws_smithy_xml::decode::try_data(&mut {tag})?.as_ref()"),
    )
}

fn protocol_parse_primitive_data(selected: &SelectedModel, target: &str, data: &str) -> String {
    let kind = protocol_shape_kind(selected, target);
    let ty = protocol_shape_type(selected, target);
    match kind {
        "enum" => format!(
            "Result::<{ty}, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(\n    crate::types::{}::from(\n        {data}\n    )\n)",
            rust_type_name(terminal(target))
        ),
        "string" => format!(
            "Result::<{ty}, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(\n    {data}\n    .into()\n)"
        ),
        "timestamp" => format!(
            "::aws_smithy_types::DateTime::from_str(\n    {data}\n    , ::aws_smithy_types::date_time::Format::{format}\n)\n.map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom(\"expected (timestamp: `{target}`)\"))",
            format = protocol_timestamp_format(selected, target)
        ),
        "boolean" | "integer" | "long" | "short" | "byte" | "float" | "double" => format!(
            " {{\n    <{ty} as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(\n        {data}\n    )\n    .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom(\"expected ({kind}: `{target}`)\"))\n}}"
        ),
        _ => format!(
            "Result::<{ty}, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(\n    {data}\n    .into()\n)"
        ),
    }
}

fn protocol_parse_expression(
    selected: &SelectedModel,
    target: &str,
    tag: &str,
    depth: &str,
) -> String {
    match protocol_shape_kind(selected, target) {
        "structure" | "union" | "list" | "map" => format!(
            "crate::protocol_serde::shape_{}::de_{}(&mut {tag}, {depth} + 1)",
            names::rust_module_name(terminal(target)),
            names::rust_module_name(terminal(target))
        ),
        _ => protocol_parse_primitive(selected, target, tag),
    }
}

fn render_protocol_structure_deserializer(
    output: &mut String,
    selected: &SelectedModel,
    shape_id: &str,
    shape: &Value,
    state: &mut ProtocolRenderState,
) {
    let name = rust_type_name(terminal(shape_id));
    output.push_str("#[allow(clippy::needless_question_mark)]\n");
    writeln!(
        output,
        "pub fn de_{}(\n    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,\n    depth: u32,\n) -> ::std::result::Result<crate::types::{name}, ::aws_smithy_xml::decode::XmlDecodeError> {{",
        names::rust_module_name(terminal(shape_id))
    )
    .unwrap();
    output.push_str(
        "    if depth >= 128u32 {\n        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(\"maximum nesting depth exceeded\"));\n    }\n    #[allow(unused_mut)]\n",
    );
    writeln!(
        output,
        "    let mut builder = crate::types::{name}::builder();"
    )
    .unwrap();
    for (member_name, member) in members(shape) {
        if !protocol_member_is_attribute(member) {
            continue;
        }
        let target = member_target(member).unwrap_or_default();
        let field = names::rust_identifier(&member_name);
        let xml_name = protocol_member_xml_name(selected, &member_name, member);
        let attrib = state.temp().replace("var_", "attrib_");
        writeln!(
                output,
                "    let {attrib} = {{\n        let s = decoder.start_el().attr({xml_name:?});\n        match s {{\n            None => None,\n            Some(s) => Some({}?),\n        }}\n    }};\n    builder.{field} = {attrib};",
                indent_expression(
                    &protocol_parse_primitive_data(selected, target, "s"),
                    12,
                )
        )
        .unwrap();
    }
    let data_members = members(shape)
        .into_iter()
        .filter(|(_, member)| !protocol_member_is_attribute(member))
        .collect::<Vec<_>>();
    if members(shape).is_empty() {
        output.push_str("    let _ = decoder;\n");
    }
    if !data_members.is_empty() {
        output.push_str(
            "    while let Some(mut tag) = decoder.next_tag() {\n        match tag.start_el() {\n",
        );
        for (member_name, member) in data_members {
            let target = member_target(member).unwrap_or_default();
            let field = names::rust_identifier(&member_name);
            let xml_name = protocol_member_xml_name(selected, &member_name, member);
            let member_id = shape_id.to_owned() + "$" + member_name.as_str();
            let outer = state.temp();
            writeln!(
                output,
                "            s if s.matches({xml_name:?}) /* {member_name} {member_id} */ =>  {{"
            )
            .unwrap();
            let kind = protocol_shape_kind(selected, target);
            let parse = if kind == "list" && has_trait(member, "smithy.api#xmlFlattened") {
                let list = state.list_accum();
                let element = selected
                    .model
                    .shapes
                    .get(target)
                    .and_then(|shape| shape.get("member"))
                    .expect("flattened list member");
                let element_target = member_target(element).unwrap_or_default();
                let element_expr = indent_expression(
                    &protocol_parse_expression(selected, element_target, "tag", "depth"),
                    8,
                );
                let list_type = selected
                    .model
                    .shapes
                    .get(target)
                    .and_then(|shape| shape.get("member"))
                    .and_then(member_target)
                    .map(|element_target| {
                        format!(
                            "::std::vec::Vec::<{}>",
                            protocol_shape_type(selected, element_target)
                        )
                    })
                    .unwrap_or_else(|| protocol_shape_type(selected, target));
                format!(
                    "Result::<{}, ::aws_smithy_xml::decode::XmlDecodeError>::Ok({{\n    let mut {list} = builder.{field}.take().unwrap_or_default();\n    {list}.push(\n        {element_expr}\n        ?\n    );\n    {list}\n}})",
                    list_type
                )
            } else {
                protocol_parse_expression(selected, target, "tag", "depth")
            };
            let parse = indent_expression(&parse, 24);
            writeln!(
                output,
                "                let {outer} =\n                    Some(\n                        {parse}\n                        ?\n                    )\n                ;\n                builder = builder.set_{field}({outer});"
            )
            .unwrap();
            output.push_str("            }\n            ,\n");
        }
        output.push_str("            _ => {}\n        }\n    }\n");
    }
    let required_members = members(shape)
        .into_iter()
        .filter(|(_, member)| member_is_required(member))
        .collect::<Vec<_>>();
    if required_members.is_empty() {
        output.push_str("    Ok(builder.build())\n");
    } else if required_members.iter().all(|(_, member)| {
        member_target(member)
            .and_then(|target| selected.model.shapes.get(target))
            .and_then(|shape| shape.get("type"))
            .and_then(Value::as_str)
            == Some("structure")
    }) {
        writeln!(
            output,
            "    Ok(crate::serde_util::{}_correct_errors(builder).build())",
            names::rust_module_name(terminal(shape_id))
        )
        .unwrap();
    } else {
        writeln!(
            output,
            "    Ok(crate::serde_util::{}_correct_errors(builder)\n        .build()\n        .map_err(|_| ::aws_smithy_xml::decode::XmlDecodeError::custom(\"missing field\"))?)",
            names::rust_module_name(terminal(shape_id))
        )
        .unwrap();
    }
    output.push_str("}\n\n");
}

fn render_protocol_union_deserializer(
    output: &mut String,
    selected: &SelectedModel,
    shape_id: &str,
) {
    let name = rust_type_name(terminal(shape_id));
    writeln!(
        output,
        "pub fn de_{}(\n    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,\n    depth: u32,\n) -> ::std::result::Result<crate::types::{name}, ::aws_smithy_xml::decode::XmlDecodeError> {{",
        names::rust_module_name(terminal(shape_id))
    )
    .unwrap();
    writeln!(
        output,
        "    if depth >= 128u32 {{\n        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(\"maximum nesting depth exceeded\"));\n    }}\n    let mut base: Option<crate::types::{name}> = None;\n    while let Some(mut tag) = decoder.next_tag() {{\n        match tag.start_el() {{"
    )
    .unwrap();
    let shape = selected.model.shapes.get(shape_id).expect("union");
    for (member_name, member) in members(shape) {
        let variant = rust_type_name(&member_name);
        let target = member_target(member).unwrap_or_default();
        let xml_name = protocol_member_xml_name(selected, &member_name, member);
        let member_id = shape_id.to_owned() + "$" + member_name.as_str();
        writeln!(
            output,
            "            s if s.matches({xml_name:?}) /* {member_name} {member_id} */ =>  {{"
        )
        .unwrap();
        if target == "smithy.api#Unit" {
            writeln!(
                output,
                "                base = Some(crate::types::{name}::{variant});"
            )
            .unwrap();
        } else {
            let expression = indent_expression(
                &protocol_parse_expression(selected, target, "tag", "depth"),
                20,
            );
            writeln!(
                output,
                "                let tmp =\n                    {expression}\n                    ?\n                ;\n                base = Some(crate::types::{name}::{variant}(tmp));"
            )
            .unwrap();
        }
        output.push_str("            }\n            ,\n");
    }
    writeln!(
        output,
        "            _unknown => base = Some(crate::types::{name}::Unknown),\n        }}\n    }}\n    base.ok_or_else(|| ::aws_smithy_xml::decode::XmlDecodeError::custom(\"expected union, got nothing\"))\n}}"
    )
    .unwrap();
    output.push_str("\n\n");
}

fn render_protocol_list_deserializer(
    output: &mut String,
    selected: &SelectedModel,
    shape_id: &str,
    shape: &Value,
) {
    let member = shape.get("member").expect("list member");
    let target = member_target(member).unwrap_or_default();
    let item_type = protocol_shape_type(selected, target);
    let xml_name = protocol_member_xml_name(selected, "member", member);
    writeln!(
        output,
        "pub fn de_{}(\n    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,\n    depth: u32,\n) -> ::std::result::Result<::std::vec::Vec<{item_type}>, ::aws_smithy_xml::decode::XmlDecodeError> {{",
        names::rust_module_name(terminal(shape_id))
    )
    .unwrap();
    output.push_str(
        "    if depth >= 128u32 {\n        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(\"maximum nesting depth exceeded\"));\n    }\n    let mut out = std::vec::Vec::new();\n    while let Some(mut tag) = decoder.next_tag() {\n        match tag.start_el() {\n",
    );
    let parse = indent_expression(
        &protocol_parse_expression(selected, target, "tag", "depth"),
        20,
    );
    writeln!(
        output,
        "            s if s.matches({xml_name:?}) /* member {shape_id}$member */ =>  {{\n                out.push(\n                    {}\n                    ?\n                );\n            }}\n            ,\n            _ => {{}}\n        }}\n    }}\n    Ok(out)\n}}\n\n",
        parse
    )
    .unwrap();
}

fn render_protocol_map_deserializer(
    output: &mut String,
    selected: &SelectedModel,
    shape_id: &str,
    shape: &Value,
) {
    let key = shape.get("key").expect("map key");
    let value = shape.get("value").expect("map value");
    let key_target = member_target(key).unwrap_or_default();
    let value_target = member_target(value).unwrap_or_default();
    let key_type = protocol_shape_type(selected, key_target);
    let value_type = protocol_shape_type(selected, value_target);
    let name = names::rust_module_name(terminal(shape_id));
    writeln!(
        output,
        "pub fn de_{name}(\n    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,\n    depth: u32,\n) -> ::std::result::Result<::std::collections::HashMap<{key_type}, {value_type}>, ::aws_smithy_xml::decode::XmlDecodeError> {{\n    if depth >= 128u32 {{\n        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(\"maximum nesting depth exceeded\"));\n    }}\n    let mut out = ::std::collections::HashMap::new();\n    while let Some(mut tag) = decoder.next_tag() {{\n        match tag.start_el() {{\n            s if s.matches(\"entry\") => {{\n                let mut k = None;\n                let mut v = None;\n                while let Some(mut entry_tag) = tag.next_tag() {{\n                    match entry_tag.start_el() {{\n                        s if s.matches(\"key\") => k = Some({}),\n                        s if s.matches(\"value\") => v = Some({}),\n                        _ => {{}},\n                    }}\n                }}\n                out.insert(k.ok_or_else(|| ::aws_smithy_xml::decode::XmlDecodeError::custom(\"missing key map entry\"))?, v.ok_or_else(|| ::aws_smithy_xml::decode::XmlDecodeError::custom(\"missing value map entry\"))?);\n            }}\n            _ => {{}},\n        }}\n    }}\n    Ok(out)\n}}\n\n",
        protocol_parse_expression(selected, key_target, "entry_tag", "depth"),
        protocol_parse_expression(selected, value_target, "entry_tag", "depth"),
    )
    .unwrap();
}

fn render_protocol_error_file(selected: &SelectedModel, shape_id: &str) -> String {
    let shape = selected.model.shapes.get(shape_id).expect("error shape");
    let name = rust_type_name(terminal(shape_id));
    let mut output = String::new();
    client_operation_header(&mut output);
    writeln!(
        output,
        "#[allow(unused_mut)]\npub fn de_{}_xml_err(\n    inp: &[u8],\n    mut builder: crate::types::error::builders::{name}Builder,\n) -> std::result::Result<crate::types::error::builders::{name}Builder, ::aws_smithy_xml::decode::XmlDecodeError> {{\n    if inp.is_empty() {{\n        return Ok(builder);\n    }}\n    let mut document = ::aws_smithy_xml::decode::Document::try_from(inp)?;\n    #[allow(unused_mut)]\n    let mut error_decoder = crate::rest_xml_unwrapped_errors::error_scope(&mut document)?;\n    #[allow(unused_variables)]\n    let depth = 0u32;\n    while let Some(mut tag) = error_decoder.next_tag() {{\n        match tag.start_el() {{",
        names::rust_module_name(terminal(shape_id))
    )
    .unwrap();
    let mut state = ProtocolRenderState::default();
    let mut error_members = Vec::new();
    if let Some((member_name, member)) = error_message_member(shape) {
        error_members.push((member_name, member));
    }
    error_members.extend(
        members(shape)
            .into_iter()
            .filter(|(member_name, _)| !member_name.eq_ignore_ascii_case("message")),
    );
    for (member_name, member) in error_members {
        let target = member_target(member).unwrap_or_default();
        let field = names::rust_identifier(&member_name);
        let xml_name = protocol_member_xml_name(selected, &member_name, member);
        let var = state.temp();
        let comment = shape_id.to_owned() + "$" + member_name.as_str();
        let parse = indent_expression(&protocol_parse_primitive(selected, target, "tag"), 24);
        writeln!(
            output,
            "            s if s.matches({xml_name:?}) /* {member_name} {comment} */ =>  {{\n                let {var} =\n                    Some(\n                        {}\n                        ?\n                    )\n                ;\n                builder = builder.set_{field}({var});\n            }}\n            ,",
            parse
        )
        .unwrap();
    }
    if serde_util_shape_needs_correction(shape) {
        let correction = format!(
            "crate::serde_util::{}_correct_errors(builder)",
            names::rust_module_name(terminal(shape_id))
        );
        if serde_util_builder_is_fallible(selected, shape) {
            writeln!(
                output,
                "            _ => {{}}\n        }}\n    }}\n    Ok({correction}.build().map_err(|_| ::aws_smithy_xml::decode::XmlDecodeError::custom(\"missing field\"))?)\n}}"
            )
            .unwrap();
        } else {
            writeln!(
                output,
                "            _ => {{}}\n        }}\n    }}\n    Ok({correction}.build())\n}}"
            )
            .unwrap();
        }
    } else {
        output.push_str("            _ => {}\n        }\n    }\n    Ok(builder)\n}\n");
    }
    output
}

fn xml_namespace(selected: &SelectedModel) -> (String, Option<String>) {
    selected
        .model
        .shapes
        .get(selected.model.entry.service_shape_id)
        .and_then(|service| service.get("traits"))
        .and_then(Value::as_object)
        .and_then(|traits| traits.get("smithy.api#xmlNamespace"))
        .and_then(Value::as_object)
        .and_then(|namespace| {
            namespace.get("uri").and_then(Value::as_str).map(|uri| {
                (
                    uri.to_owned(),
                    namespace
                        .get("prefix")
                        .and_then(Value::as_str)
                        .map(ToOwned::to_owned),
                )
            })
        })
        .unwrap_or_else(|| (String::new(), None))
}

fn render_protocol_http_error(
    output: &mut String,
    selected: &SelectedModel,
    operation_name: &str,
    operation: &Value,
    consumer_namespace: bool,
) {
    let module = names::snake_case(operation_name);
    let rust_operation = rust_type_name(operation_name);
    let output_path =
        protocol_operation_type_path(&module, &rust_operation, "Output", consumer_namespace);
    let error_path =
        protocol_operation_type_path(&module, &rust_operation, "Error", consumer_namespace);
    writeln!(
        output,
        "#[allow(clippy::unnecessary_wraps)]\npub fn de_{module}_http_error(\n    _response_status: u16,\n    _response_headers: &::aws_smithy_runtime_api::http::Headers,\n    _response_body: &[u8],\n) -> std::result::Result<{output_path}, {error_path}> {{"
    )
    .unwrap();
    output.push_str("    #[allow(unused_mut)]\n");
    writeln!(
        output,
        "    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)\n        .map_err({error_path}::unhandled)?;"
    )
    .unwrap();
    let request_ids = request_id_plan(selected);
    if request_ids.extended {
        writeln!(
            output,
            "    generic_builder = crate::s3_request_id::apply_extended_request_id(generic_builder, _response_headers);"
        )
        .unwrap();
    }
    if request_ids.standard {
        output.push_str(
            "    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);\n",
        );
    }
    output.push_str("    let generic = generic_builder.build();\n");
    let errors = operation
        .get("errors")
        .and_then(Value::as_array)
        .map(|errors| errors.iter().filter_map(target_value).collect::<Vec<_>>())
        .unwrap_or_default();
    if errors.is_empty() {
        writeln!(output, "    Err({error_path}::generic(generic))\n}}\n").unwrap();
        return;
    }

    output.push_str(
        "    let error_code = match generic.code() {\n        Some(code) => code,\n        None => return Err(",
    );
    writeln!(output, "{error_path}::unhandled(generic)),").unwrap();
    output.push_str("    };\n\n    let _error_message = generic.message().map(|msg| msg.to_owned());\n    Err(match error_code {\n");
    for error in errors {
        render_protocol_error_arm(output, &error_path, error);
    }
    writeln!(
        output,
        "        _ => {error_path}::generic(generic),\n    }})\n}}\n"
    )
    .unwrap();
}

fn render_protocol_error_arm(output: &mut String, error_path: &str, error: &str) {
    let error_name = rust_type_name(terminal(error));
    let error_module = names::snake_case(terminal(error));
    writeln!(
        output,
        "        {error_name:?} => {error_path}::{error_name}({{"
    )
    .unwrap();
    output.push_str("            #[allow(unused_mut)]\n            let mut tmp = {\n");
    writeln!(
        output,
        "                #[allow(unused_mut)]\n                let mut output = crate::types::error::builders::{error_name}Builder::default();"
    )
    .unwrap();
    writeln!(
        output,
        "                output = crate::protocol_serde::shape_{error_module}::de_{error_module}_xml_err(_response_body, output)\n                    .map_err({error_path}::unhandled)?;"
    )
    .unwrap();
    writeln!(
        output,
        "                let output = output.meta(generic);\n                output.build()\n            }};"
    )
    .unwrap();
    output.push_str("            if tmp.message.is_none() {\n                tmp.message = _error_message;\n            }\n            tmp\n        }),\n");
}

fn protocol_operation_type_path(
    module: &str,
    operation: &str,
    suffix: &str,
    consumer_namespace: bool,
) -> String {
    if consumer_namespace {
        format!("super::super::super::{module}::{operation}{suffix}")
    } else {
        format!("crate::operation::{module}::{operation}{suffix}")
    }
}

fn render_protocol_http_response(
    output: &mut String,
    selected: &SelectedModel,
    operation_name: &str,
    output_shape: Option<&Value>,
    consumer_namespace: bool,
) {
    let module = names::snake_case(operation_name);
    let rust_operation = rust_type_name(operation_name);
    let output_path =
        protocol_operation_type_path(&module, &rust_operation, "Output", consumer_namespace);
    let error_path =
        protocol_operation_type_path(&module, &rust_operation, "Error", consumer_namespace);
    let streaming_payload = output_shape.and_then(|shape| {
        members(shape).into_iter().find(|(_, member)| {
            has_trait(member, "smithy.api#httpPayload")
                && member_target(member).is_some_and(|target| {
                    selected
                        .model
                        .shapes
                        .get(target)
                        .is_some_and(shape_is_streaming)
                })
        })
    });
    if streaming_payload.is_some() {
        writeln!(
            output,
            "#[allow(clippy::unnecessary_wraps)]\npub fn de_{module}_http_response(\n    response: &mut ::aws_smithy_runtime_api::http::Response,\n) -> std::result::Result<{output_path}, {error_path}> {{\n    let mut _response_body = ::aws_smithy_types::body::SdkBody::taken();\n    std::mem::swap(&mut _response_body, response.body_mut());\n    let _response_body = &mut _response_body;\n\n    let _response_status = response.status().as_u16();\n    let _response_headers = response.headers();"
        )
        .unwrap();
    } else {
        writeln!(
            output,
            "#[allow(clippy::unnecessary_wraps)]\npub fn de_{module}_http_response(\n    _response_status: u16,\n    _response_headers: &::aws_smithy_runtime_api::http::Headers,\n    _response_body: &[u8],\n) -> std::result::Result<{output_path}, {error_path}> {{"
        )
        .unwrap();
    }
    output.push_str("    Ok({\n        #[allow(unused_mut)]\n");
    let builder_path = if consumer_namespace {
        format!("super::super::super::{module}::builders::{rust_operation}OutputBuilder")
    } else {
        format!("crate::operation::{module}::builders::{rust_operation}OutputBuilder")
    };
    writeln!(
        output,
        "        let mut output = {builder_path}::default();"
    )
    .unwrap();
    if streaming_payload.is_none()
        && protocol_operation_has_document_output(selected, operation_name)
    {
        let helper_path = if consumer_namespace {
            format!("super::super::super::protocol_serde::shape_{module}")
        } else {
            format!("crate::protocol_serde::shape_{module}")
        };
        writeln!(
            output,
            "        output = {helper_path}::de_{module}(_response_body, output)\n            .map_err({error_path}::unhandled)?;"
        )
        .unwrap();
    }
    if let Some(shape) = output_shape {
        for (name, member) in sorted_members(shape) {
            if has_trait(member, "smithy.api#httpPayload") {
                let field = names::rust_identifier(&name);
                let helper_module = format!("shape_{module}_output");
                let helper_path = if consumer_namespace {
                    format!("super::super::super::protocol_serde::{helper_module}")
                } else {
                    format!("crate::protocol_serde::{helper_module}")
                };
                let target_kind = member_target(member)
                    .map(|target| protocol_shape_kind(selected, target))
                    .unwrap_or_default();
                if target_kind == "union" && streaming_payload.is_some() {
                    writeln!(
                        output,
                        "        output = output.set_{field}(Some({helper_path}::de_{field}_payload(\n            _response_body,\n        )?));"
                    )
                    .unwrap();
                } else if streaming_payload.is_some() {
                    writeln!(
                        output,
                        "        output = output.set_{field}(Some({helper_path}::de_{field}_payload(_response_body)?));"
                    )
                    .unwrap();
                } else {
                    writeln!(
                        output,
                        "        output = output.set_{field}({helper_path}::de_{field}_payload(_response_body)?);"
                    )
                    .unwrap();
                }
                continue;
            }
            if let Some(prefix) = member
                .get("traits")
                .and_then(|traits| traits.get("smithy.api#httpPrefixHeaders"))
                .and_then(Value::as_str)
            {
                let field = names::rust_identifier(&name);
                let helper_module = format!("shape_{module}_output");
                let helper_path = if consumer_namespace {
                    format!("super::super::super::protocol_serde::{helper_module}")
                } else {
                    format!("crate::protocol_serde::{helper_module}")
                };
                let error_path = protocol_operation_type_path(
                    &module,
                    &rust_operation,
                    "Error",
                    consumer_namespace,
                );
                writeln!(
                    output,
                    "        output = output.set_{field}(\n            {helper_path}::de_{field}_prefix_header(_response_headers).map_err(|_| {{\n                {error_path}::unhandled(\"Failed to parse {name} from prefix header `{prefix}\")\n            }})?,\n        );"
                )
                .unwrap();
                continue;
            }
            let Some(header) = member
                .get("traits")
                .and_then(|traits| traits.get("smithy.api#httpHeader"))
                .and_then(Value::as_str)
            else {
                continue;
            };
            let field = names::rust_identifier(&name);
            let helper_module = format!("shape_{module}_output");
            let helper_path = if consumer_namespace {
                format!("super::super::super::protocol_serde::{helper_module}")
            } else {
                format!("crate::protocol_serde::{helper_module}")
            };
            let error_path =
                protocol_operation_type_path(&module, &rust_operation, "Error", consumer_namespace);
            writeln!(
                output,
                "        output = output.set_{field}(\n            {helper_path}::de_{field}_header(_response_headers).map_err(|_| {{\n                {error_path}::unhandled(\"Failed to parse {name} from header `{header}\")\n            }})?,\n        );"
            )
            .unwrap();
        }
    }
    let request_ids = request_id_plan(selected);
    if request_ids.extended {
        output.push_str(
            "        output._set_extended_request_id(crate::s3_request_id::RequestIdExt::extended_request_id(_response_headers).map(str::to_string));\n",
        );
    }
    if request_ids.standard {
        output.push_str(
            "        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));\n",
        );
    }
    let streaming_event = streaming_payload.is_some_and(|(_, member)| {
        member_target(member)
            .map(|target| protocol_shape_kind(selected, target) == "union")
            .unwrap_or(false)
    });
    if streaming_event {
        writeln!(
            output,
            "        output\n            .build()\n            .map_err({error_path}::unhandled)?\n    }})\n}}\n\n"
        )
        .unwrap();
    } else if let Some(shape) =
        output_shape.filter(|shape| serde_util_shape_needs_correction(shape))
    {
        let correction =
            format!("crate::serde_util::{module}_output_output_correct_errors(output)");
        if serde_util_builder_is_fallible(selected, shape) {
            writeln!(
                output,
                "        {correction}.build().map_err({error_path}::unhandled)?\n    }})\n}}\n\n"
            )
            .unwrap();
        } else {
            writeln!(output, "        {correction}.build()\n    }})\n}}\n\n").unwrap();
        }
    } else {
        output.push_str("        output.build()\n    })\n}\n\n");
    }
}

fn render_protocol_request_headers(
    output: &mut String,
    selected: &SelectedModel,
    operation_name: &str,
    input_shape: Option<&Value>,
    consumer_namespace: bool,
) {
    let has_headers = input_shape.is_some_and(|shape| {
        members(shape).iter().any(|(_, member)| {
            has_trait(member, "smithy.api#httpHeader")
                || has_trait(member, "smithy.api#httpPrefixHeaders")
        })
    });
    if !has_headers {
        return;
    }
    let module = names::snake_case(operation_name);
    let rust_operation = rust_type_name(operation_name);
    let input_path =
        protocol_operation_type_path(&module, &rust_operation, "Input", consumer_namespace);
    output.push_str(&format!(
        "pub fn ser_{module}_headers(\n    input: &{input_path},\n    mut builder: ::http_1x::request::Builder,\n) -> std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {{\n"
    ));
    let mut index = 1usize;
    if let Some(shape) = input_shape {
        for (name, member) in members(shape) {
            let Some(traits) = member.get("traits").and_then(Value::as_object) else {
                continue;
            };
            let Some(target) = member_target(member) else {
                continue;
            };
            if traits.contains_key("smithy.api#httpPrefixHeaders") {
                continue;
            }
            let Some(header) = traits.get("smithy.api#httpHeader").and_then(Value::as_str) else {
                continue;
            };
            render_protocol_request_header(output, selected, &name, member, target, header, index);
            index += 2;
        }
        for (name, member) in members(shape) {
            let Some(prefix) = member
                .get("traits")
                .and_then(|traits| traits.get("smithy.api#httpPrefixHeaders"))
                .and_then(Value::as_str)
            else {
                continue;
            };
            let Some(target) = member_target(member) else {
                continue;
            };
            render_protocol_request_prefix_header(
                output, selected, &name, member, target, prefix, index,
            );
        }
    }
    output.push_str("    Ok(builder)\n}\n");
}

fn render_protocol_request_header(
    output: &mut String,
    selected: &SelectedModel,
    name: &str,
    member: &Value,
    target: &str,
    header: &str,
    index: usize,
) {
    let field = names::rust_identifier(name);
    let shape = selected.model.shapes.get(target);
    let kind = shape
        .and_then(|shape| shape.get("type"))
        .and_then(Value::as_str)
        .or_else(|| target.strip_prefix("smithy.api#"))
        .unwrap_or("string");
    writeln!(
        output,
        "    if let ::std::option::Option::Some(inner_{index}) = &input.{field} {{"
    )
    .unwrap();
    let formatted = index + 1;
    if kind == "list" {
        let Some(list_shape) = shape else {
            return;
        };
        let Some(member_shape) = list_shape.get("member") else {
            return;
        };
        let Some(member_target) = member_target(member_shape) else {
            return;
        };
        let member_kind = selected
            .model
            .shapes
            .get(member_target)
            .and_then(|shape| shape.get("type"))
            .and_then(Value::as_str)
            .or_else(|| member_target.strip_prefix("smithy.api#"))
            .unwrap_or("string");
        if !matches!(member_kind, "string" | "enum") {
            return;
        }
        let item = index + 1;
        let item_formatted = index + 2;
        let redacted = value_should_redact(selected, member, &mut BTreeSet::new());
        let display_expression = if redacted {
            "&\"*** Sensitive Data Redacted ***\"".to_owned()
        } else {
            "&header_value".to_owned()
        };
        writeln!(
            output,
            "        // Empty vec in header is serialized as an empty string\n        if inner_{index}.is_empty() {{\n            builder = builder.header({header:?}, \"\");\n        }} else {{\n            for inner_{item} in inner_{index} {{\n                let formatted_{item_formatted} = ::aws_smithy_http::header::quote_header_value(inner_{item}.as_str());\n                let header_value = formatted_{item_formatted};\n                let header_value: ::http_1x::HeaderValue = header_value.parse().map_err(|err| {{\n                    ::aws_smithy_types::error::operation::BuildError::invalid_field(\n                        {field:?},\n                        format!(\"`{{}}` cannot be used as a header value: {{}}\", {display_expression}, err),\n                    )\n                }})?;\n                builder = builder.header({header:?}, header_value);\n            }}\n        }}"
        )
        .unwrap();
        output.push_str("    }\n");
        return;
    }
    if matches!(kind, "string" | "enum") {
        writeln!(
            output,
            "        let formatted_{formatted} = inner_{index}.as_str();"
        )
        .unwrap();
    } else if matches!(kind, "timestamp") {
        let timestamp_format = header_timestamp_format(selected, member, target);
        writeln!(
            output,
            "        let formatted_{formatted} = inner_{index}.fmt(::aws_smithy_types::date_time::Format::{timestamp_format})?;"
        )
        .unwrap();
    } else if matches!(
        kind,
        "boolean" | "integer" | "long" | "short" | "byte" | "float" | "double"
    ) {
        writeln!(
            output,
            "        let mut encoder = ::aws_smithy_types::primitive::Encoder::from(*inner_{index});\n        let formatted_{formatted} = encoder.encode();"
        )
        .unwrap();
    } else {
        writeln!(
            output,
            "        let formatted_{formatted} = inner_{index}.to_string();"
        )
        .unwrap();
    }
    let redacted = value_should_redact(selected, member, &mut BTreeSet::new());
    let display_expression = if redacted {
        "&\"*** Sensitive Data Redacted ***\"".to_owned()
    } else {
        "&header_value".to_owned()
    };
    writeln!(
        output,
        "        let header_value = formatted_{formatted};\n        let header_value: ::http_1x::HeaderValue = header_value.parse().map_err(|err| {{\n            ::aws_smithy_types::error::operation::BuildError::invalid_field(\n                {field:?},\n                format!(\"`{{}}` cannot be used as a header value: {{}}\", {display_expression}, err),\n            )\n        }})?;\n        builder = builder.header({header:?}, header_value);\n    }}"
    )
    .unwrap();
}

fn render_protocol_request_prefix_header(
    output: &mut String,
    selected: &SelectedModel,
    name: &str,
    member: &Value,
    target: &str,
    prefix: &str,
    index: usize,
) {
    let field = names::rust_identifier(name);
    let Some(map_shape) = selected.model.shapes.get(target) else {
        return;
    };
    let Some(value_target) = map_shape.get("value").and_then(member_target) else {
        return;
    };
    let value_shape = selected.model.shapes.get(value_target);
    let value_kind = value_shape
        .and_then(|shape| shape.get("type"))
        .and_then(Value::as_str)
        .or_else(|| value_target.strip_prefix("smithy.api#"))
        .unwrap_or("string");
    let value_expression = match value_kind {
        "string" | "enum" => "v.as_str()".to_owned(),
        "timestamp" => format!(
            "v.fmt(::aws_smithy_types::date_time::Format::{})?",
            header_timestamp_format(selected, member, value_target)
        ),
        "boolean" | "integer" | "long" | "short" | "byte" | "float" | "double" => {
            "::aws_smithy_types::primitive::Encoder::from(*v).encode()".to_owned()
        }
        _ => "v.to_string()".to_owned(),
    };
    writeln!(
        output,
        "    if let ::std::option::Option::Some(inner_{index}) = &input.{field} {{"
    )
    .unwrap();
    output.push_str(&format!(
        "        {{\n            for (k, v) in inner_{index} {{\n                use std::str::FromStr;\n"
    ));
    writeln!(
        output,
        "                let header_name = ::http_1x::HeaderName::from_str(&format!(\"{{}}{{}}\", {prefix:?}, &k)).map_err(|err| {{"
    )
    .unwrap();
    writeln!(
        output,
        "                    ::aws_smithy_types::error::operation::BuildError::invalid_field(\n                        {field:?},\n                        format!(\"`{{k}}` cannot be used as a header name: {{err}}\"),\n                    )\n                }})?;"
    )
    .unwrap();
    writeln!(
        output,
        "                let header_value = {value_expression};"
    )
    .unwrap();
    writeln!(
        output,
        "                let header_value: ::http_1x::HeaderValue = header_value.parse().map_err(|err| {{\n                    ::aws_smithy_types::error::operation::BuildError::invalid_field(\n                        {field:?},\n                        format!(\"`{{v}}` cannot be used as a header value: {{err}}\"),\n                    )\n                }})?;"
    )
    .unwrap();
    output.push_str("                builder = builder.header(header_name, header_value);\n            }\n        }\n    }\n");
}

fn header_timestamp_format(selected: &SelectedModel, member: &Value, target: &str) -> &'static str {
    let format = member
        .get("traits")
        .and_then(|traits| traits.get("smithy.api#timestampFormat"))
        .and_then(Value::as_str)
        .or_else(|| {
            selected
                .model
                .shapes
                .get(target)
                .and_then(|shape| shape.get("traits"))
                .and_then(|traits| traits.get("smithy.api#timestampFormat"))
                .and_then(Value::as_str)
        });
    match format {
        Some("date-time") => "DateTime",
        Some("epoch-seconds") => "EpochSeconds",
        _ => "HttpDate",
    }
}

fn response_header_timestamp_format(selected: &SelectedModel, target: &str) -> &'static str {
    match header_timestamp_format(
        selected,
        selected.model.shapes.get(target).unwrap_or(&Value::Null),
        target,
    ) {
        "DateTime" => "DateTimeWithOffset",
        format => format,
    }
}

fn protocol_output_has_headers(selected: &SelectedModel, operation_name: &str) -> bool {
    operation_shape(selected, operation_name)
        .and_then(|operation| operation.get("output"))
        .and_then(target_value)
        .and_then(|id| selected.model.shapes.get(id))
        .is_some_and(|shape| {
            members(shape).iter().any(|(_, member)| {
                member
                    .get("traits")
                    .and_then(Value::as_object)
                    .is_some_and(|traits| {
                        traits.contains_key("smithy.api#httpHeader")
                            || traits.contains_key("smithy.api#httpPrefixHeaders")
                    })
            })
        })
}

fn render_protocol_output_file(
    selected: &SelectedModel,
    operation_name: &str,
    consumer_namespace: bool,
) -> Option<String> {
    let has_headers = protocol_output_has_headers(selected, operation_name);
    let payload = render_protocol_output_payload_file(selected, operation_name);
    if !has_headers {
        return payload;
    }
    let mut output = render_protocol_output_headers(selected, operation_name, consumer_namespace);
    let Some(payload) = payload else {
        return Some(output);
    };
    let operation = operation_shape(selected, operation_name)?;
    let output_shape = operation
        .get("output")
        .and_then(target_value)
        .and_then(|id| selected.model.shapes.get(id))?;
    let (payload_name, _) = members(output_shape)
        .into_iter()
        .find(|(_, member)| has_trait(member, "smithy.api#httpPayload"))?;
    let payload = payload
        .strip_prefix(
            "// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.\n",
        )
        .unwrap_or(&payload);
    let parser_marker = format!("\npub fn de_{}(", names::rust_identifier(&payload_name));
    let (payload, parser) = payload
        .find(&parser_marker)
        .map(|position| (&payload[..position], &payload[position + 1..]))
        .unwrap_or((payload, ""));
    let mut insertion = None;
    for (name, member) in sorted_members(output_shape) {
        if name <= payload_name {
            continue;
        }
        let Some(traits) = member.get("traits").and_then(Value::as_object) else {
            continue;
        };
        let suffix = if traits.contains_key("smithy.api#httpPrefixHeaders") {
            "_prefix_header"
        } else if traits.contains_key("smithy.api#httpHeader") {
            "_header"
        } else {
            continue;
        };
        let marker = format!(
            "pub(crate) fn de_{}{suffix}(",
            names::rust_identifier(&name)
        );
        if let Some(position) = output.find(&marker) {
            insertion = Some(position);
            break;
        }
    }
    if insertion.is_none() {
        for (name, member) in sorted_members(output_shape) {
            if !has_trait(member, "smithy.api#httpPrefixHeaders") {
                continue;
            }
            let marker = format!("pub fn de_{}_inner", names::rust_identifier(&name));
            if let Some(position) = output.find(&marker) {
                insertion = Some(position);
                break;
            }
        }
    }
    if let Some(position) = insertion {
        output.insert_str(position, &format!("{payload}\n"));
    } else {
        output.push_str(payload);
    }
    if !parser.is_empty() {
        output.push_str(parser);
    }
    Some(output)
}

fn render_protocol_output_headers(
    selected: &SelectedModel,
    operation_name: &str,
    _consumer_namespace: bool,
) -> String {
    let operation = operation_shape(selected, operation_name).expect("selected operation exists");
    let output_shape = operation
        .get("output")
        .and_then(target_value)
        .and_then(|id| selected.model.shapes.get(id));
    let mut output = String::new();
    client_operation_header(&mut output);
    let Some(shape) = output_shape else {
        return output;
    };
    let mut index = 1usize;
    let mut prefix_headers = Vec::new();
    for (name, member) in sorted_members(shape) {
        let Some(target) = member_target(member) else {
            continue;
        };
        let Some(traits) = member.get("traits").and_then(Value::as_object) else {
            continue;
        };
        if let Some(prefix) = traits
            .get("smithy.api#httpPrefixHeaders")
            .and_then(Value::as_str)
        {
            render_protocol_response_prefix_header(
                &mut output,
                selected,
                &names::snake_case(operation_name),
                &name,
                target,
                prefix,
            );
            prefix_headers.push((name, target.to_owned()));
            continue;
        }
        let Some(header_name) = traits.get("smithy.api#httpHeader").and_then(Value::as_str) else {
            continue;
        };
        render_protocol_response_header(&mut output, selected, &name, target, header_name, index);
        if response_header_uses_variable(selected, target) {
            index += 1;
        }
    }
    for (name, target) in prefix_headers {
        render_protocol_response_prefix_inner(&mut output, selected, &name, &target);
    }
    output
}

fn render_protocol_output_payload_file(
    selected: &SelectedModel,
    operation_name: &str,
) -> Option<String> {
    let operation = operation_shape(selected, operation_name)?;
    let output_shape = operation
        .get("output")
        .and_then(target_value)
        .and_then(|id| selected.model.shapes.get(id))?;
    let (field_name, member) = members(output_shape)
        .into_iter()
        .find(|(_, member)| has_trait(member, "smithy.api#httpPayload"))?;
    let target = member_target(member)?;
    let field = names::rust_identifier(&field_name);
    let error = format!(
        "crate::operation::{}::{}Error",
        names::rust_module_name(operation_name),
        rust_type_name(operation_name)
    );
    let mut output = String::new();
    client_operation_header(&mut output);
    let target_shape = selected.model.shapes.get(target);
    if target_shape
        .and_then(|shape| shape.get("traits"))
        .and_then(Value::as_object)
        .is_some_and(|traits| traits.contains_key("smithy.api#streaming"))
        && target_shape
            .and_then(|shape| shape.get("type"))
            .and_then(Value::as_str)
            == Some("union")
    {
        writeln!(
            output,
            "pub fn de_{field}_payload(\n    body: &mut ::aws_smithy_types::body::SdkBody,\n) -> std::result::Result<crate::event_receiver::EventReceiver<crate::types::{}, crate::types::error::{}Error>, {error}> {{\n    let unmarshaller = crate::event_stream_serde::{}Unmarshaller::new();\n    let body = std::mem::replace(body, ::aws_smithy_types::body::SdkBody::taken());\n    let receiver = crate::event_receiver::EventReceiver::new(::aws_smithy_http::event_stream::Receiver::new(unmarshaller, body));\n    Ok(receiver)\n}}",
            rust_type_name(terminal(target)),
            rust_type_name(terminal(target)),
            rust_type_name(terminal(target)),
        )
        .unwrap();
        return Some(output);
    }
    let kind = protocol_shape_kind(selected, target);
    if matches!(kind, "string" | "enum") {
        writeln!(
            output,
            "pub(crate) fn de_{field}_payload(\n    body: &[u8],\n) -> std::result::Result<::std::option::Option<::std::string::String>, {error}> {{\n    (!body.is_empty())\n        .then(|| {{\n            let body_str = std::str::from_utf8(body).map_err({error}::unhandled)?;\n            Ok(body_str.to_string())\n        }})\n        .transpose()\n}}"
        )
        .unwrap();
        return Some(output);
    }
    if terminal(target) == "StreamingBlob" {
        writeln!(
            output,
            "pub fn de_{field}_payload(\n    body: &mut ::aws_smithy_types::body::SdkBody,\n) -> std::result::Result<::aws_smithy_types::byte_stream::ByteStream, {error}> {{\n    // replace the body with an empty body\n    let body = std::mem::replace(body, ::aws_smithy_types::body::SdkBody::taken());\n    Ok(::aws_smithy_types::byte_stream::ByteStream::new(body))\n}}",
        )
        .unwrap();
        return Some(output);
    }
    let ty = protocol_shape_type(selected, target);
    writeln!(
        output,
        "pub(crate) fn de_{field}_payload(\n    body: &[u8],\n) -> std::result::Result<::std::option::Option<{ty}>, {error}> {{\n    (!body.is_empty())\n        .then(|| {{\n            crate::protocol_serde::shape_{}_output::de_{field}(body)\n                .map_err({error}::unhandled)\n        }})\n        .transpose()\n}}\n",
        names::rust_module_name(operation_name)
    )
    .unwrap();
    let root = protocol_member_xml_name(selected, &field_name, member);
    writeln!(
        output,
        "pub fn de_{field}(inp: &[u8]) -> std::result::Result<{ty}, ::aws_smithy_xml::decode::XmlDecodeError> {{\n    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;\n    #[allow(unused_mut)]\n    let mut decoder = doc.root_element()?;\n    let start_el = decoder.start_el();\n    if !(start_el.matches({root:?})) {{\n        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(\"invalid root, expected {root} got {{start_el:?}}\")));\n    }}\n    #[allow(unused_variables)]\n    let depth = 0u32;\n    crate::protocol_serde::shape_{}::de_{}(&mut decoder, depth + 1)\n}}",
        names::rust_module_name(terminal(target)),
        names::rust_module_name(terminal(target)),
    )
    .unwrap();
    let _ = kind;
    Some(output)
}

fn response_header_uses_variable(selected: &SelectedModel, target: &str) -> bool {
    let kind = selected
        .model
        .shapes
        .get(target)
        .and_then(|shape| shape.get("type"))
        .and_then(Value::as_str)
        .or_else(|| target.strip_prefix("smithy.api#"))
        .unwrap_or("string");
    matches!(
        kind,
        "boolean" | "integer" | "long" | "short" | "byte" | "float" | "double" | "timestamp"
    )
}

fn render_protocol_response_header(
    output: &mut String,
    selected: &SelectedModel,
    name: &str,
    target: &str,
    header_name: &str,
    index: usize,
) {
    let field = names::rust_identifier(name);
    let kind = selected
        .model
        .shapes
        .get(target)
        .and_then(|shape| shape.get("type"))
        .and_then(Value::as_str)
        .or_else(|| target.strip_prefix("smithy.api#"))
        .unwrap_or("string");
    let return_type = type_expr(
        selected,
        target,
        Context::Types {
            consumer_namespace: false,
        },
    );
    writeln!(
        output,
        "pub(crate) fn de_{field}_header(\n    header_map: &::aws_smithy_runtime_api::http::Headers,\n) -> ::std::result::Result<::std::option::Option<{return_type}>, ::aws_smithy_http::header::ParseError> {{\n    let headers = header_map.get_all({header_name:?});"
    )
    .unwrap();
    match kind {
        "boolean" | "integer" | "long" | "short" | "byte" | "float" | "double" => {
            writeln!(
                output,
                "    let var_{index} = ::aws_smithy_http::header::read_many_primitive::<{return_type}>(headers)?;\n    if var_{index}.len() > 1 {{\n        Err(::aws_smithy_http::header::ParseError::new(format!(\n            \"expected one item but found {{}}\",\n            var_{index}.len()\n        )))\n    }} else {{\n        let mut var_{index} = var_{index};\n        Ok(var_{index}.pop())\n    }}"
            )
            .unwrap();
        }
        "timestamp" => {
            let timestamp_format = response_header_timestamp_format(selected, target);
            writeln!(
                output,
                "    let var_{index}: Vec<{return_type}> = ::aws_smithy_http::header::many_dates(headers, ::aws_smithy_types::date_time::Format::{timestamp_format})?;\n    if var_{index}.len() > 1 {{\n        Err(::aws_smithy_http::header::ParseError::new(format!(\n            \"expected one item but found {{}}\",\n            var_{index}.len()\n        )))\n    }} else {{\n        let mut var_{index} = var_{index};\n        Ok(var_{index}.pop())\n    }}"
            )
            .unwrap();
        }
        _ => output.push_str("    ::aws_smithy_http::header::one_or_none(headers)\n"),
    }
    output.push_str("}\n\n");
}

fn render_protocol_response_prefix_header(
    output: &mut String,
    selected: &SelectedModel,
    operation_module: &str,
    name: &str,
    target: &str,
    prefix: &str,
) {
    let field = names::rust_identifier(name);
    let Some(map_shape) = selected.model.shapes.get(target) else {
        return;
    };
    let Some(value_target) = map_shape.get("value").and_then(member_target) else {
        return;
    };
    let value_type = type_expr(
        selected,
        value_target,
        Context::Types {
            consumer_namespace: false,
        },
    );
    writeln!(
        output,
        "pub(crate) fn de_{field}_prefix_header(\n    header_map: &::aws_smithy_runtime_api::http::Headers,\n) -> std::result::Result<::std::option::Option<::std::collections::HashMap<::std::string::String, {value_type}>>, ::aws_smithy_http::header::ParseError> {{\n    let headers = ::aws_smithy_http::header::headers_for_prefix(header_map.iter().map(|(k, _)| k), {prefix:?});\n    let out: std::result::Result<_, _> = headers.map(|(key, header_name)| {{\n                            let values = header_map.get_all(header_name);\n                            crate::protocol_serde::shape_{operation_module}_output::de_{field}_inner(values).map(|v| (key.to_string(), v.expect(\n                                \"we have checked there is at least one value for this header name; please file a bug report under https://github.com/smithy-lang/smithy-rs/issues\"\n                            )))\n                        }}).collect();\n    out.map(Some)\n}}\n\n"
    )
    .unwrap();
}

fn render_protocol_response_prefix_inner(
    output: &mut String,
    selected: &SelectedModel,
    name: &str,
    target: &str,
) {
    let field = names::rust_identifier(name);
    let Some(map_shape) = selected.model.shapes.get(target) else {
        return;
    };
    let Some(value_target) = map_shape.get("value").and_then(member_target) else {
        return;
    };
    let value_type = type_expr(
        selected,
        value_target,
        Context::Types {
            consumer_namespace: false,
        },
    );
    writeln!(
        output,
        "pub fn de_{field}_inner<'a>(\n    headers: impl ::std::iter::Iterator<Item = &'a str>,\n) -> std::result::Result<Option<{value_type}>, ::aws_smithy_http::header::ParseError> {{\n    ::aws_smithy_http::header::one_or_none(headers)\n}}\n"
    )
    .unwrap();
}

fn render_response_decode(
    output: &mut String,
    operation_name: &str,
    selected: &SelectedModel,
    shape: Option<&Value>,
    protocol: crate::model::ProtocolKind,
    consumer_namespace: bool,
) {
    let rust_operation = rust_type_name(operation_name);
    let operation_symbol = operation_name;
    let request_id_plan = request_id_plan(selected);
    let byte_stream_type = if consumer_namespace {
        "super::super::super::primitives::ByteStream"
    } else {
        "::aws_smithy_types::byte_stream::ByteStream"
    };
    let output_builder = format!(
        "super::_{}_output::{}OutputBuilder",
        names::snake_case(operation_name),
        rust_operation
    );
    let output_requires_validation = shape.is_some_and(|shape| {
        members(shape).iter().any(|(_, member)| {
            let target = member_target(member).unwrap_or("smithy.api#String");
            member_is_effectively_required(selected, member, target)
        })
    });
    let has_decoded_values = shape
        .map(|shape| {
            members(shape).iter().any(|(_, member)| {
                let target = member_target(member).unwrap_or_default();
                let target_kind = selected
                    .model
                    .shapes
                    .get(target)
                    .and_then(|shape| shape.get("type"))
                    .and_then(Value::as_str)
                    .unwrap_or_default();
                let traits = member.get("traits").and_then(Value::as_object);
                (traits
                    .and_then(|traits| traits.get("smithy.api#httpPayload"))
                    .is_some()
                    && terminal(target) == "StreamingBlob")
                    || traits
                        .and_then(|traits| traits.get("smithy.api#httpHeader"))
                        .is_some()
                        && matches!(
                            target_kind,
                            "string"
                                | "boolean"
                                | "integer"
                                | "long"
                                | "short"
                                | "byte"
                                | "float"
                                | "double"
                        )
                    || is_xml_flattened_list(member, selected)
                    || (protocol == crate::model::ProtocolKind::RestXml
                        && is_xml_body_member(member))
            })
        })
        .unwrap_or(false);
    if !has_decoded_values {
        let output_is_unit = shape.map(|shape| members(shape).is_empty()).unwrap_or(true);
        if output_is_unit && !request_id_plan.standard && !request_id_plan.extended {
            writeln!(
                output,
                "                         Ok(super::{rust_operation}Output{{}})"
            )
            .unwrap();
        } else {
            output.push_str(&format!(
                "                         let mut output = {output_builder}::default();\n"
            ));
            render_response_request_ids(output, request_id_plan);
            if output_requires_validation {
                writeln!(
                    output,
                    "                         output.build().map_err(|error| super::{operation_symbol}Error::Unhandled(error.to_string()))"
                )
                .unwrap();
            } else {
                output.push_str("                         Ok(output.build())\n");
            }
        }
        return;
    }
    output.push_str(&format!(
        "                         let mut output = {output_builder}::default();\n"
    ));
    let xml_flattened_lists = shape
        .map(|shape| {
            members(shape)
                .into_iter()
                .filter(|(_, member)| is_xml_flattened_list(member, selected))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let has_xml_body = protocol == crate::model::ProtocolKind::RestXml
        && shape.is_some_and(|shape| {
            members(shape)
                .iter()
                .any(|(_, member)| is_xml_body_member(member))
        });
    if has_xml_body || !xml_flattened_lists.is_empty() {
        output.push_str(&format!(
            "                         let body = response.text().await.map_err(super::{operation_symbol}Error::Unhandled)?;\n"
        ));
    }
    if let Some(shape) = shape {
        for (name, member) in members(shape) {
            let field = names::rust_identifier(&name);
            let target = member_target(member).unwrap_or_default();
            let target_name = terminal(target);
            let target_kind = selected
                .model
                .shapes
                .get(target)
                .and_then(|shape| shape.get("type"))
                .and_then(Value::as_str)
                .unwrap_or_default();
            let traits = member.get("traits").and_then(Value::as_object);
            if traits
                .and_then(|traits| traits.get("smithy.api#httpPayload"))
                .is_some()
            {
                if target_name == "StreamingBlob" {
                    output.push_str(&format!(
                        "                         output.{field} = Some({byte_stream_type}::from(response.body().to_vec()));\n"
                    ));
                } else if protocol == crate::model::ProtocolKind::RestXml {
                    render_xml_member_decode(
                        output,
                        selected,
                        &field,
                        &name,
                        member,
                        consumer_namespace,
                    );
                }
                continue;
            }
            if is_xml_flattened_list(member, selected) {
                render_xml_flattened_list_decode(
                    output,
                    selected,
                    &name,
                    member,
                    consumer_namespace,
                );
                continue;
            }
            if let Some(header) = traits
                .and_then(|traits| traits.get("smithy.api#httpHeader"))
                .and_then(Value::as_str)
            {
                if matches!(
                    target_kind,
                    "integer" | "long" | "short" | "byte" | "float" | "double" | "boolean"
                ) {
                    output.push_str(&format!(
                        "                         output.{field} = response.header({header:?}).and_then(|value| value.parse().ok());\n"
                    ));
                } else if target_kind == "string" {
                    output.push_str(&format!(
                        "                         output.{field} = response.header({header:?}).map(str::to_owned);\n"
                    ));
                }
            } else if protocol == crate::model::ProtocolKind::RestXml {
                render_xml_member_decode(
                    output,
                    selected,
                    &field,
                    &name,
                    member,
                    consumer_namespace,
                );
            }
        }
    }
    render_response_request_ids(output, request_id_plan);
    if output_requires_validation {
        writeln!(
            output,
        "                         output.build().map_err(|error| super::{operation_symbol}Error::Unhandled(error.to_string()))"
        )
        .unwrap();
    } else {
        output.push_str("                         Ok(output.build())\n");
    }
}

fn render_response_request_ids(output: &mut String, request_id_plan: RequestIdPlan) {
    if request_id_plan.extended {
        output.push_str(
            "                         output._set_extended_request_id(response.header(\"x-amz-id-2\").map(str::to_owned));\n",
        );
    }
    if request_id_plan.standard {
        output.push_str(
            "                         output._set_request_id(response.header(\"x-amzn-requestid\").map(str::to_owned));\n",
        );
    }
}

fn render_xml_flattened_list_decode(
    output: &mut String,
    selected: &SelectedModel,
    member_name: &str,
    member: &Value,
    consumer_namespace: bool,
) {
    let Some(list_id) = member_target(member) else {
        return;
    };
    let Some(list_shape) = selected.model.shapes.get(list_id) else {
        return;
    };
    let Some(element_id) = list_shape.get("member").and_then(member_target) else {
        return;
    };
    let Some(element_shape) = selected.model.shapes.get(element_id) else {
        return;
    };
    let output_field = names::rust_identifier(member_name);
    let tag = xml_name(member).unwrap_or_else(|| member_name.to_owned());
    if element_shape.get("type").and_then(Value::as_str) != Some("structure") {
        writeln!(
            output,
            "                         let values = super::super::super::transport::xml_tags(&body, {tag:?}).into_iter().filter_map(|value| value.parse().ok()).collect();\n                         output.{output_field} = Some(values);"
        )
        .unwrap();
        return;
    }
    let item_builder_type = type_builder_path_for_shape(element_id, consumer_namespace);
    let item_has_required_members = structure_has_required_members(element_shape);
    let iterator = if item_has_required_members {
        "filter_map"
    } else {
        "map"
    };
    output.push_str(&format!(
        "                         let values = super::super::super::transport::xml_tags(&body, {tag:?}).into_iter().{iterator}(|value| {{ let mut item: {item_builder_type} = ::std::default::Default::default();"
    ));
    render_xml_structure_decode(
        output,
        selected,
        element_shape,
        "item",
        "value",
        consumer_namespace,
    );
    let build = if item_has_required_members {
        " item.build().ok() })"
    } else {
        " item.build() })"
    };
    writeln!(
        output,
        "{build}.collect();\n                         output.{output_field} = Some(values);"
    )
    .unwrap();
}

fn structure_has_required_members(shape: &Value) -> bool {
    members(shape)
        .iter()
        .any(|(_, member)| member_is_required(member))
}

fn type_builder_path_for_shape(shape_id: &str, consumer_namespace: bool) -> String {
    let name = rust_type_name(terminal(shape_id));
    if consumer_namespace {
        format!("super::super::super::types::{name}Builder")
    } else {
        format!("crate::types::{name}Builder")
    }
}

fn render_xml_member_decode(
    output: &mut String,
    selected: &SelectedModel,
    field: &str,
    member_name: &str,
    member: &Value,
    consumer_namespace: bool,
) {
    let Some(target) = member_target(member) else {
        return;
    };
    let Some(shape) = selected.model.shapes.get(target) else {
        return;
    };
    let tag = xml_name(member)
        .or_else(|| xml_name(shape))
        .unwrap_or_else(|| member_name.to_owned());
    match shape.get("type").and_then(Value::as_str) {
        Some("list") => {
            let element_tag = shape
                .get("member")
                .and_then(xml_name)
                .unwrap_or_else(|| tag.clone());
            let element_target = shape.get("member").and_then(member_target);
            let element_shape = element_target.and_then(|target| selected.model.shapes.get(target));
            let element_kind = element_shape
                .and_then(|shape| shape.get("type"))
                .and_then(Value::as_str);
            if !matches!(
                element_kind,
                Some(
                    "string"
                        | "integer"
                        | "long"
                        | "short"
                        | "byte"
                        | "float"
                        | "double"
                        | "boolean"
                        | "enum"
                )
            ) {
                writeln!(
                    output,
                    "                         output.{field} = Some(::std::vec::Vec::new());"
                )
                .unwrap();
                return;
            }
            let element_type = element_target
                .map(|target| {
                    type_expr(
                        selected,
                        target,
                        Context::Builder {
                            module: names::snake_case(member_name),
                            input: false,
                            consumer_namespace,
                        },
                    )
                })
                .unwrap_or_else(|| "::std::string::String".to_owned());
            writeln!(
                output,
                "                         output.{field} = Some(super::super::super::transport::xml_tags(&body, {tag:?}).into_iter().flat_map(|value| super::super::super::transport::xml_tags(&value, {element_tag:?})).filter_map(|value| value.parse::<{element_type}>().ok()).collect());"
            )
            .unwrap();
        }
        Some("structure") => {
            let element_builder_type = type_builder_path_for_shape(target, consumer_namespace);
            let item_has_required_members = structure_has_required_members(shape);
            output.push_str(&format!(
                "                         if let Some(value) = super::super::super::transport::xml_first(&body, {tag:?}) {{ let mut item: {element_builder_type} = ::std::default::Default::default();"
            ));
            render_xml_structure_decode(
                output,
                selected,
                shape,
                "item",
                "value",
                consumer_namespace,
            );
            if item_has_required_members {
                writeln!(
                    output,
                    " if let Ok(item) = item.build() {{ output.{field} = Some(item); }} }}"
                )
                .unwrap();
            } else {
                writeln!(
                    output,
                    " let item = item.build(); output.{field} = Some(item); }}"
                )
                .unwrap();
            }
        }
        Some(
            "string" | "integer" | "long" | "short" | "byte" | "float" | "double" | "boolean"
            | "enum",
        ) => {
            writeln!(
                output,
                "                         output.{field} = super::super::super::transport::xml_first(&body, {tag:?}).and_then(|value| value.parse().ok());"
            )
            .unwrap();
        }
        _ => {}
    }
}

fn render_xml_structure_decode(
    output: &mut String,
    selected: &SelectedModel,
    shape: &Value,
    item_expression: &str,
    xml_expression: &str,
    _consumer_namespace: bool,
) {
    for (member_name, member) in members(shape) {
        let Some(target) = member_target(member) else {
            continue;
        };
        let Some(member_shape) = selected.model.shapes.get(target) else {
            continue;
        };
        let kind = member_shape.get("type").and_then(Value::as_str);
        if !matches!(
            kind,
            Some("string" | "integer" | "long" | "short" | "byte" | "float" | "double" | "boolean")
        ) {
            continue;
        }
        let tag = xml_name(member)
            .or_else(|| xml_name(member_shape))
            .unwrap_or_else(|| member_name.clone());
        let field = names::rust_identifier(&member_name);
        writeln!(
            output,
            " {item_expression}.{field} = super::super::super::transport::xml_first(&{xml_expression}, {tag:?}).and_then(|value| value.parse().ok());"
        )
        .unwrap();
    }
}

fn is_xml_flattened_list(member: &Value, selected: &SelectedModel) -> bool {
    let Some(traits) = member.get("traits").and_then(Value::as_object) else {
        return false;
    };
    if !traits.contains_key("smithy.api#xmlFlattened") {
        return false;
    }
    let Some(target) = member_target(member) else {
        return false;
    };
    selected
        .model
        .shapes
        .get(target)
        .and_then(|shape| shape.get("type"))
        .and_then(Value::as_str)
        == Some("list")
}

fn is_string_type(target: &str, shape: Option<&Value>) -> bool {
    target.starts_with("smithy.api#String")
        || matches!(
            terminal(target),
            "String"
                | "BucketName"
                | "ObjectKey"
                | "AccountId"
                | "Token"
                | "ETag"
                | "Location"
                | "VersionId"
                | "ObjectVersionId"
        )
        || shape
            .and_then(|shape| shape.get("type"))
            .and_then(Value::as_str)
            == Some("string")
}

fn documentation(value: &Value) -> Option<String> {
    value
        .get("traits")
        .and_then(Value::as_object)
        .and_then(|traits| traits.get("smithy.api#documentation"))
        .and_then(Value::as_str)
        .map(normalize_documentation)
}

fn modeled_member_documentation(selected: &SelectedModel, member: &Value) -> Option<String> {
    documentation(member).or_else(|| {
        member_target(member)
            .and_then(|target| selected.model.shapes.get(target))
            .and_then(documentation)
    })
}

fn normalize_documentation(value: &str) -> String {
    normalize_model_documentation(value)
}

fn render_doc_lines(output: &mut String, documentation: &str, indent: usize) {
    let padding = " ".repeat(indent);
    for line in documentation.lines() {
        writeln!(output, "{padding}/// {}", line.trim_start()).unwrap();
    }
}

fn render_deprecated_attribute(output: &mut String, value: &Value, indent: usize) {
    let Some(deprecated) = value
        .get("traits")
        .and_then(Value::as_object)
        .and_then(|traits| traits.get("smithy.api#deprecated"))
    else {
        return;
    };

    let (note, since) = match deprecated {
        // Some model producers serialize the trait as a string rather than as
        // the Smithy JSON object form.
        Value::String(note) => (Some(note.as_str()), None),
        Value::Object(fields) => (
            fields.get("message").and_then(Value::as_str),
            fields.get("since").and_then(Value::as_str),
        ),
        // Annotation traits are commonly represented as `{}`; accept the
        // equivalent boolean form as well.
        Value::Bool(true) => (None, None),
        _ => return,
    };

    let padding = " ".repeat(indent);
    let mut arguments = Vec::new();
    if note.is_some_and(|note| !note.is_empty()) {
        arguments.push(format!("note = {:?}", note.unwrap()));
    }
    if since.is_some_and(|since| !since.is_empty()) {
        arguments.push(format!("since = {:?}", since.unwrap()));
    }
    if arguments.is_empty() {
        writeln!(output, "{padding}#[deprecated]").unwrap();
    } else {
        writeln!(output, "{padding}#[deprecated({})]", arguments.join(", ")).unwrap();
    }
}

fn render_builder_docs(
    output: &mut String,
    selected: &SelectedModel,
    member: &Value,
    indent: &str,
    required: bool,
) {
    if let Some(documentation) = modeled_member_documentation(selected, member) {
        for line in documentation.lines() {
            writeln!(output, "{indent}/// {}", line.trim_start()).unwrap();
        }
    }
    if required {
        writeln!(output, "{indent}/// This field is required.").unwrap();
    }
}

fn member_is_required(member: &Value) -> bool {
    member
        .get("traits")
        .and_then(Value::as_object)
        .is_some_and(|traits| traits.contains_key("smithy.api#required"))
}

fn is_streaming_target(target: &str) -> bool {
    terminal(target) == "StreamingBlob"
}

fn is_copy_type(target: &str, shape: Option<&Value>) -> bool {
    matches!(
        shape
            .and_then(|shape| shape.get("type"))
            .and_then(Value::as_str)
            .or_else(|| target.strip_prefix("smithy.api#")),
        Some("boolean" | "integer" | "long" | "short" | "byte" | "float" | "double")
    )
}

fn structure_member_type(
    selected: &SelectedModel,
    member: &Value,
    target: &str,
    context: &Context,
) -> String {
    let value_type = type_expr(selected, target, context.clone());
    if is_streaming_target(target) {
        value_type
    } else if operation_input(context) || !member_is_effectively_required(selected, member, target)
    {
        format!("::std::option::Option<{value_type}>")
    } else {
        value_type
    }
}

fn operation_input(context: &Context) -> bool {
    match context {
        Context::Operation { input, .. } | Context::Builder { input, .. } => *input,
        Context::Types { .. } | Context::Error { .. } => false,
    }
}

fn member_is_effectively_required(selected: &SelectedModel, member: &Value, target: &str) -> bool {
    member_is_required(member)
        && selected
            .model
            .shapes
            .get(target)
            .and_then(|shape| shape.get("type"))
            .and_then(Value::as_str)
            != Some("structure")
}

fn has_trait(value: &Value, trait_id: &str) -> bool {
    value
        .get("traits")
        .and_then(Value::as_object)
        .is_some_and(|traits| traits.contains_key(trait_id))
}

fn value_should_redact(
    selected: &SelectedModel,
    value: &Value,
    seen: &mut BTreeSet<String>,
) -> bool {
    has_trait(value, "smithy.api#sensitive")
        || member_target(value).is_some_and(|target| shape_should_redact(selected, target, seen))
}

fn shape_should_redact(
    selected: &SelectedModel,
    target: &str,
    seen: &mut BTreeSet<String>,
) -> bool {
    if !seen.insert(target.to_owned()) {
        return false;
    }
    let Some(shape) = selected.model.shapes.get(target) else {
        return false;
    };
    if has_trait(shape, "smithy.api#sensitive") {
        return true;
    }
    match shape.get("type").and_then(Value::as_str) {
        Some("list") => shape
            .get("member")
            .is_some_and(|member| value_should_redact(selected, member, seen)),
        Some("map") => {
            shape
                .get("key")
                .is_some_and(|key| value_should_redact(selected, key, seen))
                || shape
                    .get("value")
                    .is_some_and(|value| value_should_redact(selected, value, seen))
        }
        _ => false,
    }
}

fn structure_has_sensitive_member(selected: &SelectedModel, shape: &Value) -> bool {
    has_trait(shape, "smithy.api#sensitive")
        || members(shape)
            .iter()
            .any(|(_, member)| value_should_redact(selected, member, &mut BTreeSet::new()))
}

fn structure_has_streaming_member(shape: &Value) -> bool {
    members(shape).iter().any(|(_, member)| {
        has_trait(member, "smithy.api#streaming")
            || member_target(member).is_some_and(is_streaming_target)
    })
}

fn filter_derives(derives: &str, excluded: &[&str]) -> String {
    derives
        .split(", ")
        .filter(|derive| !excluded.contains(derive))
        .collect::<Vec<_>>()
        .join(", ")
}

fn builder_member_is_required(
    selected: &SelectedModel,
    member: &Value,
    target: &str,
    context: &Context,
) -> bool {
    !operation_input(context) && member_is_effectively_required(selected, member, target)
}

fn builder_argument_type(selected: &SelectedModel, target: &str, value_type: &str) -> String {
    if is_string_type(target, selected.model.shapes.get(target)) {
        format!("impl ::std::convert::Into<{value_type}>")
    } else {
        value_type.to_owned()
    }
}

fn builder_argument_value(argument_type: &str, value: &str) -> String {
    if argument_type.starts_with("impl ::std::convert::Into<") {
        format!("{value}.into()")
    } else {
        value.to_owned()
    }
}

fn builder_type_path(context: &Context, name: &str) -> String {
    if context.consumer_namespace() {
        if matches!(context, Context::Error { .. }) {
            return format!("builders::{name}Builder");
        }
        return format!("{name}Builder");
    }
    match context {
        Context::Types { .. } => {
            format!("crate::types::builders::{name}Builder")
        }
        Context::Error { .. } => format!("crate::types::error::builders::{name}Builder"),
        Context::Operation { module, .. } | Context::Builder { module, .. } => {
            format!("crate::operation::{module}::builders::{name}Builder")
        }
    }
}

fn value_type_path(context: &Context, name: &str) -> String {
    if context.consumer_namespace() {
        return name.to_owned();
    }
    match context {
        Context::Types { .. } => format!("crate::types::{name}"),
        Context::Error { .. } => format!("crate::types::error::{name}"),
        Context::Operation { module, .. } | Context::Builder { module, .. } => {
            format!("crate::operation::{module}::{name}")
        }
    }
}

fn build_error_type(context: &Context) -> String {
    if context.consumer_namespace() {
        match context {
            Context::Types { .. } => "super::error::BuildError".to_owned(),
            Context::Error { .. } => "super::super::error::BuildError".to_owned(),
            Context::Operation { .. } | Context::Builder { .. } => {
                "super::super::super::error::BuildError".to_owned()
            }
        }
    } else {
        "::aws_smithy_types::error::operation::BuildError".to_owned()
    }
}

fn render_structure(
    output: &mut String,
    selected: &SelectedModel,
    shape: &Value,
    name: &str,
    context: Context,
) {
    render_structure_at_indent(output, selected, shape, name, context.clone(), 4);
    render_structure_accessors(output, selected, shape, name, context.clone(), 4);
    if is_error_context(&context) {
        render_error_impls(output, selected, shape, name, &context);
    }
    render_type_builder(output, selected, shape, name, context, 4);
}

fn render_structure_at_indent(
    output: &mut String,
    selected: &SelectedModel,
    shape: &Value,
    name: &str,
    context: Context,
    indent: usize,
) {
    let padding = " ".repeat(indent);
    let is_error = is_error_context(&context);
    let request_id_plan = output_request_id_plan(selected, &context);
    if let Some(documentation) = documentation(shape) {
        render_doc_lines(output, &documentation, indent);
    } else {
        writeln!(
            output,
            "{padding}#[allow(missing_docs)] // documentation missing in model"
        )
        .unwrap();
    }
    writeln!(output, "{padding}#[non_exhaustive]").unwrap();
    let derives = "::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug";
    let mut excluded_derives = Vec::new();
    if structure_has_streaming_member(shape) {
        excluded_derives.extend(["::std::clone::Clone", "::std::cmp::PartialEq"]);
    }
    if structure_has_sensitive_member(selected, shape) {
        excluded_derives.push("::std::fmt::Debug");
    }
    let derives = filter_derives(derives, &excluded_derives);
    if members(shape).is_empty()
        && !is_error
        && !request_id_plan.standard
        && !request_id_plan.extended
    {
        if !derives.is_empty() {
            writeln!(output, "{padding}#[derive({derives})]").unwrap();
        }
        writeln!(output, "{padding}pub struct {} {{}}", rust_type_name(name)).unwrap();
        return;
    }
    if !derives.is_empty() {
        writeln!(
            output,
            "{}#[derive({derives})]\n{}pub struct {} {{",
            padding,
            padding,
            rust_type_name(name)
        )
        .unwrap();
    } else {
        writeln!(output, "{padding}pub struct {} {{", rust_type_name(name)).unwrap();
    }
    for (member_name, member) in members(shape) {
        let field = names::rust_identifier(&member_name);
        if let Some(member_doc) = modeled_member_documentation(selected, member) {
            render_doc_lines(output, &member_doc, indent + 4);
        } else if is_error {
            writeln!(
                output,
                "{padding}    #[allow(missing_docs)] // documentation missing in model"
            )
            .unwrap();
        }
        render_deprecated_attribute(output, member, indent + 4);
        let target = member_target(member).unwrap_or("smithy.api#String");
        let field_type = structure_member_type(selected, member, target, &context);
        writeln!(output, "{}    pub {}: {},", padding, field, field_type).unwrap();
    }
    if request_id_plan.extended {
        writeln!(output, "{padding}    _extended_request_id: Option<String>,").unwrap();
    }
    if request_id_plan.standard {
        writeln!(output, "{padding}    _request_id: Option<String>,").unwrap();
    }
    if is_error {
        writeln!(
            output,
            "{padding}    pub(crate) meta: ::aws_smithy_types::error::ErrorMetadata,"
        )
        .unwrap();
    }
    writeln!(output, "{}}}", padding).unwrap();
}

fn render_structure_accessors(
    output: &mut String,
    selected: &SelectedModel,
    shape: &Value,
    name: &str,
    context: Context,
    indent: usize,
) {
    let padding = " ".repeat(indent);
    let is_error = is_error_context(&context);
    let request_id_plan = output_request_id_plan(selected, &context);
    let structure_members = members(shape)
        .into_iter()
        .filter(|(member_name, _)| !is_error || !member_name.eq_ignore_ascii_case("message"))
        .collect::<Vec<_>>();
    if !structure_members.is_empty() {
        writeln!(output, "{padding}impl {} {{", rust_type_name(name)).unwrap();
        for (member_name, member) in structure_members {
            let field = names::rust_identifier(&member_name);
            let target = member_target(member).unwrap_or("smithy.api#String");
            let target_type = type_expr(selected, target, context.clone());
            if let Some(member_doc) = modeled_member_documentation(selected, member) {
                render_doc_lines(output, &member_doc, indent + 4);
            } else if is_error {
                writeln!(
                    output,
                    "{padding}    #[allow(missing_docs)] // documentation missing in model"
                )
                .unwrap();
            }
            render_deprecated_attribute(output, member, indent + 4);
            let required = is_streaming_target(target)
                || (!operation_input(&context)
                    && member_is_effectively_required(selected, member, target));
            let target_shape = selected.model.shapes.get(target);
            let target_kind = target_shape
                .and_then(|shape| shape.get("type"))
                .and_then(Value::as_str)
                .unwrap_or_else(|| terminal(target));
            if target_kind == "list" {
                let element_target = target_shape
                    .and_then(|shape| shape.get("member"))
                    .and_then(member_target)
                    .unwrap_or("smithy.api#String");
                let element_type = type_expr(selected, element_target, context.clone());
                let return_type = format!("&[{element_type}]");
                if required {
                    writeln!(
                        output,
                        "{padding}    pub fn {field}(&self) -> {return_type} {{"
                    )
                    .unwrap();
                    writeln!(output, "{padding}        use std::ops::Deref;").unwrap();
                    writeln!(output, "{padding}        self.{field}.deref()").unwrap();
                    writeln!(output, "{padding}    }}").unwrap();
                } else {
                    writeln!(output, "{padding}    ///").unwrap();
                    writeln!(
                        output,
                        "{padding}    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.{field}.is_none()`."
                    )
                    .unwrap();
                    writeln!(
                        output,
                        "{padding}    pub fn {field}(&self) -> {return_type} {{"
                    )
                    .unwrap();
                    writeln!(
                        output,
                        "{padding}        self.{field}.as_deref().unwrap_or_default()"
                    )
                    .unwrap();
                    writeln!(output, "{padding}    }}").unwrap();
                }
            } else if is_string_type(target, target_shape) {
                if required {
                    writeln!(output, "{padding}    pub fn {field}(&self) -> &str {{").unwrap();
                    writeln!(output, "{padding}        use std::ops::Deref;").unwrap();
                    writeln!(output, "{padding}        self.{field}.deref()").unwrap();
                    writeln!(output, "{padding}    }}").unwrap();
                } else {
                    writeln!(
                    output,
                    "{padding}    pub fn {field}(&self) -> ::std::option::Option<&str> {{\n{padding}        self.{field}.as_deref()\n{padding}    }}"
                )
                .unwrap();
                }
            } else if is_copy_type(target, target_shape) {
                if required {
                    writeln!(
                    output,
                    "{padding}    pub fn {field}(&self) -> {target_type} {{\n{padding}        self.{field}\n{padding}    }}"
                )
                .unwrap();
                } else {
                    writeln!(
                    output,
                    "{padding}    pub fn {field}(&self) -> ::std::option::Option<{target_type}> {{\n{padding}        self.{field}\n{padding}    }}"
                )
                .unwrap();
                }
            } else if required {
                writeln!(
                output,
                "{padding}    pub fn {field}(&self) -> &{target_type} {{\n{padding}        &self.{field}\n{padding}    }}"
            )
            .unwrap();
            } else {
                writeln!(
                output,
                "{padding}    pub fn {field}(&self) -> ::std::option::Option<&{target_type}> {{\n{padding}        self.{field}.as_ref()\n{padding}    }}"
            )
            .unwrap();
            }
        }
        writeln!(output, "{padding}}}").unwrap();
    }
    if is_error {
        writeln!(output, "{padding}impl {} {{", rust_type_name(name)).unwrap();
        writeln!(output, "{padding}    /// Returns the error message.").unwrap();
        writeln!(
            output,
            "{padding}    pub fn message(&self) -> ::std::option::Option<&str> {{"
        )
        .unwrap();
        writeln!(output, "{padding}        self.message.as_deref()").unwrap();
        writeln!(output, "{padding}    }}").unwrap();
        writeln!(output, "{padding}}}").unwrap();
    }
    if structure_has_sensitive_member(selected, shape) {
        render_sensitive_debug_impl(output, selected, shape, name, &context, indent);
    }
    if request_id_plan.extended && !is_error {
        let trait_path = if context.consumer_namespace() {
            "super::super::super::s3_request_id::RequestIdExt"
        } else {
            "crate::s3_request_id::RequestIdExt"
        };
        writeln!(
            output,
            "{padding}impl {trait_path} for {} {{\n{padding}    fn extended_request_id(&self) -> Option<&str> {{\n{padding}        self._extended_request_id.as_deref()\n{padding}    }}\n{padding}}}",
            rust_type_name(name),
        )
        .unwrap();
    }
    if request_id_plan.standard && !is_error {
        writeln!(
            output,
            "{padding}impl ::aws_types::request_id::RequestId for {} {{\n{padding}    fn request_id(&self) -> Option<&str> {{\n{padding}        self._request_id.as_deref()\n{padding}    }}\n{padding}}}",
            rust_type_name(name)
        )
        .unwrap();
    }
}

fn render_sensitive_debug_impl(
    output: &mut String,
    selected: &SelectedModel,
    shape: &Value,
    name: &str,
    context: &Context,
    indent: usize,
) {
    let padding = " ".repeat(indent);
    let request_id_plan = output_request_id_plan(selected, context);
    let shape_sensitive = has_trait(shape, "smithy.api#sensitive");
    writeln!(
        output,
        "{padding}impl ::std::fmt::Debug for {} {{",
        rust_type_name(name)
    )
    .unwrap();
    writeln!(
        output,
        "{padding}    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {{"
    )
    .unwrap();
    writeln!(
        output,
        "{padding}        let mut formatter = f.debug_struct({:?});",
        rust_type_name(name)
    )
    .unwrap();
    for (member_name, member) in members(shape) {
        let field = names::rust_identifier(&member_name);
        let debug_field = field.strip_prefix("r#").unwrap_or(&field);
        let value =
            if shape_sensitive || value_should_redact(selected, member, &mut BTreeSet::new()) {
                "&\"*** Sensitive Data Redacted ***\"".to_owned()
            } else {
                format!("&self.{field}")
            };
        writeln!(
            output,
            "{padding}        formatter.field({debug_field:?}, {value});"
        )
        .unwrap();
    }
    if request_id_plan.extended {
        writeln!(
            output,
            "{padding}        formatter.field(\"_extended_request_id\", &self._extended_request_id);"
        )
        .unwrap();
    }
    if request_id_plan.standard {
        writeln!(
            output,
            "{padding}        formatter.field(\"_request_id\", &self._request_id);"
        )
        .unwrap();
    }
    writeln!(output, "{padding}        formatter.finish()").unwrap();
    writeln!(output, "{padding}    }}").unwrap();
    writeln!(output, "{padding}}}").unwrap();
}

fn render_error_impls(
    output: &mut String,
    selected: &SelectedModel,
    shape: &Value,
    name: &str,
    context: &Context,
) {
    let rust_name = rust_type_name(name);
    let consumer_namespace = context.consumer_namespace();
    let request_id_plan = request_id_plan(selected);
    let error_type_path = if consumer_namespace {
        rust_name.clone()
    } else {
        format!("crate::types::error::{rust_name}")
    };
    let message_name = error_message_member(shape)
        .map(|(name, _)| names::rust_identifier(&name))
        .unwrap_or_else(|| "message".to_owned());

    writeln!(output, "impl ::std::fmt::Display for {rust_name} {{").unwrap();
    writeln!(
        output,
        "    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{"
    )
    .unwrap();
    writeln!(output, "        ::std::write!(f, {rust_name:?})?;").unwrap();
    writeln!(
        output,
        "        if let ::std::option::Option::Some(inner_1) = &self.{message_name} {{"
    )
    .unwrap();
    writeln!(output, "            {{").unwrap();
    writeln!(
        output,
        "                ::std::write!(f, \": {{inner_1}}\")?;"
    )
    .unwrap();
    writeln!(output, "            }}").unwrap();
    writeln!(output, "        }}").unwrap();
    writeln!(output, "        Ok(())").unwrap();
    writeln!(output, "    }}").unwrap();
    writeln!(output, "}}").unwrap();
    writeln!(output, "impl ::std::error::Error for {rust_name} {{}}").unwrap();

    if request_id_plan.extended {
        let trait_path = if consumer_namespace {
            "super::super::s3_request_id::RequestIdExt"
        } else {
            "crate::s3_request_id::RequestIdExt"
        };
        writeln!(
            output,
            "impl {trait_path} for {error_type_path} {{\n    fn extended_request_id(&self) -> Option<&str> {{\n        use ::aws_smithy_types::error::metadata::ProvideErrorMetadata;\n        self.meta().extended_request_id()\n    }}\n}}"
        )
        .unwrap();
    }
    if request_id_plan.standard {
        writeln!(
            output,
            "impl ::aws_types::request_id::RequestId for {error_type_path} {{\n    fn request_id(&self) -> Option<&str> {{\n        use ::aws_smithy_types::error::metadata::ProvideErrorMetadata;\n        self.meta().request_id()\n    }}\n}}"
        )
        .unwrap();
    }
    writeln!(
        output,
        "impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for {rust_name} {{\n    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {{\n        &self.meta\n    }}\n}}"
    )
    .unwrap();
}

fn render_type_builder(
    output: &mut String,
    selected: &SelectedModel,
    shape: &Value,
    name: &str,
    context: Context,
    indent: usize,
) {
    let rust_name = rust_type_name(name);
    let padding = " ".repeat(indent);
    let inner = " ".repeat(indent + 4);
    let builder_path = builder_type_path(&context, &rust_name);
    let value_path = value_type_path(&context, &rust_name);
    let is_error = is_error_context(&context);
    let request_id_plan = output_request_id_plan(selected, &context);
    writeln!(output, "{padding}impl {rust_name} {{").unwrap();
    writeln!(
        output,
        "{inner}/// Creates a new builder-style object to manufacture [`{rust_name}`]({value_path})."
    )
    .unwrap();
    writeln!(
        output,
        "{inner}pub fn builder() -> {builder_path} {{\n{inner}    {builder_path}::default()\n{inner}}}"
    )
    .unwrap();
    writeln!(output, "{padding}}}").unwrap();
    writeln!(output).unwrap();
    writeln!(
        output,
        "{padding}/// A builder for [`{rust_name}`]({value_path})."
    )
    .unwrap();
    let builder_derives =
        "::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug";
    let mut excluded_derives = Vec::new();
    if structure_has_streaming_member(shape) {
        excluded_derives.extend(["::std::clone::Clone", "::std::cmp::PartialEq"]);
    }
    if structure_has_sensitive_member(selected, shape) {
        excluded_derives.push("::std::fmt::Debug");
    }
    let builder_derives = filter_derives(builder_derives, &excluded_derives);
    writeln!(output, "{padding}#[derive({builder_derives})]").unwrap();
    writeln!(output, "{padding}#[non_exhaustive]").unwrap();
    if members(shape).is_empty()
        && !is_error
        && !request_id_plan.standard
        && !request_id_plan.extended
    {
        writeln!(output, "{padding}pub struct {rust_name}Builder {{}}").unwrap();
        writeln!(output, "{padding}impl {rust_name}Builder {{").unwrap();
        writeln!(
            output,
            "{inner}/// Consumes the builder and constructs a [`{rust_name}`]({value_path})."
        )
        .unwrap();
        if operation_input(&context) {
            writeln!(
                output,
                "{inner}pub fn build(self) -> ::std::result::Result<{value_path}, {}> {{",
                build_error_type(&context)
            )
            .unwrap();
            writeln!(
                output,
                "{inner}    ::std::result::Result::Ok({value_path} {{}})"
            )
            .unwrap();
        } else {
            writeln!(output, "{inner}pub fn build(self) -> {value_path} {{").unwrap();
            writeln!(output, "{inner}    {value_path} {{}}").unwrap();
        }
        writeln!(output, "{inner}}}").unwrap();
        writeln!(output, "{padding}}}").unwrap();
        return;
    }
    writeln!(output, "{padding}pub struct {rust_name}Builder {{").unwrap();
    for (member_name, member) in members(shape) {
        let field = names::rust_identifier(&member_name);
        let target = member_target(member)
            .map(|target| type_expr(selected, target, context.clone()))
            .unwrap_or_else(|| "::std::string::String".to_owned());
        writeln!(
            output,
            "{inner}pub(crate) {field}: ::std::option::Option<{target}>,"
        )
        .unwrap();
    }
    if is_error {
        writeln!(
            output,
            "{inner}meta: std::option::Option<::aws_smithy_types::error::ErrorMetadata>,"
        )
        .unwrap();
    }
    if request_id_plan.extended {
        writeln!(output, "{inner}_extended_request_id: Option<String>,").unwrap();
    }
    if request_id_plan.standard {
        writeln!(output, "{inner}_request_id: Option<String>,").unwrap();
    }
    writeln!(output, "{padding}}}").unwrap();
    writeln!(output, "{padding}impl {rust_name}Builder {{").unwrap();
    for (member_name, member) in members(shape) {
        let field = names::rust_identifier(&member_name);
        let field_method = field.strip_prefix("r#").unwrap_or(&field);
        let target = member_target(member)
            .map(|target| type_expr(selected, target, context.clone()))
            .unwrap_or_else(|| "::std::string::String".to_owned());
        let target_id = member_target(member).unwrap_or("smithy.api#String");
        if is_error && modeled_member_documentation(selected, member).is_none() {
            writeln!(
                output,
                "{inner}#[allow(missing_docs)] // documentation missing in model"
            )
            .unwrap();
        }
        if let Some(list_shape) = selected.model.shapes.get(target_id)
            && list_shape.get("type").and_then(Value::as_str) == Some("list")
        {
            let element_target = list_shape
                .get("member")
                .and_then(member_target)
                .unwrap_or("smithy.api#String");
            let element_type = type_expr(selected, element_target, context.clone());
            let argument = builder_argument_type(selected, element_target, &element_type);
            writeln!(
                output,
                "{inner}/// Appends an item to `{field_method}`.\n{inner}///\n{inner}/// To override the contents of this collection use [`set_{field_method}`](Self::set_{field_method}).\n{inner}///"
            )
            .unwrap();
            render_builder_docs(output, selected, member, &inner, false);
            render_deprecated_attribute(output, member, indent + 4);
            writeln!(
                output,
                "{inner}pub fn {field}(mut self, input: {argument}) -> Self {{\n{inner}    let mut v = self.{field}.unwrap_or_default();\n{inner}    v.push({});\n{inner}    self.{field} = ::std::option::Option::Some(v);\n{inner}    self\n{inner}}}",
                builder_argument_value(&argument, "input")
            )
            .unwrap();
        } else if let Some(map_shape) = selected.model.shapes.get(target_id)
            && map_shape.get("type").and_then(Value::as_str) == Some("map")
        {
            let key_target = map_shape
                .get("key")
                .and_then(member_target)
                .unwrap_or("smithy.api#String");
            let value_target = map_shape
                .get("value")
                .and_then(member_target)
                .unwrap_or("smithy.api#String");
            let key_type = type_expr(selected, key_target, context.clone());
            let value_type = type_expr(selected, value_target, context.clone());
            let key_argument = builder_argument_type(selected, key_target, &key_type);
            let value_argument = builder_argument_type(selected, value_target, &value_type);
            writeln!(
                output,
                "{inner}/// Adds a key-value pair to `{field_method}`.\n{inner}///\n{inner}/// To override the contents of this collection use [`set_{field_method}`](Self::set_{field_method}).\n{inner}///"
            )
            .unwrap();
            render_builder_docs(output, selected, member, &inner, false);
            render_deprecated_attribute(output, member, indent + 4);
            writeln!(
                output,
                "{inner}pub fn {field}(mut self, k: {key_argument}, v: {value_argument}) -> Self {{\n{inner}    let mut hash_map = self.{field}.unwrap_or_default();\n{inner}    hash_map.insert({}, {});\n{inner}    self.{field} = ::std::option::Option::Some(hash_map);\n{inner}    self\n{inner}}}",
                builder_argument_value(&key_argument, "k"),
                builder_argument_value(&value_argument, "v")
            )
            .unwrap();
        } else {
            let argument = builder_argument_type(selected, target_id, &target);
            render_builder_docs(output, selected, member, &inner, member_is_required(member));
            render_deprecated_attribute(output, member, indent + 4);
            writeln!(
                output,
                "{inner}pub fn {field}(mut self, input: {argument}) -> Self {{\n{inner}    self.{field} = ::std::option::Option::Some({});\n{inner}    self\n{inner}}}",
                builder_argument_value(&argument, "input")
            )
            .unwrap();
        }
        if is_error && modeled_member_documentation(selected, member).is_none() {
            writeln!(
                output,
                "{inner}#[allow(missing_docs)] // documentation missing in model"
            )
            .unwrap();
        }
        render_builder_docs(output, selected, member, &inner, false);
        render_deprecated_attribute(output, member, indent + 4);
        writeln!(
            output,
            "{inner}pub fn set_{field_method}(mut self, input: ::std::option::Option<{target}>) -> Self {{ self.{field} = input; self }}"
        )
        .unwrap();
        if is_error && modeled_member_documentation(selected, member).is_none() {
            writeln!(
                output,
                "{inner}#[allow(missing_docs)] // documentation missing in model"
            )
            .unwrap();
        }
        render_builder_docs(output, selected, member, &inner, false);
        render_deprecated_attribute(output, member, indent + 4);
        writeln!(
            output,
            "{inner}pub fn get_{field_method}(&self) -> &::std::option::Option<{target}> {{ &self.{field} }}"
        )
        .unwrap();
    }
    if request_id_plan.extended {
        writeln!(
            output,
            "{inner}pub(crate) fn _extended_request_id(mut self, extended_request_id: impl Into<String>) -> Self {{\n{inner}    self._extended_request_id = Some(extended_request_id.into());\n{inner}    self\n{inner}}}\n"
        )
        .unwrap();
        writeln!(
            output,
            "{inner}pub(crate) fn _set_extended_request_id(&mut self, extended_request_id: Option<String>) -> &mut Self {{\n{inner}    self._extended_request_id = extended_request_id;\n{inner}    self\n{inner}}}"
        )
        .unwrap();
    }
    if request_id_plan.standard {
        writeln!(
            output,
            "{inner}pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {{\n{inner}    self._request_id = Some(request_id.into());\n{inner}    self\n{inner}}}\n"
        )
        .unwrap();
        writeln!(
            output,
            "{inner}pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {{\n{inner}    self._request_id = request_id;\n{inner}    self\n{inner}}}"
        )
        .unwrap();
    }
    if is_error {
        output.push_str(&format!(
            "{inner}/// Sets error metadata\n{inner}pub fn meta(mut self, meta: ::aws_smithy_types::error::ErrorMetadata) -> Self {{\n{inner}    self.meta = Some(meta);\n{inner}    self\n{inner}}}\n\n{inner}/// Sets error metadata\n{inner}pub fn set_meta(&mut self, meta: std::option::Option<::aws_smithy_types::error::ErrorMetadata>) -> &mut Self {{\n{inner}    self.meta = meta;\n{inner}    self\n{inner}}}\n"
        ));
    }
    let required_members = members(shape)
        .into_iter()
        .filter(|(_, member)| {
            let target = member_target(member).unwrap_or("smithy.api#String");
            builder_member_is_required(selected, member, target, &context)
        })
        .map(|(member_name, _)| member_name)
        .collect::<Vec<_>>();
    let fallible_builder = operation_input(&context) || !required_members.is_empty();
    if !fallible_builder {
        writeln!(
            output,
            "{inner}/// Consumes the builder and constructs a [`{rust_name}`]({value_path})."
        )
        .unwrap();
        writeln!(output, "{inner}pub fn build(self) -> {value_path} {{").unwrap();
    } else {
        writeln!(
            output,
            "{inner}/// Consumes the builder and constructs a [`{rust_name}`]({value_path})."
        )
        .unwrap();
        if !required_members.is_empty() {
            writeln!(
                output,
                "{inner}/// This method will fail if any of the following fields are not set:"
            )
            .unwrap();
            let builder_path = builder_type_path(&context, &rust_name);
            for member_name in &required_members {
                let field_method = names::rust_identifier(member_name);
                let doc_field_method = names::rustdoc_identifier(&field_method);
                let field_link = if context.consumer_namespace() {
                    format!("Self::{doc_field_method}")
                } else {
                    format!("{builder_path}::{doc_field_method}")
                };
                writeln!(output, "{inner}/// - [`{field_method}`]({field_link})").unwrap();
            }
        }
        writeln!(
            output,
            "{inner}pub fn build(self) -> ::std::result::Result<{value_path}, {}> {{",
            build_error_type(&context)
        )
        .unwrap();
        writeln!(
            output,
            "{inner}    ::std::result::Result::Ok({value_path} {{"
        )
        .unwrap();
    }
    if !fallible_builder {
        writeln!(output, "{inner}    {value_path} {{").unwrap();
    }
    for (member_name, _) in members(shape) {
        let field = names::rust_identifier(&member_name);
        let member = members(shape)
            .into_iter()
            .find(|(name, _)| name == &member_name)
            .map(|(_, member)| member)
            .expect("member exists");
        let target = member_target(member).unwrap_or("smithy.api#String");
        if is_streaming_target(target) {
            writeln!(
                output,
                "{inner}        {field}: self.{field}.unwrap_or_default(),"
            )
            .unwrap();
        } else if builder_member_is_required(selected, member, target, &context) {
            writeln!(
                output,
                "{inner}        {field}: self.{field}.ok_or_else(|| {}::missing_field({field:?}, {message:?}))?,",
                build_error_type(&context),
                message = format!("{field} was not specified but it is required when building {rust_name}")
            )
            .unwrap();
        } else {
            writeln!(output, "{inner}        {field}: self.{field},").unwrap();
        }
    }
    if request_id_plan.extended {
        writeln!(
            output,
            "{inner}        _extended_request_id: self._extended_request_id,"
        )
        .unwrap();
    }
    if request_id_plan.standard {
        writeln!(output, "{inner}        _request_id: self._request_id,").unwrap();
    }
    if is_error {
        writeln!(
            output,
            "{inner}        meta: self.meta.unwrap_or_default(),"
        )
        .unwrap();
    }
    if !fallible_builder {
        writeln!(output, "{inner}    }}").unwrap();
        writeln!(output, "{inner}}}").unwrap();
    } else {
        writeln!(output, "{inner}    }})").unwrap();
        writeln!(output, "{inner}}}").unwrap();
    }
    writeln!(output, "{padding}}}").unwrap();
    if structure_has_sensitive_member(selected, shape) {
        render_sensitive_debug_impl_for_builder(
            output, selected, shape, &rust_name, &context, indent,
        );
    }
}

fn render_sensitive_debug_impl_for_builder(
    output: &mut String,
    selected: &SelectedModel,
    shape: &Value,
    name: &str,
    context: &Context,
    indent: usize,
) {
    let padding = " ".repeat(indent);
    let request_id_plan = output_request_id_plan(selected, context);
    let shape_sensitive = has_trait(shape, "smithy.api#sensitive");
    let builder_name = format!("{name}Builder");
    writeln!(
        output,
        "{padding}impl ::std::fmt::Debug for {builder_name} {{"
    )
    .unwrap();
    writeln!(
        output,
        "{padding}    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {{"
    )
    .unwrap();
    writeln!(
        output,
        "{padding}        let mut formatter = f.debug_struct({builder_name:?});"
    )
    .unwrap();
    for (member_name, member) in members(shape) {
        let field = names::rust_identifier(&member_name);
        let debug_field = field.strip_prefix("r#").unwrap_or(&field);
        let value =
            if shape_sensitive || value_should_redact(selected, member, &mut BTreeSet::new()) {
                "&\"*** Sensitive Data Redacted ***\"".to_owned()
            } else {
                format!("&self.{field}")
            };
        writeln!(
            output,
            "{padding}        formatter.field({debug_field:?}, {value});"
        )
        .unwrap();
    }
    if request_id_plan.extended {
        writeln!(
            output,
            "{padding}        formatter.field(\"_extended_request_id\", &self._extended_request_id);"
        )
        .unwrap();
    }
    if request_id_plan.standard {
        writeln!(
            output,
            "{padding}        formatter.field(\"_request_id\", &self._request_id);"
        )
        .unwrap();
    }
    writeln!(output, "{padding}        formatter.finish()").unwrap();
    writeln!(output, "{padding}    }}").unwrap();
    writeln!(output, "{padding}}}").unwrap();
}

fn render_union(
    output: &mut String,
    selected: &SelectedModel,
    shape: &Value,
    name: &str,
    context: &Context,
) {
    let rust_name = rust_type_name(name);
    if let Some(documentation) = documentation(shape) {
        render_doc_lines(output, &documentation, 4);
    }
    render_deprecated_attribute(output, shape, 4);
    output.push_str(
        "    #[non_exhaustive]\n    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]\n",
    );
    writeln!(output, "    pub enum {rust_name} {{").unwrap();
    let ordered_members = sorted_members(shape);
    for (member_name, member) in &ordered_members {
        if let Some(documentation) = modeled_member_documentation(selected, member) {
            render_doc_lines(output, &documentation, 8);
        }
        render_deprecated_attribute(output, member, 8);
        let variant = rust_type_name(member_name);
        let target = member_target(member).unwrap_or("smithy.api#Unit");
        if target == "smithy.api#Unit" {
            writeln!(output, "        {variant},").unwrap();
        } else {
            let target_type = type_expr(selected, target, context.clone());
            writeln!(output, "        {variant}({target_type}),").unwrap();
        }
    }
    output.push_str(
        "        /// The __BT__Unknown__BT__ variant represents cases where new union variant was received. Consider upgrading the SDK to the latest available version.\n        /// An unknown enum variant\n        ///\n        /// _Note: If you encounter this error, consider upgrading your SDK to the latest version._\n        /// The __BT__Unknown__BT__ variant represents cases where the server sent a value that wasn't recognized\n        /// by the client. This can happen when the server adds new functionality, but the client has not been updated.\n        /// To investigate this, consider turning on debug logging to print the raw HTTP response.\n        #[non_exhaustive]\n        Unknown,\n    }\n",
    );
    writeln!(output, "impl {rust_name} {{").unwrap();
    let union_path = if context.consumer_namespace() {
        format!("self::{rust_name}")
    } else {
        format!("crate::types::{rust_name}")
    };
    for (member_name, member) in &ordered_members {
        let variant = rust_type_name(member_name);
        let function = names::rust_identifier(&names::snake_case(member_name));
        let target = member_target(member).unwrap_or("smithy.api#Unit");
        let target_name = client_documentation_type(selected, target);
        let target_type = type_expr(selected, target, context.clone());
        if ordered_members.len() == 1 {
            output.push_str("    #[allow(irrefutable_let_patterns)]\n");
        }
        if target == "smithy.api#Unit" {
            writeln!(
                output,
                "    /// Tries to convert the enum instance into [__BT__{variant}__BT__]({union_path}::{variant}), extracting the inner __BT__()__BT__.\n    /// Returns __BT__Err(&Self)__BT__ if it can't be converted.\n    pub fn as_{function}(&self) -> ::std::result::Result<(), &Self> {{\n        if let {rust_name}::{variant} = &self {{\n            ::std::result::Result::Ok(())\n        }} else {{\n            ::std::result::Result::Err(self)\n        }}\n    }}\n    /// Returns true if this is a [__BT__{variant}__BT__]({union_path}::{variant}).\n    pub fn is_{function}(&self) -> bool {{\n        self.as_{function}().is_ok()\n    }}"
            )
            .unwrap();
        } else {
            writeln!(
                output,
                "    /// Tries to convert the enum instance into [__BT__{variant}__BT__]({union_path}::{variant}), extracting the inner [__BT__{target_name}__BT__]({target_type}).\n    /// Returns __BT__Err(&Self)__BT__ if it can't be converted.\n    pub fn as_{function}(&self) -> ::std::result::Result<&{target_type}, &Self> {{\n        if let {rust_name}::{variant}(val) = &self {{\n            ::std::result::Result::Ok(val)\n        }} else {{\n            ::std::result::Result::Err(self)\n        }}\n    }}\n    /// Returns true if this is a [__BT__{variant}__BT__]({union_path}::{variant}).\n    pub fn is_{function}(&self) -> bool {{\n        self.as_{function}().is_ok()\n    }}"
            )
            .unwrap();
        }
    }
    output.push_str(
        "    /// Returns true if the enum instance is the __BT__Unknown__BT__ variant.\n    pub fn is_unknown(&self) -> bool {\n        matches!(self, Self::Unknown)\n    }\n}\n\n",
    );
    *output = output.replace("__BT__", "\x60");
}

fn render_enum(
    output: &mut String,
    shape: &Value,
    name: &str,
    context: &Context,
    consumer_namespace: bool,
) {
    let rust_name = rust_type_name(name);
    let ordered_members = sorted_members(shape);
    let support_prefix = if consumer_namespace {
        match context {
            Context::Types { .. } => "super::",
            Context::Error { .. } => "super::super::",
            Context::Operation { .. } | Context::Builder { .. } => "super::super::super::",
        }
    } else {
        "crate::"
    };
    let unknown_value_type =
        format!("{support_prefix}primitives::sealed_enum_unknown::UnknownVariantValue");
    let unknown_error_type = format!("{support_prefix}error::UnknownVariantError");
    let documented = shape
        .get("traits")
        .and_then(Value::as_object)
        .and_then(|traits| traits.get("smithy.api#documentation"))
        .and_then(Value::as_str)
        .is_some();
    let lower_name = name.to_ascii_lowercase();
    writeln!(
        output,
        "    /// When writing a match expression against `{rust_name}`, it is important to ensure"
    )
    .unwrap();
    for line in [
        "    /// your code is forward-compatible. That is, if a match arm handles a case for a",
        "    /// feature that is supported by the service but has not been represented as an enum",
        "    /// variant in a current version of SDK, your code should continue to work when you",
        "    /// upgrade SDK to a future version in which the enum does include a variant for that",
        "    /// feature.",
        "    ///",
        "    /// Here is an example of how you can make a match expression forward-compatible:",
        "    ///",
        "    /// ```text",
    ] {
        output.push_str(line);
        output.push('\n');
    }
    writeln!(output, "    /// # let {lower_name} = unimplemented!();").unwrap();
    writeln!(output, "    /// match {lower_name} {{").unwrap();
    for member_name in ordered_members.keys() {
        let variant = rust_type_name(member_name);
        writeln!(
            output,
            "    ///     {rust_name}::{variant} => {{ /* ... */ }},"
        )
        .unwrap();
    }
    for line in [
        "    ///     other @ _ if other.as_str() == \"NewFeature\" => { /* handles a case for `NewFeature` */ },",
        "    ///     _ => { /* ... */ },",
        "    /// }",
        "    /// ```",
        "    /// The above code demonstrates that when `PLACEHOLDER` represents",
        "    /// `NewFeature`, the execution path will lead to the second last match arm,",
        "    /// even though the enum does not contain a variant `ENUM::NewFeature`",
        "    /// in the current version of SDK. The reason is that the variable `other`,",
        "    /// created by the `@` operator, is bound to",
        "    /// `ENUM::Unknown(UnknownVariantValue(\"NewFeature\".to_owned()))`",
        "    /// and calling `as_str` on it yields `\"NewFeature\"`.",
        "    /// This match expression is forward-compatible when executed with a newer",
        "    /// version of SDK where the variant `ENUM::NewFeature` is defined.",
        "    /// Specifically, when `PLACEHOLDER` represents `NewFeature`,",
        "    /// the execution path will hit the second last match arm as before by virtue of",
        "    /// calling `as_str` on `ENUM::NewFeature` also yielding `\"NewFeature\"`.",
        "    ///",
        "    /// Explicitly matching on the `Unknown` variant should",
        "    /// be avoided for two reasons:",
        "    /// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.",
        "    /// - It might inadvertently shadow other intended match arms.",
        "    ///",
    ] {
        let line = line
            .replace("PLACEHOLDER", &lower_name)
            .replace("ENUM", &rust_name);
        output.push_str(&line);
        output.push('\n');
    }
    if documented {
        if let Some(documentation) = shape
            .get("traits")
            .and_then(|traits| traits.get("smithy.api#documentation"))
            .and_then(Value::as_str)
        {
            for line in documentation.lines() {
                writeln!(output, "    /// {line}").unwrap();
            }
        }
    } else {
        output.push_str("    #[allow(missing_docs)] // documentation missing in model\n");
    }
    output.push_str("    #[non_exhaustive]\n");
    output.push_str("    #[derive(\n");
    output.push_str(
        "        ::std::clone::Clone, ::std::cmp::Eq, ::std::cmp::Ord, ::std::cmp::PartialEq, ::std::cmp::PartialOrd, ::std::fmt::Debug, ::std::hash::Hash,\n",
    );
    output.push_str("    )]\n");
    writeln!(output, "    pub enum {rust_name} {{").unwrap();
    for (member_name, member) in &ordered_members {
        let variant = rust_type_name(member_name);
        let member_documented = member
            .get("traits")
            .and_then(Value::as_object)
            .and_then(|traits| traits.get("smithy.api#documentation"))
            .and_then(Value::as_str)
            .is_some();
        if !member_documented {
            output.push_str("        #[allow(missing_docs)] // documentation missing in model\n");
        }
        writeln!(output, "        {variant},").unwrap();
    }
    output.push_str(
        "        /// `Unknown` contains new variants that have been added since this code was generated.\n",
    );
    output.push_str(
        "        #[deprecated(note = \"Don't directly match on `Unknown`. See the docs on this enum for the correct way to handle unknown variants.\")]\n",
    );
    writeln!(output, "        Unknown({unknown_value_type}),").unwrap();
    output.push_str("    }\n");
    writeln!(
        output,
        "    impl ::std::convert::From<&str> for {rust_name} {{"
    )
    .unwrap();
    output.push_str("        fn from(s: &str) -> Self {\n            match s {\n");
    for (member_name, member) in &ordered_members {
        let value = enum_value(member, member_name);
        writeln!(
            output,
            "                {value:?} => {rust_name}::{},",
            rust_type_name(member_name)
        )
        .unwrap();
    }
    output.push_str("                other => ");
    writeln!(
        output,
        "{rust_name}::Unknown({unknown_value_type}(other.to_owned())),"
    )
    .unwrap();
    output.push_str("            }\n        }\n    }\n");
    writeln!(
        output,
        "    impl ::std::str::FromStr for {rust_name} {{\n        type Err = ::std::convert::Infallible;\n\n        fn from_str(s: &str) -> ::std::result::Result<Self, <Self as ::std::str::FromStr>::Err> {{\n            ::std::result::Result::Ok({rust_name}::from(s))\n        }}\n    }}"
    )
    .unwrap();
    writeln!(output, "    impl {rust_name} {{").unwrap();
    output.push_str("        /// Returns the `&str` value of the enum member.\n");
    output.push_str("        pub fn as_str(&self) -> &str {\n");
    output.push_str("            match self {\n");
    for (member_name, member) in &ordered_members {
        let value = enum_value(member, member_name);
        writeln!(
            output,
            "                {rust_name}::{} => {value:?},",
            rust_type_name(member_name)
        )
        .unwrap();
    }
    writeln!(
        output,
        "                {rust_name}::Unknown(value) => value.as_str(),"
    )
    .unwrap();
    output.push_str("            }\n");
    output.push_str("        }\n");
    output.push_str("        /// Returns all the `&str` representations of the enum members.\n");
    output.push_str("        pub const fn values() -> &'static [&'static str] {\n");
    writeln!(
        output,
        "            &[{}]",
        ordered_members
            .iter()
            .map(|(member_name, member)| format!("{:?}", enum_value(member, member_name)))
            .collect::<Vec<_>>()
            .join(", ")
    )
    .unwrap();
    output.push_str("        }\n");
    output.push_str("    }\n");
    writeln!(
        output,
        "    impl ::std::convert::AsRef<str> for {rust_name} {{\n        fn as_ref(&self) -> &str {{\n            self.as_str()\n        }}\n    }}"
    )
    .unwrap();
    writeln!(output, "    impl {rust_name} {{").unwrap();
    output.push_str("        /// Parses the enum value while disallowing unknown variants.\n");
    output.push_str("        ///\n");
    output.push_str("        /// Unknown variants will result in an error.\n");
    writeln!(
        output,
        "        pub fn try_parse(value: &str) -> ::std::result::Result<Self, {unknown_error_type}> {{"
    )
    .unwrap();
    output.push_str("            match Self::from(value) {\n");
    output.push_str("                #[allow(deprecated)]\n");
    writeln!(
        output,
        "                Self::Unknown(_) => ::std::result::Result::Err({unknown_error_type}::new(value)),"
    )
    .unwrap();
    output.push_str("                known => Ok(known),\n");
    output.push_str("            }\n");
    output.push_str("        }\n");
    output.push_str("    }\n");
    writeln!(
        output,
        "    impl ::std::fmt::Display for {rust_name} {{\n        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {{\n            match self {{"
    )
    .unwrap();
    for (member_name, member) in &ordered_members {
        let value = enum_value(member, member_name);
        writeln!(
            output,
            "                {rust_name}::{} => write!(f, {value:?}),",
            rust_type_name(member_name)
        )
        .unwrap();
    }
    writeln!(
        output,
        "                {rust_name}::Unknown(value) => write!(f, \"{{value}}\"),\n            }}\n        }}\n    }}\n"
    )
    .unwrap();
}

fn enum_value(member: &Value, fallback: &str) -> String {
    member
        .get("traits")
        .and_then(|traits| traits.get("smithy.api#enumValue"))
        .and_then(Value::as_str)
        .unwrap_or(fallback)
        .to_owned()
}

fn render_client_file(service_key: &str, selected: &SelectedModel) -> String {
    let mut output = String::new();
    output.push_str(&render_aws_runtime());
    render_client(&mut output, service_key, selected);
    output
}

fn has_presignable_operations(selected: &SelectedModel) -> bool {
    selected.operations.iter().any(|operation_name| {
        operation_shape(selected, operation_name)
            .is_some_and(|operation| operation_is_presignable(selected, operation))
    })
}

fn has_idempotency_operations(selected: &SelectedModel) -> bool {
    selected.operations.iter().any(|operation_name| {
        operation_shape(selected, operation_name)
            .and_then(|operation| operation.get("input"))
            .and_then(target_value)
            .and_then(|id| selected.model.shapes.get(id))
            .is_some_and(|shape| {
                members(shape)
                    .into_iter()
                    .any(|(_, member)| has_trait(member, "smithy.api#idempotencyToken"))
            })
    })
}

fn render_client_customize_internal_file(selected: &SelectedModel) -> String {
    let mut output = include_str!("../assets/client_customize_internal.rs").to_owned();
    if has_presignable_operations(selected) {
        output.push_str(
            "\npub trait CustomizablePresigned<E>: ::std::marker::Send + ::std::marker::Sync {\n    fn presign(\n        self,\n        config_override: crate::config::Builder,\n        presigning_config: crate::presigning::PresigningConfig,\n    ) -> BoxFuture<SendResult<crate::presigning::PresignedRequest, E>>;\n}\n",
        );
    }
    output
}

fn render_client_customize_file(selected: &SelectedModel) -> String {
    let mut output = include_str!("../assets/client_customize.rs").to_owned();
    if has_presignable_operations(selected) {
        let mut payload_signing_overrides = String::new();
        let mut operation_names = selected.operations.clone();
        operation_names.sort();
        for operation_name in operation_names {
            let Some(operation) = operation_shape(selected, &operation_name) else {
                continue;
            };
            if !operation_is_presignable(selected, operation)
                || operation_http_method(operation) != "PUT"
            {
                continue;
            }
            let module = names::snake_case(&operation_name);
            let operation_type = rust_type_name(&operation_name);
            writeln!(
                payload_signing_overrides,
                "impl<E, B> CustomizableOperation<crate::operation::{module}::{operation_type}Output, E, B> {{\n    /// Disable payload signing for this request.\n    ///\n    /// **WARNING:** This is an advanced feature that removes\n    /// the cost of signing a request payload by removing a data\n    /// integrity check. Not all services/operations support\n    /// this feature.\n    pub fn disable_payload_signing(self) -> Self {{\n        self.runtime_plugin(::aws_runtime::auth::PayloadSigningOverrideRuntimePlugin::unsigned())\n    }}\n}}\n"
            )
            .unwrap();
        }
        output = output.replacen(
            "/// `CustomizableOperation`",
            &format!("{payload_signing_overrides}/// `CustomizableOperation`"),
            1,
        );
        let presigned_method = "\n\n    /// Sends the request and returns the response.\n    #[allow(unused_mut)]\n    pub async fn presigned(\n        mut self,\n        presigning_config: crate::presigning::PresigningConfig,\n    ) -> ::std::result::Result<crate::presigning::PresignedRequest, crate::error::SdkError<E>>\n    where\n        E: std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static,\n        B: crate::client::customize::internal::CustomizablePresigned<E>,\n    {\n        self.execute(move |sender, conf| sender.presign(conf, presigning_config)).await\n    }";
        output = output.replace(
            "\n}\n\npub(crate) mod internal;",
            &format!("{presigned_method}\n}}\n\npub(crate) mod internal;"),
        );
    }
    output
}

fn render_presigning_file() -> String {
    let mut output = String::new();
    client_operation_header(&mut output);
    output.push_str(include_str!("../assets/presigning.rs"));
    output
}

fn render_presigning_interceptors_file() -> String {
    let mut output = String::new();
    client_operation_header(&mut output);
    output.push_str(include_str!("../assets/presigning_interceptors.rs"));
    output
}

fn render_serialization_settings_file() -> String {
    let mut output = String::new();
    client_operation_header(&mut output);
    output.push_str(include_str!("../assets/serialization_settings.rs"));
    output
}

fn render_client(output: &mut String, service_key: &str, selected: &SelectedModel) {
    header(output);
    output.push_str(
        "#[derive(Clone, Debug, Default)]\n\
             pub struct Client {\n\
                 config: Config,\n\
                 http: transport::HttpClient,\n\
             }\n\
             impl Client {\n\
                 pub fn new(config: &Config) -> Self {\n\
                     Self { config: config.clone(), http: transport::HttpClient::new() }\n\
                 }\n\
                 pub fn config(&self) -> &Config { &self.config }\n\
                 pub(crate) async fn request(\n\
                     &self,\n\
                     method: transport::Method,\n\
                     path: &str,\n\
                     headers: &[(&str, &str)],\n\
                     body: &[u8],\n\
                 ) -> ::std::result::Result<transport::Response, ::std::string::String> {\n\
                     let url = format!(\"{}{}\", self.config.endpoint_url.trim_end_matches('/'), path);\n\
                     self.http.request(method, &url, headers, body).await\n\
                 }\n\
             }\n\n",
    );
    let mut operations = selected.operations.clone();
    operations.sort();
    for operation in operations {
        let module = names::snake_case(&operation);
        writeln!(
            output,
            "include!(concat!(env!(\"OUT_DIR\"), \"/generated/{service_key}/src/client/{module}.rs\"));"
        )
        .unwrap();
    }
}

fn render_client_operation_file(
    selected: &SelectedModel,
    operation: &str,
    consumer_namespace: bool,
) -> String {
    let module = names::snake_case(operation);
    let rust_operation = rust_type_name(operation);
    let operation_symbol = operation;
    let mut output = String::new();
    client_operation_header(&mut output);
    if !consumer_namespace {
        let operation_shape =
            operation_shape(selected, operation).expect("selected operation exists");
        let input_shape = operation_shape
            .get("input")
            .and_then(target_value)
            .and_then(|id| selected.model.shapes.get(id));
        let output_shape = operation_shape
            .get("output")
            .and_then(target_value)
            .and_then(|id| selected.model.shapes.get(id));
        let input_members = input_shape.map(members).unwrap_or_default();
        output.push_str("impl super::Client {\n");
        writeln!(
            output,
            "    /// Constructs a fluent builder for the [`{operation_symbol}`](crate::operation::{module}::builders::{operation_symbol}FluentBuilder) operation."
        )
        .unwrap();
        if operation_is_paginated(operation_shape) {
            writeln!(
                output,
                "    /// This operation supports pagination; See [`into_paginator()`](crate::operation::{module}::builders::{operation_symbol}FluentBuilder::into_paginator)."
            )
            .unwrap();
        }
        writeln!(output, "    ///").unwrap();
        if input_members.is_empty() {
            writeln!(
                output,
                "    /// - The fluent builder takes no input, just [`send`](crate::operation::{module}::builders::{operation_symbol}FluentBuilder::send) it."
            )
            .unwrap();
        } else {
            output.push_str("    /// - The fluent builder is configurable:\n");
            for (name, member) in input_members {
                let field = names::rust_identifier(&name);
                let field_method = field.strip_prefix("r#").unwrap_or(&field);
                let target_id = member_target(member).unwrap_or("smithy.api#String");
                let target = client_documentation_type(selected, target_id);
                let argument = client_documentation_argument_types(selected, target_id);
                let required = member_is_required(member);
                let documentation = member_documentation(selected, member);
                writeln!(
                    output,
                    "    ///   - [`{field_method}({argument})`](crate::operation::{module}::builders::{operation_symbol}FluentBuilder::{field_method}) / [`set_{field_method}(Option<{target}>)`](crate::operation::{module}::builders::{operation_symbol}FluentBuilder::set_{field_method}):<br>required: **{required}**<br>{documentation}<br>"
                )
                .unwrap();
            }
        }
        let output_name = format!("{rust_operation}Output");
        if let Some(output_shape) = output_shape {
            let output_members = members(output_shape);
            if output_members.is_empty() {
                writeln!(
                    output,
                    "    /// - On success, responds with [`{output_name}`](crate::operation::{module}::{output_name})"
                )
                .unwrap();
            } else {
                writeln!(
                    output,
                    "    /// - On success, responds with [`{output_name}`](crate::operation::{module}::{output_name}) with field(s):"
                )
                .unwrap();
                for (name, member) in output_members {
                    let field = names::rust_identifier(&name);
                    let target_id = member_target(member).unwrap_or("smithy.api#String");
                    let target = client_documentation_type(selected, target_id);
                    let field_type = if is_streaming_target(target_id)
                        || member_is_effectively_required(selected, member, target_id)
                    {
                        target
                    } else {
                        format!("Option<{target}>")
                    };
                    writeln!(
                        output,
                        "    ///   - [`{field}({field_type})`](crate::operation::{module}::{output_name}::{field}): {}",
                        member_documentation(selected, member)
                    )
                    .unwrap();
                }
            }
        } else {
            writeln!(
                output,
                "    /// - On success, responds with [`{output_name}`](crate::operation::{module}::{output_name})"
            )
            .unwrap();
        }
        writeln!(
            output,
            "    /// - On failure, responds with [`SdkError<{operation_symbol}Error>`](crate::operation::{module}::{operation_symbol}Error)"
        )
        .unwrap();
        let fluent_builder_path =
            format!("crate::operation::{module}::builders::{operation_symbol}FluentBuilder");
        let method_signature = format!("    pub fn {module}(&self) -> {fluent_builder_path}");
        if method_signature.len() > 160 {
            writeln!(
                output,
                "    pub fn {module}(\n        &self,\n    ) -> {fluent_builder_path} {{"
            )
            .unwrap();
        } else {
            writeln!(output, "{method_signature} {{").unwrap();
        }
        writeln!(
            output,
            "        crate::operation::{module}::builders::{operation_symbol}FluentBuilder::new(self.handle.clone())"
        )
        .unwrap();
        output.push_str("    }\n}\n");
        return output;
    }
    writeln!(
        output,
        "impl Client {{\n    pub fn {module}(&self) -> operation::{module}::{operation_symbol}FluentBuilder {{ operation::{module}::{operation_symbol}FluentBuilder::with_client(self.clone()) }}\n}}\n"
    )
    .unwrap();
    output
}

fn compact_documentation(value: &Value) -> String {
    value
        .get("traits")
        .and_then(Value::as_object)
        .and_then(|traits| traits.get("smithy.api#documentation"))
        .and_then(Value::as_str)
        .map(normalize_client_documentation)
        .filter(|documentation| !documentation.is_empty())
        .unwrap_or_else(|| "(undocumented)".to_owned())
}

fn member_documentation(selected: &SelectedModel, member: &Value) -> String {
    if member
        .get("traits")
        .and_then(Value::as_object)
        .and_then(|traits| traits.get("smithy.api#documentation"))
        .and_then(Value::as_str)
        .is_some()
    {
        return compact_documentation(member);
    }
    member_target(member)
        .and_then(|target| selected.model.shapes.get(target))
        .map(compact_documentation)
        .unwrap_or_else(|| "(undocumented)".to_owned())
}

fn client_documentation_type(selected: &SelectedModel, target: &str) -> String {
    match target {
        "smithy.api#String" => "String".to_owned(),
        "smithy.api#Blob" => "Blob".to_owned(),
        "smithy.api#Timestamp" => "DateTime".to_owned(),
        "smithy.api#Document" => "Document".to_owned(),
        "smithy.api#Boolean" => "bool".to_owned(),
        "smithy.api#Byte" => "i8".to_owned(),
        "smithy.api#Short" => "i16".to_owned(),
        "smithy.api#Integer" => "i32".to_owned(),
        "smithy.api#Long" => "i64".to_owned(),
        "smithy.api#Float" => "f32".to_owned(),
        "smithy.api#Double" => "f64".to_owned(),
        _ if terminal(target) == "StreamingBlob" => "ByteStream".to_owned(),
        _ => {
            let Some(shape) = selected.model.shapes.get(target) else {
                return rust_type_name(terminal(target));
            };
            match shape.get("type").and_then(Value::as_str) {
                Some("list") => {
                    let element = shape
                        .get("member")
                        .and_then(member_target)
                        .map(|target| client_documentation_type(selected, target))
                        .unwrap_or_else(|| "String".to_owned());
                    format!("Vec::<{element}>")
                }
                Some("map") => {
                    let key = shape
                        .get("key")
                        .and_then(member_target)
                        .map(|target| client_documentation_type(selected, target))
                        .unwrap_or_else(|| "String".to_owned());
                    let value = shape
                        .get("value")
                        .and_then(member_target)
                        .map(|target| client_documentation_type(selected, target))
                        .unwrap_or_else(|| "String".to_owned());
                    format!("HashMap::<{key}, {value}>")
                }
                Some(
                    "string" | "integer" | "long" | "short" | "byte" | "float" | "double"
                    | "boolean" | "blob" | "timestamp" | "document",
                ) => client_documentation_primitive(
                    shape
                        .get("type")
                        .and_then(Value::as_str)
                        .unwrap_or("string"),
                ),
                _ => rust_type_name(terminal(target)),
            }
        }
    }
}

fn client_documentation_primitive(kind: &str) -> String {
    match kind {
        "boolean" => "bool",
        "byte" => "i8",
        "short" => "i16",
        "integer" => "i32",
        "long" => "i64",
        "float" => "f32",
        "double" => "f64",
        "blob" => "Blob",
        "timestamp" => "DateTime",
        "document" => "Document",
        _ => "String",
    }
    .to_owned()
}

fn client_documentation_argument_types(selected: &SelectedModel, target: &str) -> String {
    if let Some(shape) = selected.model.shapes.get(target) {
        match shape.get("type").and_then(Value::as_str) {
            Some("list") => {
                let element = shape
                    .get("member")
                    .and_then(member_target)
                    .unwrap_or("smithy.api#String");
                return client_documentation_argument_type(selected, element);
            }
            Some("map") => {
                let key = shape
                    .get("key")
                    .and_then(member_target)
                    .unwrap_or("smithy.api#String");
                let value = shape
                    .get("value")
                    .and_then(member_target)
                    .unwrap_or("smithy.api#String");
                return format!(
                    "{}, {}",
                    client_documentation_argument_type(selected, key),
                    client_documentation_argument_type(selected, value)
                );
            }
            _ => {}
        }
    }
    client_documentation_argument_type(selected, target)
}

fn client_documentation_argument_type(selected: &SelectedModel, target: &str) -> String {
    let rendered = client_documentation_type(selected, target);
    if is_string_type(target, selected.model.shapes.get(target)) {
        format!("impl Into<{rendered}>")
    } else {
        rendered
    }
}

#[derive(Clone)]
enum DocumentationToken {
    Tag(String),
    Text(String),
    Whitespace(String),
}

fn documentation_tokens(value: &str) -> Vec<DocumentationToken> {
    let mut tokens = Vec::new();
    let mut rest = value;
    while !rest.is_empty() {
        if rest.starts_with('<')
            && rest.chars().nth(1).is_some_and(|character| {
                character.is_ascii_alphabetic() || matches!(character, '/' | '!')
            })
        {
            if let Some(end) = rest.find('>') {
                tokens.push(DocumentationToken::Tag(rest[..=end].to_owned()));
                rest = &rest[end + 1..];
            } else {
                tokens.push(DocumentationToken::Text(rest.to_owned()));
                break;
            }
        } else {
            let end = if let Some(stripped) = rest.strip_prefix('<') {
                stripped.find('<').map_or(rest.len(), |end| end + 1)
            } else {
                rest.find('<').unwrap_or(rest.len())
            };
            let raw = &rest[..end];
            if raw.trim().is_empty() {
                tokens.push(DocumentationToken::Whitespace(raw.to_owned()));
            } else {
                tokens.push(DocumentationToken::Text(raw.to_owned()));
            }
            rest = &rest[end..];
        }
    }
    tokens
}

fn normalize_model_documentation(value: &str) -> String {
    let tokens = documentation_tokens(value);
    let mut output = String::with_capacity(value.len());
    let mut stack = Vec::<String>::new();
    let mut previous_tag = None::<(bool, String)>;
    let mut pending_whitespace = false;
    let mut pending_newline = false;

    for token in tokens {
        match token {
            DocumentationToken::Whitespace(text) => {
                pending_whitespace = true;
                pending_newline |= text
                    .chars()
                    .any(|character| matches!(character, '\n' | '\r'));
            }
            DocumentationToken::Tag(tag) => {
                let closing = tag.trim_start().starts_with("</");
                let (normalized_tag, name) = normalize_documentation_tag(&tag, closing, &stack);

                if closing {
                    if let Some(opening_index) = stack.iter().rposition(|current| current == &name)
                    {
                        while stack.len() > opening_index + 1 {
                            let unclosed = stack.pop().expect("opening tag exists");
                            if !documentation_is_inline(&unclosed)
                                && !previous_tag.as_ref().is_some_and(|(is_closing, previous)| {
                                    !is_closing && previous == &unclosed
                                })
                            {
                                documentation_newline(&mut output);
                            }
                            output.push_str(&format!("</{unclosed}>"));
                            previous_tag = Some((true, unclosed));
                        }
                    }
                    if ((!documentation_known_tag(&name) || pending_newline)
                        && documentation_pseudo_tag_in_stack(&stack))
                        || documentation_newline_before_close(&name, previous_tag.as_ref())
                    {
                        documentation_newline(&mut output);
                    } else if pending_whitespace
                        && documentation_space_before_tag(
                            &name,
                            true,
                            &stack,
                            previous_tag.as_ref(),
                        )
                    {
                        documentation_space(&mut output);
                    }
                    output.push_str(&normalized_tag);
                    if stack.last().is_some_and(|current| current == &name) {
                        stack.pop();
                    }
                    previous_tag = Some((true, name));
                } else {
                    if documentation_block_tag(&name)
                        || (pending_newline && documentation_pseudo_tag_in_stack(&stack))
                        || (!documentation_known_tag(&name)
                            && documentation_pseudo_tag_in_stack(&stack))
                        || (documentation_pseudo_tag_in_stack(&stack)
                            && documentation_is_inline(&name))
                    {
                        documentation_newline(&mut output);
                    } else if pending_whitespace
                        && documentation_space_before_tag(
                            &name,
                            false,
                            &stack,
                            previous_tag.as_ref(),
                        )
                    {
                        documentation_space(&mut output);
                    }
                    output.push_str(&normalized_tag);
                    if !tag.trim_end().ends_with("/>")
                        && !matches!(name.as_str(), "br" | "hr" | "img" | "meta" | "link")
                    {
                        stack.push(name.clone());
                    }
                    previous_tag = Some((false, name));
                }
                pending_whitespace = false;
                pending_newline = false;
            }
            DocumentationToken::Text(text) => {
                let trimmed = text.split_whitespace().collect::<Vec<_>>().join(" ");
                if trimmed.is_empty() {
                    continue;
                }
                let has_leading_whitespace = text.chars().next().is_some_and(char::is_whitespace);
                if previous_tag
                    .as_ref()
                    .is_some_and(|(closing, name)| !closing && name == "dt")
                    || (documentation_pseudo_tag_in_stack(&stack)
                        && (pending_newline
                            || text
                                .chars()
                                .next()
                                .is_some_and(|character| matches!(character, '\n' | '\r'))
                            || previous_tag.as_ref().is_some_and(|(closing, name)| {
                                !closing && !documentation_known_tag(name)
                            })))
                {
                    documentation_newline(&mut output);
                } else if (pending_whitespace || has_leading_whitespace)
                    && documentation_space_before_text(&stack, &previous_tag, &output)
                {
                    documentation_space(&mut output);
                }
                output.push_str(&escape_documentation_text(&escape_doc_brackets(&trimmed)));
                pending_whitespace = text.chars().last().is_some_and(char::is_whitespace);
                pending_newline = text
                    .chars()
                    .last()
                    .is_some_and(|character| matches!(character, '\n' | '\r'));
                previous_tag = None;
            }
        }
    }

    output
}

fn normalize_documentation_tag(tag: &str, closing: bool, stack: &[String]) -> (String, String) {
    let name = documentation_tag_name(tag).unwrap_or_default();
    if name == "a" && !closing && !tag.to_ascii_lowercase().contains("href=") {
        return ("<code>".to_owned(), "code".to_owned());
    }
    if name == "a" && closing && stack.last().is_some_and(|current| current == "code") {
        return ("</code>".to_owned(), "code".to_owned());
    }
    (lowercase_documentation_tag(tag), name)
}

fn lowercase_documentation_tag(tag: &str) -> String {
    let Some(name_start) = tag.find(|character: char| character.is_ascii_alphabetic()) else {
        return tag.to_owned();
    };
    let name_end = tag[name_start..]
        .find(|character: char| {
            !(character.is_ascii_alphanumeric() || character == '-' || character == ':')
        })
        .map_or(tag.len(), |offset| name_start + offset);
    let original = &tag[name_start..name_end];
    if original
        .chars()
        .all(|character| !character.is_ascii_uppercase())
    {
        return tag.to_owned();
    }
    format!(
        "{}{}{}",
        &tag[..name_start],
        original.to_ascii_lowercase(),
        &tag[name_end..]
    )
}

fn documentation_pseudo_tag_in_stack(stack: &[String]) -> bool {
    stack.iter().any(|name| !documentation_known_tag(name))
}

fn documentation_known_tag(name: &str) -> bool {
    documentation_block_tag(name)
        || documentation_custom_tag(name)
        || documentation_is_inline(name)
        || matches!(name, "br" | "hr" | "img" | "meta" | "link")
}

fn documentation_block_tag(name: &str) -> bool {
    matches!(
        name,
        "address"
            | "blockquote"
            | "dd"
            | "div"
            | "dl"
            | "dt"
            | "fieldset"
            | "figcaption"
            | "figure"
            | "footer"
            | "form"
            | "h1"
            | "h2"
            | "h3"
            | "h4"
            | "h5"
            | "h6"
            | "header"
            | "hr"
            | "li"
            | "main"
            | "nav"
            | "ol"
            | "p"
            | "pre"
            | "section"
            | "table"
            | "tbody"
            | "td"
            | "tfoot"
            | "th"
            | "thead"
            | "tr"
            | "ul"
    )
}

fn documentation_custom_tag(name: &str) -> bool {
    matches!(name, "important" | "note" | "tip" | "warning")
}

fn documentation_newline_before_close(name: &str, previous_tag: Option<&(bool, String)>) -> bool {
    if matches!(name, "p" | "li") {
        return false;
    }
    if documentation_custom_tag(name) {
        return previous_tag
            .is_some_and(|(closing, previous)| *closing && documentation_block_tag(previous));
    }
    documentation_block_tag(name)
}

fn documentation_space_before_tag(
    name: &str,
    closing: bool,
    stack: &[String],
    previous_tag: Option<&(bool, String)>,
) -> bool {
    if documentation_block_tag(name)
        || (documentation_custom_tag(name)
            && previous_tag.is_some_and(|(is_closing, previous)| {
                *is_closing && documentation_block_tag(previous)
            }))
    {
        return false;
    }
    if previous_tag.is_some_and(|(is_closing, previous)| {
        !is_closing && (documentation_block_tag(previous) || documentation_custom_tag(previous))
    }) {
        return false;
    }
    let _ = (closing, stack);
    true
}

fn documentation_space_before_text(
    stack: &[String],
    previous_tag: &Option<(bool, String)>,
    output: &str,
) -> bool {
    if output.is_empty() || output.ends_with('\n') {
        return false;
    }
    if stack
        .last()
        .is_some_and(|parent| documentation_block_tag(parent) || documentation_custom_tag(parent))
    {
        return previous_tag.as_ref().is_some_and(|(closing, _)| *closing);
    }
    true
}

fn documentation_newline(output: &mut String) {
    while output.ends_with(' ') {
        output.pop();
    }
    if !output.is_empty() && !output.ends_with('\n') {
        output.push('\n');
    }
}

fn documentation_space(output: &mut String) {
    if !output.is_empty() && !output.ends_with(' ') && !output.ends_with('\n') {
        output.push(' ');
    }
}

fn escape_doc_brackets(value: &str) -> String {
    value.replace('[', "\\[").replace(']', "\\]")
}

fn normalize_client_documentation(value: &str) -> String {
    let tokens = documentation_tokens(value);

    let mut output = String::new();
    let mut stack = Vec::<String>::new();
    let mut previous = None::<DocumentationToken>;
    let mut next_significant = vec![None; tokens.len()];
    let mut next = None;
    for index in (0..tokens.len()).rev() {
        next_significant[index] = next;
        if !matches!(tokens[index], DocumentationToken::Whitespace(_)) {
            next = Some(index);
        }
    }
    for (index, token) in tokens.iter().enumerate() {
        if matches!(token, DocumentationToken::Whitespace(_)) {
            let next = next_significant[index].map(|next| &tokens[next]);
            let gap = documentation_gap(previous.as_ref(), next, &stack);
            output.push_str(&gap);
            continue;
        }
        match token {
            DocumentationToken::Tag(tag) => {
                output.push_str(tag);
                if let Some(name) = documentation_tag_name(tag) {
                    if tag.starts_with("</") {
                        if stack.last().is_some_and(|current| current == &name) {
                            stack.pop();
                        }
                    } else if !tag.ends_with("/>")
                        && !matches!(name.as_str(), "br" | "hr" | "img" | "meta" | "link")
                    {
                        stack.push(name);
                    }
                }
            }
            DocumentationToken::Text(text) => {
                output.push_str(&normalize_documentation_text(
                    text,
                    previous.as_ref(),
                    stack.last(),
                ));
            }
            DocumentationToken::Whitespace(_) => unreachable!(),
        }
        previous = Some(token.clone());
    }
    for tag in ["p", "li"] {
        output = output.replace(&format!(" </{tag}>"), &format!("</{tag}>"));
    }
    output
}

fn documentation_tag_name(tag: &str) -> Option<String> {
    let tag = tag
        .trim_start_matches('<')
        .trim_start_matches('/')
        .split(|character: char| character.is_ascii_whitespace() || character == '>')
        .next()?;
    (!tag.is_empty()).then(|| tag.to_ascii_lowercase())
}

fn documentation_is_inline(name: &str) -> bool {
    matches!(
        name,
        "a" | "b" | "code" | "em" | "i" | "s" | "small" | "span" | "strong" | "sub" | "sup"
    )
}

fn documentation_gap(
    previous: Option<&DocumentationToken>,
    next: Option<&DocumentationToken>,
    stack: &[String],
) -> String {
    let (Some(DocumentationToken::Tag(previous)), Some(next)) = (previous, next) else {
        return " ".to_owned();
    };
    let Some(previous_name) = documentation_tag_name(previous) else {
        return " ".to_owned();
    };
    let next_name = match next {
        DocumentationToken::Tag(tag) => documentation_tag_name(tag),
        DocumentationToken::Text(_) => None,
        DocumentationToken::Whitespace(_) => unreachable!(),
    };

    if let Some(next_name) = next_name {
        let next_is_close = matches!(next, DocumentationToken::Tag(tag) if tag.starts_with("</"));
        if previous.starts_with("</") {
            if next_is_close {
                return if stack.last().is_some_and(|name| {
                    matches!(
                        name.as_str(),
                        "code" | "note" | "important" | "warning" | "tip"
                    )
                }) {
                    " ".to_owned()
                } else if previous_name == "li"
                    && matches!(stack.last().map(String::as_str), Some("ul" | "ol"))
                {
                    " ".repeat(stack.len())
                } else {
                    "".to_owned()
                };
            }
            if previous_name == "li"
                && matches!(next_name.as_str(), "li" | "ul" | "ol")
                && matches!(stack.last().map(String::as_str), Some("ul" | "ol"))
            {
                return " ".repeat(stack.len() + 1);
            }
            if previous_name == "p"
                && matches!(
                    next_name.as_str(),
                    "note" | "important" | "warning" | "tip" | "br"
                )
            {
                return "".to_owned();
            }
            if stack.last().is_some_and(|name| name == "li")
                && matches!(next_name.as_str(), "p" | "ul" | "ol")
            {
                return " ".repeat(stack.len() + 1);
            }
            return " ".to_owned();
        }
        if next_is_close {
            return if matches!(
                stack.last().map(String::as_str),
                Some("code" | "note" | "important" | "warning" | "tip")
            ) {
                " ".to_owned()
            } else {
                "".to_owned()
            };
        }
        if stack.last().is_some_and(|name| name == "p") && documentation_is_inline(&next_name) {
            return "".to_owned();
        }
        if matches!(
            stack.last().map(String::as_str),
            Some("ul" | "ol" | "li" | "note" | "important" | "warning" | "tip")
        ) {
            return " ".repeat(stack.len() + 1);
        }
        return " ".to_owned();
    }

    if matches!(stack.last().map(String::as_str), Some("p" | "code")) {
        "".to_owned()
    } else {
        " ".to_owned()
    }
}

fn normalize_documentation_text(
    text: &str,
    previous: Option<&DocumentationToken>,
    parent: Option<&String>,
) -> String {
    let had_newline = text
        .chars()
        .any(|character| matches!(character, '\n' | '\r'));
    if !had_newline {
        if parent.is_some_and(|name| name == "code") {
            return escape_documentation_text(text);
        }
        let mut output = String::new();
        let preserve_leading_space = text.chars().next().is_some_and(|character| {
            character.is_whitespace()
                && previous.is_some_and(|token| {
                    matches!(token, DocumentationToken::Tag(tag) if documentation_tag_name(tag)
                            .is_some_and(|name| documentation_is_inline(&name)))
                })
        });
        let mut whitespace = false;
        for character in text.chars() {
            if character.is_whitespace() {
                whitespace = true;
            } else {
                if whitespace
                    && (preserve_leading_space
                        || !output.is_empty()
                        || previous.is_some_and(|token| {
                            matches!(token, DocumentationToken::Tag(tag) if tag.starts_with("</"))
                        }))
                {
                    output.push(' ');
                }
                output.push_str(&escape_documentation_text(&character.to_string()));
                whitespace = false;
            }
        }
        if whitespace && !output.is_empty() {
            output.push(' ');
        }
        return output;
    }
    let mut output = String::new();
    let preserve_leading_space = text.chars().next().is_some_and(|character| {
        character.is_whitespace()
            && previous.is_some_and(|token| {
                matches!(token, DocumentationToken::Tag(tag) if documentation_tag_name(tag)
                        .is_some_and(|name| documentation_is_inline(&name)))
            })
    });
    let mut whitespace = false;
    for character in text.chars() {
        if character.is_whitespace() {
            whitespace = true;
        } else {
            if whitespace
                && (preserve_leading_space
                    || !output.is_empty()
                    || previous.is_some_and(|token| {
                        matches!(token, DocumentationToken::Tag(tag) if tag.starts_with("</"))
                    }))
            {
                output.push(' ');
            }
            output.push_str(&escape_documentation_text(&character.to_string()));
            whitespace = false;
        }
    }
    if whitespace && !output.is_empty() {
        output.push(' ');
    }
    output
}

fn escape_documentation_text(text: &str) -> String {
    text.chars().fold(String::new(), |mut output, character| {
        match character {
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '&' => output.push_str("&amp;"),
            _ => output.push(character),
        }
        output
    })
}

#[derive(Clone)]
enum Context {
    Types {
        consumer_namespace: bool,
    },
    Error {
        consumer_namespace: bool,
    },
    Operation {
        module: String,
        input: bool,
        consumer_namespace: bool,
    },
    Builder {
        module: String,
        input: bool,
        consumer_namespace: bool,
    },
}

impl Context {
    fn consumer_namespace(&self) -> bool {
        match self {
            Self::Types { consumer_namespace }
            | Self::Error { consumer_namespace }
            | Self::Operation {
                consumer_namespace, ..
            }
            | Self::Builder {
                consumer_namespace, ..
            } => *consumer_namespace,
        }
    }
}

fn type_expr(selected: &SelectedModel, target: &str, context: Context) -> String {
    if terminal(target) == "StreamingBlob" {
        return match context {
            Context::Types { consumer_namespace } => {
                if consumer_namespace {
                    "super::primitives::ByteStream".to_owned()
                } else {
                    "::aws_smithy_types::byte_stream::ByteStream".to_owned()
                }
            }
            Context::Error { consumer_namespace } => {
                if consumer_namespace {
                    "super::super::primitives::ByteStream".to_owned()
                } else {
                    "::aws_smithy_types::byte_stream::ByteStream".to_owned()
                }
            }
            Context::Operation {
                consumer_namespace, ..
            }
            | Context::Builder {
                consumer_namespace, ..
            } => {
                if consumer_namespace {
                    "super::super::super::primitives::ByteStream".to_owned()
                } else {
                    "::aws_smithy_types::byte_stream::ByteStream".to_owned()
                }
            }
        };
    }
    if target.starts_with("smithy.api#") {
        return primitive_type_for_namespace(
            target.rsplit('#').next().unwrap_or("string"),
            context.consumer_namespace(),
        );
    }
    if let Some(shape) = selected.model.shapes.get(target) {
        match shape.get("type").and_then(Value::as_str) {
            Some(
                "string" | "integer" | "long" | "short" | "byte" | "float" | "double" | "boolean"
                | "blob" | "timestamp" | "document",
            ) => {
                return primitive_type_for_namespace(
                    shape
                        .get("type")
                        .and_then(Value::as_str)
                        .unwrap_or("string"),
                    context.consumer_namespace(),
                );
            }
            Some("list") => {
                let element = shape
                    .get("member")
                    .and_then(member_target)
                    .map(|target| type_expr(selected, target, context.clone()))
                    .unwrap_or_else(|| "::std::string::String".to_owned());
                return format!("::std::vec::Vec<{element}>");
            }
            Some("map") => {
                let key = shape
                    .get("key")
                    .and_then(member_target)
                    .map(|target| type_expr(selected, target, context.clone()))
                    .unwrap_or_else(|| "::std::string::String".to_owned());
                let value = shape
                    .get("value")
                    .and_then(member_target)
                    .map(|target| type_expr(selected, target, context.clone()))
                    .unwrap_or_else(|| "::std::string::String".to_owned());
                return if context.consumer_namespace() {
                    format!("::std::collections::BTreeMap<{key}, {value}>")
                } else {
                    format!("::std::collections::HashMap<{key}, {value}>")
                };
            }
            _ => {}
        }
    }
    let name = terminal(target);
    let known = rust_type_name(name);
    let consumer_namespace = context.consumer_namespace();
    match context {
        Context::Types { .. } if name == "StreamingBlob" => {
            if consumer_namespace {
                "super::primitives::ByteStream".to_owned()
            } else {
                "::aws_smithy_types::byte_stream::ByteStream".to_owned()
            }
        }
        Context::Types { .. } => {
            if consumer_namespace {
                format!("self::{known}")
            } else {
                format!("crate::types::{known}")
            }
        }
        Context::Error { .. } => {
            if consumer_namespace {
                format!("super::super::types::{known}")
            } else {
                format!("crate::types::{known}")
            }
        }
        Context::Operation { module, .. } => {
            if name == "StreamingBlob" {
                if consumer_namespace {
                    "super::super::super::primitives::ByteStream".to_owned()
                } else {
                    "::aws_smithy_types::byte_stream::ByteStream".to_owned()
                }
            } else if name.ends_with("Input") {
                if consumer_namespace {
                    format!("super::super::{module}::Input")
                } else {
                    format!("crate::operation::{module}::Input")
                }
            } else if name.ends_with("Output") {
                if consumer_namespace {
                    format!("super::super::{module}::Output")
                } else {
                    format!("crate::operation::{module}::Output")
                }
            } else {
                if consumer_namespace {
                    format!("super::super::super::types::{known}")
                } else {
                    format!("crate::types::{known}")
                }
            }
        }
        Context::Builder { module, .. } => {
            if name == "StreamingBlob" {
                if consumer_namespace {
                    "super::super::super::primitives::ByteStream".to_owned()
                } else {
                    "::aws_smithy_types::byte_stream::ByteStream".to_owned()
                }
            } else if name.ends_with("Input") {
                if consumer_namespace {
                    format!("super::{module}::Input")
                } else {
                    format!("crate::operation::{module}::Input")
                }
            } else if name.ends_with("Output") {
                if consumer_namespace {
                    format!("super::{module}::Output")
                } else {
                    format!("crate::operation::{module}::Output")
                }
            } else {
                if consumer_namespace {
                    format!("super::super::super::types::{known}")
                } else {
                    format!("crate::types::{known}")
                }
            }
        }
    }
}

fn primitive_type(name: &str) -> String {
    match name {
        "Boolean" | "boolean" => "bool",
        "Byte" | "byte" => "i8",
        "Short" | "short" => "i16",
        "Integer" | "integer" => "i32",
        "Long" | "long" => "i64",
        "Float" | "float" => "f32",
        "Double" | "double" => "f64",
        "Blob" | "blob" => "::std::vec::Vec<u8>",
        "Timestamp" | "timestamp" => "::std::time::SystemTime",
        "Document" | "document" => "::std::string::String",
        _ => "::std::string::String",
    }
    .to_owned()
}

fn primitive_type_for_namespace(name: &str, consumer_namespace: bool) -> String {
    if !consumer_namespace && matches!(name, "Timestamp" | "timestamp") {
        "::aws_smithy_types::DateTime".to_owned()
    } else {
        primitive_type(name)
    }
}

fn rust_type_name(value: &str) -> String {
    let result = names::snake_case(value)
        .split('_')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect::<String>(),
                None => String::new(),
            }
        })
        .collect::<String>();
    if result.is_empty()
        || result
            .chars()
            .next()
            .is_some_and(|character| character.is_ascii_digit())
    {
        format!("Value{result}")
    } else if result == "Self" {
        "SelfType".to_owned()
    } else {
        result
    }
}

/// Smithy keeps all-uppercase runs in operation symbols (for example, the
/// `SAML` segment of `AssumeRoleWithSAML`) while modeled member/enum names use
/// the ordinary Rust type-name normalization above.
fn operation_error_type_name(value: &str) -> String {
    let mut result = rust_type_name(value);
    let chars = value.chars().collect::<Vec<_>>();
    let mut index = 0;
    while index < chars.len() {
        if !chars[index].is_ascii_uppercase() {
            index += 1;
            continue;
        }
        let start = index;
        while index < chars.len() && chars[index].is_ascii_uppercase() {
            if index > start
                && chars
                    .get(index + 1)
                    .is_some_and(|next| next.is_ascii_lowercase())
            {
                break;
            }
            index += 1;
        }
        if index - start < 2 {
            continue;
        }
        let acronym = chars[start..index].iter().collect::<String>();
        let normalized = acronym
            .chars()
            .next()
            .into_iter()
            .flat_map(|first| first.to_uppercase())
            .chain(
                acronym
                    .chars()
                    .skip(1)
                    .flat_map(|character| character.to_lowercase()),
            )
            .collect::<String>();
        result = result.replace(&normalized, &acronym);
    }
    result
}

fn terminal(value: &str) -> &str {
    value.rsplit('#').next().unwrap_or(value)
}

fn target_value(value: &Value) -> Option<&str> {
    value.get("target").and_then(Value::as_str)
}

fn member_target(member: &Value) -> Option<&str> {
    member
        .as_str()
        .or_else(|| member.get("target").and_then(Value::as_str))
}

fn members(shape: &Value) -> Vec<(String, &Value)> {
    shape
        .get("members")
        .and_then(Value::as_object)
        .map(|members| {
            members
                .iter()
                .map(|(name, member)| (name.clone(), member))
                .collect()
        })
        .unwrap_or_default()
}

fn sorted_members(shape: &Value) -> BTreeMap<String, &Value> {
    members(shape).into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_model_documentation_preserves_malformed_html_structure() {
        assert_eq!(
            normalize_documentation_tag("</key>", true, &["code".to_owned()]),
            ("</key>".to_owned(), "key".to_owned())
        );
        let normalized = normalize_model_documentation(
            "<p><code><scanrange><start>50</start></scanrange></code></p>",
        );
        assert_eq!(
            normalized,
            "<p><code><scanrange>\n<start>\n50\n</start>\n</scanrange></code></p>"
        );
        assert_eq!(
            normalize_model_documentation(
                "<p>Returned in the <Code> tag of the error\n      XML response for a corresponding <code>GetObject</code> call. Cannot be used with a successful\n        <code>StatusCode</code> header or when the transformed object is provided in the body. All error codes\n      from S3 are sentence-cased. The regular expression (regex) value is\n      <code>\"^[A-Z][a-zA-Z]+$\"</code>.</p>"
            ),
            "<p>Returned in the <code> tag of the error XML response for a corresponding <code>GetObject</code> call. Cannot be used with a successful <code>StatusCode</code> header or when the transformed object is provided in the body. All error codes from S3 are sentence-cased. The regular expression (regex) value is <code>\"^\\[A-Z\\]\\[a-zA-Z\\]+$\"</code>.</code></p>"
        );
        assert_eq!(
            normalize_model_documentation(
                "<p>Contains a generic description of the error condition. Returned in the <Message> tag of the\n      error XML response for a corresponding <code>GetObject</code> call. Cannot be used with a successful\n        <code>StatusCode</code> header or when the transformed object is provided in body.</p>"
            ),
            "<p>Contains a generic description of the error condition. Returned in the <message>\ntag of the error XML response for a corresponding\n<code>GetObject</code> call. Cannot be used with a successful\n<code>StatusCode</code> header or when the transformed object is provided in body.\n</message></p>"
        );
    }

    #[test]
    fn xml_body_detection_respects_http_response_bindings() {
        let prefix_headers = serde_json::json!({
            "traits": { "smithy.api#httpPrefixHeaders": "x-amz-meta-" }
        });
        let response_code = serde_json::json!({
            "traits": { "smithy.api#httpResponseCode": "status" }
        });
        let streaming_payload = serde_json::json!({
            "target": "example#StreamingBlob",
            "traits": { "smithy.api#httpPayload": {} }
        });
        let document_body = serde_json::json!({
            "target": "example#Result"
        });

        assert!(!is_xml_body_member(&prefix_headers));
        assert!(!is_xml_body_member(&response_code));
        assert!(!is_xml_body_member(&streaming_payload));
        assert!(is_xml_body_member(&document_body));
    }

    #[test]
    fn generated_service_uses_reference_aligned_source_tree() {
        let stage = tempfile::tempdir().unwrap();
        let selections = [ServiceSelection {
            key: "s3".to_owned(),
            operations: vec![
                "AbortMultipartUpload".to_owned(),
                "ListObjectsV2".to_owned(),
            ],
            all_operations: false,
        }];

        generate(stage.path(), true, &selections).unwrap();

        let generated = stage.path().join("generated/s3/src");
        assert!(generated.join("lib.rs").is_file());
        assert!(!generated.join("aws_runtime.rs").exists());
        assert!(
            fs::read_to_string(generated.join("client.rs"))
                .unwrap()
                .contains("pub(crate) mod transport")
        );
        assert!(generated.join("observability_feature.rs").is_file());
        assert!(generated.join("client.rs").is_file());
        assert!(generated.join("operation.rs").is_file());
        assert!(generated.join("types/builders.rs").is_file());
        assert!(generated.join("types/error.rs").is_file());
        assert!(
            generated
                .join("operation/abort_multipart_upload.rs")
                .is_file()
        );
        assert!(!generated.join("types/_object_list.rs").exists());
        assert!(
            !fs::read_to_string(generated.join("types.rs"))
                .unwrap()
                .contains("types/_object_list.rs")
        );
        assert!(
            fs::read_to_string(
                generated.join("operation/list_objects_v2/_list_objects_v2_output.rs")
            )
            .unwrap()
            .contains("::std::vec::Vec<super::super::super::types::Object>")
        );
        assert!(generated.join("types.rs").is_file());
        assert!(!generated.join("types/_bucket_name.rs").exists());
        assert!(
            fs::read_to_string(generated.join("types.rs"))
                .unwrap()
                .contains("/generated/s3/src/types/_")
        );
        assert!(!stage.path().join("aws_sdk_build_manifest.json").exists());
        assert!(!stage.path().join("generated/aws_sdk_s3.rs").exists());
    }

    #[test]
    fn rest_xml_payload_wrapper_calls_operation_input_serializer() {
        let stage = tempfile::tempdir().unwrap();
        let selections = [ServiceSelection {
            key: "s3".to_owned(),
            operations: vec!["PutBucketWebsite".to_owned()],
            all_operations: false,
        }];

        generate(stage.path(), true, &selections).unwrap();

        let payload = fs::read_to_string(
            stage
                .path()
                .join("generated/s3/src/protocol_serde/shape_put_bucket_website_input.rs"),
        )
        .unwrap();
        assert!(payload.contains(
            "crate::protocol_serde::shape_put_bucket_website_input::ser_website_configuration_payload"
        ));
        assert!(payload.contains(
            "crate::protocol_serde::shape_website_configuration::ser_website_configuration"
        ));
    }
}
