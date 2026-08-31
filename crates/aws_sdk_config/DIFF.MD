# AWS SDK modularizer diff

- repository: `https://github.com/awslabs/aws-sdk-rust`
- revision: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`
- service: `sdk/config`
- model: `aws-models/config-service.json`
- package: `aws-sdk-config`
- library: `aws_sdk_config`
- tests exclusion: `tests/**` is excluded from the comparison.

## Operations

- `op_associate_resource_types`: `AssociateResourceTypes`
- `op_batch_get_aggregate_resource_config`: `BatchGetAggregateResourceConfig`
- `op_batch_get_resource_config`: `BatchGetResourceConfig`
- `op_delete_aggregation_authorization`: `DeleteAggregationAuthorization`
- `op_delete_config_rule`: `DeleteConfigRule`
- `op_delete_configuration_aggregator`: `DeleteConfigurationAggregator`
- `op_delete_configuration_recorder`: `DeleteConfigurationRecorder`
- `op_delete_conformance_pack`: `DeleteConformancePack`
- `op_delete_connector`: `DeleteConnector`
- `op_delete_delivery_channel`: `DeleteDeliveryChannel`
- `op_delete_evaluation_results`: `DeleteEvaluationResults`
- `op_delete_organization_config_rule`: `DeleteOrganizationConfigRule`
- `op_delete_organization_conformance_pack`: `DeleteOrganizationConformancePack`
- `op_delete_pending_aggregation_request`: `DeletePendingAggregationRequest`
- `op_delete_remediation_configuration`: `DeleteRemediationConfiguration`
- `op_delete_remediation_exceptions`: `DeleteRemediationExceptions`
- `op_delete_resource_config`: `DeleteResourceConfig`
- `op_delete_retention_configuration`: `DeleteRetentionConfiguration`
- `op_delete_service_linked_configuration_recorder`: `DeleteServiceLinkedConfigurationRecorder`
- `op_delete_stored_query`: `DeleteStoredQuery`
- `op_deliver_config_snapshot`: `DeliverConfigSnapshot`
- `op_describe_aggregate_compliance_by_config_rules`: `DescribeAggregateComplianceByConfigRules`
- `op_describe_aggregate_compliance_by_conformance_packs`: `DescribeAggregateComplianceByConformancePacks`
- `op_describe_aggregation_authorizations`: `DescribeAggregationAuthorizations`
- `op_describe_compliance_by_config_rule`: `DescribeComplianceByConfigRule`
- `op_describe_compliance_by_resource`: `DescribeComplianceByResource`
- `op_describe_config_rule_evaluation_status`: `DescribeConfigRuleEvaluationStatus`
- `op_describe_config_rules`: `DescribeConfigRules`
- `op_describe_configuration_aggregators`: `DescribeConfigurationAggregators`
- `op_describe_configuration_aggregator_sources_status`: `DescribeConfigurationAggregatorSourcesStatus`
- `op_describe_configuration_recorders`: `DescribeConfigurationRecorders`
- `op_describe_configuration_recorder_status`: `DescribeConfigurationRecorderStatus`
- `op_describe_conformance_pack_compliance`: `DescribeConformancePackCompliance`
- `op_describe_conformance_packs`: `DescribeConformancePacks`
- `op_describe_conformance_pack_status`: `DescribeConformancePackStatus`
- `op_describe_delivery_channels`: `DescribeDeliveryChannels`
- `op_describe_delivery_channel_status`: `DescribeDeliveryChannelStatus`
- `op_describe_organization_config_rules`: `DescribeOrganizationConfigRules`
- `op_describe_organization_config_rule_statuses`: `DescribeOrganizationConfigRuleStatuses`
- `op_describe_organization_conformance_packs`: `DescribeOrganizationConformancePacks`
- `op_describe_organization_conformance_pack_statuses`: `DescribeOrganizationConformancePackStatuses`
- `op_describe_pending_aggregation_requests`: `DescribePendingAggregationRequests`
- `op_describe_remediation_configurations`: `DescribeRemediationConfigurations`
- `op_describe_remediation_exceptions`: `DescribeRemediationExceptions`
- `op_describe_remediation_execution_status`: `DescribeRemediationExecutionStatus`
- `op_describe_retention_configurations`: `DescribeRetentionConfigurations`
- `op_disassociate_resource_types`: `DisassociateResourceTypes`
- `op_get_aggregate_compliance_details_by_config_rule`: `GetAggregateComplianceDetailsByConfigRule`
- `op_get_aggregate_config_rule_compliance_summary`: `GetAggregateConfigRuleComplianceSummary`
- `op_get_aggregate_conformance_pack_compliance_summary`: `GetAggregateConformancePackComplianceSummary`
- `op_get_aggregate_discovered_resource_counts`: `GetAggregateDiscoveredResourceCounts`
- `op_get_aggregate_resource_config`: `GetAggregateResourceConfig`
- `op_get_compliance_details_by_config_rule`: `GetComplianceDetailsByConfigRule`
- `op_get_compliance_details_by_resource`: `GetComplianceDetailsByResource`
- `op_get_compliance_summary_by_config_rule`: `GetComplianceSummaryByConfigRule`
- `op_get_compliance_summary_by_resource_type`: `GetComplianceSummaryByResourceType`
- `op_get_conformance_pack_compliance_details`: `GetConformancePackComplianceDetails`
- `op_get_conformance_pack_compliance_summary`: `GetConformancePackComplianceSummary`
- `op_get_connector`: `GetConnector`
- `op_get_custom_rule_policy`: `GetCustomRulePolicy`
- `op_get_discovered_resource_counts`: `GetDiscoveredResourceCounts`
- `op_get_organization_config_rule_detailed_status`: `GetOrganizationConfigRuleDetailedStatus`
- `op_get_organization_conformance_pack_detailed_status`: `GetOrganizationConformancePackDetailedStatus`
- `op_get_organization_custom_rule_policy`: `GetOrganizationCustomRulePolicy`
- `op_get_resource_config_history`: `GetResourceConfigHistory`
- `op_get_resource_evaluation_summary`: `GetResourceEvaluationSummary`
- `op_get_stored_query`: `GetStoredQuery`
- `op_list_aggregate_discovered_resources`: `ListAggregateDiscoveredResources`
- `op_list_configuration_recorders`: `ListConfigurationRecorders`
- `op_list_conformance_pack_compliance_scores`: `ListConformancePackComplianceScores`
- `op_list_connectors`: `ListConnectors`
- `op_list_discovered_resources`: `ListDiscoveredResources`
- `op_list_resource_evaluations`: `ListResourceEvaluations`
- `op_list_stored_queries`: `ListStoredQueries`
- `op_list_tags_for_resource`: `ListTagsForResource`
- `op_put_aggregation_authorization`: `PutAggregationAuthorization`
- `op_put_config_rule`: `PutConfigRule`
- `op_put_configuration_aggregator`: `PutConfigurationAggregator`
- `op_put_configuration_recorder`: `PutConfigurationRecorder`
- `op_put_conformance_pack`: `PutConformancePack`
- `op_put_connector`: `PutConnector`
- `op_put_delivery_channel`: `PutDeliveryChannel`
- `op_put_evaluations`: `PutEvaluations`
- `op_put_external_evaluation`: `PutExternalEvaluation`
- `op_put_organization_config_rule`: `PutOrganizationConfigRule`
- `op_put_organization_conformance_pack`: `PutOrganizationConformancePack`
- `op_put_remediation_configurations`: `PutRemediationConfigurations`
- `op_put_remediation_exceptions`: `PutRemediationExceptions`
- `op_put_resource_config`: `PutResourceConfig`
- `op_put_retention_configuration`: `PutRetentionConfiguration`
- `op_put_service_linked_configuration_recorder`: `PutServiceLinkedConfigurationRecorder`
- `op_put_stored_query`: `PutStoredQuery`
- `op_put_third_party_service_linked_configuration_recorder`: `PutThirdPartyServiceLinkedConfigurationRecorder`
- `op_select_aggregate_resource_config`: `SelectAggregateResourceConfig`
- `op_select_resource_config`: `SelectResourceConfig`
- `op_start_config_rules_evaluation`: `StartConfigRulesEvaluation`
- `op_start_configuration_recorder`: `StartConfigurationRecorder`
- `op_start_remediation_execution`: `StartRemediationExecution`
- `op_start_resource_evaluation`: `StartResourceEvaluation`
- `op_stop_configuration_recorder`: `StopConfigurationRecorder`
- `op_tag_resource`: `TagResource`
- `op_untag_resource`: `UntagResource`

## Changed files

- `Cargo.toml`
- `src/client.rs`
- `src/error_meta.rs`
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
