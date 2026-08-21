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
  operation, builder, error, shape, enum, and union source, resolves Smithy
  list/map shapes as inline `Vec<T>`/`BTreeMap<K, V>` expressions, ports the
  Smithy forward-compatible enum wrapper and helpers, and validates generated
  syntax with syn. It is not yet AWS SDK semantic parity.
- M4: in progress. Generated services now co-locate the initial local HTTP
  transport with `src/client.rs` and declare `aws-runtime` as a normal downstream
  dependency for AWS runtime metadata. `aws-sdk-build` remains codegen-only; the generated
  client no longer references `reqwest`. Full
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
  compares 6,584 files and has 483 exact matches (6.75% arithmetic average),
  with 3,091 mismatches, 2,887 missing files, and 123 extra files. Both comparison
  trees are checked in under `conformance/` and described by `conformance/manifest.json`.
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

### Checkpoint: 2026-08-21 — Client operation documentation parity

- State: in progress
- Changed: `crates/aws-sdk-build/src/codegen.rs` now renders client operation
  documentation from the selected Smithy model, including model-driven primitive,
  collection, and named-shape types, target-shape documentation fallbacks,
  required-member reporting, output fields, paginated-operation links, and
  Smithy-style HTML whitespace. Long client method signatures use generic rendered
  length-based wrapping, and nested list closing gaps preserve serializer spacing.
- Evidence: `cargo fmt --all`, `cargo test -p aws-sdk-build`, `git diff --check`,
  and `just conformance` completed. Conformance intentionally exits 1 because
  semantic parity remains incomplete.
- Conformance: the previous checkpoint had `400` exact matches; this checkpoint
  has `483` exact matches out of `6,584`, with `3,091` mismatches, `2,887` missing,
  `123` extra, and `6.75%` average match. The increase is primarily in client
  operation documentation and signature formatting.
- Blocker: request-ID fields, protocol/runtime behavior, endpoint/auth/retry/checksum
  support, and the remaining missing source tree are still incomplete.
- Next action: port request-ID fields and the next shared operation/protocol source
  boundary from the pinned Smithy reference.

### Checkpoint: 2026-08-21 — Generic operation builder and streaming semantics

- State: in progress
- Changed: `crates/aws-sdk-build/src/codegen.rs` now keeps operation input value
  members optional while preserving bare streaming `ByteStream` fields, derives
  `Default` for consumer operation inputs used by the fluent wrapper, re-exports
  generated `BuildError` through the consumer `error` module, and uses output
  builders for response decoding. Required-member validation is model-driven:
  structure-valued required members remain optional, while scalar/enum/collection
  required members retain bare fields and builder validation. XML nested structure
  decoding now uses generated builders instead of requiring every model structure to
  implement `Default`. The generator was also made warning-free under Clippy.
- Evidence: `cargo fmt --all`, `cargo test --workspace`, `cargo clippy --workspace
  --all-targets -- -D warnings`, `git diff --check`, and `just conformance` completed.
  Conformance intentionally exits 1 because semantic parity remains incomplete.
- Conformance: `6584` files compared; `212` exact, `3362` mismatches, `2887`
  missing, `123` extra, `0` read errors, and `2.72%` average match. S3 remains
  `1344` files with `71` exact, `757` mismatches, `516` missing, and `0` extra.
- Blocker: operation/client documentation, request IDs, protocol/runtime behavior,
  endpoint/auth/retry/checksum support, and the remaining missing source tree are
  still incomplete.
- Next action: port the next shared operation source boundary, beginning with the
  generated client operation documentation and request-ID fields, then rerun the
  all-service conformance comparison.

### Checkpoint: 2026-08-21 — Smithy-style enum compatibility

- State: in progress
- Changed: `crates/aws-sdk-build/src/codegen.rs` now uses the Smithy generated
  header, preserves the standalone type-file indentation, and emits forward-compatible
  enums with the sealed unknown-value wrapper, `From<&str>`, `FromStr`, `as_str`,
  `values`, `AsRef<str>`, `try_parse`, and `Display` behavior. It also emits the
  Smithy enum documentation, derives, and unknown-variant deprecation. Consumer
  generation uses relative service-module paths for the shared unknown-value/error
  helpers and a consumer-only allowance for generated deprecation/Clippy lints;
  standalone conformance snapshots retain the exact service-crate-root paths.
- Evidence: `cargo fmt --all`, `cargo test -p aws-sdk-build`,
  `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`,
  and `git diff --check` are passing. `just conformance` completed and intentionally
  exits 1 because semantic parity remains incomplete.
- Conformance: the previous `6584`-file checkpoint had `16` exact matches,
  `3558` mismatches, `2887` missing, and `123` extra; this checkpoint has `204`
  exact, `3370` mismatches, `2887` missing, and `123` extra. S3 improved from
  `2` exact / `826` mismatches to `65` exact / `763` mismatches, with `516` missing
  and `0` extra in both reports.
- Blocker: operation source semantics, runtime/protocol behavior, endpoint/auth/retry/
  checksum support, documentation, and many AWS-specific decorators remain incomplete.
- Next action: port the next shared operation/protocol boundary from the pinned
  Smithy reference while preserving the model-driven enum and collection rules.

### Checkpoint: 2026-08-21 — Smithy-style inline collection symbols

- State: in progress
- Changed: `crates/aws-sdk-build/src/codegen.rs` now treats Smithy list and map
  shapes as inline collection expressions, recursively resolving their member,
  key, and value targets against the complete selected model. Standalone list/map
  type files and their `types.rs` includes are no longer emitted. The repair also
  keeps per-type rendering filtered to the requested shape without narrowing the
  model used for recursive type resolution, uses Smithy-compatible underscore
  suffixes for reserved-word type module filenames, and removes two generated-code
  Clippy warnings for XML-only headers and unit outputs. The local transport is
  now emitted inside `client.rs`, eliminating the synthetic `aws_runtime.rs` file
  from every generated service.
- Evidence: `cargo fmt --all`, `cargo test -p aws-sdk-build`,
  `cargo test --workspace`, and `cargo clippy --workspace --all-targets -- -D warnings`
  pass. `just conformance` completed and intentionally exits 1 because semantic
  parity remains incomplete.
- Conformance: the current checkpoint compares `6584` files with `16` exact
  matches, `3558` mismatches, `2887` missing, and `123` extra at `0.43%` average
  match. Compared with the preceding `6906`-file checkpoint, generated extras
  fell from `445` to `123` overall and from `47` to `0` for S3; exact matches have
  not increased yet. The S3 report is `1344` files with `2` exact, `826` mismatches,
  `516` missing, and `0` extra. The reserved-word `Type` file now uses its
  reference path, and the local transport no longer creates a synthetic extra file.
- Next action: continue porting shared operation/protocol semantics against the
  Smithy reference while preserving the model-driven collection resolution.

### Checkpoint: 2026-08-21 — M3 model-driven source placement and XML response bindings

- State: in progress
- Changed: `crates/aws-sdk-build/src/codegen.rs` now classifies operation input/output and modeled error shapes from the selected model, emits per-operation input/output/builder files, places modeled errors under `types/error`, resolves XML-flattened lists from model traits, removes operation-name response branches, keeps primitive aliases inline in `types.rs`, and excludes `httpPrefixHeaders`/`httpResponseCode` members from REST-XML document-body reads. `crates/aws-sdk-build/src/model.rs` now also recognizes the packaged `aws.protocols#ec2Query` trait. The previous public aliases remain available.
- Evidence: `cargo fmt --all`, `cargo test -p aws-sdk-build`, `cargo check --manifest-path examples/my_aws_sdk/Cargo.toml`, and `just conformance` completed; conformance intentionally exits 1 because parity is incomplete.
- Conformance: the modeled-error checkpoint compared `6905` files with `16` exact matches; this preceding checkpoint compared `6906` files with `16` exact matches, `3557` mismatches, `2888` missing, and `445` extra. The XML body-binding fix removes invalid generated body reads but does not yet change byte-for-byte matches.
- Blocker: operation source semantics, runtime/protocol behavior, endpoint/auth/retry/checksum support, documentation, and many AWS-specific decorators remain incomplete.
- Next action: delete the obsolete inline-operation renderer and make the active generator warning-free before implementing the next model-driven protocol/runtime boundary.

Passing checks:

- cargo fmt --all
- cargo check -p aws-sdk-build --lib
- cargo test -p aws-sdk-build --lib --tests
- cargo test --workspace
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
  checked-in report records 483 exact matches and the remaining non-zero result;
- clean consumer compile against a semantically complete generated AWS client;
- unchanged-output failure test through a failed generation after every validation
  stage;
- live Floci operation sequence;

The project must not claim completion until those missing gates have current,
reproducible evidence.
