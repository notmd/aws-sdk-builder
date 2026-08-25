# aws-sdk-builder status

Updated 2026-08-25. Smithy-RS: `/tmp/smithy-rs` at
`f1b64a9c0dd001d4bac4277fec4041da59c1f48d`.

## Current checkpoint — M15

- Changed: order operation modules and error re-exports by Rust snake-case names;
  escape bare ampersands in quoted documentation attributes. The changes are
  generic and introduce no service-specific branches.
- Conformance: `13,119 / 48 / 0 / 0` → `13,123 / 44 / 0 / 0` files
  (matched / mismatched / missing / extra); average match is `99.64%`.
- Verification: `just conformance`, `cargo test --workspace`, and strict
  workspace clippy pass compilation/tests; the 44 remaining diffs are cosmetic.
- Priority: code semantics first; ordering, formatting, and documentation diffs
  remain last priority.

## Prior checkpoints

| Checkpoint | Commit | Conformance change | Focus |
| --- | --- | --- | --- |
| M13 | `7adc974d2` | `13,114/53` → `13,114/53` | Endpoint semantic parity checkpoint |
| M12 | `45e4462aa` | `13,114/53` → `13,114/53` | Event-stream error metadata |
| M11 | `3efb3e157` | `13,113/54` → `13,114/53` | AWS Query enum member serialization |
| M10 | `96daf3c91` | `13,112/55` → `13,113/54` | Deprecated operation errors |
| M9 | `080ddade3` | `13,110/57` → `13,112/55` | Nested sensitive containers |
| M8 | `5047237a7` | `13,109/58` → `13,110/57` | Sensitive output metadata |
| M7 | `22cea422f` | `13,107/60` → `13,109/58` | AWS Query union scopes |
| M6 | `902b62480` | `13,105/62` → `13,107/60` | XML blob base64 decoding |
| M5 | `3e6854da7` | `13,104/63` → `13,105/62` | Nonzero primitive defaults |

Counts are `matched/mismatched`; conformance reports retain the per-service
diffs and the pinned snapshot metadata.
