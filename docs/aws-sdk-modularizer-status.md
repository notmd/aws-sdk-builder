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
