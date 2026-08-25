# aws-sdk-builder status

Updated 2026-08-25. Smithy-RS: `/tmp/smithy-rs` at
`f1b64a9c0dd001d4bac4277fec4041da59c1f48d`.

## Checkpoint: 2026-08-25 — M5

- State: in progress. Commit: `3e6854da7` — preserve nonzero modeled primitive
  defaults in generated structure builders.
- Changed: render model-driven true/nonzero boolean and numeric defaults as
  literals while retaining `unwrap_or_default()` for zero/false defaults; added
  a focused regression test. This fixes Lambda `Event.event_id` (`1`).
- Evidence: focused test, `cargo test --workspace`, clippy with `-D warnings`,
  rustfmt check, and `git diff --check` all pass. `just conformance` regenerated
  all 15 snapshots and formatted all 13,167 Rust files.
- Conformance: `13,104 / 63 / 0 / 0` → `13,105 / 62 / 0 / 0`
  (matched / mismatched / missing / extra; `99.46%` → `99.47%`). Lambda is
  now `1069/1076`, with 7 remaining diffs.
- Blocker: none for this semantic change; 62 parity diffs remain overall.
- Next action: inspect the next executable semantic mismatch. Defer ordering-
  only and documentation-only diffs.
