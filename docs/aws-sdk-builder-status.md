# aws-sdk-builder status

Updated 2026-08-25. The generator is Rust-native and model-driven; the pinned
Smithy-RS reference is `/tmp/smithy-rs` at
`f1b64a9c0dd001d4bac4277fec4041da59c1f48d`.

## Current checkpoint

- Commit: `db41e7439` — SESv2 endpoint-based auth parity.
- AWS SDK reference: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`.
- Snapshot: 15 services, 1,133 operations, 13,167 files.
- Conformance: 13,093 exact, 74 mismatches, 0 missing, 0 extra (99.36%).
- SESv2: 1,154 exact, 5 mismatches, 0 missing (99.57%).
- `just conformance` completes generation, rustfmt, and generated-source
  parsing; it exits non-zero only because the remaining snapshot diffs exist.

## Remaining mismatches

Bedrock Runtime (4), CloudWatch Logs (5), CodeArtifact (2), Cognito Identity
Provider (9), Config (6), DynamoDB (3), IAM (12), KMS (6), Lambda (8), SESv2
(5), SNS (10), and SQS (4). The main clusters are shared
`protocol_serde.rs`/`serde_util.rs` ordering, documentation and export order,
endpoint-library formatting, and AWS customization parity.

## Verification

Required checks are `just conformance`, `cargo test --workspace`,
`cargo clippy --workspace --all-targets -- -D warnings`,
`cargo fmt --all -- --check`, and `git diff --check`.

Next work should continue with the generic protocol serde dependency-ordering
cluster, using the pinned Smithy-RS implementation as the reference.
