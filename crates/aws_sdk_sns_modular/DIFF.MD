# AWS SDK modularizer diff

- repository: `https://github.com/awslabs/aws-sdk-rust`
- revision: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`
- service: `sdk/sns`
- model: `models/sns.json`
- package: `aws-sdk-sns-modular`
- library: `aws_sdk_sns_modular`
- tests exclusion: `tests/**` is excluded from the comparison.

## Operations

- `op_add_permission`: `AddPermission`
- `op_check_if_phone_number_is_opted_out`: `CheckIfPhoneNumberIsOptedOut`
- `op_confirm_subscription`: `ConfirmSubscription`
- `op_create_platform_application`: `CreatePlatformApplication`
- `op_create_platform_endpoint`: `CreatePlatformEndpoint`
- `op_create_sms_sandbox_phone_number`: `CreateSMSSandboxPhoneNumber`
- `op_create_topic`: `CreateTopic`
- `op_delete_endpoint`: `DeleteEndpoint`
- `op_delete_platform_application`: `DeletePlatformApplication`
- `op_delete_sms_sandbox_phone_number`: `DeleteSMSSandboxPhoneNumber`
- `op_delete_topic`: `DeleteTopic`
- `op_get_data_protection_policy`: `GetDataProtectionPolicy`
- `op_get_endpoint_attributes`: `GetEndpointAttributes`
- `op_get_platform_application_attributes`: `GetPlatformApplicationAttributes`
- `op_get_sms_attributes`: `GetSMSAttributes`
- `op_get_sms_sandbox_account_status`: `GetSMSSandboxAccountStatus`
- `op_get_subscription_attributes`: `GetSubscriptionAttributes`
- `op_get_topic_attributes`: `GetTopicAttributes`
- `op_list_endpoints_by_platform_application`: `ListEndpointsByPlatformApplication`
- `op_list_origination_numbers`: `ListOriginationNumbers`
- `op_list_phone_numbers_opted_out`: `ListPhoneNumbersOptedOut`
- `op_list_platform_applications`: `ListPlatformApplications`
- `op_list_sms_sandbox_phone_numbers`: `ListSMSSandboxPhoneNumbers`
- `op_list_subscriptions`: `ListSubscriptions`
- `op_list_subscriptions_by_topic`: `ListSubscriptionsByTopic`
- `op_list_tags_for_resource`: `ListTagsForResource`
- `op_list_topics`: `ListTopics`
- `op_opt_in_phone_number`: `OptInPhoneNumber`
- `op_publish`: `Publish`
- `op_publish_batch`: `PublishBatch`
- `op_put_data_protection_policy`: `PutDataProtectionPolicy`
- `op_remove_permission`: `RemovePermission`
- `op_set_endpoint_attributes`: `SetEndpointAttributes`
- `op_set_platform_application_attributes`: `SetPlatformApplicationAttributes`
- `op_set_sms_attributes`: `SetSMSAttributes`
- `op_set_subscription_attributes`: `SetSubscriptionAttributes`
- `op_set_topic_attributes`: `SetTopicAttributes`
- `op_subscribe`: `Subscribe`
- `op_tag_resource`: `TagResource`
- `op_unsubscribe`: `Unsubscribe`
- `op_untag_resource`: `UntagResource`
- `op_verify_sms_sandbox_phone_number`: `VerifySMSSandboxPhoneNumber`

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
