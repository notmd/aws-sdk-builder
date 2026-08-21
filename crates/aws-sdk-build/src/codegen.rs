use serde_json::Value;
use std::{collections::BTreeMap, fmt::Write, fs, path::Path};

use crate::{
    config::ServiceSelection,
    error::BuildError,
    model::SelectedModel,
    names,
    registry::{self, GENERATOR_VERSION},
};

pub(crate) struct Generated {
    pub(crate) operations: Vec<String>,
}

pub(crate) fn generate(
    stage: &Path,
    consumer_crate_name: &str,
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
    let mut selected_services = Vec::new();
    let mut service_protocols = BTreeMap::new();
    let mut files = Vec::new();
    for selection in selections {
        let entry = registry::lookup(&selection.key)?;
        let model = crate::model::Model::load(entry)?;
        let selected = model.select(&selection.operations, selection.all_operations)?;
        let protocol = selected.model.protocol()?;
        service_protocols.insert(entry.key.to_owned(), protocol.trait_id());
        let service_dir = generated.join(entry.key);
        let mut service_files = vec![
            ("src/lib.rs".to_owned(), render_service_lib(entry.key)),
            ("src/primitives.rs".to_owned(), render_primitives()),
            ("src/aws_runtime.rs".to_owned(), render_aws_runtime()),
            ("src/config.rs".to_owned(), render_config_file()),
            ("src/error.rs".to_owned(), render_error_file()),
            ("src/meta.rs".to_owned(), render_meta(entry.key)),
            (
                "src/observability_feature.rs".to_owned(),
                render_observability_feature(),
            ),
            (
                "src/types.rs".to_owned(),
                render_types_file(entry.key, &selected),
            ),
            (
                "src/operation.rs".to_owned(),
                render_operations_file(entry.key, &selected),
            ),
            (
                "src/client.rs".to_owned(),
                render_client_file(entry.key, &selected),
            ),
        ];
        let mut operation_names = selected.operations.clone();
        operation_names.sort();
        for operation_name in operation_names {
            let module = names::snake_case(&operation_name);
            service_files.push((
                format!("src/operation/{module}.rs"),
                render_operation_file(entry.key, &selected, &operation_name),
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
            service_files.push((
                format!("src/client/{module}.rs"),
                render_client_operation_file(&operation_name),
            ));
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
                || !is_renderable_type(selected.model.shapes.get(&shape_id))
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
                        Context::Error
                    } else {
                        Context::Types
                    },
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
            files.push(format!("generated/{}/{}", entry.key, relative_path));
        }
        writeln!(
            facade,
            "#[allow(non_snake_case, dead_code, unused_imports)]"
        )
        .unwrap();
        writeln!(facade, "pub mod {} {{", entry.module_name).unwrap();
        writeln!(
            facade,
            "    include!(concat!(env!(\"OUT_DIR\"), \"/generated/{}/src/lib.rs\"));",
            entry.key
        )
        .unwrap();
        facade.push_str("}\n\n");
        selected_services.push(selection.key.clone());
        all_operations.extend(selected.operations.iter().cloned());
    }
    let include_path = stage.join("aws_sdk.rs");
    fs::write(&include_path, normalize_source(&facade)).map_err(|source| {
        BuildError::OutputWrite {
            path: include_path.clone(),
            source,
        }
    })?;
    files.push("aws_sdk.rs".to_owned());
    files.sort();
    all_operations.sort();
    let manifest = serde_json::json!({
        "generator_version": GENERATOR_VERSION,
        "consumer_crate_name": consumer_crate_name,
        "snapshot_sha": registry::AWS_SDK_RUST_SNAPSHOT,
        "smithy_reference_sha": registry::SMITHY_RS_SNAPSHOT,
        "selected_service_keys": selected_services,
        "service_protocols": service_protocols,
        "selected_operations": all_operations,
        "generated_source_files": files,
        "runtime_crate_requirements": ["aws-runtime"],
        "runtime_source_files": ["src/aws_runtime.rs"],
    });
    let manifest_path = stage.join("aws_sdk_build_manifest.json");
    let manifest_text = serde_json::to_string_pretty(&manifest)
        .map_err(|source| BuildError::ManifestSerialize { source })?
        + "\n";
    fs::write(&manifest_path, manifest_text).map_err(|source| BuildError::OutputWrite {
        path: manifest_path,
        source,
    })?;
    Ok(Generated {
        operations: all_operations,
    })
}

fn header(output: &mut String) {
    output.push_str("// Generated by aws-sdk-build ");
    output.push_str(GENERATOR_VERSION);
    output.push_str(". DO NOT EDIT.\n\n");
}

fn normalize_source(source: &str) -> String {
    format!("{}\n", source.trim_end_matches('\n'))
}

fn render_service_lib(service_key: &str) -> String {
    let mut output = String::new();
    header(&mut output);
    for file in [
        "primitives.rs",
        "aws_runtime.rs",
        "config.rs",
        "error.rs",
        "meta.rs",
        "types.rs",
        "operation.rs",
        "client.rs",
    ] {
        writeln!(
            output,
            "include!(concat!(env!(\"OUT_DIR\"), \"/generated/{service_key}/src/{file}\"));"
        )
        .unwrap();
    }
    output
}

fn render_primitives() -> String {
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
         }\n\n",
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
             pub fn meta(&self) -> ErrorMetadata { ErrorMetadata }\n\
         }\n\
         impl ::std::fmt::Display for Error {\n\
             fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {\n\
                 f.write_str(&self.message)\n\
             }\n\
         }\n\
         impl ::std::error::Error for Error {}\n\
         #[derive(Clone, Debug, Default)]\n\
         pub struct ErrorMetadata;\n\
         pub mod error { pub use super::{Error, ErrorMetadata}; }\n\n",
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

fn render_types_file(service_key: &str, selected: &SelectedModel) -> String {
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
            || (!is_renderable_type(Some(shape)) && !is_primitive_shape(shape))
            || is_error_shape(shape)
        {
            continue;
        }
        if is_primitive_shape(shape) {
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
    let mut error_ids = selected
        .model
        .shapes
        .iter()
        .filter_map(|(id, shape)| is_error_shape(shape).then_some(id.clone()))
        .collect::<Vec<_>>();
    error_ids.sort();
    for id in error_ids {
        let filename = type_file_name(&id);
        writeln!(
            output,
            "        include!(concat!(env!(\"OUT_DIR\"), \"/generated/{service_key}/src/types/error/{filename}\"));"
        )
        .unwrap();
    }
    output.push_str("    }\n");
    output.push_str("}\n\n");
    output
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

fn render_type_file(selected: &SelectedModel, shape_id: &str, context: Context) -> String {
    let is_error = matches!(context, Context::Error);
    let mut one_shape = selected.clone();
    one_shape.model.shapes = BTreeMap::from([(
        shape_id.to_owned(),
        selected
            .model
            .shapes
            .get(shape_id)
            .expect("renderable type must be present")
            .clone(),
    )]);
    let mut rendered = String::new();
    render_types_with_context(&mut rendered, &one_shape, context);
    let marker = "pub mod types {\n";
    let start = rendered.find(marker).expect("type module must be rendered") + marker.len();
    let end = rendered
        .rfind("\n}\n\n")
        .expect("type module must have a closing brace");
    let mut output = String::new();
    header(&mut output);
    output.push_str(rendered[start..end].trim_end());
    output.push('\n');
    if is_error {
        writeln!(
            output,
            "impl ::std::fmt::Display for {} {{ fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {{ f.write_str({:?}) }} }}",
            rust_type_name(terminal(shape_id)),
            terminal(shape_id)
        )
        .unwrap();
    }
    output
}

fn is_renderable_type(shape: Option<&Value>) -> bool {
    matches!(
        shape
            .and_then(|shape| shape.get("type"))
            .and_then(Value::as_str),
        Some("structure" | "union" | "enum" | "list" | "map")
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
    format!("_{}.rs", names::snake_case(terminal(shape_id)))
}

fn render_types_with_context(output: &mut String, selected: &SelectedModel, context: Context) {
    header(output);
    output.push_str("pub mod types {\n");
    let mut ids = selected.model.shapes.keys().cloned().collect::<Vec<_>>();
    ids.sort();
    for id in ids {
        if id == selected.model.entry.service_shape_id {
            continue;
        }
        let Some(shape) = selected.model.shapes.get(&id) else {
            continue;
        };
        match shape.get("type").and_then(Value::as_str) {
            Some("structure") => render_structure(output, shape, terminal(&id), context.clone()),
            Some("union") => render_union(output, shape, terminal(&id)),
            Some("enum") => render_enum(output, shape, terminal(&id)),
            Some("list") => {
                let member = shape
                    .get("member")
                    .and_then(|member| member.get("target"))
                    .and_then(Value::as_str)
                    .map(|target| type_expr(target, context.clone()))
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
                    .map(|target| type_expr(target, context.clone()))
                    .unwrap_or_else(|| "::std::string::String".to_owned());
                let value = shape
                    .get("value")
                    .and_then(|member| member.get("target"))
                    .and_then(Value::as_str)
                    .map(|target| type_expr(target, Context::Types))
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

fn render_operations_file(service_key: &str, selected: &SelectedModel) -> String {
    let mut output = String::new();
    header(&mut output);
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
    render_operation_error(&mut output, operation);
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
    let shape_id = operation
        .get(if input { "input" } else { "output" })
        .and_then(target_value);
    let shape = shape_id.and_then(|id| selected.model.shapes.get(id));
    let rust_name = operation_shape_file_name(operation_name, input);
    let module = names::snake_case(operation_name);
    let mut output = String::new();
    header(&mut output);
    if let Some(shape) = shape {
        render_structure_at_indent(
            &mut output,
            shape,
            &rust_name,
            Context::Operation(module.clone()),
            0,
        );
        render_operation_accessors(
            &mut output,
            selected,
            shape,
            &rust_name,
            Context::Operation(module),
        );
        render_operation_shape_builder(
            &mut output,
            shape,
            &rust_name,
            Context::Builder(String::new()),
        );
    } else {
        writeln!(
            output,
            "#[derive(Clone, Debug, Default)]\npub struct {rust_name};"
        )
        .unwrap();
    }
    output
}

fn render_operation_shape_builder(
    output: &mut String,
    shape: &Value,
    name: &str,
    context: Context,
) {
    let rust_name = rust_type_name(name);
    writeln!(
        output,
        "impl {rust_name} {{\n    pub fn builder() -> {rust_name}Builder {{ {rust_name}Builder::default() }}\n}}\n#[derive(Clone, Debug, Default)]\npub struct {rust_name}Builder {{"
    )
    .unwrap();
    for (member_name, member) in members(shape) {
        let field = names::rust_identifier(&member_name);
        let target = member_target(member)
            .map(|target| type_expr(target, context.clone()))
            .unwrap_or_else(|| "::std::string::String".to_owned());
        writeln!(output, "    {field}: ::std::option::Option<{target}>,").unwrap();
    }
    writeln!(output, "}}\nimpl {rust_name}Builder {{").unwrap();
    for (member_name, member) in members(shape) {
        let field = names::rust_identifier(&member_name);
        let method = field.strip_prefix("r#").unwrap_or(&field);
        let target = member_target(member)
            .map(|target| type_expr(target, context.clone()))
            .unwrap_or_else(|| "::std::string::String".to_owned());
        writeln!(
            output,
            "    pub fn {field}(mut self, input: impl ::std::convert::Into<{target}>) -> Self {{ self.{field} = Some(input.into()); self }}\n    pub fn set_{method}(mut self, input: ::std::option::Option<{target}>) -> Self {{ self.{field} = input; self }}\n    pub fn get_{method}(&self) -> &::std::option::Option<{target}> {{ &self.{field} }}"
        )
        .unwrap();
    }
    writeln!(
        output,
        "    pub fn build(self) -> {rust_name} {{ {rust_name} {{"
    )
    .unwrap();
    for (member_name, _) in members(shape) {
        let field = names::rust_identifier(&member_name);
        writeln!(output, "        {field}: self.{field},").unwrap();
    }
    output.push_str("    } }\n}\n");
}

fn render_operation_builder_file(selected: &SelectedModel, operation_name: &str) -> String {
    let operation = operation_shape(selected, operation_name).expect("selected operation exists");
    let protocol = selected
        .model
        .protocol()
        .expect("selected model protocol was validated before rendering");
    let module = names::snake_case(operation_name);
    let rust_operation = rust_type_name(operation_name);
    let mut output = String::new();
    header(&mut output);
    output.push_str(
        "#[derive(Clone, Debug, Default)]\npub struct Builder {\n    input: super::Input,\n    client: super::super::super::Client,\n}\nimpl Builder {\n    pub fn new() -> Self { Self::default() }\n    pub fn with_client(client: super::super::super::Client) -> Self {\n        Self { input: super::Input::default(), client }\n    }\n",
    );
    if let Some(input_id) = operation.get("input").and_then(target_value)
        && let Some(shape) = selected.model.shapes.get(input_id)
    {
        for (name, member) in members(shape) {
            let field = names::rust_identifier(&name);
            let target = member_target(member)
                .map(|target| type_expr(target, Context::Builder(module.clone())))
                .unwrap_or_else(|| "::std::string::String".to_owned());
            writeln!(
                output,
                "    pub fn {field}(mut self, value: impl ::std::convert::Into<{target}>) -> Self {{ self.input.{field} = Some(value.into()); self }}"
            )
            .unwrap();
        }
    }
    output.push_str("    pub fn build(self) -> super::Input { self.input }\n");
    render_operation_send(&mut output, operation_name, selected, operation, protocol);
    output.push_str("}\n");
    writeln!(output, "pub use Builder as {rust_operation}FluentBuilder;").unwrap();
    output
}

fn render_operation_error(output: &mut String, operation: &Value) {
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
    output.push_str("    Unhandled(::std::string::String),\n}\nimpl Error {\n");
    if let Some(errors) = operation.get("errors").and_then(Value::as_array) {
        for error in errors.iter().filter_map(target_value) {
            let error_name = rust_type_name(terminal(error));
            let predicate = names::snake_case(terminal(error));
            writeln!(output, "    pub fn is_{predicate}(&self) -> bool {{ matches!(self, Self::{error_name}(_)) }}").unwrap();
        }
    }
    output.push_str("}\nimpl ::std::fmt::Display for Error {\n    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {\n        match self {\n            Self::Unhandled(message) => f.write_str(message),\n");
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
}

fn render_operation_send(
    output: &mut String,
    operation_name: &str,
    selected: &SelectedModel,
    operation: &Value,
    protocol: crate::model::ProtocolKind,
) {
    let rust_operation = rust_type_name(operation_name);
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
    let path_expression = render_request_path(uri, input_shape, selected);
    let body_expression = render_request_body(selected, input_shape, protocol);
    let headers_expression = render_request_headers(input_shape, protocol);

    writeln!(
        output,
        "                     #[allow(clippy::possible_missing_else, clippy::field_reassign_with_default)]\n                     pub async fn send(self) -> ::std::result::Result<super::{rust_operation}Output, super::{rust_operation}Error> {{"
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
        "                             return Err(super::{rust_operation}Error::Unhandled(format!(\"{rust_operation} returned HTTP {{}}\", status)));"
    )
    .unwrap();
    output.push_str("                         }\n");
    render_response_decode(output, operation_name, selected, output_shape, protocol);
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
        return format!(
            "self.input.{field}.as_ref().map(|body| body.clone().into_inner()).unwrap_or_default()"
        );
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
        target,
        target_shape,
        "value",
        &root,
        false,
        1,
    );
    expression.push_str(" } body.into_bytes() }");
    expression
}

fn render_xml_value(
    output: &mut String,
    selected: &SelectedModel,
    _target: &str,
    shape: &Value,
    value_expression: &str,
    tag: &str,
    flattened: bool,
    _indent: usize,
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
                output.push_str(&format!(
                    " if let Some(value) = {field_expression}.as_ref() {{"
                ));
                render_xml_value(
                    output,
                    selected,
                    member_target,
                    member_shape,
                    "value",
                    &member_tag,
                    traits.is_some_and(|traits| traits.contains_key("smithy.api#xmlFlattened")),
                    _indent + 1,
                );
                output.push_str(" }");
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
                element_target,
                element_shape,
                "item",
                &element_tag,
                false,
                _indent + 1,
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
            members(shape).values().any(|member| {
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
    let mut output =
        String::from("{ let mut headers: ::std::vec::Vec<(&str, &str)> = ::std::vec::Vec::new();");
    let mut has_headers = false;
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
            has_headers = true;
            let field = names::rust_identifier(&name);
            output.push_str(&format!(
                " if let Some(value) = self.input.{field}.as_deref() {{ headers.push(({header:?}, value)); }}"
            ));
        }
    }
    let has_xml_payload = protocol == crate::model::ProtocolKind::RestXml
        && input_shape.is_some_and(|shape| {
            members(shape).values().any(|member| {
                member
                    .get("traits")
                    .and_then(|traits| traits.get("smithy.api#httpPayload"))
                    .is_some_and(|_| {
                        terminal(member_target(member).unwrap_or_default()) != "StreamingBlob"
                    })
            })
        });
    if has_xml_payload {
        output.push_str(" headers.push((\"content-type\", \"application/xml\"));");
    }
    if has_headers || has_xml_payload {
        output.push_str(" headers }");
        output
    } else {
        "::std::vec::Vec::new()".to_owned()
    }
}

fn render_response_decode(
    output: &mut String,
    operation_name: &str,
    selected: &SelectedModel,
    shape: Option<&Value>,
    protocol: crate::model::ProtocolKind,
) {
    let rust_operation = rust_type_name(operation_name);
    let has_decoded_values = shape
        .map(|shape| {
            members(shape).values().any(|member| {
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
        writeln!(
            output,
            "                         Ok(super::{rust_operation}Output::default())"
        )
        .unwrap();
        return;
    }
    output.push_str(&format!(
        "                         let mut output = super::{rust_operation}Output::default();\n"
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
                .values()
                .any(|member| is_xml_body_member(member))
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
                        "                         output.{field} = Some(super::super::super::primitives::ByteStream::from(response.body().to_vec()));\n"
                    ));
                } else if protocol == crate::model::ProtocolKind::RestXml {
                    render_xml_member_decode(output, selected, &field, &name, member);
                }
                continue;
            }
            if is_xml_flattened_list(member, selected) {
                render_xml_flattened_list_decode(output, selected, &name, member);
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
                render_xml_member_decode(output, selected, &field, &name, member);
            }
        }
    }
    output.push_str("                         Ok(output)\n");
}

fn render_xml_flattened_list_decode(
    output: &mut String,
    selected: &SelectedModel,
    member_name: &str,
    member: &Value,
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
    let element_type = type_expr(element_id, Context::Builder(String::new()));
    let tag = xml_name(member).unwrap_or_else(|| member_name.to_owned());
    if element_shape.get("type").and_then(Value::as_str) != Some("structure") {
        writeln!(
            output,
            "                         let values = super::super::super::transport::xml_tags(&body, {tag:?}).into_iter().filter_map(|value| value.parse().ok()).collect();\n                         output.{output_field} = Some(values);"
        )
        .unwrap();
        return;
    }
    output.push_str(&format!(
        "                         let values = super::super::super::transport::xml_tags(&body, {tag:?}).into_iter().map(|value| {{ let mut item: {element_type} = ::std::default::Default::default();"
    ));
    render_xml_structure_decode(output, selected, element_shape, "item", "value");
    writeln!(
        output,
        " item }}).collect();\n                         output.{output_field} = Some(values);"
    )
    .unwrap();
}

fn render_xml_member_decode(
    output: &mut String,
    selected: &SelectedModel,
    field: &str,
    member_name: &str,
    member: &Value,
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
                .map(|target| type_expr(target, Context::Builder(String::new())))
                .unwrap_or_else(|| "::std::string::String".to_owned());
            writeln!(
                output,
                "                         output.{field} = Some(super::super::super::transport::xml_tags(&body, {tag:?}).into_iter().flat_map(|value| super::super::super::transport::xml_tags(&value, {element_tag:?})).filter_map(|value| value.parse::<{element_type}>().ok()).collect());"
            )
            .unwrap();
        }
        Some("structure") => {
            let element_type = type_expr(target, Context::Builder(String::new()));
            output.push_str(&format!(
                "                         if let Some(value) = super::super::super::transport::xml_first(&body, {tag:?}) {{ let mut item: {element_type} = ::std::default::Default::default();"
            ));
            render_xml_structure_decode(output, selected, shape, "item", "value");
            writeln!(output, " item; output.{field} = Some(item); }}").unwrap();
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

fn render_operation_accessors(
    output: &mut String,
    selected: &SelectedModel,
    shape: &Value,
    name: &str,
    context: Context,
) {
    writeln!(output, "        impl {name} {{").unwrap();
    for (member_name, member) in members(shape) {
        let field = names::rust_identifier(&member_name);
        let target = member_target(member).unwrap_or("smithy.api#String");
        let target_shape = selected.model.shapes.get(target);
        let target_kind = target_shape
            .and_then(|shape| shape.get("type"))
            .and_then(Value::as_str)
            .unwrap_or_else(|| terminal(target));
        let method_field = &field;
        if target == "com.amazonaws.s3#StreamingBlob" {
            writeln!(
                output,
                "            pub fn {method_field}(&self) -> &super::super::super::primitives::ByteStream {{ self.{field}.as_ref().expect(\"streaming payload is present on a successful response\") }}"
            )
            .unwrap();
        } else if target_kind == "list" {
            let member_target = target_shape
                .and_then(|shape| shape.get("member"))
                .and_then(member_target)
                .unwrap_or("smithy.api#String");
            let element = type_expr(member_target, context.clone());
            writeln!(
                output,
                "            pub fn {method_field}(&self) -> &[{element}] {{ self.{field}.as_deref().unwrap_or(&[]) }}"
            )
            .unwrap();
        } else if is_string_type(target, target_shape) {
            writeln!(
                output,
                "            pub fn {method_field}(&self) -> ::std::option::Option<&str> {{ self.{field}.as_deref() }}"
            )
            .unwrap();
        } else if matches!(
            target_kind,
            "boolean" | "integer" | "long" | "short" | "byte" | "float" | "double"
        ) {
            writeln!(
                output,
                "            pub fn {method_field}(&self) -> ::std::option::Option<{}> {{ self.{field} }}",
                primitive_type(target_kind)
            )
            .unwrap();
        } else {
            let reference = type_expr(target, context.clone());
            writeln!(
                output,
                "            pub fn {method_field}(&self) -> ::std::option::Option<&{reference}> {{ self.{field}.as_ref() }}"
            )
            .unwrap();
        }
    }
    output.push_str("        }\n");
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

fn render_structure(output: &mut String, shape: &Value, name: &str, context: Context) {
    render_structure_at_indent(output, shape, name, context.clone(), 4);
    render_type_builder(output, shape, name, context);
}

fn render_structure_at_indent(
    output: &mut String,
    shape: &Value,
    name: &str,
    context: Context,
    indent: usize,
) {
    let padding = " ".repeat(indent);
    writeln!(
        output,
        "{}#[derive(Clone, Debug, Default)]\n{}pub struct {} {{",
        padding,
        padding,
        rust_type_name(name)
    )
    .unwrap();
    for (member_name, member) in members(shape) {
        let field = names::rust_identifier(&member_name);
        let target = member_target(member)
            .map(|target| type_expr(target, context.clone()))
            .unwrap_or_else(|| "::std::string::String".to_owned());
        writeln!(
            output,
            "{}    pub {}: ::std::option::Option<{}>,",
            padding, field, target
        )
        .unwrap();
    }
    writeln!(output, "{}}}", padding).unwrap();
}

fn render_type_builder(output: &mut String, shape: &Value, name: &str, context: Context) {
    let rust_name = rust_type_name(name);
    writeln!(
        output,
        "    impl {rust_name} {{\n        pub fn builder() -> {rust_name}Builder {{ {rust_name}Builder::default() }}"
    )
    .unwrap();
    for (member_name, member) in members(shape) {
        let field = names::rust_identifier(&member_name);
        let target = member_target(member)
            .map(|target| type_expr(target, context.clone()))
            .unwrap_or_else(|| "::std::string::String".to_owned());
        let raw_target = member_target(member).unwrap_or_default();
        let field_method = &field;
        if is_string_type(raw_target, None) {
            writeln!(
                output,
                "        pub fn {field_method}(&self) -> ::std::option::Option<&str> {{ self.{field}.as_deref() }}"
            )
            .unwrap();
        } else {
            writeln!(
                output,
                "        pub fn {field_method}(&self) -> &::std::option::Option<{target}> {{ &self.{field} }}"
            )
            .unwrap();
        }
    }
    output.push_str("    }\n\n");
    writeln!(
        output,
        "    #[derive(Clone, Debug, Default)]\n    pub struct {rust_name}Builder {{"
    )
    .unwrap();
    for (member_name, member) in members(shape) {
        let field = names::rust_identifier(&member_name);
        let target = member_target(member)
            .map(|target| type_expr(target, context.clone()))
            .unwrap_or_else(|| "::std::string::String".to_owned());
        writeln!(output, "        {field}: ::std::option::Option<{target}>,").unwrap();
    }
    output.push_str("    }\n\n");
    writeln!(output, "    impl {rust_name}Builder {{").unwrap();
    for (member_name, member) in members(shape) {
        let field = names::rust_identifier(&member_name);
        let field_method = field.strip_prefix("r#").unwrap_or(&field);
        let target = member_target(member)
            .map(|target| type_expr(target, context.clone()))
            .unwrap_or_else(|| "::std::string::String".to_owned());
        writeln!(
            output,
            "        pub fn {field}(mut self, input: impl ::std::convert::Into<{target}>) -> Self {{ self.{field} = Some(input.into()); self }}"
        )
        .unwrap();
        writeln!(
            output,
            "        pub fn set_{field_method}(mut self, input: ::std::option::Option<{target}>) -> Self {{ self.{field} = input; self }}"
        )
        .unwrap();
        writeln!(
            output,
            "        pub fn get_{field_method}(&self) -> &::std::option::Option<{target}> {{ &self.{field} }}"
        )
        .unwrap();
    }
    writeln!(output, "        pub fn build(self) -> {rust_name} {{").unwrap();
    writeln!(output, "            {rust_name} {{").unwrap();
    for (member_name, _) in members(shape) {
        let field = names::rust_identifier(&member_name);
        writeln!(output, "                {field}: self.{field},").unwrap();
    }
    output.push_str("            }\n        }\n    }\n\n");
}

fn render_union(output: &mut String, shape: &Value, name: &str) {
    writeln!(
        output,
        "    #[derive(Clone, Debug)]\n    pub enum {} {{",
        rust_type_name(name)
    )
    .unwrap();
    for (member_name, _) in members(shape) {
        writeln!(output, "        {},", rust_type_name(&member_name)).unwrap();
    }
    output.push_str("        Unknown,\n    }\n\n");
}

fn render_enum(output: &mut String, shape: &Value, name: &str) {
    let rust_name = rust_type_name(name);
    writeln!(
        output,
        "    #[derive(Clone, Debug, PartialEq, Eq)]\n    pub enum {rust_name} {{"
    )
    .unwrap();
    for (member_name, _) in members(shape) {
        writeln!(output, "        {},", rust_type_name(&member_name)).unwrap();
    }
    output.push_str("        Unknown(::std::string::String),\n    }\n\n");
    writeln!(output, "    impl ::std::fmt::Display for {rust_name} {{").unwrap();
    output.push_str("        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {\n            match self {\n");
    for (member_name, member) in members(shape) {
        let value = member
            .get("traits")
            .and_then(|traits| traits.get("smithy.api#enumValue"))
            .and_then(Value::as_str)
            .unwrap_or(&member_name);
        writeln!(
            output,
            "                Self::{} => f.write_str({value:?}),",
            rust_type_name(&member_name)
        )
        .unwrap();
    }
    output.push_str("                Self::Unknown(value) => f.write_str(value),\n            }\n        }\n    }\n\n");
    writeln!(
        output,
        "    impl ::std::str::FromStr for {rust_name} {{ type Err = (); fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {{ match value {{"
    )
    .unwrap();
    for (member_name, member) in members(shape) {
        let value = member
            .get("traits")
            .and_then(|traits| traits.get("smithy.api#enumValue"))
            .and_then(Value::as_str)
            .unwrap_or(&member_name);
        writeln!(
            output,
            "            {value:?} => ::std::result::Result::Ok(Self::{}),",
            rust_type_name(&member_name)
        )
        .unwrap();
    }
    output.push_str("            _ => ::std::result::Result::Err(()),\n        } } }\n\n");
}

fn render_client_file(service_key: &str, selected: &SelectedModel) -> String {
    let mut output = String::new();
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

fn render_client_operation_file(operation: &str) -> String {
    let module = names::snake_case(operation);
    let rust_operation = rust_type_name(operation);
    let mut output = String::new();
    header(&mut output);
    writeln!(
        output,
        "impl Client {{\n    pub fn {module}(&self) -> operation::{module}::{rust_operation}FluentBuilder {{ operation::{module}::{rust_operation}FluentBuilder::with_client(self.clone()) }}\n}}\n"
    )
    .unwrap();
    output
}

#[derive(Clone)]
enum Context {
    Types,
    Error,
    Operation(String),
    Builder(String),
}

fn type_expr(target: &str, context: Context) -> String {
    if target.starts_with("smithy.api#") {
        return primitive_type(target.rsplit('#').next().unwrap_or("string"));
    }
    let name = terminal(target);
    let known = rust_type_name(name);
    match context {
        Context::Types if name == "StreamingBlob" => "super::primitives::ByteStream".to_owned(),
        Context::Types => format!("self::{known}"),
        Context::Error => format!("super::super::types::{known}"),
        Context::Operation(module) => {
            if name == "StreamingBlob" {
                "super::super::super::primitives::ByteStream".to_owned()
            } else if name.ends_with("Input") {
                format!("super::super::{module}::Input")
            } else if name.ends_with("Output") {
                format!("super::super::{module}::Output")
            } else {
                format!("super::super::super::types::{known}")
            }
        }
        Context::Builder(module) => {
            if name == "StreamingBlob" {
                "super::super::super::primitives::ByteStream".to_owned()
            } else if name.ends_with("Input") {
                format!("super::{module}::Input")
            } else if name.ends_with("Output") {
                format!("super::{module}::Output")
            } else {
                format!("super::super::super::types::{known}")
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

fn members(shape: &Value) -> BTreeMap<String, &Value> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_runtime_is_valid_rust() {
        let source = render_aws_runtime();
        if let Err(error) = syn::parse_file(&source) {
            panic!("{error}\n{source}");
        }
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
            operations: vec!["AbortMultipartUpload".to_owned()],
            all_operations: false,
        }];

        generate(stage.path(), "generated_consumer", &selections).unwrap();

        let generated = stage.path().join("generated/s3/src");
        assert!(generated.join("lib.rs").is_file());
        assert!(generated.join("aws_runtime.rs").is_file());
        assert!(generated.join("observability_feature.rs").is_file());
        assert!(generated.join("client.rs").is_file());
        assert!(generated.join("operation.rs").is_file());
        assert!(
            generated
                .join("operation/abort_multipart_upload.rs")
                .is_file()
        );
        assert!(generated.join("types.rs").is_file());
        assert!(!generated.join("types/_bucket_name.rs").exists());
        assert!(
            fs::read_to_string(generated.join("types.rs"))
                .unwrap()
                .contains("pub type BucketName = ::std::string::String;")
        );
        let manifest =
            fs::read_to_string(stage.path().join("aws_sdk_build_manifest.json")).unwrap();
        assert!(manifest.contains("\"aws-runtime\""));
        assert!(manifest.contains("runtime_source_files"));
        assert!(!stage.path().join("generated/aws_sdk_s3.rs").exists());
    }
}
