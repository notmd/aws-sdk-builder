# aws-sdk-build status and audit

Updated 2026-08-23. `Prompt.md` is the project specification. Superseded checkpoint
details are intentionally kept out of this working summary; git history preserves the
full audit trail.

### Checkpoint: 2026-08-23 — Preserve source operation order for type discovery

- State: in progress
- Changed: `crates/aws-sdk-build/src/model.rs` now retains the source service
  operation sequence separately from the caller's selected operation list, including
  a deterministic fallback for resource-attached operations. `codegen.rs` uses that
  model-derived sequence for breadth-first shared-type discovery, matching Smithy's
  `ListObjectsV2`/`ListObjectVersions` ordering without service-specific logic.
  Regenerated IAM, Lambda, and S3 all-operation snapshots and conformance reports.
- Evidence: inspected the pinned Smithy-RS model/type generation behavior under
  `/tmp/smithy-rs`. `just conformance` regenerated 8 snapshots and formatted 4,575
  generated Rust files; it exits 1 because parity remains incomplete. Workspace tests,
  Clippy with `-D warnings`, formatting, and `git diff --check` pass.
- Conformance: overall `3,533/1,041/1,887/1` -> `3,537/1,037/1,887/1`;
  S3 `1,172/85/87/0` -> `1,174/83/87/0` (matched/mismatched/missing/extra).
- Blocker: shared client/config/protocol/runtime source and reference package/test
  trees remain incomplete; no new blocker introduced by this checkpoint.
- Next action: fix the generic required-field documentation link rendering for raw
  identifiers, starting with the `GranteeBuilder::r#type` versus `GranteeBuilder::type`
  mismatch.

### Checkpoint: 2026-08-23 — Model-driven shared error metadata

- State: in progress
- Changed: `crates/aws-sdk-build/src/codegen.rs` now emits the standalone
  Smithy-RS-style `error.rs`, sealed `error/sealed_unhandled.rs`, service-level
  `error_meta.rs`, service error conversions, waiter/event-stream conversions, and
  request-ID forwarding from model traits. Consumer-namespace output keeps the
  existing legacy error module. Operation error symbols preserve Smithy-RS acronym
  runs such as `SAML`, `SMS`, and `ID`; service variants and conversion impls use the
  original Smithy-RS ordering rules.
- Evidence: inspected the pinned Smithy-RS `ServiceErrorGenerator.kt` and
  `OperationErrorGenerator.kt` under `/tmp/smithy-rs`. `just conformance` regenerated
  8 all-operation snapshots and formatted 4,575 generated Rust files. All eight
  generated service `error.rs`, `error_meta.rs`, and `error/sealed_unhandled.rs`
  files are byte-exact with the reference.
- Conformance: overall `3,509/1,049/1,903/1` -> `3,533/1,041/1,887/1`
  (matched/mismatched/missing/extra); IAM `756/342/533/0` -> `757/341/533/0`,
  KMS `229/166/204/1` -> `230/165/204/1`, Lambda `498/219/367/0` ->
  `499/218/367/0`, S3 `1,171/86/87/0` -> `1,172/85/87/0`, and SQS
  `136/48/115/0` -> `137/47/115/0`.
- Verification: `cargo test --workspace`, `cargo clippy --workspace --all-targets
  -- -D warnings`, `cargo fmt --all`, and `git diff --check` pass. The full
  conformance command exits 1 because broader source-tree parity remains incomplete.
- Next action: continue the generic parity loop with the remaining shared client,
  protocol, runtime, and test/package-tree gaps.

### Checkpoint: 2026-08-23 — Shared customization, presigning, config, and idempotency modules

- State: in progress
- Changed: `crates/aws-sdk-build/src/codegen.rs` now emits standalone Smithy-RS-style
  fluent operation builders, including input/output builder re-exports, `send_with`,
  customization/config hooks, paginators, collection/scalar setters and getters,
  model-derived documentation, streaming-aware derives, and presigning methods. The
  presigning predicate is derived from HTTP path/query bindings and streaming traits;
  it has no service- or operation-name switch. The service library header regression
  from the experiment was also corrected. The generic Smithy-RS client customization
  layer is now materialized as `client/customize.rs` plus `client/customize/internal.rs`
  for every selected service; S3’s presigning and payload-signing extensions are
  derived from the same model predicate. Smithy-RS presigning runtime assets,
  serialization settings, and the byte-identical shared `config/http`,
  `config/interceptors`, `config/retry`, `config/timeout`, and
  `sdk_feature_tracker` modules are now emitted generically. Idempotency provider
  and interceptor modules are emitted when the selected model contains an
  `smithy.api#idempotencyToken` member.
- Evidence: inspected the pinned Smithy-RS `FluentBuilderGenerator.kt` and
  `AwsPresigningDecorator.kt` under `/tmp/smithy-rs`. `just conformance` regenerated
  8 all-operation snapshots and formatted 4,494 generated Rust files.
- Conformance: overall `3,463/1,049/1,949/1` -> `3,503/1,049/1,909/1` ->
  `3,509/1,049/1,903/1` and S3 `1,162/86/96/0` -> `1,167/86/91/0`
  (matched/mismatched/missing/extra). The presigning runtime files and
  serialization settings are exact, as are all 40 common config/feature-tracker
  files and the six model-gated idempotency files.
- Blocker: the reference still contains 91 missing S3 files, primarily endpoint,
  checksum, S3 decorator, and test modules; remaining mismatches include shared
  client/config/protocol/type source and a small set of builder documentation/layout
  differences.
- Next action: port the model-driven shared error metadata support, starting with
  the service-independent error metadata helpers and their Smithy-RS source layout.

## Current implementation

- M1: complete for the public surface. Builder configuration is repeated `add` calls
  followed by `compile`; model, service, operations, `out_dir`, `smithy`, and
  `rust_client_codegen` are removed.
- M2: complete for the landed registry tier. Thirty-eight P0-P3 service model
  snapshots are packaged under `crates/aws-sdk-build/models`, with service shape IDs,
  crate/module mappings, canonical operation lists, SHA-256 checksums, and pinned
  snapshot SHAs in `models-manifest.json`.
- M3: in progress. The Rust generator emits deterministic service/config/client,
  operation, builder, error, shape, enum, and union source; owns shared shapes,
  builders, and modeled errors in Smithy-RS-style modules; resolves Smithy list/map
  shapes as inline collection expressions; emits model-derived pagination lenses;
  ports forward-compatible enums; and validates generated syntax with `syn`. It is
  not yet AWS SDK semantic parity.
- M4: in progress. Generated services co-locate the initial local HTTP transport in
  `src/client.rs` and declare `aws-runtime` as a downstream dependency. Full protocol
  serialization, runtime orchestration, endpoint resolution, auth/signing, retries,
  checksums, streaming, pagination, and service decorators remain incomplete.
- M5: in progress. `aws_sdk.rs`, consumer-prefixed service modules, Rust-only output,
  syntax validation, and atomic installation are implemented. The `my_aws_sdk`
  consumer fixture compiles.
- M6: in progress. The comparator runs against the pinned AWS SDK Rust `3c6d...` P0
  service trees and checks in deterministic summary and per-service reports. The
  current report compares 6,462 files: 3,533 exact, 1,041 mismatches, 1,887 missing,
  and 1 extra (51.14% arithmetic-average match).
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

### Checkpoint: 2026-08-23 — Generic Rest XML metadata and parser parity

- State: in progress
- Changed: synthetic operation shapes now preserve Smithy-RS `originalId` metadata;
  XML operation-output roots recover names from the original shape or synthetic
  operation identity; timestamp parsing and serialization honor all Smithy timestamp
  formats, including `http-date`; empty XML structures consume their decoder;
  modeled error `Message` members render first; and root-validation errors use the
  Smithy-RS parser template. These changes are model- and protocol-driven with no
  service- or operation-specific branches.
- Evidence: inspected the Smithy-RS model transform and `RestXmlParserGenerator`
  under `/tmp/smithy-rs`. `just conformance` regenerated 8 all-operation snapshots,
  formatted 4,494 generated Rust files, and exited 1 as expected because parity
  remains incomplete. The overall report is `2,948/1,545/1,968/1`; S3 is
  `1,055/188/101/0` (matched/mismatched/missing/extra), improving the previous
  checkpoint by 8 exact files overall and in S3. The one remaining root-validation
  mismatch is formatting-only: the pinned reference retains Smithy-RS template
  indentation while the local runner rustfmt-normalizes generated sources.
- Verification: `cargo test --workspace`, `cargo clippy --workspace --all-targets
  -- -D warnings`, `cargo fmt --all -- --check`, and `git diff --check` pass.
- Blocker: operation builder architecture, Rest XML helper ordering and temporary
  naming, endpoint/auth/retry/checksum, streaming, and remaining shared
  client/runtime source parity gaps remain open.
- Next action: continue generic shared protocol-helper ordering parity while
  preserving Smithy-RS shape-dependent lazy writer behavior.

### Checkpoint: 2026-08-23 — Generic long-operation and union rendering

- State: in progress
- Changed: `crates/aws-sdk-build/src/codegen.rs` now renders long-operation
  orchestration with Smithy-RS-compatible downcast layout and payload-reference
  spacing, and renders modeled unions generically with payload-carrying variants,
  documentation/deprecation attributes, forward-compatible `Unknown`, and
  `as_*`/`is_*` helpers. No service- or operation-specific branches were added.
- Evidence: inspected the Smithy-RS reference implementation in `/tmp/smithy-rs`.
  `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo fmt --all -- --check`, and `git diff --check` pass. `just conformance`
  regenerated the all-operation snapshots and exits 1 as expected because semantic
  parity remains incomplete.
- Conformance: overall `2,931/1,562/1,968/1` -> `2,940/1,553/1,968/1`;
  S3 `1,040/203/101/0` -> `1,047/196/101/0` (matched/mismatched/missing/extra).
- Blocker: operation builder architecture, Rest XML helper ordering and temporary
  naming, remaining sensitive/debug/documentation/streaming type differences, and
  shared client/runtime source remain incomplete.
- Next action: continue with the generic Rest XML protocol-helper parity slice,
  preserving Smithy-RS shape-dependent lazy writer ordering.

### Checkpoint: 2026-08-23 — Declarative Smithy HTTP protocol-test overlay

- State: in progress
- Changed: added `crates/aws-sdk-build/models/protocol-tests/s3.json`, attached
  it through the generic model registry, and added generic request/response
  protocol-test rendering in `crates/aws-sdk-build/src/codegen.rs`. Request tests
  cover fluent input construction, endpoint/region setup, query/header/body
  assertions, and URI checks. Response tests cover output and modeled-error
  deserialization, nested builders, timestamps, enums, and XML bodies. Error-shape
  tests are inherited by every selected operation that references the shape.
- Evidence: inspected the pinned Smithy-RS `ClientProtocolTestGenerator` and
  `ProtocolTestGenerator` under `/tmp/smithy-rs`. `cargo check -p aws-sdk-build`
  passes. `just conformance` regenerated 8 all-operation snapshots, formatted
  4,494 generated Rust files, and exited 1 as expected because parity remains
  incomplete. All ten previously missing S3 operation-root protocol-test blocks
  are exact after formatting. `cargo test --workspace`, `cargo clippy
  --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`, and
  `git diff --check` pass.
- Conformance: overall `2,921/1,572/1,968/1` -> `2,931/1,562/1,968/1`; S3
  `1,030/213/101/0` -> `1,040/203/101/0` (matched/mismatched/missing/extra).
- Blocker: three long-operation S3 roots still differ in Smithy-RS source layout;
  broader endpoint/auth/retry/checksum, protocol/runtime, presigning, waiter, and
  remaining source-tree parity gaps remain open.
- Next action: commit this verified checkpoint.

### Checkpoint: 2026-08-23 — Generic waiter rendering

- State: in progress
- Changed: `crates/aws-sdk-build/src/codegen.rs` now discovers waiters from
  `smithy.waiters#waitable`, emits Smithy-RS-compatible waiter roots, matcher
  modules, per-waiter fluent builders, model-derived documentation, acceptor
  states, output-path matchers, and waiter timing. Matcher rendering covers
  success, error-type, string/boolean output paths, list projections, and the
  packaged filtered-list expression without service-name branches. Waiter
  visitation follows Smithy-RS's operation and waiter-name ordering rules.
  Consumer-prefixed services now also expose a generated `Waiters` trait and
  lightweight waiter builders that reuse the existing operation runtime,
  including consumer-correct module paths and modeled error predicates.
- Evidence: inspected `/tmp/smithy-rs` waiter generators, including
  `WaitableGenerator.kt`, `WaiterAcceptorGenerator.kt`, and
  `RustWaiterMatcherGenerator.kt`. `cargo test --workspace`, `cargo clippy
  --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`, and
  `git diff --check` pass. `just conformance` regenerated 8 all-operation
  snapshots, formatted 4,494 generated Rust files, and exited 1 as expected
  because parity remains incomplete. Every newly materialized standalone waiter
  root, matcher, and per-waiter file is exact against the pinned reference.
- Conformance: overall `2,852/1,613/1,996/1` -> `2,880/1,613/1,968/1`; S3
  `941/296/107/0` -> `947/296/101/0` (matched/mismatched/missing/extra).
- Blocker: standalone generated client/runtime waiter integration, endpoint/
  auth/retry/checksum, protocol, and the remaining reference source tree are
  incomplete; the full conformance command still exits 1. Consumer waiters use
  the repository's lightweight runtime until the shared Smithy-RS runtime is
  ported.
- Next action: port the generic standalone client-side `Waiters` trait and
  shared waiter runtime integration, then rerun the full comparison.

### Checkpoint: 2026-08-22 — Resource-bound operation discovery

- State: in progress
- Changed: `crates/aws-sdk-build/src/model.rs` now discovers operations by walking
  the service's directed shape closure, matching Smithy's `TopDownIndex` behavior.
  This includes operations attached to resources when a service intentionally omits
  them from its explicit `operations` array, while preserving explicit service
  operations and keeping the selection generic across services.
- Evidence: inspected the local `/tmp/smithy-rs` mirror, including the client and
  server generators' `TopDownIndex.getContainedOperations` usage. The packaged
  models select 568 operations across 8 services, including all 88 Lambda operations.
  The focused packaged-model closure test passes. `just conformance` regenerated 8
  all-operation snapshots, formatted 4,466 generated Rust files, and exited 1 as
  expected because parity remains incomplete.
- Conformance: overall `2,654/1,436/2,371/123` -> `2,852/1,613/1,996/1`;
  Lambda `232/91/761/122` -> `436/262/386/0` (matched/mismatched/missing/extra).
- Blocker: protocol/runtime, endpoint/auth/retry/checksum, presigning, waiter,
  test/package-tree, and remaining generated-source parity are still incomplete.
- Next action: continue the generic parity loop with the missing shared runtime and
  protocol source tree, prioritizing reusable Smithy-RS ownership boundaries.

### Checkpoint: 2026-08-22 — Model-derived pagination lenses

- State: in progress
- Changed: crates/aws-sdk-build/src/codegen.rs now indexes
  `smithy.api#paginated` paths generically, validates input/output/page-size member
  paths, sorts paginator lens functions by Smithy operation symbol, and emits borrowed
  token and owned flattened-item accessors in `src/lens.rs`. Member requiredness
  determines whether nested access uses direct moves or optional traversal. Standalone
  and consumer namespace paths are rendered separately; the consumer lens module has
  a narrowly scoped Clippy compatibility allowance. The S3 lens snapshot is now
  byte-exact with the pinned reference.
- Evidence: inspected `/tmp/smithy-rs` `PaginatorGenerator.kt` and
  `NestedAccessorGenerator.kt`. `just conformance` regenerated 8 all-operation
  snapshots (496 operations), formatted 4,148 generated Rust files, and exited 1 as
  expected because parity remains incomplete. `cargo test --workspace`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo fmt --all -- --check`, and `git diff --check` pass.
- Conformance: overall `2,584/1,435/2,442/123` -> `2,590/1,435/2,436/123`;
  S3 `937/294/113/0` -> `938/294/112/0` (matched/mismatched/missing/extra).
  The final lens files for DynamoDB, IAM, KMS, S3, SNS, and SQS are byte-exact.
- Blocker: the conformance command is still non-zero; paginator modules and the
  fluent `into_paginator` API, protocol/runtime, endpoint/auth/retry/checksum,
  presigning, waiter generation, and the missing reference test/package tree remain
  incomplete.
- Next action: port the generic Smithy-RS paginator module and fluent builder hook
  using the same pagination index, then rerun the full comparison.

### Checkpoint: 2026-08-22 — Smithy-RS shared type ownership

- State: in progress
- Changed: crates/aws-sdk-build/src/codegen.rs now emits a `types` facade with
  model-ordered public re-exports and sorted physical shape modules, plus separate
  `types/builders.rs` and `types/error.rs` files. Modeled errors and event-stream
  errors use the shared error module, service titles drive the service error docs,
  operation error roots participate in first-discovery type ordering, and consumer
  namespace output includes physical modules inside the generated `types` module.
  Primitive aliases were removed from the shared facade. The generator regression
  test now checks the new shared files and module output.
- Evidence: inspected the Smithy-RS mirror at `/tmp/smithy-rs` (`56ee88c`), including
  `CodegenDelegator.kt`, `ClientRustModule.kt`, and `BuilderGenerator.kt`. The S3
  `src/types.rs`, `src/types/builders.rs`, `src/types/error.rs`, and
  `src/types/error/builders.rs` snapshots are byte-exact with the pinned reference;
  the `my_aws_sdk` consumer fixture compiles. `just conformance` regenerated 8
  all-operation snapshots (496 operations), formatted the generated Rust, and exited
  1 as expected because parity remains incomplete. The final report compares 6,584
  files: 2,584 matched, 1,435 mismatched, 2,442 missing, 123 extra, and 0 read
  errors.
- Conformance: overall `2,555/1,446/2,460/123` -> `2,584/1,435/2,442/123`;
  S3 `934/295/115/0` -> `937/294/113/0` (matched/mismatched/missing/extra).
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo fmt --all -- --check`, and `git diff --check` pass.
- Blocker: the conformance command is still non-zero; protocol/runtime,
  endpoint/auth/retry/checksum/pagination/waiter generation, and the missing
  reference test/package tree remain incomplete.
- Next action: continue the conformance mismatch loop with the next highest-impact
  Smithy-RS module or protocol ownership rule, then rerun the full comparison.

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
