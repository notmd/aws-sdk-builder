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
  current report compares 6,584 files: 2,555 exact, 1,446 mismatches, 2,460 missing,
  and 123 extra (35.93% arithmetic-average match).
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

Concrete reverse-engineering notes now live in
[`docs/smithy-rs-reverse-engineering.md`](smithy-rs-reverse-engineering.md). They record
current upstream visitor order, normalization invariants, lazy writer/dependency
behavior, protocol helper ownership, decorator composition, runtime boundaries, and
known Rust-port migration targets. The notes distinguish the parity pin from the
inspection mirror at `/tmp/smithy-rs` so newer upstream behavior does not silently
change conformance inputs.

## Evidence

### Checkpoint: 2026-08-22 — Generic operation normalization and shared-shape closure

- State: in progress
- Changed: crates/aws-sdk-build/src/model.rs now ports Smithy-RS operation
  normalization: selected operations point to synthetic
  namespace.synthetic#OperationInput/Output structures, smithy.api#Unit becomes
  an empty structure, original modeled structures are retained only when reachable
  from the rewritten service graph, and conflicting/non-structure synthetic shapes
  fail with a packaged-model diagnostic. crates/aws-sdk-build/src/codegen.rs
  recognizes synthetic I/O traits when deciding which shapes get standalone files
  and preserves the normalized namespace in Rest XML parser metadata. Added a
  regression test for shared S3 NotificationConfiguration.
- Evidence: inspected /tmp/smithy-rs at 56ee88c5c6edd967967656f1e29f46b229105e79,
  including OperationNormalizer.kt. just conformance regenerated 8 all-operation
  snapshots (496 operations), formatted 4,126 generated Rust files, and exited 1
  as expected because parity remains incomplete. Final report compares 6,584 files:
  2,564 matched, 1,439 mismatched, 2,458 missing, 123 extra, 0 read errors.
  cargo fmt --all -- --check, cargo test --workspace,
  cargo clippy --workspace --all-targets -- -D warnings, and git diff --check pass.
- Conformance: overall 2,555/1,446/2,460/123 -> 2,564/1,439/2,458/123;
  S3 933/295/116/0 -> 934/295/115/0 (matched/mismatched/missing/extra).
  The newly generated src/types/_notification_configuration.rs is byte-exact
  with the pinned S3 reference.
- Blocker: the conformance command is still non-zero; shared types.rs module
  ownership/builders/error exports, endpoint/runtime/auth/checksum/pagination/
  waiter generation, and the reference test/package tree remain incomplete.
- Next action: port Smithy writer-style shared type module ownership so generated
  src/types.rs, src/types/builders.rs, and src/types/error.rs match the reference
  tree, then rerun the all-operation S3-focused conformance comparison.

### Checkpoint: 2026-08-22 — External Smithy API reverse-engineering

- State: in progress
- Changed: expanded `docs/smithy-rs-reverse-engineering.md` with behavior from external
  Smithy Java APIs: model blackboard/index caching, assembler validation and trait
  discovery, immutable model transforms, HTTP binding inference, nullability modes,
  operation/service/pagination indexes, symbol/writer dependency semantics, ordered
  endpoint rules, AWS trait defaults, protocol-test fixtures, and waiter defaults.
- Evidence: inspected Smithy Java source mirror `/tmp/smithy` at commit
  `0f7323128b0606a1b94b1ac482c94d3800a22708`, Maven artifact POMs at version `1.73.0`,
  and Smithy Rust consumers of those APIs. Runtime crates intentionally excluded from
  this dependency audit.
- Conformance: not rerun; documentation-only checkpoint, generated source unchanged.
- Blocker: Rust port still lacks typed model/index abstractions matching these contracts.
- Next action: implement model index and HTTP binding layers, then rerun all-operation
  conformance before adding protocol-specific exceptions.

### Checkpoint: 2026-08-22 — Smithy Rust reverse-engineering notes

- State: in progress
- Changed: added `docs/smithy-rs-reverse-engineering.md` with exact upstream client
  visitor ordering, baseline transforms, synthetic input/output invariants, directed
  closure, symbol/module ownership, lazy `RustCrate` dependency behavior, protocol
  helper placement, Rest XML root validation, decorator ordering, runtime boundaries,
  current Rust-port mapping, and mismatch-debugging workflow. Linked notes from the
  README and status document.
- Evidence: inspected `/tmp/smithy-rs` release `1.1.7` at commit
  `56ee88c5c6edd967967656f1e29f46b229105e79`, including `ClientCodegenVisitor.kt`,
  `OperationNormalizer.kt`, `CodegenDelegator.kt`, `ProtocolFunctions.kt`,
  `OperationGenerator.kt`, `CoreCodegenDecorator.kt`, `AwsCodegenDecorator.kt`, and
  S3's `S3Decorator.kt`. This is an inspection mirror; parity remains pinned to
  `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`.
- Conformance: not rerun; documentation-only checkpoint, generated source unchanged.
- Blocker: codegen parity remains incomplete; notes identify migration targets but do not
  change generator behavior.
- Next action: use source map and mismatch loop to port next reusable Smithy rule, then
  run full conformance and record before/after counts.

### Checkpoint: 2026-08-22 — Rest XML parser and correction parity

- State: in progress
- Changed: Rest XML operation output parsers, nested structure/union/list/map
  helpers, flattened-list handling, modeled S3 invalid-root exceptions, Java
  `HashMap` member ordering, request/header serialization helpers, and Smithy-style
  `serde_util` correction generation are now emitted. Serializer/deserializer role
  discovery follows the operation walk instead of a global serializer prepass.
- Evidence: inspected the pinned Smithy reference at `/tmp/smithy-rs`, including
  `RestXmlParserGenerator.kt`, `ProtocolFunctions.kt`, `OperationGenerator.kt`, and
  S3's `S3Decorator.kt`. `just conformance` regenerated 8 all-operation snapshots
  (496 operations), formatted 4,124 Rust files, compared 6,584 files, and exited 1
  because parity remains incomplete. `cargo check -p aws-sdk-build` passes.
- Conformance: overall `2,551/1,451/2,460/123` -> `2,555/1,446/2,460/123`; S3
  `930/310/116/0` -> `933/295/116/0` (matched/mismatched/missing/extra).
- Blocker: remaining helper ordering, modeled correction ordering, and the broader
  protocol/runtime parity queue remain open; the full conformance command still
  exits 1.
- Next action: align the remaining Smithy lazy helper and correction discovery order,
  then rerun conformance and retain the higher-coverage checkpoint.

### Checkpoint: 2026-08-22 — Fallible operation-input builders

- State: in progress
- Changed: `crates/aws-sdk-build/src/codegen.rs` ports the smithy-rs
  `BuilderGenerator` rule that every operation input builder is fallible, while
  keeping the model-derived required-field documentation conditional. Operation
  inputs no longer derive `Default`; the local fluent builder owns the generated
  input builder and materializes it before request serialization. Regenerated 992
  operation input/builder snapshots and all deterministic reports.
- Evidence: compared the pinned smithy-rs `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`
  `BuilderGenerator` behavior. `just conformance` regenerated 8 all-operation
  snapshots (496 operations), formatted 3,910 Rust files, compared 6,584 files,
  and exited 1 because parity remains incomplete. `cargo test --workspace`,
  `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`,
  and `git diff --check` pass.
- Conformance: overall `1,685/2,102/2,674/123` -> `2,055/1,732/2,674/123`
  (matched/mismatched/missing/extra); S3 `576/445/323/0` -> `657/364/323/0`.
  S3 operation-input files are now exact for 81 of 112 operations.
- Blocker: modeled errors, remaining protocol/runtime behavior, endpoint/auth/retry/
  checksum support, pagination, and the missing reference source tree remain
  incomplete; the full conformance command still exits 1.
- Next action: align the remaining streaming operation-input derive metadata from
  Smithy streaming traits and verify the 31 remaining S3 input mismatches.

### Checkpoint: 2026-08-22 — Operation input parity and malformed documentation

- State: in progress
- Changed: `crates/aws-sdk-build/src/codegen.rs` now preserves Smithy operation
  symbols in generated public aliases and operation errors, makes operation input
  builders fallible with required-field validation, emits required-field builder
  documentation, and normalizes malformed model HTML with Smithy-compatible
  pseudo-tag closure and formatting rules.
- Evidence: `just conformance` regenerated 8 all-operation snapshots (496
  operations), formatted 3,910 Rust files, compared 6,584 files, and exited 1
  because semantic parity remains incomplete. The four remaining S3 operation-input
  documentation diffs from the prior run are now exact.
- Conformance: `2,055/1,732/2,674/123` -> `2,331/1,456/2,674/123`
  overall and `657/364/323/0` -> `710/311/323/0` for S3
  (matched/mismatched/missing/extra). `cargo test --workspace`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo fmt --all -- --check`, and `git diff --check` pass.
- Blocker: modeled errors, remaining protocol/runtime behavior, endpoint/auth/retry/
  checksum support, pagination, and the missing reference source tree remain
  incomplete; the full conformance command still exits 1.

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
