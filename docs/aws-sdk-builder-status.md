# aws-sdk-builder status

Updated 2026-08-25. Smithy-RS: `/tmp/smithy-rs` at
`f1b64a9c0dd001d4bac4277fec4041da59c1f48d`.

## Current checkpoint — M12

- State: in progress. Commit: `45e4462aa` — match event-stream error metadata
  arms.
- Changed: use Smithy-RS’s distinct `e` and `_inner` bindings in event-stream
  error inherent and trait metadata implementations; added regression coverage.
- Conformance: `13,114 / 53 / 0 / 0` → `13,114 / 53 / 0 / 0` files
  (matched / mismatched / missing / extra). The removed error-arm token diffs
  were in files that still contain independent ordering/documentation diffs.
- Verification: `just conformance` regenerated and formatted all 15 services;
  focused regression test, `cargo test --workspace`, clippy with `-D warnings`,
  `cargo fmt --all -- --check`, and `git diff --check` pass. Conformance still
  exits 1 for the 53 remaining parity diffs.
- Priority: continue with executable semantic mismatches. Defer ordering-only
  and documentation-only diffs until semantic parity is complete.
- Next action: align deterministic emission order, then resolve documentation
  normalization differences.

## Prior checkpoints

| Checkpoint | Commit | Conformance change | Focus |
| --- | --- | --- | --- |
| M11 | `3efb3e157` | `13,113/54` → `13,114/53` | AWS Query enum member serialization |
| M10 | `96daf3c91` | `13,112/55` → `13,113/54` | Deprecated operation errors |
| M9 | `080ddade3` | `13,110/57` → `13,112/55` | Nested sensitive containers |
| M8 | `5047237a7` | `13,109/58` → `13,110/57` | Sensitive output metadata |
| M7 | `22cea422f` | `13,107/60` → `13,109/58` | AWS Query union scopes |
| M6 | `902b62480` | `13,105/62` → `13,107/60` | XML blob base64 decoding |
| M5 | `3e6854da7` | `13,104/63` → `13,105/62` | Nonzero primitive defaults |

Counts are `matched/mismatched`; conformance reports retain the per-service
diffs and the pinned snapshot metadata.
