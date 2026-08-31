# AWS SDK modularizer diff

- repository: `https://github.com/awslabs/aws-sdk-rust`
- revision: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`
- service: `sdk/s3`
- model: `aws-models/s3.json`
- package: `aws-sdk-s3`
- library: `aws_sdk_s3`
- tests exclusion: `tests/**` is excluded from the comparison.

## Operations

- `op_abort_multipart_upload`: `AbortMultipartUpload`
- `op_complete_multipart_upload`: `CompleteMultipartUpload`
- `op_copy_object`: `CopyObject`
- `op_create_bucket`: `CreateBucket`
- `op_create_bucket_metadata_configuration`: `CreateBucketMetadataConfiguration`
- `op_create_bucket_metadata_table_configuration`: `CreateBucketMetadataTableConfiguration`
- `op_create_multipart_upload`: `CreateMultipartUpload`
- `op_create_session`: `CreateSession`
- `op_delete_bucket`: `DeleteBucket`
- `op_delete_bucket_analytics_configuration`: `DeleteBucketAnalyticsConfiguration`
- `op_delete_bucket_cors`: `DeleteBucketCors`
- `op_delete_bucket_encryption`: `DeleteBucketEncryption`
- `op_delete_bucket_intelligent_tiering_configuration`: `DeleteBucketIntelligentTieringConfiguration`
- `op_delete_bucket_inventory_configuration`: `DeleteBucketInventoryConfiguration`
- `op_delete_bucket_lifecycle`: `DeleteBucketLifecycle`
- `op_delete_bucket_metadata_configuration`: `DeleteBucketMetadataConfiguration`
- `op_delete_bucket_metadata_table_configuration`: `DeleteBucketMetadataTableConfiguration`
- `op_delete_bucket_metrics_configuration`: `DeleteBucketMetricsConfiguration`
- `op_delete_bucket_ownership_controls`: `DeleteBucketOwnershipControls`
- `op_delete_bucket_policy`: `DeleteBucketPolicy`
- `op_delete_bucket_replication`: `DeleteBucketReplication`
- `op_delete_bucket_tagging`: `DeleteBucketTagging`
- `op_delete_bucket_website`: `DeleteBucketWebsite`
- `op_delete_object`: `DeleteObject`
- `op_delete_object_annotation`: `DeleteObjectAnnotation`
- `op_delete_objects`: `DeleteObjects`
- `op_delete_object_tagging`: `DeleteObjectTagging`
- `op_delete_public_access_block`: `DeletePublicAccessBlock`
- `op_get_bucket_abac`: `GetBucketAbac`
- `op_get_bucket_accelerate_configuration`: `GetBucketAccelerateConfiguration`
- `op_get_bucket_acl`: `GetBucketAcl`
- `op_get_bucket_analytics_configuration`: `GetBucketAnalyticsConfiguration`
- `op_get_bucket_cors`: `GetBucketCors`
- `op_get_bucket_encryption`: `GetBucketEncryption`
- `op_get_bucket_intelligent_tiering_configuration`: `GetBucketIntelligentTieringConfiguration`
- `op_get_bucket_inventory_configuration`: `GetBucketInventoryConfiguration`
- `op_get_bucket_lifecycle_configuration`: `GetBucketLifecycleConfiguration`
- `op_get_bucket_location`: `GetBucketLocation`
- `op_get_bucket_logging`: `GetBucketLogging`
- `op_get_bucket_metadata_configuration`: `GetBucketMetadataConfiguration`
- `op_get_bucket_metadata_table_configuration`: `GetBucketMetadataTableConfiguration`
- `op_get_bucket_metrics_configuration`: `GetBucketMetricsConfiguration`
- `op_get_bucket_notification_configuration`: `GetBucketNotificationConfiguration`
- `op_get_bucket_ownership_controls`: `GetBucketOwnershipControls`
- `op_get_bucket_policy`: `GetBucketPolicy`
- `op_get_bucket_policy_status`: `GetBucketPolicyStatus`
- `op_get_bucket_replication`: `GetBucketReplication`
- `op_get_bucket_request_payment`: `GetBucketRequestPayment`
- `op_get_bucket_tagging`: `GetBucketTagging`
- `op_get_bucket_versioning`: `GetBucketVersioning`
- `op_get_bucket_website`: `GetBucketWebsite`
- `op_get_object`: `GetObject`
- `op_get_object_acl`: `GetObjectAcl`
- `op_get_object_annotation`: `GetObjectAnnotation`
- `op_get_object_attributes`: `GetObjectAttributes`
- `op_get_object_legal_hold`: `GetObjectLegalHold`
- `op_get_object_lock_configuration`: `GetObjectLockConfiguration`
- `op_get_object_retention`: `GetObjectRetention`
- `op_get_object_tagging`: `GetObjectTagging`
- `op_get_object_torrent`: `GetObjectTorrent`
- `op_get_public_access_block`: `GetPublicAccessBlock`
- `op_head_bucket`: `HeadBucket`
- `op_head_object`: `HeadObject`
- `op_list_bucket_analytics_configurations`: `ListBucketAnalyticsConfigurations`
- `op_list_bucket_intelligent_tiering_configurations`: `ListBucketIntelligentTieringConfigurations`
- `op_list_bucket_inventory_configurations`: `ListBucketInventoryConfigurations`
- `op_list_bucket_metrics_configurations`: `ListBucketMetricsConfigurations`
- `op_list_buckets`: `ListBuckets`
- `op_list_directory_buckets`: `ListDirectoryBuckets`
- `op_list_multipart_uploads`: `ListMultipartUploads`
- `op_list_object_annotations`: `ListObjectAnnotations`
- `op_list_objects`: `ListObjects`
- `op_list_objects_v2`: `ListObjectsV2`
- `op_list_object_versions`: `ListObjectVersions`
- `op_list_parts`: `ListParts`
- `op_put_bucket_abac`: `PutBucketAbac`
- `op_put_bucket_accelerate_configuration`: `PutBucketAccelerateConfiguration`
- `op_put_bucket_acl`: `PutBucketAcl`
- `op_put_bucket_analytics_configuration`: `PutBucketAnalyticsConfiguration`
- `op_put_bucket_cors`: `PutBucketCors`
- `op_put_bucket_encryption`: `PutBucketEncryption`
- `op_put_bucket_intelligent_tiering_configuration`: `PutBucketIntelligentTieringConfiguration`
- `op_put_bucket_inventory_configuration`: `PutBucketInventoryConfiguration`
- `op_put_bucket_lifecycle_configuration`: `PutBucketLifecycleConfiguration`
- `op_put_bucket_logging`: `PutBucketLogging`
- `op_put_bucket_metrics_configuration`: `PutBucketMetricsConfiguration`
- `op_put_bucket_notification_configuration`: `PutBucketNotificationConfiguration`
- `op_put_bucket_ownership_controls`: `PutBucketOwnershipControls`
- `op_put_bucket_policy`: `PutBucketPolicy`
- `op_put_bucket_replication`: `PutBucketReplication`
- `op_put_bucket_request_payment`: `PutBucketRequestPayment`
- `op_put_bucket_tagging`: `PutBucketTagging`
- `op_put_bucket_versioning`: `PutBucketVersioning`
- `op_put_bucket_website`: `PutBucketWebsite`
- `op_put_object`: `PutObject`
- `op_put_object_acl`: `PutObjectAcl`
- `op_put_object_annotation`: `PutObjectAnnotation`
- `op_put_object_legal_hold`: `PutObjectLegalHold`
- `op_put_object_lock_configuration`: `PutObjectLockConfiguration`
- `op_put_object_retention`: `PutObjectRetention`
- `op_put_object_tagging`: `PutObjectTagging`
- `op_put_public_access_block`: `PutPublicAccessBlock`
- `op_rename_object`: `RenameObject`
- `op_restore_object`: `RestoreObject`
- `op_select_object_content`: `SelectObjectContent`
- `op_update_bucket_metadata_annotation_table_configuration`: `UpdateBucketMetadataAnnotationTableConfiguration`
- `op_update_bucket_metadata_inventory_table_configuration`: `UpdateBucketMetadataInventoryTableConfiguration`
- `op_update_bucket_metadata_journal_table_configuration`: `UpdateBucketMetadataJournalTableConfiguration`
- `op_update_object_encryption`: `UpdateObjectEncryption`
- `op_upload_part`: `UploadPart`
- `op_upload_part_copy`: `UploadPartCopy`
- `op_write_get_object_response`: `WriteGetObjectResponse`

## Changed files

- `Cargo.toml`
- `src/client.rs`
- `src/client/customize.rs`
- `src/config.rs`
- `src/error_meta.rs`
- `src/event_stream_serde.rs`
- `src/lens.rs`
- `src/operation.rs`
- `src/protocol_serde.rs`
- `src/s3_express.rs`
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
