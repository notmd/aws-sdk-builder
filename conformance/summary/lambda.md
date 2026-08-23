# AWS SDK Conformance Report: lambda

Snapshot: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`

## lambda
**Progress:** `1084/1084` files compared · `583` matched · `143` mismatches · `358` missing · `0` extra · `53.78%` match (100.00% means fully matched)

### `src/client/delete_resource_policy.rs`

```diff
--- reference/src/client/delete_resource_policy.rs
+++ generated/src/client/delete_resource_policy.rs
@@ -4,7 +4,7 @@
     ///
     /// - The fluent builder is configurable:
     ///   - [`resource_arn(impl Into<String>)`](crate::operation::delete_resource_policy::builders::DeleteResourcePolicyFluentBuilder::resource_arn) / [`set_resource_arn(Option<String>)`](crate::operation::delete_resource_policy::builders::DeleteResourcePolicyFluentBuilder::set_resource_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of the Lambda resource you want to delete the policy from. You can use a qualified or an unqualified ARN. The value must be a complete ARN, and the operation does not accept wildcard characters.</p><br>
-    ///   - [`revision_id(impl Into<String>)`](crate::operation::delete_resource_policy::builders::DeleteResourcePolicyFluentBuilder::revision_id) / [`set_revision_id(Option<String>)`](crate::operation::delete_resource_policy::builders::DeleteResourcePolicyFluentBuilder::set_revision_id):<br>required: **false**<br><p>The revision ID that the existing policy must match for the deletion to proceed. If the revision ID doesn't match, the operation fails with a <code>PreconditionFailedException</code> error. To retrieve the current revision ID, use the <code>GetResourcePolicy</code> operation.</p><br>
+    ///   - [`revision_id(impl Into<String>)`](crate::operation::delete_resource_policy::builders::DeleteResourcePolicyFluentBuilder::revision_id) / [`set_revision_id(Option<String>)`](crate::operation::delete_resource_policy::builders::DeleteResourcePolicyFluentBuilder::set_revision_id):<br>required: **false**<br><p>The revision ID that the existing policy must match for the deletion to proceed. If the revision ID doesn't match, the operation fails with a <code>PreconditionFailedException</code> error. To retrieve the current revision ID, use the <a>GetResourcePolicy</a> operation.</p><br>
     /// - On success, responds with [`DeleteResourcePolicyOutput`](crate::operation::delete_resource_policy::DeleteResourcePolicyOutput)
     /// - On failure, responds with [`SdkError<DeleteResourcePolicyError>`](crate::operation::delete_resource_policy::DeleteResourcePolicyError)
     pub fn delete_resource_policy(&self) -> crate::operation::delete_resource_policy::builders::DeleteResourcePolicyFluentBuilder {
```

### `src/client/get_function_event_invoke_config.rs`

```diff
--- reference/src/client/get_function_event_invoke_config.rs
+++ generated/src/client/get_function_event_invoke_config.rs
@@ -10,7 +10,7 @@
     ///   - [`function_arn(Option<String>)`](crate::operation::get_function_event_invoke_config::GetFunctionEventInvokeConfigOutput::function_arn): <p>The Amazon Resource Name (ARN) of the function.</p>
     ///   - [`maximum_retry_attempts(Option<i32>)`](crate::operation::get_function_event_invoke_config::GetFunctionEventInvokeConfigOutput::maximum_retry_attempts): <p>The maximum number of times to retry when the function returns an error.</p>
     ///   - [`maximum_event_age_in_seconds(Option<i32>)`](crate::operation::get_function_event_invoke_config::GetFunctionEventInvokeConfigOutput::maximum_event_age_in_seconds): <p>The maximum age of a request that Lambda sends to a function for processing.</p>
-    ///   - [`destination_config(Option<DestinationConfig>)`](crate::operation::get_function_event_invoke_config::GetFunctionEventInvokeConfigOutput::destination_config): <p>A destination for events after they have been sent to a function for processing.</p> <p class="title"><b>Destinations</b></p> <ul>  <li>   <p><b>Function</b> - The Amazon Resource Name (ARN) of a Lambda function.</p></li>  <li>   <p><b>Queue</b> - The ARN of a standard SQS queue.</p></li>  <li>   <p><b>Bucket</b> - The ARN of an Amazon S3 bucket.</p></li>  <li>   <p><b>Topic</b> - The ARN of a standard SNS topic.</p></li>  <li>   <p><b>Event Bus</b> - The ARN of an Amazon EventBridge event bus.</p></li> </ul><note>  <p>S3 buckets are supported only for on-failure destinations. To retain records of successful invocations, use another destination type.</p> </note>
+    ///   - [`destination_config(Option<DestinationConfig>)`](crate::operation::get_function_event_invoke_config::GetFunctionEventInvokeConfigOutput::destination_config): <p>A destination for events after they have been sent to a function for processing.</p> <p class="title"><b>Destinations</b></p> <ul>  <li>   <p><b>Function</b> - The Amazon Resource Name (ARN) of a Lambda function.</p></li>  <li>   <p><b>Queue</b> - The ARN of a standard SQS queue.</p></li>  <li>   <p><b>Bucket</b> - The ARN of an Amazon S3 bucket.</p></li>  <li>   <p><b>Topic</b> - The ARN of a standard SNS topic.</p></li>  <li>   <p><b>Event Bus</b> - The ARN of an Amazon EventBridge event bus.</p></li> </ul> <note>  <p>S3 buckets are supported only for on-failure destinations. To retain records of successful invocations, use another destination type.</p> </note>
     /// - On failure, responds with [`SdkError<GetFunctionEventInvokeConfigError>`](crate::operation::get_function_event_invoke_config::GetFunctionEventInvokeConfigError)
     pub fn get_function_event_invoke_config(
         &self,
```

### `src/client/get_function_recursion_config.rs`

```diff
--- reference/src/client/get_function_recursion_config.rs
+++ generated/src/client/get_function_recursion_config.rs
@@ -5,7 +5,7 @@
     /// - The fluent builder is configurable:
     ///   - [`function_name(impl Into<String>)`](crate::operation::get_function_recursion_config::builders::GetFunctionRecursionConfigFluentBuilder::function_name) / [`set_function_name(Option<String>)`](crate::operation::get_function_recursion_config::builders::GetFunctionRecursionConfigFluentBuilder::set_function_name):<br>required: **true**<br><p>The name of the function.</p><br>
     /// - On success, responds with [`GetFunctionRecursionConfigOutput`](crate::operation::get_function_recursion_config::GetFunctionRecursionConfigOutput) with field(s):
-    ///   - [`recursive_loop(Option<RecursiveLoop>)`](crate::operation::get_function_recursion_config::GetFunctionRecursionConfigOutput::recursive_loop): <p>If your function's recursive loop detection configuration is <code>Allow</code>, Lambda doesn't take any action when it detects your function being invoked as part of a recursive loop.</p> <p>If your function's recursive loop detection configuration is <code>Terminate</code>, Lambda stops your function being invoked and notifies you when it detects your function being invoked as part of a recursive loop.</p> <p>By default, Lambda sets your function's configuration to <code>Terminate</code>. You can update this configuration using the <code>PutFunctionRecursionConfig</code> action.</p>
+    ///   - [`recursive_loop(Option<RecursiveLoop>)`](crate::operation::get_function_recursion_config::GetFunctionRecursionConfigOutput::recursive_loop): <p>If your function's recursive loop detection configuration is <code>Allow</code>, Lambda doesn't take any action when it detects your function being invoked as part of a recursive loop.</p> <p>If your function's recursive loop detection configuration is <code>Terminate</code>, Lambda stops your function being invoked and notifies you when it detects your function being invoked as part of a recursive loop.</p> <p>By default, Lambda sets your function's configuration to <code>Terminate</code>. You can update this configuration using the <a>PutFunctionRecursionConfig</a> action.</p>
     /// - On failure, responds with [`SdkError<GetFunctionRecursionConfigError>`](crate::operation::get_function_recursion_config::GetFunctionRecursionConfigError)
     pub fn get_function_recursion_config(
         &self,
```

### `src/client/get_resource_policy.rs`

```diff
--- reference/src/client/get_resource_policy.rs
+++ generated/src/client/get_resource_policy.rs
@@ -6,7 +6,7 @@
     ///   - [`resource_arn(impl Into<String>)`](crate::operation::get_resource_policy::builders::GetResourcePolicyFluentBuilder::resource_arn) / [`set_resource_arn(Option<String>)`](crate::operation::get_resource_policy::builders::GetResourcePolicyFluentBuilder::set_resource_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of the Lambda resource you want to retrieve the policy for. You can use a qualified or an unqualified ARN. The value must be a complete ARN, and the operation does not accept wildcard characters.</p><br>
     /// - On success, responds with [`GetResourcePolicyOutput`](crate::operation::get_resource_policy::GetResourcePolicyOutput) with field(s):
     ///   - [`policy(Option<String>)`](crate::operation::get_resource_policy::GetResourcePolicyOutput::policy): <p>The resource-based policy attached to the Lambda resource you specified.</p>
-    ///   - [`revision_id(Option<String>)`](crate::operation::get_resource_policy::GetResourcePolicyOutput::revision_id): <p>The revision ID of the policy. Pass this value as the <code>RevisionId</code> in a <code>PutResourcePolicy</code> or <code>DeleteResourcePolicy</code> request. Doing so ensures the operation acts on the expected version of the policy.</p>
+    ///   - [`revision_id(Option<String>)`](crate::operation::get_resource_policy::GetResourcePolicyOutput::revision_id): <p>The revision ID of the policy. Pass this value as the <code>RevisionId</code> in a <a>PutResourcePolicy</a> or <a>DeleteResourcePolicy</a> request. Doing so ensures the operation acts on the expected version of the policy.</p>
     /// - On failure, responds with [`SdkError<GetResourcePolicyError>`](crate::operation::get_resource_policy::GetResourcePolicyError)
     pub fn get_resource_policy(&self) -> crate::operation::get_resource_policy::builders::GetResourcePolicyFluentBuilder {
         crate::operation::get_resource_policy::builders::GetResourcePolicyFluentBuilder::new(self.handle.clone())
```

### `src/client/publish_layer_version.rs`

```diff
--- reference/src/client/publish_layer_version.rs
+++ generated/src/client/publish_layer_version.rs
@@ -7,7 +7,7 @@
     ///   - [`description(impl Into<String>)`](crate::operation::publish_layer_version::builders::PublishLayerVersionFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::publish_layer_version::builders::PublishLayerVersionFluentBuilder::set_description):<br>required: **false**<br><p>The description of the version.</p><br>
     ///   - [`content(LayerVersionContentInput)`](crate::operation::publish_layer_version::builders::PublishLayerVersionFluentBuilder::content) / [`set_content(Option<LayerVersionContentInput>)`](crate::operation::publish_layer_version::builders::PublishLayerVersionFluentBuilder::set_content):<br>required: **true**<br><p>The function layer archive.</p><br>
     ///   - [`compatible_architectures(Architecture)`](crate::operation::publish_layer_version::builders::PublishLayerVersionFluentBuilder::compatible_architectures) / [`set_compatible_architectures(Option<Vec::<Architecture>>)`](crate::operation::publish_layer_version::builders::PublishLayerVersionFluentBuilder::set_compatible_architectures):<br>required: **false**<br><p>A list of compatible <a href="https://docs.aws.amazon.com/lambda/latest/dg/foundation-arch.html">instruction set architectures</a>.</p><br>
-    ///   - [`compatible_runtimes(Runtime)`](crate::operation::publish_layer_version::builders::PublishLayerVersionFluentBuilder::compatible_runtimes) / [`set_compatible_runtimes(Option<Vec::<Runtime>>)`](crate::operation::publish_layer_version::builders::PublishLayerVersionFluentBuilder::set_compatible_runtimes):<br>required: **false**<br><p>A list of compatible <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html">function runtimes</a>. Used for filtering with <code>ListLayers</code> and <code>ListLayerVersions</code>.</p> <p>The following list includes deprecated runtimes. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html#runtime-support-policy">Runtime deprecation policy</a>.</p><br>
+    ///   - [`compatible_runtimes(Runtime)`](crate::operation::publish_layer_version::builders::PublishLayerVersionFluentBuilder::compatible_runtimes) / [`set_compatible_runtimes(Option<Vec::<Runtime>>)`](crate::operation::publish_layer_version::builders::PublishLayerVersionFluentBuilder::set_compatible_runtimes):<br>required: **false**<br><p>A list of compatible <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html">function runtimes</a>. Used for filtering with <a>ListLayers</a> and <a>ListLayerVersions</a>.</p> <p>The following list includes deprecated runtimes. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html#runtime-support-policy">Runtime deprecation policy</a>.</p><br>
     ///   - [`license_info(impl Into<String>)`](crate::operation::publish_layer_version::builders::PublishLayerVersionFluentBuilder::license_info) / [`set_license_info(Option<String>)`](crate::operation::publish_layer_version::builders::PublishLayerVersionFluentBuilder::set_license_info):<br>required: **false**<br><p>The layer's software license. It can be any of the following:</p> <ul>  <li>   <p>An <a href="https://spdx.org/licenses/">SPDX license identifier</a>. For example, <code>MIT</code>.</p></li>  <li>   <p>The URL of a license hosted on the internet. For example, <code>https://opensource.org/licenses/MIT</code>.</p></li>  <li>   <p>The full text of the license.</p></li> </ul><br>
     /// - On success, responds with [`PublishLayerVersionOutput`](crate::operation::publish_layer_version::PublishLayerVersionOutput) with field(s):
     ///   - [`content(Option<LayerVersionContentOutput>)`](crate::operation::publish_layer_version::PublishLayerVersionOutput::content): <p>Details about the layer version.</p>
```

### `src/client/publish_version.rs`

```diff
--- reference/src/client/publish_version.rs
+++ generated/src/client/publish_version.rs
@@ -4,7 +4,7 @@
     ///
     /// - The fluent builder is configurable:
     ///   - [`function_name(impl Into<String>)`](crate::operation::publish_version::builders::PublishVersionFluentBuilder::function_name) / [`set_function_name(Option<String>)`](crate::operation::publish_version::builders::PublishVersionFluentBuilder::set_function_name):<br>required: **true**<br><p>The name or ARN of the Lambda function.</p> <p class="title"><b>Name formats</b></p> <ul>  <li>   <p><b>Function name</b> - <code>MyFunction</code>.</p></li>  <li>   <p><b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:MyFunction</code>.</p></li>  <li>   <p><b>Partial ARN</b> - <code>123456789012:function:MyFunction</code>.</p></li> </ul> <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p><br>
-    ///   - [`code_sha256(impl Into<String>)`](crate::operation::publish_version::builders::PublishVersionFluentBuilder::code_sha256) / [`set_code_sha256(Option<String>)`](crate::operation::publish_version::builders::PublishVersionFluentBuilder::set_code_sha256):<br>required: **false**<br><p>Only publish a version if the hash value matches the value that's specified. Use this option to avoid publishing a version if the function code has changed since you last updated it. You can get the hash for the version that you uploaded from the output of <code>UpdateFunctionCode</code>.</p><br>
+    ///   - [`code_sha256(impl Into<String>)`](crate::operation::publish_version::builders::PublishVersionFluentBuilder::code_sha256) / [`set_code_sha256(Option<String>)`](crate::operation::publish_version::builders::PublishVersionFluentBuilder::set_code_sha256):<br>required: **false**<br><p>Only publish a version if the hash value matches the value that's specified. Use this option to avoid publishing a version if the function code has changed since you last updated it. You can get the hash for the version that you uploaded from the output of <a>UpdateFunctionCode</a>.</p><br>
     ///   - [`description(impl Into<String>)`](crate::operation::publish_version::builders::PublishVersionFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::publish_version::builders::PublishVersionFluentBuilder::set_description):<br>required: **false**<br><p>A description for the version to override the description in the function configuration.</p><br>
     ///   - [`revision_id(impl Into<String>)`](crate::operation::publish_version::builders::PublishVersionFluentBuilder::revision_id) / [`set_revision_id(Option<String>)`](crate::operation::publish_version::builders::PublishVersionFluentBuilder::set_revision_id):<br>required: **false**<br><p>Only update the function if the revision ID matches the ID that's specified. Use this option to avoid publishing a version if the function configuration has changed since you last updated it.</p><br>
     ///   - [`publish_to(FunctionVersionLatestPublished)`](crate::operation::publish_version::builders::PublishVersionFluentBuilder::publish_to) / [`set_publish_to(Option<FunctionVersionLatestPublished>)`](crate::operation::publish_version::builders::PublishVersionFluentBuilder::set_publish_to):<br>required: **false**<br><p>Specifies where to publish the function version or configuration.</p><br>
```

### `src/client/put_function_event_invoke_config.rs`

```diff
--- reference/src/client/put_function_event_invoke_config.rs
+++ generated/src/client/put_function_event_invoke_config.rs
@@ -7,13 +7,13 @@
     ///   - [`qualifier(impl Into<String>)`](crate::operation::put_function_event_invoke_config::builders::PutFunctionEventInvokeConfigFluentBuilder::qualifier) / [`set_qualifier(Option<String>)`](crate::operation::put_function_event_invoke_config::builders::PutFunctionEventInvokeConfigFluentBuilder::set_qualifier):<br>required: **false**<br><p>A version number or alias name.</p><br>
     ///   - [`maximum_retry_attempts(i32)`](crate::operation::put_function_event_invoke_config::builders::PutFunctionEventInvokeConfigFluentBuilder::maximum_retry_attempts) / [`set_maximum_retry_attempts(Option<i32>)`](crate::operation::put_function_event_invoke_config::builders::PutFunctionEventInvokeConfigFluentBuilder::set_maximum_retry_attempts):<br>required: **false**<br><p>The maximum number of times to retry when the function returns an error.</p><br>
     ///   - [`maximum_event_age_in_seconds(i32)`](crate::operation::put_function_event_invoke_config::builders::PutFunctionEventInvokeConfigFluentBuilder::maximum_event_age_in_seconds) / [`set_maximum_event_age_in_seconds(Option<i32>)`](crate::operation::put_function_event_invoke_config::builders::PutFunctionEventInvokeConfigFluentBuilder::set_maximum_event_age_in_seconds):<br>required: **false**<br><p>The maximum age of a request that Lambda sends to a function for processing.</p><br>
-    ///   - [`destination_config(DestinationConfig)`](crate::operation::put_function_event_invoke_config::builders::PutFunctionEventInvokeConfigFluentBuilder::destination_config) / [`set_destination_config(Option<DestinationConfig>)`](crate::operation::put_function_event_invoke_config::builders::PutFunctionEventInvokeConfigFluentBuilder::set_destination_config):<br>required: **false**<br><p>A destination for events after they have been sent to a function for processing.</p> <p class="title"><b>Destinations</b></p> <ul>  <li>   <p><b>Function</b> - The Amazon Resource Name (ARN) of a Lambda function.</p></li>  <li>   <p><b>Queue</b> - The ARN of a standard SQS queue.</p></li>  <li>   <p><b>Bucket</b> - The ARN of an Amazon S3 bucket.</p></li>  <li>   <p><b>Topic</b> - The ARN of a standard SNS topic.</p></li>  <li>   <p><b>Event Bus</b> - The ARN of an Amazon EventBridge event bus.</p></li> </ul><note>  <p>S3 buckets are supported only for on-failure destinations. To retain records of successful invocations, use another destination type.</p> </note><br>
+    ///   - [`destination_config(DestinationConfig)`](crate::operation::put_function_event_invoke_config::builders::PutFunctionEventInvokeConfigFluentBuilder::destination_config) / [`set_destination_config(Option<DestinationConfig>)`](crate::operation::put_function_event_invoke_config::builders::PutFunctionEventInvokeConfigFluentBuilder::set_destination_config):<br>required: **false**<br><p>A destination for events after they have been sent to a function for processing.</p> <p class="title"><b>Destinations</b></p> <ul>  <li>   <p><b>Function</b> - The Amazon Resource Name (ARN) of a Lambda function.</p></li>  <li>   <p><b>Queue</b> - The ARN of a standard SQS queue.</p></li>  <li>   <p><b>Bucket</b> - The ARN of an Amazon S3 bucket.</p></li>  <li>   <p><b>Topic</b> - The ARN of a standard SNS topic.</p></li>  <li>   <p><b>Event Bus</b> - The ARN of an Amazon EventBridge event bus.</p></li> </ul> <note>  <p>S3 buckets are supported only for on-failure destinations. To retain records of successful invocations, use another destination type.</p> </note><br>
     /// - On success, responds with [`PutFunctionEventInvokeConfigOutput`](crate::operation::put_function_event_invoke_config::PutFunctionEventInvokeConfigOutput) with field(s):
     ///   - [`last_modified(Option<DateTime>)`](crate::operation::put_function_event_invoke_config::PutFunctionEventInvokeConfigOutput::last_modified): <p>The date and time that the configuration was last updated.</p>
     ///   - [`function_arn(Option<String>)`](crate::operation::put_function_event_invoke_config::PutFunctionEventInvokeConfigOutput::function_arn): <p>The Amazon Resource Name (ARN) of the function.</p>
     ///   - [`maximum_retry_attempts(Option<i32>)`](crate::operation::put_function_event_invoke_config::PutFunctionEventInvokeConfigOutput::maximum_retry_attempts): <p>The maximum number of times to retry when the function returns an error.</p>
     ///   - [`maximum_event_age_in_seconds(Option<i32>)`](crate::operation::put_function_event_invoke_config::PutFunctionEventInvokeConfigOutput::maximum_event_age_in_seconds): <p>The maximum age of a request that Lambda sends to a function for processing.</p>
-    ///   - [`destination_config(Option<DestinationConfig>)`](crate::operation::put_function_event_invoke_config::PutFunctionEventInvokeConfigOutput::destination_config): <p>A destination for events after they have been sent to a function for processing.</p> <p class="title"><b>Destinations</b></p> <ul>  <li>   <p><b>Function</b> - The Amazon Resource Name (ARN) of a Lambda function.</p></li>  <li>   <p><b>Queue</b> - The ARN of a standard SQS queue.</p></li>  <li>   <p><b>Bucket</b> - The ARN of an Amazon S3 bucket.</p></li>  <li>   <p><b>Topic</b> - The ARN of a standard SNS topic.</p></li>  <li>   <p><b>Event Bus</b> - The ARN of an Amazon EventBridge event bus.</p></li> </ul><note>  <p>S3 buckets are supported only for on-failure destinations. To retain records of successful invocations, use another destination type.</p> </note>
+    ///   - [`destination_config(Option<DestinationConfig>)`](crate::operation::put_function_event_invoke_config::PutFunctionEventInvokeConfigOutput::destination_config): <p>A destination for events after they have been sent to a function for processing.</p> <p class="title"><b>Destinations</b></p> <ul>  <li>   <p><b>Function</b> - The Amazon Resource Name (ARN) of a Lambda function.</p></li>  <li>   <p><b>Queue</b> - The ARN of a standard SQS queue.</p></li>  <li>   <p><b>Bucket</b> - The ARN of an Amazon S3 bucket.</p></li>  <li>   <p><b>Topic</b> - The ARN of a standard SNS topic.</p></li>  <li>   <p><b>Event Bus</b> - The ARN of an Amazon EventBridge event bus.</p></li> </ul> <note>  <p>S3 buckets are supported only for on-failure destinations. To retain records of successful invocations, use another destination type.</p> </note>
     /// - On failure, responds with [`SdkError<PutFunctionEventInvokeConfigError>`](crate::operation::put_function_event_invoke_config::PutFunctionEventInvokeConfigError)
     pub fn put_function_event_invoke_config(
         &self,
```

### `src/client/put_resource_policy.rs`

```diff
--- reference/src/client/put_resource_policy.rs
+++ generated/src/client/put_resource_policy.rs
@@ -5,7 +5,7 @@
     /// - The fluent builder is configurable:
     ///   - [`resource_arn(impl Into<String>)`](crate::operation::put_resource_policy::builders::PutResourcePolicyFluentBuilder::resource_arn) / [`set_resource_arn(Option<String>)`](crate::operation::put_resource_policy::builders::PutResourcePolicyFluentBuilder::set_resource_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of the Lambda resource you want to add the policy to. You can use a qualified or an unqualified ARN. The value must be a complete ARN, and the operation does not accept wildcard characters.</p><br>
     ///   - [`policy(impl Into<String>)`](crate::operation::put_resource_policy::builders::PutResourcePolicyFluentBuilder::policy) / [`set_policy(Option<String>)`](crate::operation::put_resource_policy::builders::PutResourcePolicyFluentBuilder::set_policy):<br>required: **true**<br><p>The policy document you want to add to your Lambda resource. This is formatted as a JSON string.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/access-control-resource-based.html">Working with resource-based policies in Lambda</a> in the <i>Lambda Developer Guide</i>.</p><br>
-    ///   - [`revision_id(impl Into<String>)`](crate::operation::put_resource_policy::builders::PutResourcePolicyFluentBuilder::revision_id) / [`set_revision_id(Option<String>)`](crate::operation::put_resource_policy::builders::PutResourcePolicyFluentBuilder::set_revision_id):<br>required: **false**<br><p>The revision ID that the existing policy must match for the replacement to proceed. If the revision ID doesn't match, the operation fails with a <code>PreconditionFailedException</code> error. To retrieve the current revision ID, use the <code>GetResourcePolicy</code> operation.</p><br>
+    ///   - [`revision_id(impl Into<String>)`](crate::operation::put_resource_policy::builders::PutResourcePolicyFluentBuilder::revision_id) / [`set_revision_id(Option<String>)`](crate::operation::put_resource_policy::builders::PutResourcePolicyFluentBuilder::set_revision_id):<br>required: **false**<br><p>The revision ID that the existing policy must match for the replacement to proceed. If the revision ID doesn't match, the operation fails with a <code>PreconditionFailedException</code> error. To retrieve the current revision ID, use the <a>GetResourcePolicy</a> operation.</p><br>
     /// - On success, responds with [`PutResourcePolicyOutput`](crate::operation::put_resource_policy::PutResourcePolicyOutput) with field(s):
     ///   - [`policy(Option<String>)`](crate::operation::put_resource_policy::PutResourcePolicyOutput::policy): <p>The resource-based policy that Lambda adds to the resource.</p>
     ///   - [`revision_id(Option<String>)`](crate::operation::put_resource_policy::PutResourcePolicyOutput::revision_id): <p>The revision ID of the policy that Lambda adds to your Lambda resource.</p>
```

### `src/client/update_function_code.rs`

```diff
--- reference/src/client/update_function_code.rs
+++ generated/src/client/update_function_code.rs
@@ -11,7 +11,7 @@
     ///   - [`s3_object_storage_mode(S3ObjectStorageMode)`](crate::operation::update_function_code::builders::UpdateFunctionCodeFluentBuilder::s3_object_storage_mode) / [`set_s3_object_storage_mode(Option<S3ObjectStorageMode>)`](crate::operation::update_function_code::builders::UpdateFunctionCodeFluentBuilder::set_s3_object_storage_mode):<br>required: **false**<br><p>Specifies how the deployment package is stored. Valid values:</p> <ul>  <li>   <p><code>COPY</code> (default) – Uploads a copy of your deployment package to Lambda.</p></li>  <li>   <p><code>REFERENCE</code> – Lambda references the deployment package from the specified Amazon S3 bucket.</p></li> </ul><br>
     ///   - [`image_uri(impl Into<String>)`](crate::operation::update_function_code::builders::UpdateFunctionCodeFluentBuilder::image_uri) / [`set_image_uri(Option<String>)`](crate::operation::update_function_code::builders::UpdateFunctionCodeFluentBuilder::set_image_uri):<br>required: **false**<br><p>URI of a container image in the Amazon ECR registry. Do not use for a function defined with a .zip file archive.</p><br>
     ///   - [`architectures(Architecture)`](crate::operation::update_function_code::builders::UpdateFunctionCodeFluentBuilder::architectures) / [`set_architectures(Option<Vec::<Architecture>>)`](crate::operation::update_function_code::builders::UpdateFunctionCodeFluentBuilder::set_architectures):<br>required: **false**<br><p>The instruction set architecture that the function supports. Enter a string array with one of the valid values (arm64 or x86_64). The default value is <code>x86_64</code>.</p><br>
-    ///   - [`publish(bool)`](crate::operation::update_function_code::builders::UpdateFunctionCodeFluentBuilder::publish) / [`set_publish(Option<bool>)`](crate::operation::update_function_code::builders::UpdateFunctionCodeFluentBuilder::set_publish):<br>required: **false**<br><p>Set to true to publish a new version of the function after updating the code. This has the same effect as calling <code>PublishVersion</code> separately.</p><br>
+    ///   - [`publish(bool)`](crate::operation::update_function_code::builders::UpdateFunctionCodeFluentBuilder::publish) / [`set_publish(Option<bool>)`](crate::operation::update_function_code::builders::UpdateFunctionCodeFluentBuilder::set_publish):<br>required: **false**<br><p>Set to true to publish a new version of the function after updating the code. This has the same effect as calling <a>PublishVersion</a> separately.</p><br>
     ///   - [`publish_to(FunctionVersionLatestPublished)`](crate::operation::update_function_code::builders::UpdateFunctionCodeFluentBuilder::publish_to) / [`set_publish_to(Option<FunctionVersionLatestPublished>)`](crate::operation::update_function_code::builders::UpdateFunctionCodeFluentBuilder::set_publish_to):<br>required: **false**<br><p>Specifies where to publish the function version or configuration.</p><br>
     ///   - [`dry_run(bool)`](crate::operation::update_function_code::builders::UpdateFunctionCodeFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::update_function_code::builders::UpdateFunctionCodeFluentBuilder::set_dry_run):<br>required: **false**<br><p>Set to true to validate the request parameters and access permissions without modifying the function code.</p><br>
     ///   - [`revision_id(impl Into<String>)`](crate::operation::update_function_code::builders::UpdateFunctionCodeFluentBuilder::revision_id) / [`set_revision_id(Option<String>)`](crate::operation::update_function_code::builders::UpdateFunctionCodeFluentBuilder::set_revision_id):<br>required: **false**<br><p>Update the function only if the revision ID matches the ID that's specified. Use this option to avoid modifying a function that has changed since you last read it.</p><br>
```

### `src/client/update_function_event_invoke_config.rs`

```diff
--- reference/src/client/update_function_event_invoke_config.rs
+++ generated/src/client/update_function_event_invoke_config.rs
@@ -7,13 +7,13 @@
     ///   - [`qualifier(impl Into<String>)`](crate::operation::update_function_event_invoke_config::builders::UpdateFunctionEventInvokeConfigFluentBuilder::qualifier) / [`set_qualifier(Option<String>)`](crate::operation::update_function_event_invoke_config::builders::UpdateFunctionEventInvokeConfigFluentBuilder::set_qualifier):<br>required: **false**<br><p>A version number or alias name.</p><br>
     ///   - [`maximum_retry_attempts(i32)`](crate::operation::update_function_event_invoke_config::builders::UpdateFunctionEventInvokeConfigFluentBuilder::maximum_retry_attempts) / [`set_maximum_retry_attempts(Option<i32>)`](crate::operation::update_function_event_invoke_config::builders::UpdateFunctionEventInvokeConfigFluentBuilder::set_maximum_retry_attempts):<br>required: **false**<br><p>The maximum number of times to retry when the function returns an error.</p><br>
     ///   - [`maximum_event_age_in_seconds(i32)`](crate::operation::update_function_event_invoke_config::builders::UpdateFunctionEventInvokeConfigFluentBuilder::maximum_event_age_in_seconds) / [`set_maximum_event_age_in_seconds(Option<i32>)`](crate::operation::update_function_event_invoke_config::builders::UpdateFunctionEventInvokeConfigFluentBuilder::set_maximum_event_age_in_seconds):<br>required: **false**<br><p>The maximum age of a request that Lambda sends to a function for processing.</p><br>
-    ///   - [`destination_config(DestinationConfig)`](crate::operation::update_function_event_invoke_config::builders::UpdateFunctionEventInvokeConfigFluentBuilder::destination_config) / [`set_destination_config(Option<DestinationConfig>)`](crate::operation::update_function_event_invoke_config::builders::UpdateFunctionEventInvokeConfigFluentBuilder::set_destination_config):<br>required: **false**<br><p>A destination for events after they have been sent to a function for processing.</p> <p class="title"><b>Destinations</b></p> <ul>  <li>   <p><b>Function</b> - The Amazon Resource Name (ARN) of a Lambda function.</p></li>  <li>   <p><b>Queue</b> - The ARN of a standard SQS queue.</p></li>  <li>   <p><b>Bucket</b> - The ARN of an Amazon S3 bucket.</p></li>  <li>   <p><b>Topic</b> - The ARN of a standard SNS topic.</p></li>  <li>   <p><b>Event Bus</b> - The ARN of an Amazon EventBridge event bus.</p></li> </ul><note>  <p>S3 buckets are supported only for on-failure destinations. To retain records of successful invocations, use another destination type.</p> </note><br>
+    ///   - [`destination_config(DestinationConfig)`](crate::operation::update_function_event_invoke_config::builders::UpdateFunctionEventInvokeConfigFluentBuilder::destination_config) / [`set_destination_config(Option<DestinationConfig>)`](crate::operation::update_function_event_invoke_config::builders::UpdateFunctionEventInvokeConfigFluentBuilder::set_destination_config):<br>required: **false**<br><p>A destination for events after they have been sent to a function for processing.</p> <p class="title"><b>Destinations</b></p> <ul>  <li>   <p><b>Function</b> - The Amazon Resource Name (ARN) of a Lambda function.</p></li>  <li>   <p><b>Queue</b> - The ARN of a standard SQS queue.</p></li>  <li>   <p><b>Bucket</b> - The ARN of an Amazon S3 bucket.</p></li>  <li>   <p><b>Topic</b> - The ARN of a standard SNS topic.</p></li>  <li>   <p><b>Event Bus</b> - The ARN of an Amazon EventBridge event bus.</p></li> </ul> <note>  <p>S3 buckets are supported only for on-failure destinations. To retain records of successful invocations, use another destination type.</p> </note><br>
     /// - On success, responds with [`UpdateFunctionEventInvokeConfigOutput`](crate::operation::update_function_event_invoke_config::UpdateFunctionEventInvokeConfigOutput) with field(s):
     ///   - [`last_modified(Option<DateTime>)`](crate::operation::update_function_event_invoke_config::UpdateFunctionEventInvokeConfigOutput::last_modified): <p>The date and time that the configuration was last updated.</p>
     ///   - [`function_arn(Option<String>)`](crate::operation::update_function_event_invoke_config::UpdateFunctionEventInvokeConfigOutput::function_arn): <p>The Amazon Resource Name (ARN) of the function.</p>
     ///   - [`maximum_retry_attempts(Option<i32>)`](crate::operation::update_function_event_invoke_config::UpdateFunctionEventInvokeConfigOutput::maximum_retry_attempts): <p>The maximum number of times to retry when the function returns an error.</p>
     ///   - [`maximum_event_age_in_seconds(Option<i32>)`](crate::operation::update_function_event_invoke_config::UpdateFunctionEventInvokeConfigOutput::maximum_event_age_in_seconds): <p>The maximum age of a request that Lambda sends to a function for processing.</p>
-    ///   - [`destination_config(Option<DestinationConfig>)`](crate::operation::update_function_event_invoke_config::UpdateFunctionEventInvokeConfigOutput::destination_config): <p>A destination for events after they have been sent to a function for processing.</p> <p class="title"><b>Destinations</b></p> <ul>  <li>   <p><b>Function</b> - The Amazon Resource Name (ARN) of a Lambda function.</p></li>  <li>   <p><b>Queue</b> - The ARN of a standard SQS queue.</p></li>  <li>   <p><b>Bucket</b> - The ARN of an Amazon S3 bucket.</p></li>  <li>   <p><b>Topic</b> - The ARN of a standard SNS topic.</p></li>  <li>   <p><b>Event Bus</b> - The ARN of an Amazon EventBridge event bus.</p></li> </ul><note>  <p>S3 buckets are supported only for on-failure destinations. To retain records of successful invocations, use another destination type.</p> </note>
+    ///   - [`destination_config(Option<DestinationConfig>)`](crate::operation::update_function_event_invoke_config::UpdateFunctionEventInvokeConfigOutput::destination_config): <p>A destination for events after they have been sent to a function for processing.</p> <p class="title"><b>Destinations</b></p> <ul>  <li>   <p><b>Function</b> - The Amazon Resource Name (ARN) of a Lambda function.</p></li>  <li>   <p><b>Queue</b> - The ARN of a standard SQS queue.</p></li>  <li>   <p><b>Bucket</b> - The ARN of an Amazon S3 bucket.</p></li>  <li>   <p><b>Topic</b> - The ARN of a standard SNS topic.</p></li>  <li>   <p><b>Event Bus</b> - The ARN of an Amazon EventBridge event bus.</p></li> </ul> <note>  <p>S3 buckets are supported only for on-failure destinations. To retain records of successful invocations, use another destination type.</p> </note>
     /// - On failure, responds with [`SdkError<UpdateFunctionEventInvokeConfigError>`](crate::operation::update_function_event_invoke_config::UpdateFunctionEventInvokeConfigError)
     pub fn update_function_event_invoke_config(
         &self,
```

### `src/event_stream_serde.rs`

```diff
--- reference/src/event_stream_serde.rs
+++ generated/src/event_stream_serde.rs
@@ -34,10 +34,10 @@
                 }
                 "InvokeComplete" => {
                     let parsed =
-                            crate::protocol_serde::shape_invoke_with_response_stream_complete_event::de_invoke_with_response_stream_complete_event_payload(&message.payload()[..])
-                                                .map_err(|err| {
-                                                    ::aws_smithy_eventstream::error::Error::unmarshalling(format!("failed to unmarshall InvokeComplete: {err}"))
-                                                })?
+                        crate::protocol_serde::shape_invoke_with_response_stream_complete_event::de_invoke_with_response_stream_complete_event_payload(&message.payload()[..])
+                            .map_err(|err| {
+                                ::aws_smithy_eventstream::error::Error::unmarshalling(format!("failed to unmarshall InvokeComplete: {err}"))
+                            })?
                         ;
                     Ok(::aws_smithy_eventstream::frame::UnmarshalledMessage::Event(
                         crate::types::InvokeWithResponseStreamResponseEvent::InvokeComplete(parsed),
```

### `src/operation/add_layer_version_permission/_add_layer_version_permission_input.rs`

```diff
--- reference/src/operation/add_layer_version_permission/_add_layer_version_permission_input.rs
+++ generated/src/operation/add_layer_version_permission/_add_layer_version_permission_input.rs
@@ -180,7 +180,7 @@
     > {
         ::std::result::Result::Ok(crate::operation::add_layer_version_permission::AddLayerVersionPermissionInput {
             layer_name: self.layer_name,
-            version_number: self.version_number,
+            version_number: self.version_number.unwrap_or_default(),
             statement_id: self.statement_id,
             action: self.action,
             principal: self.principal,
```

### `src/operation/add_layer_version_permission.rs`

```diff
--- reference/src/operation/add_layer_version_permission.rs
+++ generated/src/operation/add_layer_version_permission.rs
@@ -107,9 +107,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -141,9 +141,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::add_layer_version_permission::AddLayerVersionPermissionError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::add_layer_version_permission::AddLayerVersionPermissionError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::add_layer_version_permission::AddLayerVersionPermissionError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -277,8 +285,7 @@
                 let input_2 = input_2
                     .as_ref()
                     .ok_or_else(|| ::aws_smithy_types::error::operation::BuildError::missing_field("version_number", "cannot be empty or unset"))?;
-                let mut version_number_encoder = ::aws_smithy_types::primitive::Encoder::from(*input_2);
-                let version_number = version_number_encoder.encode();
+                let version_number = ::aws_smithy_http::label::fmt_string(input_2, ::aws_smithy_http::label::EncodingStrategy::Default);
                 if version_number.is_empty() {
                     return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
                         "version_number",
@@ -317,11 +324,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_add_layer_version_permission::ser_add_layer_version_permission_input(&input)?,
+            crate::protocol_serde::shape_add_layer_version_permission_input::ser_add_layer_version_permission_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -356,8 +363,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -535,6 +542,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::add_layer_version_permission::AddLayerVersionPermissionError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::add_layer_version_permission::AddLayerVersionPermissionError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/add_permission.rs`

```diff
--- reference/src/operation/add_permission.rs
+++ generated/src/operation/add_permission.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("AddPermission", "Lambda"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -138,9 +138,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::add_permission::AddPermissionError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::add_permission::AddPermissionError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::add_permission::AddPermissionError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -317,10 +323,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_add_permission::ser_add_permission_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_add_permission_input::ser_add_permission_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -354,8 +360,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -543,6 +549,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::add_permission::AddPermissionError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::add_permission::AddPermissionError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/checkpoint_durable_execution.rs`

```diff
--- reference/src/operation/checkpoint_durable_execution.rs
+++ generated/src/operation/checkpoint_durable_execution.rs
@@ -116,9 +116,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -150,9 +150,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::checkpoint_durable_execution::CheckpointDurableExecutionError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::checkpoint_durable_execution::CheckpointDurableExecutionError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::checkpoint_durable_execution::CheckpointDurableExecutionError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -285,11 +293,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_checkpoint_durable_execution::ser_checkpoint_durable_execution_input(&input)?,
+            crate::protocol_serde::shape_checkpoint_durable_execution_input::ser_checkpoint_durable_execution_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -324,8 +332,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -497,6 +505,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::checkpoint_durable_execution::CheckpointDurableExecutionError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::checkpoint_durable_execution::CheckpointDurableExecutionError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/create_alias.rs`

```diff
--- reference/src/operation/create_alias.rs
+++ generated/src/operation/create_alias.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("CreateAlias", "Lambda"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -138,9 +138,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::create_alias::CreateAliasError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::create_alias::CreateAliasError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::create_alias::CreateAliasError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -274,10 +280,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_create_alias::ser_create_alias_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_create_alias_input::ser_create_alias_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -311,8 +317,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -474,6 +480,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::create_alias::CreateAliasError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::create_alias::CreateAliasError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/create_capacity_provider.rs`

```diff
--- reference/src/operation/create_capacity_provider.rs
+++ generated/src/operation/create_capacity_provider.rs
@@ -107,9 +107,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -125,25 +125,34 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("CreateCapacityProvider")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                CreateCapacityProviderTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                CreateCapacityProviderEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::create_capacity_provider::CreateCapacityProviderError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::create_capacity_provider::CreateCapacityProviderError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::create_capacity_provider::CreateCapacityProviderError,
-            >::new());
+        let mut rcb =
+            ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("CreateCapacityProvider")
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    CreateCapacityProviderTelemetryInputCaptureInterceptor,
+                ))
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
+                ))
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    CreateCapacityProviderEndpointParamsInterceptor,
+                ))
+                .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+                    crate::operation::create_capacity_provider::CreateCapacityProviderError,
+                >::new())
+                .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+                    crate::operation::create_capacity_provider::CreateCapacityProviderError,
+                >::new())
+                .with_retry_classifier(
+                    ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                        crate::operation::create_capacity_provider::CreateCapacityProviderError,
+                    >::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+                );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -255,11 +264,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_create_capacity_provider::ser_create_capacity_provider_input(&input)?,
+            crate::protocol_serde::shape_create_capacity_provider_input::ser_create_capacity_provider_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -294,8 +303,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -447,6 +456,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::create_capacity_provider::CreateCapacityProviderError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::create_capacity_provider::CreateCapacityProviderError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/create_code_signing_config.rs`

```diff
--- reference/src/operation/create_code_signing_config.rs
+++ generated/src/operation/create_code_signing_config.rs
@@ -107,9 +107,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -141,9 +141,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::create_code_signing_config::CreateCodeSigningConfigError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::create_code_signing_config::CreateCodeSigningConfigError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::create_code_signing_config::CreateCodeSigningConfigError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -250,11 +258,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_create_code_signing_config::ser_create_code_signing_config_input(&input)?,
+            crate::protocol_serde::shape_create_code_signing_config_input::ser_create_code_signing_config_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -289,8 +297,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -412,6 +420,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::create_code_signing_config::CreateCodeSigningConfigError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::create_code_signing_config::CreateCodeSigningConfigError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/create_event_source_mapping.rs`

```diff
--- reference/src/operation/create_event_source_mapping.rs
+++ generated/src/operation/create_event_source_mapping.rs
@@ -107,9 +107,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -141,9 +141,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::create_event_source_mapping::CreateEventSourceMappingError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::create_event_source_mapping::CreateEventSourceMappingError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::create_event_source_mapping::CreateEventSourceMappingError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -260,11 +268,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_create_event_source_mapping::ser_create_event_source_mapping_input(&input)?,
+            crate::protocol_serde::shape_create_event_source_mapping_input::ser_create_event_source_mapping_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -299,8 +307,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -452,6 +460,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::create_event_source_mapping::CreateEventSourceMappingError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::create_event_source_mapping::CreateEventSourceMappingError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/create_function/_create_function_input.rs`

```diff
--- reference/src/operation/create_function/_create_function_input.rs
+++ generated/src/operation/create_function/_create_function_input.rs
@@ -768,7 +768,7 @@
             description: self.description,
             timeout: self.timeout,
             memory_size: self.memory_size,
-            publish: self.publish,
+            publish: self.publish.unwrap_or_default(),
             publish_to: self.publish_to,
             vpc_config: self.vpc_config,
             package_type: self.package_type,
```

### `src/operation/create_function.rs`

```diff
--- reference/src/operation/create_function.rs
+++ generated/src/operation/create_function.rs
@@ -105,9 +105,9 @@
         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::SensitiveOutput);
         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("CreateFunction", "Lambda"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -139,9 +139,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::create_function::CreateFunctionError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::create_function::CreateFunctionError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::create_function::CreateFunctionError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -273,10 +279,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_create_function::ser_create_function_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_create_function_input::ser_create_function_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -310,8 +316,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -515,6 +521,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::create_function::CreateFunctionError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::create_function::CreateFunctionError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/create_function_url_config.rs`

```diff
--- reference/src/operation/create_function_url_config.rs
+++ generated/src/operation/create_function_url_config.rs
@@ -107,9 +107,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -141,9 +141,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::create_function_url_config::CreateFunctionUrlConfigError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::create_function_url_config::CreateFunctionUrlConfigError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::create_function_url_config::CreateFunctionUrlConfigError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -279,11 +287,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_create_function_url_config::ser_create_function_url_config_input(&input)?,
+            crate::protocol_serde::shape_create_function_url_config_input::ser_create_function_url_config_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -318,8 +326,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -471,6 +479,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::create_function_url_config::CreateFunctionUrlConfigError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::create_function_url_config::CreateFunctionUrlConfigError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/delete_alias.rs`

```diff
--- reference/src/operation/delete_alias.rs
+++ generated/src/operation/delete_alias.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("DeleteAlias", "Lambda"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -138,9 +138,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::delete_alias::DeleteAliasError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::delete_alias::DeleteAliasError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::delete_alias::DeleteAliasError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -313,8 +319,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -466,6 +472,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::delete_alias::DeleteAliasError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::delete_alias::DeleteAliasError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/delete_capacity_provider.rs`

```diff
--- reference/src/operation/delete_capacity_provider.rs
+++ generated/src/operation/delete_capacity_provider.rs
@@ -107,9 +107,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -125,25 +125,34 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("DeleteCapacityProvider")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                DeleteCapacityProviderTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                DeleteCapacityProviderEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::delete_capacity_provider::DeleteCapacityProviderError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::delete_capacity_provider::DeleteCapacityProviderError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::delete_capacity_provider::DeleteCapacityProviderError,
-            >::new());
+        let mut rcb =
+            ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("DeleteCapacityProvider")
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    DeleteCapacityProviderTelemetryInputCaptureInterceptor,
+                ))
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
+                ))
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    DeleteCapacityProviderEndpointParamsInterceptor,
+                ))
+                .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+                    crate::operation::delete_capacity_provider::DeleteCapacityProviderError,
+                >::new())
+                .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+                    crate::operation::delete_capacity_provider::DeleteCapacityProviderError,
+                >::new())
+                .with_retry_classifier(
+                    ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                        crate::operation::delete_capacity_provider::DeleteCapacityProviderError,
+                    >::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+                );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -299,8 +308,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -452,6 +461,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::delete_capacity_provider::DeleteCapacityProviderError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::delete_capacity_provider::DeleteCapacityProviderError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/delete_code_signing_config.rs`

```diff
--- reference/src/operation/delete_code_signing_config.rs
+++ generated/src/operation/delete_code_signing_config.rs
@@ -107,9 +107,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -141,9 +141,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::delete_code_signing_config::DeleteCodeSigningConfigError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::delete_code_signing_config::DeleteCodeSigningConfigError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::delete_code_signing_config::DeleteCodeSigningConfigError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -299,8 +307,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -442,6 +450,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::delete_code_signing_config::DeleteCodeSigningConfigError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::delete_code_signing_config::DeleteCodeSigningConfigError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/delete_event_source_mapping.rs`

```diff
--- reference/src/operation/delete_event_source_mapping.rs
+++ generated/src/operation/delete_event_source_mapping.rs
@@ -107,9 +107,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -141,9 +141,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::delete_event_source_mapping::DeleteEventSourceMappingError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::delete_event_source_mapping::DeleteEventSourceMappingError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::delete_event_source_mapping::DeleteEventSourceMappingError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -294,8 +302,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -457,6 +465,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::delete_event_source_mapping::DeleteEventSourceMappingError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::delete_event_source_mapping::DeleteEventSourceMappingError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/delete_function.rs`

```diff
--- reference/src/operation/delete_function.rs
+++ generated/src/operation/delete_function.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("DeleteFunction", "Lambda"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -138,9 +138,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::delete_function::DeleteFunctionError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::delete_function::DeleteFunctionError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::delete_function::DeleteFunctionError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -309,8 +315,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -462,6 +468,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::delete_function::DeleteFunctionError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::delete_function::DeleteFunctionError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/delete_function_code_signing_config.rs`

```diff
--- reference/src/operation/delete_function_code_signing_config.rs
+++ generated/src/operation/delete_function_code_signing_config.rs
@@ -113,9 +113,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -147,9 +147,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::delete_function_code_signing_config::DeleteFunctionCodeSigningConfigError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::delete_function_code_signing_config::DeleteFunctionCodeSigningConfigError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::delete_function_code_signing_config::DeleteFunctionCodeSigningConfigError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -307,8 +315,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -470,6 +478,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::delete_function_code_signing_config::DeleteFunctionCodeSigningConfigError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::delete_function_code_signing_config::DeleteFunctionCodeSigningConfigError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/delete_function_concurrency.rs`

```diff
--- reference/src/operation/delete_function_concurrency.rs
+++ generated/src/operation/delete_function_concurrency.rs
@@ -107,9 +107,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -141,9 +141,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::delete_function_concurrency::DeleteFunctionConcurrencyError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::delete_function_concurrency::DeleteFunctionConcurrencyError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::delete_function_concurrency::DeleteFunctionConcurrencyError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -295,8 +303,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -448,6 +456,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::delete_function_concurrency::DeleteFunctionConcurrencyError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::delete_function_concurrency::DeleteFunctionConcurrencyError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/delete_function_event_invoke_config.rs`

```diff
--- reference/src/operation/delete_function_event_invoke_config.rs
+++ generated/src/operation/delete_function_event_invoke_config.rs
@@ -113,9 +113,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -147,9 +147,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::delete_function_event_invoke_config::DeleteFunctionEventInvokeConfigError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::delete_function_event_invoke_config::DeleteFunctionEventInvokeConfigError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::delete_function_event_invoke_config::DeleteFunctionEventInvokeConfigError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -325,8 +333,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -478,6 +486,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::delete_function_event_invoke_config::DeleteFunctionEventInvokeConfigError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::delete_function_event_invoke_config::DeleteFunctionEventInvokeConfigError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/delete_function_url_config.rs`

```diff
--- reference/src/operation/delete_function_url_config.rs
+++ generated/src/operation/delete_function_url_config.rs
@@ -107,9 +107,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -141,9 +141,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::delete_function_url_config::DeleteFunctionUrlConfigError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::delete_function_url_config::DeleteFunctionUrlConfigError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::delete_function_url_config::DeleteFunctionUrlConfigError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -312,8 +320,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -465,6 +473,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::delete_function_url_config::DeleteFunctionUrlConfigError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::delete_function_url_config::DeleteFunctionUrlConfigError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/delete_layer_version/_delete_layer_version_input.rs`

```diff
--- reference/src/operation/delete_layer_version/_delete_layer_version_input.rs
+++ generated/src/operation/delete_layer_version/_delete_layer_version_input.rs
@@ -70,7 +70,7 @@
     {
         ::std::result::Result::Ok(crate::operation::delete_layer_version::DeleteLayerVersionInput {
             layer_name: self.layer_name,
-            version_number: self.version_number,
+            version_number: self.version_number.unwrap_or_default(),
         })
     }
 }
```

### `src/operation/delete_layer_version.rs`

```diff
--- reference/src/operation/delete_layer_version.rs
+++ generated/src/operation/delete_layer_version.rs
@@ -107,9 +107,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -125,25 +125,17 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("DeleteLayerVersion")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                DeleteLayerVersionTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                DeleteLayerVersionEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::delete_layer_version::DeleteLayerVersionError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::delete_layer_version::DeleteLayerVersionError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::delete_layer_version::DeleteLayerVersionError,
-            >::new());
+                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("DeleteLayerVersion")
+                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(DeleteLayerVersionTelemetryInputCaptureInterceptor))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(DeleteLayerVersionEndpointParamsInterceptor))
+                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::delete_layer_version::DeleteLayerVersionError>::new())
+.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::delete_layer_version::DeleteLayerVersionError>::new())
+.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::delete_layer_version::DeleteLayerVersionError>::builder().transient_errors({
+                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                                            transient_errors.push("InternalError");
+                                            ::std::borrow::Cow::Owned(transient_errors)
+                                            }).build());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -252,8 +244,7 @@
                 let input_2 = input_2
                     .as_ref()
                     .ok_or_else(|| ::aws_smithy_types::error::operation::BuildError::missing_field("version_number", "cannot be empty or unset"))?;
-                let mut version_number_encoder = ::aws_smithy_types::primitive::Encoder::from(*input_2);
-                let version_number = version_number_encoder.encode();
+                let version_number = ::aws_smithy_http::label::fmt_string(input_2, ::aws_smithy_http::label::EncodingStrategy::Default);
                 if version_number.is_empty() {
                     return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
                         "version_number",
@@ -312,8 +303,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -455,6 +446,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::delete_layer_version::DeleteLayerVersionError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::delete_layer_version::DeleteLayerVersionError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/delete_provisioned_concurrency_config.rs`

```diff
--- reference/src/operation/delete_provisioned_concurrency_config.rs
+++ generated/src/operation/delete_provisioned_concurrency_config.rs
@@ -113,9 +113,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -147,9 +147,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::delete_provisioned_concurrency_config::DeleteProvisionedConcurrencyConfigError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::delete_provisioned_concurrency_config::DeleteProvisionedConcurrencyConfigError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::delete_provisioned_concurrency_config::DeleteProvisionedConcurrencyConfigError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -333,8 +341,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -486,6 +494,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::delete_provisioned_concurrency_config::DeleteProvisionedConcurrencyConfigError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::delete_provisioned_concurrency_config::DeleteProvisionedConcurrencyConfigError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/delete_resource_policy.rs`

```diff
--- reference/src/operation/delete_resource_policy.rs
+++ generated/src/operation/delete_resource_policy.rs
@@ -107,9 +107,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -125,25 +125,17 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("DeleteResourcePolicy")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                DeleteResourcePolicyTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                DeleteResourcePolicyEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::delete_resource_policy::DeleteResourcePolicyError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::delete_resource_policy::DeleteResourcePolicyError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::delete_resource_policy::DeleteResourcePolicyError,
-            >::new());
+                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("DeleteResourcePolicy")
+                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(DeleteResourcePolicyTelemetryInputCaptureInterceptor))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(DeleteResourcePolicyEndpointParamsInterceptor))
+                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::delete_resource_policy::DeleteResourcePolicyError>::new())
+.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::delete_resource_policy::DeleteResourcePolicyError>::new())
+.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::delete_resource_policy::DeleteResourcePolicyError>::builder().transient_errors({
+                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                                            transient_errors.push("InternalError");
+                                            ::std::borrow::Cow::Owned(transient_errors)
+                                            }).build());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -312,8 +304,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -481,6 +473,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::delete_resource_policy::DeleteResourcePolicyError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::delete_resource_policy::DeleteResourcePolicyError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/get_account_settings.rs`

```diff
--- reference/src/operation/get_account_settings.rs
+++ generated/src/operation/get_account_settings.rs
@@ -107,9 +107,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -125,22 +125,17 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetAccountSettings")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                GetAccountSettingsEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::get_account_settings::GetAccountSettingsError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::get_account_settings::GetAccountSettingsError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::get_account_settings::GetAccountSettingsError,
-            >::new());
+                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetAccountSettings")
+                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(GetAccountSettingsTelemetryInputCaptureInterceptor))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(GetAccountSettingsEndpointParamsInterceptor))
+                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::get_account_settings::GetAccountSettingsError>::new())
+.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::get_account_settings::GetAccountSettingsError>::new())
+.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::get_account_settings::GetAccountSettingsError>::builder().transient_errors({
+                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                                            transient_errors.push("InternalError");
+                                            ::std::borrow::Cow::Owned(transient_errors)
+                                            }).build());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -147,6 +142,44 @@
 }

 #[derive(Debug)]
+struct GetAccountSettingsTelemetryInputCaptureInterceptor;
+
+#[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept for GetAccountSettingsTelemetryInputCaptureInterceptor {
+    fn name(&self) -> &'static str {
+        "GetAccountSettingsTelemetryInputCaptureInterceptor"
+    }
+
+    fn read_before_execution(
+        &self,
+        context: &::aws_smithy_runtime_api::client::interceptors::context::BeforeSerializationInterceptorContextRef<
+            '_,
+            ::aws_smithy_runtime_api::client::interceptors::context::Input,
+            ::aws_smithy_runtime_api::client::interceptors::context::Output,
+            ::aws_smithy_runtime_api::client::interceptors::context::Error,
+        >,
+        cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
+    ) -> ::std::result::Result<(), ::aws_smithy_runtime_api::box_error::BoxError> {
+        // Nothing to do unless the customer opted in by naming members to record.
+        let ::std::option::Option::Some(requested) = cfg
+            .load::<::aws_smithy_types::telemetry::RequestedTelemetryAttributes>()
+            .filter(|r| !r.is_empty())
+        else {
+            return ::std::result::Result::Ok(());
+        };
+
+        let ::std::option::Option::Some(input) = context.input().downcast_ref::<GetAccountSettingsInput>() else {
+            // A mismatched input is not this interceptor's concern; skip quietly.
+            return ::std::result::Result::Ok(());
+        };
+
+        let mut captured = ::aws_smithy_types::telemetry::CapturedTelemetryAttributes::default();
+
+        cfg.interceptor_state().store_put(captured);
+        ::std::result::Result::Ok(())
+    }
+}
+#[derive(Debug)]
 struct GetAccountSettingsResponseDeserializer;
 impl ::aws_smithy_runtime_api::client::ser_de::DeserializeResponse for GetAccountSettingsResponseDeserializer {
     fn deserialize_nonstreaming_with_config(
@@ -237,8 +270,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -360,6 +393,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::get_account_settings::GetAccountSettingsError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::get_account_settings::GetAccountSettingsError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/get_alias.rs`

```diff
--- reference/src/operation/get_alias.rs
+++ generated/src/operation/get_alias.rs
@@ -100,9 +100,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("GetAlias", "Lambda"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -134,9 +134,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::get_alias::GetAliasError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::get_alias::GetAliasError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::get_alias::GetAliasError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -307,8 +313,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -450,6 +456,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::get_alias::GetAliasError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::get_alias::GetAliasError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/get_capacity_provider.rs`

```diff
--- reference/src/operation/get_capacity_provider.rs
+++ generated/src/operation/get_capacity_provider.rs
@@ -107,9 +107,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -125,25 +125,17 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetCapacityProvider")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                GetCapacityProviderTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                GetCapacityProviderEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::get_capacity_provider::GetCapacityProviderError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::get_capacity_provider::GetCapacityProviderError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::get_capacity_provider::GetCapacityProviderError,
-            >::new());
+                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetCapacityProvider")
+                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(GetCapacityProviderTelemetryInputCaptureInterceptor))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(GetCapacityProviderEndpointParamsInterceptor))
+                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::get_capacity_provider::GetCapacityProviderError>::new())
+.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::get_capacity_provider::GetCapacityProviderError>::new())
+.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::get_capacity_provider::GetCapacityProviderError>::builder().transient_errors({
+                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                                            transient_errors.push("InternalError");
+                                            ::std::borrow::Cow::Owned(transient_errors)
+                                            }).build());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -299,8 +291,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -442,6 +434,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::get_capacity_provider::GetCapacityProviderError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::get_capacity_provider::GetCapacityProviderError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/get_code_signing_config.rs`

```diff
--- reference/src/operation/get_code_signing_config.rs
+++ generated/src/operation/get_code_signing_config.rs
@@ -107,9 +107,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -125,25 +125,17 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetCodeSigningConfig")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                GetCodeSigningConfigTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                GetCodeSigningConfigEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::get_code_signing_config::GetCodeSigningConfigError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::get_code_signing_config::GetCodeSigningConfigError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::get_code_signing_config::GetCodeSigningConfigError,
-            >::new());
+                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetCodeSigningConfig")
+                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(GetCodeSigningConfigTelemetryInputCaptureInterceptor))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(GetCodeSigningConfigEndpointParamsInterceptor))
+                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::get_code_signing_config::GetCodeSigningConfigError>::new())
+.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::get_code_signing_config::GetCodeSigningConfigError>::new())
+.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::get_code_signing_config::GetCodeSigningConfigError>::builder().transient_errors({
+                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                                            transient_errors.push("InternalError");
+                                            ::std::borrow::Cow::Owned(transient_errors)
+                                            }).build());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -299,8 +291,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -432,6 +424,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::get_code_signing_config::GetCodeSigningConfigError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::get_code_signing_config::GetCodeSigningConfigError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/get_durable_execution/_get_durable_execution_input.rs`

```diff
--- reference/src/operation/get_durable_execution/_get_durable_execution_input.rs
+++ generated/src/operation/get_durable_execution/_get_durable_execution_input.rs
@@ -69,7 +69,7 @@
     {
         ::std::result::Result::Ok(crate::operation::get_durable_execution::GetDurableExecutionInput {
             durable_execution_arn: self.durable_execution_arn,
-            include_execution_data: self.include_execution_data,
+            include_execution_data: self.include_execution_data.unwrap_or_default(),
         })
     }
 }
```

### `src/operation/get_durable_execution.rs`

```diff
--- reference/src/operation/get_durable_execution.rs
+++ generated/src/operation/get_durable_execution.rs
@@ -108,9 +108,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -126,25 +126,17 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetDurableExecution")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                GetDurableExecutionTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                GetDurableExecutionEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::get_durable_execution::GetDurableExecutionError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::get_durable_execution::GetDurableExecutionError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::get_durable_execution::GetDurableExecutionError,
-            >::new());
+                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetDurableExecution")
+                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(GetDurableExecutionTelemetryInputCaptureInterceptor))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(GetDurableExecutionEndpointParamsInterceptor))
+                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::get_durable_execution::GetDurableExecutionError>::new())
+.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::get_durable_execution::GetDurableExecutionError>::new())
+.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::get_durable_execution::GetDurableExecutionError>::builder().transient_errors({
+                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                                            transient_errors.push("InternalError");
+                                            ::std::borrow::Cow::Owned(transient_errors)
+                                            }).build());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -313,8 +305,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -496,6 +488,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::get_durable_execution::GetDurableExecutionError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::get_durable_execution::GetDurableExecutionError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/get_durable_execution_history/_get_durable_execution_history_input.rs`

```diff
--- reference/src/operation/get_durable_execution_history/_get_durable_execution_history_input.rs
+++ generated/src/operation/get_durable_execution_history/_get_durable_execution_history_input.rs
@@ -135,7 +135,7 @@
         ::std::result::Result::Ok(crate::operation::get_durable_execution_history::GetDurableExecutionHistoryInput {
             durable_execution_arn: self.durable_execution_arn,
             include_execution_data: self.include_execution_data,
-            max_items: self.max_items,
+            max_items: self.max_items.unwrap_or_default(),
             marker: self.marker,
             reverse_order: self.reverse_order,
         })
```

### `src/operation/get_durable_execution_history.rs`

```diff
--- reference/src/operation/get_durable_execution_history.rs
+++ generated/src/operation/get_durable_execution_history.rs
@@ -108,9 +108,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -142,9 +142,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::get_durable_execution_history::GetDurableExecutionHistoryError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::get_durable_execution_history::GetDurableExecutionHistoryError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::get_durable_execution_history::GetDurableExecutionHistoryError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -333,8 +341,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -516,6 +524,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::get_durable_execution_history::GetDurableExecutionHistoryError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::get_durable_execution_history::GetDurableExecutionHistoryError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/get_durable_execution_state/_get_durable_execution_state_input.rs`

```diff
--- reference/src/operation/get_durable_execution_state/_get_durable_execution_state_input.rs
+++ generated/src/operation/get_durable_execution_state/_get_durable_execution_state_input.rs
@@ -116,7 +116,7 @@
             durable_execution_arn: self.durable_execution_arn,
             checkpoint_token: self.checkpoint_token,
             marker: self.marker,
-            max_items: self.max_items,
+            max_items: self.max_items.unwrap_or_default(),
         })
     }
 }
```

### `src/operation/get_durable_execution_state.rs`

```diff
--- reference/src/operation/get_durable_execution_state.rs
+++ generated/src/operation/get_durable_execution_state.rs
@@ -108,9 +108,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -142,9 +142,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::get_durable_execution_state::GetDurableExecutionStateError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::get_durable_execution_state::GetDurableExecutionStateError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::get_durable_execution_state::GetDurableExecutionStateError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -339,8 +347,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -512,6 +520,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::get_durable_execution_state::GetDurableExecutionStateError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::get_durable_execution_state::GetDurableExecutionStateError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/get_event_source_mapping.rs`

```diff
--- reference/src/operation/get_event_source_mapping.rs
+++ generated/src/operation/get_event_source_mapping.rs
@@ -107,9 +107,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -125,25 +125,34 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetEventSourceMapping")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                GetEventSourceMappingTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                GetEventSourceMappingEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::get_event_source_mapping::GetEventSourceMappingError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::get_event_source_mapping::GetEventSourceMappingError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::get_event_source_mapping::GetEventSourceMappingError,
-            >::new());
+        let mut rcb =
+            ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetEventSourceMapping")
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    GetEventSourceMappingTelemetryInputCaptureInterceptor,
+                ))
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
+                ))
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    GetEventSourceMappingEndpointParamsInterceptor,
+                ))
+                .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+                    crate::operation::get_event_source_mapping::GetEventSourceMappingError,
+                >::new())
+                .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+                    crate::operation::get_event_source_mapping::GetEventSourceMappingError,
+                >::new())
+                .with_retry_classifier(
+                    ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                        crate::operation::get_event_source_mapping::GetEventSourceMappingError,
+                    >::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+                );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -294,8 +303,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -437,6 +446,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::get_event_source_mapping::GetEventSourceMappingError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::get_event_source_mapping::GetEventSourceMappingError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/get_function.rs`

```diff
--- reference/src/operation/get_function.rs
+++ generated/src/operation/get_function.rs
@@ -105,9 +105,9 @@
         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::SensitiveOutput);
         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("GetFunction", "Lambda"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -139,9 +139,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::get_function::GetFunctionError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::get_function::GetFunctionError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::get_function::GetFunctionError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -310,8 +316,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -453,6 +459,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::get_function::GetFunctionError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::get_function::GetFunctionError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/get_function_code_signing_config.rs`

```diff
--- reference/src/operation/get_function_code_signing_config.rs
+++ generated/src/operation/get_function_code_signing_config.rs
@@ -113,9 +113,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -147,9 +147,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::get_function_code_signing_config::GetFunctionCodeSigningConfigError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::get_function_code_signing_config::GetFunctionCodeSigningConfigError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::get_function_code_signing_config::GetFunctionCodeSigningConfigError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -305,8 +313,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -458,6 +466,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::get_function_code_signing_config::GetFunctionCodeSigningConfigError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::get_function_code_signing_config::GetFunctionCodeSigningConfigError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/get_function_concurrency.rs`

```diff
--- reference/src/operation/get_function_concurrency.rs
+++ generated/src/operation/get_function_concurrency.rs
@@ -107,9 +107,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -125,25 +125,34 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetFunctionConcurrency")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                GetFunctionConcurrencyTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                GetFunctionConcurrencyEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::get_function_concurrency::GetFunctionConcurrencyError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::get_function_concurrency::GetFunctionConcurrencyError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::get_function_concurrency::GetFunctionConcurrencyError,
-            >::new());
+        let mut rcb =
+            ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetFunctionConcurrency")
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    GetFunctionConcurrencyTelemetryInputCaptureInterceptor,
+                ))
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
+                ))
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    GetFunctionConcurrencyEndpointParamsInterceptor,
+                ))
+                .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+                    crate::operation::get_function_concurrency::GetFunctionConcurrencyError,
+                >::new())
+                .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+                    crate::operation::get_function_concurrency::GetFunctionConcurrencyError,
+                >::new())
+                .with_retry_classifier(
+                    ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                        crate::operation::get_function_concurrency::GetFunctionConcurrencyError,
+                    >::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+                );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -295,8 +304,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -438,6 +447,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::get_function_concurrency::GetFunctionConcurrencyError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::get_function_concurrency::GetFunctionConcurrencyError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/get_function_configuration.rs`

```diff
--- reference/src/operation/get_function_configuration.rs
+++ generated/src/operation/get_function_configuration.rs
@@ -108,9 +108,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -142,9 +142,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::get_function_configuration::GetFunctionConfigurationError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::get_function_configuration::GetFunctionConfigurationError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::get_function_configuration::GetFunctionConfigurationError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -314,8 +322,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -457,6 +465,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::get_function_configuration::GetFunctionConfigurationError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::get_function_configuration::GetFunctionConfigurationError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/get_function_event_invoke_config.rs`

```diff
--- reference/src/operation/get_function_event_invoke_config.rs
+++ generated/src/operation/get_function_event_invoke_config.rs
@@ -113,9 +113,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -147,9 +147,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::get_function_event_invoke_config::GetFunctionEventInvokeConfigError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::get_function_event_invoke_config::GetFunctionEventInvokeConfigError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::get_function_event_invoke_config::GetFunctionEventInvokeConfigError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -323,8 +331,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -466,6 +474,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::get_function_event_invoke_config::GetFunctionEventInvokeConfigError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::get_function_event_invoke_config::GetFunctionEventInvokeConfigError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/get_function_recursion_config.rs`

```diff
--- reference/src/operation/get_function_recursion_config.rs
+++ generated/src/operation/get_function_recursion_config.rs
@@ -107,9 +107,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -141,9 +141,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::get_function_recursion_config::GetFunctionRecursionConfigError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::get_function_recursion_config::GetFunctionRecursionConfigError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::get_function_recursion_config::GetFunctionRecursionConfigError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -299,8 +307,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -442,6 +450,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::get_function_recursion_config::GetFunctionRecursionConfigError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::get_function_recursion_config::GetFunctionRecursionConfigError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/get_function_scaling_config.rs`

```diff
--- reference/src/operation/get_function_scaling_config.rs
+++ generated/src/operation/get_function_scaling_config.rs
@@ -107,9 +107,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -141,9 +141,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::get_function_scaling_config::GetFunctionScalingConfigError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::get_function_scaling_config::GetFunctionScalingConfigError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::get_function_scaling_config::GetFunctionScalingConfigError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -323,8 +331,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -466,6 +474,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::get_function_scaling_config::GetFunctionScalingConfigError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::get_function_scaling_config::GetFunctionScalingConfigError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/get_function_url_config.rs`

```diff
--- reference/src/operation/get_function_url_config.rs
+++ generated/src/operation/get_function_url_config.rs
@@ -107,9 +107,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -125,25 +125,17 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetFunctionUrlConfig")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                GetFunctionUrlConfigTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                GetFunctionUrlConfigEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::get_function_url_config::GetFunctionUrlConfigError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::get_function_url_config::GetFunctionUrlConfigError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::get_function_url_config::GetFunctionUrlConfigError,
-            >::new());
+                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetFunctionUrlConfig")
+                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(GetFunctionUrlConfigTelemetryInputCaptureInterceptor))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(GetFunctionUrlConfigEndpointParamsInterceptor))
+                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::get_function_url_config::GetFunctionUrlConfigError>::new())
+.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::get_function_url_config::GetFunctionUrlConfigError>::new())
+.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::get_function_url_config::GetFunctionUrlConfigError>::builder().transient_errors({
+                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                                            transient_errors.push("InternalError");
+                                            ::std::borrow::Cow::Owned(transient_errors)
+                                            }).build());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -312,8 +304,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -455,6 +447,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::get_function_url_config::GetFunctionUrlConfigError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::get_function_url_config::GetFunctionUrlConfigError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/get_layer_version/_get_layer_version_input.rs`

```diff
--- reference/src/operation/get_layer_version/_get_layer_version_input.rs
+++ generated/src/operation/get_layer_version/_get_layer_version_input.rs
@@ -69,7 +69,7 @@
     ) -> ::std::result::Result<crate::operation::get_layer_version::GetLayerVersionInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(crate::operation::get_layer_version::GetLayerVersionInput {
             layer_name: self.layer_name,
-            version_number: self.version_number,
+            version_number: self.version_number.unwrap_or_default(),
         })
     }
 }
```

### `src/operation/get_layer_version/_get_layer_version_output.rs`

```diff
--- reference/src/operation/get_layer_version/_get_layer_version_output.rs
+++ generated/src/operation/get_layer_version/_get_layer_version_output.rs
@@ -4,7 +4,7 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct GetLayerVersionOutput {
     /// <p>Details about the layer version.</p>
-    pub content: ::std::option::Option<crate::types::LayerVersionContentOutput>,
+    pub content: ::std::option::Option<crate::operation::get_layer_version::Output>,
     /// <p>The ARN of the layer.</p>
     pub layer_arn: ::std::option::Option<::std::string::String>,
     /// <p>The ARN of the layer version.</p>
@@ -27,7 +27,7 @@
 }
 impl GetLayerVersionOutput {
     /// <p>Details about the layer version.</p>
-    pub fn content(&self) -> ::std::option::Option<&crate::types::LayerVersionContentOutput> {
+    pub fn content(&self) -> ::std::option::Option<&crate::operation::get_layer_version::Output> {
         self.content.as_ref()
     }
     /// <p>The ARN of the layer.</p>
@@ -85,7 +85,7 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
 pub struct GetLayerVersionOutputBuilder {
-    pub(crate) content: ::std::option::Option<crate::types::LayerVersionContentOutput>,
+    pub(crate) content: ::std::option::Option<crate::operation::get_layer_version::Output>,
     pub(crate) layer_arn: ::std::option::Option<::std::string::String>,
     pub(crate) layer_version_arn: ::std::option::Option<::std::string::String>,
     pub(crate) description: ::std::option::Option<::std::string::String>,
@@ -98,17 +98,17 @@
 }
 impl GetLayerVersionOutputBuilder {
     /// <p>Details about the layer version.</p>
-    pub fn content(mut self, input: crate::types::LayerVersionContentOutput) -> Self {
+    pub fn content(mut self, input: crate::operation::get_layer_version::Output) -> Self {
         self.content = ::std::option::Option::Some(input);
         self
     }
     /// <p>Details about the layer version.</p>
-    pub fn set_content(mut self, input: ::std::option::Option<crate::types::LayerVersionContentOutput>) -> Self {
+    pub fn set_content(mut self, input: ::std::option::Option<crate::operation::get_layer_version::Output>) -> Self {
         self.content = input;
         self
     }
     /// <p>Details about the layer version.</p>
-    pub fn get_content(&self) -> &::std::option::Option<crate::types::LayerVersionContentOutput> {
+    pub fn get_content(&self) -> &::std::option::Option<crate::operation::get_layer_version::Output> {
         &self.content
     }
     /// <p>The ARN of the layer.</p>
```

### `src/operation/get_layer_version.rs`

```diff
--- reference/src/operation/get_layer_version.rs
+++ generated/src/operation/get_layer_version.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("GetLayerVersion", "Lambda"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -138,9 +138,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::get_layer_version::GetLayerVersionError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::get_layer_version::GetLayerVersionError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::get_layer_version::GetLayerVersionError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -249,8 +255,7 @@
                 let input_2 = input_2
                     .as_ref()
                     .ok_or_else(|| ::aws_smithy_types::error::operation::BuildError::missing_field("version_number", "cannot be empty or unset"))?;
-                let mut version_number_encoder = ::aws_smithy_types::primitive::Encoder::from(*input_2);
-                let version_number = version_number_encoder.encode();
+                let version_number = ::aws_smithy_http::label::fmt_string(input_2, ::aws_smithy_http::label::EncodingStrategy::Default);
                 if version_number.is_empty() {
                     return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
                         "version_number",
@@ -309,8 +314,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -452,6 +457,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::get_layer_version::GetLayerVersionError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::get_layer_version::GetLayerVersionError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/get_layer_version_by_arn/_get_layer_version_by_arn_output.rs`

```diff
--- reference/src/operation/get_layer_version_by_arn/_get_layer_version_by_arn_output.rs
+++ generated/src/operation/get_layer_version_by_arn/_get_layer_version_by_arn_output.rs
@@ -4,7 +4,7 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct GetLayerVersionByArnOutput {
     /// <p>Details about the layer version.</p>
-    pub content: ::std::option::Option<crate::types::LayerVersionContentOutput>,
+    pub content: ::std::option::Option<crate::operation::get_layer_version_by_arn::Output>,
     /// <p>The ARN of the layer.</p>
     pub layer_arn: ::std::option::Option<::std::string::String>,
     /// <p>The ARN of the layer version.</p>
@@ -27,7 +27,7 @@
 }
 impl GetLayerVersionByArnOutput {
     /// <p>Details about the layer version.</p>
-    pub fn content(&self) -> ::std::option::Option<&crate::types::LayerVersionContentOutput> {
+    pub fn content(&self) -> ::std::option::Option<&crate::operation::get_layer_version_by_arn::Output> {
         self.content.as_ref()
     }
     /// <p>The ARN of the layer.</p>
@@ -85,7 +85,7 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
 pub struct GetLayerVersionByArnOutputBuilder {
-    pub(crate) content: ::std::option::Option<crate::types::LayerVersionContentOutput>,
+    pub(crate) content: ::std::option::Option<crate::operation::get_layer_version_by_arn::Output>,
     pub(crate) layer_arn: ::std::option::Option<::std::string::String>,
     pub(crate) layer_version_arn: ::std::option::Option<::std::string::String>,
     pub(crate) description: ::std::option::Option<::std::string::String>,
@@ -98,17 +98,17 @@
 }
 impl GetLayerVersionByArnOutputBuilder {
     /// <p>Details about the layer version.</p>
-    pub fn content(mut self, input: crate::types::LayerVersionContentOutput) -> Self {
+    pub fn content(mut self, input: crate::operation::get_layer_version_by_arn::Output) -> Self {
         self.content = ::std::option::Option::Some(input);
         self
     }
     /// <p>Details about the layer version.</p>
-    pub fn set_content(mut self, input: ::std::option::Option<crate::types::LayerVersionContentOutput>) -> Self {
+    pub fn set_content(mut self, input: ::std::option::Option<crate::operation::get_layer_version_by_arn::Output>) -> Self {
         self.content = input;
         self
     }
     /// <p>Details about the layer version.</p>
-    pub fn get_content(&self) -> &::std::option::Option<crate::types::LayerVersionContentOutput> {
+    pub fn get_content(&self) -> &::std::option::Option<crate::operation::get_layer_version_by_arn::Output> {
         &self.content
     }
     /// <p>The ARN of the layer.</p>
```

### `src/operation/get_layer_version_by_arn.rs`

```diff
--- reference/src/operation/get_layer_version_by_arn.rs
+++ generated/src/operation/get_layer_version_by_arn.rs
@@ -107,9 +107,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -125,25 +125,34 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetLayerVersionByArn")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                GetLayerVersionByArnTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                GetLayerVersionByArnEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::get_layer_version_by_arn::GetLayerVersionByArnError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::get_layer_version_by_arn::GetLayerVersionByArnError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::get_layer_version_by_arn::GetLayerVersionByArnError,
-            >::new());
+        let mut rcb =
+            ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetLayerVersionByArn")
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    GetLayerVersionByArnTelemetryInputCaptureInterceptor,
+                ))
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
+                ))
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    GetLayerVersionByArnEndpointParamsInterceptor,
+                ))
+                .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+                    crate::operation::get_layer_version_by_arn::GetLayerVersionByArnError,
+                >::new())
+                .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+                    crate::operation::get_layer_version_by_arn::GetLayerVersionByArnError,
+                >::new())
+                .with_retry_classifier(
+                    ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                        crate::operation::get_layer_version_by_arn::GetLayerVersionByArnError,
+                    >::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+                );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -303,8 +312,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -446,6 +455,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::get_layer_version_by_arn::GetLayerVersionByArnError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::get_layer_version_by_arn::GetLayerVersionByArnError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/get_layer_version_policy/_get_layer_version_policy_input.rs`

```diff
--- reference/src/operation/get_layer_version_policy/_get_layer_version_policy_input.rs
+++ generated/src/operation/get_layer_version_policy/_get_layer_version_policy_input.rs
@@ -70,7 +70,7 @@
     {
         ::std::result::Result::Ok(crate::operation::get_layer_version_policy::GetLayerVersionPolicyInput {
             layer_name: self.layer_name,
-            version_number: self.version_number,
+            version_number: self.version_number.unwrap_or_default(),
         })
     }
 }
```

### `src/operation/get_layer_version_policy.rs`

```diff
--- reference/src/operation/get_layer_version_policy.rs
+++ generated/src/operation/get_layer_version_policy.rs
@@ -107,9 +107,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -125,25 +125,34 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetLayerVersionPolicy")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                GetLayerVersionPolicyTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                GetLayerVersionPolicyEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::get_layer_version_policy::GetLayerVersionPolicyError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::get_layer_version_policy::GetLayerVersionPolicyError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::get_layer_version_policy::GetLayerVersionPolicyError,
-            >::new());
+        let mut rcb =
+            ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetLayerVersionPolicy")
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    GetLayerVersionPolicyTelemetryInputCaptureInterceptor,
+                ))
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
+                ))
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    GetLayerVersionPolicyEndpointParamsInterceptor,
+                ))
+                .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+                    crate::operation::get_layer_version_policy::GetLayerVersionPolicyError,
+                >::new())
+                .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+                    crate::operation::get_layer_version_policy::GetLayerVersionPolicyError,
+                >::new())
+                .with_retry_classifier(
+                    ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                        crate::operation::get_layer_version_policy::GetLayerVersionPolicyError,
+                    >::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+                );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -252,8 +261,7 @@
                 let input_2 = input_2
                     .as_ref()
                     .ok_or_else(|| ::aws_smithy_types::error::operation::BuildError::missing_field("version_number", "cannot be empty or unset"))?;
-                let mut version_number_encoder = ::aws_smithy_types::primitive::Encoder::from(*input_2);
-                let version_number = version_number_encoder.encode();
+                let version_number = ::aws_smithy_http::label::fmt_string(input_2, ::aws_smithy_http::label::EncodingStrategy::Default);
                 if version_number.is_empty() {
                     return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
                         "version_number",
@@ -312,8 +320,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -455,6 +463,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::get_layer_version_policy::GetLayerVersionPolicyError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::get_layer_version_policy::GetLayerVersionPolicyError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/get_policy.rs`

```diff
--- reference/src/operation/get_policy.rs
+++ generated/src/operation/get_policy.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("GetPolicy", "Lambda"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -138,9 +138,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::get_policy::GetPolicyError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::get_policy::GetPolicyError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::get_policy::GetPolicyError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -308,8 +314,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -451,6 +457,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::get_policy::GetPolicyError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::get_policy::GetPolicyError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/get_provisioned_concurrency_config.rs`

```diff
--- reference/src/operation/get_provisioned_concurrency_config.rs
+++ generated/src/operation/get_provisioned_concurrency_config.rs
@@ -113,9 +113,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -147,9 +147,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::get_provisioned_concurrency_config::GetProvisionedConcurrencyConfigError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::get_provisioned_concurrency_config::GetProvisionedConcurrencyConfigError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::get_provisioned_concurrency_config::GetProvisionedConcurrencyConfigError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -331,8 +339,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -484,6 +492,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::get_provisioned_concurrency_config::GetProvisionedConcurrencyConfigError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::get_provisioned_concurrency_config::GetProvisionedConcurrencyConfigError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/get_resource_policy.rs`

```diff
--- reference/src/operation/get_resource_policy.rs
+++ generated/src/operation/get_resource_policy.rs
@@ -107,9 +107,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -125,25 +125,17 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetResourcePolicy")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                GetResourcePolicyTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                GetResourcePolicyEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::get_resource_policy::GetResourcePolicyError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::get_resource_policy::GetResourcePolicyError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::get_resource_policy::GetResourcePolicyError,
-            >::new());
+                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetResourcePolicy")
+                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(GetResourcePolicyTelemetryInputCaptureInterceptor))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(GetResourcePolicyEndpointParamsInterceptor))
+                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::get_resource_policy::GetResourcePolicyError>::new())
+.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::get_resource_policy::GetResourcePolicyError>::new())
+.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::get_resource_policy::GetResourcePolicyError>::builder().transient_errors({
+                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                                            transient_errors.push("InternalError");
+                                            ::std::borrow::Cow::Owned(transient_errors)
+                                            }).build());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -294,8 +286,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -437,6 +429,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::get_resource_policy::GetResourcePolicyError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::get_resource_policy::GetResourcePolicyError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/get_runtime_management_config.rs`

```diff
--- reference/src/operation/get_runtime_management_config.rs
+++ generated/src/operation/get_runtime_management_config.rs
@@ -107,9 +107,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -141,9 +141,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::get_runtime_management_config::GetRuntimeManagementConfigError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::get_runtime_management_config::GetRuntimeManagementConfigError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::get_runtime_management_config::GetRuntimeManagementConfigError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -317,8 +325,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -460,6 +468,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::get_runtime_management_config::GetRuntimeManagementConfigError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::get_runtime_management_config::GetRuntimeManagementConfigError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/invoke.rs`

```diff
--- reference/src/operation/invoke.rs
+++ generated/src/operation/invoke.rs
@@ -97,9 +97,9 @@
         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::SensitiveOutput);
         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("Invoke", "Lambda"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -120,6 +120,9 @@
                 InvokeTelemetryInputCaptureInterceptor,
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
+            ))
+            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 InvokeEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
@@ -128,9 +131,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::invoke::InvokeError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::invoke::InvokeError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::invoke::InvokeError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -315,8 +324,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -848,6 +857,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::invoke::InvokeError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::invoke::InvokeError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/invoke_async.rs`

```diff
--- reference/src/operation/invoke_async.rs
+++ generated/src/operation/invoke_async.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("InvokeAsync", "Lambda"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -127,6 +127,9 @@
                 InvokeAsyncTelemetryInputCaptureInterceptor,
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
+            ))
+            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 InvokeAsyncEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
@@ -135,9 +138,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::invoke_async::InvokeAsyncError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::invoke_async::InvokeAsyncError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::invoke_async::InvokeAsyncError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -259,7 +268,9 @@
             builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/octet-stream");
             builder
         };
-        let body = crate::protocol_serde::shape_invoke_async_input::ser_invoke_args_http_payload(input.invoke_args)?.into_inner();
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_invoke_async_input::ser_invoke_args_http_payload(
+            input.invoke_args,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -293,8 +304,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -686,6 +697,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::invoke_async::InvokeAsyncError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::invoke_async::InvokeAsyncError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/invoke_with_response_stream.rs`

```diff
--- reference/src/operation/invoke_with_response_stream.rs
+++ generated/src/operation/invoke_with_response_stream.rs
@@ -108,9 +108,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -139,9 +139,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::invoke_with_response_stream::InvokeWithResponseStreamError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::invoke_with_response_stream::InvokeWithResponseStreamError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::invoke_with_response_stream::InvokeWithResponseStreamError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -214,6 +222,7 @@
     ) -> ::std::option::Option<::aws_smithy_runtime_api::client::interceptors::context::OutputOrError> {
         #[allow(unused_mut)]
         let mut force_error = false;
+        ::tracing::debug!(extended_request_id = ?crate::s3_request_id::RequestIdExt::extended_request_id(response));
         ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));

         // If this is an error, defer to the non-streaming parser
@@ -346,8 +355,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -819,6 +828,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::invoke_with_response_stream::InvokeWithResponseStreamError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::invoke_with_response_stream::InvokeWithResponseStreamError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/list_aliases.rs`

```diff
--- reference/src/operation/list_aliases.rs
+++ generated/src/operation/list_aliases.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("ListAliases", "Lambda"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -138,9 +138,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::list_aliases::ListAliasesError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::list_aliases::ListAliasesError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::list_aliases::ListAliasesError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -325,8 +331,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -468,6 +474,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::list_aliases::ListAliasesError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::list_aliases::ListAliasesError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/list_capacity_providers.rs`

```diff
--- reference/src/operation/list_capacity_providers.rs
+++ generated/src/operation/list_capacity_providers.rs
@@ -107,9 +107,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -125,25 +125,34 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ListCapacityProviders")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ListCapacityProvidersTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ListCapacityProvidersEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::list_capacity_providers::ListCapacityProvidersError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::list_capacity_providers::ListCapacityProvidersError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::list_capacity_providers::ListCapacityProvidersError,
-            >::new());
+        let mut rcb =
+            ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ListCapacityProviders")
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    ListCapacityProvidersTelemetryInputCaptureInterceptor,
+                ))
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
+                ))
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    ListCapacityProvidersEndpointParamsInterceptor,
+                ))
+                .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+                    crate::operation::list_capacity_providers::ListCapacityProvidersError,
+                >::new())
+                .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+                    crate::operation::list_capacity_providers::ListCapacityProvidersError,
+                >::new())
+                .with_retry_classifier(
+                    ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                        crate::operation::list_capacity_providers::ListCapacityProvidersError,
+                    >::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+                );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -306,8 +315,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -439,6 +448,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::list_capacity_providers::ListCapacityProvidersError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::list_capacity_providers::ListCapacityProvidersError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/list_code_signing_configs.rs`

```diff
--- reference/src/operation/list_code_signing_configs.rs
+++ generated/src/operation/list_code_signing_configs.rs
@@ -107,9 +107,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -125,25 +125,34 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ListCodeSigningConfigs")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ListCodeSigningConfigsTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ListCodeSigningConfigsEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::list_code_signing_configs::ListCodeSigningConfigsError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::list_code_signing_configs::ListCodeSigningConfigsError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::list_code_signing_configs::ListCodeSigningConfigsError,
-            >::new());
+        let mut rcb =
+            ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ListCodeSigningConfigs")
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    ListCodeSigningConfigsTelemetryInputCaptureInterceptor,
+                ))
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
+                ))
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    ListCodeSigningConfigsEndpointParamsInterceptor,
+                ))
+                .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+                    crate::operation::list_code_signing_configs::ListCodeSigningConfigsError,
+                >::new())
+                .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+                    crate::operation::list_code_signing_configs::ListCodeSigningConfigsError,
+                >::new())
+                .with_retry_classifier(
+                    ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                        crate::operation::list_code_signing_configs::ListCodeSigningConfigsError,
+                    >::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+                );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -301,8 +310,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -424,6 +433,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::list_code_signing_configs::ListCodeSigningConfigsError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::list_code_signing_configs::ListCodeSigningConfigsError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/list_durable_executions_by_function/_list_durable_executions_by_function_input.rs`

```diff
--- reference/src/operation/list_durable_executions_by_function/_list_durable_executions_by_function_input.rs
+++ generated/src/operation/list_durable_executions_by_function/_list_durable_executions_by_function_input.rs
@@ -234,7 +234,7 @@
                 started_before: self.started_before,
                 reverse_order: self.reverse_order,
                 marker: self.marker,
-                max_items: self.max_items,
+                max_items: self.max_items.unwrap_or_default(),
             },
         )
     }
```

### `src/operation/list_durable_executions_by_function.rs`

```diff
--- reference/src/operation/list_durable_executions_by_function.rs
+++ generated/src/operation/list_durable_executions_by_function.rs
@@ -113,9 +113,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -147,9 +147,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::list_durable_executions_by_function::ListDurableExecutionsByFunctionError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::list_durable_executions_by_function::ListDurableExecutionsByFunctionError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::list_durable_executions_by_function::ListDurableExecutionsByFunctionError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -296,40 +304,38 @@
                 }
                 if let ::std::option::Option::Some(inner_4) = &_input.statuses {
                     {
-                        for inner_5 in inner_4 {
-                            query.push_kv("Statuses", &::aws_smithy_http::query::fmt_string(inner_5.as_str()));
-                        }
+                        query.push_kv("Statuses", ::aws_smithy_types::primitive::Encoder::from(*inner_4).encode());
                     }
                 }
-                if let ::std::option::Option::Some(inner_6) = &_input.started_after {
+                if let ::std::option::Option::Some(inner_5) = &_input.started_after {
                     {
                         query.push_kv(
                             "StartedAfter",
-                            &::aws_smithy_http::query::fmt_timestamp(inner_6, ::aws_smithy_types::date_time::Format::DateTime)?,
+                            &::aws_smithy_http::query::fmt_timestamp(inner_5, ::aws_smithy_types::date_time::Format::HttpDate)?,
                         );
                     }
                 }
-                if let ::std::option::Option::Some(inner_7) = &_input.started_before {
+                if let ::std::option::Option::Some(inner_6) = &_input.started_before {
                     {
                         query.push_kv(
                             "StartedBefore",
-                            &::aws_smithy_http::query::fmt_timestamp(inner_7, ::aws_smithy_types::date_time::Format::DateTime)?,
+                            &::aws_smithy_http::query::fmt_timestamp(inner_6, ::aws_smithy_types::date_time::Format::HttpDate)?,
                         );
                     }
                 }
-                if let ::std::option::Option::Some(inner_8) = &_input.reverse_order {
+                if let ::std::option::Option::Some(inner_7) = &_input.reverse_order {
                     {
-                        query.push_kv("ReverseOrder", ::aws_smithy_types::primitive::Encoder::from(*inner_8).encode());
+                        query.push_kv("ReverseOrder", ::aws_smithy_types::primitive::Encoder::from(*inner_7).encode());
                     }
                 }
-                if let ::std::option::Option::Some(inner_9) = &_input.marker {
+                if let ::std::option::Option::Some(inner_8) = &_input.marker {
                     {
-                        query.push_kv("Marker", &::aws_smithy_http::query::fmt_string(inner_9));
+                        query.push_kv("Marker", &::aws_smithy_http::query::fmt_string(inner_8));
                     }
                 }
-                if let ::std::option::Option::Some(inner_10) = &_input.max_items {
+                if let ::std::option::Option::Some(inner_9) = &_input.max_items {
                     {
-                        query.push_kv("MaxItems", ::aws_smithy_types::primitive::Encoder::from(*inner_10).encode());
+                        query.push_kv("MaxItems", ::aws_smithy_types::primitive::Encoder::from(*inner_9).encode());
                     }
                 }
                 ::std::result::Result::Ok(())
@@ -378,8 +384,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -521,6 +527,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::list_durable_executions_by_function::ListDurableExecutionsByFunctionError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::list_durable_executions_by_function::ListDurableExecutionsByFunctionError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/list_event_source_mappings.rs`

```diff
--- reference/src/operation/list_event_source_mappings.rs
+++ generated/src/operation/list_event_source_mappings.rs
@@ -107,9 +107,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -141,9 +141,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::list_event_source_mappings::ListEventSourceMappingsError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::list_event_source_mappings::ListEventSourceMappingsError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::list_event_source_mappings::ListEventSourceMappingsError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -321,8 +329,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -464,6 +472,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::list_event_source_mappings::ListEventSourceMappingsError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::list_event_source_mappings::ListEventSourceMappingsError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/list_function_event_invoke_configs.rs`

```diff
--- reference/src/operation/list_function_event_invoke_configs.rs
+++ generated/src/operation/list_function_event_invoke_configs.rs
@@ -113,9 +113,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -147,9 +147,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::list_function_event_invoke_configs::ListFunctionEventInvokeConfigsError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::list_function_event_invoke_configs::ListFunctionEventInvokeConfigsError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::list_function_event_invoke_configs::ListFunctionEventInvokeConfigsError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -330,8 +338,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -473,6 +481,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::list_function_event_invoke_configs::ListFunctionEventInvokeConfigsError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::list_function_event_invoke_configs::ListFunctionEventInvokeConfigsError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/list_function_url_configs.rs`

```diff
--- reference/src/operation/list_function_url_configs.rs
+++ generated/src/operation/list_function_url_configs.rs
@@ -107,9 +107,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -125,25 +125,34 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ListFunctionUrlConfigs")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ListFunctionUrlConfigsTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ListFunctionUrlConfigsEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::list_function_url_configs::ListFunctionUrlConfigsError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::list_function_url_configs::ListFunctionUrlConfigsError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::list_function_url_configs::ListFunctionUrlConfigsError,
-            >::new());
+        let mut rcb =
+            ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ListFunctionUrlConfigs")
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    ListFunctionUrlConfigsTelemetryInputCaptureInterceptor,
+                ))
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
+                ))
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    ListFunctionUrlConfigsEndpointParamsInterceptor,
+                ))
+                .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+                    crate::operation::list_function_url_configs::ListFunctionUrlConfigsError,
+                >::new())
+                .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+                    crate::operation::list_function_url_configs::ListFunctionUrlConfigsError,
+                >::new())
+                .with_retry_classifier(
+                    ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                        crate::operation::list_function_url_configs::ListFunctionUrlConfigsError,
+                    >::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+                );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -317,8 +326,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -460,6 +469,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::list_function_url_configs::ListFunctionUrlConfigsError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::list_function_url_configs::ListFunctionUrlConfigsError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/list_function_versions_by_capacity_provider.rs`

```diff
--- reference/src/operation/list_function_versions_by_capacity_provider.rs
+++ generated/src/operation/list_function_versions_by_capacity_provider.rs
@@ -113,9 +113,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -147,9 +147,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::list_function_versions_by_capacity_provider::ListFunctionVersionsByCapacityProviderError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::list_function_versions_by_capacity_provider::ListFunctionVersionsByCapacityProviderError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::list_function_versions_by_capacity_provider::ListFunctionVersionsByCapacityProviderError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -332,8 +340,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -475,6 +483,13 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt
+    for crate::operation::list_function_versions_by_capacity_provider::ListFunctionVersionsByCapacityProviderError
+{
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId
     for crate::operation::list_function_versions_by_capacity_provider::ListFunctionVersionsByCapacityProviderError
 {
```

### `src/operation/list_functions.rs`

```diff
--- reference/src/operation/list_functions.rs
+++ generated/src/operation/list_functions.rs
@@ -105,9 +105,9 @@
         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::SensitiveOutput);
         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("ListFunctions", "Lambda"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -139,9 +139,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::list_functions::ListFunctionsError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::list_functions::ListFunctionsError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::list_functions::ListFunctionsError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -314,8 +320,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -447,6 +453,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::list_functions::ListFunctionsError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::list_functions::ListFunctionsError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/list_functions_by_code_signing_config.rs`

```diff
--- reference/src/operation/list_functions_by_code_signing_config.rs
+++ generated/src/operation/list_functions_by_code_signing_config.rs
@@ -113,9 +113,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -147,9 +147,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::list_functions_by_code_signing_config::ListFunctionsByCodeSigningConfigError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::list_functions_by_code_signing_config::ListFunctionsByCodeSigningConfigError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::list_functions_by_code_signing_config::ListFunctionsByCodeSigningConfigError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -332,8 +340,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -465,6 +473,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::list_functions_by_code_signing_config::ListFunctionsByCodeSigningConfigError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::list_functions_by_code_signing_config::ListFunctionsByCodeSigningConfigError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/list_layer_versions.rs`

```diff
--- reference/src/operation/list_layer_versions.rs
+++ generated/src/operation/list_layer_versions.rs
@@ -107,9 +107,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -125,25 +125,17 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ListLayerVersions")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ListLayerVersionsTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ListLayerVersionsEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::list_layer_versions::ListLayerVersionsError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::list_layer_versions::ListLayerVersionsError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::list_layer_versions::ListLayerVersionsError,
-            >::new());
+                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ListLayerVersions")
+                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(ListLayerVersionsTelemetryInputCaptureInterceptor))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(ListLayerVersionsEndpointParamsInterceptor))
+                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::list_layer_versions::ListLayerVersionsError>::new())
+.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::list_layer_versions::ListLayerVersionsError>::new())
+.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::list_layer_versions::ListLayerVersionsError>::builder().transient_errors({
+                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                                            transient_errors.push("InternalError");
+                                            ::std::borrow::Cow::Owned(transient_errors)
+                                            }).build());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -327,8 +319,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -470,6 +462,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::list_layer_versions::ListLayerVersionsError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::list_layer_versions::ListLayerVersionsError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/list_layers.rs`

```diff
--- reference/src/operation/list_layers.rs
+++ generated/src/operation/list_layers.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("ListLayers", "Lambda"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -138,9 +138,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::list_layers::ListLayersError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::list_layers::ListLayersError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::list_layers::ListLayersError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -306,8 +312,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -439,6 +445,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::list_layers::ListLayersError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::list_layers::ListLayersError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/list_provisioned_concurrency_configs.rs`

```diff
--- reference/src/operation/list_provisioned_concurrency_configs.rs
+++ generated/src/operation/list_provisioned_concurrency_configs.rs
@@ -113,9 +113,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -147,9 +147,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::list_provisioned_concurrency_configs::ListProvisionedConcurrencyConfigsError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::list_provisioned_concurrency_configs::ListProvisionedConcurrencyConfigsError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::list_provisioned_concurrency_configs::ListProvisionedConcurrencyConfigsError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -333,8 +341,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -476,6 +484,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::list_provisioned_concurrency_configs::ListProvisionedConcurrencyConfigsError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::list_provisioned_concurrency_configs::ListProvisionedConcurrencyConfigsError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/list_tags.rs`

```diff
--- reference/src/operation/list_tags.rs
+++ generated/src/operation/list_tags.rs
@@ -100,9 +100,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("ListTags", "Lambda"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -134,9 +134,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::list_tags::ListTagsError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::list_tags::ListTagsError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::list_tags::ListTagsError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -285,8 +291,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -428,6 +434,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::list_tags::ListTagsError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::list_tags::ListTagsError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/list_versions_by_function.rs`

```diff
--- reference/src/operation/list_versions_by_function.rs
+++ generated/src/operation/list_versions_by_function.rs
@@ -108,9 +108,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -126,25 +126,34 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ListVersionsByFunction")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ListVersionsByFunctionTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ListVersionsByFunctionEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::list_versions_by_function::ListVersionsByFunctionError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::list_versions_by_function::ListVersionsByFunctionError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::list_versions_by_function::ListVersionsByFunctionError,
-            >::new());
+        let mut rcb =
+            ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ListVersionsByFunction")
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    ListVersionsByFunctionTelemetryInputCaptureInterceptor,
+                ))
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
+                ))
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    ListVersionsByFunctionEndpointParamsInterceptor,
+                ))
+                .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+                    crate::operation::list_versions_by_function::ListVersionsByFunctionError,
+                >::new())
+                .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+                    crate::operation::list_versions_by_function::ListVersionsByFunctionError,
+                >::new())
+                .with_retry_classifier(
+                    ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                        crate::operation::list_versions_by_function::ListVersionsByFunctionError,
+                    >::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+                );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -319,8 +328,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -462,6 +471,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::list_versions_by_function::ListVersionsByFunctionError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::list_versions_by_function::ListVersionsByFunctionError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/publish_layer_version/_publish_layer_version_input.rs`

```diff
--- reference/src/operation/publish_layer_version/_publish_layer_version_input.rs
+++ generated/src/operation/publish_layer_version/_publish_layer_version_input.rs
@@ -8,7 +8,7 @@
     /// <p>The description of the version.</p>
     pub description: ::std::option::Option<::std::string::String>,
     /// <p>The function layer archive.</p>
-    pub content: ::std::option::Option<crate::types::LayerVersionContentInput>,
+    pub content: ::std::option::Option<crate::operation::publish_layer_version::Input>,
     /// <p>A list of compatible <a href="https://docs.aws.amazon.com/lambda/latest/dg/foundation-arch.html">instruction set architectures</a>.</p>
     pub compatible_architectures: ::std::option::Option<::std::vec::Vec<crate::types::Architecture>>,
     /// <p>A list of compatible <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html">function runtimes</a>. Used for filtering with <code>ListLayers</code> and <code>ListLayerVersions</code>.</p>
@@ -35,7 +35,7 @@
         self.description.as_deref()
     }
     /// <p>The function layer archive.</p>
-    pub fn content(&self) -> ::std::option::Option<&crate::types::LayerVersionContentInput> {
+    pub fn content(&self) -> ::std::option::Option<&crate::operation::publish_layer_version::Input> {
         self.content.as_ref()
     }
     /// <p>A list of compatible <a href="https://docs.aws.amazon.com/lambda/latest/dg/foundation-arch.html">instruction set architectures</a>.</p>
@@ -77,7 +77,7 @@
 pub struct PublishLayerVersionInputBuilder {
     pub(crate) layer_name: ::std::option::Option<::std::string::String>,
     pub(crate) description: ::std::option::Option<::std::string::String>,
-    pub(crate) content: ::std::option::Option<crate::types::LayerVersionContentInput>,
+    pub(crate) content: ::std::option::Option<crate::operation::publish_layer_version::Input>,
     pub(crate) compatible_architectures: ::std::option::Option<::std::vec::Vec<crate::types::Architecture>>,
     pub(crate) compatible_runtimes: ::std::option::Option<::std::vec::Vec<crate::types::Runtime>>,
     pub(crate) license_info: ::std::option::Option<::std::string::String>,
@@ -114,17 +114,17 @@
     }
     /// <p>The function layer archive.</p>
     /// This field is required.
-    pub fn content(mut self, input: crate::types::LayerVersionContentInput) -> Self {
+    pub fn content(mut self, input: crate::operation::publish_layer_version::Input) -> Self {
         self.content = ::std::option::Option::Some(input);
         self
     }
     /// <p>The function layer archive.</p>
-    pub fn set_content(mut self, input: ::std::option::Option<crate::types::LayerVersionContentInput>) -> Self {
+    pub fn set_content(mut self, input: ::std::option::Option<crate::operation::publish_layer_version::Input>) -> Self {
         self.content = input;
         self
     }
     /// <p>The function layer archive.</p>
-    pub fn get_content(&self) -> &::std::option::Option<crate::types::LayerVersionContentInput> {
+    pub fn get_content(&self) -> &::std::option::Option<crate::operation::publish_layer_version::Input> {
         &self.content
     }
     /// Appends an item to `compatible_architectures`.
```

### `src/operation/publish_layer_version/_publish_layer_version_output.rs`

```diff
--- reference/src/operation/publish_layer_version/_publish_layer_version_output.rs
+++ generated/src/operation/publish_layer_version/_publish_layer_version_output.rs
@@ -4,7 +4,7 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct PublishLayerVersionOutput {
     /// <p>Details about the layer version.</p>
-    pub content: ::std::option::Option<crate::types::LayerVersionContentOutput>,
+    pub content: ::std::option::Option<crate::operation::publish_layer_version::Output>,
     /// <p>The ARN of the layer.</p>
     pub layer_arn: ::std::option::Option<::std::string::String>,
     /// <p>The ARN of the layer version.</p>
@@ -27,7 +27,7 @@
 }
 impl PublishLayerVersionOutput {
     /// <p>Details about the layer version.</p>
-    pub fn content(&self) -> ::std::option::Option<&crate::types::LayerVersionContentOutput> {
+    pub fn content(&self) -> ::std::option::Option<&crate::operation::publish_layer_version::Output> {
         self.content.as_ref()
     }
     /// <p>The ARN of the layer.</p>
@@ -85,7 +85,7 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
 pub struct PublishLayerVersionOutputBuilder {
-    pub(crate) content: ::std::option::Option<crate::types::LayerVersionContentOutput>,
+    pub(crate) content: ::std::option::Option<crate::operation::publish_layer_version::Output>,
     pub(crate) layer_arn: ::std::option::Option<::std::string::String>,
     pub(crate) layer_version_arn: ::std::option::Option<::std::string::String>,
     pub(crate) description: ::std::option::Option<::std::string::String>,
@@ -98,17 +98,17 @@
 }
 impl PublishLayerVersionOutputBuilder {
     /// <p>Details about the layer version.</p>
-    pub fn content(mut self, input: crate::types::LayerVersionContentOutput) -> Self {
+    pub fn content(mut self, input: crate::operation::publish_layer_version::Output) -> Self {
         self.content = ::std::option::Option::Some(input);
         self
     }
     /// <p>Details about the layer version.</p>
-    pub fn set_content(mut self, input: ::std::option::Option<crate::types::LayerVersionContentOutput>) -> Self {
+    pub fn set_content(mut self, input: ::std::option::Option<crate::operation::publish_layer_version::Output>) -> Self {
         self.content = input;
         self
     }
     /// <p>Details about the layer version.</p>
-    pub fn get_content(&self) -> &::std::option::Option<crate::types::LayerVersionContentOutput> {
+    pub fn get_content(&self) -> &::std::option::Option<crate::operation::publish_layer_version::Output> {
         &self.content
     }
     /// <p>The ARN of the layer.</p>
```

### `src/operation/publish_layer_version/builders.rs`

```diff
--- reference/src/operation/publish_layer_version/builders.rs
+++ generated/src/operation/publish_layer_version/builders.rs
@@ -138,17 +138,17 @@
         self.inner.get_description()
     }
     /// <p>The function layer archive.</p>
-    pub fn content(mut self, input: crate::types::LayerVersionContentInput) -> Self {
+    pub fn content(mut self, input: crate::operation::publish_layer_version::Input) -> Self {
         self.inner = self.inner.content(input);
         self
     }
     /// <p>The function layer archive.</p>
-    pub fn set_content(mut self, input: ::std::option::Option<crate::types::LayerVersionContentInput>) -> Self {
+    pub fn set_content(mut self, input: ::std::option::Option<crate::operation::publish_layer_version::Input>) -> Self {
         self.inner = self.inner.set_content(input);
         self
     }
     /// <p>The function layer archive.</p>
-    pub fn get_content(&self) -> &::std::option::Option<crate::types::LayerVersionContentInput> {
+    pub fn get_content(&self) -> &::std::option::Option<crate::operation::publish_layer_version::Input> {
         self.inner.get_content()
     }
     ///
```

### `src/operation/publish_layer_version.rs`

```diff
--- reference/src/operation/publish_layer_version.rs
+++ generated/src/operation/publish_layer_version.rs
@@ -107,9 +107,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -125,25 +125,17 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("PublishLayerVersion")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                PublishLayerVersionTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                PublishLayerVersionEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::publish_layer_version::PublishLayerVersionError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::publish_layer_version::PublishLayerVersionError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::publish_layer_version::PublishLayerVersionError,
-            >::new());
+                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("PublishLayerVersion")
+                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(PublishLayerVersionTelemetryInputCaptureInterceptor))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(PublishLayerVersionEndpointParamsInterceptor))
+                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::publish_layer_version::PublishLayerVersionError>::new())
+.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::publish_layer_version::PublishLayerVersionError>::new())
+.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::publish_layer_version::PublishLayerVersionError>::builder().transient_errors({
+                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                                            transient_errors.push("InternalError");
+                                            ::std::borrow::Cow::Owned(transient_errors)
+                                            }).build());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -271,12 +263,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_publish_layer_version::ser_publish_layer_version_input(
-            &input,
-        )?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            crate::protocol_serde::shape_publish_layer_version_input::ser_publish_layer_version_op_input(&input)?,
+        );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -310,8 +302,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -463,6 +455,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::publish_layer_version::PublishLayerVersionError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::publish_layer_version::PublishLayerVersionError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/publish_version.rs`

```diff
--- reference/src/operation/publish_version.rs
+++ generated/src/operation/publish_version.rs
@@ -105,9 +105,9 @@
         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::SensitiveOutput);
         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("PublishVersion", "Lambda"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -139,9 +139,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::publish_version::PublishVersionError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::publish_version::PublishVersionError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::publish_version::PublishVersionError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -275,10 +281,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_publish_version::ser_publish_version_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_publish_version_input::ser_publish_version_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -312,8 +318,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -503,6 +509,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::publish_version::PublishVersionError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::publish_version::PublishVersionError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/put_function_code_signing_config.rs`

```diff
--- reference/src/operation/put_function_code_signing_config.rs
+++ generated/src/operation/put_function_code_signing_config.rs
@@ -113,9 +113,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -147,9 +147,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::put_function_code_signing_config::PutFunctionCodeSigningConfigError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::put_function_code_signing_config::PutFunctionCodeSigningConfigError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::put_function_code_signing_config::PutFunctionCodeSigningConfigError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -277,11 +285,11 @@
                 ::std::result::Result::Ok(builder.method("PUT").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_put_function_code_signing_config::ser_put_function_code_signing_config_input(&input)?,
+            crate::protocol_serde::shape_put_function_code_signing_config_input::ser_put_function_code_signing_config_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -316,8 +324,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -479,6 +487,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::put_function_code_signing_config::PutFunctionCodeSigningConfigError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::put_function_code_signing_config::PutFunctionCodeSigningConfigError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/put_function_concurrency.rs`

```diff
--- reference/src/operation/put_function_concurrency.rs
+++ generated/src/operation/put_function_concurrency.rs
@@ -107,9 +107,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -125,25 +125,34 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("PutFunctionConcurrency")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                PutFunctionConcurrencyTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                PutFunctionConcurrencyEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::put_function_concurrency::PutFunctionConcurrencyError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::put_function_concurrency::PutFunctionConcurrencyError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::put_function_concurrency::PutFunctionConcurrencyError,
-            >::new());
+        let mut rcb =
+            ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("PutFunctionConcurrency")
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    PutFunctionConcurrencyTelemetryInputCaptureInterceptor,
+                ))
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
+                ))
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    PutFunctionConcurrencyEndpointParamsInterceptor,
+                ))
+                .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+                    crate::operation::put_function_concurrency::PutFunctionConcurrencyError,
+                >::new())
+                .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+                    crate::operation::put_function_concurrency::PutFunctionConcurrencyError,
+                >::new())
+                .with_retry_classifier(
+                    ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                        crate::operation::put_function_concurrency::PutFunctionConcurrencyError,
+                    >::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+                );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -262,11 +271,11 @@
                 ::std::result::Result::Ok(builder.method("PUT").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_put_function_concurrency::ser_put_function_concurrency_input(&input)?,
+            crate::protocol_serde::shape_put_function_concurrency_input::ser_put_function_concurrency_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -301,8 +310,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -454,6 +463,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::put_function_concurrency::PutFunctionConcurrencyError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::put_function_concurrency::PutFunctionConcurrencyError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/put_function_event_invoke_config.rs`

```diff
--- reference/src/operation/put_function_event_invoke_config.rs
+++ generated/src/operation/put_function_event_invoke_config.rs
@@ -113,9 +113,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -147,9 +147,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::put_function_event_invoke_config::PutFunctionEventInvokeConfigError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::put_function_event_invoke_config::PutFunctionEventInvokeConfigError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::put_function_event_invoke_config::PutFunctionEventInvokeConfigError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -290,11 +298,11 @@
                 ::std::result::Result::Ok(builder.method("PUT").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_put_function_event_invoke_config::ser_put_function_event_invoke_config_input(&input)?,
+            crate::protocol_serde::shape_put_function_event_invoke_config_input::ser_put_function_event_invoke_config_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -329,8 +337,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -482,6 +490,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::put_function_event_invoke_config::PutFunctionEventInvokeConfigError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::put_function_event_invoke_config::PutFunctionEventInvokeConfigError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/put_function_recursion_config.rs`

```diff
--- reference/src/operation/put_function_recursion_config.rs
+++ generated/src/operation/put_function_recursion_config.rs
@@ -107,9 +107,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -141,9 +141,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::put_function_recursion_config::PutFunctionRecursionConfigError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::put_function_recursion_config::PutFunctionRecursionConfigError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::put_function_recursion_config::PutFunctionRecursionConfigError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -266,11 +274,11 @@
                 ::std::result::Result::Ok(builder.method("PUT").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_put_function_recursion_config::ser_put_function_recursion_config_input(&input)?,
+            crate::protocol_serde::shape_put_function_recursion_config_input::ser_put_function_recursion_config_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -305,8 +313,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -458,6 +466,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::put_function_recursion_config::PutFunctionRecursionConfigError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::put_function_recursion_config::PutFunctionRecursionConfigError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/put_function_scaling_config.rs`

```diff
--- reference/src/operation/put_function_scaling_config.rs
+++ generated/src/operation/put_function_scaling_config.rs
@@ -107,9 +107,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -141,9 +141,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::put_function_scaling_config::PutFunctionScalingConfigError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::put_function_scaling_config::PutFunctionScalingConfigError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::put_function_scaling_config::PutFunctionScalingConfigError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -290,11 +298,11 @@
                 ::std::result::Result::Ok(builder.method("PUT").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_put_function_scaling_config::ser_put_function_scaling_config_input(&input)?,
+            crate::protocol_serde::shape_put_function_scaling_config_input::ser_put_function_scaling_config_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -329,8 +337,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -482,6 +490,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::put_function_scaling_config::PutFunctionScalingConfigError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::put_function_scaling_config::PutFunctionScalingConfigError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/put_provisioned_concurrency_config.rs`

```diff
--- reference/src/operation/put_provisioned_concurrency_config.rs
+++ generated/src/operation/put_provisioned_concurrency_config.rs
@@ -113,9 +113,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -147,9 +147,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::put_provisioned_concurrency_config::PutProvisionedConcurrencyConfigError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::put_provisioned_concurrency_config::PutProvisionedConcurrencyConfigError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::put_provisioned_concurrency_config::PutProvisionedConcurrencyConfigError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -298,11 +306,11 @@
                 ::std::result::Result::Ok(builder.method("PUT").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_put_provisioned_concurrency_config::ser_put_provisioned_concurrency_config_input(&input)?,
+            crate::protocol_serde::shape_put_provisioned_concurrency_config_input::ser_put_provisioned_concurrency_config_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -337,8 +345,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -490,6 +498,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::put_provisioned_concurrency_config::PutProvisionedConcurrencyConfigError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::put_provisioned_concurrency_config::PutProvisionedConcurrencyConfigError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/put_resource_policy.rs`

```diff
--- reference/src/operation/put_resource_policy.rs
+++ generated/src/operation/put_resource_policy.rs
@@ -107,9 +107,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -125,25 +125,17 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("PutResourcePolicy")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                PutResourcePolicyTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                PutResourcePolicyEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::put_resource_policy::PutResourcePolicyError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::put_resource_policy::PutResourcePolicyError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::put_resource_policy::PutResourcePolicyError,
-            >::new());
+                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("PutResourcePolicy")
+                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(PutResourcePolicyTelemetryInputCaptureInterceptor))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(PutResourcePolicyEndpointParamsInterceptor))
+                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::put_resource_policy::PutResourcePolicyError>::new())
+.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::put_resource_policy::PutResourcePolicyError>::new())
+.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::put_resource_policy::PutResourcePolicyError>::builder().transient_errors({
+                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                                            transient_errors.push("InternalError");
+                                            ::std::borrow::Cow::Owned(transient_errors)
+                                            }).build());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -271,10 +263,12 @@
                 ::std::result::Result::Ok(builder.method("PUT").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_put_resource_policy::ser_put_resource_policy_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_put_resource_policy_input::ser_put_resource_policy_op_input(
+            &input,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -308,8 +302,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -497,6 +491,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::put_resource_policy::PutResourcePolicyError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::put_resource_policy::PutResourcePolicyError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/put_runtime_management_config.rs`

```diff
--- reference/src/operation/put_runtime_management_config.rs
+++ generated/src/operation/put_runtime_management_config.rs
@@ -107,9 +107,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -141,9 +141,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::put_runtime_management_config::PutRuntimeManagementConfigError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::put_runtime_management_config::PutRuntimeManagementConfigError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::put_runtime_management_config::PutRuntimeManagementConfigError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -289,11 +297,11 @@
                 ::std::result::Result::Ok(builder.method("PUT").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_put_runtime_management_config::ser_put_runtime_management_config_input(&input)?,
+            crate::protocol_serde::shape_put_runtime_management_config_input::ser_put_runtime_management_config_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -328,8 +336,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -481,6 +489,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::put_runtime_management_config::PutRuntimeManagementConfigError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::put_runtime_management_config::PutRuntimeManagementConfigError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/remove_layer_version_permission/_remove_layer_version_permission_input.rs`

```diff
--- reference/src/operation/remove_layer_version_permission/_remove_layer_version_permission_input.rs
+++ generated/src/operation/remove_layer_version_permission/_remove_layer_version_permission_input.rs
@@ -115,7 +115,7 @@
     > {
         ::std::result::Result::Ok(crate::operation::remove_layer_version_permission::RemoveLayerVersionPermissionInput {
             layer_name: self.layer_name,
-            version_number: self.version_number,
+            version_number: self.version_number.unwrap_or_default(),
             statement_id: self.statement_id,
             revision_id: self.revision_id,
         })
```

### `src/operation/remove_layer_version_permission.rs`

```diff
--- reference/src/operation/remove_layer_version_permission.rs
+++ generated/src/operation/remove_layer_version_permission.rs
@@ -113,9 +113,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -147,9 +147,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::remove_layer_version_permission::RemoveLayerVersionPermissionError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::remove_layer_version_permission::RemoveLayerVersionPermissionError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::remove_layer_version_permission::RemoveLayerVersionPermissionError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -268,8 +276,7 @@
                 let input_2 = input_2
                     .as_ref()
                     .ok_or_else(|| ::aws_smithy_types::error::operation::BuildError::missing_field("version_number", "cannot be empty or unset"))?;
-                let mut version_number_encoder = ::aws_smithy_types::primitive::Encoder::from(*input_2);
-                let version_number = version_number_encoder.encode();
+                let version_number = ::aws_smithy_http::label::fmt_string(input_2, ::aws_smithy_http::label::EncodingStrategy::Default);
                 if version_number.is_empty() {
                     return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
                         "version_number",
@@ -353,8 +360,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -512,6 +519,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::remove_layer_version_permission::RemoveLayerVersionPermissionError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::remove_layer_version_permission::RemoveLayerVersionPermissionError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/remove_permission.rs`

```diff
--- reference/src/operation/remove_permission.rs
+++ generated/src/operation/remove_permission.rs
@@ -107,9 +107,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -141,9 +141,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::remove_permission::RemovePermissionError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::remove_permission::RemovePermissionError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::remove_permission::RemovePermissionError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -344,8 +350,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -513,6 +519,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::remove_permission::RemovePermissionError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::remove_permission::RemovePermissionError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/send_durable_execution_callback_failure.rs`

```diff
--- reference/src/operation/send_durable_execution_callback_failure.rs
+++ generated/src/operation/send_durable_execution_callback_failure.rs
@@ -113,9 +113,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -147,9 +147,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::send_durable_execution_callback_failure::SendDurableExecutionCallbackFailureError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::send_durable_execution_callback_failure::SendDurableExecutionCallbackFailureError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::send_durable_execution_callback_failure::SendDurableExecutionCallbackFailureError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -276,7 +284,7 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
@@ -315,8 +323,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -508,6 +516,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::send_durable_execution_callback_failure::SendDurableExecutionCallbackFailureError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::send_durable_execution_callback_failure::SendDurableExecutionCallbackFailureError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/send_durable_execution_callback_heartbeat.rs`

```diff
--- reference/src/operation/send_durable_execution_callback_heartbeat.rs
+++ generated/src/operation/send_durable_execution_callback_heartbeat.rs
@@ -113,9 +113,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -147,9 +147,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::send_durable_execution_callback_heartbeat::SendDurableExecutionCallbackHeartbeatError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::send_durable_execution_callback_heartbeat::SendDurableExecutionCallbackHeartbeatError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::send_durable_execution_callback_heartbeat::SendDurableExecutionCallbackHeartbeatError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -309,8 +317,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -462,6 +470,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::send_durable_execution_callback_heartbeat::SendDurableExecutionCallbackHeartbeatError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::send_durable_execution_callback_heartbeat::SendDurableExecutionCallbackHeartbeatError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/send_durable_execution_callback_success.rs`

```diff
--- reference/src/operation/send_durable_execution_callback_success.rs
+++ generated/src/operation/send_durable_execution_callback_success.rs
@@ -113,9 +113,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -147,9 +147,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::send_durable_execution_callback_success::SendDurableExecutionCallbackSuccessError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::send_durable_execution_callback_success::SendDurableExecutionCallbackSuccessError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::send_durable_execution_callback_success::SendDurableExecutionCallbackSuccessError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -315,8 +323,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -508,6 +516,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::send_durable_execution_callback_success::SendDurableExecutionCallbackSuccessError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::send_durable_execution_callback_success::SendDurableExecutionCallbackSuccessError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/stop_durable_execution.rs`

```diff
--- reference/src/operation/stop_durable_execution.rs
+++ generated/src/operation/stop_durable_execution.rs
@@ -107,9 +107,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -125,25 +125,17 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("StopDurableExecution")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                StopDurableExecutionTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                StopDurableExecutionEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::stop_durable_execution::StopDurableExecutionError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::stop_durable_execution::StopDurableExecutionError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::stop_durable_execution::StopDurableExecutionError,
-            >::new());
+                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("StopDurableExecution")
+                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(StopDurableExecutionTelemetryInputCaptureInterceptor))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(StopDurableExecutionEndpointParamsInterceptor))
+                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::stop_durable_execution::StopDurableExecutionError>::new())
+.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::stop_durable_execution::StopDurableExecutionError>::new())
+.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::stop_durable_execution::StopDurableExecutionError>::builder().transient_errors({
+                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                                            transient_errors.push("InternalError");
+                                            ::std::borrow::Cow::Owned(transient_errors)
+                                            }).build());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -266,7 +258,7 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_stop_durable_execution_input::ser_error_http_payload(
@@ -305,8 +297,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
+            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -488,6 +480,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::stop_durable_execution::StopDurableExecutionError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::stop_durable_execution::StopDurableExecutionError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/tag_resource.rs`

```diff
--- reference/src/operation/tag_resource.rs
+++ generated/src/operation/tag_resource.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("TagResource", "Lambda"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -138,9 +138,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::tag_resource::TagResourceError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::tag_resource::TagResourceError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::tag_resource::TagResourceError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -258,10 +264,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_tag_resource::ser_tag_resource_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_tag_resource_input::ser_tag_resource_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -295,8 +301,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -448,6 +454,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::tag_resource::TagResourceError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::tag_resource::TagResourceError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/untag_resource.rs`

```diff
--- reference/src/operation/untag_resource.rs
+++ generated/src/operation/untag_resource.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("UntagResource", "Lambda"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -138,9 +138,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::untag_resource::UntagResourceError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::untag_resource::UntagResourceError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::untag_resource::UntagResourceError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -257,9 +263,7 @@
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
@@ -306,8 +310,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -459,6 +463,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::untag_resource::UntagResourceError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::untag_resource::UntagResourceError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/update_alias.rs`

```diff
--- reference/src/operation/update_alias.rs
+++ generated/src/operation/update_alias.rs
@@ -104,9 +104,9 @@

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("UpdateAlias", "Lambda"));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -138,9 +138,15 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::update_alias::UpdateAliasError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::update_alias::UpdateAliasError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::update_alias::UpdateAliasError>::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -295,10 +301,10 @@
                 ::std::result::Result::Ok(builder.method("PUT").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_update_alias::ser_update_alias_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_update_alias_input::ser_update_alias_op_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -332,8 +338,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -501,6 +507,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::update_alias::UpdateAliasError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::update_alias::UpdateAliasError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/update_capacity_provider.rs`

```diff
--- reference/src/operation/update_capacity_provider.rs
+++ generated/src/operation/update_capacity_provider.rs
@@ -107,9 +107,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -125,25 +125,34 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("UpdateCapacityProvider")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                UpdateCapacityProviderTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                UpdateCapacityProviderEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::update_capacity_provider::UpdateCapacityProviderError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::update_capacity_provider::UpdateCapacityProviderError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::update_capacity_provider::UpdateCapacityProviderError,
-            >::new());
+        let mut rcb =
+            ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("UpdateCapacityProvider")
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    UpdateCapacityProviderTelemetryInputCaptureInterceptor,
+                ))
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
+                ))
+                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                    UpdateCapacityProviderEndpointParamsInterceptor,
+                ))
+                .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+                    crate::operation::update_capacity_provider::UpdateCapacityProviderError,
+                >::new())
+                .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+                    crate::operation::update_capacity_provider::UpdateCapacityProviderError,
+                >::new())
+                .with_retry_classifier(
+                    ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                        crate::operation::update_capacity_provider::UpdateCapacityProviderError,
+                    >::builder()
+                    .transient_errors({
+                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                        transient_errors.push("InternalError");
+                        ::std::borrow::Cow::Owned(transient_errors)
+                    })
+                    .build(),
+                );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -266,11 +275,11 @@
                 ::std::result::Result::Ok(builder.method("PUT").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_update_capacity_provider::ser_update_capacity_provider_input(&input)?,
+            crate::protocol_serde::shape_update_capacity_provider_input::ser_update_capacity_provider_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -305,8 +314,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -458,6 +467,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::update_capacity_provider::UpdateCapacityProviderError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::update_capacity_provider::UpdateCapacityProviderError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/update_code_signing_config.rs`

```diff
--- reference/src/operation/update_code_signing_config.rs
+++ generated/src/operation/update_code_signing_config.rs
@@ -107,9 +107,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -141,9 +141,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::update_code_signing_config::UpdateCodeSigningConfigError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::update_code_signing_config::UpdateCodeSigningConfigError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::update_code_signing_config::UpdateCodeSigningConfigError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -271,11 +279,11 @@
                 ::std::result::Result::Ok(builder.method("PUT").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_update_code_signing_config::ser_update_code_signing_config_input(&input)?,
+            crate::protocol_serde::shape_update_code_signing_config_input::ser_update_code_signing_config_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -310,8 +318,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -443,6 +451,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::update_code_signing_config::UpdateCodeSigningConfigError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::update_code_signing_config::UpdateCodeSigningConfigError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/update_event_source_mapping.rs`

```diff
--- reference/src/operation/update_event_source_mapping.rs
+++ generated/src/operation/update_event_source_mapping.rs
@@ -107,9 +107,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -141,9 +141,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::update_event_source_mapping::UpdateEventSourceMappingError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::update_event_source_mapping::UpdateEventSourceMappingError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::update_event_source_mapping::UpdateEventSourceMappingError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -271,11 +279,11 @@
                 ::std::result::Result::Ok(builder.method("PUT").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_update_event_source_mapping::ser_update_event_source_mapping_input(&input)?,
+            crate::protocol_serde::shape_update_event_source_mapping_input::ser_update_event_source_mapping_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -310,8 +318,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -473,6 +481,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::update_event_source_mapping::UpdateEventSourceMappingError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::update_event_source_mapping::UpdateEventSourceMappingError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/update_function_code/_update_function_code_input.rs`

```diff
--- reference/src/operation/update_function_code/_update_function_code_input.rs
+++ generated/src/operation/update_function_code/_update_function_code_input.rs
@@ -414,9 +414,9 @@
             s3_object_storage_mode: self.s3_object_storage_mode,
             image_uri: self.image_uri,
             architectures: self.architectures,
-            publish: self.publish,
+            publish: self.publish.unwrap_or_default(),
             publish_to: self.publish_to,
-            dry_run: self.dry_run,
+            dry_run: self.dry_run.unwrap_or_default(),
             revision_id: self.revision_id,
             source_kms_key_arn: self.source_kms_key_arn,
         })
```

### `src/operation/update_function_code.rs`

```diff
--- reference/src/operation/update_function_code.rs
+++ generated/src/operation/update_function_code.rs
@@ -108,9 +108,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -126,25 +126,17 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("UpdateFunctionCode")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                UpdateFunctionCodeTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                UpdateFunctionCodeEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::update_function_code::UpdateFunctionCodeError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
-                crate::operation::update_function_code::UpdateFunctionCodeError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::update_function_code::UpdateFunctionCodeError,
-            >::new());
+                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("UpdateFunctionCode")
+                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(UpdateFunctionCodeTelemetryInputCaptureInterceptor))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
+.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(UpdateFunctionCodeEndpointParamsInterceptor))
+                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::update_function_code::UpdateFunctionCodeError>::new())
+.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::update_function_code::UpdateFunctionCodeError>::new())
+.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::update_function_code::UpdateFunctionCodeError>::builder().transient_errors({
+                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                                            transient_errors.push("InternalError");
+                                            ::std::borrow::Cow::Owned(transient_errors)
+                                            }).build());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -292,11 +284,12 @@
                 ::std::result::Result::Ok(builder.method("PUT").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
-        let body =
-            ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_update_function_code::ser_update_function_code_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            crate::protocol_serde::shape_update_function_code_input::ser_update_function_code_op_input(&input)?,
+        );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
@@ -330,8 +323,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -539,6 +532,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::update_function_code::UpdateFunctionCodeError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::update_function_code::UpdateFunctionCodeError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/update_function_configuration.rs`

```diff
--- reference/src/operation/update_function_configuration.rs
+++ generated/src/operation/update_function_configuration.rs
@@ -114,9 +114,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -148,9 +148,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::update_function_configuration::UpdateFunctionConfigurationError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::update_function_configuration::UpdateFunctionConfigurationError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::update_function_configuration::UpdateFunctionConfigurationError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -294,11 +302,11 @@
                 ::std::result::Result::Ok(builder.method("PUT").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_update_function_configuration::ser_update_function_configuration_input(&input)?,
+            crate::protocol_serde::shape_update_function_configuration_input::ser_update_function_configuration_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -333,8 +341,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -532,6 +540,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::update_function_configuration::UpdateFunctionConfigurationError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::update_function_configuration::UpdateFunctionConfigurationError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/update_function_event_invoke_config.rs`

```diff
--- reference/src/operation/update_function_event_invoke_config.rs
+++ generated/src/operation/update_function_event_invoke_config.rs
@@ -113,9 +113,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -147,9 +147,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::update_function_event_invoke_config::UpdateFunctionEventInvokeConfigError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::update_function_event_invoke_config::UpdateFunctionEventInvokeConfigError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::update_function_event_invoke_config::UpdateFunctionEventInvokeConfigError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -292,11 +300,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_update_function_event_invoke_config::ser_update_function_event_invoke_config_input(&input)?,
+            crate::protocol_serde::shape_update_function_event_invoke_config_input::ser_update_function_event_invoke_config_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -331,8 +339,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -484,6 +492,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::update_function_event_invoke_config::UpdateFunctionEventInvokeConfigError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::update_function_event_invoke_config::UpdateFunctionEventInvokeConfigError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/operation/update_function_url_config.rs`

```diff
--- reference/src/operation/update_function_url_config.rs
+++ generated/src/operation/update_function_url_config.rs
@@ -107,9 +107,9 @@
             "Lambda",
         ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
-        signing_options.double_uri_encode = true;
-        signing_options.content_sha256_header = false;
-        signing_options.normalize_uri_path = true;
+        signing_options.double_uri_encode = false;
+        signing_options.content_sha256_header = true;
+        signing_options.normalize_uri_path = false;
         signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
@@ -141,9 +141,17 @@
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::update_function_url_config::UpdateFunctionUrlConfigError,
             >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::update_function_url_config::UpdateFunctionUrlConfigError,
-            >::new());
+            .with_retry_classifier(
+                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                    crate::operation::update_function_url_config::UpdateFunctionUrlConfigError,
+                >::builder()
+                .transient_errors({
+                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                    transient_errors.push("InternalError");
+                    ::std::borrow::Cow::Owned(transient_errors)
+                })
+                .build(),
+            );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -279,11 +287,11 @@
                 ::std::result::Result::Ok(builder.method("PUT").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/xml");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_update_function_url_config::ser_update_function_url_config_input(&input)?,
+            crate::protocol_serde::shape_update_function_url_config_input::ser_update_function_url_config_op_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
@@ -318,8 +326,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -471,6 +479,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::operation::update_function_url_config::UpdateFunctionUrlConfigError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::operation::update_function_url_config::UpdateFunctionUrlConfigError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/serde_util.rs`

```diff
--- reference/src/serde_util.rs
+++ generated/src/serde_util.rs
@@ -295,6 +295,27 @@
     builder
 }

+pub(crate) fn capacity_provider_vpc_config_correct_errors(
+    mut builder: crate::types::builders::CapacityProviderVpcConfigBuilder,
+) -> crate::types::builders::CapacityProviderVpcConfigBuilder {
+    if builder.subnet_ids.is_none() {
+        builder.subnet_ids = Some(Default::default())
+    }
+    if builder.security_group_ids.is_none() {
+        builder.security_group_ids = Some(Default::default())
+    }
+    builder
+}
+
+pub(crate) fn capacity_provider_permissions_config_correct_errors(
+    mut builder: crate::types::builders::CapacityProviderPermissionsConfigBuilder,
+) -> crate::types::builders::CapacityProviderPermissionsConfigBuilder {
+    if builder.capacity_provider_operator_role_arn.is_none() {
+        builder.capacity_provider_operator_role_arn = Some(Default::default())
+    }
+    builder
+}
+
 pub(crate) fn code_signing_config_correct_errors(
     mut builder: crate::types::builders::CodeSigningConfigBuilder,
 ) -> crate::types::builders::CodeSigningConfigBuilder {
@@ -322,6 +343,15 @@
     builder
 }

+pub(crate) fn allowed_publishers_correct_errors(
+    mut builder: crate::types::builders::AllowedPublishersBuilder,
+) -> crate::types::builders::AllowedPublishersBuilder {
+    if builder.signing_profile_version_arns.is_none() {
+        builder.signing_profile_version_arns = Some(Default::default())
+    }
+    builder
+}
+
 pub(crate) fn capacity_provider_config_correct_errors(
     mut builder: crate::types::builders::CapacityProviderConfigBuilder,
 ) -> crate::types::builders::CapacityProviderConfigBuilder {
@@ -364,36 +394,6 @@
     builder
 }

-pub(crate) fn capacity_provider_vpc_config_correct_errors(
-    mut builder: crate::types::builders::CapacityProviderVpcConfigBuilder,
-) -> crate::types::builders::CapacityProviderVpcConfigBuilder {
-    if builder.subnet_ids.is_none() {
-        builder.subnet_ids = Some(Default::default())
-    }
-    if builder.security_group_ids.is_none() {
-        builder.security_group_ids = Some(Default::default())
-    }
-    builder
-}
-
-pub(crate) fn capacity_provider_permissions_config_correct_errors(
-    mut builder: crate::types::builders::CapacityProviderPermissionsConfigBuilder,
-) -> crate::types::builders::CapacityProviderPermissionsConfigBuilder {
-    if builder.capacity_provider_operator_role_arn.is_none() {
-        builder.capacity_provider_operator_role_arn = Some(Default::default())
-    }
-    builder
-}
-
-pub(crate) fn allowed_publishers_correct_errors(
-    mut builder: crate::types::builders::AllowedPublishersBuilder,
-) -> crate::types::builders::AllowedPublishersBuilder {
-    if builder.signing_profile_version_arns.is_none() {
-        builder.signing_profile_version_arns = Some(Default::default())
-    }
-    builder
-}
-
 pub(crate) fn execution_correct_errors(mut builder: crate::types::builders::ExecutionBuilder) -> crate::types::builders::ExecutionBuilder {
     if builder.durable_execution_arn.is_none() {
         builder.durable_execution_arn = Some(Default::default())
@@ -687,7 +687,7 @@
     if builder.retry_details.is_none() {
         builder.retry_details = {
             let builder = crate::types::builders::RetryDetailsBuilder::default();
-            Some(builder.build())
+            builder.build().ok()
         }
     }
     builder
@@ -705,7 +705,7 @@
     if builder.retry_details.is_none() {
         builder.retry_details = {
             let builder = crate::types::builders::RetryDetailsBuilder::default();
-            Some(builder.build())
+            builder.build().ok()
         }
     }
     builder
```

### `src/types/_capacity_provider_logging_config.rs`

```diff
--- reference/src/types/_capacity_provider_logging_config.rs
+++ generated/src/types/_capacity_provider_logging_config.rs
@@ -6,7 +6,7 @@
 pub struct CapacityProviderLoggingConfig {
     /// <p>Set this property to filter the system logs for your capacity provider that Lambda sends to CloudWatch. Lambda only sends system logs at the selected level of detail and lower, where <code>DEBUG</code> is the highest level and <code>WARN</code> is the lowest.</p>
     pub system_log_level: ::std::option::Option<crate::types::SystemLogLevel>,
-    /// <p>The name of the Amazon CloudWatch log group the capacity provider sends logs to. By default, Lambda capacity providers send logs to a default log group named <code>/aws/lambda/capacity-provider/&lt;capacity provider name&gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
+    /// <p>The name of the Amazon CloudWatch log group the capacity provider sends logs to. By default, Lambda capacity providers send logs to a default log group named <code>/aws/lambda/capacity-provider/&amp;lt;capacity provider name&amp;gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
     pub log_group: ::std::option::Option<::std::string::String>,
 }
 impl CapacityProviderLoggingConfig {
@@ -14,7 +14,7 @@
     pub fn system_log_level(&self) -> ::std::option::Option<&crate::types::SystemLogLevel> {
         self.system_log_level.as_ref()
     }
-    /// <p>The name of the Amazon CloudWatch log group the capacity provider sends logs to. By default, Lambda capacity providers send logs to a default log group named <code>/aws/lambda/capacity-provider/&lt;capacity provider name&gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
+    /// <p>The name of the Amazon CloudWatch log group the capacity provider sends logs to. By default, Lambda capacity providers send logs to a default log group named <code>/aws/lambda/capacity-provider/&amp;lt;capacity provider name&amp;gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
     pub fn log_group(&self) -> ::std::option::Option<&str> {
         self.log_group.as_deref()
     }
@@ -48,17 +48,17 @@
     pub fn get_system_log_level(&self) -> &::std::option::Option<crate::types::SystemLogLevel> {
         &self.system_log_level
     }
-    /// <p>The name of the Amazon CloudWatch log group the capacity provider sends logs to. By default, Lambda capacity providers send logs to a default log group named <code>/aws/lambda/capacity-provider/&lt;capacity provider name&gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
+    /// <p>The name of the Amazon CloudWatch log group the capacity provider sends logs to. By default, Lambda capacity providers send logs to a default log group named <code>/aws/lambda/capacity-provider/&amp;lt;capacity provider name&amp;gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
     pub fn log_group(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
         self.log_group = ::std::option::Option::Some(input.into());
         self
     }
-    /// <p>The name of the Amazon CloudWatch log group the capacity provider sends logs to. By default, Lambda capacity providers send logs to a default log group named <code>/aws/lambda/capacity-provider/&lt;capacity provider name&gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
+    /// <p>The name of the Amazon CloudWatch log group the capacity provider sends logs to. By default, Lambda capacity providers send logs to a default log group named <code>/aws/lambda/capacity-provider/&amp;lt;capacity provider name&amp;gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
     pub fn set_log_group(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
         self.log_group = input;
         self
     }
-    /// <p>The name of the Amazon CloudWatch log group the capacity provider sends logs to. By default, Lambda capacity providers send logs to a default log group named <code>/aws/lambda/capacity-provider/&lt;capacity provider name&gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
+    /// <p>The name of the Amazon CloudWatch log group the capacity provider sends logs to. By default, Lambda capacity providers send logs to a default log group named <code>/aws/lambda/capacity-provider/&amp;lt;capacity provider name&amp;gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
     pub fn get_log_group(&self) -> &::std::option::Option<::std::string::String> {
         &self.log_group
     }
```

### `src/types/_event.rs`

```diff
--- reference/src/types/_event.rs
+++ generated/src/types/_event.rs
@@ -676,7 +676,7 @@
         crate::types::Event {
             event_type: self.event_type,
             sub_type: self.sub_type,
-            event_id: self.event_id.unwrap_or(1),
+            event_id: self.event_id.unwrap_or_default(),
             id: self.id,
             name: self.name,
             event_timestamp: self.event_timestamp,
```

### `src/types/_lambda_managed_instances_capacity_provider_config.rs`

```diff
--- reference/src/types/_lambda_managed_instances_capacity_provider_config.rs
+++ generated/src/types/_lambda_managed_instances_capacity_provider_config.rs
@@ -9,7 +9,7 @@
     /// <p>The maximum number of concurrent execution environments that can run on each compute instance.</p>
     pub per_execution_environment_max_concurrency: ::std::option::Option<i32>,
     /// <p>The amount of memory in GiB allocated per vCPU for execution environments.</p>
-    pub execution_environment_memory_gib_per_v_cpu: ::std::option::Option<f64>,
+    pub execution_environment_memory_gi_b_per_v_cpu: ::std::option::Option<f64>,
 }
 impl LambdaManagedInstancesCapacityProviderConfig {
     /// <p>The Amazon Resource Name (ARN) of the capacity provider.</p>
@@ -22,8 +22,8 @@
         self.per_execution_environment_max_concurrency
     }
     /// <p>The amount of memory in GiB allocated per vCPU for execution environments.</p>
-    pub fn execution_environment_memory_gib_per_v_cpu(&self) -> ::std::option::Option<f64> {
-        self.execution_environment_memory_gib_per_v_cpu
+    pub fn execution_environment_memory_gi_b_per_v_cpu(&self) -> ::std::option::Option<f64> {
+        self.execution_environment_memory_gi_b_per_v_cpu
     }
 }
 impl LambdaManagedInstancesCapacityProviderConfig {
@@ -39,7 +39,7 @@
 pub struct LambdaManagedInstancesCapacityProviderConfigBuilder {
     pub(crate) capacity_provider_arn: ::std::option::Option<::std::string::String>,
     pub(crate) per_execution_environment_max_concurrency: ::std::option::Option<i32>,
-    pub(crate) execution_environment_memory_gib_per_v_cpu: ::std::option::Option<f64>,
+    pub(crate) execution_environment_memory_gi_b_per_v_cpu: ::std::option::Option<f64>,
 }
 impl LambdaManagedInstancesCapacityProviderConfigBuilder {
     /// <p>The Amazon Resource Name (ARN) of the capacity provider.</p>
@@ -72,18 +72,18 @@
         &self.per_execution_environment_max_concurrency
     }
     /// <p>The amount of memory in GiB allocated per vCPU for execution environments.</p>
-    pub fn execution_environment_memory_gib_per_v_cpu(mut self, input: f64) -> Self {
-        self.execution_environment_memory_gib_per_v_cpu = ::std::option::Option::Some(input);
+    pub fn execution_environment_memory_gi_b_per_v_cpu(mut self, input: f64) -> Self {
+        self.execution_environment_memory_gi_b_per_v_cpu = ::std::option::Option::Some(input);
         self
     }
     /// <p>The amount of memory in GiB allocated per vCPU for execution environments.</p>
-    pub fn set_execution_environment_memory_gib_per_v_cpu(mut self, input: ::std::option::Option<f64>) -> Self {
-        self.execution_environment_memory_gib_per_v_cpu = input;
+    pub fn set_execution_environment_memory_gi_b_per_v_cpu(mut self, input: ::std::option::Option<f64>) -> Self {
+        self.execution_environment_memory_gi_b_per_v_cpu = input;
         self
     }
     /// <p>The amount of memory in GiB allocated per vCPU for execution environments.</p>
-    pub fn get_execution_environment_memory_gib_per_v_cpu(&self) -> &::std::option::Option<f64> {
-        &self.execution_environment_memory_gib_per_v_cpu
+    pub fn get_execution_environment_memory_gi_b_per_v_cpu(&self) -> &::std::option::Option<f64> {
+        &self.execution_environment_memory_gi_b_per_v_cpu
     }
     /// Consumes the builder and constructs a [`LambdaManagedInstancesCapacityProviderConfig`](crate::types::LambdaManagedInstancesCapacityProviderConfig).
     /// This method will fail if any of the following fields are not set:
@@ -99,7 +99,7 @@
                 )
             })?,
             per_execution_environment_max_concurrency: self.per_execution_environment_max_concurrency,
-            execution_environment_memory_gib_per_v_cpu: self.execution_environment_memory_gib_per_v_cpu,
+            execution_environment_memory_gi_b_per_v_cpu: self.execution_environment_memory_gi_b_per_v_cpu,
         })
     }
 }
```

### `src/types/_logging_config.rs`

```diff
--- reference/src/types/_logging_config.rs
+++ generated/src/types/_logging_config.rs
@@ -10,7 +10,7 @@
     pub application_log_level: ::std::option::Option<crate::types::ApplicationLogLevel>,
     /// <p>Set this property to filter the system logs for your function that Lambda sends to CloudWatch. Lambda only sends system logs at the selected level of detail and lower, where <code>DEBUG</code> is the highest level and <code>WARN</code> is the lowest.</p>
     pub system_log_level: ::std::option::Option<crate::types::SystemLogLevel>,
-    /// <p>The name of the Amazon CloudWatch log group the function sends logs to. By default, Lambda functions send logs to a default log group named <code>/aws/lambda/&lt;function name&gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
+    /// <p>The name of the Amazon CloudWatch log group the function sends logs to. By default, Lambda functions send logs to a default log group named <code>/aws/lambda/&amp;lt;function name&amp;gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
     pub log_group: ::std::option::Option<::std::string::String>,
 }
 impl LoggingConfig {
@@ -26,7 +26,7 @@
     pub fn system_log_level(&self) -> ::std::option::Option<&crate::types::SystemLogLevel> {
         self.system_log_level.as_ref()
     }
-    /// <p>The name of the Amazon CloudWatch log group the function sends logs to. By default, Lambda functions send logs to a default log group named <code>/aws/lambda/&lt;function name&gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
+    /// <p>The name of the Amazon CloudWatch log group the function sends logs to. By default, Lambda functions send logs to a default log group named <code>/aws/lambda/&amp;lt;function name&amp;gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
     pub fn log_group(&self) -> ::std::option::Option<&str> {
         self.log_group.as_deref()
     }
@@ -90,17 +90,17 @@
     pub fn get_system_log_level(&self) -> &::std::option::Option<crate::types::SystemLogLevel> {
         &self.system_log_level
     }
-    /// <p>The name of the Amazon CloudWatch log group the function sends logs to. By default, Lambda functions send logs to a default log group named <code>/aws/lambda/&lt;function name&gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
+    /// <p>The name of the Amazon CloudWatch log group the function sends logs to. By default, Lambda functions send logs to a default log group named <code>/aws/lambda/&amp;lt;function name&amp;gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
     pub fn log_group(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
         self.log_group = ::std::option::Option::Some(input.into());
         self
     }
-    /// <p>The name of the Amazon CloudWatch log group the function sends logs to. By default, Lambda functions send logs to a default log group named <code>/aws/lambda/&lt;function name&gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
+    /// <p>The name of the Amazon CloudWatch log group the function sends logs to. By default, Lambda functions send logs to a default log group named <code>/aws/lambda/&amp;lt;function name&amp;gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
     pub fn set_log_group(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
         self.log_group = input;
         self
     }
-    /// <p>The name of the Amazon CloudWatch log group the function sends logs to. By default, Lambda functions send logs to a default log group named <code>/aws/lambda/&lt;function name&gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
+    /// <p>The name of the Amazon CloudWatch log group the function sends logs to. By default, Lambda functions send logs to a default log group named <code>/aws/lambda/&amp;lt;function name&amp;gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
     pub fn get_log_group(&self) -> &::std::option::Option<::std::string::String> {
         &self.log_group
     }
```

### `src/types/_operation_update.rs`

```diff
--- reference/src/types/_operation_update.rs
+++ generated/src/types/_operation_update.rs
@@ -92,7 +92,7 @@
         formatter.field("id", &self.id);
         formatter.field("parent_id", &self.parent_id);
         formatter.field("name", &self.name);
-        formatter.field("r#type", &self.r#type);
+        formatter.field("type", &self.r#type);
         formatter.field("sub_type", &self.sub_type);
         formatter.field("action", &self.action);
         formatter.field("payload", &"*** Sensitive Data Redacted ***");
@@ -360,7 +360,7 @@
         formatter.field("id", &self.id);
         formatter.field("parent_id", &self.parent_id);
         formatter.field("name", &self.name);
-        formatter.field("r#type", &self.r#type);
+        formatter.field("type", &self.r#type);
         formatter.field("sub_type", &self.sub_type);
         formatter.field("action", &self.action);
         formatter.field("payload", &"*** Sensitive Data Redacted ***");
```

### `src/types/_propagate_tags_mode.rs`

```diff
--- reference/src/types/_propagate_tags_mode.rs
+++ generated/src/types/_propagate_tags_mode.rs
@@ -42,9 +42,7 @@
     ::std::clone::Clone, ::std::cmp::Eq, ::std::cmp::Ord, ::std::cmp::PartialEq, ::std::cmp::PartialOrd, ::std::fmt::Debug, ::std::hash::Hash,
 )]
 pub enum PropagateTagsMode {
-    /// <p>Tags specified in <code>ExplicitTags</code> are applied to managed resources at launch.</p>
     Explicit,
-    /// <p>Tag propagation is disabled. No tags are applied to managed resources.</p>
     None,
     /// `Unknown` contains new variants that have been added since this code was generated.
     #[deprecated(note = "Don't directly match on `Unknown`. See the docs on this enum for the correct way to handle unknown variants.")]
```

### `src/types/_s3_object_storage_mode.rs`

```diff
--- reference/src/types/_s3_object_storage_mode.rs
+++ generated/src/types/_s3_object_storage_mode.rs
@@ -42,9 +42,7 @@
     ::std::clone::Clone, ::std::cmp::Eq, ::std::cmp::Ord, ::std::cmp::PartialEq, ::std::cmp::PartialOrd, ::std::fmt::Debug, ::std::hash::Hash,
 )]
 pub enum S3ObjectStorageMode {
-    /// <p>The default storage mode. Uploads a copy of your deployment package to Lambda.</p>
     Copy,
-    /// <p>The reference storage mode. Lambda references the deployment package from the specified Amazon S3 bucket without uploading a copy.</p>
     Reference,
     /// `Unknown` contains new variants that have been added since this code was generated.
     #[deprecated(note = "Don't directly match on `Unknown`. See the docs on this enum for the correct way to handle unknown variants.")]
```

### `src/types/builders.rs`

```diff
--- reference/src/types/builders.rs
+++ generated/src/types/builders.rs
@@ -7,6 +7,10 @@

 pub use crate::types::_error_object::ErrorObjectBuilder;

+pub use crate::types::_checkpoint_updated_execution_state::CheckpointUpdatedExecutionStateBuilder;
+
+pub use crate::types::_alias_routing_configuration::AliasRoutingConfigurationBuilder;
+
 pub use crate::types::_capacity_provider_vpc_config::CapacityProviderVpcConfigBuilder;

 pub use crate::types::_capacity_provider_permissions_config::CapacityProviderPermissionsConfigBuilder;
@@ -27,12 +31,6 @@

 pub use crate::types::_code_signing_config::CodeSigningConfigBuilder;

-pub use crate::types::_trace_header::TraceHeaderBuilder;
-
-pub use crate::types::_durable_config::DurableConfigBuilder;
-
-pub use crate::types::_checkpoint_updated_execution_state::CheckpointUpdatedExecutionStateBuilder;
-
 pub use crate::types::_filter_criteria::FilterCriteriaBuilder;

 pub use crate::types::_event_source_mapping_metrics_config::EventSourceMappingMetricsConfigBuilder;
@@ -75,6 +73,8 @@

 pub use crate::types::_capacity_provider_config::CapacityProviderConfigBuilder;

+pub use crate::types::_durable_config::DurableConfigBuilder;
+
 pub use crate::types::_vpc_config_response::VpcConfigResponseBuilder;

 pub use crate::types::_environment_response::EnvironmentResponseBuilder;
@@ -89,6 +89,8 @@

 pub use crate::types::_cors::CorsBuilder;

+pub use crate::types::_trace_header::TraceHeaderBuilder;
+
 pub use crate::types::_function_configuration::FunctionConfigurationBuilder;

 pub use crate::types::_function_code_location::FunctionCodeLocationBuilder;
@@ -99,8 +101,6 @@

 pub use crate::types::_function_scaling_config::FunctionScalingConfigBuilder;

-pub use crate::types::_alias_routing_configuration::AliasRoutingConfigurationBuilder;
-
 pub use crate::types::_layer_version_content_output::LayerVersionContentOutputBuilder;

 pub use crate::types::_layer_version_content_input::LayerVersionContentInputBuilder;
@@ -111,17 +111,9 @@

 pub use crate::types::_function_event_invoke_config::FunctionEventInvokeConfigBuilder;

-pub use crate::types::_capacity_provider_logging_config::CapacityProviderLoggingConfigBuilder;
-
-pub use crate::types::_function_versions_by_capacity_provider_list_item::FunctionVersionsByCapacityProviderListItemBuilder;
-
 pub use crate::types::_operation_update::OperationUpdateBuilder;

-pub use crate::types::_event::EventBuilder;
-
-pub use crate::types::_operation::OperationBuilder;
-
-pub use crate::types::_event_source_mapping_configuration::EventSourceMappingConfigurationBuilder;
+pub use crate::types::_capacity_provider_logging_config::CapacityProviderLoggingConfigBuilder;

 pub use crate::types::_source_access_configuration::SourceAccessConfigurationBuilder;

@@ -139,7 +131,9 @@

 pub use crate::types::_runtime_version_error::RuntimeVersionErrorBuilder;

-pub use crate::types::_provisioned_concurrency_config_list_item::ProvisionedConcurrencyConfigListItemBuilder;
+pub use crate::types::_event::EventBuilder;
+
+pub use crate::types::_operation::OperationBuilder;

 pub use crate::types::_resolved_s3_object::ResolvedS3ObjectBuilder;

@@ -149,18 +143,22 @@

 pub use crate::types::_invoke_with_response_stream_complete_event::InvokeWithResponseStreamCompleteEventBuilder;

+pub use crate::types::_alias_configuration::AliasConfigurationBuilder;
+
 pub use crate::types::_execution::ExecutionBuilder;

+pub use crate::types::_event_source_mapping_configuration::EventSourceMappingConfigurationBuilder;
+
 pub use crate::types::_function_url_config::FunctionUrlConfigBuilder;

-pub use crate::types::_alias_configuration::AliasConfigurationBuilder;
-
-pub use crate::types::_layers_list_item::LayersListItemBuilder;
+pub use crate::types::_function_versions_by_capacity_provider_list_item::FunctionVersionsByCapacityProviderListItemBuilder;

 pub use crate::types::_layer_versions_list_item::LayerVersionsListItemBuilder;

-pub use crate::types::_target_tracking_scaling_policy::TargetTrackingScalingPolicyBuilder;
+pub use crate::types::_layers_list_item::LayersListItemBuilder;

+pub use crate::types::_provisioned_concurrency_config_list_item::ProvisionedConcurrencyConfigListItemBuilder;
+
 pub use crate::types::_context_options::ContextOptionsBuilder;

 pub use crate::types::_step_options::StepOptionsBuilder;
@@ -171,6 +169,10 @@

 pub use crate::types::_chained_invoke_options::ChainedInvokeOptionsBuilder;

+pub use crate::types::_target_tracking_scaling_policy::TargetTrackingScalingPolicyBuilder;
+
+pub use crate::types::_filter::FilterBuilder;
+
 pub use crate::types::_execution_started_details::ExecutionStartedDetailsBuilder;

 pub use crate::types::_execution_succeeded_details::ExecutionSucceededDetailsBuilder;
@@ -231,7 +233,9 @@

 pub use crate::types::_chained_invoke_details::ChainedInvokeDetailsBuilder;

-pub use crate::types::_filter::FilterBuilder;
+pub use crate::types::_kafka_schema_registry_access_config::KafkaSchemaRegistryAccessConfigBuilder;
+
+pub use crate::types::_kafka_schema_validation_config::KafkaSchemaValidationConfigBuilder;

 pub use crate::types::_event_input::EventInputBuilder;

@@ -240,7 +244,3 @@
 pub use crate::types::_event_error::EventErrorBuilder;

 pub use crate::types::_retry_details::RetryDetailsBuilder;
-
-pub use crate::types::_kafka_schema_registry_access_config::KafkaSchemaRegistryAccessConfigBuilder;
-
-pub use crate::types::_kafka_schema_validation_config::KafkaSchemaValidationConfigBuilder;
```

### `src/types/error/_ec2_access_denied_exception.rs`

```diff
--- reference/src/types/error/_ec2_access_denied_exception.rs
+++ generated/src/types/error/_ec2_access_denied_exception.rs
@@ -24,7 +24,7 @@
 }
 impl ::std::fmt::Display for Ec2AccessDeniedException {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
-        ::std::write!(f, "Ec2AccessDeniedException [EC2AccessDeniedException]")?;
+        ::std::write!(f, "Ec2AccessDeniedException")?;
         if let ::std::option::Option::Some(inner_1) = &self.message {
             {
                 ::std::write!(f, ": {inner_1}")?;
```

### `src/types/error/_ec2_throttled_exception.rs`

```diff
--- reference/src/types/error/_ec2_throttled_exception.rs
+++ generated/src/types/error/_ec2_throttled_exception.rs
@@ -24,7 +24,7 @@
 }
 impl ::std::fmt::Display for Ec2ThrottledException {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
-        ::std::write!(f, "Ec2ThrottledException [EC2ThrottledException]")?;
+        ::std::write!(f, "Ec2ThrottledException")?;
         if let ::std::option::Option::Some(inner_1) = &self.message {
             {
                 ::std::write!(f, ": {inner_1}")?;
```

### `src/types/error/_ec2_unexpected_exception.rs`

```diff
--- reference/src/types/error/_ec2_unexpected_exception.rs
+++ generated/src/types/error/_ec2_unexpected_exception.rs
@@ -30,7 +30,7 @@
 }
 impl ::std::fmt::Display for Ec2UnexpectedException {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
-        ::std::write!(f, "Ec2UnexpectedException [EC2UnexpectedException]")?;
+        ::std::write!(f, "Ec2UnexpectedException")?;
         if let ::std::option::Option::Some(inner_1) = &self.message {
             {
                 ::std::write!(f, ": {inner_1}")?;
```

### `src/types/error/_efs_mount_connectivity_exception.rs`

```diff
--- reference/src/types/error/_efs_mount_connectivity_exception.rs
+++ generated/src/types/error/_efs_mount_connectivity_exception.rs
@@ -24,7 +24,7 @@
 }
 impl ::std::fmt::Display for EfsMountConnectivityException {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
-        ::std::write!(f, "EfsMountConnectivityException [EFSMountConnectivityException]")?;
+        ::std::write!(f, "EfsMountConnectivityException")?;
         if let ::std::option::Option::Some(inner_1) = &self.message {
             {
                 ::std::write!(f, ": {inner_1}")?;
```

### `src/types/error/_efs_mount_failure_exception.rs`

```diff
--- reference/src/types/error/_efs_mount_failure_exception.rs
+++ generated/src/types/error/_efs_mount_failure_exception.rs
@@ -24,7 +24,7 @@
 }
 impl ::std::fmt::Display for EfsMountFailureException {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
-        ::std::write!(f, "EfsMountFailureException [EFSMountFailureException]")?;
+        ::std::write!(f, "EfsMountFailureException")?;
         if let ::std::option::Option::Some(inner_1) = &self.message {
             {
                 ::std::write!(f, ": {inner_1}")?;
```

### `src/types/error/_efs_mount_timeout_exception.rs`

```diff
--- reference/src/types/error/_efs_mount_timeout_exception.rs
+++ generated/src/types/error/_efs_mount_timeout_exception.rs
@@ -24,7 +24,7 @@
 }
 impl ::std::fmt::Display for EfsMountTimeoutException {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
-        ::std::write!(f, "EfsMountTimeoutException [EFSMountTimeoutException]")?;
+        ::std::write!(f, "EfsMountTimeoutException")?;
         if let ::std::option::Option::Some(inner_1) = &self.message {
             {
                 ::std::write!(f, ": {inner_1}")?;
```

### `src/types/error/_efsio_exception.rs`

```diff
--- reference/src/types/error/_efsio_exception.rs
+++ generated/src/types/error/_efsio_exception.rs
@@ -24,7 +24,7 @@
 }
 impl ::std::fmt::Display for EfsioException {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
-        ::std::write!(f, "EfsioException [EFSIOException]")?;
+        ::std::write!(f, "EfsioException")?;
         if let ::std::option::Option::Some(inner_1) = &self.message {
             {
                 ::std::write!(f, ": {inner_1}")?;
```

### `src/types/error/_eni_limit_reached_exception.rs`

```diff
--- reference/src/types/error/_eni_limit_reached_exception.rs
+++ generated/src/types/error/_eni_limit_reached_exception.rs
@@ -24,7 +24,7 @@
 }
 impl ::std::fmt::Display for EniLimitReachedException {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
-        ::std::write!(f, "EniLimitReachedException [ENILimitReachedException]")?;
+        ::std::write!(f, "EniLimitReachedException")?;
         if let ::std::option::Option::Some(inner_1) = &self.message {
             {
                 ::std::write!(f, ": {inner_1}")?;
```

### `src/types/error/_eni_not_ready_exception.rs`

```diff
--- reference/src/types/error/_eni_not_ready_exception.rs
+++ generated/src/types/error/_eni_not_ready_exception.rs
@@ -24,7 +24,7 @@
 }
 impl ::std::fmt::Display for EniNotReadyException {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
-        ::std::write!(f, "EniNotReadyException [ENINotReadyException]")?;
+        ::std::write!(f, "EniNotReadyException")?;
         if let ::std::option::Option::Some(inner_1) = &self.message {
             {
                 ::std::write!(f, ": {inner_1}")?;
```

### `src/types/error/_invalid_security_group_id_exception.rs`

```diff
--- reference/src/types/error/_invalid_security_group_id_exception.rs
+++ generated/src/types/error/_invalid_security_group_id_exception.rs
@@ -24,7 +24,7 @@
 }
 impl ::std::fmt::Display for InvalidSecurityGroupIdException {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
-        ::std::write!(f, "InvalidSecurityGroupIdException [InvalidSecurityGroupIDException]")?;
+        ::std::write!(f, "InvalidSecurityGroupIdException")?;
         if let ::std::option::Option::Some(inner_1) = &self.message {
             {
                 ::std::write!(f, ": {inner_1}")?;
```

### `src/types/error/_invalid_subnet_id_exception.rs`

```diff
--- reference/src/types/error/_invalid_subnet_id_exception.rs
+++ generated/src/types/error/_invalid_subnet_id_exception.rs
@@ -24,7 +24,7 @@
 }
 impl ::std::fmt::Display for InvalidSubnetIdException {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
-        ::std::write!(f, "InvalidSubnetIdException [InvalidSubnetIDException]")?;
+        ::std::write!(f, "InvalidSubnetIdException")?;
         if let ::std::option::Option::Some(inner_1) = &self.message {
             {
                 ::std::write!(f, ": {inner_1}")?;
```

### `src/types/error/_kms_access_denied_exception.rs`

```diff
--- reference/src/types/error/_kms_access_denied_exception.rs
+++ generated/src/types/error/_kms_access_denied_exception.rs
@@ -24,7 +24,7 @@
 }
 impl ::std::fmt::Display for KmsAccessDeniedException {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
-        ::std::write!(f, "KmsAccessDeniedException [KMSAccessDeniedException]")?;
+        ::std::write!(f, "KmsAccessDeniedException")?;
         if let ::std::option::Option::Some(inner_1) = &self.message {
             {
                 ::std::write!(f, ": {inner_1}")?;
```

### `src/types/error/_kms_disabled_exception.rs`

```diff
--- reference/src/types/error/_kms_disabled_exception.rs
+++ generated/src/types/error/_kms_disabled_exception.rs
@@ -24,7 +24,7 @@
 }
 impl ::std::fmt::Display for KmsDisabledException {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
-        ::std::write!(f, "KmsDisabledException [KMSDisabledException]")?;
+        ::std::write!(f, "KmsDisabledException")?;
         if let ::std::option::Option::Some(inner_1) = &self.message {
             {
                 ::std::write!(f, ": {inner_1}")?;
```

### `src/types/error/_kms_invalid_state_exception.rs`

```diff
--- reference/src/types/error/_kms_invalid_state_exception.rs
+++ generated/src/types/error/_kms_invalid_state_exception.rs
@@ -24,7 +24,7 @@
 }
 impl ::std::fmt::Display for KmsInvalidStateException {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
-        ::std::write!(f, "KmsInvalidStateException [KMSInvalidStateException]")?;
+        ::std::write!(f, "KmsInvalidStateException")?;
         if let ::std::option::Option::Some(inner_1) = &self.message {
             {
                 ::std::write!(f, ": {inner_1}")?;
```

### `src/types/error/_kms_not_found_exception.rs`

```diff
--- reference/src/types/error/_kms_not_found_exception.rs
+++ generated/src/types/error/_kms_not_found_exception.rs
@@ -24,7 +24,7 @@
 }
 impl ::std::fmt::Display for KmsNotFoundException {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
-        ::std::write!(f, "KmsNotFoundException [KMSNotFoundException]")?;
+        ::std::write!(f, "KmsNotFoundException")?;
         if let ::std::option::Option::Some(inner_1) = &self.message {
             {
                 ::std::write!(f, ": {inner_1}")?;
```

### `src/types/error/_subnet_ip_address_limit_reached_exception.rs`

```diff
--- reference/src/types/error/_subnet_ip_address_limit_reached_exception.rs
+++ generated/src/types/error/_subnet_ip_address_limit_reached_exception.rs
@@ -24,7 +24,7 @@
 }
 impl ::std::fmt::Display for SubnetIpAddressLimitReachedException {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
-        ::std::write!(f, "SubnetIpAddressLimitReachedException [SubnetIPAddressLimitReachedException]")?;
+        ::std::write!(f, "SubnetIpAddressLimitReachedException")?;
         if let ::std::option::Option::Some(inner_1) = &self.message {
             {
                 ::std::write!(f, ": {inner_1}")?;
```

### `src/types/error/builders.rs`

```diff
--- reference/src/types/error/builders.rs
+++ generated/src/types/error/builders.rs
@@ -1,6 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub use crate::types::error::_invalid_parameter_value_exception::InvalidParameterValueExceptionBuilder;

+pub use crate::types::error::_policy_length_exceeded_exception::PolicyLengthExceededExceptionBuilder;
+
+pub use crate::types::error::_precondition_failed_exception::PreconditionFailedExceptionBuilder;
+
 pub use crate::types::error::_resource_conflict_exception::ResourceConflictExceptionBuilder;

 pub use crate::types::error::_resource_not_found_exception::ResourceNotFoundExceptionBuilder;
@@ -9,14 +13,8 @@

 pub use crate::types::error::_too_many_requests_exception::TooManyRequestsExceptionBuilder;

-pub use crate::types::error::_precondition_failed_exception::PreconditionFailedExceptionBuilder;
-
-pub use crate::types::error::_policy_length_exceeded_exception::PolicyLengthExceededExceptionBuilder;
-
 pub use crate::types::error::_public_policy_exception::PublicPolicyExceptionBuilder;

-pub use crate::types::error::_callback_timeout_exception::CallbackTimeoutExceptionBuilder;
-
 pub use crate::types::error::_kms_access_denied_exception::KmsAccessDeniedExceptionBuilder;

 pub use crate::types::error::_kms_disabled_exception::KmsDisabledExceptionBuilder;
@@ -25,10 +23,10 @@

 pub use crate::types::error::_kms_not_found_exception::KmsNotFoundExceptionBuilder;

+pub use crate::types::error::_alias_limit_exceeded_exception::AliasLimitExceededExceptionBuilder;
+
 pub use crate::types::error::_capacity_provider_limit_exceeded_exception::CapacityProviderLimitExceededExceptionBuilder;

-pub use crate::types::error::_resource_in_use_exception::ResourceInUseExceptionBuilder;
-
 pub use crate::types::error::_code_signing_config_not_found_exception::CodeSigningConfigNotFoundExceptionBuilder;

 pub use crate::types::error::_code_storage_exceeded_exception::CodeStorageExceededExceptionBuilder;
@@ -39,6 +37,10 @@

 pub use crate::types::error::_invalid_code_signature_exception::InvalidCodeSignatureExceptionBuilder;

+pub use crate::types::error::_resource_in_use_exception::ResourceInUseExceptionBuilder;
+
+pub use crate::types::error::_provisioned_concurrency_config_not_found_exception::ProvisionedConcurrencyConfigNotFoundExceptionBuilder;
+
 pub use crate::types::error::_code_artifact_user_deleted_exception::CodeArtifactUserDeletedExceptionBuilder;

 pub use crate::types::error::_code_artifact_user_failed_exception::CodeArtifactUserFailedExceptionBuilder;
@@ -107,6 +109,4 @@

 pub use crate::types::error::_unsupported_media_type_exception::UnsupportedMediaTypeExceptionBuilder;

-pub use crate::types::error::_alias_limit_exceeded_exception::AliasLimitExceededExceptionBuilder;
-
-pub use crate::types::error::_provisioned_concurrency_config_not_found_exception::ProvisionedConcurrencyConfigNotFoundExceptionBuilder;
+pub use crate::types::error::_callback_timeout_exception::CallbackTimeoutExceptionBuilder;
```

### `src/types/error.rs`

```diff
--- reference/src/types/error.rs
+++ generated/src/types/error.rs
@@ -1,6 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub use crate::types::error::_invalid_parameter_value_exception::InvalidParameterValueException;

+pub use crate::types::error::_policy_length_exceeded_exception::PolicyLengthExceededException;
+
+pub use crate::types::error::_precondition_failed_exception::PreconditionFailedException;
+
 pub use crate::types::error::_resource_conflict_exception::ResourceConflictException;

 pub use crate::types::error::_resource_not_found_exception::ResourceNotFoundException;
@@ -9,14 +13,8 @@

 pub use crate::types::error::_too_many_requests_exception::TooManyRequestsException;

-pub use crate::types::error::_precondition_failed_exception::PreconditionFailedException;
-
-pub use crate::types::error::_policy_length_exceeded_exception::PolicyLengthExceededException;
-
 pub use crate::types::error::_public_policy_exception::PublicPolicyException;

-pub use crate::types::error::_callback_timeout_exception::CallbackTimeoutException;
-
 pub use crate::types::error::_kms_access_denied_exception::KmsAccessDeniedException;

 pub use crate::types::error::_kms_disabled_exception::KmsDisabledException;
@@ -25,9 +23,9 @@

 pub use crate::types::error::_kms_not_found_exception::KmsNotFoundException;

-pub use crate::types::error::_capacity_provider_limit_exceeded_exception::CapacityProviderLimitExceededException;
+pub use crate::types::error::_alias_limit_exceeded_exception::AliasLimitExceededException;

-pub use crate::types::error::_resource_in_use_exception::ResourceInUseException;
+pub use crate::types::error::_capacity_provider_limit_exceeded_exception::CapacityProviderLimitExceededException;

 pub use crate::types::error::_code_signing_config_not_found_exception::CodeSigningConfigNotFoundException;

@@ -39,6 +37,10 @@

 pub use crate::types::error::_invalid_code_signature_exception::InvalidCodeSignatureException;

+pub use crate::types::error::_resource_in_use_exception::ResourceInUseException;
+
+pub use crate::types::error::_provisioned_concurrency_config_not_found_exception::ProvisionedConcurrencyConfigNotFoundException;
+
 pub use crate::types::error::_code_artifact_user_deleted_exception::CodeArtifactUserDeletedException;

 pub use crate::types::error::_code_artifact_user_failed_exception::CodeArtifactUserFailedException;
@@ -107,9 +109,7 @@

 pub use crate::types::error::_unsupported_media_type_exception::UnsupportedMediaTypeException;

-pub use crate::types::error::_alias_limit_exceeded_exception::AliasLimitExceededException;
-
-pub use crate::types::error::_provisioned_concurrency_config_not_found_exception::ProvisionedConcurrencyConfigNotFoundException;
+pub use crate::types::error::_callback_timeout_exception::CallbackTimeoutException;

 /// Error type for the `InvokeWithResponseStreamResponseEventError` operation.
 #[non_exhaustive]
@@ -198,6 +198,11 @@
         })
     }
 }
+impl crate::s3_request_id::RequestIdExt for crate::types::error::InvokeWithResponseStreamResponseEventError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for crate::types::error::InvokeWithResponseStreamResponseEventError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/types.rs`

```diff
--- reference/src/types.rs
+++ generated/src/types.rs
@@ -9,7 +9,11 @@

 pub use crate::types::_error_object::ErrorObject;

-pub use crate::types::_capacity_provider_state::CapacityProviderState;
+pub use crate::types::_function_url_auth_type::FunctionUrlAuthType;
+
+pub use crate::types::_checkpoint_updated_execution_state::CheckpointUpdatedExecutionState;
+
+pub use crate::types::_alias_routing_configuration::AliasRoutingConfiguration;

 pub use crate::types::_capacity_provider_vpc_config::CapacityProviderVpcConfig;

@@ -31,14 +35,6 @@

 pub use crate::types::_code_signing_config::CodeSigningConfig;

-pub use crate::types::_execution_status::ExecutionStatus;
-
-pub use crate::types::_trace_header::TraceHeader;
-
-pub use crate::types::_durable_config::DurableConfig;
-
-pub use crate::types::_checkpoint_updated_execution_state::CheckpointUpdatedExecutionState;
-
 pub use crate::types::_filter_criteria::FilterCriteria;

 pub use crate::types::_event_source_mapping_metrics_config::EventSourceMappingMetricsConfig;
@@ -61,8 +57,6 @@

 pub use crate::types::_filter_criteria_error::FilterCriteriaError;

-pub use crate::types::_function_version::FunctionVersion;
-
 pub use crate::types::_runtime::Runtime;

 pub use crate::types::_function_code::FunctionCode;
@@ -91,6 +85,8 @@

 pub use crate::types::_capacity_provider_config::CapacityProviderConfig;

+pub use crate::types::_durable_config::DurableConfig;
+
 pub use crate::types::_vpc_config_response::VpcConfigResponse;

 pub use crate::types::_environment_response::EnvironmentResponse;
@@ -111,14 +107,14 @@

 pub use crate::types::_runtime_version_config::RuntimeVersionConfig;

-pub use crate::types::_s3_object_storage_mode::S3ObjectStorageMode;
-
-pub use crate::types::_function_url_auth_type::FunctionUrlAuthType;
-
 pub use crate::types::_cors::Cors;

 pub use crate::types::_invoke_mode::InvokeMode;

+pub use crate::types::_execution_status::ExecutionStatus;
+
+pub use crate::types::_trace_header::TraceHeader;
+
 pub use crate::types::_function_configuration::FunctionConfiguration;

 pub use crate::types::_function_code_location::FunctionCodeLocation;
@@ -131,6 +127,10 @@

 pub use crate::types::_function_scaling_config::FunctionScalingConfig;

+pub use crate::types::_layer_version_content_output::LayerVersionContentOutput;
+
+pub use crate::types::_provisioned_concurrency_status_enum::ProvisionedConcurrencyStatusEnum;
+
 pub use crate::types::_update_runtime_on::UpdateRuntimeOn;

 pub use crate::types::_invocation_type::InvocationType;
@@ -141,15 +141,15 @@

 pub use crate::types::_invoke_with_response_stream_response_event::InvokeWithResponseStreamResponseEvent;

-pub use crate::types::_alias_routing_configuration::AliasRoutingConfiguration;
+pub use crate::types::_capacity_provider_state::CapacityProviderState;
+
+pub use crate::types::_function_version::FunctionVersion;

 pub use crate::types::_architecture::Architecture;

-pub use crate::types::_layer_version_content_output::LayerVersionContentOutput;
-
 pub use crate::types::_layer_version_content_input::LayerVersionContentInput;

-pub use crate::types::_provisioned_concurrency_status_enum::ProvisionedConcurrencyStatusEnum;
+pub use crate::types::_s3_object_storage_mode::S3ObjectStorageMode;

 pub use crate::types::_on_success::OnSuccess;

@@ -157,6 +157,8 @@

 pub use crate::types::_function_event_invoke_config::FunctionEventInvokeConfig;

+pub use crate::types::_operation_update::OperationUpdate;
+
 pub use crate::types::_capacity_provider_scaling_mode::CapacityProviderScalingMode;

 pub use crate::types::_propagate_tags_mode::PropagateTagsMode;
@@ -163,18 +165,8 @@

 pub use crate::types::_capacity_provider_logging_config::CapacityProviderLoggingConfig;

-pub use crate::types::_function_versions_by_capacity_provider_list_item::FunctionVersionsByCapacityProviderListItem;
-
 pub use crate::types::_code_signing_policy::CodeSigningPolicy;

-pub use crate::types::_operation_update::OperationUpdate;
-
-pub use crate::types::_event::Event;
-
-pub use crate::types::_operation::Operation;
-
-pub use crate::types::_event_source_mapping_configuration::EventSourceMappingConfiguration;
-
 pub use crate::types::_event_source_mapping_system_log_level::EventSourceMappingSystemLogLevel;

 pub use crate::types::_source_access_configuration::SourceAccessConfiguration;
@@ -211,7 +203,9 @@

 pub use crate::types::_runtime_version_error::RuntimeVersionError;

-pub use crate::types::_provisioned_concurrency_config_list_item::ProvisionedConcurrencyConfigListItem;
+pub use crate::types::_event::Event;
+
+pub use crate::types::_operation::Operation;

 pub use crate::types::_resolved_s3_object::ResolvedS3Object;

@@ -221,17 +215,21 @@

 pub use crate::types::_invoke_with_response_stream_complete_event::InvokeWithResponseStreamCompleteEvent;

+pub use crate::types::_alias_configuration::AliasConfiguration;
+
 pub use crate::types::_execution::Execution;

+pub use crate::types::_event_source_mapping_configuration::EventSourceMappingConfiguration;
+
 pub use crate::types::_function_url_config::FunctionUrlConfig;

-pub use crate::types::_alias_configuration::AliasConfiguration;
+pub use crate::types::_function_versions_by_capacity_provider_list_item::FunctionVersionsByCapacityProviderListItem;
+
+pub use crate::types::_layer_versions_list_item::LayerVersionsListItem;

 pub use crate::types::_layers_list_item::LayersListItem;

-pub use crate::types::_layer_versions_list_item::LayerVersionsListItem;
-
-pub use crate::types::_target_tracking_scaling_policy::TargetTrackingScalingPolicy;
+pub use crate::types::_provisioned_concurrency_config_list_item::ProvisionedConcurrencyConfigListItem;

 pub use crate::types::_operation_type::OperationType;

@@ -247,6 +245,18 @@

 pub use crate::types::_chained_invoke_options::ChainedInvokeOptions;

+pub use crate::types::_target_tracking_scaling_policy::TargetTrackingScalingPolicy;
+
+pub use crate::types::_filter::Filter;
+
+pub use crate::types::_event_source_mapping_metric::EventSourceMappingMetric;
+
+pub use crate::types::_source_access_type::SourceAccessType;
+
+pub use crate::types::_end_point_type::EndPointType;
+
+pub use crate::types::_schema_registry_event_record_format::SchemaRegistryEventRecordFormat;
+
 pub use crate::types::_event_type::EventType;

 pub use crate::types::_execution_started_details::ExecutionStartedDetails;
@@ -311,17 +321,11 @@

 pub use crate::types::_chained_invoke_details::ChainedInvokeDetails;

-pub use crate::types::_filter::Filter;
-
-pub use crate::types::_event_source_mapping_metric::EventSourceMappingMetric;
-
-pub use crate::types::_source_access_type::SourceAccessType;
+pub use crate::types::_capacity_provider_predefined_metric_type::CapacityProviderPredefinedMetricType;

-pub use crate::types::_end_point_type::EndPointType;
+pub use crate::types::_kafka_schema_registry_access_config::KafkaSchemaRegistryAccessConfig;

-pub use crate::types::_schema_registry_event_record_format::SchemaRegistryEventRecordFormat;
-
-pub use crate::types::_capacity_provider_predefined_metric_type::CapacityProviderPredefinedMetricType;
+pub use crate::types::_kafka_schema_validation_config::KafkaSchemaValidationConfig;

 pub use crate::types::_event_input::EventInput;

@@ -331,10 +335,6 @@

 pub use crate::types::_retry_details::RetryDetails;

-pub use crate::types::_kafka_schema_registry_access_config::KafkaSchemaRegistryAccessConfig;
-
-pub use crate::types::_kafka_schema_validation_config::KafkaSchemaValidationConfig;
-
 pub use crate::types::_kafka_schema_registry_auth_type::KafkaSchemaRegistryAuthType;

 pub use crate::types::_kafka_schema_validation_attribute::KafkaSchemaValidationAttribute;
```

### Missing reference files

- `Cargo.toml`
- `LICENSE`
- `README.md`
- `src/json_errors.rs`
- `src/protocol_serde/shape_account_limit.rs`
- `src/protocol_serde/shape_account_usage.rs`
- `src/protocol_serde/shape_add_layer_version_permission.rs`
- `src/protocol_serde/shape_add_layer_version_permission_input.rs`
- `src/protocol_serde/shape_add_permission.rs`
- `src/protocol_serde/shape_add_permission_input.rs`
- `src/protocol_serde/shape_additional_version_weights.rs`
- `src/protocol_serde/shape_alias_configuration.rs`
- `src/protocol_serde/shape_alias_limit_exceeded_exception.rs`
- `src/protocol_serde/shape_alias_list.rs`
- `src/protocol_serde/shape_alias_routing_configuration.rs`
- `src/protocol_serde/shape_allow_methods_list.rs`
- `src/protocol_serde/shape_allow_origins_list.rs`
- `src/protocol_serde/shape_allowed_publishers.rs`
- `src/protocol_serde/shape_amazon_managed_kafka_event_source_config.rs`
- `src/protocol_serde/shape_architectures_list.rs`
- `src/protocol_serde/shape_callback_details.rs`
- `src/protocol_serde/shape_callback_failed_details.rs`
- `src/protocol_serde/shape_callback_options.rs`
- `src/protocol_serde/shape_callback_started_details.rs`
- `src/protocol_serde/shape_callback_succeeded_details.rs`
- `src/protocol_serde/shape_callback_timed_out_details.rs`
- `src/protocol_serde/shape_callback_timeout_exception.rs`
- `src/protocol_serde/shape_capacity_provider.rs`
- `src/protocol_serde/shape_capacity_provider_config.rs`
- `src/protocol_serde/shape_capacity_provider_limit_exceeded_exception.rs`
- `src/protocol_serde/shape_capacity_provider_logging_config.rs`
- `src/protocol_serde/shape_capacity_provider_permissions_config.rs`
- `src/protocol_serde/shape_capacity_provider_scaling_config.rs`
- `src/protocol_serde/shape_capacity_provider_scaling_policies_list.rs`
- `src/protocol_serde/shape_capacity_provider_security_group_ids.rs`
- `src/protocol_serde/shape_capacity_provider_subnet_ids.rs`
- `src/protocol_serde/shape_capacity_provider_telemetry_config.rs`
- `src/protocol_serde/shape_capacity_provider_vpc_config.rs`
- `src/protocol_serde/shape_capacity_providers_list.rs`
- `src/protocol_serde/shape_chained_invoke_details.rs`
- `src/protocol_serde/shape_chained_invoke_failed_details.rs`
- `src/protocol_serde/shape_chained_invoke_options.rs`
- `src/protocol_serde/shape_chained_invoke_started_details.rs`
- `src/protocol_serde/shape_chained_invoke_stopped_details.rs`
- `src/protocol_serde/shape_chained_invoke_succeeded_details.rs`
- `src/protocol_serde/shape_chained_invoke_timed_out_details.rs`
- `src/protocol_serde/shape_checkpoint_durable_execution.rs`
- `src/protocol_serde/shape_checkpoint_durable_execution_input.rs`
- `src/protocol_serde/shape_checkpoint_updated_execution_state.rs`
- `src/protocol_serde/shape_code_artifact_user_deleted_exception.rs`
- `src/protocol_serde/shape_code_artifact_user_failed_exception.rs`
- `src/protocol_serde/shape_code_artifact_user_pending_exception.rs`
- `src/protocol_serde/shape_code_signing_config.rs`
- `src/protocol_serde/shape_code_signing_config_list.rs`
- `src/protocol_serde/shape_code_signing_config_not_found_exception.rs`
- `src/protocol_serde/shape_code_signing_policies.rs`
- `src/protocol_serde/shape_code_storage_exceeded_exception.rs`
- `src/protocol_serde/shape_code_verification_failed_exception.rs`
- `src/protocol_serde/shape_compatible_architectures.rs`
- `src/protocol_serde/shape_compatible_runtimes.rs`
- `src/protocol_serde/shape_concurrency.rs`
- `src/protocol_serde/shape_context_details.rs`
- `src/protocol_serde/shape_context_failed_details.rs`
- `src/protocol_serde/shape_context_options.rs`
- `src/protocol_serde/shape_context_started_details.rs`
- `src/protocol_serde/shape_context_succeeded_details.rs`
- `src/protocol_serde/shape_cors.rs`
- `src/protocol_serde/shape_create_alias.rs`
- `src/protocol_serde/shape_create_alias_input.rs`
- `src/protocol_serde/shape_create_capacity_provider.rs`
- `src/protocol_serde/shape_create_capacity_provider_input.rs`
- `src/protocol_serde/shape_create_code_signing_config.rs`
- `src/protocol_serde/shape_create_code_signing_config_input.rs`
- `src/protocol_serde/shape_create_event_source_mapping.rs`
- `src/protocol_serde/shape_create_event_source_mapping_input.rs`
- `src/protocol_serde/shape_create_function.rs`
- `src/protocol_serde/shape_create_function_input.rs`
- `src/protocol_serde/shape_create_function_url_config.rs`
- `src/protocol_serde/shape_create_function_url_config_input.rs`
- `src/protocol_serde/shape_dead_letter_config.rs`
- `src/protocol_serde/shape_delete_alias.rs`
- `src/protocol_serde/shape_delete_capacity_provider.rs`
- `src/protocol_serde/shape_delete_code_signing_config.rs`
- `src/protocol_serde/shape_delete_event_source_mapping.rs`
- `src/protocol_serde/shape_delete_function.rs`
- `src/protocol_serde/shape_delete_function_code_signing_config.rs`
- `src/protocol_serde/shape_delete_function_concurrency.rs`
- `src/protocol_serde/shape_delete_function_event_invoke_config.rs`
- `src/protocol_serde/shape_delete_function_url_config.rs`
- `src/protocol_serde/shape_delete_layer_version.rs`
- `src/protocol_serde/shape_delete_provisioned_concurrency_config.rs`
- `src/protocol_serde/shape_delete_resource_policy.rs`
- `src/protocol_serde/shape_destination_config.rs`
- `src/protocol_serde/shape_document_db_event_source_config.rs`
- `src/protocol_serde/shape_durable_config.rs`
- `src/protocol_serde/shape_durable_execution_already_started_exception.rs`
- `src/protocol_serde/shape_durable_executions.rs`
- `src/protocol_serde/shape_ec2_access_denied_exception.rs`
- `src/protocol_serde/shape_ec2_throttled_exception.rs`
- `src/protocol_serde/shape_ec2_unexpected_exception.rs`
- `src/protocol_serde/shape_efs_mount_connectivity_exception.rs`
- `src/protocol_serde/shape_efs_mount_failure_exception.rs`
- `src/protocol_serde/shape_efs_mount_timeout_exception.rs`
- `src/protocol_serde/shape_efsio_exception.rs`
- `src/protocol_serde/shape_endpoint_lists.rs`
- `src/protocol_serde/shape_endpoints.rs`
- `src/protocol_serde/shape_eni_limit_reached_exception.rs`
- `src/protocol_serde/shape_eni_not_ready_exception.rs`
- `src/protocol_serde/shape_environment.rs`
- `src/protocol_serde/shape_environment_error.rs`
- `src/protocol_serde/shape_environment_response.rs`
- `src/protocol_serde/shape_environment_variables.rs`
- `src/protocol_serde/shape_ephemeral_storage.rs`
- `src/protocol_serde/shape_error_object.rs`
- `src/protocol_serde/shape_event.rs`
- `src/protocol_serde/shape_event_error.rs`
- `src/protocol_serde/shape_event_input.rs`
- `src/protocol_serde/shape_event_result.rs`
- `src/protocol_serde/shape_event_source_mapping_configuration.rs`
- `src/protocol_serde/shape_event_source_mapping_logging_config.rs`
- `src/protocol_serde/shape_event_source_mapping_metric_list.rs`
- `src/protocol_serde/shape_event_source_mapping_metrics_config.rs`
- `src/protocol_serde/shape_event_source_mappings_list.rs`
- `src/protocol_serde/shape_events.rs`
- `src/protocol_serde/shape_execution.rs`
- `src/protocol_serde/shape_execution_details.rs`
- `src/protocol_serde/shape_execution_failed_details.rs`
- `src/protocol_serde/shape_execution_started_details.rs`
- `src/protocol_serde/shape_execution_stopped_details.rs`
- `src/protocol_serde/shape_execution_succeeded_details.rs`
- `src/protocol_serde/shape_execution_timed_out_details.rs`
- `src/protocol_serde/shape_file_system_config.rs`
- `src/protocol_serde/shape_file_system_config_list.rs`
- `src/protocol_serde/shape_filter.rs`
- `src/protocol_serde/shape_filter_criteria.rs`
- `src/protocol_serde/shape_filter_criteria_error.rs`
- `src/protocol_serde/shape_filter_list.rs`
- `src/protocol_serde/shape_function_arn_list.rs`
- `src/protocol_serde/shape_function_code.rs`
- `src/protocol_serde/shape_function_code_location.rs`
- `src/protocol_serde/shape_function_code_location_error.rs`
- `src/protocol_serde/shape_function_configuration.rs`
- `src/protocol_serde/shape_function_event_invoke_config.rs`
- `src/protocol_serde/shape_function_event_invoke_config_list.rs`
- `src/protocol_serde/shape_function_list.rs`
- `src/protocol_serde/shape_function_response_type_list.rs`
- `src/protocol_serde/shape_function_scaling_config.rs`
- `src/protocol_serde/shape_function_url_config.rs`
- `src/protocol_serde/shape_function_url_config_list.rs`
- `src/protocol_serde/shape_function_versions_by_capacity_provider_list.rs`
- `src/protocol_serde/shape_function_versions_by_capacity_provider_list_item.rs`
- `src/protocol_serde/shape_function_versions_per_capacity_provider_limit_exceeded_exception.rs`
- `src/protocol_serde/shape_get_account_settings.rs`
- `src/protocol_serde/shape_get_alias.rs`
- `src/protocol_serde/shape_get_capacity_provider.rs`
- `src/protocol_serde/shape_get_code_signing_config.rs`
- `src/protocol_serde/shape_get_durable_execution.rs`
- `src/protocol_serde/shape_get_durable_execution_history.rs`
- `src/protocol_serde/shape_get_durable_execution_state.rs`
- `src/protocol_serde/shape_get_event_source_mapping.rs`
- `src/protocol_serde/shape_get_function.rs`
- `src/protocol_serde/shape_get_function_code_signing_config.rs`
- `src/protocol_serde/shape_get_function_concurrency.rs`
- `src/protocol_serde/shape_get_function_configuration.rs`
- `src/protocol_serde/shape_get_function_event_invoke_config.rs`
- `src/protocol_serde/shape_get_function_recursion_config.rs`
- `src/protocol_serde/shape_get_function_scaling_config.rs`
- `src/protocol_serde/shape_get_function_url_config.rs`
- `src/protocol_serde/shape_get_layer_version.rs`
- `src/protocol_serde/shape_get_layer_version_by_arn.rs`
- `src/protocol_serde/shape_get_layer_version_policy.rs`
- `src/protocol_serde/shape_get_policy.rs`
- `src/protocol_serde/shape_get_provisioned_concurrency_config.rs`
- `src/protocol_serde/shape_get_resource_policy.rs`
- `src/protocol_serde/shape_get_runtime_management_config.rs`
- `src/protocol_serde/shape_headers_list.rs`
- `src/protocol_serde/shape_image_config.rs`
- `src/protocol_serde/shape_image_config_error.rs`
- `src/protocol_serde/shape_image_config_response.rs`
- `src/protocol_serde/shape_instance_requirements.rs`
- `src/protocol_serde/shape_instance_type_set.rs`
- `src/protocol_serde/shape_invalid_code_signature_exception.rs`
- `src/protocol_serde/shape_invalid_parameter_value_exception.rs`
- `src/protocol_serde/shape_invalid_request_content_exception.rs`
- `src/protocol_serde/shape_invalid_runtime_exception.rs`
- `src/protocol_serde/shape_invalid_security_group_id_exception.rs`
- `src/protocol_serde/shape_invalid_subnet_id_exception.rs`
- `src/protocol_serde/shape_invalid_zip_file_exception.rs`
- `src/protocol_serde/shape_invocation_completed_details.rs`
- `src/protocol_serde/shape_invoke.rs`
- `src/protocol_serde/shape_invoke_async.rs`
- `src/protocol_serde/shape_invoke_async_input.rs`
- `src/protocol_serde/shape_invoke_input.rs`
- `src/protocol_serde/shape_invoke_output.rs`
- `src/protocol_serde/shape_invoke_with_response_stream.rs`
- `src/protocol_serde/shape_invoke_with_response_stream_complete_event.rs`
- `src/protocol_serde/shape_invoke_with_response_stream_input.rs`
- `src/protocol_serde/shape_invoke_with_response_stream_output.rs`
- `src/protocol_serde/shape_kafka_schema_registry_access_config.rs`
- `src/protocol_serde/shape_kafka_schema_registry_access_config_list.rs`
- `src/protocol_serde/shape_kafka_schema_registry_config.rs`
- `src/protocol_serde/shape_kafka_schema_validation_config.rs`
- `src/protocol_serde/shape_kafka_schema_validation_config_list.rs`
- `src/protocol_serde/shape_kms_access_denied_exception.rs`
- `src/protocol_serde/shape_kms_disabled_exception.rs`
- `src/protocol_serde/shape_kms_invalid_state_exception.rs`
- `src/protocol_serde/shape_kms_not_found_exception.rs`
- `src/protocol_serde/shape_lambda_managed_instances_capacity_provider_config.rs`
- `src/protocol_serde/shape_layer.rs`
- `src/protocol_serde/shape_layer_version_content_input.rs`
- `src/protocol_serde/shape_layer_version_content_output.rs`
- `src/protocol_serde/shape_layer_versions_list.rs`
- `src/protocol_serde/shape_layer_versions_list_item.rs`
- `src/protocol_serde/shape_layers_list.rs`
- `src/protocol_serde/shape_layers_list_item.rs`
- `src/protocol_serde/shape_layers_reference_list.rs`
- `src/protocol_serde/shape_list_aliases.rs`
- `src/protocol_serde/shape_list_capacity_providers.rs`
- `src/protocol_serde/shape_list_code_signing_configs.rs`
- `src/protocol_serde/shape_list_durable_executions_by_function.rs`
- `src/protocol_serde/shape_list_event_source_mappings.rs`
- `src/protocol_serde/shape_list_function_event_invoke_configs.rs`
- `src/protocol_serde/shape_list_function_url_configs.rs`
- `src/protocol_serde/shape_list_function_versions_by_capacity_provider.rs`
- `src/protocol_serde/shape_list_functions.rs`
- `src/protocol_serde/shape_list_functions_by_code_signing_config.rs`
- `src/protocol_serde/shape_list_layer_versions.rs`
- `src/protocol_serde/shape_list_layers.rs`
- `src/protocol_serde/shape_list_provisioned_concurrency_configs.rs`
- `src/protocol_serde/shape_list_tags.rs`
- `src/protocol_serde/shape_list_versions_by_function.rs`
- `src/protocol_serde/shape_logging_config.rs`
- `src/protocol_serde/shape_mode_not_supported_exception.rs`
- `src/protocol_serde/shape_no_published_version_exception.rs`
- `src/protocol_serde/shape_on_failure.rs`
- `src/protocol_serde/shape_on_success.rs`
- `src/protocol_serde/shape_operation.rs`
- `src/protocol_serde/shape_operation_update.rs`
- `src/protocol_serde/shape_operations.rs`
- `src/protocol_serde/shape_policy_length_exceeded_exception.rs`
- `src/protocol_serde/shape_precondition_failed_exception.rs`
- `src/protocol_serde/shape_propagate_tags.rs`
- `src/protocol_serde/shape_provisioned_concurrency_config_list.rs`
- `src/protocol_serde/shape_provisioned_concurrency_config_list_item.rs`
- `src/protocol_serde/shape_provisioned_concurrency_config_not_found_exception.rs`
- `src/protocol_serde/shape_provisioned_poller_config.rs`
- `src/protocol_serde/shape_public_policy_exception.rs`
- `src/protocol_serde/shape_publish_layer_version.rs`
- `src/protocol_serde/shape_publish_layer_version_input.rs`
- `src/protocol_serde/shape_publish_version.rs`
- `src/protocol_serde/shape_publish_version_input.rs`
- `src/protocol_serde/shape_put_function_code_signing_config.rs`
- `src/protocol_serde/shape_put_function_code_signing_config_input.rs`
- `src/protocol_serde/shape_put_function_concurrency.rs`
- `src/protocol_serde/shape_put_function_concurrency_input.rs`
- `src/protocol_serde/shape_put_function_event_invoke_config.rs`
- `src/protocol_serde/shape_put_function_event_invoke_config_input.rs`
- `src/protocol_serde/shape_put_function_recursion_config.rs`
- `src/protocol_serde/shape_put_function_recursion_config_input.rs`
- `src/protocol_serde/shape_put_function_scaling_config.rs`
- `src/protocol_serde/shape_put_function_scaling_config_input.rs`
- `src/protocol_serde/shape_put_provisioned_concurrency_config.rs`
- `src/protocol_serde/shape_put_provisioned_concurrency_config_input.rs`
- `src/protocol_serde/shape_put_resource_policy.rs`
- `src/protocol_serde/shape_put_resource_policy_input.rs`
- `src/protocol_serde/shape_put_runtime_management_config.rs`
- `src/protocol_serde/shape_put_runtime_management_config_input.rs`
- `src/protocol_serde/shape_queues.rs`
- `src/protocol_serde/shape_recursive_invocation_exception.rs`
- `src/protocol_serde/shape_remove_layer_version_permission.rs`
- `src/protocol_serde/shape_remove_permission.rs`
- `src/protocol_serde/shape_request_too_large_exception.rs`
- `src/protocol_serde/shape_resolved_s3_object.rs`
- `src/protocol_serde/shape_resource_conflict_exception.rs`
- `src/protocol_serde/shape_resource_in_use_exception.rs`
- `src/protocol_serde/shape_resource_not_found_exception.rs`
- `src/protocol_serde/shape_resource_not_ready_exception.rs`
- `src/protocol_serde/shape_retry_details.rs`
- `src/protocol_serde/shape_runtime_version_config.rs`
- `src/protocol_serde/shape_runtime_version_error.rs`
- `src/protocol_serde/shape_s3_files_mount_connectivity_exception.rs`
- `src/protocol_serde/shape_s3_files_mount_failure_exception.rs`
- `src/protocol_serde/shape_s3_files_mount_timeout_exception.rs`
- `src/protocol_serde/shape_scaling_config.rs`
- `src/protocol_serde/shape_security_group_ids.rs`
- `src/protocol_serde/shape_self_managed_event_source.rs`
- `src/protocol_serde/shape_self_managed_kafka_event_source_config.rs`
- `src/protocol_serde/shape_send_durable_execution_callback_failure.rs`
- `src/protocol_serde/shape_send_durable_execution_callback_failure_input.rs`
- `src/protocol_serde/shape_send_durable_execution_callback_heartbeat.rs`
- `src/protocol_serde/shape_send_durable_execution_callback_success.rs`
- `src/protocol_serde/shape_send_durable_execution_callback_success_input.rs`
- `src/protocol_serde/shape_serialized_request_entity_too_large_exception.rs`
- `src/protocol_serde/shape_service_exception.rs`
- `src/protocol_serde/shape_service_quota_exceeded_exception.rs`
- `src/protocol_serde/shape_signing_profile_version_arns.rs`
- `src/protocol_serde/shape_snap_start.rs`
- `src/protocol_serde/shape_snap_start_exception.rs`
- `src/protocol_serde/shape_snap_start_not_ready_exception.rs`
- `src/protocol_serde/shape_snap_start_regeneration_failure_exception.rs`
- `src/protocol_serde/shape_snap_start_response.rs`
- `src/protocol_serde/shape_snap_start_timeout_exception.rs`
- `src/protocol_serde/shape_source_access_configuration.rs`
- `src/protocol_serde/shape_source_access_configurations.rs`
- `src/protocol_serde/shape_stack_trace_entries.rs`
- `src/protocol_serde/shape_step_details.rs`
- `src/protocol_serde/shape_step_failed_details.rs`
- `src/protocol_serde/shape_step_options.rs`
- `src/protocol_serde/shape_step_started_details.rs`
- `src/protocol_serde/shape_step_succeeded_details.rs`
- `src/protocol_serde/shape_stop_durable_execution.rs`
- `src/protocol_serde/shape_stop_durable_execution_input.rs`
- `src/protocol_serde/shape_string_list.rs`
- `src/protocol_serde/shape_subnet_ids.rs`
- `src/protocol_serde/shape_subnet_ip_address_limit_reached_exception.rs`
- `src/protocol_serde/shape_tag_resource.rs`
- `src/protocol_serde/shape_tag_resource_input.rs`
- `src/protocol_serde/shape_tags.rs`
- `src/protocol_serde/shape_tags_error.rs`
- `src/protocol_serde/shape_target_tracking_scaling_policy.rs`
- `src/protocol_serde/shape_tenancy_config.rs`
- `src/protocol_serde/shape_too_many_requests_exception.rs`
- `src/protocol_serde/shape_topics.rs`
- `src/protocol_serde/shape_trace_header.rs`
- `src/protocol_serde/shape_tracing_config.rs`
- `src/protocol_serde/shape_tracing_config_response.rs`
- `src/protocol_serde/shape_unsupported_media_type_exception.rs`
- `src/protocol_serde/shape_untag_resource.rs`
- `src/protocol_serde/shape_update_alias.rs`
- `src/protocol_serde/shape_update_alias_input.rs`
- `src/protocol_serde/shape_update_capacity_provider.rs`
- `src/protocol_serde/shape_update_capacity_provider_input.rs`
- `src/protocol_serde/shape_update_code_signing_config.rs`
- `src/protocol_serde/shape_update_code_signing_config_input.rs`
- `src/protocol_serde/shape_update_event_source_mapping.rs`
- `src/protocol_serde/shape_update_event_source_mapping_input.rs`
- `src/protocol_serde/shape_update_function_code.rs`
- `src/protocol_serde/shape_update_function_code_input.rs`
- `src/protocol_serde/shape_update_function_configuration.rs`
- `src/protocol_serde/shape_update_function_configuration_input.rs`
- `src/protocol_serde/shape_update_function_event_invoke_config.rs`
- `src/protocol_serde/shape_update_function_event_invoke_config_input.rs`
- `src/protocol_serde/shape_update_function_url_config.rs`
- `src/protocol_serde/shape_update_function_url_config_input.rs`
- `src/protocol_serde/shape_vpc_config.rs`
- `src/protocol_serde/shape_vpc_config_response.rs`
- `src/protocol_serde/shape_wait_cancelled_details.rs`
- `src/protocol_serde/shape_wait_details.rs`
- `src/protocol_serde/shape_wait_options.rs`
- `src/protocol_serde/shape_wait_started_details.rs`
- `src/protocol_serde/shape_wait_succeeded_details.rs`
- `src/protocol_serde.rs`
- `src/serialization_settings.rs`
- `tests/blns/LICENSE`
- `tests/blns/blns.txt`
- `tests/endpoint_tests.rs`
- `tests/naughty-strings-client-context.rs`
- `tests/request_id.rs`

### Rust token differences

- `src/client/delete_resource_policy.rs`
- `src/client/get_function_event_invoke_config.rs`
- `src/client/get_function_recursion_config.rs`
- `src/client/get_resource_policy.rs`
- `src/client/publish_layer_version.rs`
- `src/client/publish_version.rs`
- `src/client/put_function_event_invoke_config.rs`
- `src/client/put_resource_policy.rs`
- `src/client/update_function_code.rs`
- `src/client/update_function_event_invoke_config.rs`
- `src/operation/add_layer_version_permission/_add_layer_version_permission_input.rs`
- `src/operation/add_layer_version_permission.rs`
- `src/operation/add_permission.rs`
- `src/operation/checkpoint_durable_execution.rs`
- `src/operation/create_alias.rs`
- `src/operation/create_capacity_provider.rs`
- `src/operation/create_code_signing_config.rs`
- `src/operation/create_event_source_mapping.rs`
- `src/operation/create_function/_create_function_input.rs`
- `src/operation/create_function.rs`
- `src/operation/create_function_url_config.rs`
- `src/operation/delete_alias.rs`
- `src/operation/delete_capacity_provider.rs`
- `src/operation/delete_code_signing_config.rs`
- `src/operation/delete_event_source_mapping.rs`
- `src/operation/delete_function.rs`
- `src/operation/delete_function_code_signing_config.rs`
- `src/operation/delete_function_concurrency.rs`
- `src/operation/delete_function_event_invoke_config.rs`
- `src/operation/delete_function_url_config.rs`
- `src/operation/delete_layer_version/_delete_layer_version_input.rs`
- `src/operation/delete_layer_version.rs`
- `src/operation/delete_provisioned_concurrency_config.rs`
- `src/operation/delete_resource_policy.rs`
- `src/operation/get_account_settings.rs`
- `src/operation/get_alias.rs`
- `src/operation/get_capacity_provider.rs`
- `src/operation/get_code_signing_config.rs`
- `src/operation/get_durable_execution/_get_durable_execution_input.rs`
- `src/operation/get_durable_execution.rs`
- `src/operation/get_durable_execution_history/_get_durable_execution_history_input.rs`
- `src/operation/get_durable_execution_history.rs`
- `src/operation/get_durable_execution_state/_get_durable_execution_state_input.rs`
- `src/operation/get_durable_execution_state.rs`
- `src/operation/get_event_source_mapping.rs`
- `src/operation/get_function.rs`
- `src/operation/get_function_code_signing_config.rs`
- `src/operation/get_function_concurrency.rs`
- `src/operation/get_function_configuration.rs`
- `src/operation/get_function_event_invoke_config.rs`
- `src/operation/get_function_recursion_config.rs`
- `src/operation/get_function_scaling_config.rs`
- `src/operation/get_function_url_config.rs`
- `src/operation/get_layer_version/_get_layer_version_input.rs`
- `src/operation/get_layer_version/_get_layer_version_output.rs`
- `src/operation/get_layer_version.rs`
- `src/operation/get_layer_version_by_arn/_get_layer_version_by_arn_output.rs`
- `src/operation/get_layer_version_by_arn.rs`
- `src/operation/get_layer_version_policy/_get_layer_version_policy_input.rs`
- `src/operation/get_layer_version_policy.rs`
- `src/operation/get_policy.rs`
- `src/operation/get_provisioned_concurrency_config.rs`
- `src/operation/get_resource_policy.rs`
- `src/operation/get_runtime_management_config.rs`
- `src/operation/invoke.rs`
- `src/operation/invoke_async.rs`
- `src/operation/invoke_with_response_stream.rs`
- `src/operation/list_aliases.rs`
- `src/operation/list_capacity_providers.rs`
- `src/operation/list_code_signing_configs.rs`
- `src/operation/list_durable_executions_by_function/_list_durable_executions_by_function_input.rs`
- `src/operation/list_durable_executions_by_function.rs`
- `src/operation/list_event_source_mappings.rs`
- `src/operation/list_function_event_invoke_configs.rs`
- `src/operation/list_function_url_configs.rs`
- `src/operation/list_function_versions_by_capacity_provider.rs`
- `src/operation/list_functions.rs`
- `src/operation/list_functions_by_code_signing_config.rs`
- `src/operation/list_layer_versions.rs`
- `src/operation/list_layers.rs`
- `src/operation/list_provisioned_concurrency_configs.rs`
- `src/operation/list_tags.rs`
- `src/operation/list_versions_by_function.rs`
- `src/operation/publish_layer_version/_publish_layer_version_input.rs`
- `src/operation/publish_layer_version/_publish_layer_version_output.rs`
- `src/operation/publish_layer_version/builders.rs`
- `src/operation/publish_layer_version.rs`
- `src/operation/publish_version.rs`
- `src/operation/put_function_code_signing_config.rs`
- `src/operation/put_function_concurrency.rs`
- `src/operation/put_function_event_invoke_config.rs`
- `src/operation/put_function_recursion_config.rs`
- `src/operation/put_function_scaling_config.rs`
- `src/operation/put_provisioned_concurrency_config.rs`
- `src/operation/put_resource_policy.rs`
- `src/operation/put_runtime_management_config.rs`
- `src/operation/remove_layer_version_permission/_remove_layer_version_permission_input.rs`
- `src/operation/remove_layer_version_permission.rs`
- `src/operation/remove_permission.rs`
- `src/operation/send_durable_execution_callback_failure.rs`
- `src/operation/send_durable_execution_callback_heartbeat.rs`
- `src/operation/send_durable_execution_callback_success.rs`
- `src/operation/stop_durable_execution.rs`
- `src/operation/tag_resource.rs`
- `src/operation/untag_resource.rs`
- `src/operation/update_alias.rs`
- `src/operation/update_capacity_provider.rs`
- `src/operation/update_code_signing_config.rs`
- `src/operation/update_event_source_mapping.rs`
- `src/operation/update_function_code/_update_function_code_input.rs`
- `src/operation/update_function_code.rs`
- `src/operation/update_function_configuration.rs`
- `src/operation/update_function_event_invoke_config.rs`
- `src/operation/update_function_url_config.rs`
- `src/serde_util.rs`
- `src/types/_capacity_provider_logging_config.rs`
- `src/types/_event.rs`
- `src/types/_lambda_managed_instances_capacity_provider_config.rs`
- `src/types/_logging_config.rs`
- `src/types/_operation_update.rs`
- `src/types/_propagate_tags_mode.rs`
- `src/types/_s3_object_storage_mode.rs`
- `src/types/builders.rs`
- `src/types/error/_ec2_access_denied_exception.rs`
- `src/types/error/_ec2_throttled_exception.rs`
- `src/types/error/_ec2_unexpected_exception.rs`
- `src/types/error/_efs_mount_connectivity_exception.rs`
- `src/types/error/_efs_mount_failure_exception.rs`
- `src/types/error/_efs_mount_timeout_exception.rs`
- `src/types/error/_efsio_exception.rs`
- `src/types/error/_eni_limit_reached_exception.rs`
- `src/types/error/_eni_not_ready_exception.rs`
- `src/types/error/_invalid_security_group_id_exception.rs`
- `src/types/error/_invalid_subnet_id_exception.rs`
- `src/types/error/_kms_access_denied_exception.rs`
- `src/types/error/_kms_disabled_exception.rs`
- `src/types/error/_kms_invalid_state_exception.rs`
- `src/types/error/_kms_not_found_exception.rs`
- `src/types/error/_subnet_ip_address_limit_reached_exception.rs`
- `src/types/error/builders.rs`
- `src/types/error.rs`
- `src/types.rs`
