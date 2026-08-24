# AWS SDK Conformance Report: batch

Snapshot: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`

## batch
**Progress:** `762/762` files compared · `703` matched · `59` mismatches · `0` missing · `0` extra · `92.26%` match (100.00% means fully matched)

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

### `src/client/list_jobs.rs`

```diff
--- reference/src/client/list_jobs.rs
+++ generated/src/client/list_jobs.rs
@@ -10,7 +10,7 @@
     ///   - [`job_status(JobStatus)`](crate::operation::list_jobs::builders::ListJobsFluentBuilder::job_status) / [`set_job_status(Option<JobStatus>)`](crate::operation::list_jobs::builders::ListJobsFluentBuilder::set_job_status):<br>required: **false**<br><p>The job status used to filter jobs in the specified queue. If the <code>filters</code> parameter is specified, the <code>jobStatus</code> parameter is ignored and jobs with any status are returned. The exception is the <code>SHARE_IDENTIFIER</code> filter and <code>jobStatus</code> can be used together. If you don't specify a status, only <code>RUNNING</code> jobs are returned.</p><note>  <p>Array job parents are updated to <code>PENDING</code> when any child job is updated to <code>RUNNABLE</code> and remain in <code>PENDING</code> status while child jobs are running. To view these jobs, filter by <code>PENDING</code> status until all child jobs reach a terminal state.</p> </note><br>
     ///   - [`max_results(i32)`](crate::operation::list_jobs::builders::ListJobsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_jobs::builders::ListJobsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results returned by <code>ListJobs</code> in a paginated output. When this parameter is used, <code>ListJobs</code> returns up to <code>maxResults</code> results in a single page and a <code>nextToken</code> response element, if applicable. The remaining results of the initial request can be seen by sending another <code>ListJobs</code> request with the returned <code>nextToken</code> value.</p> <p>The following outlines key parameters and limitations:</p> <ul>  <li>   <p>The minimum value is 1.</p></li>  <li>   <p>When <code>--job-status</code> is used, Batch returns up to 1000 values.</p></li>  <li>   <p>When <code>--filters</code> is used, Batch returns up to 100 values.</p></li>  <li>   <p>If neither parameter is used, then <code>ListJobs</code> returns up to 1000 results (jobs that are in the <code>RUNNING</code> status) and a <code>nextToken</code> value, if applicable.</p></li> </ul><br>
     ///   - [`next_token(impl Into<String>)`](crate::operation::list_jobs::builders::ListJobsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_jobs::builders::ListJobsFluentBuilder::set_next_token):<br>required: **false**<br><p>The <code>nextToken</code> value returned from a previous paginated <code>ListJobs</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. This value is <code>null</code> when there are no more results to return.</p><note>  <p>Treat this token as an opaque identifier that's only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note><br>
-    ///   - [`filters(KeyValuesPair)`](crate::operation::list_jobs::builders::ListJobsFluentBuilder::filters) / [`set_filters(Option<Vec::<KeyValuesPair>>)`](crate::operation::list_jobs::builders::ListJobsFluentBuilder::set_filters):<br>required: **false**<br><p>The filter to apply to the query. Only one filter can be used at a time. When the filter is used, <code>jobStatus</code> is ignored with the exception that <code>SHARE_IDENTIFIER</code> and <code>jobStatus</code> can be used together. The filter doesn't apply to child jobs in an array or multi-node parallel (MNP) jobs. The results are sorted by the <code>createdAt</code> field, with the most recent jobs being first.</p><note>  <p>The <code>SHARE_IDENTIFIER</code> filter and the <code>jobStatus</code> field can be used together to filter results.</p> </note> <dl>  <dt>   JOB_NAME  </dt>  <dd>   <p>The value of the filter is a case-insensitive match for the job name. If the value ends with an asterisk (*), the filter matches any job name that begins with the string before the '*'. This corresponds to the <code>jobName</code> value. For example, <code>test1</code> matches both <code>Test1</code> and <code>test1</code>, and <code>test1*</code> matches both <code>test1</code> and <code>Test10</code>. When the <code>JOB_NAME</code> filter is used, the results are grouped by the job name and version.</p>  </dd>  <dt>   JOB_DEFINITION  </dt>  <dd>   <p>The value for the filter is the name or Amazon Resource Name (ARN) of the job definition. This corresponds to the <code>jobDefinition</code> value. The value is case sensitive. When the value for the filter is the job definition name, the results include all the jobs that used any revision of that job definition name. If the value ends with an asterisk (*), the filter matches any job definition name that begins with the string before the '*'. For example, <code>jd1</code> matches only <code>jd1</code>, and <code>jd1*</code> matches both <code>jd1</code> and <code>jd1A</code>. The version of the job definition that's used doesn't affect the sort order. When the <code>JOB_DEFINITION</code> filter is used and the ARN is used (which is in the form <code>arn:${Partition}:batch:${Region}:${Account}:job-definition/${JobDefinitionName}:${Revision}</code>), the results include jobs that used the specified revision of the job definition. Asterisk (*) isn't supported when the ARN is used.</p>  </dd>  <dt>   BEFORE_CREATED_AT  </dt>  <dd>   <p>The value for the filter is the time that's before the job was created. This corresponds to the <code>createdAt</code> value. The value is a string representation of the number of milliseconds since 00:00:00 UTC (midnight) on January 1, 1970.</p>  </dd>  <dt>   AFTER_CREATED_AT  </dt>  <dd>   <p>The value for the filter is the time that's after the job was created. This corresponds to the <code>createdAt</code> value. The value is a string representation of the number of milliseconds since 00:00:00 UTC (midnight) on January 1, 1970.</p>  </dd>  <dt>   SHARE_IDENTIFIER  </dt>  <dd>   <p>The value for the filter is the fairshare scheduling share identifier.</p>  </dd> </dl><br>
+    ///   - [`filters(KeyValuesPair)`](crate::operation::list_jobs::builders::ListJobsFluentBuilder::filters) / [`set_filters(Option<Vec::<KeyValuesPair>>)`](crate::operation::list_jobs::builders::ListJobsFluentBuilder::set_filters):<br>required: **false**<br><p>The filter to apply to the query. Only one filter can be used at a time. When the filter is used, <code>jobStatus</code> is ignored with the exception that <code>SHARE_IDENTIFIER</code> and <code>jobStatus</code> can be used together. The filter doesn't apply to child jobs in an array or multi-node parallel (MNP) jobs. The results are sorted by the <code>createdAt</code> field, with the most recent jobs being first.</p><note>  <p>The <code>SHARE_IDENTIFIER</code> filter and the <code>jobStatus</code> field can be used together to filter results.</p> </note> <dl> <dt>JOB_NAME</dt> <dd> <p>The value of the filter is a case-insensitive match for the job name. If the value ends with an asterisk (*), the filter matches any job name that begins with the string before the '*'. This corresponds to the <code>jobName</code> value. For example, <code>test1</code> matches both <code>Test1</code> and <code>test1</code>, and <code>test1*</code> matches both <code>test1</code> and <code>Test10</code>. When the <code>JOB_NAME</code> filter is used, the results are grouped by the job name and version.</p></dd> <dt>JOB_DEFINITION</dt> <dd> <p>The value for the filter is the name or Amazon Resource Name (ARN) of the job definition. This corresponds to the <code>jobDefinition</code> value. The value is case sensitive. When the value for the filter is the job definition name, the results include all the jobs that used any revision of that job definition name. If the value ends with an asterisk (*), the filter matches any job definition name that begins with the string before the '*'. For example, <code>jd1</code> matches only <code>jd1</code>, and <code>jd1*</code> matches both <code>jd1</code> and <code>jd1A</code>. The version of the job definition that's used doesn't affect the sort order. When the <code>JOB_DEFINITION</code> filter is used and the ARN is used (which is in the form <code>arn:${Partition}:batch:${Region}:${Account}:job-definition/${JobDefinitionName}:${Revision}</code>), the results include jobs that used the specified revision of the job definition. Asterisk (*) isn't supported when the ARN is used.</p></dd> <dt>BEFORE_CREATED_AT</dt> <dd> <p>The value for the filter is the time that's before the job was created. This corresponds to the <code>createdAt</code> value. The value is a string representation of the number of milliseconds since 00:00:00 UTC (midnight) on January 1, 1970.</p></dd> <dt>AFTER_CREATED_AT</dt> <dd> <p>The value for the filter is the time that's after the job was created. This corresponds to the <code>createdAt</code> value. The value is a string representation of the number of milliseconds since 00:00:00 UTC (midnight) on January 1, 1970.</p></dd> <dt>SHARE_IDENTIFIER</dt> <dd> <p>The value for the filter is the fairshare scheduling share identifier.</p></dd></dl><br>
     /// - On success, responds with [`ListJobsOutput`](crate::operation::list_jobs::ListJobsOutput) with field(s):
     ///   - [`job_summary_list(Option<Vec::<JobSummary>>)`](crate::operation::list_jobs::ListJobsOutput::job_summary_list): <p>A list of job summaries that match the request.</p>
     ///   - [`next_token(Option<String>)`](crate::operation::list_jobs::ListJobsOutput::next_token): <p>The <code>nextToken</code> value to include in a future <code>ListJobs</code> request. When the results of a <code>ListJobs</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
```

### `src/client/list_service_jobs.rs`

```diff
--- reference/src/client/list_service_jobs.rs
+++ generated/src/client/list_service_jobs.rs
@@ -8,7 +8,7 @@
     ///   - [`job_status(ServiceJobStatus)`](crate::operation::list_service_jobs::builders::ListServiceJobsFluentBuilder::job_status) / [`set_job_status(Option<ServiceJobStatus>)`](crate::operation::list_service_jobs::builders::ListServiceJobsFluentBuilder::set_job_status):<br>required: **false**<br><p>The job status used to filter service jobs in the specified queue. If the <code>filters</code> parameter is specified, the <code>jobStatus</code> parameter is ignored and jobs with any status are returned. The exceptions are the <code>SHARE_IDENTIFIER</code> filter and <code>QUOTA_SHARE_NAME</code> filter, which can be used with <code>jobStatus</code>. If you don't specify a status, only <code>RUNNING</code> jobs are returned.</p><note>  <p>The <code>SHARE_IDENTIFIER</code> filter or <code>QUOTA_SHARE_NAME</code> filter can be used with the <code>jobStatus</code> field to filter results.</p> </note><br>
     ///   - [`max_results(i32)`](crate::operation::list_service_jobs::builders::ListServiceJobsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_service_jobs::builders::ListServiceJobsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results returned by <code>ListServiceJobs</code> in paginated output. When this parameter is used, <code>ListServiceJobs</code> only returns <code>maxResults</code> results in a single page and a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListServiceJobs</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter isn't used, then <code>ListServiceJobs</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p><br>
     ///   - [`next_token(impl Into<String>)`](crate::operation::list_service_jobs::builders::ListServiceJobsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_service_jobs::builders::ListServiceJobsFluentBuilder::set_next_token):<br>required: **false**<br><p>The <code>nextToken</code> value returned from a previous paginated <code>ListServiceJobs</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. This value is <code>null</code> when there are no more results to return.</p><note>  <p>Treat this token as an opaque identifier that's only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note><br>
-    ///   - [`filters(KeyValuesPair)`](crate::operation::list_service_jobs::builders::ListServiceJobsFluentBuilder::filters) / [`set_filters(Option<Vec::<KeyValuesPair>>)`](crate::operation::list_service_jobs::builders::ListServiceJobsFluentBuilder::set_filters):<br>required: **false**<br><p>The filter to apply to the query. Only one filter can be used at a time. When the filter is used, <code>jobStatus</code> is ignored with the exception that <code>SHARE_IDENTIFIER</code> or <code>QUOTA_SHARE_NAME</code> and <code>jobStatus</code> can be used together. The results are sorted by the <code>createdAt</code> field, with the most recent jobs being first.</p><note>  <p>The <code>SHARE_IDENTIFIER</code> or <code>QUOTA_SHARE_NAME</code> filter and the <code>jobStatus</code> field can be used together to filter results.</p> </note> <dl>  <dt>   JOB_NAME  </dt>  <dd>   <p>The value of the filter is a case-insensitive match for the job name. If the value ends with an asterisk (*), the filter matches any job name that begins with the string before the '*'. This corresponds to the <code>jobName</code> value. For example, <code>test1</code> matches both <code>Test1</code> and <code>test1</code>, and <code>test1*</code> matches both <code>test1</code> and <code>Test10</code>. When the <code>JOB_NAME</code> filter is used, the results are grouped by the job name and version.</p>  </dd>  <dt>   BEFORE_CREATED_AT  </dt>  <dd>   <p>The value for the filter is the time that's before the job was created. This corresponds to the <code>createdAt</code> value. The value is a string representation of the number of milliseconds since 00:00:00 UTC (midnight) on January 1, 1970.</p>  </dd>  <dt>   AFTER_CREATED_AT  </dt>  <dd>   <p>The value for the filter is the time that's after the job was created. This corresponds to the <code>createdAt</code> value. The value is a string representation of the number of milliseconds since 00:00:00 UTC (midnight) on January 1, 1970.</p>  </dd>  <dt>   SHARE_IDENTIFIER  </dt>  <dd>   <p>The value for the filter is the fairshare scheduling share identifier.</p>  </dd>  <dt>   QUOTA_SHARE_NAME  </dt>  <dd>   <p>The value for the filter is the quota management share name.</p>  </dd> </dl><br>
+    ///   - [`filters(KeyValuesPair)`](crate::operation::list_service_jobs::builders::ListServiceJobsFluentBuilder::filters) / [`set_filters(Option<Vec::<KeyValuesPair>>)`](crate::operation::list_service_jobs::builders::ListServiceJobsFluentBuilder::set_filters):<br>required: **false**<br><p>The filter to apply to the query. Only one filter can be used at a time. When the filter is used, <code>jobStatus</code> is ignored with the exception that <code>SHARE_IDENTIFIER</code> or <code>QUOTA_SHARE_NAME</code> and <code>jobStatus</code> can be used together. The results are sorted by the <code>createdAt</code> field, with the most recent jobs being first.</p><note>  <p>The <code>SHARE_IDENTIFIER</code> or <code>QUOTA_SHARE_NAME</code> filter and the <code>jobStatus</code> field can be used together to filter results.</p> </note> <dl> <dt>JOB_NAME</dt> <dd> <p>The value of the filter is a case-insensitive match for the job name. If the value ends with an asterisk (*), the filter matches any job name that begins with the string before the '*'. This corresponds to the <code>jobName</code> value. For example, <code>test1</code> matches both <code>Test1</code> and <code>test1</code>, and <code>test1*</code> matches both <code>test1</code> and <code>Test10</code>. When the <code>JOB_NAME</code> filter is used, the results are grouped by the job name and version.</p></dd> <dt>BEFORE_CREATED_AT</dt> <dd> <p>The value for the filter is the time that's before the job was created. This corresponds to the <code>createdAt</code> value. The value is a string representation of the number of milliseconds since 00:00:00 UTC (midnight) on January 1, 1970.</p></dd> <dt>AFTER_CREATED_AT</dt> <dd> <p>The value for the filter is the time that's after the job was created. This corresponds to the <code>createdAt</code> value. The value is a string representation of the number of milliseconds since 00:00:00 UTC (midnight) on January 1, 1970.</p></dd> <dt>SHARE_IDENTIFIER</dt> <dd> <p>The value for the filter is the fairshare scheduling share identifier.</p></dd> <dt>QUOTA_SHARE_NAME</dt> <dd> <p>The value for the filter is the quota management share name.</p></dd></dl><br>
     /// - On success, responds with [`ListServiceJobsOutput`](crate::operation::list_service_jobs::ListServiceJobsOutput) with field(s):
     ///   - [`job_summary_list(Option<Vec::<ServiceJobSummary>>)`](crate::operation::list_service_jobs::ListServiceJobsOutput::job_summary_list): <p>A list of service job summaries.</p>
     ///   - [`next_token(Option<String>)`](crate::operation::list_service_jobs::ListServiceJobsOutput::next_token): <p>The <code>nextToken</code> value to include in a future <code>ListServiceJobs</code> request. When the results of a <code>ListServiceJobs</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
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
@@ -103,15 +103,15 @@
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
@@ -103,15 +103,15 @@
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
@@ -103,15 +103,15 @@
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
@@ -99,10 +99,10 @@
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
@@ -113,22 +113,22 @@
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
@@ -141,6 +141,13 @@
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
@@ -148,13 +155,6 @@
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
@@ -160,27 +160,6 @@
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
@@ -202,6 +181,13 @@
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
@@ -209,20 +195,34 @@
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

### `src/protocol_serde/shape_ephemeral_storage.rs`

```diff
--- reference/src/protocol_serde/shape_ephemeral_storage.rs
+++ generated/src/protocol_serde/shape_ephemeral_storage.rs
@@ -3,7 +3,7 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::EphemeralStorage,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.size_in_gib {
+    if let Some(var_1) = &input.size_in_gi_b {
         object.key("sizeInGiB").number(
             #[allow(clippy::useless_conversion)]
             ::aws_smithy_types::Number::NegInt((*var_1).into()),
@@ -35,7 +35,7 @@
                     Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                     Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                         "sizeInGiB" => {
-                            builder = builder.set_size_in_gib(
+                            builder = builder.set_size_in_gi_b(
                                 ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                     .map(i32::try_from)
                                     .transpose()?,
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

### `src/protocol_serde/shape_register_job_definition.rs`

```diff
--- reference/src/protocol_serde/shape_register_job_definition.rs
+++ generated/src/protocol_serde/shape_register_job_definition.rs
@@ -99,15 +99,15 @@
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
+    if let Some(var_1) = &input.attempts {
+        object.key("attempts").number(
+            #[allow(clippy::useless_conversion)]
+            ::aws_smithy_types::Number::NegInt((*var_1).into()),
+        );
+    }
+    if let Some(var_2) = &input.evaluate_on_exit {
+        let mut array_3 = object.key("evaluateOnExit").start_array();
+        for item_4 in var_2 {
+            {
+                #[allow(unused_mut)]
+                let mut object_5 = array_3.value().start_object();
+                super::super::protocol_serde::shape_service_job_evaluate_on_exit::ser_service_job_evaluate_on_exit(&mut object_5, item_4)?;
+                object_5.finish();
+            }
+        }
+        array_3.finish();
+    }
+    Ok(())
+}
+
 pub(crate) fn de_service_job_retry_strategy<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
@@ -53,28 +78,3 @@
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

### `src/protocol_serde/shape_submit_job.rs`

```diff
--- reference/src/protocol_serde/shape_submit_job.rs
+++ generated/src/protocol_serde/shape_submit_job.rs
@@ -97,15 +97,15 @@
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
@@ -103,15 +103,15 @@
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
@@ -103,15 +103,15 @@
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
+    pub size_in_gi_b: ::std::option::Option<i32>,
 }
 impl EphemeralStorage {
     /// <p>The total amount, in GiB, of ephemeral storage to set for the task. The minimum supported value is <code>21</code> GiB and the maximum supported value is <code>200</code> GiB.</p>
-    pub fn size_in_gib(&self) -> ::std::option::Option<i32> {
-        self.size_in_gib
+    pub fn size_in_gi_b(&self) -> ::std::option::Option<i32> {
+        self.size_in_gi_b
     }
 }
 impl EphemeralStorage {
@@ -24,28 +24,28 @@
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
     pub fn build(self) -> super::super::types::EphemeralStorage {
         super::super::types::EphemeralStorage {
-            size_in_gib: self.size_in_gib,
+            size_in_gi_b: self.size_in_gi_b,
         }
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
@@ -33,7 +33,7 @@
     /// </note>
     pub log_configuration: ::std::option::Option<super::super::types::LogConfiguration>,
     /// <p>The mount points for data volumes in your container.</p>
-    /// <p>This parameter maps to <code>Volumes</code> in the <a href="https://docs.docker.com/engine/api/latest/#tag/Container/operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/latest/">Docker Remote API</a> and the <code>--volume</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p>
+    /// <p>This parameter maps to <code>Volumes</code> in the <a href="https://docs.docker.com/engine/api/latest/#tag/Container/operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/latest/">Docker Remote API</a> and the <a href="">--volume</a> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p>
     /// <p>Windows containers can mount whole directories on the same drive as <code>$env:ProgramData</code>. Windows containers can't mount directories on a different drive, and mount point can't be across drives.</p>
     pub mount_points: ::std::option::Option<::std::vec::Vec<super::super::types::MountPoint>>,
     /// <p>The name of a container. The name can be used as a unique identifier to target your <code>dependsOn</code> and <code>Overrides</code> objects.</p>
@@ -136,7 +136,7 @@
         self.log_configuration.as_ref()
     }
     /// <p>The mount points for data volumes in your container.</p>
-    /// <p>This parameter maps to <code>Volumes</code> in the <a href="https://docs.docker.com/engine/api/latest/#tag/Container/operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/latest/">Docker Remote API</a> and the <code>--volume</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p>
+    /// <p>This parameter maps to <code>Volumes</code> in the <a href="https://docs.docker.com/engine/api/latest/#tag/Container/operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/latest/">Docker Remote API</a> and the <a href="">--volume</a> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p>
     /// <p>Windows containers can mount whole directories on the same drive as <code>$env:ProgramData</code>. Windows containers can't mount directories on a different drive, and mount point can't be across drives.</p>
     ///
     /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.mount_points.is_none()`.
@@ -421,7 +421,7 @@
     /// To override the contents of this collection use [`set_mount_points`](Self::set_mount_points).
     ///
     /// <p>The mount points for data volumes in your container.</p>
-    /// <p>This parameter maps to <code>Volumes</code> in the <a href="https://docs.docker.com/engine/api/latest/#tag/Container/operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/latest/">Docker Remote API</a> and the <code>--volume</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p>
+    /// <p>This parameter maps to <code>Volumes</code> in the <a href="https://docs.docker.com/engine/api/latest/#tag/Container/operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/latest/">Docker Remote API</a> and the <a href="">--volume</a> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p>
     /// <p>Windows containers can mount whole directories on the same drive as <code>$env:ProgramData</code>. Windows containers can't mount directories on a different drive, and mount point can't be across drives.</p>
     pub fn mount_points(mut self, input: super::super::types::MountPoint) -> Self {
         let mut v = self.mount_points.unwrap_or_default();
@@ -430,7 +430,7 @@
         self
     }
     /// <p>The mount points for data volumes in your container.</p>
-    /// <p>This parameter maps to <code>Volumes</code> in the <a href="https://docs.docker.com/engine/api/latest/#tag/Container/operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/latest/">Docker Remote API</a> and the <code>--volume</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p>
+    /// <p>This parameter maps to <code>Volumes</code> in the <a href="https://docs.docker.com/engine/api/latest/#tag/Container/operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/latest/">Docker Remote API</a> and the <a href="">--volume</a> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p>
     /// <p>Windows containers can mount whole directories on the same drive as <code>$env:ProgramData</code>. Windows containers can't mount directories on a different drive, and mount point can't be across drives.</p>
     pub fn set_mount_points(mut self, input: ::std::option::Option<::std::vec::Vec<super::super::types::MountPoint>>) -> Self {
         self.mount_points = input;
@@ -437,7 +437,7 @@
         self
     }
     /// <p>The mount points for data volumes in your container.</p>
-    /// <p>This parameter maps to <code>Volumes</code> in the <a href="https://docs.docker.com/engine/api/latest/#tag/Container/operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/latest/">Docker Remote API</a> and the <code>--volume</code> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p>
+    /// <p>This parameter maps to <code>Volumes</code> in the <a href="https://docs.docker.com/engine/api/latest/#tag/Container/operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/latest/">Docker Remote API</a> and the <a href="">--volume</a> option to <a href="https://docs.docker.com/engine/reference/run/#security-configuration">docker run</a>.</p>
     /// <p>Windows containers can mount whole directories on the same drive as <code>$env:ProgramData</code>. Windows containers can't mount directories on a different drive, and mount point can't be across drives.</p>
     pub fn get_mount_points(&self) -> &::std::option::Option<::std::vec::Vec<super::super::types::MountPoint>> {
         &self.mount_points
```
