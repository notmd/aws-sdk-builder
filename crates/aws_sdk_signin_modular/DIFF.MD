# AWS SDK modularizer diff

- repository: `https://github.com/awslabs/aws-sdk-rust`
- revision: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`
- service: `sdk/signin`
- model: `models/signin.json`
- package: `aws-sdk-signin-modular`
- library: `aws_sdk_signin_modular`
- tests exclusion: `tests/**` is excluded from the comparison.

## Operations

- `op_create_o_auth2_token`: `CreateOAuth2Token`
- `op_create_o_auth2_token_with_iam`: `CreateOAuth2TokenWithIAM`
- `op_delete_console_authorization_configuration`: `DeleteConsoleAuthorizationConfiguration`
- `op_delete_resource_permission_statement`: `DeleteResourcePermissionStatement`
- `op_get_console_authorization_configuration`: `GetConsoleAuthorizationConfiguration`
- `op_get_resource_policy`: `GetResourcePolicy`
- `op_introspect_o_auth2_token_with_iam`: `IntrospectOAuth2TokenWithIAM`
- `op_list_resource_permission_statements`: `ListResourcePermissionStatements`
- `op_put_console_authorization_configuration`: `PutConsoleAuthorizationConfiguration`
- `op_put_resource_permission_statement`: `PutResourcePermissionStatement`
- `op_revoke_o_auth2_token_with_iam`: `RevokeOAuth2TokenWithIAM`

## Changed files

- `Cargo.toml`
- `src/client.rs`
- `src/config.rs`
- `src/error_meta.rs`
- `src/lens.rs`
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
