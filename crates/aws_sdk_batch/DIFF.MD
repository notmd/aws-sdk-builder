# AWS SDK modularizer diff

- repository: `https://github.com/awslabs/aws-sdk-rust`
- revision: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`
- service: `sdk/batch`
- model: `aws-models/batch.json`
- package: `aws-sdk-batch`
- library: `aws_sdk_batch`
- tests exclusion: `tests/**` is excluded from the comparison.

## Operations

- `op_cancel_job`: `CancelJob`
- `op_create_compute_environment`: `CreateComputeEnvironment`
- `op_create_consumable_resource`: `CreateConsumableResource`
- `op_create_job_queue`: `CreateJobQueue`
- `op_create_quota_share`: `CreateQuotaShare`
- `op_create_scheduling_policy`: `CreateSchedulingPolicy`
- `op_create_service_environment`: `CreateServiceEnvironment`
- `op_delete_compute_environment`: `DeleteComputeEnvironment`
- `op_delete_consumable_resource`: `DeleteConsumableResource`
- `op_delete_job_queue`: `DeleteJobQueue`
- `op_delete_quota_share`: `DeleteQuotaShare`
- `op_delete_scheduling_policy`: `DeleteSchedulingPolicy`
- `op_delete_service_environment`: `DeleteServiceEnvironment`
- `op_deregister_job_definition`: `DeregisterJobDefinition`
- `op_describe_compute_environments`: `DescribeComputeEnvironments`
- `op_describe_consumable_resource`: `DescribeConsumableResource`
- `op_describe_job_definitions`: `DescribeJobDefinitions`
- `op_describe_job_queues`: `DescribeJobQueues`
- `op_describe_jobs`: `DescribeJobs`
- `op_describe_quota_share`: `DescribeQuotaShare`
- `op_describe_scheduling_policies`: `DescribeSchedulingPolicies`
- `op_describe_service_environments`: `DescribeServiceEnvironments`
- `op_describe_service_job`: `DescribeServiceJob`
- `op_get_job_queue_snapshot`: `GetJobQueueSnapshot`
- `op_list_consumable_resources`: `ListConsumableResources`
- `op_list_jobs`: `ListJobs`
- `op_list_jobs_by_consumable_resource`: `ListJobsByConsumableResource`
- `op_list_quota_shares`: `ListQuotaShares`
- `op_list_scheduling_policies`: `ListSchedulingPolicies`
- `op_list_service_jobs`: `ListServiceJobs`
- `op_list_tags_for_resource`: `ListTagsForResource`
- `op_register_job_definition`: `RegisterJobDefinition`
- `op_submit_job`: `SubmitJob`
- `op_submit_service_job`: `SubmitServiceJob`
- `op_tag_resource`: `TagResource`
- `op_terminate_job`: `TerminateJob`
- `op_terminate_service_job`: `TerminateServiceJob`
- `op_untag_resource`: `UntagResource`
- `op_update_compute_environment`: `UpdateComputeEnvironment`
- `op_update_consumable_resource`: `UpdateConsumableResource`
- `op_update_job_queue`: `UpdateJobQueue`
- `op_update_quota_share`: `UpdateQuotaShare`
- `op_update_scheduling_policy`: `UpdateSchedulingPolicy`
- `op_update_service_environment`: `UpdateServiceEnvironment`
- `op_update_service_job`: `UpdateServiceJob`

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
