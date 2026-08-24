# AWS SDK Conformance Report: batch

Snapshot: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`

## batch
**Progress:** `762/762` files compared · `744` matched · `18` mismatches · `0` missing · `0` extra · `97.64%` match (100.00% means fully matched)

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
