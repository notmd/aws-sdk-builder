# aws-sdk-build status and audit

Updated 2026-08-21. Prompt.md is the project specification.

## Current implementation

- M1: complete for the public surface. Builder configuration is repeated add
  calls followed by compile; model, service, operations, out_dir, smithy, and
  rust_client_codegen are removed.
- M2: complete for the landed registry tier. Thirty-eight P0-P3 service model
  snapshots are packaged under crates/aws-sdk-build/models, with service shape
  IDs, crate/module mappings, source/canonical operation lists, SHA-256
  checksums, and pinned snapshot SHAs in models-manifest.json.
- M3: in progress. The Rust generator emits deterministic service/config/client,
  operation, builder, error, shape, list, map, enum, and union source and
  validates generated syntax with syn. It is not yet AWS SDK semantic parity.
- M4: not complete. Protocol serialization, runtime orchestration, endpoint
  resolution, auth/signing, retries, checksums, streaming, pagination, and
  service decorators remain to be ported.
- M5: in progress. aws_sdk.rs, consumer-prefixed service modules, output
  manifests, syntax validation, and atomic installation are implemented. The
  clean generated-consumer cargo check passes.
- M6: the comparator has now run against the pinned AWS SDK Rust `3c6d...` P0
  service trees and the deterministic summary plus per-service results are checked in
  under `conformance/summary.md` and `conformance/summary/`. The
  report is intentionally nonconformant: the current generator has a different source
  layout and no files matched. Both comparison trees are checked in under
  `conformance/` and described by `conformance/manifest.json`.
- M6a: launcher and Rust Floci example are implemented. A live emulator result
  is not recorded in this audit.
- M7: not complete; semantic parity gates for the priority queue remain open.
- M8: stale Smithy CLI implementation/docs were removed and the user README was
  rewritten. A full audit remains open until the gates below pass.

## Evidence

Passing checks:

- cargo fmt --all
- cargo check -p aws-sdk-build --lib
- cargo test -p aws-sdk-build --lib --tests
- cargo test --workspace
- cargo clippy --workspace --all-targets -- -D warnings
- cargo check --manifest-path examples/generated-consumer/Cargo.toml
- cargo check --manifest-path examples/floci-s3-smoke/Cargo.toml
- all-operation S3 generated consumer compile;
- selected S3/DynamoDB consumer compile with unselected S3 operations absent;
- repeated consumer output hashes are identical;
- cargo package -p aws-sdk-build --allow-dirty --no-verify includes 38 model assets
  and the registry manifest;
- scripts/check-s3-floci.sh passes shell syntax, explicit skip behavior, and
  non-loopback endpoint refusal;
- git diff --check after the final working-tree changes.

Unavailable or not yet passing:

- a passing full pinned AWS SDK source/token conformance comparison; the current
  checked-in report records the non-zero result;
- clean consumer compile against a semantically complete generated AWS client;
- unchanged-output failure test through a failed generation after every validation
  stage;
- live Floci operation sequence;

The project must not claim completion until those missing gates have current,
reproducible evidence.
