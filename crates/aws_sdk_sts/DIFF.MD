# AWS SDK modularizer diff

- repository: `https://github.com/awslabs/aws-sdk-rust`
- revision: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`
- service: `sdk/sts`
- model: `models/sts.json`
- package: `aws-sdk-sts`
- library: `aws_sdk_sts`
- tests exclusion: `tests/**` is excluded from the comparison.

## Operations

- `op_assume_role`: `AssumeRole`
- `op_assume_role_with_saml`: `AssumeRoleWithSAML`
- `op_assume_role_with_web_identity`: `AssumeRoleWithWebIdentity`
- `op_assume_root`: `AssumeRoot`
- `op_decode_authorization_message`: `DecodeAuthorizationMessage`
- `op_get_access_key_info`: `GetAccessKeyInfo`
- `op_get_caller_identity`: `GetCallerIdentity`
- `op_get_delegated_access_token`: `GetDelegatedAccessToken`
- `op_get_federation_token`: `GetFederationToken`
- `op_get_session_token`: `GetSessionToken`
- `op_get_web_identity_token`: `GetWebIdentityToken`

## Changed files

- `Cargo.toml`
- `src/client.rs`
- `src/config.rs`
- `src/error_meta.rs`
- `src/lib.rs`
- `src/operation.rs`
- `src/protocol_serde.rs`
- `src/serde_util.rs`
- `src/types.rs`
- `src/types/builders.rs`
- `src/types/error.rs`
- `src/types/error/builders.rs`

## Customizations

- Add one non-default Cargo feature and matching cfg gates for each model operation.
- Rename the Cargo package/library from manifest data.
- Remove upstream `tests/` from generated output.

## Reproduction

```text
cargo run -p aws-sdk-modularizer -- --manifest services-manifest.json
```
