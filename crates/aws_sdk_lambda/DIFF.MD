# AWS SDK modularizer diff

- repository: `https://github.com/awslabs/aws-sdk-rust`
- revision: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`
- service: `sdk/lambda`
- model: `aws-models/lambda.json`
- package: `aws-sdk-lambda`
- library: `aws_sdk_lambda`
- tests exclusion: `tests/**` is excluded from the comparison.

## Operations

- `op_delete_function`: `DeleteFunction`
- `op_delete_function_event_invoke_config`: `DeleteFunctionEventInvokeConfig`
- `op_delete_resource_policy`: `DeleteResourcePolicy`
- `op_get_account_settings`: `GetAccountSettings`
- `op_get_function_event_invoke_config`: `GetFunctionEventInvokeConfig`
- `op_get_resource_policy`: `GetResourcePolicy`
- `op_list_function_event_invoke_configs`: `ListFunctionEventInvokeConfigs`
- `op_list_tags`: `ListTags`
- `op_put_function_event_invoke_config`: `PutFunctionEventInvokeConfig`
- `op_put_resource_policy`: `PutResourcePolicy`
- `op_send_durable_execution_callback_failure`: `SendDurableExecutionCallbackFailure`
- `op_send_durable_execution_callback_heartbeat`: `SendDurableExecutionCallbackHeartbeat`
- `op_send_durable_execution_callback_success`: `SendDurableExecutionCallbackSuccess`
- `op_tag_resource`: `TagResource`
- `op_untag_resource`: `UntagResource`
- `op_update_function_event_invoke_config`: `UpdateFunctionEventInvokeConfig`
- `op_create_capacity_provider`: `CreateCapacityProvider`
- `op_get_capacity_provider`: `GetCapacityProvider`
- `op_update_capacity_provider`: `UpdateCapacityProvider`
- `op_delete_capacity_provider`: `DeleteCapacityProvider`
- `op_list_capacity_providers`: `ListCapacityProviders`
- `op_list_function_versions_by_capacity_provider`: `ListFunctionVersionsByCapacityProvider`
- `op_list_code_signing_configs`: `ListCodeSigningConfigs`
- `op_delete_code_signing_config`: `DeleteCodeSigningConfig`
- `op_get_code_signing_config`: `GetCodeSigningConfig`
- `op_list_functions_by_code_signing_config`: `ListFunctionsByCodeSigningConfig`
- `op_update_code_signing_config`: `UpdateCodeSigningConfig`
- `op_get_durable_execution`: `GetDurableExecution`
- `op_checkpoint_durable_execution`: `CheckpointDurableExecution`
- `op_get_durable_execution_history`: `GetDurableExecutionHistory`
- `op_get_durable_execution_state`: `GetDurableExecutionState`
- `op_stop_durable_execution`: `StopDurableExecution`
- `op_get_event_source_mapping`: `GetEventSourceMapping`
- `op_update_event_source_mapping`: `UpdateEventSourceMapping`
- `op_delete_event_source_mapping`: `DeleteEventSourceMapping`
- `op_list_event_source_mappings`: `ListEventSourceMappings`
- `op_create_function`: `CreateFunction`
- `op_list_functions`: `ListFunctions`
- `op_delete_function_concurrency`: `DeleteFunctionConcurrency`
- `op_get_function_concurrency`: `GetFunctionConcurrency`
- `op_list_provisioned_concurrency_configs`: `ListProvisionedConcurrencyConfigs`
- `op_put_function_concurrency`: `PutFunctionConcurrency`
- `op_update_function_code`: `UpdateFunctionCode`
- `op_update_function_configuration`: `UpdateFunctionConfiguration`
- `op_create_function_url_config`: `CreateFunctionUrlConfig`
- `op_delete_function_code_signing_config`: `DeleteFunctionCodeSigningConfig`
- `op_delete_function_url_config`: `DeleteFunctionUrlConfig`
- `op_get_function`: `GetFunction`
- `op_get_function_code_signing_config`: `GetFunctionCodeSigningConfig`
- `op_get_function_configuration`: `GetFunctionConfiguration`
- `op_get_function_recursion_config`: `GetFunctionRecursionConfig`
- `op_get_function_scaling_config`: `GetFunctionScalingConfig`
- `op_get_function_url_config`: `GetFunctionUrlConfig`
- `op_get_policy`: `GetPolicy`
- `op_get_runtime_management_config`: `GetRuntimeManagementConfig`
- `op_invoke`: `Invoke`
- `op_invoke_async`: `InvokeAsync`
- `op_invoke_with_response_stream`: `InvokeWithResponseStream`
- `op_list_durable_executions_by_function`: `ListDurableExecutionsByFunction`
- `op_list_function_url_configs`: `ListFunctionUrlConfigs`
- `op_put_function_code_signing_config`: `PutFunctionCodeSigningConfig`
- `op_put_function_recursion_config`: `PutFunctionRecursionConfig`
- `op_put_function_scaling_config`: `PutFunctionScalingConfig`
- `op_put_runtime_management_config`: `PutRuntimeManagementConfig`
- `op_update_function_url_config`: `UpdateFunctionUrlConfig`
- `op_create_alias`: `CreateAlias`
- `op_get_alias`: `GetAlias`
- `op_update_alias`: `UpdateAlias`
- `op_delete_alias`: `DeleteAlias`
- `op_list_aliases`: `ListAliases`
- `op_list_versions_by_function`: `ListVersionsByFunction`
- `op_list_layers`: `ListLayers`
- `op_list_layer_versions`: `ListLayerVersions`
- `op_add_layer_version_permission`: `AddLayerVersionPermission`
- `op_delete_layer_version`: `DeleteLayerVersion`
- `op_get_layer_version`: `GetLayerVersion`
- `op_get_layer_version_by_arn`: `GetLayerVersionByArn`
- `op_get_layer_version_policy`: `GetLayerVersionPolicy`
- `op_publish_layer_version`: `PublishLayerVersion`
- `op_remove_layer_version_permission`: `RemoveLayerVersionPermission`
- `op_add_permission`: `AddPermission`
- `op_remove_permission`: `RemovePermission`
- `op_put_provisioned_concurrency_config`: `PutProvisionedConcurrencyConfig`
- `op_get_provisioned_concurrency_config`: `GetProvisionedConcurrencyConfig`
- `op_delete_provisioned_concurrency_config`: `DeleteProvisionedConcurrencyConfig`

## Changed files

- `Cargo.toml`
- `src/client.rs`
- `src/client/create_code_signing_config.rs`
- `src/client/create_event_source_mapping.rs`
- `src/client/publish_version.rs`
- `src/error_meta.rs`
- `src/event_stream_serde.rs`
- `src/lens.rs`
- `src/operation.rs`
- `src/operation/create_code_signing_config.rs`
- `src/operation/create_code_signing_config/_create_code_signing_config_input.rs`
- `src/operation/create_code_signing_config/_create_code_signing_config_output.rs`
- `src/operation/create_code_signing_config/builders.rs`
- `src/operation/create_event_source_mapping.rs`
- `src/operation/create_event_source_mapping/_create_event_source_mapping_input.rs`
- `src/operation/create_event_source_mapping/_create_event_source_mapping_output.rs`
- `src/operation/create_event_source_mapping/builders.rs`
- `src/operation/publish_version.rs`
- `src/operation/publish_version/_publish_version_input.rs`
- `src/operation/publish_version/_publish_version_output.rs`
- `src/operation/publish_version/builders.rs`
- `src/protocol_serde.rs`
- `src/protocol_serde/shape_create_code_signing_config.rs`
- `src/protocol_serde/shape_create_code_signing_config_input.rs`
- `src/protocol_serde/shape_create_event_source_mapping.rs`
- `src/protocol_serde/shape_create_event_source_mapping_input.rs`
- `src/protocol_serde/shape_publish_version.rs`
- `src/protocol_serde/shape_publish_version_input.rs`
- `src/serde_util.rs`
- `src/types.rs`
- `src/types/builders.rs`
- `src/types/error.rs`
- `src/types/error/builders.rs`
- `src/waiters.rs`
- `src/waiters/matchers.rs`

## Customizations

- Add one non-default Cargo feature and matching cfg gates for each model operation.
- Rename the Cargo package/library from manifest data.
- Remove upstream `tests/` from generated output.

## Reproduction

```text
cargo run -p aws-sdk-modularizer -- --manifest services-manifest.json
```
