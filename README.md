# aws-sdk-builder

`aws-sdk-builder` is the core Rust-only generator. Service models are shipped in
small, build-time provider crates for the fifteen supported services:
Batch, Bedrock Runtime, CloudWatch Logs, CodeArtifact, Cognito Identity Provider,
Config, DynamoDB, IAM, KMS, Lambda, S3, SES v2, SNS, SQS, and STS.

For example, a consumer selects operations in `build.rs` through the service
crate and includes that service inside a wrapper module:

    [dependencies]
    aws-sdk-builder-s3 = "0.1"

    [build-dependencies]
    aws-sdk-builder-s3 = "0.1"

    // build.rs
    fn main() -> Result<(), Box<dyn std::error::Error>> {
        aws_sdk_builder_s3::compile(["CreateBucket", "PutObject"])?;
        Ok(())
    }

    // src/lib.rs
    mod aws_s3_sdk {
        aws_sdk_builder_s3::include_sdk!();
    }

Each service crate packages exactly one `model.json`. An empty operation array
selects all operations, and repeated calls for the same service merge their
selections deterministically. Generated service modules are installed
atomically below Cargo's `OUT_DIR`.

The wrapper exposes paths such as
`aws_s3_sdk::operation::abort_multipart_upload::AbortMultipartUpload`.
Generated output is installed atomically, so a failed build cannot replace a
previous result.

Generated clients use the AWS runtime contract supplied by the downstream
consumer; `aws-sdk-builder` and the service provider crates remain codegen-only.

Pinned upstream repository, commit, selected services, model paths, roots, and
comparison exclusions live in [`services-manifest.json`](services-manifest.json).
Refresh reference source and every provider `model.json` with:

    just conformance-sync

Run conformance with:

    just conformance

The summary is `conformance/summary.md`; detailed per-service reports are stored under
`conformance/summary/`. Reference and generated source trees are checked in under
`conformance/`; Cargo metadata, README files, LICENSE files, tests, and benches are
excluded from both comparison inputs.

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

```
/goal
  Refactor the AWS SDK codegen so consumer and conformance use one canonical generated
  implementation, while preserving the current conformance coverage exactly.

  First read Prompt.md, docs/aws-sdk-builder-status.md, git status, and
  conformance/summary.md. Record the current baseline before editing. The current
  baseline is approximately:

    4904 matched / 6396 compared
    724 mismatched, 713 missing, 55 extra

  Generate one file per service:

    OUT_DIR/generated/<service>/original.rs

  The provider's include_sdk!() macro must include that service's original.rs.
  Conformance must preserve the same file at:

    conformance/generated/<service>/original.rs

  Remove consumer_namespace, consumer_crate, and relative_snapshot_paths from the
  rendering pipeline. Do not introduce another consumer/conformance mode flag.
  Generated paths must be resolved from the Rust module tree so original.rs works at
  crate root and inside any caller-owned wrapper.

  Derive the comparison tree from each original.rs:

    conformance/generated/<service>/normalized/src/**

  Use a deterministic syn-based or shared module-tree transform. Preserve module paths,
  visibility, attributes, docs, item order, macro scope, and relative paths. The
  normalized tree is only a projection for comparison, not a second codegen mode.

  This task is architecture migration only. Do not expand protocol support, fix
  unrelated parity gaps, add services, or change generated semantics.

  Acceptance criteria:

  1. Consumer and conformance use the same per-service original.rs.
  2. No consumer/conformance renderer branches remain.
  3. Splitting original.rs is deterministic and semantics-preserving.
  4. The post-migration conformance report matches the recorded baseline:
     no fewer exact matches and no increase in mismatched, missing, or extra files.
  5. Existing consumer and conformance tests continue to pass.

  Run after the migration:

    cargo fmt --all
    just conformance
    cargo test --workspace
    cargo clippy --workspace --all-targets -- -D warnings
    git diff --check

  `just conformance` may continue to exit 1 because the baseline is not fully
  conformant. Do not require full parity for this goal. Record the before/after counts
  and any changed generated files in docs/aws-sdk-builder-status.md.
```
