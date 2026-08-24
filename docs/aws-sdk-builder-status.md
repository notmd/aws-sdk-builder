# aws-sdk-builder status and audit

Updated 2026-08-25. `Prompt.md` is the project specification. Superseded checkpoint
details are intentionally kept out of this working summary; git history preserves the
full audit trail.

### Checkpoint: 2026-08-25 — Match Smithy acronym-plural member identifiers
- State: in progress
- Changed: member and method identifiers now use the pinned Smithy-RS `toSnakeCase`
  acronym boundary rule, keeping suffixes such as `URLs` together. Legacy shape-name
  casing remains separate, so type and module symbols retain their established
  normalization contract.
- Evidence: compared `Strings.kt` and `SymbolVisitor.kt` in the pinned Smithy-RS
  checkout at `/tmp/smithy-rs`, commit
  `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`. Added focused `CallbackURLs` and
  `LogoutURLs` naming regressions. `just conformance` regenerated and formatted all
  `13,166` snapshots and compiled all 15 selected service crates. Workspace tests,
  clippy with `-D warnings`, formatting, and `git diff --check` pass.
- Conformance: `13,011/13,168` exact, `154` mismatches, `2` missing, and `1` extra
  (`98.65%`) -> `13,020/13,168` exact, `145` mismatches, `2` missing, and `1` extra
  (`98.70%`). Cognito Identity Provider improved from 19 to 10 mismatches.
- Blocker: `just conformance` still exits 1 because unrelated parity gaps remain.
- Next action: continue with the next highest-impact generic mismatch after this
  checkpoint.

### Checkpoint: 2026-08-25 — Match Smithy paginator naming
- State: in progress
- Changed: paginator symbols now use the generator's normalized Rust type casing,
  matching Smithy-RS's `PaginatorGenerator`. Fluent builder paginator docs, return
  types, and constructors use the same normalized symbol while operation and error
  symbols remain unchanged.
- Evidence: compared the pinned Smithy-RS `PaginatorGenerator.kt` at `/tmp/smithy-rs`,
  commit `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`. `just conformance` regenerated
  and formatted all `13,166` snapshots and compiled the selected service crates.
- Conformance: `12,997/13,168` exact, `168` mismatches, `2` missing, and `1` extra
  (`98.57%`) -> `13,011/13,168` exact, `154` mismatches, `2` missing, and `1` extra
  (`98.65%`). IAM improved from `1,602/24` to `1,614/12` mismatches; SNS from
  `432/13` to `434/11`.
- Blocker: `just conformance` still exits 1 because unrelated parity gaps remain.
- Next action: continue investigating the remaining generic mismatches after this
  checkpoint.

### Checkpoint: 2026-08-25 — Box recursive structure and union members
- State: in progress
- Changed: the model-selection transform now mirrors Smithy-RS's recursive-shape
  boxing rule. It deterministically marks direct structure/union cycles, treats list
  and map paths as existing indirection, and applies `Box` consistently to public
  fields, accessors, builders, JSON deserializers, and XML deserializers. This fixes
  the CloudWatch Logs `LogFieldType.element` recursive Rust layout without a
  service-specific branch. A model regression covers direct recursive structures.
- Evidence: compared the pinned Smithy-RS `RecursiveShapeBoxer.kt`, `SymbolVisitor.kt`,
  and `JsonParserGenerator.kt` at `/tmp/smithy-rs`, commit
  `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`. `just conformance` compiled all selected
  service crates and regenerated/formatted all `13,166` snapshot files without
  generated-source parse errors.
- Conformance: `12,997/13,168` exact, `168` mismatches, `2` missing, and `1` extra
  (`98.57%`) versus `12,995/13,168` exact (`98.56%`) before this checkpoint.
  CloudWatch Logs improved from `1,276/11` to `1,278/9` mismatched files.
- Blocker: `just conformance` still exits 1 because unrelated parity gaps remain; the
  recursive generated source now matches the reference and compiles.
- Next action: stop after committing this compile fix, unless broader conformance
  parity work is requested.

### Checkpoint: 2026-08-25 — Match Smithy error-correction defaults
- State: in progress
- Changed: shared `serde_util` correction generation now uses Smithy-RS defaults for
  required blob and union members: empty blobs use `Blob::new("")`, while unions use
  their `Unknown` variant. The rule is model-driven and applies to operation outputs,
  event-stream structures, and nested correction helpers; a focused regression covers
  both target kinds.
- Evidence: compared the pinned Smithy-RS `ErrorCorrection.kt` at `/tmp/smithy-rs`,
  commit `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`. `just conformance` regenerated
  and formatted all `13,166` snapshot files without generated-source parse errors.
  The generated Bedrock `serde_util.rs` now has the Smithy fallback expressions; its
  remaining diff is limited to helper reachability/order and other independent gaps.
  Workspace tests, clippy with `-D warnings`, formatting, and `git diff --check` pass.
- Conformance: `12,995/13,168` exact, `170` mismatches, `2` missing, and `1` extra
  (`98.56%`) before and after this checkpoint; the semantic diff shrank although no
  complete file crossed the exact-match boundary.
- Blocker: broader protocol ordering, documentation, shape, and runtime parity gaps
  remain; no blocker in this checkpoint.
- Next action: align the remaining shared protocol helper ordering and dependency
  ownership after committing this checkpoint.

### Checkpoint: 2026-08-25 — Match Smithy modeled-error fallback arms
- State: in progress
- Changed: shared JSON and XML HTTP error dispatch now renders the modeled-error
  fallback arm exactly as Smithy-RS's `ProtocolParserGenerator`: `_ =>
  Error::generic(generic)` has no trailing comma. The correction is shared by every
  service and protocol renderer, with a focused JSON regression.
- Evidence: inspected the pinned Smithy-RS `ProtocolParserGenerator.kt` and
  `ResponseDeserializerGenerator.kt` at `/tmp/smithy-rs`, commit
  `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`. `just conformance` regenerated and
  formatted all `13,166` snapshot files without parse errors. Workspace tests, clippy
  with `-D warnings`, formatting, and `git diff --check` pass.
- Conformance: `12,966/13,168` exact, `199` mismatches, `2` missing, and `1` extra
  (`98.35%`) -> `12,988/13,168` exact, `177` mismatches, `2` missing, and `1` extra
  (`98.46%`). Config improved from `1,236/26` to `1,251/11`; IAM from `1,597/29` to
  `1,602/24`; SESv2 from `1,139/19` to `1,141/17` mismatches.
- Blocker: broader protocol ordering, documentation, shape, and runtime parity gaps
  remain; no blocker in this checkpoint.
- Next action: continue with the next highest-impact generic mismatch after committing
  this checkpoint.

### Checkpoint: 2026-08-25 — Match JSON response/request dependency phase ordering
- State: in progress
- Changed: shared JSON protocol dependency discovery now follows the pinned
  Smithy-RS `OperationGenerator` phase order: response deserializers and modeled error
  parsers establish first-role ownership before request serializers. The rule is
  model-driven and applies uniformly to shared structure, union, list, and map helpers.
  A focused regression covers a shape shared by an operation's input and output. The
  adjacent operation/input module insertion experiment was reverted after validation
  showed it reduced parity.
- Evidence: inspected `OperationGenerator.kt`, `ResponseDeserializerGenerator.kt`,
  `RequestSerializerGenerator.kt`, `ProtocolFunctions.kt`, and `CodegenDelegator.kt`
  in the pinned `/tmp/smithy-rs` checkout at commit
  `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`. `just conformance` generated and
  formatted all `13,166` snapshot files without parse errors. Focused and workspace
  tests, clippy with `-D warnings`, formatting, and `git diff --check` pass.
- Conformance: `12,936/13,168` exact, `229` mismatches, `2` missing, and `1` extra
  (`98.15%`) -> `12,966/13,168` exact, `199` mismatches, `2` missing, and `1` extra
  (`98.35%`).
- Blocker: broader protocol, shape, documentation, and runtime parity gaps remain; no
  blocker in this checkpoint.
- Next action: continue with the next highest-impact generic mismatch after committing
  this checkpoint.

### Checkpoint: 2026-08-25 — Match Smithy note boundaries after HTML lists
- State: in progress
- Changed: shared client documentation normalization now keeps custom note tags
  adjacent to the preceding `ul`/`ol` closing tag, matching the Jsoup-based
  `normalizeHtml` behavior in the pinned Smithy-RS `RustWriter`. The model-independent
  rule has a focused regression and applies to all services.
- Evidence: focused documentation regressions, `just conformance` regeneration and
  formatting, `cargo test --workspace`, `cargo clippy --workspace --all-targets --
  -D warnings`, `cargo fmt --all -- --check`, and `git diff --check` pass. Conformance
  generated and formatted all `13,166` snapshot files without generated-source parse
  errors.
- Conformance: `12,916/13,168` exact, `249` mismatches, `2` missing, and `1` extra
  (`97.94%`) -> `12,936/13,168` exact, `229` mismatches, `2` missing, and `1` extra
  (`98.15%`). Batch improved from `760/2` to `761/1`; DynamoDB from `862/20` to
  `870/12`; IAM from `1,595/31` to `1,597/29`; KMS from `583/8` to `585/6`; Lambda
  from `1,027/49` to `1,030/46`; and SQS from `283/10` to `287/6`.
- Blocker: broader protocol, shape, documentation, and runtime parity gaps remain; no
  blocker in this checkpoint.
- Next action: continue with the next highest-impact generic parity mismatch after
  committing this checkpoint.

### Checkpoint: 2026-08-25 — Match Smithy documentation normalization for anchors and lists
- State: in progress
- Changed: shared client documentation normalization now renders anchors with missing or
  empty `href` attributes as code, and preserves Smithy-RS spacing inside HTML
  description lists (`dl`/`dt`/`dd`). These model-independent rules follow the pinned
  Smithy-RS reference at `/tmp/smithy-rs`, commit
  `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`, with focused regressions.
- Evidence: focused documentation regressions, `just conformance` regeneration and
  formatting, `cargo test --workspace`, `cargo clippy --workspace --all-targets --
  -D warnings`, `cargo fmt --all -- --check`, and `git diff --check` pass. Conformance
  generated and formatted all `13,166` snapshot files without generated-source parse
  errors.
- Conformance: `12,906/13,168` exact, `259` mismatches, `2` missing, and `1` extra
  (`97.88%`) -> `12,916/13,168` exact, `249` mismatches, `2` missing, and `1` extra
  (`97.94%`). Batch improved from `756/6` to `760/2`; Cognito Identity Provider from
  `1,336/25` to `1,340/21`; IAM from `1,593/33` to `1,595/31`.
- Blocker: broader protocol, shape, documentation, and runtime parity gaps remain; no
  blocker in this checkpoint.
- Next action: continue with the next highest-impact generic parity mismatch after
  committing this checkpoint.

### Checkpoint: 2026-08-25 — Preserve raw identifiers in client operation docs
- State: in progress
- Changed: client operation documentation now renders reserved Rust member methods with
  their raw identifier spelling, such as `r#type(...)`, while using the unescaped
  spelling for intra-doc link paths. The rule is shared across all services and follows
  Smithy-RS's symbol-provider spelling for fluent builder documentation, with a focused
  regression.
- Evidence: focused documentation regression, `just conformance` regeneration and
  formatting, `cargo test --workspace`, `cargo clippy --workspace --all-targets --
  -D warnings`, `cargo fmt --all -- --check`, and `git diff --check` pass. Conformance
  generated and formatted all `13,166` snapshot files without generated-source parse
  errors.
- Conformance: `12,905/13,168` exact, `260` mismatches, `2` missing, and `1` extra
  (`97.87%`) -> `12,906/13,168` exact, `259` mismatches, `2` missing, and `1` extra
  (`97.88%`). Batch improved from `755/7` to `756/6` mismatched files.
- Blocker: broader protocol, shape, documentation, and runtime parity gaps remain;
  no blocker in this checkpoint.
- Next action: continue with the next highest-impact generic parity mismatch after
  committing this checkpoint.

### Checkpoint: 2026-08-25 — Preserve Smithy complete-word identifier spellings
- State: in progress
- Changed: the shared Rust identifier splitter now keeps Smithy-RS complete words such
  as `GiB` together as `gib`, while retaining the existing legacy acronym behavior for
  symbols such as `CMKs`. This model-independent naming rule updates all generated
  structure fields, builders, accessors, and protocol serializers consistently. It
  follows the pinned Smithy-RS `Strings.toSnakeCase` complete-word set at
  `/tmp/smithy-rs`, commit `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`, with focused
  naming regressions.
- Evidence: focused naming tests, `just conformance` regeneration and formatting,
  `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo fmt --all -- --check`, and `git diff --check` pass. Conformance generated
  and formatted all `13,166` snapshot files without generated-source parse errors.
- Conformance: `12,899/13,168` exact, `266` mismatches, `2` missing, and `1` extra
  (`97.82%`) -> `12,905/13,168` exact, `260` mismatches, `2` missing, and `1` extra
  (`97.87%`). Batch improved from `750/12` to `755/7`; Lambda from `1,026/50` to
  `1,027/49` (matched/mismatched files).
- Blocker: broader protocol, shape, documentation, and runtime parity gaps remain;
  no blocker in this checkpoint.
- Next action: continue with the next highest-impact generic parity mismatch after
  committing this checkpoint.

### Checkpoint: 2026-08-25 — Keep modeled event-stream errors out of shared JSON parsers
- State: in progress
- Changed: shared AWS JSON role discovery now excludes modeled error roots from the
  ordinary shape parser set while still traversing their members. Event-stream and
  HTTP error paths therefore use the dedicated `de_*_json_err` parser generated by
  Smithy-RS, without an extra `de_<error>` helper or duplicate module composition.
  This follows the pinned Smithy-RS `JsonParserGenerator` and event-stream error
  generation at `/tmp/smithy-rs`, commit `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`,
  with focused regressions.
- Evidence: focused regressions, `just conformance` regeneration and formatting,
  `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo fmt --all -- --check`, and `git diff --check` pass. Conformance generated
  and formatted all `13,166` snapshot files without generated-source parse errors.
- Conformance: `12,890/13,168` exact, `275` mismatches, `2` missing, and `1` extra
  (`97.73%`) -> `12,899/13,168` exact, `266` mismatches, `2` missing, and `1` extra
  (`97.82%`). Bedrock Runtime improved from `506/30` to `512/24`; CloudWatch Logs
  improved from `1,273/14` to `1,276/11`.
- Blocker: broader protocol, shape, documentation, and runtime parity gaps remain;
  no blocker in this checkpoint.
- Next action: continue with the next highest-impact generic parity mismatch after
  committing this checkpoint.
### Checkpoint: 2026-08-25 — Match Smithy enum reserved-member renames and notes
- State: in progress
- Changed: enum member symbols now follow the pinned Smithy-RS reserved-word maps:
  `Self` becomes `SelfValue`, `SelfValue` becomes `SelfValue_`, and modeled
  `Unknown`/`UnknownValue` members avoid the generated catch-all variant. Enum
  documentation, match examples, conversion arms, and rename notes are rendered
  from the same model-derived symbol, including the missing-docs suppression rule.
  This follows `RustReservedWords`, `ClientReservedWords`, and `EnumGenerator` at
  `/tmp/smithy-rs`, commit `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`, with focused
  regressions.
- Evidence: focused regressions, `just conformance` regeneration and formatting,
  `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo fmt --all -- --check`, and `git diff --check` pass. Conformance generated and
  formatted all `13,166` snapshot files without generated-source parse errors.
- Conformance: `12,882/13,168` exact, `283` mismatches, `2` missing, and `1` extra
  (`97.67%`) -> `12,890/13,168` exact, `275` mismatches, `2` missing, and `1` extra
  (`97.73%`). Lambda improved from `1,024/52` to `1,026/50`; Config from `1,234/28`
  to `1,236/26`; Bedrock Runtime from `505/31` to `506/30` mismatches.
- Blocker: broader protocol, shape, documentation, and runtime parity gaps remain; no
  blocker in this checkpoint.
- Next action: continue with the next highest-impact generic parity mismatch after
  committing this checkpoint.

### Checkpoint: 2026-08-25 — Render modeled enum defaults as validated values
- State: in progress
- Changed: structure builders now render non-optional members targeting modeled enum
  shapes with an explicit parsed wire-value default and Smithy-RS's
  `static value validated to member` assertion. Primitive defaults and operation-input
  optionality retain their existing model-driven behavior. The rule follows the pinned
  Smithy-RS `BuilderGenerator` and `DefaultValueGenerator` at `/tmp/smithy-rs`, commit
  `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`, with a focused regression test.
- Evidence: focused regression, `just conformance` regeneration and formatting,
  `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo fmt --all -- --check`, and `git diff --check` pass. Conformance generated and
  formatted all `13,166` snapshot files without generated-source parse errors.
- Conformance: `12,878/13,168` exact, `287` mismatches, `2` missing, and `1` extra
  (`97.62%`) -> `12,882/13,168` exact, `283` mismatches, `2` missing, and `1` extra
  (`97.67%`). Bedrock Runtime improved from `501/35` to `505/31` mismatches.
- Blocker: broader protocol, shape, documentation, and runtime parity gaps remain; no
  blocker in this checkpoint.
- Next action: continue with the next highest-impact generic parity mismatch after
  committing this checkpoint.

### Checkpoint: 2026-08-25 — Order modeled enums by wire values
- State: in progress
- Changed: enum rendering now orders documentation examples, variants, parsers,
  accessors, and values by each member's model-provided `smithy.api#enumValue`, matching
  Smithy-RS `EnumGenerator`'s wire-value ordering. It falls back to the member name
  only when a model omits that trait, and includes a focused regression test.
- Evidence: the focused regression, `just conformance` generation and formatting,
  `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo fmt --all -- --check`, and `git diff --check` pass. Conformance generated and
  formatted all `13,166` snapshot files without generated-source parse errors.
- Conformance: `12,876/13,168` exact, `289` mismatches, `2` missing, and `1` extra
  (`97.60%`) -> `12,878/13,168` exact, `287` mismatches, `2` missing, and `1` extra
  (`97.62%`). Bedrock Runtime improved from `500/36` to `501/35`; Config from
  `1,233/29` to `1,234/28`.
- Blocker: broader protocol, shape, documentation, and runtime parity gaps remain; no
  blocker in this checkpoint.
- Next action: continue with the next largest generic parity mismatch after committing
  this checkpoint.

### Checkpoint: 2026-08-25 — Match Smithy sensitive-union debug generation
- State: in progress
- Changed: unions now derive `Debug` only when the union and all reachable member
  values are non-sensitive. Model-sensitive unions use Smithy-RS's fully redacted
  implementation; unions with sensitive members use per-variant redaction while
  retaining normal debug output for safe variants. This follows the pinned Smithy-RS
  `UnionGenerator` and `Shape.shouldRedact` at `/tmp/smithy-rs`, commit
  `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`, with a focused regression test.
- Evidence: the focused regression, `just conformance` generation and formatting,
  `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo fmt --all -- --check`, and `git diff --check` pass. Conformance generated and
  formatted all `13,166` snapshot files without generated-source parse errors.
- Conformance: `12,863/13,168` exact, `302` mismatches, `2` missing, and `1` extra
  (`97.44%`) -> `12,876/13,168` exact, `289` mismatches, `2` missing, and `1` extra
  (`97.60%`). Bedrock Runtime improved from `487/49` to `500/36` mismatches.
- Blocker: broader protocol, shape, documentation, and runtime parity gaps remain; no
  blocker in this checkpoint.
- Next action: continue with the next largest generic parity mismatch after committing
  this checkpoint.

### Checkpoint: 2026-08-25 — Qualify empty JSON structure deserializers once
- State: in progress
- Changed: the shared JSON structure deserializer now uses the complete model-derived
  type expression for empty structures and derives the builder name separately. This
  prevents `crate::types::crate::types::...` and `crate::types::builders::crate::types::...`
  paths in generated helpers. The rule follows the pinned Smithy-RS
  `JsonParserGenerator` symbol-based structure parser at `/tmp/smithy-rs`, commit
  `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`, and has a focused regression test.
- Evidence: the focused regression, `just conformance` generation and formatting,
  `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo fmt --all -- --check`, and `git diff --check` pass. Conformance generated and
  formatted all `13,166` snapshot files without generated-source parse errors.
- Conformance: `12,859/13,168` exact, `306` mismatches, `2` missing, and `1` extra
  (`97.40%`) -> `12,863/13,168` exact, `302` mismatches, `2` missing, and `1` extra
  (`97.44%`). Bedrock Runtime improved from `485/51` to `487/49`; Lambda from
  `1,022/54` to `1,024/52`.
- Blocker: broader protocol, shape, documentation, and runtime parity gaps remain; no
  blocker in this checkpoint.
- Next action: continue with the next largest generic parity mismatch after committing
  this checkpoint.

### Checkpoint: 2026-08-25 — Handle AwsJson event-stream initial responses
- State: in progress
- Changed: the shared model/protocol planner now recognizes AwsJson operations whose
  output contains an event stream as Smithy-RS initial-response operations. Output types
  emit `into_builder`, fluent builders document deferred non-stream fields, and `send`
  receives the initial response with `try_recv_initial_response`, reparsing its payload
  through the generated JSON operation parser when non-stream document members exist.
  The rule is driven by the selected protocol and event-stream target union, with a
  focused regression covering the generated output and builder paths. This follows the
  pinned Smithy-RS `AwsJsonHttpBindingResolver` and `FluentBuilderGenerator` at
  `/tmp/smithy-rs`, commit `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`.
- Evidence: `just conformance` regenerated and formatted all `13,166` generated Rust
  files without parse errors. The focused event-stream suite, `cargo test --workspace`,
  `cargo clippy --workspace --all-targets -- -D warnings`, formatting, and
  `git diff --check` pass. `just conformance` exits 1 only because broader parity gaps
  remain.
- Conformance: `12,855/13,168` exact, `310` mismatches, `2` missing, and `1` extra
  (`97.38%`) -> `12,859/13,168` exact, `306` mismatches, `2` missing, and `1` extra
  (`97.39%`). CloudWatch Logs improved from `1,268/19` to `1,272/15` exact/mismatched
  files after the two initial-response operation projections became exact.
- Blocker: broader protocol, shape, documentation, and runtime parity gaps remain; no
  blocker in this checkpoint.
- Next action: continue with the next largest generic protocol or shape mismatch after
  committing this checkpoint.

### Checkpoint: 2026-08-25 — Detect event streams from target union shapes
- State: in progress
- Changed: event-stream discovery now follows Smithy-RS `MemberShape.isEventStream` and
  recognizes a member whose target is a streaming union even when the member has no
  `smithy.api#httpPayload` trait. The shared rule feeds event-stream serde generation,
  request/response streaming classification, JSON payload helpers, and lazy protocol
  module ordering. A focused regression covers an output event stream without
  `httpPayload`; single modeled event-stream errors also now use Smithy-RS's single
  `if` dispatch form without a nested condition. The implementation follows the pinned
  Smithy-RS `Smithy.kt` event-stream helpers at `/tmp/smithy-rs`, commit
  `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`.
- Evidence: the focused event-stream suite, `just conformance`, `cargo test --workspace`,
  `cargo clippy --workspace --all-targets -- -D warnings`, formatting, and
  `git diff --check` pass. Conformance generation and rustfmt completed all `13,166`
  generated Rust files without generated-source parse errors; `just conformance` exits
  1 only because broader parity gaps remain.
- Conformance: `12,847/13,168` exact, `316` mismatches, `4` missing, and `1` extra
  (`97.34%`) -> `12,855/13,168` exact, `310` mismatches, `2` missing, and `1` extra
  (`97.38%`). CloudWatch Logs improved from `1,260/25` to `1,268/19` and both
  previously missing event-stream projection files are now generated.
- Blocker: broader protocol, streaming, shape, and runtime parity gaps remain; no
  blocker in this checkpoint.
- Next action: continue with the remaining generic event-stream operation/runtime
  parity gaps, then the next largest protocol or shape mismatch.

### Checkpoint: 2026-08-25 — Omit modeled JSON primitive defaults
- State: in progress
- Changed: shared JSON structure serializers now omit non-required primitive members when
  their values equal modeled `@default` values, matching Smithy-RS `SerializerUtil`.
  Required, `clientOptional`, `addedDefault`, operation-input, collection, and
  non-primitive members retain unconditional serialization. The implementation follows
  the pinned Smithy-RS source at `/tmp/smithy-rs`, commit
  `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`, and has a focused regression test.
- Evidence: `cargo check -p aws-sdk-builder`, the focused regression, formatting, and
  `git diff --check` pass. `just conformance` generated and formatted all `13,164`
  Rust snapshot files, reported no generated-source parse errors, and exits 1 only
  because broader parity gaps remain.
- Conformance: `12,820/13,168` exact, `343` mismatches, `4` missing, and `1` extra
  (`97.20%`) -> `12,847/13,168` exact, `316` mismatches, `4` missing, and `1` extra
  (`97.34%`). CloudWatch Logs improved from `1,253/32` to `1,260/25`; Cognito
  Identity Provider from `1,326/35` to `1,335/26`; Config from `1,228/34` to
  `1,233/29`.
- Blocker: broader protocol, streaming, shape, and runtime parity gaps remain; no
  blocker in this checkpoint.
- Next action: continue with the next highest-impact generic protocol or shape parity
  mismatch.

### Checkpoint: 2026-08-25 — Preserve peekable JSON Document deserialization
- State: in progress
- Changed: JSON Document members now pass the peekable token iterator directly to
  `expect_document`, matching Smithy-RS's `JsonParserGenerator.deserializeDocument`
  instead of consuming the next token first. The shared renderer rule follows the
  pinned Smithy-RS source at `/tmp/smithy-rs`, commit
  `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`, and has a focused regression test.
- Evidence: focused regression, `just conformance` generation and formatting,
  `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo fmt --all -- --check`, and `git diff --check` pass. Conformance reported no
  generated-source parse errors and exits 1 only because broader parity gaps remain.
- Conformance: `12,813/13,168` exact, `350` mismatches, `4` missing, and `1` extra
  (`97.13%`) -> `12,820/13,168` exact, `343` mismatches, `4` missing, and `1` extra
  (`97.20%`). Bedrock Runtime improved from `480/56` to `485/51`; Cognito Identity
  Provider from `1,324/37` to `1,326/35`.
- Blocker: broader protocol, streaming, shape, and runtime parity gaps remain; no
  blocker in this checkpoint.
- Next action: continue with the next highest-impact generic protocol or shape parity
  mismatch.

### Checkpoint: 2026-08-25 — Deserialize JSON map keys through modeled enums
- State: in progress
- Changed: JSON map deserializers now convert modeled enum keys through the generated
  enum type after unescaping JSON object keys, while string keys retain their owned
  string conversion and other key kinds retain their token value. This generic rule
  follows the pinned Smithy-RS JSON map deserializer behavior inspected at
  `/tmp/smithy-rs`, commit `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`.
- Evidence: `cargo check -p aws-sdk-builder`, `cargo test --workspace`, `cargo clippy
  --workspace --all-targets -- -D warnings`, formatting, and `git diff --check` pass.
  `just conformance` generated and formatted all `13,164` Rust snapshot files,
  reported no generated-source parse errors, and exits 1 only because broader parity
  gaps remain.
- Conformance: `12,805/13,168` exact, `358` mismatches, `4` missing, and `1` extra
  (`97.01%`) -> `12,813/13,168` exact, `350` mismatches, `4` missing, and `1` extra
  (`97.13%`). CodeArtifact improved from `440/19` to `444/15`; Lambda from `1,020/56`
  to `1,021/55`; SESv2 from `1,132/26` to `1,133/25`; SQS from `281/12` to `283/10`.
- Blocker: broader protocol, streaming, shape, and runtime parity gaps remain; no
  blocker in this checkpoint.
- Next action: continue with the next highest-impact generic protocol or shape parity
  mismatch.

### Checkpoint: 2026-08-25 — Preserve composed JSON serializer modules and writer names
- State: in progress
- Changed: canonical file assembly now composes duplicate generated paths instead of
  silently overwriting an operation serializer with a shared shape serializer. JSON
  serializer rendering now tracks Smithy-RS's per-writer safe-name state for nested
  union serializers, propagates union serializer root names from their first model-driven
  call site, preserves state when an operation and shared serializer intentionally share
  a file, and orders shared serializers/parsers by their first protocol role. The rules
  are generic and follow the pinned Smithy-RS `RustWriter`, `SafeNamer`, and
  `JsonSerializerGenerator` at `/tmp/smithy-rs`, commit
  `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`.
- Evidence: `just conformance` generated and formatted all `13,164` Rust snapshot files,
  compared `13,168` files, and reported no generated-source parse errors. Workspace
  tests, clippy with `-D warnings`, formatting, and `git diff --check` pass. The
  conformance command exits 1 only because broader parity gaps remain.
- Conformance: `12,767/13,168` exact, `396` mismatches, `4` missing, and `1` extra
  (`96.67%`) -> `12,805/13,168` exact, `358` mismatches, `4` missing, and `1` extra
  (`97.01%`). Bedrock Runtime improved from `462/74` to `480/56`; Batch from
  `747/15` to `750/12`.
- Blocker: remaining generic protocol, streaming, shape, and runtime parity gaps
  remain; no blocker in this checkpoint.
- Next action: continue with the next highest-impact generic protocol or shape parity
  mismatch.

### Checkpoint: 2026-08-25 — Emit JSON event-stream payload parsers
- State: in progress
- Changed: JSON protocol shape modules now emit Smithy-RS's generic
  `de_<shape>_payload` wrapper for non-error structure and union shapes used as
  output event-stream members without an explicit `eventPayload` member. The
  wrapper tokenizes the event payload, rejects null and trailing tokens, and
  delegates to the shared shape parser. The predicate is model-driven and keeps
  modeled event-stream errors on their dedicated `de_*_json_err` path. A focused
  regression covers both ordinary event members and promoted event-stream errors.
  This follows the pinned Smithy-RS `JsonParserGenerator.payloadParser` and
  `EventStreamUnmarshallerGenerator` at `/tmp/smithy-rs`, commit
  `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`.
- Evidence: focused event-stream regressions, `just conformance` regeneration and
  parsing, formatting, and `git diff --check` pass. Conformance formatted all
  `13,164` generated Rust files and reported no generated-source parse errors;
  `just conformance` exits 1 only because broader parity gaps remain.
- Conformance: `12,759/13,168` exact, `404` mismatches, `4` missing, and `1` extra
  -> `12,767/13,168` exact, `396` mismatches, `4` missing, and `1` extra
  (`96.67%`). Bedrock Runtime improved from `455/81` to `462/74`, and Lambda from
  `1,017/59` to `1,018/58`.
- Blocker: remaining generic protocol, streaming, shape, and runtime parity gaps
  remain; no blocker in this checkpoint.
- Next action: continue with the next highest-impact generic protocol or shape
  parity mismatch.

### Checkpoint: 2026-08-25 — Sign input event streams with an empty payload
- State: in progress
- Changed: SigV4 operation signing now uses Smithy-RS's model-driven
  `Some(::aws_sigv4::http_request::SignableBody::Bytes(&[]))` payload override for
  operations whose input contains a streaming event union. Unsigned-payload
  operations retain their higher-priority `UnsignedPayload` override, and ordinary
  operations retain `None`. The shared streaming response deserializer also now
  matches Smithy-RS's spacing around request-ID logging. Focused regressions cover
  input event-stream detection, the signing override, and both logging paths. This
  follows the pinned Smithy-RS `SigV4AuthDecorator` and streaming response output at
  `/tmp/smithy-rs`, commit `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`.
- Evidence: focused regressions, `just conformance` regeneration and parsing,
  `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo fmt --all -- --check`, and `git diff --check` pass. Conformance formatted
  all `13,164` generated Rust files and reported no generated-source parse errors;
  `just conformance` exits 1 only because broader parity gaps remain.
- Conformance: `12,758/13,168` exact, `405` mismatches -> `12,759/13,168` exact,
  `404` mismatches, `4` missing, and `1` extra (`96.57%`). Bedrock Runtime improved
  from `454/82` to `455/81`.
- Blocker: remaining generic protocol, streaming, shape, and runtime parity gaps
  remain; no blocker in this checkpoint.
- Next action: align the next highest-impact generic protocol-serialization mismatch,
  then regenerate and repeat the conformance verification loop.

### Checkpoint: 2026-08-25 — Match Smithy-qualified JSON union null peeks
- State: in progress
- Changed: JSON union deserializers now emit the fully qualified
  `::std::option::Option::Some(::std::result::Result::Ok(...))` `ValueNull` peek
  pattern used by Smithy-RS, rather than relying on prelude imports and shortened
  paths. The rule is shared by every generated JSON union and has a focused
  regression test. It follows the pinned Smithy-RS `JsonParserGenerator` output and
  `RustWriter` formatting at `/tmp/smithy-rs`, commit
  `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`.
- Evidence: focused regression, `just conformance` regeneration and parsing,
  `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo fmt --all -- --check`, and `git diff --check` pass. Conformance formatted
  all `13,164` generated Rust files and reported no generated-source parse errors;
  `just conformance` exits 1 only because broader parity gaps remain.
- Conformance: `12,753/13,168` exact, `410` mismatches -> `12,758/13,168` exact,
  `405` mismatches, `4` missing, and `1` extra (`96.56%`). Bedrock Runtime improved
  from `450/86` to `454/82`; CloudWatch Logs improved from `1,251/34` to `1,252/33`.
- Blocker: remaining generic protocol, streaming, shape, and runtime parity gaps
  remain; no blocker in this checkpoint.
- Next action: align the next highest-impact generic JSON protocol mismatch, then
  regenerate and repeat the conformance verification loop.

### Checkpoint: 2026-08-25 — Match lazy event-stream module ordering
- State: in progress
- Changed: top-level `event_stream_serde` registration now follows the model-driven
  lazy dependency phases used by Smithy-RS: input event-stream marshallers register
  before idempotency modules, modeled-error event streams register after `lens`, and
  ordinary output streams retain their late placement. Added a focused regression for
  the input and modeled-error phases. The rule follows the pinned Smithy-RS
  `EventStreamMarshallerGenerator`, `EventStreamUnmarshallerGenerator`,
  `RustCrate.injectInlineDependencies`, and `WriterDelegator` behavior at
  `/tmp/smithy-rs`, commit `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`.
- Evidence: `just conformance` generated 15 all-operation snapshots and 1,133
  operations, formatted 13,164 generated Rust files, compared 13,168 files, and
  completed without generated-source parse errors. `cargo test --workspace`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo fmt --all -- --check`, and `git diff --check` pass. `just conformance`
  exits 1 only because broader parity gaps remain.
- Conformance: `12,725/13,168` exact, `438` mismatches, `4` missing, `1` extra
  (`96.17%`) -> `12,727/13,168` exact, `436` mismatches, `4` missing, `1` extra
  (`96.18%`). Bedrock Runtime improved from `423/113` to `424/112`; CloudWatch
  Logs improved from `1,250/35` to `1,251/34`.
- Blocker: broader Bedrock Runtime protocol/shape differences and remaining generic
  parity gaps remain; no blocker in this checkpoint.
- Next action: align the model-derived copy-type accessor signature, starting with
  the `CountTokensOutput::input_tokens` mismatch.

### Checkpoint: 2026-08-25 — Gate extended request-ID logging by the S3 decorator
- State: in progress
- Changed: streaming response deserialization now receives the shared request-ID plan
  and emits `s3_request_id::RequestIdExt` logging only when the model-selected S3
  decorator is active. The prior renderer unconditionally emitted this S3-only path
  for every streaming operation. A focused regression covers both enabled and disabled
  plans, checked against Smithy-RS's `S3ExtendedRequestIdDecorator` at `/tmp/smithy-rs`,
  commit `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`.
- Evidence: focused regression, `just conformance` generation and parsing,
  `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo fmt --all -- --check`, and `git diff --check` pass. `just conformance` exits
  1 only because broader parity gaps remain.
- Conformance: `12,721/13,168` exact, `442` mismatches, `4` missing, `1` extra ->
  `12,725/13,168` exact, `438` mismatches, `4` missing, `1` extra (`96.17%` average).
  Bedrock Runtime improved from `421/115` to `423/113`; CodeArtifact from `439/20`
  to `440/19`; Lambda from `1,016/60` to `1,017/59`.
- Blocker: broader protocol, shape, and installation parity gaps remain; no blocker in
  this checkpoint.
- Next action: correct the event-stream module ordering in `src/lib.rs`, then continue
  the remaining generic Bedrock Runtime protocol/shape mismatches.

### Checkpoint: 2026-08-25 — Distinguish streaming blobs from event streams
- State: in progress
- Changed: the shared primitive-export predicate now enables `SdkBody` and byte-stream
  re-exports only when a reachable shape is a streaming Smithy blob. Streaming unions
  used for event streams no longer incorrectly enable those raw-body primitives. A
  focused regression covers both shape kinds, following Smithy-RS's streaming blob
  symbol behavior at `/tmp/smithy-rs`, commit
  `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`.
- Evidence: focused regression, `just conformance` generation and parsing,
  `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo fmt --all -- --check`, and `git diff --check` pass. `just conformance` exits
  1 only because broader parity gaps remain.
- Conformance: `12,719/13,168` exact, `444` mismatches, `4` missing, `1` extra ->
  `12,721/13,168` exact, `442` mismatches, `4` missing, `1` extra (`96.12%` average).
  Bedrock Runtime improved from `420/116` to `421/115`; CloudWatch Logs improved
  from `1,249/36` to `1,250/35`.
- Blocker: broader protocol, shape, and installation parity gaps remain; no blocker in
  this checkpoint.
- Next action: fix the model-driven request-ID extension predicate and continue the
  remaining generic Bedrock Runtime protocol/shape mismatches.

### Checkpoint: 2026-08-25 — Match event-stream input direction
- State: in progress
- Changed: the shared model-driven type planner now distinguishes client input event
  streams from output event streams, emitting `EventStreamSender<...>` for input
  members and retaining `EventReceiver<...>` for outputs. Event-stream input builders
  now treat the stream as required, and client operation documentation uses the sender
  type. A focused regression covers generated input types, builder validation, and
  client documentation.
- Evidence: focused event-stream regression, `cargo test --workspace`,
  `cargo clippy --workspace --all-targets -- -D warnings`, formatting, and
  `git diff --check` pass. Generated-source parsing completed without errors.
- Conformance: `12,716/13,168` exact, `447` mismatches, `4` missing, `1` extra ->
  `12,719/13,168` exact, `444` mismatches, `4` missing, `1` extra (`96.10%`).
  Bedrock Runtime improved from `417/119` to `420/116` exact/mismatched files.
- Blocker: broader protocol, shape, and installation parity gaps remain; `just
  conformance` exits 1 while those differences remain.
- Next action: commit this checkpoint, then investigate the next generic Bedrock
  Runtime mismatch class.

### Checkpoint: 2026-08-25 — Normalize unlinked documentation anchors
- State: in progress
- Changed: the shared client-documentation tokenizer now applies Smithy-RS's
  pseudo-anchor normalization to unlinked `<a>...</a>` fragments, emitting
  `<code>...</code>` while preserving real links with `href` attributes. This removes
  the generated-only unlinked-anchor form across operation/client documentation. A
  focused regression covers the conversion.
- Conformance: `just conformance` generated 15 services and 1,133 operations,
  formatted 13,164 generated Rust files, and compared `13,168` files: `12,716`
  matched, `447` mismatched, `4` missing, and `1` extra (`96.06%` average match).
  This is `+61` exact files and `-61` mismatches from the `12,655/508` checkpoint;
  KMS improved from `540/51` to `583/8`, Batch from `744/18` to `747/15`, and SQS
  from `275/18` to `281/12`. Generated snapshots now contain zero unlinked `<a>`
  tags. Generation and snapshot parsing completed without generated-source parse
  errors. The command exits 1 only because broader parity gaps remain.
- Verification: focused documentation regression, `cargo test --workspace`,
  `cargo clippy --workspace --all-targets -- -D warnings`, formatting, and
  `git diff --check` pass.
- Next action: complete the verification gate, commit this checkpoint, then continue
  with the remaining event-stream, shape, and protocol parity gaps.

### Checkpoint: 2026-08-25 — Resolve AWS Query-compatible modeled error codes
- State: in progress
- Changed: JSON protocol error arms now use the model-driven AWS Query error-code
  resolver whenever the selected service advertises `aws.protocols#awsQuery` or
  `aws.protocols#awsQueryCompatible`. This matches Smithy-RS's compatible protocol,
  which delegates JSON serialization while resolving errors through
  `AwsQueryBindingResolver`; non-Query protocols continue to match shape names. A
  focused regression covers both the compatibility marker and a modeled Query error.
- Conformance: `just conformance` generated 15 services and 1,133 operations,
  formatted 13,164 generated Rust files, and compared `13,168` files: `12,655`
  matched, `508` mismatched, `4` missing, and `1` extra (`95.36%` average match).
  This is `+23` exact files and `-23` mismatches from the `12,632/531` checkpoint;
  SQS improved from `252/41` to `275/18`. Generation and snapshot parsing completed
  without generated-source parse errors. The command exits 1 only because broader
  parity gaps remain.
- Verification: focused Query-compatible regression, `cargo test --workspace`,
  `cargo clippy --workspace --all-targets -- -D warnings`, formatting, and
  `git diff --check` pass.
- Next action: complete the verification gate, commit this checkpoint, then continue
  with the remaining protocol, shape, and documentation parity gaps.

### Checkpoint: 2026-08-25 — Render protocol-decorator request headers
- State: in progress
- Changed: standalone operation request serialization now renders additional headers
  supplied by the model-selected protocol decorator. In particular,
  `aws.protocols#awsQueryCompatible` delegates its JSON wire format while adding the
  Smithy-RS `x-amzn-query-mode: true` marker to every request. The same header plan is
  emitted independently of whether an operation has a document body; JSON target
  headers now follow that rule as well. A focused regression covers the compatible
  protocol trait without a modeled input body.
- Conformance: `just conformance` generated 15 services and 1,133 operations,
  formatted 13,164 generated Rust files, and compared `13,168` files: `12,632`
  matched, `531` mismatched, `4` missing, and `1` extra (`94.84%` average match).
  This is `+21` exact files and `-21` mismatches from the `12,611/552` checkpoint;
  SQS improved from `231/62` to `252/41`. Generation and snapshot parsing completed
  without generated-source parse errors. The command exits 1 only because broader
  parity gaps remain.
- Verification: focused protocol-header regressions, `cargo test --workspace`,
  `cargo clippy --workspace --all-targets -- -D warnings`, formatting, and
  `git diff --check` pass.
- Next action: complete the verification gate, commit this checkpoint, then continue
  with the remaining protocol and documentation parity gaps.

### Checkpoint: 2026-08-25 — Normalize event-stream errors and marshaller output
- State: in progress
- Changed: event-stream model normalization now promotes errors carried by streaming
  unions into operation error lists and removes those error members from public event
  unions, matching Smithy-RS `EventStreamNormalizer`. Event-stream serde emits input
  marshallers before output unmarshaller definitions, uses PascalCase event names in
  parse diagnostics, and resolves input union payload helpers from the union protocol
  module without a duplicate `_input` suffix. Focused regressions cover normalization,
  diagnostic naming, and the generated helper path.
- Conformance: `just conformance` generated 15 services and 1,133 operations,
  formatted 13,164 generated Rust files, and compared `13,168` files: `12,611`
  matched, `552` mismatched, `4` missing, and `1` extra (`94.37%` average match).
  This is `+8` exact files and `-8` mismatches from the `12,603/560` checkpoint;
  Bedrock Runtime improved from `413/123` to `417/119` and CloudWatch Logs from
  `1,245/40` to `1,249/36` exact/mismatched files. Generation and snapshot parsing
  completed without generated-source parse errors. The command exits 1 only because
  broader parity gaps remain.
- Verification: focused event-stream regressions, `cargo test --workspace`,
  `cargo clippy --workspace --all-targets -- -D warnings`, formatting, and
  `git diff --check` pass.
- Next action: continue with the remaining generic protocol and service parity gaps.

### Checkpoint: 2026-08-25 — Match careful nullability for required union members
- State: in progress
- Changed: member nullability now follows Smithy-RS `NullableIndex.CheckMode.CLIENT_CAREFUL`
  for required members targeting both structures and unions. Required union members are
  therefore represented as `Option<T>` in generated types and serializers, matching the
  Smithy-RS `SymbolVisitor` behavior. A focused regression covers a required union member.
- Conformance: `just conformance` generated 15 services and 1,133 operations,
  formatted 13,164 generated Rust files, and compared `13,168` files: `12,603`
  matched, `560` mismatched, `4` missing, and `1` extra (`94.30%` average match).
  This is `+20` exact files and `-20` mismatches from the `12,583/580` checkpoint;
  Bedrock Runtime improved from `393/143` to `413/123` exact/mismatched files.
  Generation and snapshot parsing completed without generated-source parse errors.
  The command exits 1 only because broader parity gaps remain.
- Verification: focused union-nullability regression, `cargo test --workspace`,
  `cargo clippy --workspace --all-targets -- -D warnings`, formatting, and
  `git diff --check` pass.
- Next action: continue with the remaining generic protocol and service parity gaps.

### Checkpoint: 2026-08-25 — Match RestJson operation document member ordering
- State: in progress
- Changed: RestJson operation-level JSON serializers and deserializers now iterate HTTP
  document bindings in member-name order, matching Smithy-RS `HttpBindingResolver`.
  AWS JSON keeps its model-defined member order through the protocol-specific binding
  behavior. Nested structure and modeled-error helpers retain their model order. A
  focused regression covers both operation input serialization and output parsing.
- Conformance: `just conformance` generated 15 services and 1,133 operations,
  formatted 13,164 generated Rust files, and compared `13,168` files: `12,583`
  matched, `580` mismatched, `4` missing, and `1` extra (`94.05%` average match).
  This is `+184` exact files and `-184` mismatches from the `12,399/764` checkpoint.
  Generation and snapshot parsing completed without generated-source parse errors.
  The command exits 1 only because broader parity gaps remain.
- Verification: focused ordering regression, `cargo test --workspace`,
  `cargo clippy --workspace --all-targets -- -D warnings`, formatting, and
  `git diff --check` pass.
- Next action: continue with the remaining generic protocol and service parity gaps.

### Checkpoint: 2026-08-25 — Match HTTP query collection and timestamp bindings
- State: in progress
- Changed: standalone HTTP request serializers now recursively emit one query parameter
  per `@httpQuery` list item. Query timestamp serialization now honors the member-level
  or target-shape `smithy.api#timestampFormat` trait and defaults to `DateTime`, matching
  Smithy-RS `RequestBindingGenerator` behavior at `/tmp/smithy-rs`, commit
  `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`.
- Conformance: `just conformance` generated 15 services and 1,133 operations,
  formatted 13,164 generated Rust files, and compared `13,168` files: `12,354`
  matched, `809` mismatched, `4` missing, and `1` extra (`91.99%` average match).
  This is `+9` exact files and `-9` mismatches from the `12,345/818` checkpoint.
  Generation and snapshot parsing completed without generated-source parse errors.
  The command exits 1 only because broader parity gaps remain.
- Verification: focused HTTP query regression, `cargo test --workspace`,
  `cargo clippy --workspace --all-targets -- -D warnings`, formatting, and
  `git diff --check` pass. Conformance generation and rustfmt complete without
  generated-source parse errors.
- Next action: commit this generic serializer fix.

### Checkpoint: 2026-08-25 — Preserve optional defaulted operation inputs
- State: in progress
- Changed: operation-input builders now preserve members carrying
  `smithy.api#default` as `Option<T>` when constructing the input. This follows
  Smithy-RS `BuilderGenerator`: modeled defaults are applied only when the generated
  member symbol is non-optional, while operation-input symbols remain optional. A
  focused regression covers a defaulted boolean input member.
- Conformance: `just conformance` generated 15 services and 1,133 operations,
  formatted 13,164 generated Rust files, and compared `13,168` files: `12,345`
  matched, `818` mismatched, `4` missing, and `1` extra (`91.93%` average match).
  This is `+92` exact files and `-92` mismatches from the `12,253/910` checkpoint;
  Config improved from `1,179/83` to `1,219/43`, Lambda from `940/136` to
  `951/125`, and SESv2 from `1,060/98` to `1,069/89`. Generation and snapshot
  parsing completed without generated-source parse errors. The command exits 1
  only because broader parity gaps remain.
- Verification: focused defaulted-input test, `cargo test --workspace`,
  `cargo clippy --workspace --all-targets -- -D warnings`, formatting, and
  `git diff --check` pass.
- Next action: continue with the generic HTTP query binding serializer mismatch,
  including collection and member-level timestamp handling.

### Checkpoint: 2026-08-25 — Fix generated event-stream Rust symbols and duplicate request IDs
- State: in progress
- Changed: event-stream marshaller and error-marshaller names now follow the union symbol
  (`UnionMarshaller` and `UnionErrorMarshaller`) as in the pinned Smithy-RS
  `EventStreamMarshallerGenerator`; event-stream error types emit the standard request-ID
  implementation only once. The static Bedrock bearer-token renderer was also simplified to
  satisfy workspace Clippy.
- Conformance: `just conformance` generated 15 services and 1,133 operations,
  formatted 13,164 generated Rust files, and compared `13,168` files: `12,215` matched,
  `948` mismatched, `4` missing, and `1` extra (`90.88%` average match). This restores
  `+1` exact file from the pre-fix event-stream run (`12,214` matched); generation and
  snapshot parsing complete without generated-source parse errors. The command exits 1
  only because broader parity gaps remain.
- Verification: focused event-stream and bearer-auth tests, `cargo test --workspace`,
  `cargo clippy --workspace --all-targets -- -D warnings`, formatting, and
  `git diff --check` pass.
- Next action: continue with the largest remaining generic protocol mismatch.

### Checkpoint: 2026-08-25 — Apply modeled error corrections in JSON operation errors
- State: in progress
- Changed: Rest JSON operation error arms now apply the existing model-driven
  `serde_util::*_correct_errors` builder corrections for required modeled error members,
  matching the XML error path and Smithy-RS output. Corrected errors use the fallible
  builder mapping and do not add the generic-message fallback that applies to optional
  errors. A focused regression covers a required modeled error message plus an HTTP
  error header.
- Conformance: `just conformance` generated 15 services and 1,133 operations,
  formatted 13,164 generated Rust files, and compared `13,168` files: `12,246` matched,
  `917` mismatched, `4` missing, and `1` extra (`91.33%` average match). This is `+31`
  exact files and `-31` mismatches from the `12,215/948` checkpoint; CodeArtifact
  improved from `382/77` to `413/46` exact/mismatched files. Generation and snapshot
  parsing completed without generated-source parse errors. The command exits 1 only
  because broader parity gaps remain.
- Verification: focused JSON modeled-error test, `cargo test --workspace`,
  `cargo clippy --workspace --all-targets -- -D warnings`, formatting, and
  `git diff --check` pass.
- Next action: continue with the next largest generic mismatch class.

### Checkpoint: 2026-08-25 — Resolve operation types from synthetic-shape traits
- State: in progress
- Changed: operation and builder contexts now map only shapes carrying
  `smithy.api.internal#syntheticInput` or `smithy.api.internal#syntheticOutput` to
  operation-local `Input`/`Output` types. Modeled shapes whose names merely end in
  `Input` or `Output` remain public `types::*`, matching Smithy-RS symbol ownership.
  A focused regression covers both synthetic roots and a name-colliding modeled shape.
- Conformance: `just conformance` generated 15 services and 1,133 operations,
  formatted 13,164 generated Rust files, and compared `13,168` files: `12,253` matched,
  `910` mismatched, `4` missing, and `1` extra (`91.38%` average match). This is `+7`
  exact files and `-7` mismatches from the `12,246/917` checkpoint; Lambda improved
  from `935/141` to `940/136`, and Bedrock Runtime from `337/199` to `339/197`.
  Generation and snapshot parsing completed without generated-source parse errors. The
  command exits 1 only because broader parity gaps remain.
- Verification: focused synthetic-shape test, `cargo test --workspace`,
  `cargo clippy --workspace --all-targets -- -D warnings`, formatting, and
  `git diff --check` pass.
- Next action: continue with the next largest generic mismatch class.

### Checkpoint: 2026-08-24 — Use Smithy Document runtime type
- State: in progress
- Changed: Smithy `Document` shapes now map to
  `::aws_smithy_types::Document` instead of `String`, so generated structures,
  builders, and protocol code use the Smithy runtime document representation. The
  mapping follows the pinned Smithy-RS checkout at `/tmp/smithy-rs`, commit
  `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`.
- Conformance: `just conformance` generated 15 services and 1,133 operations,
  formatted 13,164 generated Rust files, and compared `13,168` files: `12,211`
  matched, `952` mismatched, `4` missing, and `1` extra (`90.84%` average match).
  This is `+18` exact files and `-18` mismatches from the previous
  `12,193/970/4/1` checkpoint. The command exits 1 only because broader parity
  gaps remain; generation completed without generated-source parse errors.
- Verification: focused Document mapping test, workspace tests, workspace Clippy
  with `-D warnings`, formatting, and `git diff --check` pass.
- Next action: continue with the largest remaining generic protocol mismatch.

### Checkpoint: 2026-08-24 — Deserialize modeled HTTP headers on protocol errors
- State: in progress
- Changed: JSON and XML protocol error files now emit the same model-driven HTTP header
  and prefix-header deserializers used by response outputs. Operation error arms apply
  those parsed values to the modeled error builders and map parse failures to the
  operation error. This covers Lambda's `TooManyRequestsException.retryAfterSeconds`
  and CodeArtifact's equivalent `ThrottlingException` without service-specific logic.
  The rule follows Smithy-RS `ProtocolParserGenerator` and
  `HttpBindingGenerator` at `/tmp/smithy-rs`, commit
  `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`.
- Evidence: focused modeled-error-header regression passes; conformance regeneration
  completed all 15 services and 1,133 operations without generated-source parse
  errors. Workspace tests, workspace Clippy with `-D warnings`, formatting, and
  `git diff --check` pass.
- Conformance: `12,149/13,168` matched, `1,014` mismatched, `4` missing, and `1`
  extra -> `12,193/13,168` matched, `970` mismatched, `4` missing, and `1` extra
  (`90.66%` average). `just conformance` exits 1 only for the remaining parity gaps.
- Blocker: remaining mismatches are broader generic codegen parity gaps, primarily
  Bedrock Runtime and the residual Lambda/SESv2 differences; no blocker in this
  modeled-header implementation.
- Next action: continue with the largest remaining generic protocol mismatch.

### Checkpoint: 2026-08-24 — Match lazy query protocol helper ordering
- State: in progress
- Changed: query-protocol `protocol_serde` modules now register the first lazy wave as
  the sorted union of modeled error helpers and input serializers, followed by output
  deserializers. REST XML keeps its existing lazy ordering. This is model-driven and
  matches the pinned Smithy-RS checkout at `/tmp/smithy-rs`, commit
  `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`.
- Conformance: `just conformance` generated 15 services and 1,133 operations,
  formatted 13,164 generated Rust files, and compared `13,168` files: `12,033`
  matched, `1,130` mismatched, `4` missing, and `1` extra (`89.44%` average match).
  This is `+3` exact files from the `12,030` baseline. STS and S3 are exact at
  `146/146` and `1,281/1,281`; generation completed without generated-source parse
  errors, including `shape_converse_stream.rs`.
- Verification: focused builder tests, workspace tests, workspace Clippy with
  `-D warnings`, formatting, and `git diff --check` are the final checks for this
  checkpoint. Conformance exits 1 only because broader parity gaps remain.

### Checkpoint: 2026-08-24 — Respect explicit per-operation auth overrides
- State: in progress
- Changed: per-operation SigV4 signing configuration now follows the operation's
  effective `smithy.api#auth` trait. Explicit empty auth lists suppress signing
  configuration while operations that inherit or explicitly select SigV4 retain it.
  The shared service-auth fallback remains model-driven, and a focused regression test
  covers inherited, explicit-empty, and explicit-SigV4 cases. The rule was checked
  against the pinned Smithy-RS `SigV4AuthDecorator` and `ServiceIndex` behavior at
  `/tmp/smithy-rs`, commit `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`.
- Conformance: `just conformance` generated 15 services and 1,133 operations,
  formatted 13,164 generated Rust files, and compared `13,168` files: `11,997`
  matched, `1,166` mismatched, `4` missing, and `1` extra (`88.76%` average match).
  This is `+32` exact files and `-32` mismatches from the previous `11,965/1,198`
  checkpoint. The command exits 1 only because broader parity gaps remain.
- Verification: focused auth regression, `cargo test --workspace`, workspace Clippy
  with `-D warnings`, formatting, and `git diff --check` pass. Conformance generation
  and rustfmt complete without generated-source parse errors.
- Next action: port the model-driven STS retryable-error customization, then continue
  shared protocol and formatting parity work.

### Checkpoint: 2026-08-24 — Match event-stream error dispatch cardinality
- State: in progress
- Changed: event-stream response deserializers now derive modeled error dispatch from
  the union members: no type dispatch is emitted when there are no modeled errors, a
  single modeled error uses Smithy-RS's `if` form, and multiple modeled errors use a
  `match` with an unknown fallback. The rule is generic and model-driven. A focused
  regression test covers the no-modeled-error case. The implementation was checked
  against the pinned Smithy-RS `EventStreamUnmarshallerGenerator` at `/tmp/smithy-rs`,
  commit `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`.
- Conformance: `just conformance` generated 15 services and 1,133 operations,
  formatted 13,164 generated Rust files, and compared `13,168` files: `11,965`
  matched, `1,198` mismatched, `4` missing, and `1` extra (`88.52%` average match).
  This is `+2` exact files and `-2` mismatches from the previous `11,963/1,200`
  checkpoint; S3 is now exact at `1,281/1,281`. The command exits 1 only because
  broader parity gaps remain.
- Verification: focused event-stream regression, `cargo test --workspace`, workspace
  Clippy with `-D warnings`, formatting, and `git diff --check` pass. Conformance
  generation and rustfmt complete without generated-source parse errors.
- Next action: continue generic protocol, auth, retry, and formatting parity work.

### Checkpoint: 2026-08-24 — Honor client-optional required members
- State: in progress
- Changed: member requiredness now follows Smithy-RS client nullability for members
  carrying `smithy.api#clientOptional`, including members that also carry
  `smithy.api#required`. This shared rule feeds generated structures, builders,
  accessors, and protocol serializers. A focused regression test covers the
  required-plus-client-optional case. The implementation was checked against the
  pinned Smithy-RS checkout at `/tmp/smithy-rs`, commit
  `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`.
- Conformance: `just conformance` generated 15 services and 1,133 operations,
  formatted 13,164 generated Rust files, and compared `13,168` files: `11,963`
  matched, `1,200` mismatched, `4` missing, and `1` extra (`88.51%` average match).
  This is `+129` exact files and `-129` mismatches from the `11,834/1,329/4/1`
  checkpoint; the command exits 1 only because broader parity gaps remain.
- Verification: focused nullability test, workspace Clippy with `-D warnings`,
  formatting, and `git diff --check` pass. Conformance generation and rustfmt
  complete without generated-source parse errors.
- Next action: continue generic nullability and protocol parity work.

### Checkpoint: 2026-08-24 — Preserve acronym operation symbols
- State: in progress
- Changed: operation-root, error, fluent-builder, paginator, waiter, and protocol
  renderers now preserve Smithy operation symbols such as `CreateSMSSandboxPhoneNumber`,
  `ListSAMLProviders`, and `AssumeRoleWithSAML`, while synthetic input/output and
  builder types retain normalized names such as `CreateSmsSandboxPhoneNumberInput`.
  The behavior is covered by a focused codegen regression test and follows the pinned
  Smithy-RS `SymbolVisitor` implementation at `/tmp/smithy-rs`, commit
  `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`.
- Conformance: `just conformance` generated 15 services and 1,133 operations,
  compared `13,301` files, and matched `11,570`, with `1,580` mismatches, `17` missing,
  and `134` extra (`84.89%` average match). This is an increase of 116 exact matches
  from the previous `11,454` checkpoint; S3 remains exact at `1,281/1,281`. The command
  exits 1 only because broader parity gaps remain.
- Verification: focused acronym test, `cargo test --workspace`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo fmt --all -- --check`, and `git diff --check` pass.
- Next action: continue generic codegen parity work while retaining the exact S3
  checkpoint.

### Checkpoint: 2026-08-24 — Match required modeled error accessors
- State: in progress
- Changed: modeled error structures now generate Smithy-RS-compatible required
  `message() -> &str` accessors and required-message `Display` formatting; optional
  modeled messages retain their `Option<&str>` accessors and conditional display.
- Evidence: the implementation was checked against the pinned Smithy-RS reference at
  `/tmp/smithy-rs`, commit `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`.
- Conformance: `just conformance` compared `13,301` files and matched `11,454`, with
  `1,696` mismatches, `17` missing, and `134` extra (`84.16%` average match). This is
  an increase of 9 exact matches from the previous `11,445` checkpoint; S3 remains
  exact at `1,281/1,281`. The command exits 1 only because broader parity gaps remain.
- Verification: `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D
  warnings`, `cargo fmt --all -- --check`, and `git diff --check` pass.
- Next action: continue generic codegen parity work while retaining the exact S3
  checkpoint.

### Checkpoint: 2026-08-24 — Remove redundant conformance work
- State: in progress
- Changed: `aws-sdk-conformance` now collects formatter inputs during the exclusion walk,
  formats and restores each batch in one pass, skips unchanged restoration writes, and
  avoids redundant post-projection and leaf-module syntax parses. Rayon 1.12.0 remains
  scoped to the conformance runner.
- Evidence: the pinned Smithy-RS reference remains `/tmp/smithy-rs` at
  `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`; the final timed `just conformance` run took
  `87.56s` wall time, consistent with the warm `87.49s` run.
- Conformance: `10628/1815/724/134` matched/mismatched/missing/extra, `13,301` files
  compared, `77.18%` average match, and S3 remains `1,281/1,281`. Coverage is unchanged;
  the command exits 1 only for existing parity gaps.
- Verification: 21 conformance tests, release Clippy with `-D warnings`, formatting, and
  `git diff --check` pass.
- Blocker: remaining conformance differences are generator parity gaps, not runner
  behavior.
- Next action: continue parity work while retaining the reduced post-processing work.

### Checkpoint: 2026-08-24 — Parallelize conformance post-processing
- State: in progress
- Changed: `crates/aws-sdk-conformance/src/main.rs` now projects completed services while
  generation workers are active, formats disjoint Rust batches with Rayon 1.12.0, and
  restores canonical paths in parallel. `normalize.rs` skips syntax parsing for
  already-restored files without `crate::` paths.
- Evidence: the pinned Smithy-RS reference remains `/tmp/smithy-rs` at
  `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`; focused conformance tests pass. The timed
  `just conformance` run improved from `2:12.75` to `1:22.09` wall time.
- Conformance: `10628/1815/724/134` matched/mismatched/missing/extra before and after;
  `13,301` files compared, `77.18%` average match, and S3 remains `1,281/1,281`.
  Coverage is baseline-equivalent; the command exits 1 only for existing parity gaps.
- Blocker: remaining conformance differences are generator parity gaps, not runner
  behavior.
- Next action: preserve the optimized conformance runner and continue parity work.

### Checkpoint: 2026-08-24 — Make per-service `original.rs` the canonical artifact
- State: in progress
- Changed: each generated service now has one canonical `original.rs` artifact under
  `generated/<service>/` and each conformance snapshot has the corresponding
  `conformance/generated/<service>/original.rs`. Provider `include_sdk!()` macros
  include that artifact directly. The canonical artifact wraps its generated crate
  contents in a private inline module so inner attributes and crate documentation remain
  valid inside caller-owned wrappers; conformance derives its normalized physical module
  tree from that same source. Consumer/conformance renderer
  flags and the obsolete consumer renderer paths were removed. The example consumers
  use Rust edition 2024; generated reference projections retain the pinned 2021
  rustfmt layout for conformance.
- Evidence: canonical composition/splitting is syntax-aware, preserves module
  attributes and documentation, rewrites `crate::` paths for projections, handles
  Unicode source spans, rewrites `crate::` paths in ordinary and macro tokens, and
  materializes nested modules in descending source-offset order. Canonical and
  normalization tests pass; the pinned Smithy-RS reference is `/tmp/smithy-rs` at
  `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`.
- Conformance: `just conformance` generated 15 services and 1,133 operations,
  compared 13,301 files, and matched 10,628 (77.18% average): 1,815 mismatches,
  724 missing, and 134 extra. S3 is exact at 1,281/1,281. Coverage recovered from
  the intermediate 8,991-match run after refreshing the reference normalization
  patches, and remains at the recorded 10,628-match checkpoint. It exits 1 because
  broader generator parity remains incomplete.
- Verification: both example consumers compile under edition 2024; focused canonical
  and normalization tests pass; the mandatory conformance regeneration completed.
  Full workspace tests, clippy, formatting, and final diff checks pass.
- Blocker: remaining conformance differences are generator parity gaps, not canonical
  artifact ownership or normalization projection.
- Next action: preserve this direct-inclusion fix in a follow-up commit.

### Checkpoint: 2026-08-24 — Resolve direct canonical inclusion
- State: completed
- Changed: removed the generated `consumer.rs` indirection from build output and changed
  all 15 provider `include_sdk!()` macros to include exactly their service-owned
  `generated/<service>/original.rs`. `original.rs` now owns a private inline module and
  re-export so its inner crate attributes and `//!` documentation compile at crate root
  and inside any caller-owned wrapper.
- Evidence: canonical splitting unwraps that private module with `syn` spans before
  projecting the original physical module tree; canonical tests pass, and both
  `examples/my_aws_sdk` and `examples/multi_service` compile under Rust edition 2024.
  The pinned Smithy-RS reference remains `/tmp/smithy-rs` at
  `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`.
- Conformance: `just conformance` generated 15 services and 1,133 operations, formatted
  12,577 normalized Rust files, and retained `10,628/13,301` exact matches
  (`1,815` mismatches, `724` missing, `134` extra; `77.18%`). S3 remains exact at
  `1,281/1,281`; the command exits 1 only for the existing parity gaps.
- Verification: the focused canonical tests and both consumer checks pass. `cargo test
  --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo fmt --all -- --check`, and `git diff --check` all pass.
- Next action: preserve this direct-inclusion fix in a follow-up commit.

### Checkpoint: 2026-08-24 — Make patch normalization and consumer ownership explicit
- State: in progress
- Changed: `update-reference` now stages and atomically installs the reference tree,
  `conformance/patches`, and provider model assets together. It parses every included
  reference Rust file with `syn`, rewrites parsed `crate::...` paths to relative
  `super::...` paths, and stores the source-preserving transformation as a `diffy`
  `.patch`. The comparator loads those patches and applies them in memory, while
  applying the same path normalization to generated Rust for the consumer namespace.
  It also removes inline `#[cfg(test)]` modules and their attached test-only attributes
  before comparison; generated SDK sources do not emit those modules.
  The generated aggregate `aws_sdk.rs` facade was removed; each provider owns
  `include_sdk!()`, and callers choose the wrapper module name.
- Evidence: 6,162 checked-in patches exist under `conformance/patches/`; focused
  conformance tests pass; both `examples/my_aws_sdk` and `examples/multi_service`
  compile with caller-owned wrapper modules. `services-manifest.json` no longer
  carries redundant `crate_name`, `module_name`, or derived file-count metadata.
  Error dispatch and display now preserve Smithy-RS's modeled error spelling while
  retaining Rust's acronym-normalized symbol spelling, based on the pinned
  `/tmp/smithy-rs` checkout at `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`.
- Conformance: `just conformance` compares 6,396 files with `4,905/723/713/55`
  matched/mismatched/missing/extra (75.51% average), including S3 at 1,281/1,281
  after removing upstream-only inline unit-test modules from comparison. It exits 1
  because broader generator parity remains incomplete. This architecture checkpoint
  does not claim full conformance completion.
- Verification: `cargo test -p aws-sdk-conformance` passes all 15 tests; the mandatory
  `just conformance` regeneration and comparison completed.
- Blocker: remaining source parity mismatches and missing/extra generated files are
  unrelated to patch storage or wrapper ownership.
- Next action: continue the highest-impact generic codegen mismatch loop while
  retaining this patch and caller-owned-module contract.

### Checkpoint: 2026-08-24 — Keep conformance normalization reference-only
- State: in progress
- Changed: removed generated-side path/test normalization from
  `aws-sdk-conformance/src/report.rs`. `aws-sdk-builder::generate_all` now writes
  conformance snapshots with relative `super::` paths directly, while normal
  `compile()`/`include_sdk!()` output retains standalone `crate::` paths and codegen
  removes inline unit-test modules before writing either output.
- Evidence: the pinned Smithy-RS checkout remains `/tmp/smithy-rs` at
  `f1b64a9c0`; `just conformance` completed generation and formatting, and the
  comparison retained `4904/6396` exact matches (`75.50%` average). No generated-side
  normalizer remains in the comparator.
- Conformance: `4904/724/713/55` matched/mismatched/missing/extra, unchanged in
  exact-match coverage from the previous checkpoint; the command exits 1 because
  broader generator parity is still incomplete.
- Verification: `cargo check -p aws-sdk-builder -p aws-sdk-conformance`, formatting,
  and `git diff --check` pass.
- Blocker: remaining source parity mismatches and missing files are unrelated to
  normalization ownership.
- Next action: run the workspace tests and inspect the resulting generated snapshot
  diff before committing this checkpoint.

### Checkpoint: 2026-08-23 — Emit the complete pinned S3 integration asset tree
- State: in progress
- Changed: expanded the generic registry-backed integration-asset plan to include
  the 42 pinned Rust integration-test sources in addition to the 17 data/license
  fixtures. These files are emitted as opaque SDK test assets; the conformance
  formatter identifies generated Rust by the Smithy-RS generator header and still
  formats every generated `src` file and generated `endpoint_tests.rs`. Asset
  emission is gated by the selected model's packaged protocol-test capability, not
  by a service or operation-name branch.
- Evidence: compared all 59 S3 asset files byte-for-byte with the pinned Smithy-RS
  checkout at `/tmp/smithy-rs`, commit `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`; the
  S3 report is now exact. `just conformance` regenerated and formatted the complete
  snapshot tree.
- Conformance: overall `4,019/674/1,768/1` -> `4,061/674/1,726/1`; S3
  `1,302/0/42/0` -> `1,344/0/0/0` (matched/mismatched/missing/extra). The combined
  mismatch-plus-missing-plus-extra diff shrank from 2,443 to 2,401 files.
- Verification: conformance formatting and comparison completed; the command still
  exits 1 because the other seven P0 service trees remain incomplete.
- Blocker: the remaining parity work is outside S3's reference tree; overall
  mismatches and missing files remain in DynamoDB, IAM, KMS, Lambda, SNS, SQS, and
  STS. The asset path is intentionally limited to the pinned test suite and does
  not replace model-driven source generation.
- Next action: run the full workspace verification suite, commit this S3-exact
  checkpoint, then resume model-driven source parity for the next P0 service.

### Checkpoint: 2026-08-23 — Split the builder into the core and eight service crates
- State: in progress
- Changed: renamed the core package/library to `aws-sdk-builder`/`aws_sdk_builder`,
  moved each supported model into exactly one provider crate, and removed all other
  service packages and the core model directory. The supported providers are
  DynamoDB, IAM, KMS, Lambda, S3, SNS, SQS, and STS. Build scripts now call a
  provider's `compile()` and service-owned `include_sdk!()` macro; consumers choose
  their own wrapper modules rather than using an aggregate facade.
- Evidence: registry metadata covers exactly eight services;
  package-content tests assert one `model.json` plus manifest/glue per provider;
  `/tmp/smithy-rs` is pinned at `f1b64a9c0` for the codegen reference.
- Conformance: `just conformance` generated 8 services and 568 operations, compared
  6,518 files, and matched 4,871 (73.12% average): 838 mismatches, 752 missing,
  and 57 extra. S3 is 1,314/1,344 matched (97.77%); the command remains non-zero
  because parity is intentionally incomplete.
- Verification: core/provider/package tests and the multi-service aggregate example
  pass. The smithy-rs reference checkout remains `/tmp/smithy-rs` at
  `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`.
- Blocker: remaining conformance differences are generator parity gaps, not service
  package ownership; no removed service is present in the workspace registry.
- Verification: `cargo fmt --all -- --check`, `cargo test --workspace`,
  `cargo clippy --workspace --all-targets -- -D warnings`, and `git diff --check`
  all pass. `cargo metadata` lists only the eight provider crates plus the core,
  conformance harness, and two examples.
- Blocker: conformance remains non-zero only for the documented generator parity
  gaps; the eight-service package split itself is verified.
- Next action: continue generic codegen parity work from this stable eight-service
  baseline.

### Checkpoint: 2026-08-23 — Package model-selected S3 test fixtures
- State: in progress
- Changed: added a generic registry-backed integration-asset plan. The selected model
  can contribute pinned non-Rust test fixtures to standalone snapshots without a
  service- or operation-name branch; the S3 plan packages the 17 Smithy-RS fixtures
  under `tests/blns`, `tests/data`, and `tests/select-object-content.json`. The crate
  package include list now carries the asset tree. The conformance formatter now
  formats every generated Rust file; the prior endpoint-test exclusion was removed.
- Evidence: compared every imported fixture with the pinned Smithy-RS checkout at
  `/tmp/smithy-rs`, commit `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`; all 17 are exact
  in the generated snapshot. `just conformance` completed generation, formatting,
  and comparison.
- Conformance: overall `4,002/674/1,785/1` -> `4,019/674/1,768/1`; S3
  `1,285/0/59/0` -> `1,302/0/42/0` (matched/mismatched/missing/extra). The combined
  mismatch-plus-missing-plus-extra diff shrank from 2,460 to 2,443 files.
- Verification: `cargo fmt --all -- --check`, `cargo test --workspace`,
  `cargo clippy --workspace --all-targets -- -D warnings`, and `git diff --check`
  pass. Conformance still exits 1 because 42 S3 handwritten Rust integration tests
  and broader service parity remain incomplete.
- Blocker: the remaining S3 test sources are handwritten Smithy-RS integration tests;
  formatting their copied bytes with the current required rustfmt creates mismatches,
  so they need a generated-test design rather than another formatter exclusion.
- Next action: derive a reusable model/capability-driven renderer for the first S3
  integration-test family, starting with request/response fixture tests.

### Checkpoint: 2026-08-23 — Generate model-driven endpoint operation tests

- State: in progress
- Changed: added generic lowering for Smithy-RS `smithy.rules#endpointTests`
  operation inputs when their modeled parameters are string-compatible. Deprecated
  global-endpoint built-ins are filtered as in Smithy-RS, and unsupported streaming
  HTTP payload inputs are excluded from this renderer using their modeled traits. The
  endpoint test source retains the pinned Smithy-RS rustfmt-era macro layout; the
  conformance formatter leaves only this generated test file class untouched so those
  byte-level fixtures remain stable.
- Evidence: compared the filter and renderer against the pinned Smithy-RS checkout at
  `/tmp/smithy-rs`, commit `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`; the generated
  S3 `tests/endpoint_tests.rs` is byte-identical to its reference.
- Conformance: overall `4,001/674/1,785/1` -> `4,002/674/1,785/1`; S3
  `1,284/1/59/0` -> `1,285/0/59/0` (matched/mismatched/missing/extra). The combined
  mismatch-plus-missing diff shrank from 2,460 to 2,459 files.
- Verification: `just conformance` completed generation, formatting, and comparison;
  source formatting and `git diff --check` pass. Conformance still exits 1 because
  the broader reference test/data tree and remaining source parity are incomplete.
- Next action: run the full workspace test and lint suite, then commit this endpoint
  test parity checkpoint before continuing with the remaining reference tree.

### Checkpoint: 2026-08-23 — Emit model-filtered endpoint test placeholders

- State: in progress
- Changed: added the reusable Smithy-RS endpoint integration-test placeholder when a
  model's `smithy.rules#endpointTests` trait has no operation inputs after filtering
  the deprecated global-endpoint built-ins documented by Smithy-RS. This emits exact
  `tests/endpoint_tests.rs` files for IAM, KMS, Lambda, SNS, SQS, and STS without
  service- or operation-name branches; models with real operation inputs remain for the
  next endpoint-test renderer.
- Evidence: compared the filter behavior with the pinned Smithy-RS checkout at
  `/tmp/smithy-rs`, commit `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`; 6 generated
  endpoint test files are byte-identical to their references.
- Conformance: overall `3,995/674/1,792/1` -> `4,001/674/1,786/1`; S3
  `1,284/0/60/0` -> `1,284/0/60/0` (matched/mismatched/missing/extra). The combined
  mismatch-plus-missing diff shrank from 2,466 to 2,460 files.
- Verification: `just conformance` completed generation, formatting, and comparison;
  `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo fmt --all -- --check`, and `git diff --check` pass. Conformance still exits 1
  because parity remains incomplete.
- Blocker: S3's endpoint test file has model operation inputs and needs generic lowering;
  the remaining S3 gaps are otherwise the reference test/data tree.
- Next action: commit this endpoint-test checkpoint, then lower modeled endpoint test
  operation inputs generically for S3 and DynamoDB.

### Checkpoint: 2026-08-23 — Match model-driven S3 Cargo manifest

- State: in progress
- Changed: added a generic Cargo manifest renderer aligned with Smithy-RS's
  `CargoTomlGenerator` and AWS dependency decorators. Package metadata comes from the
  registry/model; dependency tables and feature flags derive from protocol, endpoint
  library functions, blob/streaming shapes, checksums, presigning, S3 Express, and the
  packaged protocol-test set. The manifest is emitted for selections with packaged
  protocol tests, currently producing the exact S3 package manifest without a service
  name branch.
- Evidence: compared dependency versions, feature order, and manifest layout with the
  pinned Smithy-RS checkout at `/tmp/smithy-rs`, commit
  `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`; `conformance/reference/s3/Cargo.toml`
  and `conformance/generated/s3/Cargo.toml` are byte-identical.
- Conformance: overall `3,994/674/1,793/1` -> `3,995/674/1,792/1`; S3
  `1,283/0/61/0` -> `1,284/0/60/0` (matched/mismatched/missing/extra). The combined
  mismatch-plus-missing diff shrank from 2,467 to 2,466 files.
- Verification: `just conformance` completed generation, formatting, and comparison;
  `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo fmt --all -- --check`, and `git diff --check` pass. Conformance still exits 1
  because parity remains incomplete.
- Blocker: the remaining S3 gaps are the 60 reference test/data files. Cargo manifests
  for services without packaged protocol tests remain deferred until their generic
  integration-test capability inputs are represented.
- Next action: commit this checkpoint, then derive and generate the reusable S3 test/data
  tree from modeled protocol, endpoint, auth, checksum, and streaming capabilities.

### Checkpoint: 2026-08-23 — Emit generic package README and license metadata

- State: in progress
- Changed: `codegen.rs` now emits the model-derived service README and shared Apache
  license asset for every standalone generated crate. README crate names, versions,
  descriptions, module aliases, and SigV4a example mode come from the selected model
  and registry metadata; no service-name branch was added.
- Evidence: the license asset is byte-identical to the pinned reference license;
  `just conformance` regenerated 8 all-operation snapshots and formatted 4,653
  generated Rust files. All 16 generated README/LICENSE files are exact.
- Conformance: overall `3,978/674/1,809/1` -> `3,994/674/1,793/1`; S3
  `1,281/0/63/0` -> `1,283/0/61/0` (matched/mismatched/missing/extra). The combined
  mismatch-plus-missing diff shrank from 2,483 to 2,467 files.
- Verification: `cargo test --workspace`, `cargo clippy --workspace --all-targets
  -- -D warnings`, `cargo fmt --all -- --check`, and `git diff --check` pass. Conformance
  generation and comparison completed; the recipe still exits 1 because parity remains
  incomplete.
- Blocker: generated package manifests and the reference test/data tree remain missing;
  S3 has no source mismatches after this checkpoint.
- Next action: commit this metadata checkpoint, then implement generic
  model/capability-driven `Cargo.toml` rendering.

### Checkpoint: 2026-08-23 — Add model-driven standalone runtime modules

- State: in progress
- Changed: emitted the Smithy-RS runtime assets for AWS-chunked encoding, endpoint-based
  auth option merging, request/response checksums, and S3 Express when the selected model
  requires those capabilities. Added a model-driven event-stream unmarshaller renderer
  for streaming union shapes. The runtime source modules are shared assets; generation
  is selected by model traits and endpoint rules rather than service or operation names.
- Evidence: compared the assets and event-stream lowering with the pinned Smithy-RS
  checkout at `/tmp/smithy-rs`, commit `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`.
  `just conformance` regenerated 8 all-operation snapshots and formatted 4,653 generated
  Rust files.
- Conformance: runtime-asset stage overall `3,968/673/1,820/1` -> `3,972/673/1,816/1`;
  S3 `1,272/0/72/0` -> `1,275/0/69/0`. Runtime-module stage overall
  `3,972/673/1,816/1` -> `3,978/674/1,809/1`; S3 `1,275/0/69/0` ->
  `1,281/0/63/0`. The Lambda event-stream file is emitted and differs only in a
  legacy Smithy-RS rustfmt indentation quirk; the net missing-plus-mismatched diff still
  shrank by six files.
- Verification: `cargo test --workspace`, `cargo clippy --workspace --all-targets
  -- -D warnings`, `cargo fmt --all -- --check`, and `git diff --check` pass. `just
  conformance` completed generation, formatting, and comparison but exits 1 because
  parity remains incomplete.
- Next action: continue with the remaining S3 package metadata and reference test/data
  tree after recording this runtime-module parity checkpoint.

### Checkpoint: 2026-08-23 — Align model-driven BDD endpoint lowering

- State: in progress
- Changed: endpoint generation now follows the Smithy-RS BDD lowering for optional
  function arguments, optional string equality, infallible assignments, `getAttr`
  conditions, URI encoding, and model-driven region validation. Result formatting also
  derives its multiline threshold from endpoint model data.
- Evidence: compared behavior with the pinned Smithy-RS checkout at `/tmp/smithy-rs`,
  commit `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`; `just conformance` regenerated
  eight all-operation snapshots and formatted 4,642 generated Rust files.
- Conformance: overall `3,967/674/1,820/1` -> `3,968/673/1,820/1`; S3
  `1,271/1/72/0` -> `1,272/0/72/0` (matched/mismatched/missing/extra). Average match
  increased from `58.02%` to `58.03%`.
- Verification: `cargo test --workspace`, `cargo clippy --workspace --all-targets
  -- -D warnings`, `cargo fmt --all -- --check`, and `git diff --check` pass.
  `just conformance` still exits 1 because parity remains incomplete; DynamoDB's
  endpoint source remains the primary endpoint mismatch.
- Next action: align the remaining DynamoDB BDD result-arm formatting and ownership
  lowering generically from the Smithy-RS endpoint generators.

### Checkpoint: 2026-08-23 — Match model-driven standalone config generation

- State: in progress
- Changed: added a generic model-driven standalone `src/config.rs` template. The
  renderer now derives checksum, SigV4a, idempotency, S3 Express, DynamoDB account-ID
  endpoint, retry, and aws-chunked configuration capabilities from the selected model;
  it does not branch on service or operation names. Regenerated all eight standalone
  config snapshots, which now match their references exactly.
- Evidence: compared the generated structure with the pinned Smithy-RS checkout at
  `/tmp/smithy-rs` commit `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`; `just conformance`
  regenerated 8 snapshots and formatted 4,583 Rust files.
- Conformance: overall `3,906/676/1,879/1` -> `3,910/672/1,879/1`; S3
  `1,257/1/86/0` -> `1,258/0/86/0` (matched/mismatched/missing/extra). Average match
  increased from `56.59%` to `56.64%`.
- Verification: `cargo test --workspace`, `cargo clippy --workspace --all-targets
  -- -D warnings`, `cargo fmt --all -- --check`, and `git diff --check` pass. The
  conformance recipe still exits nonzero because parity remains incomplete.
- Next action: port the generic S3 endpoint/runtime source tree, starting with
  `src/config/endpoint.rs` and `src/endpoint_lib`.

### Checkpoint: 2026-08-23 — Match long operation-builder templates

- State: in progress
- Changed: `codegen.rs` now derives Smithy-RS long-template indentation from rendered
  field and runtime-plugin expression widths. Builder fields whose modeled input path
  crosses the 150-column threshold and runtime-plugin calls whose rendered path is
  long now retain the pinned writer's continuation and closing indentation. No
  service- or operation-name branch was added. Regenerated all eight all-operation
  snapshots.
- Evidence: compared the rule with the pinned Smithy-RS checkout at `/tmp/smithy-rs`
  commit `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`; `just conformance` generated 8
  snapshots and formatted 4,583 Rust files. All five affected S3/IAM builder files
  are now exact.
- Conformance: overall `3,897/685/1,879/1` -> `3,902/680/1,879/1`; S3
  `1,254/4/86/0` -> `1,257/1/86/0` (matched/mismatched/missing/extra). Average
  match increased from `56.38%` to `56.43%`. The recipe still exits 1 because
  parity remains incomplete.
- Verification: `cargo test --workspace`, `cargo clippy --workspace --all-targets
  -- -D warnings`, `cargo fmt --all -- --check`, and `git diff --check` pass.
- Blocker: S3 `src/config.rs` remains the only S3 mismatch; shared config/endpoint/
  runtime surfaces, reference package/test trees, and broader missing files remain.
- Next action: compare the generic `render_config_file` output with the pinned
  Smithy-RS config/runtime generators and implement the first model-driven shared
  config layer.

### Checkpoint: 2026-08-23 — Match nested custom-tag documentation gaps

- State: in progress
- Changed: `codegen.rs` now derives client-documentation whitespace from the active
  malformed-HTML stack for nested `<note>`, `<important>`, `<warning>`, and `<tip>`
  transitions. The rule is generic and preserves shallow custom-tag spacing while
  matching Smithy-RS/Jsoup depth for nested lists and paragraphs. Added a focused
  regression test for the nested-note list case.
- Evidence: compared the implementation with Smithy-RS `RustWriter.kt` and its
  `normalizeHtml` behavior at `/tmp/smithy-rs` commit
  `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`; `just conformance` regenerated all
  eight all-operation snapshots and formatted 4,583 generated files. The four S3
  client-documentation mismatches are now exact.
- Conformance: overall `3,890/692/1,879/1` -> `3,897/685/1,879/1`
  (matched/mismatched/missing/extra); S3 `1,250/8/86/0` -> `1,254/4/86/0`.
  Average match increased to `56.38%`.
- Verification: focused and workspace tests, Clippy with `-D warnings`, formatting,
  and `git diff --check` pass. The conformance recipe still exits nonzero because
  shared config/endpoint/runtime files, reference test/package trees, and remaining
  builder-layout mismatches are not yet complete.
- Next action: align the generic operation-builder template indentation, then continue
  with the model-driven S3 config/endpoint runtime surface.

### Checkpoint: 2026-08-23 — Generate model-driven auth resolvers

- State: in progress
- Changed: `codegen.rs` now emits generic `src/config/auth.rs` modules from service and
  operation `smithy.api#auth` traits, including SigV4/SigV4a options, no-auth overrides,
  `aws.auth#unsignedPayload` properties, and model-derived S3 Express endpoint-auth
  resolution. Standalone and consumer config trees wire the generated auth module using
  the appropriate source layout.
- Evidence: compared the implementation with the pinned Smithy-RS checkout at
  `/tmp/smithy-rs` commit `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`; `just conformance`
  regenerated all eight service snapshots and formatted 4,583 generated files. All
  eight generated auth snapshots are exact.
- Conformance: overall `3,882/692/1,887/1` -> `3,890/692/1,879/1`
  (matched/mismatched/missing/extra); average match increased to `56.28%`.
- Verification: workspace tests, Clippy with `-D warnings`, formatting, and
  `git diff --check` pass. The conformance recipe still exits nonzero because shared
  config/endpoint/runtime files, reference test/package trees, and remaining source
  mismatches are not yet complete.
- Next action: continue the generic model-driven config and endpoint/runtime parity loop.

### Checkpoint: 2026-08-23 — Refine malformed inline HTML spacing

- State: in progress
- Changed: `codegen.rs` now suppresses duplicate indentation after a closed inline tag
  inside a malformed pseudo-tag and emits Jsoup-style spacing while auto-closing nested
  pseudo-tags. The rules remain model-driven and generic. Regenerated all eight
  all-operation snapshots and conformance reports.
- Evidence: compared the remaining S3 `Message` and pseudo-tag outputs with the pinned
  Smithy-RS `RustWriter.kt` behavior at `/tmp/smithy-rs` commit `f1b64a9c0`; the latest
  `just conformance` regenerated 8 snapshots and formatted 4,575 files.
- Conformance: overall `3,881/693/1,887/1` -> `3,882/692/1,887/1`; S3
  `1,248/9/87/0` -> `1,249/8/87/0` (matched/mismatched/missing/extra). Average
  match increased to `56.07%`.
- Blocker: shared service config/auth/endpoint runtime files, reference test/package
  trees, and remaining nested-note/pseudo-tag spacing differences are still open.
- Next action: run the verification suite, commit this normalizer refinement, then
  begin the generic model-driven endpoint/auth/config runtime gap.

### Checkpoint: 2026-08-23 — Normalize malformed client HTML

- State: in progress
- Changed: `codegen.rs` now tracks implicit and whitespace-derived HTML gaps, limits
  pseudo-tag indentation to the active parent, auto-closes malformed nested tags when
  an ancestor closes, and escapes brackets in client documentation text. These rules
  port the pinned Smithy-RS `normalizeHtml`/Jsoup behavior without service-specific
  branches. Regenerated all eight all-operation snapshots and conformance reports.
- Evidence: inspected Smithy-RS `RustWriter.kt` and the raw S3 documentation traits at
  `/tmp/smithy-rs` commit `f1b64a9c0`; `just conformance` regenerated 8 snapshots and
  formatted 4,575 files. `cargo test --workspace`, Clippy with `-D warnings`,
  formatting, and `git diff --check` pass.
- Conformance: overall `3,871/703/1,887/1` -> `3,881/693/1,887/1`; S3
  `1,244/13/87/0` -> `1,248/9/87/0` (matched/mismatched/missing/extra). Average
  match increased to `56.06%`.
- Blocker: shared service config/auth/endpoint runtime files and reference test/package
  trees remain incomplete; several S3 client docs still differ in Jsoup pretty-print
  spacing.
- Next action: finish the remaining generic client-documentation spacing cases, then
  compare model-driven endpoint/auth/config generation against Smithy-RS.

### Checkpoint: 2026-08-23 — Preserve long fluent method signatures

- State: in progress
- Changed: `codegen.rs` no longer adds a manual column-width branch for standalone
  client operation methods. Smithy-RS writes the fluent method signature as one logical
  source line; formatting/conformance then preserves that layout for long modeled names.
- Evidence: `just conformance` regenerated 8 all-operation snapshots and formatted
  4,575 generated Rust files. `cargo fmt --all`, workspace tests, Clippy with
  `-D warnings`, `cargo fmt --all -- --check`, and `git diff --check` pass.
- Conformance: overall `3,870/704/1,887/1` -> `3,871/703/1,887/1`; S3
  `1,243/14/87/0` -> `1,244/13/87/0` (matched/mismatched/missing/extra). Average
  match increased to `55.77%`.
- Blocker: shared service config/auth/endpoint runtime files and reference test/package
  trees remain incomplete; the full conformance recipe exits nonzero for those known
  parity gaps.
- Next action: align the generic client-operation documentation normalizer with
  Smithy-RS `normalizeHtml(...).replace("\\n", " ")`, starting with the remaining S3
  client documentation mismatches.

### Checkpoint: 2026-08-23 — Preserve required streaming builder errors

- State: in progress
- Changed: `codegen.rs` now checks model-derived builder requiredness before applying
  the `unwrap_or_default()` path for streaming members. Required event-stream and
  streaming targets therefore retain Smithy-RS missing-field errors; optional streams
  and modeled defaults keep their existing behavior.
- Evidence: inspected Smithy-RS `BuilderGenerator.kt` at the pinned
  `/tmp/smithy-rs` checkout; `just conformance` regenerated 8 all-operation snapshots
  and formatted 4,575 generated Rust files. Workspace tests, Clippy with `-D warnings`,
  formatting, and `git diff --check` pass.
- Conformance: overall `3,868/706/1,887/1` -> `3,870/704/1,887/1`; S3
  `1,242/15/87/0` -> `1,243/14/87/0` (matched/mismatched/missing/extra). Lambda
  also gained one exact streaming-output match; average match increased to `55.76%`.
- Blocker: shared service config/auth/endpoint runtime files and reference test/package
  trees remain incomplete; the full conformance recipe exits nonzero for those known
  parity gaps.
- Next action: align the generic client-operation documentation normalizer with
  Smithy-RS `normalizeHtml(...).replace("\\n", " ")`, starting with the remaining S3
  client documentation mismatches.

### Checkpoint: 2026-08-23 — Align builder fallibility with rendered member symbols

- State: in progress
- Changed: `codegen.rs` now derives non-operation builder requiredness from the
  model-resolved rendered member type, matching Smithy-RS `BuilderGenerator`: required
  structure targets remain optional and infallible, while required scalar, collection,
  and event-stream targets retain fallible builders. No service- or operation-specific
  branch was added.
- Evidence: inspected the pinned Smithy-RS `BuilderGenerator.kt` at
  `/tmp/smithy-rs` commit `f1b64a9c0`; `just conformance` regenerated 8 all-operation
  snapshots and formatted 4,575 generated Rust files. `cargo test --workspace`,
  `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`,
  and `git diff --check` pass.
- Conformance: overall `3,795/779/1,887/1` -> `3,868/706/1,887/1`; S3
  `1,223/34/87/0` -> `1,242/15/87/0` (matched/mismatched/missing/extra). Average
  match increased from `54.98%` to `55.74%`.
- Blocker: shared service config/auth/endpoint runtime files and reference test/package
  trees remain incomplete; the full conformance recipe exits nonzero for those known
  parity gaps.
- Next action: compare the remaining S3 `src/config.rs` gap with the pinned generic
  endpoint and auth generators, then add the next model-derived endpoint/config rule.

### Checkpoint: 2026-08-23 — Match standalone service roots

- State: in progress
- Changed: standalone `lib.rs` generation now follows the pinned Smithy-RS crate-doc
  HTML-to-Markdown normalization and model-derived inline-module dependency ordering.
  Service protocol capabilities, checksum/streaming shapes, endpoint rules, waiter and
  paginator presence, request IDs, S3 expiry support, and long-poll inputs determine
  additional modules without service-name branches.
- Evidence: inspected `AwsCrateDocsDecorator.kt`, `RustCrate`, and the relevant Smithy-RS
  decorators in `/tmp/smithy-rs` at snapshot `f1b64a9c0`. `just conformance` regenerated
  8 all-operation snapshots and formatted 4,575 generated Rust files.
- Conformance: overall `3,787/787/1,887/1` -> `3,795/779/1,887/1`
  (matched/mismatched/missing/extra); average match increased from `54.77%` to `54.98%`.
  All eight standalone service `src/lib.rs` snapshots are now exact.
- Next action: continue the generic parity loop with remaining shared client, protocol,
  runtime, and package-tree gaps.

### Checkpoint: 2026-08-23 — Match model defaults, deprecations, and streaming targets

- State: in progress
- Changed: modeled `smithy.api#default` members now use concrete Rust field types and
  `unwrap_or_default()` builder construction while remaining non-required builder
  inputs. Structure and operation deprecations are emitted from model traits. Streaming
  target shapes and streaming members resolve to `ByteStream` across fields, accessors,
  builders, and client documentation, while event-stream unions retain their
  `EventReceiver` documentation.
- Evidence: inspected Smithy-RS `SymbolVisitor`, `StructureGenerator`, serializer
  nullability, and AWS decorator behavior in the pinned `/tmp/smithy-rs` checkout.
  `just conformance` regenerated 8 all-operation snapshots and formatted 4,575
  generated Rust files after each codegen-affecting patch. `cargo test --workspace`,
  `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`,
  and `git diff --check` pass.
- Conformance: `3,732/842/1,887/1` -> `3,787/787/1,887/1`
  (matched/mismatched/missing/extra); average match increased from `54.06%` to
  `54.77%`. The corrected Lambda streaming operation and S3 event-stream documentation
  are now exact.
- Next action: continue with generic standalone `lib.rs` and `config.rs` parity.

### Checkpoint: 2026-08-23 — Match standalone client operation ordering and waiter docs

- State: in progress
- Changed: standalone `operation.rs` declarations now use Smithy-RS-rendered module
  ordering and documentation. Standalone `client.rs` now orders operation modules by
  rendered snake-case names and derives waiter trait ordering and documentation from
  the model. The client usage example also selects a modeled string or enum member
  without service-specific logic.
- Evidence: inspected the pinned Smithy-RS client and operation generators under
  `/tmp/smithy-rs`. `just conformance` regenerated 8 all-operation snapshots and
  formatted 4,575 generated Rust files; the two targeted IAM/Lambda client diffs are
  now exact.
- Conformance: overall `3,730/844/1,887/1` -> `3,732/842/1,887/1`; the report is
  `54.06%` average match. IAM and Lambda each gained one exact standalone client
  snapshot match.
- Next action: run workspace tests, Clippy, formatting, and whitespace validation;
  then continue with generic standalone `lib.rs` and `config.rs` parity.

### Checkpoint: 2026-08-23 — Match correction dependency waves

- State: in progress
- Changed: `codegen.rs` now models Smithy-RS inline correction discovery with
  model-derived protocol role waves. It keeps operation-output corrections first,
  preserves required nested correction dependencies, filters shared corrections to
  deserializer-reachable structures, retains both serialize/deserialize states while
  discovering lazy dependencies, and deduplicates repeated correction names such as
  SNS validation exceptions.
- Evidence: inspected the pinned Smithy-RS `ProtocolFunctions.kt`,
  `CodegenDelegator.kt`, `ErrorCorrection.kt`, and `ClientBuilderInstantiator.kt`.
  `just conformance` regenerated 8 all-operation snapshots and formatted 4,575
  generated Rust files; it exits 1 because broader parity remains incomplete.
  `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`,
  formatting, and `git diff --check` pass.
- Conformance: overall `3,697/877/1,887/1` -> `3,702/872/1,887/1`; S3
  `1,230/27/87/0` -> `1,231/26/87/0` (matched/mismatched/missing/extra).
  DynamoDB, S3, SNS, SQS, and STS serde-util snapshots are now exact.
- Blocker: shared client/runtime/package-tree gaps and remaining IAM/Lambda serde
  ordering plus unrelated source mismatches remain.
- Next action: reconcile the remaining generic IAM/Lambda correction ordering without
  regressing the five services whose serde-util snapshots are now exact.

### Checkpoint: 2026-08-23 — Match Rest XML lazy protocol dependency ordering

- State: in progress
- Changed: Rest XML protocol modules now follow a model-driven lazy dependency plan:
  lexical operation/input roots, deferred event-stream and output helpers, breadth-
  first shared-shape waves, and the role that first reaches each helper. Aggregate
  intermediates are retained during output traversal so nested serializer helpers do
  not get preempted by deserializer paths. Model-derived placement for unset payload
  helpers, event-stream error metadata, and S3 404 metadata remains intact.
- Evidence: inspected Smithy-RS `ProtocolFunctions.kt`,
  `CodegenDelegator.kt`, `RequestSerializerGenerator.kt`, and
  `ResponseDeserializerGenerator.kt` under the pinned `/tmp/smithy-rs` checkout.
  `just conformance` regenerated 8 snapshots and formatted 4,575 generated Rust
  files; it exits 1 because broader parity remains incomplete. Workspace tests,
  Clippy with `-D warnings`, formatting, and `git diff --check` pass.
- Conformance: overall `3,607/967/1,887/1` -> `3,639/935/1,887/1`; S3
  `1,192/65/87/0` -> `1,224/33/87/0` (matched/mismatched/missing/extra).
- Blocker: one remaining S3 protocol mismatch is Smithy-RS template indentation in
  `shape_list_bucket_intelligent_tiering_configurations.rs`; shared client,
  runtime, package-tree, and broader protocol parity gaps remain.
- Next action: compare the remaining S3 root-validation formatting gap and then
  continue with the next model-driven shared runtime mismatch.

### Checkpoint: 2026-08-23 — Match Smithy-RS sensitivity propagation

- State: in progress
- Changed: `codegen.rs` now follows Smithy-RS `Shape.shouldRedact`: sensitivity
  recurses through member targets and list/map elements, but not through arbitrary
  nested structures. `model.rs` adds the AWS decorator's model-derived promotion for
  a `Credentials` aggregate containing sensitive members, with a focused transform
  test. This removes custom Debug implementations from containing S3 and operation
  structures while preserving full STS credential redaction.
- Evidence: inspected Smithy-RS `Smithy.kt`, `StructureGenerator.kt`, and the pinned
  AWS `STSDecorator.kt` under `/tmp/smithy-rs`. `just conformance` regenerated 8
  snapshots and formatted 4,575 generated Rust files; it exits 1 because parity
  remains incomplete. Workspace tests (17), Clippy with `-D warnings`, formatting,
  and `git diff --check` pass.
- Conformance: overall `3,539/1,035/1,887/1` -> `3,603/971/1,887/1`;
  S3 `1,175/82/87/0` -> `1,188/69/87/0`; STS `73/23/56/0` -> `74/22/56/0`
  (matched/mismatched/missing/extra).
- Blocker: shared client/config/protocol/runtime source and reference package/test
  trees remain incomplete; no unresolved regression remains from this checkpoint.
- Next action: compare the remaining S3 protocol and shared runtime mismatches to the
  pinned Smithy-RS serializer/deserializer helper ownership and ordering.

### Checkpoint: 2026-08-23 — Normalize raw identifiers in Rustdoc links

- State: in progress
- Changed: `crates/aws-sdk-builder/src/names.rs` now exposes the generic Rustdoc
  identifier spelling rule used by Smithy-RS, removing `r#` only from intra-doc link
  paths while preserving the public method label. The required-field builder links in
  `codegen.rs` use that rule. Regenerated IAM, Lambda, and S3 snapshots and reports.
- Evidence: inspected Smithy-RS `BuilderGenerator.kt` and `RustWriter.kt` under
  `/tmp/smithy-rs`; the latter documents Rustdoc's raw-identifier link behavior.
  `just conformance` regenerated 8 snapshots and formatted 4,575 generated Rust
  files; it exits 1 because parity remains incomplete. Workspace tests, Clippy with
  `-D warnings`, formatting, and `git diff --check` pass.
- Conformance: overall `3,537/1,037/1,887/1` -> `3,539/1,035/1,887/1`;
  S3 `1,174/83/87/0` -> `1,175/82/87/0` (matched/mismatched/missing/extra).
- Blocker: shared client/config/protocol/runtime source and reference package/test
  trees remain incomplete; no new blocker introduced by this checkpoint.
- Next action: reconcile the generic sensitivity/debug-derive predicate with the
  Smithy-RS `Shape.shouldRedact` behavior, starting from remaining S3 type diffs.

### Checkpoint: 2026-08-23 — Preserve source operation order for type discovery

- State: in progress
- Changed: `crates/aws-sdk-builder/src/model.rs` now retains the source service
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
- Changed: `crates/aws-sdk-builder/src/codegen.rs` now emits the standalone
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
- Changed: `crates/aws-sdk-builder/src/codegen.rs` now emits standalone Smithy-RS-style
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

- M1: complete for the public surface. The eight service provider crates expose
  `compile(operations)` and service-owned `include_sdk!()` macros, while the core owns
  shared generation and atomic installation. There is no generated aggregate facade.
- M2: complete for the supported tier. Each provider packages one `model.json`; the
  core contains metadata, service/module mappings, and pinned
  snapshot SHAs without embedding service models.
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
- M5: in progress. Caller-owned wrapper modules, service-owned macros, relative
  consumer paths, Rust-only output, syntax validation, and atomic installation are
  implemented. The `my_aws_sdk` consumer fixture compiles; `aws_sdk.rs` is obsolete
  and is not generated.
- M6: in progress. The comparator runs against the pinned AWS SDK Rust `3c6d...` P0
  service trees and checks in deterministic summary and per-service reports. The
  current report compares 6,462 files: 3,607 exact, 967 mismatches, 1,887 missing,
  and 1 extra (52.05% arithmetic-average match).
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
- Changed: `crates/aws-sdk-builder/src/codegen.rs` now renders long-operation
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
- Changed: added `crates/aws-sdk-builder/models/protocol-tests/s3.json`, attached
  it through the generic model registry, and added generic request/response
  protocol-test rendering in `crates/aws-sdk-builder/src/codegen.rs`. Request tests
  cover fluent input construction, endpoint/region setup, query/header/body
  assertions, and URI checks. Response tests cover output and modeled-error
  deserialization, nested builders, timestamps, enums, and XML bodies. Error-shape
  tests are inherited by every selected operation that references the shape.
- Evidence: inspected the pinned Smithy-RS `ClientProtocolTestGenerator` and
  `ProtocolTestGenerator` under `/tmp/smithy-rs`. `cargo check -p aws-sdk-builder`
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
- Changed: `crates/aws-sdk-builder/src/codegen.rs` now discovers waiters from
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
- Changed: `crates/aws-sdk-builder/src/model.rs` now discovers operations by walking
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
- Changed: crates/aws-sdk-builder/src/codegen.rs now indexes
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
- Changed: crates/aws-sdk-builder/src/codegen.rs now emits a `types` facade with
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
- Changed: crates/aws-sdk-builder/src/model.rs now ports Smithy-RS operation
  normalization: selected operations point to synthetic
  namespace.synthetic#OperationInput/Output structures, smithy.api#Unit becomes
  an empty structure, original modeled structures are retained only when reachable
  from the rewritten service graph, and conflicting/non-structure synthetic shapes
  fail with a packaged-model diagnostic. crates/aws-sdk-builder/src/codegen.rs
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
  because parity remains incomplete. `cargo check -p aws-sdk-builder` passes.
- Conformance: overall `2,551/1,451/2,460/123` -> `2,555/1,446/2,460/123`; S3
  `930/310/116/0` -> `933/295/116/0` (matched/mismatched/missing/extra).
- Blocker: remaining helper ordering, modeled correction ordering, and the broader
  protocol/runtime parity queue remain open; the full conformance command still
  exits 1.
- Next action: align the remaining Smithy lazy helper and correction discovery order,
  then rerun conformance and retain the higher-coverage checkpoint.

### Checkpoint: 2026-08-22 — Fallible operation-input builders

- State: in progress
- Changed: `crates/aws-sdk-builder/src/codegen.rs` ports the smithy-rs
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
- Changed: `crates/aws-sdk-builder/src/codegen.rs` now preserves Smithy operation
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
- `cargo test -p aws-sdk-builder`
- `cargo test -p aws-sdk-conformance`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo check -p aws-sdk-builder`
- `cargo check --manifest-path examples/my_aws_sdk/Cargo.toml`
- `cargo check --manifest-path examples/floci-s3-smoke/Cargo.toml`
- checked-in all-operation conformance source snapshot
- `cargo package -p aws-sdk-builder --allow-dirty --no-verify` includes all model assets
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
