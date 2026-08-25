# aws-sdk-builder status

Updated 2026-08-25. Smithy-RS: `/tmp/smithy-rs` at
`f1b64a9c0dd001d4bac4277fec4041da59c1f48d`.

## Current checkpoint — M10

- State: in progress. Commit: `96daf3c91` — preserve deprecated operation
  error variants.
- Changed: emit `smithy.api#deprecated` on modeled variants in standalone
  operation error enums, matching the generic error renderer; added a focused
  regression test for deprecated operation errors.
- Conformance: `13,112 / 55 / 0 / 0` → `13,113 / 54 / 0 / 0` files
  (matched / mismatched / missing / extra; `99.52%` → `99.54%`). SQS is
  `291/294` matched.
- Verification: focused regression test, `cargo test --workspace`, clippy with
  `-D warnings`, `cargo fmt --all -- --check`, and `git diff --check` pass.
- Priority: continue with executable semantic mismatches. Defer ordering-only
  and documentation-only diffs until semantic parity is complete.
- Next action: inspect the next executable mismatch.

## Prior checkpoints

| Checkpoint | Commit | Conformance change | Focus |
| --- | --- | --- | --- |
| M9 | `080ddade3` | `13,110/57` → `13,112/55` | Nested sensitive containers |
| M8 | `5047237a7` | `13,109/58` → `13,110/57` | Sensitive output metadata |
| M7 | `22cea422f` | `13,107/60` → `13,109/58` | AWS Query union scopes |
| M6 | `902b62480` | `13,105/62` → `13,107/60` | XML blob base64 decoding |
| M5 | `3e6854da7` | `13,104/63` → `13,105/62` | Nonzero primitive defaults |

Counts are `matched/mismatched`; conformance reports retain the per-service
diffs and the pinned snapshot metadata.
