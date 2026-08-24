# AWS SDK Conformance Report: batch

Snapshot: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`

## batch
**Progress:** `762/762` files compared · `573` matched · `189` mismatches · `0` missing · `0` extra · `75.20%` match (100.00% means fully matched)

### `src/client/cancel_job.rs`

```diff
--- reference/src/client/cancel_job.rs
+++ generated/src/client/cancel_job.rs
@@ -4,7 +4,7 @@
     ///
     /// - The fluent builder is configurable:
     ///   - [`job_id(impl Into<String>)`](crate::operation::cancel_job::builders::CancelJobFluentBuilder::job_id) / [`set_job_id(Option<String>)`](crate::operation::cancel_job::builders::CancelJobFluentBuilder::set_job_id):<br>required: **true**<br><p>The Batch job ID of the job to cancel.</p><br>
-    ///   - [`reason(impl Into<String>)`](crate::operation::cancel_job::builders::CancelJobFluentBuilder::reason) / [`set_reason(Option<String>)`](crate::operation::cancel_job::builders::CancelJobFluentBuilder::set_reason):<br>required: **true**<br><p>A message to attach to the job that explains the reason for canceling it. This message is returned by future <code>DescribeJobs</code> operations on the job. It is also recorded in the Batch activity logs.</p> <p>This parameter has as limit of 1024 characters.</p><br>
+    ///   - [`reason(impl Into<String>)`](crate::operation::cancel_job::builders::CancelJobFluentBuilder::reason) / [`set_reason(Option<String>)`](crate::operation::cancel_job::builders::CancelJobFluentBuilder::set_reason):<br>required: **true**<br><p>A message to attach to the job that explains the reason for canceling it. This message is returned by future <a>DescribeJobs</a> operations on the job. It is also recorded in the Batch activity logs.</p> <p>This parameter has as limit of 1024 characters.</p><br>
     /// - On success, responds with [`CancelJobOutput`](crate::operation::cancel_job::CancelJobOutput)
     /// - On failure, responds with [`SdkError<CancelJobError>`](crate::operation::cancel_job::CancelJobError)
     pub fn cancel_job(&self) -> super::super::operation::cancel_job::builders::CancelJobFluentBuilder {
```

### `src/client/create_compute_environment.rs`

```diff
--- reference/src/client/create_compute_environment.rs
+++ generated/src/client/create_compute_environment.rs
@@ -4,7 +4,7 @@
     ///
     /// - The fluent builder is configurable:
     ///   - [`compute_environment_name(impl Into<String>)`](crate::operation::create_compute_environment::builders::CreateComputeEnvironmentFluentBuilder::compute_environment_name) / [`set_compute_environment_name(Option<String>)`](crate::operation::create_compute_environment::builders::CreateComputeEnvironmentFluentBuilder::set_compute_environment_name):<br>required: **true**<br><p>The name for your compute environment. It can be up to 128 characters long. It can contain uppercase and lowercase letters, numbers, hyphens (-), and underscores (_).</p><br>
-    ///   - [`r#type(CeType)`](crate::operation::create_compute_environment::builders::CreateComputeEnvironmentFluentBuilder::type) / [`set_type(Option<CeType>)`](crate::operation::create_compute_environment::builders::CreateComputeEnvironmentFluentBuilder::set_type):<br>required: **true**<br><p>The type of the compute environment: <code>MANAGED</code> or <code>UNMANAGED</code>. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/compute_environments.html">Compute Environments</a> in the <i>Batch User Guide</i>.</p><br>
+    ///   - [`type(CeType)`](crate::operation::create_compute_environment::builders::CreateComputeEnvironmentFluentBuilder::type) / [`set_type(Option<CeType>)`](crate::operation::create_compute_environment::builders::CreateComputeEnvironmentFluentBuilder::set_type):<br>required: **true**<br><p>The type of the compute environment: <code>MANAGED</code> or <code>UNMANAGED</code>. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/compute_environments.html">Compute Environments</a> in the <i>Batch User Guide</i>.</p><br>
     ///   - [`state(CeState)`](crate::operation::create_compute_environment::builders::CreateComputeEnvironmentFluentBuilder::state) / [`set_state(Option<CeState>)`](crate::operation::create_compute_environment::builders::CreateComputeEnvironmentFluentBuilder::set_state):<br>required: **false**<br><p>The state of the compute environment. A compute environment must be created in the <code>ENABLED</code> state.</p> <p>If the state is <code>ENABLED</code>, then the compute environment accepts jobs from a queue and can scale out automatically based on queues.</p> <p>If the state is <code>ENABLED</code>, then the Batch scheduler can attempt to place jobs from an associated job queue on the compute resources within the environment. If the compute environment is managed, then it can scale its instances out or in automatically, based on the job queue demand.</p> <p>If the state is <code>DISABLED</code>, then the Batch scheduler doesn't attempt to place jobs within the environment. Jobs in a <code>STARTING</code> or <code>RUNNING</code> state continue to progress normally. Managed compute environments in the <code>DISABLED</code> state don't scale out.</p><note>  <p>Compute environments in a <code>DISABLED</code> state may continue to incur billing charges, for example, if they have running instances due to jobs that are still executing or a non-zero <code>minvCpus</code> setting. To prevent additional charges, disable and delete the compute environment.</p> </note> <p>When an instance is idle, the instance scales down to the <code>minvCpus</code> value. However, the instance size doesn't change. For example, consider a <code>c5.8xlarge</code> instance with a <code>minvCpus</code> value of <code>4</code> and a <code>desiredvCpus</code> value of <code>36</code>. This instance doesn't scale down to a <code>c5.large</code> instance.</p><br>
     ///   - [`unmanagedv_cpus(i32)`](crate::operation::create_compute_environment::builders::CreateComputeEnvironmentFluentBuilder::unmanagedv_cpus) / [`set_unmanagedv_cpus(Option<i32>)`](crate::operation::create_compute_environment::builders::CreateComputeEnvironmentFluentBuilder::set_unmanagedv_cpus):<br>required: **false**<br><p>The maximum number of vCPUs for an unmanaged compute environment. This parameter is only used for fair-share scheduling to reserve vCPU capacity for new share identifiers. If this parameter isn't provided for a fair-share job queue, no vCPU capacity is reserved.</p><note>  <p>This parameter is only supported when the <code>type</code> parameter is set to <code>UNMANAGED</code>.</p> </note><br>
     ///   - [`compute_resources(ComputeResource)`](crate::operation::create_compute_environment::builders::CreateComputeEnvironmentFluentBuilder::compute_resources) / [`set_compute_resources(Option<ComputeResource>)`](crate::operation::create_compute_environment::builders::CreateComputeEnvironmentFluentBuilder::set_compute_resources):<br>required: **false**<br><p>Details about the compute resources managed by the compute environment. This parameter is required for managed compute environments. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/compute_environments.html">Compute Environments</a> in the <i>Batch User Guide</i>.</p><br>
```

### `src/client/create_consumable_resource.rs`

```diff
--- reference/src/client/create_consumable_resource.rs
+++ generated/src/client/create_consumable_resource.rs
@@ -8,8 +8,8 @@
     ///   - [`resource_type(impl Into<String>)`](crate::operation::create_consumable_resource::builders::CreateConsumableResourceFluentBuilder::resource_type) / [`set_resource_type(Option<String>)`](crate::operation::create_consumable_resource::builders::CreateConsumableResourceFluentBuilder::set_resource_type):<br>required: **false**<br><p>Indicates whether the resource is available to be re-used after a job completes. Can be one of:</p> <ul>  <li>   <p><code>REPLENISHABLE</code> (default)</p></li>  <li>   <p><code>NON_REPLENISHABLE</code></p></li> </ul><br>
     ///   - [`tags(impl Into<String>, impl Into<String>)`](crate::operation::create_consumable_resource::builders::CreateConsumableResourceFluentBuilder::tags) / [`set_tags(Option<HashMap::<String, String>>)`](crate::operation::create_consumable_resource::builders::CreateConsumableResourceFluentBuilder::set_tags):<br>required: **false**<br><p>The tags that you apply to the consumable resource to help you categorize and organize your resources. Each tag consists of a key and an optional value. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/using-tags.html">Tagging your Batch resources</a>.</p><br>
     /// - On success, responds with [`CreateConsumableResourceOutput`](crate::operation::create_consumable_resource::CreateConsumableResourceOutput) with field(s):
-    ///   - [`consumable_resource_name(Option<String>)`](crate::operation::create_consumable_resource::CreateConsumableResourceOutput::consumable_resource_name): <p>The name of the consumable resource.</p>
-    ///   - [`consumable_resource_arn(Option<String>)`](crate::operation::create_consumable_resource::CreateConsumableResourceOutput::consumable_resource_arn): <p>The Amazon Resource Name (ARN) of the consumable resource.</p>
+    ///   - [`consumable_resource_name(String)`](crate::operation::create_consumable_resource::CreateConsumableResourceOutput::consumable_resource_name): <p>The name of the consumable resource.</p>
+    ///   - [`consumable_resource_arn(String)`](crate::operation::create_consumable_resource::CreateConsumableResourceOutput::consumable_resource_arn): <p>The Amazon Resource Name (ARN) of the consumable resource.</p>
     /// - On failure, responds with [`SdkError<CreateConsumableResourceError>`](crate::operation::create_consumable_resource::CreateConsumableResourceError)
     pub fn create_consumable_resource(&self) -> super::super::operation::create_consumable_resource::builders::CreateConsumableResourceFluentBuilder {
         super::super::operation::create_consumable_resource::builders::CreateConsumableResourceFluentBuilder::new(self.handle.clone())
```

### `src/client/create_job_queue.rs`

```diff
--- reference/src/client/create_job_queue.rs
+++ generated/src/client/create_job_queue.rs
@@ -13,8 +13,8 @@
     ///   - [`tags(impl Into<String>, impl Into<String>)`](crate::operation::create_job_queue::builders::CreateJobQueueFluentBuilder::tags) / [`set_tags(Option<HashMap::<String, String>>)`](crate::operation::create_job_queue::builders::CreateJobQueueFluentBuilder::set_tags):<br>required: **false**<br><p>The tags that you apply to the job queue to help you categorize and organize your resources. Each tag consists of a key and an optional value. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/using-tags.html">Tagging your Batch resources</a> in <i>Batch User Guide</i>.</p><br>
     ///   - [`job_state_time_limit_actions(JobStateTimeLimitAction)`](crate::operation::create_job_queue::builders::CreateJobQueueFluentBuilder::job_state_time_limit_actions) / [`set_job_state_time_limit_actions(Option<Vec::<JobStateTimeLimitAction>>)`](crate::operation::create_job_queue::builders::CreateJobQueueFluentBuilder::set_job_state_time_limit_actions):<br>required: **false**<br><p>The set of actions that Batch performs on jobs that remain at the head of the job queue in the specified state longer than specified times. Batch will perform each action after <code>maxTimeSeconds</code> has passed. (<b>Note</b>: The minimum value for maxTimeSeconds is 600 (10 minutes) and its maximum value is 86,400 (24 hours).)</p><br>
     /// - On success, responds with [`CreateJobQueueOutput`](crate::operation::create_job_queue::CreateJobQueueOutput) with field(s):
-    ///   - [`job_queue_name(Option<String>)`](crate::operation::create_job_queue::CreateJobQueueOutput::job_queue_name): <p>The name of the job queue.</p>
-    ///   - [`job_queue_arn(Option<String>)`](crate::operation::create_job_queue::CreateJobQueueOutput::job_queue_arn): <p>The Amazon Resource Name (ARN) of the job queue.</p>
+    ///   - [`job_queue_name(String)`](crate::operation::create_job_queue::CreateJobQueueOutput::job_queue_name): <p>The name of the job queue.</p>
+    ///   - [`job_queue_arn(String)`](crate::operation::create_job_queue::CreateJobQueueOutput::job_queue_arn): <p>The Amazon Resource Name (ARN) of the job queue.</p>
     /// - On failure, responds with [`SdkError<CreateJobQueueError>`](crate::operation::create_job_queue::CreateJobQueueError)
     pub fn create_job_queue(&self) -> super::super::operation::create_job_queue::builders::CreateJobQueueFluentBuilder {
         super::super::operation::create_job_queue::builders::CreateJobQueueFluentBuilder::new(self.handle.clone())
```

### `src/client/create_scheduling_policy.rs`

```diff
--- reference/src/client/create_scheduling_policy.rs
+++ generated/src/client/create_scheduling_policy.rs
@@ -8,8 +8,8 @@
     ///   - [`fairshare_policy(FairsharePolicy)`](crate::operation::create_scheduling_policy::builders::CreateSchedulingPolicyFluentBuilder::fairshare_policy) / [`set_fairshare_policy(Option<FairsharePolicy>)`](crate::operation::create_scheduling_policy::builders::CreateSchedulingPolicyFluentBuilder::set_fairshare_policy):<br>required: **false**<br><p>The fair-share scheduling policy details. Only one of fairsharePolicy or quotaSharePolicy can be set. Once set, this policy type cannot be removed or changed to a quotaSharePolicy.</p><br>
     ///   - [`tags(impl Into<String>, impl Into<String>)`](crate::operation::create_scheduling_policy::builders::CreateSchedulingPolicyFluentBuilder::tags) / [`set_tags(Option<HashMap::<String, String>>)`](crate::operation::create_scheduling_policy::builders::CreateSchedulingPolicyFluentBuilder::set_tags):<br>required: **false**<br><p>The tags that you apply to the scheduling policy to help you categorize and organize your resources. Each tag consists of a key and an optional value. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services Resources</a> in <i>Amazon Web Services General Reference</i>.</p> <p>These tags can be updated or removed using the <a href="https://docs.aws.amazon.com/batch/latest/APIReference/API_TagResource.html">TagResource</a> and <a href="https://docs.aws.amazon.com/batch/latest/APIReference/API_UntagResource.html">UntagResource</a> API operations.</p><br>
     /// - On success, responds with [`CreateSchedulingPolicyOutput`](crate::operation::create_scheduling_policy::CreateSchedulingPolicyOutput) with field(s):
-    ///   - [`name(Option<String>)`](crate::operation::create_scheduling_policy::CreateSchedulingPolicyOutput::name): <p>The name of the scheduling policy.</p>
-    ///   - [`arn(Option<String>)`](crate::operation::create_scheduling_policy::CreateSchedulingPolicyOutput::arn): <p>The Amazon Resource Name (ARN) of the scheduling policy. The format is <code>aws:<i>Partition</i>:batch:<i>Region</i>:<i>Account</i>:scheduling-policy/<i>Name</i> </code>. For example, <code>aws:aws:batch:us-west-2:123456789012:scheduling-policy/MySchedulingPolicy</code>.</p>
+    ///   - [`name(String)`](crate::operation::create_scheduling_policy::CreateSchedulingPolicyOutput::name): <p>The name of the scheduling policy.</p>
+    ///   - [`arn(String)`](crate::operation::create_scheduling_policy::CreateSchedulingPolicyOutput::arn): <p>The Amazon Resource Name (ARN) of the scheduling policy. The format is <code>aws:<i>Partition</i>:batch:<i>Region</i>:<i>Account</i>:scheduling-policy/<i>Name</i> </code>. For example, <code>aws:aws:batch:us-west-2:123456789012:scheduling-policy/MySchedulingPolicy</code>.</p>
     /// - On failure, responds with [`SdkError<CreateSchedulingPolicyError>`](crate::operation::create_scheduling_policy::CreateSchedulingPolicyError)
     pub fn create_scheduling_policy(&self) -> super::super::operation::create_scheduling_policy::builders::CreateSchedulingPolicyFluentBuilder {
         super::super::operation::create_scheduling_policy::builders::CreateSchedulingPolicyFluentBuilder::new(self.handle.clone())
```

### `src/client/create_service_environment.rs`

```diff
--- reference/src/client/create_service_environment.rs
+++ generated/src/client/create_service_environment.rs
@@ -9,8 +9,8 @@
     ///   - [`capacity_limits(CapacityLimit)`](crate::operation::create_service_environment::builders::CreateServiceEnvironmentFluentBuilder::capacity_limits) / [`set_capacity_limits(Option<Vec::<CapacityLimit>>)`](crate::operation::create_service_environment::builders::CreateServiceEnvironmentFluentBuilder::set_capacity_limits):<br>required: **true**<br><p>The capacity limits for the service environment. The number of instances a job consumes is the total number of instances requested in the submit training job request resource configuration.</p><br>
     ///   - [`tags(impl Into<String>, impl Into<String>)`](crate::operation::create_service_environment::builders::CreateServiceEnvironmentFluentBuilder::tags) / [`set_tags(Option<HashMap::<String, String>>)`](crate::operation::create_service_environment::builders::CreateServiceEnvironmentFluentBuilder::set_tags):<br>required: **false**<br><p>The tags that you apply to the service environment to help you categorize and organize your resources. Each tag consists of a key and an optional value. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/using-tags.html">Tagging your Batch resources</a>.</p><br>
     /// - On success, responds with [`CreateServiceEnvironmentOutput`](crate::operation::create_service_environment::CreateServiceEnvironmentOutput) with field(s):
-    ///   - [`service_environment_name(Option<String>)`](crate::operation::create_service_environment::CreateServiceEnvironmentOutput::service_environment_name): <p>The name of the service environment.</p>
-    ///   - [`service_environment_arn(Option<String>)`](crate::operation::create_service_environment::CreateServiceEnvironmentOutput::service_environment_arn): <p>The Amazon Resource Name (ARN) of the service environment.</p>
+    ///   - [`service_environment_name(String)`](crate::operation::create_service_environment::CreateServiceEnvironmentOutput::service_environment_name): <p>The name of the service environment.</p>
+    ///   - [`service_environment_arn(String)`](crate::operation::create_service_environment::CreateServiceEnvironmentOutput::service_environment_arn): <p>The Amazon Resource Name (ARN) of the service environment.</p>
     /// - On failure, responds with [`SdkError<CreateServiceEnvironmentError>`](crate::operation::create_service_environment::CreateServiceEnvironmentError)
     pub fn create_service_environment(&self) -> super::super::operation::create_service_environment::builders::CreateServiceEnvironmentFluentBuilder {
         super::super::operation::create_service_environment::builders::CreateServiceEnvironmentFluentBuilder::new(self.handle.clone())
```

### `src/client/describe_consumable_resource.rs`

```diff
--- reference/src/client/describe_consumable_resource.rs
+++ generated/src/client/describe_consumable_resource.rs
@@ -5,8 +5,8 @@
     /// - The fluent builder is configurable:
     ///   - [`consumable_resource(impl Into<String>)`](crate::operation::describe_consumable_resource::builders::DescribeConsumableResourceFluentBuilder::consumable_resource) / [`set_consumable_resource(Option<String>)`](crate::operation::describe_consumable_resource::builders::DescribeConsumableResourceFluentBuilder::set_consumable_resource):<br>required: **true**<br><p>The name or ARN of the consumable resource whose description will be returned.</p><br>
     /// - On success, responds with [`DescribeConsumableResourceOutput`](crate::operation::describe_consumable_resource::DescribeConsumableResourceOutput) with field(s):
-    ///   - [`consumable_resource_name(Option<String>)`](crate::operation::describe_consumable_resource::DescribeConsumableResourceOutput::consumable_resource_name): <p>The name of the consumable resource.</p>
-    ///   - [`consumable_resource_arn(Option<String>)`](crate::operation::describe_consumable_resource::DescribeConsumableResourceOutput::consumable_resource_arn): <p>The Amazon Resource Name (ARN) of the consumable resource.</p>
+    ///   - [`consumable_resource_name(String)`](crate::operation::describe_consumable_resource::DescribeConsumableResourceOutput::consumable_resource_name): <p>The name of the consumable resource.</p>
+    ///   - [`consumable_resource_arn(String)`](crate::operation::describe_consumable_resource::DescribeConsumableResourceOutput::consumable_resource_arn): <p>The Amazon Resource Name (ARN) of the consumable resource.</p>
     ///   - [`total_quantity(Option<i64>)`](crate::operation::describe_consumable_resource::DescribeConsumableResourceOutput::total_quantity): <p>The total amount of the consumable resource that is available.</p>
     ///   - [`in_use_quantity(Option<i64>)`](crate::operation::describe_consumable_resource::DescribeConsumableResourceOutput::in_use_quantity): <p>The amount of the consumable resource that is currently in use.</p>
     ///   - [`available_quantity(Option<i64>)`](crate::operation::describe_consumable_resource::DescribeConsumableResourceOutput::available_quantity): <p>The amount of the consumable resource that is currently available to use.</p>
```

### `src/client/describe_service_job.rs`

```diff
--- reference/src/client/describe_service_job.rs
+++ generated/src/client/describe_service_job.rs
@@ -10,21 +10,21 @@
     ///   - [`created_at(Option<i64>)`](crate::operation::describe_service_job::DescribeServiceJobOutput::created_at): <p>The Unix timestamp (in milliseconds) for when the service job was created.</p>
     ///   - [`is_terminated(Option<bool>)`](crate::operation::describe_service_job::DescribeServiceJobOutput::is_terminated): <p>Indicates whether the service job has been terminated.</p>
     ///   - [`job_arn(Option<String>)`](crate::operation::describe_service_job::DescribeServiceJobOutput::job_arn): <p>The Amazon Resource Name (ARN) of the service job.</p>
-    ///   - [`job_id(Option<String>)`](crate::operation::describe_service_job::DescribeServiceJobOutput::job_id): <p>The job ID for the service job.</p>
-    ///   - [`job_name(Option<String>)`](crate::operation::describe_service_job::DescribeServiceJobOutput::job_name): <p>The name of the service job.</p>
-    ///   - [`job_queue(Option<String>)`](crate::operation::describe_service_job::DescribeServiceJobOutput::job_queue): <p>The ARN of the job queue that the service job is associated with.</p>
+    ///   - [`job_id(String)`](crate::operation::describe_service_job::DescribeServiceJobOutput::job_id): <p>The job ID for the service job.</p>
+    ///   - [`job_name(String)`](crate::operation::describe_service_job::DescribeServiceJobOutput::job_name): <p>The name of the service job.</p>
+    ///   - [`job_queue(String)`](crate::operation::describe_service_job::DescribeServiceJobOutput::job_queue): <p>The ARN of the job queue that the service job is associated with.</p>
     ///   - [`latest_attempt(Option<LatestServiceJobAttempt>)`](crate::operation::describe_service_job::DescribeServiceJobOutput::latest_attempt): <p>The latest attempt associated with the service job.</p>
     ///   - [`retry_strategy(Option<ServiceJobRetryStrategy>)`](crate::operation::describe_service_job::DescribeServiceJobOutput::retry_strategy): <p>The retry strategy to use for failed service jobs that are submitted with this service job.</p>
     ///   - [`scheduled_at(Option<i64>)`](crate::operation::describe_service_job::DescribeServiceJobOutput::scheduled_at): <p>The Unix timestamp (in milliseconds) for when the service job was scheduled. This represents when the service job was dispatched to SageMaker and the service job transitioned to the <code>SCHEDULED</code> state.</p>
     ///   - [`scheduling_priority(Option<i32>)`](crate::operation::describe_service_job::DescribeServiceJobOutput::scheduling_priority): <p>The scheduling priority of the service job.</p>
     ///   - [`service_request_payload(Option<String>)`](crate::operation::describe_service_job::DescribeServiceJobOutput::service_request_payload): <p>The request, in JSON, for the service that the <code>SubmitServiceJob</code> operation is queueing.</p>
-    ///   - [`service_job_type(Option<ServiceJobType>)`](crate::operation::describe_service_job::DescribeServiceJobOutput::service_job_type): <p>The type of service job. For SageMaker Training jobs, this value is <code>SAGEMAKER_TRAINING</code>.</p>
+    ///   - [`service_job_type(ServiceJobType)`](crate::operation::describe_service_job::DescribeServiceJobOutput::service_job_type): <p>The type of service job. For SageMaker Training jobs, this value is <code>SAGEMAKER_TRAINING</code>.</p>
     ///   - [`share_identifier(Option<String>)`](crate::operation::describe_service_job::DescribeServiceJobOutput::share_identifier): <p>The share identifier for the service job. This is used for fair-share scheduling.</p>
     ///   - [`quota_share_name(Option<String>)`](crate::operation::describe_service_job::DescribeServiceJobOutput::quota_share_name): <p>The name of the quota share that the service job is associated with.</p>
     ///   - [`preemption_configuration(Option<ServiceJobPreemptionConfiguration>)`](crate::operation::describe_service_job::DescribeServiceJobOutput::preemption_configuration): <p>Specifies the service job behavior when preempted.</p>
     ///   - [`preemption_summary(Option<ServiceJobPreemptionSummary>)`](crate::operation::describe_service_job::DescribeServiceJobOutput::preemption_summary): <p>Summarizes the preemptions of the service job. This field appears on a service job when it has been preempted.</p>
-    ///   - [`started_at(Option<i64>)`](crate::operation::describe_service_job::DescribeServiceJobOutput::started_at): <p>The Unix timestamp (in milliseconds) for when the service job was started.</p>
-    ///   - [`status(Option<ServiceJobStatus>)`](crate::operation::describe_service_job::DescribeServiceJobOutput::status): <p>The current status of the service job.</p>
+    ///   - [`started_at(i64)`](crate::operation::describe_service_job::DescribeServiceJobOutput::started_at): <p>The Unix timestamp (in milliseconds) for when the service job was started.</p>
+    ///   - [`status(ServiceJobStatus)`](crate::operation::describe_service_job::DescribeServiceJobOutput::status): <p>The current status of the service job.</p>
     ///   - [`status_reason(Option<String>)`](crate::operation::describe_service_job::DescribeServiceJobOutput::status_reason): <p>A short, human-readable string to provide more details for the current status of the service job.</p>
     ///   - [`stopped_at(Option<i64>)`](crate::operation::describe_service_job::DescribeServiceJobOutput::stopped_at): <p>The Unix timestamp (in milliseconds) for when the service job stopped running.</p>
     ///   - [`tags(Option<HashMap::<String, String>>)`](crate::operation::describe_service_job::DescribeServiceJobOutput::tags): <p>The tags that are associated with the service job. Each tag consists of a key and an optional value. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/using-tags.html">Tagging your Batch resources</a>.</p>
```

### `src/client/list_consumable_resources.rs`

```diff
--- reference/src/client/list_consumable_resources.rs
+++ generated/src/client/list_consumable_resources.rs
@@ -8,7 +8,7 @@
     ///   - [`max_results(i32)`](crate::operation::list_consumable_resources::builders::ListConsumableResourcesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_consumable_resources::builders::ListConsumableResourcesFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results returned by <code>ListConsumableResources</code> in paginated output. When this parameter is used, <code>ListConsumableResources</code> only returns <code>maxResults</code> results in a single page and a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListConsumableResources</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter isn't used, then <code>ListConsumableResources</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p><br>
     ///   - [`next_token(impl Into<String>)`](crate::operation::list_consumable_resources::builders::ListConsumableResourcesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_consumable_resources::builders::ListConsumableResourcesFluentBuilder::set_next_token):<br>required: **false**<br><p>The <code>nextToken</code> value returned from a previous paginated <code>ListConsumableResources</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. This value is <code>null</code> when there are no more results to return.</p><note>  <p>Treat this token as an opaque identifier that's only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note><br>
     /// - On success, responds with [`ListConsumableResourcesOutput`](crate::operation::list_consumable_resources::ListConsumableResourcesOutput) with field(s):
-    ///   - [`consumable_resources(Option<Vec::<ConsumableResourceSummary>>)`](crate::operation::list_consumable_resources::ListConsumableResourcesOutput::consumable_resources): <p>A list of consumable resources that match the request.</p>
+    ///   - [`consumable_resources(Vec::<ConsumableResourceSummary>)`](crate::operation::list_consumable_resources::ListConsumableResourcesOutput::consumable_resources): <p>A list of consumable resources that match the request.</p>
     ///   - [`next_token(Option<String>)`](crate::operation::list_consumable_resources::ListConsumableResourcesOutput::next_token): <p>The <code>nextToken</code> value to include in a future <code>ListConsumableResources</code> request. When the results of a <code>ListConsumableResources</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
     /// - On failure, responds with [`SdkError<ListConsumableResourcesError>`](crate::operation::list_consumable_resources::ListConsumableResourcesError)
     pub fn list_consumable_resources(&self) -> super::super::operation::list_consumable_resources::builders::ListConsumableResourcesFluentBuilder {
```

### `src/client/list_jobs.rs`

```diff
--- reference/src/client/list_jobs.rs
+++ generated/src/client/list_jobs.rs
@@ -10,9 +10,9 @@
     ///   - [`job_status(JobStatus)`](crate::operation::list_jobs::builders::ListJobsFluentBuilder::job_status) / [`set_job_status(Option<JobStatus>)`](crate::operation::list_jobs::builders::ListJobsFluentBuilder::set_job_status):<br>required: **false**<br><p>The job status used to filter jobs in the specified queue. If the <code>filters</code> parameter is specified, the <code>jobStatus</code> parameter is ignored and jobs with any status are returned. The exception is the <code>SHARE_IDENTIFIER</code> filter and <code>jobStatus</code> can be used together. If you don't specify a status, only <code>RUNNING</code> jobs are returned.</p><note>  <p>Array job parents are updated to <code>PENDING</code> when any child job is updated to <code>RUNNABLE</code> and remain in <code>PENDING</code> status while child jobs are running. To view these jobs, filter by <code>PENDING</code> status until all child jobs reach a terminal state.</p> </note><br>
     ///   - [`max_results(i32)`](crate::operation::list_jobs::builders::ListJobsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_jobs::builders::ListJobsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results returned by <code>ListJobs</code> in a paginated output. When this parameter is used, <code>ListJobs</code> returns up to <code>maxResults</code> results in a single page and a <code>nextToken</code> response element, if applicable. The remaining results of the initial request can be seen by sending another <code>ListJobs</code> request with the returned <code>nextToken</code> value.</p> <p>The following outlines key parameters and limitations:</p> <ul>  <li>   <p>The minimum value is 1.</p></li>  <li>   <p>When <code>--job-status</code> is used, Batch returns up to 1000 values.</p></li>  <li>   <p>When <code>--filters</code> is used, Batch returns up to 100 values.</p></li>  <li>   <p>If neither parameter is used, then <code>ListJobs</code> returns up to 1000 results (jobs that are in the <code>RUNNING</code> status) and a <code>nextToken</code> value, if applicable.</p></li> </ul><br>
     ///   - [`next_token(impl Into<String>)`](crate::operation::list_jobs::builders::ListJobsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_jobs::builders::ListJobsFluentBuilder::set_next_token):<br>required: **false**<br><p>The <code>nextToken</code> value returned from a previous paginated <code>ListJobs</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. This value is <code>null</code> when there are no more results to return.</p><note>  <p>Treat this token as an opaque identifier that's only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note><br>
-    ///   - [`filters(KeyValuesPair)`](crate::operation::list_jobs::builders::ListJobsFluentBuilder::filters) / [`set_filters(Option<Vec::<KeyValuesPair>>)`](crate::operation::list_jobs::builders::ListJobsFluentBuilder::set_filters):<br>required: **false**<br><p>The filter to apply to the query. Only one filter can be used at a time. When the filter is used, <code>jobStatus</code> is ignored with the exception that <code>SHARE_IDENTIFIER</code> and <code>jobStatus</code> can be used together. The filter doesn't apply to child jobs in an array or multi-node parallel (MNP) jobs. The results are sorted by the <code>createdAt</code> field, with the most recent jobs being first.</p><note>  <p>The <code>SHARE_IDENTIFIER</code> filter and the <code>jobStatus</code> field can be used together to filter results.</p> </note> <dl>  <dt>   JOB_NAME  </dt>  <dd>   <p>The value of the filter is a case-insensitive match for the job name. If the value ends with an asterisk (*), the filter matches any job name that begins with the string before the '*'. This corresponds to the <code>jobName</code> value. For example, <code>test1</code> matches both <code>Test1</code> and <code>test1</code>, and <code>test1*</code> matches both <code>test1</code> and <code>Test10</code>. When the <code>JOB_NAME</code> filter is used, the results are grouped by the job name and version.</p>  </dd>  <dt>   JOB_DEFINITION  </dt>  <dd>   <p>The value for the filter is the name or Amazon Resource Name (ARN) of the job definition. This corresponds to the <code>jobDefinition</code> value. The value is case sensitive. When the value for the filter is the job definition name, the results include all the jobs that used any revision of that job definition name. If the value ends with an asterisk (*), the filter matches any job definition name that begins with the string before the '*'. For example, <code>jd1</code> matches only <code>jd1</code>, and <code>jd1*</code> matches both <code>jd1</code> and <code>jd1A</code>. The version of the job definition that's used doesn't affect the sort order. When the <code>JOB_DEFINITION</code> filter is used and the ARN is used (which is in the form <code>arn:${Partition}:batch:${Region}:${Account}:job-definition/${JobDefinitionName}:${Revision}</code>), the results include jobs that used the specified revision of the job definition. Asterisk (*) isn't supported when the ARN is used.</p>  </dd>  <dt>   BEFORE_CREATED_AT  </dt>  <dd>   <p>The value for the filter is the time that's before the job was created. This corresponds to the <code>createdAt</code> value. The value is a string representation of the number of milliseconds since 00:00:00 UTC (midnight) on January 1, 1970.</p>  </dd>  <dt>   AFTER_CREATED_AT  </dt>  <dd>   <p>The value for the filter is the time that's after the job was created. This corresponds to the <code>createdAt</code> value. The value is a string representation of the number of milliseconds since 00:00:00 UTC (midnight) on January 1, 1970.</p>  </dd>  <dt>   SHARE_IDENTIFIER  </dt>  <dd>   <p>The value for the filter is the fairshare scheduling share identifier.</p>  </dd> </dl><br>
+    ///   - [`filters(KeyValuesPair)`](crate::operation::list_jobs::builders::ListJobsFluentBuilder::filters) / [`set_filters(Option<Vec::<KeyValuesPair>>)`](crate::operation::list_jobs::builders::ListJobsFluentBuilder::set_filters):<br>required: **false**<br><p>The filter to apply to the query. Only one filter can be used at a time. When the filter is used, <code>jobStatus</code> is ignored with the exception that <code>SHARE_IDENTIFIER</code> and <code>jobStatus</code> can be used together. The filter doesn't apply to child jobs in an array or multi-node parallel (MNP) jobs. The results are sorted by the <code>createdAt</code> field, with the most recent jobs being first.</p><note>  <p>The <code>SHARE_IDENTIFIER</code> filter and the <code>jobStatus</code> field can be used together to filter results.</p> </note> <dl> <dt>JOB_NAME</dt> <dd> <p>The value of the filter is a case-insensitive match for the job name. If the value ends with an asterisk (*), the filter matches any job name that begins with the string before the '*'. This corresponds to the <code>jobName</code> value. For example, <code>test1</code> matches both <code>Test1</code> and <code>test1</code>, and <code>test1*</code> matches both <code>test1</code> and <code>Test10</code>. When the <code>JOB_NAME</code> filter is used, the results are grouped by the job name and version.</p></dd> <dt>JOB_DEFINITION</dt> <dd> <p>The value for the filter is the name or Amazon Resource Name (ARN) of the job definition. This corresponds to the <code>jobDefinition</code> value. The value is case sensitive. When the value for the filter is the job definition name, the results include all the jobs that used any revision of that job definition name. If the value ends with an asterisk (*), the filter matches any job definition name that begins with the string before the '*'. For example, <code>jd1</code> matches only <code>jd1</code>, and <code>jd1*</code> matches both <code>jd1</code> and <code>jd1A</code>. The version of the job definition that's used doesn't affect the sort order. When the <code>JOB_DEFINITION</code> filter is used and the ARN is used (which is in the form <code>arn:${Partition}:batch:${Region}:${Account}:job-definition/${JobDefinitionName}:${Revision}</code>), the results include jobs that used the specified revision of the job definition. Asterisk (*) isn't supported when the ARN is used.</p></dd> <dt>BEFORE_CREATED_AT</dt> <dd> <p>The value for the filter is the time that's before the job was created. This corresponds to the <code>createdAt</code> value. The value is a string representation of the number of milliseconds since 00:00:00 UTC (midnight) on January 1, 1970.</p></dd> <dt>AFTER_CREATED_AT</dt> <dd> <p>The value for the filter is the time that's after the job was created. This corresponds to the <code>createdAt</code> value. The value is a string representation of the number of milliseconds since 00:00:00 UTC (midnight) on January 1, 1970.</p></dd> <dt>SHARE_IDENTIFIER</dt> <dd> <p>The value for the filter is the fairshare scheduling share identifier.</p></dd></dl><br>
     /// - On success, responds with [`ListJobsOutput`](crate::operation::list_jobs::ListJobsOutput) with field(s):
-    ///   - [`job_summary_list(Option<Vec::<JobSummary>>)`](crate::operation::list_jobs::ListJobsOutput::job_summary_list): <p>A list of job summaries that match the request.</p>
+    ///   - [`job_summary_list(Vec::<JobSummary>)`](crate::operation::list_jobs::ListJobsOutput::job_summary_list): <p>A list of job summaries that match the request.</p>
     ///   - [`next_token(Option<String>)`](crate::operation::list_jobs::ListJobsOutput::next_token): <p>The <code>nextToken</code> value to include in a future <code>ListJobs</code> request. When the results of a <code>ListJobs</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
     /// - On failure, responds with [`SdkError<ListJobsError>`](crate::operation::list_jobs::ListJobsError)
     pub fn list_jobs(&self) -> super::super::operation::list_jobs::builders::ListJobsFluentBuilder {
```

### `src/client/list_jobs_by_consumable_resource.rs`

```diff
--- reference/src/client/list_jobs_by_consumable_resource.rs
+++ generated/src/client/list_jobs_by_consumable_resource.rs
@@ -9,7 +9,7 @@
     ///   - [`max_results(i32)`](crate::operation::list_jobs_by_consumable_resource::builders::ListJobsByConsumableResourceFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_jobs_by_consumable_resource::builders::ListJobsByConsumableResourceFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results returned by <code>ListJobsByConsumableResource</code> in paginated output. When this parameter is used, <code>ListJobsByConsumableResource</code> only returns <code>maxResults</code> results in a single page and a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListJobsByConsumableResource</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter isn't used, then <code>ListJobsByConsumableResource</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p><br>
     ///   - [`next_token(impl Into<String>)`](crate::operation::list_jobs_by_consumable_resource::builders::ListJobsByConsumableResourceFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_jobs_by_consumable_resource::builders::ListJobsByConsumableResourceFluentBuilder::set_next_token):<br>required: **false**<br><p>The <code>nextToken</code> value returned from a previous paginated <code>ListJobsByConsumableResource</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. This value is <code>null</code> when there are no more results to return.</p><note>  <p>Treat this token as an opaque identifier that's only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note><br>
     /// - On success, responds with [`ListJobsByConsumableResourceOutput`](crate::operation::list_jobs_by_consumable_resource::ListJobsByConsumableResourceOutput) with field(s):
-    ///   - [`jobs(Option<Vec::<ListJobsByConsumableResourceSummary>>)`](crate::operation::list_jobs_by_consumable_resource::ListJobsByConsumableResourceOutput::jobs): <p>The list of jobs that require the specified consumable resources.</p>
+    ///   - [`jobs(Vec::<ListJobsByConsumableResourceSummary>)`](crate::operation::list_jobs_by_consumable_resource::ListJobsByConsumableResourceOutput::jobs): <p>The list of jobs that require the specified consumable resources.</p>
     ///   - [`next_token(Option<String>)`](crate::operation::list_jobs_by_consumable_resource::ListJobsByConsumableResourceOutput::next_token): <p>The <code>nextToken</code> value to include in a future <code>ListJobsByConsumableResource</code> request. When the results of a <code>ListJobsByConsumableResource</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
     /// - On failure, responds with [`SdkError<ListJobsByConsumableResourceError>`](crate::operation::list_jobs_by_consumable_resource::ListJobsByConsumableResourceError)
     pub fn list_jobs_by_consumable_resource(
```

### `src/client/list_service_jobs.rs`

```diff
--- reference/src/client/list_service_jobs.rs
+++ generated/src/client/list_service_jobs.rs
@@ -8,9 +8,9 @@
     ///   - [`job_status(ServiceJobStatus)`](crate::operation::list_service_jobs::builders::ListServiceJobsFluentBuilder::job_status) / [`set_job_status(Option<ServiceJobStatus>)`](crate::operation::list_service_jobs::builders::ListServiceJobsFluentBuilder::set_job_status):<br>required: **false**<br><p>The job status used to filter service jobs in the specified queue. If the <code>filters</code> parameter is specified, the <code>jobStatus</code> parameter is ignored and jobs with any status are returned. The exceptions are the <code>SHARE_IDENTIFIER</code> filter and <code>QUOTA_SHARE_NAME</code> filter, which can be used with <code>jobStatus</code>. If you don't specify a status, only <code>RUNNING</code> jobs are returned.</p><note>  <p>The <code>SHARE_IDENTIFIER</code> filter or <code>QUOTA_SHARE_NAME</code> filter can be used with the <code>jobStatus</code> field to filter results.</p> </note><br>
     ///   - [`max_results(i32)`](crate::operation::list_service_jobs::builders::ListServiceJobsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_service_jobs::builders::ListServiceJobsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results returned by <code>ListServiceJobs</code> in paginated output. When this parameter is used, <code>ListServiceJobs</code> only returns <code>maxResults</code> results in a single page and a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListServiceJobs</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter isn't used, then <code>ListServiceJobs</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p><br>
     ///   - [`next_token(impl Into<String>)`](crate::operation::list_service_jobs::builders::ListServiceJobsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_service_jobs::builders::ListServiceJobsFluentBuilder::set_next_token):<br>required: **false**<br><p>The <code>nextToken</code> value returned from a previous paginated <code>ListServiceJobs</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. This value is <code>null</code> when there are no more results to return.</p><note>  <p>Treat this token as an opaque identifier that's only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note><br>
-    ///   - [`filters(KeyValuesPair)`](crate::operation::list_service_jobs::builders::ListServiceJobsFluentBuilder::filters) / [`set_filters(Option<Vec::<KeyValuesPair>>)`](crate::operation::list_service_jobs::builders::ListServiceJobsFluentBuilder::set_filters):<br>required: **false**<br><p>The filter to apply to the query. Only one filter can be used at a time. When the filter is used, <code>jobStatus</code> is ignored with the exception that <code>SHARE_IDENTIFIER</code> or <code>QUOTA_SHARE_NAME</code> and <code>jobStatus</code> can be used together. The results are sorted by the <code>createdAt</code> field, with the most recent jobs being first.</p><note>  <p>The <code>SHARE_IDENTIFIER</code> or <code>QUOTA_SHARE_NAME</code> filter and the <code>jobStatus</code> field can be used together to filter results.</p> </note> <dl>  <dt>   JOB_NAME  </dt>  <dd>   <p>The value of the filter is a case-insensitive match for the job name. If the value ends with an asterisk (*), the filter matches any job name that begins with the string before the '*'. This corresponds to the <code>jobName</code> value. For example, <code>test1</code> matches both <code>Test1</code> and <code>test1</code>, and <code>test1*</code> matches both <code>test1</code> and <code>Test10</code>. When the <code>JOB_NAME</code> filter is used, the results are grouped by the job name and version.</p>  </dd>  <dt>   BEFORE_CREATED_AT  </dt>  <dd>   <p>The value for the filter is the time that's before the job was created. This corresponds to the <code>createdAt</code> value. The value is a string representation of the number of milliseconds since 00:00:00 UTC (midnight) on January 1, 1970.</p>  </dd>  <dt>   AFTER_CREATED_AT  </dt>  <dd>   <p>The value for the filter is the time that's after the job was created. This corresponds to the <code>createdAt</code> value. The value is a string representation of the number of milliseconds since 00:00:00 UTC (midnight) on January 1, 1970.</p>  </dd>  <dt>   SHARE_IDENTIFIER  </dt>  <dd>   <p>The value for the filter is the fairshare scheduling share identifier.</p>  </dd>  <dt>   QUOTA_SHARE_NAME  </dt>  <dd>   <p>The value for the filter is the quota management share name.</p>  </dd> </dl><br>
+    ///   - [`filters(KeyValuesPair)`](crate::operation::list_service_jobs::builders::ListServiceJobsFluentBuilder::filters) / [`set_filters(Option<Vec::<KeyValuesPair>>)`](crate::operation::list_service_jobs::builders::ListServiceJobsFluentBuilder::set_filters):<br>required: **false**<br><p>The filter to apply to the query. Only one filter can be used at a time. When the filter is used, <code>jobStatus</code> is ignored with the exception that <code>SHARE_IDENTIFIER</code> or <code>QUOTA_SHARE_NAME</code> and <code>jobStatus</code> can be used together. The results are sorted by the <code>createdAt</code> field, with the most recent jobs being first.</p><note>  <p>The <code>SHARE_IDENTIFIER</code> or <code>QUOTA_SHARE_NAME</code> filter and the <code>jobStatus</code> field can be used together to filter results.</p> </note> <dl> <dt>JOB_NAME</dt> <dd> <p>The value of the filter is a case-insensitive match for the job name. If the value ends with an asterisk (*), the filter matches any job name that begins with the string before the '*'. This corresponds to the <code>jobName</code> value. For example, <code>test1</code> matches both <code>Test1</code> and <code>test1</code>, and <code>test1*</code> matches both <code>test1</code> and <code>Test10</code>. When the <code>JOB_NAME</code> filter is used, the results are grouped by the job name and version.</p></dd> <dt>BEFORE_CREATED_AT</dt> <dd> <p>The value for the filter is the time that's before the job was created. This corresponds to the <code>createdAt</code> value. The value is a string representation of the number of milliseconds since 00:00:00 UTC (midnight) on January 1, 1970.</p></dd> <dt>AFTER_CREATED_AT</dt> <dd> <p>The value for the filter is the time that's after the job was created. This corresponds to the <code>createdAt</code> value. The value is a string representation of the number of milliseconds since 00:00:00 UTC (midnight) on January 1, 1970.</p></dd> <dt>SHARE_IDENTIFIER</dt> <dd> <p>The value for the filter is the fairshare scheduling share identifier.</p></dd> <dt>QUOTA_SHARE_NAME</dt> <dd> <p>The value for the filter is the quota management share name.</p></dd></dl><br>
     /// - On success, responds with [`ListServiceJobsOutput`](crate::operation::list_service_jobs::ListServiceJobsOutput) with field(s):
-    ///   - [`job_summary_list(Option<Vec::<ServiceJobSummary>>)`](crate::operation::list_service_jobs::ListServiceJobsOutput::job_summary_list): <p>A list of service job summaries.</p>
+    ///   - [`job_summary_list(Vec::<ServiceJobSummary>)`](crate::operation::list_service_jobs::ListServiceJobsOutput::job_summary_list): <p>A list of service job summaries.</p>
     ///   - [`next_token(Option<String>)`](crate::operation::list_service_jobs::ListServiceJobsOutput::next_token): <p>The <code>nextToken</code> value to include in a future <code>ListServiceJobs</code> request. When the results of a <code>ListServiceJobs</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
     /// - On failure, responds with [`SdkError<ListServiceJobsError>`](crate::operation::list_service_jobs::ListServiceJobsError)
     pub fn list_service_jobs(&self) -> super::super::operation::list_service_jobs::builders::ListServiceJobsFluentBuilder {
```

### `src/client/register_job_definition.rs`

```diff
--- reference/src/client/register_job_definition.rs
+++ generated/src/client/register_job_definition.rs
@@ -4,14 +4,14 @@
     ///
     /// - The fluent builder is configurable:
     ///   - [`job_definition_name(impl Into<String>)`](crate::operation::register_job_definition::builders::RegisterJobDefinitionFluentBuilder::job_definition_name) / [`set_job_definition_name(Option<String>)`](crate::operation::register_job_definition::builders::RegisterJobDefinitionFluentBuilder::set_job_definition_name):<br>required: **true**<br><p>The name of the job definition to register. It can be up to 128 letters long. It can contain uppercase and lowercase letters, numbers, hyphens (-), and underscores (_).</p><br>
-    ///   - [`r#type(JobDefinitionType)`](crate::operation::register_job_definition::builders::RegisterJobDefinitionFluentBuilder::type) / [`set_type(Option<JobDefinitionType>)`](crate::operation::register_job_definition::builders::RegisterJobDefinitionFluentBuilder::set_type):<br>required: **true**<br><p>The type of job definition. For more information about multi-node parallel jobs, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/multi-node-job-def.html">Creating a multi-node parallel job definition</a> in the <i>Batch User Guide</i>.</p> <ul>  <li>   <p>If the value is <code>container</code>, then one of the following is required: <code>containerProperties</code>, <code>ecsProperties</code>, or <code>eksProperties</code>.</p></li>  <li>   <p>If the value is <code>multinode</code>, then <code>nodeProperties</code> is required.</p></li> </ul><note>  <p>If the job is run on Fargate resources, then <code>multinode</code> isn't supported.</p> </note><br>
+    ///   - [`type(JobDefinitionType)`](crate::operation::register_job_definition::builders::RegisterJobDefinitionFluentBuilder::type) / [`set_type(Option<JobDefinitionType>)`](crate::operation::register_job_definition::builders::RegisterJobDefinitionFluentBuilder::set_type):<br>required: **true**<br><p>The type of job definition. For more information about multi-node parallel jobs, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/multi-node-job-def.html">Creating a multi-node parallel job definition</a> in the <i>Batch User Guide</i>.</p> <ul>  <li>   <p>If the value is <code>container</code>, then one of the following is required: <code>containerProperties</code>, <code>ecsProperties</code>, or <code>eksProperties</code>.</p></li>  <li>   <p>If the value is <code>multinode</code>, then <code>nodeProperties</code> is required.</p></li> </ul> <note>  <p>If the job is run on Fargate resources, then <code>multinode</code> isn't supported.</p> </note><br>
     ///   - [`parameters(impl Into<String>, impl Into<String>)`](crate::operation::register_job_definition::builders::RegisterJobDefinitionFluentBuilder::parameters) / [`set_parameters(Option<HashMap::<String, String>>)`](crate::operation::register_job_definition::builders::RegisterJobDefinitionFluentBuilder::set_parameters):<br>required: **false**<br><p>Default parameter substitution placeholders to set in the job definition. Parameters are specified as a key-value pair mapping. Parameters in a <code>SubmitJob</code> request override any corresponding parameter defaults from the job definition.</p><br>
     ///   - [`scheduling_priority(i32)`](crate::operation::register_job_definition::builders::RegisterJobDefinitionFluentBuilder::scheduling_priority) / [`set_scheduling_priority(Option<i32>)`](crate::operation::register_job_definition::builders::RegisterJobDefinitionFluentBuilder::set_scheduling_priority):<br>required: **false**<br><p>The scheduling priority for jobs that are submitted with this job definition. This only affects jobs in job queues with a fair-share policy. Jobs with a higher scheduling priority are scheduled before jobs with a lower scheduling priority.</p> <p>The minimum supported value is 0 and the maximum supported value is 9999.</p><br>
     ///   - [`container_properties(ContainerProperties)`](crate::operation::register_job_definition::builders::RegisterJobDefinitionFluentBuilder::container_properties) / [`set_container_properties(Option<ContainerProperties>)`](crate::operation::register_job_definition::builders::RegisterJobDefinitionFluentBuilder::set_container_properties):<br>required: **false**<br><p>An object with properties specific to Amazon ECS-based single-node container-based jobs. If the job definition's <code>type</code> parameter is <code>container</code>, then you must specify either <code>containerProperties</code> or <code>nodeProperties</code>. This must not be specified for Amazon EKS-based job definitions.</p><note>  <p>If the job runs on Fargate resources, then you must not specify <code>nodeProperties</code>; use only <code>containerProperties</code>.</p> </note><br>
     ///   - [`node_properties(NodeProperties)`](crate::operation::register_job_definition::builders::RegisterJobDefinitionFluentBuilder::node_properties) / [`set_node_properties(Option<NodeProperties>)`](crate::operation::register_job_definition::builders::RegisterJobDefinitionFluentBuilder::set_node_properties):<br>required: **false**<br><p>An object with properties specific to multi-node parallel jobs. If you specify node properties for a job, it becomes a multi-node parallel job. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/multi-node-parallel-jobs.html">Multi-node Parallel Jobs</a> in the <i>Batch User Guide</i>.</p><note>  <p>If the job runs on Fargate resources, then you must not specify <code>nodeProperties</code>; use <code>containerProperties</code> instead.</p> </note> <note>  <p>If the job runs on Amazon EKS resources, then you must not specify <code>nodeProperties</code>.</p> </note><br>
-    ///   - [`retry_strategy(RetryStrategy)`](crate::operation::register_job_definition::builders::RegisterJobDefinitionFluentBuilder::retry_strategy) / [`set_retry_strategy(Option<RetryStrategy>)`](crate::operation::register_job_definition::builders::RegisterJobDefinitionFluentBuilder::set_retry_strategy):<br>required: **false**<br><p>The retry strategy to use for failed jobs that are submitted with this job definition. Any retry strategy that's specified during a <code>SubmitJob</code> operation overrides the retry strategy defined here. If a job is terminated due to a timeout, it isn't retried.</p><br>
+    ///   - [`retry_strategy(RetryStrategy)`](crate::operation::register_job_definition::builders::RegisterJobDefinitionFluentBuilder::retry_strategy) / [`set_retry_strategy(Option<RetryStrategy>)`](crate::operation::register_job_definition::builders::RegisterJobDefinitionFluentBuilder::set_retry_strategy):<br>required: **false**<br><p>The retry strategy to use for failed jobs that are submitted with this job definition. Any retry strategy that's specified during a <a>SubmitJob</a> operation overrides the retry strategy defined here. If a job is terminated due to a timeout, it isn't retried.</p><br>
     ///   - [`propagate_tags(bool)`](crate::operation::register_job_definition::builders::RegisterJobDefinitionFluentBuilder::propagate_tags) / [`set_propagate_tags(Option<bool>)`](crate::operation::register_job_definition::builders::RegisterJobDefinitionFluentBuilder::set_propagate_tags):<br>required: **false**<br><p>Specifies whether to propagate the tags from the job or job definition to the corresponding Amazon ECS task. If no value is specified, the tags are not propagated. Tags can only be propagated to the tasks during task creation. For tags with the same name, job tags are given priority over job definitions tags. If the total number of combined tags from the job and job definition is over 50, the job is moved to the <code>FAILED</code> state.</p><note>  <p>If the job runs on Amazon EKS resources, then you must not specify <code>propagateTags</code>.</p> </note><br>
-    ///   - [`timeout(JobTimeout)`](crate::operation::register_job_definition::builders::RegisterJobDefinitionFluentBuilder::timeout) / [`set_timeout(Option<JobTimeout>)`](crate::operation::register_job_definition::builders::RegisterJobDefinitionFluentBuilder::set_timeout):<br>required: **false**<br><p>The timeout configuration for jobs that are submitted with this job definition, after which Batch terminates your jobs if they have not finished. If a job is terminated due to a timeout, it isn't retried. The minimum value for the timeout is 60 seconds. Any timeout configuration that's specified during a <code>SubmitJob</code> operation overrides the timeout configuration defined here. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/job_timeouts.html">Job Timeouts</a> in the <i>Batch User Guide</i>.</p><br>
+    ///   - [`timeout(JobTimeout)`](crate::operation::register_job_definition::builders::RegisterJobDefinitionFluentBuilder::timeout) / [`set_timeout(Option<JobTimeout>)`](crate::operation::register_job_definition::builders::RegisterJobDefinitionFluentBuilder::set_timeout):<br>required: **false**<br><p>The timeout configuration for jobs that are submitted with this job definition, after which Batch terminates your jobs if they have not finished. If a job is terminated due to a timeout, it isn't retried. The minimum value for the timeout is 60 seconds. Any timeout configuration that's specified during a <a>SubmitJob</a> operation overrides the timeout configuration defined here. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/job_timeouts.html">Job Timeouts</a> in the <i>Batch User Guide</i>.</p><br>
     ///   - [`tags(impl Into<String>, impl Into<String>)`](crate::operation::register_job_definition::builders::RegisterJobDefinitionFluentBuilder::tags) / [`set_tags(Option<HashMap::<String, String>>)`](crate::operation::register_job_definition::builders::RegisterJobDefinitionFluentBuilder::set_tags):<br>required: **false**<br><p>The tags that you apply to the job definition to help you categorize and organize your resources. Each tag consists of a key and an optional value. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/using-tags.html">Tagging Amazon Web Services Resources</a> in <i>Batch User Guide</i>.</p><br>
     ///   - [`platform_capabilities(PlatformCapability)`](crate::operation::register_job_definition::builders::RegisterJobDefinitionFluentBuilder::platform_capabilities) / [`set_platform_capabilities(Option<Vec::<PlatformCapability>>)`](crate::operation::register_job_definition::builders::RegisterJobDefinitionFluentBuilder::set_platform_capabilities):<br>required: **false**<br><p>The platform capabilities required by the job definition. If no value is specified, it defaults to <code>EC2</code>. To run the job on Fargate resources, specify <code>FARGATE</code>. To run the job on Amazon ECS Managed Instances, specify <code>MANAGED_INSTANCES</code>.</p> <p>Jobs with the <code>MANAGED_INSTANCES</code> platform capability must use <code>ecsProperties</code> (not <code>containerProperties</code>) and do not support multi-node parallel jobs.</p><note>  <p>If the job runs on Amazon EKS resources, then you must not specify <code>platformCapabilities</code>.</p> </note><br>
     ///   - [`eks_properties(EksProperties)`](crate::operation::register_job_definition::builders::RegisterJobDefinitionFluentBuilder::eks_properties) / [`set_eks_properties(Option<EksProperties>)`](crate::operation::register_job_definition::builders::RegisterJobDefinitionFluentBuilder::set_eks_properties):<br>required: **false**<br><p>An object with properties that are specific to Amazon EKS-based jobs. This must not be specified for Amazon ECS based job definitions.</p><br>
@@ -18,9 +18,9 @@
     ///   - [`ecs_properties(EcsProperties)`](crate::operation::register_job_definition::builders::RegisterJobDefinitionFluentBuilder::ecs_properties) / [`set_ecs_properties(Option<EcsProperties>)`](crate::operation::register_job_definition::builders::RegisterJobDefinitionFluentBuilder::set_ecs_properties):<br>required: **false**<br><p>An object with properties that are specific to Amazon ECS-based jobs. This must not be specified for Amazon EKS-based job definitions.</p><br>
     ///   - [`consumable_resource_properties(ConsumableResourceProperties)`](crate::operation::register_job_definition::builders::RegisterJobDefinitionFluentBuilder::consumable_resource_properties) / [`set_consumable_resource_properties(Option<ConsumableResourceProperties>)`](crate::operation::register_job_definition::builders::RegisterJobDefinitionFluentBuilder::set_consumable_resource_properties):<br>required: **false**<br><p>Contains a list of consumable resources required by the job.</p><br>
     /// - On success, responds with [`RegisterJobDefinitionOutput`](crate::operation::register_job_definition::RegisterJobDefinitionOutput) with field(s):
-    ///   - [`job_definition_name(Option<String>)`](crate::operation::register_job_definition::RegisterJobDefinitionOutput::job_definition_name): <p>The name of the job definition.</p>
-    ///   - [`job_definition_arn(Option<String>)`](crate::operation::register_job_definition::RegisterJobDefinitionOutput::job_definition_arn): <p>The Amazon Resource Name (ARN) of the job definition.</p>
-    ///   - [`revision(Option<i32>)`](crate::operation::register_job_definition::RegisterJobDefinitionOutput::revision): <p>The revision of the job definition.</p>
+    ///   - [`job_definition_name(String)`](crate::operation::register_job_definition::RegisterJobDefinitionOutput::job_definition_name): <p>The name of the job definition.</p>
+    ///   - [`job_definition_arn(String)`](crate::operation::register_job_definition::RegisterJobDefinitionOutput::job_definition_arn): <p>The Amazon Resource Name (ARN) of the job definition.</p>
+    ///   - [`revision(i32)`](crate::operation::register_job_definition::RegisterJobDefinitionOutput::revision): <p>The revision of the job definition.</p>
     /// - On failure, responds with [`SdkError<RegisterJobDefinitionError>`](crate::operation::register_job_definition::RegisterJobDefinitionError)
     pub fn register_job_definition(&self) -> super::super::operation::register_job_definition::builders::RegisterJobDefinitionFluentBuilder {
         super::super::operation::register_job_definition::builders::RegisterJobDefinitionFluentBuilder::new(self.handle.clone())
```

### `src/client/submit_job.rs`

```diff
--- reference/src/client/submit_job.rs
+++ generated/src/client/submit_job.rs
@@ -13,9 +13,9 @@
     ///   - [`parameters(impl Into<String>, impl Into<String>)`](crate::operation::submit_job::builders::SubmitJobFluentBuilder::parameters) / [`set_parameters(Option<HashMap::<String, String>>)`](crate::operation::submit_job::builders::SubmitJobFluentBuilder::set_parameters):<br>required: **false**<br><p>Additional parameters passed to the job that replace parameter substitution placeholders that are set in the job definition. Parameters are specified as a key and value pair mapping. Parameters in a <code>SubmitJob</code> request override any corresponding parameter defaults from the job definition.</p><br>
     ///   - [`container_overrides(ContainerOverrides)`](crate::operation::submit_job::builders::SubmitJobFluentBuilder::container_overrides) / [`set_container_overrides(Option<ContainerOverrides>)`](crate::operation::submit_job::builders::SubmitJobFluentBuilder::set_container_overrides):<br>required: **false**<br><p>An object with properties that override the defaults for the job definition that specify the name of a container in the specified job definition and the overrides it should receive. You can override the default command for a container, which is specified in the job definition or the Docker image, with a <code>command</code> override. You can also override existing environment variables on a container or add new environment variables to it with an <code>environment</code> override.</p><br>
     ///   - [`node_overrides(NodeOverrides)`](crate::operation::submit_job::builders::SubmitJobFluentBuilder::node_overrides) / [`set_node_overrides(Option<NodeOverrides>)`](crate::operation::submit_job::builders::SubmitJobFluentBuilder::set_node_overrides):<br>required: **false**<br><p>A list of node overrides in JSON format that specify the node range to target and the container overrides for that node range.</p><note>  <p>This parameter isn't applicable to jobs that are running on Fargate resources; use <code>containerOverrides</code> instead.</p> </note><br>
-    ///   - [`retry_strategy(RetryStrategy)`](crate::operation::submit_job::builders::SubmitJobFluentBuilder::retry_strategy) / [`set_retry_strategy(Option<RetryStrategy>)`](crate::operation::submit_job::builders::SubmitJobFluentBuilder::set_retry_strategy):<br>required: **false**<br><p>The retry strategy to use for failed jobs from this <code>SubmitJob</code> operation. When a retry strategy is specified here, it overrides the retry strategy defined in the job definition.</p><br>
+    ///   - [`retry_strategy(RetryStrategy)`](crate::operation::submit_job::builders::SubmitJobFluentBuilder::retry_strategy) / [`set_retry_strategy(Option<RetryStrategy>)`](crate::operation::submit_job::builders::SubmitJobFluentBuilder::set_retry_strategy):<br>required: **false**<br><p>The retry strategy to use for failed jobs from this <a>SubmitJob</a> operation. When a retry strategy is specified here, it overrides the retry strategy defined in the job definition.</p><br>
     ///   - [`propagate_tags(bool)`](crate::operation::submit_job::builders::SubmitJobFluentBuilder::propagate_tags) / [`set_propagate_tags(Option<bool>)`](crate::operation::submit_job::builders::SubmitJobFluentBuilder::set_propagate_tags):<br>required: **false**<br><p>Specifies whether to propagate the tags from the job or job definition to the corresponding Amazon ECS task. If no value is specified, the tags aren't propagated. Tags can only be propagated to the tasks during task creation. For tags with the same name, job tags are given priority over job definitions tags. If the total number of combined tags from the job and job definition is over 50, the job is moved to the <code>FAILED</code> state. When specified, this overrides the tag propagation setting in the job definition.</p><br>
-    ///   - [`timeout(JobTimeout)`](crate::operation::submit_job::builders::SubmitJobFluentBuilder::timeout) / [`set_timeout(Option<JobTimeout>)`](crate::operation::submit_job::builders::SubmitJobFluentBuilder::set_timeout):<br>required: **false**<br><p>The timeout configuration for this <code>SubmitJob</code> operation. You can specify a timeout duration after which Batch terminates your jobs if they haven't finished. If a job is terminated due to a timeout, it isn't retried. The minimum value for the timeout is 60 seconds. This configuration overrides any timeout configuration specified in the job definition. For array jobs, child jobs have the same timeout configuration as the parent job. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/job_timeouts.html">Job Timeouts</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p><br>
+    ///   - [`timeout(JobTimeout)`](crate::operation::submit_job::builders::SubmitJobFluentBuilder::timeout) / [`set_timeout(Option<JobTimeout>)`](crate::operation::submit_job::builders::SubmitJobFluentBuilder::set_timeout):<br>required: **false**<br><p>The timeout configuration for this <a>SubmitJob</a> operation. You can specify a timeout duration after which Batch terminates your jobs if they haven't finished. If a job is terminated due to a timeout, it isn't retried. The minimum value for the timeout is 60 seconds. This configuration overrides any timeout configuration specified in the job definition. For array jobs, child jobs have the same timeout configuration as the parent job. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/job_timeouts.html">Job Timeouts</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p><br>
     ///   - [`tags(impl Into<String>, impl Into<String>)`](crate::operation::submit_job::builders::SubmitJobFluentBuilder::tags) / [`set_tags(Option<HashMap::<String, String>>)`](crate::operation::submit_job::builders::SubmitJobFluentBuilder::set_tags):<br>required: **false**<br><p>The tags that you apply to the job request to help you categorize and organize your resources. Each tag consists of a key and an optional value. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services Resources</a> in <i>Amazon Web Services General Reference</i>.</p><br>
     ///   - [`eks_properties_override(EksPropertiesOverride)`](crate::operation::submit_job::builders::SubmitJobFluentBuilder::eks_properties_override) / [`set_eks_properties_override(Option<EksPropertiesOverride>)`](crate::operation::submit_job::builders::SubmitJobFluentBuilder::set_eks_properties_override):<br>required: **false**<br><p>An object, with properties that override defaults for the job definition, can only be specified for jobs that are run on Amazon EKS resources.</p><br>
     ///   - [`ecs_properties_override(EcsPropertiesOverride)`](crate::operation::submit_job::builders::SubmitJobFluentBuilder::ecs_properties_override) / [`set_ecs_properties_override(Option<EcsPropertiesOverride>)`](crate::operation::submit_job::builders::SubmitJobFluentBuilder::set_ecs_properties_override):<br>required: **false**<br><p>An object, with properties that override defaults for the job definition, can only be specified for jobs that are run on Amazon ECS resources.</p><br>
@@ -22,8 +22,8 @@
     ///   - [`consumable_resource_properties_override(ConsumableResourceProperties)`](crate::operation::submit_job::builders::SubmitJobFluentBuilder::consumable_resource_properties_override) / [`set_consumable_resource_properties_override(Option<ConsumableResourceProperties>)`](crate::operation::submit_job::builders::SubmitJobFluentBuilder::set_consumable_resource_properties_override):<br>required: **false**<br><p>An object that contains overrides for the consumable resources of a job.</p><br>
     /// - On success, responds with [`SubmitJobOutput`](crate::operation::submit_job::SubmitJobOutput) with field(s):
     ///   - [`job_arn(Option<String>)`](crate::operation::submit_job::SubmitJobOutput::job_arn): <p>The Amazon Resource Name (ARN) for the job.</p>
-    ///   - [`job_name(Option<String>)`](crate::operation::submit_job::SubmitJobOutput::job_name): <p>The name of the job.</p>
-    ///   - [`job_id(Option<String>)`](crate::operation::submit_job::SubmitJobOutput::job_id): <p>The unique identifier for the job.</p>
+    ///   - [`job_name(String)`](crate::operation::submit_job::SubmitJobOutput::job_name): <p>The name of the job.</p>
+    ///   - [`job_id(String)`](crate::operation::submit_job::SubmitJobOutput::job_id): <p>The unique identifier for the job.</p>
     /// - On failure, responds with [`SdkError<SubmitJobError>`](crate::operation::submit_job::SubmitJobError)
     pub fn submit_job(&self) -> super::super::operation::submit_job::builders::SubmitJobFluentBuilder {
         super::super::operation::submit_job::builders::SubmitJobFluentBuilder::new(self.handle.clone())
```

### `src/client/submit_service_job.rs`

```diff
--- reference/src/client/submit_service_job.rs
+++ generated/src/client/submit_service_job.rs
@@ -17,8 +17,8 @@
     ///   - [`client_token(impl Into<String>)`](crate::operation::submit_service_job::builders::SubmitServiceJobFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::submit_service_job::builders::SubmitServiceJobFluentBuilder::set_client_token):<br>required: **false**<br><p>A unique identifier for the request. This token is used to ensure idempotency of requests. If this parameter is specified and two submit requests with identical payloads and <code>clientToken</code>s are received, these requests are considered the same request and the second request is rejected.</p><br>
     /// - On success, responds with [`SubmitServiceJobOutput`](crate::operation::submit_service_job::SubmitServiceJobOutput) with field(s):
     ///   - [`job_arn(Option<String>)`](crate::operation::submit_service_job::SubmitServiceJobOutput::job_arn): <p>The Amazon Resource Name (ARN) for the service job.</p>
-    ///   - [`job_name(Option<String>)`](crate::operation::submit_service_job::SubmitServiceJobOutput::job_name): <p>The name of the service job.</p>
-    ///   - [`job_id(Option<String>)`](crate::operation::submit_service_job::SubmitServiceJobOutput::job_id): <p>The unique identifier for the service job.</p>
+    ///   - [`job_name(String)`](crate::operation::submit_service_job::SubmitServiceJobOutput::job_name): <p>The name of the service job.</p>
+    ///   - [`job_id(String)`](crate::operation::submit_service_job::SubmitServiceJobOutput::job_id): <p>The unique identifier for the service job.</p>
     /// - On failure, responds with [`SdkError<SubmitServiceJobError>`](crate::operation::submit_service_job::SubmitServiceJobError)
     pub fn submit_service_job(&self) -> super::super::operation::submit_service_job::builders::SubmitServiceJobFluentBuilder {
         super::super::operation::submit_service_job::builders::SubmitServiceJobFluentBuilder::new(self.handle.clone())
```

### `src/client/terminate_job.rs`

```diff
--- reference/src/client/terminate_job.rs
+++ generated/src/client/terminate_job.rs
@@ -4,7 +4,7 @@
     ///
     /// - The fluent builder is configurable:
     ///   - [`job_id(impl Into<String>)`](crate::operation::terminate_job::builders::TerminateJobFluentBuilder::job_id) / [`set_job_id(Option<String>)`](crate::operation::terminate_job::builders::TerminateJobFluentBuilder::set_job_id):<br>required: **true**<br><p>The Batch job ID of the job to terminate.</p><br>
-    ///   - [`reason(impl Into<String>)`](crate::operation::terminate_job::builders::TerminateJobFluentBuilder::reason) / [`set_reason(Option<String>)`](crate::operation::terminate_job::builders::TerminateJobFluentBuilder::set_reason):<br>required: **true**<br><p>A message to attach to the job that explains the reason for canceling it. This message is returned by future <code>DescribeJobs</code> operations on the job. It is also recorded in the Batch activity logs.</p> <p>This parameter has as limit of 1024 characters.</p><br>
+    ///   - [`reason(impl Into<String>)`](crate::operation::terminate_job::builders::TerminateJobFluentBuilder::reason) / [`set_reason(Option<String>)`](crate::operation::terminate_job::builders::TerminateJobFluentBuilder::set_reason):<br>required: **true**<br><p>A message to attach to the job that explains the reason for canceling it. This message is returned by future <a>DescribeJobs</a> operations on the job. It is also recorded in the Batch activity logs.</p> <p>This parameter has as limit of 1024 characters.</p><br>
     /// - On success, responds with [`TerminateJobOutput`](crate::operation::terminate_job::TerminateJobOutput)
     /// - On failure, responds with [`SdkError<TerminateJobError>`](crate::operation::terminate_job::TerminateJobError)
     pub fn terminate_job(&self) -> super::super::operation::terminate_job::builders::TerminateJobFluentBuilder {
```

### `src/client/update_consumable_resource.rs`

```diff
--- reference/src/client/update_consumable_resource.rs
+++ generated/src/client/update_consumable_resource.rs
@@ -8,8 +8,8 @@
     ///   - [`quantity(i64)`](crate::operation::update_consumable_resource::builders::UpdateConsumableResourceFluentBuilder::quantity) / [`set_quantity(Option<i64>)`](crate::operation::update_consumable_resource::builders::UpdateConsumableResourceFluentBuilder::set_quantity):<br>required: **false**<br><p>The change in the total quantity of the consumable resource. The <code>operation</code> parameter determines whether the value specified here will be the new total quantity, or the amount by which the total quantity will be increased or reduced. Must be a non-negative value.</p><br>
     ///   - [`client_token(impl Into<String>)`](crate::operation::update_consumable_resource::builders::UpdateConsumableResourceFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::update_consumable_resource::builders::UpdateConsumableResourceFluentBuilder::set_client_token):<br>required: **false**<br><p>If this parameter is specified and two update requests with identical payloads and <code>clientToken</code>s are received, these requests are considered the same request. Both requests will succeed, but the update will only happen once. A <code>clientToken</code> is valid for 8 hours.</p><br>
     /// - On success, responds with [`UpdateConsumableResourceOutput`](crate::operation::update_consumable_resource::UpdateConsumableResourceOutput) with field(s):
-    ///   - [`consumable_resource_name(Option<String>)`](crate::operation::update_consumable_resource::UpdateConsumableResourceOutput::consumable_resource_name): <p>The name of the consumable resource to be updated.</p>
-    ///   - [`consumable_resource_arn(Option<String>)`](crate::operation::update_consumable_resource::UpdateConsumableResourceOutput::consumable_resource_arn): <p>The Amazon Resource Name (ARN) of the consumable resource.</p>
+    ///   - [`consumable_resource_name(String)`](crate::operation::update_consumable_resource::UpdateConsumableResourceOutput::consumable_resource_name): <p>The name of the consumable resource to be updated.</p>
+    ///   - [`consumable_resource_arn(String)`](crate::operation::update_consumable_resource::UpdateConsumableResourceOutput::consumable_resource_arn): <p>The Amazon Resource Name (ARN) of the consumable resource.</p>
     ///   - [`total_quantity(Option<i64>)`](crate::operation::update_consumable_resource::UpdateConsumableResourceOutput::total_quantity): <p>The total amount of the consumable resource that is available.</p>
     /// - On failure, responds with [`SdkError<UpdateConsumableResourceError>`](crate::operation::update_consumable_resource::UpdateConsumableResourceError)
     pub fn update_consumable_resource(&self) -> super::super::operation::update_consumable_resource::builders::UpdateConsumableResourceFluentBuilder {
```

### `src/client/update_service_environment.rs`

```diff
--- reference/src/client/update_service_environment.rs
+++ generated/src/client/update_service_environment.rs
@@ -7,8 +7,8 @@
     ///   - [`state(ServiceEnvironmentState)`](crate::operation::update_service_environment::builders::UpdateServiceEnvironmentFluentBuilder::state) / [`set_state(Option<ServiceEnvironmentState>)`](crate::operation::update_service_environment::builders::UpdateServiceEnvironmentFluentBuilder::set_state):<br>required: **false**<br><p>The state of the service environment.</p><br>
     ///   - [`capacity_limits(CapacityLimit)`](crate::operation::update_service_environment::builders::UpdateServiceEnvironmentFluentBuilder::capacity_limits) / [`set_capacity_limits(Option<Vec::<CapacityLimit>>)`](crate::operation::update_service_environment::builders::UpdateServiceEnvironmentFluentBuilder::set_capacity_limits):<br>required: **false**<br><p>The capacity limits for the service environment. This defines the maximum resources that can be used by service jobs in this environment.</p><br>
     /// - On success, responds with [`UpdateServiceEnvironmentOutput`](crate::operation::update_service_environment::UpdateServiceEnvironmentOutput) with field(s):
-    ///   - [`service_environment_name(Option<String>)`](crate::operation::update_service_environment::UpdateServiceEnvironmentOutput::service_environment_name): <p>The name of the service environment that was updated.</p>
-    ///   - [`service_environment_arn(Option<String>)`](crate::operation::update_service_environment::UpdateServiceEnvironmentOutput::service_environment_arn): <p>The Amazon Resource Name (ARN) of the service environment that was updated.</p>
+    ///   - [`service_environment_name(String)`](crate::operation::update_service_environment::UpdateServiceEnvironmentOutput::service_environment_name): <p>The name of the service environment that was updated.</p>
+    ///   - [`service_environment_arn(String)`](crate::operation::update_service_environment::UpdateServiceEnvironmentOutput::service_environment_arn): <p>The Amazon Resource Name (ARN) of the service environment that was updated.</p>
     /// - On failure, responds with [`SdkError<UpdateServiceEnvironmentError>`](crate::operation::update_service_environment::UpdateServiceEnvironmentError)
     pub fn update_service_environment(&self) -> super::super::operation::update_service_environment::builders::UpdateServiceEnvironmentFluentBuilder {
         super::super::operation::update_service_environment::builders::UpdateServiceEnvironmentFluentBuilder::new(self.handle.clone())
```

### `src/lens.rs`

```diff
--- reference/src/lens.rs
+++ generated/src/lens.rs
@@ -130,7 +130,7 @@
 pub(crate) fn lens_list_consumable_resources_output_output_consumable_resources(
     input: super::operation::list_consumable_resources::ListConsumableResourcesOutput,
 ) -> ::std::option::Option<::std::vec::Vec<super::types::ConsumableResourceSummary>> {
-    let input = input.consumable_resources?;
+    let input = input.consumable_resources;
     ::std::option::Option::Some(input)
 }

@@ -137,7 +137,7 @@
 pub(crate) fn lens_list_jobs_output_output_job_summary_list(
     input: super::operation::list_jobs::ListJobsOutput,
 ) -> ::std::option::Option<::std::vec::Vec<super::types::JobSummary>> {
-    let input = input.job_summary_list?;
+    let input = input.job_summary_list;
     ::std::option::Option::Some(input)
 }

@@ -144,7 +144,7 @@
 pub(crate) fn lens_list_jobs_by_consumable_resource_output_output_jobs(
     input: super::operation::list_jobs_by_consumable_resource::ListJobsByConsumableResourceOutput,
 ) -> ::std::option::Option<::std::vec::Vec<super::types::ListJobsByConsumableResourceSummary>> {
-    let input = input.jobs?;
+    let input = input.jobs;
     ::std::option::Option::Some(input)
 }

@@ -165,6 +165,6 @@
 pub(crate) fn lens_list_service_jobs_output_output_job_summary_list(
     input: super::operation::list_service_jobs::ListServiceJobsOutput,
 ) -> ::std::option::Option<::std::vec::Vec<super::types::ServiceJobSummary>> {
-    let input = input.job_summary_list?;
+    let input = input.job_summary_list;
     ::std::option::Option::Some(input)
 }
```

### `src/operation/create_consumable_resource/_create_consumable_resource_output.rs`

```diff
--- reference/src/operation/create_consumable_resource/_create_consumable_resource_output.rs
+++ generated/src/operation/create_consumable_resource/_create_consumable_resource_output.rs
@@ -4,19 +4,21 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct CreateConsumableResourceOutput {
     /// <p>The name of the consumable resource.</p>
-    pub consumable_resource_name: ::std::option::Option<::std::string::String>,
+    pub consumable_resource_name: ::std::string::String,
     /// <p>The Amazon Resource Name (ARN) of the consumable resource.</p>
-    pub consumable_resource_arn: ::std::option::Option<::std::string::String>,
+    pub consumable_resource_arn: ::std::string::String,
     _request_id: Option<String>,
 }
 impl CreateConsumableResourceOutput {
     /// <p>The name of the consumable resource.</p>
-    pub fn consumable_resource_name(&self) -> ::std::option::Option<&str> {
-        self.consumable_resource_name.as_deref()
+    pub fn consumable_resource_name(&self) -> &str {
+        use std::ops::Deref;
+        self.consumable_resource_name.deref()
     }
     /// <p>The Amazon Resource Name (ARN) of the consumable resource.</p>
-    pub fn consumable_resource_arn(&self) -> ::std::option::Option<&str> {
-        self.consumable_resource_arn.as_deref()
+    pub fn consumable_resource_arn(&self) -> &str {
+        use std::ops::Deref;
+        self.consumable_resource_arn.deref()
     }
 }
 impl ::aws_types::request_id::RequestId for CreateConsumableResourceOutput {
@@ -80,11 +82,29 @@
         self
     }
     /// Consumes the builder and constructs a [`CreateConsumableResourceOutput`](crate::operation::create_consumable_resource::CreateConsumableResourceOutput).
-    pub fn build(self) -> super::super::super::operation::create_consumable_resource::CreateConsumableResourceOutput {
-        super::super::super::operation::create_consumable_resource::CreateConsumableResourceOutput {
-            consumable_resource_name: self.consumable_resource_name,
-            consumable_resource_arn: self.consumable_resource_arn,
+    /// This method will fail if any of the following fields are not set:
+    /// - [`consumable_resource_name`](crate::operation::create_consumable_resource::builders::CreateConsumableResourceOutputBuilder::consumable_resource_name)
+    /// - [`consumable_resource_arn`](crate::operation::create_consumable_resource::builders::CreateConsumableResourceOutputBuilder::consumable_resource_arn)
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<
+        super::super::super::operation::create_consumable_resource::CreateConsumableResourceOutput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
+        ::std::result::Result::Ok(super::super::super::operation::create_consumable_resource::CreateConsumableResourceOutput {
+            consumable_resource_name: self.consumable_resource_name.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "consumable_resource_name",
+                    "consumable_resource_name was not specified but it is required when building CreateConsumableResourceOutput",
+                )
+            })?,
+            consumable_resource_arn: self.consumable_resource_arn.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "consumable_resource_arn",
+                    "consumable_resource_arn was not specified but it is required when building CreateConsumableResourceOutput",
+                )
+            })?,
             _request_id: self._request_id,
-        }
+        })
     }
 }
```

### `src/operation/create_job_queue/_create_job_queue_output.rs`

```diff
--- reference/src/operation/create_job_queue/_create_job_queue_output.rs
+++ generated/src/operation/create_job_queue/_create_job_queue_output.rs
@@ -4,19 +4,21 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct CreateJobQueueOutput {
     /// <p>The name of the job queue.</p>
-    pub job_queue_name: ::std::option::Option<::std::string::String>,
+    pub job_queue_name: ::std::string::String,
     /// <p>The Amazon Resource Name (ARN) of the job queue.</p>
-    pub job_queue_arn: ::std::option::Option<::std::string::String>,
+    pub job_queue_arn: ::std::string::String,
     _request_id: Option<String>,
 }
 impl CreateJobQueueOutput {
     /// <p>The name of the job queue.</p>
-    pub fn job_queue_name(&self) -> ::std::option::Option<&str> {
-        self.job_queue_name.as_deref()
+    pub fn job_queue_name(&self) -> &str {
+        use std::ops::Deref;
+        self.job_queue_name.deref()
     }
     /// <p>The Amazon Resource Name (ARN) of the job queue.</p>
-    pub fn job_queue_arn(&self) -> ::std::option::Option<&str> {
-        self.job_queue_arn.as_deref()
+    pub fn job_queue_arn(&self) -> &str {
+        use std::ops::Deref;
+        self.job_queue_arn.deref()
     }
 }
 impl ::aws_types::request_id::RequestId for CreateJobQueueOutput {
@@ -80,11 +82,26 @@
         self
     }
     /// Consumes the builder and constructs a [`CreateJobQueueOutput`](crate::operation::create_job_queue::CreateJobQueueOutput).
-    pub fn build(self) -> super::super::super::operation::create_job_queue::CreateJobQueueOutput {
-        super::super::super::operation::create_job_queue::CreateJobQueueOutput {
-            job_queue_name: self.job_queue_name,
-            job_queue_arn: self.job_queue_arn,
+    /// This method will fail if any of the following fields are not set:
+    /// - [`job_queue_name`](crate::operation::create_job_queue::builders::CreateJobQueueOutputBuilder::job_queue_name)
+    /// - [`job_queue_arn`](crate::operation::create_job_queue::builders::CreateJobQueueOutputBuilder::job_queue_arn)
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::create_job_queue::CreateJobQueueOutput, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::super::operation::create_job_queue::CreateJobQueueOutput {
+            job_queue_name: self.job_queue_name.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "job_queue_name",
+                    "job_queue_name was not specified but it is required when building CreateJobQueueOutput",
+                )
+            })?,
+            job_queue_arn: self.job_queue_arn.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "job_queue_arn",
+                    "job_queue_arn was not specified but it is required when building CreateJobQueueOutput",
+                )
+            })?,
             _request_id: self._request_id,
-        }
+        })
     }
 }
```

### `src/operation/create_scheduling_policy/_create_scheduling_policy_output.rs`

```diff
--- reference/src/operation/create_scheduling_policy/_create_scheduling_policy_output.rs
+++ generated/src/operation/create_scheduling_policy/_create_scheduling_policy_output.rs
@@ -4,19 +4,21 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct CreateSchedulingPolicyOutput {
     /// <p>The name of the scheduling policy.</p>
-    pub name: ::std::option::Option<::std::string::String>,
+    pub name: ::std::string::String,
     /// <p>The Amazon Resource Name (ARN) of the scheduling policy. The format is <code>aws:<i>Partition</i>:batch:<i>Region</i>:<i>Account</i>:scheduling-policy/<i>Name</i> </code>. For example, <code>aws:aws:batch:us-west-2:123456789012:scheduling-policy/MySchedulingPolicy</code>.</p>
-    pub arn: ::std::option::Option<::std::string::String>,
+    pub arn: ::std::string::String,
     _request_id: Option<String>,
 }
 impl CreateSchedulingPolicyOutput {
     /// <p>The name of the scheduling policy.</p>
-    pub fn name(&self) -> ::std::option::Option<&str> {
-        self.name.as_deref()
+    pub fn name(&self) -> &str {
+        use std::ops::Deref;
+        self.name.deref()
     }
     /// <p>The Amazon Resource Name (ARN) of the scheduling policy. The format is <code>aws:<i>Partition</i>:batch:<i>Region</i>:<i>Account</i>:scheduling-policy/<i>Name</i> </code>. For example, <code>aws:aws:batch:us-west-2:123456789012:scheduling-policy/MySchedulingPolicy</code>.</p>
-    pub fn arn(&self) -> ::std::option::Option<&str> {
-        self.arn.as_deref()
+    pub fn arn(&self) -> &str {
+        use std::ops::Deref;
+        self.arn.deref()
     }
 }
 impl ::aws_types::request_id::RequestId for CreateSchedulingPolicyOutput {
@@ -80,11 +82,29 @@
         self
     }
     /// Consumes the builder and constructs a [`CreateSchedulingPolicyOutput`](crate::operation::create_scheduling_policy::CreateSchedulingPolicyOutput).
-    pub fn build(self) -> super::super::super::operation::create_scheduling_policy::CreateSchedulingPolicyOutput {
-        super::super::super::operation::create_scheduling_policy::CreateSchedulingPolicyOutput {
-            name: self.name,
-            arn: self.arn,
+    /// This method will fail if any of the following fields are not set:
+    /// - [`name`](crate::operation::create_scheduling_policy::builders::CreateSchedulingPolicyOutputBuilder::name)
+    /// - [`arn`](crate::operation::create_scheduling_policy::builders::CreateSchedulingPolicyOutputBuilder::arn)
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<
+        super::super::super::operation::create_scheduling_policy::CreateSchedulingPolicyOutput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
+        ::std::result::Result::Ok(super::super::super::operation::create_scheduling_policy::CreateSchedulingPolicyOutput {
+            name: self.name.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "name",
+                    "name was not specified but it is required when building CreateSchedulingPolicyOutput",
+                )
+            })?,
+            arn: self.arn.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "arn",
+                    "arn was not specified but it is required when building CreateSchedulingPolicyOutput",
+                )
+            })?,
             _request_id: self._request_id,
-        }
+        })
     }
 }
```

### `src/operation/create_service_environment/_create_service_environment_output.rs`

```diff
--- reference/src/operation/create_service_environment/_create_service_environment_output.rs
+++ generated/src/operation/create_service_environment/_create_service_environment_output.rs
@@ -4,19 +4,21 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct CreateServiceEnvironmentOutput {
     /// <p>The name of the service environment.</p>
-    pub service_environment_name: ::std::option::Option<::std::string::String>,
+    pub service_environment_name: ::std::string::String,
     /// <p>The Amazon Resource Name (ARN) of the service environment.</p>
-    pub service_environment_arn: ::std::option::Option<::std::string::String>,
+    pub service_environment_arn: ::std::string::String,
     _request_id: Option<String>,
 }
 impl CreateServiceEnvironmentOutput {
     /// <p>The name of the service environment.</p>
-    pub fn service_environment_name(&self) -> ::std::option::Option<&str> {
-        self.service_environment_name.as_deref()
+    pub fn service_environment_name(&self) -> &str {
+        use std::ops::Deref;
+        self.service_environment_name.deref()
     }
     /// <p>The Amazon Resource Name (ARN) of the service environment.</p>
-    pub fn service_environment_arn(&self) -> ::std::option::Option<&str> {
-        self.service_environment_arn.as_deref()
+    pub fn service_environment_arn(&self) -> &str {
+        use std::ops::Deref;
+        self.service_environment_arn.deref()
     }
 }
 impl ::aws_types::request_id::RequestId for CreateServiceEnvironmentOutput {
@@ -80,11 +82,29 @@
         self
     }
     /// Consumes the builder and constructs a [`CreateServiceEnvironmentOutput`](crate::operation::create_service_environment::CreateServiceEnvironmentOutput).
-    pub fn build(self) -> super::super::super::operation::create_service_environment::CreateServiceEnvironmentOutput {
-        super::super::super::operation::create_service_environment::CreateServiceEnvironmentOutput {
-            service_environment_name: self.service_environment_name,
-            service_environment_arn: self.service_environment_arn,
+    /// This method will fail if any of the following fields are not set:
+    /// - [`service_environment_name`](crate::operation::create_service_environment::builders::CreateServiceEnvironmentOutputBuilder::service_environment_name)
+    /// - [`service_environment_arn`](crate::operation::create_service_environment::builders::CreateServiceEnvironmentOutputBuilder::service_environment_arn)
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<
+        super::super::super::operation::create_service_environment::CreateServiceEnvironmentOutput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
+        ::std::result::Result::Ok(super::super::super::operation::create_service_environment::CreateServiceEnvironmentOutput {
+            service_environment_name: self.service_environment_name.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "service_environment_name",
+                    "service_environment_name was not specified but it is required when building CreateServiceEnvironmentOutput",
+                )
+            })?,
+            service_environment_arn: self.service_environment_arn.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "service_environment_arn",
+                    "service_environment_arn was not specified but it is required when building CreateServiceEnvironmentOutput",
+                )
+            })?,
             _request_id: self._request_id,
-        }
+        })
     }
 }
```

### `src/operation/describe_consumable_resource/_describe_consumable_resource_output.rs`

```diff
--- reference/src/operation/describe_consumable_resource/_describe_consumable_resource_output.rs
+++ generated/src/operation/describe_consumable_resource/_describe_consumable_resource_output.rs
@@ -4,9 +4,9 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct DescribeConsumableResourceOutput {
     /// <p>The name of the consumable resource.</p>
-    pub consumable_resource_name: ::std::option::Option<::std::string::String>,
+    pub consumable_resource_name: ::std::string::String,
     /// <p>The Amazon Resource Name (ARN) of the consumable resource.</p>
-    pub consumable_resource_arn: ::std::option::Option<::std::string::String>,
+    pub consumable_resource_arn: ::std::string::String,
     /// <p>The total amount of the consumable resource that is available.</p>
     pub total_quantity: ::std::option::Option<i64>,
     /// <p>The amount of the consumable resource that is currently in use.</p>
@@ -29,12 +29,14 @@
 }
 impl DescribeConsumableResourceOutput {
     /// <p>The name of the consumable resource.</p>
-    pub fn consumable_resource_name(&self) -> ::std::option::Option<&str> {
-        self.consumable_resource_name.as_deref()
+    pub fn consumable_resource_name(&self) -> &str {
+        use std::ops::Deref;
+        self.consumable_resource_name.deref()
     }
     /// <p>The Amazon Resource Name (ARN) of the consumable resource.</p>
-    pub fn consumable_resource_arn(&self) -> ::std::option::Option<&str> {
-        self.consumable_resource_arn.as_deref()
+    pub fn consumable_resource_arn(&self) -> &str {
+        use std::ops::Deref;
+        self.consumable_resource_arn.deref()
     }
     /// <p>The total amount of the consumable resource that is available.</p>
     pub fn total_quantity(&self) -> ::std::option::Option<i64> {
@@ -242,10 +244,28 @@
         self
     }
     /// Consumes the builder and constructs a [`DescribeConsumableResourceOutput`](crate::operation::describe_consumable_resource::DescribeConsumableResourceOutput).
-    pub fn build(self) -> super::super::super::operation::describe_consumable_resource::DescribeConsumableResourceOutput {
-        super::super::super::operation::describe_consumable_resource::DescribeConsumableResourceOutput {
-            consumable_resource_name: self.consumable_resource_name,
-            consumable_resource_arn: self.consumable_resource_arn,
+    /// This method will fail if any of the following fields are not set:
+    /// - [`consumable_resource_name`](crate::operation::describe_consumable_resource::builders::DescribeConsumableResourceOutputBuilder::consumable_resource_name)
+    /// - [`consumable_resource_arn`](crate::operation::describe_consumable_resource::builders::DescribeConsumableResourceOutputBuilder::consumable_resource_arn)
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<
+        super::super::super::operation::describe_consumable_resource::DescribeConsumableResourceOutput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
+        ::std::result::Result::Ok(super::super::super::operation::describe_consumable_resource::DescribeConsumableResourceOutput {
+            consumable_resource_name: self.consumable_resource_name.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "consumable_resource_name",
+                    "consumable_resource_name was not specified but it is required when building DescribeConsumableResourceOutput",
+                )
+            })?,
+            consumable_resource_arn: self.consumable_resource_arn.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "consumable_resource_arn",
+                    "consumable_resource_arn was not specified but it is required when building DescribeConsumableResourceOutput",
+                )
+            })?,
             total_quantity: self.total_quantity,
             in_use_quantity: self.in_use_quantity,
             available_quantity: self.available_quantity,
@@ -253,6 +273,6 @@
             created_at: self.created_at,
             tags: self.tags,
             _request_id: self._request_id,
-        }
+        })
     }
 }
```

### `src/operation/describe_service_job/_describe_service_job_output.rs`

```diff
--- reference/src/operation/describe_service_job/_describe_service_job_output.rs
+++ generated/src/operation/describe_service_job/_describe_service_job_output.rs
@@ -14,11 +14,11 @@
     /// <p>The Amazon Resource Name (ARN) of the service job.</p>
     pub job_arn: ::std::option::Option<::std::string::String>,
     /// <p>The job ID for the service job.</p>
-    pub job_id: ::std::option::Option<::std::string::String>,
+    pub job_id: ::std::string::String,
     /// <p>The name of the service job.</p>
-    pub job_name: ::std::option::Option<::std::string::String>,
+    pub job_name: ::std::string::String,
     /// <p>The ARN of the job queue that the service job is associated with.</p>
-    pub job_queue: ::std::option::Option<::std::string::String>,
+    pub job_queue: ::std::string::String,
     /// <p>The latest attempt associated with the service job.</p>
     pub latest_attempt: ::std::option::Option<super::super::super::types::LatestServiceJobAttempt>,
     /// <p>The retry strategy to use for failed service jobs that are submitted with this service job.</p>
@@ -30,7 +30,7 @@
     /// <p>The request, in JSON, for the service that the <code>SubmitServiceJob</code> operation is queueing.</p>
     pub service_request_payload: ::std::option::Option<::std::string::String>,
     /// <p>The type of service job. For SageMaker Training jobs, this value is <code>SAGEMAKER_TRAINING</code>.</p>
-    pub service_job_type: ::std::option::Option<super::super::super::types::ServiceJobType>,
+    pub service_job_type: super::super::super::types::ServiceJobType,
     /// <p>The share identifier for the service job. This is used for fair-share scheduling.</p>
     pub share_identifier: ::std::option::Option<::std::string::String>,
     /// <p>The name of the quota share that the service job is associated with.</p>
@@ -40,9 +40,9 @@
     /// <p>Summarizes the preemptions of the service job. This field appears on a service job when it has been preempted.</p>
     pub preemption_summary: ::std::option::Option<super::super::super::types::ServiceJobPreemptionSummary>,
     /// <p>The Unix timestamp (in milliseconds) for when the service job was started.</p>
-    pub started_at: ::std::option::Option<i64>,
+    pub started_at: i64,
     /// <p>The current status of the service job.</p>
-    pub status: ::std::option::Option<super::super::super::types::ServiceJobStatus>,
+    pub status: super::super::super::types::ServiceJobStatus,
     /// <p>A short, human-readable string to provide more details for the current status of the service job.</p>
     pub status_reason: ::std::option::Option<::std::string::String>,
     /// <p>The Unix timestamp (in milliseconds) for when the service job stopped running.</p>
@@ -79,16 +79,19 @@
         self.job_arn.as_deref()
     }
     /// <p>The job ID for the service job.</p>
-    pub fn job_id(&self) -> ::std::option::Option<&str> {
-        self.job_id.as_deref()
+    pub fn job_id(&self) -> &str {
+        use std::ops::Deref;
+        self.job_id.deref()
     }
     /// <p>The name of the service job.</p>
-    pub fn job_name(&self) -> ::std::option::Option<&str> {
-        self.job_name.as_deref()
+    pub fn job_name(&self) -> &str {
+        use std::ops::Deref;
+        self.job_name.deref()
     }
     /// <p>The ARN of the job queue that the service job is associated with.</p>
-    pub fn job_queue(&self) -> ::std::option::Option<&str> {
-        self.job_queue.as_deref()
+    pub fn job_queue(&self) -> &str {
+        use std::ops::Deref;
+        self.job_queue.deref()
     }
     /// <p>The latest attempt associated with the service job.</p>
     pub fn latest_attempt(&self) -> ::std::option::Option<&super::super::super::types::LatestServiceJobAttempt> {
@@ -111,8 +114,8 @@
         self.service_request_payload.as_deref()
     }
     /// <p>The type of service job. For SageMaker Training jobs, this value is <code>SAGEMAKER_TRAINING</code>.</p>
-    pub fn service_job_type(&self) -> ::std::option::Option<&super::super::super::types::ServiceJobType> {
-        self.service_job_type.as_ref()
+    pub fn service_job_type(&self) -> &super::super::super::types::ServiceJobType {
+        &self.service_job_type
     }
     /// <p>The share identifier for the service job. This is used for fair-share scheduling.</p>
     pub fn share_identifier(&self) -> ::std::option::Option<&str> {
@@ -131,12 +134,12 @@
         self.preemption_summary.as_ref()
     }
     /// <p>The Unix timestamp (in milliseconds) for when the service job was started.</p>
-    pub fn started_at(&self) -> ::std::option::Option<i64> {
+    pub fn started_at(&self) -> i64 {
         self.started_at
     }
     /// <p>The current status of the service job.</p>
-    pub fn status(&self) -> ::std::option::Option<&super::super::super::types::ServiceJobStatus> {
-        self.status.as_ref()
+    pub fn status(&self) -> &super::super::super::types::ServiceJobStatus {
+        &self.status
     }
     /// <p>A short, human-readable string to provide more details for the current status of the service job.</p>
     pub fn status_reason(&self) -> ::std::option::Option<&str> {
@@ -568,33 +571,73 @@
         self
     }
     /// Consumes the builder and constructs a [`DescribeServiceJobOutput`](crate::operation::describe_service_job::DescribeServiceJobOutput).
-    pub fn build(self) -> super::super::super::operation::describe_service_job::DescribeServiceJobOutput {
-        super::super::super::operation::describe_service_job::DescribeServiceJobOutput {
+    /// This method will fail if any of the following fields are not set:
+    /// - [`job_id`](crate::operation::describe_service_job::builders::DescribeServiceJobOutputBuilder::job_id)
+    /// - [`job_name`](crate::operation::describe_service_job::builders::DescribeServiceJobOutputBuilder::job_name)
+    /// - [`job_queue`](crate::operation::describe_service_job::builders::DescribeServiceJobOutputBuilder::job_queue)
+    /// - [`service_job_type`](crate::operation::describe_service_job::builders::DescribeServiceJobOutputBuilder::service_job_type)
+    /// - [`started_at`](crate::operation::describe_service_job::builders::DescribeServiceJobOutputBuilder::started_at)
+    /// - [`status`](crate::operation::describe_service_job::builders::DescribeServiceJobOutputBuilder::status)
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::describe_service_job::DescribeServiceJobOutput, ::aws_smithy_types::error::operation::BuildError>
+    {
+        ::std::result::Result::Ok(super::super::super::operation::describe_service_job::DescribeServiceJobOutput {
             attempts: self.attempts,
             capacity_usage: self.capacity_usage,
             created_at: self.created_at,
             is_terminated: self.is_terminated,
             job_arn: self.job_arn,
-            job_id: self.job_id,
-            job_name: self.job_name,
-            job_queue: self.job_queue,
+            job_id: self.job_id.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "job_id",
+                    "job_id was not specified but it is required when building DescribeServiceJobOutput",
+                )
+            })?,
+            job_name: self.job_name.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "job_name",
+                    "job_name was not specified but it is required when building DescribeServiceJobOutput",
+                )
+            })?,
+            job_queue: self.job_queue.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "job_queue",
+                    "job_queue was not specified but it is required when building DescribeServiceJobOutput",
+                )
+            })?,
             latest_attempt: self.latest_attempt,
             retry_strategy: self.retry_strategy,
             scheduled_at: self.scheduled_at,
             scheduling_priority: self.scheduling_priority,
             service_request_payload: self.service_request_payload,
-            service_job_type: self.service_job_type,
+            service_job_type: self.service_job_type.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "service_job_type",
+                    "service_job_type was not specified but it is required when building DescribeServiceJobOutput",
+                )
+            })?,
             share_identifier: self.share_identifier,
             quota_share_name: self.quota_share_name,
             preemption_configuration: self.preemption_configuration,
             preemption_summary: self.preemption_summary,
-            started_at: self.started_at,
-            status: self.status,
+            started_at: self.started_at.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "started_at",
+                    "started_at was not specified but it is required when building DescribeServiceJobOutput",
+                )
+            })?,
+            status: self.status.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "status",
+                    "status was not specified but it is required when building DescribeServiceJobOutput",
+                )
+            })?,
             status_reason: self.status_reason,
             stopped_at: self.stopped_at,
             tags: self.tags,
             timeout_config: self.timeout_config,
             _request_id: self._request_id,
-        }
+        })
     }
 }
```

### `src/operation/list_consumable_resources/_list_consumable_resources_output.rs`

```diff
--- reference/src/operation/list_consumable_resources/_list_consumable_resources_output.rs
+++ generated/src/operation/list_consumable_resources/_list_consumable_resources_output.rs
@@ -4,7 +4,7 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct ListConsumableResourcesOutput {
     /// <p>A list of consumable resources that match the request.</p>
-    pub consumable_resources: ::std::option::Option<::std::vec::Vec<super::super::super::types::ConsumableResourceSummary>>,
+    pub consumable_resources: ::std::vec::Vec<super::super::super::types::ConsumableResourceSummary>,
     /// <p>The <code>nextToken</code> value to include in a future <code>ListConsumableResources</code> request. When the results of a <code>ListConsumableResources</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
     pub next_token: ::std::option::Option<::std::string::String>,
     _request_id: Option<String>,
@@ -11,10 +11,9 @@
 }
 impl ListConsumableResourcesOutput {
     /// <p>A list of consumable resources that match the request.</p>
-    ///
-    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.consumable_resources.is_none()`.
     pub fn consumable_resources(&self) -> &[super::super::super::types::ConsumableResourceSummary] {
-        self.consumable_resources.as_deref().unwrap_or_default()
+        use std::ops::Deref;
+        self.consumable_resources.deref()
     }
     /// <p>The <code>nextToken</code> value to include in a future <code>ListConsumableResources</code> request. When the results of a <code>ListConsumableResources</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
     pub fn next_token(&self) -> ::std::option::Option<&str> {
@@ -86,11 +85,23 @@
         self
     }
     /// Consumes the builder and constructs a [`ListConsumableResourcesOutput`](crate::operation::list_consumable_resources::ListConsumableResourcesOutput).
-    pub fn build(self) -> super::super::super::operation::list_consumable_resources::ListConsumableResourcesOutput {
-        super::super::super::operation::list_consumable_resources::ListConsumableResourcesOutput {
-            consumable_resources: self.consumable_resources,
+    /// This method will fail if any of the following fields are not set:
+    /// - [`consumable_resources`](crate::operation::list_consumable_resources::builders::ListConsumableResourcesOutputBuilder::consumable_resources)
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<
+        super::super::super::operation::list_consumable_resources::ListConsumableResourcesOutput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
+        ::std::result::Result::Ok(super::super::super::operation::list_consumable_resources::ListConsumableResourcesOutput {
+            consumable_resources: self.consumable_resources.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "consumable_resources",
+                    "consumable_resources was not specified but it is required when building ListConsumableResourcesOutput",
+                )
+            })?,
             next_token: self.next_token,
             _request_id: self._request_id,
-        }
+        })
     }
 }
```

### `src/operation/list_jobs/_list_jobs_output.rs`

```diff
--- reference/src/operation/list_jobs/_list_jobs_output.rs
+++ generated/src/operation/list_jobs/_list_jobs_output.rs
@@ -4,7 +4,7 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct ListJobsOutput {
     /// <p>A list of job summaries that match the request.</p>
-    pub job_summary_list: ::std::option::Option<::std::vec::Vec<super::super::super::types::JobSummary>>,
+    pub job_summary_list: ::std::vec::Vec<super::super::super::types::JobSummary>,
     /// <p>The <code>nextToken</code> value to include in a future <code>ListJobs</code> request. When the results of a <code>ListJobs</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
     pub next_token: ::std::option::Option<::std::string::String>,
     _request_id: Option<String>,
@@ -11,10 +11,9 @@
 }
 impl ListJobsOutput {
     /// <p>A list of job summaries that match the request.</p>
-    ///
-    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.job_summary_list.is_none()`.
     pub fn job_summary_list(&self) -> &[super::super::super::types::JobSummary] {
-        self.job_summary_list.as_deref().unwrap_or_default()
+        use std::ops::Deref;
+        self.job_summary_list.deref()
     }
     /// <p>The <code>nextToken</code> value to include in a future <code>ListJobs</code> request. When the results of a <code>ListJobs</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
     pub fn next_token(&self) -> ::std::option::Option<&str> {
@@ -86,11 +85,18 @@
         self
     }
     /// Consumes the builder and constructs a [`ListJobsOutput`](crate::operation::list_jobs::ListJobsOutput).
-    pub fn build(self) -> super::super::super::operation::list_jobs::ListJobsOutput {
-        super::super::super::operation::list_jobs::ListJobsOutput {
-            job_summary_list: self.job_summary_list,
+    /// This method will fail if any of the following fields are not set:
+    /// - [`job_summary_list`](crate::operation::list_jobs::builders::ListJobsOutputBuilder::job_summary_list)
+    pub fn build(self) -> ::std::result::Result<super::super::super::operation::list_jobs::ListJobsOutput, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::super::operation::list_jobs::ListJobsOutput {
+            job_summary_list: self.job_summary_list.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "job_summary_list",
+                    "job_summary_list was not specified but it is required when building ListJobsOutput",
+                )
+            })?,
             next_token: self.next_token,
             _request_id: self._request_id,
-        }
+        })
     }
 }
```

### `src/operation/list_jobs_by_consumable_resource/_list_jobs_by_consumable_resource_output.rs`

```diff
--- reference/src/operation/list_jobs_by_consumable_resource/_list_jobs_by_consumable_resource_output.rs
+++ generated/src/operation/list_jobs_by_consumable_resource/_list_jobs_by_consumable_resource_output.rs
@@ -4,7 +4,7 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct ListJobsByConsumableResourceOutput {
     /// <p>The list of jobs that require the specified consumable resources.</p>
-    pub jobs: ::std::option::Option<::std::vec::Vec<super::super::super::types::ListJobsByConsumableResourceSummary>>,
+    pub jobs: ::std::vec::Vec<super::super::super::types::ListJobsByConsumableResourceSummary>,
     /// <p>The <code>nextToken</code> value to include in a future <code>ListJobsByConsumableResource</code> request. When the results of a <code>ListJobsByConsumableResource</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
     pub next_token: ::std::option::Option<::std::string::String>,
     _request_id: Option<String>,
@@ -11,10 +11,9 @@
 }
 impl ListJobsByConsumableResourceOutput {
     /// <p>The list of jobs that require the specified consumable resources.</p>
-    ///
-    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.jobs.is_none()`.
     pub fn jobs(&self) -> &[super::super::super::types::ListJobsByConsumableResourceSummary] {
-        self.jobs.as_deref().unwrap_or_default()
+        use std::ops::Deref;
+        self.jobs.deref()
     }
     /// <p>The <code>nextToken</code> value to include in a future <code>ListJobsByConsumableResource</code> request. When the results of a <code>ListJobsByConsumableResource</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
     pub fn next_token(&self) -> ::std::option::Option<&str> {
@@ -86,11 +85,23 @@
         self
     }
     /// Consumes the builder and constructs a [`ListJobsByConsumableResourceOutput`](crate::operation::list_jobs_by_consumable_resource::ListJobsByConsumableResourceOutput).
-    pub fn build(self) -> super::super::super::operation::list_jobs_by_consumable_resource::ListJobsByConsumableResourceOutput {
-        super::super::super::operation::list_jobs_by_consumable_resource::ListJobsByConsumableResourceOutput {
-            jobs: self.jobs,
+    /// This method will fail if any of the following fields are not set:
+    /// - [`jobs`](crate::operation::list_jobs_by_consumable_resource::builders::ListJobsByConsumableResourceOutputBuilder::jobs)
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<
+        super::super::super::operation::list_jobs_by_consumable_resource::ListJobsByConsumableResourceOutput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
+        ::std::result::Result::Ok(super::super::super::operation::list_jobs_by_consumable_resource::ListJobsByConsumableResourceOutput {
+            jobs: self.jobs.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "jobs",
+                    "jobs was not specified but it is required when building ListJobsByConsumableResourceOutput",
+                )
+            })?,
             next_token: self.next_token,
             _request_id: self._request_id,
-        }
+        })
     }
 }
```

### `src/operation/list_service_jobs/_list_service_jobs_output.rs`

```diff
--- reference/src/operation/list_service_jobs/_list_service_jobs_output.rs
+++ generated/src/operation/list_service_jobs/_list_service_jobs_output.rs
@@ -4,7 +4,7 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct ListServiceJobsOutput {
     /// <p>A list of service job summaries.</p>
-    pub job_summary_list: ::std::option::Option<::std::vec::Vec<super::super::super::types::ServiceJobSummary>>,
+    pub job_summary_list: ::std::vec::Vec<super::super::super::types::ServiceJobSummary>,
     /// <p>The <code>nextToken</code> value to include in a future <code>ListServiceJobs</code> request. When the results of a <code>ListServiceJobs</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
     pub next_token: ::std::option::Option<::std::string::String>,
     _request_id: Option<String>,
@@ -11,10 +11,9 @@
 }
 impl ListServiceJobsOutput {
     /// <p>A list of service job summaries.</p>
-    ///
-    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.job_summary_list.is_none()`.
     pub fn job_summary_list(&self) -> &[super::super::super::types::ServiceJobSummary] {
-        self.job_summary_list.as_deref().unwrap_or_default()
+        use std::ops::Deref;
+        self.job_summary_list.deref()
     }
     /// <p>The <code>nextToken</code> value to include in a future <code>ListServiceJobs</code> request. When the results of a <code>ListServiceJobs</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
     pub fn next_token(&self) -> ::std::option::Option<&str> {
@@ -86,11 +85,20 @@
         self
     }
     /// Consumes the builder and constructs a [`ListServiceJobsOutput`](crate::operation::list_service_jobs::ListServiceJobsOutput).
-    pub fn build(self) -> super::super::super::operation::list_service_jobs::ListServiceJobsOutput {
-        super::super::super::operation::list_service_jobs::ListServiceJobsOutput {
-            job_summary_list: self.job_summary_list,
+    /// This method will fail if any of the following fields are not set:
+    /// - [`job_summary_list`](crate::operation::list_service_jobs::builders::ListServiceJobsOutputBuilder::job_summary_list)
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::list_service_jobs::ListServiceJobsOutput, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::super::operation::list_service_jobs::ListServiceJobsOutput {
+            job_summary_list: self.job_summary_list.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "job_summary_list",
+                    "job_summary_list was not specified but it is required when building ListServiceJobsOutput",
+                )
+            })?,
             next_token: self.next_token,
             _request_id: self._request_id,
-        }
+        })
     }
 }
```

### `src/operation/list_tags_for_resource.rs`

```diff
--- reference/src/operation/list_tags_for_resource.rs
+++ generated/src/operation/list_tags_for_resource.rs
@@ -261,10 +261,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_list_tags_for_resource::ser_list_tags_for_resource_input(
+            &input,
+        )?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/register_job_definition/_register_job_definition_output.rs`

```diff
--- reference/src/operation/register_job_definition/_register_job_definition_output.rs
+++ generated/src/operation/register_job_definition/_register_job_definition_output.rs
@@ -4,24 +4,26 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct RegisterJobDefinitionOutput {
     /// <p>The name of the job definition.</p>
-    pub job_definition_name: ::std::option::Option<::std::string::String>,
+    pub job_definition_name: ::std::string::String,
     /// <p>The Amazon Resource Name (ARN) of the job definition.</p>
-    pub job_definition_arn: ::std::option::Option<::std::string::String>,
+    pub job_definition_arn: ::std::string::String,
     /// <p>The revision of the job definition.</p>
-    pub revision: ::std::option::Option<i32>,
+    pub revision: i32,
     _request_id: Option<String>,
 }
 impl RegisterJobDefinitionOutput {
     /// <p>The name of the job definition.</p>
-    pub fn job_definition_name(&self) -> ::std::option::Option<&str> {
-        self.job_definition_name.as_deref()
+    pub fn job_definition_name(&self) -> &str {
+        use std::ops::Deref;
+        self.job_definition_name.deref()
     }
     /// <p>The Amazon Resource Name (ARN) of the job definition.</p>
-    pub fn job_definition_arn(&self) -> ::std::option::Option<&str> {
-        self.job_definition_arn.as_deref()
+    pub fn job_definition_arn(&self) -> &str {
+        use std::ops::Deref;
+        self.job_definition_arn.deref()
     }
     /// <p>The revision of the job definition.</p>
-    pub fn revision(&self) -> ::std::option::Option<i32> {
+    pub fn revision(&self) -> i32 {
         self.revision
     }
 }
@@ -102,12 +104,34 @@
         self
     }
     /// Consumes the builder and constructs a [`RegisterJobDefinitionOutput`](crate::operation::register_job_definition::RegisterJobDefinitionOutput).
-    pub fn build(self) -> super::super::super::operation::register_job_definition::RegisterJobDefinitionOutput {
-        super::super::super::operation::register_job_definition::RegisterJobDefinitionOutput {
-            job_definition_name: self.job_definition_name,
-            job_definition_arn: self.job_definition_arn,
-            revision: self.revision,
+    /// This method will fail if any of the following fields are not set:
+    /// - [`job_definition_name`](crate::operation::register_job_definition::builders::RegisterJobDefinitionOutputBuilder::job_definition_name)
+    /// - [`job_definition_arn`](crate::operation::register_job_definition::builders::RegisterJobDefinitionOutputBuilder::job_definition_arn)
+    /// - [`revision`](crate::operation::register_job_definition::builders::RegisterJobDefinitionOutputBuilder::revision)
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::register_job_definition::RegisterJobDefinitionOutput, ::aws_smithy_types::error::operation::BuildError>
+    {
+        ::std::result::Result::Ok(super::super::super::operation::register_job_definition::RegisterJobDefinitionOutput {
+            job_definition_name: self.job_definition_name.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "job_definition_name",
+                    "job_definition_name was not specified but it is required when building RegisterJobDefinitionOutput",
+                )
+            })?,
+            job_definition_arn: self.job_definition_arn.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "job_definition_arn",
+                    "job_definition_arn was not specified but it is required when building RegisterJobDefinitionOutput",
+                )
+            })?,
+            revision: self.revision.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "revision",
+                    "revision was not specified but it is required when building RegisterJobDefinitionOutput",
+                )
+            })?,
             _request_id: self._request_id,
-        }
+        })
     }
 }
```

### `src/operation/submit_job/_submit_job_output.rs`

```diff
--- reference/src/operation/submit_job/_submit_job_output.rs
+++ generated/src/operation/submit_job/_submit_job_output.rs
@@ -6,9 +6,9 @@
     /// <p>The Amazon Resource Name (ARN) for the job.</p>
     pub job_arn: ::std::option::Option<::std::string::String>,
     /// <p>The name of the job.</p>
-    pub job_name: ::std::option::Option<::std::string::String>,
+    pub job_name: ::std::string::String,
     /// <p>The unique identifier for the job.</p>
-    pub job_id: ::std::option::Option<::std::string::String>,
+    pub job_id: ::std::string::String,
     _request_id: Option<String>,
 }
 impl SubmitJobOutput {
@@ -17,12 +17,14 @@
         self.job_arn.as_deref()
     }
     /// <p>The name of the job.</p>
-    pub fn job_name(&self) -> ::std::option::Option<&str> {
-        self.job_name.as_deref()
+    pub fn job_name(&self) -> &str {
+        use std::ops::Deref;
+        self.job_name.deref()
     }
     /// <p>The unique identifier for the job.</p>
-    pub fn job_id(&self) -> ::std::option::Option<&str> {
-        self.job_id.as_deref()
+    pub fn job_id(&self) -> &str {
+        use std::ops::Deref;
+        self.job_id.deref()
     }
 }
 impl ::aws_types::request_id::RequestId for SubmitJobOutput {
@@ -101,12 +103,25 @@
         self
     }
     /// Consumes the builder and constructs a [`SubmitJobOutput`](crate::operation::submit_job::SubmitJobOutput).
-    pub fn build(self) -> super::super::super::operation::submit_job::SubmitJobOutput {
-        super::super::super::operation::submit_job::SubmitJobOutput {
+    /// This method will fail if any of the following fields are not set:
+    /// - [`job_name`](crate::operation::submit_job::builders::SubmitJobOutputBuilder::job_name)
+    /// - [`job_id`](crate::operation::submit_job::builders::SubmitJobOutputBuilder::job_id)
+    pub fn build(self) -> ::std::result::Result<super::super::super::operation::submit_job::SubmitJobOutput, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::super::operation::submit_job::SubmitJobOutput {
             job_arn: self.job_arn,
-            job_name: self.job_name,
-            job_id: self.job_id,
+            job_name: self.job_name.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "job_name",
+                    "job_name was not specified but it is required when building SubmitJobOutput",
+                )
+            })?,
+            job_id: self.job_id.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "job_id",
+                    "job_id was not specified but it is required when building SubmitJobOutput",
+                )
+            })?,
             _request_id: self._request_id,
-        }
+        })
     }
 }
```

### `src/operation/submit_service_job/_submit_service_job_output.rs`

```diff
--- reference/src/operation/submit_service_job/_submit_service_job_output.rs
+++ generated/src/operation/submit_service_job/_submit_service_job_output.rs
@@ -6,9 +6,9 @@
     /// <p>The Amazon Resource Name (ARN) for the service job.</p>
     pub job_arn: ::std::option::Option<::std::string::String>,
     /// <p>The name of the service job.</p>
-    pub job_name: ::std::option::Option<::std::string::String>,
+    pub job_name: ::std::string::String,
     /// <p>The unique identifier for the service job.</p>
-    pub job_id: ::std::option::Option<::std::string::String>,
+    pub job_id: ::std::string::String,
     _request_id: Option<String>,
 }
 impl SubmitServiceJobOutput {
@@ -17,12 +17,14 @@
         self.job_arn.as_deref()
     }
     /// <p>The name of the service job.</p>
-    pub fn job_name(&self) -> ::std::option::Option<&str> {
-        self.job_name.as_deref()
+    pub fn job_name(&self) -> &str {
+        use std::ops::Deref;
+        self.job_name.deref()
     }
     /// <p>The unique identifier for the service job.</p>
-    pub fn job_id(&self) -> ::std::option::Option<&str> {
-        self.job_id.as_deref()
+    pub fn job_id(&self) -> &str {
+        use std::ops::Deref;
+        self.job_id.deref()
     }
 }
 impl ::aws_types::request_id::RequestId for SubmitServiceJobOutput {
@@ -101,12 +103,27 @@
         self
     }
     /// Consumes the builder and constructs a [`SubmitServiceJobOutput`](crate::operation::submit_service_job::SubmitServiceJobOutput).
-    pub fn build(self) -> super::super::super::operation::submit_service_job::SubmitServiceJobOutput {
-        super::super::super::operation::submit_service_job::SubmitServiceJobOutput {
+    /// This method will fail if any of the following fields are not set:
+    /// - [`job_name`](crate::operation::submit_service_job::builders::SubmitServiceJobOutputBuilder::job_name)
+    /// - [`job_id`](crate::operation::submit_service_job::builders::SubmitServiceJobOutputBuilder::job_id)
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::super::operation::submit_service_job::SubmitServiceJobOutput, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::super::operation::submit_service_job::SubmitServiceJobOutput {
             job_arn: self.job_arn,
-            job_name: self.job_name,
-            job_id: self.job_id,
+            job_name: self.job_name.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "job_name",
+                    "job_name was not specified but it is required when building SubmitServiceJobOutput",
+                )
+            })?,
+            job_id: self.job_id.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "job_id",
+                    "job_id was not specified but it is required when building SubmitServiceJobOutput",
+                )
+            })?,
             _request_id: self._request_id,
-        }
+        })
     }
 }
```

### `src/operation/untag_resource.rs`

```diff
--- reference/src/operation/untag_resource.rs
+++ generated/src/operation/untag_resource.rs
@@ -257,9 +257,7 @@
                 let inner_2 = inner_2
                     .as_ref()
                     .ok_or_else(|| ::aws_smithy_types::error::operation::BuildError::missing_field("tag_keys", "cannot be empty or unset"))?;
-                for inner_3 in inner_2 {
-                    query.push_kv("tagKeys", &::aws_smithy_http::query::fmt_string(inner_3));
-                }
+                query.push_kv("tagKeys", ::aws_smithy_types::primitive::Encoder::from(*inner_2).encode());
                 ::std::result::Result::Ok(())
             }
             #[allow(clippy::unnecessary_wraps)]
@@ -273,10 +271,14 @@
                 ::std::result::Result::Ok(builder.method("DELETE").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_untag_resource::ser_untag_resource_input(&input)?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/update_consumable_resource/_update_consumable_resource_output.rs`

```diff
--- reference/src/operation/update_consumable_resource/_update_consumable_resource_output.rs
+++ generated/src/operation/update_consumable_resource/_update_consumable_resource_output.rs
@@ -4,9 +4,9 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct UpdateConsumableResourceOutput {
     /// <p>The name of the consumable resource to be updated.</p>
-    pub consumable_resource_name: ::std::option::Option<::std::string::String>,
+    pub consumable_resource_name: ::std::string::String,
     /// <p>The Amazon Resource Name (ARN) of the consumable resource.</p>
-    pub consumable_resource_arn: ::std::option::Option<::std::string::String>,
+    pub consumable_resource_arn: ::std::string::String,
     /// <p>The total amount of the consumable resource that is available.</p>
     pub total_quantity: ::std::option::Option<i64>,
     _request_id: Option<String>,
@@ -13,12 +13,14 @@
 }
 impl UpdateConsumableResourceOutput {
     /// <p>The name of the consumable resource to be updated.</p>
-    pub fn consumable_resource_name(&self) -> ::std::option::Option<&str> {
-        self.consumable_resource_name.as_deref()
+    pub fn consumable_resource_name(&self) -> &str {
+        use std::ops::Deref;
+        self.consumable_resource_name.deref()
     }
     /// <p>The Amazon Resource Name (ARN) of the consumable resource.</p>
-    pub fn consumable_resource_arn(&self) -> ::std::option::Option<&str> {
-        self.consumable_resource_arn.as_deref()
+    pub fn consumable_resource_arn(&self) -> &str {
+        use std::ops::Deref;
+        self.consumable_resource_arn.deref()
     }
     /// <p>The total amount of the consumable resource that is available.</p>
     pub fn total_quantity(&self) -> ::std::option::Option<i64> {
@@ -101,12 +103,30 @@
         self
     }
     /// Consumes the builder and constructs a [`UpdateConsumableResourceOutput`](crate::operation::update_consumable_resource::UpdateConsumableResourceOutput).
-    pub fn build(self) -> super::super::super::operation::update_consumable_resource::UpdateConsumableResourceOutput {
-        super::super::super::operation::update_consumable_resource::UpdateConsumableResourceOutput {
-            consumable_resource_name: self.consumable_resource_name,
-            consumable_resource_arn: self.consumable_resource_arn,
+    /// This method will fail if any of the following fields are not set:
+    /// - [`consumable_resource_name`](crate::operation::update_consumable_resource::builders::UpdateConsumableResourceOutputBuilder::consumable_resource_name)
+    /// - [`consumable_resource_arn`](crate::operation::update_consumable_resource::builders::UpdateConsumableResourceOutputBuilder::consumable_resource_arn)
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<
+        super::super::super::operation::update_consumable_resource::UpdateConsumableResourceOutput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
+        ::std::result::Result::Ok(super::super::super::operation::update_consumable_resource::UpdateConsumableResourceOutput {
+            consumable_resource_name: self.consumable_resource_name.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "consumable_resource_name",
+                    "consumable_resource_name was not specified but it is required when building UpdateConsumableResourceOutput",
+                )
+            })?,
+            consumable_resource_arn: self.consumable_resource_arn.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "consumable_resource_arn",
+                    "consumable_resource_arn was not specified but it is required when building UpdateConsumableResourceOutput",
+                )
+            })?,
             total_quantity: self.total_quantity,
             _request_id: self._request_id,
-        }
+        })
     }
 }
```

### `src/operation/update_service_environment/_update_service_environment_output.rs`

```diff
--- reference/src/operation/update_service_environment/_update_service_environment_output.rs
+++ generated/src/operation/update_service_environment/_update_service_environment_output.rs
@@ -4,19 +4,21 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct UpdateServiceEnvironmentOutput {
     /// <p>The name of the service environment that was updated.</p>
-    pub service_environment_name: ::std::option::Option<::std::string::String>,
+    pub service_environment_name: ::std::string::String,
     /// <p>The Amazon Resource Name (ARN) of the service environment that was updated.</p>
-    pub service_environment_arn: ::std::option::Option<::std::string::String>,
+    pub service_environment_arn: ::std::string::String,
     _request_id: Option<String>,
 }
 impl UpdateServiceEnvironmentOutput {
     /// <p>The name of the service environment that was updated.</p>
-    pub fn service_environment_name(&self) -> ::std::option::Option<&str> {
-        self.service_environment_name.as_deref()
+    pub fn service_environment_name(&self) -> &str {
+        use std::ops::Deref;
+        self.service_environment_name.deref()
     }
     /// <p>The Amazon Resource Name (ARN) of the service environment that was updated.</p>
-    pub fn service_environment_arn(&self) -> ::std::option::Option<&str> {
-        self.service_environment_arn.as_deref()
+    pub fn service_environment_arn(&self) -> &str {
+        use std::ops::Deref;
+        self.service_environment_arn.deref()
     }
 }
 impl ::aws_types::request_id::RequestId for UpdateServiceEnvironmentOutput {
@@ -80,11 +82,29 @@
         self
     }
     /// Consumes the builder and constructs a [`UpdateServiceEnvironmentOutput`](crate::operation::update_service_environment::UpdateServiceEnvironmentOutput).
-    pub fn build(self) -> super::super::super::operation::update_service_environment::UpdateServiceEnvironmentOutput {
-        super::super::super::operation::update_service_environment::UpdateServiceEnvironmentOutput {
-            service_environment_name: self.service_environment_name,
-            service_environment_arn: self.service_environment_arn,
+    /// This method will fail if any of the following fields are not set:
+    /// - [`service_environment_name`](crate::operation::update_service_environment::builders::UpdateServiceEnvironmentOutputBuilder::service_environment_name)
+    /// - [`service_environment_arn`](crate::operation::update_service_environment::builders::UpdateServiceEnvironmentOutputBuilder::service_environment_arn)
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<
+        super::super::super::operation::update_service_environment::UpdateServiceEnvironmentOutput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
+        ::std::result::Result::Ok(super::super::super::operation::update_service_environment::UpdateServiceEnvironmentOutput {
+            service_environment_name: self.service_environment_name.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "service_environment_name",
+                    "service_environment_name was not specified but it is required when building UpdateServiceEnvironmentOutput",
+                )
+            })?,
+            service_environment_arn: self.service_environment_arn.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "service_environment_arn",
+                    "service_environment_arn was not specified but it is required when building UpdateServiceEnvironmentOutput",
+                )
+            })?,
             _request_id: self._request_id,
-        }
+        })
     }
 }
```

### `src/protocol_serde/shape_compute_environment_detail.rs`

```diff
--- reference/src/protocol_serde/shape_compute_environment_detail.rs
+++ generated/src/protocol_serde/shape_compute_environment_detail.rs
@@ -143,7 +143,11 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::compute_environment_detail_correct_errors(builder).build()))
+            Ok(Some(
+                super::super::serde_util::compute_environment_detail_correct_errors(builder)
+                    .build()
+                    .map_err(|err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err))?,
+            ))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_compute_environment_order.rs`

```diff
--- reference/src/protocol_serde/shape_compute_environment_order.rs
+++ generated/src/protocol_serde/shape_compute_environment_order.rs
@@ -3,14 +3,14 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::ComputeEnvironmentOrder,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.order {
+    {
         object.key("order").number(
             #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_1).into()),
+            ::aws_smithy_types::Number::NegInt((input.order).into()),
         );
     }
-    if let Some(var_2) = &input.compute_environment {
-        object.key("computeEnvironment").string(var_2.as_str());
+    {
+        object.key("computeEnvironment").string(input.compute_environment.as_str());
     }
     Ok(())
 }
@@ -60,7 +60,11 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::compute_environment_order_correct_errors(builder).build()))
+            Ok(Some(
+                super::super::serde_util::compute_environment_order_correct_errors(builder)
+                    .build()
+                    .map_err(|err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err))?,
+            ))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_compute_resource.rs`

```diff
--- reference/src/protocol_serde/shape_compute_resource.rs
+++ generated/src/protocol_serde/shape_compute_resource.rs
@@ -3,127 +3,127 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::ComputeResource,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.r#type {
-        object.key("type").string(var_1.as_str());
+    {
+        object.key("type").string(input.r#type.as_str());
     }
-    if let Some(var_2) = &input.allocation_strategy {
-        object.key("allocationStrategy").string(var_2.as_str());
+    if let Some(var_1) = &input.allocation_strategy {
+        object.key("allocationStrategy").string(var_1.as_str());
     }
-    if let Some(var_3) = &input.minv_cpus {
+    if let Some(var_2) = &input.minv_cpus {
         object.key("minvCpus").number(
             #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_3).into()),
+            ::aws_smithy_types::Number::NegInt((*var_2).into()),
         );
     }
-    if let Some(var_4) = &input.maxv_cpus {
+    {
         object.key("maxvCpus").number(
             #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_4).into()),
+            ::aws_smithy_types::Number::NegInt((input.maxv_cpus).into()),
         );
     }
-    if let Some(var_5) = &input.desiredv_cpus {
+    if let Some(var_3) = &input.desiredv_cpus {
         object.key("desiredvCpus").number(
             #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_5).into()),
+            ::aws_smithy_types::Number::NegInt((*var_3).into()),
         );
     }
-    if let Some(var_6) = &input.instance_types {
-        let mut array_7 = object.key("instanceTypes").start_array();
-        for item_8 in var_6 {
+    if let Some(var_4) = &input.instance_types {
+        let mut array_5 = object.key("instanceTypes").start_array();
+        for item_6 in var_4 {
             {
-                array_7.value().string(item_8.as_str());
+                array_5.value().string(item_6.as_str());
             }
         }
-        array_7.finish();
+        array_5.finish();
     }
-    if let Some(var_9) = &input.image_id {
-        object.key("imageId").string(var_9.as_str());
+    if let Some(var_7) = &input.image_id {
+        object.key("imageId").string(var_7.as_str());
     }
-    if let Some(var_10) = &input.subnets {
-        let mut array_11 = object.key("subnets").start_array();
-        for item_12 in var_10 {
+    if let Some(var_8) = &input.subnets {
+        let mut array_9 = object.key("subnets").start_array();
+        for item_10 in var_8 {
             {
-                array_11.value().string(item_12.as_str());
+                array_9.value().string(item_10.as_str());
             }
         }
-        array_11.finish();
+        array_9.finish();
     }
-    if let Some(var_13) = &input.security_group_ids {
-        let mut array_14 = object.key("securityGroupIds").start_array();
-        for item_15 in var_13 {
+    if let Some(var_11) = &input.security_group_ids {
+        let mut array_12 = object.key("securityGroupIds").start_array();
+        for item_13 in var_11 {
             {
-                array_14.value().string(item_15.as_str());
+                array_12.value().string(item_13.as_str());
             }
         }
-        array_14.finish();
+        array_12.finish();
     }
-    if let Some(var_16) = &input.ec2_key_pair {
-        object.key("ec2KeyPair").string(var_16.as_str());
+    if let Some(var_14) = &input.ec2_key_pair {
+        object.key("ec2KeyPair").string(var_14.as_str());
     }
-    if let Some(var_17) = &input.instance_role {
-        object.key("instanceRole").string(var_17.as_str());
+    if let Some(var_15) = &input.instance_role {
+        object.key("instanceRole").string(var_15.as_str());
     }
-    if let Some(var_18) = &input.tags {
+    if let Some(var_16) = &input.tags {
         #[allow(unused_mut)]
-        let mut object_19 = object.key("tags").start_object();
-        for (key_20, value_21) in var_18 {
+        let mut object_17 = object.key("tags").start_object();
+        for (key_18, value_19) in var_16 {
             {
-                object_19.key(key_20.as_str()).string(value_21.as_str());
+                object_17.key(key_18.as_str()).string(value_19.as_str());
             }
         }
-        object_19.finish();
+        object_17.finish();
     }
-    if let Some(var_22) = &input.placement_group {
-        object.key("placementGroup").string(var_22.as_str());
+    if let Some(var_20) = &input.placement_group {
+        object.key("placementGroup").string(var_20.as_str());
     }
-    if let Some(var_23) = &input.bid_percentage {
+    if let Some(var_21) = &input.bid_percentage {
         object.key("bidPercentage").number(
             #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_23).into()),
+            ::aws_smithy_types::Number::NegInt((*var_21).into()),
         );
     }
-    if let Some(var_24) = &input.spot_iam_fleet_role {
-        object.key("spotIamFleetRole").string(var_24.as_str());
+    if let Some(var_22) = &input.spot_iam_fleet_role {
+        object.key("spotIamFleetRole").string(var_22.as_str());
     }
-    if let Some(var_25) = &input.launch_template {
+    if let Some(var_23) = &input.launch_template {
         #[allow(unused_mut)]
-        let mut object_26 = object.key("launchTemplate").start_object();
-        super::super::protocol_serde::shape_launch_template_specification::ser_launch_template_specification(&mut object_26, var_25)?;
-        object_26.finish();
+        let mut object_24 = object.key("launchTemplate").start_object();
+        super::super::protocol_serde::shape_launch_template_specification::ser_launch_template_specification(&mut object_24, var_23)?;
+        object_24.finish();
     }
-    if let Some(var_27) = &input.ec2_configuration {
-        let mut array_28 = object.key("ec2Configuration").start_array();
-        for item_29 in var_27 {
+    if let Some(var_25) = &input.ec2_configuration {
+        let mut array_26 = object.key("ec2Configuration").start_array();
+        for item_27 in var_25 {
             {
                 #[allow(unused_mut)]
-                let mut object_30 = array_28.value().start_object();
-                super::super::protocol_serde::shape_ec2_configuration::ser_ec2_configuration(&mut object_30, item_29)?;
-                object_30.finish();
+                let mut object_28 = array_26.value().start_object();
+                super::super::protocol_serde::shape_ec2_configuration::ser_ec2_configuration(&mut object_28, item_27)?;
+                object_28.finish();
             }
         }
-        array_28.finish();
+        array_26.finish();
     }
-    if let Some(var_31) = &input.scaling_policy {
+    if let Some(var_29) = &input.scaling_policy {
         #[allow(unused_mut)]
-        let mut object_32 = object.key("scalingPolicy").start_object();
-        super::super::protocol_serde::shape_compute_scaling_policy::ser_compute_scaling_policy(&mut object_32, var_31)?;
-        object_32.finish();
+        let mut object_30 = object.key("scalingPolicy").start_object();
+        super::super::protocol_serde::shape_compute_scaling_policy::ser_compute_scaling_policy(&mut object_30, var_29)?;
+        object_30.finish();
     }
-    if let Some(var_33) = &input.managed_instances_provider {
+    if let Some(var_31) = &input.managed_instances_provider {
         #[allow(unused_mut)]
-        let mut object_34 = object.key("managedInstancesProvider").start_object();
-        super::super::protocol_serde::shape_managed_instances_provider::ser_managed_instances_provider(&mut object_34, var_33)?;
-        object_34.finish();
+        let mut object_32 = object.key("managedInstancesProvider").start_object();
+        super::super::protocol_serde::shape_managed_instances_provider::ser_managed_instances_provider(&mut object_32, var_31)?;
+        object_32.finish();
     }
-    if let Some(var_35) = &input.capacity_tags {
+    if let Some(var_33) = &input.capacity_tags {
         #[allow(unused_mut)]
-        let mut object_36 = object.key("capacityTags").start_object();
-        for (key_37, value_38) in var_35 {
+        let mut object_34 = object.key("capacityTags").start_object();
+        for (key_35, value_36) in var_33 {
             {
-                object_36.key(key_37.as_str()).string(value_38.as_str());
+                object_34.key(key_35.as_str()).string(value_36.as_str());
             }
         }
-        object_36.finish();
+        object_34.finish();
     }
     Ok(())
 }
@@ -285,7 +285,9 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::compute_resource_correct_errors(builder).build()))
+            Ok(Some(super::super::serde_util::compute_resource_correct_errors(builder).build().map_err(
+                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
+            )?))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_consumable_resource_summary.rs`

```diff
--- reference/src/protocol_serde/shape_consumable_resource_summary.rs
+++ generated/src/protocol_serde/shape_consumable_resource_summary.rs
@@ -65,7 +65,11 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::consumable_resource_summary_correct_errors(builder).build()))
+            Ok(Some(
+                super::super::serde_util::consumable_resource_summary_correct_errors(builder)
+                    .build()
+                    .map_err(|err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err))?,
+            ))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_create_compute_environment.rs`

```diff
--- reference/src/protocol_serde/shape_create_compute_environment.rs
+++ generated/src/protocol_serde/shape_create_compute_environment.rs
@@ -103,15 +103,15 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "computeEnvironmentArn" => {
-                    builder = builder.set_compute_environment_arn(
+                "computeEnvironmentName" => {
+                    builder = builder.set_compute_environment_name(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "computeEnvironmentName" => {
-                    builder = builder.set_compute_environment_name(
+                "computeEnvironmentArn" => {
+                    builder = builder.set_compute_environment_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
```

### `src/protocol_serde/shape_create_compute_environment_input.rs`

```diff
--- reference/src/protocol_serde/shape_create_compute_environment_input.rs
+++ generated/src/protocol_serde/shape_create_compute_environment_input.rs
@@ -6,51 +6,51 @@
     if let Some(var_1) = &input.compute_environment_name {
         object.key("computeEnvironmentName").string(var_1.as_str());
     }
-    if let Some(var_2) = &input.compute_resources {
-        #[allow(unused_mut)]
-        let mut object_3 = object.key("computeResources").start_object();
-        super::super::protocol_serde::shape_compute_resource::ser_compute_resource(&mut object_3, var_2)?;
-        object_3.finish();
+    if let Some(var_2) = &input.r#type {
+        object.key("type").string(var_2.as_str());
     }
-    if let Some(var_4) = &input.context {
-        object.key("context").string(var_4.as_str());
+    if let Some(var_3) = &input.state {
+        object.key("state").string(var_3.as_str());
     }
-    if let Some(var_5) = &input.ecs_settings {
+    if let Some(var_4) = &input.unmanagedv_cpus {
+        object.key("unmanagedvCpus").number(
+            #[allow(clippy::useless_conversion)]
+            ::aws_smithy_types::Number::NegInt((*var_4).into()),
+        );
+    }
+    if let Some(var_5) = &input.compute_resources {
         #[allow(unused_mut)]
-        let mut object_6 = object.key("ecsSettings").start_object();
-        super::super::protocol_serde::shape_ecs_settings::ser_ecs_settings(&mut object_6, var_5)?;
+        let mut object_6 = object.key("computeResources").start_object();
+        super::super::protocol_serde::shape_compute_resource::ser_compute_resource(&mut object_6, var_5)?;
         object_6.finish();
     }
-    if let Some(var_7) = &input.eks_configuration {
-        #[allow(unused_mut)]
-        let mut object_8 = object.key("eksConfiguration").start_object();
-        super::super::protocol_serde::shape_eks_configuration::ser_eks_configuration(&mut object_8, var_7)?;
-        object_8.finish();
+    if let Some(var_7) = &input.service_role {
+        object.key("serviceRole").string(var_7.as_str());
     }
-    if let Some(var_9) = &input.service_role {
-        object.key("serviceRole").string(var_9.as_str());
-    }
-    if let Some(var_10) = &input.state {
-        object.key("state").string(var_10.as_str());
-    }
-    if let Some(var_11) = &input.tags {
+    if let Some(var_8) = &input.tags {
         #[allow(unused_mut)]
-        let mut object_12 = object.key("tags").start_object();
-        for (key_13, value_14) in var_11 {
+        let mut object_9 = object.key("tags").start_object();
+        for (key_10, value_11) in var_8 {
             {
-                object_12.key(key_13.as_str()).string(value_14.as_str());
+                object_9.key(key_10.as_str()).string(value_11.as_str());
             }
         }
-        object_12.finish();
+        object_9.finish();
     }
-    if let Some(var_15) = &input.r#type {
-        object.key("type").string(var_15.as_str());
+    if let Some(var_12) = &input.eks_configuration {
+        #[allow(unused_mut)]
+        let mut object_13 = object.key("eksConfiguration").start_object();
+        super::super::protocol_serde::shape_eks_configuration::ser_eks_configuration(&mut object_13, var_12)?;
+        object_13.finish();
+    }
+    if let Some(var_14) = &input.context {
+        object.key("context").string(var_14.as_str());
     }
-    if let Some(var_16) = &input.unmanagedv_cpus {
-        object.key("unmanagedvCpus").number(
-            #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_16).into()),
-        );
+    if let Some(var_15) = &input.ecs_settings {
+        #[allow(unused_mut)]
+        let mut object_16 = object.key("ecsSettings").start_object();
+        super::super::protocol_serde::shape_ecs_settings::ser_ecs_settings(&mut object_16, var_15)?;
+        object_16.finish();
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_create_consumable_resource.rs`

```diff
--- reference/src/protocol_serde/shape_create_consumable_resource.rs
+++ generated/src/protocol_serde/shape_create_consumable_resource.rs
@@ -73,7 +73,9 @@
         output = super::super::protocol_serde::shape_create_consumable_resource::de_create_consumable_resource(_response_body, output)
             .map_err(super::super::operation::create_consumable_resource::CreateConsumableResourceError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
-        super::super::serde_util::create_consumable_resource_output_output_correct_errors(output).build()
+        super::super::serde_util::create_consumable_resource_output_output_correct_errors(output)
+            .build()
+            .map_err(super::super::operation::create_consumable_resource::CreateConsumableResourceError::unhandled)?
     })
 }

@@ -103,15 +105,15 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "consumableResourceArn" => {
-                    builder = builder.set_consumable_resource_arn(
+                "consumableResourceName" => {
+                    builder = builder.set_consumable_resource_name(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "consumableResourceName" => {
-                    builder = builder.set_consumable_resource_name(
+                "consumableResourceArn" => {
+                    builder = builder.set_consumable_resource_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
```

### `src/protocol_serde/shape_create_consumable_resource_input.rs`

```diff
--- reference/src/protocol_serde/shape_create_consumable_resource_input.rs
+++ generated/src/protocol_serde/shape_create_consumable_resource_input.rs
@@ -6,24 +6,24 @@
     if let Some(var_1) = &input.consumable_resource_name {
         object.key("consumableResourceName").string(var_1.as_str());
     }
-    if let Some(var_2) = &input.resource_type {
-        object.key("resourceType").string(var_2.as_str());
+    if let Some(var_2) = &input.total_quantity {
+        object.key("totalQuantity").number(
+            #[allow(clippy::useless_conversion)]
+            ::aws_smithy_types::Number::NegInt((*var_2).into()),
+        );
+    }
+    if let Some(var_3) = &input.resource_type {
+        object.key("resourceType").string(var_3.as_str());
     }
-    if let Some(var_3) = &input.tags {
+    if let Some(var_4) = &input.tags {
         #[allow(unused_mut)]
-        let mut object_4 = object.key("tags").start_object();
-        for (key_5, value_6) in var_3 {
+        let mut object_5 = object.key("tags").start_object();
+        for (key_6, value_7) in var_4 {
             {
-                object_4.key(key_5.as_str()).string(value_6.as_str());
+                object_5.key(key_6.as_str()).string(value_7.as_str());
             }
         }
-        object_4.finish();
-    }
-    if let Some(var_7) = &input.total_quantity {
-        object.key("totalQuantity").number(
-            #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_7).into()),
-        );
+        object_5.finish();
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_create_job_queue.rs`

```diff
--- reference/src/protocol_serde/shape_create_job_queue.rs
+++ generated/src/protocol_serde/shape_create_job_queue.rs
@@ -63,7 +63,9 @@
         output = super::super::protocol_serde::shape_create_job_queue::de_create_job_queue(_response_body, output)
             .map_err(super::super::operation::create_job_queue::CreateJobQueueError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
-        super::super::serde_util::create_job_queue_output_output_correct_errors(output).build()
+        super::super::serde_util::create_job_queue_output_output_correct_errors(output)
+            .build()
+            .map_err(super::super::operation::create_job_queue::CreateJobQueueError::unhandled)?
     })
 }

@@ -93,15 +95,15 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "jobQueueArn" => {
-                    builder = builder.set_job_queue_arn(
+                "jobQueueName" => {
+                    builder = builder.set_job_queue_name(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "jobQueueName" => {
-                    builder = builder.set_job_queue_name(
+                "jobQueueArn" => {
+                    builder = builder.set_job_queue_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
```

### `src/protocol_serde/shape_create_job_queue_input.rs`

```diff
--- reference/src/protocol_serde/shape_create_job_queue_input.rs
+++ generated/src/protocol_serde/shape_create_job_queue_input.rs
@@ -3,69 +3,69 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::operation::create_job_queue::CreateJobQueueInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.compute_environment_order {
-        let mut array_2 = object.key("computeEnvironmentOrder").start_array();
-        for item_3 in var_1 {
+    if let Some(var_1) = &input.job_queue_name {
+        object.key("jobQueueName").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.state {
+        object.key("state").string(var_2.as_str());
+    }
+    if let Some(var_3) = &input.scheduling_policy_arn {
+        object.key("schedulingPolicyArn").string(var_3.as_str());
+    }
+    if let Some(var_4) = &input.priority {
+        object.key("priority").number(
+            #[allow(clippy::useless_conversion)]
+            ::aws_smithy_types::Number::NegInt((*var_4).into()),
+        );
+    }
+    if let Some(var_5) = &input.compute_environment_order {
+        let mut array_6 = object.key("computeEnvironmentOrder").start_array();
+        for item_7 in var_5 {
             {
                 #[allow(unused_mut)]
-                let mut object_4 = array_2.value().start_object();
-                super::super::protocol_serde::shape_compute_environment_order::ser_compute_environment_order(&mut object_4, item_3)?;
-                object_4.finish();
+                let mut object_8 = array_6.value().start_object();
+                super::super::protocol_serde::shape_compute_environment_order::ser_compute_environment_order(&mut object_8, item_7)?;
+                object_8.finish();
             }
         }
-        array_2.finish();
+        array_6.finish();
     }
-    if let Some(var_5) = &input.job_queue_name {
-        object.key("jobQueueName").string(var_5.as_str());
-    }
-    if let Some(var_6) = &input.job_queue_type {
-        object.key("jobQueueType").string(var_6.as_str());
-    }
-    if let Some(var_7) = &input.job_state_time_limit_actions {
-        let mut array_8 = object.key("jobStateTimeLimitActions").start_array();
-        for item_9 in var_7 {
+    if let Some(var_9) = &input.service_environment_order {
+        let mut array_10 = object.key("serviceEnvironmentOrder").start_array();
+        for item_11 in var_9 {
             {
                 #[allow(unused_mut)]
-                let mut object_10 = array_8.value().start_object();
-                super::super::protocol_serde::shape_job_state_time_limit_action::ser_job_state_time_limit_action(&mut object_10, item_9)?;
-                object_10.finish();
+                let mut object_12 = array_10.value().start_object();
+                super::super::protocol_serde::shape_service_environment_order::ser_service_environment_order(&mut object_12, item_11)?;
+                object_12.finish();
             }
         }
-        array_8.finish();
-    }
-    if let Some(var_11) = &input.priority {
-        object.key("priority").number(
-            #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_11).into()),
-        );
+        array_10.finish();
     }
-    if let Some(var_12) = &input.scheduling_policy_arn {
-        object.key("schedulingPolicyArn").string(var_12.as_str());
+    if let Some(var_13) = &input.job_queue_type {
+        object.key("jobQueueType").string(var_13.as_str());
     }
-    if let Some(var_13) = &input.service_environment_order {
-        let mut array_14 = object.key("serviceEnvironmentOrder").start_array();
-        for item_15 in var_13 {
+    if let Some(var_14) = &input.tags {
+        #[allow(unused_mut)]
+        let mut object_15 = object.key("tags").start_object();
+        for (key_16, value_17) in var_14 {
             {
-                #[allow(unused_mut)]
-                let mut object_16 = array_14.value().start_object();
-                super::super::protocol_serde::shape_service_environment_order::ser_service_environment_order(&mut object_16, item_15)?;
-                object_16.finish();
+                object_15.key(key_16.as_str()).string(value_17.as_str());
             }
         }
-        array_14.finish();
-    }
-    if let Some(var_17) = &input.state {
-        object.key("state").string(var_17.as_str());
+        object_15.finish();
     }
-    if let Some(var_18) = &input.tags {
-        #[allow(unused_mut)]
-        let mut object_19 = object.key("tags").start_object();
-        for (key_20, value_21) in var_18 {
+    if let Some(var_18) = &input.job_state_time_limit_actions {
+        let mut array_19 = object.key("jobStateTimeLimitActions").start_array();
+        for item_20 in var_18 {
             {
-                object_19.key(key_20.as_str()).string(value_21.as_str());
+                #[allow(unused_mut)]
+                let mut object_21 = array_19.value().start_object();
+                super::super::protocol_serde::shape_job_state_time_limit_action::ser_job_state_time_limit_action(&mut object_21, item_20)?;
+                object_21.finish();
             }
         }
-        object_19.finish();
+        array_19.finish();
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_create_quota_share.rs`

```diff
--- reference/src/protocol_serde/shape_create_quota_share.rs
+++ generated/src/protocol_serde/shape_create_quota_share.rs
@@ -93,15 +93,15 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "quotaShareArn" => {
-                    builder = builder.set_quota_share_arn(
+                "quotaShareName" => {
+                    builder = builder.set_quota_share_name(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "quotaShareName" => {
-                    builder = builder.set_quota_share_name(
+                "quotaShareArn" => {
+                    builder = builder.set_quota_share_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
```

### `src/protocol_serde/shape_create_quota_share_input.rs`

```diff
--- reference/src/protocol_serde/shape_create_quota_share_input.rs
+++ generated/src/protocol_serde/shape_create_quota_share_input.rs
@@ -3,37 +3,37 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::operation::create_quota_share::CreateQuotaShareInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.capacity_limits {
-        let mut array_2 = object.key("capacityLimits").start_array();
-        for item_3 in var_1 {
+    if let Some(var_1) = &input.quota_share_name {
+        object.key("quotaShareName").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.job_queue {
+        object.key("jobQueue").string(var_2.as_str());
+    }
+    if let Some(var_3) = &input.capacity_limits {
+        let mut array_4 = object.key("capacityLimits").start_array();
+        for item_5 in var_3 {
             {
                 #[allow(unused_mut)]
-                let mut object_4 = array_2.value().start_object();
-                super::super::protocol_serde::shape_quota_share_capacity_limit::ser_quota_share_capacity_limit(&mut object_4, item_3)?;
-                object_4.finish();
+                let mut object_6 = array_4.value().start_object();
+                super::super::protocol_serde::shape_quota_share_capacity_limit::ser_quota_share_capacity_limit(&mut object_6, item_5)?;
+                object_6.finish();
             }
         }
-        array_2.finish();
+        array_4.finish();
     }
-    if let Some(var_5) = &input.job_queue {
-        object.key("jobQueue").string(var_5.as_str());
-    }
-    if let Some(var_6) = &input.preemption_configuration {
-        #[allow(unused_mut)]
-        let mut object_7 = object.key("preemptionConfiguration").start_object();
-        super::super::protocol_serde::shape_quota_share_preemption_configuration::ser_quota_share_preemption_configuration(&mut object_7, var_6)?;
-        object_7.finish();
-    }
-    if let Some(var_8) = &input.quota_share_name {
-        object.key("quotaShareName").string(var_8.as_str());
-    }
-    if let Some(var_9) = &input.resource_sharing_configuration {
+    if let Some(var_7) = &input.resource_sharing_configuration {
         #[allow(unused_mut)]
-        let mut object_10 = object.key("resourceSharingConfiguration").start_object();
+        let mut object_8 = object.key("resourceSharingConfiguration").start_object();
         super::super::protocol_serde::shape_quota_share_resource_sharing_configuration::ser_quota_share_resource_sharing_configuration(
-            &mut object_10,
-            var_9,
+            &mut object_8,
+            var_7,
         )?;
+        object_8.finish();
+    }
+    if let Some(var_9) = &input.preemption_configuration {
+        #[allow(unused_mut)]
+        let mut object_10 = object.key("preemptionConfiguration").start_object();
+        super::super::protocol_serde::shape_quota_share_preemption_configuration::ser_quota_share_preemption_configuration(&mut object_10, var_9)?;
         object_10.finish();
     }
     if let Some(var_11) = &input.state {
```

### `src/protocol_serde/shape_create_scheduling_policy.rs`

```diff
--- reference/src/protocol_serde/shape_create_scheduling_policy.rs
+++ generated/src/protocol_serde/shape_create_scheduling_policy.rs
@@ -73,7 +73,9 @@
         output = super::super::protocol_serde::shape_create_scheduling_policy::de_create_scheduling_policy(_response_body, output)
             .map_err(super::super::operation::create_scheduling_policy::CreateSchedulingPolicyError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
-        super::super::serde_util::create_scheduling_policy_output_output_correct_errors(output).build()
+        super::super::serde_util::create_scheduling_policy_output_output_correct_errors(output)
+            .build()
+            .map_err(super::super::operation::create_scheduling_policy::CreateSchedulingPolicyError::unhandled)?
     })
 }

@@ -103,15 +105,15 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "arn" => {
-                    builder = builder.set_arn(
+                "name" => {
+                    builder = builder.set_name(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "name" => {
-                    builder = builder.set_name(
+                "arn" => {
+                    builder = builder.set_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
```

### `src/protocol_serde/shape_create_scheduling_policy_input.rs`

```diff
--- reference/src/protocol_serde/shape_create_scheduling_policy_input.rs
+++ generated/src/protocol_serde/shape_create_scheduling_policy_input.rs
@@ -3,19 +3,19 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::operation::create_scheduling_policy::CreateSchedulingPolicyInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.fairshare_policy {
+    if let Some(var_1) = &input.name {
+        object.key("name").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.quota_share_policy {
         #[allow(unused_mut)]
-        let mut object_2 = object.key("fairsharePolicy").start_object();
-        super::super::protocol_serde::shape_fairshare_policy::ser_fairshare_policy(&mut object_2, var_1)?;
-        object_2.finish();
+        let mut object_3 = object.key("quotaSharePolicy").start_object();
+        super::super::protocol_serde::shape_quota_share_policy::ser_quota_share_policy(&mut object_3, var_2)?;
+        object_3.finish();
     }
-    if let Some(var_3) = &input.name {
-        object.key("name").string(var_3.as_str());
-    }
-    if let Some(var_4) = &input.quota_share_policy {
+    if let Some(var_4) = &input.fairshare_policy {
         #[allow(unused_mut)]
-        let mut object_5 = object.key("quotaSharePolicy").start_object();
-        super::super::protocol_serde::shape_quota_share_policy::ser_quota_share_policy(&mut object_5, var_4)?;
+        let mut object_5 = object.key("fairsharePolicy").start_object();
+        super::super::protocol_serde::shape_fairshare_policy::ser_fairshare_policy(&mut object_5, var_4)?;
         object_5.finish();
     }
     if let Some(var_6) = &input.tags {
```

### `src/protocol_serde/shape_create_service_environment.rs`

```diff
--- reference/src/protocol_serde/shape_create_service_environment.rs
+++ generated/src/protocol_serde/shape_create_service_environment.rs
@@ -73,7 +73,9 @@
         output = super::super::protocol_serde::shape_create_service_environment::de_create_service_environment(_response_body, output)
             .map_err(super::super::operation::create_service_environment::CreateServiceEnvironmentError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
-        super::super::serde_util::create_service_environment_output_output_correct_errors(output).build()
+        super::super::serde_util::create_service_environment_output_output_correct_errors(output)
+            .build()
+            .map_err(super::super::operation::create_service_environment::CreateServiceEnvironmentError::unhandled)?
     })
 }

@@ -103,15 +105,15 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "serviceEnvironmentArn" => {
-                    builder = builder.set_service_environment_arn(
+                "serviceEnvironmentName" => {
+                    builder = builder.set_service_environment_name(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "serviceEnvironmentName" => {
-                    builder = builder.set_service_environment_name(
+                "serviceEnvironmentArn" => {
+                    builder = builder.set_service_environment_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
```

### `src/protocol_serde/shape_create_service_environment_input.rs`

```diff
--- reference/src/protocol_serde/shape_create_service_environment_input.rs
+++ generated/src/protocol_serde/shape_create_service_environment_input.rs
@@ -3,26 +3,26 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::operation::create_service_environment::CreateServiceEnvironmentInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.capacity_limits {
-        let mut array_2 = object.key("capacityLimits").start_array();
-        for item_3 in var_1 {
+    if let Some(var_1) = &input.service_environment_name {
+        object.key("serviceEnvironmentName").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.service_environment_type {
+        object.key("serviceEnvironmentType").string(var_2.as_str());
+    }
+    if let Some(var_3) = &input.state {
+        object.key("state").string(var_3.as_str());
+    }
+    if let Some(var_4) = &input.capacity_limits {
+        let mut array_5 = object.key("capacityLimits").start_array();
+        for item_6 in var_4 {
             {
                 #[allow(unused_mut)]
-                let mut object_4 = array_2.value().start_object();
-                super::super::protocol_serde::shape_capacity_limit::ser_capacity_limit(&mut object_4, item_3)?;
-                object_4.finish();
+                let mut object_7 = array_5.value().start_object();
+                super::super::protocol_serde::shape_capacity_limit::ser_capacity_limit(&mut object_7, item_6)?;
+                object_7.finish();
             }
         }
-        array_2.finish();
-    }
-    if let Some(var_5) = &input.service_environment_name {
-        object.key("serviceEnvironmentName").string(var_5.as_str());
-    }
-    if let Some(var_6) = &input.service_environment_type {
-        object.key("serviceEnvironmentType").string(var_6.as_str());
-    }
-    if let Some(var_7) = &input.state {
-        object.key("state").string(var_7.as_str());
+        array_5.finish();
     }
     if let Some(var_8) = &input.tags {
         #[allow(unused_mut)]
```

### `src/protocol_serde/shape_describe_consumable_resource.rs`

```diff
--- reference/src/protocol_serde/shape_describe_consumable_resource.rs
+++ generated/src/protocol_serde/shape_describe_consumable_resource.rs
@@ -69,7 +69,9 @@
         output = super::super::protocol_serde::shape_describe_consumable_resource::de_describe_consumable_resource(_response_body, output)
             .map_err(super::super::operation::describe_consumable_resource::DescribeConsumableResourceError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
-        super::super::serde_util::describe_consumable_resource_output_output_correct_errors(output).build()
+        super::super::serde_util::describe_consumable_resource_output_output_correct_errors(output)
+            .build()
+            .map_err(super::super::operation::describe_consumable_resource::DescribeConsumableResourceError::unhandled)?
     })
 }

@@ -99,10 +101,10 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "availableQuantity" => {
-                    builder = builder.set_available_quantity(
-                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
-                            .map(i64::try_from)
+                "consumableResourceName" => {
+                    builder = builder.set_consumable_resource_name(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
@@ -113,22 +115,22 @@
                             .transpose()?,
                     );
                 }
-                "consumableResourceName" => {
-                    builder = builder.set_consumable_resource_name(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                "totalQuantity" => {
+                    builder = builder.set_total_quantity(
+                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                            .map(i64::try_from)
                             .transpose()?,
                     );
                 }
-                "createdAt" => {
-                    builder = builder.set_created_at(
+                "inUseQuantity" => {
+                    builder = builder.set_in_use_quantity(
                         ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                             .map(i64::try_from)
                             .transpose()?,
                     );
                 }
-                "inUseQuantity" => {
-                    builder = builder.set_in_use_quantity(
+                "availableQuantity" => {
+                    builder = builder.set_available_quantity(
                         ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                             .map(i64::try_from)
                             .transpose()?,
@@ -141,6 +143,13 @@
                             .transpose()?,
                     );
                 }
+                "createdAt" => {
+                    builder = builder.set_created_at(
+                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                            .map(i64::try_from)
+                            .transpose()?,
+                    );
+                }
                 "tags" => {
                     builder = builder.set_tags(super::super::protocol_serde::shape_tagris_tags_map::de_tagris_tags_map(
                         tokens,
@@ -148,13 +157,6 @@
                         depth + 1,
                     )?);
                 }
-                "totalQuantity" => {
-                    builder = builder.set_total_quantity(
-                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
-                            .map(i64::try_from)
-                            .transpose()?,
-                    );
-                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_describe_job_definitions_input.rs`

```diff
--- reference/src/protocol_serde/shape_describe_job_definitions_input.rs
+++ generated/src/protocol_serde/shape_describe_job_definitions_input.rs
@@ -3,29 +3,29 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::operation::describe_job_definitions::DescribeJobDefinitionsInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.job_definition_name {
-        object.key("jobDefinitionName").string(var_1.as_str());
-    }
-    if let Some(var_2) = &input.job_definitions {
-        let mut array_3 = object.key("jobDefinitions").start_array();
-        for item_4 in var_2 {
+    if let Some(var_1) = &input.job_definitions {
+        let mut array_2 = object.key("jobDefinitions").start_array();
+        for item_3 in var_1 {
             {
-                array_3.value().string(item_4.as_str());
+                array_2.value().string(item_3.as_str());
             }
         }
-        array_3.finish();
+        array_2.finish();
     }
-    if let Some(var_5) = &input.max_results {
+    if let Some(var_4) = &input.max_results {
         object.key("maxResults").number(
             #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_5).into()),
+            ::aws_smithy_types::Number::NegInt((*var_4).into()),
         );
     }
-    if let Some(var_6) = &input.next_token {
-        object.key("nextToken").string(var_6.as_str());
+    if let Some(var_5) = &input.job_definition_name {
+        object.key("jobDefinitionName").string(var_5.as_str());
+    }
+    if let Some(var_6) = &input.status {
+        object.key("status").string(var_6.as_str());
     }
-    if let Some(var_7) = &input.status {
-        object.key("status").string(var_7.as_str());
+    if let Some(var_7) = &input.next_token {
+        object.key("nextToken").string(var_7.as_str());
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_describe_quota_share.rs`

```diff
--- reference/src/protocol_serde/shape_describe_quota_share.rs
+++ generated/src/protocol_serde/shape_describe_quota_share.rs
@@ -100,27 +100,13 @@
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                 match key.to_unescaped()?.as_ref() {
-                    "capacityLimits" => {
-                        builder = builder.set_capacity_limits(
-                            super::super::protocol_serde::shape_quota_share_capacity_limits::de_quota_share_capacity_limits(tokens, _value, depth + 1)?,
-                        );
-                    }
-                    "jobQueueArn" => {
-                        builder = builder.set_job_queue_arn(
+                    "quotaShareName" => {
+                        builder = builder.set_quota_share_name(
                             ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                 .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                 .transpose()?,
                         );
                     }
-                    "preemptionConfiguration" => {
-                        builder = builder.set_preemption_configuration(
-                            super::super::protocol_serde::shape_quota_share_preemption_configuration::de_quota_share_preemption_configuration(
-                                tokens,
-                                _value,
-                                depth + 1,
-                            )?,
-                        );
-                    }
                     "quotaShareArn" => {
                         builder = builder.set_quota_share_arn(
                             ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -128,13 +114,18 @@
                                 .transpose()?,
                         );
                     }
-                    "quotaShareName" => {
-                        builder = builder.set_quota_share_name(
+                    "jobQueueArn" => {
+                        builder = builder.set_job_queue_arn(
                             ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                 .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                 .transpose()?,
                         );
                     }
+                    "capacityLimits" => {
+                        builder = builder.set_capacity_limits(
+                            super::super::protocol_serde::shape_quota_share_capacity_limits::de_quota_share_capacity_limits(tokens, _value, depth + 1)?,
+                        );
+                    }
                     "resourceSharingConfiguration" => {
                         builder = builder.set_resource_sharing_configuration(
                             super::super::protocol_serde::shape_quota_share_resource_sharing_configuration::de_quota_share_resource_sharing_configuration(
@@ -144,6 +135,15 @@
                             )?,
                         );
                     }
+                    "preemptionConfiguration" => {
+                        builder = builder.set_preemption_configuration(
+                            super::super::protocol_serde::shape_quota_share_preemption_configuration::de_quota_share_preemption_configuration(
+                                tokens,
+                                _value,
+                                depth + 1,
+                            )?,
+                        );
+                    }
                     "state" => {
                         builder = builder.set_state(
                             ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
```

### `src/protocol_serde/shape_describe_service_environments.rs`

```diff
--- reference/src/protocol_serde/shape_describe_service_environments.rs
+++ generated/src/protocol_serde/shape_describe_service_environments.rs
@@ -99,6 +99,11 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                "serviceEnvironments" => {
+                    builder = builder.set_service_environments(
+                        super::super::protocol_serde::shape_service_environment_detail_list::de_service_environment_detail_list(tokens, _value, depth + 1)?,
+                    );
+                }
                 "nextToken" => {
                     builder = builder.set_next_token(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -106,11 +111,6 @@
                             .transpose()?,
                     );
                 }
-                "serviceEnvironments" => {
-                    builder = builder.set_service_environments(
-                        super::super::protocol_serde::shape_service_environment_detail_list::de_service_environment_detail_list(tokens, _value, depth + 1)?,
-                    );
-                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_describe_service_environments_input.rs`

```diff
--- reference/src/protocol_serde/shape_describe_service_environments_input.rs
+++ generated/src/protocol_serde/shape_describe_service_environments_input.rs
@@ -3,23 +3,23 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::operation::describe_service_environments::DescribeServiceEnvironmentsInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.max_results {
+    if let Some(var_1) = &input.service_environments {
+        let mut array_2 = object.key("serviceEnvironments").start_array();
+        for item_3 in var_1 {
+            {
+                array_2.value().string(item_3.as_str());
+            }
+        }
+        array_2.finish();
+    }
+    if let Some(var_4) = &input.max_results {
         object.key("maxResults").number(
             #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_1).into()),
+            ::aws_smithy_types::Number::NegInt((*var_4).into()),
         );
     }
-    if let Some(var_2) = &input.next_token {
-        object.key("nextToken").string(var_2.as_str());
-    }
-    if let Some(var_3) = &input.service_environments {
-        let mut array_4 = object.key("serviceEnvironments").start_array();
-        for item_5 in var_3 {
-            {
-                array_4.value().string(item_5.as_str());
-            }
-        }
-        array_4.finish();
+    if let Some(var_5) = &input.next_token {
+        object.key("nextToken").string(var_5.as_str());
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_describe_service_job.rs`

```diff
--- reference/src/protocol_serde/shape_describe_service_job.rs
+++ generated/src/protocol_serde/shape_describe_service_job.rs
@@ -69,7 +69,9 @@
         output = super::super::protocol_serde::shape_describe_service_job::de_describe_service_job(_response_body, output)
             .map_err(super::super::operation::describe_service_job::DescribeServiceJobError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
-        super::super::serde_util::describe_service_job_output_output_correct_errors(output).build()
+        super::super::serde_util::describe_service_job_output_output_correct_errors(output)
+            .build()
+            .map_err(super::super::operation::describe_service_job::DescribeServiceJobError::unhandled)?
     })
 }

@@ -160,27 +162,6 @@
                         depth + 1,
                     )?);
                 }
-                "preemptionConfiguration" => {
-                    builder = builder.set_preemption_configuration(
-                        super::super::protocol_serde::shape_service_job_preemption_configuration::de_service_job_preemption_configuration(
-                            tokens,
-                            _value,
-                            depth + 1,
-                        )?,
-                    );
-                }
-                "preemptionSummary" => {
-                    builder = builder.set_preemption_summary(
-                        super::super::protocol_serde::shape_service_job_preemption_summary::de_service_job_preemption_summary(tokens, _value, depth + 1)?,
-                    );
-                }
-                "quotaShareName" => {
-                    builder = builder.set_quota_share_name(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
-                }
                 "retryStrategy" => {
                     builder = builder.set_retry_strategy(super::super::protocol_serde::shape_service_job_retry_strategy::de_service_job_retry_strategy(
                         tokens,
@@ -202,6 +183,13 @@
                             .transpose()?,
                     );
                 }
+                "serviceRequestPayload" => {
+                    builder = builder.set_service_request_payload(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
+                }
                 "serviceJobType" => {
                     builder = builder.set_service_job_type(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -209,20 +197,34 @@
                             .transpose()?,
                     );
                 }
-                "serviceRequestPayload" => {
-                    builder = builder.set_service_request_payload(
+                "shareIdentifier" => {
+                    builder = builder.set_share_identifier(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "shareIdentifier" => {
-                    builder = builder.set_share_identifier(
+                "quotaShareName" => {
+                    builder = builder.set_quota_share_name(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
+                "preemptionConfiguration" => {
+                    builder = builder.set_preemption_configuration(
+                        super::super::protocol_serde::shape_service_job_preemption_configuration::de_service_job_preemption_configuration(
+                            tokens,
+                            _value,
+                            depth + 1,
+                        )?,
+                    );
+                }
+                "preemptionSummary" => {
+                    builder = builder.set_preemption_summary(
+                        super::super::protocol_serde::shape_service_job_preemption_summary::de_service_job_preemption_summary(tokens, _value, depth + 1)?,
+                    );
+                }
                 "startedAt" => {
                     builder = builder.set_started_at(
                         ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
```

### `src/protocol_serde/shape_device.rs`

```diff
--- reference/src/protocol_serde/shape_device.rs
+++ generated/src/protocol_serde/shape_device.rs
@@ -3,20 +3,20 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::Device,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.host_path {
-        object.key("hostPath").string(var_1.as_str());
+    {
+        object.key("hostPath").string(input.host_path.as_str());
     }
-    if let Some(var_2) = &input.container_path {
-        object.key("containerPath").string(var_2.as_str());
+    if let Some(var_1) = &input.container_path {
+        object.key("containerPath").string(var_1.as_str());
     }
-    if let Some(var_3) = &input.permissions {
-        let mut array_4 = object.key("permissions").start_array();
-        for item_5 in var_3 {
+    if let Some(var_2) = &input.permissions {
+        let mut array_3 = object.key("permissions").start_array();
+        for item_4 in var_2 {
             {
-                array_4.value().string(item_5.as_str());
+                array_3.value().string(item_4.as_str());
             }
         }
-        array_4.finish();
+        array_3.finish();
     }
     Ok(())
 }
@@ -73,7 +73,9 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::device_correct_errors(builder).build()))
+            Ok(Some(super::super::serde_util::device_correct_errors(builder).build().map_err(|err| {
+                ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err)
+            })?))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_ec2_configuration.rs`

```diff
--- reference/src/protocol_serde/shape_ec2_configuration.rs
+++ generated/src/protocol_serde/shape_ec2_configuration.rs
@@ -3,17 +3,17 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::Ec2Configuration,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.image_type {
-        object.key("imageType").string(var_1.as_str());
+    {
+        object.key("imageType").string(input.image_type.as_str());
     }
-    if let Some(var_2) = &input.image_id_override {
-        object.key("imageIdOverride").string(var_2.as_str());
+    if let Some(var_1) = &input.image_id_override {
+        object.key("imageIdOverride").string(var_1.as_str());
     }
-    if let Some(var_3) = &input.batch_image_status {
-        object.key("batchImageStatus").string(var_3.as_str());
+    if let Some(var_2) = &input.batch_image_status {
+        object.key("batchImageStatus").string(var_2.as_str());
     }
-    if let Some(var_4) = &input.image_kubernetes_version {
-        object.key("imageKubernetesVersion").string(var_4.as_str());
+    if let Some(var_3) = &input.image_kubernetes_version {
+        object.key("imageKubernetesVersion").string(var_3.as_str());
     }
     Ok(())
 }
@@ -77,7 +77,9 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::ec2_configuration_correct_errors(builder).build()))
+            Ok(Some(super::super::serde_util::ec2_configuration_correct_errors(builder).build().map_err(
+                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
+            )?))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_ecs_properties.rs`

```diff
--- reference/src/protocol_serde/shape_ecs_properties.rs
+++ generated/src/protocol_serde/shape_ecs_properties.rs
@@ -3,17 +3,17 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::EcsProperties,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.task_properties {
-        let mut array_2 = object.key("taskProperties").start_array();
-        for item_3 in var_1 {
+    {
+        let mut array_1 = object.key("taskProperties").start_array();
+        for item_2 in &input.task_properties {
             {
                 #[allow(unused_mut)]
-                let mut object_4 = array_2.value().start_object();
-                super::super::protocol_serde::shape_ecs_task_properties::ser_ecs_task_properties(&mut object_4, item_3)?;
-                object_4.finish();
+                let mut object_3 = array_1.value().start_object();
+                super::super::protocol_serde::shape_ecs_task_properties::ser_ecs_task_properties(&mut object_3, item_2)?;
+                object_3.finish();
             }
         }
-        array_2.finish();
+        array_1.finish();
     }
     Ok(())
 }
@@ -54,7 +54,9 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::ecs_properties_correct_errors(builder).build()))
+            Ok(Some(super::super::serde_util::ecs_properties_correct_errors(builder).build().map_err(
+                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
+            )?))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_ecs_task_properties.rs`

```diff
--- reference/src/protocol_serde/shape_ecs_task_properties.rs
+++ generated/src/protocol_serde/shape_ecs_task_properties.rs
@@ -3,68 +3,68 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::EcsTaskProperties,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.containers {
-        let mut array_2 = object.key("containers").start_array();
-        for item_3 in var_1 {
+    {
+        let mut array_1 = object.key("containers").start_array();
+        for item_2 in &input.containers {
             {
                 #[allow(unused_mut)]
-                let mut object_4 = array_2.value().start_object();
-                super::super::protocol_serde::shape_task_container_properties::ser_task_container_properties(&mut object_4, item_3)?;
-                object_4.finish();
+                let mut object_3 = array_1.value().start_object();
+                super::super::protocol_serde::shape_task_container_properties::ser_task_container_properties(&mut object_3, item_2)?;
+                object_3.finish();
             }
         }
-        array_2.finish();
+        array_1.finish();
     }
-    if let Some(var_5) = &input.ephemeral_storage {
+    if let Some(var_4) = &input.ephemeral_storage {
         #[allow(unused_mut)]
-        let mut object_6 = object.key("ephemeralStorage").start_object();
-        super::super::protocol_serde::shape_ephemeral_storage::ser_ephemeral_storage(&mut object_6, var_5)?;
-        object_6.finish();
+        let mut object_5 = object.key("ephemeralStorage").start_object();
+        super::super::protocol_serde::shape_ephemeral_storage::ser_ephemeral_storage(&mut object_5, var_4)?;
+        object_5.finish();
     }
-    if let Some(var_7) = &input.execution_role_arn {
-        object.key("executionRoleArn").string(var_7.as_str());
+    if let Some(var_6) = &input.execution_role_arn {
+        object.key("executionRoleArn").string(var_6.as_str());
     }
-    if let Some(var_8) = &input.platform_version {
-        object.key("platformVersion").string(var_8.as_str());
+    if let Some(var_7) = &input.platform_version {
+        object.key("platformVersion").string(var_7.as_str());
     }
-    if let Some(var_9) = &input.ipc_mode {
-        object.key("ipcMode").string(var_9.as_str());
+    if let Some(var_8) = &input.ipc_mode {
+        object.key("ipcMode").string(var_8.as_str());
     }
-    if let Some(var_10) = &input.task_role_arn {
-        object.key("taskRoleArn").string(var_10.as_str());
+    if let Some(var_9) = &input.task_role_arn {
+        object.key("taskRoleArn").string(var_9.as_str());
     }
-    if let Some(var_11) = &input.pid_mode {
-        object.key("pidMode").string(var_11.as_str());
+    if let Some(var_10) = &input.pid_mode {
+        object.key("pidMode").string(var_10.as_str());
     }
-    if let Some(var_12) = &input.network_configuration {
+    if let Some(var_11) = &input.network_configuration {
         #[allow(unused_mut)]
-        let mut object_13 = object.key("networkConfiguration").start_object();
-        super::super::protocol_serde::shape_network_configuration::ser_network_configuration(&mut object_13, var_12)?;
-        object_13.finish();
+        let mut object_12 = object.key("networkConfiguration").start_object();
+        super::super::protocol_serde::shape_network_configuration::ser_network_configuration(&mut object_12, var_11)?;
+        object_12.finish();
     }
-    if let Some(var_14) = &input.runtime_platform {
+    if let Some(var_13) = &input.runtime_platform {
         #[allow(unused_mut)]
-        let mut object_15 = object.key("runtimePlatform").start_object();
-        super::super::protocol_serde::shape_runtime_platform::ser_runtime_platform(&mut object_15, var_14)?;
-        object_15.finish();
+        let mut object_14 = object.key("runtimePlatform").start_object();
+        super::super::protocol_serde::shape_runtime_platform::ser_runtime_platform(&mut object_14, var_13)?;
+        object_14.finish();
     }
-    if let Some(var_16) = &input.volumes {
-        let mut array_17 = object.key("volumes").start_array();
-        for item_18 in var_16 {
+    if let Some(var_15) = &input.volumes {
+        let mut array_16 = object.key("volumes").start_array();
+        for item_17 in var_15 {
             {
                 #[allow(unused_mut)]
-                let mut object_19 = array_17.value().start_object();
-                super::super::protocol_serde::shape_volume::ser_volume(&mut object_19, item_18)?;
-                object_19.finish();
+                let mut object_18 = array_16.value().start_object();
+                super::super::protocol_serde::shape_volume::ser_volume(&mut object_18, item_17)?;
+                object_18.finish();
             }
         }
-        array_17.finish();
+        array_16.finish();
     }
-    if let Some(var_20) = &input.enable_execute_command {
-        object.key("enableExecuteCommand").boolean(*var_20);
+    if let Some(var_19) = &input.enable_execute_command {
+        object.key("enableExecuteCommand").boolean(*var_19);
     }
-    if let Some(var_21) = &input.network_mode {
-        object.key("networkMode").string(var_21.as_str());
+    if let Some(var_20) = &input.network_mode {
+        object.key("networkMode").string(var_20.as_str());
     }
     Ok(())
 }
@@ -176,7 +176,9 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::ecs_task_properties_correct_errors(builder).build()))
+            Ok(Some(super::super::serde_util::ecs_task_properties_correct_errors(builder).build().map_err(
+                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
+            )?))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_efs_volume_configuration.rs`

```diff
--- reference/src/protocol_serde/shape_efs_volume_configuration.rs
+++ generated/src/protocol_serde/shape_efs_volume_configuration.rs
@@ -3,26 +3,26 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::EfsVolumeConfiguration,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.file_system_id {
-        object.key("fileSystemId").string(var_1.as_str());
+    {
+        object.key("fileSystemId").string(input.file_system_id.as_str());
     }
-    if let Some(var_2) = &input.root_directory {
-        object.key("rootDirectory").string(var_2.as_str());
+    if let Some(var_1) = &input.root_directory {
+        object.key("rootDirectory").string(var_1.as_str());
     }
-    if let Some(var_3) = &input.transit_encryption {
-        object.key("transitEncryption").string(var_3.as_str());
+    if let Some(var_2) = &input.transit_encryption {
+        object.key("transitEncryption").string(var_2.as_str());
     }
-    if let Some(var_4) = &input.transit_encryption_port {
+    if let Some(var_3) = &input.transit_encryption_port {
         object.key("transitEncryptionPort").number(
             #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_4).into()),
+            ::aws_smithy_types::Number::NegInt((*var_3).into()),
         );
     }
-    if let Some(var_5) = &input.authorization_config {
+    if let Some(var_4) = &input.authorization_config {
         #[allow(unused_mut)]
-        let mut object_6 = object.key("authorizationConfig").start_object();
-        super::super::protocol_serde::shape_efs_authorization_config::ser_efs_authorization_config(&mut object_6, var_5)?;
-        object_6.finish();
+        let mut object_5 = object.key("authorizationConfig").start_object();
+        super::super::protocol_serde::shape_efs_authorization_config::ser_efs_authorization_config(&mut object_5, var_4)?;
+        object_5.finish();
     }
     Ok(())
 }
@@ -91,7 +91,11 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::efs_volume_configuration_correct_errors(builder).build()))
+            Ok(Some(
+                super::super::serde_util::efs_volume_configuration_correct_errors(builder)
+                    .build()
+                    .map_err(|err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err))?,
+            ))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_eks_configuration.rs`

```diff
--- reference/src/protocol_serde/shape_eks_configuration.rs
+++ generated/src/protocol_serde/shape_eks_configuration.rs
@@ -3,11 +3,11 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::EksConfiguration,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.eks_cluster_arn {
-        object.key("eksClusterArn").string(var_1.as_str());
+    {
+        object.key("eksClusterArn").string(input.eks_cluster_arn.as_str());
     }
-    if let Some(var_2) = &input.kubernetes_namespace {
-        object.key("kubernetesNamespace").string(var_2.as_str());
+    {
+        object.key("kubernetesNamespace").string(input.kubernetes_namespace.as_str());
     }
     Ok(())
 }
@@ -57,7 +57,9 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::eks_configuration_correct_errors(builder).build()))
+            Ok(Some(super::super::serde_util::eks_configuration_correct_errors(builder).build().map_err(
+                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
+            )?))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_eks_container.rs`

```diff
--- reference/src/protocol_serde/shape_eks_container.rs
+++ generated/src/protocol_serde/shape_eks_container.rs
@@ -6,65 +6,65 @@
     if let Some(var_1) = &input.name {
         object.key("name").string(var_1.as_str());
     }
-    if let Some(var_2) = &input.image {
-        object.key("image").string(var_2.as_str());
+    {
+        object.key("image").string(input.image.as_str());
     }
-    if let Some(var_3) = &input.image_pull_policy {
-        object.key("imagePullPolicy").string(var_3.as_str());
+    if let Some(var_2) = &input.image_pull_policy {
+        object.key("imagePullPolicy").string(var_2.as_str());
     }
-    if let Some(var_4) = &input.command {
-        let mut array_5 = object.key("command").start_array();
-        for item_6 in var_4 {
+    if let Some(var_3) = &input.command {
+        let mut array_4 = object.key("command").start_array();
+        for item_5 in var_3 {
             {
-                array_5.value().string(item_6.as_str());
+                array_4.value().string(item_5.as_str());
             }
         }
-        array_5.finish();
+        array_4.finish();
     }
-    if let Some(var_7) = &input.args {
-        let mut array_8 = object.key("args").start_array();
-        for item_9 in var_7 {
+    if let Some(var_6) = &input.args {
+        let mut array_7 = object.key("args").start_array();
+        for item_8 in var_6 {
             {
-                array_8.value().string(item_9.as_str());
+                array_7.value().string(item_8.as_str());
             }
         }
-        array_8.finish();
+        array_7.finish();
     }
-    if let Some(var_10) = &input.env {
-        let mut array_11 = object.key("env").start_array();
-        for item_12 in var_10 {
+    if let Some(var_9) = &input.env {
+        let mut array_10 = object.key("env").start_array();
+        for item_11 in var_9 {
             {
                 #[allow(unused_mut)]
-                let mut object_13 = array_11.value().start_object();
-                super::super::protocol_serde::shape_eks_container_environment_variable::ser_eks_container_environment_variable(&mut object_13, item_12)?;
-                object_13.finish();
+                let mut object_12 = array_10.value().start_object();
+                super::super::protocol_serde::shape_eks_container_environment_variable::ser_eks_container_environment_variable(&mut object_12, item_11)?;
+                object_12.finish();
             }
         }
-        array_11.finish();
+        array_10.finish();
     }
-    if let Some(var_14) = &input.resources {
+    if let Some(var_13) = &input.resources {
         #[allow(unused_mut)]
-        let mut object_15 = object.key("resources").start_object();
-        super::super::protocol_serde::shape_eks_container_resource_requirements::ser_eks_container_resource_requirements(&mut object_15, var_14)?;
-        object_15.finish();
+        let mut object_14 = object.key("resources").start_object();
+        super::super::protocol_serde::shape_eks_container_resource_requirements::ser_eks_container_resource_requirements(&mut object_14, var_13)?;
+        object_14.finish();
     }
-    if let Some(var_16) = &input.volume_mounts {
-        let mut array_17 = object.key("volumeMounts").start_array();
-        for item_18 in var_16 {
+    if let Some(var_15) = &input.volume_mounts {
+        let mut array_16 = object.key("volumeMounts").start_array();
+        for item_17 in var_15 {
             {
                 #[allow(unused_mut)]
-                let mut object_19 = array_17.value().start_object();
-                super::super::protocol_serde::shape_eks_container_volume_mount::ser_eks_container_volume_mount(&mut object_19, item_18)?;
-                object_19.finish();
+                let mut object_18 = array_16.value().start_object();
+                super::super::protocol_serde::shape_eks_container_volume_mount::ser_eks_container_volume_mount(&mut object_18, item_17)?;
+                object_18.finish();
             }
         }
-        array_17.finish();
+        array_16.finish();
     }
-    if let Some(var_20) = &input.security_context {
+    if let Some(var_19) = &input.security_context {
         #[allow(unused_mut)]
-        let mut object_21 = object.key("securityContext").start_object();
-        super::super::protocol_serde::shape_eks_container_security_context::ser_eks_container_security_context(&mut object_21, var_20)?;
-        object_21.finish();
+        let mut object_20 = object.key("securityContext").start_object();
+        super::super::protocol_serde::shape_eks_container_security_context::ser_eks_container_security_context(&mut object_20, var_19)?;
+        object_20.finish();
     }
     Ok(())
 }
@@ -159,7 +159,9 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::eks_container_correct_errors(builder).build()))
+            Ok(Some(super::super::serde_util::eks_container_correct_errors(builder).build().map_err(|err| {
+                ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err)
+            })?))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_eks_container_environment_variable.rs`

```diff
--- reference/src/protocol_serde/shape_eks_container_environment_variable.rs
+++ generated/src/protocol_serde/shape_eks_container_environment_variable.rs
@@ -3,11 +3,11 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::EksContainerEnvironmentVariable,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.name {
-        object.key("name").string(var_1.as_str());
+    {
+        object.key("name").string(input.name.as_str());
     }
-    if let Some(var_2) = &input.value {
-        object.key("value").string(var_2.as_str());
+    if let Some(var_1) = &input.value {
+        object.key("value").string(var_1.as_str());
     }
     Ok(())
 }
@@ -58,7 +58,9 @@
                 }
             }
             Ok(Some(
-                super::super::serde_util::eks_container_environment_variable_correct_errors(builder).build(),
+                super::super::serde_util::eks_container_environment_variable_correct_errors(builder)
+                    .build()
+                    .map_err(|err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err))?,
             ))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
```

### `src/protocol_serde/shape_eks_persistent_volume_claim.rs`

```diff
--- reference/src/protocol_serde/shape_eks_persistent_volume_claim.rs
+++ generated/src/protocol_serde/shape_eks_persistent_volume_claim.rs
@@ -3,11 +3,11 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::EksPersistentVolumeClaim,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.claim_name {
-        object.key("claimName").string(var_1.as_str());
+    {
+        object.key("claimName").string(input.claim_name.as_str());
     }
-    if let Some(var_2) = &input.read_only {
-        object.key("readOnly").boolean(*var_2);
+    if let Some(var_1) = &input.read_only {
+        object.key("readOnly").boolean(*var_1);
     }
     Ok(())
 }
@@ -53,7 +53,11 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::eks_persistent_volume_claim_correct_errors(builder).build()))
+            Ok(Some(
+                super::super::serde_util::eks_persistent_volume_claim_correct_errors(builder)
+                    .build()
+                    .map_err(|err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err))?,
+            ))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_eks_secret.rs`

```diff
--- reference/src/protocol_serde/shape_eks_secret.rs
+++ generated/src/protocol_serde/shape_eks_secret.rs
@@ -3,11 +3,11 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::EksSecret,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.secret_name {
-        object.key("secretName").string(var_1.as_str());
+    {
+        object.key("secretName").string(input.secret_name.as_str());
     }
-    if let Some(var_2) = &input.optional {
-        object.key("optional").boolean(*var_2);
+    if let Some(var_1) = &input.optional {
+        object.key("optional").boolean(*var_1);
     }
     Ok(())
 }
@@ -53,7 +53,9 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::eks_secret_correct_errors(builder).build()))
+            Ok(Some(super::super::serde_util::eks_secret_correct_errors(builder).build().map_err(|err| {
+                ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err)
+            })?))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_eks_volume.rs`

```diff
--- reference/src/protocol_serde/shape_eks_volume.rs
+++ generated/src/protocol_serde/shape_eks_volume.rs
@@ -3,32 +3,32 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::EksVolume,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.name {
-        object.key("name").string(var_1.as_str());
+    {
+        object.key("name").string(input.name.as_str());
     }
-    if let Some(var_2) = &input.host_path {
+    if let Some(var_1) = &input.host_path {
         #[allow(unused_mut)]
-        let mut object_3 = object.key("hostPath").start_object();
-        super::super::protocol_serde::shape_eks_host_path::ser_eks_host_path(&mut object_3, var_2)?;
-        object_3.finish();
+        let mut object_2 = object.key("hostPath").start_object();
+        super::super::protocol_serde::shape_eks_host_path::ser_eks_host_path(&mut object_2, var_1)?;
+        object_2.finish();
     }
-    if let Some(var_4) = &input.empty_dir {
+    if let Some(var_3) = &input.empty_dir {
         #[allow(unused_mut)]
-        let mut object_5 = object.key("emptyDir").start_object();
-        super::super::protocol_serde::shape_eks_empty_dir::ser_eks_empty_dir(&mut object_5, var_4)?;
-        object_5.finish();
+        let mut object_4 = object.key("emptyDir").start_object();
+        super::super::protocol_serde::shape_eks_empty_dir::ser_eks_empty_dir(&mut object_4, var_3)?;
+        object_4.finish();
     }
-    if let Some(var_6) = &input.secret {
+    if let Some(var_5) = &input.secret {
         #[allow(unused_mut)]
-        let mut object_7 = object.key("secret").start_object();
-        super::super::protocol_serde::shape_eks_secret::ser_eks_secret(&mut object_7, var_6)?;
-        object_7.finish();
+        let mut object_6 = object.key("secret").start_object();
+        super::super::protocol_serde::shape_eks_secret::ser_eks_secret(&mut object_6, var_5)?;
+        object_6.finish();
     }
-    if let Some(var_8) = &input.persistent_volume_claim {
+    if let Some(var_7) = &input.persistent_volume_claim {
         #[allow(unused_mut)]
-        let mut object_9 = object.key("persistentVolumeClaim").start_object();
-        super::super::protocol_serde::shape_eks_persistent_volume_claim::ser_eks_persistent_volume_claim(&mut object_9, var_8)?;
-        object_9.finish();
+        let mut object_8 = object.key("persistentVolumeClaim").start_object();
+        super::super::protocol_serde::shape_eks_persistent_volume_claim::ser_eks_persistent_volume_claim(&mut object_8, var_7)?;
+        object_8.finish();
     }
     Ok(())
 }
@@ -85,7 +85,9 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::eks_volume_correct_errors(builder).build()))
+            Ok(Some(super::super::serde_util::eks_volume_correct_errors(builder).build().map_err(|err| {
+                ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err)
+            })?))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_ephemeral_storage.rs`

```diff
--- reference/src/protocol_serde/shape_ephemeral_storage.rs
+++ generated/src/protocol_serde/shape_ephemeral_storage.rs
@@ -3,10 +3,10 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::EphemeralStorage,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.size_in_gib {
+    {
         object.key("sizeInGiB").number(
             #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_1).into()),
+            ::aws_smithy_types::Number::NegInt((input.size_in_gi_b).into()),
         );
     }
     Ok(())
@@ -35,7 +35,7 @@
                     Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                     Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                         "sizeInGiB" => {
-                            builder = builder.set_size_in_gib(
+                            builder = builder.set_size_in_gi_b(
                                 ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                     .map(i32::try_from)
                                     .transpose()?,
@@ -50,7 +50,9 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::ephemeral_storage_correct_errors(builder).build()))
+            Ok(Some(super::super::serde_util::ephemeral_storage_correct_errors(builder).build().map_err(
+                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
+            )?))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_evaluate_on_exit.rs`

```diff
--- reference/src/protocol_serde/shape_evaluate_on_exit.rs
+++ generated/src/protocol_serde/shape_evaluate_on_exit.rs
@@ -12,8 +12,8 @@
     if let Some(var_3) = &input.on_exit_code {
         object.key("onExitCode").string(var_3.as_str());
     }
-    if let Some(var_4) = &input.action {
-        object.key("action").string(var_4.as_str());
+    {
+        object.key("action").string(input.action.as_str());
     }
     Ok(())
 }
@@ -77,7 +77,9 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::evaluate_on_exit_correct_errors(builder).build()))
+            Ok(Some(super::super::serde_util::evaluate_on_exit_correct_errors(builder).build().map_err(
+                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
+            )?))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_firelens_configuration.rs`

```diff
--- reference/src/protocol_serde/shape_firelens_configuration.rs
+++ generated/src/protocol_serde/shape_firelens_configuration.rs
@@ -3,18 +3,18 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::FirelensConfiguration,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.r#type {
-        object.key("type").string(var_1.as_str());
+    {
+        object.key("type").string(input.r#type.as_str());
     }
-    if let Some(var_2) = &input.options {
+    if let Some(var_1) = &input.options {
         #[allow(unused_mut)]
-        let mut object_3 = object.key("options").start_object();
-        for (key_4, value_5) in var_2 {
+        let mut object_2 = object.key("options").start_object();
+        for (key_3, value_4) in var_1 {
             {
-                object_3.key(key_4.as_str()).string(value_5.as_str());
+                object_2.key(key_3.as_str()).string(value_4.as_str());
             }
         }
-        object_3.finish();
+        object_2.finish();
     }
     Ok(())
 }
@@ -66,7 +66,9 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::firelens_configuration_correct_errors(builder).build()))
+            Ok(Some(super::super::serde_util::firelens_configuration_correct_errors(builder).build().map_err(
+                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
+            )?))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_image_pull_secret.rs`

```diff
--- reference/src/protocol_serde/shape_image_pull_secret.rs
+++ generated/src/protocol_serde/shape_image_pull_secret.rs
@@ -3,8 +3,8 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::ImagePullSecret,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.name {
-        object.key("name").string(var_1.as_str());
+    {
+        object.key("name").string(input.name.as_str());
     }
     Ok(())
 }
@@ -47,7 +47,9 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::image_pull_secret_correct_errors(builder).build()))
+            Ok(Some(super::super::serde_util::image_pull_secret_correct_errors(builder).build().map_err(
+                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
+            )?))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_instance_launch_template.rs`

```diff
--- reference/src/protocol_serde/shape_instance_launch_template.rs
+++ generated/src/protocol_serde/shape_instance_launch_template.rs
@@ -3,53 +3,53 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::InstanceLaunchTemplate,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.ec2_instance_profile_arn {
-        object.key("ec2InstanceProfileArn").string(var_1.as_str());
+    {
+        object.key("ec2InstanceProfileArn").string(input.ec2_instance_profile_arn.as_str());
     }
-    if let Some(var_2) = &input.network_configuration {
+    if let Some(var_1) = &input.network_configuration {
         #[allow(unused_mut)]
-        let mut object_3 = object.key("networkConfiguration").start_object();
-        super::super::protocol_serde::shape_managed_instances_network_configuration::ser_managed_instances_network_configuration(&mut object_3, var_2)?;
-        object_3.finish();
+        let mut object_2 = object.key("networkConfiguration").start_object();
+        super::super::protocol_serde::shape_managed_instances_network_configuration::ser_managed_instances_network_configuration(&mut object_2, var_1)?;
+        object_2.finish();
     }
-    if let Some(var_4) = &input.instance_requirements {
+    if let Some(var_3) = &input.instance_requirements {
         #[allow(unused_mut)]
-        let mut object_5 = object.key("instanceRequirements").start_object();
-        super::super::protocol_serde::shape_instance_requirements_request::ser_instance_requirements_request(&mut object_5, var_4)?;
-        object_5.finish();
+        let mut object_4 = object.key("instanceRequirements").start_object();
+        super::super::protocol_serde::shape_instance_requirements_request::ser_instance_requirements_request(&mut object_4, var_3)?;
+        object_4.finish();
     }
-    if let Some(var_6) = &input.capacity_option_type {
-        object.key("capacityOptionType").string(var_6.as_str());
+    if let Some(var_5) = &input.capacity_option_type {
+        object.key("capacityOptionType").string(var_5.as_str());
     }
-    if let Some(var_7) = &input.storage_configuration {
+    if let Some(var_6) = &input.storage_configuration {
         #[allow(unused_mut)]
-        let mut object_8 = object.key("storageConfiguration").start_object();
-        super::super::protocol_serde::shape_managed_instances_storage_configuration::ser_managed_instances_storage_configuration(&mut object_8, var_7)?;
-        object_8.finish();
+        let mut object_7 = object.key("storageConfiguration").start_object();
+        super::super::protocol_serde::shape_managed_instances_storage_configuration::ser_managed_instances_storage_configuration(&mut object_7, var_6)?;
+        object_7.finish();
     }
-    if let Some(var_9) = &input.monitoring {
-        object.key("monitoring").string(var_9.as_str());
+    if let Some(var_8) = &input.monitoring {
+        object.key("monitoring").string(var_8.as_str());
     }
-    if let Some(var_10) = &input.fips_enabled {
-        object.key("fipsEnabled").boolean(*var_10);
+    if let Some(var_9) = &input.fips_enabled {
+        object.key("fipsEnabled").boolean(*var_9);
     }
-    if let Some(var_11) = &input.capacity_reservations {
+    if let Some(var_10) = &input.capacity_reservations {
         #[allow(unused_mut)]
-        let mut object_12 = object.key("capacityReservations").start_object();
-        super::super::protocol_serde::shape_capacity_reservation_request::ser_capacity_reservation_request(&mut object_12, var_11)?;
-        object_12.finish();
+        let mut object_11 = object.key("capacityReservations").start_object();
+        super::super::protocol_serde::shape_capacity_reservation_request::ser_capacity_reservation_request(&mut object_11, var_10)?;
+        object_11.finish();
     }
-    if let Some(var_13) = &input.instance_metadata_tags_propagation {
-        object.key("instanceMetadataTagsPropagation").boolean(*var_13);
+    if let Some(var_12) = &input.instance_metadata_tags_propagation {
+        object.key("instanceMetadataTagsPropagation").boolean(*var_12);
     }
-    if let Some(var_14) = &input.local_storage_configuration {
+    if let Some(var_13) = &input.local_storage_configuration {
         #[allow(unused_mut)]
-        let mut object_15 = object.key("localStorageConfiguration").start_object();
+        let mut object_14 = object.key("localStorageConfiguration").start_object();
         super::super::protocol_serde::shape_managed_instances_local_storage_configuration::ser_managed_instances_local_storage_configuration(
-            &mut object_15,
-            var_14,
+            &mut object_14,
+            var_13,
         )?;
-        object_15.finish();
+        object_14.finish();
     }
     Ok(())
 }
@@ -141,9 +141,7 @@
                                 .set_instance_metadata_tags_propagation(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                         }
                         "localStorageConfiguration" => {
-                            builder = builder.set_local_storage_configuration(
-                                    super::super::protocol_serde::shape_managed_instances_local_storage_configuration::de_managed_instances_local_storage_configuration(tokens, _value, depth + 1)?
-                                );
+                            builder = builder.set_local_storage_configuration(super::super::protocol_serde::shape_managed_instances_local_storage_configuration::de_managed_instances_local_storage_configuration(tokens, _value, depth + 1)?);
                         }
                         _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                     },
@@ -154,7 +152,11 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::instance_launch_template_correct_errors(builder).build()))
+            Ok(Some(
+                super::super::serde_util::instance_launch_template_correct_errors(builder)
+                    .build()
+                    .map_err(|err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err))?,
+            ))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_job_definition.rs`

```diff
--- reference/src/protocol_serde/shape_job_definition.rs
+++ generated/src/protocol_serde/shape_job_definition.rs
@@ -145,7 +145,9 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::job_definition_correct_errors(builder).build()))
+            Ok(Some(super::super::serde_util::job_definition_correct_errors(builder).build().map_err(
+                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
+            )?))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_job_detail.rs`

```diff
--- reference/src/protocol_serde/shape_job_detail.rs
+++ generated/src/protocol_serde/shape_job_detail.rs
@@ -218,7 +218,9 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::job_detail_correct_errors(builder).build()))
+            Ok(Some(super::super::serde_util::job_detail_correct_errors(builder).build().map_err(|err| {
+                ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err)
+            })?))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_job_queue_detail.rs`

```diff
--- reference/src/protocol_serde/shape_job_queue_detail.rs
+++ generated/src/protocol_serde/shape_job_queue_detail.rs
@@ -112,7 +112,9 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::job_queue_detail_correct_errors(builder).build()))
+            Ok(Some(super::super::serde_util::job_queue_detail_correct_errors(builder).build().map_err(
+                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
+            )?))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_job_state_time_limit_action.rs`

```diff
--- reference/src/protocol_serde/shape_job_state_time_limit_action.rs
+++ generated/src/protocol_serde/shape_job_state_time_limit_action.rs
@@ -3,20 +3,20 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::JobStateTimeLimitAction,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.reason {
-        object.key("reason").string(var_1.as_str());
+    {
+        object.key("reason").string(input.reason.as_str());
     }
-    if let Some(var_2) = &input.state {
-        object.key("state").string(var_2.as_str());
+    {
+        object.key("state").string(input.state.as_str());
     }
-    if let Some(var_3) = &input.max_time_seconds {
+    {
         object.key("maxTimeSeconds").number(
             #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_3).into()),
+            ::aws_smithy_types::Number::NegInt((input.max_time_seconds).into()),
         );
     }
-    if let Some(var_4) = &input.action {
-        object.key("action").string(var_4.as_str());
+    {
+        object.key("action").string(input.action.as_str());
     }
     Ok(())
 }
@@ -80,7 +80,11 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::job_state_time_limit_action_correct_errors(builder).build()))
+            Ok(Some(
+                super::super::serde_util::job_state_time_limit_action_correct_errors(builder)
+                    .build()
+                    .map_err(|err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err))?,
+            ))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_job_summary.rs`

```diff
--- reference/src/protocol_serde/shape_job_summary.rs
+++ generated/src/protocol_serde/shape_job_summary.rs
@@ -135,7 +135,9 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::job_summary_correct_errors(builder).build()))
+            Ok(Some(super::super::serde_util::job_summary_correct_errors(builder).build().map_err(|err| {
+                ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err)
+            })?))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_list_consumable_resources.rs`

```diff
--- reference/src/protocol_serde/shape_list_consumable_resources.rs
+++ generated/src/protocol_serde/shape_list_consumable_resources.rs
@@ -73,7 +73,9 @@
         output = super::super::protocol_serde::shape_list_consumable_resources::de_list_consumable_resources(_response_body, output)
             .map_err(super::super::operation::list_consumable_resources::ListConsumableResourcesError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
-        super::super::serde_util::list_consumable_resources_output_output_correct_errors(output).build()
+        super::super::serde_util::list_consumable_resources_output_output_correct_errors(output)
+            .build()
+            .map_err(super::super::operation::list_consumable_resources::ListConsumableResourcesError::unhandled)?
     })
 }

```

### `src/protocol_serde/shape_list_jobs.rs`

```diff
--- reference/src/protocol_serde/shape_list_jobs.rs
+++ generated/src/protocol_serde/shape_list_jobs.rs
@@ -63,7 +63,9 @@
         output = super::super::protocol_serde::shape_list_jobs::de_list_jobs(_response_body, output)
             .map_err(super::super::operation::list_jobs::ListJobsError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
-        super::super::serde_util::list_jobs_output_output_correct_errors(output).build()
+        super::super::serde_util::list_jobs_output_output_correct_errors(output)
+            .build()
+            .map_err(super::super::operation::list_jobs::ListJobsError::unhandled)?
     })
 }

```

### `src/protocol_serde/shape_list_jobs_by_consumable_resource.rs`

```diff
--- reference/src/protocol_serde/shape_list_jobs_by_consumable_resource.rs
+++ generated/src/protocol_serde/shape_list_jobs_by_consumable_resource.rs
@@ -69,7 +69,9 @@
         output = super::super::protocol_serde::shape_list_jobs_by_consumable_resource::de_list_jobs_by_consumable_resource(_response_body, output)
             .map_err(super::super::operation::list_jobs_by_consumable_resource::ListJobsByConsumableResourceError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
-        super::super::serde_util::list_jobs_by_consumable_resource_output_output_correct_errors(output).build()
+        super::super::serde_util::list_jobs_by_consumable_resource_output_output_correct_errors(output)
+            .build()
+            .map_err(super::super::operation::list_jobs_by_consumable_resource::ListJobsByConsumableResourceError::unhandled)?
     })
 }

@@ -101,9 +103,7 @@
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                 match key.to_unescaped()?.as_ref() {
                     "jobs" => {
-                        builder = builder.set_jobs(
-                            super::super::protocol_serde::shape_list_jobs_by_consumable_resource_summary_list::de_list_jobs_by_consumable_resource_summary_list(tokens, _value, depth + 1)?
-                        );
+                        builder = builder.set_jobs(super::super::protocol_serde::shape_list_jobs_by_consumable_resource_summary_list::de_list_jobs_by_consumable_resource_summary_list(tokens, _value, depth + 1)?);
                     }
                     "nextToken" => {
                         builder = builder.set_next_token(
```

### `src/protocol_serde/shape_list_jobs_by_consumable_resource_summary.rs`

```diff
--- reference/src/protocol_serde/shape_list_jobs_by_consumable_resource_summary.rs
+++ generated/src/protocol_serde/shape_list_jobs_by_consumable_resource_summary.rs
@@ -110,7 +110,9 @@
                 }
             }
             Ok(Some(
-                super::super::serde_util::list_jobs_by_consumable_resource_summary_correct_errors(builder).build(),
+                super::super::serde_util::list_jobs_by_consumable_resource_summary_correct_errors(builder)
+                    .build()
+                    .map_err(|err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err))?,
             ))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
```

### `src/protocol_serde/shape_list_jobs_input.rs`

```diff
--- reference/src/protocol_serde/shape_list_jobs_input.rs
+++ generated/src/protocol_serde/shape_list_jobs_input.rs
@@ -3,38 +3,38 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::operation::list_jobs::ListJobsInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.array_job_id {
-        object.key("arrayJobId").string(var_1.as_str());
+    if let Some(var_1) = &input.job_queue {
+        object.key("jobQueue").string(var_1.as_str());
     }
-    if let Some(var_2) = &input.filters {
-        let mut array_3 = object.key("filters").start_array();
-        for item_4 in var_2 {
-            {
-                #[allow(unused_mut)]
-                let mut object_5 = array_3.value().start_object();
-                super::super::protocol_serde::shape_key_values_pair::ser_key_values_pair(&mut object_5, item_4)?;
-                object_5.finish();
-            }
-        }
-        array_3.finish();
+    if let Some(var_2) = &input.array_job_id {
+        object.key("arrayJobId").string(var_2.as_str());
     }
-    if let Some(var_6) = &input.job_queue {
-        object.key("jobQueue").string(var_6.as_str());
+    if let Some(var_3) = &input.multi_node_job_id {
+        object.key("multiNodeJobId").string(var_3.as_str());
     }
-    if let Some(var_7) = &input.job_status {
-        object.key("jobStatus").string(var_7.as_str());
+    if let Some(var_4) = &input.job_status {
+        object.key("jobStatus").string(var_4.as_str());
     }
-    if let Some(var_8) = &input.max_results {
+    if let Some(var_5) = &input.max_results {
         object.key("maxResults").number(
             #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_8).into()),
+            ::aws_smithy_types::Number::NegInt((*var_5).into()),
         );
     }
-    if let Some(var_9) = &input.multi_node_job_id {
-        object.key("multiNodeJobId").string(var_9.as_str());
+    if let Some(var_6) = &input.next_token {
+        object.key("nextToken").string(var_6.as_str());
     }
-    if let Some(var_10) = &input.next_token {
-        object.key("nextToken").string(var_10.as_str());
+    if let Some(var_7) = &input.filters {
+        let mut array_8 = object.key("filters").start_array();
+        for item_9 in var_7 {
+            {
+                #[allow(unused_mut)]
+                let mut object_10 = array_8.value().start_object();
+                super::super::protocol_serde::shape_key_values_pair::ser_key_values_pair(&mut object_10, item_9)?;
+                object_10.finish();
+            }
+        }
+        array_8.finish();
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_list_quota_shares.rs`

```diff
--- reference/src/protocol_serde/shape_list_quota_shares.rs
+++ generated/src/protocol_serde/shape_list_quota_shares.rs
@@ -93,6 +93,13 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                "quotaShares" => {
+                    builder = builder.set_quota_shares(super::super::protocol_serde::shape_quota_share_list::de_quota_share_list(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
                 "nextToken" => {
                     builder = builder.set_next_token(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -100,13 +107,6 @@
                             .transpose()?,
                     );
                 }
-                "quotaShares" => {
-                    builder = builder.set_quota_shares(super::super::protocol_serde::shape_quota_share_list::de_quota_share_list(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
-                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_list_scheduling_policies.rs`

```diff
--- reference/src/protocol_serde/shape_list_scheduling_policies.rs
+++ generated/src/protocol_serde/shape_list_scheduling_policies.rs
@@ -103,13 +103,6 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "nextToken" => {
-                    builder = builder.set_next_token(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
-                }
                 "schedulingPolicies" => {
                     builder = builder.set_scheduling_policies(
                         super::super::protocol_serde::shape_scheduling_policy_listing_detail_list::de_scheduling_policy_listing_detail_list(
@@ -119,6 +112,13 @@
                         )?,
                     );
                 }
+                "nextToken" => {
+                    builder = builder.set_next_token(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
+                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_list_service_jobs.rs`

```diff
--- reference/src/protocol_serde/shape_list_service_jobs.rs
+++ generated/src/protocol_serde/shape_list_service_jobs.rs
@@ -63,7 +63,9 @@
         output = super::super::protocol_serde::shape_list_service_jobs::de_list_service_jobs(_response_body, output)
             .map_err(super::super::operation::list_service_jobs::ListServiceJobsError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
-        super::super::serde_util::list_service_jobs_output_output_correct_errors(output).build()
+        super::super::serde_util::list_service_jobs_output_output_correct_errors(output)
+            .build()
+            .map_err(super::super::operation::list_service_jobs::ListServiceJobsError::unhandled)?
     })
 }

```

### `src/protocol_serde/shape_list_service_jobs_input.rs`

```diff
--- reference/src/protocol_serde/shape_list_service_jobs_input.rs
+++ generated/src/protocol_serde/shape_list_service_jobs_input.rs
@@ -3,32 +3,32 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::operation::list_service_jobs::ListServiceJobsInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.filters {
-        let mut array_2 = object.key("filters").start_array();
-        for item_3 in var_1 {
-            {
-                #[allow(unused_mut)]
-                let mut object_4 = array_2.value().start_object();
-                super::super::protocol_serde::shape_key_values_pair::ser_key_values_pair(&mut object_4, item_3)?;
-                object_4.finish();
-            }
-        }
-        array_2.finish();
+    if let Some(var_1) = &input.job_queue {
+        object.key("jobQueue").string(var_1.as_str());
     }
-    if let Some(var_5) = &input.job_queue {
-        object.key("jobQueue").string(var_5.as_str());
+    if let Some(var_2) = &input.job_status {
+        object.key("jobStatus").string(var_2.as_str());
     }
-    if let Some(var_6) = &input.job_status {
-        object.key("jobStatus").string(var_6.as_str());
-    }
-    if let Some(var_7) = &input.max_results {
+    if let Some(var_3) = &input.max_results {
         object.key("maxResults").number(
             #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_7).into()),
+            ::aws_smithy_types::Number::NegInt((*var_3).into()),
         );
     }
-    if let Some(var_8) = &input.next_token {
-        object.key("nextToken").string(var_8.as_str());
+    if let Some(var_4) = &input.next_token {
+        object.key("nextToken").string(var_4.as_str());
+    }
+    if let Some(var_5) = &input.filters {
+        let mut array_6 = object.key("filters").start_array();
+        for item_7 in var_5 {
+            {
+                #[allow(unused_mut)]
+                let mut object_8 = array_6.value().start_object();
+                super::super::protocol_serde::shape_key_values_pair::ser_key_values_pair(&mut object_8, item_7)?;
+                object_8.finish();
+            }
+        }
+        array_6.finish();
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_log_configuration.rs`

```diff
--- reference/src/protocol_serde/shape_log_configuration.rs
+++ generated/src/protocol_serde/shape_log_configuration.rs
@@ -3,30 +3,30 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::LogConfiguration,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.log_driver {
-        object.key("logDriver").string(var_1.as_str());
+    {
+        object.key("logDriver").string(input.log_driver.as_str());
     }
-    if let Some(var_2) = &input.options {
+    if let Some(var_1) = &input.options {
         #[allow(unused_mut)]
-        let mut object_3 = object.key("options").start_object();
-        for (key_4, value_5) in var_2 {
+        let mut object_2 = object.key("options").start_object();
+        for (key_3, value_4) in var_1 {
             {
-                object_3.key(key_4.as_str()).string(value_5.as_str());
+                object_2.key(key_3.as_str()).string(value_4.as_str());
             }
         }
-        object_3.finish();
+        object_2.finish();
     }
-    if let Some(var_6) = &input.secret_options {
-        let mut array_7 = object.key("secretOptions").start_array();
-        for item_8 in var_6 {
+    if let Some(var_5) = &input.secret_options {
+        let mut array_6 = object.key("secretOptions").start_array();
+        for item_7 in var_5 {
             {
                 #[allow(unused_mut)]
-                let mut object_9 = array_7.value().start_object();
-                super::super::protocol_serde::shape_secret::ser_secret(&mut object_9, item_8)?;
-                object_9.finish();
+                let mut object_8 = array_6.value().start_object();
+                super::super::protocol_serde::shape_secret::ser_secret(&mut object_8, item_7)?;
+                object_8.finish();
             }
         }
-        array_7.finish();
+        array_6.finish();
     }
     Ok(())
 }
@@ -82,7 +82,9 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::log_configuration_correct_errors(builder).build()))
+            Ok(Some(super::super::serde_util::log_configuration_correct_errors(builder).build().map_err(
+                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
+            )?))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_managed_instances_network_configuration.rs`

```diff
--- reference/src/protocol_serde/shape_managed_instances_network_configuration.rs
+++ generated/src/protocol_serde/shape_managed_instances_network_configuration.rs
@@ -3,23 +3,23 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::ManagedInstancesNetworkConfiguration,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.subnets {
-        let mut array_2 = object.key("subnets").start_array();
-        for item_3 in var_1 {
+    {
+        let mut array_1 = object.key("subnets").start_array();
+        for item_2 in &input.subnets {
             {
-                array_2.value().string(item_3.as_str());
+                array_1.value().string(item_2.as_str());
             }
         }
-        array_2.finish();
+        array_1.finish();
     }
-    if let Some(var_4) = &input.security_groups {
-        let mut array_5 = object.key("securityGroups").start_array();
-        for item_6 in var_4 {
+    {
+        let mut array_3 = object.key("securityGroups").start_array();
+        for item_4 in &input.security_groups {
             {
-                array_5.value().string(item_6.as_str());
+                array_3.value().string(item_4.as_str());
             }
         }
-        array_5.finish();
+        array_3.finish();
     }
     Ok(())
 }
@@ -63,7 +63,9 @@
                 }
             }
             Ok(Some(
-                super::super::serde_util::managed_instances_network_configuration_correct_errors(builder).build(),
+                super::super::serde_util::managed_instances_network_configuration_correct_errors(builder)
+                    .build()
+                    .map_err(|err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err))?,
             ))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
```

### `src/protocol_serde/shape_managed_instances_provider.rs`

```diff
--- reference/src/protocol_serde/shape_managed_instances_provider.rs
+++ generated/src/protocol_serde/shape_managed_instances_provider.rs
@@ -6,20 +6,20 @@
     if let Some(var_1) = &input.propagate_tags {
         object.key("propagateTags").string(var_1.as_str());
     }
-    if let Some(var_2) = &input.infrastructure_role_arn {
-        object.key("infrastructureRoleArn").string(var_2.as_str());
+    {
+        object.key("infrastructureRoleArn").string(input.infrastructure_role_arn.as_str());
     }
-    if let Some(var_3) = &input.instance_launch_template {
+    if let Some(var_2) = &input.instance_launch_template {
         #[allow(unused_mut)]
-        let mut object_4 = object.key("instanceLaunchTemplate").start_object();
-        super::super::protocol_serde::shape_instance_launch_template::ser_instance_launch_template(&mut object_4, var_3)?;
-        object_4.finish();
+        let mut object_3 = object.key("instanceLaunchTemplate").start_object();
+        super::super::protocol_serde::shape_instance_launch_template::ser_instance_launch_template(&mut object_3, var_2)?;
+        object_3.finish();
     }
-    if let Some(var_5) = &input.infrastructure_optimization {
+    if let Some(var_4) = &input.infrastructure_optimization {
         #[allow(unused_mut)]
-        let mut object_6 = object.key("infrastructureOptimization").start_object();
-        super::super::protocol_serde::shape_infrastructure_optimization::ser_infrastructure_optimization(&mut object_6, var_5)?;
-        object_6.finish();
+        let mut object_5 = object.key("infrastructureOptimization").start_object();
+        super::super::protocol_serde::shape_infrastructure_optimization::ser_infrastructure_optimization(&mut object_5, var_4)?;
+        object_5.finish();
     }
     Ok(())
 }
@@ -79,7 +79,11 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::managed_instances_provider_correct_errors(builder).build()))
+            Ok(Some(
+                super::super::serde_util::managed_instances_provider_correct_errors(builder)
+                    .build()
+                    .map_err(|err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err))?,
+            ))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_managed_instances_storage_configuration.rs`

```diff
--- reference/src/protocol_serde/shape_managed_instances_storage_configuration.rs
+++ generated/src/protocol_serde/shape_managed_instances_storage_configuration.rs
@@ -3,7 +3,7 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::ManagedInstancesStorageConfiguration,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.storage_size_gib {
+    if let Some(var_1) = &input.storage_size_gi_b {
         object.key("storageSizeGiB").number(
             #[allow(clippy::useless_conversion)]
             ::aws_smithy_types::Number::NegInt((*var_1).into()),
@@ -35,7 +35,7 @@
                     Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                     Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                         "storageSizeGiB" => {
-                            builder = builder.set_storage_size_gib(
+                            builder = builder.set_storage_size_gi_b(
                                 ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                     .map(i32::try_from)
                                     .transpose()?,
```

### `src/protocol_serde/shape_node_properties.rs`

```diff
--- reference/src/protocol_serde/shape_node_properties.rs
+++ generated/src/protocol_serde/shape_node_properties.rs
@@ -3,29 +3,29 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::NodeProperties,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.num_nodes {
+    {
         object.key("numNodes").number(
             #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_1).into()),
+            ::aws_smithy_types::Number::NegInt((input.num_nodes).into()),
         );
     }
-    if let Some(var_2) = &input.main_node {
+    {
         object.key("mainNode").number(
             #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_2).into()),
+            ::aws_smithy_types::Number::NegInt((input.main_node).into()),
         );
     }
-    if let Some(var_3) = &input.node_range_properties {
-        let mut array_4 = object.key("nodeRangeProperties").start_array();
-        for item_5 in var_3 {
+    {
+        let mut array_1 = object.key("nodeRangeProperties").start_array();
+        for item_2 in &input.node_range_properties {
             {
                 #[allow(unused_mut)]
-                let mut object_6 = array_4.value().start_object();
-                super::super::protocol_serde::shape_node_range_property::ser_node_range_property(&mut object_6, item_5)?;
-                object_6.finish();
+                let mut object_3 = array_1.value().start_object();
+                super::super::protocol_serde::shape_node_range_property::ser_node_range_property(&mut object_3, item_2)?;
+                object_3.finish();
             }
         }
-        array_4.finish();
+        array_1.finish();
     }
     Ok(())
 }
@@ -80,7 +80,9 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::node_properties_correct_errors(builder).build()))
+            Ok(Some(super::super::serde_util::node_properties_correct_errors(builder).build().map_err(
+                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
+            )?))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_node_property_override.rs`

```diff
--- reference/src/protocol_serde/shape_node_property_override.rs
+++ generated/src/protocol_serde/shape_node_property_override.rs
@@ -3,41 +3,41 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::NodePropertyOverride,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.target_nodes {
-        object.key("targetNodes").string(var_1.as_str());
+    {
+        object.key("targetNodes").string(input.target_nodes.as_str());
     }
-    if let Some(var_2) = &input.container_overrides {
+    if let Some(var_1) = &input.container_overrides {
         #[allow(unused_mut)]
-        let mut object_3 = object.key("containerOverrides").start_object();
-        super::super::protocol_serde::shape_container_overrides::ser_container_overrides(&mut object_3, var_2)?;
-        object_3.finish();
+        let mut object_2 = object.key("containerOverrides").start_object();
+        super::super::protocol_serde::shape_container_overrides::ser_container_overrides(&mut object_2, var_1)?;
+        object_2.finish();
     }
-    if let Some(var_4) = &input.ecs_properties_override {
+    if let Some(var_3) = &input.ecs_properties_override {
         #[allow(unused_mut)]
-        let mut object_5 = object.key("ecsPropertiesOverride").start_object();
-        super::super::protocol_serde::shape_ecs_properties_override::ser_ecs_properties_override(&mut object_5, var_4)?;
-        object_5.finish();
+        let mut object_4 = object.key("ecsPropertiesOverride").start_object();
+        super::super::protocol_serde::shape_ecs_properties_override::ser_ecs_properties_override(&mut object_4, var_3)?;
+        object_4.finish();
     }
-    if let Some(var_6) = &input.instance_types {
-        let mut array_7 = object.key("instanceTypes").start_array();
-        for item_8 in var_6 {
+    if let Some(var_5) = &input.instance_types {
+        let mut array_6 = object.key("instanceTypes").start_array();
+        for item_7 in var_5 {
             {
-                array_7.value().string(item_8.as_str());
+                array_6.value().string(item_7.as_str());
             }
         }
-        array_7.finish();
+        array_6.finish();
     }
-    if let Some(var_9) = &input.eks_properties_override {
+    if let Some(var_8) = &input.eks_properties_override {
         #[allow(unused_mut)]
-        let mut object_10 = object.key("eksPropertiesOverride").start_object();
-        super::super::protocol_serde::shape_eks_properties_override::ser_eks_properties_override(&mut object_10, var_9)?;
-        object_10.finish();
+        let mut object_9 = object.key("eksPropertiesOverride").start_object();
+        super::super::protocol_serde::shape_eks_properties_override::ser_eks_properties_override(&mut object_9, var_8)?;
+        object_9.finish();
     }
-    if let Some(var_11) = &input.consumable_resource_properties_override {
+    if let Some(var_10) = &input.consumable_resource_properties_override {
         #[allow(unused_mut)]
-        let mut object_12 = object.key("consumableResourcePropertiesOverride").start_object();
-        super::super::protocol_serde::shape_consumable_resource_properties::ser_consumable_resource_properties(&mut object_12, var_11)?;
-        object_12.finish();
+        let mut object_11 = object.key("consumableResourcePropertiesOverride").start_object();
+        super::super::protocol_serde::shape_consumable_resource_properties::ser_consumable_resource_properties(&mut object_11, var_10)?;
+        object_11.finish();
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_node_range_property.rs`

```diff
--- reference/src/protocol_serde/shape_node_range_property.rs
+++ generated/src/protocol_serde/shape_node_range_property.rs
@@ -3,41 +3,41 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::NodeRangeProperty,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.target_nodes {
-        object.key("targetNodes").string(var_1.as_str());
+    {
+        object.key("targetNodes").string(input.target_nodes.as_str());
     }
-    if let Some(var_2) = &input.container {
+    if let Some(var_1) = &input.container {
         #[allow(unused_mut)]
-        let mut object_3 = object.key("container").start_object();
-        super::super::protocol_serde::shape_container_properties::ser_container_properties(&mut object_3, var_2)?;
-        object_3.finish();
+        let mut object_2 = object.key("container").start_object();
+        super::super::protocol_serde::shape_container_properties::ser_container_properties(&mut object_2, var_1)?;
+        object_2.finish();
     }
-    if let Some(var_4) = &input.instance_types {
-        let mut array_5 = object.key("instanceTypes").start_array();
-        for item_6 in var_4 {
+    if let Some(var_3) = &input.instance_types {
+        let mut array_4 = object.key("instanceTypes").start_array();
+        for item_5 in var_3 {
             {
-                array_5.value().string(item_6.as_str());
+                array_4.value().string(item_5.as_str());
             }
         }
-        array_5.finish();
+        array_4.finish();
     }
-    if let Some(var_7) = &input.ecs_properties {
+    if let Some(var_6) = &input.ecs_properties {
         #[allow(unused_mut)]
-        let mut object_8 = object.key("ecsProperties").start_object();
-        super::super::protocol_serde::shape_ecs_properties::ser_ecs_properties(&mut object_8, var_7)?;
-        object_8.finish();
+        let mut object_7 = object.key("ecsProperties").start_object();
+        super::super::protocol_serde::shape_ecs_properties::ser_ecs_properties(&mut object_7, var_6)?;
+        object_7.finish();
     }
-    if let Some(var_9) = &input.eks_properties {
+    if let Some(var_8) = &input.eks_properties {
         #[allow(unused_mut)]
-        let mut object_10 = object.key("eksProperties").start_object();
-        super::super::protocol_serde::shape_eks_properties::ser_eks_properties(&mut object_10, var_9)?;
-        object_10.finish();
+        let mut object_9 = object.key("eksProperties").start_object();
+        super::super::protocol_serde::shape_eks_properties::ser_eks_properties(&mut object_9, var_8)?;
+        object_9.finish();
     }
-    if let Some(var_11) = &input.consumable_resource_properties {
+    if let Some(var_10) = &input.consumable_resource_properties {
         #[allow(unused_mut)]
-        let mut object_12 = object.key("consumableResourceProperties").start_object();
-        super::super::protocol_serde::shape_consumable_resource_properties::ser_consumable_resource_properties(&mut object_12, var_11)?;
-        object_12.finish();
+        let mut object_11 = object.key("consumableResourceProperties").start_object();
+        super::super::protocol_serde::shape_consumable_resource_properties::ser_consumable_resource_properties(&mut object_11, var_10)?;
+        object_11.finish();
     }
     Ok(())
 }
@@ -114,7 +114,9 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::node_range_property_correct_errors(builder).build()))
+            Ok(Some(super::super::serde_util::node_range_property_correct_errors(builder).build().map_err(
+                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
+            )?))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_quota_share_capacity_limit.rs`

```diff
--- reference/src/protocol_serde/shape_quota_share_capacity_limit.rs
+++ generated/src/protocol_serde/shape_quota_share_capacity_limit.rs
@@ -3,14 +3,14 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::QuotaShareCapacityLimit,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.max_capacity {
+    {
         object.key("maxCapacity").number(
             #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_1).into()),
+            ::aws_smithy_types::Number::NegInt((input.max_capacity).into()),
         );
     }
-    if let Some(var_2) = &input.capacity_unit {
-        object.key("capacityUnit").string(var_2.as_str());
+    {
+        object.key("capacityUnit").string(input.capacity_unit.as_str());
     }
     Ok(())
 }
@@ -60,7 +60,11 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::quota_share_capacity_limit_correct_errors(builder).build()))
+            Ok(Some(
+                super::super::serde_util::quota_share_capacity_limit_correct_errors(builder)
+                    .build()
+                    .map_err(|err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err))?,
+            ))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_quota_share_policy.rs`

```diff
--- reference/src/protocol_serde/shape_quota_share_policy.rs
+++ generated/src/protocol_serde/shape_quota_share_policy.rs
@@ -3,8 +3,10 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::QuotaSharePolicy,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.idle_resource_assignment_strategy {
-        object.key("idleResourceAssignmentStrategy").string(var_1.as_str());
+    {
+        object
+            .key("idleResourceAssignmentStrategy")
+            .string(input.idle_resource_assignment_strategy.as_str());
     }
     Ok(())
 }
@@ -50,7 +52,9 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::quota_share_policy_correct_errors(builder).build()))
+            Ok(Some(super::super::serde_util::quota_share_policy_correct_errors(builder).build().map_err(
+                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
+            )?))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_quota_share_preemption_configuration.rs`

```diff
--- reference/src/protocol_serde/shape_quota_share_preemption_configuration.rs
+++ generated/src/protocol_serde/shape_quota_share_preemption_configuration.rs
@@ -3,8 +3,8 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::QuotaSharePreemptionConfiguration,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.in_share_preemption {
-        object.key("inSharePreemption").string(var_1.as_str());
+    {
+        object.key("inSharePreemption").string(input.in_share_preemption.as_str());
     }
     Ok(())
 }
@@ -48,7 +48,9 @@
                 }
             }
             Ok(Some(
-                super::super::serde_util::quota_share_preemption_configuration_correct_errors(builder).build(),
+                super::super::serde_util::quota_share_preemption_configuration_correct_errors(builder)
+                    .build()
+                    .map_err(|err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err))?,
             ))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
```

### `src/protocol_serde/shape_quota_share_resource_sharing_configuration.rs`

```diff
--- reference/src/protocol_serde/shape_quota_share_resource_sharing_configuration.rs
+++ generated/src/protocol_serde/shape_quota_share_resource_sharing_configuration.rs
@@ -3,13 +3,13 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::QuotaShareResourceSharingConfiguration,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.strategy {
-        object.key("strategy").string(var_1.as_str());
+    {
+        object.key("strategy").string(input.strategy.as_str());
     }
-    if let Some(var_2) = &input.borrow_limit {
+    if let Some(var_1) = &input.borrow_limit {
         object.key("borrowLimit").number(
             #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_2).into()),
+            ::aws_smithy_types::Number::NegInt((*var_1).into()),
         );
     }
     Ok(())
@@ -64,7 +64,9 @@
                 }
             }
             Ok(Some(
-                super::super::serde_util::quota_share_resource_sharing_configuration_correct_errors(builder).build(),
+                super::super::serde_util::quota_share_resource_sharing_configuration_correct_errors(builder)
+                    .build()
+                    .map_err(|err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err))?,
             ))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
```

### `src/protocol_serde/shape_register_job_definition.rs`

```diff
--- reference/src/protocol_serde/shape_register_job_definition.rs
+++ generated/src/protocol_serde/shape_register_job_definition.rs
@@ -69,7 +69,9 @@
         output = super::super::protocol_serde::shape_register_job_definition::de_register_job_definition(_response_body, output)
             .map_err(super::super::operation::register_job_definition::RegisterJobDefinitionError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
-        super::super::serde_util::register_job_definition_output_output_correct_errors(output).build()
+        super::super::serde_util::register_job_definition_output_output_correct_errors(output)
+            .build()
+            .map_err(super::super::operation::register_job_definition::RegisterJobDefinitionError::unhandled)?
     })
 }

@@ -99,15 +101,15 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "jobDefinitionArn" => {
-                    builder = builder.set_job_definition_arn(
+                "jobDefinitionName" => {
+                    builder = builder.set_job_definition_name(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "jobDefinitionName" => {
-                    builder = builder.set_job_definition_name(
+                "jobDefinitionArn" => {
+                    builder = builder.set_job_definition_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
```

### `src/protocol_serde/shape_register_job_definition_input.rs`

```diff
--- reference/src/protocol_serde/shape_register_job_definition_input.rs
+++ generated/src/protocol_serde/shape_register_job_definition_input.rs
@@ -3,32 +3,33 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::operation::register_job_definition::RegisterJobDefinitionInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.consumable_resource_properties {
-        #[allow(unused_mut)]
-        let mut object_2 = object.key("consumableResourceProperties").start_object();
-        super::super::protocol_serde::shape_consumable_resource_properties::ser_consumable_resource_properties(&mut object_2, var_1)?;
-        object_2.finish();
+    if let Some(var_1) = &input.job_definition_name {
+        object.key("jobDefinitionName").string(var_1.as_str());
     }
-    if let Some(var_3) = &input.container_properties {
+    if let Some(var_2) = &input.r#type {
+        object.key("type").string(var_2.as_str());
+    }
+    if let Some(var_3) = &input.parameters {
         #[allow(unused_mut)]
-        let mut object_4 = object.key("containerProperties").start_object();
-        super::super::protocol_serde::shape_container_properties::ser_container_properties(&mut object_4, var_3)?;
+        let mut object_4 = object.key("parameters").start_object();
+        for (key_5, value_6) in var_3 {
+            {
+                object_4.key(key_5.as_str()).string(value_6.as_str());
+            }
+        }
         object_4.finish();
     }
-    if let Some(var_5) = &input.ecs_properties {
-        #[allow(unused_mut)]
-        let mut object_6 = object.key("ecsProperties").start_object();
-        super::super::protocol_serde::shape_ecs_properties::ser_ecs_properties(&mut object_6, var_5)?;
-        object_6.finish();
+    if let Some(var_7) = &input.scheduling_priority {
+        object.key("schedulingPriority").number(
+            #[allow(clippy::useless_conversion)]
+            ::aws_smithy_types::Number::NegInt((*var_7).into()),
+        );
     }
-    if let Some(var_7) = &input.eks_properties {
+    if let Some(var_8) = &input.container_properties {
         #[allow(unused_mut)]
-        let mut object_8 = object.key("eksProperties").start_object();
-        super::super::protocol_serde::shape_eks_properties::ser_eks_properties(&mut object_8, var_7)?;
-        object_8.finish();
-    }
-    if let Some(var_9) = &input.job_definition_name {
-        object.key("jobDefinitionName").string(var_9.as_str());
+        let mut object_9 = object.key("containerProperties").start_object();
+        super::super::protocol_serde::shape_container_properties::ser_container_properties(&mut object_9, var_8)?;
+        object_9.finish();
     }
     if let Some(var_10) = &input.node_properties {
         #[allow(unused_mut)]
@@ -36,58 +37,57 @@
         super::super::protocol_serde::shape_node_properties::ser_node_properties(&mut object_11, var_10)?;
         object_11.finish();
     }
-    if let Some(var_12) = &input.parameters {
+    if let Some(var_12) = &input.retry_strategy {
+        #[allow(unused_mut)]
+        let mut object_13 = object.key("retryStrategy").start_object();
+        super::super::protocol_serde::shape_retry_strategy::ser_retry_strategy(&mut object_13, var_12)?;
+        object_13.finish();
+    }
+    if let Some(var_14) = &input.propagate_tags {
+        object.key("propagateTags").boolean(*var_14);
+    }
+    if let Some(var_15) = &input.timeout {
+        #[allow(unused_mut)]
+        let mut object_16 = object.key("timeout").start_object();
+        super::super::protocol_serde::shape_job_timeout::ser_job_timeout(&mut object_16, var_15)?;
+        object_16.finish();
+    }
+    if let Some(var_17) = &input.tags {
         #[allow(unused_mut)]
-        let mut object_13 = object.key("parameters").start_object();
-        for (key_14, value_15) in var_12 {
+        let mut object_18 = object.key("tags").start_object();
+        for (key_19, value_20) in var_17 {
             {
-                object_13.key(key_14.as_str()).string(value_15.as_str());
+                object_18.key(key_19.as_str()).string(value_20.as_str());
             }
         }
-        object_13.finish();
+        object_18.finish();
     }
-    if let Some(var_16) = &input.platform_capabilities {
-        let mut array_17 = object.key("platformCapabilities").start_array();
-        for item_18 in var_16 {
+    if let Some(var_21) = &input.platform_capabilities {
+        let mut array_22 = object.key("platformCapabilities").start_array();
+        for item_23 in var_21 {
             {
-                array_17.value().string(item_18.as_str());
+                array_22.value().string(item_23.as_str());
             }
         }
-        array_17.finish();
-    }
-    if let Some(var_19) = &input.propagate_tags {
-        object.key("propagateTags").boolean(*var_19);
+        array_22.finish();
     }
-    if let Some(var_20) = &input.retry_strategy {
+    if let Some(var_24) = &input.eks_properties {
         #[allow(unused_mut)]
-        let mut object_21 = object.key("retryStrategy").start_object();
-        super::super::protocol_serde::shape_retry_strategy::ser_retry_strategy(&mut object_21, var_20)?;
-        object_21.finish();
+        let mut object_25 = object.key("eksProperties").start_object();
+        super::super::protocol_serde::shape_eks_properties::ser_eks_properties(&mut object_25, var_24)?;
+        object_25.finish();
     }
-    if let Some(var_22) = &input.scheduling_priority {
-        object.key("schedulingPriority").number(
-            #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_22).into()),
-        );
-    }
-    if let Some(var_23) = &input.tags {
+    if let Some(var_26) = &input.ecs_properties {
         #[allow(unused_mut)]
-        let mut object_24 = object.key("tags").start_object();
-        for (key_25, value_26) in var_23 {
-            {
-                object_24.key(key_25.as_str()).string(value_26.as_str());
-            }
-        }
-        object_24.finish();
+        let mut object_27 = object.key("ecsProperties").start_object();
+        super::super::protocol_serde::shape_ecs_properties::ser_ecs_properties(&mut object_27, var_26)?;
+        object_27.finish();
     }
-    if let Some(var_27) = &input.timeout {
+    if let Some(var_28) = &input.consumable_resource_properties {
         #[allow(unused_mut)]
-        let mut object_28 = object.key("timeout").start_object();
-        super::super::protocol_serde::shape_job_timeout::ser_job_timeout(&mut object_28, var_27)?;
-        object_28.finish();
-    }
-    if let Some(var_29) = &input.r#type {
-        object.key("type").string(var_29.as_str());
+        let mut object_29 = object.key("consumableResourceProperties").start_object();
+        super::super::protocol_serde::shape_consumable_resource_properties::ser_consumable_resource_properties(&mut object_29, var_28)?;
+        object_29.finish();
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_repository_credentials.rs`

```diff
--- reference/src/protocol_serde/shape_repository_credentials.rs
+++ generated/src/protocol_serde/shape_repository_credentials.rs
@@ -3,8 +3,8 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::RepositoryCredentials,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.credentials_parameter {
-        object.key("credentialsParameter").string(var_1.as_str());
+    {
+        object.key("credentialsParameter").string(input.credentials_parameter.as_str());
     }
     Ok(())
 }
@@ -47,7 +47,9 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::repository_credentials_correct_errors(builder).build()))
+            Ok(Some(super::super::serde_util::repository_credentials_correct_errors(builder).build().map_err(
+                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
+            )?))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_resource_requirement.rs`

```diff
--- reference/src/protocol_serde/shape_resource_requirement.rs
+++ generated/src/protocol_serde/shape_resource_requirement.rs
@@ -3,11 +3,11 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::ResourceRequirement,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.value {
-        object.key("value").string(var_1.as_str());
+    {
+        object.key("value").string(input.value.as_str());
     }
-    if let Some(var_2) = &input.r#type {
-        object.key("type").string(var_2.as_str());
+    {
+        object.key("type").string(input.r#type.as_str());
     }
     Ok(())
 }
@@ -57,7 +57,9 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::resource_requirement_correct_errors(builder).build()))
+            Ok(Some(super::super::serde_util::resource_requirement_correct_errors(builder).build().map_err(
+                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
+            )?))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_s3_files_volume_configuration.rs`

```diff
--- reference/src/protocol_serde/shape_s3_files_volume_configuration.rs
+++ generated/src/protocol_serde/shape_s3_files_volume_configuration.rs
@@ -3,20 +3,20 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::S3FilesVolumeConfiguration,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.file_system_arn {
-        object.key("fileSystemArn").string(var_1.as_str());
+    {
+        object.key("fileSystemArn").string(input.file_system_arn.as_str());
     }
-    if let Some(var_2) = &input.root_directory {
-        object.key("rootDirectory").string(var_2.as_str());
+    if let Some(var_1) = &input.root_directory {
+        object.key("rootDirectory").string(var_1.as_str());
     }
-    if let Some(var_3) = &input.transit_encryption_port {
+    if let Some(var_2) = &input.transit_encryption_port {
         object.key("transitEncryptionPort").number(
             #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_3).into()),
+            ::aws_smithy_types::Number::NegInt((*var_2).into()),
         );
     }
-    if let Some(var_4) = &input.access_point_arn {
-        object.key("accessPointArn").string(var_4.as_str());
+    if let Some(var_3) = &input.access_point_arn {
+        object.key("accessPointArn").string(var_3.as_str());
     }
     Ok(())
 }
@@ -80,7 +80,11 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::s3_files_volume_configuration_correct_errors(builder).build()))
+            Ok(Some(
+                super::super::serde_util::s3_files_volume_configuration_correct_errors(builder)
+                    .build()
+                    .map_err(|err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err))?,
+            ))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_scheduling_policy_detail.rs`

```diff
--- reference/src/protocol_serde/shape_scheduling_policy_detail.rs
+++ generated/src/protocol_serde/shape_scheduling_policy_detail.rs
@@ -65,7 +65,11 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::scheduling_policy_detail_correct_errors(builder).build()))
+            Ok(Some(
+                super::super::serde_util::scheduling_policy_detail_correct_errors(builder)
+                    .build()
+                    .map_err(|err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err))?,
+            ))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_scheduling_policy_listing_detail.rs`

```diff
--- reference/src/protocol_serde/shape_scheduling_policy_listing_detail.rs
+++ generated/src/protocol_serde/shape_scheduling_policy_listing_detail.rs
@@ -37,7 +37,11 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::scheduling_policy_listing_detail_correct_errors(builder).build()))
+            Ok(Some(
+                super::super::serde_util::scheduling_policy_listing_detail_correct_errors(builder)
+                    .build()
+                    .map_err(|err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err))?,
+            ))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_secret.rs`

```diff
--- reference/src/protocol_serde/shape_secret.rs
+++ generated/src/protocol_serde/shape_secret.rs
@@ -3,11 +3,11 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::Secret,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.name {
-        object.key("name").string(var_1.as_str());
+    {
+        object.key("name").string(input.name.as_str());
     }
-    if let Some(var_2) = &input.value_from {
-        object.key("valueFrom").string(var_2.as_str());
+    {
+        object.key("valueFrom").string(input.value_from.as_str());
     }
     Ok(())
 }
@@ -57,7 +57,9 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::secret_correct_errors(builder).build()))
+            Ok(Some(super::super::serde_util::secret_correct_errors(builder).build().map_err(|err| {
+                ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err)
+            })?))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_service_environment_detail.rs`

```diff
--- reference/src/protocol_serde/shape_service_environment_detail.rs
+++ generated/src/protocol_serde/shape_service_environment_detail.rs
@@ -79,7 +79,11 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::service_environment_detail_correct_errors(builder).build()))
+            Ok(Some(
+                super::super::serde_util::service_environment_detail_correct_errors(builder)
+                    .build()
+                    .map_err(|err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err))?,
+            ))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_service_environment_order.rs`

```diff
--- reference/src/protocol_serde/shape_service_environment_order.rs
+++ generated/src/protocol_serde/shape_service_environment_order.rs
@@ -3,14 +3,14 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::ServiceEnvironmentOrder,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.order {
+    {
         object.key("order").number(
             #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_1).into()),
+            ::aws_smithy_types::Number::NegInt((input.order).into()),
         );
     }
-    if let Some(var_2) = &input.service_environment {
-        object.key("serviceEnvironment").string(var_2.as_str());
+    {
+        object.key("serviceEnvironment").string(input.service_environment.as_str());
     }
     Ok(())
 }
@@ -60,7 +60,11 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::service_environment_order_correct_errors(builder).build()))
+            Ok(Some(
+                super::super::serde_util::service_environment_order_correct_errors(builder)
+                    .build()
+                    .map_err(|err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err))?,
+            ))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_service_job_preemption_configuration.rs`

```diff
--- reference/src/protocol_serde/shape_service_job_preemption_configuration.rs
+++ generated/src/protocol_serde/shape_service_job_preemption_configuration.rs
@@ -1,4 +1,17 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_service_job_preemption_configuration(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::super::types::ServiceJobPreemptionConfiguration,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    if let Some(var_1) = &input.preemption_retries_before_termination {
+        object.key("preemptionRetriesBeforeTermination").number(
+            #[allow(clippy::useless_conversion)]
+            ::aws_smithy_types::Number::NegInt((*var_1).into()),
+        );
+    }
+    Ok(())
+}
+
 pub(crate) fn de_service_job_preemption_configuration<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
@@ -44,16 +57,3 @@
         )),
     }
 }
-
-pub fn ser_service_job_preemption_configuration(
-    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
-    input: &super::super::types::ServiceJobPreemptionConfiguration,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.preemption_retries_before_termination {
-        object.key("preemptionRetriesBeforeTermination").number(
-            #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_1).into()),
-        );
-    }
-    Ok(())
-}
```

### `src/protocol_serde/shape_service_job_retry_strategy.rs`

```diff
--- reference/src/protocol_serde/shape_service_job_retry_strategy.rs
+++ generated/src/protocol_serde/shape_service_job_retry_strategy.rs
@@ -1,4 +1,29 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_service_job_retry_strategy(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::super::types::ServiceJobRetryStrategy,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    {
+        object.key("attempts").number(
+            #[allow(clippy::useless_conversion)]
+            ::aws_smithy_types::Number::NegInt((input.attempts).into()),
+        );
+    }
+    if let Some(var_1) = &input.evaluate_on_exit {
+        let mut array_2 = object.key("evaluateOnExit").start_array();
+        for item_3 in var_1 {
+            {
+                #[allow(unused_mut)]
+                let mut object_4 = array_2.value().start_object();
+                super::super::protocol_serde::shape_service_job_evaluate_on_exit::ser_service_job_evaluate_on_exit(&mut object_4, item_3)?;
+                object_4.finish();
+            }
+        }
+        array_2.finish();
+    }
+    Ok(())
+}
+
 pub(crate) fn de_service_job_retry_strategy<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
@@ -46,7 +71,11 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::service_job_retry_strategy_correct_errors(builder).build()))
+            Ok(Some(
+                super::super::serde_util::service_job_retry_strategy_correct_errors(builder)
+                    .build()
+                    .map_err(|err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err))?,
+            ))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
@@ -53,28 +82,3 @@
         )),
     }
 }
-
-pub fn ser_service_job_retry_strategy(
-    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
-    input: &super::super::types::ServiceJobRetryStrategy,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.attempts {
-        object.key("attempts").number(
-            #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_1).into()),
-        );
-    }
-    if let Some(var_2) = &input.evaluate_on_exit {
-        let mut array_3 = object.key("evaluateOnExit").start_array();
-        for item_4 in var_2 {
-            {
-                #[allow(unused_mut)]
-                let mut object_5 = array_3.value().start_object();
-                super::super::protocol_serde::shape_service_job_evaluate_on_exit::ser_service_job_evaluate_on_exit(&mut object_5, item_4)?;
-                object_5.finish();
-            }
-        }
-        array_3.finish();
-    }
-    Ok(())
-}
```

### `src/protocol_serde/shape_service_job_summary.rs`

```diff
--- reference/src/protocol_serde/shape_service_job_summary.rs
+++ generated/src/protocol_serde/shape_service_job_summary.rs
@@ -128,7 +128,9 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::service_job_summary_correct_errors(builder).build()))
+            Ok(Some(super::super::serde_util::service_job_summary_correct_errors(builder).build().map_err(
+                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
+            )?))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_service_job_timeout.rs`

```diff
--- reference/src/protocol_serde/shape_service_job_timeout.rs
+++ generated/src/protocol_serde/shape_service_job_timeout.rs
@@ -1,4 +1,17 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_service_job_timeout(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::super::types::ServiceJobTimeout,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    if let Some(var_1) = &input.attempt_duration_seconds {
+        object.key("attemptDurationSeconds").number(
+            #[allow(clippy::useless_conversion)]
+            ::aws_smithy_types::Number::NegInt((*var_1).into()),
+        );
+    }
+    Ok(())
+}
+
 pub(crate) fn de_service_job_timeout<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
@@ -44,16 +57,3 @@
         )),
     }
 }
-
-pub fn ser_service_job_timeout(
-    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
-    input: &super::super::types::ServiceJobTimeout,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.attempt_duration_seconds {
-        object.key("attemptDurationSeconds").number(
-            #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_1).into()),
-        );
-    }
-    Ok(())
-}
```

### `src/protocol_serde/shape_service_resource_id.rs`

```diff
--- reference/src/protocol_serde/shape_service_resource_id.rs
+++ generated/src/protocol_serde/shape_service_resource_id.rs
@@ -44,7 +44,9 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::service_resource_id_correct_errors(builder).build()))
+            Ok(Some(super::super::serde_util::service_resource_id_correct_errors(builder).build().map_err(
+                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
+            )?))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_share_attributes.rs`

```diff
--- reference/src/protocol_serde/shape_share_attributes.rs
+++ generated/src/protocol_serde/shape_share_attributes.rs
@@ -3,13 +3,13 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::ShareAttributes,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.share_identifier {
-        object.key("shareIdentifier").string(var_1.as_str());
+    {
+        object.key("shareIdentifier").string(input.share_identifier.as_str());
     }
-    if let Some(var_2) = &input.weight_factor {
+    if let Some(var_1) = &input.weight_factor {
         object.key("weightFactor").number(
             #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::Float((*var_2).into()),
+            ::aws_smithy_types::Number::Float((*var_1).into()),
         );
     }
     Ok(())
@@ -58,7 +58,9 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::share_attributes_correct_errors(builder).build()))
+            Ok(Some(super::super::serde_util::share_attributes_correct_errors(builder).build().map_err(
+                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
+            )?))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_submit_job.rs`

```diff
--- reference/src/protocol_serde/shape_submit_job.rs
+++ generated/src/protocol_serde/shape_submit_job.rs
@@ -63,7 +63,9 @@
         output = super::super::protocol_serde::shape_submit_job::de_submit_job(_response_body, output)
             .map_err(super::super::operation::submit_job::SubmitJobError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
-        super::super::serde_util::submit_job_output_output_correct_errors(output).build()
+        super::super::serde_util::submit_job_output_output_correct_errors(output)
+            .build()
+            .map_err(super::super::operation::submit_job::SubmitJobError::unhandled)?
     })
 }

@@ -97,15 +99,15 @@
                             .transpose()?,
                     );
                 }
-                "jobId" => {
-                    builder = builder.set_job_id(
+                "jobName" => {
+                    builder = builder.set_job_name(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "jobName" => {
-                    builder = builder.set_job_name(
+                "jobId" => {
+                    builder = builder.set_job_id(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
```

### `src/protocol_serde/shape_submit_job_input.rs`

```diff
--- reference/src/protocol_serde/shape_submit_job_input.rs
+++ generated/src/protocol_serde/shape_submit_job_input.rs
@@ -3,22 +3,25 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::operation::submit_job::SubmitJobInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.array_properties {
-        #[allow(unused_mut)]
-        let mut object_2 = object.key("arrayProperties").start_object();
-        super::super::protocol_serde::shape_array_properties::ser_array_properties(&mut object_2, var_1)?;
-        object_2.finish();
+    if let Some(var_1) = &input.job_name {
+        object.key("jobName").string(var_1.as_str());
     }
-    if let Some(var_3) = &input.consumable_resource_properties_override {
-        #[allow(unused_mut)]
-        let mut object_4 = object.key("consumableResourcePropertiesOverride").start_object();
-        super::super::protocol_serde::shape_consumable_resource_properties::ser_consumable_resource_properties(&mut object_4, var_3)?;
-        object_4.finish();
+    if let Some(var_2) = &input.job_queue {
+        object.key("jobQueue").string(var_2.as_str());
+    }
+    if let Some(var_3) = &input.share_identifier {
+        object.key("shareIdentifier").string(var_3.as_str());
+    }
+    if let Some(var_4) = &input.scheduling_priority_override {
+        object.key("schedulingPriorityOverride").number(
+            #[allow(clippy::useless_conversion)]
+            ::aws_smithy_types::Number::NegInt((*var_4).into()),
+        );
     }
-    if let Some(var_5) = &input.container_overrides {
+    if let Some(var_5) = &input.array_properties {
         #[allow(unused_mut)]
-        let mut object_6 = object.key("containerOverrides").start_object();
-        super::super::protocol_serde::shape_container_overrides::ser_container_overrides(&mut object_6, var_5)?;
+        let mut object_6 = object.key("arrayProperties").start_object();
+        super::super::protocol_serde::shape_array_properties::ser_array_properties(&mut object_6, var_5)?;
         object_6.finish();
     }
     if let Some(var_7) = &input.depends_on {
@@ -33,26 +36,24 @@
         }
         array_8.finish();
     }
-    if let Some(var_11) = &input.ecs_properties_override {
-        #[allow(unused_mut)]
-        let mut object_12 = object.key("ecsPropertiesOverride").start_object();
-        super::super::protocol_serde::shape_ecs_properties_override::ser_ecs_properties_override(&mut object_12, var_11)?;
-        object_12.finish();
+    if let Some(var_11) = &input.job_definition {
+        object.key("jobDefinition").string(var_11.as_str());
     }
-    if let Some(var_13) = &input.eks_properties_override {
+    if let Some(var_12) = &input.parameters {
         #[allow(unused_mut)]
-        let mut object_14 = object.key("eksPropertiesOverride").start_object();
-        super::super::protocol_serde::shape_eks_properties_override::ser_eks_properties_override(&mut object_14, var_13)?;
-        object_14.finish();
-    }
-    if let Some(var_15) = &input.job_definition {
-        object.key("jobDefinition").string(var_15.as_str());
-    }
-    if let Some(var_16) = &input.job_name {
-        object.key("jobName").string(var_16.as_str());
+        let mut object_13 = object.key("parameters").start_object();
+        for (key_14, value_15) in var_12 {
+            {
+                object_13.key(key_14.as_str()).string(value_15.as_str());
+            }
+        }
+        object_13.finish();
     }
-    if let Some(var_17) = &input.job_queue {
-        object.key("jobQueue").string(var_17.as_str());
+    if let Some(var_16) = &input.container_overrides {
+        #[allow(unused_mut)]
+        let mut object_17 = object.key("containerOverrides").start_object();
+        super::super::protocol_serde::shape_container_overrides::ser_container_overrides(&mut object_17, var_16)?;
+        object_17.finish();
     }
     if let Some(var_18) = &input.node_overrides {
         #[allow(unused_mut)]
@@ -60,48 +61,47 @@
         super::super::protocol_serde::shape_node_overrides::ser_node_overrides(&mut object_19, var_18)?;
         object_19.finish();
     }
-    if let Some(var_20) = &input.parameters {
+    if let Some(var_20) = &input.retry_strategy {
         #[allow(unused_mut)]
-        let mut object_21 = object.key("parameters").start_object();
-        for (key_22, value_23) in var_20 {
-            {
-                object_21.key(key_22.as_str()).string(value_23.as_str());
-            }
-        }
+        let mut object_21 = object.key("retryStrategy").start_object();
+        super::super::protocol_serde::shape_retry_strategy::ser_retry_strategy(&mut object_21, var_20)?;
         object_21.finish();
     }
-    if let Some(var_24) = &input.propagate_tags {
-        object.key("propagateTags").boolean(*var_24);
+    if let Some(var_22) = &input.propagate_tags {
+        object.key("propagateTags").boolean(*var_22);
     }
-    if let Some(var_25) = &input.retry_strategy {
+    if let Some(var_23) = &input.timeout {
         #[allow(unused_mut)]
-        let mut object_26 = object.key("retryStrategy").start_object();
-        super::super::protocol_serde::shape_retry_strategy::ser_retry_strategy(&mut object_26, var_25)?;
-        object_26.finish();
-    }
-    if let Some(var_27) = &input.scheduling_priority_override {
-        object.key("schedulingPriorityOverride").number(
-            #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_27).into()),
-        );
-    }
-    if let Some(var_28) = &input.share_identifier {
-        object.key("shareIdentifier").string(var_28.as_str());
+        let mut object_24 = object.key("timeout").start_object();
+        super::super::protocol_serde::shape_job_timeout::ser_job_timeout(&mut object_24, var_23)?;
+        object_24.finish();
     }
-    if let Some(var_29) = &input.tags {
+    if let Some(var_25) = &input.tags {
         #[allow(unused_mut)]
-        let mut object_30 = object.key("tags").start_object();
-        for (key_31, value_32) in var_29 {
+        let mut object_26 = object.key("tags").start_object();
+        for (key_27, value_28) in var_25 {
             {
-                object_30.key(key_31.as_str()).string(value_32.as_str());
+                object_26.key(key_27.as_str()).string(value_28.as_str());
             }
         }
+        object_26.finish();
+    }
+    if let Some(var_29) = &input.eks_properties_override {
+        #[allow(unused_mut)]
+        let mut object_30 = object.key("eksPropertiesOverride").start_object();
+        super::super::protocol_serde::shape_eks_properties_override::ser_eks_properties_override(&mut object_30, var_29)?;
         object_30.finish();
     }
-    if let Some(var_33) = &input.timeout {
+    if let Some(var_31) = &input.ecs_properties_override {
+        #[allow(unused_mut)]
+        let mut object_32 = object.key("ecsPropertiesOverride").start_object();
+        super::super::protocol_serde::shape_ecs_properties_override::ser_ecs_properties_override(&mut object_32, var_31)?;
+        object_32.finish();
+    }
+    if let Some(var_33) = &input.consumable_resource_properties_override {
         #[allow(unused_mut)]
-        let mut object_34 = object.key("timeout").start_object();
-        super::super::protocol_serde::shape_job_timeout::ser_job_timeout(&mut object_34, var_33)?;
+        let mut object_34 = object.key("consumableResourcePropertiesOverride").start_object();
+        super::super::protocol_serde::shape_consumable_resource_properties::ser_consumable_resource_properties(&mut object_34, var_33)?;
         object_34.finish();
     }
     Ok(())
```

### `src/protocol_serde/shape_submit_service_job.rs`

```diff
--- reference/src/protocol_serde/shape_submit_service_job.rs
+++ generated/src/protocol_serde/shape_submit_service_job.rs
@@ -63,7 +63,9 @@
         output = super::super::protocol_serde::shape_submit_service_job::de_submit_service_job(_response_body, output)
             .map_err(super::super::operation::submit_service_job::SubmitServiceJobError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
-        super::super::serde_util::submit_service_job_output_output_correct_errors(output).build()
+        super::super::serde_util::submit_service_job_output_output_correct_errors(output)
+            .build()
+            .map_err(super::super::operation::submit_service_job::SubmitServiceJobError::unhandled)?
     })
 }

@@ -100,15 +102,15 @@
                             .transpose()?,
                     );
                 }
-                "jobId" => {
-                    builder = builder.set_job_id(
+                "jobName" => {
+                    builder = builder.set_job_name(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "jobName" => {
-                    builder = builder.set_job_name(
+                "jobId" => {
+                    builder = builder.set_job_id(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
```

### `src/protocol_serde/shape_submit_service_job_input.rs`

```diff
--- reference/src/protocol_serde/shape_submit_service_job_input.rs
+++ generated/src/protocol_serde/shape_submit_service_job_input.rs
@@ -3,60 +3,60 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::operation::submit_service_job::SubmitServiceJobInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.client_token {
-        object.key("clientToken").string(var_1.as_str());
+    if let Some(var_1) = &input.job_name {
+        object.key("jobName").string(var_1.as_str());
     }
-    if let Some(var_2) = &input.job_name {
-        object.key("jobName").string(var_2.as_str());
-    }
-    if let Some(var_3) = &input.job_queue {
-        object.key("jobQueue").string(var_3.as_str());
+    if let Some(var_2) = &input.job_queue {
+        object.key("jobQueue").string(var_2.as_str());
     }
-    if let Some(var_4) = &input.preemption_configuration {
+    if let Some(var_3) = &input.retry_strategy {
         #[allow(unused_mut)]
-        let mut object_5 = object.key("preemptionConfiguration").start_object();
-        super::super::protocol_serde::shape_service_job_preemption_configuration::ser_service_job_preemption_configuration(&mut object_5, var_4)?;
-        object_5.finish();
+        let mut object_4 = object.key("retryStrategy").start_object();
+        super::super::protocol_serde::shape_service_job_retry_strategy::ser_service_job_retry_strategy(&mut object_4, var_3)?;
+        object_4.finish();
     }
-    if let Some(var_6) = &input.quota_share_name {
-        object.key("quotaShareName").string(var_6.as_str());
-    }
-    if let Some(var_7) = &input.retry_strategy {
-        #[allow(unused_mut)]
-        let mut object_8 = object.key("retryStrategy").start_object();
-        super::super::protocol_serde::shape_service_job_retry_strategy::ser_service_job_retry_strategy(&mut object_8, var_7)?;
-        object_8.finish();
-    }
-    if let Some(var_9) = &input.scheduling_priority {
+    if let Some(var_5) = &input.scheduling_priority {
         object.key("schedulingPriority").number(
             #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_9).into()),
+            ::aws_smithy_types::Number::NegInt((*var_5).into()),
         );
     }
-    if let Some(var_10) = &input.service_job_type {
-        object.key("serviceJobType").string(var_10.as_str());
+    if let Some(var_6) = &input.service_request_payload {
+        object.key("serviceRequestPayload").string(var_6.as_str());
+    }
+    if let Some(var_7) = &input.service_job_type {
+        object.key("serviceJobType").string(var_7.as_str());
+    }
+    if let Some(var_8) = &input.share_identifier {
+        object.key("shareIdentifier").string(var_8.as_str());
     }
-    if let Some(var_11) = &input.service_request_payload {
-        object.key("serviceRequestPayload").string(var_11.as_str());
+    if let Some(var_9) = &input.quota_share_name {
+        object.key("quotaShareName").string(var_9.as_str());
     }
-    if let Some(var_12) = &input.share_identifier {
-        object.key("shareIdentifier").string(var_12.as_str());
+    if let Some(var_10) = &input.preemption_configuration {
+        #[allow(unused_mut)]
+        let mut object_11 = object.key("preemptionConfiguration").start_object();
+        super::super::protocol_serde::shape_service_job_preemption_configuration::ser_service_job_preemption_configuration(&mut object_11, var_10)?;
+        object_11.finish();
+    }
+    if let Some(var_12) = &input.timeout_config {
+        #[allow(unused_mut)]
+        let mut object_13 = object.key("timeoutConfig").start_object();
+        super::super::protocol_serde::shape_service_job_timeout::ser_service_job_timeout(&mut object_13, var_12)?;
+        object_13.finish();
     }
-    if let Some(var_13) = &input.tags {
+    if let Some(var_14) = &input.tags {
         #[allow(unused_mut)]
-        let mut object_14 = object.key("tags").start_object();
-        for (key_15, value_16) in var_13 {
+        let mut object_15 = object.key("tags").start_object();
+        for (key_16, value_17) in var_14 {
             {
-                object_14.key(key_15.as_str()).string(value_16.as_str());
+                object_15.key(key_16.as_str()).string(value_17.as_str());
             }
         }
-        object_14.finish();
+        object_15.finish();
     }
-    if let Some(var_17) = &input.timeout_config {
-        #[allow(unused_mut)]
-        let mut object_18 = object.key("timeoutConfig").start_object();
-        super::super::protocol_serde::shape_service_job_timeout::ser_service_job_timeout(&mut object_18, var_17)?;
-        object_18.finish();
+    if let Some(var_18) = &input.client_token {
+        object.key("clientToken").string(var_18.as_str());
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_task_container_properties.rs`

```diff
--- reference/src/protocol_serde/shape_task_container_properties.rs
+++ generated/src/protocol_serde/shape_task_container_properties.rs
@@ -45,97 +45,97 @@
         super::super::protocol_serde::shape_firelens_configuration::ser_firelens_configuration(&mut object_14, var_13)?;
         object_14.finish();
     }
-    if let Some(var_15) = &input.image {
-        object.key("image").string(var_15.as_str());
+    {
+        object.key("image").string(input.image.as_str());
     }
-    if let Some(var_16) = &input.linux_parameters {
+    if let Some(var_15) = &input.linux_parameters {
         #[allow(unused_mut)]
-        let mut object_17 = object.key("linuxParameters").start_object();
-        super::super::protocol_serde::shape_linux_parameters::ser_linux_parameters(&mut object_17, var_16)?;
-        object_17.finish();
+        let mut object_16 = object.key("linuxParameters").start_object();
+        super::super::protocol_serde::shape_linux_parameters::ser_linux_parameters(&mut object_16, var_15)?;
+        object_16.finish();
     }
-    if let Some(var_18) = &input.log_configuration {
+    if let Some(var_17) = &input.log_configuration {
         #[allow(unused_mut)]
-        let mut object_19 = object.key("logConfiguration").start_object();
-        super::super::protocol_serde::shape_log_configuration::ser_log_configuration(&mut object_19, var_18)?;
-        object_19.finish();
+        let mut object_18 = object.key("logConfiguration").start_object();
+        super::super::protocol_serde::shape_log_configuration::ser_log_configuration(&mut object_18, var_17)?;
+        object_18.finish();
     }
-    if let Some(var_20) = &input.mount_points {
-        let mut array_21 = object.key("mountPoints").start_array();
-        for item_22 in var_20 {
+    if let Some(var_19) = &input.mount_points {
+        let mut array_20 = object.key("mountPoints").start_array();
+        for item_21 in var_19 {
             {
                 #[allow(unused_mut)]
-                let mut object_23 = array_21.value().start_object();
-                super::super::protocol_serde::shape_mount_point::ser_mount_point(&mut object_23, item_22)?;
-                object_23.finish();
+                let mut object_22 = array_20.value().start_object();
+                super::super::protocol_serde::shape_mount_point::ser_mount_point(&mut object_22, item_21)?;
+                object_22.finish();
             }
         }
-        array_21.finish();
+        array_20.finish();
     }
-    if let Some(var_24) = &input.name {
-        object.key("name").string(var_24.as_str());
+    if let Some(var_23) = &input.name {
+        object.key("name").string(var_23.as_str());
     }
-    if let Some(var_25) = &input.privileged {
-        object.key("privileged").boolean(*var_25);
+    if let Some(var_24) = &input.privileged {
+        object.key("privileged").boolean(*var_24);
     }
-    if let Some(var_26) = &input.readonly_root_filesystem {
-        object.key("readonlyRootFilesystem").boolean(*var_26);
+    if let Some(var_25) = &input.readonly_root_filesystem {
+        object.key("readonlyRootFilesystem").boolean(*var_25);
     }
-    if let Some(var_27) = &input.repository_credentials {
+    if let Some(var_26) = &input.repository_credentials {
         #[allow(unused_mut)]
-        let mut object_28 = object.key("repositoryCredentials").start_object();
-        super::super::protocol_serde::shape_repository_credentials::ser_repository_credentials(&mut object_28, var_27)?;
-        object_28.finish();
+        let mut object_27 = object.key("repositoryCredentials").start_object();
+        super::super::protocol_serde::shape_repository_credentials::ser_repository_credentials(&mut object_27, var_26)?;
+        object_27.finish();
     }
-    if let Some(var_29) = &input.resource_requirements {
-        let mut array_30 = object.key("resourceRequirements").start_array();
-        for item_31 in var_29 {
+    if let Some(var_28) = &input.resource_requirements {
+        let mut array_29 = object.key("resourceRequirements").start_array();
+        for item_30 in var_28 {
             {
                 #[allow(unused_mut)]
-                let mut object_32 = array_30.value().start_object();
-                super::super::protocol_serde::shape_resource_requirement::ser_resource_requirement(&mut object_32, item_31)?;
-                object_32.finish();
+                let mut object_31 = array_29.value().start_object();
+                super::super::protocol_serde::shape_resource_requirement::ser_resource_requirement(&mut object_31, item_30)?;
+                object_31.finish();
             }
         }
-        array_30.finish();
+        array_29.finish();
     }
-    if let Some(var_33) = &input.secrets {
-        let mut array_34 = object.key("secrets").start_array();
-        for item_35 in var_33 {
+    if let Some(var_32) = &input.secrets {
+        let mut array_33 = object.key("secrets").start_array();
+        for item_34 in var_32 {
             {
                 #[allow(unused_mut)]
-                let mut object_36 = array_34.value().start_object();
-                super::super::protocol_serde::shape_secret::ser_secret(&mut object_36, item_35)?;
-                object_36.finish();
+                let mut object_35 = array_33.value().start_object();
+                super::super::protocol_serde::shape_secret::ser_secret(&mut object_35, item_34)?;
+                object_35.finish();
             }
         }
-        array_34.finish();
+        array_33.finish();
     }
-    if let Some(var_37) = &input.ulimits {
-        let mut array_38 = object.key("ulimits").start_array();
-        for item_39 in var_37 {
+    if let Some(var_36) = &input.ulimits {
+        let mut array_37 = object.key("ulimits").start_array();
+        for item_38 in var_36 {
             {
                 #[allow(unused_mut)]
-                let mut object_40 = array_38.value().start_object();
-                super::super::protocol_serde::shape_ulimit::ser_ulimit(&mut object_40, item_39)?;
-                object_40.finish();
+                let mut object_39 = array_37.value().start_object();
+                super::super::protocol_serde::shape_ulimit::ser_ulimit(&mut object_39, item_38)?;
+                object_39.finish();
             }
         }
-        array_38.finish();
+        array_37.finish();
     }
-    if let Some(var_41) = &input.user {
-        object.key("user").string(var_41.as_str());
+    if let Some(var_40) = &input.user {
+        object.key("user").string(var_40.as_str());
     }
-    if let Some(var_42) = &input.start_timeout {
+    if let Some(var_41) = &input.start_timeout {
         object.key("startTimeout").number(
             #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_42).into()),
+            ::aws_smithy_types::Number::NegInt((*var_41).into()),
         );
     }
-    if let Some(var_43) = &input.stop_timeout {
+    if let Some(var_42) = &input.stop_timeout {
         object.key("stopTimeout").number(
             #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_43).into()),
+            ::aws_smithy_types::Number::NegInt((*var_42).into()),
         );
     }
     Ok(())
@@ -275,7 +275,11 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::task_container_properties_correct_errors(builder).build()))
+            Ok(Some(
+                super::super::serde_util::task_container_properties_correct_errors(builder)
+                    .build()
+                    .map_err(|err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err))?,
+            ))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_tmpfs.rs`

```diff
--- reference/src/protocol_serde/shape_tmpfs.rs
+++ generated/src/protocol_serde/shape_tmpfs.rs
@@ -3,23 +3,23 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::Tmpfs,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.container_path {
-        object.key("containerPath").string(var_1.as_str());
+    {
+        object.key("containerPath").string(input.container_path.as_str());
     }
-    if let Some(var_2) = &input.size {
+    {
         object.key("size").number(
             #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_2).into()),
+            ::aws_smithy_types::Number::NegInt((input.size).into()),
         );
     }
-    if let Some(var_3) = &input.mount_options {
-        let mut array_4 = object.key("mountOptions").start_array();
-        for item_5 in var_3 {
+    if let Some(var_1) = &input.mount_options {
+        let mut array_2 = object.key("mountOptions").start_array();
+        for item_3 in var_1 {
             {
-                array_4.value().string(item_5.as_str());
+                array_2.value().string(item_3.as_str());
             }
         }
-        array_4.finish();
+        array_2.finish();
     }
     Ok(())
 }
@@ -72,7 +72,9 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::tmpfs_correct_errors(builder).build()))
+            Ok(Some(super::super::serde_util::tmpfs_correct_errors(builder).build().map_err(|err| {
+                ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err)
+            })?))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_ulimit.rs`

```diff
--- reference/src/protocol_serde/shape_ulimit.rs
+++ generated/src/protocol_serde/shape_ulimit.rs
@@ -3,19 +3,19 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::Ulimit,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.hard_limit {
+    {
         object.key("hardLimit").number(
             #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_1).into()),
+            ::aws_smithy_types::Number::NegInt((input.hard_limit).into()),
         );
     }
-    if let Some(var_2) = &input.name {
-        object.key("name").string(var_2.as_str());
+    {
+        object.key("name").string(input.name.as_str());
     }
-    if let Some(var_3) = &input.soft_limit {
+    {
         object.key("softLimit").number(
             #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_3).into()),
+            ::aws_smithy_types::Number::NegInt((input.soft_limit).into()),
         );
     }
     Ok(())
@@ -73,7 +73,9 @@
                     }
                 }
             }
-            Ok(Some(super::super::serde_util::ulimit_correct_errors(builder).build()))
+            Ok(Some(super::super::serde_util::ulimit_correct_errors(builder).build().map_err(|err| {
+                ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err)
+            })?))
         }
         _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
             "expected start object or null",
```

### `src/protocol_serde/shape_update_compute_environment.rs`

```diff
--- reference/src/protocol_serde/shape_update_compute_environment.rs
+++ generated/src/protocol_serde/shape_update_compute_environment.rs
@@ -103,15 +103,15 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "computeEnvironmentArn" => {
-                    builder = builder.set_compute_environment_arn(
+                "computeEnvironmentName" => {
+                    builder = builder.set_compute_environment_name(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "computeEnvironmentName" => {
-                    builder = builder.set_compute_environment_name(
+                "computeEnvironmentArn" => {
+                    builder = builder.set_compute_environment_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
```

### `src/protocol_serde/shape_update_compute_environment_input.rs`

```diff
--- reference/src/protocol_serde/shape_update_compute_environment_input.rs
+++ generated/src/protocol_serde/shape_update_compute_environment_input.rs
@@ -6,37 +6,37 @@
     if let Some(var_1) = &input.compute_environment {
         object.key("computeEnvironment").string(var_1.as_str());
     }
-    if let Some(var_2) = &input.compute_resources {
-        #[allow(unused_mut)]
-        let mut object_3 = object.key("computeResources").start_object();
-        super::super::protocol_serde::shape_compute_resource_update::ser_compute_resource_update(&mut object_3, var_2)?;
-        object_3.finish();
+    if let Some(var_2) = &input.state {
+        object.key("state").string(var_2.as_str());
     }
-    if let Some(var_4) = &input.context {
-        object.key("context").string(var_4.as_str());
+    if let Some(var_3) = &input.unmanagedv_cpus {
+        object.key("unmanagedvCpus").number(
+            #[allow(clippy::useless_conversion)]
+            ::aws_smithy_types::Number::NegInt((*var_3).into()),
+        );
     }
-    if let Some(var_5) = &input.ecs_settings {
+    if let Some(var_4) = &input.compute_resources {
         #[allow(unused_mut)]
-        let mut object_6 = object.key("ecsSettings").start_object();
-        super::super::protocol_serde::shape_ecs_settings::ser_ecs_settings(&mut object_6, var_5)?;
-        object_6.finish();
+        let mut object_5 = object.key("computeResources").start_object();
+        super::super::protocol_serde::shape_compute_resource_update::ser_compute_resource_update(&mut object_5, var_4)?;
+        object_5.finish();
     }
-    if let Some(var_7) = &input.service_role {
-        object.key("serviceRole").string(var_7.as_str());
+    if let Some(var_6) = &input.service_role {
+        object.key("serviceRole").string(var_6.as_str());
     }
-    if let Some(var_8) = &input.state {
-        object.key("state").string(var_8.as_str());
+    if let Some(var_7) = &input.update_policy {
+        #[allow(unused_mut)]
+        let mut object_8 = object.key("updatePolicy").start_object();
+        super::super::protocol_serde::shape_update_policy::ser_update_policy(&mut object_8, var_7)?;
+        object_8.finish();
     }
-    if let Some(var_9) = &input.unmanagedv_cpus {
-        object.key("unmanagedvCpus").number(
-            #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_9).into()),
-        );
+    if let Some(var_9) = &input.context {
+        object.key("context").string(var_9.as_str());
     }
-    if let Some(var_10) = &input.update_policy {
+    if let Some(var_10) = &input.ecs_settings {
         #[allow(unused_mut)]
-        let mut object_11 = object.key("updatePolicy").start_object();
-        super::super::protocol_serde::shape_update_policy::ser_update_policy(&mut object_11, var_10)?;
+        let mut object_11 = object.key("ecsSettings").start_object();
+        super::super::protocol_serde::shape_ecs_settings::ser_ecs_settings(&mut object_11, var_10)?;
         object_11.finish();
     }
     Ok(())
```

### `src/protocol_serde/shape_update_consumable_resource.rs`

```diff
--- reference/src/protocol_serde/shape_update_consumable_resource.rs
+++ generated/src/protocol_serde/shape_update_consumable_resource.rs
@@ -73,7 +73,9 @@
         output = super::super::protocol_serde::shape_update_consumable_resource::de_update_consumable_resource(_response_body, output)
             .map_err(super::super::operation::update_consumable_resource::UpdateConsumableResourceError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
-        super::super::serde_util::update_consumable_resource_output_output_correct_errors(output).build()
+        super::super::serde_util::update_consumable_resource_output_output_correct_errors(output)
+            .build()
+            .map_err(super::super::operation::update_consumable_resource::UpdateConsumableResourceError::unhandled)?
     })
 }

@@ -103,15 +105,15 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "consumableResourceArn" => {
-                    builder = builder.set_consumable_resource_arn(
+                "consumableResourceName" => {
+                    builder = builder.set_consumable_resource_name(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "consumableResourceName" => {
-                    builder = builder.set_consumable_resource_name(
+                "consumableResourceArn" => {
+                    builder = builder.set_consumable_resource_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
```

### `src/protocol_serde/shape_update_consumable_resource_input.rs`

```diff
--- reference/src/protocol_serde/shape_update_consumable_resource_input.rs
+++ generated/src/protocol_serde/shape_update_consumable_resource_input.rs
@@ -3,20 +3,20 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::operation::update_consumable_resource::UpdateConsumableResourceInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.client_token {
-        object.key("clientToken").string(var_1.as_str());
+    if let Some(var_1) = &input.consumable_resource {
+        object.key("consumableResource").string(var_1.as_str());
     }
-    if let Some(var_2) = &input.consumable_resource {
-        object.key("consumableResource").string(var_2.as_str());
-    }
-    if let Some(var_3) = &input.operation {
-        object.key("operation").string(var_3.as_str());
+    if let Some(var_2) = &input.operation {
+        object.key("operation").string(var_2.as_str());
     }
-    if let Some(var_4) = &input.quantity {
+    if let Some(var_3) = &input.quantity {
         object.key("quantity").number(
             #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_4).into()),
+            ::aws_smithy_types::Number::NegInt((*var_3).into()),
         );
     }
+    if let Some(var_4) = &input.client_token {
+        object.key("clientToken").string(var_4.as_str());
+    }
     Ok(())
 }
```

### `src/protocol_serde/shape_update_job_queue.rs`

```diff
--- reference/src/protocol_serde/shape_update_job_queue.rs
+++ generated/src/protocol_serde/shape_update_job_queue.rs
@@ -93,15 +93,15 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "jobQueueArn" => {
-                    builder = builder.set_job_queue_arn(
+                "jobQueueName" => {
+                    builder = builder.set_job_queue_name(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "jobQueueName" => {
-                    builder = builder.set_job_queue_name(
+                "jobQueueArn" => {
+                    builder = builder.set_job_queue_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
```

### `src/protocol_serde/shape_update_job_queue_input.rs`

```diff
--- reference/src/protocol_serde/shape_update_job_queue_input.rs
+++ generated/src/protocol_serde/shape_update_job_queue_input.rs
@@ -3,56 +3,56 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::operation::update_job_queue::UpdateJobQueueInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.compute_environment_order {
-        let mut array_2 = object.key("computeEnvironmentOrder").start_array();
-        for item_3 in var_1 {
+    if let Some(var_1) = &input.job_queue {
+        object.key("jobQueue").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.state {
+        object.key("state").string(var_2.as_str());
+    }
+    if let Some(var_3) = &input.scheduling_policy_arn {
+        object.key("schedulingPolicyArn").string(var_3.as_str());
+    }
+    if let Some(var_4) = &input.priority {
+        object.key("priority").number(
+            #[allow(clippy::useless_conversion)]
+            ::aws_smithy_types::Number::NegInt((*var_4).into()),
+        );
+    }
+    if let Some(var_5) = &input.compute_environment_order {
+        let mut array_6 = object.key("computeEnvironmentOrder").start_array();
+        for item_7 in var_5 {
             {
                 #[allow(unused_mut)]
-                let mut object_4 = array_2.value().start_object();
-                super::super::protocol_serde::shape_compute_environment_order::ser_compute_environment_order(&mut object_4, item_3)?;
-                object_4.finish();
+                let mut object_8 = array_6.value().start_object();
+                super::super::protocol_serde::shape_compute_environment_order::ser_compute_environment_order(&mut object_8, item_7)?;
+                object_8.finish();
             }
         }
-        array_2.finish();
-    }
-    if let Some(var_5) = &input.job_queue {
-        object.key("jobQueue").string(var_5.as_str());
+        array_6.finish();
     }
-    if let Some(var_6) = &input.job_state_time_limit_actions {
-        let mut array_7 = object.key("jobStateTimeLimitActions").start_array();
-        for item_8 in var_6 {
+    if let Some(var_9) = &input.service_environment_order {
+        let mut array_10 = object.key("serviceEnvironmentOrder").start_array();
+        for item_11 in var_9 {
             {
                 #[allow(unused_mut)]
-                let mut object_9 = array_7.value().start_object();
-                super::super::protocol_serde::shape_job_state_time_limit_action::ser_job_state_time_limit_action(&mut object_9, item_8)?;
-                object_9.finish();
+                let mut object_12 = array_10.value().start_object();
+                super::super::protocol_serde::shape_service_environment_order::ser_service_environment_order(&mut object_12, item_11)?;
+                object_12.finish();
             }
         }
-        array_7.finish();
+        array_10.finish();
     }
-    if let Some(var_10) = &input.priority {
-        object.key("priority").number(
-            #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_10).into()),
-        );
-    }
-    if let Some(var_11) = &input.scheduling_policy_arn {
-        object.key("schedulingPolicyArn").string(var_11.as_str());
-    }
-    if let Some(var_12) = &input.service_environment_order {
-        let mut array_13 = object.key("serviceEnvironmentOrder").start_array();
-        for item_14 in var_12 {
+    if let Some(var_13) = &input.job_state_time_limit_actions {
+        let mut array_14 = object.key("jobStateTimeLimitActions").start_array();
+        for item_15 in var_13 {
             {
                 #[allow(unused_mut)]
-                let mut object_15 = array_13.value().start_object();
-                super::super::protocol_serde::shape_service_environment_order::ser_service_environment_order(&mut object_15, item_14)?;
-                object_15.finish();
+                let mut object_16 = array_14.value().start_object();
+                super::super::protocol_serde::shape_job_state_time_limit_action::ser_job_state_time_limit_action(&mut object_16, item_15)?;
+                object_16.finish();
             }
         }
-        array_13.finish();
-    }
-    if let Some(var_16) = &input.state {
-        object.key("state").string(var_16.as_str());
+        array_14.finish();
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_update_quota_share.rs`

```diff
--- reference/src/protocol_serde/shape_update_quota_share.rs
+++ generated/src/protocol_serde/shape_update_quota_share.rs
@@ -93,15 +93,15 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "quotaShareArn" => {
-                    builder = builder.set_quota_share_arn(
+                "quotaShareName" => {
+                    builder = builder.set_quota_share_name(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "quotaShareName" => {
-                    builder = builder.set_quota_share_name(
+                "quotaShareArn" => {
+                    builder = builder.set_quota_share_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
```

### `src/protocol_serde/shape_update_quota_share_input.rs`

```diff
--- reference/src/protocol_serde/shape_update_quota_share_input.rs
+++ generated/src/protocol_serde/shape_update_quota_share_input.rs
@@ -3,34 +3,34 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::operation::update_quota_share::UpdateQuotaShareInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.capacity_limits {
-        let mut array_2 = object.key("capacityLimits").start_array();
-        for item_3 in var_1 {
+    if let Some(var_1) = &input.quota_share_arn {
+        object.key("quotaShareArn").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.capacity_limits {
+        let mut array_3 = object.key("capacityLimits").start_array();
+        for item_4 in var_2 {
             {
                 #[allow(unused_mut)]
-                let mut object_4 = array_2.value().start_object();
-                super::super::protocol_serde::shape_quota_share_capacity_limit::ser_quota_share_capacity_limit(&mut object_4, item_3)?;
-                object_4.finish();
+                let mut object_5 = array_3.value().start_object();
+                super::super::protocol_serde::shape_quota_share_capacity_limit::ser_quota_share_capacity_limit(&mut object_5, item_4)?;
+                object_5.finish();
             }
         }
-        array_2.finish();
+        array_3.finish();
     }
-    if let Some(var_5) = &input.preemption_configuration {
+    if let Some(var_6) = &input.resource_sharing_configuration {
         #[allow(unused_mut)]
-        let mut object_6 = object.key("preemptionConfiguration").start_object();
-        super::super::protocol_serde::shape_quota_share_preemption_configuration::ser_quota_share_preemption_configuration(&mut object_6, var_5)?;
-        object_6.finish();
+        let mut object_7 = object.key("resourceSharingConfiguration").start_object();
+        super::super::protocol_serde::shape_quota_share_resource_sharing_configuration::ser_quota_share_resource_sharing_configuration(
+            &mut object_7,
+            var_6,
+        )?;
+        object_7.finish();
     }
-    if let Some(var_7) = &input.quota_share_arn {
-        object.key("quotaShareArn").string(var_7.as_str());
-    }
-    if let Some(var_8) = &input.resource_sharing_configuration {
+    if let Some(var_8) = &input.preemption_configuration {
         #[allow(unused_mut)]
-        let mut object_9 = object.key("resourceSharingConfiguration").start_object();
-        super::super::protocol_serde::shape_quota_share_resource_sharing_configuration::ser_quota_share_resource_sharing_configuration(
-            &mut object_9,
-            var_8,
-        )?;
+        let mut object_9 = object.key("preemptionConfiguration").start_object();
+        super::super::protocol_serde::shape_quota_share_preemption_configuration::ser_quota_share_preemption_configuration(&mut object_9, var_8)?;
         object_9.finish();
     }
     if let Some(var_10) = &input.state {
```

### `src/protocol_serde/shape_update_scheduling_policy_input.rs`

```diff
--- reference/src/protocol_serde/shape_update_scheduling_policy_input.rs
+++ generated/src/protocol_serde/shape_update_scheduling_policy_input.rs
@@ -6,16 +6,16 @@
     if let Some(var_1) = &input.arn {
         object.key("arn").string(var_1.as_str());
     }
-    if let Some(var_2) = &input.fairshare_policy {
+    if let Some(var_2) = &input.quota_share_policy {
         #[allow(unused_mut)]
-        let mut object_3 = object.key("fairsharePolicy").start_object();
-        super::super::protocol_serde::shape_fairshare_policy::ser_fairshare_policy(&mut object_3, var_2)?;
+        let mut object_3 = object.key("quotaSharePolicy").start_object();
+        super::super::protocol_serde::shape_quota_share_policy::ser_quota_share_policy(&mut object_3, var_2)?;
         object_3.finish();
     }
-    if let Some(var_4) = &input.quota_share_policy {
+    if let Some(var_4) = &input.fairshare_policy {
         #[allow(unused_mut)]
-        let mut object_5 = object.key("quotaSharePolicy").start_object();
-        super::super::protocol_serde::shape_quota_share_policy::ser_quota_share_policy(&mut object_5, var_4)?;
+        let mut object_5 = object.key("fairsharePolicy").start_object();
+        super::super::protocol_serde::shape_fairshare_policy::ser_fairshare_policy(&mut object_5, var_4)?;
         object_5.finish();
     }
     Ok(())
```

### `src/protocol_serde/shape_update_service_environment.rs`

```diff
--- reference/src/protocol_serde/shape_update_service_environment.rs
+++ generated/src/protocol_serde/shape_update_service_environment.rs
@@ -73,7 +73,9 @@
         output = super::super::protocol_serde::shape_update_service_environment::de_update_service_environment(_response_body, output)
             .map_err(super::super::operation::update_service_environment::UpdateServiceEnvironmentError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
-        super::super::serde_util::update_service_environment_output_output_correct_errors(output).build()
+        super::super::serde_util::update_service_environment_output_output_correct_errors(output)
+            .build()
+            .map_err(super::super::operation::update_service_environment::UpdateServiceEnvironmentError::unhandled)?
     })
 }

@@ -103,15 +105,15 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "serviceEnvironmentArn" => {
-                    builder = builder.set_service_environment_arn(
+                "serviceEnvironmentName" => {
+                    builder = builder.set_service_environment_name(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "serviceEnvironmentName" => {
-                    builder = builder.set_service_environment_name(
+                "serviceEnvironmentArn" => {
+                    builder = builder.set_service_environment_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
```

### `src/protocol_serde/shape_update_service_environment_input.rs`

```diff
--- reference/src/protocol_serde/shape_update_service_environment_input.rs
+++ generated/src/protocol_serde/shape_update_service_environment_input.rs
@@ -3,23 +3,23 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::operation::update_service_environment::UpdateServiceEnvironmentInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.capacity_limits {
-        let mut array_2 = object.key("capacityLimits").start_array();
-        for item_3 in var_1 {
+    if let Some(var_1) = &input.service_environment {
+        object.key("serviceEnvironment").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.state {
+        object.key("state").string(var_2.as_str());
+    }
+    if let Some(var_3) = &input.capacity_limits {
+        let mut array_4 = object.key("capacityLimits").start_array();
+        for item_5 in var_3 {
             {
                 #[allow(unused_mut)]
-                let mut object_4 = array_2.value().start_object();
-                super::super::protocol_serde::shape_capacity_limit::ser_capacity_limit(&mut object_4, item_3)?;
-                object_4.finish();
+                let mut object_6 = array_4.value().start_object();
+                super::super::protocol_serde::shape_capacity_limit::ser_capacity_limit(&mut object_6, item_5)?;
+                object_6.finish();
             }
         }
-        array_2.finish();
-    }
-    if let Some(var_5) = &input.service_environment {
-        object.key("serviceEnvironment").string(var_5.as_str());
-    }
-    if let Some(var_6) = &input.state {
-        object.key("state").string(var_6.as_str());
+        array_4.finish();
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_update_service_job.rs`

```diff
--- reference/src/protocol_serde/shape_update_service_job.rs
+++ generated/src/protocol_serde/shape_update_service_job.rs
@@ -100,15 +100,15 @@
                             .transpose()?,
                     );
                 }
-                "jobId" => {
-                    builder = builder.set_job_id(
+                "jobName" => {
+                    builder = builder.set_job_name(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "jobName" => {
-                    builder = builder.set_job_name(
+                "jobId" => {
+                    builder = builder.set_job_id(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
```

### `src/protocol_serde.rs`

```diff
--- reference/src/protocol_serde.rs
+++ generated/src/protocol_serde.rs
@@ -113,8 +113,6 @@

 pub(crate) mod shape_update_service_job;

-pub(crate) mod shape_cancel_job_input;
-
 pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
     if data.is_empty() {
         b"{}"
@@ -123,6 +121,8 @@
     }
 }

+pub(crate) mod shape_cancel_job_input;
+
 pub(crate) mod shape_client_exception;

 pub(crate) mod shape_create_compute_environment_input;
```

### `src/serde_util.rs`

```diff
--- reference/src/serde_util.rs
+++ generated/src/serde_util.rs
@@ -484,8 +484,8 @@
 pub(crate) fn ephemeral_storage_correct_errors(
     mut builder: super::types::builders::EphemeralStorageBuilder,
 ) -> super::types::builders::EphemeralStorageBuilder {
-    if builder.size_in_gib.is_none() {
-        builder.size_in_gib = Some(Default::default())
+    if builder.size_in_gi_b.is_none() {
+        builder.size_in_gi_b = Some(Default::default())
     }
     builder
 }
@@ -526,7 +526,7 @@
     if builder.instance_launch_template.is_none() {
         builder.instance_launch_template = {
             let builder = super::types::builders::InstanceLaunchTemplateBuilder::default();
-            Some(super::serde_util::instance_launch_template_correct_errors(builder).build())
+            super::serde_util::instance_launch_template_correct_errors(builder).build().ok()
         }
     }
     builder
@@ -589,7 +589,9 @@
     if builder.network_configuration.is_none() {
         builder.network_configuration = {
             let builder = super::types::builders::ManagedInstancesNetworkConfigurationBuilder::default();
-            Some(super::serde_util::managed_instances_network_configuration_correct_errors(builder).build())
+            super::serde_util::managed_instances_network_configuration_correct_errors(builder)
+                .build()
+                .ok()
         }
     }
     builder
```

### `src/types/_compute_environment_detail.rs`

```diff
--- reference/src/types/_compute_environment_detail.rs
+++ generated/src/types/_compute_environment_detail.rs
@@ -5,9 +5,9 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct ComputeEnvironmentDetail {
     /// <p>The name of the compute environment. It can be up to 128 characters long. It can contain uppercase and lowercase letters, numbers, hyphens (-), and underscores (_).</p>
-    pub compute_environment_name: ::std::option::Option<::std::string::String>,
+    pub compute_environment_name: ::std::string::String,
     /// <p>The Amazon Resource Name (ARN) of the compute environment.</p>
-    pub compute_environment_arn: ::std::option::Option<::std::string::String>,
+    pub compute_environment_arn: ::std::string::String,
     /// <p>The maximum number of VCPUs expected to be used for an unmanaged compute environment.</p>
     pub unmanagedv_cpus: ::std::option::Option<i32>,
     /// <p>The Amazon Resource Name (ARN) of the underlying Amazon ECS cluster that the compute environment uses.</p>
@@ -46,12 +46,14 @@
 }
 impl ComputeEnvironmentDetail {
     /// <p>The name of the compute environment. It can be up to 128 characters long. It can contain uppercase and lowercase letters, numbers, hyphens (-), and underscores (_).</p>
-    pub fn compute_environment_name(&self) -> ::std::option::Option<&str> {
-        self.compute_environment_name.as_deref()
+    pub fn compute_environment_name(&self) -> &str {
+        use std::ops::Deref;
+        self.compute_environment_name.deref()
     }
     /// <p>The Amazon Resource Name (ARN) of the compute environment.</p>
-    pub fn compute_environment_arn(&self) -> ::std::option::Option<&str> {
-        self.compute_environment_arn.as_deref()
+    pub fn compute_environment_arn(&self) -> &str {
+        use std::ops::Deref;
+        self.compute_environment_arn.deref()
     }
     /// <p>The maximum number of VCPUs expected to be used for an unmanaged compute environment.</p>
     pub fn unmanagedv_cpus(&self) -> ::std::option::Option<i32> {
@@ -411,10 +413,23 @@
         &self.ecs_settings
     }
     /// Consumes the builder and constructs a [`ComputeEnvironmentDetail`](crate::types::ComputeEnvironmentDetail).
-    pub fn build(self) -> super::super::types::ComputeEnvironmentDetail {
-        super::super::types::ComputeEnvironmentDetail {
-            compute_environment_name: self.compute_environment_name,
-            compute_environment_arn: self.compute_environment_arn,
+    /// This method will fail if any of the following fields are not set:
+    /// - [`compute_environment_name`](crate::types::builders::ComputeEnvironmentDetailBuilder::compute_environment_name)
+    /// - [`compute_environment_arn`](crate::types::builders::ComputeEnvironmentDetailBuilder::compute_environment_arn)
+    pub fn build(self) -> ::std::result::Result<super::super::types::ComputeEnvironmentDetail, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::ComputeEnvironmentDetail {
+            compute_environment_name: self.compute_environment_name.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "compute_environment_name",
+                    "compute_environment_name was not specified but it is required when building ComputeEnvironmentDetail",
+                )
+            })?,
+            compute_environment_arn: self.compute_environment_arn.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "compute_environment_arn",
+                    "compute_environment_arn was not specified but it is required when building ComputeEnvironmentDetail",
+                )
+            })?,
             unmanagedv_cpus: self.unmanagedv_cpus,
             ecs_cluster_arn: self.ecs_cluster_arn,
             tags: self.tags,
@@ -430,6 +445,6 @@
             uuid: self.uuid,
             context: self.context,
             ecs_settings: self.ecs_settings,
-        }
+        })
     }
 }
```

### `src/types/_compute_environment_order.rs`

```diff
--- reference/src/types/_compute_environment_order.rs
+++ generated/src/types/_compute_environment_order.rs
@@ -7,18 +7,19 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct ComputeEnvironmentOrder {
     /// <p>The order of the compute environment. Compute environments are tried in ascending order. For example, if two compute environments are associated with a job queue, the compute environment with a lower <code>order</code> integer value is tried for job placement first.</p>
-    pub order: ::std::option::Option<i32>,
+    pub order: i32,
     /// <p>The Amazon Resource Name (ARN) of the compute environment.</p>
-    pub compute_environment: ::std::option::Option<::std::string::String>,
+    pub compute_environment: ::std::string::String,
 }
 impl ComputeEnvironmentOrder {
     /// <p>The order of the compute environment. Compute environments are tried in ascending order. For example, if two compute environments are associated with a job queue, the compute environment with a lower <code>order</code> integer value is tried for job placement first.</p>
-    pub fn order(&self) -> ::std::option::Option<i32> {
+    pub fn order(&self) -> i32 {
         self.order
     }
     /// <p>The Amazon Resource Name (ARN) of the compute environment.</p>
-    pub fn compute_environment(&self) -> ::std::option::Option<&str> {
-        self.compute_environment.as_deref()
+    pub fn compute_environment(&self) -> &str {
+        use std::ops::Deref;
+        self.compute_environment.deref()
     }
 }
 impl ComputeEnvironmentOrder {
@@ -67,10 +68,23 @@
         &self.compute_environment
     }
     /// Consumes the builder and constructs a [`ComputeEnvironmentOrder`](crate::types::ComputeEnvironmentOrder).
-    pub fn build(self) -> super::super::types::ComputeEnvironmentOrder {
-        super::super::types::ComputeEnvironmentOrder {
-            order: self.order,
-            compute_environment: self.compute_environment,
-        }
+    /// This method will fail if any of the following fields are not set:
+    /// - [`order`](crate::types::builders::ComputeEnvironmentOrderBuilder::order)
+    /// - [`compute_environment`](crate::types::builders::ComputeEnvironmentOrderBuilder::compute_environment)
+    pub fn build(self) -> ::std::result::Result<super::super::types::ComputeEnvironmentOrder, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::ComputeEnvironmentOrder {
+            order: self.order.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "order",
+                    "order was not specified but it is required when building ComputeEnvironmentOrder",
+                )
+            })?,
+            compute_environment: self.compute_environment.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "compute_environment",
+                    "compute_environment was not specified but it is required when building ComputeEnvironmentOrder",
+                )
+            })?,
+        })
     }
 }
```

### `src/types/_compute_resource.rs`

```diff
--- reference/src/types/_compute_resource.rs
+++ generated/src/types/_compute_resource.rs
@@ -9,7 +9,7 @@
     /// <p>If you choose <code>ECS_MANAGED_INSTANCES</code>, you must also specify a <code>managedInstancesProvider</code> configuration. To use Spot capacity, set <code>capacityOptionType</code> to <code>SPOT</code> in the <code>managedInstancesProvider.instanceLaunchTemplate</code> configuration. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/ecs_managed_instances.html">Amazon ECS Managed Instances compute environments</a> in the <i>Batch User Guide</i>.</p><note>
     /// <p>Multi-node parallel jobs aren't supported on Spot Instances or Amazon ECS Managed Instances.</p>
     /// </note>
-    pub r#type: ::std::option::Option<super::super::types::CrType>,
+    pub r#type: super::super::types::CrType,
     /// <p>The allocation strategy to use for the compute resource if not enough instances of the best fitting instance type can be allocated. This might be because of availability of the instance type in the Region or <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-resource-limits.html">Amazon EC2 service limits</a>. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/allocation-strategies.html">Allocation strategies</a> in the <i>Batch User Guide</i>.</p><note>
     /// <p>This parameter isn't applicable to jobs that are running on Fargate resources. Don't specify it.</p>
     /// </note> <note>
@@ -71,7 +71,7 @@
     /// <p>The maximum number of vCPUs that a compute environment can support.</p><note>
     /// <p>With any allocation strategy except <code>BEST_FIT</code> using On-Demand (<code>EC2</code>) compute resources, Batch might need to exceed <code>maxvCpus</code> to meet your capacity requirements. In this event, Batch never exceeds <code>maxvCpus</code> by more than a single instance.</p>
     /// </note>
-    pub maxv_cpus: ::std::option::Option<i32>,
+    pub maxv_cpus: i32,
     /// <p>The desired number of vCPUS in the compute environment. Batch modifies this value between the minimum and maximum values based on job queue demand.</p><note>
     /// <p>This parameter isn't applicable to jobs that are running on Fargate resources. Don't specify it.</p>
     /// </note>
@@ -161,8 +161,8 @@
     /// <p>If you choose <code>ECS_MANAGED_INSTANCES</code>, you must also specify a <code>managedInstancesProvider</code> configuration. To use Spot capacity, set <code>capacityOptionType</code> to <code>SPOT</code> in the <code>managedInstancesProvider.instanceLaunchTemplate</code> configuration. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/ecs_managed_instances.html">Amazon ECS Managed Instances compute environments</a> in the <i>Batch User Guide</i>.</p><note>
     /// <p>Multi-node parallel jobs aren't supported on Spot Instances or Amazon ECS Managed Instances.</p>
     /// </note>
-    pub fn r#type(&self) -> ::std::option::Option<&super::super::types::CrType> {
-        self.r#type.as_ref()
+    pub fn r#type(&self) -> &super::super::types::CrType {
+        &self.r#type
     }
     /// <p>The allocation strategy to use for the compute resource if not enough instances of the best fitting instance type can be allocated. This might be because of availability of the instance type in the Region or <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-resource-limits.html">Amazon EC2 service limits</a>. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/allocation-strategies.html">Allocation strategies</a> in the <i>Batch User Guide</i>.</p><note>
     /// <p>This parameter isn't applicable to jobs that are running on Fargate resources. Don't specify it.</p>
@@ -229,7 +229,7 @@
     /// <p>The maximum number of vCPUs that a compute environment can support.</p><note>
     /// <p>With any allocation strategy except <code>BEST_FIT</code> using On-Demand (<code>EC2</code>) compute resources, Batch might need to exceed <code>maxvCpus</code> to meet your capacity requirements. In this event, Batch never exceeds <code>maxvCpus</code> by more than a single instance.</p>
     /// </note>
-    pub fn maxv_cpus(&self) -> ::std::option::Option<i32> {
+    pub fn maxv_cpus(&self) -> i32 {
         self.maxv_cpus
     }
     /// <p>The desired number of vCPUS in the compute environment. Batch modifies this value between the minimum and maximum values based on job queue demand.</p><note>
@@ -1040,12 +1040,25 @@
         &self.capacity_tags
     }
     /// Consumes the builder and constructs a [`ComputeResource`](crate::types::ComputeResource).
-    pub fn build(self) -> super::super::types::ComputeResource {
-        super::super::types::ComputeResource {
-            r#type: self.r#type,
+    /// This method will fail if any of the following fields are not set:
+    /// - [`r#type`](crate::types::builders::ComputeResourceBuilder::type)
+    /// - [`maxv_cpus`](crate::types::builders::ComputeResourceBuilder::maxv_cpus)
+    pub fn build(self) -> ::std::result::Result<super::super::types::ComputeResource, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::ComputeResource {
+            r#type: self.r#type.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "r#type",
+                    "r#type was not specified but it is required when building ComputeResource",
+                )
+            })?,
             allocation_strategy: self.allocation_strategy,
             minv_cpus: self.minv_cpus,
-            maxv_cpus: self.maxv_cpus,
+            maxv_cpus: self.maxv_cpus.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "maxv_cpus",
+                    "maxv_cpus was not specified but it is required when building ComputeResource",
+                )
+            })?,
             desiredv_cpus: self.desiredv_cpus,
             instance_types: self.instance_types,
             image_id: self.image_id,
@@ -1062,6 +1075,6 @@
             scaling_policy: self.scaling_policy,
             managed_instances_provider: self.managed_instances_provider,
             capacity_tags: self.capacity_tags,
-        }
+        })
     }
 }
```

### `src/types/_consumable_resource_summary.rs`

```diff
--- reference/src/types/_consumable_resource_summary.rs
+++ generated/src/types/_consumable_resource_summary.rs
@@ -5,9 +5,9 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct ConsumableResourceSummary {
     /// <p>The Amazon Resource Name (ARN) of the consumable resource.</p>
-    pub consumable_resource_arn: ::std::option::Option<::std::string::String>,
+    pub consumable_resource_arn: ::std::string::String,
     /// <p>The name of the consumable resource.</p>
-    pub consumable_resource_name: ::std::option::Option<::std::string::String>,
+    pub consumable_resource_name: ::std::string::String,
     /// <p>The total amount of the consumable resource that is available.</p>
     pub total_quantity: ::std::option::Option<i64>,
     /// <p>The amount of the consumable resource that is currently in use.</p>
@@ -23,12 +23,14 @@
 }
 impl ConsumableResourceSummary {
     /// <p>The Amazon Resource Name (ARN) of the consumable resource.</p>
-    pub fn consumable_resource_arn(&self) -> ::std::option::Option<&str> {
-        self.consumable_resource_arn.as_deref()
+    pub fn consumable_resource_arn(&self) -> &str {
+        use std::ops::Deref;
+        self.consumable_resource_arn.deref()
     }
     /// <p>The name of the consumable resource.</p>
-    pub fn consumable_resource_name(&self) -> ::std::option::Option<&str> {
-        self.consumable_resource_name.as_deref()
+    pub fn consumable_resource_name(&self) -> &str {
+        use std::ops::Deref;
+        self.consumable_resource_name.deref()
     }
     /// <p>The total amount of the consumable resource that is available.</p>
     pub fn total_quantity(&self) -> ::std::option::Option<i64> {
@@ -158,13 +160,26 @@
         &self.resource_type
     }
     /// Consumes the builder and constructs a [`ConsumableResourceSummary`](crate::types::ConsumableResourceSummary).
-    pub fn build(self) -> super::super::types::ConsumableResourceSummary {
-        super::super::types::ConsumableResourceSummary {
-            consumable_resource_arn: self.consumable_resource_arn,
-            consumable_resource_name: self.consumable_resource_name,
+    /// This method will fail if any of the following fields are not set:
+    /// - [`consumable_resource_arn`](crate::types::builders::ConsumableResourceSummaryBuilder::consumable_resource_arn)
+    /// - [`consumable_resource_name`](crate::types::builders::ConsumableResourceSummaryBuilder::consumable_resource_name)
+    pub fn build(self) -> ::std::result::Result<super::super::types::ConsumableResourceSummary, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::ConsumableResourceSummary {
+            consumable_resource_arn: self.consumable_resource_arn.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "consumable_resource_arn",
+                    "consumable_resource_arn was not specified but it is required when building ConsumableResourceSummary",
+                )
+            })?,
+            consumable_resource_name: self.consumable_resource_name.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "consumable_resource_name",
+                    "consumable_resource_name was not specified but it is required when building ConsumableResourceSummary",
+                )
+            })?,
             total_quantity: self.total_quantity,
             in_use_quantity: self.in_use_quantity,
             resource_type: self.resource_type,
-        }
+        })
     }
 }
```

### `src/types/_device.rs`

```diff
--- reference/src/types/_device.rs
+++ generated/src/types/_device.rs
@@ -7,7 +7,7 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct Device {
     /// <p>The path for the device on the host container instance.</p>
-    pub host_path: ::std::option::Option<::std::string::String>,
+    pub host_path: ::std::string::String,
     /// <p>The path inside the container that's used to expose the host device. By default, the <code>hostPath</code> value is used.</p>
     pub container_path: ::std::option::Option<::std::string::String>,
     /// <p>The explicit permissions to provide to the container for the device. By default, the container has permissions for <code>read</code>, <code>write</code>, and <code>mknod</code> for the device.</p>
@@ -15,8 +15,9 @@
 }
 impl Device {
     /// <p>The path for the device on the host container instance.</p>
-    pub fn host_path(&self) -> ::std::option::Option<&str> {
-        self.host_path.as_deref()
+    pub fn host_path(&self) -> &str {
+        use std::ops::Deref;
+        self.host_path.deref()
     }
     /// <p>The path inside the container that's used to expose the host device. By default, the <code>hostPath</code> value is used.</p>
     pub fn container_path(&self) -> ::std::option::Option<&str> {
@@ -95,11 +96,18 @@
         &self.permissions
     }
     /// Consumes the builder and constructs a [`Device`](crate::types::Device).
-    pub fn build(self) -> super::super::types::Device {
-        super::super::types::Device {
-            host_path: self.host_path,
+    /// This method will fail if any of the following fields are not set:
+    /// - [`host_path`](crate::types::builders::DeviceBuilder::host_path)
+    pub fn build(self) -> ::std::result::Result<super::super::types::Device, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::Device {
+            host_path: self.host_path.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "host_path",
+                    "host_path was not specified but it is required when building Device",
+                )
+            })?,
             container_path: self.container_path,
             permissions: self.permissions,
-        }
+        })
     }
 }
```

### `src/types/_ec2_configuration.rs`

```diff
--- reference/src/types/_ec2_configuration.rs
+++ generated/src/types/_ec2_configuration.rs
@@ -84,7 +84,7 @@
     /// </dl>
     /// </dd>
     /// </dl>
-    pub image_type: ::std::option::Option<::std::string::String>,
+    pub image_type: ::std::string::String,
     /// <p>The AMI ID used for instances launched in the compute environment that match the image type. This setting overrides the <code>imageId</code> set in the <code>computeResource</code> object.</p><note>
     /// <p>The AMI that you choose for a compute environment must match the architecture of the instance types that you intend to use for that compute environment. For example, if your compute environment uses A1 instance types, the compute resource AMI that you choose must support ARM instances. Amazon ECS vends both x86 and ARM versions of the Amazon ECS-optimized Amazon Linux 2023 AMI. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-optimized_AMI.html#ecs-optimized-ami-linux-variants.html">Amazon ECS-optimized Amazon Linux 2023 AMI</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
     /// </note>
@@ -190,8 +190,9 @@
     /// </dl>
     /// </dd>
     /// </dl>
-    pub fn image_type(&self) -> ::std::option::Option<&str> {
-        self.image_type.as_deref()
+    pub fn image_type(&self) -> &str {
+        use std::ops::Deref;
+        self.image_type.deref()
     }
     /// <p>The AMI ID used for instances launched in the compute environment that match the image type. This setting overrides the <code>imageId</code> set in the <code>computeResource</code> object.</p><note>
     /// <p>The AMI that you choose for a compute environment must match the architecture of the instance types that you intend to use for that compute environment. For example, if your compute environment uses A1 instance types, the compute resource AMI that you choose must support ARM instances. Amazon ECS vends both x86 and ARM versions of the Amazon ECS-optimized Amazon Linux 2023 AMI. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-optimized_AMI.html#ecs-optimized-ami-linux-variants.html">Amazon ECS-optimized Amazon Linux 2023 AMI</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
@@ -588,12 +589,19 @@
         &self.image_kubernetes_version
     }
     /// Consumes the builder and constructs a [`Ec2Configuration`](crate::types::Ec2Configuration).
-    pub fn build(self) -> super::super::types::Ec2Configuration {
-        super::super::types::Ec2Configuration {
-            image_type: self.image_type,
+    /// This method will fail if any of the following fields are not set:
+    /// - [`image_type`](crate::types::builders::Ec2ConfigurationBuilder::image_type)
+    pub fn build(self) -> ::std::result::Result<super::super::types::Ec2Configuration, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::Ec2Configuration {
+            image_type: self.image_type.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "image_type",
+                    "image_type was not specified but it is required when building Ec2Configuration",
+                )
+            })?,
             image_id_override: self.image_id_override,
             batch_image_status: self.batch_image_status,
             image_kubernetes_version: self.image_kubernetes_version,
-        }
+        })
     }
 }
```

### `src/types/_ecs_properties.rs`

```diff
--- reference/src/types/_ecs_properties.rs
+++ generated/src/types/_ecs_properties.rs
@@ -7,16 +7,15 @@
     /// <p>An object that contains the properties for the Amazon ECS task definition of a job.</p><note>
     /// <p>This object is currently limited to one task element. However, the task element can run up to 10 containers.</p>
     /// </note>
-    pub task_properties: ::std::option::Option<::std::vec::Vec<super::super::types::EcsTaskProperties>>,
+    pub task_properties: ::std::vec::Vec<super::super::types::EcsTaskProperties>,
 }
 impl EcsProperties {
     /// <p>An object that contains the properties for the Amazon ECS task definition of a job.</p><note>
     /// <p>This object is currently limited to one task element. However, the task element can run up to 10 containers.</p>
     /// </note>
-    ///
-    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.task_properties.is_none()`.
     pub fn task_properties(&self) -> &[super::super::types::EcsTaskProperties] {
-        self.task_properties.as_deref().unwrap_or_default()
+        use std::ops::Deref;
+        self.task_properties.deref()
     }
 }
 impl EcsProperties {
@@ -60,9 +59,16 @@
         &self.task_properties
     }
     /// Consumes the builder and constructs a [`EcsProperties`](crate::types::EcsProperties).
-    pub fn build(self) -> super::super::types::EcsProperties {
-        super::super::types::EcsProperties {
-            task_properties: self.task_properties,
-        }
+    /// This method will fail if any of the following fields are not set:
+    /// - [`task_properties`](crate::types::builders::EcsPropertiesBuilder::task_properties)
+    pub fn build(self) -> ::std::result::Result<super::super::types::EcsProperties, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::EcsProperties {
+            task_properties: self.task_properties.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "task_properties",
+                    "task_properties was not specified but it is required when building EcsProperties",
+                )
+            })?,
+        })
     }
 }
```

### `src/types/_ecs_task_properties.rs`

```diff
--- reference/src/types/_ecs_task_properties.rs
+++ generated/src/types/_ecs_task_properties.rs
@@ -5,7 +5,7 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct EcsTaskProperties {
     /// <p>This object is a list of containers.</p>
-    pub containers: ::std::option::Option<::std::vec::Vec<super::super::types::TaskContainerProperties>>,
+    pub containers: ::std::vec::Vec<super::super::types::TaskContainerProperties>,
     /// <p>The amount of ephemeral storage to allocate for the task. This parameter is used to expand the total amount of ephemeral storage available, beyond the default amount, for tasks hosted on Fargate.</p>
     pub ephemeral_storage: ::std::option::Option<super::super::types::EphemeralStorage>,
     /// <p>The Amazon Resource Name (ARN) of the execution role that Batch can assume. For jobs that run on Fargate resources, you must provide an execution role. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/execution-IAM-role.html">Batch execution IAM role</a> in the <i>Batch User Guide</i>.</p>
@@ -45,10 +45,9 @@
 }
 impl EcsTaskProperties {
     /// <p>This object is a list of containers.</p>
-    ///
-    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.containers.is_none()`.
     pub fn containers(&self) -> &[super::super::types::TaskContainerProperties] {
-        self.containers.as_deref().unwrap_or_default()
+        use std::ops::Deref;
+        self.containers.deref()
     }
     /// <p>The amount of ephemeral storage to allocate for the task. This parameter is used to expand the total amount of ephemeral storage available, beyond the default amount, for tasks hosted on Fargate.</p>
     pub fn ephemeral_storage(&self) -> ::std::option::Option<&super::super::types::EphemeralStorage> {
@@ -359,9 +358,16 @@
         &self.network_mode
     }
     /// Consumes the builder and constructs a [`EcsTaskProperties`](crate::types::EcsTaskProperties).
-    pub fn build(self) -> super::super::types::EcsTaskProperties {
-        super::super::types::EcsTaskProperties {
-            containers: self.containers,
+    /// This method will fail if any of the following fields are not set:
+    /// - [`containers`](crate::types::builders::EcsTaskPropertiesBuilder::containers)
+    pub fn build(self) -> ::std::result::Result<super::super::types::EcsTaskProperties, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::EcsTaskProperties {
+            containers: self.containers.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "containers",
+                    "containers was not specified but it is required when building EcsTaskProperties",
+                )
+            })?,
             ephemeral_storage: self.ephemeral_storage,
             execution_role_arn: self.execution_role_arn,
             platform_version: self.platform_version,
@@ -373,6 +379,6 @@
             volumes: self.volumes,
             enable_execute_command: self.enable_execute_command,
             network_mode: self.network_mode,
-        }
+        })
     }
 }
```

### `src/types/_efs_volume_configuration.rs`

```diff
--- reference/src/types/_efs_volume_configuration.rs
+++ generated/src/types/_efs_volume_configuration.rs
@@ -5,7 +5,7 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct EfsVolumeConfiguration {
     /// <p>The Amazon EFS file system ID to use.</p>
-    pub file_system_id: ::std::option::Option<::std::string::String>,
+    pub file_system_id: ::std::string::String,
     /// <p>The directory within the Amazon EFS file system to mount as the root directory inside the host. If this parameter is omitted, the root of the Amazon EFS volume is used instead. Specifying <code>/</code> has the same effect as omitting this parameter. The maximum length is 4,096 characters.</p><important>
     /// <p>If an EFS access point is specified in the <code>authorizationConfig</code>, the root directory parameter must either be omitted or set to <code>/</code>, which enforces the path set on the Amazon EFS access point.</p>
     /// </important>
@@ -19,8 +19,9 @@
 }
 impl EfsVolumeConfiguration {
     /// <p>The Amazon EFS file system ID to use.</p>
-    pub fn file_system_id(&self) -> ::std::option::Option<&str> {
-        self.file_system_id.as_deref()
+    pub fn file_system_id(&self) -> &str {
+        use std::ops::Deref;
+        self.file_system_id.deref()
     }
     /// <p>The directory within the Amazon EFS file system to mount as the root directory inside the host. If this parameter is omitted, the root of the Amazon EFS volume is used instead. Specifying <code>/</code> has the same effect as omitting this parameter. The maximum length is 4,096 characters.</p><important>
     /// <p>If an EFS access point is specified in the <code>authorizationConfig</code>, the root directory parameter must either be omitted or set to <code>/</code>, which enforces the path set on the Amazon EFS access point.</p>
@@ -137,13 +138,20 @@
         &self.authorization_config
     }
     /// Consumes the builder and constructs a [`EfsVolumeConfiguration`](crate::types::EfsVolumeConfiguration).
-    pub fn build(self) -> super::super::types::EfsVolumeConfiguration {
-        super::super::types::EfsVolumeConfiguration {
-            file_system_id: self.file_system_id,
+    /// This method will fail if any of the following fields are not set:
+    /// - [`file_system_id`](crate::types::builders::EfsVolumeConfigurationBuilder::file_system_id)
+    pub fn build(self) -> ::std::result::Result<super::super::types::EfsVolumeConfiguration, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::EfsVolumeConfiguration {
+            file_system_id: self.file_system_id.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "file_system_id",
+                    "file_system_id was not specified but it is required when building EfsVolumeConfiguration",
+                )
+            })?,
             root_directory: self.root_directory,
             transit_encryption: self.transit_encryption,
             transit_encryption_port: self.transit_encryption_port,
             authorization_config: self.authorization_config,
-        }
+        })
     }
 }
```

### `src/types/_eks_configuration.rs`

```diff
--- reference/src/types/_eks_configuration.rs
+++ generated/src/types/_eks_configuration.rs
@@ -5,18 +5,20 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct EksConfiguration {
     /// <p>The Amazon Resource Name (ARN) of the Amazon EKS cluster. An example is <code>arn:<i>aws</i>:eks:<i>us-east-1</i>:<i>123456789012</i>:cluster/<i>ClusterForBatch</i> </code>.</p>
-    pub eks_cluster_arn: ::std::option::Option<::std::string::String>,
+    pub eks_cluster_arn: ::std::string::String,
     /// <p>The namespace of the Amazon EKS cluster. Batch manages pods in this namespace. The value can't left empty or null. It must be fewer than 64 characters long, can't be set to <code>default</code>, can't start with "<code>kube-</code>," and must match this regular expression: <code>^\[a-z0-9\](\[-a-z0-9\]*\[a-z0-9\])?$</code>. For more information, see <a href="https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/">Namespaces</a> in the Kubernetes documentation.</p>
-    pub kubernetes_namespace: ::std::option::Option<::std::string::String>,
+    pub kubernetes_namespace: ::std::string::String,
 }
 impl EksConfiguration {
     /// <p>The Amazon Resource Name (ARN) of the Amazon EKS cluster. An example is <code>arn:<i>aws</i>:eks:<i>us-east-1</i>:<i>123456789012</i>:cluster/<i>ClusterForBatch</i> </code>.</p>
-    pub fn eks_cluster_arn(&self) -> ::std::option::Option<&str> {
-        self.eks_cluster_arn.as_deref()
+    pub fn eks_cluster_arn(&self) -> &str {
+        use std::ops::Deref;
+        self.eks_cluster_arn.deref()
     }
     /// <p>The namespace of the Amazon EKS cluster. Batch manages pods in this namespace. The value can't left empty or null. It must be fewer than 64 characters long, can't be set to <code>default</code>, can't start with "<code>kube-</code>," and must match this regular expression: <code>^\[a-z0-9\](\[-a-z0-9\]*\[a-z0-9\])?$</code>. For more information, see <a href="https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/">Namespaces</a> in the Kubernetes documentation.</p>
-    pub fn kubernetes_namespace(&self) -> ::std::option::Option<&str> {
-        self.kubernetes_namespace.as_deref()
+    pub fn kubernetes_namespace(&self) -> &str {
+        use std::ops::Deref;
+        self.kubernetes_namespace.deref()
     }
 }
 impl EksConfiguration {
@@ -65,10 +67,23 @@
         &self.kubernetes_namespace
     }
     /// Consumes the builder and constructs a [`EksConfiguration`](crate::types::EksConfiguration).
-    pub fn build(self) -> super::super::types::EksConfiguration {
-        super::super::types::EksConfiguration {
-            eks_cluster_arn: self.eks_cluster_arn,
-            kubernetes_namespace: self.kubernetes_namespace,
-        }
+    /// This method will fail if any of the following fields are not set:
+    /// - [`eks_cluster_arn`](crate::types::builders::EksConfigurationBuilder::eks_cluster_arn)
+    /// - [`kubernetes_namespace`](crate::types::builders::EksConfigurationBuilder::kubernetes_namespace)
+    pub fn build(self) -> ::std::result::Result<super::super::types::EksConfiguration, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::EksConfiguration {
+            eks_cluster_arn: self.eks_cluster_arn.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "eks_cluster_arn",
+                    "eks_cluster_arn was not specified but it is required when building EksConfiguration",
+                )
+            })?,
+            kubernetes_namespace: self.kubernetes_namespace.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "kubernetes_namespace",
+                    "kubernetes_namespace was not specified but it is required when building EksConfiguration",
+                )
+            })?,
+        })
     }
 }
```

### `src/types/_eks_container.rs`

```diff
--- reference/src/types/_eks_container.rs
+++ generated/src/types/_eks_container.rs
@@ -7,7 +7,7 @@
     /// <p>The name of the container. If the name isn't specified, the default name "<code>Default</code>" is used. Each container in a pod must have a unique name.</p>
     pub name: ::std::option::Option<::std::string::String>,
     /// <p>The Docker image used to start the container.</p>
-    pub image: ::std::option::Option<::std::string::String>,
+    pub image: ::std::string::String,
     /// <p>The image pull policy for the container. Supported values are <code>Always</code>, <code>IfNotPresent</code>, and <code>Never</code>. This parameter defaults to <code>IfNotPresent</code>. However, if the <code>:latest</code> tag is specified, it defaults to <code>Always</code>. For more information, see <a href="https://kubernetes.io/docs/concepts/containers/images/#updating-images">Updating images</a> in the <i>Kubernetes documentation</i>.</p>
     pub image_pull_policy: ::std::option::Option<::std::string::String>,
     /// <p>The entrypoint for the container. This isn't run within a shell. If this isn't specified, the <code>ENTRYPOINT</code> of the container image is used. Environment variable references are expanded using the container's environment.</p>
@@ -33,8 +33,9 @@
         self.name.as_deref()
     }
     /// <p>The Docker image used to start the container.</p>
-    pub fn image(&self) -> ::std::option::Option<&str> {
-        self.image.as_deref()
+    pub fn image(&self) -> &str {
+        use std::ops::Deref;
+        self.image.deref()
     }
     /// <p>The image pull policy for the container. Supported values are <code>Always</code>, <code>IfNotPresent</code>, and <code>Never</code>. This parameter defaults to <code>IfNotPresent</code>. However, if the <code>:latest</code> tag is specified, it defaults to <code>Always</code>. For more information, see <a href="https://kubernetes.io/docs/concepts/containers/images/#updating-images">Updating images</a> in the <i>Kubernetes documentation</i>.</p>
     pub fn image_pull_policy(&self) -> ::std::option::Option<&str> {
@@ -263,10 +264,17 @@
         &self.security_context
     }
     /// Consumes the builder and constructs a [`EksContainer`](crate::types::EksContainer).
-    pub fn build(self) -> super::super::types::EksContainer {
-        super::super::types::EksContainer {
+    /// This method will fail if any of the following fields are not set:
+    /// - [`image`](crate::types::builders::EksContainerBuilder::image)
+    pub fn build(self) -> ::std::result::Result<super::super::types::EksContainer, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::EksContainer {
             name: self.name,
-            image: self.image,
+            image: self.image.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "image",
+                    "image was not specified but it is required when building EksContainer",
+                )
+            })?,
             image_pull_policy: self.image_pull_policy,
             command: self.command,
             args: self.args,
@@ -274,6 +282,6 @@
             resources: self.resources,
             volume_mounts: self.volume_mounts,
             security_context: self.security_context,
-        }
+        })
     }
 }
```

### `src/types/_eks_container_environment_variable.rs`

```diff
--- reference/src/types/_eks_container_environment_variable.rs
+++ generated/src/types/_eks_container_environment_variable.rs
@@ -5,14 +5,15 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct EksContainerEnvironmentVariable {
     /// <p>The name of the environment variable.</p>
-    pub name: ::std::option::Option<::std::string::String>,
+    pub name: ::std::string::String,
     /// <p>The value of the environment variable.</p>
     pub value: ::std::option::Option<::std::string::String>,
 }
 impl EksContainerEnvironmentVariable {
     /// <p>The name of the environment variable.</p>
-    pub fn name(&self) -> ::std::option::Option<&str> {
-        self.name.as_deref()
+    pub fn name(&self) -> &str {
+        use std::ops::Deref;
+        self.name.deref()
     }
     /// <p>The value of the environment variable.</p>
     pub fn value(&self) -> ::std::option::Option<&str> {
@@ -64,10 +65,17 @@
         &self.value
     }
     /// Consumes the builder and constructs a [`EksContainerEnvironmentVariable`](crate::types::EksContainerEnvironmentVariable).
-    pub fn build(self) -> super::super::types::EksContainerEnvironmentVariable {
-        super::super::types::EksContainerEnvironmentVariable {
-            name: self.name,
+    /// This method will fail if any of the following fields are not set:
+    /// - [`name`](crate::types::builders::EksContainerEnvironmentVariableBuilder::name)
+    pub fn build(self) -> ::std::result::Result<super::super::types::EksContainerEnvironmentVariable, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::EksContainerEnvironmentVariable {
+            name: self.name.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "name",
+                    "name was not specified but it is required when building EksContainerEnvironmentVariable",
+                )
+            })?,
             value: self.value,
-        }
+        })
     }
 }
```

### `src/types/_eks_persistent_volume_claim.rs`

```diff
--- reference/src/types/_eks_persistent_volume_claim.rs
+++ generated/src/types/_eks_persistent_volume_claim.rs
@@ -5,14 +5,15 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct EksPersistentVolumeClaim {
     /// <p>The name of the <code>persistentVolumeClaim</code> bounded to a <code>persistentVolume</code>. For more information, see <a href="https://kubernetes.io/docs/concepts/storage/persistent-volumes/#persistentvolumeclaims"> Persistent Volume Claims</a> in the <i>Kubernetes documentation</i>.</p>
-    pub claim_name: ::std::option::Option<::std::string::String>,
+    pub claim_name: ::std::string::String,
     /// <p>An optional boolean value indicating if the mount is read only. Default is false. For more information, see <a href="https://kubernetes.io/docs/concepts/storage/volumes/#read-only-mounts"> Read Only Mounts</a> in the <i>Kubernetes documentation</i>.</p>
     pub read_only: ::std::option::Option<bool>,
 }
 impl EksPersistentVolumeClaim {
     /// <p>The name of the <code>persistentVolumeClaim</code> bounded to a <code>persistentVolume</code>. For more information, see <a href="https://kubernetes.io/docs/concepts/storage/persistent-volumes/#persistentvolumeclaims"> Persistent Volume Claims</a> in the <i>Kubernetes documentation</i>.</p>
-    pub fn claim_name(&self) -> ::std::option::Option<&str> {
-        self.claim_name.as_deref()
+    pub fn claim_name(&self) -> &str {
+        use std::ops::Deref;
+        self.claim_name.deref()
     }
     /// <p>An optional boolean value indicating if the mount is read only. Default is false. For more information, see <a href="https://kubernetes.io/docs/concepts/storage/volumes/#read-only-mounts"> Read Only Mounts</a> in the <i>Kubernetes documentation</i>.</p>
     pub fn read_only(&self) -> ::std::option::Option<bool> {
@@ -64,10 +65,17 @@
         &self.read_only
     }
     /// Consumes the builder and constructs a [`EksPersistentVolumeClaim`](crate::types::EksPersistentVolumeClaim).
-    pub fn build(self) -> super::super::types::EksPersistentVolumeClaim {
-        super::super::types::EksPersistentVolumeClaim {
-            claim_name: self.claim_name,
+    /// This method will fail if any of the following fields are not set:
+    /// - [`claim_name`](crate::types::builders::EksPersistentVolumeClaimBuilder::claim_name)
+    pub fn build(self) -> ::std::result::Result<super::super::types::EksPersistentVolumeClaim, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::EksPersistentVolumeClaim {
+            claim_name: self.claim_name.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "claim_name",
+                    "claim_name was not specified but it is required when building EksPersistentVolumeClaim",
+                )
+            })?,
             read_only: self.read_only,
-        }
+        })
     }
 }
```

### `src/types/_eks_secret.rs`

```diff
--- reference/src/types/_eks_secret.rs
+++ generated/src/types/_eks_secret.rs
@@ -5,14 +5,15 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct EksSecret {
     /// <p>The name of the secret. The name must be allowed as a DNS subdomain name. For more information, see <a href="https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#dns-subdomain-names">DNS subdomain names</a> in the <i>Kubernetes documentation</i>.</p>
-    pub secret_name: ::std::option::Option<::std::string::String>,
+    pub secret_name: ::std::string::String,
     /// <p>Specifies whether the secret or the secret's keys must be defined.</p>
     pub optional: ::std::option::Option<bool>,
 }
 impl EksSecret {
     /// <p>The name of the secret. The name must be allowed as a DNS subdomain name. For more information, see <a href="https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#dns-subdomain-names">DNS subdomain names</a> in the <i>Kubernetes documentation</i>.</p>
-    pub fn secret_name(&self) -> ::std::option::Option<&str> {
-        self.secret_name.as_deref()
+    pub fn secret_name(&self) -> &str {
+        use std::ops::Deref;
+        self.secret_name.deref()
     }
     /// <p>Specifies whether the secret or the secret's keys must be defined.</p>
     pub fn optional(&self) -> ::std::option::Option<bool> {
@@ -64,10 +65,17 @@
         &self.optional
     }
     /// Consumes the builder and constructs a [`EksSecret`](crate::types::EksSecret).
-    pub fn build(self) -> super::super::types::EksSecret {
-        super::super::types::EksSecret {
-            secret_name: self.secret_name,
+    /// This method will fail if any of the following fields are not set:
+    /// - [`secret_name`](crate::types::builders::EksSecretBuilder::secret_name)
+    pub fn build(self) -> ::std::result::Result<super::super::types::EksSecret, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::EksSecret {
+            secret_name: self.secret_name.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "secret_name",
+                    "secret_name was not specified but it is required when building EksSecret",
+                )
+            })?,
             optional: self.optional,
-        }
+        })
     }
 }
```

### `src/types/_eks_volume.rs`

```diff
--- reference/src/types/_eks_volume.rs
+++ generated/src/types/_eks_volume.rs
@@ -5,7 +5,7 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct EksVolume {
     /// <p>The name of the volume. The name must be allowed as a DNS subdomain name. For more information, see <a href="https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#dns-subdomain-names">DNS subdomain names</a> in the <i>Kubernetes documentation</i>.</p>
-    pub name: ::std::option::Option<::std::string::String>,
+    pub name: ::std::string::String,
     /// <p>Specifies the configuration of a Kubernetes <code>hostPath</code> volume. For more information, see <a href="https://kubernetes.io/docs/concepts/storage/volumes/#hostpath">hostPath</a> in the <i>Kubernetes documentation</i>.</p>
     pub host_path: ::std::option::Option<super::super::types::EksHostPath>,
     /// <p>Specifies the configuration of a Kubernetes <code>emptyDir</code> volume. For more information, see <a href="https://kubernetes.io/docs/concepts/storage/volumes/#emptydir">emptyDir</a> in the <i>Kubernetes documentation</i>.</p>
@@ -17,8 +17,9 @@
 }
 impl EksVolume {
     /// <p>The name of the volume. The name must be allowed as a DNS subdomain name. For more information, see <a href="https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#dns-subdomain-names">DNS subdomain names</a> in the <i>Kubernetes documentation</i>.</p>
-    pub fn name(&self) -> ::std::option::Option<&str> {
-        self.name.as_deref()
+    pub fn name(&self) -> &str {
+        use std::ops::Deref;
+        self.name.deref()
     }
     /// <p>Specifies the configuration of a Kubernetes <code>hostPath</code> volume. For more information, see <a href="https://kubernetes.io/docs/concepts/storage/volumes/#hostpath">hostPath</a> in the <i>Kubernetes documentation</i>.</p>
     pub fn host_path(&self) -> ::std::option::Option<&super::super::types::EksHostPath> {
@@ -127,13 +128,20 @@
         &self.persistent_volume_claim
     }
     /// Consumes the builder and constructs a [`EksVolume`](crate::types::EksVolume).
-    pub fn build(self) -> super::super::types::EksVolume {
-        super::super::types::EksVolume {
-            name: self.name,
+    /// This method will fail if any of the following fields are not set:
+    /// - [`name`](crate::types::builders::EksVolumeBuilder::name)
+    pub fn build(self) -> ::std::result::Result<super::super::types::EksVolume, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::EksVolume {
+            name: self.name.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "name",
+                    "name was not specified but it is required when building EksVolume",
+                )
+            })?,
             host_path: self.host_path,
             empty_dir: self.empty_dir,
             secret: self.secret,
             persistent_volume_claim: self.persistent_volume_claim,
-        }
+        })
     }
 }
```

### `src/types/_ephemeral_storage.rs`

```diff
--- reference/src/types/_ephemeral_storage.rs
+++ generated/src/types/_ephemeral_storage.rs
@@ -5,12 +5,12 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct EphemeralStorage {
     /// <p>The total amount, in GiB, of ephemeral storage to set for the task. The minimum supported value is <code>21</code> GiB and the maximum supported value is <code>200</code> GiB.</p>
-    pub size_in_gib: ::std::option::Option<i32>,
+    pub size_in_gi_b: i32,
 }
 impl EphemeralStorage {
     /// <p>The total amount, in GiB, of ephemeral storage to set for the task. The minimum supported value is <code>21</code> GiB and the maximum supported value is <code>200</code> GiB.</p>
-    pub fn size_in_gib(&self) -> ::std::option::Option<i32> {
-        self.size_in_gib
+    pub fn size_in_gi_b(&self) -> i32 {
+        self.size_in_gi_b
     }
 }
 impl EphemeralStorage {
@@ -24,28 +24,35 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
 pub struct EphemeralStorageBuilder {
-    pub(crate) size_in_gib: ::std::option::Option<i32>,
+    pub(crate) size_in_gi_b: ::std::option::Option<i32>,
 }
 impl EphemeralStorageBuilder {
     /// <p>The total amount, in GiB, of ephemeral storage to set for the task. The minimum supported value is <code>21</code> GiB and the maximum supported value is <code>200</code> GiB.</p>
     /// This field is required.
-    pub fn size_in_gib(mut self, input: i32) -> Self {
-        self.size_in_gib = ::std::option::Option::Some(input);
+    pub fn size_in_gi_b(mut self, input: i32) -> Self {
+        self.size_in_gi_b = ::std::option::Option::Some(input);
         self
     }
     /// <p>The total amount, in GiB, of ephemeral storage to set for the task. The minimum supported value is <code>21</code> GiB and the maximum supported value is <code>200</code> GiB.</p>
-    pub fn set_size_in_gib(mut self, input: ::std::option::Option<i32>) -> Self {
-        self.size_in_gib = input;
+    pub fn set_size_in_gi_b(mut self, input: ::std::option::Option<i32>) -> Self {
+        self.size_in_gi_b = input;
         self
     }
     /// <p>The total amount, in GiB, of ephemeral storage to set for the task. The minimum supported value is <code>21</code> GiB and the maximum supported value is <code>200</code> GiB.</p>
-    pub fn get_size_in_gib(&self) -> &::std::option::Option<i32> {
-        &self.size_in_gib
+    pub fn get_size_in_gi_b(&self) -> &::std::option::Option<i32> {
+        &self.size_in_gi_b
     }
     /// Consumes the builder and constructs a [`EphemeralStorage`](crate::types::EphemeralStorage).
-    pub fn build(self) -> super::super::types::EphemeralStorage {
-        super::super::types::EphemeralStorage {
-            size_in_gib: self.size_in_gib,
-        }
+    /// This method will fail if any of the following fields are not set:
+    /// - [`size_in_gi_b`](crate::types::builders::EphemeralStorageBuilder::size_in_gi_b)
+    pub fn build(self) -> ::std::result::Result<super::super::types::EphemeralStorage, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::EphemeralStorage {
+            size_in_gi_b: self.size_in_gi_b.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "size_in_gi_b",
+                    "size_in_gi_b was not specified but it is required when building EphemeralStorage",
+                )
+            })?,
+        })
     }
 }
```

### `src/types/_evaluate_on_exit.rs`

```diff
--- reference/src/types/_evaluate_on_exit.rs
+++ generated/src/types/_evaluate_on_exit.rs
@@ -12,7 +12,7 @@
     /// <p>The string can contain up to 512 characters.</p>
     pub on_exit_code: ::std::option::Option<::std::string::String>,
     /// <p>Specifies the action to take if all of the specified conditions (<code>onStatusReason</code>, <code>onReason</code>, and <code>onExitCode</code>) are met. The values aren't case sensitive.</p>
-    pub action: ::std::option::Option<super::super::types::RetryAction>,
+    pub action: super::super::types::RetryAction,
 }
 impl EvaluateOnExit {
     /// <p>Contains a glob pattern to match against the <code>StatusReason</code> returned for a job. The pattern can contain up to 512 characters. It can contain letters, numbers, periods (.), colons (:), and white spaces (including spaces or tabs). It can optionally end with an asterisk (*) so that only the start of the string needs to be an exact match.</p>
@@ -29,8 +29,8 @@
         self.on_exit_code.as_deref()
     }
     /// <p>Specifies the action to take if all of the specified conditions (<code>onStatusReason</code>, <code>onReason</code>, and <code>onExitCode</code>) are met. The values aren't case sensitive.</p>
-    pub fn action(&self) -> ::std::option::Option<&super::super::types::RetryAction> {
-        self.action.as_ref()
+    pub fn action(&self) -> &super::super::types::RetryAction {
+        &self.action
     }
 }
 impl EvaluateOnExit {
@@ -111,12 +111,19 @@
         &self.action
     }
     /// Consumes the builder and constructs a [`EvaluateOnExit`](crate::types::EvaluateOnExit).
-    pub fn build(self) -> super::super::types::EvaluateOnExit {
-        super::super::types::EvaluateOnExit {
+    /// This method will fail if any of the following fields are not set:
+    /// - [`action`](crate::types::builders::EvaluateOnExitBuilder::action)
+    pub fn build(self) -> ::std::result::Result<super::super::types::EvaluateOnExit, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::EvaluateOnExit {
             on_status_reason: self.on_status_reason,
             on_reason: self.on_reason,
             on_exit_code: self.on_exit_code,
-            action: self.action,
-        }
+            action: self.action.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "action",
+                    "action was not specified but it is required when building EvaluateOnExit",
+                )
+            })?,
+        })
     }
 }
```

### `src/types/_firelens_configuration.rs`

```diff
--- reference/src/types/_firelens_configuration.rs
+++ generated/src/types/_firelens_configuration.rs
@@ -5,14 +5,14 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct FirelensConfiguration {
     /// <p>The log router to use. The valid values are <code>fluentd</code> or <code>fluentbit</code>.</p>
-    pub r#type: ::std::option::Option<super::super::types::FirelensConfigurationType>,
+    pub r#type: super::super::types::FirelensConfigurationType,
     /// <p>The options to use when configuring the log router. This field is optional and can be used to specify a custom configuration file or to add additional metadata, such as the task, task definition, cluster, and container instance details to the log event. If specified, the syntax to use is <code>"options":{"enable-ecs-log-metadata":"true|false","config-file-type:"s3|file","config-file-value":"arn:aws:s3:::mybucket/fluent.conf|filepath"}</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/using_firelens.html#firelens-taskdef">Creating a task definition that uses a FireLens configuration</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
     pub options: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
 }
 impl FirelensConfiguration {
     /// <p>The log router to use. The valid values are <code>fluentd</code> or <code>fluentbit</code>.</p>
-    pub fn r#type(&self) -> ::std::option::Option<&super::super::types::FirelensConfigurationType> {
-        self.r#type.as_ref()
+    pub fn r#type(&self) -> &super::super::types::FirelensConfigurationType {
+        &self.r#type
     }
     /// <p>The options to use when configuring the log router. This field is optional and can be used to specify a custom configuration file or to add additional metadata, such as the task, task definition, cluster, and container instance details to the log event. If specified, the syntax to use is <code>"options":{"enable-ecs-log-metadata":"true|false","config-file-type:"s3|file","config-file-value":"arn:aws:s3:::mybucket/fluent.conf|filepath"}</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/using_firelens.html#firelens-taskdef">Creating a task definition that uses a FireLens configuration</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
     pub fn options(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, ::std::string::String>> {
@@ -70,10 +70,17 @@
         &self.options
     }
     /// Consumes the builder and constructs a [`FirelensConfiguration`](crate::types::FirelensConfiguration).
-    pub fn build(self) -> super::super::types::FirelensConfiguration {
-        super::super::types::FirelensConfiguration {
-            r#type: self.r#type,
+    /// This method will fail if any of the following fields are not set:
+    /// - [`r#type`](crate::types::builders::FirelensConfigurationBuilder::type)
+    pub fn build(self) -> ::std::result::Result<super::super::types::FirelensConfiguration, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::FirelensConfiguration {
+            r#type: self.r#type.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "r#type",
+                    "r#type was not specified but it is required when building FirelensConfiguration",
+                )
+            })?,
             options: self.options,
-        }
+        })
     }
 }
```

### `src/types/_image_pull_secret.rs`

```diff
--- reference/src/types/_image_pull_secret.rs
+++ generated/src/types/_image_pull_secret.rs
@@ -5,12 +5,13 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct ImagePullSecret {
     /// <p>Provides a unique identifier for the <code>ImagePullSecret</code>. This object is required when <code>EksPodProperties$imagePullSecrets</code> is used.</p>
-    pub name: ::std::option::Option<::std::string::String>,
+    pub name: ::std::string::String,
 }
 impl ImagePullSecret {
     /// <p>Provides a unique identifier for the <code>ImagePullSecret</code>. This object is required when <code>EksPodProperties$imagePullSecrets</code> is used.</p>
-    pub fn name(&self) -> ::std::option::Option<&str> {
-        self.name.as_deref()
+    pub fn name(&self) -> &str {
+        use std::ops::Deref;
+        self.name.deref()
     }
 }
 impl ImagePullSecret {
@@ -43,7 +44,16 @@
         &self.name
     }
     /// Consumes the builder and constructs a [`ImagePullSecret`](crate::types::ImagePullSecret).
-    pub fn build(self) -> super::super::types::ImagePullSecret {
-        super::super::types::ImagePullSecret { name: self.name }
+    /// This method will fail if any of the following fields are not set:
+    /// - [`name`](crate::types::builders::ImagePullSecretBuilder::name)
+    pub fn build(self) -> ::std::result::Result<super::super::types::ImagePullSecret, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::ImagePullSecret {
+            name: self.name.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "name",
+                    "name was not specified but it is required when building ImagePullSecret",
+                )
+            })?,
+        })
     }
 }
```

### `src/types/_instance_launch_template.rs`

```diff
--- reference/src/types/_instance_launch_template.rs
+++ generated/src/types/_instance_launch_template.rs
@@ -5,7 +5,7 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct InstanceLaunchTemplate {
     /// <p>The Amazon Resource Name (ARN) of the Amazon EC2 instance profile for the managed instances. The instance profile must use the <code>AmazonECSInstanceRolePolicyForManagedInstances</code> managed policy with a trust policy for <code>ec2.amazonaws.com</code>.</p>
-    pub ec2_instance_profile_arn: ::std::option::Option<::std::string::String>,
+    pub ec2_instance_profile_arn: ::std::string::String,
     /// <p>The network configuration for the managed instances. Specifies the VPC subnets and security groups where instances are launched.</p>
     pub network_configuration: ::std::option::Option<super::super::types::ManagedInstancesNetworkConfiguration>,
     /// <p>The instance type requirements for the capacity provider. Use this to constrain which Amazon EC2 instance types Amazon ECS can launch. If not specified, all available instance types are eligible.</p>
@@ -33,8 +33,9 @@
 }
 impl InstanceLaunchTemplate {
     /// <p>The Amazon Resource Name (ARN) of the Amazon EC2 instance profile for the managed instances. The instance profile must use the <code>AmazonECSInstanceRolePolicyForManagedInstances</code> managed policy with a trust policy for <code>ec2.amazonaws.com</code>.</p>
-    pub fn ec2_instance_profile_arn(&self) -> ::std::option::Option<&str> {
-        self.ec2_instance_profile_arn.as_deref()
+    pub fn ec2_instance_profile_arn(&self) -> &str {
+        use std::ops::Deref;
+        self.ec2_instance_profile_arn.deref()
     }
     /// <p>The network configuration for the managed instances. Specifies the VPC subnets and security groups where instances are launched.</p>
     pub fn network_configuration(&self) -> ::std::option::Option<&super::super::types::ManagedInstancesNetworkConfiguration> {
@@ -263,9 +264,16 @@
         &self.local_storage_configuration
     }
     /// Consumes the builder and constructs a [`InstanceLaunchTemplate`](crate::types::InstanceLaunchTemplate).
-    pub fn build(self) -> super::super::types::InstanceLaunchTemplate {
-        super::super::types::InstanceLaunchTemplate {
-            ec2_instance_profile_arn: self.ec2_instance_profile_arn,
+    /// This method will fail if any of the following fields are not set:
+    /// - [`ec2_instance_profile_arn`](crate::types::builders::InstanceLaunchTemplateBuilder::ec2_instance_profile_arn)
+    pub fn build(self) -> ::std::result::Result<super::super::types::InstanceLaunchTemplate, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::InstanceLaunchTemplate {
+            ec2_instance_profile_arn: self.ec2_instance_profile_arn.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "ec2_instance_profile_arn",
+                    "ec2_instance_profile_arn was not specified but it is required when building InstanceLaunchTemplate",
+                )
+            })?,
             network_configuration: self.network_configuration,
             instance_requirements: self.instance_requirements,
             capacity_option_type: self.capacity_option_type,
@@ -275,6 +283,6 @@
             capacity_reservations: self.capacity_reservations,
             instance_metadata_tags_propagation: self.instance_metadata_tags_propagation,
             local_storage_configuration: self.local_storage_configuration,
-        }
+        })
     }
 }
```

### `src/types/_job_definition.rs`

```diff
--- reference/src/types/_job_definition.rs
+++ generated/src/types/_job_definition.rs
@@ -5,15 +5,15 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct JobDefinition {
     /// <p>The name of the job definition.</p>
-    pub job_definition_name: ::std::option::Option<::std::string::String>,
+    pub job_definition_name: ::std::string::String,
     /// <p>The Amazon Resource Name (ARN) for the job definition.</p>
-    pub job_definition_arn: ::std::option::Option<::std::string::String>,
+    pub job_definition_arn: ::std::string::String,
     /// <p>The revision of the job definition.</p>
-    pub revision: ::std::option::Option<i32>,
+    pub revision: i32,
     /// <p>The status of the job definition.</p>
     pub status: ::std::option::Option<::std::string::String>,
     /// <p>The type of job definition. It's either <code>container</code> or <code>multinode</code>. If the job is run on Fargate resources, then <code>multinode</code> isn't supported. For more information about multi-node parallel jobs, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/multi-node-job-def.html">Creating a multi-node parallel job definition</a> in the <i>Batch User Guide</i>.</p>
-    pub r#type: ::std::option::Option<::std::string::String>,
+    pub r#type: ::std::string::String,
     /// <p>The scheduling priority of the job definition. This only affects jobs in job queues with a fair-share policy. Jobs with a higher scheduling priority are scheduled before jobs with a lower scheduling priority.</p>
     pub scheduling_priority: ::std::option::Option<i32>,
     /// <p>Default parameters or parameter substitution placeholders that are set in the job definition. Parameters are specified as a key-value pair mapping. Parameters in a <code>SubmitJob</code> request override any corresponding parameter defaults from the job definition. For more information about specifying parameters, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/job_definition_parameters.html">Job definition parameters</a> in the <i>Batch User Guide</i>.</p>
@@ -45,15 +45,17 @@
 }
 impl JobDefinition {
     /// <p>The name of the job definition.</p>
-    pub fn job_definition_name(&self) -> ::std::option::Option<&str> {
-        self.job_definition_name.as_deref()
+    pub fn job_definition_name(&self) -> &str {
+        use std::ops::Deref;
+        self.job_definition_name.deref()
     }
     /// <p>The Amazon Resource Name (ARN) for the job definition.</p>
-    pub fn job_definition_arn(&self) -> ::std::option::Option<&str> {
-        self.job_definition_arn.as_deref()
+    pub fn job_definition_arn(&self) -> &str {
+        use std::ops::Deref;
+        self.job_definition_arn.deref()
     }
     /// <p>The revision of the job definition.</p>
-    pub fn revision(&self) -> ::std::option::Option<i32> {
+    pub fn revision(&self) -> i32 {
         self.revision
     }
     /// <p>The status of the job definition.</p>
@@ -61,8 +63,9 @@
         self.status.as_deref()
     }
     /// <p>The type of job definition. It's either <code>container</code> or <code>multinode</code>. If the job is run on Fargate resources, then <code>multinode</code> isn't supported. For more information about multi-node parallel jobs, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/multi-node-job-def.html">Creating a multi-node parallel job definition</a> in the <i>Batch User Guide</i>.</p>
-    pub fn r#type(&self) -> ::std::option::Option<&str> {
-        self.r#type.as_deref()
+    pub fn r#type(&self) -> &str {
+        use std::ops::Deref;
+        self.r#type.deref()
     }
     /// <p>The scheduling priority of the job definition. This only affects jobs in job queues with a fair-share policy. Jobs with a higher scheduling priority are scheduled before jobs with a lower scheduling priority.</p>
     pub fn scheduling_priority(&self) -> ::std::option::Option<i32> {
@@ -433,13 +436,38 @@
         &self.consumable_resource_properties
     }
     /// Consumes the builder and constructs a [`JobDefinition`](crate::types::JobDefinition).
-    pub fn build(self) -> super::super::types::JobDefinition {
-        super::super::types::JobDefinition {
-            job_definition_name: self.job_definition_name,
-            job_definition_arn: self.job_definition_arn,
-            revision: self.revision,
+    /// This method will fail if any of the following fields are not set:
+    /// - [`job_definition_name`](crate::types::builders::JobDefinitionBuilder::job_definition_name)
+    /// - [`job_definition_arn`](crate::types::builders::JobDefinitionBuilder::job_definition_arn)
+    /// - [`revision`](crate::types::builders::JobDefinitionBuilder::revision)
+    /// - [`r#type`](crate::types::builders::JobDefinitionBuilder::type)
+    pub fn build(self) -> ::std::result::Result<super::super::types::JobDefinition, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::JobDefinition {
+            job_definition_name: self.job_definition_name.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "job_definition_name",
+                    "job_definition_name was not specified but it is required when building JobDefinition",
+                )
+            })?,
+            job_definition_arn: self.job_definition_arn.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "job_definition_arn",
+                    "job_definition_arn was not specified but it is required when building JobDefinition",
+                )
+            })?,
+            revision: self.revision.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "revision",
+                    "revision was not specified but it is required when building JobDefinition",
+                )
+            })?,
             status: self.status,
-            r#type: self.r#type,
+            r#type: self.r#type.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "r#type",
+                    "r#type was not specified but it is required when building JobDefinition",
+                )
+            })?,
             scheduling_priority: self.scheduling_priority,
             parameters: self.parameters,
             retry_strategy: self.retry_strategy,
@@ -453,6 +481,6 @@
             eks_properties: self.eks_properties,
             container_orchestration_type: self.container_orchestration_type,
             consumable_resource_properties: self.consumable_resource_properties,
-        }
+        })
     }
 }
```

### `src/types/_job_detail.rs`

```diff
--- reference/src/types/_job_detail.rs
+++ generated/src/types/_job_detail.rs
@@ -7,15 +7,15 @@
     /// <p>The Amazon Resource Name (ARN) of the job.</p>
     pub job_arn: ::std::option::Option<::std::string::String>,
     /// <p>The job name.</p>
-    pub job_name: ::std::option::Option<::std::string::String>,
+    pub job_name: ::std::string::String,
     /// <p>The job ID.</p>
-    pub job_id: ::std::option::Option<::std::string::String>,
+    pub job_id: ::std::string::String,
     /// <p>The Amazon Resource Name (ARN) of the job queue that the job is associated with.</p>
-    pub job_queue: ::std::option::Option<::std::string::String>,
+    pub job_queue: ::std::string::String,
     /// <p>The current status for the job.</p><note>
     /// <p>If your jobs don't progress to <code>STARTING</code>, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/troubleshooting.html#job_stuck_in_runnable">Jobs stuck in RUNNABLE status</a> in the troubleshooting section of the <i>Batch User Guide</i>.</p>
     /// </note>
-    pub status: ::std::option::Option<super::super::types::JobStatus>,
+    pub status: super::super::types::JobStatus,
     /// <p>The share identifier for the job.</p>
     pub share_identifier: ::std::option::Option<::std::string::String>,
     /// <p>The scheduling policy of the job definition. This only affects jobs in job queues with a fair-share policy. Jobs with a higher scheduling priority are scheduled before jobs with a lower scheduling priority.</p>
@@ -39,13 +39,13 @@
     /// <p>The retry strategy to use for this job if an attempt fails.</p>
     pub retry_strategy: ::std::option::Option<super::super::types::RetryStrategy>,
     /// <p>The Unix timestamp (in milliseconds) for when the job was started. More specifically, it's when the job transitioned from the <code>STARTING</code> state to the <code>RUNNING</code> state.</p>
-    pub started_at: ::std::option::Option<i64>,
+    pub started_at: i64,
     /// <p>The Unix timestamp (in milliseconds) for when the job was stopped. More specifically, it's when the job transitioned from the <code>RUNNING</code> state to a terminal state, such as <code>SUCCEEDED</code> or <code>FAILED</code>.</p>
     pub stopped_at: ::std::option::Option<i64>,
     /// <p>A list of job IDs that this job depends on.</p>
     pub depends_on: ::std::option::Option<::std::vec::Vec<super::super::types::JobDependency>>,
     /// <p>The Amazon Resource Name (ARN) of the job definition that this job uses.</p>
-    pub job_definition: ::std::option::Option<::std::string::String>,
+    pub job_definition: ::std::string::String,
     /// <p>Additional parameters that are passed to the job that replace parameter substitution placeholders or override any corresponding parameter defaults from the job definition.</p>
     pub parameters: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
     /// <p>An object that represents the details for the container that's associated with the job. If the details are for a multiple-container job, this object will be empty.</p>
@@ -85,22 +85,25 @@
         self.job_arn.as_deref()
     }
     /// <p>The job name.</p>
-    pub fn job_name(&self) -> ::std::option::Option<&str> {
-        self.job_name.as_deref()
+    pub fn job_name(&self) -> &str {
+        use std::ops::Deref;
+        self.job_name.deref()
     }
     /// <p>The job ID.</p>
-    pub fn job_id(&self) -> ::std::option::Option<&str> {
-        self.job_id.as_deref()
+    pub fn job_id(&self) -> &str {
+        use std::ops::Deref;
+        self.job_id.deref()
     }
     /// <p>The Amazon Resource Name (ARN) of the job queue that the job is associated with.</p>
-    pub fn job_queue(&self) -> ::std::option::Option<&str> {
-        self.job_queue.as_deref()
+    pub fn job_queue(&self) -> &str {
+        use std::ops::Deref;
+        self.job_queue.deref()
     }
     /// <p>The current status for the job.</p><note>
     /// <p>If your jobs don't progress to <code>STARTING</code>, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/troubleshooting.html#job_stuck_in_runnable">Jobs stuck in RUNNABLE status</a> in the troubleshooting section of the <i>Batch User Guide</i>.</p>
     /// </note>
-    pub fn status(&self) -> ::std::option::Option<&super::super::types::JobStatus> {
-        self.status.as_ref()
+    pub fn status(&self) -> &super::super::types::JobStatus {
+        &self.status
     }
     /// <p>The share identifier for the job.</p>
     pub fn share_identifier(&self) -> ::std::option::Option<&str> {
@@ -139,7 +142,7 @@
         self.retry_strategy.as_ref()
     }
     /// <p>The Unix timestamp (in milliseconds) for when the job was started. More specifically, it's when the job transitioned from the <code>STARTING</code> state to the <code>RUNNING</code> state.</p>
-    pub fn started_at(&self) -> ::std::option::Option<i64> {
+    pub fn started_at(&self) -> i64 {
         self.started_at
     }
     /// <p>The Unix timestamp (in milliseconds) for when the job was stopped. More specifically, it's when the job transitioned from the <code>RUNNING</code> state to a terminal state, such as <code>SUCCEEDED</code> or <code>FAILED</code>.</p>
@@ -153,8 +156,9 @@
         self.depends_on.as_deref().unwrap_or_default()
     }
     /// <p>The Amazon Resource Name (ARN) of the job definition that this job uses.</p>
-    pub fn job_definition(&self) -> ::std::option::Option<&str> {
-        self.job_definition.as_deref()
+    pub fn job_definition(&self) -> &str {
+        use std::ops::Deref;
+        self.job_definition.deref()
     }
     /// <p>Additional parameters that are passed to the job that replace parameter substitution placeholders or override any corresponding parameter defaults from the job definition.</p>
     pub fn parameters(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, ::std::string::String>> {
@@ -771,13 +775,40 @@
         &self.consumable_resource_properties
     }
     /// Consumes the builder and constructs a [`JobDetail`](crate::types::JobDetail).
-    pub fn build(self) -> super::super::types::JobDetail {
-        super::super::types::JobDetail {
+    /// This method will fail if any of the following fields are not set:
+    /// - [`job_name`](crate::types::builders::JobDetailBuilder::job_name)
+    /// - [`job_id`](crate::types::builders::JobDetailBuilder::job_id)
+    /// - [`job_queue`](crate::types::builders::JobDetailBuilder::job_queue)
+    /// - [`status`](crate::types::builders::JobDetailBuilder::status)
+    /// - [`started_at`](crate::types::builders::JobDetailBuilder::started_at)
+    /// - [`job_definition`](crate::types::builders::JobDetailBuilder::job_definition)
+    pub fn build(self) -> ::std::result::Result<super::super::types::JobDetail, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::JobDetail {
             job_arn: self.job_arn,
-            job_name: self.job_name,
-            job_id: self.job_id,
-            job_queue: self.job_queue,
-            status: self.status,
+            job_name: self.job_name.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "job_name",
+                    "job_name was not specified but it is required when building JobDetail",
+                )
+            })?,
+            job_id: self.job_id.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "job_id",
+                    "job_id was not specified but it is required when building JobDetail",
+                )
+            })?,
+            job_queue: self.job_queue.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "job_queue",
+                    "job_queue was not specified but it is required when building JobDetail",
+                )
+            })?,
+            status: self.status.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "status",
+                    "status was not specified but it is required when building JobDetail",
+                )
+            })?,
             share_identifier: self.share_identifier,
             scheduling_priority: self.scheduling_priority,
             attempts: self.attempts,
@@ -784,10 +815,20 @@
             status_reason: self.status_reason,
             created_at: self.created_at,
             retry_strategy: self.retry_strategy,
-            started_at: self.started_at,
+            started_at: self.started_at.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "started_at",
+                    "started_at was not specified but it is required when building JobDetail",
+                )
+            })?,
             stopped_at: self.stopped_at,
             depends_on: self.depends_on,
-            job_definition: self.job_definition,
+            job_definition: self.job_definition.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "job_definition",
+                    "job_definition was not specified but it is required when building JobDetail",
+                )
+            })?,
             parameters: self.parameters,
             container: self.container,
             node_details: self.node_details,
@@ -803,6 +844,6 @@
             is_cancelled: self.is_cancelled,
             is_terminated: self.is_terminated,
             consumable_resource_properties: self.consumable_resource_properties,
-        }
+        })
     }
 }
```

### `src/types/_job_queue_detail.rs`

```diff
--- reference/src/types/_job_queue_detail.rs
+++ generated/src/types/_job_queue_detail.rs
@@ -5,11 +5,11 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct JobQueueDetail {
     /// <p>The job queue name.</p>
-    pub job_queue_name: ::std::option::Option<::std::string::String>,
+    pub job_queue_name: ::std::string::String,
     /// <p>The Amazon Resource Name (ARN) of the job queue.</p>
-    pub job_queue_arn: ::std::option::Option<::std::string::String>,
+    pub job_queue_arn: ::std::string::String,
     /// <p>Describes the ability of the queue to accept new jobs. If the job queue state is <code>ENABLED</code>, it can accept jobs. If the job queue state is <code>DISABLED</code>, new jobs can't be added to the queue, but jobs already in the queue can finish.</p>
-    pub state: ::std::option::Option<super::super::types::JqState>,
+    pub state: super::super::types::JqState,
     /// <p>The Amazon Resource Name (ARN) of the scheduling policy. The format is <code>aws:<i>Partition</i>:batch:<i>Region</i>:<i>Account</i>:scheduling-policy/<i>Name</i> </code>. For example, <code>aws:aws:batch:us-west-2:123456789012:scheduling-policy/MySchedulingPolicy</code>.</p>
     pub scheduling_policy_arn: ::std::option::Option<::std::string::String>,
     /// <p>The status of the job queue (for example, <code>CREATING</code> or <code>VALID</code>).</p>
@@ -19,9 +19,9 @@
     /// <p>The priority of the job queue. Job queue priority determines the order that job queues are evaluated when multiple queues dispatch jobs within a shared compute environment. A higher value for <code>priority</code> indicates a higher priority. Queues are evaluated in cycles, in descending order by priority. For example, a job queue with a priority value of <code>10</code> is evaluated before a queue with a priority value of <code>1</code>. All of the compute environments must be either Amazon EC2 (<code>EC2</code> or <code>SPOT</code>) or Fargate (<code>FARGATE</code> or <code>FARGATE_SPOT</code>). Amazon EC2 and Fargate compute environments can't be mixed.</p><note>
     /// <p>Job queue priority doesn't guarantee that a particular job executes before a job in a lower priority queue. Jobs added to higher priority queues during the queue evaluation cycle might not be evaluated until the next cycle. A job is dispatched from a queue only if resources are available when the queue is evaluated. If there are insufficient resources available at that time, the cycle proceeds to the next queue. This means that jobs added to higher priority queues might have to wait for jobs in multiple lower priority queues to complete before they are dispatched. You can use job dependencies to control the order for jobs from queues with different priorities. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/job_dependencies.html">Job Dependencies</a> in the <i>Batch User Guide</i>.</p>
     /// </note>
-    pub priority: ::std::option::Option<i32>,
+    pub priority: i32,
     /// <p>The compute environments that are attached to the job queue and the order that job placement is preferred. Compute environments are selected for job placement in ascending order.</p>
-    pub compute_environment_order: ::std::option::Option<::std::vec::Vec<super::super::types::ComputeEnvironmentOrder>>,
+    pub compute_environment_order: ::std::vec::Vec<super::super::types::ComputeEnvironmentOrder>,
     /// <p>The order of the service environment associated with the job queue. Job queues with a higher priority are evaluated first when associated with the same service environment.</p>
     pub service_environment_order: ::std::option::Option<::std::vec::Vec<super::super::types::ServiceEnvironmentOrder>>,
     /// <p>The type of job queue. For service jobs that run on SageMaker Training, this value is <code>SAGEMAKER_TRAINING</code>. For regular container jobs, this value is <code>EKS</code>, <code>ECS</code>, or <code>ECS_FARGATE</code> depending on the compute environment.</p>
@@ -33,16 +33,18 @@
 }
 impl JobQueueDetail {
     /// <p>The job queue name.</p>
-    pub fn job_queue_name(&self) -> ::std::option::Option<&str> {
-        self.job_queue_name.as_deref()
+    pub fn job_queue_name(&self) -> &str {
+        use std::ops::Deref;
+        self.job_queue_name.deref()
     }
     /// <p>The Amazon Resource Name (ARN) of the job queue.</p>
-    pub fn job_queue_arn(&self) -> ::std::option::Option<&str> {
-        self.job_queue_arn.as_deref()
+    pub fn job_queue_arn(&self) -> &str {
+        use std::ops::Deref;
+        self.job_queue_arn.deref()
     }
     /// <p>Describes the ability of the queue to accept new jobs. If the job queue state is <code>ENABLED</code>, it can accept jobs. If the job queue state is <code>DISABLED</code>, new jobs can't be added to the queue, but jobs already in the queue can finish.</p>
-    pub fn state(&self) -> ::std::option::Option<&super::super::types::JqState> {
-        self.state.as_ref()
+    pub fn state(&self) -> &super::super::types::JqState {
+        &self.state
     }
     /// <p>The Amazon Resource Name (ARN) of the scheduling policy. The format is <code>aws:<i>Partition</i>:batch:<i>Region</i>:<i>Account</i>:scheduling-policy/<i>Name</i> </code>. For example, <code>aws:aws:batch:us-west-2:123456789012:scheduling-policy/MySchedulingPolicy</code>.</p>
     pub fn scheduling_policy_arn(&self) -> ::std::option::Option<&str> {
@@ -59,14 +61,13 @@
     /// <p>The priority of the job queue. Job queue priority determines the order that job queues are evaluated when multiple queues dispatch jobs within a shared compute environment. A higher value for <code>priority</code> indicates a higher priority. Queues are evaluated in cycles, in descending order by priority. For example, a job queue with a priority value of <code>10</code> is evaluated before a queue with a priority value of <code>1</code>. All of the compute environments must be either Amazon EC2 (<code>EC2</code> or <code>SPOT</code>) or Fargate (<code>FARGATE</code> or <code>FARGATE_SPOT</code>). Amazon EC2 and Fargate compute environments can't be mixed.</p><note>
     /// <p>Job queue priority doesn't guarantee that a particular job executes before a job in a lower priority queue. Jobs added to higher priority queues during the queue evaluation cycle might not be evaluated until the next cycle. A job is dispatched from a queue only if resources are available when the queue is evaluated. If there are insufficient resources available at that time, the cycle proceeds to the next queue. This means that jobs added to higher priority queues might have to wait for jobs in multiple lower priority queues to complete before they are dispatched. You can use job dependencies to control the order for jobs from queues with different priorities. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/job_dependencies.html">Job Dependencies</a> in the <i>Batch User Guide</i>.</p>
     /// </note>
-    pub fn priority(&self) -> ::std::option::Option<i32> {
+    pub fn priority(&self) -> i32 {
         self.priority
     }
     /// <p>The compute environments that are attached to the job queue and the order that job placement is preferred. Compute environments are selected for job placement in ascending order.</p>
-    ///
-    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.compute_environment_order.is_none()`.
     pub fn compute_environment_order(&self) -> &[super::super::types::ComputeEnvironmentOrder] {
-        self.compute_environment_order.as_deref().unwrap_or_default()
+        use std::ops::Deref;
+        self.compute_environment_order.deref()
     }
     /// <p>The order of the service environment associated with the job queue. Job queues with a higher priority are evaluated first when associated with the same service environment.</p>
     ///
@@ -317,20 +318,51 @@
         &self.job_state_time_limit_actions
     }
     /// Consumes the builder and constructs a [`JobQueueDetail`](crate::types::JobQueueDetail).
-    pub fn build(self) -> super::super::types::JobQueueDetail {
-        super::super::types::JobQueueDetail {
-            job_queue_name: self.job_queue_name,
-            job_queue_arn: self.job_queue_arn,
-            state: self.state,
+    /// This method will fail if any of the following fields are not set:
+    /// - [`job_queue_name`](crate::types::builders::JobQueueDetailBuilder::job_queue_name)
+    /// - [`job_queue_arn`](crate::types::builders::JobQueueDetailBuilder::job_queue_arn)
+    /// - [`state`](crate::types::builders::JobQueueDetailBuilder::state)
+    /// - [`priority`](crate::types::builders::JobQueueDetailBuilder::priority)
+    /// - [`compute_environment_order`](crate::types::builders::JobQueueDetailBuilder::compute_environment_order)
+    pub fn build(self) -> ::std::result::Result<super::super::types::JobQueueDetail, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::JobQueueDetail {
+            job_queue_name: self.job_queue_name.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "job_queue_name",
+                    "job_queue_name was not specified but it is required when building JobQueueDetail",
+                )
+            })?,
+            job_queue_arn: self.job_queue_arn.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "job_queue_arn",
+                    "job_queue_arn was not specified but it is required when building JobQueueDetail",
+                )
+            })?,
+            state: self.state.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "state",
+                    "state was not specified but it is required when building JobQueueDetail",
+                )
+            })?,
             scheduling_policy_arn: self.scheduling_policy_arn,
             status: self.status,
             status_reason: self.status_reason,
-            priority: self.priority,
-            compute_environment_order: self.compute_environment_order,
+            priority: self.priority.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "priority",
+                    "priority was not specified but it is required when building JobQueueDetail",
+                )
+            })?,
+            compute_environment_order: self.compute_environment_order.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "compute_environment_order",
+                    "compute_environment_order was not specified but it is required when building JobQueueDetail",
+                )
+            })?,
             service_environment_order: self.service_environment_order,
             job_queue_type: self.job_queue_type,
             tags: self.tags,
             job_state_time_limit_actions: self.job_state_time_limit_actions,
-        }
+        })
     }
 }
```

### `src/types/_job_state_time_limit_action.rs`

```diff
--- reference/src/types/_job_state_time_limit_action.rs
+++ generated/src/types/_job_state_time_limit_action.rs
@@ -5,30 +5,31 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct JobStateTimeLimitAction {
     /// <p>The reason to log for the action being taken.</p>
-    pub reason: ::std::option::Option<::std::string::String>,
+    pub reason: ::std::string::String,
     /// <p>The state of the job needed to trigger the action. The only supported value is <code>RUNNABLE</code>.</p>
-    pub state: ::std::option::Option<super::super::types::JobStateTimeLimitActionsState>,
+    pub state: super::super::types::JobStateTimeLimitActionsState,
     /// <p>The approximate amount of time, in seconds, that must pass with the job in the specified state before the action is taken. The minimum value is 600 (10 minutes) and the maximum value is 86,400 (24 hours).</p>
-    pub max_time_seconds: ::std::option::Option<i32>,
+    pub max_time_seconds: i32,
     /// <p>The action to take when a job is at the head of the job queue in the specified state for the specified period of time. For job queues connected to a <code>ECS</code>, <code>FARGATE</code> or <code>EKS</code> compute environment, the only supported value is <code>CANCEL</code>, which will cancel the job. For job queues connected to a <code>SAGEMAKER_TRAINING</code> service environment, the only supported value is <code>TERMINATE</code>, which will terminate the job.</p>
-    pub action: ::std::option::Option<super::super::types::JobStateTimeLimitActionsAction>,
+    pub action: super::super::types::JobStateTimeLimitActionsAction,
 }
 impl JobStateTimeLimitAction {
     /// <p>The reason to log for the action being taken.</p>
-    pub fn reason(&self) -> ::std::option::Option<&str> {
-        self.reason.as_deref()
+    pub fn reason(&self) -> &str {
+        use std::ops::Deref;
+        self.reason.deref()
     }
     /// <p>The state of the job needed to trigger the action. The only supported value is <code>RUNNABLE</code>.</p>
-    pub fn state(&self) -> ::std::option::Option<&super::super::types::JobStateTimeLimitActionsState> {
-        self.state.as_ref()
+    pub fn state(&self) -> &super::super::types::JobStateTimeLimitActionsState {
+        &self.state
     }
     /// <p>The approximate amount of time, in seconds, that must pass with the job in the specified state before the action is taken. The minimum value is 600 (10 minutes) and the maximum value is 86,400 (24 hours).</p>
-    pub fn max_time_seconds(&self) -> ::std::option::Option<i32> {
+    pub fn max_time_seconds(&self) -> i32 {
         self.max_time_seconds
     }
     /// <p>The action to take when a job is at the head of the job queue in the specified state for the specified period of time. For job queues connected to a <code>ECS</code>, <code>FARGATE</code> or <code>EKS</code> compute environment, the only supported value is <code>CANCEL</code>, which will cancel the job. For job queues connected to a <code>SAGEMAKER_TRAINING</code> service environment, the only supported value is <code>TERMINATE</code>, which will terminate the job.</p>
-    pub fn action(&self) -> ::std::option::Option<&super::super::types::JobStateTimeLimitActionsAction> {
-        self.action.as_ref()
+    pub fn action(&self) -> &super::super::types::JobStateTimeLimitActionsAction {
+        &self.action
     }
 }
 impl JobStateTimeLimitAction {
@@ -109,12 +110,37 @@
         &self.action
     }
     /// Consumes the builder and constructs a [`JobStateTimeLimitAction`](crate::types::JobStateTimeLimitAction).
-    pub fn build(self) -> super::super::types::JobStateTimeLimitAction {
-        super::super::types::JobStateTimeLimitAction {
-            reason: self.reason,
-            state: self.state,
-            max_time_seconds: self.max_time_seconds,
-            action: self.action,
-        }
+    /// This method will fail if any of the following fields are not set:
+    /// - [`reason`](crate::types::builders::JobStateTimeLimitActionBuilder::reason)
+    /// - [`state`](crate::types::builders::JobStateTimeLimitActionBuilder::state)
+    /// - [`max_time_seconds`](crate::types::builders::JobStateTimeLimitActionBuilder::max_time_seconds)
+    /// - [`action`](crate::types::builders::JobStateTimeLimitActionBuilder::action)
+    pub fn build(self) -> ::std::result::Result<super::super::types::JobStateTimeLimitAction, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::JobStateTimeLimitAction {
+            reason: self.reason.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "reason",
+                    "reason was not specified but it is required when building JobStateTimeLimitAction",
+                )
+            })?,
+            state: self.state.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "state",
+                    "state was not specified but it is required when building JobStateTimeLimitAction",
+                )
+            })?,
+            max_time_seconds: self.max_time_seconds.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "max_time_seconds",
+                    "max_time_seconds was not specified but it is required when building JobStateTimeLimitAction",
+                )
+            })?,
+            action: self.action.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "action",
+                    "action was not specified but it is required when building JobStateTimeLimitAction",
+                )
+            })?,
+        })
     }
 }
```

### `src/types/_job_summary.rs`

```diff
--- reference/src/types/_job_summary.rs
+++ generated/src/types/_job_summary.rs
@@ -7,9 +7,9 @@
     /// <p>The Amazon Resource Name (ARN) of the job.</p>
     pub job_arn: ::std::option::Option<::std::string::String>,
     /// <p>The job ID.</p>
-    pub job_id: ::std::option::Option<::std::string::String>,
+    pub job_id: ::std::string::String,
     /// <p>The job name.</p>
-    pub job_name: ::std::option::Option<::std::string::String>,
+    pub job_name: ::std::string::String,
     /// <p>The configured capacity usage information for this job, including the unit of measure and quantity of resources.</p>
     pub capacity_usage: ::std::option::Option<::std::vec::Vec<super::super::types::JobCapacityUsageSummary>>,
     /// <p>The Unix timestamp (in milliseconds) for when the job was created. For non-array jobs and parent array jobs, this is when the job entered the <code>SUBMITTED</code> state (at the time <a href="https://docs.aws.amazon.com/batch/latest/APIReference/API_SubmitJob.html">SubmitJob</a> was called). For array child jobs, this is when the child job was spawned by its parent and entered the <code>PENDING</code> state.</p>
@@ -43,12 +43,14 @@
         self.job_arn.as_deref()
     }
     /// <p>The job ID.</p>
-    pub fn job_id(&self) -> ::std::option::Option<&str> {
-        self.job_id.as_deref()
+    pub fn job_id(&self) -> &str {
+        use std::ops::Deref;
+        self.job_id.deref()
     }
     /// <p>The job name.</p>
-    pub fn job_name(&self) -> ::std::option::Option<&str> {
-        self.job_name.as_deref()
+    pub fn job_name(&self) -> &str {
+        use std::ops::Deref;
+        self.job_name.deref()
     }
     /// <p>The configured capacity usage information for this job, including the unit of measure and quantity of resources.</p>
     ///
@@ -356,11 +358,24 @@
         &self.job_definition
     }
     /// Consumes the builder and constructs a [`JobSummary`](crate::types::JobSummary).
-    pub fn build(self) -> super::super::types::JobSummary {
-        super::super::types::JobSummary {
+    /// This method will fail if any of the following fields are not set:
+    /// - [`job_id`](crate::types::builders::JobSummaryBuilder::job_id)
+    /// - [`job_name`](crate::types::builders::JobSummaryBuilder::job_name)
+    pub fn build(self) -> ::std::result::Result<super::super::types::JobSummary, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::JobSummary {
             job_arn: self.job_arn,
-            job_id: self.job_id,
-            job_name: self.job_name,
+            job_id: self.job_id.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "job_id",
+                    "job_id was not specified but it is required when building JobSummary",
+                )
+            })?,
+            job_name: self.job_name.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "job_name",
+                    "job_name was not specified but it is required when building JobSummary",
+                )
+            })?,
             capacity_usage: self.capacity_usage,
             created_at: self.created_at,
             scheduled_at: self.scheduled_at,
@@ -373,6 +388,6 @@
             array_properties: self.array_properties,
             node_properties: self.node_properties,
             job_definition: self.job_definition,
-        }
+        })
     }
 }
```

### `src/types/_list_jobs_by_consumable_resource_summary.rs`

```diff
--- reference/src/types/_list_jobs_by_consumable_resource_summary.rs
+++ generated/src/types/_list_jobs_by_consumable_resource_summary.rs
@@ -5,11 +5,11 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct ListJobsByConsumableResourceSummary {
     /// <p>The Amazon Resource Name (ARN) of the job.</p>
-    pub job_arn: ::std::option::Option<::std::string::String>,
+    pub job_arn: ::std::string::String,
     /// <p>The Amazon Resource Name (ARN) of the job queue.</p>
-    pub job_queue_arn: ::std::option::Option<::std::string::String>,
+    pub job_queue_arn: ::std::string::String,
     /// <p>The name of the job.</p>
-    pub job_name: ::std::option::Option<::std::string::String>,
+    pub job_name: ::std::string::String,
     /// <p>The Amazon Resource Name (ARN) of the job definition.</p>
     pub job_definition_arn: ::std::option::Option<::std::string::String>,
     /// <p>The fair-share scheduling identifier for the job.</p>
@@ -31,30 +31,33 @@
     /// <li>
     /// <p><code>FAILED</code></p></li>
     /// </ul>
-    pub job_status: ::std::option::Option<::std::string::String>,
+    pub job_status: ::std::string::String,
     /// <p>The total amount of the consumable resource that is available.</p>
-    pub quantity: ::std::option::Option<i64>,
+    pub quantity: i64,
     /// <p>A short, human-readable string to provide more details for the current status of the job.</p>
     pub status_reason: ::std::option::Option<::std::string::String>,
     /// <p>The Unix timestamp for when the job was started. More specifically, it's when the job transitioned from the <code>STARTING</code> state to the <code>RUNNING</code> state.</p>
     pub started_at: ::std::option::Option<i64>,
     /// <p>The Unix timestamp (in milliseconds) for when the consumable resource was created.</p>
-    pub created_at: ::std::option::Option<i64>,
+    pub created_at: i64,
     /// <p>Contains a list of consumable resources required by the job.</p>
     pub consumable_resource_properties: ::std::option::Option<super::super::types::ConsumableResourceProperties>,
 }
 impl ListJobsByConsumableResourceSummary {
     /// <p>The Amazon Resource Name (ARN) of the job.</p>
-    pub fn job_arn(&self) -> ::std::option::Option<&str> {
-        self.job_arn.as_deref()
+    pub fn job_arn(&self) -> &str {
+        use std::ops::Deref;
+        self.job_arn.deref()
     }
     /// <p>The Amazon Resource Name (ARN) of the job queue.</p>
-    pub fn job_queue_arn(&self) -> ::std::option::Option<&str> {
-        self.job_queue_arn.as_deref()
+    pub fn job_queue_arn(&self) -> &str {
+        use std::ops::Deref;
+        self.job_queue_arn.deref()
     }
     /// <p>The name of the job.</p>
-    pub fn job_name(&self) -> ::std::option::Option<&str> {
-        self.job_name.as_deref()
+    pub fn job_name(&self) -> &str {
+        use std::ops::Deref;
+        self.job_name.deref()
     }
     /// <p>The Amazon Resource Name (ARN) of the job definition.</p>
     pub fn job_definition_arn(&self) -> ::std::option::Option<&str> {
@@ -81,11 +84,12 @@
     /// <li>
     /// <p><code>FAILED</code></p></li>
     /// </ul>
-    pub fn job_status(&self) -> ::std::option::Option<&str> {
-        self.job_status.as_deref()
+    pub fn job_status(&self) -> &str {
+        use std::ops::Deref;
+        self.job_status.deref()
     }
     /// <p>The total amount of the consumable resource that is available.</p>
-    pub fn quantity(&self) -> ::std::option::Option<i64> {
+    pub fn quantity(&self) -> i64 {
         self.quantity
     }
     /// <p>A short, human-readable string to provide more details for the current status of the job.</p>
@@ -97,7 +101,7 @@
         self.started_at
     }
     /// <p>The Unix timestamp (in milliseconds) for when the consumable resource was created.</p>
-    pub fn created_at(&self) -> ::std::option::Option<i64> {
+    pub fn created_at(&self) -> i64 {
         self.created_at
     }
     /// <p>Contains a list of consumable resources required by the job.</p>
@@ -339,19 +343,56 @@
         &self.consumable_resource_properties
     }
     /// Consumes the builder and constructs a [`ListJobsByConsumableResourceSummary`](crate::types::ListJobsByConsumableResourceSummary).
-    pub fn build(self) -> super::super::types::ListJobsByConsumableResourceSummary {
-        super::super::types::ListJobsByConsumableResourceSummary {
-            job_arn: self.job_arn,
-            job_queue_arn: self.job_queue_arn,
-            job_name: self.job_name,
+    /// This method will fail if any of the following fields are not set:
+    /// - [`job_arn`](crate::types::builders::ListJobsByConsumableResourceSummaryBuilder::job_arn)
+    /// - [`job_queue_arn`](crate::types::builders::ListJobsByConsumableResourceSummaryBuilder::job_queue_arn)
+    /// - [`job_name`](crate::types::builders::ListJobsByConsumableResourceSummaryBuilder::job_name)
+    /// - [`job_status`](crate::types::builders::ListJobsByConsumableResourceSummaryBuilder::job_status)
+    /// - [`quantity`](crate::types::builders::ListJobsByConsumableResourceSummaryBuilder::quantity)
+    /// - [`created_at`](crate::types::builders::ListJobsByConsumableResourceSummaryBuilder::created_at)
+    pub fn build(self) -> ::std::result::Result<super::super::types::ListJobsByConsumableResourceSummary, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::ListJobsByConsumableResourceSummary {
+            job_arn: self.job_arn.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "job_arn",
+                    "job_arn was not specified but it is required when building ListJobsByConsumableResourceSummary",
+                )
+            })?,
+            job_queue_arn: self.job_queue_arn.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "job_queue_arn",
+                    "job_queue_arn was not specified but it is required when building ListJobsByConsumableResourceSummary",
+                )
+            })?,
+            job_name: self.job_name.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "job_name",
+                    "job_name was not specified but it is required when building ListJobsByConsumableResourceSummary",
+                )
+            })?,
             job_definition_arn: self.job_definition_arn,
             share_identifier: self.share_identifier,
-            job_status: self.job_status,
-            quantity: self.quantity,
+            job_status: self.job_status.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "job_status",
+                    "job_status was not specified but it is required when building ListJobsByConsumableResourceSummary",
+                )
+            })?,
+            quantity: self.quantity.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "quantity",
+                    "quantity was not specified but it is required when building ListJobsByConsumableResourceSummary",
+                )
+            })?,
             status_reason: self.status_reason,
             started_at: self.started_at,
-            created_at: self.created_at,
+            created_at: self.created_at.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "created_at",
+                    "created_at was not specified but it is required when building ListJobsByConsumableResourceSummary",
+                )
+            })?,
             consumable_resource_properties: self.consumable_resource_properties,
-        }
+        })
     }
 }
```

### `src/types/_log_configuration.rs`

```diff
--- reference/src/types/_log_configuration.rs
+++ generated/src/types/_log_configuration.rs
@@ -61,7 +61,7 @@
     /// <p>If you have a custom driver that's not listed earlier that you want to work with the Amazon ECS container agent, you can fork the Amazon ECS container agent project that's <a href="https://github.com/aws/amazon-ecs-agent">available on GitHub</a> and customize it to work with that driver. We encourage you to submit pull requests for changes that you want to have included. However, Amazon Web Services doesn't currently support running modified copies of this software.</p>
     /// </note>
     /// <p>This parameter requires version 1.18 of the Docker Remote API or greater on your container instance. To check the Docker Remote API version on your container instance, log in to your container instance and run the following command: <code>sudo docker version | grep "Server API version"</code></p>
-    pub log_driver: ::std::option::Option<super::super::types::LogDriver>,
+    pub log_driver: super::super::types::LogDriver,
     /// <p>The configuration options to send to the log driver. This parameter requires version 1.19 of the Docker Remote API or greater on your container instance. To check the Docker Remote API version on your container instance, log in to your container instance and run the following command: <code>sudo docker version | grep "Server API version"</code></p>
     pub options: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
     /// <p>The secrets to pass to the log configuration. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/specifying-sensitive-data.html">Specifying sensitive data</a> in the <i>Batch User Guide</i>.</p>
@@ -125,8 +125,8 @@
     /// <p>If you have a custom driver that's not listed earlier that you want to work with the Amazon ECS container agent, you can fork the Amazon ECS container agent project that's <a href="https://github.com/aws/amazon-ecs-agent">available on GitHub</a> and customize it to work with that driver. We encourage you to submit pull requests for changes that you want to have included. However, Amazon Web Services doesn't currently support running modified copies of this software.</p>
     /// </note>
     /// <p>This parameter requires version 1.18 of the Docker Remote API or greater on your container instance. To check the Docker Remote API version on your container instance, log in to your container instance and run the following command: <code>sudo docker version | grep "Server API version"</code></p>
-    pub fn log_driver(&self) -> ::std::option::Option<&super::super::types::LogDriver> {
-        self.log_driver.as_ref()
+    pub fn log_driver(&self) -> &super::super::types::LogDriver {
+        &self.log_driver
     }
     /// <p>The configuration options to send to the log driver. This parameter requires version 1.19 of the Docker Remote API or greater on your container instance. To check the Docker Remote API version on your container instance, log in to your container instance and run the following command: <code>sudo docker version | grep "Server API version"</code></p>
     pub fn options(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, ::std::string::String>> {
@@ -379,11 +379,18 @@
         &self.secret_options
     }
     /// Consumes the builder and constructs a [`LogConfiguration`](crate::types::LogConfiguration).
-    pub fn build(self) -> super::super::types::LogConfiguration {
-        super::super::types::LogConfiguration {
-            log_driver: self.log_driver,
+    /// This method will fail if any of the following fields are not set:
+    /// - [`log_driver`](crate::types::builders::LogConfigurationBuilder::log_driver)
+    pub fn build(self) -> ::std::result::Result<super::super::types::LogConfiguration, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::LogConfiguration {
+            log_driver: self.log_driver.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "log_driver",
+                    "log_driver was not specified but it is required when building LogConfiguration",
+                )
+            })?,
             options: self.options,
             secret_options: self.secret_options,
-        }
+        })
     }
 }
```

### `src/types/_managed_instances_network_configuration.rs`

```diff
--- reference/src/types/_managed_instances_network_configuration.rs
+++ generated/src/types/_managed_instances_network_configuration.rs
@@ -5,22 +5,20 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct ManagedInstancesNetworkConfiguration {
     /// <p>The VPC subnets where managed instances are launched. If your subnets don't provide public IP addresses, they must have a NAT gateway for outbound internet access.</p>
-    pub subnets: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
+    pub subnets: ::std::vec::Vec<::std::string::String>,
     /// <p>The VPC security groups to associate with the managed instances.</p>
-    pub security_groups: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
+    pub security_groups: ::std::vec::Vec<::std::string::String>,
 }
 impl ManagedInstancesNetworkConfiguration {
     /// <p>The VPC subnets where managed instances are launched. If your subnets don't provide public IP addresses, they must have a NAT gateway for outbound internet access.</p>
-    ///
-    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.subnets.is_none()`.
     pub fn subnets(&self) -> &[::std::string::String] {
-        self.subnets.as_deref().unwrap_or_default()
+        use std::ops::Deref;
+        self.subnets.deref()
     }
     /// <p>The VPC security groups to associate with the managed instances.</p>
-    ///
-    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.security_groups.is_none()`.
     pub fn security_groups(&self) -> &[::std::string::String] {
-        self.security_groups.as_deref().unwrap_or_default()
+        use std::ops::Deref;
+        self.security_groups.deref()
     }
 }
 impl ManagedInstancesNetworkConfiguration {
@@ -79,10 +77,25 @@
         &self.security_groups
     }
     /// Consumes the builder and constructs a [`ManagedInstancesNetworkConfiguration`](crate::types::ManagedInstancesNetworkConfiguration).
-    pub fn build(self) -> super::super::types::ManagedInstancesNetworkConfiguration {
-        super::super::types::ManagedInstancesNetworkConfiguration {
-            subnets: self.subnets,
-            security_groups: self.security_groups,
-        }
+    /// This method will fail if any of the following fields are not set:
+    /// - [`subnets`](crate::types::builders::ManagedInstancesNetworkConfigurationBuilder::subnets)
+    /// - [`security_groups`](crate::types::builders::ManagedInstancesNetworkConfigurationBuilder::security_groups)
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::types::ManagedInstancesNetworkConfiguration, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::ManagedInstancesNetworkConfiguration {
+            subnets: self.subnets.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "subnets",
+                    "subnets was not specified but it is required when building ManagedInstancesNetworkConfiguration",
+                )
+            })?,
+            security_groups: self.security_groups.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "security_groups",
+                    "security_groups was not specified but it is required when building ManagedInstancesNetworkConfiguration",
+                )
+            })?,
+        })
     }
 }
```

### `src/types/_managed_instances_provider.rs`

```diff
--- reference/src/types/_managed_instances_provider.rs
+++ generated/src/types/_managed_instances_provider.rs
@@ -13,7 +13,7 @@
     /// </ul>
     pub propagate_tags: ::std::option::Option<::std::string::String>,
     /// <p>The Amazon Resource Name (ARN) of the IAM role that Amazon ECS assumes to manage Amazon EC2 instances on your behalf. This role must have a trust policy for <code>ecs.amazonaws.com</code>. You must have the <code>iam:PassRole</code> permission for this role with the condition <code>iam:PassedToService: ecs.amazonaws.com</code>.</p>
-    pub infrastructure_role_arn: ::std::option::Option<::std::string::String>,
+    pub infrastructure_role_arn: ::std::string::String,
     /// <p>The instance launch configuration for the Amazon ECS Managed Instances capacity provider. Contains networking, instance profile, instance requirements, capacity type, storage, and monitoring configuration.</p>
     pub instance_launch_template: ::std::option::Option<super::super::types::InstanceLaunchTemplate>,
     /// <p>The infrastructure optimization configuration for the capacity provider. Specifies the idle-instance scale-in behavior.</p>
@@ -31,8 +31,9 @@
         self.propagate_tags.as_deref()
     }
     /// <p>The Amazon Resource Name (ARN) of the IAM role that Amazon ECS assumes to manage Amazon EC2 instances on your behalf. This role must have a trust policy for <code>ecs.amazonaws.com</code>. You must have the <code>iam:PassRole</code> permission for this role with the condition <code>iam:PassedToService: ecs.amazonaws.com</code>.</p>
-    pub fn infrastructure_role_arn(&self) -> ::std::option::Option<&str> {
-        self.infrastructure_role_arn.as_deref()
+    pub fn infrastructure_role_arn(&self) -> &str {
+        use std::ops::Deref;
+        self.infrastructure_role_arn.deref()
     }
     /// <p>The instance launch configuration for the Amazon ECS Managed Instances capacity provider. Contains networking, instance profile, instance requirements, capacity type, storage, and monitoring configuration.</p>
     pub fn instance_launch_template(&self) -> ::std::option::Option<&super::super::types::InstanceLaunchTemplate> {
@@ -137,12 +138,19 @@
         &self.infrastructure_optimization
     }
     /// Consumes the builder and constructs a [`ManagedInstancesProvider`](crate::types::ManagedInstancesProvider).
-    pub fn build(self) -> super::super::types::ManagedInstancesProvider {
-        super::super::types::ManagedInstancesProvider {
+    /// This method will fail if any of the following fields are not set:
+    /// - [`infrastructure_role_arn`](crate::types::builders::ManagedInstancesProviderBuilder::infrastructure_role_arn)
+    pub fn build(self) -> ::std::result::Result<super::super::types::ManagedInstancesProvider, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::ManagedInstancesProvider {
             propagate_tags: self.propagate_tags,
-            infrastructure_role_arn: self.infrastructure_role_arn,
+            infrastructure_role_arn: self.infrastructure_role_arn.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "infrastructure_role_arn",
+                    "infrastructure_role_arn was not specified but it is required when building ManagedInstancesProvider",
+                )
+            })?,
             instance_launch_template: self.instance_launch_template,
             infrastructure_optimization: self.infrastructure_optimization,
-        }
+        })
     }
 }
```

### `src/types/_managed_instances_storage_configuration.rs`

```diff
--- reference/src/types/_managed_instances_storage_configuration.rs
+++ generated/src/types/_managed_instances_storage_configuration.rs
@@ -5,12 +5,12 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct ManagedInstancesStorageConfiguration {
     /// <p>The size of the root EBS volume in GiB for the managed instances.</p>
-    pub storage_size_gib: ::std::option::Option<i32>,
+    pub storage_size_gi_b: ::std::option::Option<i32>,
 }
 impl ManagedInstancesStorageConfiguration {
     /// <p>The size of the root EBS volume in GiB for the managed instances.</p>
-    pub fn storage_size_gib(&self) -> ::std::option::Option<i32> {
-        self.storage_size_gib
+    pub fn storage_size_gi_b(&self) -> ::std::option::Option<i32> {
+        self.storage_size_gi_b
     }
 }
 impl ManagedInstancesStorageConfiguration {
@@ -24,27 +24,27 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
 pub struct ManagedInstancesStorageConfigurationBuilder {
-    pub(crate) storage_size_gib: ::std::option::Option<i32>,
+    pub(crate) storage_size_gi_b: ::std::option::Option<i32>,
 }
 impl ManagedInstancesStorageConfigurationBuilder {
     /// <p>The size of the root EBS volume in GiB for the managed instances.</p>
-    pub fn storage_size_gib(mut self, input: i32) -> Self {
-        self.storage_size_gib = ::std::option::Option::Some(input);
+    pub fn storage_size_gi_b(mut self, input: i32) -> Self {
+        self.storage_size_gi_b = ::std::option::Option::Some(input);
         self
     }
     /// <p>The size of the root EBS volume in GiB for the managed instances.</p>
-    pub fn set_storage_size_gib(mut self, input: ::std::option::Option<i32>) -> Self {
-        self.storage_size_gib = input;
+    pub fn set_storage_size_gi_b(mut self, input: ::std::option::Option<i32>) -> Self {
+        self.storage_size_gi_b = input;
         self
     }
     /// <p>The size of the root EBS volume in GiB for the managed instances.</p>
-    pub fn get_storage_size_gib(&self) -> &::std::option::Option<i32> {
-        &self.storage_size_gib
+    pub fn get_storage_size_gi_b(&self) -> &::std::option::Option<i32> {
+        &self.storage_size_gi_b
     }
     /// Consumes the builder and constructs a [`ManagedInstancesStorageConfiguration`](crate::types::ManagedInstancesStorageConfiguration).
     pub fn build(self) -> super::super::types::ManagedInstancesStorageConfiguration {
         super::super::types::ManagedInstancesStorageConfiguration {
-            storage_size_gib: self.storage_size_gib,
+            storage_size_gi_b: self.storage_size_gi_b,
         }
     }
 }
```

### `src/types/_node_properties.rs`

```diff
--- reference/src/types/_node_properties.rs
+++ generated/src/types/_node_properties.rs
@@ -7,26 +7,25 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct NodeProperties {
     /// <p>The number of nodes that are associated with a multi-node parallel job.</p>
-    pub num_nodes: ::std::option::Option<i32>,
+    pub num_nodes: i32,
     /// <p>Specifies the node index for the main node of a multi-node parallel job. This node index value must be fewer than the number of nodes.</p>
-    pub main_node: ::std::option::Option<i32>,
+    pub main_node: i32,
     /// <p>A list of node ranges and their properties that are associated with a multi-node parallel job.</p>
-    pub node_range_properties: ::std::option::Option<::std::vec::Vec<super::super::types::NodeRangeProperty>>,
+    pub node_range_properties: ::std::vec::Vec<super::super::types::NodeRangeProperty>,
 }
 impl NodeProperties {
     /// <p>The number of nodes that are associated with a multi-node parallel job.</p>
-    pub fn num_nodes(&self) -> ::std::option::Option<i32> {
+    pub fn num_nodes(&self) -> i32 {
         self.num_nodes
     }
     /// <p>Specifies the node index for the main node of a multi-node parallel job. This node index value must be fewer than the number of nodes.</p>
-    pub fn main_node(&self) -> ::std::option::Option<i32> {
+    pub fn main_node(&self) -> i32 {
         self.main_node
     }
     /// <p>A list of node ranges and their properties that are associated with a multi-node parallel job.</p>
-    ///
-    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.node_range_properties.is_none()`.
     pub fn node_range_properties(&self) -> &[super::super::types::NodeRangeProperty] {
-        self.node_range_properties.as_deref().unwrap_or_default()
+        use std::ops::Deref;
+        self.node_range_properties.deref()
     }
 }
 impl NodeProperties {
@@ -96,11 +95,30 @@
         &self.node_range_properties
     }
     /// Consumes the builder and constructs a [`NodeProperties`](crate::types::NodeProperties).
-    pub fn build(self) -> super::super::types::NodeProperties {
-        super::super::types::NodeProperties {
-            num_nodes: self.num_nodes,
-            main_node: self.main_node,
-            node_range_properties: self.node_range_properties,
-        }
+    /// This method will fail if any of the following fields are not set:
+    /// - [`num_nodes`](crate::types::builders::NodePropertiesBuilder::num_nodes)
+    /// - [`main_node`](crate::types::builders::NodePropertiesBuilder::main_node)
+    /// - [`node_range_properties`](crate::types::builders::NodePropertiesBuilder::node_range_properties)
+    pub fn build(self) -> ::std::result::Result<super::super::types::NodeProperties, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::NodeProperties {
+            num_nodes: self.num_nodes.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "num_nodes",
+                    "num_nodes was not specified but it is required when building NodeProperties",
+                )
+            })?,
+            main_node: self.main_node.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "main_node",
+                    "main_node was not specified but it is required when building NodeProperties",
+                )
+            })?,
+            node_range_properties: self.node_range_properties.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "node_range_properties",
+                    "node_range_properties was not specified but it is required when building NodeProperties",
+                )
+            })?,
+        })
     }
 }
```

### `src/types/_node_property_override.rs`

```diff
--- reference/src/types/_node_property_override.rs
+++ generated/src/types/_node_property_override.rs
@@ -5,7 +5,7 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct NodePropertyOverride {
     /// <p>The range of nodes, using node index values, that's used to override. A range of <code>0:3</code> indicates nodes with index values of <code>0</code> through <code>3</code>. If the starting range value is omitted (<code>:n</code>), then <code>0</code> is used to start the range. If the ending range value is omitted (<code>n:</code>), then the highest possible node index is used to end the range.</p>
-    pub target_nodes: ::std::option::Option<::std::string::String>,
+    pub target_nodes: ::std::string::String,
     /// <p>The overrides that are sent to a node range.</p>
     pub container_overrides: ::std::option::Option<super::super::types::ContainerOverrides>,
     /// <p>An object that contains the properties that you want to replace for the existing Amazon ECS resources of a job.</p>
@@ -19,8 +19,9 @@
 }
 impl NodePropertyOverride {
     /// <p>The range of nodes, using node index values, that's used to override. A range of <code>0:3</code> indicates nodes with index values of <code>0</code> through <code>3</code>. If the starting range value is omitted (<code>:n</code>), then <code>0</code> is used to start the range. If the ending range value is omitted (<code>n:</code>), then the highest possible node index is used to end the range.</p>
-    pub fn target_nodes(&self) -> ::std::option::Option<&str> {
-        self.target_nodes.as_deref()
+    pub fn target_nodes(&self) -> &str {
+        use std::ops::Deref;
+        self.target_nodes.deref()
     }
     /// <p>The overrides that are sent to a node range.</p>
     pub fn container_overrides(&self) -> ::std::option::Option<&super::super::types::ContainerOverrides> {
@@ -156,14 +157,21 @@
         &self.consumable_resource_properties_override
     }
     /// Consumes the builder and constructs a [`NodePropertyOverride`](crate::types::NodePropertyOverride).
-    pub fn build(self) -> super::super::types::NodePropertyOverride {
-        super::super::types::NodePropertyOverride {
-            target_nodes: self.target_nodes,
+    /// This method will fail if any of the following fields are not set:
+    /// - [`target_nodes`](crate::types::builders::NodePropertyOverrideBuilder::target_nodes)
+    pub fn build(self) -> ::std::result::Result<super::super::types::NodePropertyOverride, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::NodePropertyOverride {
+            target_nodes: self.target_nodes.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "target_nodes",
+                    "target_nodes was not specified but it is required when building NodePropertyOverride",
+                )
+            })?,
             container_overrides: self.container_overrides,
             ecs_properties_override: self.ecs_properties_override,
             instance_types: self.instance_types,
             eks_properties_override: self.eks_properties_override,
             consumable_resource_properties_override: self.consumable_resource_properties_override,
-        }
+        })
     }
 }
```

### `src/types/_node_range_property.rs`

```diff
--- reference/src/types/_node_range_property.rs
+++ generated/src/types/_node_range_property.rs
@@ -5,7 +5,7 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct NodeRangeProperty {
     /// <p>The range of nodes, using node index values. A range of <code>0:3</code> indicates nodes with index values of <code>0</code> through <code>3</code>. If the starting range value is omitted (<code>:n</code>), then <code>0</code> is used to start the range. If the ending range value is omitted (<code>n:</code>), then the highest possible node index is used to end the range. Your accumulative node ranges must account for all nodes (<code>0:n</code>). You can nest node ranges (for example, <code>0:10</code> and <code>4:5</code>). In this case, the <code>4:5</code> range properties override the <code>0:10</code> properties.</p>
-    pub target_nodes: ::std::option::Option<::std::string::String>,
+    pub target_nodes: ::std::string::String,
     /// <p>The container details for the node range.</p>
     pub container: ::std::option::Option<super::super::types::ContainerProperties>,
     /// <p>The instance types of the underlying host infrastructure of a multi-node parallel job.</p><note>
@@ -22,8 +22,9 @@
 }
 impl NodeRangeProperty {
     /// <p>The range of nodes, using node index values. A range of <code>0:3</code> indicates nodes with index values of <code>0</code> through <code>3</code>. If the starting range value is omitted (<code>:n</code>), then <code>0</code> is used to start the range. If the ending range value is omitted (<code>n:</code>), then the highest possible node index is used to end the range. Your accumulative node ranges must account for all nodes (<code>0:n</code>). You can nest node ranges (for example, <code>0:10</code> and <code>4:5</code>). In this case, the <code>4:5</code> range properties override the <code>0:10</code> properties.</p>
-    pub fn target_nodes(&self) -> ::std::option::Option<&str> {
-        self.target_nodes.as_deref()
+    pub fn target_nodes(&self) -> &str {
+        use std::ops::Deref;
+        self.target_nodes.deref()
     }
     /// <p>The container details for the node range.</p>
     pub fn container(&self) -> ::std::option::Option<&super::super::types::ContainerProperties> {
@@ -171,14 +172,21 @@
         &self.consumable_resource_properties
     }
     /// Consumes the builder and constructs a [`NodeRangeProperty`](crate::types::NodeRangeProperty).
-    pub fn build(self) -> super::super::types::NodeRangeProperty {
-        super::super::types::NodeRangeProperty {
-            target_nodes: self.target_nodes,
+    /// This method will fail if any of the following fields are not set:
+    /// - [`target_nodes`](crate::types::builders::NodeRangePropertyBuilder::target_nodes)
+    pub fn build(self) -> ::std::result::Result<super::super::types::NodeRangeProperty, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::NodeRangeProperty {
+            target_nodes: self.target_nodes.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "target_nodes",
+                    "target_nodes was not specified but it is required when building NodeRangeProperty",
+                )
+            })?,
             container: self.container,
             instance_types: self.instance_types,
             ecs_properties: self.ecs_properties,
             eks_properties: self.eks_properties,
             consumable_resource_properties: self.consumable_resource_properties,
-        }
+        })
     }
 }
```

### `src/types/_quota_share_capacity_limit.rs`

```diff
--- reference/src/types/_quota_share_capacity_limit.rs
+++ generated/src/types/_quota_share_capacity_limit.rs
@@ -5,18 +5,19 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct QuotaShareCapacityLimit {
     /// <p>The maximum capacity available for the quota share. This value represents the maximum quantity of a resource that can be allocated to jobs in the quota share without borrowing.</p>
-    pub max_capacity: ::std::option::Option<i32>,
+    pub max_capacity: i32,
     /// <p>The unit of compute capacity for the capacityLimit. For example, <code>ml.m5.large</code>.</p>
-    pub capacity_unit: ::std::option::Option<::std::string::String>,
+    pub capacity_unit: ::std::string::String,
 }
 impl QuotaShareCapacityLimit {
     /// <p>The maximum capacity available for the quota share. This value represents the maximum quantity of a resource that can be allocated to jobs in the quota share without borrowing.</p>
-    pub fn max_capacity(&self) -> ::std::option::Option<i32> {
+    pub fn max_capacity(&self) -> i32 {
         self.max_capacity
     }
     /// <p>The unit of compute capacity for the capacityLimit. For example, <code>ml.m5.large</code>.</p>
-    pub fn capacity_unit(&self) -> ::std::option::Option<&str> {
-        self.capacity_unit.as_deref()
+    pub fn capacity_unit(&self) -> &str {
+        use std::ops::Deref;
+        self.capacity_unit.deref()
     }
 }
 impl QuotaShareCapacityLimit {
@@ -65,10 +66,23 @@
         &self.capacity_unit
     }
     /// Consumes the builder and constructs a [`QuotaShareCapacityLimit`](crate::types::QuotaShareCapacityLimit).
-    pub fn build(self) -> super::super::types::QuotaShareCapacityLimit {
-        super::super::types::QuotaShareCapacityLimit {
-            max_capacity: self.max_capacity,
-            capacity_unit: self.capacity_unit,
-        }
+    /// This method will fail if any of the following fields are not set:
+    /// - [`max_capacity`](crate::types::builders::QuotaShareCapacityLimitBuilder::max_capacity)
+    /// - [`capacity_unit`](crate::types::builders::QuotaShareCapacityLimitBuilder::capacity_unit)
+    pub fn build(self) -> ::std::result::Result<super::super::types::QuotaShareCapacityLimit, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::QuotaShareCapacityLimit {
+            max_capacity: self.max_capacity.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "max_capacity",
+                    "max_capacity was not specified but it is required when building QuotaShareCapacityLimit",
+                )
+            })?,
+            capacity_unit: self.capacity_unit.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "capacity_unit",
+                    "capacity_unit was not specified but it is required when building QuotaShareCapacityLimit",
+                )
+            })?,
+        })
     }
 }
```

### `src/types/_quota_share_policy.rs`

```diff
--- reference/src/types/_quota_share_policy.rs
+++ generated/src/types/_quota_share_policy.rs
@@ -5,12 +5,12 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct QuotaSharePolicy {
     /// <p>The strategy that determines how idle resources are assigned to quota shares that are borrowing capacity. Currently, only <code>FIFO</code> is supported.</p>
-    pub idle_resource_assignment_strategy: ::std::option::Option<super::super::types::QuotaShareIdleResourceAssignmentStrategy>,
+    pub idle_resource_assignment_strategy: super::super::types::QuotaShareIdleResourceAssignmentStrategy,
 }
 impl QuotaSharePolicy {
     /// <p>The strategy that determines how idle resources are assigned to quota shares that are borrowing capacity. Currently, only <code>FIFO</code> is supported.</p>
-    pub fn idle_resource_assignment_strategy(&self) -> ::std::option::Option<&super::super::types::QuotaShareIdleResourceAssignmentStrategy> {
-        self.idle_resource_assignment_strategy.as_ref()
+    pub fn idle_resource_assignment_strategy(&self) -> &super::super::types::QuotaShareIdleResourceAssignmentStrategy {
+        &self.idle_resource_assignment_strategy
     }
 }
 impl QuotaSharePolicy {
@@ -46,9 +46,16 @@
         &self.idle_resource_assignment_strategy
     }
     /// Consumes the builder and constructs a [`QuotaSharePolicy`](crate::types::QuotaSharePolicy).
-    pub fn build(self) -> super::super::types::QuotaSharePolicy {
-        super::super::types::QuotaSharePolicy {
-            idle_resource_assignment_strategy: self.idle_resource_assignment_strategy,
-        }
+    /// This method will fail if any of the following fields are not set:
+    /// - [`idle_resource_assignment_strategy`](crate::types::builders::QuotaSharePolicyBuilder::idle_resource_assignment_strategy)
+    pub fn build(self) -> ::std::result::Result<super::super::types::QuotaSharePolicy, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::QuotaSharePolicy {
+            idle_resource_assignment_strategy: self.idle_resource_assignment_strategy.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "idle_resource_assignment_strategy",
+                    "idle_resource_assignment_strategy was not specified but it is required when building QuotaSharePolicy",
+                )
+            })?,
+        })
     }
 }
```

### `src/types/_quota_share_preemption_configuration.rs`

```diff
--- reference/src/types/_quota_share_preemption_configuration.rs
+++ generated/src/types/_quota_share_preemption_configuration.rs
@@ -5,12 +5,12 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct QuotaSharePreemptionConfiguration {
     /// <p>Specifies whether jobs within a quota share can be preempted by another, higher priority job in the same quota share.</p>
-    pub in_share_preemption: ::std::option::Option<super::super::types::QuotaShareInSharePreemptionState>,
+    pub in_share_preemption: super::super::types::QuotaShareInSharePreemptionState,
 }
 impl QuotaSharePreemptionConfiguration {
     /// <p>Specifies whether jobs within a quota share can be preempted by another, higher priority job in the same quota share.</p>
-    pub fn in_share_preemption(&self) -> ::std::option::Option<&super::super::types::QuotaShareInSharePreemptionState> {
-        self.in_share_preemption.as_ref()
+    pub fn in_share_preemption(&self) -> &super::super::types::QuotaShareInSharePreemptionState {
+        &self.in_share_preemption
     }
 }
 impl QuotaSharePreemptionConfiguration {
@@ -43,9 +43,16 @@
         &self.in_share_preemption
     }
     /// Consumes the builder and constructs a [`QuotaSharePreemptionConfiguration`](crate::types::QuotaSharePreemptionConfiguration).
-    pub fn build(self) -> super::super::types::QuotaSharePreemptionConfiguration {
-        super::super::types::QuotaSharePreemptionConfiguration {
-            in_share_preemption: self.in_share_preemption,
-        }
+    /// This method will fail if any of the following fields are not set:
+    /// - [`in_share_preemption`](crate::types::builders::QuotaSharePreemptionConfigurationBuilder::in_share_preemption)
+    pub fn build(self) -> ::std::result::Result<super::super::types::QuotaSharePreemptionConfiguration, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::QuotaSharePreemptionConfiguration {
+            in_share_preemption: self.in_share_preemption.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "in_share_preemption",
+                    "in_share_preemption was not specified but it is required when building QuotaSharePreemptionConfiguration",
+                )
+            })?,
+        })
     }
 }
```

### `src/types/_quota_share_resource_sharing_configuration.rs`

```diff
--- reference/src/types/_quota_share_resource_sharing_configuration.rs
+++ generated/src/types/_quota_share_resource_sharing_configuration.rs
@@ -5,7 +5,7 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct QuotaShareResourceSharingConfiguration {
     /// <p>The resource sharing strategy for the quota share. The <code>RESERVE</code> strategy allows a quota share to reserve idle capacity for itself. <code>LEND</code> configures the share to lend its idle capacity to another share in need of capacity. The <code>LEND_AND_BORROW</code> strategy configures the share to borrow idle capacity from an underutilized share, as well as lend to another share.</p>
-    pub strategy: ::std::option::Option<super::super::types::QuotaShareResourceSharingStrategy>,
+    pub strategy: super::super::types::QuotaShareResourceSharingStrategy,
     /// <p>The maximum percentage of additional capacity that the quota share can borrow from other shares. <code>borrowLimit</code> can only be applied to quota shares with a strategy of <code>LEND_AND_BORROW</code>. This value is expressed as a percentage of the quota share's configured <a href="https://docs.aws.amazon.com/batch/latest/APIReference/API_QuotaShareCapacityLimit.html">CapacityLimits</a>.</p>
     /// <p>The <code>borrowLimit</code> is applied uniformly across all capacity units. For example, if the <code>borrowLimit</code> is 200, the quota share can borrow up to 200% of its configured <code>maxCapacity</code> for each capacity unit. The default <code>borrowLimit</code> is -1, which indicates unlimited borrowing.</p>
     pub borrow_limit: ::std::option::Option<i32>,
@@ -12,8 +12,8 @@
 }
 impl QuotaShareResourceSharingConfiguration {
     /// <p>The resource sharing strategy for the quota share. The <code>RESERVE</code> strategy allows a quota share to reserve idle capacity for itself. <code>LEND</code> configures the share to lend its idle capacity to another share in need of capacity. The <code>LEND_AND_BORROW</code> strategy configures the share to borrow idle capacity from an underutilized share, as well as lend to another share.</p>
-    pub fn strategy(&self) -> ::std::option::Option<&super::super::types::QuotaShareResourceSharingStrategy> {
-        self.strategy.as_ref()
+    pub fn strategy(&self) -> &super::super::types::QuotaShareResourceSharingStrategy {
+        &self.strategy
     }
     /// <p>The maximum percentage of additional capacity that the quota share can borrow from other shares. <code>borrowLimit</code> can only be applied to quota shares with a strategy of <code>LEND_AND_BORROW</code>. This value is expressed as a percentage of the quota share's configured <a href="https://docs.aws.amazon.com/batch/latest/APIReference/API_QuotaShareCapacityLimit.html">CapacityLimits</a>.</p>
     /// <p>The <code>borrowLimit</code> is applied uniformly across all capacity units. For example, if the <code>borrowLimit</code> is 200, the quota share can borrow up to 200% of its configured <code>maxCapacity</code> for each capacity unit. The default <code>borrowLimit</code> is -1, which indicates unlimited borrowing.</p>
@@ -69,10 +69,19 @@
         &self.borrow_limit
     }
     /// Consumes the builder and constructs a [`QuotaShareResourceSharingConfiguration`](crate::types::QuotaShareResourceSharingConfiguration).
-    pub fn build(self) -> super::super::types::QuotaShareResourceSharingConfiguration {
-        super::super::types::QuotaShareResourceSharingConfiguration {
-            strategy: self.strategy,
+    /// This method will fail if any of the following fields are not set:
+    /// - [`strategy`](crate::types::builders::QuotaShareResourceSharingConfigurationBuilder::strategy)
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<super::super::types::QuotaShareResourceSharingConfiguration, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::QuotaShareResourceSharingConfiguration {
+            strategy: self.strategy.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "strategy",
+                    "strategy was not specified but it is required when building QuotaShareResourceSharingConfiguration",
+                )
+            })?,
             borrow_limit: self.borrow_limit,
-        }
+        })
     }
 }
```

### `src/types/_repository_credentials.rs`

```diff
--- reference/src/types/_repository_credentials.rs
+++ generated/src/types/_repository_credentials.rs
@@ -5,12 +5,13 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct RepositoryCredentials {
     /// <p>The Amazon Resource Name (ARN) of the secret containing the private repository credentials.</p>
-    pub credentials_parameter: ::std::option::Option<::std::string::String>,
+    pub credentials_parameter: ::std::string::String,
 }
 impl RepositoryCredentials {
     /// <p>The Amazon Resource Name (ARN) of the secret containing the private repository credentials.</p>
-    pub fn credentials_parameter(&self) -> ::std::option::Option<&str> {
-        self.credentials_parameter.as_deref()
+    pub fn credentials_parameter(&self) -> &str {
+        use std::ops::Deref;
+        self.credentials_parameter.deref()
     }
 }
 impl RepositoryCredentials {
@@ -43,9 +44,16 @@
         &self.credentials_parameter
     }
     /// Consumes the builder and constructs a [`RepositoryCredentials`](crate::types::RepositoryCredentials).
-    pub fn build(self) -> super::super::types::RepositoryCredentials {
-        super::super::types::RepositoryCredentials {
-            credentials_parameter: self.credentials_parameter,
-        }
+    /// This method will fail if any of the following fields are not set:
+    /// - [`credentials_parameter`](crate::types::builders::RepositoryCredentialsBuilder::credentials_parameter)
+    pub fn build(self) -> ::std::result::Result<super::super::types::RepositoryCredentials, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::RepositoryCredentials {
+            credentials_parameter: self.credentials_parameter.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "credentials_parameter",
+                    "credentials_parameter was not specified but it is required when building RepositoryCredentials",
+                )
+            })?,
+        })
     }
 }
```

### `src/types/_resource_requirement.rs`

```diff
--- reference/src/types/_resource_requirement.rs
+++ generated/src/types/_resource_requirement.rs
@@ -186,9 +186,9 @@
     /// </dl>
     /// </dd>
     /// </dl>
-    pub value: ::std::option::Option<::std::string::String>,
+    pub value: ::std::string::String,
     /// <p>The type of resource to assign to a container. The supported resources include <code>GPU</code>, <code>MEMORY</code>, and <code>VCPU</code>.</p>
-    pub r#type: ::std::option::Option<super::super::types::ResourceType>,
+    pub r#type: super::super::types::ResourceType,
 }
 impl ResourceRequirement {
     /// <p>The quantity of the specified resource to reserve for the container. The values vary based on the <code>type</code> specified.</p>
@@ -373,12 +373,13 @@
     /// </dl>
     /// </dd>
     /// </dl>
-    pub fn value(&self) -> ::std::option::Option<&str> {
-        self.value.as_deref()
+    pub fn value(&self) -> &str {
+        use std::ops::Deref;
+        self.value.deref()
     }
     /// <p>The type of resource to assign to a container. The supported resources include <code>GPU</code>, <code>MEMORY</code>, and <code>VCPU</code>.</p>
-    pub fn r#type(&self) -> ::std::option::Option<&super::super::types::ResourceType> {
-        self.r#type.as_ref()
+    pub fn r#type(&self) -> &super::super::types::ResourceType {
+        &self.r#type
     }
 }
 impl ResourceRequirement {
@@ -970,10 +971,23 @@
         &self.r#type
     }
     /// Consumes the builder and constructs a [`ResourceRequirement`](crate::types::ResourceRequirement).
-    pub fn build(self) -> super::super::types::ResourceRequirement {
-        super::super::types::ResourceRequirement {
-            value: self.value,
-            r#type: self.r#type,
-        }
+    /// This method will fail if any of the following fields are not set:
+    /// - [`value`](crate::types::builders::ResourceRequirementBuilder::value)
+    /// - [`r#type`](crate::types::builders::ResourceRequirementBuilder::type)
+    pub fn build(self) -> ::std::result::Result<super::super::types::ResourceRequirement, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::ResourceRequirement {
+            value: self.value.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "value",
+                    "value was not specified but it is required when building ResourceRequirement",
+                )
+            })?,
+            r#type: self.r#type.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "r#type",
+                    "r#type was not specified but it is required when building ResourceRequirement",
+                )
+            })?,
+        })
     }
 }
```

### `src/types/_s3_files_volume_configuration.rs`

```diff
--- reference/src/types/_s3_files_volume_configuration.rs
+++ generated/src/types/_s3_files_volume_configuration.rs
@@ -5,7 +5,7 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct S3FilesVolumeConfiguration {
     /// <p>The Amazon Resource Name (ARN) of the S3Files file system to use.</p>
-    pub file_system_arn: ::std::option::Option<::std::string::String>,
+    pub file_system_arn: ::std::string::String,
     /// <p>The directory within the S3Files file system to mount as the root directory.</p>
     pub root_directory: ::std::option::Option<::std::string::String>,
     /// <p>The port to use when sending encrypted data between the Amazon ECS host and the S3Files file system server.</p>
@@ -15,8 +15,9 @@
 }
 impl S3FilesVolumeConfiguration {
     /// <p>The Amazon Resource Name (ARN) of the S3Files file system to use.</p>
-    pub fn file_system_arn(&self) -> ::std::option::Option<&str> {
-        self.file_system_arn.as_deref()
+    pub fn file_system_arn(&self) -> &str {
+        use std::ops::Deref;
+        self.file_system_arn.deref()
     }
     /// <p>The directory within the S3Files file system to mount as the root directory.</p>
     pub fn root_directory(&self) -> ::std::option::Option<&str> {
@@ -106,12 +107,19 @@
         &self.access_point_arn
     }
     /// Consumes the builder and constructs a [`S3FilesVolumeConfiguration`](crate::types::S3FilesVolumeConfiguration).
-    pub fn build(self) -> super::super::types::S3FilesVolumeConfiguration {
-        super::super::types::S3FilesVolumeConfiguration {
-            file_system_arn: self.file_system_arn,
+    /// This method will fail if any of the following fields are not set:
+    /// - [`file_system_arn`](crate::types::builders::S3FilesVolumeConfigurationBuilder::file_system_arn)
+    pub fn build(self) -> ::std::result::Result<super::super::types::S3FilesVolumeConfiguration, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::S3FilesVolumeConfiguration {
+            file_system_arn: self.file_system_arn.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "file_system_arn",
+                    "file_system_arn was not specified but it is required when building S3FilesVolumeConfiguration",
+                )
+            })?,
             root_directory: self.root_directory,
             transit_encryption_port: self.transit_encryption_port,
             access_point_arn: self.access_point_arn,
-        }
+        })
     }
 }
```

### `src/types/_scheduling_policy_detail.rs`

```diff
--- reference/src/types/_scheduling_policy_detail.rs
+++ generated/src/types/_scheduling_policy_detail.rs
@@ -5,9 +5,9 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct SchedulingPolicyDetail {
     /// <p>The name of the fair-share scheduling policy.</p>
-    pub name: ::std::option::Option<::std::string::String>,
+    pub name: ::std::string::String,
     /// <p>The Amazon Resource Name (ARN) of the scheduling policy. An example is <code>arn:<i>aws</i>:batch:<i>us-east-1</i>:<i>123456789012</i>:scheduling-policy/<i>HighPriority</i> </code>.</p>
-    pub arn: ::std::option::Option<::std::string::String>,
+    pub arn: ::std::string::String,
     /// <p>The quota share scheduling policy details.</p>
     pub quota_share_policy: ::std::option::Option<super::super::types::QuotaSharePolicy>,
     /// <p>The fair-share scheduling policy details.</p>
@@ -17,12 +17,14 @@
 }
 impl SchedulingPolicyDetail {
     /// <p>The name of the fair-share scheduling policy.</p>
-    pub fn name(&self) -> ::std::option::Option<&str> {
-        self.name.as_deref()
+    pub fn name(&self) -> &str {
+        use std::ops::Deref;
+        self.name.deref()
     }
     /// <p>The Amazon Resource Name (ARN) of the scheduling policy. An example is <code>arn:<i>aws</i>:batch:<i>us-east-1</i>:<i>123456789012</i>:scheduling-policy/<i>HighPriority</i> </code>.</p>
-    pub fn arn(&self) -> ::std::option::Option<&str> {
-        self.arn.as_deref()
+    pub fn arn(&self) -> &str {
+        use std::ops::Deref;
+        self.arn.deref()
     }
     /// <p>The quota share scheduling policy details.</p>
     pub fn quota_share_policy(&self) -> ::std::option::Option<&super::super::types::QuotaSharePolicy> {
@@ -134,13 +136,26 @@
         &self.tags
     }
     /// Consumes the builder and constructs a [`SchedulingPolicyDetail`](crate::types::SchedulingPolicyDetail).
-    pub fn build(self) -> super::super::types::SchedulingPolicyDetail {
-        super::super::types::SchedulingPolicyDetail {
-            name: self.name,
-            arn: self.arn,
+    /// This method will fail if any of the following fields are not set:
+    /// - [`name`](crate::types::builders::SchedulingPolicyDetailBuilder::name)
+    /// - [`arn`](crate::types::builders::SchedulingPolicyDetailBuilder::arn)
+    pub fn build(self) -> ::std::result::Result<super::super::types::SchedulingPolicyDetail, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::SchedulingPolicyDetail {
+            name: self.name.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "name",
+                    "name was not specified but it is required when building SchedulingPolicyDetail",
+                )
+            })?,
+            arn: self.arn.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "arn",
+                    "arn was not specified but it is required when building SchedulingPolicyDetail",
+                )
+            })?,
             quota_share_policy: self.quota_share_policy,
             fairshare_policy: self.fairshare_policy,
             tags: self.tags,
-        }
+        })
     }
 }
```

### `src/types/_scheduling_policy_listing_detail.rs`

```diff
--- reference/src/types/_scheduling_policy_listing_detail.rs
+++ generated/src/types/_scheduling_policy_listing_detail.rs
@@ -5,12 +5,13 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct SchedulingPolicyListingDetail {
     /// <p>Amazon Resource Name (ARN) of the scheduling policy.</p>
-    pub arn: ::std::option::Option<::std::string::String>,
+    pub arn: ::std::string::String,
 }
 impl SchedulingPolicyListingDetail {
     /// <p>Amazon Resource Name (ARN) of the scheduling policy.</p>
-    pub fn arn(&self) -> ::std::option::Option<&str> {
-        self.arn.as_deref()
+    pub fn arn(&self) -> &str {
+        use std::ops::Deref;
+        self.arn.deref()
     }
 }
 impl SchedulingPolicyListingDetail {
@@ -43,7 +44,16 @@
         &self.arn
     }
     /// Consumes the builder and constructs a [`SchedulingPolicyListingDetail`](crate::types::SchedulingPolicyListingDetail).
-    pub fn build(self) -> super::super::types::SchedulingPolicyListingDetail {
-        super::super::types::SchedulingPolicyListingDetail { arn: self.arn }
+    /// This method will fail if any of the following fields are not set:
+    /// - [`arn`](crate::types::builders::SchedulingPolicyListingDetailBuilder::arn)
+    pub fn build(self) -> ::std::result::Result<super::super::types::SchedulingPolicyListingDetail, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::SchedulingPolicyListingDetail {
+            arn: self.arn.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "arn",
+                    "arn was not specified but it is required when building SchedulingPolicyListingDetail",
+                )
+            })?,
+        })
     }
 }
```

### `src/types/_secret.rs`

```diff
--- reference/src/types/_secret.rs
+++ generated/src/types/_secret.rs
@@ -12,22 +12,24 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct Secret {
     /// <p>The name of the secret.</p>
-    pub name: ::std::option::Option<::std::string::String>,
+    pub name: ::std::string::String,
     /// <p>The secret to expose to the container. The supported values are either the full Amazon Resource Name (ARN) of the Secrets Manager secret or the full ARN of the parameter in the Amazon Web Services Systems Manager Parameter Store.</p><note>
     /// <p>If the Amazon Web Services Systems Manager Parameter Store parameter exists in the same Region as the job you're launching, then you can use either the full Amazon Resource Name (ARN) or name of the parameter. If the parameter exists in a different Region, then the full ARN must be specified.</p>
     /// </note>
-    pub value_from: ::std::option::Option<::std::string::String>,
+    pub value_from: ::std::string::String,
 }
 impl Secret {
     /// <p>The name of the secret.</p>
-    pub fn name(&self) -> ::std::option::Option<&str> {
-        self.name.as_deref()
+    pub fn name(&self) -> &str {
+        use std::ops::Deref;
+        self.name.deref()
     }
     /// <p>The secret to expose to the container. The supported values are either the full Amazon Resource Name (ARN) of the Secrets Manager secret or the full ARN of the parameter in the Amazon Web Services Systems Manager Parameter Store.</p><note>
     /// <p>If the Amazon Web Services Systems Manager Parameter Store parameter exists in the same Region as the job you're launching, then you can use either the full Amazon Resource Name (ARN) or name of the parameter. If the parameter exists in a different Region, then the full ARN must be specified.</p>
     /// </note>
-    pub fn value_from(&self) -> ::std::option::Option<&str> {
-        self.value_from.as_deref()
+    pub fn value_from(&self) -> &str {
+        use std::ops::Deref;
+        self.value_from.deref()
     }
 }
 impl Secret {
@@ -82,10 +84,23 @@
         &self.value_from
     }
     /// Consumes the builder and constructs a [`Secret`](crate::types::Secret).
-    pub fn build(self) -> super::super::types::Secret {
-        super::super::types::Secret {
-            name: self.name,
-            value_from: self.value_from,
-        }
+    /// This method will fail if any of the following fields are not set:
+    /// - [`name`](crate::types::builders::SecretBuilder::name)
+    /// - [`value_from`](crate::types::builders::SecretBuilder::value_from)
+    pub fn build(self) -> ::std::result::Result<super::super::types::Secret, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::Secret {
+            name: self.name.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "name",
+                    "name was not specified but it is required when building Secret",
+                )
+            })?,
+            value_from: self.value_from.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "value_from",
+                    "value_from was not specified but it is required when building Secret",
+                )
+            })?,
+        })
     }
 }
```

### `src/types/_service_environment_detail.rs`

```diff
--- reference/src/types/_service_environment_detail.rs
+++ generated/src/types/_service_environment_detail.rs
@@ -5,32 +5,34 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct ServiceEnvironmentDetail {
     /// <p>The name of the service environment.</p>
-    pub service_environment_name: ::std::option::Option<::std::string::String>,
+    pub service_environment_name: ::std::string::String,
     /// <p>The Amazon Resource Name (ARN) of the service environment.</p>
-    pub service_environment_arn: ::std::option::Option<::std::string::String>,
+    pub service_environment_arn: ::std::string::String,
     /// <p>The type of service environment. For SageMaker Training jobs, this value is <code>SAGEMAKER_TRAINING</code>.</p>
-    pub service_environment_type: ::std::option::Option<super::super::types::ServiceEnvironmentType>,
+    pub service_environment_type: super::super::types::ServiceEnvironmentType,
     /// <p>The state of the service environment. Valid values are <code>ENABLED</code> and <code>DISABLED</code>.</p>
     pub state: ::std::option::Option<super::super::types::ServiceEnvironmentState>,
     /// <p>The current status of the service environment.</p>
     pub status: ::std::option::Option<super::super::types::ServiceEnvironmentStatus>,
     /// <p>The capacity limits for the service environment. This defines the maximum resources that can be used by service jobs in this environment.</p>
-    pub capacity_limits: ::std::option::Option<::std::vec::Vec<super::super::types::CapacityLimit>>,
+    pub capacity_limits: ::std::vec::Vec<super::super::types::CapacityLimit>,
     /// <p>The tags associated with the service environment. Each tag consists of a key and an optional value. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/using-tags.html">Tagging your Batch resources</a>.</p>
     pub tags: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
 }
 impl ServiceEnvironmentDetail {
     /// <p>The name of the service environment.</p>
-    pub fn service_environment_name(&self) -> ::std::option::Option<&str> {
-        self.service_environment_name.as_deref()
+    pub fn service_environment_name(&self) -> &str {
+        use std::ops::Deref;
+        self.service_environment_name.deref()
     }
     /// <p>The Amazon Resource Name (ARN) of the service environment.</p>
-    pub fn service_environment_arn(&self) -> ::std::option::Option<&str> {
-        self.service_environment_arn.as_deref()
+    pub fn service_environment_arn(&self) -> &str {
+        use std::ops::Deref;
+        self.service_environment_arn.deref()
     }
     /// <p>The type of service environment. For SageMaker Training jobs, this value is <code>SAGEMAKER_TRAINING</code>.</p>
-    pub fn service_environment_type(&self) -> ::std::option::Option<&super::super::types::ServiceEnvironmentType> {
-        self.service_environment_type.as_ref()
+    pub fn service_environment_type(&self) -> &super::super::types::ServiceEnvironmentType {
+        &self.service_environment_type
     }
     /// <p>The state of the service environment. Valid values are <code>ENABLED</code> and <code>DISABLED</code>.</p>
     pub fn state(&self) -> ::std::option::Option<&super::super::types::ServiceEnvironmentState> {
@@ -41,10 +43,9 @@
         self.status.as_ref()
     }
     /// <p>The capacity limits for the service environment. This defines the maximum resources that can be used by service jobs in this environment.</p>
-    ///
-    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.capacity_limits.is_none()`.
     pub fn capacity_limits(&self) -> &[super::super::types::CapacityLimit] {
-        self.capacity_limits.as_deref().unwrap_or_default()
+        use std::ops::Deref;
+        self.capacity_limits.deref()
     }
     /// <p>The tags associated with the service environment. Each tag consists of a key and an optional value. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/using-tags.html">Tagging your Batch resources</a>.</p>
     pub fn tags(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, ::std::string::String>> {
@@ -185,15 +186,40 @@
         &self.tags
     }
     /// Consumes the builder and constructs a [`ServiceEnvironmentDetail`](crate::types::ServiceEnvironmentDetail).
-    pub fn build(self) -> super::super::types::ServiceEnvironmentDetail {
-        super::super::types::ServiceEnvironmentDetail {
-            service_environment_name: self.service_environment_name,
-            service_environment_arn: self.service_environment_arn,
-            service_environment_type: self.service_environment_type,
+    /// This method will fail if any of the following fields are not set:
+    /// - [`service_environment_name`](crate::types::builders::ServiceEnvironmentDetailBuilder::service_environment_name)
+    /// - [`service_environment_arn`](crate::types::builders::ServiceEnvironmentDetailBuilder::service_environment_arn)
+    /// - [`service_environment_type`](crate::types::builders::ServiceEnvironmentDetailBuilder::service_environment_type)
+    /// - [`capacity_limits`](crate::types::builders::ServiceEnvironmentDetailBuilder::capacity_limits)
+    pub fn build(self) -> ::std::result::Result<super::super::types::ServiceEnvironmentDetail, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::ServiceEnvironmentDetail {
+            service_environment_name: self.service_environment_name.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "service_environment_name",
+                    "service_environment_name was not specified but it is required when building ServiceEnvironmentDetail",
+                )
+            })?,
+            service_environment_arn: self.service_environment_arn.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "service_environment_arn",
+                    "service_environment_arn was not specified but it is required when building ServiceEnvironmentDetail",
+                )
+            })?,
+            service_environment_type: self.service_environment_type.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "service_environment_type",
+                    "service_environment_type was not specified but it is required when building ServiceEnvironmentDetail",
+                )
+            })?,
             state: self.state,
             status: self.status,
-            capacity_limits: self.capacity_limits,
+            capacity_limits: self.capacity_limits.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "capacity_limits",
+                    "capacity_limits was not specified but it is required when building ServiceEnvironmentDetail",
+                )
+            })?,
             tags: self.tags,
-        }
+        })
     }
 }
```

### `src/types/_service_environment_order.rs`

```diff
--- reference/src/types/_service_environment_order.rs
+++ generated/src/types/_service_environment_order.rs
@@ -5,18 +5,19 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct ServiceEnvironmentOrder {
     /// <p>The order of the service environment. Job queues with a higher priority are evaluated first when associated with the same service environment.</p>
-    pub order: ::std::option::Option<i32>,
+    pub order: i32,
     /// <p>The name or ARN of the service environment.</p>
-    pub service_environment: ::std::option::Option<::std::string::String>,
+    pub service_environment: ::std::string::String,
 }
 impl ServiceEnvironmentOrder {
     /// <p>The order of the service environment. Job queues with a higher priority are evaluated first when associated with the same service environment.</p>
-    pub fn order(&self) -> ::std::option::Option<i32> {
+    pub fn order(&self) -> i32 {
         self.order
     }
     /// <p>The name or ARN of the service environment.</p>
-    pub fn service_environment(&self) -> ::std::option::Option<&str> {
-        self.service_environment.as_deref()
+    pub fn service_environment(&self) -> &str {
+        use std::ops::Deref;
+        self.service_environment.deref()
     }
 }
 impl ServiceEnvironmentOrder {
@@ -65,10 +66,23 @@
         &self.service_environment
     }
     /// Consumes the builder and constructs a [`ServiceEnvironmentOrder`](crate::types::ServiceEnvironmentOrder).
-    pub fn build(self) -> super::super::types::ServiceEnvironmentOrder {
-        super::super::types::ServiceEnvironmentOrder {
-            order: self.order,
-            service_environment: self.service_environment,
-        }
+    /// This method will fail if any of the following fields are not set:
+    /// - [`order`](crate::types::builders::ServiceEnvironmentOrderBuilder::order)
+    /// - [`service_environment`](crate::types::builders::ServiceEnvironmentOrderBuilder::service_environment)
+    pub fn build(self) -> ::std::result::Result<super::super::types::ServiceEnvironmentOrder, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::ServiceEnvironmentOrder {
+            order: self.order.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "order",
+                    "order was not specified but it is required when building ServiceEnvironmentOrder",
+                )
+            })?,
+            service_environment: self.service_environment.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "service_environment",
+                    "service_environment was not specified but it is required when building ServiceEnvironmentOrder",
+                )
+            })?,
+        })
     }
 }
```

### `src/types/_service_job_retry_strategy.rs`

```diff
--- reference/src/types/_service_job_retry_strategy.rs
+++ generated/src/types/_service_job_retry_strategy.rs
@@ -5,13 +5,13 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct ServiceJobRetryStrategy {
     /// <p>The number of times to move a service job to <code>RUNNABLE</code> status. You can specify between 1 and 10 attempts.</p>
-    pub attempts: ::std::option::Option<i32>,
+    pub attempts: i32,
     /// <p>Array of <code>ServiceJobEvaluateOnExit</code> objects that specify conditions under which the service job should be retried or failed.</p>
     pub evaluate_on_exit: ::std::option::Option<::std::vec::Vec<super::super::types::ServiceJobEvaluateOnExit>>,
 }
 impl ServiceJobRetryStrategy {
     /// <p>The number of times to move a service job to <code>RUNNABLE</code> status. You can specify between 1 and 10 attempts.</p>
-    pub fn attempts(&self) -> ::std::option::Option<i32> {
+    pub fn attempts(&self) -> i32 {
         self.attempts
     }
     /// <p>Array of <code>ServiceJobEvaluateOnExit</code> objects that specify conditions under which the service job should be retried or failed.</p>
@@ -72,10 +72,17 @@
         &self.evaluate_on_exit
     }
     /// Consumes the builder and constructs a [`ServiceJobRetryStrategy`](crate::types::ServiceJobRetryStrategy).
-    pub fn build(self) -> super::super::types::ServiceJobRetryStrategy {
-        super::super::types::ServiceJobRetryStrategy {
-            attempts: self.attempts,
+    /// This method will fail if any of the following fields are not set:
+    /// - [`attempts`](crate::types::builders::ServiceJobRetryStrategyBuilder::attempts)
+    pub fn build(self) -> ::std::result::Result<super::super::types::ServiceJobRetryStrategy, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::ServiceJobRetryStrategy {
+            attempts: self.attempts.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "attempts",
+                    "attempts was not specified but it is required when building ServiceJobRetryStrategy",
+                )
+            })?,
             evaluate_on_exit: self.evaluate_on_exit,
-        }
+        })
     }
 }
```

### `src/types/_service_job_summary.rs`

```diff
--- reference/src/types/_service_job_summary.rs
+++ generated/src/types/_service_job_summary.rs
@@ -13,13 +13,13 @@
     /// <p>The Amazon Resource Name (ARN) of the service job.</p>
     pub job_arn: ::std::option::Option<::std::string::String>,
     /// <p>The job ID for the service job.</p>
-    pub job_id: ::std::option::Option<::std::string::String>,
+    pub job_id: ::std::string::String,
     /// <p>The name of the service job.</p>
-    pub job_name: ::std::option::Option<::std::string::String>,
+    pub job_name: ::std::string::String,
     /// <p>The Unix timestamp (in milliseconds) for when the service job was scheduled for execution.</p>
     pub scheduled_at: ::std::option::Option<i64>,
     /// <p>The type of service job. For SageMaker Training jobs, this value is <code>SAGEMAKER_TRAINING</code>.</p>
-    pub service_job_type: ::std::option::Option<super::super::types::ServiceJobType>,
+    pub service_job_type: super::super::types::ServiceJobType,
     /// <p>The share identifier for the job.</p>
     pub share_identifier: ::std::option::Option<::std::string::String>,
     /// <p>The quota share for the service job.</p>
@@ -53,12 +53,14 @@
         self.job_arn.as_deref()
     }
     /// <p>The job ID for the service job.</p>
-    pub fn job_id(&self) -> ::std::option::Option<&str> {
-        self.job_id.as_deref()
+    pub fn job_id(&self) -> &str {
+        use std::ops::Deref;
+        self.job_id.deref()
     }
     /// <p>The name of the service job.</p>
-    pub fn job_name(&self) -> ::std::option::Option<&str> {
-        self.job_name.as_deref()
+    pub fn job_name(&self) -> &str {
+        use std::ops::Deref;
+        self.job_name.deref()
     }
     /// <p>The Unix timestamp (in milliseconds) for when the service job was scheduled for execution.</p>
     pub fn scheduled_at(&self) -> ::std::option::Option<i64> {
@@ -65,8 +67,8 @@
         self.scheduled_at
     }
     /// <p>The type of service job. For SageMaker Training jobs, this value is <code>SAGEMAKER_TRAINING</code>.</p>
-    pub fn service_job_type(&self) -> ::std::option::Option<&super::super::types::ServiceJobType> {
-        self.service_job_type.as_ref()
+    pub fn service_job_type(&self) -> &super::super::types::ServiceJobType {
+        &self.service_job_type
     }
     /// <p>The share identifier for the job.</p>
     pub fn share_identifier(&self) -> ::std::option::Option<&str> {
@@ -326,16 +328,35 @@
         &self.stopped_at
     }
     /// Consumes the builder and constructs a [`ServiceJobSummary`](crate::types::ServiceJobSummary).
-    pub fn build(self) -> super::super::types::ServiceJobSummary {
-        super::super::types::ServiceJobSummary {
+    /// This method will fail if any of the following fields are not set:
+    /// - [`job_id`](crate::types::builders::ServiceJobSummaryBuilder::job_id)
+    /// - [`job_name`](crate::types::builders::ServiceJobSummaryBuilder::job_name)
+    /// - [`service_job_type`](crate::types::builders::ServiceJobSummaryBuilder::service_job_type)
+    pub fn build(self) -> ::std::result::Result<super::super::types::ServiceJobSummary, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::ServiceJobSummary {
             latest_attempt: self.latest_attempt,
             capacity_usage: self.capacity_usage,
             created_at: self.created_at,
             job_arn: self.job_arn,
-            job_id: self.job_id,
-            job_name: self.job_name,
+            job_id: self.job_id.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "job_id",
+                    "job_id was not specified but it is required when building ServiceJobSummary",
+                )
+            })?,
+            job_name: self.job_name.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "job_name",
+                    "job_name was not specified but it is required when building ServiceJobSummary",
+                )
+            })?,
             scheduled_at: self.scheduled_at,
-            service_job_type: self.service_job_type,
+            service_job_type: self.service_job_type.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "service_job_type",
+                    "service_job_type was not specified but it is required when building ServiceJobSummary",
+                )
+            })?,
             share_identifier: self.share_identifier,
             quota_share_name: self.quota_share_name,
             status: self.status,
@@ -342,6 +363,6 @@
             status_reason: self.status_reason,
             started_at: self.started_at,
             stopped_at: self.stopped_at,
-        }
+        })
     }
 }
```

### `src/types/_service_resource_id.rs`

```diff
--- reference/src/types/_service_resource_id.rs
+++ generated/src/types/_service_resource_id.rs
@@ -5,18 +5,19 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct ServiceResourceId {
     /// <p>The name of the resource identifier.</p>
-    pub name: ::std::option::Option<super::super::types::ServiceResourceIdName>,
+    pub name: super::super::types::ServiceResourceIdName,
     /// <p>The value of the resource identifier.</p>
-    pub value: ::std::option::Option<::std::string::String>,
+    pub value: ::std::string::String,
 }
 impl ServiceResourceId {
     /// <p>The name of the resource identifier.</p>
-    pub fn name(&self) -> ::std::option::Option<&super::super::types::ServiceResourceIdName> {
-        self.name.as_ref()
+    pub fn name(&self) -> &super::super::types::ServiceResourceIdName {
+        &self.name
     }
     /// <p>The value of the resource identifier.</p>
-    pub fn value(&self) -> ::std::option::Option<&str> {
-        self.value.as_deref()
+    pub fn value(&self) -> &str {
+        use std::ops::Deref;
+        self.value.deref()
     }
 }
 impl ServiceResourceId {
@@ -65,10 +66,23 @@
         &self.value
     }
     /// Consumes the builder and constructs a [`ServiceResourceId`](crate::types::ServiceResourceId).
-    pub fn build(self) -> super::super::types::ServiceResourceId {
-        super::super::types::ServiceResourceId {
-            name: self.name,
-            value: self.value,
-        }
+    /// This method will fail if any of the following fields are not set:
+    /// - [`name`](crate::types::builders::ServiceResourceIdBuilder::name)
+    /// - [`value`](crate::types::builders::ServiceResourceIdBuilder::value)
+    pub fn build(self) -> ::std::result::Result<super::super::types::ServiceResourceId, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::ServiceResourceId {
+            name: self.name.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "name",
+                    "name was not specified but it is required when building ServiceResourceId",
+                )
+            })?,
+            value: self.value.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "value",
+                    "value was not specified but it is required when building ServiceResourceId",
+                )
+            })?,
+        })
     }
 }
```

### `src/types/_share_attributes.rs`

```diff
--- reference/src/types/_share_attributes.rs
+++ generated/src/types/_share_attributes.rs
@@ -7,7 +7,7 @@
     /// <p>A share identifier or share identifier prefix. If the string ends with an asterisk (*), this entry specifies the weight factor to use for share identifiers that start with that prefix. The list of share identifiers in a fair-share policy can't overlap. For example, you can't have one that specifies a <code>shareIdentifier</code> of <code>UserA*</code> and another that specifies a <code>shareIdentifier</code> of <code>UserA1</code>.</p>
     /// <p>There can be no more than 500 share identifiers active in a job queue.</p>
     /// <p>The string is limited to 255 alphanumeric characters, and can be followed by an asterisk (*).</p>
-    pub share_identifier: ::std::option::Option<::std::string::String>,
+    pub share_identifier: ::std::string::String,
     /// <p>The weight factor for the share identifier. The default value is 1.0. A lower value has a higher priority for compute resources. For example, jobs that use a share identifier with a weight factor of 0.125 (1/8) get 8 times the compute resources of jobs that use a share identifier with a weight factor of 1.</p>
     /// <p>The smallest supported value is 0.0001, and the largest supported value is 999.9999.</p>
     pub weight_factor: ::std::option::Option<f32>,
@@ -16,8 +16,9 @@
     /// <p>A share identifier or share identifier prefix. If the string ends with an asterisk (*), this entry specifies the weight factor to use for share identifiers that start with that prefix. The list of share identifiers in a fair-share policy can't overlap. For example, you can't have one that specifies a <code>shareIdentifier</code> of <code>UserA*</code> and another that specifies a <code>shareIdentifier</code> of <code>UserA1</code>.</p>
     /// <p>There can be no more than 500 share identifiers active in a job queue.</p>
     /// <p>The string is limited to 255 alphanumeric characters, and can be followed by an asterisk (*).</p>
-    pub fn share_identifier(&self) -> ::std::option::Option<&str> {
-        self.share_identifier.as_deref()
+    pub fn share_identifier(&self) -> &str {
+        use std::ops::Deref;
+        self.share_identifier.deref()
     }
     /// <p>The weight factor for the share identifier. The default value is 1.0. A lower value has a higher priority for compute resources. For example, jobs that use a share identifier with a weight factor of 0.125 (1/8) get 8 times the compute resources of jobs that use a share identifier with a weight factor of 1.</p>
     /// <p>The smallest supported value is 0.0001, and the largest supported value is 999.9999.</p>
@@ -79,10 +80,17 @@
         &self.weight_factor
     }
     /// Consumes the builder and constructs a [`ShareAttributes`](crate::types::ShareAttributes).
-    pub fn build(self) -> super::super::types::ShareAttributes {
-        super::super::types::ShareAttributes {
-            share_identifier: self.share_identifier,
+    /// This method will fail if any of the following fields are not set:
+    /// - [`share_identifier`](crate::types::builders::ShareAttributesBuilder::share_identifier)
+    pub fn build(self) -> ::std::result::Result<super::super::types::ShareAttributes, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::ShareAttributes {
+            share_identifier: self.share_identifier.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "share_identifier",
+                    "share_identifier was not specified but it is required when building ShareAttributes",
+                )
+            })?,
             weight_factor: self.weight_factor,
-        }
+        })
     }
 }
```

### `src/types/_task_container_details.rs`

```diff
--- reference/src/types/_task_container_details.rs
+++ generated/src/types/_task_container_details.rs
@@ -33,7 +33,7 @@
     /// </note>
     pub log_configuration: ::std::option::Option<super::super::types::LogConfiguration>,
     /// <p>The mount points for data volumes in your container.</p>
-    /// <p>This parameter maps to <code>Volumes</code> in the <a href="https://docs.docker.com/engine/api/latest/#tag/Container/operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/latest/">Docker Remote API</a> and the <code>--volume</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p>
+    /// <p>This parameter maps to <code>Volumes</code> in the <a href="https://docs.docker.com/engine/api/latest/#tag/Container/operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/latest/">Docker Remote API</a> and the <a href="">--volume</a> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p>
     /// <p>Windows containers can mount whole directories on the same drive as <code>$env:ProgramData</code>. Windows containers can't mount directories on a different drive, and mount point can't be across drives.</p>
     pub mount_points: ::std::option::Option<::std::vec::Vec<super::super::types::MountPoint>>,
     /// <p>The name of a container.</p>
@@ -146,7 +146,7 @@
         self.log_configuration.as_ref()
     }
     /// <p>The mount points for data volumes in your container.</p>
-    /// <p>This parameter maps to <code>Volumes</code> in the <a href="https://docs.docker.com/engine/api/latest/#tag/Container/operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/latest/">Docker Remote API</a> and the <code>--volume</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p>
+    /// <p>This parameter maps to <code>Volumes</code> in the <a href="https://docs.docker.com/engine/api/latest/#tag/Container/operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/latest/">Docker Remote API</a> and the <a href="">--volume</a> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p>
     /// <p>Windows containers can mount whole directories on the same drive as <code>$env:ProgramData</code>. Windows containers can't mount directories on a different drive, and mount point can't be across drives.</p>
     ///
     /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.mount_points.is_none()`.
@@ -454,7 +454,7 @@
     /// To override the contents of this collection use [`set_mount_points`](Self::set_mount_points).
     ///
     /// <p>The mount points for data volumes in your container.</p>
-    /// <p>This parameter maps to <code>Volumes</code> in the <a href="https://docs.docker.com/engine/api/latest/#tag/Container/operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/latest/">Docker Remote API</a> and the <code>--volume</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p>
+    /// <p>This parameter maps to <code>Volumes</code> in the <a href="https://docs.docker.com/engine/api/latest/#tag/Container/operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/latest/">Docker Remote API</a> and the <a href="">--volume</a> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p>
     /// <p>Windows containers can mount whole directories on the same drive as <code>$env:ProgramData</code>. Windows containers can't mount directories on a different drive, and mount point can't be across drives.</p>
     pub fn mount_points(mut self, input: super::super::types::MountPoint) -> Self {
         let mut v = self.mount_points.unwrap_or_default();
@@ -463,7 +463,7 @@
         self
     }
     /// <p>The mount points for data volumes in your container.</p>
-    /// <p>This parameter maps to <code>Volumes</code> in the <a href="https://docs.docker.com/engine/api/latest/#tag/Container/operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/latest/">Docker Remote API</a> and the <code>--volume</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p>
+    /// <p>This parameter maps to <code>Volumes</code> in the <a href="https://docs.docker.com/engine/api/latest/#tag/Container/operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/latest/">Docker Remote API</a> and the <a href="">--volume</a> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p>
     /// <p>Windows containers can mount whole directories on the same drive as <code>$env:ProgramData</code>. Windows containers can't mount directories on a different drive, and mount point can't be across drives.</p>
     pub fn set_mount_points(mut self, input: ::std::option::Option<::std::vec::Vec<super::super::types::MountPoint>>) -> Self {
         self.mount_points = input;
@@ -470,7 +470,7 @@
         self
     }
     /// <p>The mount points for data volumes in your container.</p>
-    /// <p>This parameter maps to <code>Volumes</code> in the <a href="https://docs.docker.com/engine/api/latest/#tag/Container/operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/latest/">Docker Remote API</a> and the <code>--volume</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p>
+    /// <p>This parameter maps to <code>Volumes</code> in the <a href="https://docs.docker.com/engine/api/latest/#tag/Container/operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/latest/">Docker Remote API</a> and the <a href="">--volume</a> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p>
     /// <p>Windows containers can mount whole directories on the same drive as <code>$env:ProgramData</code>. Windows containers can't mount directories on a different drive, and mount point can't be across drives.</p>
     pub fn get_mount_points(&self) -> &::std::option::Option<::std::vec::Vec<super::super::types::MountPoint>> {
         &self.mount_points
```

### `src/types/_task_container_properties.rs`

```diff
--- reference/src/types/_task_container_properties.rs
+++ generated/src/types/_task_container_properties.rs
@@ -20,7 +20,7 @@
     /// <p>The FireLens configuration for the container. This is used to specify and configure a log router for container logs. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/using_firelens.html">Custom log</a> routing in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
     pub firelens_configuration: ::std::option::Option<super::super::types::FirelensConfiguration>,
     /// <p>The image used to start a container. This string is passed directly to the Docker daemon. By default, images in the Docker Hub registry are available. Other repositories are specified with either <code>repository-url/image:tag</code> or <code>repository-url/image@digest</code>. Up to 255 letters (uppercase and lowercase), numbers, hyphens, underscores, colons, periods, forward slashes, and number signs are allowed. This parameter maps to <code>Image</code> in the <a href="https://docs.docker.com/engine/api/latest/#tag/Container/operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/latest/">Docker Remote API</a> and the <code>IMAGE</code> parameter of the <a href="https://docs.docker.com/engine/reference/run/#security-configuration"> <i>docker run</i> </a>.</p>
-    pub image: ::std::option::Option<::std::string::String>,
+    pub image: ::std::string::String,
     /// <p>Linux-specific modifications that are applied to the container, such as Linux kernel capabilities. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_KernelCapabilities.html">KernelCapabilities</a>.</p>
     pub linux_parameters: ::std::option::Option<super::super::types::LinuxParameters>,
     /// <p>The log configuration specification for the container.</p>
@@ -33,7 +33,7 @@
     /// </note>
     pub log_configuration: ::std::option::Option<super::super::types::LogConfiguration>,
     /// <p>The mount points for data volumes in your container.</p>
-    /// <p>This parameter maps to <code>Volumes</code> in the <a href="https://docs.docker.com/engine/api/latest/#tag/Container/operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/latest/">Docker Remote API</a> and the <code>--volume</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p>
+    /// <p>This parameter maps to <code>Volumes</code> in the <a href="https://docs.docker.com/engine/api/latest/#tag/Container/operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/latest/">Docker Remote API</a> and the <a href="">--volume</a> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p>
     /// <p>Windows containers can mount whole directories on the same drive as <code>$env:ProgramData</code>. Windows containers can't mount directories on a different drive, and mount point can't be across drives.</p>
     pub mount_points: ::std::option::Option<::std::vec::Vec<super::super::types::MountPoint>>,
     /// <p>The name of a container. The name can be used as a unique identifier to target your <code>dependsOn</code> and <code>Overrides</code> objects.</p>
@@ -117,8 +117,9 @@
         self.firelens_configuration.as_ref()
     }
     /// <p>The image used to start a container. This string is passed directly to the Docker daemon. By default, images in the Docker Hub registry are available. Other repositories are specified with either <code>repository-url/image:tag</code> or <code>repository-url/image@digest</code>. Up to 255 letters (uppercase and lowercase), numbers, hyphens, underscores, colons, periods, forward slashes, and number signs are allowed. This parameter maps to <code>Image</code> in the <a href="https://docs.docker.com/engine/api/latest/#tag/Container/operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/latest/">Docker Remote API</a> and the <code>IMAGE</code> parameter of the <a href="https://docs.docker.com/engine/reference/run/#security-configuration"> <i>docker run</i> </a>.</p>
-    pub fn image(&self) -> ::std::option::Option<&str> {
-        self.image.as_deref()
+    pub fn image(&self) -> &str {
+        use std::ops::Deref;
+        self.image.deref()
     }
     /// <p>Linux-specific modifications that are applied to the container, such as Linux kernel capabilities. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_KernelCapabilities.html">KernelCapabilities</a>.</p>
     pub fn linux_parameters(&self) -> ::std::option::Option<&super::super::types::LinuxParameters> {
@@ -136,7 +137,7 @@
         self.log_configuration.as_ref()
     }
     /// <p>The mount points for data volumes in your container.</p>
-    /// <p>This parameter maps to <code>Volumes</code> in the <a href="https://docs.docker.com/engine/api/latest/#tag/Container/operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/latest/">Docker Remote API</a> and the <code>--volume</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p>
+    /// <p>This parameter maps to <code>Volumes</code> in the <a href="https://docs.docker.com/engine/api/latest/#tag/Container/operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/latest/">Docker Remote API</a> and the <a href="">--volume</a> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p>
     /// <p>Windows containers can mount whole directories on the same drive as <code>$env:ProgramData</code>. Windows containers can't mount directories on a different drive, and mount point can't be across drives.</p>
     ///
     /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.mount_points.is_none()`.
@@ -421,7 +422,7 @@
     /// To override the contents of this collection use [`set_mount_points`](Self::set_mount_points).
     ///
     /// <p>The mount points for data volumes in your container.</p>
-    /// <p>This parameter maps to <code>Volumes</code> in the <a href="https://docs.docker.com/engine/api/latest/#tag/Container/operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/latest/">Docker Remote API</a> and the <code>--volume</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p>
+    /// <p>This parameter maps to <code>Volumes</code> in the <a href="https://docs.docker.com/engine/api/latest/#tag/Container/operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/latest/">Docker Remote API</a> and the <a href="">--volume</a> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p>
     /// <p>Windows containers can mount whole directories on the same drive as <code>$env:ProgramData</code>. Windows containers can't mount directories on a different drive, and mount point can't be across drives.</p>
     pub fn mount_points(mut self, input: super::super::types::MountPoint) -> Self {
         let mut v = self.mount_points.unwrap_or_default();
@@ -430,7 +431,7 @@
         self
     }
     /// <p>The mount points for data volumes in your container.</p>
-    /// <p>This parameter maps to <code>Volumes</code> in the <a href="https://docs.docker.com/engine/api/latest/#tag/Container/operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/latest/">Docker Remote API</a> and the <code>--volume</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p>
+    /// <p>This parameter maps to <code>Volumes</code> in the <a href="https://docs.docker.com/engine/api/latest/#tag/Container/operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/latest/">Docker Remote API</a> and the <a href="">--volume</a> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p>
     /// <p>Windows containers can mount whole directories on the same drive as <code>$env:ProgramData</code>. Windows containers can't mount directories on a different drive, and mount point can't be across drives.</p>
     pub fn set_mount_points(mut self, input: ::std::option::Option<::std::vec::Vec<super::super::types::MountPoint>>) -> Self {
         self.mount_points = input;
@@ -437,7 +438,7 @@
         self
     }
     /// <p>The mount points for data volumes in your container.</p>
-    /// <p>This parameter maps to <code>Volumes</code> in the <a href="https://docs.docker.com/engine/api/latest/#tag/Container/operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/latest/">Docker Remote API</a> and the <code>--volume</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p>
+    /// <p>This parameter maps to <code>Volumes</code> in the <a href="https://docs.docker.com/engine/api/latest/#tag/Container/operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/latest/">Docker Remote API</a> and the <a href="">--volume</a> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p>
     /// <p>Windows containers can mount whole directories on the same drive as <code>$env:ProgramData</code>. Windows containers can't mount directories on a different drive, and mount point can't be across drives.</p>
     pub fn get_mount_points(&self) -> &::std::option::Option<::std::vec::Vec<super::super::types::MountPoint>> {
         &self.mount_points
@@ -682,14 +683,21 @@
         &self.stop_timeout
     }
     /// Consumes the builder and constructs a [`TaskContainerProperties`](crate::types::TaskContainerProperties).
-    pub fn build(self) -> super::super::types::TaskContainerProperties {
-        super::super::types::TaskContainerProperties {
+    /// This method will fail if any of the following fields are not set:
+    /// - [`image`](crate::types::builders::TaskContainerPropertiesBuilder::image)
+    pub fn build(self) -> ::std::result::Result<super::super::types::TaskContainerProperties, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::TaskContainerProperties {
             command: self.command,
             depends_on: self.depends_on,
             environment: self.environment,
             essential: self.essential,
             firelens_configuration: self.firelens_configuration,
-            image: self.image,
+            image: self.image.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "image",
+                    "image was not specified but it is required when building TaskContainerProperties",
+                )
+            })?,
             linux_parameters: self.linux_parameters,
             log_configuration: self.log_configuration,
             mount_points: self.mount_points,
@@ -703,6 +711,6 @@
             user: self.user,
             start_timeout: self.start_timeout,
             stop_timeout: self.stop_timeout,
-        }
+        })
     }
 }
```

### `src/types/_tmpfs.rs`

```diff
--- reference/src/types/_tmpfs.rs
+++ generated/src/types/_tmpfs.rs
@@ -7,9 +7,9 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct Tmpfs {
     /// <p>The absolute file path in the container where the <code>tmpfs</code> volume is mounted.</p>
-    pub container_path: ::std::option::Option<::std::string::String>,
+    pub container_path: ::std::string::String,
     /// <p>The size (in MiB) of the <code>tmpfs</code> volume.</p>
-    pub size: ::std::option::Option<i32>,
+    pub size: i32,
     /// <p>The list of <code>tmpfs</code> volume mount options.</p>
     /// <p>Valid values: "<code>defaults</code>" | "<code>ro</code>" | "<code>rw</code>" | "<code>suid</code>" | "<code>nosuid</code>" | "<code>dev</code>" | "<code>nodev</code>" | "<code>exec</code>" | "<code>noexec</code>" | "<code>sync</code>" | "<code>async</code>" | "<code>dirsync</code>" | "<code>remount</code>" | "<code>mand</code>" | "<code>nomand</code>" | "<code>atime</code>" | "<code>noatime</code>" | "<code>diratime</code>" | "<code>nodiratime</code>" | "<code>bind</code>" | "<code>rbind" | "unbindable" | "runbindable" | "private" | "rprivate" | "shared" | "rshared" | "slave" | "rslave" | "relatime</code>" | "<code>norelatime</code>" | "<code>strictatime</code>" | "<code>nostrictatime</code>" | "<code>mode</code>" | "<code>uid</code>" | "<code>gid</code>" | "<code>nr_inodes</code>" | "<code>nr_blocks</code>" | "<code>mpol</code>"</p>
     pub mount_options: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
@@ -16,11 +16,12 @@
 }
 impl Tmpfs {
     /// <p>The absolute file path in the container where the <code>tmpfs</code> volume is mounted.</p>
-    pub fn container_path(&self) -> ::std::option::Option<&str> {
-        self.container_path.as_deref()
+    pub fn container_path(&self) -> &str {
+        use std::ops::Deref;
+        self.container_path.deref()
     }
     /// <p>The size (in MiB) of the <code>tmpfs</code> volume.</p>
-    pub fn size(&self) -> ::std::option::Option<i32> {
+    pub fn size(&self) -> i32 {
         self.size
     }
     /// <p>The list of <code>tmpfs</code> volume mount options.</p>
@@ -101,11 +102,24 @@
         &self.mount_options
     }
     /// Consumes the builder and constructs a [`Tmpfs`](crate::types::Tmpfs).
-    pub fn build(self) -> super::super::types::Tmpfs {
-        super::super::types::Tmpfs {
-            container_path: self.container_path,
-            size: self.size,
+    /// This method will fail if any of the following fields are not set:
+    /// - [`container_path`](crate::types::builders::TmpfsBuilder::container_path)
+    /// - [`size`](crate::types::builders::TmpfsBuilder::size)
+    pub fn build(self) -> ::std::result::Result<super::super::types::Tmpfs, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::Tmpfs {
+            container_path: self.container_path.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "container_path",
+                    "container_path was not specified but it is required when building Tmpfs",
+                )
+            })?,
+            size: self.size.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "size",
+                    "size was not specified but it is required when building Tmpfs",
+                )
+            })?,
             mount_options: self.mount_options,
-        }
+        })
     }
 }
```

### `src/types/_ulimit.rs`

```diff
--- reference/src/types/_ulimit.rs
+++ generated/src/types/_ulimit.rs
@@ -7,23 +7,24 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct Ulimit {
     /// <p>The hard limit for the <code>ulimit</code> type.</p>
-    pub hard_limit: ::std::option::Option<i32>,
+    pub hard_limit: i32,
     /// <p>The <code>type</code> of the <code>ulimit</code>. Valid values are: <code>core</code> | <code>cpu</code> | <code>data</code> | <code>fsize</code> | <code>locks</code> | <code>memlock</code> | <code>msgqueue</code> | <code>nice</code> | <code>nofile</code> | <code>nproc</code> | <code>rss</code> | <code>rtprio</code> | <code>rttime</code> | <code>sigpending</code> | <code>stack</code>.</p>
-    pub name: ::std::option::Option<::std::string::String>,
+    pub name: ::std::string::String,
     /// <p>The soft limit for the <code>ulimit</code> type.</p>
-    pub soft_limit: ::std::option::Option<i32>,
+    pub soft_limit: i32,
 }
 impl Ulimit {
     /// <p>The hard limit for the <code>ulimit</code> type.</p>
-    pub fn hard_limit(&self) -> ::std::option::Option<i32> {
+    pub fn hard_limit(&self) -> i32 {
         self.hard_limit
     }
     /// <p>The <code>type</code> of the <code>ulimit</code>. Valid values are: <code>core</code> | <code>cpu</code> | <code>data</code> | <code>fsize</code> | <code>locks</code> | <code>memlock</code> | <code>msgqueue</code> | <code>nice</code> | <code>nofile</code> | <code>nproc</code> | <code>rss</code> | <code>rtprio</code> | <code>rttime</code> | <code>sigpending</code> | <code>stack</code>.</p>
-    pub fn name(&self) -> ::std::option::Option<&str> {
-        self.name.as_deref()
+    pub fn name(&self) -> &str {
+        use std::ops::Deref;
+        self.name.deref()
     }
     /// <p>The soft limit for the <code>ulimit</code> type.</p>
-    pub fn soft_limit(&self) -> ::std::option::Option<i32> {
+    pub fn soft_limit(&self) -> i32 {
         self.soft_limit
     }
 }
@@ -89,11 +90,30 @@
         &self.soft_limit
     }
     /// Consumes the builder and constructs a [`Ulimit`](crate::types::Ulimit).
-    pub fn build(self) -> super::super::types::Ulimit {
-        super::super::types::Ulimit {
-            hard_limit: self.hard_limit,
-            name: self.name,
-            soft_limit: self.soft_limit,
-        }
+    /// This method will fail if any of the following fields are not set:
+    /// - [`hard_limit`](crate::types::builders::UlimitBuilder::hard_limit)
+    /// - [`name`](crate::types::builders::UlimitBuilder::name)
+    /// - [`soft_limit`](crate::types::builders::UlimitBuilder::soft_limit)
+    pub fn build(self) -> ::std::result::Result<super::super::types::Ulimit, ::aws_smithy_types::error::operation::BuildError> {
+        ::std::result::Result::Ok(super::super::types::Ulimit {
+            hard_limit: self.hard_limit.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "hard_limit",
+                    "hard_limit was not specified but it is required when building Ulimit",
+                )
+            })?,
+            name: self.name.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "name",
+                    "name was not specified but it is required when building Ulimit",
+                )
+            })?,
+            soft_limit: self.soft_limit.ok_or_else(|| {
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "soft_limit",
+                    "soft_limit was not specified but it is required when building Ulimit",
+                )
+            })?,
+        })
     }
 }
```
