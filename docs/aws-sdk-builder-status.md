# aws-sdk-builder status

Updated 2026-08-25. Smithy-RS: `/tmp/smithy-rs` at
`f1b64a9c0dd001d4bac4277fec4041da59c1f48d`.

## Current checkpoint — M16

- Changed: follow Smithy directed operation discovery for modeled errors and
  protocol shape roles, qualify event-stream error predicates with their
  containing error type, and preserve generated definition spacing. The changes
  are generic and introduce no service-specific branches.
- Conformance: `13,123 / 44 / 0 / 0` → `13,127 / 40 / 0 / 0` files
  (matched / mismatched / missing / extra); average match is `99.67%`.
- Verification: `just conformance` generated and formatted all 13,167 snapshots;
  the service builders compile. The remaining 40 diffs are currently ordering,
  formatting, and documentation differences; exact parity remains required.
- Priority: code semantics first; ordering, formatting, and documentation diffs
  remain last priority. The next pass should confirm there is no executable
  semantic mismatch before addressing cosmetic ordering.

## Prior checkpoints

| Checkpoint | Commit | Conformance change | Focus |
| --- | --- | --- | --- |
| M15 | `56153bd37` | `13,119/48` → `13,123/44` | Operation/error ordering and documentation escaping |
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
