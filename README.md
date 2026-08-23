# aws-sdk-builder

`aws-sdk-builder` is the core Rust-only generator. Service models are shipped in
small, build-time provider crates for the eight supported services:
DynamoDB, IAM, KMS, Lambda, S3, SNS, SQS, and STS.

For example, a consumer selects operations in `build.rs` through the service
crate and includes the one aggregate facade from its source:

    [dependencies]
    aws-sdk-builder = "0.1"

    [build-dependencies]
    aws-sdk-builder-s3 = "0.1"

    // build.rs
    fn main() -> Result<(), Box<dyn std::error::Error>> {
        aws_sdk_builder_s3::compile(["CreateBucket", "PutObject"])?;
        Ok(())
    }

    // src/lib.rs
    aws_sdk_builder::include_sdk!();

Each service crate packages exactly one `model.json`. An empty operation array
selects all operations, and repeated calls for the same service merge their
selections deterministically. The generated facade and service modules are
installed atomically below Cargo's `OUT_DIR`.

The generated facade exposes paths such as
consumer_crate_name::aws_sdk_s3::operation::abort_multipart_upload::AbortMultipartUpload.
Generated output is installed atomically, so a failed build cannot replace a
previous result.

Generated clients use the AWS runtime contract supplied by the downstream
consumer; `aws-sdk-builder` and the service provider crates remain codegen-only.

The pinned snapshot metadata and model checksums are in the core registry
(`crates/aws-sdk-builder/src/registry.rs`). The conformance harness is invoked
with:

    cargo run -p aws-sdk-conformance -- \
      --reference conformance/reference \
      --generated conformance/generated \
      --output conformance/summary.md \
      --snapshot 3c6d526c9d4775f41a8ef1ed2ef574d1b14481db

or use `just conformance`. The summary is `conformance/summary.md`; detailed
per-service reports are stored under `conformance/summary/`. The pinned reference
and generated source trees are checked in under `conformance/`.

For a local S3 emulator smoke test, start Floci yourself and run
scripts/check-s3-floci.sh. The launcher never starts or stops the emulator
and refuses non-loopback endpoints unless ALLOW_NONLOCAL_FLOCI=1 is set.

### Prompt

```
/goal
  Continue the Rust-native AWS SDK codegen parity project in this repository.

  Read Prompt.md and docs/aws-sdk-builder-status.md first. Continue from the current
  repository state. Keep codegen generic and driven by packaged Smithy JSON models.
  Use the pinned AWS SDK Rust and smithy-rs implementations as references. Refactor current codegen to follow smithy codegen design patterns. See `docs/smithy-codegen-design.md` and `docs/smithy-rs-reverse-engineering.md`.

  After every codegen change:
  1. Regenerate all-operation snapshots.
  2. Run conformance and verify the diff shrinks.
  3. Run tests, clippy, formatting, and git diff --check.
  4. Record a checkpoint in docs/aws-sdk-builder-status.md and commit the change.

  Do not stop at compilation or partial S3 support. Completion requires exact parity
  with conformance/reference and a passing conformance command.

  For now focus on s3 sdk only
```
