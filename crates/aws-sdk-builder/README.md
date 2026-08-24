# aws-sdk-builder

Core generator for the service-specific `aws-sdk-builder-*` model providers.
The core crate contains code generation, shared assets, and output installation;
it intentionally contains no service models or consumer-facing include macro.

Use a service provider from a Cargo build script, for example
`aws_sdk_builder_s3::compile(["PutObject"])?`, and include that provider's
service-owned macro in the module you choose:

```rust
mod aws_s3_sdk {
    aws_sdk_builder_s3::include_sdk!();
}
```

Generated clients use the downstream consumer's ordinary AWS runtime
dependencies and are installed atomically below `OUT_DIR`. The generated
source uses relative internal paths, so the wrapper module can have any valid
Rust name.
