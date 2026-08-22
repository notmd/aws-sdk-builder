use serde_json::Value;
use std::{
    collections::{BTreeMap, BTreeSet},
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
                render_service_lib(entry.key, &selected),
            ),
            (
                "src/primitives.rs".to_owned(),
                render_primitives(&selected, consumer_namespace),
            ),
            ("src/config.rs".to_owned(), render_config_file()),
            ("src/error.rs".to_owned(), render_error_file()),
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
        ];
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
            service_files.push((
                format!("src/client/{module}.rs"),
                render_client_operation_file(&selected, &operation_name, consumer_namespace),
            ));
            if protocol == crate::model::ProtocolKind::RestXml {
                service_files.push((
                    format!("src/protocol_serde/shape_{module}.rs"),
                    render_protocol_operation_file(&selected, &operation_name, consumer_namespace),
                ));
                if let Some(payload_source) =
                    render_protocol_input_payload_file(&selected, &operation_name)
                {
                    service_files.push((
                        format!("src/protocol_serde/shape_{module}_input.rs"),
                        payload_source,
                    ));
                }
                if protocol_output_has_headers(&selected, &operation_name) {
                    service_files.push((
                        format!("src/protocol_serde/shape_{module}_output.rs"),
                        render_protocol_output_headers(
                            &selected,
                            &operation_name,
                            consumer_namespace,
                        ),
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

fn render_service_lib(service_key: &str, selected: &SelectedModel) -> String {
    let mut output = String::new();
    header(&mut output);
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
        writeln!(
            output,
            "include!(concat!(env!(\"OUT_DIR\"), \"/generated/{service_key}/src/{file}\"));"
        )
        .unwrap();
    }
    output
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

fn render_error_file() -> String {
    let mut output = String::new();
    render_error(&mut output);
    output
}

fn render_error(output: &mut String) {
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
    header(&mut output);
    output.push_str("pub mod types {\n");
    let mut ids = selected.model.shapes.keys().cloned().collect::<Vec<_>>();
    ids.sort();
    for id in ids {
        let Some(shape) = selected.model.shapes.get(&id) else {
            continue;
        };
        if id == selected.model.entry.service_shape_id
            || operation_shape_ids(selected).contains(&id)
            || (!is_file_renderable_type(Some(shape)) && !is_primitive_shape(shape))
            || is_error_shape(shape)
        {
            continue;
        }
        if is_primitive_shape(shape) {
            writeln!(
                output,
                "    pub type {} = {};",
                rust_type_name(terminal(&id)),
                primitive_type_for_namespace(
                    shape
                        .get("type")
                        .and_then(Value::as_str)
                        .unwrap_or("string"),
                    consumer_namespace,
                )
            )
            .unwrap();
            continue;
        }
        let filename = type_file_name(&id);
        writeln!(
            output,
            "    include!(concat!(env!(\"OUT_DIR\"), \"/generated/{service_key}/src/types/{filename}\"));"
        )
        .unwrap();
    }
    output.push_str("    pub mod error {\n");
    for id in error_shape_ids(selected) {
        let filename = type_file_name(&id);
        writeln!(
            output,
            "        include!(concat!(env!(\"OUT_DIR\"), \"/generated/{service_key}/src/types/error/{filename}\"));"
        )
        .unwrap();
    }
    output.push_str("    pub mod builders {\n");
    writeln!(
        output,
        "        include!(concat!(env!(\"OUT_DIR\"), \"/generated/{service_key}/src/types/error/builders.rs\"));"
    )
    .unwrap();
    output.push_str("    }\n");
    output.push_str("    }\n");
    output.push_str("}\n\n");
    output
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
                .map(ToOwned::to_owned)
                .collect::<Vec<_>>()
        })
        .collect()
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

fn is_primitive_shape(shape: &Value) -> bool {
    matches!(
        shape.get("type").and_then(Value::as_str),
        Some(
            "string"
                | "integer"
                | "long"
                | "short"
                | "byte"
                | "float"
                | "double"
                | "boolean"
                | "blob"
                | "timestamp"
                | "document"
        )
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
            Some("union") => render_union(output, shape, terminal(&id)),
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
    let module = names::snake_case(operation_name);
    let operation = operation_shape(selected, operation_name).expect("selected operation exists");
    let rust_operation = rust_type_name(operation_name);
    let mut output = String::new();
    header(&mut output);
    writeln!(
        output,
        "#[derive(Clone, Debug, Default)]\npub struct {rust_operation};\nimpl {rust_operation} {{ pub fn new() -> Self {{ Self }} }}"
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
    writeln!(
        output,
        "pub type {rust_operation}Error = Error;\npub type {rust_operation}FluentBuilder = builders::Builder;"
    )
    .unwrap();
    output
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
    output.pop();
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
        writeln!(
            output,
            "#[derive(Clone, Debug, Default)]\npub struct {rust_name};"
        )
        .unwrap();
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
    let operation = operation_shape(selected, operation_name).expect("selected operation exists");
    let protocol = selected
        .model
        .protocol()
        .expect("selected model protocol was validated before rendering");
    let module = names::snake_case(operation_name);
    let rust_operation = rust_type_name(operation_name);
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
    writeln!(
        output,
        "#[derive({builder_derives})]\npub struct Builder {{\n    input: super::Input,\n    client: super::super::super::Client,\n}}\nimpl Builder {{\n    pub fn new() -> Self {{ Self::default() }}\n    pub fn with_client(client: super::super::super::Client) -> Self {{\n        Self {{ input: super::Input::default(), client }}\n    }}"
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
            let assignment = if member_target(member).is_some_and(is_streaming_target) {
                format!("self.input.{field} = value.into()")
            } else {
                format!("self.input.{field} = Some(value.into())")
            };
            writeln!(
                output,
                "    pub fn {field}(mut self, value: impl ::std::convert::Into<{target}>) -> Self {{ {assignment}; self }}"
            )
            .unwrap();
        }
    }
    output.push_str("    pub fn build(self) -> super::Input { self.input }\n");
    render_operation_send(
        &mut output,
        operation_name,
        selected,
        operation,
        protocol,
        consumer_namespace,
    );
    output.push_str("}\n");
    writeln!(output, "pub use Builder as {rust_operation}FluentBuilder;").unwrap();
    output
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
        "                     {send_allow}\n                     pub async fn send(self) -> ::std::result::Result<super::{rust_operation}Output, super::{rust_operation}Error> {{"
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
                    "                         let {field} = self.input.{field}.as_deref().ok_or_else(|| super::{rust_operation}Error::Unhandled(\"{rust_operation} requires {field}\".to_owned()))?;"
                )
                .unwrap();
            }
        }
    }
    writeln!(
        output,
        "                         let path = {path_expression};\n                         let body = {body_expression};\n                         let headers = {headers_expression};\n                         let response = self.client.request(super::super::super::transport::Method::{http_method}, &path, &headers, &body).await.map_err(super::{rust_operation}Error::Unhandled)?;"
    )
    .unwrap();
    output.push_str("                         let status = response.status();\n                         if !status.is_success() {\n");
    writeln!(
        output,
        "                             return Err(super::{rust_operation}Error::unhandled_with_request_ids(format!(\"{rust_operation} returned HTTP {{}}\", status), response.header(\"x-amzn-requestid\").map(str::to_owned), {extended_request_id}));"
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
        return format!("self.input.{field}.clone().into_inner()");
    }
    if protocol != crate::model::ProtocolKind::RestXml {
        return "::std::vec::Vec::new()".to_owned();
    }

    let Some(target_shape) = selected.model.shapes.get(target) else {
        return "::std::vec::Vec::new()".to_owned();
    };
    let root = xml_name(member).unwrap_or_else(|| terminal(target).to_owned());
    let mut expression = String::from(
        "{ let mut body = ::std::string::String::new(); if let Some(value) = self.input.",
    );
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

fn is_xml_body_member(member: &Value) -> bool {
    let Some(traits) = member.get("traits").and_then(Value::as_object) else {
        return true;
    };
    if traits.contains_key("smithy.api#httpHeader")
        || traits.contains_key("smithy.api#httpPrefixHeaders")
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
                        " if let Some(value) = self.input.{field}.as_deref() {{ path.push_str(if path.contains('?') {{ \"&\" }} else {{ \"?\" }}); path.push_str({query:?}); path.push('='); path.push_str(&super::super::super::transport::encode_path(value)); }}"
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
                        " if let Some(value) = self.input.{field}.as_ref() {{ path.push_str(if path.contains('?') {{ \"&\" }} else {{ \"?\" }}); path.push_str({query:?}); path.push('='); path.push_str(&super::super::super::transport::encode_path(&value.to_string())); }}"
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
                " if let Some(value) = self.input.{field}.as_deref() {{ headers.push(({header:?}, value)); }}"
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
    render_protocol_request_headers(
        &mut output,
        selected,
        operation_name,
        input_shape,
        consumer_namespace,
    );
    output
}

/// Render the protocol-owned request payload wrapper for a RestXml operation.
///
/// Smithy keeps this code in a separate `shape_<operation>_input.rs` module.
/// The wrapper is deliberately derived from the HTTP payload member and its
/// target shape; the service and operation names only participate in stable
/// module/function names supplied by the model.
fn render_protocol_input_payload_file(
    selected: &SelectedModel,
    operation_name: &str,
) -> Option<String> {
    let operation = operation_shape(selected, operation_name)?;
    let input_shape = operation
        .get("input")
        .and_then(target_value)
        .and_then(|id| selected.model.shapes.get(id))?;
    let (member_name, member) = members(input_shape).into_iter().find(|(_, member)| {
        member
            .get("traits")
            .and_then(|traits| traits.get("smithy.api#httpPayload"))
            .is_some()
    })?;
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

    writeln!(
        output,
        "pub fn ser_{field}_http_payload(\n    payload: &::std::option::Option<crate::types::{target_name}>,\n) -> ::std::result::Result<::std::vec::Vec<u8>, ::aws_smithy_types::error::operation::BuildError> {{\n    let payload = match payload.as_ref() {{\n        Some(t) => t,\n        None => return Ok(crate::protocol_serde::{unset_payload}()),\n    }};\n    Ok(crate::protocol_serde::shape_{module}::ser_{field}_payload(\n        payload,\n    )?)\n}}\n\npub fn ser_{field}_payload(\n    input: &crate::types::{target_name},\n) -> std::result::Result<std::vec::Vec<u8>, ::aws_smithy_types::error::operation::SerializationError> {{\n    let mut out = String::new();\n    {{\n        let mut writer = ::aws_smithy_xml::encode::XmlWriter::new(&mut out);\n        #[allow(unused_mut)]\n        let mut root = writer.start_el({root:?}){namespace};\n        crate::protocol_serde::shape_{target_module}::ser_{target_function}(input, root)?\n    }}\n    Ok(out.into_bytes())\n}}",
        module = names::rust_module_name(operation_name),
    )
    .unwrap();
    Some(output)
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
    writeln!(
        output,
        "#[allow(clippy::unnecessary_wraps)]\npub fn de_{module}_http_response(\n    _response_status: u16,\n    _response_headers: &::aws_smithy_runtime_api::http::Headers,\n    _response_body: &[u8],\n) -> std::result::Result<{output_path}, {error_path}> {{"
    )
    .unwrap();
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
    if let Some(shape) = output_shape {
        for (name, member) in sorted_members(shape) {
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
    output.push_str("        output.build()\n    })\n}\n\n");
}

fn render_protocol_request_headers(
    output: &mut String,
    selected: &SelectedModel,
    operation_name: &str,
    input_shape: Option<&Value>,
    consumer_namespace: bool,
) {
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
                    "                         output.build().map_err(|error| super::{rust_operation}Error::Unhandled(error.to_string()))"
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
            "                         let body = response.text().await.map_err(super::{rust_operation}Error::Unhandled)?;\n"
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
            "                         output.build().map_err(|error| super::{rust_operation}Error::Unhandled(error.to_string()))"
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
    let derives = if operation_input(&context) {
        "::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug"
    } else {
        "::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug"
    };
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
        writeln!(output, "{inner}pub fn build(self) -> {value_path} {{").unwrap();
        writeln!(output, "{inner}    {value_path} {{}}").unwrap();
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
            render_builder_docs(
                output,
                selected,
                member,
                &inner,
                member_is_effectively_required(selected, member, target_id),
            );
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
    if required_members.is_empty() {
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
        writeln!(
            output,
            "{inner}/// This method will fail if any of the following fields are not set:"
        )
        .unwrap();
        let builder_path = builder_type_path(&context, &rust_name);
        for member_name in &required_members {
            let field_method = names::rust_identifier(member_name);
            let field_link = if context.consumer_namespace() {
                format!("Self::{field_method}")
            } else {
                format!("{builder_path}::{field_method}")
            };
            writeln!(output, "{inner}/// - [`{field_method}`]({field_link})").unwrap();
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
    if required_members.is_empty() {
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
    if required_members.is_empty() {
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

fn render_union(output: &mut String, shape: &Value, name: &str) {
    writeln!(
        output,
        "    #[derive(Clone, PartialEq, Debug)]\n    pub enum {} {{",
        rust_type_name(name)
    )
    .unwrap();
    for (member_name, _) in sorted_members(shape) {
        writeln!(output, "        {},", rust_type_name(&member_name)).unwrap();
    }
    output.push_str("        Unknown,\n    }\n\n");
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
            "    /// Constructs a fluent builder for the [`{rust_operation}`](crate::operation::{module}::builders::{rust_operation}FluentBuilder) operation."
        )
        .unwrap();
        if operation_is_paginated(operation_shape) {
            writeln!(
                output,
                "    /// This operation supports pagination; See [`into_paginator()`](crate::operation::{module}::builders::{rust_operation}FluentBuilder::into_paginator)."
            )
            .unwrap();
        }
        writeln!(output, "    ///").unwrap();
        if input_members.is_empty() {
            writeln!(
                output,
                "    /// - The fluent builder takes no input, just [`send`](crate::operation::{module}::builders::{rust_operation}FluentBuilder::send) it."
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
                    "    ///   - [`{field_method}({argument})`](crate::operation::{module}::builders::{rust_operation}FluentBuilder::{field_method}) / [`set_{field_method}(Option<{target}>)`](crate::operation::{module}::builders::{rust_operation}FluentBuilder::set_{field_method}):<br>required: **{required}**<br>{documentation}<br>"
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
            "    /// - On failure, responds with [`SdkError<{rust_operation}Error>`](crate::operation::{module}::{rust_operation}Error)"
        )
        .unwrap();
        let fluent_builder_path =
            format!("crate::operation::{module}::builders::{rust_operation}FluentBuilder");
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
            "        crate::operation::{module}::builders::{rust_operation}FluentBuilder::new(self.handle.clone())"
        )
        .unwrap();
        output.push_str("    }\n}\n");
        return output;
    }
    writeln!(
        output,
        "impl Client {{\n    pub fn {module}(&self) -> operation::{module}::{rust_operation}FluentBuilder {{ operation::{module}::{rust_operation}FluentBuilder::with_client(self.clone()) }}\n}}\n"
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
    Whitespace,
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
                tokens.push(DocumentationToken::Whitespace);
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

    for token in tokens {
        match token {
            DocumentationToken::Whitespace => pending_whitespace = true,
            DocumentationToken::Tag(tag) => {
                let closing = tag.trim_start().starts_with("</");
                let (normalized_tag, name) = normalize_documentation_tag(&tag, closing, &stack);

                if closing {
                    if documentation_newline_before_close(&name, previous_tag.as_ref()) {
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
                    if documentation_block_tag(&name) {
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
                {
                    documentation_newline(&mut output);
                } else if (pending_whitespace || has_leading_whitespace)
                    && documentation_space_before_text(&stack, &previous_tag, &output)
                {
                    documentation_space(&mut output);
                }
                output.push_str(&escape_documentation_text(&escape_doc_brackets(&trimmed)));
                pending_whitespace = text.chars().last().is_some_and(char::is_whitespace);
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
    (tag.to_owned(), name)
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
    if documentation_block_tag(name) || documentation_custom_tag(name) {
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
        if !matches!(tokens[index], DocumentationToken::Whitespace) {
            next = Some(index);
        }
    }
    for (index, token) in tokens.iter().enumerate() {
        if matches!(token, DocumentationToken::Whitespace) {
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
            DocumentationToken::Whitespace => unreachable!(),
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
        DocumentationToken::Whitespace => unreachable!(),
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
                .contains("pub type BucketName = ::std::string::String;")
        );
        assert!(!stage.path().join("aws_sdk_build_manifest.json").exists());
        assert!(!stage.path().join("generated/aws_sdk_s3.rs").exists());
    }
}
