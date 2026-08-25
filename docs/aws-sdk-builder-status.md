# aws-sdk-builder status

Updated 2026-08-25. Smithy-RS: `/tmp/smithy-rs` at
`f1b64a9c0dd001d4bac4277fec4041da59c1f48d`.

## Current checkpoint — M14

- State: semantic parity remains complete; the cosmetic documentation follow-up
  is committed as `dbc1fa192`.
- Changed: scope description-list whitespace to the active `dl`, `dt`, or `dd`
  node. This fixes nested Cognito documentation without service-specific code or
  runtime changes.
- Conformance: `13,114 / 53 / 0 / 0` → `13,116 / 51 / 0 / 0` files
  (matched / mismatched / missing / extra); average match is `99.56%`.
- Verification: `just conformance` compiles the generator and all service
  builders, then formats and compares all 13,167 snapshots. It exits 1 only for
  the remaining cosmetic diffs.
- Priority: code-semantic parity first. Ordering, formatting, and documentation
  diffs are last priority and remain deferred.

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
