use std::{collections::BTreeMap, fmt::Write, fs, path::Path};

use serde_json::Value;

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
    let mut all_operations = Vec::new();
    let mut selected_services = Vec::new();
    let mut files = Vec::new();
    for selection in selections {
        let entry = registry::lookup(&selection.key)?;
        let model = crate::model::Model::load(entry)?;
        let selected = model.select(&selection.operations, selection.all_operations)?;
        let service_dir = generated.join(entry.key);
        let mut service_files = vec![
            ("src/lib.rs".to_owned(), render_service_lib(entry.key)),
            ("src/primitives.rs".to_owned(), render_primitives()),
            ("src/config.rs".to_owned(), render_config_file()),
            ("src/error.rs".to_owned(), render_error_file()),
            ("src/meta.rs".to_owned(), render_meta()),
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
            service_files.push((
                format!("src/operation/{}.rs", names::snake_case(&operation_name)),
                render_operation_file(&selected, &operation_name),
            ));
            service_files.push((
                format!("src/client/{}.rs", names::snake_case(&operation_name)),
                render_client_operation_file(&operation_name),
            ));
        }
        let mut shape_ids = selected.model.shapes.keys().cloned().collect::<Vec<_>>();
        shape_ids.sort();
        for shape_id in shape_ids {
            if shape_id == selected.model.entry.service_shape_id
                || !is_renderable_type(selected.model.shapes.get(&shape_id))
            {
                continue;
            }
            service_files.push((
                format!("src/types/{}", type_file_name(&shape_id)),
                render_type_file(&selected, &shape_id),
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
        "selected_operations": all_operations,
        "generated_source_files": files,
        "runtime_crate_requirements": [],
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
        "#[derive(Clone, Debug, Default)]\n\
         pub struct Config;\n\n\
         pub mod config {\n\
             #[derive(Clone, Debug, Default)]\n\
             pub struct Builder;\n\
             impl Builder {\n\
                 pub fn build(self) -> super::Config { super::Config }\n\
             }\n\
             impl From<&super::Config> for Builder {\n\
                 fn from(_: &super::Config) -> Self { Self }\n\
             }\n\
         }\n\n\
         impl Config {\n\
             pub fn builder() -> config::Builder { config::Builder }\n\
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

fn render_meta() -> String {
    let mut output = String::new();
    header(&mut output);
    output.push_str(
        "pub mod meta {\n\
             pub const PKG_VERSION: &str = \"0.1.0\";\n\
         }\n\n",
    );
    output
}

fn render_types_file(service_key: &str, selected: &SelectedModel) -> String {
    let mut output = String::new();
    header(&mut output);
    output.push_str("pub mod types {\n");
    let mut ids = selected.model.shapes.keys().cloned().collect::<Vec<_>>();
    ids.sort();
    for id in ids {
        if id == selected.model.entry.service_shape_id
            || !is_renderable_type(selected.model.shapes.get(&id))
        {
            continue;
        }
        let filename = type_file_name(&id);
        writeln!(
            output,
            "    include!(concat!(env!(\"OUT_DIR\"), \"/generated/{service_key}/src/types/{filename}\"));"
        )
        .unwrap();
    }
    output.push_str("}\n\n");
    output
}

fn render_type_file(selected: &SelectedModel, shape_id: &str) -> String {
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
    render_types(&mut rendered, &one_shape);
    let marker = "pub mod types {\n";
    let start = rendered.find(marker).expect("type module must be rendered") + marker.len();
    let end = rendered
        .rfind("\n}\n\n")
        .expect("type module must have a closing brace");
    let mut output = String::new();
    header(&mut output);
    output.push_str(rendered[start..end].trim_end());
    output.push('\n');
    output
}

fn is_renderable_type(shape: Option<&Value>) -> bool {
    matches!(
        shape
            .and_then(|shape| shape.get("type"))
            .and_then(Value::as_str),
        Some(
            "structure"
                | "union"
                | "enum"
                | "list"
                | "map"
                | "string"
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

fn render_types(output: &mut String, selected: &SelectedModel) {
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
            Some("structure") => render_structure(output, shape, terminal(&id), Context::Types),
            Some("union") => render_union(output, shape, terminal(&id)),
            Some("enum") => render_enum(output, shape, terminal(&id)),
            Some("list") => {
                let member = shape
                    .get("member")
                    .and_then(|member| member.get("target"))
                    .and_then(Value::as_str)
                    .map(|target| type_expr(target, Context::Types))
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
                    .map(|target| type_expr(target, Context::Types))
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

fn render_operation_file(selected: &SelectedModel, operation_name: &str) -> String {
    let module = names::snake_case(operation_name);
    let mut one_operation = selected.clone();
    one_operation.operations = vec![operation_name.to_owned()];
    let mut rendered = String::new();
    render_operations(&mut rendered, &one_operation);
    let marker = format!("    pub mod {module} {{\n");
    let start = rendered
        .find(&marker)
        .expect("selected operation must be rendered")
        + marker.len();
    let end = rendered
        .rfind("\n}\n\n")
        .expect("operation module must have an outer closing brace");
    let body = rendered[start..end].trim_end();
    let body_end = body
        .rfind('\n')
        .expect("operation module must have a closing brace");
    let mut output = String::new();
    header(&mut output);
    output.push_str(&body[..body_end]);
    output.push('\n');
    output
}

fn render_operations(output: &mut String, selected: &SelectedModel) {
    header(output);
    output.push_str("pub mod operation {\n");
    let mut operations = selected.operations.clone();
    operations.sort();
    for operation_name in operations {
        let namespace = selected
            .model
            .entry
            .service_shape_id
            .split('#')
            .next()
            .unwrap_or_default();
        let operation_id = format!("{namespace}#{operation_name}");
        let Some(operation) = selected.model.shapes.get(&operation_id) else {
            continue;
        };
        let module = names::snake_case(&operation_name);
        writeln!(output, "    pub mod {} {{", module).unwrap();
        writeln!(
            output,
            "        #[derive(Clone, Debug, Default)]\n        pub struct {};\n        impl {} {{ pub fn new() -> Self {{ Self }} }}",
            rust_type_name(&operation_name)
            , rust_type_name(&operation_name)
        )
        .unwrap();
        let input = operation
            .get("input")
            .and_then(Value::as_str)
            .or_else(|| operation.get("input").and_then(target_value));
        let output_shape = operation
            .get("output")
            .and_then(Value::as_str)
            .or_else(|| operation.get("output").and_then(target_value));
        render_operation_io(
            output,
            selected,
            input,
            "Input",
            Context::Operation(module.clone()),
        );
        render_operation_io(
            output,
            selected,
            output_shape,
            "Output",
            Context::Operation(module.clone()),
        );
        output.push_str("        #[derive(Clone, Debug)]\n        pub enum Error {\n");
        if let Some(errors) = operation.get("errors").and_then(Value::as_array) {
            for error in errors.iter().filter_map(target_value) {
                writeln!(
                    output,
                    "            {}({}),",
                    rust_type_name(terminal(error)),
                    rust_type_name(terminal(error))
                )
                .unwrap();
            }
        }
        output.push_str("            Unhandled(::std::string::String),\n        }\n");
        if let Some(errors) = operation.get("errors").and_then(Value::as_array) {
            for error in errors.iter().filter_map(target_value) {
                let error_name = rust_type_name(terminal(error));
                writeln!(
                    output,
                    "        #[derive(Clone, Debug)]\n        pub struct {} {{ pub meta: super::super::ErrorMetadata }}\n        impl {} {{ pub fn meta(&self) -> &super::super::ErrorMetadata {{ &self.meta }} }}",
                    error_name, error_name
                )
                .unwrap();
            }
        }
        output.push_str(
            "        impl ::std::fmt::Display for Error {\n\
                 fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {\n\
                     match self {\n\
                         Self::Unhandled(message) => f.write_str(message),\n",
        );
        if let Some(errors) = operation.get("errors").and_then(Value::as_array) {
            for error in errors.iter().filter_map(target_value) {
                let error_name = rust_type_name(terminal(error));
                writeln!(
                    output,
                    "                         Self::{}(_) => f.write_str(\"{}\"),",
                    error_name, error_name
                )
                .unwrap();
            }
        }
        output.push_str(
            "                     }\n\
                 }\n\
             }\n\
             impl ::std::error::Error for Error {}\n\
             pub mod builders {\n\
                 #[derive(Clone, Debug, Default)]\n\
                 pub struct Builder { input: super::Input }\n\
                 impl Builder {\n\
                     pub fn new() -> Self { Self::default() }\n",
        );
        if let Some(input_id) = input
            && let Some(shape) = selected.model.shapes.get(input_id)
        {
            for (name, member) in members(shape) {
                let field = names::rust_identifier(&name);
                let target = member_target(member)
                    .map(|target| type_expr(target, Context::Builder(module.clone())))
                    .unwrap_or_else(|| "::std::string::String".to_owned());
                writeln!(
                    output,
                    "                     pub fn {}(mut self, value: impl ::std::convert::Into<{}>) -> Self {{ self.input.{} = Some(value.into()); self }}",
                    field, target, field
                )
                .unwrap();
            }
        }
        output.push_str(
            "                     pub fn build(self) -> super::Input { self.input }\n\
                     pub async fn send(self) -> ::std::result::Result<super::Output, super::Error> {\n\
                         Err(super::Error::Unhandled(\"operation execution is not linked to a runtime\".to_owned()))\n\
                     }\n\
                 }\n\
             }\n\
             pub use builders::Builder;\n\
         }\n",
        );
    }
    output.push_str("}\n\n");
}

fn render_operation_io(
    output: &mut String,
    selected: &SelectedModel,
    shape_id: Option<&str>,
    name: &str,
    context: Context,
) {
    let Some(shape_id) = shape_id else {
        writeln!(
            output,
            "        #[derive(Clone, Debug, Default)]\n        pub struct {};",
            name
        )
        .unwrap();
        return;
    };
    let Some(shape) = selected.model.shapes.get(shape_id) else {
        writeln!(
            output,
            "        #[derive(Clone, Debug, Default)]\n        pub struct {};",
            name
        )
        .unwrap();
        return;
    };
    render_structure_at_indent(output, shape, name, context, 8);
}

fn render_structure(output: &mut String, shape: &Value, name: &str, context: Context) {
    render_structure_at_indent(output, shape, name, context, 4);
    render_type_builder(output, shape, name);
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

fn render_type_builder(output: &mut String, shape: &Value, name: &str) {
    let rust_name = rust_type_name(name);
    writeln!(
        output,
        "    impl {rust_name} {{\n        pub fn builder() -> {rust_name}Builder {{ {rust_name}Builder::default() }}"
    )
    .unwrap();
    for (member_name, member) in members(shape) {
        let field = names::rust_identifier(&member_name);
        let target = member_target(member)
            .map(|target| type_expr(target, Context::Types))
            .unwrap_or_else(|| "::std::string::String".to_owned());
        writeln!(
            output,
            "        pub fn {field}(&self) -> &::std::option::Option<{target}> {{ &self.{field} }}"
        )
        .unwrap();
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
            .map(|target| type_expr(target, Context::Types))
            .unwrap_or_else(|| "::std::string::String".to_owned());
        writeln!(output, "        {field}: ::std::option::Option<{target}>,").unwrap();
    }
    output.push_str("    }\n\n");
    writeln!(output, "    impl {rust_name}Builder {{").unwrap();
    for (member_name, member) in members(shape) {
        let field = names::rust_identifier(&member_name);
        let field_method = field.strip_prefix("r#").unwrap_or(&field);
        let target = member_target(member)
            .map(|target| type_expr(target, Context::Types))
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
    writeln!(
        output,
        "    #[derive(Clone, Debug, PartialEq, Eq)]\n    pub enum {} {{",
        rust_type_name(name)
    )
    .unwrap();
    if let Some(values) = shape.get("values").and_then(Value::as_array) {
        for value in values.iter().filter_map(Value::as_str) {
            writeln!(output, "        {},", rust_type_name(value)).unwrap();
        }
    }
    output.push_str("        Unknown(::std::string::String),\n    }\n\n");
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
             pub struct Client { config: Config }\n\
             impl Client {\n\
                 pub fn new(config: &Config) -> Self { Self { config: config.clone() } }\n\
             pub fn config(&self) -> &Config { &self.config }\n\
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
    let mut output = String::new();
    header(&mut output);
    writeln!(
        output,
        "impl Client {{\n    pub fn {module}(&self) -> operation::{module}::Builder {{ operation::{module}::Builder::new() }}\n}}\n"
    )
    .unwrap();
    output
}

#[derive(Clone)]
enum Context {
    Types,
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
        Context::Types => format!("self::{known}"),
        Context::Operation(module) => {
            if name.ends_with("Input") {
                format!("super::{module}::Input")
            } else if name.ends_with("Output") {
                format!("super::{module}::Output")
            } else {
                format!("super::super::types::{known}")
            }
        }
        Context::Builder(module) => {
            if name.ends_with("Input") {
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
        assert!(generated.join("client.rs").is_file());
        assert!(generated.join("operation.rs").is_file());
        assert!(
            generated
                .join("operation/abort_multipart_upload.rs")
                .is_file()
        );
        assert!(generated.join("types.rs").is_file());
        assert!(generated.join("types/_bucket_name.rs").is_file());
        assert!(!stage.path().join("generated/aws_sdk_s3.rs").exists());
    }
}
