# aws-sdk-build

Rust-native AWS SDK model selection and source generation for Cargo build
scripts. The packaged model registry is described by models-manifest.json.
Generated clients require the normal downstream dependency `aws-runtime` for
AWS runtime metadata and wiring. The build output contains only `aws_sdk.rs` and
generated Rust source; `aws-sdk-build` itself remains codegen-only.
