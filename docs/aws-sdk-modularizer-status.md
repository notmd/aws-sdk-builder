# AWS SDK modularizer checkpoint log

## 2026-08-28 — `673e3f1b8`

- Objective: keep shared protocol helpers available only when an owning operation needs them.
- Generic rule: infer ownership for precise `protocol_serde::shape_*` references without propagating ownership from broad `types` or `error` namespaces.
- Changed files: `crates/aws-sdk-modularizer/src/transform.rs`.
- Commands: `cargo test -p aws-sdk-modularizer` passed; `cargo fmt --all` passed; the required conformance run failed at Bedrock Runtime with the zero-feature matrix because shared `event_stream_serde` items still need ownership propagation from gated protocol helpers.
- Operation coverage: 15 services and all model operations mapped before and after; no mapping coverage change.
- Remaining blocker: operation-specific types and errors are still broadly exposed, and conformance requires shared protocol references to be gated consistently.
- Next action: extend precise ownership propagation to operation-specific types/errors, then rerun conformance.

## 2026-08-28 — `cb9b69c10`

- Objective: make the required conformance command use the optimized build profile.
- Generic rule: run the existing conformance recipe with `cargo run --release`.
- Changed files: `justfile`.
- Commands: `AWS_SDK_MODULARIZER_ARCHIVE=/tmp/aws-sdk-rust.tar.gz RUSTFLAGS='-Awarnings' just conformance` ran and reproduced the Bedrock Runtime zero-feature failure described above.
- Operation coverage: unchanged; all 15 services remain mapped.
- Remaining blocker: shared event-stream/protocol ownership and operation-specific type/error gating.
- Next action: add the ownership rules and commit them separately.

## 2026-08-28 — `702672a9a`

- Objective: gate operation-specific types and errors while preserving shared modeled definitions.
- Generic rule: collect type references from operation modules, intersect owners for items that combine operation-specific and shared types, and propagate owners through type declarations, re-exports, builders, and dependent protocol helpers.
- Changed files: `crates/aws-sdk-modularizer/src/transform.rs`.
- Commands: `cargo test -p aws-sdk-modularizer` passed; `cargo fmt --all` passed; conformance is pending for this commit.
- Operation coverage: unchanged; all 15 services remain mapped.
- Remaining blocker: confirm the feature matrix and identify any next ownership or generated-surface failures.
- Next action: run `AWS_SDK_MODULARIZER_ARCHIVE=/tmp/aws-sdk-rust.tar.gz RUSTFLAGS='-Awarnings' just conformance`.

## 2026-08-28 — `f3d90071b`

- Objective: correct the scope of type ownership inference.
- Generic rule: apply type-reference ownership only inside `types`, `types/error`, `protocol_serde`, and `event_stream_serde`; keep service-wide runtime/config modules unconditional.
- Changed files: `crates/aws-sdk-modularizer/src/transform.rs`.
- Commands: `cargo test -p aws-sdk-modularizer` passed; `cargo fmt --all` passed; conformance on `702672a9a` failed in Batch zero-feature mode because shared `error_meta`, config auth/endpoint, and endpoint partition modules were over-gated.
- Operation coverage: unchanged; all 15 services remain mapped.
- Remaining blocker: rerun conformance with the narrowed rule and continue from the next compiler failure.
- Next action: run the required conformance command.

## 2026-08-28 — `df414241c`

- Objective: keep shared `types` modules and runtime helpers unconditional while still gating operation-specific errors.
- Generic rule: gate only synthetic operation error enums and impls in `types/error.rs`; use the operation type-reference map only for event-stream and protocol ownership inference.
- Changed files: `crates/aws-sdk-modularizer/src/transform.rs`.
- Commands: `cargo test -p aws-sdk-modularizer` passed; `cargo fmt --all` passed; conformance on `f3d90071b` failed because shared `types::error`/`types::builders` modules were gated through dependency propagation.
- Operation coverage: unchanged; all 15 services remain mapped.
- Remaining blocker: rerun conformance with shared type modules restored and validate operation-specific error gating.
- Next action: run the required conformance command.

## 2026-08-29 — `503a22672`

- Objective: keep `error_meta` compilable when operation-specific synthetic errors are disabled.
- Generic rule: gate individual items that reference exact `types::error::<...Error>` operation error types; keep the shared service `Error` and modeled exceptions available.
- Changed files: `crates/aws-sdk-modularizer/src/transform.rs`.
- Commands: `cargo test -p aws-sdk-modularizer` passed; `cargo fmt --all` passed; conformance on `df414241c` reached the remaining un-gated `error_meta` conversion references.
- Operation coverage: unchanged; all 15 services remain mapped.
- Remaining blocker: validate the corrected error conversion gates and then handle redundant child-module cfg cleanup.
- Next action: run the required conformance command.

## 2026-08-29 — `f80f54ed8`

- Objective: remove redundant operation feature gates from files already included by a gated parent module.
- Generic rule: use `syn` spans to remove only matching `op_*` attributes from operation child files and protocol child files; preserve unrelated attributes such as `#[cfg(test)]`.
- Changed files: `crates/aws-sdk-modularizer/src/transform.rs`.
- Commands: `cargo test -p aws-sdk-modularizer` passed (11/11); `just conformance` passed for all 15 services; `cargo fmt --all -- --check` passed; `git diff --check` passed; `cargo check --workspace` and `cargo test --workspace` remain blocked by obsolete builder crates that include missing `model.json` files.
- Operation coverage: before and after, all 15 services had complete model coverage with zero missing and zero ambiguous operations; coverage delta `+0`.
- Remaining blocker: obsolete builder/conformance infrastructure still prevents workspace-wide checks.
- Next action: add feature gates for operation-owned types and modeled errors, then regenerate SDK crates in a separate commit.
