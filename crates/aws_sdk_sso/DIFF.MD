# AWS SDK modularizer diff

- repository: `https://github.com/awslabs/aws-sdk-rust`
- revision: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`
- service: `sdk/sso`
- model: `models/sso.json`
- package: `aws-sdk-sso`
- library: `aws_sdk_sso`
- tests exclusion: `tests/**` is excluded from the comparison.

## Operations

- `op_get_role_credentials`: `GetRoleCredentials`
- `op_list_account_roles`: `ListAccountRoles`
- `op_list_accounts`: `ListAccounts`
- `op_logout`: `Logout`

## Changed files

- `Cargo.toml`
- `src/client.rs`
- `src/config.rs`
- `src/error_meta.rs`
- `src/lens.rs`
- `src/lib.rs`
- `src/operation.rs`
- `src/protocol_serde.rs`
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
