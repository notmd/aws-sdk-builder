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
  current report compares 6,584 files: 1,685 exact, 2,102 mismatches, 2,674 missing,
  and 123 extra (24.00% arithmetic-average match).
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

### Checkpoint: 2026-08-22 — RestXml payload wrapper module alignment

- State: in progress
- Changed: `crates/aws-sdk-build/src/codegen.rs` now derives the structured RestXml
  payload wrapper’s self-call from `shape_<operation>_input`; nested model serializers
  continue to target `shape_<model>`. Added a focused regression test and regenerated
  34 affected S3 protocol snapshots plus deterministic conformance reports.
- Evidence: inspected the pinned smithy-rs checkout at
  `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`. `just conformance` regenerated 8
  all-operation snapshots (496 operations), formatted 3,910 Rust files, compared
  6,584 files, and exited 1 because parity remains incomplete. `cargo test
  --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo fmt --all -- --check`, and `git diff --check` pass.
- Conformance: overall `1,651/2,136/2,674/123` -> `1,685/2,102/2,674/123`
  (matched/mismatched/missing/extra); S3 `542/479/323/0` -> `576/445/323/0`.
- Blocker: modeled errors, remaining protocol/runtime behavior, endpoint/auth/retry/
  checksum support, pagination, and the missing reference source tree remain
  incomplete; the full conformance command still exits 1.
- Next action: inspect the next largest S3 mismatch category in
  `conformance/summary/s3.md` and port one reusable model/protocol rule.

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
