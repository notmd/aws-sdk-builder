# AWS SDK modularizer diff

- repository: `https://github.com/awslabs/aws-sdk-rust`
- revision: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`
- service: `sdk/dynamodb`
- model: `models/dynamodb.json`
- package: `aws-sdk-dynamodb-modular`
- library: `aws_sdk_dynamodb_modular`
- tests exclusion: `tests/**` is excluded from the comparison.

## Operations

- `op_batch_execute_statement`: `BatchExecuteStatement`
- `op_batch_get_item`: `BatchGetItem`
- `op_batch_write_item`: `BatchWriteItem`
- `op_create_backup`: `CreateBackup`
- `op_create_global_table`: `CreateGlobalTable`
- `op_create_table`: `CreateTable`
- `op_delete_backup`: `DeleteBackup`
- `op_delete_item`: `DeleteItem`
- `op_delete_resource_policy`: `DeleteResourcePolicy`
- `op_delete_table`: `DeleteTable`
- `op_describe_backup`: `DescribeBackup`
- `op_describe_continuous_backups`: `DescribeContinuousBackups`
- `op_describe_contributor_insights`: `DescribeContributorInsights`
- `op_describe_endpoints`: `DescribeEndpoints`
- `op_describe_export`: `DescribeExport`
- `op_describe_global_table`: `DescribeGlobalTable`
- `op_describe_global_table_settings`: `DescribeGlobalTableSettings`
- `op_describe_import`: `DescribeImport`
- `op_describe_kinesis_streaming_destination`: `DescribeKinesisStreamingDestination`
- `op_describe_limits`: `DescribeLimits`
- `op_describe_table`: `DescribeTable`
- `op_describe_table_replica_auto_scaling`: `DescribeTableReplicaAutoScaling`
- `op_describe_time_to_live`: `DescribeTimeToLive`
- `op_disable_kinesis_streaming_destination`: `DisableKinesisStreamingDestination`
- `op_enable_kinesis_streaming_destination`: `EnableKinesisStreamingDestination`
- `op_execute_statement`: `ExecuteStatement`
- `op_execute_transaction`: `ExecuteTransaction`
- `op_export_table_to_point_in_time`: `ExportTableToPointInTime`
- `op_get_item`: `GetItem`
- `op_get_resource_policy`: `GetResourcePolicy`
- `op_import_table`: `ImportTable`
- `op_list_backups`: `ListBackups`
- `op_list_contributor_insights`: `ListContributorInsights`
- `op_list_exports`: `ListExports`
- `op_list_global_tables`: `ListGlobalTables`
- `op_list_imports`: `ListImports`
- `op_list_tables`: `ListTables`
- `op_list_tags_of_resource`: `ListTagsOfResource`
- `op_put_item`: `PutItem`
- `op_put_resource_policy`: `PutResourcePolicy`
- `op_query`: `Query`
- `op_restore_table_from_backup`: `RestoreTableFromBackup`
- `op_restore_table_to_point_in_time`: `RestoreTableToPointInTime`
- `op_scan`: `Scan`
- `op_search_vectors`: `SearchVectors`
- `op_tag_resource`: `TagResource`
- `op_transact_get_items`: `TransactGetItems`
- `op_transact_write_items`: `TransactWriteItems`
- `op_untag_resource`: `UntagResource`
- `op_update_continuous_backups`: `UpdateContinuousBackups`
- `op_update_contributor_insights`: `UpdateContributorInsights`
- `op_update_global_table`: `UpdateGlobalTable`
- `op_update_global_table_settings`: `UpdateGlobalTableSettings`
- `op_update_item`: `UpdateItem`
- `op_update_kinesis_streaming_destination`: `UpdateKinesisStreamingDestination`
- `op_update_table`: `UpdateTable`
- `op_update_table_replica_auto_scaling`: `UpdateTableReplicaAutoScaling`
- `op_update_time_to_live`: `UpdateTimeToLive`

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
