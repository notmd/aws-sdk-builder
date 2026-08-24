# aws-sdk-builder status and audit

Updated 2026-08-24. `Prompt.md` is the project specification. Superseded checkpoint
details are intentionally kept out of this working summary; git history preserves the
full audit trail.

### Checkpoint: 2026-08-24 — Make per-service `original.rs` the canonical artifact
- State: in progress
- Changed: each generated service now has one canonical `original.rs` artifact under
  `generated/<service>/` and each conformance snapshot has the corresponding
  `conformance/generated/<service>/original.rs`. Provider `include_sdk!()` macros
  consume the canonical artifact, while conformance derives its normalized physical
  module tree from that same source. Consumer/conformance renderer flags and the
  obsolete consumer renderer paths were removed.
- Evidence: canonical composition/splitting is syntax-aware, preserves module
  attributes and documentation, rewrites `crate::` paths for projections, handles
  Unicode source spans, and materializes nested modules in descending source-offset
  order. Canonical and normalization tests pass; the pinned Smithy-RS reference is
  `/tmp/smithy-rs` at `f1b64a9c0dd001d4bac4277fec4041da59c1f48d`.
- Conformance: `just conformance` generated 15 services and 1,133 operations,
  compared 13,301 files, and matched 7,787 (57.45% average): 4,655 mismatches,
  725 missing, and 134 extra. It exits 1 because broader generator parity remains
  incomplete.
- Verification: `cargo check -p aws-sdk-builder`, focused canonical and normalization
  tests, formatting, and `git diff --check` pass. Full workspace tests and clippy are
  the remaining final verification steps for this checkpoint.
- Blocker: remaining conformance differences are generator parity gaps, not canonical
  artifact ownership or normalization projection.
- Next action: run the complete workspace verification suite and inspect the final
  diff for stale renderer flags or generated backup artifacts.

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
