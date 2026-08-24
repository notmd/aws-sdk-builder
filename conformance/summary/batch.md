# AWS SDK Conformance Report: batch

Snapshot: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`

## batch
**Progress:** `762/762` files compared · `760` matched · `2` mismatches · `0` missing · `0` extra · `99.74%` match (100.00% means fully matched)

### `src/client/register_job_definition.rs`

```diff
--- reference/src/client/register_job_definition.rs
+++ generated/src/client/register_job_definition.rs
@@ -4,7 +4,7 @@
     ///
     /// - The fluent builder is configurable:
     ///   - [`job_definition_name(impl Into<String>)`](crate::operation::register_job_definition::builders::RegisterJobDefinitionFluentBuilder::job_definition_name) / [`set_job_definition_name(Option<String>)`](crate::operation::register_job_definition::builders::RegisterJobDefinitionFluentBuilder::set_job_definition_name):<br>required: **true**<br><p>The name of the job definition to register. It can be up to 128 letters long. It can contain uppercase and lowercase letters, numbers, hyphens (-), and underscores (_).</p><br>
-    ///   - [`r#type(JobDefinitionType)`](crate::operation::register_job_definition::builders::RegisterJobDefinitionFluentBuilder::type) / [`set_type(Option<JobDefinitionType>)`](crate::operation::register_job_definition::builders::RegisterJobDefinitionFluentBuilder::set_type):<br>required: **true**<br><p>The type of job definition. For more information about multi-node parallel jobs, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/multi-node-job-def.html">Creating a multi-node parallel job definition</a> in the <i>Batch User Guide</i>.</p> <ul>  <li>   <p>If the value is <code>container</code>, then one of the following is required: <code>containerProperties</code>, <code>ecsProperties</code>, or <code>eksProperties</code>.</p></li>  <li>   <p>If the value is <code>multinode</code>, then <code>nodeProperties</code> is required.</p></li> </ul><note>  <p>If the job is run on Fargate resources, then <code>multinode</code> isn't supported.</p> </note><br>
+    ///   - [`r#type(JobDefinitionType)`](crate::operation::register_job_definition::builders::RegisterJobDefinitionFluentBuilder::type) / [`set_type(Option<JobDefinitionType>)`](crate::operation::register_job_definition::builders::RegisterJobDefinitionFluentBuilder::set_type):<br>required: **true**<br><p>The type of job definition. For more information about multi-node parallel jobs, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/multi-node-job-def.html">Creating a multi-node parallel job definition</a> in the <i>Batch User Guide</i>.</p> <ul>  <li>   <p>If the value is <code>container</code>, then one of the following is required: <code>containerProperties</code>, <code>ecsProperties</code>, or <code>eksProperties</code>.</p></li>  <li>   <p>If the value is <code>multinode</code>, then <code>nodeProperties</code> is required.</p></li> </ul> <note>  <p>If the job is run on Fargate resources, then <code>multinode</code> isn't supported.</p> </note><br>
     ///   - [`parameters(impl Into<String>, impl Into<String>)`](crate::operation::register_job_definition::builders::RegisterJobDefinitionFluentBuilder::parameters) / [`set_parameters(Option<HashMap::<String, String>>)`](crate::operation::register_job_definition::builders::RegisterJobDefinitionFluentBuilder::set_parameters):<br>required: **false**<br><p>Default parameter substitution placeholders to set in the job definition. Parameters are specified as a key-value pair mapping. Parameters in a <code>SubmitJob</code> request override any corresponding parameter defaults from the job definition.</p><br>
     ///   - [`scheduling_priority(i32)`](crate::operation::register_job_definition::builders::RegisterJobDefinitionFluentBuilder::scheduling_priority) / [`set_scheduling_priority(Option<i32>)`](crate::operation::register_job_definition::builders::RegisterJobDefinitionFluentBuilder::set_scheduling_priority):<br>required: **false**<br><p>The scheduling priority for jobs that are submitted with this job definition. This only affects jobs in job queues with a fair-share policy. Jobs with a higher scheduling priority are scheduled before jobs with a lower scheduling priority.</p> <p>The minimum supported value is 0 and the maximum supported value is 9999.</p><br>
     ///   - [`container_properties(ContainerProperties)`](crate::operation::register_job_definition::builders::RegisterJobDefinitionFluentBuilder::container_properties) / [`set_container_properties(Option<ContainerProperties>)`](crate::operation::register_job_definition::builders::RegisterJobDefinitionFluentBuilder::set_container_properties):<br>required: **false**<br><p>An object with properties specific to Amazon ECS-based single-node container-based jobs. If the job definition's <code>type</code> parameter is <code>container</code>, then you must specify either <code>containerProperties</code> or <code>nodeProperties</code>. This must not be specified for Amazon EKS-based job definitions.</p><note>  <p>If the job runs on Fargate resources, then you must not specify <code>nodeProperties</code>; use only <code>containerProperties</code>.</p> </note><br>
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
