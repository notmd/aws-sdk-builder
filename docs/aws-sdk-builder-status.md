# aws-sdk-builder status

Updated 2026-08-25. Smithy-RS: `/tmp/smithy-rs` at
`f1b64a9c0dd001d4bac4277fec4041da59c1f48d`.

## Checkpoint: 2026-08-25 — M4

- State: in progress. Commit: `e50afb7c7` — align AWS JSON HTTP bindings with
  Smithy-RS's document-only `AwsJsonHttpBindingResolver`.
- Changed: classify non-streaming AWS JSON members as document fields and avoid
  HTTP-header serializers/deserializers for that protocol; added a regression test.
- Evidence: focused test, `cargo test --workspace`, clippy with `-D warnings`,
  rustfmt check, and `git diff --check` all pass. `just conformance` regenerated
  all 15 snapshots and parsed/formatted all 13,167 Rust files.
- Conformance: `13,102 / 65 / 0 / 0` → `13,104 / 63 / 0 / 0`
  (matched / mismatched / missing / extra; `99.45%` → `99.46%`). DynamoDB is
  now `881/882`, with its one remaining diff limited to endpoint formatting/order.
- Blocker: none for this semantic change; 63 parity diffs remain overall.
- Next action: inspect the next highest-impact semantic mismatch. Defer ordering-
  only and documentation-only diffs.
