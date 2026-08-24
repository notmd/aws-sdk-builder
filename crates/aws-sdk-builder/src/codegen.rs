use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    fmt::Write,
    fs,
    path::Path,
};

use crate::{
    artifact,
    config::ServiceSelection,
    error::BuildError,
    model::{ProtocolKind, SelectedModel},
    names,
};

pub(crate) struct Generated {
    pub(crate) operations: Vec<String>,
    pub(crate) files: BTreeMap<String, BTreeMap<String, String>>,
}

pub(crate) fn generate(
    stage: &Path,
    selections: &[ServiceSelection],
) -> Result<Generated, BuildError> {
    let generated = stage.join("generated");
    fs::create_dir_all(&generated).map_err(|source| BuildError::OutputWrite {
        path: generated.clone(),
        source,
    })?;
    let mut all_operations = Vec::new();
    let mut generated_files = BTreeMap::new();
    for selection in selections {
        let entry = selection.source.metadata;
        let model = crate::model::Model::load(selection.source)?;
        let selected = model.select(&selection.operations, selection.all_operations)?;
        let protocol = selected.model.protocol()?;
        let request_id_plan = request_id_plan(&selected);
        let service_dir = generated.join(entry.key);
        let mut service_files = vec![
            ("src/lib.rs".to_owned(), render_service_lib(&selected)),
            ("src/primitives.rs".to_owned(), render_primitives(&selected)),
            ("src/config.rs".to_owned(), render_config_file(&selected)),
            ("src/config/auth.rs".to_owned(), render_auth_file(&selected)),
            (
                "src/error.rs".to_owned(),
                render_error_file(model_has_enum(&selected)),
            ),
            ("src/meta.rs".to_owned(), render_meta(entry.key)),
            (
                "src/observability_feature.rs".to_owned(),
                render_observability_feature(),
            ),
            ("src/types.rs".to_owned(), render_types_file(&selected)),
            (
                "src/types/builders.rs".to_owned(),
                render_types_builders_file(&selected),
            ),
            (
                "src/types/error.rs".to_owned(),
                render_error_types_file(&selected),
            ),
            (
                "src/types/error/builders.rs".to_owned(),
                render_error_builders_file(&selected),
            ),
            (
                "src/operation.rs".to_owned(),
                render_operations_file(&selected),
            ),
            ("src/client.rs".to_owned(), render_client_file(&selected)),
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
        {
            if model_has_event_stream(&selected) {
                service_files.push((
                    "src/event_receiver.rs".to_owned(),
                    include_str!("../assets/event_receiver.rs").to_owned(),
                ));
                service_files.push((
                    "src/event_stream_serde.rs".to_owned(),
                    render_event_stream_serde_file(&selected),
                ));
            }
            if model_contains_string(&selected, "AccountId") {
                service_files.push((
                    "src/account_id_endpoint.rs".to_owned(),
                    include_str!("../assets/account_id_endpoint.rs").to_owned(),
                ));
            }
            if model_has_aws_chunked_operations(&selected) {
                service_files.push((
                    "src/aws_chunked.rs".to_owned(),
                    include_str!("../assets/aws_chunked.rs").to_owned(),
                ));
                service_files.push((
                    "src/endpoint_auth.rs".to_owned(),
                    include_str!("../assets/endpoint_auth.rs").to_owned(),
                ));
            }
            if model_contains_trait(&selected, "aws.protocols#httpChecksum") {
                service_files.push((
                    "src/http_request_checksum.rs".to_owned(),
                    include_str!("../assets/http_request_checksum.rs").to_owned(),
                ));
                service_files.push((
                    "src/http_response_checksum.rs".to_owned(),
                    include_str!("../assets/http_response_checksum.rs").to_owned(),
                ));
            }
            if service_supports_s3_express(&selected) {
                service_files.push((
                    "src/s3_express.rs".to_owned(),
                    include_str!("../assets/s3_express.rs").to_owned(),
                ));
            }
            if service_has_rest_xml_unwrapped_errors(&selected) {
                service_files.push((
                    "src/rest_xml_unwrapped_errors.rs".to_owned(),
                    include_str!("../assets/rest_xml_unwrapped_errors.rs").to_owned(),
                ));
            }
            if service_has_protocol(&selected, ProtocolKind::AwsQuery) {
                service_files.push((
                    "src/rest_xml_wrapped_errors.rs".to_owned(),
                    include_str!("../assets/rest_xml_wrapped_errors.rs").to_owned(),
                ));
            }
            if selected
                .operations
                .iter()
                .filter_map(|name| operation_shape(&selected, name))
                .any(|operation| operation_has_s3_expires_output(&selected, operation))
            {
                service_files.push((
                    "src/s3_expires_interceptor.rs".to_owned(),
                    include_str!("../assets/s3_expires_interceptor.rs").to_owned(),
                ));
            }
            if service_has_endpoint_rules(&selected) {
                service_files.push((
                    "src/config/endpoint.rs".to_owned(),
                    crate::endpoint_codegen::render_endpoint_config_file(&selected),
                ));
            }
            service_files.push((
                "src/error_meta.rs".to_owned(),
                render_service_error_metadata(&selected),
            ));
            service_files.push((
                "src/error/sealed_unhandled.rs".to_owned(),
                include_str!("../assets/error_sealed_unhandled.rs").to_owned(),
            ));
            if service_has_endpoint_rules(&selected) {
                service_files.push((
                    "src/endpoint_lib.rs".to_owned(),
                    render_endpoint_lib(&selected),
                ));
                for module in endpoint_lib_module_names(&selected) {
                    service_files.push((
                        format!("src/endpoint_lib/{module}.rs"),
                        render_endpoint_lib_module(&module),
                    ));
                }
            }
        }
        {
            service_files.push((
                "src/primitives/event_stream.rs".to_owned(),
                render_event_stream_primitives(model_has_event_stream(&selected)),
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
        }
        {
            service_files.push((
                "src/serialization_settings.rs".to_owned(),
                render_serialization_settings_file(),
            ));
            if service_has_protocol(&selected, crate::model::ProtocolKind::AwsQueryCompatible) {
                service_files.push((
                    "src/aws_query_compatible_errors.rs".to_owned(),
                    include_str!("../assets/aws_query_compatible_errors.rs").to_owned(),
                ));
            }
            if matches!(
                protocol,
                crate::model::ProtocolKind::RestJson1
                    | crate::model::ProtocolKind::AwsJson1_0
                    | crate::model::ProtocolKind::AwsJson1_1
            ) {
                service_files.push((
                    "src/json_errors.rs".to_owned(),
                    include_str!("../assets/json_errors.rs").to_owned(),
                ));
            }
        }
        if matches!(
            protocol,
            crate::model::ProtocolKind::RestXml
                | crate::model::ProtocolKind::AwsQuery
                | crate::model::ProtocolKind::Ec2Query
        ) {
            let (protocol_module, protocol_shape_files) =
                render_protocol_serde_files(&selected, protocol);
            service_files.push(("src/protocol_serde.rs".to_owned(), protocol_module));
            service_files.extend(protocol_shape_files);
        } else if matches!(
            protocol,
            crate::model::ProtocolKind::RestJson1
                | crate::model::ProtocolKind::AwsJson1_0
                | crate::model::ProtocolKind::AwsJson1_1
        ) {
            let (protocol_module, protocol_shape_files) =
                render_json_protocol_serde_files(&selected);
            service_files.push(("src/protocol_serde.rs".to_owned(), protocol_module));
            service_files.extend(protocol_shape_files);
        }
        if has_paginated_operations(&selected) {
            service_files.push(("src/lens.rs".to_owned(), render_lens_file(&selected)));
        }
        if has_waiters(&selected) {
            service_files.push(("src/waiters.rs".to_owned(), render_waiters_file(&selected)));
            service_files.push((
                "src/waiters/matchers.rs".to_owned(),
                render_waiter_matchers_file(&selected),
            ));
            for (_, waiter_name, waiter) in waiter_specs(&selected) {
                service_files.push((
                    format!("src/waiters/{waiter_name}.rs"),
                    render_waiter_file(&selected, &waiter_name, &waiter),
                ));
            }
        }
        let mut operation_names = selected.operations.clone();
        operation_names.sort();
        for operation_name in operation_names {
            let module = names::snake_case(&operation_name);
            service_files.push((
                format!("src/operation/{module}.rs"),
                render_operation_file(&selected, &operation_name),
            ));
            service_files.push((
                format!("src/operation/{module}/_{module}_input.rs"),
                render_operation_shape_file(&selected, &operation_name, true),
            ));
            service_files.push((
                format!("src/operation/{module}/_{module}_output.rs"),
                render_operation_shape_file(&selected, &operation_name, false),
            ));
            service_files.push((
                format!("src/operation/{module}/builders.rs"),
                render_operation_builder_file(&selected, &operation_name),
            ));
            if operation_pagination_info(&selected, &operation_name).is_some() {
                service_files.push((
                    format!("src/operation/{module}/paginator.rs"),
                    render_paginator_file(&selected, &operation_name),
                ));
            }
            service_files.push((
                format!("src/client/{module}.rs"),
                render_client_operation_file(&selected, &operation_name),
            ));
            if matches!(
                protocol,
                crate::model::ProtocolKind::RestXml
                    | crate::model::ProtocolKind::AwsQuery
                    | crate::model::ProtocolKind::Ec2Query
            ) {
                service_files.push((
                    format!("src/protocol_serde/shape_{module}.rs"),
                    render_protocol_operation_file(&selected, &operation_name, protocol),
                ));
                let input_source = if is_query_protocol(protocol) {
                    Some(render_query_protocol_input_file(&selected, &operation_name))
                } else {
                    render_protocol_input_file(&selected, &operation_name)
                };
                if let Some(payload_source) = input_source {
                    service_files.push((
                        format!("src/protocol_serde/shape_{module}_input.rs"),
                        payload_source,
                    ));
                }
                if let Some(output_source) = render_protocol_output_file(&selected, &operation_name)
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
            if shape_id == selected.model.service_shape_id.as_str()
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
                        Context::Error {}
                    } else {
                        Context::Types {}
                    },
                ),
            ));
        }
        let mut canonical_files = BTreeMap::new();
        for (relative_path, source) in service_files {
            let rendered_source = normalize_source(&source);

            canonical_files.insert(relative_path, rendered_source);
        }
        let original = artifact::compose(&canonical_files)?;
        fs::create_dir_all(&service_dir).map_err(|source| BuildError::OutputWrite {
            path: service_dir.clone(),
            source,
        })?;
        let original_path = service_dir.join(artifact::ORIGINAL_FILE);
        fs::write(&original_path, original).map_err(|source| BuildError::OutputWrite {
            path: original_path.clone(),
            source,
        })?;
        generated_files.insert(entry.key.to_owned(), canonical_files);
        all_operations.extend(selected.operations.iter().cloned());
    }
    all_operations.sort();
    Ok(Generated {
        operations: all_operations,
        files: generated_files,
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

fn is_query_protocol(protocol: ProtocolKind) -> bool {
    matches!(protocol, ProtocolKind::AwsQuery | ProtocolKind::Ec2Query)
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
        .get(selected.model.service_shape_id.as_str());
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

fn render_service_lib(selected: &SelectedModel) -> String {
    render_standalone_service_lib(selected)
}

fn render_standalone_service_lib(selected: &SelectedModel) -> String {
    let service = selected
        .model
        .shapes
        .get(selected.model.service_shape_id.as_str())
        .expect("selected service shape exists");
    let protocol = selected.model.protocol().unwrap_or(ProtocolKind::RestJson1);
    let service_title = service_title(selected);
    let crate_name = service_crate_name(selected.model.entry.key);
    let module_name = selected.model.entry.module_name;
    let module_alias = module_name.strip_prefix("aws_sdk_").unwrap_or(module_name);
    let sdk_version = selected.model.entry.sdk_version.unwrap_or("0.0.0");
    let mut output = String::new();
    output.push_str(
        "#![allow(deprecated)]\n#![allow(unknown_lints)]\n#![allow(clippy::module_inception)]\n#![allow(clippy::upper_case_acronyms)]\n#![allow(clippy::large_enum_variant)]\n#![allow(clippy::wrong_self_convention)]\n#![allow(clippy::should_implement_trait)]\n#![allow(clippy::disallowed_names)]\n#![allow(clippy::vec_init_then_push)]\n#![allow(clippy::type_complexity)]\n#![allow(clippy::needless_return)]\n#![allow(clippy::derive_partial_eq_without_eq)]\n#![allow(clippy::result_large_err)]\n#![allow(clippy::unnecessary_map_on_constructor)]\n#![allow(clippy::useless_conversion)]\n#![allow(clippy::deprecated_semver)]\n#![allow(rustdoc::bare_urls)]\n#![allow(rustdoc::redundant_explicit_links)]\n#![allow(rustdoc::broken_intra_doc_links)]\n#![allow(rustdoc::invalid_html_tags)]\n#![forbid(unsafe_code)]\n#![warn(missing_docs)]\n#![cfg_attr(docsrs, feature(doc_cfg))]\n",
    );
    if let Some(documentation) = service_crate_documentation(service) {
        for line in documentation.lines() {
            writeln!(output, "//! {line}").unwrap();
        }
    } else {
        writeln!(output, "//! {service_title}").unwrap();
    }
    output.push_str(
        "//!\n//! ## Getting Started\n//!\n//! > Examples are available for many services and operations, check out the\n//! > [usage examples](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/rustv1).\n//!\n//! The SDK provides one crate per AWS service. You must add [Tokio](https://crates.io/crates/tokio)\n//! as a dependency within your Rust project to execute asynchronous code. To add [crate] to\n//! your project, add the following to your **Cargo.toml** file:\n//!\n//! ```toml\n//! [dependencies]\n//! aws-config = { version = \"1.1.7\", features = [\"behavior-version-latest\"] }\n",
    );
    output = output.replace("To add [crate] to", &format!("To add `{crate_name}` to"));
    writeln!(
        output,
        "//! {crate_name} = \"{sdk_version}\"\n//! tokio = {{ version = \"1\", features = [\"full\"] }}\n//! ```\n//!\n//! Then in code, a client can be created with the following:\n//!\n//! ```rust{}\n//! use {module_name} as {module_alias};\n//!\n//! #[::tokio::main]\n//! async fn main() -> Result<(), {module_alias}::Error> {{\n//!     let config = aws_config::load_from_env().await;\n//!     let client = {module_name}::Client::new(&config);\n//!\n//!     // ... make some calls with the client\n//!\n//!     Ok(())\n//! }}\n//! ```\n//!\n//! See the [client documentation](https://docs.rs/{crate_name}/latest/{module_name}/client/struct.Client.html)\n//! for information on what calls can be made, and the inputs and outputs for each of those calls.\n//!\n//! ## Using the SDK\n//!\n//! Until the SDK is released, we will be adding information about using the SDK to the\n//! [Developer Guide](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/welcome.html). Feel free to suggest\n//! additional sections for the guide by opening an issue and describing what you are trying to do.\n//!\n//! ## Getting Help\n//!\n//! * [GitHub discussions](https://github.com/awslabs/aws-sdk-rust/discussions) - For ideas, RFCs & general questions\n//! * [GitHub issues](https://github.com/awslabs/aws-sdk-rust/issues/new/choose) - For bug reports & feature requests\n//! * [Generated Docs (latest version)](https://awslabs.github.io/aws-sdk-rust/)\n//! * [Usage examples](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/rustv1)\n//!\n//!\n//! # Crate Organization\n//!\n//! The entry point for most customers will be [`Client`], which exposes one method for each API\n//! offered by {service_title}. The return value of each of these methods is a \"fluent builder\",\n//! where the different inputs for that API are added by builder-style function call chaining,\n//! followed by calling `send()` to get a [`Future`](std::future::Future) that will result in\n//! either a successful output or a [`SdkError`](crate::error::SdkError).\n//!\n//! Some of these API inputs may be structs or enums to provide more complex structured information.\n//! These structs and enums live in [`types`](crate::types). There are some simpler types for\n//! representing data such as date times or binary blobs that live in [`primitives`](crate::primitives).\n//!\n//! All types required to configure a client via the [`Config`](crate::Config) struct live\n//! in [`config`](crate::config).\n//!\n//! The [`operation`](crate::operation) module has a submodule for every API, and in each submodule\n//! is the input, output, and error type for that API, as well as builders to construct each of those.\n//!\n//! There is a top-level [`Error`](crate::Error) type that encompasses all the errors that the\n//! client can return. Any other error type can be converted to this `Error` type via the\n//! [`From`](std::convert::From) trait.\n//!\n//! The other modules within this crate are not required for normal usage.\n\n",
        if has_trait(service, "aws.auth#sigv4a") {
            ",ignore"
        } else {
            ",no_run"
        }
    )
    .unwrap();
    output.push_str("// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.\npub use error_meta::Error;\n\n#[doc(inline)]\npub use config::Config;\n\n");
    client_docs_for_standalone_lib(&mut output, selected, service, &service_title);

    output.push_str("pub mod client;\n\n");
    writeln!(output, "/// Configuration for {service_title}.").unwrap();
    output.push_str("pub mod config;\n\n/// Common errors and error handling utilities.\npub mod error;\n\nmod error_meta;\n\n/// Information about this crate.\npub mod meta;\n\n/// All operations that this crate can perform.\npub mod operation;\n\n/// Primitives such as `Blob` or `DateTime` used by other types.\npub mod primitives;\n\n/// Data structures used by operation inputs/outputs.\npub mod types;\n");

    render_standalone_extra_modules(&mut output, selected, protocol);
    output.push_str("\n#[doc(inline)]\npub use client::Client;\n");
    output
}

fn client_docs_for_standalone_lib(
    output: &mut String,
    selected: &SelectedModel,
    service: &Value,
    service_title: &str,
) {
    writeln!(output, "/// Client for calling {service_title}.").unwrap();
    if !has_trait(service, "aws.auth#sigv4a") {
        output.push_str(
            "/// ## Constructing a `Client`\n///\n/// A [`Config`] is required to construct a client. For most use cases, the [`aws-config`]\n/// crate should be used to automatically resolve this config using\n/// [`aws_config::load_from_env()`], since this will resolve an [`SdkConfig`] which can be shared\n/// across multiple different AWS SDK clients. This config resolution process can be customized\n/// by calling [`aws_config::from_env()`] instead, which returns a [`ConfigLoader`] that uses\n/// the [builder pattern] to customize the default config.\n///\n/// In the simplest case, creating a client looks as follows:\n/// ```rust,no_run\n/// # async fn wrapper() {\n",
        );
        writeln!(
            output,
            "/// let config = aws_config::load_from_env().await;\n/// let client = {}::Client::new(&config);\n/// # }}\n/// ```\n///\n/// Occasionally, SDKs may have additional service-specific values that can be set on the [`Config`] that\n/// is absent from [`SdkConfig`], or slightly different settings for a specific client may be desired.\n/// The [`Builder`](crate::config::Builder) struct implements `From<&SdkConfig>`, so setting these specific settings can be\n/// done as follows:\n///\n/// ```rust,no_run\n/// # async fn wrapper() {{\n/// let sdk_config = ::aws_config::load_from_env().await;\n/// let config = {}::config::Builder::from(&sdk_config)\n/// # /*\n///     .some_service_specific_setting(\"value\")\n/// # */\n///     .build();\n/// # }}\n/// ```\n///\n/// See the [`aws-config` docs] and [`Config`] for more information on customizing configuration.\n///\n/// _Note:_ Client construction is expensive due to connection thread pool initialization, and should\n/// be done once at application start-up.\n///\n/// [`Config`]: crate::Config\n/// [`ConfigLoader`]: https://docs.rs/aws-config/*/aws_config/struct.ConfigLoader.html\n/// [`SdkConfig`]: https://docs.rs/aws-config/*/aws_config/struct.SdkConfig.html\n/// [`aws-config` docs]: https://docs.rs/aws-config/*\n/// [`aws-config`]: https://crates.io/crates/aws-config\n/// [`aws_config::from_env()`]: https://docs.rs/aws-config/*/aws_config/fn.from_env.html\n/// [`aws_config::load_from_env()`]: https://docs.rs/aws-config/*/aws_config/fn.load_from_env.html\n/// [builder pattern]: https://rust-lang.github.io/api-guidelines/type-safety.html#builders-enable-construction-of-complex-values-c-builder\n",
            selected.model.entry.module_name,
            selected.model.entry.module_name
        )
        .unwrap();
        output.pop();
    }
    if let Some((operation_name, member_name)) = client_usage_example(selected) {
        let module = names::snake_case(&operation_name);
        let field = names::rust_identifier(&member_name);
        let field = field.strip_prefix("r#").unwrap_or(&field);
        writeln!(
            output,
            "/// # Using the `Client`\n///\n/// A client has a function for every operation that can be performed by the service.\n/// For example, the [`{operation_name}`](crate::operation::{module}) operation has\n/// a [`Client::{module}`], function which returns a builder for that operation.\n/// The fluent builder ultimately has a `send()` function that returns an async future that\n/// returns a result, as illustrated below:\n///\n/// ```rust,ignore\n/// let result = client.{module}()\n///     .{field}(\"example\")\n///     .send()\n///     .await;\n/// ```\n///\n/// The underlying HTTP requests that get made by this can be modified with the `customize_operation`\n/// function on the fluent builder. See the [`customize`](crate::client::customize) module for more\n/// information."
        )
        .unwrap();
    }
    if has_waiters(selected) {
        output.push_str(
            "/// # Waiters\n///\n/// This client provides `wait_until` methods behind the [`Waiters`](crate::client::Waiters) trait.\n/// To use them, simply import the trait, and then call one of the `wait_until` methods. This will\n/// return a waiter fluent builder that takes various parameters, which are documented on the builder\n/// type. Once parameters have been provided, the `wait` method can be called to initiate waiting.\n///\n/// For example, if there was a `wait_until_thing` method, it could look like:\n/// ```rust,ignore\n/// let result = client.wait_until_thing()\n///     .thing_id(\"someId\")\n///     .wait(Duration::from_secs(120))\n///     .await;\n/// ```\n",
        );
    }
}

fn service_crate_documentation(service: &Value) -> Option<String> {
    service
        .get("traits")
        .and_then(Value::as_object)
        .and_then(|traits| traits.get("smithy.api#documentation"))
        .and_then(Value::as_str)
        .map(normalize_service_documentation)
}

#[derive(Debug)]
struct ServiceDocumentationNode {
    name: String,
    tag: String,
    children: Vec<ServiceDocumentationChild>,
}

#[derive(Debug)]
enum ServiceDocumentationChild {
    Element(ServiceDocumentationNode),
    Text(String),
}

fn normalize_service_documentation(value: &str) -> String {
    let root = parse_service_documentation(value);
    let rendered = render_service_documentation_node(&root, 0);
    normalize_service_documentation_whitespace(&rendered)
}

fn parse_service_documentation(value: &str) -> ServiceDocumentationNode {
    let mut stack = vec![ServiceDocumentationNode {
        name: "root".to_owned(),
        tag: String::new(),
        children: Vec::new(),
    }];
    for token in documentation_tokens(value) {
        match token {
            DocumentationToken::Whitespace(_) => {}
            DocumentationToken::Text(text) => {
                stack
                    .last_mut()
                    .expect("service documentation stack is non-empty")
                    .children
                    .push(ServiceDocumentationChild::Text(text));
            }
            DocumentationToken::Tag(tag) => {
                let closing = tag.trim_start().starts_with("</");
                let Some(name) = documentation_tag_name(&tag) else {
                    continue;
                };
                if closing {
                    if stack.len() > 1 {
                        let node = stack.pop().expect("service documentation node exists");
                        if node.name == name {
                            stack
                                .last_mut()
                                .expect("service documentation parent exists")
                                .children
                                .push(ServiceDocumentationChild::Element(node));
                        }
                    }
                    continue;
                }
                let node = ServiceDocumentationNode {
                    name,
                    tag: tag.clone(),
                    children: Vec::new(),
                };
                if tag.trim_end().ends_with("/>")
                    || matches!(node.name.as_str(), "br" | "hr" | "img" | "meta" | "link")
                {
                    stack
                        .last_mut()
                        .expect("service documentation stack is non-empty")
                        .children
                        .push(ServiceDocumentationChild::Element(node));
                } else {
                    stack.push(node);
                }
            }
        }
    }
    while stack.len() > 1 {
        let node = stack.pop().expect("service documentation node exists");
        stack
            .last_mut()
            .expect("service documentation parent exists")
            .children
            .push(ServiceDocumentationChild::Element(node));
    }
    stack.pop().expect("service documentation root exists")
}

fn render_service_documentation_node(node: &ServiceDocumentationNode, list_depth: usize) -> String {
    match node.name.as_str() {
        "root" => render_service_documentation_children(&node.children, list_depth),
        // Smithy-RS removes the entire note, not just the note wrapper.
        "fullname" | "note" => String::new(),
        "important" => render_service_documentation_children(&node.children, list_depth),
        "b" => format!(
            "__{}__",
            render_service_documentation_children(&node.children, list_depth)
        ),
        "i" => format!(
            "_{}_",
            render_service_documentation_children(&node.children, list_depth)
        ),
        "a" => {
            let text = render_service_documentation_children(&node.children, list_depth);
            let text = text.trim();
            let Some(href) = service_documentation_attribute(&node.tag, "href") else {
                return text.to_owned();
            };
            if href.is_empty() {
                text.to_owned()
            } else {
                format!("[{text}]({href})")
            }
        }
        "br" => "\n".to_owned(),
        "p" => {
            let text = render_service_documentation_children(&node.children, list_depth);
            format!("\n{}\n", text.trim())
        }
        "ul" | "ol" => render_service_documentation_list(node, list_depth + 1),
        "dl" => render_service_documentation_description_list(node, list_depth),
        _ => render_service_documentation_children(&node.children, list_depth),
    }
}

fn render_service_documentation_children(
    children: &[ServiceDocumentationChild],
    list_depth: usize,
) -> String {
    children
        .iter()
        .map(|child| match child {
            ServiceDocumentationChild::Element(node) => {
                render_service_documentation_node(node, list_depth)
            }
            ServiceDocumentationChild::Text(text) => normalize_service_documentation_text(text),
        })
        .collect()
}

fn render_service_documentation_list(node: &ServiceDocumentationNode, list_depth: usize) -> String {
    let prefix = if node.name == "ol" { "1. " } else { "- " };
    let indent = " ".repeat(list_depth * 2);
    let mut output = String::new();
    if list_depth > 1 {
        output.push('\n');
    }
    for child in &node.children {
        let ServiceDocumentationChild::Element(item) = child else {
            continue;
        };
        if item.name != "li" {
            continue;
        }
        let text = item
            .children
            .iter()
            .map(|child| match child {
                ServiceDocumentationChild::Element(node) if node.name == "p" => {
                    render_service_documentation_children(&node.children, list_depth)
                }
                ServiceDocumentationChild::Element(node) => {
                    render_service_documentation_node(node, list_depth)
                }
                ServiceDocumentationChild::Text(text) => normalize_service_documentation_text(text),
            })
            .collect::<String>();
        output.push_str(&indent);
        output.push_str(prefix);
        let has_nested_list = item.children.iter().any(|child| {
            matches!(
                child,
                ServiceDocumentationChild::Element(node) if matches!(node.name.as_str(), "ul" | "ol")
            )
        });
        output.push_str(text.trim());
        if has_nested_list {
            output.push('\n');
        }
        output.push('\n');
    }
    output
}

fn render_service_documentation_description_list(
    node: &ServiceDocumentationNode,
    list_depth: usize,
) -> String {
    let mut output = String::new();
    for child in &node.children {
        let ServiceDocumentationChild::Element(item) = child else {
            continue;
        };
        match item.name.as_str() {
            "dt" => {
                let text = render_service_documentation_children(&item.children, list_depth);
                output.push_str("\n__");
                output.push_str(text.trim());
                output.push_str("__\n");
            }
            "dd" => {
                let text = render_service_documentation_children(&item.children, list_depth);
                output.push('\n');
                output.push_str(text.trim());
                output.push('\n');
            }
            _ => {}
        }
    }
    output.push('\n');
    output
}

fn service_documentation_attribute(tag: &str, attribute: &str) -> Option<String> {
    let name = documentation_tag_name(tag)?;
    let start = tag.find(&name)? + name.len();
    let mut rest = &tag[start..tag.len().saturating_sub(1)];
    while !rest.is_empty() {
        rest = rest.trim_start();
        if rest.is_empty() {
            break;
        }
        let end = rest
            .find(|character: char| character.is_ascii_whitespace() || character == '=')
            .unwrap_or(rest.len());
        let current = &rest[..end];
        rest = &rest[end..];
        rest = rest.trim_start();
        if !rest.starts_with('=') {
            continue;
        }
        rest = rest[1..].trim_start();
        let (value, consumed) =
            if let Some(quote) = rest.chars().next().filter(|c| *c == '\'' || *c == '"') {
                let rest_without_quote = &rest[quote.len_utf8()..];
                let end = rest_without_quote
                    .find(quote)
                    .unwrap_or(rest_without_quote.len());
                (
                    &rest_without_quote[..end],
                    (quote.len_utf8() + end + quote.len_utf8()).min(rest.len()),
                )
            } else {
                let end = rest
                    .find(|character: char| character.is_ascii_whitespace() || character == '>')
                    .unwrap_or(rest.len());
                (&rest[..end], end)
            };
        if current.eq_ignore_ascii_case(attribute) {
            return Some(decode_service_documentation_entities(value));
        }
        rest = &rest[consumed.min(rest.len())..];
    }
    None
}

fn normalize_service_documentation_text(text: &str) -> String {
    let decoded = decode_service_documentation_entities(text);
    let mut output = String::new();
    let mut whitespace = false;
    for character in decoded.chars() {
        if character.is_whitespace() {
            whitespace = true;
        } else {
            if whitespace {
                output.push(' ');
            }
            output.push(character);
            whitespace = false;
        }
    }
    if whitespace && !output.is_empty() {
        output.push(' ');
    }
    output
}

fn decode_service_documentation_entities(value: &str) -> String {
    let mut output = String::with_capacity(value.len());
    let mut rest = value;
    while let Some(start) = rest.find('&') {
        output.push_str(&rest[..start]);
        let Some(end_offset) = rest[start..].find(';') else {
            output.push_str(&rest[start..]);
            return output;
        };
        let end = start + end_offset;
        let entity = &rest[start + 1..end];
        let decoded = match entity {
            "amp" => Some('&'),
            "lt" => Some('<'),
            "gt" => Some('>'),
            "quot" => Some('"'),
            "apos" => Some('\''),
            "nbsp" => Some(' '),
            _ if entity.starts_with("#x") || entity.starts_with("#X") => {
                u32::from_str_radix(&entity[2..], 16)
                    .ok()
                    .and_then(char::from_u32)
            }
            _ if entity.starts_with('#') => {
                entity[1..].parse::<u32>().ok().and_then(char::from_u32)
            }
            _ => None,
        };
        if let Some(character) = decoded {
            output.push(character);
        } else {
            output.push_str(&rest[start..=end]);
        }
        rest = &rest[end + 1..];
    }
    output.push_str(rest);
    output
}

fn normalize_service_documentation_whitespace(value: &str) -> String {
    let mut lines = Vec::new();
    for line in value.split('\n') {
        let line = line.trim_end_matches([' ', '\t']);
        let trimmed = line.trim_start_matches([' ', '\t']);
        let line = if trimmed.starts_with('-') || trimmed.starts_with('1') {
            line
        } else {
            trimmed
        };
        lines.push(line);
    }
    let mut output = String::new();
    let mut blank_lines = 0;
    for line in lines {
        if line.is_empty() {
            blank_lines += 1;
            if blank_lines <= 1 {
                output.push('\n');
            }
        } else {
            blank_lines = 0;
            output.push_str(line);
            output.push('\n');
        }
    }
    output.trim_matches('\n').to_owned()
}

fn render_standalone_extra_modules(
    output: &mut String,
    selected: &SelectedModel,
    protocol: ProtocolKind,
) {
    let has_aws_chunked = model_has_aws_chunked_operations(selected);
    let has_long_polling = model_has_long_polling_operations(selected);
    let has_query_compatible = service_has_protocol(selected, ProtocolKind::AwsQueryCompatible);
    let has_aws_query = service_has_protocol(selected, ProtocolKind::AwsQuery);
    let has_rest_xml = service_has_protocol(selected, ProtocolKind::RestXml);
    let has_json = service_has_protocol(selected, ProtocolKind::RestJson1)
        || service_has_protocol(selected, ProtocolKind::AwsJson1_0)
        || service_has_protocol(selected, ProtocolKind::AwsJson1_1);
    let has_waiters = has_waiters(selected);
    let has_paginators = has_paginated_operations(selected);
    let has_event_stream = model_has_event_stream(selected);
    let has_wrapped_xml_errors = has_aws_query
        || (has_rest_xml
            && !selected
                .model
                .shapes
                .get(selected.model.service_shape_id.as_str())
                .is_some_and(|service| {
                    has_trait_value(service, "aws.protocols#restXml", "noErrorWrapping")
                }));

    if has_aws_chunked {
        output.push_str("\npub(crate) mod aws_chunked;\n");
    }
    if has_long_polling {
        output.push_str("\npub(crate) mod long_polling;\n");
    }
    if model_contains_string(selected, "AccountId") {
        output.push_str("\nmod account_id_endpoint;\n");
    }
    if has_idempotency_operations(selected) {
        output.push_str("\npub(crate) mod client_idempotency_token;\n");
    }
    if has_event_stream {
        output.push_str("\nmod event_receiver;\n");
    }
    if model_contains_trait(selected, "aws.protocols#httpChecksum") {
        output.push_str(
            "\npub(crate) mod http_request_checksum;\n\npub(crate) mod http_response_checksum;\n",
        );
    }
    if has_idempotency_operations(selected) {
        output.push_str("\nmod idempotency_token;\n");
    }
    output.push_str("\nmod observability_feature;\n");
    if has_presignable_operations(selected) {
        output.push_str("\npub mod presigning;\n\npub(crate) mod presigning_interceptors;\n");
    }
    output.push_str("\npub(crate) mod protocol_serde;\n");
    if has_rest_xml
        && selected
            .model
            .shapes
            .get(selected.model.service_shape_id.as_str())
            .is_some_and(|service| {
                has_trait_value(service, "aws.protocols#restXml", "noErrorWrapping")
            })
    {
        output.push_str("\nmod rest_xml_unwrapped_errors;\n");
    }
    if selected
        .operations
        .iter()
        .filter_map(|name| operation_shape(selected, name))
        .any(|operation| operation_has_s3_expires_output(selected, operation))
    {
        output.push_str("\nmod s3_expires_interceptor;\n");
    }
    if service_supports_s3_express(selected) {
        output.push_str("\nmod s3_express;\n");
    }
    if request_id_plan(selected).extended {
        output.push_str("\nmod s3_request_id;\n");
    }
    output.push_str("\nmod sdk_feature_tracker;\n\nmod serialization_settings;\n");
    if has_aws_chunked {
        output.push_str("\npub(crate) mod endpoint_auth;\n");
    }
    output.push_str("\nmod endpoint_lib;\n");
    if has_paginated_operations(selected) {
        output.push_str("\nmod lens;\n");
    }
    if protocol == ProtocolKind::AwsJson1_1 && has_json {
        output.push_str("\nmod json_errors;\n");
    }
    if has_wrapped_xml_errors && !has_waiters && !has_paginators {
        output.push_str("\nmod rest_xml_wrapped_errors;\n");
    }
    output.push_str("\nmod serde_util;\n");
    if has_waiters {
        output.push_str("\n/// Supporting types for waiters.\n///\n/// Note: to use waiters, import the [`Waiters`](crate::client::Waiters) trait, which adds methods prefixed with `wait_until` to the client.\npub mod waiters;\n");
    }
    if has_wrapped_xml_errors && has_waiters {
        output.push_str("\nmod rest_xml_wrapped_errors;\n");
    }
    if has_event_stream && !has_rest_xml {
        output.push_str("\nmod event_stream_serde;\n");
    }
    if has_query_compatible {
        output.push_str("\nmod aws_query_compatible_errors;\n");
    }
    if has_event_stream && has_rest_xml {
        output.push_str("\nmod event_stream_serde;\n");
    }
    if (has_json || has_query_compatible) && protocol != ProtocolKind::AwsJson1_1 {
        output.push_str("\nmod json_errors;\n");
    }
    if has_wrapped_xml_errors && !has_waiters && has_paginators {
        output.push_str("\nmod rest_xml_wrapped_errors;\n");
    }
}

fn service_has_endpoint_rules(selected: &SelectedModel) -> bool {
    selected
        .model
        .shapes
        .get(selected.model.service_shape_id.as_str())
        .and_then(|service| service.get("traits"))
        .is_some_and(|traits| {
            traits.get("smithy.rules#endpointBdd").is_some()
                || traits.get("smithy.rules#endpointRuleSet").is_some()
        })
}

fn service_has_rest_xml_unwrapped_errors(selected: &SelectedModel) -> bool {
    service_has_protocol(selected, ProtocolKind::RestXml)
        && selected
            .model
            .shapes
            .get(selected.model.service_shape_id.as_str())
            .is_some_and(|service| {
                has_trait_value(service, "aws.protocols#restXml", "noErrorWrapping")
            })
}

fn endpoint_rule_function_ids(selected: &SelectedModel) -> BTreeSet<String> {
    let mut functions = BTreeSet::new();
    let Some(traits) = selected
        .model
        .shapes
        .get(selected.model.service_shape_id.as_str())
        .and_then(|service| service.get("traits"))
    else {
        return functions;
    };
    for trait_id in [
        "smithy.rules#endpointBdd",
        "smithy.rules#endpointRuleSet",
        "smithy.rules#endpointTests",
    ] {
        collect_endpoint_rule_functions(traits.get(trait_id), &mut functions);
    }
    functions
}

fn collect_endpoint_rule_functions(value: Option<&Value>, functions: &mut BTreeSet<String>) {
    let Some(value) = value else {
        return;
    };
    match value {
        Value::Object(object) => {
            if let Some(function) = object.get("fn").and_then(Value::as_str) {
                functions.insert(function.to_owned());
            }
            for value in object.values() {
                collect_endpoint_rule_functions(Some(value), functions);
            }
        }
        Value::Array(values) => {
            for value in values {
                collect_endpoint_rule_functions(Some(value), functions);
            }
        }
        _ => {}
    }
}

fn endpoint_lib_module_names(selected: &SelectedModel) -> Vec<String> {
    let functions = endpoint_rule_function_ids(selected);
    let mut modules = vec![
        "bdd_interpreter".to_owned(),
        "diagnostic".to_owned(),
        "partition".to_owned(),
        "host".to_owned(),
    ];
    if functions.contains("aws.parseArn") {
        modules.insert(0, "arn".to_owned());
    }
    if functions.contains("coalesce") {
        modules.push("coalesce".to_owned());
    }
    if functions.contains("ite") {
        modules.push("ite".to_owned());
    }
    if functions.contains("parseURL") {
        modules.push("parse_url".to_owned());
    }
    if functions.contains("aws.isVirtualHostableS3Bucket") {
        modules.push("s3".to_owned());
    }
    if functions.contains("split") {
        modules.push("split".to_owned());
    }
    if functions.contains("substring") {
        modules.push("substring".to_owned());
    }
    if functions.contains("uriEncode") {
        modules.push("uri_encode".to_owned());
    }

    // The inlineable module tree is emitted in dependency insertion order. The
    // richer endpoint rule sets add the standard library modules before the AWS
    // partition resolver, while the minimal AWS rulesets register partition
    // support before the host-label customization.
    if functions.contains("coalesce") || functions.contains("parseURL") {
        modules.sort();
    }
    modules
}

fn render_endpoint_lib(selected: &SelectedModel) -> String {
    let partition_json = include_str!("../assets/default-partitions.json").trim();
    let escaped_partition_json = partition_json.replace('\\', "\\\\").replace('"', "\\\"");
    let mut output = String::new();
    output.push_str(
        "// Loading the partition JSON is expensive since it involves many regex compilations,\n\
         // so cache the result so that it only need to be paid for the first constructed client.\n\
         pub(crate) static DEFAULT_PARTITION_RESOLVER: std::sync::LazyLock<crate::endpoint_lib::partition::PartitionResolver> = std::sync::LazyLock::new(\n\
             || match std::env::var(\"SMITHY_CLIENT_SDK_CUSTOM_PARTITION\") {\n\
                 Ok(partitions) => {\n\
                     ::tracing::debug!(\"loading custom partitions located at {partitions}\");\n\
                     let partition_dot_json = std::fs::read_to_string(partitions).expect(\"should be able to read a custom partition JSON\");\n\
                     crate::endpoint_lib::partition::PartitionResolver::new_from_json(partition_dot_json.as_bytes()).expect(\"valid JSON\")\n\
                 }\n\
                 _ => {\n\
                     ::tracing::debug!(\"loading default partitions\");\n\
                     crate::endpoint_lib::partition::PartitionResolver::new_from_json(b\"",
    );
    output.push_str(&escaped_partition_json);
    output.push_str(
        "\").expect(\"valid JSON\")\n\
                 }\n\
             },\n\
         );\n\n\
",
    );
    for module in endpoint_lib_module_names(selected) {
        writeln!(output, "pub(crate) mod {module};").unwrap();
        output.push('\n');
    }
    output
}

fn render_endpoint_lib_module(module: &str) -> String {
    let source = match module {
        "arn" => include_str!("../assets/endpoint_lib/arn.rs"),
        "bdd_interpreter" => include_str!("../assets/endpoint_lib/bdd_interpreter.rs"),
        "coalesce" => include_str!("../assets/endpoint_lib/coalesce.rs"),
        "diagnostic" => include_str!("../assets/endpoint_lib/diagnostic.rs"),
        "host" => include_str!("../assets/endpoint_lib/host.rs"),
        "ite" => include_str!("../assets/endpoint_lib/ite.rs"),
        "parse_url" => include_str!("../assets/endpoint_lib/parse_url.rs"),
        "partition" => include_str!("../assets/endpoint_lib/partition.rs"),
        "s3" => include_str!("../assets/endpoint_lib/s3.rs"),
        "split" => include_str!("../assets/endpoint_lib/split.rs"),
        "substring" => include_str!("../assets/endpoint_lib/substring.rs"),
        "uri_encode" => include_str!("../assets/endpoint_lib/uri_encode.rs"),
        _ => panic!("unknown endpoint library module: {module}"),
    };
    let mut output = String::new();
    client_operation_header(&mut output);
    output.push_str(source);
    output
}

fn service_has_protocol(selected: &SelectedModel, protocol: ProtocolKind) -> bool {
    selected
        .model
        .shapes
        .get(selected.model.service_shape_id.as_str())
        .and_then(|service| service.get("traits"))
        .and_then(Value::as_object)
        .is_some_and(|traits| traits.contains_key(protocol.trait_id()))
}

fn model_has_aws_chunked_operations(selected: &SelectedModel) -> bool {
    selected.operations.iter().any(|operation_name| {
        let Some(operation) = operation_shape(selected, operation_name) else {
            return false;
        };
        let input = operation
            .get("input")
            .and_then(target_value)
            .and_then(|id| selected.model.shapes.get(id));
        operation_requires_aws_chunked(selected, operation, input)
    })
}

fn model_has_long_polling_operations(selected: &SelectedModel) -> bool {
    selected.operations.iter().any(|operation_name| {
        let Some(operation) = operation_shape(selected, operation_name) else {
            return false;
        };
        if has_trait(operation, "aws.api#longPoll") {
            return true;
        }
        let Some(input) = operation
            .get("input")
            .and_then(target_value)
            .and_then(|id| selected.model.shapes.get(id))
        else {
            return false;
        };
        members(input)
            .into_iter()
            .any(|(name, _)| name.eq_ignore_ascii_case("waitTimeSeconds"))
    })
}

fn model_contains_trait(selected: &SelectedModel, trait_id: &str) -> bool {
    selected.model.shapes.values().any(|shape| {
        has_trait(shape, trait_id)
            || shape
                .get("members")
                .and_then(Value::as_object)
                .is_some_and(|members| members.values().any(|member| has_trait(member, trait_id)))
    })
}

fn model_contains_string(selected: &SelectedModel, needle: &str) -> bool {
    selected
        .model
        .shapes
        .values()
        .any(|shape| value_contains_string(shape, needle))
}

fn has_trait_value(value: &Value, trait_id: &str, field: &str) -> bool {
    value
        .get("traits")
        .and_then(Value::as_object)
        .and_then(|traits| traits.get(trait_id))
        .and_then(Value::as_object)
        .is_some_and(|fields| fields.contains_key(field))
}

fn render_serde_util_file(selected: &SelectedModel) -> String {
    let mut output = String::new();
    client_operation_header(&mut output);
    let mut nested_order = Vec::new();
    let mut nested_seen = BTreeSet::new();
    let mut emitted = BTreeSet::new();

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
            let function_name = format!("{module}_output_output_correct_errors");
            if emitted.insert(function_name.clone()) {
                render_serde_util_correction(
                    &mut output,
                    selected,
                    shape,
                    &function_name,
                    &format!("crate::operation::{module}::builders::{operation_type}OutputBuilder"),
                );
            }
            serde_util_walk_correction_dependencies(
                selected,
                output_id,
                &mut nested_seen,
                &mut nested_order,
            );
        }
        if let Some(errors) = operation.get("errors").and_then(Value::as_array) {
            for error_id in errors.iter().filter_map(target_value) {
                let Some(shape) = selected.model.shapes.get(error_id) else {
                    continue;
                };
                if serde_util_shape_needs_correction(shape) {
                    let name = rust_type_name(terminal(error_id));
                    let function_name = format!(
                        "{}_correct_errors",
                        names::rust_module_name(terminal(error_id))
                    );
                    if emitted.insert(function_name.clone()) {
                        render_serde_util_correction(
                            &mut output,
                            selected,
                            shape,
                            &function_name,
                            &format!("crate::types::error::builders::{name}Builder"),
                        );
                    }
                    serde_util_walk_correction_dependencies(
                        selected,
                        error_id,
                        &mut nested_seen,
                        &mut nested_order,
                    );
                }
            }
        }
    }

    for shape_id in nested_order
        .into_iter()
        .chain(serde_util_protocol_correction_order(selected))
    {
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
            let function_name = format!(
                "{}_correct_errors",
                names::rust_module_name(terminal(&shape_id))
            );
            if emitted.insert(function_name.clone()) {
                render_serde_util_correction(
                    &mut output,
                    selected,
                    shape,
                    &function_name,
                    &builder_path,
                );
            }
        }
    }
    output
}

fn serde_util_walk_correction_dependencies(
    selected: &SelectedModel,
    shape_id: &str,
    seen: &mut BTreeSet<String>,
    order: &mut Vec<String>,
) {
    let Some(shape) = selected.model.shapes.get(shape_id) else {
        return;
    };
    for (_, member) in members(shape) {
        if !member_is_required(member) {
            continue;
        }
        let Some(target) = member_target(member) else {
            continue;
        };
        if !selected
            .model
            .shapes
            .get(target)
            .is_some_and(serde_util_shape_needs_correction)
        {
            continue;
        }
        if seen.insert(target.to_owned()) {
            order.push(target.to_owned());
            serde_util_walk_correction_dependencies(selected, target, seen, order);
        }
    }
}

fn serde_util_protocol_correction_order(selected: &SelectedModel) -> Vec<String> {
    let query_mode = selected.model.protocol().is_ok_and(is_query_protocol);
    let roles = protocol_serde_roles(selected, query_mode);
    let mut order = Vec::new();
    for wave in protocol_serde_shape_waves(selected, &roles, query_mode) {
        for (shape_id, role) in wave {
            if !matches!(role, ProtocolSerdeRole::Deserialize) {
                continue;
            }
            let Some(shape) = selected.model.shapes.get(&shape_id) else {
                continue;
            };
            if serde_util_shape_needs_correction(shape) {
                order.push(shape_id);
            }
        }
    }
    order
}

fn serde_util_shape_needs_correction(shape: &Value) -> bool {
    shape.get("type").and_then(Value::as_str) == Some("structure")
        && members(shape)
            .iter()
            .any(|(_, member)| member_is_required(member))
}

fn serde_util_builder_is_fallible(selected: &SelectedModel, shape: &Value) -> bool {
    members(shape).iter().any(|(_, member)| {
        member_target(member)
            .is_some_and(|target| member_is_effectively_required(selected, member, target))
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

"#,
    )
}

fn render_primitives(selected: &SelectedModel) -> String {
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

fn model_has_event_stream(selected: &SelectedModel) -> bool {
    selected.model.shapes.values().any(|shape| {
        shape.get("type").and_then(Value::as_str) == Some("union") && shape_is_streaming(shape)
    })
}

fn event_stream_union_ids(selected: &SelectedModel) -> Vec<String> {
    selected
        .model
        .shapes
        .iter()
        .filter_map(|(id, shape)| {
            (shape.get("type").and_then(Value::as_str) == Some("union")
                && shape_is_streaming(shape))
            .then_some(id.clone())
        })
        .collect()
}

fn render_event_stream_serde_file(selected: &SelectedModel) -> String {
    let mut output = String::new();
    client_operation_header(&mut output);
    let mut union_ids = event_stream_union_ids(selected);
    union_ids.sort();
    let input_unions = event_stream_input_union_ids(selected);
    let output_unions = event_stream_output_union_ids(selected);
    for union_id in union_ids {
        let Some(shape) = selected.model.shapes.get(&union_id) else {
            continue;
        };
        let union_name = rust_type_name(terminal(&union_id));
        if input_unions.contains(&union_id) {
            render_event_stream_error_marshaller(&mut output, &union_name);
            render_event_stream_marshaller(&mut output, selected, &union_name, &union_id, shape);
        }
        if !output_unions.contains(&union_id) {
            continue;
        }
        let unmarshaller_name = format!("{union_name}Unmarshaller");
        writeln!(
            output,
            "#[non_exhaustive]\n#[derive(Debug)]\npub struct {unmarshaller_name};\n\nimpl {unmarshaller_name} {{\n    pub fn new() -> Self {{\n        {unmarshaller_name}\n    }}\n}}"
        )
        .unwrap();
        writeln!(
            output,
            "impl ::aws_smithy_eventstream::frame::UnmarshallMessage for {unmarshaller_name} {{\n    type Output = crate::types::{union_name};\n    type Error = crate::types::error::{union_name}Error;\n    fn unmarshall(\n        &self,\n        message: &::aws_smithy_types::event_stream::Message,\n    ) -> std::result::Result<::aws_smithy_eventstream::frame::UnmarshalledMessage<Self::Output, Self::Error>, ::aws_smithy_eventstream::error::Error>\n    {{\n        let response_headers = ::aws_smithy_eventstream::smithy::parse_response_headers(message)?;\n        match response_headers.message_type.as_str() {{\n            \"event\" => match response_headers.smithy_type.as_str() {{"
        )
        .unwrap();
        for (member_name, member) in members(shape) {
            if selected
                .model
                .shapes
                .get(member_target(member).unwrap_or_default())
                .is_some_and(is_error_shape)
            {
                continue;
            }
            render_event_stream_member(&mut output, selected, &union_name, &member_name, member);
        }
        output.push_str(
            "                _unknown_variant => Ok(::aws_smithy_eventstream::frame::UnmarshalledMessage::Event(\n                    crate::types::",
        );
        output.push_str(&union_name);
        output.push_str(
            "::Unknown,\n                )),\n            },\n            \"exception\" => {\n",
        );
        output.push_str("                let generic = match crate::protocol_serde::parse_event_stream_error_metadata(message.payload()) {\n                    Ok(builder) => builder.build(),\n                    Err(err) => {\n                        return Ok(::aws_smithy_eventstream::frame::UnmarshalledMessage::Error(\n                            crate::types::error::");
        output.push_str(&union_name);
        output.push_str("Error::unhandled(err),\n                        ))\n                    }\n                };\n                match response_headers.smithy_type.as_str() {\n");
        for (member_name, member) in members(shape) {
            let Some(target) = member_target(member) else {
                continue;
            };
            if selected
                .model
                .shapes
                .get(target)
                .is_some_and(is_error_shape)
            {
                render_event_stream_error_member(
                    &mut output,
                    selected,
                    &union_name,
                    &member_name,
                    target,
                );
            }
        }
        output.push_str("                    _ => {}\n                }\n                Ok(::aws_smithy_eventstream::frame::UnmarshalledMessage::Error(\n                    crate::types::error::");
        output.push_str(&union_name);
        output.push_str("Error::generic(generic),\n                ))\n            }\n            value => {\n                return Err(::aws_smithy_eventstream::error::Error::unmarshalling(format!(\n                    \"unrecognized :message-type: {value}\"\n                )));\n            }\n        }\n    }\n}\n");
    }
    output
}

fn event_stream_input_union_ids(selected: &SelectedModel) -> BTreeSet<String> {
    selected
        .operations
        .iter()
        .filter_map(|operation_name| operation_shape(selected, operation_name))
        .filter_map(|operation| operation.get("input").and_then(target_value))
        .filter_map(|input_id| selected.model.shapes.get(input_id))
        .flat_map(members)
        .filter_map(|(_, member)| {
            has_trait(member, "smithy.api#httpPayload")
                .then(|| member_target(member).map(ToOwned::to_owned))
                .flatten()
        })
        .filter(|target| is_event_stream_target(selected, target))
        .collect()
}

fn event_stream_output_union_ids(selected: &SelectedModel) -> BTreeSet<String> {
    selected
        .operations
        .iter()
        .filter_map(|operation_name| operation_shape(selected, operation_name))
        .filter_map(|operation| operation.get("output").and_then(target_value))
        .filter_map(|output_id| selected.model.shapes.get(output_id))
        .flat_map(members)
        .filter_map(|(_, member)| {
            has_trait(member, "smithy.api#httpPayload")
                .then(|| member_target(member).map(ToOwned::to_owned))
                .flatten()
        })
        .filter(|target| is_event_stream_target(selected, target))
        .collect()
}

fn render_event_stream_error_marshaller(output: &mut String, union_name: &str) {
    writeln!(
        output,
        "#[non_exhaustive]\n#[derive(Debug)]\npub struct {union_name}InputErrorMarshaller;\n\nimpl {union_name}InputErrorMarshaller {{\n    pub fn new() -> Self {{\n        {union_name}InputErrorMarshaller\n    }}\n}}\nimpl ::aws_smithy_eventstream::frame::MarshallMessage for {union_name}InputErrorMarshaller {{\n    type Input = crate::types::error::{union_name}InputError;\n    fn marshall(\n        &self,\n        _input: Self::Input,\n    ) -> std::result::Result<::aws_smithy_types::event_stream::Message, ::aws_smithy_eventstream::error::Error> {{\n        let mut headers = Vec::new();\n        headers.push(::aws_smithy_types::event_stream::Header::new(\n            \":message-type\",\n            ::aws_smithy_types::event_stream::HeaderValue::String(\"exception\".into()),\n        ));\n        let payload = ::bytes::Bytes::new();\n        Ok(::aws_smithy_types::event_stream::Message::new_from_parts(headers, payload))\n    }}\n}}\n"
    )
    .unwrap();
}

fn render_event_stream_marshaller(
    output: &mut String,
    selected: &SelectedModel,
    union_name: &str,
    union_id: &str,
    shape: &Value,
) {
    writeln!(
        output,
        "#[non_exhaustive]\n#[derive(Debug)]\npub struct {union_name}InputMarshaller;\n\nimpl {union_name}InputMarshaller {{\n    pub fn new() -> Self {{\n        {union_name}InputMarshaller\n    }}\n}}\nimpl ::aws_smithy_eventstream::frame::MarshallMessage for {union_name}InputMarshaller {{\n    type Input = crate::types::{union_name};\n    fn marshall(&self, input: Self::Input) -> std::result::Result<::aws_smithy_types::event_stream::Message, ::aws_smithy_eventstream::error::Error> {{\n        let mut headers = Vec::new();\n        headers.push(::aws_smithy_types::event_stream::Header::new(\n            \":message-type\",\n            ::aws_smithy_types::event_stream::HeaderValue::String(\"event\".into()),\n        ));\n        let payload = match input {{"
    )
    .unwrap();
    let type_name = format!("crate::types::{union_name}");
    for (member_name, member) in members(shape) {
        let variant = rust_type_name(&member_name);
        let target = member_target(member).unwrap_or("smithy.api#Unit");
        let target_shape = selected.model.shapes.get(target);
        let target_kind = protocol_shape_kind(selected, target);
        let has_inner = target_kind != "unit";
        let pattern = if has_inner {
            format!("Self::Input::{variant}(inner)")
        } else {
            format!("Self::Input::{variant}")
        };
        writeln!(output, "            {pattern} => {{").unwrap();
        writeln!(
            output,
            "                headers.push(::aws_smithy_types::event_stream::Header::new(\":event-type\", ::aws_smithy_types::event_stream::HeaderValue::String({member_name:?}.into())));"
        )
        .unwrap();
        let payload = render_event_stream_marshaller_payload(
            output,
            selected,
            union_id,
            &member_name,
            target,
            target_shape,
            has_inner,
        );
        writeln!(output, "                {payload}").unwrap();
        output.push_str("            }\n");
    }
    writeln!(
        output,
        "            Self::Input::Unknown => return Err(\n                ::aws_smithy_eventstream::error::Error::marshalling(\"Cannot serialize `{union_name}::Unknown` for the request. The `Unknown` variant is intended for responses only. It occurs when an outdated client is used after a new enum variant was added on the server side.\".to_owned())\n            )\n        }};\n        Ok(::aws_smithy_types::event_stream::Message::new_from_parts(headers, payload))\n    }}\n}}"
    )
    .unwrap();
    let _ = type_name;
}

fn render_event_stream_marshaller_payload(
    output: &mut String,
    selected: &SelectedModel,
    union_id: &str,
    member_name: &str,
    target: &str,
    target_shape: Option<&Value>,
    has_inner: bool,
) -> String {
    let Some(target_shape) = target_shape else {
        return "::bytes::Bytes::new()".to_owned();
    };
    let event_payload = members(target_shape)
        .into_iter()
        .find(|(_, member)| has_trait(member, "smithy.api#eventPayload"));
    if let Some((field_name, payload_member)) = event_payload {
        let field = names::rust_identifier(&field_name);
        let payload_target = member_target(payload_member).unwrap_or("smithy.api#Unit");
        let kind = protocol_shape_kind(selected, payload_target);
        let input = if has_inner {
            format!("inner.{field}")
        } else {
            "crate::types::Unit::builder().build()".to_owned()
        };
        let content_type = match kind {
            "blob" => "application/octet-stream",
            "string" | "enum" => "text/plain",
            _ => "application/json",
        };
        output.push_str(&format!(
            "                headers.push(::aws_smithy_types::event_stream::Header::new(\":content-type\", ::aws_smithy_types::event_stream::HeaderValue::String(\"{content_type}\".into())));\n"
        ));
        return match kind {
            "blob" => format!(
                "::bytes::Bytes::from(::aws_smithy_types::Blob::from({input}).into_bytes())"
            ),
            "string" | "enum" => format!("::bytes::Bytes::from({input}.to_string().into_bytes())"),
            _ => format!(
                "::bytes::Bytes::from(crate::protocol_serde::shape_{}_input::ser_{}_payload(&{input}).map_err(|err| ::aws_smithy_eventstream::error::Error::marshalling(format!(\"{{err}}\")))?)",
                names::rust_module_name(terminal(union_id)),
                names::rust_identifier(member_name),
            ),
        };
    }
    if target == "smithy.api#Unit" || (members(target_shape).is_empty() && !has_inner) {
        return "::bytes::Bytes::new()".to_owned();
    }
    output.push_str(&format!(
        "                headers.push(::aws_smithy_types::event_stream::Header::new(\":content-type\", ::aws_smithy_types::event_stream::HeaderValue::String(\"application/json\".into())));\n"
    ));
    format!(
        "::bytes::Bytes::from(crate::protocol_serde::shape_{}_input::ser_{}_payload(&inner).map_err(|err| ::aws_smithy_eventstream::error::Error::marshalling(format!(\"{{err}}\")))?)",
        names::rust_module_name(terminal(union_id)),
        names::rust_identifier(member_name),
    )
}

fn render_event_stream_error_member(
    output: &mut String,
    selected: &SelectedModel,
    union_name: &str,
    member_name: &str,
    target: &str,
) {
    let variant = rust_type_name(member_name);
    let error_name = rust_type_name(terminal(target));
    let module = names::rust_module_name(terminal(target));
    writeln!(
        output,
        "                    {member_name:?} => {{\n                        let mut builder = crate::types::error::builders::{error_name}Builder::default();\n                        builder = crate::protocol_serde::shape_{module}::de_{module}_json_err(\n                            &message.payload()[..],\n                            builder,\n                        )\n                        .map_err(|err| {{\n                            ::aws_smithy_eventstream::error::Error::unmarshalling(format!(\"failed to unmarshall {member_name}: {{err}}\"))\n                        }})?;\n                        builder.set_meta(Some(generic));\n                        return Ok(::aws_smithy_eventstream::frame::UnmarshalledMessage::Error(\n                            crate::types::error::{union_name}Error::{variant}(builder.build()),\n                        ));\n                    }}"
    )
    .unwrap();
    let _ = selected;
}

fn render_event_stream_member(
    output: &mut String,
    selected: &SelectedModel,
    union_name: &str,
    member_name: &str,
    member: &Value,
) {
    let variant = rust_type_name(member_name);
    let target = member_target(member).unwrap_or("smithy.api#Unit");
    let Some(target_shape) = selected.model.shapes.get(target) else {
        return;
    };
    let target_name = rust_type_name(terminal(target));
    writeln!(output, "                {member_name:?} => {{").unwrap();
    let event_members = members(target_shape);
    let payload = event_members
        .iter()
        .find(|(_, event_member)| has_trait(event_member, "smithy.api#eventPayload"));
    if target == "smithy.api#Unit" {
        writeln!(
            output,
            "                    Ok(::aws_smithy_eventstream::frame::UnmarshalledMessage::Event(\n                        crate::types::{union_name}::{variant},\n                    ))"
        )
        .unwrap();
    } else if event_members.is_empty() {
        writeln!(
            output,
            "                    Ok(::aws_smithy_eventstream::frame::UnmarshalledMessage::Event(\n                        crate::types::{union_name}::{variant}(crate::types::{target_name}::builder().build()),\n                    ))"
        )
        .unwrap();
    } else if let Some((payload_name, payload_member)) = payload {
        render_event_stream_explicit_payload(
            output,
            selected,
            union_name,
            &variant,
            &target_name,
            payload_name,
            payload_member,
        );
    } else {
        let module = names::rust_module_name(terminal(target));
        writeln!(
            output,
            "                    let parsed =\n                        crate::protocol_serde::shape_{module}::de_{module}_payload(&message.payload()[..])\n                            .map_err(|err| {{\n                                ::aws_smithy_eventstream::error::Error::unmarshalling(format!(\"failed to unmarshall {member_name}: {{err}}\"))\n                            }})?\n                        ;\n                    Ok(::aws_smithy_eventstream::frame::UnmarshalledMessage::Event(\n                        crate::types::{union_name}::{variant}(parsed),\n                    ))"
        )
        .unwrap();
    }
    output.push_str("                }\n");
}

fn render_event_stream_explicit_payload(
    output: &mut String,
    selected: &SelectedModel,
    union_name: &str,
    variant: &str,
    event_name: &str,
    payload_name: &str,
    payload_member: &Value,
) {
    let field = names::rust_identifier(payload_name);
    let payload_target = member_target(payload_member).unwrap_or("smithy.api#Unit");
    let payload_kind = selected
        .model
        .shapes
        .get(payload_target)
        .and_then(|shape| shape.get("type"))
        .and_then(Value::as_str)
        .or_else(|| payload_target.strip_prefix("smithy.api#"))
        .unwrap_or("structure");
    writeln!(
        output,
        "                    let mut builder = crate::types::builders::{event_name}Builder::default();"
    )
    .unwrap();
    match payload_kind {
        "blob" => {
            output.push_str(
                "                    let content_type = response_headers.content_type().unwrap_or_default();\n                    if content_type != \"application/octet-stream\" {\n                        return Err(::aws_smithy_eventstream::error::Error::unmarshalling(format!(\n                            \"expected :content-type to be 'application/octet-stream', but was '{content_type}'\"\n                        )));\n                    }\n",
            );
            writeln!(
                output,
                "                    builder = builder.set_{field}(Some(::aws_smithy_types::Blob::from_maybe_shared(message.payload().clone())));"
            )
            .unwrap();
        }
        "string" => {
            output.push_str(
                "                    let content_type = response_headers.content_type().unwrap_or_default();\n                    if content_type != \"text/plain\" {\n                        return Err(::aws_smithy_eventstream::error::Error::unmarshalling(format!(\n                            \"expected :content-type to be 'text/plain', but was '{content_type}'\"\n                        )));\n                    }\n",
            );
            writeln!(
                output,
                "                    builder = builder.set_{field}(Some(::std::str::from_utf8(message.payload()).map_err(|_| ::aws_smithy_eventstream::error::Error::unmarshalling(\"message payload is not valid UTF-8\"))?.to_owned()));"
            )
            .unwrap();
        }
        _ => {
            let module = names::rust_module_name(terminal(event_name));
            writeln!(
                output,
                "                    builder = builder.set_{field}(Some(\n                        crate::protocol_serde::shape_{module}::de_{field}(&message.payload()[..])\n                            .map_err(|err| ::aws_smithy_eventstream::error::Error::unmarshalling(format!(\"failed to unmarshall {field}: {{err}}\")))?,\n                    ));"
            )
            .unwrap();
        }
    }
    writeln!(
        output,
        "                    Ok(::aws_smithy_eventstream::frame::UnmarshalledMessage::Event(\n                        crate::types::{union_name}::{variant}(builder.build()),\n                    ))"
    )
    .unwrap();
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

fn render_config_file(selected: &SelectedModel) -> String {
    render_standalone_config_file(selected)
}

fn render_standalone_config_file(selected: &SelectedModel) -> String {
    let mut output = include_str!("../assets/config_base.rs.in").to_owned();
    let checksums = model_contains_trait(selected, "aws.protocols#httpChecksum");
    let s3_express = service_supports_s3_express(selected);
    let sigv4a = service_uses_sigv4a(selected);
    let idempotency = has_idempotency_operations(selected);
    let account_id_endpoint =
        service_has_endpoint_builtin(selected, "AWS::Auth::AccountIdEndpointMode");
    let dynamodb_retry = account_id_endpoint;
    let aws_chunked = model_has_aws_chunked_operations(selected);

    replace_config_placeholder(
        &mut output,
        "__S3_TOP_BLANK__",
        if s3_express {
            "\n".to_owned()
        } else {
            String::new()
        },
    );
    replace_config_placeholder(
        &mut output,
        "__S3_CONFIG_BLANK__",
        if s3_express {
            "\n".to_owned()
        } else {
            String::new()
        },
    );

    replace_config_placeholder(
        &mut output,
        "__CONFIG_CHECKSUM_GETTERS__",
        if checksums {
            r#"    /// Return a reference to the response_checksum_validation value contained in this config, if any.
    pub fn response_checksum_validation(&self) -> ::std::option::Option<&crate::config::ResponseChecksumValidation> {
        self.config.load::<crate::config::ResponseChecksumValidation>()
    }
    /// Return a reference to the request_checksum_calculation value contained in this config, if any.
    pub fn request_checksum_calculation(&self) -> ::std::option::Option<&crate::config::RequestChecksumCalculation> {
        self.config.load::<crate::config::RequestChecksumCalculation>()
    }"#
                .to_owned()
        } else {
            String::new()
        },
    );
    replace_config_placeholder(
        &mut output,
        "__CONFIG_SIGV4A_GETTER__",
        if sigv4a {
            r#"    /// Returns the SigV4a signing region set, if configured.
    pub fn sigv4a_signing_region_set(&self) -> Option<&::aws_types::region::SigningRegionSet> {
        self.config.load::<::aws_types::region::SigningRegionSet>()
    }"#
            .to_owned()
        } else {
            String::new()
        },
    );
    replace_config_placeholder(
        &mut output,
        "__CONFIG_S3_BUILTINS__",
        render_s3_config_bag(selected),
    );
    replace_config_placeholder(
        &mut output,
        "__CONFIG_ACCOUNT_ID__",
        if account_id_endpoint {
            "        builder.set_account_id_endpoint_mode(config_bag.load::<::aws_types::endpoint_config::AccountIdEndpointMode>().cloned());".to_owned()
        } else {
            String::new()
        },
    );
    replace_config_placeholder(
        &mut output,
        "__CONFIG_CHECKSUMS__",
        if checksums {
            "        builder.set_response_checksum_validation(config_bag.load::<crate::config::ResponseChecksumValidation>().cloned());\n        builder.set_request_checksum_calculation(config_bag.load::<crate::config::RequestChecksumCalculation>().cloned());".to_owned()
        } else {
            String::new()
        },
    );
    replace_config_placeholder(
        &mut output,
        "__CONFIG_SIGV4A__",
        if sigv4a {
            "        builder.set_sigv4a_signing_region_set(config_bag.load::<::aws_types::region::SigningRegionSet>().cloned());".to_owned()
        } else {
            String::new()
        },
    );
    replace_config_placeholder(
        &mut output,
        "__IDEMPOTENCY_BUILDER__",
        if idempotency {
            render_idempotency_builder()
        } else {
            String::new()
        },
    );
    replace_config_placeholder(&mut output, "__S3_BUILDER__", render_s3_builder(selected));
    replace_config_placeholder(
        &mut output,
        "__S3_EXPRESS_BUILDER__",
        if s3_express {
            render_s3_express_builder()
        } else {
            String::new()
        },
    );
    replace_config_placeholder(
        &mut output,
        "__ACCOUNT_ID_BUILDER__",
        if account_id_endpoint {
            render_account_id_builder()
        } else {
            String::new()
        },
    );
    replace_config_placeholder(
        &mut output,
        "__CONFIG_CHECKSUM_BUILDER__",
        if checksums {
            render_checksum_builder()
        } else {
            String::new()
        },
    );
    replace_config_placeholder(
        &mut output,
        "__CONFIG_SIGV4A_BUILDER__",
        if sigv4a {
            render_sigv4a_builder()
        } else {
            String::new()
        },
    );
    replace_config_placeholder(
        &mut output,
        "__SIGV4A_CREDENTIALS__",
        if sigv4a {
            "            #[cfg(feature = \"sigv4a\")]\n            {\n                self.runtime_components\n                    .set_identity_resolver(::aws_runtime::auth::sigv4a::SCHEME_ID, credentials_provider.clone());\n            }".to_owned()
        } else {
            String::new()
        },
    );
    replace_config_placeholder(
        &mut output,
        "__AWS_CHUNKED_BUILDER__",
        if aws_chunked {
            render_aws_chunked_builder()
        } else {
            String::new()
        },
    );
    replace_config_placeholder(
        &mut output,
        "__IDEMPOTENCY_TEST_DEFAULT__",
        if idempotency {
            "        self.set_idempotency_token_provider(Some(\"00000000-0000-4000-8000-000000000000\".into()));".to_owned()
        } else {
            String::new()
        },
    );
    replace_config_placeholder(
        &mut output,
        "__S3_IDEMPOTENCY_BLANK__",
        if s3_express {
            "\n".to_owned()
        } else {
            String::new()
        },
    );
    replace_config_placeholder(
        &mut output,
        "__S3_TEST_V2_BLANK__",
        if s3_express {
            "\n".to_owned()
        } else {
            String::new()
        },
    );
    replace_config_placeholder(
        &mut output,
        "__S3_BUILD_BLANK__",
        if s3_express {
            "\n".to_owned()
        } else {
            String::new()
        },
    );
    replace_config_placeholder(
        &mut output,
        "__IDEMPOTENCY_RUNTIME_CONFIG__",
        if idempotency {
            "            cfg.store_put(crate::idempotency_token::default_provider());".to_owned()
        } else {
            String::new()
        },
    );
    replace_config_placeholder(
        &mut output,
        "__S3_EXPRESS_AUTH_SCHEME__",
        if s3_express {
            "        runtime_components.push_auth_scheme(::aws_smithy_runtime_api::client::auth::SharedAuthScheme::new(\n            crate::s3_express::auth::S3ExpressAuthScheme::new(),\n        ));".to_owned()
        } else {
            String::new()
        },
    );
    replace_config_placeholder(
        &mut output,
        "__ACCOUNT_ID_INTERCEPTOR__",
        if account_id_endpoint {
            "        runtime_components.push_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(\n            crate::account_id_endpoint::AccountIdEndpointFeatureTrackerInterceptor,\n        ));".to_owned()
        } else {
            String::new()
        },
    );
    replace_config_placeholder(
        &mut output,
        "__SIGV4A_RUNTIME_AUTH_SCHEME__",
        if sigv4a {
            "        #[cfg(feature = \"sigv4a\")]\n        {\n            runtime_components.push_auth_scheme(::aws_smithy_runtime_api::client::auth::SharedAuthScheme::new(\n                ::aws_runtime::auth::sigv4a::SigV4aAuthScheme::new(),\n            ));\n        }".to_owned()
        } else {
            String::new()
        },
    );
    replace_config_placeholder(
        &mut output,
        "__S3_EXPRESS_RUNTIME_PLUGIN__",
        if s3_express {
            "\n    plugins = plugins.with_client_plugin(crate::s3_express::runtime_plugin::S3ExpressRuntimePlugin::new(config.clone()));".to_owned()
        } else {
            String::new()
        },
    );
    replace_config_placeholder(
        &mut output,
        "__SDK_CONFIG_SIGV4A__",
        if sigv4a {
            "        builder.set_sigv4a_signing_region_set(input.sigv4a_signing_region_set().cloned());".to_owned()
        } else {
            String::new()
        },
    );
    replace_config_placeholder(
        &mut output,
        "__SDK_CONFIG_CHECKSUMS__",
        if checksums {
            "        builder.set_request_checksum_calculation(input.request_checksum_calculation());\n        builder.set_response_checksum_validation(input.response_checksum_validation());".to_owned()
        } else {
            String::new()
        },
    );
    replace_config_placeholder(
        &mut output,
        "__SDK_CONFIG_ACCOUNT_ID__",
        if account_id_endpoint {
            "        builder.set_account_id_endpoint_mode(input.account_id_endpoint_mode().cloned());".to_owned()
        } else {
            String::new()
        },
    );
    replace_config_placeholder(
        &mut output,
        "__SDK_CONFIG_S3_EXPRESS__",
        if s3_express {
            format!(
                "        builder.set_disable_s3_express_session_auth(input.service_config().and_then(|conf| {{\n            let str_config = conf.load_config(service_config_key(\n                \"{}\",\n                \"AWS_S3_DISABLE_EXPRESS_SESSION_AUTH\",\n                \"s3_disable_express_session_auth\",\n            ));\n            str_config.and_then(|it| it.parse::<bool>().ok())\n        }}));",
                service_sdk_id(selected)
            )
        } else {
            String::new()
        },
    );
    replace_config_placeholder(
        &mut output,
        "__SDK_CONFIG_DYNAMODB_RETRY__",
        if dynamodb_retry {
            r#"        if let ::std::option::Option::Some(existing_rc) = input.retry_config().cloned() {
            if existing_rc
                .retry_spec()
                .is_some_and(|s| s.is_at_least(::aws_smithy_types::retry::RetrySpec::V2_1))
            {
                let mut rc = existing_rc.with_retry_spec(
                    ::aws_smithy_types::retry::RetrySpec::v2_1().with_non_throttling_initial_backoff(::std::time::Duration::from_millis(25)),
                );
                if !input.get_origin("retry_config").is_client_config() {
                    rc = rc.with_max_attempts(4);
                }
                builder = builder.retry_config(rc);
            }
        }"#
                .to_owned()
        } else {
            String::new()
        },
    );
    replace_config_placeholder(
        &mut output,
        "__CONFIG_CHECKSUM_EXPORTS__",
        if checksums {
            "\npub use ::aws_smithy_types::checksum_config::ResponseChecksumValidation;\n\npub use ::aws_smithy_types::checksum_config::RequestChecksumCalculation;".to_owned()
        } else {
            String::new()
        },
    );
    replace_config_placeholder(
        &mut output,
        "__S3_STORABLES__",
        render_s3_storables(selected),
    );

    output = output
        .replace("__SDK_CRATE__", selected.model.entry.module_name)
        .replace(
            "__SERVICE_SHAPE__",
            terminal(selected.model.service_shape_id.as_str()),
        )
        .replace("__SERVICE_TITLE__", &service_sdk_id(selected))
        .replace("__SERVICE_KEY__", selected.model.entry.key);
    output
}

fn replace_config_placeholder(output: &mut String, marker: &str, value: String) {
    let marker_line = format!("{marker}\n");
    if value.is_empty() {
        *output = output.replace(&marker_line, "").replace(marker, "");
    } else {
        *output = output.replace(marker, &value);
    }
}

fn selected_service(selected: &SelectedModel) -> Option<&Value> {
    selected
        .model
        .shapes
        .get(selected.model.service_shape_id.as_str())
}

fn service_has_endpoint_builtin(selected: &SelectedModel, built_in: &str) -> bool {
    selected_service(selected)
        .and_then(|service| service.get("traits"))
        .and_then(Value::as_object)
        .into_iter()
        .flat_map(|traits| traits.values())
        .filter_map(|trait_value| trait_value.get("parameters"))
        .filter_map(Value::as_object)
        .flat_map(|parameters| parameters.values())
        .any(|parameter| {
            parameter
                .get("builtIn")
                .and_then(Value::as_str)
                .is_some_and(|value| value == built_in)
        })
}

fn service_has_client_context_param(selected: &SelectedModel, name: &str) -> bool {
    selected_service(selected)
        .and_then(|service| service.get("traits"))
        .and_then(|traits| traits.get("smithy.rules#clientContextParams"))
        .and_then(Value::as_object)
        .is_some_and(|params| params.contains_key(name))
}

fn service_uses_sigv4a(selected: &SelectedModel) -> bool {
    service_supports_s3_express(selected)
        || selected_service(selected).is_some_and(|service| {
            has_trait(service, "aws.auth#sigv4a")
                || service
                    .get("traits")
                    .and_then(|traits| traits.get("smithy.api#auth"))
                    .and_then(Value::as_array)
                    .is_some_and(|auth| {
                        auth.iter().any(|id| id.as_str() == Some("aws.auth#sigv4a"))
                    })
        })
}

fn render_s3_config_bag(selected: &SelectedModel) -> String {
    let mut output = String::new();
    let controls = [
        (
            "ForcePathStyle",
            "ForcePathStyle",
            "        builder.set_force_path_style(config_bag.load::<crate::config::ForcePathStyle>().map(|ty| ty.0));",
        ),
        (
            "UseArnRegion",
            "UseArnRegion",
            "        builder.set_use_arn_region(config_bag.load::<crate::config::UseArnRegion>().map(|ty| ty.0));",
        ),
        (
            "DisableMultiRegionAccessPoints",
            "DisableMultiRegionAccessPoints",
            "        builder.set_disable_multi_region_access_points(config_bag.load::<crate::config::DisableMultiRegionAccessPoints>().map(|ty| ty.0));",
        ),
        (
            "Accelerate",
            "Accelerate",
            "        builder.set_accelerate(config_bag.load::<crate::config::Accelerate>().map(|ty| ty.0));",
        ),
        (
            "DisableS3ExpressSessionAuth",
            "DisableS3ExpressSessionAuth",
            "        builder.set_disable_s3_express_session_auth(config_bag.load::<crate::config::DisableS3ExpressSessionAuth>().map(|ty| ty.0));",
        ),
    ];
    for (name, _rust_name, line) in controls {
        if service_has_client_context_param(selected, name) {
            if !output.is_empty() {
                output.push('\n');
            }
            output.push_str(line);
            if name != "DisableS3ExpressSessionAuth" {
                output.push('\n');
            }
        }
    }
    output
}

fn render_idempotency_builder() -> String {
    r#"    /// Sets the idempotency token provider to use for service calls that require tokens.
    pub fn idempotency_token_provider(
        mut self,
        idempotency_token_provider: impl ::std::convert::Into<crate::idempotency_token::IdempotencyTokenProvider>,
    ) -> Self {
        self.set_idempotency_token_provider(::std::option::Option::Some(idempotency_token_provider.into()));
        self
    }
    /// Sets the idempotency token provider to use for service calls that require tokens.
    pub fn set_idempotency_token_provider(
        &mut self,
        idempotency_token_provider: ::std::option::Option<crate::idempotency_token::IdempotencyTokenProvider>,
    ) -> &mut Self {
        self.config.store_or_unset(idempotency_token_provider);
        self
    }"#
        .to_owned()
}

fn render_s3_builder(selected: &SelectedModel) -> String {
    let mut output = String::new();
    let controls = [
        (
            "ForcePathStyle",
            r#"    /// Forces this client to use path-style addressing for buckets.
    pub fn force_path_style(mut self, force_path_style: impl Into<bool>) -> Self {
        self.set_force_path_style(Some(force_path_style.into()));
        self
    }
    /// Forces this client to use path-style addressing for buckets.
    pub fn set_force_path_style(&mut self, force_path_style: Option<bool>) -> &mut Self {
        self.config.store_or_unset(force_path_style.map(crate::config::ForcePathStyle));
        self
    }

"#,
        ),
        (
            "UseArnRegion",
            r#"    /// Enables this client to use an ARN's region when constructing an endpoint instead of the client's configured region.
    pub fn use_arn_region(mut self, use_arn_region: impl Into<bool>) -> Self {
        self.set_use_arn_region(Some(use_arn_region.into()));
        self
    }
    /// Enables this client to use an ARN's region when constructing an endpoint instead of the client's configured region.
    pub fn set_use_arn_region(&mut self, use_arn_region: Option<bool>) -> &mut Self {
        self.config.store_or_unset(use_arn_region.map(crate::config::UseArnRegion));
        self
    }

"#,
        ),
        (
            "DisableMultiRegionAccessPoints",
            r#"    /// Disables this client's usage of Multi-Region Access Points.
    pub fn disable_multi_region_access_points(mut self, disable_multi_region_access_points: impl Into<bool>) -> Self {
        self.set_disable_multi_region_access_points(Some(disable_multi_region_access_points.into()));
        self
    }
    /// Disables this client's usage of Multi-Region Access Points.
    pub fn set_disable_multi_region_access_points(&mut self, disable_multi_region_access_points: Option<bool>) -> &mut Self {
        self.config
            .store_or_unset(disable_multi_region_access_points.map(crate::config::DisableMultiRegionAccessPoints));
        self
    }

"#,
        ),
        (
            "Accelerate",
            r#"    /// Enables this client to use S3 Transfer Acceleration endpoints.
    pub fn accelerate(mut self, accelerate: impl Into<bool>) -> Self {
        self.set_accelerate(Some(accelerate.into()));
        self
    }
    /// Enables this client to use S3 Transfer Acceleration endpoints.
    pub fn set_accelerate(&mut self, accelerate: Option<bool>) -> &mut Self {
        self.config.store_or_unset(accelerate.map(crate::config::Accelerate));
        self
    }

"#,
        ),
        (
            "DisableS3ExpressSessionAuth",
            r#"    /// Disables this client's usage of Session Auth for S3Express       buckets and reverts to using conventional SigV4 for those.
    pub fn disable_s3_express_session_auth(mut self, disable_s3_express_session_auth: impl Into<bool>) -> Self {
        self.set_disable_s3_express_session_auth(Some(disable_s3_express_session_auth.into()));
        self
    }
    /// Disables this client's usage of Session Auth for S3Express       buckets and reverts to using conventional SigV4 for those.
    pub fn set_disable_s3_express_session_auth(&mut self, disable_s3_express_session_auth: Option<bool>) -> &mut Self {
        self.config
            .store_or_unset(disable_s3_express_session_auth.map(crate::config::DisableS3ExpressSessionAuth));
        self
    }
"#,
        ),
    ];
    for (name, block) in controls {
        if service_has_client_context_param(selected, name) {
            output.push_str(block);
        }
    }
    output.trim_end_matches('\n').to_owned()
}

fn render_s3_express_builder() -> String {
    r#"    /// Sets the credentials provider for S3 Express One Zone
    pub fn express_credentials_provider(mut self, credentials_provider: impl crate::config::ProvideCredentials + 'static) -> Self {
        self.set_express_credentials_provider(::std::option::Option::Some(crate::config::SharedCredentialsProvider::new(
            credentials_provider,
        )));
        self
    }
    /// Sets the credentials provider for S3 Express One Zone
    pub fn set_express_credentials_provider(
        &mut self,
        credentials_provider: ::std::option::Option<crate::config::SharedCredentialsProvider>,
    ) -> &mut Self {
        if let ::std::option::Option::Some(credentials_provider) = credentials_provider {
            self.runtime_components
                .set_identity_resolver(crate::s3_express::auth::SCHEME_ID, credentials_provider);
        }
        self
    }"#
        .to_owned()
}

fn render_account_id_builder() -> String {
    r#"    /// The AccountId Endpoint Mode.
    pub fn account_id_endpoint_mode(mut self, account_id_endpoint_mode: ::aws_types::endpoint_config::AccountIdEndpointMode) -> Self {
        self.set_account_id_endpoint_mode(::std::option::Option::Some(account_id_endpoint_mode));
        self
    }
    /// The AccountId Endpoint Mode.
    pub fn set_account_id_endpoint_mode(
        &mut self,
        account_id_endpoint_mode: ::std::option::Option<::aws_types::endpoint_config::AccountIdEndpointMode>,
    ) -> &mut Self {
        self.config.store_or_unset(account_id_endpoint_mode);
        self
    }"#
        .to_owned()
}

fn render_checksum_builder() -> String {
    r#"    /// Set the [`ResponseChecksumValidation`](crate::config::ResponseChecksumValidation)
    /// to determine when checksum validation will be performed on response payloads.
    pub fn response_checksum_validation(mut self, response_checksum_validation: crate::config::ResponseChecksumValidation) -> Self {
        self.set_response_checksum_validation(::std::option::Option::Some(response_checksum_validation));
        self
    }
    /// Set the [`ResponseChecksumValidation`](crate::config::ResponseChecksumValidation)
    /// to determine when checksum validation will be performed on response payloads.
    pub fn set_response_checksum_validation(
        &mut self,
        response_checksum_validation: ::std::option::Option<crate::config::ResponseChecksumValidation>,
    ) -> &mut Self {
        self.config.store_or_unset(response_checksum_validation);
        self
    }
    /// Set the [`RequestChecksumCalculation`](crate::config::RequestChecksumCalculation)
    /// to determine when a checksum will be calculated for request payloads.
    pub fn request_checksum_calculation(mut self, request_checksum_calculation: crate::config::RequestChecksumCalculation) -> Self {
        self.set_request_checksum_calculation(::std::option::Option::Some(request_checksum_calculation));
        self
    }
    /// Set the [`RequestChecksumCalculation`](crate::config::RequestChecksumCalculation)
    /// to determine when a checksum will be calculated for request payloads.
    pub fn set_request_checksum_calculation(
        &mut self,
        request_checksum_calculation: ::std::option::Option<crate::config::RequestChecksumCalculation>,
    ) -> &mut Self {
        self.config.store_or_unset(request_checksum_calculation);
        self
    }"#
        .to_owned()
}

fn render_sigv4a_builder() -> String {
    r#"    /// Sets the SigV4a signing region set.
    pub fn sigv4a_signing_region_set(mut self, v: impl Into<::aws_types::region::SigningRegionSet>) -> Self {
        self.set_sigv4a_signing_region_set(Some(v.into()));
        self
    }

    /// Sets the SigV4a signing region set.
    pub fn set_sigv4a_signing_region_set(&mut self, v: Option<::aws_types::region::SigningRegionSet>) -> &mut Self {
        self.config.store_or_unset(v);
        self
    }"#
        .to_owned()
}

fn render_aws_chunked_builder() -> String {
    r#"    /// Sets the chunk size for [`aws-chunked encoding`].
    ///
    /// Pass `Some(size)` to use a specific chunk size (minimum 8 KiB).
    /// Pass `None` to use the content-length as chunk size (no chunking).
    ///
    /// The minimum chunk size of 8 KiB is validated when the request is sent.
    ///
    /// **Note:** This setting only applies to operations that support aws-chunked encoding
    /// and has no effect on other operations. If this method is not invoked, a default
    /// chunk size of 64 KiB is used.
    ///
    /// [`aws-chunked encoding`]: https://docs.aws.amazon.com/AmazonS3/latest/API/sigv4-streaming.html
    ///
    /// # Example - Custom chunk size
    /// ```no_run
    /// # use __SDK_CRATE__::{Client, Config};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::builder()
    ///     .aws_chunked_encoding_chunk_size(Some(10240)) // 10 KiB chunks
    ///     .build();
    /// let client = Client::from_conf(config);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Example - No chunking (buffers entire body in memory)
    /// ```no_run
    /// # use __SDK_CRATE__::{Client, Config};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::builder()
    ///     .aws_chunked_encoding_chunk_size(None) // Use entire content as one chunk
    ///     .build();
    /// let client = Client::from_conf(config);
    /// # Ok(())
    /// # }
    /// ```
    pub fn aws_chunked_encoding_chunk_size(mut self, chunk_size: ::std::option::Option<usize>) -> Self {
        self.set_aws_chunked_encoding_chunk_size(::std::option::Option::Some(chunk_size));
        self
    }

    /// Sets the chunk size for aws-chunked encoding.
    pub fn set_aws_chunked_encoding_chunk_size(&mut self, chunk_size: ::std::option::Option<::std::option::Option<usize>>) -> &mut Self {
        if let ::std::option::Option::Some(chunk_size) = chunk_size {
            let chunk_size = match chunk_size {
                ::std::option::Option::Some(size) => crate::aws_chunked::ChunkSize::Configured(size),
                ::std::option::Option::None => crate::aws_chunked::ChunkSize::DisableChunking,
            };
            self.push_runtime_plugin(crate::aws_chunked::ChunkSizeRuntimePlugin::new(chunk_size).into_shared());
        }
        self
    }"#
        .to_owned()
}

fn render_s3_storables(selected: &SelectedModel) -> String {
    let controls = [
        (
            "ForcePathStyle",
            r#"#[derive(Debug, Clone)]
pub(crate) struct ForcePathStyle(pub(crate) bool);
impl ::aws_smithy_types::config_bag::Storable for ForcePathStyle {
    type Storer = ::aws_smithy_types::config_bag::StoreReplace<Self>;
}
"#,
        ),
        (
            "UseArnRegion",
            r#"#[derive(Debug, Clone)]
pub(crate) struct UseArnRegion(pub(crate) bool);
impl ::aws_smithy_types::config_bag::Storable for UseArnRegion {
    type Storer = ::aws_smithy_types::config_bag::StoreReplace<Self>;
}
"#,
        ),
        (
            "DisableMultiRegionAccessPoints",
            r#"#[derive(Debug, Clone)]
pub(crate) struct DisableMultiRegionAccessPoints(pub(crate) bool);
impl ::aws_smithy_types::config_bag::Storable for DisableMultiRegionAccessPoints {
    type Storer = ::aws_smithy_types::config_bag::StoreReplace<Self>;
}
"#,
        ),
        (
            "Accelerate",
            r#"#[derive(Debug, Clone)]
pub(crate) struct Accelerate(pub(crate) bool);
impl ::aws_smithy_types::config_bag::Storable for Accelerate {
    type Storer = ::aws_smithy_types::config_bag::StoreReplace<Self>;
}
"#,
        ),
        (
            "DisableS3ExpressSessionAuth",
            r#"#[derive(Debug, Clone)]
pub(crate) struct DisableS3ExpressSessionAuth(pub(crate) bool);
impl ::aws_smithy_types::config_bag::Storable for DisableS3ExpressSessionAuth {
    type Storer = ::aws_smithy_types::config_bag::StoreReplace<Self>;
}
"#,
        ),
    ];
    let mut output = String::new();
    for block in controls
        .into_iter()
        .filter(|(name, _)| service_has_client_context_param(selected, name))
        .map(|(_, block)| block)
    {
        if !output.is_empty() {
            output.push('\n');
        }
        output.push_str(block);
    }
    if output.is_empty() {
        output
    } else {
        format!("\n{output}")
    }
}

fn render_error_file(has_enum: bool) -> String {
    let mut output = String::new();
    {
        render_standalone_error(&mut output, has_enum);
    }
    output
}

fn render_auth_file(selected: &SelectedModel) -> String {
    let service = selected
        .model
        .shapes
        .get(selected.model.service_shape_id.as_str())
        .expect("selected service shape exists");
    let title = service_title(selected);
    let service_options = auth_options_for_service(selected, service);
    let operation_overrides = auth_operation_overrides(selected);
    let resolve_allow = { "" };
    let endpoint_auth = if service_supports_s3_express(selected) {
        "        let _fut = ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionsFuture::new(async move {\n            crate::endpoint_auth::resolve_endpoint_based_auth_scheme_options(modeled_auth_options, _cfg, _runtime_components).await\n        });\n\n"
    } else {
        ""
    };
    let mut output = String::new();
    client_operation_header(&mut output);
    writeln!(
        output,
        "/// Auth scheme resolver trait specific to {title}\n\
         pub trait ResolveAuthScheme: ::std::marker::Send + ::std::marker::Sync + ::std::fmt::Debug {{\n\
             /// Resolve a priority list of auth scheme options with the given parameters\n\
             fn resolve_auth_scheme<'a>(\n\
                 &'a self,\n\
                 params: &'a crate::config::auth::Params,\n\
                 cfg: &'a ::aws_smithy_types::config_bag::ConfigBag,\n\
                 runtime_components: &'a ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponents,\n\
             ) -> ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionsFuture<'a>;\n\n\
             /// Convert this service-specific resolver into a `SharedAuthSchemeOptionResolver`\n\
             fn into_shared_resolver(self) -> ::aws_smithy_runtime_api::client::auth::SharedAuthSchemeOptionResolver\n\
             where\n\
                 Self: ::std::marker::Sized + 'static,\n\
             {{\n\
                 ::aws_smithy_runtime_api::client::auth::SharedAuthSchemeOptionResolver::new(DowncastParams(self))\n\
             }}\n\
         }}\n\n\
         #[derive(Debug)]\n\
         struct DowncastParams<T>(T);\n\
         impl<T> ::aws_smithy_runtime_api::client::auth::ResolveAuthSchemeOptions for DowncastParams<T>\n\
         where\n\
             T: ResolveAuthScheme,\n\
         {{\n\
             fn resolve_auth_scheme_options_v2<'a>(\n\
                 &'a self,\n\
                 params: &'a ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams,\n\
                 cfg: &'a ::aws_smithy_types::config_bag::ConfigBag,\n\
                 runtime_components: &'a ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponents,\n\
             ) -> ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionsFuture<'a> {{\n\
                 match params.get::<crate::config::auth::Params>() {{\n\
                     ::std::option::Option::Some(concrete_params) => self.0.resolve_auth_scheme(concrete_params, cfg, runtime_components),\n\
                     ::std::option::Option::None => ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionsFuture::ready(::std::result::Result::Err(\n\
                         \"params of expected type was not present\".into(),\n\
                     )),\n\
                 }}\n\
             }}\n\
         }}\n\n\
         /// The default auth scheme resolver\n\
         #[derive(Debug)]\n\
         #[allow(dead_code)]\n\
         pub struct DefaultAuthSchemeResolver {{\n\
             service_defaults: Vec<::aws_smithy_runtime_api::client::auth::AuthSchemeOption>,\n\
             operation_overrides: ::std::collections::HashMap<&'static str, Vec<::aws_smithy_runtime_api::client::auth::AuthSchemeOption>>,\n\
         }}\n\n\
         // TODO(https://github.com/smithy-lang/smithy-rs/issues/4177): Remove `allow(...)` once the issue is addressed.\n\
         // When generating code for tests (e.g., `codegen-client-test`), this manual implementation\n\
         // of the `Default` trait may appear as if it could be derived automatically.\n\
         // However, that is not the case in production.\n\
         #[allow(clippy::derivable_impls)]\n\
         impl Default for DefaultAuthSchemeResolver {{\n\
             fn default() -> Self {{\n\
                 Self {{\n\
                     service_defaults: {service_options},\n\
                     operation_overrides: {operation_overrides},\n\
                 }}\n\
             }}\n\
         }}\n\n\
         impl crate::config::auth::ResolveAuthScheme for DefaultAuthSchemeResolver {{\n\
{resolve_allow}             fn resolve_auth_scheme<'a>(\n\
                 &'a self,\n\
                 params: &'a crate::config::auth::Params,\n\
                 _cfg: &'a ::aws_smithy_types::config_bag::ConfigBag,\n\
                 _runtime_components: &'a ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponents,\n\
             ) -> ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionsFuture<'a> {{\n\
                 let operation_name = params.operation_name();\n\n\
                 let modeled_auth_options = match self.operation_overrides.get(operation_name) {{\n\
                     Some(overrides) => overrides,\n\
                     None => &self.service_defaults,\n\
                 }};\n\n\
                 let _fut = ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionsFuture::ready(Ok(modeled_auth_options.clone()));\n\n{endpoint_auth}                 _fut\n\
             }}\n\
         }}\n\n\
         /// Configuration parameters for resolving the correct auth scheme\n\
         #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]\n\
         pub struct Params {{\n\
             operation_name: ::std::borrow::Cow<'static, str>,\n\
         }}\n\
         impl Params {{\n\
             /// Create a builder for [`Params`]\n\
             pub fn builder() -> crate::config::auth::ParamsBuilder {{\n\
                 crate::config::auth::ParamsBuilder::default()\n\
             }}\n\n\
             /// Return the operation name for [`Params`]\n\
             pub fn operation_name(&self) -> &str {{\n\
                 self.operation_name.as_ref()\n\
             }}\n\
         }}\n\n\
         #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]\n\
         /// Builder for [`Params`]\n\
         pub struct ParamsBuilder {{\n\
             operation_name: ::std::option::Option<::std::borrow::Cow<'static, str>>,\n\
         }}\n\
         impl ParamsBuilder {{\n\
             /// Set the operation name for the builder\n\
             pub fn operation_name(self, operation_name: impl Into<::std::borrow::Cow<'static, str>>) -> Self {{\n\
                 self.set_operation_name(::std::option::Option::Some(operation_name.into()))\n\
             }}\n\n\
             /// Set the operation name for the builder\n\
             pub fn set_operation_name(mut self, operation_name: ::std::option::Option<::std::borrow::Cow<'static, str>>) -> Self {{\n\
                 self.operation_name = operation_name;\n\
                 self\n\
             }}\n\
             /// Consume this builder, create [`Params`].\"\n\
             ///\n\
             /// Return [`BuildError`] if any of the required fields are unset.\n\
             ///\n\
             pub fn build(self) -> ::std::result::Result<crate::config::auth::Params, crate::config::auth::BuildError> {{\n\
                 ::std::result::Result::Ok(crate::config::auth::Params {{\n\
                     operation_name: self.operation_name.ok_or_else(|| BuildError::missing(\"operation_name\"))?,\n\
                 }})\n\
             }}\n\
         }}\n\n\
         /// An error that occurred while constructing `config::auth::Params`\n\
         #[derive(Debug)]\n\
         pub struct BuildError {{\n\
             field: ::std::borrow::Cow<'static, str>,\n\
         }}\n\n\
         impl BuildError {{\n\
             fn missing(field: &'static str) -> Self {{\n\
                 Self {{ field: field.into() }}\n\
             }}\n\
         }}\n\n\
         impl std::fmt::Display for BuildError {{\n\
             fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{\n\
                 write!(f, \"a required field was missing: `{{}}`\", self.field)\n\
             }}\n\
         }}\n\n\
         impl std::error::Error for BuildError {{}}\n"
    )
    .unwrap();

    output
}

fn auth_options_for_service(selected: &SelectedModel, service: &Value) -> String {
    let mut ids = service
        .get("traits")
        .and_then(Value::as_object)
        .and_then(|traits| traits.get("smithy.api#auth"))
        .and_then(Value::as_array)
        .map(|auth| {
            auth.iter()
                .filter_map(Value::as_str)
                .map(ToOwned::to_owned)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    if ids.is_empty() {
        for id in ["aws.auth#sigv4", "aws.auth#sigv4a"] {
            if has_trait(service, id) {
                ids.push(id.to_owned());
            }
        }
    }
    if service_supports_s3_express(selected) && !ids.iter().any(|id| id == "aws.auth#sigv4a") {
        ids.push("aws.auth#sigv4a".to_owned());
    }
    if service_supports_s3_express(selected) {
        ids.push("smithy.api#noAuth".to_owned());
    }
    render_auth_option_vec(&ids, None, None, true)
}

fn auth_operation_overrides(selected: &SelectedModel) -> String {
    let mut overrides = Vec::new();
    for operation_name in &selected.operations {
        let Some(operation) = operation_shape(selected, operation_name) else {
            continue;
        };
        let Some(auth) = operation
            .get("traits")
            .and_then(Value::as_object)
            .and_then(|traits| traits.get("smithy.api#auth"))
        else {
            continue;
        };
        let ids = auth
            .as_array()
            .map(|values| {
                values
                    .iter()
                    .filter_map(Value::as_str)
                    .map(ToOwned::to_owned)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        if ids.is_empty() {
            overrides.push(format!(
                "({operation_name:?}, vec![{}])",
                render_no_auth_option()
            ));
        } else {
            overrides.push(format!(
                "({operation_name:?}, {})",
                render_auth_option_vec(&ids, Some(operation_name), Some(operation), false,)
            ));
        }
    }
    if overrides.is_empty() {
        "::std::collections::HashMap::new()".to_owned()
    } else {
        format!("[{}].into()", overrides.join(", "))
    }
}

fn render_auth_option_vec(
    ids: &[String],
    operation_name: Option<&str>,
    operation: Option<&Value>,
    service_defaults: bool,
) -> String {
    let options = ids
        .iter()
        .map(|id| render_auth_option(id, operation_name, operation, service_defaults))
        .filter(|option| !option.is_empty())
        .collect::<Vec<_>>();
    format!("vec![{}]", options.join(", "))
}

fn render_auth_option(
    id: &str,
    operation_name: Option<&str>,
    operation: Option<&Value>,
    service_defaults: bool,
) -> String {
    if id == "smithy.api#noAuth" {
        return render_no_auth_option();
    }
    let (module, cfg) = match id {
        "aws.auth#sigv4" => ("sigv4", false),
        "aws.auth#sigv4a" => ("sigv4a", true),
        _ => {
            return "::aws_smithy_runtime_api::client::auth::AuthSchemeOption::from(\"unknown\")"
                .to_owned();
        }
    };

    let properties = operation
        .filter(|operation| has_trait(operation, "aws.auth#unsignedPayload"))
        .map(|_| {
            let operation_name = operation_name.unwrap_or_default();
            format!(
                ".properties({{ let mut layer = ::aws_smithy_types::config_bag::Layer::new(\"{operation_name}AuthOptionProperties\"); layer.store_put(::aws_runtime::auth::PayloadSigningOverride::unsigned_payload()); layer.freeze() }})"
            )
        })
        .unwrap_or_default();
    let expression = format!(
        "::aws_smithy_runtime_api::client::auth::AuthSchemeOption::builder().scheme_id(::aws_runtime::auth::{module}::SCHEME_ID){properties}.build().expect(\"required fields set\")"
    );
    if cfg && service_defaults {
        format!("#[cfg(feature = \"sigv4a\")] {{ {expression} }}")
    } else {
        expression
    }
}

fn render_no_auth_option() -> String {
    let scheme_id = { "::aws_smithy_runtime::client::auth::no_auth::NO_AUTH_SCHEME_ID" };
    format!("::aws_smithy_runtime_api::client::auth::AuthSchemeOption::from({scheme_id})")
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
    let version_path = { "crate::meta::PKG_VERSION" };
    if matches!(service_key, "dynamodb" | "lambda") {
        writeln!(
            output,
            "pub(crate) static API_METADATA: ::aws_runtime::user_agent::ApiMetadata =\n    ::aws_runtime::user_agent::ApiMetadata::new(\"{service_key}\", {version_path});"
        )
        .unwrap();
    } else {
        writeln!(
            output,
            "pub(crate) static API_METADATA: ::aws_runtime::user_agent::ApiMetadata = ::aws_runtime::user_agent::ApiMetadata::new(\"{service_key}\", {version_path});"
        )
        .unwrap();
    }
    output.push_str("\n/// Crate version number.\npub static PKG_VERSION: &str = env!(\"CARGO_PKG_VERSION\");\n");
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

fn render_types_file(selected: &SelectedModel) -> String {
    let mut output = String::new();
    client_operation_header(&mut output);

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
            (id != selected.model.service_shape_id.as_str()
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

fn render_types_builders_file(selected: &SelectedModel) -> String {
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
        let path = { format!("crate::types::{module}::{name}Builder") };
        writeln!(output, "pub use {path};\n").unwrap();
    }
    output
}

fn render_error_types_file(selected: &SelectedModel) -> String {
    let mut output = String::new();
    client_operation_header(&mut output);

    for id in error_shape_ids(selected) {
        let name = rust_type_name(terminal(&id));
        let module = type_file_name(&id).trim_end_matches(".rs").to_owned();
        let path = { format!("crate::types::error::{module}::{name}") };
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
        render_event_stream_error(&mut output, &id);
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

fn render_event_stream_error(output: &mut String, union_id: &str) {
    let error_name = format!("{}Error", rust_type_name(terminal(union_id)));
    let error_type_path = { format!("crate::types::error::{error_name}") };
    let request_id_path = { "crate::s3_request_id".to_owned() };
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

fn service_crate_name(service_key: &str) -> String {
    format!("aws-sdk-{service_key}")
}

fn service_title(selected: &SelectedModel) -> String {
    selected
        .model
        .shapes
        .get(selected.model.service_shape_id.as_str())
        .and_then(|shape| shape.get("traits"))
        .and_then(Value::as_object)
        .and_then(|traits| traits.get("smithy.api#title"))
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| terminal(selected.model.service_shape_id.as_str()).to_owned())
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

fn render_error_builders_file(selected: &SelectedModel) -> String {
    let mut output = String::new();
    client_operation_header(&mut output);
    for id in error_shape_ids(selected) {
        let name = rust_type_name(terminal(&id));
        let module = type_file_name(&id).trim_end_matches(".rs").to_owned();
        let path = { format!("crate::types::error::{module}::{name}Builder") };
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
        .service_shape_id
        .as_str()
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
        .service_shape_id
        .as_str()
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

fn render_type_file(selected: &SelectedModel, shape_id: &str, context: Context) -> String {
    let mut rendered = String::new();
    render_types_with_context(&mut rendered, selected, context, Some(shape_id));
    let marker = "pub mod types {\n";
    let start = rendered.find(marker).expect("type module must be rendered") + marker.len();
    let end = rendered
        .rfind("\n}\n\n")
        .expect("type module must have a closing brace");
    let mut output = String::new();
    header(&mut output);
    if rendered[start..]
        .trim_start()
        .starts_with("#[allow(missing_docs)] // documentation missing in model")
    {
        output.pop();
    }
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
) {
    header(output);
    output.push_str("pub mod types {\n");
    let mut ids = selected.model.shapes.keys().cloned().collect::<Vec<_>>();
    ids.sort();
    for id in ids {
        if id == selected.model.service_shape_id.as_str()
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
            Some("enum") => render_enum(output, shape, terminal(&id)),
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

fn render_operations_file(selected: &SelectedModel) -> String {
    render_standalone_operations_file(selected)
}

fn render_standalone_operations_file(selected: &SelectedModel) -> String {
    let mut output = String::new();
    client_operation_header(&mut output);
    output.push_str("pub use ::aws_types::request_id::RequestId;\n\n");
    if request_id_plan(selected).extended {
        output.push_str("pub use crate::s3_request_id::RequestIdExt;\n\n");
    }

    let mut operations = selected.operations.clone();
    operations.sort();
    for operation_name in operations {
        let module = names::snake_case(&operation_name);
        writeln!(
            output,
            "/// Types for the `{operation_name}` operation.\npub mod {module};"
        )
        .unwrap();
        output.push('\n');
    }
    output
}

fn render_operation_file(selected: &SelectedModel, operation_name: &str) -> String {
    render_standalone_operation_file(selected, operation_name)
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
    let operation_type = operation_error_type_name(operation_name);
    let shape_type = rust_type_name(operation_name);
    let service_id = service_sdk_id(selected);
    let input_path = format!("crate::operation::{module}::{shape_type}Input");
    let output_path = format!("crate::operation::{module}::{shape_type}Output");
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
        &shape_type,
        &error_path,
    );
    if !operation_has_telemetry_members(selected, operation) {
        output.push('\n');
    }
    if operation_has_telemetry_members(selected, operation) {
        render_standalone_telemetry_interceptor(
            &mut output,
            selected,
            operation_name,
            &operation_type,
            &shape_type,
        );
    }
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
        &shape_type,
        &error_path,
    );
    render_standalone_endpoint_interceptor(
        &mut output,
        selected,
        operation_name,
        &operation_type,
        &shape_type,
    );
    render_standalone_operation_error(&mut output, selected, operation_name, operation);
    writeln!(
        output,
        "\npub use crate::operation::{module}::_{module}_input::{shape_type}Input;\n\npub use crate::operation::{module}::_{module}_output::{shape_type}Output;\n\nmod _{module}_input;\n\nmod _{module}_output;\n\n/// Builders\npub mod builders;"
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
        .get(selected.model.service_shape_id.as_str())
        .and_then(|shape| shape.get("traits"))
        .and_then(|traits| traits.get("aws.api#service"))
        .and_then(|service| service.get("sdkId"))
        .and_then(Value::as_str)
        .unwrap_or_else(|| terminal(selected.model.service_shape_id.as_str()))
        .to_owned()
}

fn operation_has_telemetry_members(selected: &SelectedModel, operation: &Value) -> bool {
    let Some(input_shape) = operation
        .get("input")
        .and_then(target_value)
        .and_then(|id| selected.model.shapes.get(id))
    else {
        return false;
    };
    if has_trait(input_shape, "smithy.api#sensitive") {
        return false;
    }
    members(input_shape).into_iter().any(|(_, member)| {
        let Some(target) = member_target(member) else {
            return false;
        };
        !has_trait(member, "smithy.api#sensitive")
            && !selected.model.shapes.get(target).is_some_and(|shape| {
                has_trait(shape, "smithy.api#sensitive")
                    || shape.get("type").and_then(Value::as_str) == Some("enum")
            })
            && is_string_type(target, selected.model.shapes.get(target))
    })
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
        .operations
        .iter()
        .any(|operation| operation == "CreateSession")
        && selected
            .model
            .shapes
            .get(selected.model.service_shape_id.as_str())
            .and_then(|shape| shape.get("traits"))
            .and_then(|traits| traits.get("smithy.rules#endpointRuleSet"))
            .is_some_and(|rules| value_contains_string(rules, "sigv4-s3express"))
}

fn service_uses_s3_sigv4_overrides(selected: &SelectedModel) -> bool {
    endpoint_rule_function_ids(selected).contains("aws.isVirtualHostableS3Bucket")
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
        .get(selected.model.service_shape_id.as_str())
        .and_then(|shape| shape.get("traits"))
        .and_then(|traits| {
            traits
                .get("smithy.rules#endpointRuleSet")
                .or_else(|| traits.get("smithy.rules#endpointBdd"))
        })
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
    let protocol = selected.model.protocol().expect("selected protocol exists");
    if is_query_protocol(protocol) {
        return (
            format!(
                "::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_{module}_input::ser_{module}_input_input_input(&input)?)"
            ),
            Some("application/x-www-form-urlencoded".to_owned()),
        );
    }
    let Some(input_shape) = input_shape else {
        if matches!(
            protocol,
            ProtocolKind::AwsJson1_0 | ProtocolKind::AwsJson1_1 | ProtocolKind::RestJson1
        ) {
            let function = format!("ser_{module}_input");
            let content_type = match protocol {
                ProtocolKind::AwsJson1_0 => "application/x-amz-json-1.0",
                ProtocolKind::AwsJson1_1 => "application/x-amz-json-1.1",
                ProtocolKind::RestJson1 => "application/json",
                _ => unreachable!(),
            };
            return (
                format!("crate::protocol_serde::shape_{module}::{function}(&input)?"),
                Some(content_type.to_owned()),
            );
        }
        return (
            "::aws_smithy_types::body::SdkBody::from(\"\")".to_owned(),
            None,
        );
    };
    if let Some((name, member)) = members(input_shape).into_iter().find(|(_, member)| {
        has_trait(member, "smithy.api#httpPayload")
            && member_target(member).is_some_and(|target| is_event_stream_target(selected, target))
    }) {
        let field = names::rust_identifier(&name);
        let union_name = rust_type_name(terminal(member_target(member).unwrap_or_default()));
        return (
            format!(
                "::aws_smithy_types::body::SdkBody::from({{\n            let error_marshaller = crate::event_stream_serde::{union_name}InputErrorMarshaller::new();\n            let marshaller = crate::event_stream_serde::{union_name}InputMarshaller::new();\n\n            let (signer, signer_sender) = ::aws_smithy_eventstream::frame::DeferredSigner::new();\n            _cfg.interceptor_state().store_put(signer_sender);\n            ::aws_smithy_types::body::SdkBody::from_body_1_x(::http_body_util::StreamBody::new(input.{field}.into_body_stream(\n                marshaller,\n                error_marshaller,\n                signer,\n            )))\n        }})"
            ),
            Some("application/vnd.amazon.eventstream".to_owned()),
        );
    }
    if matches!(
        protocol,
        ProtocolKind::AwsJson1_0 | ProtocolKind::AwsJson1_1 | ProtocolKind::RestJson1
    ) {
        if let Some((name, member)) = members(input_shape)
            .into_iter()
            .find(|(_, member)| has_trait(member, "smithy.api#httpPayload"))
        {
            let field = names::rust_identifier(&name);
            let target = member_target(member).unwrap_or_default();
            let target_shape = selected.model.shapes.get(target);
            let target_kind = protocol_shape_kind(selected, target);
            if !(target_kind == "union"
                && selected
                    .model
                    .shapes
                    .get(target)
                    .is_some_and(shape_is_streaming))
            {
                let helper = format!(
                    "crate::protocol_serde::shape_{module}_input::ser_{field}_http_payload"
                );
                let expression = if matches!(target_kind, "string" | "enum" | "blob") {
                    format!("{helper}(input.{field})?")
                } else {
                    format!("{helper}(&input.{field})?")
                };
                let content_type = target_shape
                    .and_then(shape_media_type)
                    .or(match target_kind {
                        "string" | "enum" => Some("text/plain"),
                        "blob" => Some("application/octet-stream"),
                        _ => Some("application/json"),
                    })
                    .map(str::to_owned);
                return (
                    format!("::aws_smithy_types::body::SdkBody::from({expression})"),
                    content_type,
                );
            }
        }
    }
    if matches!(
        protocol,
        ProtocolKind::AwsJson1_0 | ProtocolKind::AwsJson1_1 | ProtocolKind::RestJson1
    ) {
        let content_type = match protocol {
            ProtocolKind::AwsJson1_0 => "application/x-amz-json-1.0",
            ProtocolKind::AwsJson1_1 => "application/x-amz-json-1.1",
            ProtocolKind::RestJson1 => "application/json",
            _ => unreachable!(),
        };
        return (
            format!(
                "::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_{module}::ser_{module}_input(&input)?)"
            ),
            Some(content_type.to_owned()),
        );
    }
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

#[allow(clippy::too_many_arguments)]
fn render_standalone_runtime_plugin(
    output: &mut String,
    selected: &SelectedModel,
    operation_name: &str,
    operation: &Value,
    module: &str,
    operation_type: &str,
    shape_type: &str,
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
        output_shape.is_some_and(|shape| operation_output_is_sensitive(selected, shape));
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
    let disable_sigv4_overrides = service_uses_s3_sigv4_overrides(selected);
    let double_uri_encode = unsigned_payload || !disable_sigv4_overrides;
    let content_sha256_header = disable_sigv4_overrides || unsigned_payload;
    let normalize_uri_path = !disable_sigv4_overrides;
    let aws_error_classifier = if disable_sigv4_overrides {
        format!(
            "::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<{error_path}>::builder().transient_errors({{\n                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();\n                                            transient_errors.push(\"InternalError\");\n                                            ::std::borrow::Cow::Owned(transient_errors)\n                                            }}).build()"
        )
    } else {
        format!(
            "::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<{error_path}>::new()"
        )
    };
    let telemetry_interceptor = if operation_has_telemetry_members(selected, operation) {
        format!(
            "                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent({operation_type}TelemetryInputCaptureInterceptor))\n"
        )
    } else {
        String::new()
    };

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
        let input_type = format!("crate::operation::{module}::{shape_type}Input");
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
            let input_type = format!("crate::operation::{module}::{shape_type}Input");
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
        "        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new({operation_name:?}, {service_id:?}));\n{config_extras}        let mut signing_options = ::aws_runtime::auth::SigningOptions::default();\n        signing_options.double_uri_encode = {double_uri_encode};\n        signing_options.content_sha256_header = {content_sha256_header};\n        signing_options.normalize_uri_path = {normalize_uri_path};\n        signing_options.payload_override = {payload_override};\n\n        cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {{\n            signing_options,\n            ..::std::default::Default::default()\n        }});\n\n        ::std::option::Option::Some(cfg.freeze())\n    }}\n\n    fn runtime_components(\n        &self,\n        _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,\n    ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {{\n        #[allow(unused_mut)]\n                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new({operation_name:?})\n{telemetry_interceptor}.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))\n.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent({operation_type}EndpointParamsInterceptor))\n{additional_interceptors}                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<{error_path}>::new())\n.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<{error_path}>::new())\n.with_retry_classifier({aws_error_classifier});\n\n        ::std::borrow::Cow::Owned(rcb)\n    }}\n}}",
        double_uri_encode = double_uri_encode,
        content_sha256_header = content_sha256_header,
        normalize_uri_path = normalize_uri_path,
        payload_override = if unsigned_payload {
            "Some(::aws_sigv4::http_request::SignableBody::UnsignedPayload)"
        } else {
            "None"
        },
        telemetry_interceptor = telemetry_interceptor,
        aws_error_classifier = aws_error_classifier,
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
    shape_type: &str,
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
    *output = output
        .replace(
            &format!("downcast_ref::<{operation_type}Input>()"),
            &format!("downcast_ref::<{shape_type}Input>()"),
        )
        .replace(
            &format!("failed to downcast to {operation_type}Input"),
            &format!("failed to downcast to {shape_type}Input"),
        );
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
    shape_type: &str,
    error_path: &str,
) {
    let input_shape = operation
        .get("input")
        .and_then(target_value)
        .and_then(|id| selected.model.shapes.get(id));
    let input_path = format!("crate::operation::{module}::{shape_type}Input");
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
    let protocol = selected.model.protocol().expect("selected protocol exists");
    let has_headers = protocol != ProtocolKind::AwsJson1_0
        && protocol != ProtocolKind::AwsJson1_1
        && input_shape.is_some_and(|shape| {
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
        let target_header = standalone_json_target_header(selected, operation_name);
        let target_header = target_header
            .map(|target| {
                format!(
                    "\n            builder = _header_serialization_settings.set_default_header(\n                builder,\n                ::http_1x::header::HeaderName::from_static(\"x-amz-target\"),\n                {target:?},\n            );"
                )
            })
            .unwrap_or_default();
        let builder_replacement = format!(
            "            let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;\n            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, {content_type:?});{target_header}\n            builder"
        );
        *output = output.replace(&builder_marker, &builder_replacement);
    }
    let body_marker = "        let body = ::aws_smithy_types::body::SdkBody::from(\"\");\n\n";
    let mut body_replacement = format!("        let body = {body_expression};\n");
    body_replacement.push_str("        if let Some(content_length) = body.content_length() {\n            let content_length = content_length.to_string();\n            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);\n        }\n");
    let add_content_length = content_type.is_some()
        && (protocol != ProtocolKind::AwsJson1_0 && protocol != ProtocolKind::AwsJson1_1
            || input_shape.is_some_and(|shape| !members(shape).is_empty()));
    if add_content_length {
        *output = output.replacen(body_marker, &body_replacement, 1);
    }
    *output = output.replace(
        "\n            #[allow(clippy::unnecessary_wraps)]",
        "            #[allow(clippy::unnecessary_wraps)]",
    );
    let _ = error_path;
}

fn standalone_json_target_header(selected: &SelectedModel, operation_name: &str) -> Option<String> {
    let protocol = selected.model.protocol().ok()?;
    if !matches!(
        protocol,
        ProtocolKind::AwsJson1_0 | ProtocolKind::AwsJson1_1
    ) {
        return None;
    }
    Some(format!(
        "{}.{}",
        terminal(selected.model.service_shape_id.as_str()),
        operation_name
    ))
}

fn render_standalone_endpoint_interceptor(
    output: &mut String,
    selected: &SelectedModel,
    operation_name: &str,
    operation_type: &str,
    shape_type: &str,
) {
    let operation = operation_shape(selected, operation_name).expect("selected operation exists");
    let service = selected
        .model
        .shapes
        .get(selected.model.service_shape_id.as_str());
    let service_traits = service
        .and_then(|shape| shape.get("traits"))
        .and_then(Value::as_object);
    let endpoint_params = service_traits
        .and_then(|traits| {
            traits
                .get("smithy.rules#endpointRuleSet")
                .or_else(|| traits.get("smithy.rules#endpointBdd"))
        })
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
    *output = output
        .replace(
            &format!("downcast_ref::<{operation_type}Input>()"),
            &format!("downcast_ref::<{shape_type}Input>()"),
        )
        .replace(
            &format!("failed to downcast to {operation_type}Input"),
            &format!("failed to downcast to {shape_type}Input"),
        );
    if let Some(endpoint_prefix) = render_standalone_endpoint_prefix(operation, input_shape) {
        let params_marker = "        let params = crate::config::endpoint::Params::builder()";
        *output = output.replace(params_marker, &format!("{endpoint_prefix}{params_marker}"));
    }
    if let Some(endpoint_params) = endpoint_params {
        for (name, parameter) in endpoint_params {
            let expression = match name.as_str() {
                "Region" => {
                    Some("cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned())")
                }
                "UseFIPS" => {
                    Some("cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0)")
                }
                "UseDualStack" => {
                    Some("cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0)")
                }
                "Endpoint" => Some(
                    "cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone())",
                ),
                _ => None,
            };
            if let Some(expression) = expression {
                writeln!(
                    output,
                    "            .set_{}({expression})",
                    names::snake_case(name)
                )
                .unwrap();
            }
            if parameter.get("builtIn").and_then(Value::as_str)
                == Some("AWS::Auth::AccountIdEndpointMode")
            {
                output.push_str(
                    "            .set_account_id_endpoint_mode(::std::option::Option::Some(\n                cfg.load::<::aws_types::endpoint_config::AccountIdEndpointMode>()\n                    .cloned()\n                    .unwrap_or_default()\n                    .to_string(),\n            ))\n",
                );
            }
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
    let mut operation_context_getters = String::new();
    if let Some(operation_context_params) = operation
        .get("traits")
        .and_then(|traits| traits.get("smithy.rules#operationContextParams"))
        .and_then(Value::as_object)
    {
        for (name, parameter) in operation_context_params {
            if let Some(path) = parameter.get("path").and_then(Value::as_str)
                && let Some((setter, source)) = render_operation_context_param(
                    selected,
                    operation_name,
                    input_shape,
                    name,
                    path,
                )
            {
                writeln!(
                    output,
                    "            .set_{}({setter})",
                    names::snake_case(name)
                )
                .unwrap();
                operation_context_getters.push_str(&source);
            }
        }
    }
    if let Some(input_shape) = input_shape {
        let mut context_members = members(input_shape)
            .into_iter()
            .filter_map(|(name, member)| {
                member
                    .get("traits")
                    .and_then(|traits| traits.get("smithy.rules#contextParam"))
                    .and_then(|value| value.get("name"))
                    .and_then(Value::as_str)
                    .map(|context_param| (name, member, context_param.to_owned()))
            })
            .collect::<Vec<_>>();
        context_members.sort_by(|left, right| left.0.cmp(&right.0));
        for (name, member, context_param) in context_members {
            let field = names::rust_identifier(&name);
            let setter = names::snake_case(&context_param);
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
    output.push_str(&operation_context_getters);
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

fn render_operation_context_param(
    selected: &SelectedModel,
    operation_name: &str,
    input_shape: Option<&Value>,
    parameter_name: &str,
    path: &str,
) -> Option<(String, String)> {
    let input_shape = input_shape?;
    let getter = format!("get_{}", names::snake_case(parameter_name));
    let input_module = names::rust_module_name(operation_name);
    let input_type = rust_type_name(operation_name);
    let input_path = format!("crate::operation::{input_module}::{input_type}Input");

    if let Some(inner) = path
        .strip_prefix("keys(")
        .and_then(|value| value.strip_suffix(')'))
    {
        let field = inner.trim();
        let (member_name, _member) = members(input_shape)
            .into_iter()
            .find(|(name, _)| name == field)?;
        let field = names::rust_identifier(&member_name);
        let source = format!(
            "// Generated from JMESPath Expression: {path}\nfn {getter}(input: &{input_path}) -> Option<::std::vec::Vec<::std::string::String>> {{\n    let _fld_2 = input.{field}.as_ref()?;\n    let _ret_1 = _fld_2.keys().map(Clone::clone).collect::<Vec<String>>();\n    Some(_ret_1)\n}}\n\n"
        );
        return Some((format!("{getter}(_input)"), source));
    }

    if let Some((outer, rest)) = path.split_once("[*].") {
        let (outer_name, outer_member) = members(input_shape)
            .into_iter()
            .find(|(name, _)| name == outer)?;
        let outer_field = names::rust_identifier(&outer_name);
        let outer_target = member_target(outer_member)?;
        let list_shape = selected.model.shapes.get(outer_target)?;
        let element_target = list_shape.get("member").and_then(member_target)?;
        let element_shape = selected.model.shapes.get(element_target)?;
        let element_type = rust_type_name(terminal(element_target));
        let (_variant_path, _) = rest.split_once(".")?;

        let output_path = if rest.starts_with('[') {
            rest.trim_start_matches('[')
                .split_once(',')
                .map(|(value, _)| value.trim())?
        } else {
            rest
        };
        let output_target = nested_member_target(selected, element_shape, output_path)?;
        let output_type = protocol_shape_type(selected, output_target);
        let output_type_ref = format!("&{output_type}");
        let paths = if rest.starts_with('[') && rest.ends_with("][]") {
            let variants = rest
                .trim_end_matches("[]")
                .trim_start_matches('[')
                .split(", ")
                .collect::<Vec<_>>();
            if variants.is_empty() {
                return None;
            }
            let mut lines = String::new();
            for (index, variant) in variants.iter().enumerate() {
                let (variant_name, member_name) = variant.split_once('.')?;
                let variant_field = names::rust_identifier(variant_name);
                let member_field = names::rust_identifier(member_name);
                let first = 2 + index * 2;
                let second = first + 1;
                writeln!(
                    lines,
                    "                let _fld_{first} = _v.{variant_field}.as_ref();\n                let _fld_{second} = _fld_{first}.map(|v| &v.{member_field});"
                )
                .ok()?;
            }
            let option_type = format!("::std::option::Option<{output_type_ref}>");
            let mut values = Vec::new();
            for index in 0..variants.len() {
                values.push(format!("_fld_{}", 3 + index * 2));
            }
            let last = 2 + variants.len() * 2;
            let mapped = format!(
                "let _msl_{last} = vec![{}];\n                ::std::option::Option::Some(_msl_{last})",
                values.join(", ")
            );
            (lines, mapped, option_type)
        } else {
            let (variant_name, member_name) = rest.split_once('.')?;
            let variant_field = names::rust_identifier(variant_name);
            let member_field = names::rust_identifier(member_name);
            let variant = format!(
                "let _fld_2 = _v.{variant_field}.as_ref();\n                let _fld_3 = _fld_2.map(|v| &v.{member_field});\n                _fld_3"
            );
            (String::new(), variant, String::new())
        };

        let (inner_lines, mapped, option_type) = paths;
        let (return_type, flatten) = if rest.starts_with('[') {
            (
                format!("Option<::std::vec::Vec<{output_type_ref}>>"),
                ".flatten()\n        .flatten()",
            )
        } else {
            (format!("Option<::std::vec::Vec<{output_type_ref}>>"), "")
        };
        let (mapped, helper_return, collect) = if rest.starts_with('[') {
            (
                format!("{inner_lines}                {mapped}"),
                format!("::std::option::Option<::std::vec::Vec<{option_type}>>"),
                format!("{flatten}\n        .collect::<::std::vec::Vec<_>>()"),
            )
        } else {
            (
                mapped,
                format!("::std::option::Option<{output_type_ref}>"),
                "\n        .collect::<::std::vec::Vec<_>>()".to_owned(),
            )
        };
        let source = if rest.starts_with('[') {
            format!(
                "// Generated from JMESPath Expression: {path}\nfn {getter}(input: &{input_path}) -> Option<::std::vec::Vec<{output_type_ref}>> {{\n    let _fld_1 = input.{outer_field}.as_ref()?;\n    let _prj_11 = _fld_1\n        .iter()\n        .flat_map(|v| {{\n            #[allow(clippy::let_and_return)]\n            fn map(_v: &crate::types::{element_type}) -> {helper_return} {{\n{mapped}\n            }}\n            map(v)\n        }}){collect};\n    Some(_prj_11)\n}}\n\n"
            )
        } else {
            format!(
                "// Generated from JMESPath Expression: {path}\nfn {getter}(input: &{input_path}) -> Option<::std::vec::Vec<{output_type_ref}>> {{\n    let _fld_1 = input.{outer_field}.as_ref()?;\n    let _prj_4 = _fld_1\n        .iter()\n        .flat_map(|v| {{\n            #[allow(clippy::let_and_return)]\n            fn map(_v: &crate::types::{element_type}) -> ::std::option::Option<{output_type_ref}> {{\n                {mapped}\n            }}\n            map(v)\n        }})\n        .collect::<::std::vec::Vec<_>>();\n    Some(_prj_4)\n}}\n\n"
            )
        };
        let setter = if rest.starts_with('[') || !output_type_ref.is_empty() {
            format!("{getter}(_input).map(|v| v.into_iter().cloned().collect::<Vec<_>>())")
        } else {
            format!("{getter}(_input)")
        };
        let _ = (return_type, option_type, inner_lines);
        return Some((setter, source));
    }

    let mut current_shape = input_shape;
    let mut statements = String::new();
    let mut current = "input".to_owned();
    let mut fields = path.split('.').peekable();
    let mut index = 0;
    while let Some(name) = fields.next() {
        let (_, member) = members(current_shape)
            .into_iter()
            .find(|(member_name, _)| member_name == name)?;
        let field = names::rust_identifier(name);
        let target = member_target(member)?;
        if fields.peek().is_some() {
            index += 1;
            writeln!(
                statements,
                "    let _fld_{index} = {current}.{field}.as_ref()?;"
            )
            .ok()?;
            current = format!("_fld_{index}");
            current_shape = selected.model.shapes.get(target)?;
        } else {
            index += 1;
            writeln!(statements, "    let _fld_{index} = &{current}.{field};").ok()?;
            let source = format!(
                "// Generated from JMESPath Expression: {path}\nfn {getter}(input: &{input_path}) -> Option<&{}> {{\n{statements}    Some(_fld_{index})\n}}\n\n",
                protocol_shape_type(selected, target)
            );
            return Some((format!("{getter}(_input).cloned()"), source));
        }
    }
    None
}

fn nested_member_target<'a>(
    selected: &'a SelectedModel,
    mut shape: &'a Value,
    path: &str,
) -> Option<&'a str> {
    let mut target = None;
    for field in path.split('.') {
        let (_, member) = members(shape).into_iter().find(|(name, _)| name == field)?;
        target = member_target(member);
        if let Some(next) = target.and_then(|target| selected.model.shapes.get(target)) {
            shape = next;
        }
    }
    target
}

/// Render Smithy HTTP protocol tests supplied by a declarative model overlay.
/// The renderer consumes the generic request/response test shape and does not
/// identify a service or operation by name.
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
        } else {
            output.push_str("    #[allow(missing_docs)] // documentation missing in model\n");
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
    let retryable_arms = errors
        .iter()
        .filter(|error| {
            selected
                .model
                .shapes
                .get::<str>(*error)
                .is_some_and(|shape| has_trait(shape, "smithy.api#retryable"))
        })
        .map(|error| {
            format!(
                "Self::{}(inner) => ::std::option::Option::Some(inner.retryable_error_kind()),",
                rust_type_name(terminal(error))
            )
        })
        .collect::<Vec<_>>();
    let retryable_body = if retryable_arms.is_empty() {
        "::std::option::Option::None".to_owned()
    } else {
        let arms = retryable_arms
            .into_iter()
            .chain(std::iter::once(
                "_ => ::std::option::Option::None,".to_owned(),
            ))
            .collect::<Vec<_>>()
            .join("\n            ");
        format!("match self {{\n            {arms}\n        }}")
    };
    writeln!(output, "impl ::aws_smithy_types::retry::ProvideErrorKind for {operation_type}Error {{\n    fn code(&self) -> ::std::option::Option<&str> {{\n        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)\n    }}\n    fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {{\n        {retryable_body}\n    }}\n}}\nimpl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for {operation_type}Error {{\n    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {{\n        match self {{\n            {arms}\n        }}\n    }}\n}}", arms = if errors.is_empty() { "Self::Unhandled(_inner) => &_inner.meta,".to_owned() } else { errors.iter().map(|error| format!("Self::{}(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),", rust_type_name(terminal(error)))).chain(std::iter::once("Self::Unhandled(_inner) => &_inner.meta,".to_owned())).collect::<Vec<_>>().join("\n            ") }).unwrap();
    let extended_request_id = if request_id_plan(selected).extended {
        format!(
            "impl crate::s3_request_id::RequestIdExt for {error_path} {{\n    fn extended_request_id(&self) -> Option<&str> {{\n        self.meta().extended_request_id()\n    }}\n}}\n"
        )
    } else {
        String::new()
    };
    writeln!(output, "impl ::aws_smithy_runtime_api::client::result::CreateUnhandledError for {operation_type}Error {{\n    fn create_unhandled_error(\n        source: ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,\n        meta: ::std::option::Option<::aws_smithy_types::error::ErrorMetadata>,\n    ) -> Self {{\n        Self::Unhandled(crate::error::sealed_unhandled::Unhandled {{\n            source,\n            meta: meta.unwrap_or_default(),\n        }})\n    }}\n}}\n{extended_request_id}impl ::aws_types::request_id::RequestId for {error_path} {{\n    fn request_id(&self) -> Option<&str> {{\n        self.meta().request_id()\n    }}\n}}",).unwrap();
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
        .service_shape_id
        .as_str()
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
        .get(selected.model.service_shape_id.as_str())
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

fn waiter_matcher_type(selected: &SelectedModel, target: &str, operation_module: &str) -> String {
    if selected
        .model
        .shapes
        .get(target)
        .is_some_and(|shape| has_trait(shape, "smithy.api#enum"))
    {
        format!("crate::types::{}", rust_type_name(terminal(target)))
    } else {
        type_expr(
            selected,
            target,
            Context::Operation {
                module: operation_module.to_owned(),
                input: false,
            },
        )
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
    output_type: &str,
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
    let output_path = format!("{operation_prefix}::{operation_module}::{output_type}Output");
    let target_type = waiter_matcher_type(selected, &target, operation_module);
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
        let element_type = waiter_matcher_type(selected, element_target, operation_module);
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
        let element_type = waiter_matcher_type(selected, element_target, operation_module);
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

fn render_waiters_file(selected: &SelectedModel) -> String {
    let mut output = String::new();
    header(&mut output);
    for (_, waiter_name, _) in waiter_specs_by_name(selected) {
        writeln!(
            output,
            "/// Supporting types for the `{waiter_name}` waiter."
        )
        .unwrap();
        {
            writeln!(output, "pub mod {waiter_name};\n").unwrap();
        }
    }
    output.push_str("#[allow(clippy::needless_lifetimes)]\n#[allow(clippy::let_and_return)]\n");
    {
        output.push_str("pub(crate) mod matchers;\n");
    }
    output
}

fn render_waiter_matchers_file(selected: &SelectedModel) -> String {
    let mut output = String::new();
    header(&mut output);
    let operation_prefix = { "crate::operation" };
    let mut seen = BTreeSet::new();
    for (operation_name, _, waiter) in waiter_specs_by_name(selected) {
        let operation_module = names::snake_case(&operation_name);
        let operation_type = operation_error_type_name(&operation_name);
        let output_type = rust_type_name(&operation_name);
        for (_, matcher) in waiter_acceptors(&waiter) {
            let matcher_json = waiter_matcher_json(&matcher);
            if !seen.insert(format!("{operation_name}\0{matcher_json}")) {
                continue;
            }
            let matcher_name = waiter_matcher_name(&operation_name, &matcher);
            writeln!(output, "/// Matcher union: {matcher_json}").unwrap();
            writeln!(
                output,
                "pub(crate) fn {matcher_name}(\n    _result: ::std::result::Result<&{operation_prefix}::{operation_module}::{output_type}Output, &{operation_prefix}::{operation_module}::{operation_type}Error>,\n) -> bool {{"
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
                {
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
                &output_type,
            ) {
            } else {
                output.push_str("    false\n");
            }
            output.push_str("}\n\n");
        }
    }
    output
}

fn render_waiter_file(selected: &SelectedModel, waiter_name: &str, waiter: &Value) -> String {
    let (operation_name, _, _) = waiter_specs(selected)
        .into_iter()
        .find(|(_, name, _)| name == waiter_name)
        .expect("waiter belongs to selected model");
    let operation_module = names::snake_case(&operation_name);
    let operation_type = operation_error_type_name(&operation_name);
    let shape_type = rust_type_name(&operation_name);
    let waiter_type = rust_type_name(waiter_name);
    let operation_prefix = { "crate::operation" };
    let client_path = { "crate::client" };
    let matcher_prefix = { "crate::waiters::matchers" };
    let input_builder_path =
        format!("{operation_prefix}::{operation_module}::builders::{shape_type}InputBuilder");
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
    let waiter_module_path = { format!("crate::waiters::{waiter_name}") };
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
    output = output.replace(
        &format!("{operation_prefix}::{operation_module}::{operation_type}Output"),
        &format!("{operation_prefix}::{operation_module}::{shape_type}Output"),
    );
    // FluentBuilderGenerator places input helpers immediately after the
    // overridden wait method; keep the doc comment adjacent to the method as
    // rustfmt does in Smithy-RS output.
    output.replace("\n\n    ///", "\n    ///")
}

fn render_waiter_input_methods(
    selected: &SelectedModel,
    operation_name: &str,
    operation_module: &str,
    operation_prefix: &str,
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

fn render_lens_file(selected: &SelectedModel) -> String {
    let mut output = String::new();
    client_operation_header(&mut output);
    let operation_prefix = { "crate::operation" };
    let types_prefix = { "crate::types" };

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
        let token_type = lens_type_expr(selected, &token_target);
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

fn render_paginator_file(selected: &SelectedModel, operation_name: &str) -> String {
    let info = operation_pagination_info(selected, operation_name)
        .expect("paginator file only exists for paginated operations");
    let operation_module = names::snake_case(operation_name);
    let operation_type = operation_error_type_name(operation_name);
    let shape_type = rust_type_name(operation_name);
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
        "/// Paginator for [`{operation_name}`]({operation_path}::{operation_type})\npub struct {paginator_name} {{\n    handle: std::sync::Arc<crate::client::Handle>,\n    builder: {operation_path}::builders::{shape_type}InputBuilder,\n    stop_on_duplicate_token: bool,\n}}\n\nimpl {paginator_name} {{\n    /// Create a new paginator-wrapper\n    pub(crate) fn new(\n        handle: std::sync::Arc<crate::client::Handle>,\n        builder: {operation_path}::builders::{shape_type}InputBuilder,\n    ) -> Self {{\n        Self {{\n            handle,\n            builder,\n            stop_on_duplicate_token: true,\n        }}\n    }}\n"
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

    let output_type = format!("{operation_path}::{shape_type}Output");
    let error_type = format!("{operation_path}::{operation_type}Error");
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
        let item_type = paginator_item_type(selected, &items_target);
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

    output
}

fn paginator_item_type(selected: &SelectedModel, target: &str) -> String {
    let Some(shape) = selected.model.shapes.get(target) else {
        return lens_type_expr(selected, target);
    };
    let type_expr = |target: &str| lens_type_expr(selected, target);
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

fn lens_type_expr(selected: &SelectedModel, target: &str) -> String {
    if target.starts_with("smithy.api#") {
        return primitive_type_for_namespace(terminal(target));
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
        ),
        Some("list") => shape
            .get("member")
            .and_then(member_target)
            .map(|member| format!("::std::vec::Vec<{}>", lens_type_expr(selected, member)))
            .unwrap_or_else(|| "::std::vec::Vec<::std::string::String>".to_owned()),
        Some("map") => {
            let key = shape
                .get("key")
                .and_then(member_target)
                .map(|member| lens_type_expr(selected, member))
                .unwrap_or_else(|| "::std::string::String".to_owned());
            let value = shape
                .get("value")
                .and_then(member_target)
                .map(|member| lens_type_expr(selected, member))
                .unwrap_or_else(|| "::std::string::String".to_owned());
            format!("::std::collections::HashMap<{key}, {value}>")
        }
        _ => format!(
            "{}::{}",
            { "crate::types" },
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
            .map(|member| format!("::std::vec::Vec<{}>", lens_type_expr(selected, member)))
            .unwrap_or_else(|| "::std::vec::Vec<::std::string::String>".to_owned()),
        Some("map") => {
            let key = shape
                .get("key")
                .and_then(member_target)
                .map(|member| lens_type_expr(selected, member))
                .unwrap_or_else(|| "::std::string::String".to_owned());
            let value = shape
                .get("value")
                .and_then(member_target)
                .map(|member| lens_type_expr(selected, member))
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
            },
            0,
        );
    }
    output
}

fn render_operation_builder_file(selected: &SelectedModel, operation_name: &str) -> String {
    render_standalone_fluent_operation_builder_file(selected, operation_name)
}

fn render_standalone_fluent_operation_builder_file(
    selected: &SelectedModel,
    operation_name: &str,
) -> String {
    let operation = operation_shape(selected, operation_name).expect("selected operation exists");
    let module = names::snake_case(operation_name);
    let operation_type = operation_error_type_name(operation_name);
    let shape_type = rust_type_name(operation_name);
    let input_builder_path =
        format!("crate::operation::{module}::builders::{shape_type}InputBuilder");
    let output_path = format!("crate::operation::{module}::{shape_type}Output");
    let error_path = format!("crate::operation::{module}::{operation_type}Error");
    let long_builder_field = 4 + "inner: ".len() + input_builder_path.len() + 1 > 150;
    let (field_indent, config_indent, struct_close_indent) = if long_builder_field {
        ("                ", "", "            ")
    } else {
        ("    ", "    ", "")
    };
    let operation_runtime_plugins_path =
        format!("crate::operation::{module}::{operation_type}::operation_runtime_plugins");
    let long_runtime_plugin_call =
        format!("        let runtime_plugins = {operation_runtime_plugins_path}(").len() > 150;
    let runtime_plugin_argument_indent = if long_runtime_plugin_call {
        "                            "
    } else {
        "            "
    };
    let runtime_plugin_close_indent = if long_runtime_plugin_call {
        "                        "
    } else {
        "        "
    };
    let input_shape = operation
        .get("input")
        .and_then(target_value)
        .and_then(|id| selected.model.shapes.get(id));
    let mut output = String::new();
    client_operation_header(&mut output);
    writeln!(
        output,
        "pub use crate::operation::{module}::_{module}_input::{shape_type}InputBuilder;\n\npub use crate::operation::{module}::_{module}_output::{shape_type}OutputBuilder;\n"
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
    if input_shape.is_some_and(|shape| structure_has_streaming_member(selected, shape)) {
        output.push_str("#[derive(::std::fmt::Debug)]\n");
    } else {
        output.push_str("#[derive(::std::clone::Clone, ::std::fmt::Debug)]\n");
    }
    writeln!(
        output,
        "pub struct {operation_type}FluentBuilder {{\n{field_indent}handle: ::std::sync::Arc<crate::client::Handle>,\n{field_indent}inner: {input_builder_path},\n{config_indent}config_override: ::std::option::Option<crate::config::Builder>,\n{struct_close_indent}}}"
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
        "        {output_path},\n        ::aws_smithy_runtime_api::client::result::SdkError<\n            {error_path},\n            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,\n        >,\n    > {{\n        let input = self\n            .inner\n            .build()\n            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;\n        let runtime_plugins = {operation_runtime_plugins_path}(\n{runtime_plugin_argument_indent}self.handle.runtime_plugins.clone(),\n{runtime_plugin_argument_indent}&self.handle.conf,\n{runtime_plugin_argument_indent}self.config_override,\n{runtime_plugin_argument_indent});\n        crate::operation::{module}::{operation_type}::orchestrate(&runtime_plugins, input).await\n    }}\n\n    /// Consumes this builder, creating a customizable operation that can be modified before being sent.\n    pub fn customize(\n        self,\n    ) -> crate::client::customize::CustomizableOperation<\n        {output_path},\n        {error_path},\n        Self,\n    > {{\n        crate::client::customize::CustomizableOperation::new(self)\n    }}\n    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {{\n        self.set_config_override(::std::option::Option::Some(config_override.into()));\n        self\n    }}\n\n    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {{\n        self.config_override = config_override;\n        self\n    }}"
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
    if long_runtime_plugin_call {
        let call_suffix = format!(
            "{runtime_plugin_argument_indent});\n        crate::operation::{module}::{operation_type}::orchestrate"
        );
        let corrected_suffix = format!(
            "{runtime_plugin_close_indent});\n        crate::operation::{module}::{operation_type}::orchestrate"
        );
        output = output.replacen(&call_suffix, &corrected_suffix, 1);
    }
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
            query.starts_with("x-id=")
                && output_shape.is_some_and(|shape| structure_has_streaming_member(selected, shape))
        }
        "HEAD" => query.is_empty(),
        "PUT" => {
            query.starts_with("x-id=")
                && input_shape.is_some_and(|shape| structure_has_streaming_member(selected, shape))
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

fn render_protocol_operation_file(
    selected: &SelectedModel,
    operation_name: &str,
    protocol: ProtocolKind,
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
        render_protocol_http_response(&mut output, selected, operation_name, output_shape);
        render_protocol_http_error(&mut output, selected, operation_name, operation);
    } else {
        render_protocol_http_error(&mut output, selected, operation_name, operation);
        render_protocol_http_response(&mut output, selected, operation_name, output_shape);
    }
    render_protocol_request_headers(&mut output, selected, operation_name, input_shape);
    if !is_query_protocol(protocol)
        && let Some(serializer) =
            render_protocol_operation_input_serializer(selected, operation_name)
    {
        output.push_str(&serializer);
    }
    if let Some(parser) = render_protocol_operation_output_parser(
        selected,
        operation_name,
        &output,
        protocol == ProtocolKind::AwsQuery,
    ) {
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

fn render_query_protocol_input_file(selected: &SelectedModel, operation_name: &str) -> String {
    let operation = operation_shape(selected, operation_name).expect("selected operation exists");
    let input_shape = operation
        .get("input")
        .and_then(target_value)
        .and_then(|id| selected.model.shapes.get(id))
        .expect("selected operation input exists");
    let operation_module = names::rust_module_name(operation_name);
    let operation_type = rust_type_name(operation_name);
    let function = format!("ser_{operation_module}_input_input_input");
    let version = selected
        .model
        .shapes
        .get(&selected.model.service_shape_id)
        .and_then(|service| service.get("version"))
        .and_then(Value::as_str)
        .unwrap_or_default();
    let mut output = String::new();
    client_operation_header(&mut output);
    writeln!(
        output,
        "pub fn {function}(\n    input: &crate::operation::{operation_module}::{operation_type}Input,\n) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {{"
    )
    .unwrap();
    let mut state = ProtocolRenderState::default();
    if members(input_shape).is_empty() {
        output.push_str("    let _ = input;\n");
    }
    output.push_str("    let mut out = String::new();\n    #[allow(unused_mut)]\n");
    writeln!(
        output,
        "    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, {operation_name:?}, {version:?});"
    )
    .unwrap();
    for (member_name, member) in members(input_shape) {
        render_query_member(
            &mut output,
            selected,
            &member_name,
            member,
            "writer",
            &format!("input.{}", names::rust_identifier(&member_name)),
            false,
            true,
            &mut state,
        );
    }
    output.push_str(
        "    writer.finish();\n    Ok(::aws_smithy_types::body::SdkBody::from(out))\n}\n",
    );
    output
}

fn query_member_wire_name(member_name: &str, member: &Value) -> String {
    xml_name(member).unwrap_or_else(|| member_name.to_owned())
}

fn rust_method_name(member_name: &str) -> String {
    let identifier = names::rust_identifier(member_name);
    identifier
        .strip_prefix("r#")
        .unwrap_or(&identifier)
        .to_owned()
}

fn query_value_reference(expression: &str, expression_is_ref: bool) -> String {
    if expression_is_ref {
        expression.to_owned()
    } else {
        format!("&{expression}")
    }
}

fn query_shape_is_enum(selected: &SelectedModel, target: &str) -> bool {
    selected
        .model
        .shapes
        .get(target)
        .is_some_and(|shape| shape.get("type").and_then(Value::as_str) == Some("enum"))
}

fn query_primitive_value(
    selected: &SelectedModel,
    target: &str,
    expression: &str,
    expression_is_ref: bool,
) -> String {
    let kind = protocol_shape_kind(selected, target);
    let reference = query_value_reference(expression, expression_is_ref);
    match kind {
        "string" | "enum" => {
            if query_shape_is_enum(selected, target) {
                format!("{expression}.as_str()")
            } else {
                reference
            }
        }
        "bigInteger" | "bigDecimal" => format!("{reference}.as_ref()"),
        "blob" => format!("&::aws_smithy_types::base64::encode({reference})"),
        "boolean" => {
            if expression_is_ref {
                format!("*{expression}")
            } else {
                expression.to_owned()
            }
        }
        "float" | "double" => {
            let value = if expression_is_ref {
                format!("(*{expression}).into()")
            } else {
                format!("{expression}.into()")
            };
            format!(
                "#[allow(clippy::useless_conversion)]\n::aws_smithy_types::Number::Float({value})"
            )
        }
        "byte" | "short" | "integer" | "long" => {
            let value = if expression_is_ref {
                format!("(*{expression}).into()")
            } else {
                format!("{expression}.into()")
            };
            format!(
                "#[allow(clippy::useless_conversion)]\n::aws_smithy_types::Number::NegInt({value})"
            )
        }
        "timestamp" => format!(
            "{reference}, ::aws_smithy_types::date_time::Format::{}",
            protocol_timestamp_format(selected, target)
        ),
        _ => reference,
    }
}

#[allow(clippy::too_many_arguments)]
fn render_query_member(
    output: &mut String,
    selected: &SelectedModel,
    member_name: &str,
    member: &Value,
    writer: &str,
    expression: &str,
    expression_is_ref: bool,
    optional: bool,
    state: &mut ProtocolRenderState,
) {
    let Some(target) = member_target(member) else {
        return;
    };
    let scope = state.scope();
    let wire_name = query_member_wire_name(member_name, member);
    writeln!(
        output,
        "    #[allow(unused_mut)]\n    let mut {scope} = {writer}.prefix({wire_name:?});"
    )
    .unwrap();
    if optional {
        let variable = state.temp();
        let optional_expression = if expression.starts_with('&') {
            expression.to_owned()
        } else {
            format!("&{expression}")
        };
        writeln!(
            output,
            "    if let Some({variable}) = {optional_expression} {{"
        )
        .unwrap();
        render_query_value(
            output, selected, member, target, &scope, &variable, true, state,
        );
        output.push_str("    }\n");
    } else {
        output.push_str("    {\n");
        render_query_value(
            output,
            selected,
            member,
            target,
            &scope,
            expression,
            expression_is_ref,
            state,
        );
        output.push_str("    }\n");
    }
}

#[allow(clippy::too_many_arguments)]
fn render_query_value(
    output: &mut String,
    selected: &SelectedModel,
    member: &Value,
    target: &str,
    writer: &str,
    expression: &str,
    expression_is_ref: bool,
    state: &mut ProtocolRenderState,
) {
    match protocol_shape_kind(selected, target) {
        "string" | "enum" | "boolean" | "byte" | "short" | "integer" | "long" | "float"
        | "double" => {
            let value = query_primitive_value(selected, target, expression, expression_is_ref);
            let method = match protocol_shape_kind(selected, target) {
                "string" | "enum" => "string",
                "boolean" => "boolean",
                _ => "number",
            };
            writeln!(output, "        {writer}.{method}({value});").unwrap();
        }
        "timestamp" => {
            let value = query_primitive_value(selected, target, expression, expression_is_ref);
            writeln!(output, "        {writer}.date_time({value})?;").unwrap();
        }
        "blob" => {
            let value = query_primitive_value(selected, target, expression, expression_is_ref);
            writeln!(output, "        {writer}.string({value});").unwrap();
        }
        "structure" | "union" => {
            if selected.model.shapes.get(target).is_some_and(|shape| {
                shape.get("type").and_then(Value::as_str) == Some("structure")
                    && members(shape).is_empty()
            }) {
                return;
            }
            let value = query_value_reference(expression, expression_is_ref);
            writeln!(
                output,
                "        crate::protocol_serde::shape_{}::ser_{}({writer}, {value})?;",
                names::rust_module_name(terminal(target)),
                names::rust_module_name(terminal(target)),
            )
            .unwrap();
        }
        "list" => {
            let item = state.query_list_item();
            let list = state.list_accum();
            let list_shape = selected.model.shapes.get(target).expect("list shape");
            let list_member = list_shape.get("member").expect("list member");
            let flat = has_trait(member, "smithy.api#xmlFlattened");
            let member_override = xml_name(list_member)
                .map(|name| format!("Some({name:?})"))
                .unwrap_or_else(|| "None".to_owned());
            let value = query_value_reference(expression, expression_is_ref);
            let entry = state.entry();
            writeln!(
                output,
                "        let mut {list} = {writer}.start_list({flat}, {member_override});\n        for {item} in {value} {{\n            #[allow(unused_mut)]\n            let mut {entry} = {list}.entry();"
            )
            .unwrap();
            render_query_value(
                output,
                selected,
                list_member,
                member_target(list_member).unwrap_or_default(),
                &entry,
                &item,
                true,
                state,
            );
            output.push_str(&format!("        }}\n        {list}.finish();\n"));
        }
        "map" => {
            let map = state.map();
            let key = state.key();
            let value = state.value();
            let map_shape = selected.model.shapes.get(target).expect("map shape");
            let key_member = map_shape.get("key").expect("map key");
            let value_member = map_shape.get("value").expect("map value");
            let key_name = xml_name(key_member).unwrap_or_else(|| "key".to_owned());
            let value_name = xml_name(value_member).unwrap_or_else(|| "value".to_owned());
            let input = query_value_reference(expression, expression_is_ref);
            let key_expression =
                if query_shape_is_enum(selected, member_target(key_member).unwrap_or_default()) {
                    format!("{key}.as_str()")
                } else {
                    key.clone()
                };
            let entry = state.entry();
            writeln!(
                output,
                "        let mut {map} = {writer}.start_map({}, {:?}, {:?});\n        for ({key}, {value}) in {input} {{\n            #[allow(unused_mut)]\n            let mut {entry} = {map}.entry({key_expression});",
                has_trait(member, "smithy.api#xmlFlattened"),
                key_name,
                value_name,
            )
            .unwrap();
            output.push_str("            {\n");
            render_query_value(
                output,
                selected,
                value_member,
                member_target(value_member).unwrap_or_default(),
                &entry,
                &value,
                true,
                state,
            );
            output.push_str(&format!(
                "            }}\n        }}\n        {map}.finish();\n"
            ));
        }
        _ => {}
    }
}

fn render_query_protocol_structure_serializer(
    output: &mut String,
    selected: &SelectedModel,
    shape_id: &str,
    shape: &Value,
    state: &mut ProtocolRenderState,
) {
    let name = rust_type_name(terminal(shape_id));
    writeln!(
        output,
        "#[allow(unused_mut)]\npub fn ser_{}(\n    mut writer: ::aws_smithy_query::QueryValueWriter,\n    input: &crate::types::{name},\n) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {{",
        names::rust_module_name(terminal(shape_id)),
    )
    .unwrap();
    for (member_name, member) in members(shape) {
        render_query_member(
            output,
            selected,
            &member_name,
            member,
            "writer",
            &format!("&input.{}", names::rust_identifier(&member_name)),
            true,
            protocol_member_is_optional(selected, member),
            state,
        );
    }
    output.push_str("    Ok(())\n}\n");
}

fn render_query_protocol_union_serializer(
    output: &mut String,
    selected: &SelectedModel,
    shape_id: &str,
    shape: &Value,
    state: &mut ProtocolRenderState,
) {
    let name = rust_type_name(terminal(shape_id));
    writeln!(
        output,
        "#[allow(unused_mut)]\npub fn ser_{}(\n    mut writer: ::aws_smithy_query::QueryValueWriter,\n    input: &crate::types::{name},\n) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {{\n    match input {{",
        names::rust_module_name(terminal(shape_id)),
    )
    .unwrap();
    for (member_name, member) in members(shape) {
        let target = member_target(member).unwrap_or_default();
        let variant = rust_type_name(&member_name);
        let target_shape = selected.model.shapes.get(target);
        let is_unit = target == "smithy.api#Unit";
        let is_empty_structure = target_shape.is_some_and(|shape| {
            shape.get("type").and_then(Value::as_str) == Some("structure")
                && members(shape).is_empty()
        });
        if is_unit {
            writeln!(output, "        crate::types::{name}::{variant} => {{}}",).unwrap();
        } else {
            let binding = if is_empty_structure {
                "_inner"
            } else {
                "inner"
            };
            writeln!(
                output,
                "        crate::types::{name}::{variant}({binding}) => {{",
            )
            .unwrap();
            render_query_member(
                output,
                selected,
                &member_name,
                member,
                "writer",
                binding,
                true,
                false,
                state,
            );
            output.push_str("        }\n");
        }
    }
    writeln!(
        output,
        "        crate::types::{name}::Unknown => return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant({name:?})),"
    )
    .unwrap();
    output.push_str("    }\n    Ok(())\n}\n");
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
    if !selected.model.protocol().is_ok_and(is_query_protocol) {
        let capacity = java_hash_map_capacity(all_members.len());
        all_members.sort_by_key(|(name, _)| java_string_hash(name) & (capacity as u32 - 1));
    }
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
    let hash = java_string_hash_raw(value);
    hash ^ (hash >> 16)
}

fn java_string_hash_raw(value: &str) -> u32 {
    value.encode_utf16().fold(0u32, |hash, code_unit| {
        hash.wrapping_mul(31).wrapping_add(u32::from(code_unit))
    })
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
    query_wrapped: bool,
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

    if query_wrapped {
        let response_name = format!("{operation_name}Response");
        let result_name = format!("{operation_name}Result");
        writeln!(
            output,
            "    if !(start_el.matches({:?})) {{\n        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(\"invalid root, expected {} got {{start_el:?}}\")));\n    }}\n    if let Some(mut result_tag) = decoder.next_tag() {{\n        let start_el = result_tag.start_el();\n        if !(start_el.matches({:?})) {{\n            return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(\"invalid result, expected {} got {{start_el:?}}\")));\n        }}",
            response_name,
            response_name,
            result_name,
            result_name,
        )
        .unwrap();
    }

    if has_trait(operation, "aws.customizations#s3UnwrappedXmlOutput")
        && document_members.len() == 1
    {
        let (member_name, member) = &document_members[0];
        let target = member_target(member).unwrap_or_default();
        let field_method = rust_method_name(member_name);
        let xml_name = protocol_member_xml_name(selected, member_name, member);
        let var = state.temp();
        let parse = indent_expression(
            &protocol_parse_expression(selected, target, "decoder", "depth"),
            20,
        );
        writeln!(
            output,
            "    match start_el {{\n        s if s.matches({xml_name:?}) /* {xml_name} {synthetic_shape_id}${member_name} */ =>  {{\n            let {var} =\n                Some(\n                    {parse}\n                    ?\n                )\n            ;\n            builder = builder.set_{field_method}({var});\n        }}\n        ,\n        _ => return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(\"expected {xml_name} tag\"))\n    }}"
        )
        .unwrap();
    } else {
        if !query_wrapped {
            let allow_invalid_root =
                has_trait(output_shape, "smithy.api.internal#allowInvalidXmlRoot");
            if let Some(root) =
                protocol_operation_output_xml_name(selected, output_shape_id, output_shape)
                    .filter(|_| !allow_invalid_root)
            {
                writeln!(
                    output,
                    "    if !start_el.matches({root:?}) {{\n        return Err(\n                                ::aws_smithy_xml::decode::XmlDecodeError::custom(\n                                    format!(\"encountered invalid XML root: expected {root} but got {{start_el:?}}. This is likely a bug in the SDK.\")\n                                )\n                            );\n    }}"
                )
                .unwrap();
            }
        }
        let decoder = if query_wrapped {
            "result_tag"
        } else {
            "decoder"
        };
        writeln!(
            output,
            "    while let Some(mut tag) = {decoder}.next_tag() {{\n        match tag.start_el() {{"
        )
        .unwrap();
        for (member_name, member) in document_members {
            let target = member_target(member).unwrap_or_default();
            let field = names::rust_identifier(&member_name);
            let field_method = rust_method_name(&member_name);
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
                "            s if s.matches({xml_name:?}) /* {member_name} {member_id} */ =>  {{\n                let {outer} =\n                    Some(\n                        {parse}\n                        ?\n                    )\n                ;\n                builder = builder.set_{field_method}({outer});\n            }}\n            ,"
            )
            .unwrap();
        }
        output.push_str("            _ => {}\n        }\n    }\n");
    }
    if query_wrapped {
        writeln!(
            output,
            "    }} else {{\n        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(\"expected {operation_name}Result tag\"));\n    }};"
        )
        .unwrap();
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

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
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
fn render_protocol_serde_files(
    selected: &SelectedModel,
    protocol: ProtocolKind,
) -> (String, Vec<(String, String)>) {
    let query_mode = is_query_protocol(protocol);
    let roles = protocol_serde_roles(selected, query_mode);
    let mut files = Vec::new();
    let mut module_names = Vec::new();
    let mut module_names_seen = BTreeSet::new();
    let mut add_module = |module: String| {
        if module_names_seen.insert(module.clone()) {
            module_names.push(module);
        }
    };

    let mut deferred_modules = BTreeSet::new();
    let mut initial_modules = BTreeSet::new();
    for operation_name in &selected.operations {
        let module = names::rust_module_name(operation_name);
        initial_modules.insert(module.clone());
        if query_mode || render_protocol_input_file(selected, operation_name).is_some() {
            let input_module = format!("{module}_input");
            let output_is_event_stream = operation_shape(selected, operation_name)
                .and_then(|operation| operation.get("output"))
                .and_then(target_value)
                .and_then(|id| selected.model.shapes.get(id))
                .is_some_and(|output| {
                    members(output).into_iter().any(|(_, member)| {
                        has_trait(member, "smithy.api#httpPayload")
                            && member_target(member)
                                .is_some_and(|target| is_event_stream_target(selected, target))
                    })
                });
            if output_is_event_stream {
                deferred_modules.insert(input_module);
            } else {
                initial_modules.insert(input_module);
            }
        }
    }
    for module in initial_modules {
        add_module(module);
    }
    for operation_name in &selected.operations {
        let module = names::rust_module_name(operation_name);
        if protocol_output_has_headers(selected, operation_name)
            || render_protocol_output_payload_file(selected, operation_name).is_some()
        {
            deferred_modules.insert(format!("{module}_output"));
        }
    }
    for error_id in error_shape_ids(selected) {
        deferred_modules.insert(names::rust_module_name(terminal(&error_id)));
    }
    for module in deferred_modules {
        add_module(module);
    }

    for (shape_id, role) in protocol_serde_shape_order(selected, &roles, query_mode) {
        let module = names::rust_module_name(terminal(&shape_id));
        add_module(module.clone());
        let mut shape_roles = roles
            .get(&shape_id)
            .copied()
            .expect("ordered protocol shape exists");
        shape_roles.first = Some(role);
        files.push((
            format!("src/protocol_serde/shape_{module}.rs"),
            render_protocol_shape_file(selected, &shape_id, shape_roles, query_mode),
        ));
    }

    for error_id in error_shape_ids(selected) {
        let module = names::rust_module_name(terminal(&error_id));
        let path = format!("src/protocol_serde/shape_{module}.rs");
        if !files.iter().any(|(file, _)| file == &path) {
            files.push((
                path,
                render_protocol_error_file(selected, &error_id, query_mode),
            ));
        }
    }
    files.sort_by(|left, right| left.0.cmp(&right.0));

    let shared_start = module_names
        .iter()
        .position(|name| {
            roles
                .keys()
                .any(|shape_id| names::rust_module_name(terminal(shape_id)) == *name)
        })
        .unwrap_or(module_names.len());
    let struct_unset_before = selected
        .operations
        .iter()
        .find_map(|operation_name| {
            (protocol_input_payload_kind(selected, operation_name) == Some("structure")).then(
                || {
                    let module = format!("{}_input", names::rust_module_name(operation_name));
                    module_names.iter().position(|name| name == &module)
                },
            )
        })
        .flatten();
    let union_unset_before = selected
        .operations
        .iter()
        .find_map(|operation_name| {
            (protocol_input_payload_kind(selected, operation_name) == Some("union")).then(|| {
                let module = format!("{}_input", names::rust_module_name(operation_name));
                module_names.iter().position(|name| name == &module)
            })
        })
        .flatten();

    let mut module = String::new();
    client_operation_header(&mut module);
    module.push_str(
        "pub(crate) fn type_erase_result<O, E>(\n    result: ::std::result::Result<O, E>,\n) -> ::std::result::Result<\n    ::aws_smithy_runtime_api::client::interceptors::context::Output,\n    ::aws_smithy_runtime_api::client::orchestrator::OrchestratorError<::aws_smithy_runtime_api::client::interceptors::context::Error>,\n>\nwhere\n    O: ::std::fmt::Debug + ::std::marker::Send + ::std::marker::Sync + 'static,\n    E: ::std::error::Error + std::fmt::Debug + ::std::marker::Send + ::std::marker::Sync + 'static,\n{\n    result\n        .map(|output| ::aws_smithy_runtime_api::client::interceptors::context::Output::erase(output))\n        .map_err(|error| ::aws_smithy_runtime_api::client::interceptors::context::Error::erase(error))\n        .map_err(::std::convert::Into::into)\n}\n\n",
    );
    if request_id_plan(selected).extended {
        module.push_str(
            "pub fn parse_http_error_metadata(\n    response_status: u16,\n    _response_headers: &::aws_smithy_runtime_api::http::Headers,\n    response_body: &[u8],\n) -> ::std::result::Result<::aws_smithy_types::error::metadata::Builder, ::aws_smithy_xml::decode::XmlDecodeError> {\n    // S3 HEAD responses have no response body to for an error code. Therefore,\n    // check the HTTP response status and populate an error code for 404s.\n    if response_body.is_empty() {\n        let mut builder = ::aws_smithy_types::error::ErrorMetadata::builder();\n        if response_status == 404 {\n            builder = builder.code(\"NotFound\");\n        }\n        Ok(builder)\n    } else {\n        crate::rest_xml_unwrapped_errors::parse_error_metadata(response_body)\n    }\n}\n\n",
        );
    } else if query_mode {
        module.push_str(
            "pub fn parse_http_error_metadata(\n    _response_status: u16,\n    _response_headers: &::aws_smithy_runtime_api::http::Headers,\n    response_body: &[u8],\n) -> ::std::result::Result<::aws_smithy_types::error::metadata::Builder, ::aws_smithy_xml::decode::XmlDecodeError> {\n    crate::rest_xml_wrapped_errors::parse_error_metadata(response_body)\n}\n\n",
        );
    } else {
        module.push_str(
            "pub fn parse_http_error_metadata(\n    _response_status: u16,\n    _response_headers: &::aws_smithy_runtime_api::http::Headers,\n    response_body: &[u8],\n) -> ::std::result::Result<::aws_smithy_types::error::metadata::Builder, ::aws_smithy_xml::decode::XmlDecodeError> {\n    if response_body.is_empty() {\n        Ok(::aws_smithy_types::error::ErrorMetadata::builder())\n    } else {\n        crate::rest_xml_unwrapped_errors::parse_error_metadata(response_body)\n    }\n}\n\n",
        );
    }
    for (index, name) in module_names.into_iter().enumerate() {
        if !query_mode && struct_unset_before == Some(index) {
            module.push_str(
                "pub fn rest_xml_unset_struct_payload() -> ::std::vec::Vec<u8> {\n    Vec::new()\n}\n\n",
            );
        }
        if !query_mode && union_unset_before == Some(index) {
            module.push_str(
                "pub fn rest_xml_unset_union_payload() -> ::std::vec::Vec<u8> {\n    ::std::vec::Vec::new()\n}\n\n",
            );
        }
        if index == shared_start && !streaming_error_union_ids(selected).is_empty() {
            module.push_str(
                "pub fn parse_event_stream_error_metadata(\n    payload: &::bytes::Bytes,\n) -> ::std::result::Result<::aws_smithy_types::error::metadata::Builder, ::aws_smithy_xml::decode::XmlDecodeError> {\n    crate::rest_xml_unwrapped_errors::parse_error_metadata(payload.as_ref())\n}\n\n",
            );
        }
        writeln!(module, "pub(crate) mod shape_{name};").unwrap();
        module.push('\n');
    }
    (module, files)
}

/// Render the document serializers and parsers used by the AWS JSON and
/// REST-JSON protocols. AwsJson uses the same Smithy JSON token parser as
/// RestJson; the protocol difference is in the HTTP bindings, not in the
/// modeled document walk. Keeping this renderer shape-driven also means that
/// services with different operation sets share the same implementation.
fn render_json_protocol_serde_files(selected: &SelectedModel) -> (String, Vec<(String, String)>) {
    let roles = json_protocol_serde_roles(selected);
    let mut files = Vec::new();
    let mut modules = Vec::new();
    let mut seen = BTreeSet::new();
    for operation_name in &selected.operations {
        let module = names::rust_module_name(operation_name);
        if seen.insert(module.clone()) {
            modules.push(module);
        }
    }
    let operation_module_count = modules.len();
    for operation_name in &selected.operations {
        let Some(input) = operation_shape(selected, operation_name)
            .and_then(|operation| operation.get("input"))
            .and_then(target_value)
            .and_then(|id| selected.model.shapes.get(id))
        else {
            continue;
        };
        if json_protocol_input_needs_file(input) {
            let module = format!("{}_input", names::rust_module_name(operation_name));
            if seen.insert(module.clone()) {
                modules.push(module);
            }
        }
    }
    for operation_name in &selected.operations {
        let Some(output) = operation_shape(selected, operation_name)
            .and_then(|operation| operation.get("output"))
            .and_then(target_value)
            .and_then(|id| selected.model.shapes.get(id))
        else {
            continue;
        };
        if json_protocol_output_needs_file(output) {
            let module = format!("{}_output", names::rust_module_name(operation_name));
            if seen.insert(module.clone()) {
                modules.push(module);
            }
        }
    }
    for error_id in error_shape_ids(selected) {
        let module = names::rust_module_name(terminal(&error_id));
        if seen.insert(module.clone()) {
            modules.push(module);
        }
    }
    let shared_module_start = modules.len();
    for shape_id in roles.keys() {
        let module = names::rust_module_name(terminal(shape_id));
        if seen.insert(module.clone()) {
            modules.push(module);
        }
    }
    modules[operation_module_count..shared_module_start].sort();
    let shared_modules = json_protocol_shape_order(selected, &roles);
    modules.truncate(shared_module_start);
    let mut module_seen = modules.iter().cloned().collect::<BTreeSet<_>>();
    for (shape_id, _) in shared_modules {
        let module = names::rust_module_name(terminal(&shape_id));
        if module_seen.insert(module.clone()) {
            modules.push(module);
        }
    }

    for operation_name in &selected.operations {
        let module = names::rust_module_name(operation_name);
        files.push((
            format!("src/protocol_serde/shape_{module}.rs"),
            render_json_protocol_operation_file(selected, operation_name),
        ));
        let has_input = operation_shape(selected, operation_name)
            .and_then(|operation| operation.get("input"))
            .and_then(target_value)
            .and_then(|id| selected.model.shapes.get(id))
            .is_some_and(json_protocol_input_needs_file);
        if has_input {
            files.push((
                format!("src/protocol_serde/shape_{module}_input.rs"),
                render_json_protocol_operation_input_file(selected, operation_name),
            ));
        }
        let has_output = operation_shape(selected, operation_name)
            .and_then(|operation| operation.get("output"))
            .and_then(target_value)
            .and_then(|id| selected.model.shapes.get(id))
            .is_some_and(json_protocol_output_needs_file);
        if has_output {
            files.push((
                format!("src/protocol_serde/shape_{module}_output.rs"),
                render_json_protocol_operation_output_file(selected, operation_name),
            ));
        }
    }
    for (shape_id, shape_roles) in &roles {
        files.push((
            format!(
                "src/protocol_serde/shape_{}.rs",
                names::rust_module_name(terminal(shape_id))
            ),
            render_json_protocol_shape_file(selected, shape_id, *shape_roles),
        ));
    }
    for error_id in error_shape_ids(selected) {
        let module = names::rust_module_name(terminal(&error_id));
        let path = format!("src/protocol_serde/shape_{module}.rs");
        if !files.iter().any(|(file, _)| file == &path) {
            files.push((path, render_json_protocol_error_file(selected, &error_id)));
        }
    }
    files.sort_by(|left, right| left.0.cmp(&right.0));

    let mut module = String::new();
    client_operation_header(&mut module);
    module.push_str(
        "pub(crate) fn type_erase_result<O, E>(\n    result: ::std::result::Result<O, E>,\n) -> ::std::result::Result<\n    ::aws_smithy_runtime_api::client::interceptors::context::Output,\n    ::aws_smithy_runtime_api::client::orchestrator::OrchestratorError<::aws_smithy_runtime_api::client::interceptors::context::Error>,\n>\nwhere\n    O: ::std::fmt::Debug + ::std::marker::Send + ::std::marker::Sync + 'static,\n    E: ::std::error::Error + std::fmt::Debug + ::std::marker::Send + ::std::marker::Sync + 'static,\n{\n    result\n        .map(|output| ::aws_smithy_runtime_api::client::interceptors::context::Output::erase(output))\n        .map_err(|error| ::aws_smithy_runtime_api::client::interceptors::context::Error::erase(error))\n        .map_err(::std::convert::Into::into)\n}\n\n",
    );
    module.push_str(
        "pub fn parse_http_error_metadata(\n    _response_status: u16,\n    response_headers: &::aws_smithy_runtime_api::http::Headers,\n    response_body: &[u8],\n) -> ::std::result::Result<::aws_smithy_types::error::metadata::Builder, ::aws_smithy_json::deserialize::error::DeserializeError> {\n    crate::json_errors::parse_error_metadata(response_body, response_headers)\n}\n\n",
    );
    let unset_struct_before = selected.operations.iter().find_map(|operation_name| {
        let shape = operation_shape(selected, operation_name)
            .and_then(|operation| operation.get("input"))
            .and_then(target_value)
            .and_then(|id| selected.model.shapes.get(id))?;
        let has_payload = members(shape).into_iter().any(|(_, member)| {
            let target = member_target(member).unwrap_or_default();
            has_trait(member, "smithy.api#httpPayload")
                && protocol_shape_kind(selected, target) == "structure"
                && !member_is_effectively_required(selected, member, target)
        });
        has_payload.then(|| format!("{}_input", names::rust_module_name(operation_name)))
    });
    let unset_union_before = selected.operations.iter().find_map(|operation_name| {
        let shape = operation_shape(selected, operation_name)
            .and_then(|operation| operation.get("input"))
            .and_then(target_value)
            .and_then(|id| selected.model.shapes.get(id))?;
        let has_payload = members(shape).into_iter().any(|(_, member)| {
            let target = member_target(member).unwrap_or_default();
            has_trait(member, "smithy.api#httpPayload")
                && protocol_shape_kind(selected, target) == "union"
                && !member_is_effectively_required(selected, member, target)
        });
        has_payload.then(|| format!("{}_input", names::rust_module_name(operation_name)))
    });
    for (index, name) in modules.into_iter().enumerate() {
        if unset_struct_before.as_deref() == Some(name.as_str()) {
            module.push_str(
                "pub fn rest_json_unset_struct_payload() -> ::std::vec::Vec<u8> {\n    b\"{}\"[..].into()\n}\n\n",
            );
        }
        if unset_union_before.as_deref() == Some(name.as_str()) {
            module.push_str(
                "pub fn rest_json_unset_union_payload() -> ::std::vec::Vec<u8> {\n    ::std::vec::Vec::new()\n}\n\n",
            );
        }
        if index == operation_module_count {
            module.push_str(
                "pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {\n    if data.is_empty() {\n        b\"{}\"\n    } else {\n        data\n    }\n}\n\n",
            );
        }
        if index == shared_module_start && model_has_event_stream(selected) {
            module.push_str(
                "pub fn parse_event_stream_error_metadata(\n    payload: &::bytes::Bytes,\n) -> ::std::result::Result<::aws_smithy_types::error::metadata::Builder, ::aws_smithy_json::deserialize::error::DeserializeError> {\n    crate::json_errors::parse_error_metadata(payload, &::aws_smithy_runtime_api::http::Headers::new())\n}\n\n",
            );
        }
        writeln!(module, "pub(crate) mod shape_{name};").unwrap();
        module.push('\n');
    }
    (module, files)
}

fn json_protocol_input_needs_file(shape: &Value) -> bool {
    members(shape).into_iter().any(|(_, member)| {
        is_json_document_member(member) || has_trait(member, "smithy.api#httpPayload")
    })
}

fn json_protocol_output_needs_file(shape: &Value) -> bool {
    members(shape).into_iter().any(|(_, member)| {
        has_trait(member, "smithy.api#httpPayload")
            || has_trait(member, "smithy.api#httpHeader")
            || has_trait(member, "smithy.api#httpPrefixHeaders")
    })
}

fn json_protocol_shape_order(
    selected: &SelectedModel,
    roles: &BTreeMap<String, ProtocolSerdeRoles>,
) -> Vec<(String, ProtocolSerdeRole)> {
    let mut phase_one = Vec::new();
    for operation_name in &selected.operations {
        let Some(operation) = operation_shape(selected, operation_name) else {
            continue;
        };
        if let Some(input) = operation
            .get("input")
            .and_then(target_value)
            .and_then(|id| selected.model.shapes.get(id))
        {
            for (_, member) in members(input) {
                if (is_json_document_member(member) || has_trait(member, "smithy.api#httpPayload"))
                    && let Some(target) = member_target(member)
                {
                    json_protocol_first_role_dependencies(
                        selected,
                        target,
                        ProtocolSerdeRole::Serialize,
                        roles,
                        &mut BTreeSet::new(),
                        &mut phase_one,
                    );
                }
            }
        }
        if let Some(output) = operation
            .get("output")
            .and_then(target_value)
            .and_then(|id| selected.model.shapes.get(id))
        {
            for (_, member) in members(output) {
                if (is_json_document_member(member) || has_trait(member, "smithy.api#httpPayload"))
                    && let Some(target) = member_target(member)
                {
                    json_protocol_first_role_dependencies(
                        selected,
                        target,
                        ProtocolSerdeRole::Deserialize,
                        roles,
                        &mut BTreeSet::new(),
                        &mut phase_one,
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
                        json_protocol_first_role_dependencies(
                            selected,
                            target,
                            ProtocolSerdeRole::Deserialize,
                            roles,
                            &mut BTreeSet::new(),
                            &mut phase_one,
                        );
                    }
                }
            }
        }
    }

    let mut state_seen = BTreeSet::new();
    let mut current = phase_one;
    current.sort_by_key(|(shape_id, _)| names::rust_module_name(terminal(shape_id)));
    let mut ordered = Vec::new();
    while !current.is_empty() {
        current.retain(|(shape_id, role)| state_seen.insert((shape_id.clone(), *role)));
        if current.is_empty() {
            break;
        }
        current.sort_by_key(|(shape_id, _)| names::rust_module_name(terminal(shape_id)));
        ordered.extend(current.iter().cloned());
        let mut next = Vec::new();
        for (shape_id, role) in &current {
            json_protocol_role_dependencies(selected, shape_id, *role, roles, &mut next);
        }
        current = next;
    }

    let mut seen_modules = BTreeSet::new();
    let mut result = Vec::new();
    for (shape_id, role) in ordered {
        if seen_modules.insert(names::rust_module_name(terminal(&shape_id))) {
            result.push((shape_id, role));
        }
    }
    for shape_id in roles.keys() {
        if seen_modules.insert(names::rust_module_name(terminal(shape_id))) {
            result.push((
                shape_id.clone(),
                roles
                    .get(shape_id)
                    .and_then(|roles| roles.first)
                    .unwrap_or(ProtocolSerdeRole::Deserialize),
            ));
        }
    }
    result
}

fn json_protocol_first_role_dependencies(
    selected: &SelectedModel,
    shape_id: &str,
    role: ProtocolSerdeRole,
    roles: &BTreeMap<String, ProtocolSerdeRoles>,
    seen: &mut BTreeSet<(String, ProtocolSerdeRole)>,
    output: &mut Vec<(String, ProtocolSerdeRole)>,
) {
    if !seen.insert((shape_id.to_owned(), role)) {
        return;
    }
    if protocol_role_enabled(roles, shape_id, role) {
        output.push((shape_id.to_owned(), role));
        return;
    }
    let Some(shape) = selected.model.shapes.get(shape_id) else {
        return;
    };
    for (_, member) in protocol_shape_members(selected, shape) {
        if let Some(target) = member_target(member) {
            json_protocol_first_role_dependencies(selected, target, role, roles, seen, output);
        }
    }
}

fn json_protocol_role_dependencies(
    selected: &SelectedModel,
    shape_id: &str,
    role: ProtocolSerdeRole,
    roles: &BTreeMap<String, ProtocolSerdeRoles>,
    output: &mut Vec<(String, ProtocolSerdeRole)>,
) {
    let Some(shape) = selected.model.shapes.get(shape_id) else {
        return;
    };
    for (_, member) in protocol_shape_members(selected, shape) {
        if let Some(target) = member_target(member) {
            json_protocol_first_role_dependencies(
                selected,
                target,
                role,
                roles,
                &mut BTreeSet::new(),
                output,
            );
        }
    }
}

fn json_protocol_serde_roles(selected: &SelectedModel) -> BTreeMap<String, ProtocolSerdeRoles> {
    let mut roles = BTreeMap::new();
    for operation_name in &selected.operations {
        let Some(operation) = operation_shape(selected, operation_name) else {
            continue;
        };
        if let Some(input) = operation
            .get("input")
            .and_then(target_value)
            .and_then(|id| selected.model.shapes.get(id))
        {
            for (_, member) in members(input) {
                if (is_json_document_member(member) || has_trait(member, "smithy.api#httpPayload"))
                    && let Some(target) = member_target(member)
                {
                    json_protocol_mark_role(
                        selected,
                        target,
                        ProtocolSerdeRole::Serialize,
                        &mut roles,
                        &mut BTreeSet::new(),
                    );
                }
            }
        }
        if let Some(output) = operation
            .get("output")
            .and_then(target_value)
            .and_then(|id| selected.model.shapes.get(id))
        {
            for (_, member) in members(output) {
                if (is_json_document_member(member) || has_trait(member, "smithy.api#httpPayload"))
                    && let Some(target) = member_target(member)
                {
                    json_protocol_mark_role(
                        selected,
                        target,
                        ProtocolSerdeRole::Deserialize,
                        &mut roles,
                        &mut BTreeSet::new(),
                    );
                }
            }
        }
        if let Some(errors) = operation.get("errors").and_then(Value::as_array) {
            for error_id in errors.iter().filter_map(target_value) {
                if let Some(shape) = selected.model.shapes.get(error_id) {
                    for (_, member) in members(shape) {
                        if let Some(target) = member_target(member) {
                            json_protocol_mark_role(
                                selected,
                                target,
                                ProtocolSerdeRole::Deserialize,
                                &mut roles,
                                &mut BTreeSet::new(),
                            );
                        }
                    }
                }
            }
        }
    }
    roles.retain(|shape_id, role| {
        selected.model.shapes.contains_key(shape_id)
            && (role.serialize || role.deserialize)
            && matches!(
                protocol_shape_kind(selected, shape_id),
                "structure" | "union" | "list" | "map"
            )
    });
    roles
}

fn json_protocol_mark_role(
    selected: &SelectedModel,
    shape_id: &str,
    role: ProtocolSerdeRole,
    roles: &mut BTreeMap<String, ProtocolSerdeRoles>,
    seen: &mut BTreeSet<String>,
) {
    if !seen.insert(shape_id.to_owned()) {
        return;
    }
    let Some(shape) = selected.model.shapes.get(shape_id) else {
        return;
    };
    let kind = protocol_shape_kind(selected, shape_id);
    // Smithy-RS emits shared JSON serializers only for structures and unions;
    // collection and map values are serialized inline by their containing
    // serializer. Deserializers, however, need helpers for all compound
    // shapes so they can recursively build collection/map values.
    if !shape_is_streaming(shape)
        && ((matches!(role, ProtocolSerdeRole::Serialize) && matches!(kind, "structure" | "union"))
            || (matches!(role, ProtocolSerdeRole::Deserialize)
                && matches!(kind, "structure" | "union" | "list" | "map")))
    {
        record_protocol_role(roles, shape_id, role);
    }
    for (_, member) in protocol_shape_members(selected, shape) {
        if let Some(target) = member_target(member) {
            json_protocol_mark_role(selected, target, role, roles, seen);
        }
    }
}

fn render_json_protocol_operation_file(selected: &SelectedModel, operation_name: &str) -> String {
    let operation = operation_shape(selected, operation_name).expect("operation exists");
    let module = names::rust_module_name(operation_name);
    let operation_type = rust_type_name(operation_name);
    let operation_symbol = operation_error_type_name(operation_name);
    let error_path = format!("crate::operation::{module}::{operation_symbol}Error");
    let output_path = format!("crate::operation::{module}::{operation_type}Output");
    let mut output = String::new();
    client_operation_header(&mut output);
    let output_shape = operation
        .get("output")
        .and_then(target_value)
        .and_then(|id| selected.model.shapes.get(id));
    let output_payload = output_shape.and_then(|shape| {
        members(shape)
            .into_iter()
            .find(|(_, member)| has_trait(member, "smithy.api#httpPayload"))
    });
    let streaming_output = output_payload.as_ref().is_some_and(|(_, member)| {
        member_target(member).is_some_and(|target| {
            selected
                .model
                .shapes
                .get(target)
                .is_some_and(shape_is_streaming)
        })
    });
    if !streaming_output {
        render_json_protocol_http_error(&mut output, selected, operation_name, operation);
    }
    if streaming_output {
        writeln!(
            output,
            "#[allow(clippy::unnecessary_wraps)]\npub fn de_{module}_http_response(\n    response: &mut ::aws_smithy_runtime_api::http::Response,\n) -> std::result::Result<{output_path}, {error_path}> {{\n    let mut _response_body = ::aws_smithy_types::body::SdkBody::taken();\n    ::std::mem::swap(&mut _response_body, response.body_mut());\n    let _response_body = &mut _response_body;\n\n    let _response_status = response.status().as_u16();\n    let _response_headers = response.headers();\n    Ok({{\n        #[allow(unused_mut)]\n        let mut output = crate::operation::{module}::builders::{operation_type}OutputBuilder::default();"
        )
        .unwrap();
    } else {
        writeln!(
            output,
            "#[allow(clippy::unnecessary_wraps)]\npub fn de_{module}_http_response(\n    _response_status: u16,\n    _response_headers: &::aws_smithy_runtime_api::http::Headers,\n    _response_body: &[u8],\n) -> std::result::Result<{output_path}, {error_path}> {{\n    Ok({{\n        #[allow(unused_mut)]\n        let mut output = crate::operation::{module}::builders::{operation_type}OutputBuilder::default();"
        )
        .unwrap();
    }
    if !streaming_output
        && output_shape.is_some_and(|shape| {
            members(shape)
                .into_iter()
                .any(|(_, member)| is_json_document_member(member))
        })
    {
        writeln!(
            output,
            "        output = crate::protocol_serde::shape_{module}::de_{module}(_response_body, output)\n            .map_err({error_path}::unhandled)?;"
        )
        .unwrap();
    }
    if let Some(shape) = output_shape {
        for (name, member) in sorted_members(shape) {
            let field = names::rust_identifier(&name);
            if has_trait(member, "smithy.api#httpPayload") {
                let helper = format!("crate::protocol_serde::shape_{module}_output");
                if streaming_output {
                    writeln!(
                        output,
                        "        output = output.set_{field}(Some({helper}::de_{field}_payload(_response_body)?));"
                    )
                    .unwrap();
                } else {
                    writeln!(
                        output,
                        "        output = output.set_{field}({helper}::de_{field}_payload(_response_body)?);"
                    )
                    .unwrap();
                }
            } else if let Some(header) = member
                .get("traits")
                .and_then(|traits| traits.get("smithy.api#httpHeader"))
                .and_then(Value::as_str)
            {
                writeln!(
                    output,
                    "        output = output.set_{field}(crate::protocol_serde::shape_{module}_output::de_{field}_header(_response_headers).map_err(|_| {error_path}::unhandled(\"Failed to parse {name} from header `{header}`\"))?);"
                )
                .unwrap();
            } else if let Some(prefix) = member
                .get("traits")
                .and_then(|traits| traits.get("smithy.api#httpPrefixHeaders"))
                .and_then(Value::as_str)
            {
                writeln!(
                    output,
                    "        output = output.set_{field}(crate::protocol_serde::shape_{module}_output::de_{field}_prefix_header(_response_headers).map_err(|_| {error_path}::unhandled(\"Failed to parse {name} from prefix header `{prefix}`\"))?);"
                )
                .unwrap();
            } else if has_trait(member, "smithy.api#httpResponseCode") {
                writeln!(
                    output,
                    "        output = output.set_{field}(Some(_response_status as _));"
                )
                .unwrap();
            }
        }
    }
    if request_id_plan(selected).standard {
        output.push_str(
            "        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));\n",
        );
    }
    let streaming_event = streaming_output
        && output_payload.is_some_and(|(_, member)| {
            member_target(member)
                .is_some_and(|target| protocol_shape_kind(selected, target) == "union")
        });
    if let Some(shape) =
        output_shape.filter(|shape| serde_util_shape_needs_correction(shape) && !streaming_event)
    {
        let correction = format!(
            "crate::serde_util::{}_output_output_correct_errors(output)",
            module
        );
        if serde_util_builder_is_fallible(selected, shape) {
            writeln!(
                output,
                "        {correction}\n            .build()\n            .map_err({error_path}::unhandled)?\n    }})\n}}"
            )
            .unwrap();
        } else {
            writeln!(output, "        {correction}.build()\n    }})\n}}\n").unwrap();
        }
    } else if streaming_event {
        writeln!(
            output,
            "        output\n            .build()\n            .map_err({error_path}::unhandled)?\n    }})\n}}"
        )
        .unwrap();
    } else {
        output.push_str("        output.build()\n    })\n}\n");
    }
    if streaming_output {
        render_json_protocol_http_error(&mut output, selected, operation_name, operation);
    }
    render_json_protocol_operation_input_and_parser(
        &mut output,
        selected,
        operation_name,
        operation,
    );
    output = output
        .replace("::std::mem::swap", "std::mem::swap")
        .replace("`\"))?", "\"))?");
    output
}

fn render_json_protocol_operation_input_and_parser(
    output: &mut String,
    selected: &SelectedModel,
    operation_name: &str,
    operation: &Value,
) {
    let module = names::rust_module_name(operation_name);
    let operation_type = rust_type_name(operation_name);
    let input_shape = operation
        .get("input")
        .and_then(target_value)
        .and_then(|id| selected.model.shapes.get(id));
    let input_has_document_members = input_shape.is_some_and(|shape| {
        members(shape)
            .into_iter()
            .any(|(_, member)| is_json_document_member(member))
    });
    if input_has_document_members {
        writeln!(
            output,
            "\npub fn ser_{module}_input(\n    input: &crate::operation::{module}::{operation_type}Input,\n) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {{"
        )
        .unwrap();
        output.push_str("    let mut out = String::new();\n    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);\n");
        writeln!(
            output,
            "    crate::protocol_serde::shape_{module}_input::ser_{module}_input_input(&mut object, input)?;\n    object.finish();\n    Ok(::aws_smithy_types::body::SdkBody::from(out))\n}}\n"
        )
        .unwrap();
    }
    let output_shape = operation
        .get("output")
        .and_then(target_value)
        .and_then(|id| selected.model.shapes.get(id));
    if output_shape.is_none_or(|shape| {
        !members(shape)
            .into_iter()
            .any(|(_, member)| is_json_document_member(member))
    }) {
        return;
    }
    let output_path =
        format!("crate::operation::{module}::builders::{operation_type}OutputBuilder");
    writeln!(
        output,
        "\npub(crate) fn de_{module}(\n    _value: &[u8],\n    mut builder: {output_path},\n) -> ::std::result::Result<{output_path}, ::aws_smithy_json::deserialize::error::DeserializeError> {{"
    )
    .unwrap();
    output.push_str("    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();\n    let tokens = &mut tokens_owned;\n    #[allow(unused_variables)]\n    let depth = 0u32;\n    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;\n");
    render_json_structure_deserializer_loop(
        output,
        selected,
        output_shape,
        "builder",
        "depth",
        None,
        true,
    );
    output.push_str("    if tokens.next().is_some() {\n        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(\n            \"found more JSON tokens after completing parsing\",\n        ));\n    }\n    Ok(builder)\n}\n");
}

fn render_json_protocol_operation_input_file(
    selected: &SelectedModel,
    operation_name: &str,
) -> String {
    let operation = operation_shape(selected, operation_name).expect("operation exists");
    let module = names::rust_module_name(operation_name);
    let operation_type = rust_type_name(operation_name);
    let input_shape = operation
        .get("input")
        .and_then(target_value)
        .and_then(|id| selected.model.shapes.get(id))
        .expect("input shape");
    let mut output = String::new();
    client_operation_header(&mut output);
    let has_document_members = members(input_shape)
        .into_iter()
        .any(|(_, member)| is_json_document_member(member));
    if has_document_members {
        writeln!(
            output,
            "pub fn ser_{module}_input_input(\n    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,\n    input: &crate::operation::{module}::{operation_type}Input,\n) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {{"
        )
        .unwrap();
        let mut state = JsonRenderState::default();
        render_json_structure_serializer_body(
            &mut output,
            selected,
            input_shape,
            "object",
            "input",
            &mut state,
            true,
            true,
        );
        output.push_str("}\n");
    }
    if let Some((field, member)) = members(input_shape)
        .into_iter()
        .find(|(_, member)| has_trait(member, "smithy.api#httpPayload"))
    {
        let target = member_target(member).unwrap_or_default();
        let target_kind = protocol_shape_kind(selected, target);
        if target_kind == "union"
            && selected
                .model
                .shapes
                .get(target)
                .is_some_and(shape_is_streaming)
        {
            if let Some(shape) = selected.model.shapes.get(target) {
                for (event_name, event_member) in members(shape) {
                    if let Some(event_target) = member_target(event_member) {
                        render_json_payload_serializer(
                            &mut output,
                            selected,
                            &module,
                            &names::rust_identifier(&event_name),
                            event_target,
                            false,
                            false,
                        );
                    }
                }
            }
        } else {
            render_json_payload_serializer(
                &mut output,
                selected,
                &module,
                &names::rust_identifier(&field),
                target,
                !member_is_effectively_required(selected, member, target),
                true,
            );
        }
    }
    output
}

fn render_json_payload_serializer(
    output: &mut String,
    selected: &SelectedModel,
    operation_module: &str,
    field: &str,
    target: &str,
    optional: bool,
    http_payload: bool,
) {
    let target_kind = protocol_shape_kind(selected, target);
    if target_kind == "string" || target_kind == "enum" {
        writeln!(
            output,
            "pub fn ser_{field}_http_payload(\n    payload: ::std::option::Option<::std::string::String>,\n) -> ::std::result::Result<::std::vec::Vec<u8>, ::aws_smithy_types::error::operation::BuildError> {{\n    let payload = match payload {{\n        Some(t) => t,\n        None => return Ok(Vec::new()),\n    }};\n    Ok(payload.into_bytes())\n}}"
        )
        .unwrap();
        return;
    }
    if target_kind == "blob" {
        writeln!(
            output,
            "pub fn ser_{field}_http_payload(\n    payload: ::std::option::Option<::aws_smithy_types::Blob>,\n) -> ::std::result::Result<::bytes::Bytes, ::aws_smithy_types::error::operation::BuildError> {{\n    let payload = match payload {{\n        Some(t) => t,\n        None => return Ok(::bytes::Bytes::new()),\n    }};\n    Ok(::aws_smithy_types::Blob::from(payload).into_bytes())\n}}"
        )
        .unwrap();
        return;
    }
    if target_kind == "document" {
        writeln!(
            output,
            "pub fn ser_{field}_http_payload(\n    payload: &::std::option::Option<::aws_smithy_types::Document>,\n) -> ::std::result::Result<::std::vec::Vec<u8>, ::aws_smithy_types::error::operation::BuildError> {{\n    let payload = match payload.as_ref() {{\n        Some(t) => t,\n        None => return Ok(Vec::new()),\n    }};\n    let mut out = String::new();\n    ::aws_smithy_json::serialize::JsonValueWriter::new(&mut out).document(payload);\n    Ok(out.into_bytes())\n}}"
        )
        .unwrap();
        return;
    }
    if !matches!(target_kind, "structure" | "union") {
        return;
    }
    let target_name = rust_type_name(terminal(target));
    let target_module = names::rust_module_name(terminal(target));
    if !http_payload {
        writeln!(
            output,
            "pub fn ser_{field}_payload(\n    input: &crate::types::{target_name},\n) -> ::std::result::Result<::std::vec::Vec<u8>, ::aws_smithy_types::error::operation::SerializationError> {{\n    let mut out = String::new();\n    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);\n    crate::protocol_serde::shape_{target_module}::ser_{target_module}(&mut object, input)?;\n    object.finish();\n    Ok(out.into_bytes())\n}}"
        )
        .unwrap();
        return;
    }
    let payload_type = if optional {
        format!("&::std::option::Option<crate::types::{target_name}>")
    } else {
        format!("&crate::types::{target_name}")
    };
    let payload_binding = if optional {
        format!(
            "let payload = match payload.as_ref() {{\n        Some(t) => t,\n        None => return Ok(crate::protocol_serde::rest_json_unset_{}_payload()),\n    }};",
            if target_kind == "union" {
                "union"
            } else {
                "struct"
            }
        )
    } else {
        "let payload = payload;".to_owned()
    };
    writeln!(
        output,
        "pub fn ser_{field}_http_payload(\n    payload: {payload_type},\n) -> ::std::result::Result<::std::vec::Vec<u8>, ::aws_smithy_types::error::operation::BuildError> {{\n    {payload_binding}\n    Ok(crate::protocol_serde::shape_{operation_module}_input::ser_{field}_payload(payload)?)\n}}\n\npub fn ser_{field}_payload(\n    input: &crate::types::{target_name},\n) -> ::std::result::Result<::std::vec::Vec<u8>, ::aws_smithy_types::error::operation::SerializationError> {{\n    let mut out = String::new();\n    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);\n    crate::protocol_serde::shape_{target_module}::ser_{target_module}(&mut object, input)?;\n    object.finish();\n    Ok(out.into_bytes())\n}}",
    )
    .unwrap();
}

fn render_json_protocol_operation_output_file(
    selected: &SelectedModel,
    operation_name: &str,
) -> String {
    let operation = operation_shape(selected, operation_name).expect("operation exists");
    let module = names::rust_module_name(operation_name);
    let output_shape = operation
        .get("output")
        .and_then(target_value)
        .and_then(|id| selected.model.shapes.get(id))
        .expect("output shape");
    let mut output = String::new();
    client_operation_header(&mut output);
    if let Some((field_name, member)) = members(output_shape)
        .into_iter()
        .find(|(_, member)| has_trait(member, "smithy.api#httpPayload"))
    {
        let field = names::rust_identifier(&field_name);
        let target = member_target(member).unwrap_or_default();
        let kind = protocol_shape_kind(selected, target);
        let error_path = format!(
            "crate::operation::{}::{}Error",
            module,
            operation_error_type_name(operation_name)
        );
        let target_shape = selected.model.shapes.get(target);
        if kind == "union" && target_shape.is_some_and(shape_is_streaming) {
            writeln!(
                output,
                "pub fn de_{field}_payload(\n    body: &mut ::aws_smithy_types::body::SdkBody,\n) -> std::result::Result<crate::event_receiver::EventReceiver<crate::types::{}, crate::types::error::{}Error>, {error_path}> {{\n    let unmarshaller = crate::event_stream_serde::{}Unmarshaller::new();\n    let body = std::mem::replace(body, ::aws_smithy_types::body::SdkBody::taken());\n    let receiver = crate::event_receiver::EventReceiver::new(::aws_smithy_http::event_stream::Receiver::new(unmarshaller, body));\n    Ok(receiver)\n}}",
                rust_type_name(terminal(target)),
                rust_type_name(terminal(target)),
                rust_type_name(terminal(target)),
            )
            .unwrap();
        } else if terminal(target) == "StreamingBlob" {
            writeln!(
                output,
                "pub fn de_{field}_payload(\n    body: &mut ::aws_smithy_types::body::SdkBody,\n) -> std::result::Result<::aws_smithy_types::byte_stream::ByteStream, {error_path}> {{\n    let body = std::mem::replace(body, ::aws_smithy_types::body::SdkBody::taken());\n    Ok(::aws_smithy_types::byte_stream::ByteStream::new(body))\n}}"
            )
            .unwrap();
        } else if matches!(kind, "string" | "enum") {
            writeln!(
                output,
                "pub(crate) fn de_{field}_payload(\n    body: &[u8],\n) -> std::result::Result<::std::option::Option<::std::string::String>, {error_path}> {{\n    (!body.is_empty()).then(|| {{\n        let body_str = ::std::str::from_utf8(body).map_err({error_path}::unhandled)?;\n        Ok(body_str.to_owned())\n    }}).transpose()\n}}"
            )
            .unwrap();
        } else if kind == "blob" {
            writeln!(
                output,
                "pub(crate) fn de_{field}_payload(\n    body: &[u8],\n) -> std::result::Result<::std::option::Option<::aws_smithy_types::Blob>, {error_path}> {{\n    (!body.is_empty()).then(|| Ok(::aws_smithy_types::Blob::new(body))).transpose()\n}}"
            )
            .unwrap();
        } else if matches!(kind, "structure" | "union") {
            let target_name = protocol_shape_type(selected, target);
            writeln!(
                output,
                "pub(crate) fn de_{field}_payload(\n    body: &[u8],\n) -> std::result::Result<::std::option::Option<{target_name}>, {error_path}> {{\n    (!body.is_empty()).then(|| crate::protocol_serde::shape_{module}_output::de_{field}(body).map_err({error_path}::unhandled)).transpose()\n}}\n\npub(crate) fn de_{field}(\n    body: &[u8],\n) -> std::result::Result<{target_name}, ::aws_smithy_json::deserialize::error::DeserializeError> {{\n    let mut tokens = ::aws_smithy_json::deserialize::json_token_iter(body).peekable();\n    let value = crate::protocol_serde::shape_{}::de_{}(&mut tokens, body, 0)?\n        .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom(\"expected payload member value\"))?;\n    if tokens.next().is_some() {{\n        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(\"found more JSON tokens after completing parsing\"));\n    }}\n    Ok(value)\n}}",
                names::rust_module_name(terminal(target)),
                names::rust_module_name(terminal(target)),
            )
            .unwrap();
        }
    }
    for (name, member) in members(output_shape) {
        let Some(target) = member_target(member) else {
            continue;
        };
        if let Some(prefix) = member
            .get("traits")
            .and_then(|traits| traits.get("smithy.api#httpPrefixHeaders"))
            .and_then(Value::as_str)
        {
            render_protocol_response_prefix_header(
                &mut output,
                selected,
                &module,
                &name,
                target,
                prefix,
            );
            render_protocol_response_prefix_inner(&mut output, selected, &name, target);
        } else if let Some(header) = member
            .get("traits")
            .and_then(|traits| traits.get("smithy.api#httpHeader"))
            .and_then(Value::as_str)
        {
            render_protocol_response_header(&mut output, selected, &name, target, header, 1);
        }
    }
    output
}

#[derive(Default)]
struct JsonRenderState {
    next_name: usize,
}

impl JsonRenderState {
    fn var(&mut self) -> String {
        self.next_name += 1;
        format!("var_{}", self.next_name)
    }
    fn object(&mut self) -> String {
        self.next_name += 1;
        format!("object_{}", self.next_name)
    }
    fn array(&mut self) -> String {
        self.next_name += 1;
        format!("array_{}", self.next_name)
    }
    fn item(&mut self) -> String {
        self.next_name += 1;
        format!("item_{}", self.next_name)
    }
    fn key(&mut self) -> String {
        self.next_name += 1;
        format!("key_{}", self.next_name)
    }
    fn value(&mut self) -> String {
        self.next_name += 1;
        format!("value_{}", self.next_name)
    }
}

fn render_json_protocol_http_error(
    output: &mut String,
    selected: &SelectedModel,
    operation_name: &str,
    operation: &Value,
) {
    let module = names::rust_module_name(operation_name);
    let operation_type = rust_type_name(operation_name);
    let operation_symbol = operation_error_type_name(operation_name);
    let output_path = format!("crate::operation::{module}::{operation_type}Output");
    let error_path = format!("crate::operation::{module}::{operation_symbol}Error");
    writeln!(
        output,
        "#[allow(clippy::unnecessary_wraps)]\npub fn de_{module}_http_error(\n    _response_status: u16,\n    _response_headers: &::aws_smithy_runtime_api::http::Headers,\n    _response_body: &[u8],\n) -> std::result::Result<{output_path}, {error_path}> {{\n    #[allow(unused_mut)]\n    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)\n        .map_err({error_path}::unhandled)?;"
    )
    .unwrap();
    if request_id_plan(selected).standard {
        output.push_str("    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);\n");
    }
    let errors = operation
        .get("errors")
        .and_then(Value::as_array)
        .map(|errors| errors.iter().filter_map(target_value).collect::<Vec<_>>())
        .unwrap_or_default();
    if errors.is_empty() {
        writeln!(
            output,
            "    let generic = generic_builder.build();\n    Err({error_path}::generic(generic))\n}}\n"
        )
        .unwrap();
        return;
    }
    output.push_str("    let generic = generic_builder.build();\n    let error_code = match generic.code() {\n        Some(code) => code,\n        None => return Err(");
    writeln!(output, "{error_path}::unhandled(generic)),").unwrap();
    output.push_str("    };\n\n    let _error_message = generic.message().map(|msg| msg.to_owned());\n    Err(match error_code {\n");
    for error in errors {
        render_json_protocol_error_arm(output, &error_path, error);
    }
    writeln!(
        output,
        "        _ => {error_path}::generic(generic),\n    }})\n}}\n"
    )
    .unwrap();
}

fn render_json_protocol_error_arm(output: &mut String, error_path: &str, error: &str) {
    let error_name = rust_type_name(terminal(error));
    let error_module = names::rust_module_name(terminal(error));
    writeln!(
        output,
        "        {:?} => {error_path}::{error_name}({{",
        terminal(error)
    )
    .unwrap();
    output.push_str("            #[allow(unused_mut)]\n            let mut tmp = {\n");
    writeln!(
        output,
        "                #[allow(unused_mut)]\n                let mut output = crate::types::error::builders::{error_name}Builder::default();\n                output = crate::protocol_serde::shape_{error_module}::de_{error_module}_json_err(_response_body, output).map_err({error_path}::unhandled)?;\n                let output = output.meta(generic);\n                output.build()\n            }};"
    )
    .unwrap();
    output.push_str("            if tmp.message.is_none() {\n                tmp.message = _error_message;\n            }\n            tmp\n        }),\n");
}

fn render_json_protocol_error_file(selected: &SelectedModel, shape_id: &str) -> String {
    let shape = selected.model.shapes.get(shape_id).expect("error shape");
    let name = rust_type_name(terminal(shape_id));
    let module = names::rust_module_name(terminal(shape_id));
    let mut output = String::new();
    client_operation_header(&mut output);
    writeln!(
        output,
        "pub(crate) fn de_{module}_json_err(\n    _value: &[u8],\n    mut builder: crate::types::error::builders::{name}Builder,\n) -> ::std::result::Result<crate::types::error::builders::{name}Builder, ::aws_smithy_json::deserialize::error::DeserializeError> {{"
    )
    .unwrap();
    output.push_str("    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();\n    let tokens = &mut tokens_owned;\n    #[allow(unused_variables)]\n    let depth = 0u32;\n    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;\n");
    render_json_structure_deserializer_loop(
        &mut output,
        selected,
        Some(shape),
        "builder",
        "depth",
        Some("error"),
        false,
    );
    output.push_str("    if tokens.next().is_some() {\n        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(\n            \"found more JSON tokens after completing parsing\",\n        ));\n    }\n");
    if serde_util_shape_needs_correction(shape) {
        writeln!(
            output,
            "    Ok(crate::serde_util::{module}_correct_errors(builder).build().map_err(|_| ::aws_smithy_json::deserialize::error::DeserializeError::custom(\"missing field\"))?)\n}}\n"
        )
        .unwrap();
    } else {
        output.push_str("    Ok(builder)\n}\n");
    }
    output
}

fn render_json_protocol_shape_file(
    selected: &SelectedModel,
    shape_id: &str,
    roles: ProtocolSerdeRoles,
) -> String {
    let shape = selected.model.shapes.get(shape_id).expect("protocol shape");
    let mut output = String::new();
    client_operation_header(&mut output);
    if roles.serialize {
        match protocol_shape_kind(selected, shape_id) {
            "structure" => {
                render_json_shared_structure_serializer(&mut output, selected, shape_id, shape)
            }
            "union" => render_json_union_serializer(&mut output, selected, shape_id, shape),
            _ => {}
        }
    }
    if roles.deserialize {
        if roles.serialize {
            output.push('\n');
        }
        match protocol_shape_kind(selected, shape_id) {
            "structure" => {
                render_json_shared_structure_deserializer(&mut output, selected, shape_id, shape)
            }
            "union" => render_json_union_deserializer(&mut output, selected, shape_id, shape),
            "list" => render_json_list_deserializer(&mut output, selected, shape_id, shape),
            "map" => render_json_map_deserializer(&mut output, selected, shape_id, shape),
            _ => {}
        }
    }
    if roles.deserialize && protocol_shape_kind(selected, shape_id) == "union" {
        let type_name = protocol_shape_type(selected, shape_id);
        let old = format!("Some({type_name}::Unknown),\n                    }};");
        let new = format!(
            "Some({type_name}::Unknown)\n                        }}\n                    }};"
        );
        output = output.replace(&old, &new);
    }
    output
}

fn render_json_shared_structure_serializer(
    output: &mut String,
    selected: &SelectedModel,
    shape_id: &str,
    shape: &Value,
) {
    let module = names::rust_module_name(terminal(shape_id));
    let type_name = protocol_shape_type(selected, shape_id);
    if members(shape).is_empty() {
        writeln!(
            output,
            "pub fn ser_{module}(\n    #[allow(unused_variables)] object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,\n    #[allow(unused_variables)] input: &{type_name},\n) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {{\n    Ok(())\n}}"
        )
        .unwrap();
        return;
    }
    writeln!(
        output,
        "pub fn ser_{module}(\n    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,\n    input: &{type_name},\n) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {{"
    )
    .unwrap();
    let mut state = JsonRenderState::default();
    render_json_structure_serializer_body(
        output, selected, shape, "object", "input", &mut state, false, false,
    );
    output.push_str("}\n");
}

fn render_json_structure_serializer_body(
    output: &mut String,
    selected: &SelectedModel,
    shape: &Value,
    writer: &str,
    input: &str,
    state: &mut JsonRenderState,
    force_optional: bool,
    document_only: bool,
) {
    for (name, member) in members(shape) {
        if document_only && !is_json_document_member(member) {
            continue;
        }
        let field = names::rust_identifier(&name);
        let expression = format!("{input}.{field}");
        render_json_serialize_member(
            output,
            selected,
            &name,
            member,
            writer,
            &expression,
            state,
            0,
            force_optional || protocol_member_is_optional(selected, member),
        );
    }
    output.push_str("    Ok(())\n");
}

#[allow(clippy::too_many_arguments)]
fn render_json_serialize_member(
    output: &mut String,
    selected: &SelectedModel,
    member_name: &str,
    member: &Value,
    writer: &str,
    expression: &str,
    state: &mut JsonRenderState,
    indent: usize,
    optional: bool,
) {
    let Some(target) = member_target(member) else {
        return;
    };
    let prefix = " ".repeat(indent);
    if optional {
        let variable = state.var();
        writeln!(output, "{prefix}if let Some({variable}) = &{expression} {{").unwrap();
        render_json_serialize_value(
            output,
            selected,
            member_name,
            target,
            writer,
            &variable,
            state,
            indent + 4,
        );
        writeln!(output, "{prefix}}}").unwrap();
    } else {
        writeln!(output, "{prefix}{{").unwrap();
        render_json_serialize_value(
            output,
            selected,
            member_name,
            target,
            writer,
            expression,
            state,
            indent + 4,
        );
        writeln!(output, "{prefix}}}").unwrap();
    }
}

#[allow(clippy::too_many_arguments)]
fn render_json_serialize_value(
    output: &mut String,
    selected: &SelectedModel,
    member_name: &str,
    target: &str,
    writer: &str,
    value: &str,
    state: &mut JsonRenderState,
    indent: usize,
) {
    let prefix = " ".repeat(indent);
    let key = member_name;
    let writer = if member_name.is_empty() {
        writer.to_owned()
    } else {
        format!("{writer}.key({key:?})")
    };
    let value_ref = if value.starts_with('&')
        || value
            .chars()
            .all(|character| character == '_' || character.is_ascii_alphanumeric())
    {
        value.to_owned()
    } else {
        format!("&{value}")
    };
    let value_without_reference = value.strip_prefix('&').unwrap_or(value);
    let scalar_value = if value.starts_with('&') || !value.contains('.') {
        format!("*{value_ref}")
    } else {
        value_without_reference.to_owned()
    };
    match protocol_shape_kind(selected, target) {
        "string" | "enum" => writeln!(output, "{prefix}{writer}.string({value_without_reference}.as_str());").unwrap(),
        "boolean" => writeln!(output, "{prefix}{writer}.boolean({scalar_value});").unwrap(),
        "integer" | "long" | "short" | "byte" => writeln!(
            output,
            "{prefix}{writer}.number(\n{prefix}    #[allow(clippy::useless_conversion)]\n{prefix}    ::aws_smithy_types::Number::NegInt(({scalar_value}).into()),\n{prefix});"
        )
        .unwrap(),
        "float" | "double" => writeln!(
            output,
            "{prefix}{writer}.number(\n{prefix}    #[allow(clippy::useless_conversion)]\n{prefix}    ::aws_smithy_types::Number::Float(({scalar_value}).into()),\n{prefix});"
        )
        .unwrap(),
        "timestamp" => writeln!(
            output,
            "{prefix}{writer}.date_time({value_without_reference}, ::aws_smithy_types::date_time::Format::{})?;",
            json_timestamp_format(selected, target)
        )
        .unwrap(),
        "blob" => writeln!(
            output,
            "{prefix}{writer}.string_unchecked(&::aws_smithy_types::base64::encode({value_without_reference}));"
        )
        .unwrap(),
        "list" => {
            let array = state.array();
            writeln!(output, "{prefix}let mut {array} = {writer}.start_array();").unwrap();
            let item = state.item();
            let item_member = selected.model.shapes.get(target).and_then(|shape| shape.get("member"));
            writeln!(output, "{prefix}for {item} in {value_ref} {{").unwrap();
            if let Some(item_member) = item_member {
                let sparse = selected
                    .model
                    .shapes
                    .get(target)
                    .is_some_and(|shape| has_trait(shape, "smithy.api#sparse"));
                render_json_serialize_member(
                    output,
                    selected,
                    "",
                    item_member,
                    &format!("{array}.value()"),
                    &item,
                    state,
                    indent + 4,
                    sparse,
                );
            }
            writeln!(output, "{prefix}}}\n{prefix}{array}.finish();").unwrap();
        }
        "map" => {
            let object = state.object();
            writeln!(output, "{prefix}#[allow(unused_mut)]\n{prefix}let mut {object} = {writer}.start_object();").unwrap();
            let key_var = state.key();
            let value_var = state.value();
            let value_member = selected.model.shapes.get(target).and_then(|shape| shape.get("value"));
            writeln!(output, "{prefix}for ({key_var}, {value_var}) in {value_ref} {{").unwrap();
            if let Some(value_member) = value_member {
                let value_target = member_target(value_member).unwrap_or_default();
                let sparse = selected
                    .model
                    .shapes
                    .get(target)
                    .is_some_and(|shape| has_trait(shape, "smithy.api#sparse"));
                render_json_serialize_map_value(
                    output,
                    selected,
                    value_member,
                    value_target,
                    &object,
                    &key_var,
                    &value_var,
                    state,
                    indent + 4,
                    sparse,
                );
            }
            writeln!(output, "{prefix}}}\n{prefix}{object}.finish();").unwrap();
        }
        "structure" => {
            let object = state.object();
            writeln!(output, "{prefix}#[allow(unused_mut)]\n{prefix}let mut {object} = {writer}.start_object();").unwrap();
            writeln!(output, "{prefix}crate::protocol_serde::shape_{}::ser_{}(&mut {object}, {value_ref})?;", names::rust_module_name(terminal(target)), names::rust_module_name(terminal(target))).unwrap();
            writeln!(output, "{prefix}{object}.finish();").unwrap();
        }
        "union" => {
            let object = state.object();
            writeln!(output, "{prefix}#[allow(unused_mut)]\n{prefix}let mut {object} = {writer}.start_object();").unwrap();
            writeln!(output, "{prefix}crate::protocol_serde::shape_{}::ser_{}(&mut {object}, {value_ref})?;", names::rust_module_name(terminal(target)), names::rust_module_name(terminal(target))).unwrap();
            writeln!(output, "{prefix}{object}.finish();").unwrap();
        }
        "document" => writeln!(output, "{prefix}{writer}.document({value_ref});").unwrap(),
        _ => {}
    }
}

#[allow(clippy::too_many_arguments)]
fn render_json_serialize_map_value(
    output: &mut String,
    selected: &SelectedModel,
    member: &Value,
    target: &str,
    object: &str,
    key: &str,
    value: &str,
    state: &mut JsonRenderState,
    indent: usize,
    sparse: bool,
) {
    let prefix = " ".repeat(indent);
    let writer = format!("{object}.key({key}.as_str())");
    if sparse {
        let variable = state.var();
        writeln!(
            output,
            "{prefix}if let Some({variable}) = {value}.as_ref() {{"
        )
        .unwrap();
        render_json_serialize_value(
            output,
            selected,
            "",
            target,
            &writer,
            &variable,
            state,
            indent + 4,
        );
        writeln!(
            output,
            "{prefix}}} else {{\n{prefix}    {writer}.null();\n{prefix}}}"
        )
        .unwrap();
    } else {
        render_json_serialize_member(
            output, selected, "", member, &writer, value, state, indent, false,
        );
    }
}

fn render_json_union_serializer(
    output: &mut String,
    selected: &SelectedModel,
    shape_id: &str,
    shape: &Value,
) {
    let module = names::rust_module_name(terminal(shape_id));
    let type_name = protocol_shape_type(selected, shape_id);
    writeln!(
        output,
        "pub fn ser_{module}(\n    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,\n    input: &{type_name},\n) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {{\n    match input {{"
    )
    .unwrap();
    for (name, member) in members(shape) {
        let variant = rust_type_name(&name);
        let target = member_target(member).unwrap_or_default();
        if protocol_shape_kind(selected, target) == "unit" {
            writeln!(output, "        {type_name}::{variant} => {{\n            object.key({name:?}).null();\n        }}").unwrap();
        } else {
            writeln!(output, "        {type_name}::{variant}(inner) => {{").unwrap();
            render_json_serialize_value(
                output,
                selected,
                &name,
                target,
                "object",
                "inner",
                &mut JsonRenderState::default(),
                12,
            );
            output.push_str("        }\n");
        }
    }
    writeln!(
        output,
        "        {type_name}::Unknown => {{\n            return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant(\n                {:?},\n            ))\n        }}",
        rust_type_name(terminal(shape_id))
    )
    .unwrap();
    output.push_str("    }\n    Ok(())\n}\n");
}

fn render_json_shared_structure_deserializer(
    output: &mut String,
    selected: &SelectedModel,
    shape_id: &str,
    shape: &Value,
) {
    let module = names::rust_module_name(terminal(shape_id));
    let type_name = protocol_shape_type(selected, shape_id);
    if members(shape).is_empty() {
        writeln!(
            output,
            "pub(crate) fn de_{module}<'a, I>(\n    tokens: &mut ::std::iter::Peekable<I>,\n    _value: &'a [u8],\n    depth: u32,\n) -> ::std::result::Result<Option<crate::types::{type_name}>, ::aws_smithy_json::deserialize::error::DeserializeError>\nwhere\n    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,\n{{\n    if depth >= 128u32 {{\n        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(\n            \"maximum nesting depth exceeded\",\n        ));\n    }}\n    match tokens.next().transpose()? {{\n        Some(::aws_smithy_json::deserialize::Token::ValueNull {{ .. }}) => Ok(None),\n        Some(::aws_smithy_json::deserialize::Token::StartObject {{ .. }}) => {{\n            #[allow(unused_mut)]\n            let mut builder = crate::types::builders::{type_name}Builder::default();\n            ::aws_smithy_json::deserialize::token::skip_to_end(tokens)?;\n            Ok(Some(builder.build()))\n        }}\n        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(\n            \"expected start object or null\",\n        )),\n    }}\n}}"
        )
        .unwrap();
        return;
    }
    writeln!(
        output,
        "pub(crate) fn de_{module}<'a, I>(\n    tokens: &mut ::std::iter::Peekable<I>,\n    _value: &'a [u8],\n    depth: u32,\n) -> ::std::result::Result<Option<{type_name}>, ::aws_smithy_json::deserialize::error::DeserializeError>\nwhere\n    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,\n{{"
    )
    .unwrap();
    output.push_str("    if depth >= 128u32 {\n        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(\n            \"maximum nesting depth exceeded\",\n        ));\n    }\n    match tokens.next().transpose()? {\n        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),\n        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {\n            #[allow(unused_mut)]\n");
    writeln!(
        output,
        "            let mut builder = crate::types::builders::{}Builder::default();",
        rust_type_name(terminal(shape_id))
    )
    .unwrap();
    render_json_structure_deserializer_loop(
        output,
        selected,
        Some(shape),
        "builder",
        "depth",
        None,
        false,
    );
    let result = json_builder_result(selected, shape_id, "builder", "Response was invalid");
    writeln!(output, "            Ok(Some({result}))").unwrap();
    output.push_str("        }\n        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(\n            \"expected start object or null\",\n        )),\n    }\n}\n");
}

fn render_json_list_deserializer(
    output: &mut String,
    selected: &SelectedModel,
    shape_id: &str,
    shape: &Value,
) {
    let module = names::rust_module_name(terminal(shape_id));
    let type_name = protocol_shape_type(selected, shape_id);
    let member = shape.get("member").expect("list member");
    let target = member_target(member).unwrap_or_default();
    let sparse = has_trait(shape, "smithy.api#sparse");
    writeln!(
        output,
        "pub(crate) fn de_{module}<'a, I>(\n    tokens: &mut ::std::iter::Peekable<I>,\n    _value: &'a [u8],\n    depth: u32,\n) -> ::std::result::Result<Option<{type_name}>, ::aws_smithy_json::deserialize::error::DeserializeError>\nwhere\n    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,\n{{"
    )
    .unwrap();
    output.push_str("    if depth >= 128u32 {\n        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(\n            \"maximum nesting depth exceeded\",\n        ));\n    }\n    match tokens.next().transpose()? {\n        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),\n        Some(::aws_smithy_json::deserialize::Token::StartArray { .. }) => {\n            let mut items = Vec::new();\n            loop {\n                match tokens.peek() {\n                    Some(Ok(::aws_smithy_json::deserialize::Token::EndArray { .. })) => {\n                        tokens.next().transpose().unwrap();\n                        break;\n                    }\n                    _ => {\n");
    let expression = json_deserialize_expression(selected, target, "tokens", "_value", "depth + 1");
    if sparse {
        writeln!(output, "                        let value = {expression};\n                        items.push(value);").unwrap();
    } else {
        writeln!(output, "                        let value = {expression};\n                        if let Some(value) = value {{\n                            items.push(value);\n                        }} else {{\n                            return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(\n                                \"dense list cannot contain null values\",\n                            ));\n                        }}").unwrap();
    }
    output.push_str("                    }\n                }\n            }\n            Ok(Some(items))\n        }\n        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(\n            \"expected start array or null\",\n        )),\n    }\n}\n");
}

fn render_json_map_deserializer(
    output: &mut String,
    selected: &SelectedModel,
    shape_id: &str,
    shape: &Value,
) {
    let module = names::rust_module_name(terminal(shape_id));
    let type_name = protocol_shape_type(selected, shape_id);
    let key = shape.get("key").expect("map key");
    let value = shape.get("value").expect("map value");
    let key_target = member_target(key).unwrap_or("smithy.api#String");
    let value_target = member_target(value).unwrap_or_default();
    let sparse = has_trait(shape, "smithy.api#sparse");
    writeln!(
        output,
        "pub(crate) fn de_{module}<'a, I>(\n    tokens: &mut ::std::iter::Peekable<I>,\n    _value: &'a [u8],\n    depth: u32,\n) -> ::std::result::Result<Option<{type_name}>, ::aws_smithy_json::deserialize::error::DeserializeError>\nwhere\n    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,\n{{"
    )
    .unwrap();
    output.push_str("    if depth >= 128u32 {\n        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(\n            \"maximum nesting depth exceeded\",\n        ));\n    }\n    match tokens.next().transpose()? {\n        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),\n        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {\n            let mut map = ::std::collections::HashMap::new();\n            loop {\n                match tokens.next().transpose()? {\n                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,\n                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {\n                        let key = key.to_unescaped().map(|u| u.into_owned())?;\n");
    let key_expression = json_deserialize_key_expression(selected, key_target, "key");
    if key_expression != "key" {
        writeln!(
            output,
            "                        let key = {key_expression};"
        )
        .unwrap();
    }
    let value_expression =
        json_deserialize_expression(selected, value_target, "tokens", "_value", "depth + 1");
    if sparse {
        writeln!(
            output,
            "                        map.insert(key, {value_expression});"
        )
        .unwrap();
    } else {
        writeln!(output, "                        let value = {value_expression};\n                        match value {{\n                            Some(value) => {{\n                                map.insert(key, value);\n                            }}\n                            None => return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(\n                                \"dense map cannot contain null values\",\n                            )),\n                        }}").unwrap();
    }
    output.push_str("                    }\n                    other => {\n                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(\n                            \"expected object key or end object, found: {other:?}\"\n                        )))\n                    }\n                }\n            }\n            Ok(Some(map))\n        }\n        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(\n            \"expected start object or null\",\n        )),\n    }\n}\n");
}

fn render_json_union_deserializer(
    output: &mut String,
    selected: &SelectedModel,
    shape_id: &str,
    shape: &Value,
) {
    let module = names::rust_module_name(terminal(shape_id));
    let type_name = protocol_shape_type(selected, shape_id);
    writeln!(
        output,
        "pub(crate) fn de_{module}<'a, I>(\n    tokens: &mut ::std::iter::Peekable<I>,\n    _value: &'a [u8],\n    depth: u32,\n) -> ::std::result::Result<Option<{type_name}>, ::aws_smithy_json::deserialize::error::DeserializeError>\nwhere\n    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,\n{{"
    )
    .unwrap();
    output.push_str("    if depth >= 128u32 {\n        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(\n            \"maximum nesting depth exceeded\",\n        ));\n    }\n    let mut variant = None;\n    match tokens.next().transpose()? {\n        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => return Ok(None),\n        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => loop {\n            match tokens.next().transpose()? {\n                Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,\n                Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {\n                    if let Some(Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) = tokens.peek() {\n                        let _ = tokens.next().expect(\"peek returned a token\")?;\n                        continue;\n                    }\n                    let key = key.to_unescaped()?;\n                    if key == \"__type\" {\n                        ::aws_smithy_json::deserialize::token::skip_value(tokens)?;\n                        continue;\n                    }\n                    if variant.is_some() {\n                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(\n                            \"encountered mixed variants in union\",\n                        ));\n                    }\n                    variant = match key.as_ref() {\n");
    for (name, member) in members(shape) {
        let target = member_target(member).unwrap_or_default();
        let variant_name = rust_type_name(&name);
        let expression =
            json_deserialize_expression(selected, target, "tokens", "_value", "depth + 1");
        if protocol_shape_kind(selected, target) == "unit" {
            writeln!(output, "                        {name:?} => {{\n                            ::aws_smithy_json::deserialize::token::skip_value(tokens)?;\n                            Some({type_name}::{variant_name})\n                        }}").unwrap();
        } else {
            writeln!(output, "                        {name:?} => Some({type_name}::{variant_name}({expression}.ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom(\"value for '{name}' cannot be null\"))?)),").unwrap();
        }
    }
    output.push_str("                        _ => {\n                            ::aws_smithy_json::deserialize::token::skip_value(tokens)?;\n                            Some(");
    output.push_str(&format!("{type_name}::Unknown"));
    output.push_str("),\n                    };\n                }\n                other => {\n                    return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(\n                        \"expected object key or end object, found: {other:?}\"\n                    )))\n                }\n            }\n        },\n        _ => {\n            return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(\n                \"expected start object or null\",\n            ))\n        }\n    }\n    if variant.is_none() {\n        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(\n            \"Union did not contain a valid variant.\",\n        ));\n    }\n    Ok(variant)\n}\n");
}

fn render_json_structure_deserializer_loop(
    output: &mut String,
    selected: &SelectedModel,
    shape: Option<&Value>,
    builder: &str,
    depth: &str,
    _context: Option<&str>,
    document_only: bool,
) {
    output.push_str("    loop {\n        match tokens.next().transpose()? {\n            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,\n            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {\n");
    if let Some(shape) = shape {
        for (name, member) in members(shape) {
            if document_only && !is_json_document_member(member) {
                continue;
            }
            let target = member_target(member).unwrap_or_default();
            let field = names::rust_identifier(&name);
            let setter = names::rustdoc_identifier(&field);
            let expression = json_deserialize_expression(
                selected,
                target,
                "tokens",
                "_value",
                &format!("{depth} + 1"),
            );
            writeln!(output, "                {name:?} => {{\n                    {builder} = {builder}.set_{setter}({expression});\n                }},").unwrap();
        }
    }
    output.push_str("                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,\n            },\n            other => {\n                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(\n                    \"expected object key or end object, found: {other:?}\"\n                )))\n            }\n        }\n    }\n");
}

fn json_deserialize_expression(
    selected: &SelectedModel,
    target: &str,
    tokens: &str,
    value: &str,
    depth: &str,
) -> String {
    match protocol_shape_kind(selected, target) {
        "string" => format!(
            "::aws_smithy_json::deserialize::token::expect_string_or_null({tokens}.next())?\n                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))\n                            .transpose()?"
        ),
        "enum" => format!(
            "::aws_smithy_json::deserialize::token::expect_string_or_null({tokens}.next())?\n                            .map(|s| s.to_unescaped().map(|u| crate::types::{}::from(u.as_ref())))\n                            .transpose()?",
            rust_type_name(terminal(target))
        ),
        "boolean" => {
            format!("::aws_smithy_json::deserialize::token::expect_bool_or_null({tokens}.next())?")
        }
        "integer" | "long" | "short" | "byte" => format!(
            "::aws_smithy_json::deserialize::token::expect_number_or_null({tokens}.next())?\n                            .map({}::try_from)\n                            .transpose()?",
            match protocol_shape_kind(selected, target) {
                "integer" => "i32",
                "long" => "i64",
                "short" => "i16",
                "byte" => "i8",
                _ => unreachable!(),
            }
        ),
        "float" => format!(
            "::aws_smithy_json::deserialize::token::expect_number_or_null({tokens}.next())?.map(|v| v.to_f32_lossy())"
        ),
        "double" => format!(
            "::aws_smithy_json::deserialize::token::expect_number_or_null({tokens}.next())?.map(|v| v.to_f64_lossy())"
        ),
        "timestamp" => format!(
            "::aws_smithy_json::deserialize::token::expect_timestamp_or_null({tokens}.next(), ::aws_smithy_types::date_time::Format::{})?",
            json_timestamp_format(selected, target)
        ),
        "blob" => {
            format!("::aws_smithy_json::deserialize::token::expect_blob_or_null({tokens}.next())?")
        }
        "list" | "map" | "structure" | "union" => format!(
            "crate::protocol_serde::shape_{}::de_{}({tokens}, {value}, {depth})?",
            names::rust_module_name(terminal(target)),
            names::rust_module_name(terminal(target))
        ),
        "document" => format!(
            "Some(::aws_smithy_json::deserialize::token::expect_document({tokens}.next())?)"
        ),
        _ => format!("::aws_smithy_json::deserialize::token::skip_value({tokens})?"),
    }
}

fn json_deserialize_key_expression(selected: &SelectedModel, target: &str, key: &str) -> String {
    if protocol_shape_kind(selected, target) == "string"
        || protocol_shape_kind(selected, target) == "enum"
    {
        "key".to_owned()
    } else {
        key.to_owned()
    }
}

fn json_builder_result(
    selected: &SelectedModel,
    shape_id: &str,
    builder: &str,
    source: &str,
) -> String {
    if serde_util_shape_needs_correction(selected.model.shapes.get(shape_id).expect("shape")) {
        if serde_util_builder_is_fallible(
            selected,
            selected.model.shapes.get(shape_id).expect("shape"),
        ) {
            format!(
                "crate::serde_util::{}_correct_errors({builder}).build().map_err(|err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source(\"{source}\", err))?",
                names::rust_module_name(terminal(shape_id))
            )
        } else {
            format!(
                "crate::serde_util::{}_correct_errors({builder}).build()",
                names::rust_module_name(terminal(shape_id))
            )
        }
    } else {
        format!("{builder}.build()")
    }
}

fn protocol_input_payload_kind(
    selected: &SelectedModel,
    operation_name: &str,
) -> Option<&'static str> {
    let operation = operation_shape(selected, operation_name)?;
    let input = operation
        .get("input")
        .and_then(target_value)
        .and_then(|id| selected.model.shapes.get(id))?;
    let member = members(input)
        .into_iter()
        .find(|(_, member)| has_trait(member, "smithy.api#httpPayload"))?
        .1;
    let target = member_target(member)?;
    match protocol_shape_kind(selected, target) {
        "structure" => Some("structure"),
        "union" => Some("union"),
        _ => None,
    }
}

/// Return shared protocol helpers in the order in which Smithy's lazy inline
/// dependencies become reachable. Operation wrappers and input helpers are
/// loaded first. Output payload and error helpers are loaded next, so their
/// direct dependencies join the first shared-helper wave. Each later wave is
/// made from the dependencies of the preceding wave and sorted by module name
/// to preserve deterministic output.
fn protocol_serde_shape_order(
    selected: &SelectedModel,
    roles: &BTreeMap<String, ProtocolSerdeRoles>,
    query_mode: bool,
) -> Vec<(String, ProtocolSerdeRole)> {
    let mut seen = BTreeSet::new();
    protocol_serde_shape_waves(selected, roles, query_mode)
        .into_iter()
        .flatten()
        .filter(|(shape_id, _)| seen.insert(names::rust_module_name(terminal(shape_id))))
        .collect()
}

fn protocol_serde_shape_waves(
    selected: &SelectedModel,
    roles: &BTreeMap<String, ProtocolSerdeRoles>,
    query_mode: bool,
) -> Vec<Vec<(String, ProtocolSerdeRole)>> {
    let mut phase_one = Vec::new();
    let mut phase_two = Vec::new();
    for operation_name in &selected.operations {
        let Some(operation) = operation_shape(selected, operation_name) else {
            continue;
        };
        if let Some(input) = operation
            .get("input")
            .and_then(target_value)
            .and_then(|id| selected.model.shapes.get(id))
        {
            for (_, member) in members(input) {
                let Some(target) = member_target(member) else {
                    continue;
                };
                if query_mode || is_xml_document_member(member) {
                    protocol_first_role_dependencies(
                        selected,
                        target,
                        member,
                        ProtocolSerdeRole::Serialize,
                        roles,
                        &mut BTreeSet::new(),
                        &mut phase_one,
                    );
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
                    if has_trait(member, "smithy.api#httpPayload") {
                        protocol_first_role_dependencies(
                            selected,
                            target,
                            member,
                            ProtocolSerdeRole::Deserialize,
                            roles,
                            &mut BTreeSet::new(),
                            &mut phase_two,
                        );
                    } else {
                        protocol_first_role_dependencies_preserving_intermediates(
                            selected,
                            target,
                            member,
                            ProtocolSerdeRole::Deserialize,
                            roles,
                            &mut BTreeSet::new(),
                            &mut phase_one,
                        );
                    }
                }
            }
        }
        if let Some(errors) = operation.get("errors").and_then(Value::as_array) {
            for error in errors.iter().filter_map(target_value) {
                let Some(shape) = selected.model.shapes.get(error) else {
                    continue;
                };
                for (_, member) in members(shape) {
                    let Some(target) = member_target(member) else {
                        continue;
                    };
                    protocol_first_role_dependencies(
                        selected,
                        target,
                        member,
                        ProtocolSerdeRole::Deserialize,
                        roles,
                        &mut BTreeSet::new(),
                        &mut phase_two,
                    );
                }
            }
        }
    }

    let mut state_seen = BTreeSet::new();
    let (initial_rendered, initial_intermediates) = phase_one
        .into_iter()
        .map(|(shape_id, role)| (shape_id.to_owned(), role))
        .partition::<Vec<_>, _>(|(shape_id, role)| protocol_role_enabled(roles, shape_id, *role));
    let mut current = initial_rendered
        .into_iter()
        .chain(initial_intermediates)
        .collect::<Vec<_>>();
    current.sort_by_key(|(shape_id, _)| names::rust_module_name(terminal(shape_id)));
    let deferred = phase_two
        .into_iter()
        .filter(|(shape_id, role)| protocol_role_enabled(roles, shape_id, *role))
        .map(|(shape_id, role)| (shape_id.to_owned(), role))
        .collect::<Vec<_>>();
    let mut ordered_waves = Vec::new();

    while !current.is_empty() {
        let first_level = ordered_waves.is_empty();
        current.retain(|state| state_seen.insert((state.0.clone(), protocol_role_key(state.1))));
        if current.is_empty() {
            break;
        }
        let mut level = current
            .iter()
            .filter_map(|(shape_id, role)| {
                if !protocol_role_enabled(roles, shape_id, *role) {
                    return None;
                }
                Some((shape_id.clone(), *role))
            })
            .collect::<Vec<_>>();
        level.sort_by_key(|(shape_id, _)| names::rust_module_name(terminal(shape_id)));
        ordered_waves.push(level);

        let mut next = Vec::new();
        for (shape_id, role) in &current {
            protocol_role_dependencies(selected, shape_id, *role, roles, &mut next);
        }
        if first_level {
            next.extend(deferred.iter().cloned());
        }
        current = next;
    }

    let state_modules = ordered_waves
        .iter()
        .flatten()
        .map(|(shape_id, _)| names::rust_module_name(terminal(shape_id)))
        .collect::<BTreeSet<_>>();
    let mut remainder = roles
        .keys()
        .filter(|shape_id| !state_modules.contains(&names::rust_module_name(terminal(shape_id))))
        .filter_map(|shape_id| {
            roles
                .get(shape_id)
                .and_then(|role| role.first)
                .map(|role| (shape_id.clone(), role))
        })
        .collect::<Vec<_>>();
    remainder.sort_by_key(|(shape_id, _)| names::rust_module_name(terminal(shape_id)));
    if !remainder.is_empty() {
        ordered_waves.push(remainder);
    }
    ordered_waves
}

fn protocol_role_key(role: ProtocolSerdeRole) -> bool {
    matches!(role, ProtocolSerdeRole::Serialize)
}

fn protocol_role_enabled(
    roles: &BTreeMap<String, ProtocolSerdeRoles>,
    shape_id: &str,
    role: ProtocolSerdeRole,
) -> bool {
    roles.get(shape_id).is_some_and(|roles| match role {
        ProtocolSerdeRole::Serialize => roles.serialize,
        ProtocolSerdeRole::Deserialize => roles.deserialize,
    })
}

fn protocol_first_role_dependencies(
    selected: &SelectedModel,
    shape_id: &str,
    member: &Value,
    role: ProtocolSerdeRole,
    roles: &BTreeMap<String, ProtocolSerdeRoles>,
    seen: &mut BTreeSet<String>,
    output: &mut Vec<(String, ProtocolSerdeRole)>,
) {
    protocol_first_role_dependencies_mode(
        selected, shape_id, member, role, roles, seen, output, false,
    );
}

fn protocol_first_role_dependencies_preserving_intermediates(
    selected: &SelectedModel,
    shape_id: &str,
    member: &Value,
    role: ProtocolSerdeRole,
    roles: &BTreeMap<String, ProtocolSerdeRoles>,
    seen: &mut BTreeSet<String>,
    output: &mut Vec<(String, ProtocolSerdeRole)>,
) {
    protocol_first_role_dependencies_mode(
        selected, shape_id, member, role, roles, seen, output, true,
    );
}

#[allow(clippy::too_many_arguments)]
fn protocol_first_role_dependencies_mode(
    selected: &SelectedModel,
    shape_id: &str,
    member: &Value,
    role: ProtocolSerdeRole,
    roles: &BTreeMap<String, ProtocolSerdeRoles>,
    seen: &mut BTreeSet<String>,
    output: &mut Vec<(String, ProtocolSerdeRole)>,
    preserve_unrendered: bool,
) {
    let mut shape_id = shape_id;
    if matches!(role, ProtocolSerdeRole::Deserialize)
        && protocol_shape_kind(selected, shape_id) == "list"
        && has_trait(member, "smithy.api#xmlFlattened")
    {
        let Some(element) = selected
            .model
            .shapes
            .get(shape_id)
            .and_then(|shape| shape.get("member"))
            .and_then(member_target)
        else {
            return;
        };
        shape_id = element;
    }
    if !seen.insert(shape_id.to_owned()) {
        return;
    }
    if protocol_role_enabled(roles, shape_id, role) {
        output.push((shape_id.to_owned(), role));
        return;
    }
    let Some(shape) = selected.model.shapes.get(shape_id) else {
        return;
    };
    if let Some((_, payload)) = event_payload_member(shape)
        && !matches!(
            protocol_shape_kind(selected, member_target(payload).unwrap_or_default()),
            "structure" | "union"
        )
    {
        return;
    }
    if preserve_unrendered
        && matches!(
            protocol_shape_kind(selected, shape_id),
            "structure" | "union" | "list" | "map"
        )
    {
        output.push((shape_id.to_owned(), role));
        return;
    }
    for (_, child) in protocol_shape_members(selected, shape) {
        let Some(target) = member_target(child) else {
            continue;
        };
        protocol_first_role_dependencies_mode(
            selected,
            target,
            child,
            role,
            roles,
            seen,
            output,
            preserve_unrendered,
        );
    }
}

fn protocol_shape_members<'a>(
    selected: &'a SelectedModel,
    shape: &'a Value,
) -> Vec<(String, &'a Value)> {
    match shape.get("type").and_then(Value::as_str) {
        Some("structure" | "union") => members(shape),
        Some("list") => shape
            .get("member")
            .map(|member| vec![("member".to_owned(), member)])
            .unwrap_or_default(),
        Some("map") => ["key", "value"]
            .into_iter()
            .filter_map(|name| shape.get(name).map(|member| (name.to_owned(), member)))
            .collect(),
        _ => {
            let _ = selected;
            Vec::new()
        }
    }
}

fn protocol_role_dependencies(
    selected: &SelectedModel,
    shape_id: &str,
    role: ProtocolSerdeRole,
    roles: &BTreeMap<String, ProtocolSerdeRoles>,
    output: &mut Vec<(String, ProtocolSerdeRole)>,
) {
    let Some(shape) = selected.model.shapes.get(shape_id) else {
        return;
    };
    for (_, member) in protocol_shape_members(selected, shape) {
        let Some(target) = member_target(member) else {
            continue;
        };
        protocol_first_role_dependencies(
            selected,
            target,
            member,
            role,
            roles,
            &mut BTreeSet::new(),
            output,
        );
    }
}

fn protocol_serde_roles(
    selected: &SelectedModel,
    query_mode: bool,
) -> BTreeMap<String, ProtocolSerdeRoles> {
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
                if query_mode || is_xml_document_member(member) {
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
    query_mode: bool,
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
    let render_serializer = |output: &mut String, state: &mut ProtocolRenderState| {
        if query_mode {
            match kind {
                "structure" => render_query_protocol_structure_serializer(
                    output, selected, shape_id, shape, state,
                ),
                "union" => {
                    render_query_protocol_union_serializer(output, selected, shape_id, shape, state)
                }
                _ => {}
            }
        } else {
            match kind {
                "structure" => {
                    render_protocol_structure_serializer(output, selected, shape_id, shape, state)
                }
                "union" => render_protocol_union_serializer(output, selected, shape_id, shape),
                _ => {}
            }
        }
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
    fn scope(&mut self) -> String {
        self.next_name += 1;
        format!("scope_{}", self.next_name)
    }

    fn temp(&mut self) -> String {
        self.next_name += 1;
        format!("var_{}", self.next_name)
    }

    fn list_item(&mut self) -> String {
        self.next_name += 1;
        format!("list_item_{}", self.next_name)
    }

    fn query_list_item(&mut self) -> String {
        self.next_name += 1;
        format!("item_{}", self.next_name)
    }

    fn key(&mut self) -> String {
        self.next_name += 1;
        format!("key_{}", self.next_name)
    }

    fn value(&mut self) -> String {
        self.next_name += 1;
        format!("value_{}", self.next_name)
    }

    fn map(&mut self) -> String {
        self.next_name += 1;
        format!("map_{}", self.next_name)
    }

    fn entry(&mut self) -> String {
        self.next_name += 1;
        format!("entry_{}", self.next_name)
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
    type_expr(selected, target, Context::Types {})
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

fn is_json_document_member(member: &Value) -> bool {
    is_xml_document_member(member) && !has_trait(member, "smithy.api#httpPayload")
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

fn json_timestamp_format(selected: &SelectedModel, target: &str) -> &'static str {
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
        _ => "EpochSeconds",
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
            let field_method = rust_method_name(&member_name);
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
                "                let {outer} =\n                    Some(\n                        {parse}\n                        ?\n                    )\n                ;\n                builder = builder.set_{field_method}({outer});"
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
    let key_xml_name = protocol_member_xml_name(selected, "key", key);
    let value_xml_name = protocol_member_xml_name(selected, "value", value);
    let key_parse = indent_expression(
        &protocol_parse_expression(selected, key_target, "tag", "depth"),
        20,
    );
    let value_parse = indent_expression(
        &protocol_parse_expression(selected, value_target, "tag", "depth"),
        20,
    );
    writeln!(
        output,
        "pub fn de_{name}(\n    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,\n    depth: u32,\n) -> ::std::result::Result<::std::collections::HashMap<{key_type}, {value_type}>, ::aws_smithy_xml::decode::XmlDecodeError> {{\n    if depth >= 128u32 {{\n        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(\"maximum nesting depth exceeded\"));\n    }}\n    let mut out = ::std::collections::HashMap::new();\n    while let Some(mut tag) = decoder.next_tag() {{\n        match tag.start_el() {{\n            s if s.matches(\"entry\") => {{\n                crate::protocol_serde::shape_{name}::de_{name}_entry(&mut tag, &mut out, depth)?;\n            }}\n            _ => {{}}\n        }}\n    }}\n    Ok(out)\n}}\n\npub fn de_{name}_entry(\n    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,\n    out: &mut ::std::collections::HashMap<{key_type}, {value_type}>,\n    depth: u32,\n) -> ::std::result::Result<(), ::aws_smithy_xml::decode::XmlDecodeError> {{\n    if depth >= 128u32 {{\n        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(\"maximum nesting depth exceeded\"));\n    }}\n    let mut k: Option<{key_type}> = None;\n    let mut v: Option<{value_type}> = None;\n    while let Some(mut tag) = decoder.next_tag() {{\n        match tag.start_el() {{\n            s if s.matches({key_xml_name:?}) /* key {shape_id}$key */ =>  {{\n                k = Some(\n                    {}\n                    ?\n                )\n            }}\n            ,\n            s if s.matches({value_xml_name:?}) /* value {shape_id}$value */ =>  {{\n                v = Some(\n                    {}\n                    ?\n                )\n            }}\n            ,\n            _ => {{}}\n        }}\n    }}\n    let k = k.ok_or_else(|| ::aws_smithy_xml::decode::XmlDecodeError::custom(\"missing key map entry\"))?;\n    let v = v.ok_or_else(|| ::aws_smithy_xml::decode::XmlDecodeError::custom(\"missing value map entry\"))?;\n    out.insert(k, v);\n    Ok(())\n}}\n\n",
        key_parse,
        value_parse,
    )
    .unwrap();
}

fn render_protocol_error_file(
    selected: &SelectedModel,
    shape_id: &str,
    query_mode: bool,
) -> String {
    let shape = selected.model.shapes.get(shape_id).expect("error shape");
    let name = rust_type_name(terminal(shape_id));
    let mut output = String::new();
    client_operation_header(&mut output);
    let error_scope = if query_mode {
        "crate::rest_xml_wrapped_errors::error_scope"
    } else {
        "crate::rest_xml_unwrapped_errors::error_scope"
    };
    writeln!(
        output,
        "#[allow(unused_mut)]\npub fn de_{}_xml_err(\n    inp: &[u8],\n    mut builder: crate::types::error::builders::{name}Builder,\n) -> std::result::Result<crate::types::error::builders::{name}Builder, ::aws_smithy_xml::decode::XmlDecodeError> {{\n    if inp.is_empty() {{\n        return Ok(builder);\n    }}\n    let mut document = ::aws_smithy_xml::decode::Document::try_from(inp)?;\n    #[allow(unused_mut)]\n    let mut error_decoder = {error_scope}(&mut document)?;\n    #[allow(unused_variables)]\n    let depth = 0u32;\n    while let Some(mut tag) = error_decoder.next_tag() {{\n        match tag.start_el() {{",
        names::rust_module_name(terminal(shape_id))
    )
    .unwrap();
    let mut state = ProtocolRenderState::default();
    let error_members = if query_mode {
        let mut members = members(shape);
        let capacity = java_hash_map_capacity(members.len());
        members.sort_by_key(|(name, _)| java_string_hash_raw(name) & (capacity as u32 - 1));
        members
    } else {
        let mut ordered = Vec::new();
        if let Some((member_name, member)) = error_message_member(shape) {
            ordered.push((member_name, member));
        }
        ordered.extend(
            members(shape)
                .into_iter()
                .filter(|(member_name, _)| !member_name.eq_ignore_ascii_case("message")),
        );
        ordered
    };
    for (member_name, member) in error_members {
        let target = member_target(member).unwrap_or_default();
        let field_method = rust_method_name(&member_name);
        let xml_name = protocol_member_xml_name(selected, &member_name, member);
        let var = state.temp();
        let comment = shape_id.to_owned() + "$" + member_name.as_str();
        let parse = indent_expression(&protocol_parse_primitive(selected, target, "tag"), 24);
        writeln!(
            output,
            "            s if s.matches({xml_name:?}) /* {member_name} {comment} */ =>  {{\n                let {var} =\n                    Some(\n                        {}\n                        ?\n                    )\n                ;\n                builder = builder.set_{field_method}({var});\n            }}\n            ,",
            parse
        )
        .unwrap();
    }
    output.push_str("            _ => {}\n        }\n    }\n    Ok(builder)\n}\n");
    output
}

fn xml_namespace(selected: &SelectedModel) -> (String, Option<String>) {
    selected
        .model
        .shapes
        .get(selected.model.service_shape_id.as_str())
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
) {
    let module = names::snake_case(operation_name);
    let rust_operation = rust_type_name(operation_name);
    let operation_symbol = operation_error_type_name(operation_name);
    let output_path = protocol_operation_type_path(&module, &rust_operation, "Output");
    let error_path = protocol_operation_type_path(&module, &operation_symbol, "Error");
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
        render_protocol_error_arm(output, selected, &error_path, error);
    }
    writeln!(
        output,
        "        _ => {error_path}::generic(generic),\n    }})\n}}\n"
    )
    .unwrap();
}

fn render_protocol_error_arm(
    output: &mut String,
    selected: &SelectedModel,
    error_path: &str,
    error: &str,
) {
    let error_name = rust_type_name(terminal(error));
    let error_module = names::snake_case(terminal(error));
    let error_code = protocol_error_code(selected, error);
    writeln!(
        output,
        "        {error_code:?} => {error_path}::{error_name}({{",
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
    output.push_str("                let output = output.meta(generic);\n");
    if let Some(shape) = selected.model.shapes.get(error)
        && serde_util_shape_needs_correction(shape)
    {
        let correction = format!(
            "crate::serde_util::{}_correct_errors(output)",
            names::rust_module_name(terminal(error))
        );
        if serde_util_builder_is_fallible(selected, shape) {
            writeln!(
                output,
                "                {correction}.build().map_err({error_path}::unhandled)?"
            )
            .unwrap();
        } else {
            writeln!(output, "                {correction}.build()").unwrap();
        }
    } else {
        output.push_str("                output.build()\n");
    }
    output.push_str("            };");
    output.push_str("            if tmp.message.is_none() {\n                tmp.message = _error_message;\n            }\n            tmp\n        }),\n");
}

/// Resolve an error's wire code using the protocol trait, matching Smithy's
/// `HttpBindingResolver.errorCode` behavior for AWS Query. Other protocols
/// continue to use the modeled shape name.
fn protocol_error_code(selected: &SelectedModel, error: &str) -> String {
    selected
        .model
        .shapes
        .get(error)
        .and_then(|shape| shape.get("traits"))
        .and_then(Value::as_object)
        .and_then(|traits| traits.get("aws.protocols#awsQueryError"))
        .and_then(Value::as_object)
        .and_then(|trait_value| trait_value.get("code"))
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| terminal(error).to_owned())
}

fn protocol_operation_type_path(module: &str, operation: &str, suffix: &str) -> String {
    format!("crate::operation::{module}::{operation}{suffix}")
}

fn render_protocol_http_response(
    output: &mut String,
    selected: &SelectedModel,
    operation_name: &str,
    output_shape: Option<&Value>,
) {
    let module = names::snake_case(operation_name);
    let rust_operation = rust_type_name(operation_name);
    let operation_symbol = operation_error_type_name(operation_name);
    let output_path = protocol_operation_type_path(&module, &rust_operation, "Output");
    let error_path = protocol_operation_type_path(&module, &operation_symbol, "Error");
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
    let builder_path =
        { format!("crate::operation::{module}::builders::{rust_operation}OutputBuilder") };
    writeln!(
        output,
        "        let mut output = {builder_path}::default();"
    )
    .unwrap();
    if streaming_payload.is_none()
        && protocol_operation_has_document_output(selected, operation_name)
    {
        let helper_path = format!("crate::protocol_serde::shape_{module}");
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
                let helper_path = format!("crate::protocol_serde::{helper_module}");
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
                let helper_path = format!("crate::protocol_serde::{helper_module}");
                let error_path = protocol_operation_type_path(&module, &rust_operation, "Error");
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
            let helper_path = format!("crate::protocol_serde::{helper_module}");
            let error_path = protocol_operation_type_path(&module, &rust_operation, "Error");
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
    let input_path = protocol_operation_type_path(&module, &rust_operation, "Input");
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

fn render_protocol_output_file(selected: &SelectedModel, operation_name: &str) -> Option<String> {
    let has_headers = protocol_output_has_headers(selected, operation_name);
    let payload = render_protocol_output_payload_file(selected, operation_name);
    if !has_headers {
        return payload;
    }
    let mut output = render_protocol_output_headers(selected, operation_name);
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

fn render_protocol_output_headers(selected: &SelectedModel, operation_name: &str) -> String {
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
        operation_error_type_name(operation_name)
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
    let return_type = type_expr(selected, target, Context::Types {});
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
    let value_type = type_expr(selected, value_target, Context::Types {});
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
    let value_type = type_expr(selected, value_target, Context::Types {});
    writeln!(
        output,
        "pub fn de_{field}_inner<'a>(\n    headers: impl ::std::iter::Iterator<Item = &'a str>,\n) -> std::result::Result<Option<{value_type}>, ::aws_smithy_http::header::ParseError> {{\n    ::aws_smithy_http::header::one_or_none(headers)\n}}\n"
    )
    .unwrap();
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
    if has_trait(member, "smithy.api#streaming")
        || is_streaming_shape_target(selected, target)
        || is_streaming_output_target(selected, target, context)
    {
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
    (member_is_required(member) || has_trait(member, "smithy.api#default"))
        && selected
            .model
            .shapes
            .get(target)
            .and_then(|shape| shape.get("type"))
            .and_then(Value::as_str)
            != Some("structure")
}

fn is_streaming_output_target(selected: &SelectedModel, target: &str, context: &Context) -> bool {
    !operation_input(context) && is_streaming_shape_target(selected, target)
}

fn is_streaming_shape_target(selected: &SelectedModel, target: &str) -> bool {
    is_streaming_target(target)
        || selected
            .model
            .shapes
            .get(target)
            .is_some_and(shape_is_streaming)
}

fn is_event_stream_target(selected: &SelectedModel, target: &str) -> bool {
    selected.model.shapes.get(target).is_some_and(|shape| {
        shape.get("type").and_then(Value::as_str) == Some("union") && shape_is_streaming(shape)
    })
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

fn operation_output_is_sensitive(selected: &SelectedModel, shape: &Value) -> bool {
    has_trait(shape, "smithy.api#sensitive")
        || members(shape)
            .iter()
            .any(|(_, member)| value_has_sensitive_target(selected, member, &mut BTreeSet::new()))
}

fn value_has_sensitive_target(
    selected: &SelectedModel,
    value: &Value,
    seen: &mut BTreeSet<String>,
) -> bool {
    has_trait(value, "smithy.api#sensitive")
        || member_target(value)
            .is_some_and(|target| shape_has_sensitive_target(selected, target, seen))
}

fn shape_has_sensitive_target(
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
        Some("structure" | "union") => members(shape)
            .iter()
            .any(|(_, member)| value_has_sensitive_target(selected, member, seen)),
        Some("list") => shape
            .get("member")
            .is_some_and(|member| value_has_sensitive_target(selected, member, seen)),
        Some("map") => {
            shape
                .get("key")
                .is_some_and(|key| value_has_sensitive_target(selected, key, seen))
                || shape
                    .get("value")
                    .is_some_and(|value| value_has_sensitive_target(selected, value, seen))
        }
        _ => false,
    }
}

fn structure_has_streaming_member(selected: &SelectedModel, shape: &Value) -> bool {
    members(shape).iter().any(|(_, member)| {
        has_trait(member, "smithy.api#streaming")
            || member_target(member)
                .is_some_and(|target| is_streaming_shape_target(selected, target))
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
    !operation_input(context)
        && (is_event_stream_target(selected, target)
            || (member_is_required(member)
                && !has_trait(member, "smithy.api#default")
                && member_is_effectively_required(selected, member, target)))
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
    match context {
        Context::Types { .. } => format!("crate::types::{name}"),
        Context::Error { .. } => format!("crate::types::error::{name}"),
        Context::Operation { module, .. } | Context::Builder { module, .. } => {
            format!("crate::operation::{module}::{name}")
        }
    }
}

fn build_error_type(context: &Context) -> String {
    let _ = context;
    "::aws_smithy_types::error::operation::BuildError".to_owned()
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
        render_error_impls(output, selected, shape, name);
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
    render_deprecated_attribute(output, shape, indent);
    writeln!(output, "{padding}#[non_exhaustive]").unwrap();
    let derives = "::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug";
    let mut excluded_derives = Vec::new();
    if structure_has_streaming_member(selected, shape) {
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
            let required = has_trait(member, "smithy.api#streaming")
                || is_streaming_shape_target(selected, target)
                || is_streaming_target(target)
                || is_streaming_output_target(selected, target, &context)
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
        let (message_name, message_required) = error_message_member(shape)
            .map(|(name, member)| {
                let target = member_target(member).unwrap_or("smithy.api#String");
                (
                    names::rust_identifier(&name),
                    member_is_effectively_required(selected, member, target),
                )
            })
            .unwrap_or_else(|| ("message".to_owned(), false));
        writeln!(output, "{padding}impl {} {{", rust_type_name(name)).unwrap();
        writeln!(output, "{padding}    /// Returns the error message.").unwrap();
        if message_required {
            writeln!(
                output,
                "{padding}    pub fn message(&self) -> &str {{\n{padding}        &self.{message_name}\n{padding}    }}"
            )
            .unwrap();
        } else {
            writeln!(
                output,
                "{padding}    pub fn message(&self) -> ::std::option::Option<&str> {{\n{padding}        self.{message_name}.as_deref()\n{padding}    }}"
            )
            .unwrap();
        }
        writeln!(output, "{padding}}}").unwrap();
    }
    if structure_has_sensitive_member(selected, shape) {
        render_sensitive_debug_impl(output, selected, shape, name, &context, indent);
    }
    if request_id_plan.extended && !is_error {
        let trait_path = "crate::s3_request_id::RequestIdExt";
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

fn render_error_impls(output: &mut String, selected: &SelectedModel, shape: &Value, name: &str) {
    let rust_name = rust_type_name(name);
    let request_id_plan = request_id_plan(selected);
    let error_type_path = { format!("crate::types::error::{rust_name}") };
    let message_name = error_message_member(shape)
        .map(|(name, _)| names::rust_identifier(&name))
        .unwrap_or_else(|| "message".to_owned());
    let message_required = error_message_member(shape).is_some_and(|(_, member)| {
        let target = member_target(member).unwrap_or("smithy.api#String");
        member_is_effectively_required(selected, member, target)
    });
    let display_name = if rust_name != terminal(name) {
        format!("{rust_name} [{}]", terminal(name))
    } else {
        rust_name.clone()
    };

    writeln!(output, "impl ::std::fmt::Display for {rust_name} {{").unwrap();
    writeln!(
        output,
        "    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{"
    )
    .unwrap();
    writeln!(output, "        ::std::write!(f, {display_name:?})?;").unwrap();
    if message_required {
        writeln!(
            output,
            "        {{\n            ::std::write!(f, \": {{}}\", &self.{message_name})?;\n        }}"
        )
        .unwrap();
    } else {
        writeln!(
            output,
            "        if let ::std::option::Option::Some(inner_1) = &self.{message_name} {{\n            {{\n                ::std::write!(f, \": {{inner_1}}\")?;\n            }}\n        }}"
        )
        .unwrap();
    }
    writeln!(output, "        Ok(())").unwrap();
    writeln!(output, "    }}").unwrap();
    writeln!(output, "}}").unwrap();
    writeln!(output, "impl ::std::error::Error for {rust_name} {{}}").unwrap();

    if request_id_plan.extended {
        let trait_path = "crate::s3_request_id::RequestIdExt";
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
    if structure_has_streaming_member(selected, shape) {
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
                let field_link = format!("{builder_path}::{doc_field_method}");
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
        if has_trait(member, "smithy.api#default") {
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
        } else if has_trait(member, "smithy.api#streaming")
            || is_streaming_shape_target(selected, target)
        {
            writeln!(
                output,
                "{inner}        {field}: self.{field}.unwrap_or_default(),"
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
    let union_path = format!("crate::types::{rust_name}");
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

fn render_enum(output: &mut String, shape: &Value, name: &str) {
    let rust_name = rust_type_name(name);
    let ordered_members = sorted_members(shape);
    let support_prefix = { "crate::" };
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
                writeln!(output, "    /// {}", line.trim_start()).unwrap();
            }
        }
    } else {
        output.push_str("    #[allow(missing_docs)] // documentation missing in model\n");
    }
    render_deprecated_attribute(output, shape, 4);
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

fn render_client_file(selected: &SelectedModel) -> String {
    render_standalone_client_file(selected)
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

fn render_standalone_client_file(selected: &SelectedModel) -> String {
    let mut output = String::new();
    client_operation_header(&mut output);
    output.push_str(
        "#[derive(Debug)]\n\
         pub(crate) struct Handle {\n\
             pub(crate) conf: crate::Config,\n\
             #[allow(dead_code)] // unused when a service does not provide any operations\n\
             pub(crate) runtime_plugins: ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,\n\
         }\n\n",
    );

    let service_title = service_title(selected);
    let module_name = selected.model.entry.module_name;
    writeln!(output, "/// Client for {service_title}").unwrap();
    writeln!(
        output,
        "///\n/// Client for invoking operations on {service_title}. Each operation on {service_title} is a method on this"
    )
    .unwrap();
    output.push_str(
        "/// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.\n",
    );
    output.push_str(
        "/// ## Constructing a `Client`\n///\n/// A [`Config`] is required to construct a client. For most use cases, the [`aws-config`]\n/// crate should be used to automatically resolve this config using\n/// [`aws_config::load_from_env()`], since this will resolve an [`SdkConfig`] which can be shared\n/// across multiple different AWS SDK clients. This config resolution process can be customized\n/// by calling [`aws_config::from_env()`] instead, which returns a [`ConfigLoader`] that uses\n/// the [builder pattern] to customize the default config.\n///\n/// In the simplest case, creating a client looks as follows:\n/// ```rust,no_run\n/// # async fn wrapper() {\n",
    );
    writeln!(
        output,
        "/// let config = aws_config::load_from_env().await;\n/// let client = {module_name}::Client::new(&config);"
    )
    .unwrap();
    output.push_str(
        "/// # }\n/// ```\n///\n/// Occasionally, SDKs may have additional service-specific values that can be set on the [`Config`] that\n/// is absent from [`SdkConfig`], or slightly different settings for a specific client may be desired.\n/// The [`Builder`](crate::config::Builder) struct implements `From<&SdkConfig>`, so setting these specific settings can be\n/// done as follows:\n///\n/// ```rust,no_run\n/// # async fn wrapper() {\n",
    );
    writeln!(
        output,
        "/// let sdk_config = ::aws_config::load_from_env().await;\n/// let config = {module_name}::config::Builder::from(&sdk_config)"
    )
    .unwrap();
    output.push_str(
        "/// # /*\n///     .some_service_specific_setting(\"value\")\n/// # */\n///     .build();\n/// # }\n/// ```\n///\n/// See the [`aws-config` docs] and [`Config`] for more information on customizing configuration.\n///\n/// _Note:_ Client construction is expensive due to connection thread pool initialization, and should\n/// be done once at application start-up.\n///\n/// [`Config`]: crate::Config\n/// [`ConfigLoader`]: https://docs.rs/aws-config/*/aws_config/struct.ConfigLoader.html\n/// [`SdkConfig`]: https://docs.rs/aws-config/*/aws_config/struct.SdkConfig.html\n/// [`aws-config` docs]: https://docs.rs/aws-config/*\n/// [`aws-config`]: https://crates.io/crates/aws-config\n/// [`aws_config::from_env()`]: https://docs.rs/aws-config/*/aws_config/fn.from_env.html\n/// [`aws_config::load_from_env()`]: https://docs.rs/aws-config/*/aws_config/fn.load_from_env.html\n/// [builder pattern]: https://rust-lang.github.io/api-guidelines/type-safety.html#builders-enable-construction-of-complex-values-c-builder\n",
    );

    if let Some((operation_name, member_name)) = client_usage_example(selected) {
        let module = names::snake_case(&operation_name);
        let field = names::rust_identifier(&member_name);
        let field = field.strip_prefix("r#").unwrap_or(&field);
        writeln!(
            output,
            "/// # Using the `Client`\n///\n/// A client has a function for every operation that can be performed by the service.\n/// For example, the [`{operation_name}`](crate::operation::{module}) operation has\n/// a [`Client::{module}`], function which returns a builder for that operation.\n/// The fluent builder ultimately has a `send()` function that returns an async future that\n/// returns a result, as illustrated below:\n///\n/// ```rust,ignore\n/// let result = client.{module}()\n///     .{field}(\"example\")\n///     .send()\n///     .await;\n/// ```\n///\n/// The underlying HTTP requests that get made by this can be modified with the `customize_operation`\n/// function on the fluent builder. See the [`customize`](crate::client::customize) module for more\n/// information."
        )
        .unwrap();
    }
    if has_waiters(selected) {
        output.push_str(
            "/// # Waiters\n///\n/// This client provides `wait_until` methods behind the [`Waiters`](crate::client::Waiters) trait.\n/// To use them, simply import the trait, and then call one of the `wait_until` methods. This will\n/// return a waiter fluent builder that takes various parameters, which are documented on the builder\n/// type. Once parameters have been provided, the `wait` method can be called to initiate waiting.\n///\n/// For example, if there was a `wait_until_thing` method, it could look like:\n/// ```rust,ignore\n/// let result = client.wait_until_thing()\n///     .thing_id(\"someId\")\n///     .wait(Duration::from_secs(120))\n///     .await;\n/// ```\n",
        );
    }

    output.push_str("#[derive(::std::clone::Clone, ::std::fmt::Debug)]\npub struct Client {\n    handle: ::std::sync::Arc<Handle>,\n}\n\n");
    output.push_str(
        "impl Client {\n    /// Creates a new client from the service [`Config`](crate::Config).\n    ///\n    /// # Panics\n    ///\n    /// This method will panic in the following cases:\n    ///\n    /// - Retries or timeouts are enabled without a `sleep_impl` configured.\n    /// - Identity caching is enabled without a `sleep_impl` and `time_source` configured.\n    /// - No `behavior_version` is provided.\n    ///\n    /// The panic message for each of these will have instructions on how to resolve them.\n    #[track_caller]\n    pub fn from_conf(conf: crate::Config) -> Self {\n        let handle = Handle {\n            conf: conf.clone(),\n            runtime_plugins: crate::config::base_client_runtime_plugins(conf),\n        };\n        if let Err(err) = Self::validate_config(&handle) {\n            panic!(\"Invalid client configuration: {err}\");\n        }\n        Self {\n            handle: ::std::sync::Arc::new(handle),\n        }\n    }\n\n    /// Returns the client's configuration.\n    pub fn config(&self) -> &crate::Config {\n        &self.handle.conf\n    }\n\n    fn validate_config(handle: &Handle) -> ::std::result::Result<(), ::aws_smithy_runtime_api::box_error::BoxError> {\n        let mut cfg = ::aws_smithy_types::config_bag::ConfigBag::base();\n        handle\n            .runtime_plugins\n            .apply_client_configuration(&mut cfg)?\n            .validate_base_client_config(&cfg)?;\n        Ok(())\n    }\n}\n",
    );

    if has_waiters(selected) {
        output.push_str(
            "\n///\n/// Waiter functions for the client.\n///\n/// Import this trait to get `wait_until` methods on the client.\n///\npub trait Waiters {\n",
        );
        for (_, waiter_name, waiter) in waiter_specs(selected) {
            let waiter_type = rust_type_name(&waiter_name);
            if let Some(documentation) = waiter.get("documentation").and_then(Value::as_str) {
                writeln!(
                    output,
                    "    /// {documentation}\n    fn wait_until_{waiter_name}(&self) -> crate::waiters::{waiter_name}::{waiter_type}FluentBuilder;"
                )
                .unwrap();
            } else {
                writeln!(
                    output,
                    "    /// Wait for `{waiter_name}`\n    fn wait_until_{waiter_name}(&self) -> crate::waiters::{waiter_name}::{waiter_type}FluentBuilder;"
                )
                .unwrap();
            }
        }
        output.push_str("}\nimpl Waiters for Client {\n");
        for (_, waiter_name, _) in waiter_specs(selected) {
            let waiter_type = rust_type_name(&waiter_name);
            writeln!(
                output,
                "    fn wait_until_{waiter_name}(&self) -> crate::waiters::{waiter_name}::{waiter_type}FluentBuilder {{\n        crate::waiters::{waiter_name}::{waiter_type}FluentBuilder::new(self.handle.clone())\n    }}"
            )
            .unwrap();
        }
        output.push_str("}\n");
    }

    output.push_str(
        "\nimpl Client {\n    /// Creates a new client from an [SDK Config](::aws_types::sdk_config::SdkConfig).\n    ///\n    /// # Panics\n    ///\n    /// - This method will panic if the `sdk_config` is missing an async sleep implementation. If you experience this panic, set\n    ///   the `sleep_impl` on the Config passed into this function to fix it.\n    /// - This method will panic if the `sdk_config` is missing an HTTP connector. If you experience this panic, set the\n    ///   `http_connector` on the Config passed into this function to fix it.\n    /// - This method will panic if no `BehaviorVersion` is provided. If you experience this panic, set `behavior_version` on the Config or enable the `behavior-version-latest` Cargo feature.\n    #[track_caller]\n    pub fn new(sdk_config: &::aws_types::sdk_config::SdkConfig) -> Self {\n        Self::from_conf(sdk_config.into())\n    }\n}\n",
    );

    output.push('\n');
    let mut operations = selected.operations.clone();
    operations.sort_by_key(|operation| names::snake_case(operation));
    let mut customize_written = false;
    for operation_name in operations {
        let module = names::snake_case(&operation_name);
        if !customize_written && "customize" < module.as_str() {
            render_standalone_customize_module(&mut output, selected);
            customize_written = true;
        }
        writeln!(output, "mod {module};\n").unwrap();
    }
    if !customize_written {
        render_standalone_customize_module(&mut output, selected);
    }
    output
}

fn render_standalone_customize_module(output: &mut String, selected: &SelectedModel) {
    let mut operations = selected.operations.clone();
    operations.sort();
    let operation = operations
        .first()
        .map(String::as_str)
        .unwrap_or("operation");
    let module = names::snake_case(operation);
    let crate_name = selected.model.entry.module_name;
    output.push_str(
        "\n/// Operation customization and supporting types.\n///\n/// The underlying HTTP requests made during an operation can be customized\n/// by calling the `customize()` method on the builder returned from a client\n/// operation call. For example, this can be used to add an additional HTTP header:\n///\n/// ```ignore\n",
    );
    writeln!(
        output,
        "/// # async fn wrapper() -> ::std::result::Result<(), {crate_name}::Error> {{\n/// # let client: {crate_name}::Client = unimplemented!();"
    )
    .unwrap();
    output.push_str(
        "/// use ::http_1x::header::{HeaderName, HeaderValue};\n///\n/// let result = client.",
    );
    writeln!(output, "{module}()").unwrap();
    output.push_str(
        "///     .customize()\n///     .mutate_request(|req| {\n///         // Add `x-example-header` with value\n///         req.headers_mut()\n///             .insert(\n///                 HeaderName::from_static(\"x-example-header\"),\n///                 HeaderValue::from_static(\"1\"),\n///             );\n///     })\n///     .send()\n///     .await;\n/// # }\n/// ```\npub mod customize;\n\n",
    );
}

fn client_usage_example(selected: &SelectedModel) -> Option<(String, String)> {
    let mut operations = selected.operations.clone();
    operations.sort();
    operations.into_iter().find_map(|operation_name| {
        let operation = operation_shape(selected, &operation_name)?;
        let input_id = operation.get("input").and_then(target_value)?;
        let input = selected.model.shapes.get(input_id)?;
        members(input)
            .into_iter()
            .find_map(|(member_name, member)| {
                let target = member_target(member)?;
                let shape = selected.model.shapes.get(target);
                let string_shape = is_string_type(target, shape)
                    || shape
                        .and_then(|shape| shape.get("type"))
                        .and_then(Value::as_str)
                        == Some("enum");
                string_shape.then_some((operation_name.clone(), member_name))
            })
    })
}

fn render_client_operation_file(selected: &SelectedModel, operation: &str) -> String {
    let module = names::snake_case(operation);
    let rust_operation = rust_type_name(operation);
    let operation_symbol = operation;
    let mut output = String::new();
    client_operation_header(&mut output);
    {
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
                let setter_type = if is_streaming_shape_target(selected, target_id) {
                    target.clone()
                } else {
                    format!("Option<{target}>")
                };
                writeln!(
                    output,
                    "    ///   - [`{field_method}({argument})`](crate::operation::{module}::builders::{operation_symbol}FluentBuilder::{field_method}) / [`set_{field_method}({setter_type})`](crate::operation::{module}::builders::{operation_symbol}FluentBuilder::set_{field_method}):<br>required: **{required}**<br>{documentation}<br>"
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
                    let field_type = if is_streaming_shape_target(selected, target_id)
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
        render_deprecated_attribute(&mut output, operation_shape, 4);
        let fluent_builder_path =
            format!("crate::operation::{module}::builders::{operation_symbol}FluentBuilder");
        writeln!(
            output,
            "    pub fn {module}(&self) -> {fluent_builder_path}{{"
        )
        .unwrap();
        writeln!(
            output,
            "        crate::operation::{module}::builders::{operation_symbol}FluentBuilder::new(self.handle.clone())"
        )
        .unwrap();
        output.push_str("    }\n}\n");
        output
    }
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
        _ if is_streaming_shape_target(selected, target)
            && !is_event_stream_target(selected, target) =>
        {
            "ByteStream".to_owned()
        }
        _ => {
            let Some(shape) = selected.model.shapes.get(target) else {
                return rust_type_name(terminal(target));
            };
            if is_event_stream_target(selected, target) {
                let name = rust_type_name(terminal(target));
                return format!("EventReceiver<{name}, {name}Error>");
            }
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
                    let Some(opening_index) = stack.iter().rposition(|current| current == &name)
                    else {
                        pending_whitespace = false;
                        pending_newline = false;
                        continue;
                    };
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
                        || (pending_newline
                            && documentation_custom_tag(&name)
                            && previous_tag
                                .as_ref()
                                .is_some_and(|(closing, previous)| !closing && previous == "dd"))
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
                let trimmed = collapse_documentation_whitespace(&text);
                if trimmed.is_empty() {
                    continue;
                }
                let has_leading_whitespace = text
                    .chars()
                    .next()
                    .is_some_and(is_collapsible_documentation_whitespace);
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
                pending_whitespace = text
                    .chars()
                    .last()
                    .is_some_and(is_collapsible_documentation_whitespace);
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
    let tag = compact_documentation_tag_whitespace(tag);
    let Some(name_start) = tag.find(|character: char| character.is_ascii_alphabetic()) else {
        return tag;
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
        return tag;
    }
    format!(
        "{}{}{}",
        &tag[..name_start],
        original.to_ascii_lowercase(),
        &tag[name_end..]
    )
}

fn compact_documentation_tag_whitespace(tag: &str) -> String {
    let mut output = String::with_capacity(tag.len());
    let mut quoted = None;
    let mut pending_whitespace = false;
    for character in tag.chars() {
        if let Some(quote) = quoted {
            output.push(character);
            if character == quote {
                quoted = None;
            }
        } else if matches!(character, '\'' | '"') {
            if pending_whitespace && !matches!(output.chars().last(), Some('<' | '/')) {
                output.push(' ');
            }
            pending_whitespace = false;
            output.push(character);
            quoted = Some(character);
        } else if character.is_whitespace() {
            pending_whitespace = true;
        } else {
            if pending_whitespace
                && !output.is_empty()
                && !matches!(output.chars().last(), Some('<' | '/'))
            {
                output.push(' ');
            }
            pending_whitespace = false;
            output.push(character);
        }
    }
    output
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

fn is_collapsible_documentation_whitespace(character: char) -> bool {
    character.is_whitespace() && character != '\u{a0}'
}

fn collapse_documentation_whitespace(value: &str) -> String {
    let mut output = String::new();
    let mut pending_whitespace = false;
    for character in value.chars() {
        if is_collapsible_documentation_whitespace(character) {
            pending_whitespace = true;
        } else {
            if pending_whitespace && !output.is_empty() {
                output.push(' ');
            }
            output.push(character);
            pending_whitespace = false;
        }
    }
    output
}

fn normalize_client_documentation(value: &str) -> String {
    let tokens = documentation_tokens(value);

    let mut output = String::new();
    let mut stack = Vec::<String>::new();
    let mut previous = None::<DocumentationToken>;
    let mut next_significant = vec![None; tokens.len()];
    let mut next = None;
    let mut whitespace_since_previous = false;
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
            whitespace_since_previous = true;
            continue;
        }
        if !whitespace_since_previous {
            output.push_str(&documentation_implicit_gap(
                token,
                &stack,
                previous.as_ref(),
            ));
        }
        match token {
            DocumentationToken::Tag(tag) => {
                if let Some(name) = documentation_tag_name(tag) {
                    if tag.starts_with("</")
                        && let Some(opening_index) =
                            stack.iter().rposition(|current| current == &name)
                    {
                        let mut auto_closed = false;
                        while stack.len() > opening_index + 1 {
                            let pseudo_depth = stack
                                .iter()
                                .filter(|name| !documentation_known_tag(name))
                                .count();
                            if auto_closed && pseudo_depth > 0 {
                                output.push_str(&" ".repeat(pseudo_depth + 4));
                            }
                            let unclosed = stack.pop().expect("opening tag exists");
                            output.push_str(&format!("</{unclosed}>"));
                            auto_closed = true;
                        }
                    }
                    output.push_str(&lowercase_documentation_tag(tag));
                    if tag.starts_with("</") {
                        if stack.last().is_some_and(|current| current == &name) {
                            stack.pop();
                        }
                    } else if !tag.ends_with("/>")
                        && !matches!(name.as_str(), "br" | "hr" | "img" | "meta" | "link")
                    {
                        stack.push(name);
                    }
                } else {
                    output.push_str(&lowercase_documentation_tag(tag));
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
        whitespace_since_previous = false;
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
    if documentation_pseudo_parent(stack) {
        if matches!(next, DocumentationToken::Text(_))
            && previous.starts_with("</")
            && documentation_is_inline(&previous_name)
        {
            return " ".to_owned();
        }
        return match next {
            DocumentationToken::Tag(tag) if tag.starts_with("</") => " ".repeat(stack.len()),
            _ => " ".repeat(stack.len() + 1),
        };
    }
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
                }) || (previous_name == "code"
                    && stack
                        .last()
                        .is_some_and(|name| documentation_is_inline(name)))
                {
                    if stack
                        .last()
                        .is_some_and(|name| documentation_custom_tag(name))
                    {
                        " ".repeat(stack.len().max(1))
                    } else {
                        " ".to_owned()
                    }
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
            if previous_name == "p"
                && matches!(next_name.as_str(), "ul" | "ol")
                && stack
                    .last()
                    .is_some_and(|name| documentation_custom_tag(name))
            {
                return " ".repeat(stack.len() + 1);
            }
            if stack.last().is_some_and(|name| name == "li")
                && matches!(next_name.as_str(), "p" | "ul" | "ol")
            {
                return " ".repeat(stack.len() + 1);
            }
            if stack
                .last()
                .is_some_and(|name| documentation_custom_tag(name))
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

fn documentation_implicit_gap(
    token: &DocumentationToken,
    stack: &[String],
    previous: Option<&DocumentationToken>,
) -> String {
    if !documentation_pseudo_parent(stack) {
        return String::new();
    }
    if matches!(token, DocumentationToken::Tag(tag) if tag.starts_with("</"))
        && previous.is_some_and(|previous| {
            matches!(previous, DocumentationToken::Tag(previous) if !previous.starts_with("</")
                && documentation_tag_name(previous)
                    .is_some_and(|name| !documentation_known_tag(&name)))
        })
    {
        return String::new();
    }
    if matches!(token, DocumentationToken::Text(_))
        && previous.is_some_and(|previous| {
            matches!(previous, DocumentationToken::Tag(tag) if tag.starts_with("</")
                && documentation_tag_name(tag).is_some_and(|name| documentation_is_inline(&name)))
        })
    {
        return String::new();
    }
    match token {
        DocumentationToken::Tag(tag) if tag.starts_with("</") => " ".repeat(stack.len()),
        _ => " ".repeat(stack.len() + 1),
    }
}

fn documentation_pseudo_parent(stack: &[String]) -> bool {
    stack
        .last()
        .is_some_and(|name| !documentation_known_tag(name))
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
            return escape_documentation_text(&escape_doc_brackets(text));
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
            if character.is_whitespace() && character != '\u{a0}' {
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
                output.push_str(&escape_documentation_text(&escape_doc_brackets(
                    &character.to_string(),
                )));
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
        if character.is_whitespace() && character != '\u{a0}' {
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
            output.push_str(&escape_documentation_text(&escape_doc_brackets(
                &character.to_string(),
            )));
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
            '\u{a0}' => output.push_str("&nbsp;"),
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
    Types {},
    Error {},
    Operation { module: String, input: bool },
    Builder { module: String, input: bool },
}

fn type_expr(selected: &SelectedModel, target: &str, context: Context) -> String {
    if !operation_input(&context) && is_event_stream_target(selected, target) {
        let name = rust_type_name(terminal(target));
        let (value_path, error_path) = match &context {
            Context::Types {} => (
                format!("self::{name}"),
                format!("super::error::{name}Error"),
            ),
            Context::Error {} => (
                format!("super::super::types::{name}"),
                format!("super::{name}Error"),
            ),
            Context::Operation { .. } | Context::Builder { .. } => (
                format!("super::super::super::types::{name}"),
                format!("super::super::super::types::error::{name}Error"),
            ),
        };
        return format!("crate::event_receiver::EventReceiver<{value_path}, {error_path}>");
    }
    if is_streaming_shape_target(selected, target) {
        return match context {
            Context::Types {}
            | Context::Error {}
            | Context::Operation { .. }
            | Context::Builder { .. } => "::aws_smithy_types::byte_stream::ByteStream".to_owned(),
        };
    }
    if target.starts_with("smithy.api#") {
        return primitive_type_for_namespace(target.rsplit('#').next().unwrap_or("string"));
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
                return format!("::std::collections::HashMap<{key}, {value}>");
            }
            _ => {}
        }
    }
    let name = terminal(target);
    let known = rust_type_name(name);
    match context {
        Context::Types { .. } if name == "StreamingBlob" => {
            "::aws_smithy_types::byte_stream::ByteStream".to_owned()
        }
        Context::Types { .. } => {
            format!("crate::types::{known}")
        }
        Context::Error { .. } => {
            format!("crate::types::{known}")
        }
        Context::Operation { module, .. } => {
            if name == "StreamingBlob" {
                "::aws_smithy_types::byte_stream::ByteStream".to_owned()
            } else if name.ends_with("Input") {
                format!("crate::operation::{module}::Input")
            } else if name.ends_with("Output") {
                format!("crate::operation::{module}::Output")
            } else {
                format!("crate::types::{known}")
            }
        }
        Context::Builder { module, .. } => {
            if name == "StreamingBlob" {
                "::aws_smithy_types::byte_stream::ByteStream".to_owned()
            } else if name.ends_with("Input") {
                format!("crate::operation::{module}::Input")
            } else if name.ends_with("Output") {
                format!("crate::operation::{module}::Output")
            } else {
                format!("crate::types::{known}")
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

fn primitive_type_for_namespace(name: &str) -> String {
    {
        match name {
            "Timestamp" | "timestamp" => return "::aws_smithy_types::DateTime".to_owned(),
            "Blob" | "blob" => return "::aws_smithy_types::Blob".to_owned(),
            _ => {}
        }
    }
    primitive_type(name)
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

/// Smithy operation symbols preserve the operation shape name, only
/// uppercasing its first character. Modeled operation input/output shapes use
/// the ordinary Rust type-name normalization above instead.
fn operation_error_type_name(value: &str) -> String {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return String::new();
    };
    first.to_uppercase().chain(chars).collect()
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
    use crate::{ServiceMetadata, ServiceSource};

    #[test]
    fn operation_symbols_preserve_acronyms_but_shapes_are_normalized() {
        assert_eq!(
            operation_error_type_name("CreateSMSSandboxPhoneNumber"),
            "CreateSMSSandboxPhoneNumber"
        );
        assert_eq!(
            rust_type_name("CreateSMSSandboxPhoneNumber"),
            "CreateSmsSandboxPhoneNumber"
        );
        assert_eq!(
            operation_error_type_name("ListSAMLProviders"),
            "ListSAMLProviders"
        );
        assert_eq!(
            operation_error_type_name("AssumeRoleWithSAML"),
            "AssumeRoleWithSAML"
        );
        assert_eq!(operation_error_type_name("CreateThing"), "CreateThing");
    }

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
    fn normalize_client_documentation_preserves_nested_note_list_gap() {
        let normalized = normalize_client_documentation(
            "<ul><li><p>Text</p><note><p>Nested text</p>\n    <ul><li><p>Nested item</p></li></ul></note></li></ul>",
        );
        assert_eq!(
            normalized,
            "<ul><li><p>Text</p><note><p>Nested text</p>    <ul><li><p>Nested item</p></li></ul></note></li></ul>"
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
    fn streaming_json_unions_are_left_to_event_stream_codegen() {
        let metadata = ServiceMetadata {
            key: "example",
            filename: "model.json",
            module_name: "aws_sdk_example",
            sdk_version: None,
        };
        let model = crate::model::Model::load(ServiceSource::new(
            metadata,
            br#"{
                "shapes": {
                    "example#Service": {
                        "type": "service",
                        "version": "2024-01-01",
                        "operations": ["example#Invoke"],
                        "traits": {"aws.protocols#restJson1": {}}
                    },
                    "example#Invoke": {
                        "type": "operation",
                        "input": {"target": "example#Input"},
                        "output": {"target": "smithy.api#Unit"}
                    },
                    "example#Input": {
                        "type": "structure",
                        "members": {
                            "events": {
                                "target": "example#InputEvents",
                                "traits": {"smithy.api#httpPayload": {}}
                            }
                        }
                    },
                    "example#InputEvents": {
                        "type": "union",
                        "members": {"chunk": {"target": "example#Chunk"}},
                        "traits": {"smithy.api#streaming": {}}
                    },
                    "example#Chunk": {
                        "type": "structure",
                        "members": {"data": {"target": "smithy.api#Blob"}}
                    }
                }
            }"#,
        ))
        .unwrap();
        let selected = model.select(&[], true).unwrap();
        let roles = json_protocol_serde_roles(&selected);

        assert!(
            !roles
                .get("example#InputEvents")
                .is_some_and(|roles| roles.serialize || roles.deserialize)
        );
        assert!(
            roles
                .get("example#Chunk")
                .is_some_and(|roles| roles.serialize)
        );
    }
}
