# AWS SDK modularizer diff

- repository: `https://github.com/awslabs/aws-sdk-rust`
- revision: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`
- service: `sdk/kms`
- model: `models/kms.json`
- package: `aws-sdk-kms`
- library: `aws_sdk_kms`
- tests exclusion: `tests/**` is excluded from the comparison.

## Operations

- `op_cancel_key_deletion`: `CancelKeyDeletion`
- `op_connect_custom_key_store`: `ConnectCustomKeyStore`
- `op_create_alias`: `CreateAlias`
- `op_create_custom_key_store`: `CreateCustomKeyStore`
- `op_create_grant`: `CreateGrant`
- `op_create_key`: `CreateKey`
- `op_decrypt`: `Decrypt`
- `op_delete_alias`: `DeleteAlias`
- `op_delete_custom_key_store`: `DeleteCustomKeyStore`
- `op_delete_imported_key_material`: `DeleteImportedKeyMaterial`
- `op_derive_shared_secret`: `DeriveSharedSecret`
- `op_describe_custom_key_stores`: `DescribeCustomKeyStores`
- `op_describe_key`: `DescribeKey`
- `op_disable_key`: `DisableKey`
- `op_disable_key_rotation`: `DisableKeyRotation`
- `op_disconnect_custom_key_store`: `DisconnectCustomKeyStore`
- `op_enable_key`: `EnableKey`
- `op_enable_key_rotation`: `EnableKeyRotation`
- `op_encrypt`: `Encrypt`
- `op_generate_data_key`: `GenerateDataKey`
- `op_generate_data_key_pair`: `GenerateDataKeyPair`
- `op_generate_data_key_pair_without_plaintext`: `GenerateDataKeyPairWithoutPlaintext`
- `op_generate_data_key_without_plaintext`: `GenerateDataKeyWithoutPlaintext`
- `op_generate_mac`: `GenerateMac`
- `op_generate_random`: `GenerateRandom`
- `op_get_key_last_usage`: `GetKeyLastUsage`
- `op_get_key_policy`: `GetKeyPolicy`
- `op_get_key_rotation_status`: `GetKeyRotationStatus`
- `op_get_parameters_for_import`: `GetParametersForImport`
- `op_get_public_key`: `GetPublicKey`
- `op_import_key_material`: `ImportKeyMaterial`
- `op_list_aliases`: `ListAliases`
- `op_list_grants`: `ListGrants`
- `op_list_key_policies`: `ListKeyPolicies`
- `op_list_key_rotations`: `ListKeyRotations`
- `op_list_keys`: `ListKeys`
- `op_list_resource_tags`: `ListResourceTags`
- `op_list_retirable_grants`: `ListRetirableGrants`
- `op_put_key_policy`: `PutKeyPolicy`
- `op_re_encrypt`: `ReEncrypt`
- `op_replicate_key`: `ReplicateKey`
- `op_retire_grant`: `RetireGrant`
- `op_revoke_grant`: `RevokeGrant`
- `op_rotate_key_on_demand`: `RotateKeyOnDemand`
- `op_schedule_key_deletion`: `ScheduleKeyDeletion`
- `op_sign`: `Sign`
- `op_tag_resource`: `TagResource`
- `op_untag_resource`: `UntagResource`
- `op_update_alias`: `UpdateAlias`
- `op_update_custom_key_store`: `UpdateCustomKeyStore`
- `op_update_key_description`: `UpdateKeyDescription`
- `op_update_primary_region`: `UpdatePrimaryRegion`
- `op_verify`: `Verify`
- `op_verify_mac`: `VerifyMac`

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
