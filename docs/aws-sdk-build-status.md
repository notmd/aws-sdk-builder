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
- M4: in progress. Generated services now always include `src/aws_runtime.rs`
  with the initial local HTTP transport and declare `aws-runtime` as a normal
  downstream dependency for AWS runtime metadata. `aws-sdk-build` remains
  codegen-only; the generated client no longer references `reqwest`. Full
  protocol serialization, runtime orchestration, endpoint resolution,
  auth/signing, retries, checksums, streaming, pagination, and service
  decorators remain to be ported.
- M5: in progress. aws_sdk.rs, consumer-prefixed service modules, output
  manifests, syntax validation, and atomic installation are implemented. The
  `my_aws_sdk` consumer fixture compiles, and the checked-in generated source snapshot
  remains available under `conformance/generated`.
- M6: the comparator has now run against the pinned AWS SDK Rust `3c6d...` P0
  service trees and the deterministic summary plus per-service results are checked in
  under `conformance/summary.md` and `conformance/summary/`. The current report
  compares 8,780 files and has 16 exact matches (0.30% arithmetic average),
  while remaining intentionally nonconformant. Both comparison trees are checked
  in under `conformance/` and described by `conformance/manifest.json`.
- M6a: launcher and Rust Floci example are implemented. The live
  `my_aws_sdk::tests::creates_then_heads_a_bucket` test passes against the
  developer's local `http://localhost:4566` emulator.
- M7: not complete; semantic parity gates for the priority queue remain open.
- M8: stale Smithy CLI implementation/docs were removed and the user README was
  rewritten. A full audit remains open until the gates below pass.

## Reusable design reference

The distilled generic Smithy/Rust codegen architecture, reference source map, model
transforms, closure rules, protocol abstractions, decorator model, and mismatch
diagnosis workflow are documented in
[`docs/smithy-codegen-design.md`](smithy-codegen-design.md). It is derived from the
pinned `smithy-rs` commit `f1b64a9c0dd001d4bac4277fec4041da59c1f48d` and should be
updated when the port adopts a new reusable abstraction.

## Evidence

Passing checks:

- cargo fmt --all
- cargo check -p aws-sdk-build --lib
- cargo test -p aws-sdk-build --lib --tests
- cargo test --workspace
- cargo clippy --workspace --all-targets -- -D warnings
- cargo check --manifest-path examples/my_aws_sdk/Cargo.toml
- `AWS_ENDPOINT_URL=http://localhost:4566 cargo test --manifest-path
  examples/my_aws_sdk/Cargo.toml creates_then_heads_a_bucket` (Floci live
  test);
- cargo check --manifest-path examples/floci-s3-smoke/Cargo.toml
- checked-in all-operation conformance source snapshot;
- cargo package -p aws-sdk-build --allow-dirty --no-verify includes 38 model assets
  and the registry manifest;
- scripts/check-s3-floci.sh passes shell syntax, explicit skip behavior, and
  non-loopback endpoint refusal;
- git diff --check after the final working-tree changes.

Unavailable or not yet passing:

- a passing full pinned AWS SDK source/token conformance comparison; the current
  checked-in report records 16 exact matches and the remaining non-zero result;
- clean consumer compile against a semantically complete generated AWS client;
- unchanged-output failure test through a failed generation after every validation
  stage;
- live Floci operation sequence;

The project must not claim completion until those missing gates have current,
reproducible evidence.
