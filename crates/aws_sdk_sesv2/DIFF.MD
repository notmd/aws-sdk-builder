# AWS SDK modularizer diff

- repository: `https://github.com/awslabs/aws-sdk-rust`
- revision: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`
- service: `sdk/sesv2`
- model: `aws-models/sesv2.json`
- package: `aws-sdk-sesv2`
- library: `aws_sdk_sesv2`
- tests exclusion: `tests/**` is excluded from the comparison.

## Operations

- `op_batch_get_metric_data`: `BatchGetMetricData`
- `op_cancel_export_job`: `CancelExportJob`
- `op_create_configuration_set`: `CreateConfigurationSet`
- `op_create_configuration_set_event_destination`: `CreateConfigurationSetEventDestination`
- `op_create_contact`: `CreateContact`
- `op_create_contact_list`: `CreateContactList`
- `op_create_custom_verification_email_template`: `CreateCustomVerificationEmailTemplate`
- `op_create_dedicated_ip_pool`: `CreateDedicatedIpPool`
- `op_create_deliverability_test_report`: `CreateDeliverabilityTestReport`
- `op_create_email_identity`: `CreateEmailIdentity`
- `op_create_email_identity_policy`: `CreateEmailIdentityPolicy`
- `op_create_email_template`: `CreateEmailTemplate`
- `op_create_export_job`: `CreateExportJob`
- `op_create_import_job`: `CreateImportJob`
- `op_create_multi_region_endpoint`: `CreateMultiRegionEndpoint`
- `op_create_tenant`: `CreateTenant`
- `op_create_tenant_resource_association`: `CreateTenantResourceAssociation`
- `op_delete_configuration_set`: `DeleteConfigurationSet`
- `op_delete_configuration_set_event_destination`: `DeleteConfigurationSetEventDestination`
- `op_delete_contact`: `DeleteContact`
- `op_delete_contact_list`: `DeleteContactList`
- `op_delete_custom_verification_email_template`: `DeleteCustomVerificationEmailTemplate`
- `op_delete_dedicated_ip_pool`: `DeleteDedicatedIpPool`
- `op_delete_email_identity`: `DeleteEmailIdentity`
- `op_delete_email_identity_policy`: `DeleteEmailIdentityPolicy`
- `op_delete_email_template`: `DeleteEmailTemplate`
- `op_delete_multi_region_endpoint`: `DeleteMultiRegionEndpoint`
- `op_delete_suppressed_destination`: `DeleteSuppressedDestination`
- `op_delete_tenant`: `DeleteTenant`
- `op_delete_tenant_resource_association`: `DeleteTenantResourceAssociation`
- `op_get_account`: `GetAccount`
- `op_get_blacklist_reports`: `GetBlacklistReports`
- `op_get_configuration_set`: `GetConfigurationSet`
- `op_get_configuration_set_event_destinations`: `GetConfigurationSetEventDestinations`
- `op_get_contact`: `GetContact`
- `op_get_contact_list`: `GetContactList`
- `op_get_custom_verification_email_template`: `GetCustomVerificationEmailTemplate`
- `op_get_dedicated_ip`: `GetDedicatedIp`
- `op_get_dedicated_ip_pool`: `GetDedicatedIpPool`
- `op_get_dedicated_ips`: `GetDedicatedIps`
- `op_get_deliverability_dashboard_options`: `GetDeliverabilityDashboardOptions`
- `op_get_deliverability_test_report`: `GetDeliverabilityTestReport`
- `op_get_domain_deliverability_campaign`: `GetDomainDeliverabilityCampaign`
- `op_get_domain_statistics_report`: `GetDomainStatisticsReport`
- `op_get_email_address_insights`: `GetEmailAddressInsights`
- `op_get_email_identity`: `GetEmailIdentity`
- `op_get_email_identity_policies`: `GetEmailIdentityPolicies`
- `op_get_email_template`: `GetEmailTemplate`
- `op_get_export_job`: `GetExportJob`
- `op_get_import_job`: `GetImportJob`
- `op_get_message_insights`: `GetMessageInsights`
- `op_get_multi_region_endpoint`: `GetMultiRegionEndpoint`
- `op_get_reputation_entity`: `GetReputationEntity`
- `op_get_suppressed_destination`: `GetSuppressedDestination`
- `op_get_tenant`: `GetTenant`
- `op_list_configuration_sets`: `ListConfigurationSets`
- `op_list_contact_lists`: `ListContactLists`
- `op_list_contacts`: `ListContacts`
- `op_list_custom_verification_email_templates`: `ListCustomVerificationEmailTemplates`
- `op_list_dedicated_ip_pools`: `ListDedicatedIpPools`
- `op_list_deliverability_test_reports`: `ListDeliverabilityTestReports`
- `op_list_domain_deliverability_campaigns`: `ListDomainDeliverabilityCampaigns`
- `op_list_email_identities`: `ListEmailIdentities`
- `op_list_email_templates`: `ListEmailTemplates`
- `op_list_export_jobs`: `ListExportJobs`
- `op_list_import_jobs`: `ListImportJobs`
- `op_list_multi_region_endpoints`: `ListMultiRegionEndpoints`
- `op_list_recommendations`: `ListRecommendations`
- `op_list_reputation_entities`: `ListReputationEntities`
- `op_list_resource_tenants`: `ListResourceTenants`
- `op_list_suppressed_destinations`: `ListSuppressedDestinations`
- `op_list_tags_for_resource`: `ListTagsForResource`
- `op_list_tenant_resources`: `ListTenantResources`
- `op_list_tenants`: `ListTenants`
- `op_put_account_dedicated_ip_warmup_attributes`: `PutAccountDedicatedIpWarmupAttributes`
- `op_put_account_details`: `PutAccountDetails`
- `op_put_account_pricing_attributes`: `PutAccountPricingAttributes`
- `op_put_account_sending_attributes`: `PutAccountSendingAttributes`
- `op_put_account_suppression_attributes`: `PutAccountSuppressionAttributes`
- `op_put_account_vdm_attributes`: `PutAccountVdmAttributes`
- `op_put_configuration_set_archiving_options`: `PutConfigurationSetArchivingOptions`
- `op_put_configuration_set_delivery_options`: `PutConfigurationSetDeliveryOptions`
- `op_put_configuration_set_reputation_options`: `PutConfigurationSetReputationOptions`
- `op_put_configuration_set_sending_options`: `PutConfigurationSetSendingOptions`
- `op_put_configuration_set_suppression_options`: `PutConfigurationSetSuppressionOptions`
- `op_put_configuration_set_tracking_options`: `PutConfigurationSetTrackingOptions`
- `op_put_configuration_set_vdm_options`: `PutConfigurationSetVdmOptions`
- `op_put_dedicated_ip_in_pool`: `PutDedicatedIpInPool`
- `op_put_dedicated_ip_pool_scaling_attributes`: `PutDedicatedIpPoolScalingAttributes`
- `op_put_dedicated_ip_warmup_attributes`: `PutDedicatedIpWarmupAttributes`
- `op_put_deliverability_dashboard_option`: `PutDeliverabilityDashboardOption`
- `op_put_email_identity_configuration_set_attributes`: `PutEmailIdentityConfigurationSetAttributes`
- `op_put_email_identity_dkim_attributes`: `PutEmailIdentityDkimAttributes`
- `op_put_email_identity_dkim_signing_attributes`: `PutEmailIdentityDkimSigningAttributes`
- `op_put_email_identity_feedback_attributes`: `PutEmailIdentityFeedbackAttributes`
- `op_put_email_identity_mail_from_attributes`: `PutEmailIdentityMailFromAttributes`
- `op_put_suppressed_destination`: `PutSuppressedDestination`
- `op_put_tenant_suppression_attributes`: `PutTenantSuppressionAttributes`
- `op_send_bulk_email`: `SendBulkEmail`
- `op_send_custom_verification_email`: `SendCustomVerificationEmail`
- `op_send_email`: `SendEmail`
- `op_tag_resource`: `TagResource`
- `op_test_render_email_template`: `TestRenderEmailTemplate`
- `op_untag_resource`: `UntagResource`
- `op_update_configuration_set_event_destination`: `UpdateConfigurationSetEventDestination`
- `op_update_contact`: `UpdateContact`
- `op_update_contact_list`: `UpdateContactList`
- `op_update_custom_verification_email_template`: `UpdateCustomVerificationEmailTemplate`
- `op_update_email_identity_policy`: `UpdateEmailIdentityPolicy`
- `op_update_email_template`: `UpdateEmailTemplate`
- `op_update_reputation_entity_customer_managed_status`: `UpdateReputationEntityCustomerManagedStatus`
- `op_update_reputation_entity_policy`: `UpdateReputationEntityPolicy`

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
