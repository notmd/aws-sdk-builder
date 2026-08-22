# aws-sdk-build status and audit

Updated 2026-08-22. Prompt.md is the project specification.

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
  compares 6,584 files and has 1,137 exact matches (17.38% arithmetic average),
  with 2,461 mismatches, 2,863 missing files, and 123 extra files. Both comparison
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

### Checkpoint: 2026-08-22 — Generic setter documentation parity

- State: in progress
- Changed: `crates/aws-sdk-build/src/codegen.rs` now renders modeled member
  documentation before generated `set_<field>` builder methods, matching Smithy
  Rust's `BuilderGenerator`/`RustWriter` behavior. The rule is shared by every
  structure and operation builder; it is not service- or operation-specific.
- Evidence: the pinned Smithy source was consulted at
  `/tmp/smithy-rs` commit `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`;
  `cargo fmt --all`, `cargo fmt --all -- --check`, `cargo test --workspace`,
  `cargo clippy --workspace --all-targets -- -D warnings`, `git diff --check`,
  and `just conformance` completed. Conformance intentionally exits 1 because
  parity remains incomplete.
- Conformance: the previous checkpoint had `952` exact, `2,646` mismatches,
  `2,863` missing, and `123` extra files overall; this checkpoint has `1,137`
  exact, `2,461` mismatches, `2,863` missing, and `123` extra. S3 increased from
  `247` exact / `585` mismatches / `512` missing / `0` extra to `293` exact /
  `539` mismatches / `512` missing / `0` extra. Exact coverage increased by 185
  files overall and 46 files for S3.
- Blocker: model documentation normalization, the missing protocol/runtime source
  tree, endpoint/auth/retry/checksum behavior, and full operation semantics remain.
- Next action: align the generic modeled HTML documentation normalization used by
  shape and enum renderers, then regenerate and compare the S3 source tree.

### Checkpoint: 2026-08-22 — Model-driven primitive module closure and formatter audit

- State: in progress
- Changed: `crates/aws-sdk-build/src/codegen.rs` now emits standalone primitive
  reexports and child modules from selected model closure data: streaming models
  receive the `ByteStream` aliases and event-stream module contents, while enum
  models receive `sealed_enum_unknown`. Standalone streaming fields resolve to
  `::aws_smithy_types::byte_stream::ByteStream`; consumer generation retains its
  local primitive shim. Generated primitive files now match every corresponding
  pinned reference file exactly.
- Formatter audit: the pinned Smithy `ClientCodegenVisitor.execute()` finalizes a
  generated crate with `cargo fmt -- --config max_width=150`. This implementation
  uses `rustfmt --edition 2021 --config max_width=150,skip_children=true` per
  snapshot file because conformance snapshots do not include a temporary Cargo
  manifest; the resulting primitive files were byte-compared against the Smithy
  output and matched exactly. The Smithy source was verified at
  `/tmp/smithy-rs` commit `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`.
- Evidence: `cargo fmt --all`, `cargo fmt --all -- --check`,
  `cargo check -p aws-sdk-build`, `cargo test --workspace`,
  `cargo clippy --workspace --all-targets -- -D warnings`, `git diff --check`,
  and `just conformance` completed. Conformance intentionally exits 1 because
  parity remains incomplete.
- Conformance: the previous checkpoint had `940` exact, `2,658` mismatches,
  `2,863` missing, and `124` extra files overall; this checkpoint has `952`
  exact, `2,646` mismatches, `2,863` missing, and `123` extra. S3 remains at
  `247` exact / `585` mismatches / `512` missing / `0` extra. Exact coverage
  increased by 12 files overall.
- Blocker: modeled protocol behavior, endpoint/auth/retry/checksum support, and
  the remaining missing source tree are still incomplete.
- Next action: port the next shared modeled protocol/runtime source boundary while
  preserving the Smithy-compatible formatter and model-driven primitive closure.

### Checkpoint: 2026-08-21 — Smithy rustfmt parity and modeled-error ordering

- State: in progress
- Changed: `crates/aws-sdk-build/src/codegen.rs` now formats every generated Rust
  file with `rustfmt --edition 2021 --config max_width=150`, matching the pinned
  Smithy `ClientCodegenVisitor` finalization step (`cargo fmt -- --config
  max_width=150`). Modeled error and builder reexports now follow first appearance
  in operation `errors` lists, and formatter failures are reported through
  `BuildError`. Consumer-only generated send methods allow the modeled-error
  `clippy::result_large_err` lint. The generated snapshots and deterministic reports
  were regenerated.
- Evidence: `/tmp/smithy-rs` is at pinned commit
  `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`; `cargo fmt --all`,
  `cargo check -p aws-sdk-build`, `cargo fmt --all -- --check`, and
  `git diff --check` pass. `just conformance` completed and intentionally exits 1
  because parity remains incomplete.
- Conformance: the immediately preceding regenerated output had `661` exact,
  `2,922` mismatches, `2,878` missing, and `123` extra files overall; this
  checkpoint has `929` exact, `2,654` mismatches, `2,878` missing, and `123` extra.
  S3 increased from `193` exact / `637` mismatches / `514` missing to `244` exact /
  `586` mismatches / `514` missing. Exact coverage increased by `268` files overall
  and `51` files for S3.
- Blocker: modeled error behavior beyond the current metadata surface, full
  protocol/runtime behavior, endpoint/auth/retry/checksum support, and the remaining
  missing source tree are still incomplete.
- Next action: port the next shared modeled-error/protocol source boundary while
  preserving the formatter-backed snapshot generation.

### Checkpoint: 2026-08-21 — Generic request-ID decorator parity

- State: in progress
- Changed: `crates/aws-sdk-build/src/codegen.rs` now derives a request-ID rendering
  plan from the selected service metadata. Every operation output, including empty
  outputs, receives standard request-ID fields, RequestId implementations, builder
  setters, and response-header propagation. S3’s aws.api#service ARN namespace also
  selects the Smithy-aligned s3_request_id helper and extended request-ID trait.
  Unhandled operation errors retain both request-ID values. The fixture declares the
  generated runtime dependencies directly, and the all-operation snapshots and
  reports were regenerated.
- Evidence: `/tmp/smithy-rs` at pinned Smithy commit `f1b64a9...` was consulted;
  `cargo fmt --all`, `just conformance`, `cargo test --workspace`, `cargo clippy
  --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`, and
  `git diff --check` pass (conformance intentionally exits 1 while parity is
  incomplete).
- Conformance: the previous checkpoint had `492` exact matches, `3,082` mismatches,
  `2,887` missing, and `123` extra overall, with `155` exact / `673` mismatches /
  `516` missing for S3. This checkpoint has `661` exact, `2,914` mismatches, `2,886`
  missing, and `123` extra overall (`9.07%`), with `193` exact / `636` mismatches /
  `515` missing for S3 (`14.36%`). Exact coverage increased by `169` files overall
  and `38` files for S3.
- Blocker: modeled error metadata, full protocol/runtime behavior, endpoint/auth/
  retry/checksum support, and the remaining missing source tree are still incomplete.
- Next action: port request-ID metadata into modeled error structures and generic
  protocol deserializers, then rerun the all-service conformance comparison.

### Checkpoint: 2026-08-21 — Inline client documentation whitespace parity

- State: in progress
- Changed: `crates/aws-sdk-build/src/codegen.rs` now renders client operation
  documentation from the selected Smithy model, including model-driven primitive,
  collection, and named-shape types, target-shape documentation fallbacks,
  required-member reporting, output fields, paginated-operation links, and
  Smithy-style HTML whitespace. Inline text immediately inside opening tags now
  preserves the leading space emitted by Smithy’s Jsoup normalizer. Long client
  method signatures use generic rendered length-based wrapping, and nested list
  closing gaps preserve serializer spacing.
- Evidence: `cargo fmt --all`, `cargo test --workspace`, `cargo clippy
  --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`,
  `git diff --check`, and `just conformance` completed. Conformance
  intentionally exits 1 because semantic parity remains incomplete.
- Conformance: the previous checkpoint had `483` exact matches; this checkpoint
  has `490` exact matches out of `6,584`, with `3,084` mismatches, `2,887` missing,
  `123` extra, and `6.82%` average match. S3 improved from `148` to `153` exact
  matches, with `675` mismatches and `516` missing files.
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
