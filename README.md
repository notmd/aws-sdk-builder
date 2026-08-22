# aws-sdk-build

aws-sdk-build is a Rust-only build dependency for generating a selected,
consumer-prefixed AWS SDK module from packaged Smithy JSON snapshots.

    [dependencies]
    aws-sdk-build = "0.1"

    [build-dependencies]
    aws-sdk-build = "0.1"

    // build.rs
    fn main() -> Result<(), Box<dyn std::error::Error>> {
        aws_sdk_build::configure()
            .add("s3", ["AbortMultipartUpload", "CompleteMultipartUpload"])
            .add("dynamodb", ["GetItem"])
            .compile()?;
        Ok(())
    }

    // src/lib.rs
    aws_sdk_build::include_sdk!();

compile() discovers OUT_DIR and CARGO_PKG_NAME from Cargo. Service models are
immutable packaged assets selected by short service key; consumers do not
provide model paths, shape IDs, output directories, Smithy executables, or
codegen plugins. Empty operation iterators select all operations. Repeated
service entries are merged deterministically.

The generated facade exposes paths such as
consumer_crate_name::aws_sdk_s3::operation::abort_multipart_upload::AbortMultipartUpload.
Generated output includes a deterministic manifest and is installed atomically,
so a failed build cannot replace a previous result.

Generated clients use the AWS runtime contract supplied by the downstream
consumer. The generated manifest records `aws-runtime` as the required runtime
crate; `aws-sdk-build` itself remains codegen-only.

The pinned snapshot metadata and model checksums are in
crates/aws-sdk-build/models-manifest.json. The conformance harness is invoked
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

  Read Prompt.md and docs/aws-sdk-build-status.md first. Continue from the current
  repository state. Keep codegen generic and driven by packaged Smithy JSON models.
  Use the pinned AWS SDK Rust and smithy-rs implementations as references. Refactor current codegen to follow smithy codegen design patterns. See `docs/smithy-codegen-design.md` and `docs/smithy-rs-reverse-engineering.md`.

  After every codegen change:
  1. Regenerate all-operation snapshots.
  2. Run conformance and verify the diff shrinks.
  3. Run tests, clippy, formatting, and git diff --check.
  4. Record a checkpoint in docs/aws-sdk-build-status.md and commit the change.

  Do not stop at compilation or partial S3 support. Completion requires exact parity
  with conformance/reference and a passing conformance command.

  For now focus on s3 sdk only
```
