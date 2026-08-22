# aws-sdk-build status and audit

Updated 2026-08-22. `Prompt.md` is the project specification. Superseded checkpoint
details are intentionally kept out of this working summary; git history preserves the
full audit trail.

## Current implementation

- M1: complete for the public surface. Builder configuration is repeated `add` calls
  followed by `compile`; model, service, operations, `out_dir`, `smithy`, and
  `rust_client_codegen` are removed.
- M2: complete for the landed registry tier. Thirty-eight P0-P3 service model
  snapshots are packaged under `crates/aws-sdk-build/models`, with service shape IDs,
  crate/module mappings, canonical operation lists, SHA-256 checksums, and pinned
  snapshot SHAs in `models-manifest.json`.
- M3: in progress. The Rust generator emits deterministic service/config/client,
  operation, builder, error, shape, enum, and union source; resolves Smithy list/map
  shapes as inline collection expressions; ports forward-compatible enums; and
  validates generated syntax with `syn`. It is not yet AWS SDK semantic parity.
- M4: in progress. Generated services co-locate the initial local HTTP transport in
  `src/client.rs` and declare `aws-runtime` as a downstream dependency. Full protocol
  serialization, runtime orchestration, endpoint resolution, auth/signing, retries,
  checksums, streaming, pagination, and service decorators remain incomplete.
- M5: in progress. `aws_sdk.rs`, consumer-prefixed service modules, Rust-only output,
  syntax validation, and atomic installation are implemented. The `my_aws_sdk`
  consumer fixture compiles.
- M6: in progress. The comparator runs against the pinned AWS SDK Rust `3c6d...` P0
  service trees and checks in deterministic summary and per-service reports. The
  current report compares 6,584 files: 1,651 exact, 2,136 mismatches, 2,674 missing,
  and 123 extra (23.68% arithmetic-average match).
- M6a: launcher and Rust Floci example are implemented; the local S3 create/head
  smoke test passes against `http://localhost:4566`.
- M7: not complete; semantic parity gates for the priority queue remain open.
- M8: stale Smithy CLI implementation/docs were removed and the user README was
  rewritten. A full audit remains open until the gates below pass.

## Reusable design reference

The distilled generic Smithy/Rust codegen architecture, reference source map, model
transforms, closure rules, protocol abstractions, decorator model, and mismatch
diagnosis workflow are documented in
[`docs/smithy-codegen-design.md`](smithy-codegen-design.md). It is derived from the
pinned `smithy-rs` commit
`f1b64a9c0dd001d4bac4277fec4041da59c1f48d` and should be updated when the port adopts
a new reusable abstraction.

## Evidence

### Checkpoint: 2026-08-22 — RestXml payload wrappers and conformance formatting

- State: in progress
- Changed: `crates/aws-sdk-build/src/codegen.rs` emits generic model-driven RestXml
  payload input wrappers for streaming blobs, raw string/blob payloads, and
  structure/union XML payloads, deriving payload roots and namespaces from model
  traits. `aws-sdk-build` no longer invokes `rustfmt` or writes a generated manifest;
  `aws-sdk-conformance` formats generated `.rs` files immediately before comparison.
- Formatting reference: pinned Smithy-RS `ClientCodegenVisitor.kt` finalizes generated
  crates with `cargo fmt -- --config max_width=150`. The checkout has no
  client-generator `.rustfmt.toml`; its unrelated HTTP-server configs use
  `max_width = 120`. Conformance uses
  `rustfmt --edition 2021 --config max_width=150,skip_children=true` per snapshot
  file because snapshots have no temporary Cargo manifest.
- Evidence: `just conformance` regenerated 8 all-operation snapshots (496 operations),
  formatted 3,910 Rust files, compared 6,584 files, and exited 1 because parity
  remains incomplete. Exact coverage is 1,651 overall and 542 for S3; the S3 report
  has 479 mismatches, 323 missing, and 0 extra files.
- Blocker: the payload helper self-call for structure-valued wrappers still needs to
  target `shape_<operation>_input`, while nested serializers target the model shape
  module. Modeled errors, remaining protocol/runtime behavior, endpoint/auth/retry/
  checksum support, pagination, and the missing source tree also remain incomplete.
- Next action: fix the generic payload-wrapper module path and rerun conformance,
  keeping the checkpoint only if exact coverage increases.

## Passing checks

- `cargo fmt --all -- --check`
- `cargo test -p aws-sdk-build`
- `cargo test -p aws-sdk-conformance`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo check -p aws-sdk-build`
- `cargo check --manifest-path examples/my_aws_sdk/Cargo.toml`
- `cargo check --manifest-path examples/floci-s3-smoke/Cargo.toml`
- checked-in all-operation conformance source snapshot
- `cargo package -p aws-sdk-build --allow-dirty --no-verify` includes all model assets
- `scripts/check-s3-floci.sh` passes syntax, skip, and non-loopback endpoint checks
- `git diff --check`

## Unavailable or not yet passing

- a passing full pinned AWS SDK source/token conformance comparison; the current
  report intentionally remains non-zero;
- clean consumer compile against a semantically complete generated AWS client;
- unchanged-output failure test through every validation stage;
- a live Floci operation sequence beyond the current smoke test.

The project must not claim completion until those missing gates have current,
reproducible evidence.
