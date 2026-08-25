# aws-sdk-builder status

Updated 2026-08-25. Smithy-RS reference: `/tmp/smithy-rs` at
`f1b64a9c0dd001d4bac4277fec4041da59c1f48d`.

## Current checkpoint

- Commit: `8958ba975` — treat null Smithy defaults as optional.
- AWS SDK snapshot: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`.
- Snapshot: 15 services, 1,133 operations, 13,167 files.
- Conformance: 13,096 exact, 71 mismatches, 0 missing, 0 extra (99.38%).
- Cognito Identity Provider: 1,355 exact, 6 mismatches (the remaining diffs are
  documentation, module ordering, or serde-util ordering).

The fix distinguishes `smithy.api#default: null` from a non-null default in
requiredness and builder construction. It restores optional Rust fields and
conditional serialization for modeled nullable defaults.

## Remaining work

Remaining mismatches are in Bedrock Runtime (4), CloudWatch Logs (5), CodeArtifact
(2), Cognito Identity Provider (6), Config (6), DynamoDB (3), IAM (12), KMS (6),
Lambda (8), SESv2 (5), SNS (10), and SQS (4). Prioritize executable semantic code
differences; ordering-only and documentation-only diffs are last priority.

## Verification

`just conformance` completes generation, rustfmt, and generated-source parsing but
exits 1 for the remaining snapshot diffs. `cargo test --workspace`,
`cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`,
and `git diff --check` pass.
