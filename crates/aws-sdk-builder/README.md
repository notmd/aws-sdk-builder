# aws-sdk-builder

Core generator for the service-specific `aws-sdk-builder-*` model providers.
The core crate contains code generation, shared assets, output installation, and
the aggregate `include_sdk!()` macro; it intentionally contains no service
models.

Use a service provider from a Cargo build script, for example
`aws_sdk_builder_s3::compile(["PutObject"])?`. Add `aws-sdk-builder` as a
normal dependency so the consumer can invoke `aws_sdk_builder::include_sdk!()`.
Generated clients use the downstream consumer's ordinary AWS runtime
dependencies and are installed atomically below `OUT_DIR`.
