# aws-sdk-builder status

Updated 2026-08-25. Smithy-RS: `/tmp/smithy-rs` at
`f1b64a9c0dd001d4bac4277fec4041da59c1f48d`.

## Checkpoint

- Commit: `b14a46e24` — apply modeled error-message fallback only to optional
  `message` members, matching Smithy-RS.
- Snapshot: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`, 15 services, 1,133
  operations, 13,167 files.
- Conformance: `13,098 exact / 69 mismatches` → `13,102 exact / 65 mismatches`;
  missing `0`, extra `0`, match `99.39%` → `99.45%`.
- SNS improved from 10 mismatches to 6; required modeled `Message` fields no
  longer receive a generic fallback.

## Evidence and next action

- Focused regression test passed:
  `xml_error_message_fallback_only_targets_optional_message_members`.
- `just conformance` completed generation, rustfmt, and generated-source parsing;
  it still exits 1 for the remaining parity diffs.
- Remaining mismatches are in Bedrock Runtime (4), CloudWatch Logs (5),
  CodeArtifact (2), Cognito Identity Provider (6), Config (4), DynamoDB (3),
  IAM (12), KMS (6), Lambda (8), SESv2 (5), SNS (6), and SQS (4).
- Next semantic target: DynamoDB `PutResourcePolicy` HTTP-header versus AWS JSON
  body binding. Defer ordering-only and documentation-only diffs until semantic
  parity is exhausted.
