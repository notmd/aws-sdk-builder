# Modular AWS SDK Build-Time Code Generation

## Goal

Provide a Rust build-dependency that lets a consumer select AWS service operations in `build.rs` and include generated Rust code containing only the selected operations and the Smithy shapes they transitively require. The generated API should follow the AWS SDK for Rust shape (`Client`, `Config`, fluent operation builders, inputs, outputs, errors, and model modules) and should use the AWS/Smithy runtime crates at runtime.

## Scope and assumptions

- The first release accepts a local, pinned Smithy model file or directory. Model acquisition is deliberately separate from code generation so builds are reproducible and work offline.
- The first release invokes an installed Smithy CLI with the smithy-rs Rust client codegen plugin. smithy-rs is the source of generated client semantics; this project does not reimplement protocol serialization, signing, retries, endpoint resolution, or fluent-client generation.
- The consumer owns runtime dependencies in its normal `[dependencies]`; `aws-sdk-build` is only a `[build-dependencies]` crate.
- The initial supported target is a single AWS service projection per generated module. Multiple service calls can be configured by creating multiple builders/output modules.
- Operation selection is explicit. An omitted operation list means all operations in the selected service, while an empty list is rejected to prevent accidental generation of a full SDK.
- Generated code is emitted under Cargo's `OUT_DIR`; consumers include the generated root with `include!(concat!(env!("OUT_DIR"), "/aws_sdk.rs"));`.

## Public build API

The public API is modeled after tonic/prost build configuration:

```rust
// build.rs
fn main() -> Result<(), Box<dyn std::error::Error>> {
    aws_sdk_build::configure()
        .model("models/s3.json")
        .service("com.amazonaws.s3#AmazonS3")
        .operations(["GetObject", "PutObject"])
        .out_dir(std::env::var_os("OUT_DIR").unwrap())
        .compile()?;
    Ok(())
}
```

The builder exposes `model`, `service`, `operations`, `out_dir`, `smithy`, `rust_client_codegen`, and `compile` methods. `compile` returns a typed error and emits `cargo:rerun-if-changed` for model/configuration inputs. It writes a generated root file plus the Smithy projection output below `OUT_DIR` and does not mutate the consumer's source tree.

## Architecture

### `aws-sdk-build`

This is the consumer-facing crate. It validates the builder configuration, discovers model files, computes the selected operation closure, writes a temporary Smithy projection, invokes the configured Smithy executable, and copies/normalizes the generated Rust crate into the requested output directory. The builder has no runtime dependencies on generated SDK crates.

### Model selection

The model loader accepts Smithy JSON AST files, the format used by AWS service model repositories and smithy-rs tests. It identifies the requested service shape and operation shapes, then computes a fixed-point closure over shape references:

1. Start with the service shape, selected operation shapes, operation inputs/outputs/errors, and service-level protocol/auth/endpoint traits.
2. Visit every member target, list/map value/key target, union member, resource identifier, and trait shape referenced by a retained shape.
3. Retain model-wide trait definitions and dependencies required by smithy-rs's AWS decorators.
4. Fail with a diagnostic listing the missing service or operation if the selection is invalid.

The pruned model is written deterministically. Shape and member ordering follows the source model's stable lexical order, and the selected operation names are included in the generated configuration hash. This ensures Cargo rebuilds when selection changes and makes generated output reviewable.

### Smithy invocation

The runner uses the Smithy CLI as the external tool, analogous to tonic-build using `protoc`. The build API resolves the executable in this order: an explicit `.smithy(...)` path, `SMITHY_CLI`, then `smithy` on `PATH`. The generated `smithy-build.json` configures the `rust-client-codegen` plugin with AWS SDK customizations, the selected service shape, module name, output directory, and runtime configuration. Consumer builds set `customizationConfig.awsSdk.awsSdkBuild` to `false` and suppress the release-only README: upstream smithy-rs requires a partitions file and `awsConfigVersion` when that flag is `true`, while the AWS customizations, runtime types, auth, and fluent client remain available in consumer mode. Smithy stdout/stderr are captured and included in errors with the exact command and output.

The project documents the required Smithy CLI and smithy-rs plugin versions and supports an explicit plugin/classpath option so CI can pin the generator. It does not silently download code or execute an unpinned remote artifact from a consumer build.

### Generated runtime contract

Generated code uses the public AWS SDK runtime APIs (`aws-runtime`, `aws-smithy-runtime`, `aws-smithy-runtime-api`, `aws-smithy-http`, `aws-smithy-types`, `aws-types`, and related protocol crates as required by the selected model). The generator emits a manifest fragment describing the exact runtime crates used; the consumer adds those crates as normal dependencies. Runtime code is never compiled as a build dependency and generated code does not depend on `aws-sdk-*` service crates, preserving modularity.

### Generated API

For a selected service, generated code contains:

- `Config` and `ConfigBuilder` for service configuration;
- `Client` and `Client::from_conf`/`Client::new` construction paths;
- one fluent operation method per selected operation;
- operation input builders, output types, modeled errors, and serialization/deserialization code;
- only model shapes reachable from selected operations, plus required service/runtime support types.

The generated root also exposes a stable include target and a machine-readable manifest containing selected operation names and generated files. The manifest is used by integration tests to prove that an unselected operation and an unreachable model shape were not generated.

## Error handling

Configuration errors are reported before invoking Smithy and identify the offending path, service ID, or operation. Model parsing and closure errors identify the shape ID and reference chain. Tool errors include the executable path, arguments, exit status, and captured output. Existing output files are replaced only after a successful generation in a temporary directory, so a failed build cannot leave a partially generated module.

## Testing strategy

- Unit tests cover builder validation, operation lookup, deterministic closure, missing references, and rerun directives.
- A fixture model contains two operations with disjoint input/output shapes and a shared error. A pruning test asserts that selected output includes the shared shape and excludes the unselected operation and disjoint shapes.
- A fake Smithy executable test verifies the generated invocation configuration without requiring a JDK or Smithy installation.
- An opt-in end-to-end test runs against a pinned smithy-rs/Smithy installation, generates a fixture AWS-like client, and compiles a consumer using `cargo check`.
- The repository's default CI test suite remains offline and does not require external AWS credentials or network access.

## Non-goals for the first release

- Automatically downloading or embedding every AWS service model.
- Replacing smithy-rs's Kotlin/Java code generator with a new Rust code generator.
- Supporting every AWS-specific handwritten customization before the Smithy runner path is proven.
- Generating a single cross-service client; services remain separate generated modules.
