# AWS SDK modularizer diff

- repository: `https://github.com/awslabs/aws-sdk-rust`
- revision: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`
- service: `sdk/cloudwatchlogs`
- model: `aws-models/cloudwatch-logs.json`
- package: `aws-sdk-cloudwatchlogs`
- library: `aws_sdk_cloudwatchlogs`
- tests exclusion: `tests/**` is excluded from the comparison.

## Operations

- `op_associate_kms_key`: `AssociateKmsKey`
- `op_associate_source_to_s3_table_integration`: `AssociateSourceToS3TableIntegration`
- `op_cancel_export_task`: `CancelExportTask`
- `op_cancel_import_task`: `CancelImportTask`
- `op_create_delivery`: `CreateDelivery`
- `op_create_export_task`: `CreateExportTask`
- `op_create_import_task`: `CreateImportTask`
- `op_create_log_anomaly_detector`: `CreateLogAnomalyDetector`
- `op_create_log_group`: `CreateLogGroup`
- `op_create_log_stream`: `CreateLogStream`
- `op_create_lookup_table`: `CreateLookupTable`
- `op_create_scheduled_query`: `CreateScheduledQuery`
- `op_delete_account_policy`: `DeleteAccountPolicy`
- `op_delete_data_protection_policy`: `DeleteDataProtectionPolicy`
- `op_delete_delivery`: `DeleteDelivery`
- `op_delete_delivery_destination`: `DeleteDeliveryDestination`
- `op_delete_delivery_destination_policy`: `DeleteDeliveryDestinationPolicy`
- `op_delete_delivery_source`: `DeleteDeliverySource`
- `op_delete_destination`: `DeleteDestination`
- `op_delete_index_policy`: `DeleteIndexPolicy`
- `op_delete_integration`: `DeleteIntegration`
- `op_delete_log_anomaly_detector`: `DeleteLogAnomalyDetector`
- `op_delete_log_group`: `DeleteLogGroup`
- `op_delete_log_stream`: `DeleteLogStream`
- `op_delete_lookup_table`: `DeleteLookupTable`
- `op_delete_metric_filter`: `DeleteMetricFilter`
- `op_delete_query_definition`: `DeleteQueryDefinition`
- `op_delete_resource_policy`: `DeleteResourcePolicy`
- `op_delete_retention_policy`: `DeleteRetentionPolicy`
- `op_delete_scheduled_query`: `DeleteScheduledQuery`
- `op_delete_subscription_filter`: `DeleteSubscriptionFilter`
- `op_delete_syslog_configuration`: `DeleteSyslogConfiguration`
- `op_delete_transformer`: `DeleteTransformer`
- `op_describe_account_policies`: `DescribeAccountPolicies`
- `op_describe_configuration_templates`: `DescribeConfigurationTemplates`
- `op_describe_deliveries`: `DescribeDeliveries`
- `op_describe_delivery_destinations`: `DescribeDeliveryDestinations`
- `op_describe_delivery_sources`: `DescribeDeliverySources`
- `op_describe_destinations`: `DescribeDestinations`
- `op_describe_export_tasks`: `DescribeExportTasks`
- `op_describe_field_indexes`: `DescribeFieldIndexes`
- `op_describe_import_task_batches`: `DescribeImportTaskBatches`
- `op_describe_import_tasks`: `DescribeImportTasks`
- `op_describe_index_policies`: `DescribeIndexPolicies`
- `op_describe_log_groups`: `DescribeLogGroups`
- `op_describe_log_streams`: `DescribeLogStreams`
- `op_describe_lookup_tables`: `DescribeLookupTables`
- `op_describe_metric_filters`: `DescribeMetricFilters`
- `op_describe_queries`: `DescribeQueries`
- `op_describe_query_definitions`: `DescribeQueryDefinitions`
- `op_describe_resource_policies`: `DescribeResourcePolicies`
- `op_describe_subscription_filters`: `DescribeSubscriptionFilters`
- `op_disassociate_kms_key`: `DisassociateKmsKey`
- `op_disassociate_source_from_s3_table_integration`: `DisassociateSourceFromS3TableIntegration`
- `op_filter_log_events`: `FilterLogEvents`
- `op_get_data_protection_policy`: `GetDataProtectionPolicy`
- `op_get_delivery`: `GetDelivery`
- `op_get_delivery_destination`: `GetDeliveryDestination`
- `op_get_delivery_destination_policy`: `GetDeliveryDestinationPolicy`
- `op_get_delivery_source`: `GetDeliverySource`
- `op_get_integration`: `GetIntegration`
- `op_get_log_anomaly_detector`: `GetLogAnomalyDetector`
- `op_get_log_events`: `GetLogEvents`
- `op_get_log_fields`: `GetLogFields`
- `op_get_log_group_fields`: `GetLogGroupFields`
- `op_get_log_object`: `GetLogObject`
- `op_get_log_record`: `GetLogRecord`
- `op_get_lookup_table`: `GetLookupTable`
- `op_get_query_results`: `GetQueryResults`
- `op_get_scheduled_query`: `GetScheduledQuery`
- `op_get_scheduled_query_history`: `GetScheduledQueryHistory`
- `op_get_storage_tier_policy`: `GetStorageTierPolicy`
- `op_get_transformer`: `GetTransformer`
- `op_list_aggregate_log_group_summaries`: `ListAggregateLogGroupSummaries`
- `op_list_anomalies`: `ListAnomalies`
- `op_list_integrations`: `ListIntegrations`
- `op_list_log_anomaly_detectors`: `ListLogAnomalyDetectors`
- `op_list_log_groups`: `ListLogGroups`
- `op_list_log_groups_for_query`: `ListLogGroupsForQuery`
- `op_list_scheduled_queries`: `ListScheduledQueries`
- `op_list_sources_for_s3_table_integration`: `ListSourcesForS3TableIntegration`
- `op_list_syslog_configurations`: `ListSyslogConfigurations`
- `op_list_tags_for_resource`: `ListTagsForResource`
- `op_list_tags_log_group`: `ListTagsLogGroup`
- `op_put_account_policy`: `PutAccountPolicy`
- `op_put_bearer_token_authentication`: `PutBearerTokenAuthentication`
- `op_put_data_protection_policy`: `PutDataProtectionPolicy`
- `op_put_delivery_destination`: `PutDeliveryDestination`
- `op_put_delivery_destination_policy`: `PutDeliveryDestinationPolicy`
- `op_put_delivery_source`: `PutDeliverySource`
- `op_put_destination`: `PutDestination`
- `op_put_destination_policy`: `PutDestinationPolicy`
- `op_put_index_policy`: `PutIndexPolicy`
- `op_put_integration`: `PutIntegration`
- `op_put_log_events`: `PutLogEvents`
- `op_put_log_group_deletion_protection`: `PutLogGroupDeletionProtection`
- `op_put_metric_filter`: `PutMetricFilter`
- `op_put_query_definition`: `PutQueryDefinition`
- `op_put_resource_policy`: `PutResourcePolicy`
- `op_put_retention_policy`: `PutRetentionPolicy`
- `op_put_storage_tier_policy`: `PutStorageTierPolicy`
- `op_put_subscription_filter`: `PutSubscriptionFilter`
- `op_put_syslog_configuration`: `PutSyslogConfiguration`
- `op_put_transformer`: `PutTransformer`
- `op_start_live_tail`: `StartLiveTail`
- `op_start_query`: `StartQuery`
- `op_stop_query`: `StopQuery`
- `op_tag_log_group`: `TagLogGroup`
- `op_tag_resource`: `TagResource`
- `op_test_metric_filter`: `TestMetricFilter`
- `op_test_transformer`: `TestTransformer`
- `op_untag_log_group`: `UntagLogGroup`
- `op_untag_resource`: `UntagResource`
- `op_update_anomaly`: `UpdateAnomaly`
- `op_update_delivery_configuration`: `UpdateDeliveryConfiguration`
- `op_update_log_anomaly_detector`: `UpdateLogAnomalyDetector`
- `op_update_lookup_table`: `UpdateLookupTable`
- `op_update_scheduled_query`: `UpdateScheduledQuery`

## Changed files

- `Cargo.toml`
- `src/client.rs`
- `src/error_meta.rs`
- `src/event_stream_serde.rs`
- `src/lens.rs`
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
