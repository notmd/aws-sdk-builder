# AWS SDK Conformance Report: lambda

Snapshot: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`

## lambda
**Progress:** `1131/1131` files compared · `821` matched · `251` mismatches · `4` missing · `55` extra · `72.59%` match (100.00% means fully matched)

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
     pub fn delete_resource_policy(&self) -> super::operation::delete_resource_policy::builders::DeleteResourcePolicyFluentBuilder {
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
     pub fn get_resource_policy(&self) -> super::operation::get_resource_policy::builders::GetResourcePolicyFluentBuilder {
         super::operation::get_resource_policy::builders::GetResourcePolicyFluentBuilder::new(self.handle.clone())
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
-                            super::protocol_serde::shape_invoke_with_response_stream_complete_event::de_invoke_with_response_stream_complete_event_payload(&message.payload()[..])
-                                                .map_err(|err| {
-                                                    ::aws_smithy_eventstream::error::Error::unmarshalling(format!("failed to unmarshall InvokeComplete: {err}"))
-                                                })?
+                        super::protocol_serde::shape_invoke_with_response_stream_complete_event::de_invoke_with_response_stream_complete_event_payload(&message.payload()[..])
+                            .map_err(|err| {
+                                ::aws_smithy_eventstream::error::Error::unmarshalling(format!("failed to unmarshall InvokeComplete: {err}"))
+                            })?
                         ;
                     Ok(::aws_smithy_eventstream::frame::UnmarshalledMessage::Event(
                         super::types::InvokeWithResponseStreamResponseEvent::InvokeComplete(parsed),
```

### `src/operation/add_layer_version_permission/_add_layer_version_permission_input.rs`

```diff
--- reference/src/operation/add_layer_version_permission/_add_layer_version_permission_input.rs
+++ generated/src/operation/add_layer_version_permission/_add_layer_version_permission_input.rs
@@ -180,7 +180,7 @@
     > {
         ::std::result::Result::Ok(super::operation::add_layer_version_permission::AddLayerVersionPermissionInput {
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
@@ -277,8 +277,7 @@
                 let input_2 = input_2
                     .as_ref()
                     .ok_or_else(|| ::aws_smithy_types::error::operation::BuildError::missing_field("version_number", "cannot be empty or unset"))?;
-                let mut version_number_encoder = ::aws_smithy_types::primitive::Encoder::from(*input_2);
-                let version_number = version_number_encoder.encode();
+                let version_number = ::aws_smithy_http::label::fmt_string(input_2, ::aws_smithy_http::label::EncodingStrategy::Default);
                 if version_number.is_empty() {
                     return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
                         "version_number",
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

### `src/operation/delete_alias.rs`

```diff
--- reference/src/operation/delete_alias.rs
+++ generated/src/operation/delete_alias.rs
@@ -280,10 +280,14 @@
                 ::std::result::Result::Ok(builder.method("DELETE").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_delete_alias::ser_delete_alias_input(&input)?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/delete_capacity_provider.rs`

```diff
--- reference/src/operation/delete_capacity_provider.rs
+++ generated/src/operation/delete_capacity_provider.rs
@@ -266,10 +266,16 @@
                 ::std::result::Result::Ok(builder.method("DELETE").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_delete_capacity_provider::ser_delete_capacity_provider_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/delete_code_signing_config.rs`

```diff
--- reference/src/operation/delete_code_signing_config.rs
+++ generated/src/operation/delete_code_signing_config.rs
@@ -266,10 +266,16 @@
                 ::std::result::Result::Ok(builder.method("DELETE").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_delete_code_signing_config::ser_delete_code_signing_config_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/delete_event_source_mapping.rs`

```diff
--- reference/src/operation/delete_event_source_mapping.rs
+++ generated/src/operation/delete_event_source_mapping.rs
@@ -261,10 +261,16 @@
                 ::std::result::Result::Ok(builder.method("DELETE").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_delete_event_source_mapping::ser_delete_event_source_mapping_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/delete_function.rs`

```diff
--- reference/src/operation/delete_function.rs
+++ generated/src/operation/delete_function.rs
@@ -276,10 +276,14 @@
                 ::std::result::Result::Ok(builder.method("DELETE").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_delete_function::ser_delete_function_input(&input)?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/delete_function_code_signing_config.rs`

```diff
--- reference/src/operation/delete_function_code_signing_config.rs
+++ generated/src/operation/delete_function_code_signing_config.rs
@@ -274,10 +274,16 @@
                 ::std::result::Result::Ok(builder.method("DELETE").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_delete_function_code_signing_config::ser_delete_function_code_signing_config_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/delete_function_concurrency.rs`

```diff
--- reference/src/operation/delete_function_concurrency.rs
+++ generated/src/operation/delete_function_concurrency.rs
@@ -262,10 +262,16 @@
                 ::std::result::Result::Ok(builder.method("DELETE").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_delete_function_concurrency::ser_delete_function_concurrency_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/delete_function_event_invoke_config.rs`

```diff
--- reference/src/operation/delete_function_event_invoke_config.rs
+++ generated/src/operation/delete_function_event_invoke_config.rs
@@ -292,10 +292,16 @@
                 ::std::result::Result::Ok(builder.method("DELETE").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_delete_function_event_invoke_config::ser_delete_function_event_invoke_config_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/delete_function_url_config.rs`

```diff
--- reference/src/operation/delete_function_url_config.rs
+++ generated/src/operation/delete_function_url_config.rs
@@ -279,10 +279,16 @@
                 ::std::result::Result::Ok(builder.method("DELETE").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_delete_function_url_config::ser_delete_function_url_config_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/delete_layer_version/_delete_layer_version_input.rs`

```diff
--- reference/src/operation/delete_layer_version/_delete_layer_version_input.rs
+++ generated/src/operation/delete_layer_version/_delete_layer_version_input.rs
@@ -70,7 +70,7 @@
     {
         ::std::result::Result::Ok(super::operation::delete_layer_version::DeleteLayerVersionInput {
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
@@ -252,8 +252,7 @@
                 let input_2 = input_2
                     .as_ref()
                     .ok_or_else(|| ::aws_smithy_types::error::operation::BuildError::missing_field("version_number", "cannot be empty or unset"))?;
-                let mut version_number_encoder = ::aws_smithy_types::primitive::Encoder::from(*input_2);
-                let version_number = version_number_encoder.encode();
+                let version_number = ::aws_smithy_http::label::fmt_string(input_2, ::aws_smithy_http::label::EncodingStrategy::Default);
                 if version_number.is_empty() {
                     return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
                         "version_number",
@@ -279,10 +278,15 @@
                 ::std::result::Result::Ok(builder.method("DELETE").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body =
+            ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_delete_layer_version::ser_delete_layer_version_input(&input)?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/delete_provisioned_concurrency_config.rs`

```diff
--- reference/src/operation/delete_provisioned_concurrency_config.rs
+++ generated/src/operation/delete_provisioned_concurrency_config.rs
@@ -300,10 +300,16 @@
                 ::std::result::Result::Ok(builder.method("DELETE").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_delete_provisioned_concurrency_config::ser_delete_provisioned_concurrency_config_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/delete_resource_policy.rs`

```diff
--- reference/src/operation/delete_resource_policy.rs
+++ generated/src/operation/delete_resource_policy.rs
@@ -279,10 +279,16 @@
                 ::std::result::Result::Ok(builder.method("DELETE").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_delete_resource_policy::ser_delete_resource_policy_input(
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

### `src/operation/get_account_settings.rs`

```diff
--- reference/src/operation/get_account_settings.rs
+++ generated/src/operation/get_account_settings.rs
@@ -204,10 +204,15 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body =
+            ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_get_account_settings::ser_get_account_settings_input(&input)?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/get_alias.rs`

```diff
--- reference/src/operation/get_alias.rs
+++ generated/src/operation/get_alias.rs
@@ -274,10 +274,14 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_get_alias::ser_get_alias_input(&input)?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/get_capacity_provider.rs`

```diff
--- reference/src/operation/get_capacity_provider.rs
+++ generated/src/operation/get_capacity_provider.rs
@@ -266,10 +266,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_get_capacity_provider::ser_get_capacity_provider_input(
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

### `src/operation/get_code_signing_config.rs`

```diff
--- reference/src/operation/get_code_signing_config.rs
+++ generated/src/operation/get_code_signing_config.rs
@@ -266,10 +266,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_get_code_signing_config::ser_get_code_signing_config_input(
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

### `src/operation/get_durable_execution/_get_durable_execution_input.rs`

```diff
--- reference/src/operation/get_durable_execution/_get_durable_execution_input.rs
+++ generated/src/operation/get_durable_execution/_get_durable_execution_input.rs
@@ -69,7 +69,7 @@
     {
         ::std::result::Result::Ok(super::operation::get_durable_execution::GetDurableExecutionInput {
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
@@ -280,10 +280,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_get_durable_execution::ser_get_durable_execution_input(
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

### `src/operation/get_durable_execution_history/_get_durable_execution_history_input.rs`

```diff
--- reference/src/operation/get_durable_execution_history/_get_durable_execution_history_input.rs
+++ generated/src/operation/get_durable_execution_history/_get_durable_execution_history_input.rs
@@ -135,7 +135,7 @@
         ::std::result::Result::Ok(super::operation::get_durable_execution_history::GetDurableExecutionHistoryInput {
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
@@ -300,10 +300,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_get_durable_execution_history::ser_get_durable_execution_history_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
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
@@ -306,10 +306,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_get_durable_execution_state::ser_get_durable_execution_state_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/get_event_source_mapping.rs`

```diff
--- reference/src/operation/get_event_source_mapping.rs
+++ generated/src/operation/get_event_source_mapping.rs
@@ -261,10 +261,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_get_event_source_mapping::ser_get_event_source_mapping_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/get_function.rs`

```diff
--- reference/src/operation/get_function.rs
+++ generated/src/operation/get_function.rs
@@ -277,10 +277,14 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_get_function::ser_get_function_input(&input)?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/get_function_code_signing_config.rs`

```diff
--- reference/src/operation/get_function_code_signing_config.rs
+++ generated/src/operation/get_function_code_signing_config.rs
@@ -272,10 +272,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_get_function_code_signing_config::ser_get_function_code_signing_config_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/get_function_concurrency.rs`

```diff
--- reference/src/operation/get_function_concurrency.rs
+++ generated/src/operation/get_function_concurrency.rs
@@ -262,10 +262,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_get_function_concurrency::ser_get_function_concurrency_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/get_function_configuration.rs`

```diff
--- reference/src/operation/get_function_configuration.rs
+++ generated/src/operation/get_function_configuration.rs
@@ -281,10 +281,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_get_function_configuration::ser_get_function_configuration_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/get_function_event_invoke_config.rs`

```diff
--- reference/src/operation/get_function_event_invoke_config.rs
+++ generated/src/operation/get_function_event_invoke_config.rs
@@ -290,10 +290,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_get_function_event_invoke_config::ser_get_function_event_invoke_config_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/get_function_recursion_config.rs`

```diff
--- reference/src/operation/get_function_recursion_config.rs
+++ generated/src/operation/get_function_recursion_config.rs
@@ -266,10 +266,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_get_function_recursion_config::ser_get_function_recursion_config_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/get_function_scaling_config.rs`

```diff
--- reference/src/operation/get_function_scaling_config.rs
+++ generated/src/operation/get_function_scaling_config.rs
@@ -290,10 +290,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_get_function_scaling_config::ser_get_function_scaling_config_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/get_function_url_config.rs`

```diff
--- reference/src/operation/get_function_url_config.rs
+++ generated/src/operation/get_function_url_config.rs
@@ -279,10 +279,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_get_function_url_config::ser_get_function_url_config_input(
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

### `src/operation/get_layer_version/_get_layer_version_input.rs`

```diff
--- reference/src/operation/get_layer_version/_get_layer_version_input.rs
+++ generated/src/operation/get_layer_version/_get_layer_version_input.rs
@@ -69,7 +69,7 @@
     ) -> ::std::result::Result<super::operation::get_layer_version::GetLayerVersionInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::operation::get_layer_version::GetLayerVersionInput {
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
-    pub content: ::std::option::Option<super::types::LayerVersionContentOutput>,
+    pub content: ::std::option::Option<super::operation::get_layer_version::Output>,
     /// <p>The ARN of the layer.</p>
     pub layer_arn: ::std::option::Option<::std::string::String>,
     /// <p>The ARN of the layer version.</p>
@@ -27,7 +27,7 @@
 }
 impl GetLayerVersionOutput {
     /// <p>Details about the layer version.</p>
-    pub fn content(&self) -> ::std::option::Option<&super::types::LayerVersionContentOutput> {
+    pub fn content(&self) -> ::std::option::Option<&super::operation::get_layer_version::Output> {
         self.content.as_ref()
     }
     /// <p>The ARN of the layer.</p>
@@ -85,7 +85,7 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
 pub struct GetLayerVersionOutputBuilder {
-    pub(crate) content: ::std::option::Option<super::types::LayerVersionContentOutput>,
+    pub(crate) content: ::std::option::Option<super::operation::get_layer_version::Output>,
     pub(crate) layer_arn: ::std::option::Option<::std::string::String>,
     pub(crate) layer_version_arn: ::std::option::Option<::std::string::String>,
     pub(crate) description: ::std::option::Option<::std::string::String>,
@@ -98,17 +98,17 @@
 }
 impl GetLayerVersionOutputBuilder {
     /// <p>Details about the layer version.</p>
-    pub fn content(mut self, input: super::types::LayerVersionContentOutput) -> Self {
+    pub fn content(mut self, input: super::operation::get_layer_version::Output) -> Self {
         self.content = ::std::option::Option::Some(input);
         self
     }
     /// <p>Details about the layer version.</p>
-    pub fn set_content(mut self, input: ::std::option::Option<super::types::LayerVersionContentOutput>) -> Self {
+    pub fn set_content(mut self, input: ::std::option::Option<super::operation::get_layer_version::Output>) -> Self {
         self.content = input;
         self
     }
     /// <p>Details about the layer version.</p>
-    pub fn get_content(&self) -> &::std::option::Option<super::types::LayerVersionContentOutput> {
+    pub fn get_content(&self) -> &::std::option::Option<super::operation::get_layer_version::Output> {
         &self.content
     }
     /// <p>The ARN of the layer.</p>
```

### `src/operation/get_layer_version.rs`

```diff
--- reference/src/operation/get_layer_version.rs
+++ generated/src/operation/get_layer_version.rs
@@ -249,8 +249,7 @@
                 let input_2 = input_2
                     .as_ref()
                     .ok_or_else(|| ::aws_smithy_types::error::operation::BuildError::missing_field("version_number", "cannot be empty or unset"))?;
-                let mut version_number_encoder = ::aws_smithy_types::primitive::Encoder::from(*input_2);
-                let version_number = version_number_encoder.encode();
+                let version_number = ::aws_smithy_http::label::fmt_string(input_2, ::aws_smithy_http::label::EncodingStrategy::Default);
                 if version_number.is_empty() {
                     return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
                         "version_number",
@@ -276,10 +275,14 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_get_layer_version::ser_get_layer_version_input(&input)?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/get_layer_version_by_arn/_get_layer_version_by_arn_output.rs`

```diff
--- reference/src/operation/get_layer_version_by_arn/_get_layer_version_by_arn_output.rs
+++ generated/src/operation/get_layer_version_by_arn/_get_layer_version_by_arn_output.rs
@@ -4,7 +4,7 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct GetLayerVersionByArnOutput {
     /// <p>Details about the layer version.</p>
-    pub content: ::std::option::Option<super::types::LayerVersionContentOutput>,
+    pub content: ::std::option::Option<super::operation::get_layer_version_by_arn::Output>,
     /// <p>The ARN of the layer.</p>
     pub layer_arn: ::std::option::Option<::std::string::String>,
     /// <p>The ARN of the layer version.</p>
@@ -27,7 +27,7 @@
 }
 impl GetLayerVersionByArnOutput {
     /// <p>Details about the layer version.</p>
-    pub fn content(&self) -> ::std::option::Option<&super::types::LayerVersionContentOutput> {
+    pub fn content(&self) -> ::std::option::Option<&super::operation::get_layer_version_by_arn::Output> {
         self.content.as_ref()
     }
     /// <p>The ARN of the layer.</p>
@@ -85,7 +85,7 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
 pub struct GetLayerVersionByArnOutputBuilder {
-    pub(crate) content: ::std::option::Option<super::types::LayerVersionContentOutput>,
+    pub(crate) content: ::std::option::Option<super::operation::get_layer_version_by_arn::Output>,
     pub(crate) layer_arn: ::std::option::Option<::std::string::String>,
     pub(crate) layer_version_arn: ::std::option::Option<::std::string::String>,
     pub(crate) description: ::std::option::Option<::std::string::String>,
@@ -98,17 +98,17 @@
 }
 impl GetLayerVersionByArnOutputBuilder {
     /// <p>Details about the layer version.</p>
-    pub fn content(mut self, input: super::types::LayerVersionContentOutput) -> Self {
+    pub fn content(mut self, input: super::operation::get_layer_version_by_arn::Output) -> Self {
         self.content = ::std::option::Option::Some(input);
         self
     }
     /// <p>Details about the layer version.</p>
-    pub fn set_content(mut self, input: ::std::option::Option<super::types::LayerVersionContentOutput>) -> Self {
+    pub fn set_content(mut self, input: ::std::option::Option<super::operation::get_layer_version_by_arn::Output>) -> Self {
         self.content = input;
         self
     }
     /// <p>Details about the layer version.</p>
-    pub fn get_content(&self) -> &::std::option::Option<super::types::LayerVersionContentOutput> {
+    pub fn get_content(&self) -> &::std::option::Option<super::operation::get_layer_version_by_arn::Output> {
         &self.content
     }
     /// <p>The ARN of the layer.</p>
```

### `src/operation/get_layer_version_by_arn.rs`

```diff
--- reference/src/operation/get_layer_version_by_arn.rs
+++ generated/src/operation/get_layer_version_by_arn.rs
@@ -270,10 +270,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_get_layer_version_by_arn::ser_get_layer_version_by_arn_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/get_layer_version_policy/_get_layer_version_policy_input.rs`

```diff
--- reference/src/operation/get_layer_version_policy/_get_layer_version_policy_input.rs
+++ generated/src/operation/get_layer_version_policy/_get_layer_version_policy_input.rs
@@ -70,7 +70,7 @@
     {
         ::std::result::Result::Ok(super::operation::get_layer_version_policy::GetLayerVersionPolicyInput {
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
@@ -252,8 +252,7 @@
                 let input_2 = input_2
                     .as_ref()
                     .ok_or_else(|| ::aws_smithy_types::error::operation::BuildError::missing_field("version_number", "cannot be empty or unset"))?;
-                let mut version_number_encoder = ::aws_smithy_types::primitive::Encoder::from(*input_2);
-                let version_number = version_number_encoder.encode();
+                let version_number = ::aws_smithy_http::label::fmt_string(input_2, ::aws_smithy_http::label::EncodingStrategy::Default);
                 if version_number.is_empty() {
                     return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
                         "version_number",
@@ -279,10 +278,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_get_layer_version_policy::ser_get_layer_version_policy_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/get_policy.rs`

```diff
--- reference/src/operation/get_policy.rs
+++ generated/src/operation/get_policy.rs
@@ -275,10 +275,14 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_get_policy::ser_get_policy_input(&input)?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/get_provisioned_concurrency_config.rs`

```diff
--- reference/src/operation/get_provisioned_concurrency_config.rs
+++ generated/src/operation/get_provisioned_concurrency_config.rs
@@ -298,10 +298,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_get_provisioned_concurrency_config::ser_get_provisioned_concurrency_config_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/get_resource_policy.rs`

```diff
--- reference/src/operation/get_resource_policy.rs
+++ generated/src/operation/get_resource_policy.rs
@@ -261,10 +261,14 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_get_resource_policy::ser_get_resource_policy_input(&input)?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/get_runtime_management_config.rs`

```diff
--- reference/src/operation/get_runtime_management_config.rs
+++ generated/src/operation/get_runtime_management_config.rs
@@ -284,10 +284,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_get_runtime_management_config::ser_get_runtime_management_config_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/invoke.rs`

```diff
--- reference/src/operation/invoke.rs
+++ generated/src/operation/invoke.rs
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
@@ -281,10 +284,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/octet-stream");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_invoke_input::ser_payload_http_payload(input.payload)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_invoke::ser_invoke_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/invoke_async.rs`

```diff
--- reference/src/operation/invoke_async.rs
+++ generated/src/operation/invoke_async.rs
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
@@ -256,10 +259,10 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/octet-stream");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = super::protocol_serde::shape_invoke_async_input::ser_invoke_args_http_payload(input.invoke_args)?.into_inner();
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_invoke_async::ser_invoke_async_input(&input)?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/invoke_with_response_stream.rs`

```diff
--- reference/src/operation/invoke_with_response_stream.rs
+++ generated/src/operation/invoke_with_response_stream.rs
@@ -214,6 +214,7 @@
     ) -> ::std::option::Option<::aws_smithy_runtime_api::client::interceptors::context::OutputOrError> {
         #[allow(unused_mut)]
         let mut force_error = false;
+        ::tracing::debug!(extended_request_id = ?crate::s3_request_id::RequestIdExt::extended_request_id(response));
         ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));

         // If this is an error, defer to the non-streaming parser
@@ -307,12 +308,12 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/octet-stream");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_invoke_with_response_stream_input::ser_payload_http_payload(
-            input.payload,
-        )?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_invoke_with_response_stream::ser_invoke_with_response_stream_input(&input)?,
+        );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
```

### `src/operation/list_aliases.rs`

```diff
--- reference/src/operation/list_aliases.rs
+++ generated/src/operation/list_aliases.rs
@@ -292,10 +292,14 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_list_aliases::ser_list_aliases_input(&input)?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/list_capacity_providers.rs`

```diff
--- reference/src/operation/list_capacity_providers.rs
+++ generated/src/operation/list_capacity_providers.rs
@@ -273,10 +273,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_list_capacity_providers::ser_list_capacity_providers_input(
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

### `src/operation/list_code_signing_configs.rs`

```diff
--- reference/src/operation/list_code_signing_configs.rs
+++ generated/src/operation/list_code_signing_configs.rs
@@ -268,10 +268,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_list_code_signing_configs::ser_list_code_signing_configs_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
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
@@ -296,40 +296,38 @@
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
@@ -345,10 +343,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_list_durable_executions_by_function::ser_list_durable_executions_by_function_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/list_event_source_mappings.rs`

```diff
--- reference/src/operation/list_event_source_mappings.rs
+++ generated/src/operation/list_event_source_mappings.rs
@@ -288,10 +288,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_list_event_source_mappings::ser_list_event_source_mappings_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/list_function_event_invoke_configs.rs`

```diff
--- reference/src/operation/list_function_event_invoke_configs.rs
+++ generated/src/operation/list_function_event_invoke_configs.rs
@@ -297,10 +297,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_list_function_event_invoke_configs::ser_list_function_event_invoke_configs_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/list_function_url_configs.rs`

```diff
--- reference/src/operation/list_function_url_configs.rs
+++ generated/src/operation/list_function_url_configs.rs
@@ -284,10 +284,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_list_function_url_configs::ser_list_function_url_configs_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/list_function_versions_by_capacity_provider.rs`

```diff
--- reference/src/operation/list_function_versions_by_capacity_provider.rs
+++ generated/src/operation/list_function_versions_by_capacity_provider.rs
@@ -299,10 +299,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_list_function_versions_by_capacity_provider::ser_list_function_versions_by_capacity_provider_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/list_functions.rs`

```diff
--- reference/src/operation/list_functions.rs
+++ generated/src/operation/list_functions.rs
@@ -281,10 +281,14 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_list_functions::ser_list_functions_input(&input)?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/list_functions_by_code_signing_config.rs`

```diff
--- reference/src/operation/list_functions_by_code_signing_config.rs
+++ generated/src/operation/list_functions_by_code_signing_config.rs
@@ -299,10 +299,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_list_functions_by_code_signing_config::ser_list_functions_by_code_signing_config_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/list_layer_versions.rs`

```diff
--- reference/src/operation/list_layer_versions.rs
+++ generated/src/operation/list_layer_versions.rs
@@ -294,10 +294,14 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_list_layer_versions::ser_list_layer_versions_input(&input)?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/list_layers.rs`

```diff
--- reference/src/operation/list_layers.rs
+++ generated/src/operation/list_layers.rs
@@ -273,10 +273,14 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_list_layers::ser_list_layers_input(&input)?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/list_provisioned_concurrency_configs.rs`

```diff
--- reference/src/operation/list_provisioned_concurrency_configs.rs
+++ generated/src/operation/list_provisioned_concurrency_configs.rs
@@ -300,10 +300,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_list_provisioned_concurrency_configs::ser_list_provisioned_concurrency_configs_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/list_tags.rs`

```diff
--- reference/src/operation/list_tags.rs
+++ generated/src/operation/list_tags.rs
@@ -252,10 +252,14 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_list_tags::ser_list_tags_input(&input)?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/list_versions_by_function.rs`

```diff
--- reference/src/operation/list_versions_by_function.rs
+++ generated/src/operation/list_versions_by_function.rs
@@ -286,10 +286,16 @@
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_list_versions_by_function::ser_list_versions_by_function_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/publish_layer_version/_publish_layer_version_input.rs`

```diff
--- reference/src/operation/publish_layer_version/_publish_layer_version_input.rs
+++ generated/src/operation/publish_layer_version/_publish_layer_version_input.rs
@@ -8,7 +8,7 @@
     /// <p>The description of the version.</p>
     pub description: ::std::option::Option<::std::string::String>,
     /// <p>The function layer archive.</p>
-    pub content: ::std::option::Option<super::types::LayerVersionContentInput>,
+    pub content: ::std::option::Option<super::operation::publish_layer_version::Input>,
     /// <p>A list of compatible <a href="https://docs.aws.amazon.com/lambda/latest/dg/foundation-arch.html">instruction set architectures</a>.</p>
     pub compatible_architectures: ::std::option::Option<::std::vec::Vec<super::types::Architecture>>,
     /// <p>A list of compatible <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html">function runtimes</a>. Used for filtering with <code>ListLayers</code> and <code>ListLayerVersions</code>.</p>
@@ -35,7 +35,7 @@
         self.description.as_deref()
     }
     /// <p>The function layer archive.</p>
-    pub fn content(&self) -> ::std::option::Option<&super::types::LayerVersionContentInput> {
+    pub fn content(&self) -> ::std::option::Option<&super::operation::publish_layer_version::Input> {
         self.content.as_ref()
     }
     /// <p>A list of compatible <a href="https://docs.aws.amazon.com/lambda/latest/dg/foundation-arch.html">instruction set architectures</a>.</p>
@@ -77,7 +77,7 @@
 pub struct PublishLayerVersionInputBuilder {
     pub(crate) layer_name: ::std::option::Option<::std::string::String>,
     pub(crate) description: ::std::option::Option<::std::string::String>,
-    pub(crate) content: ::std::option::Option<super::types::LayerVersionContentInput>,
+    pub(crate) content: ::std::option::Option<super::operation::publish_layer_version::Input>,
     pub(crate) compatible_architectures: ::std::option::Option<::std::vec::Vec<super::types::Architecture>>,
     pub(crate) compatible_runtimes: ::std::option::Option<::std::vec::Vec<super::types::Runtime>>,
     pub(crate) license_info: ::std::option::Option<::std::string::String>,
@@ -114,17 +114,17 @@
     }
     /// <p>The function layer archive.</p>
     /// This field is required.
-    pub fn content(mut self, input: super::types::LayerVersionContentInput) -> Self {
+    pub fn content(mut self, input: super::operation::publish_layer_version::Input) -> Self {
         self.content = ::std::option::Option::Some(input);
         self
     }
     /// <p>The function layer archive.</p>
-    pub fn set_content(mut self, input: ::std::option::Option<super::types::LayerVersionContentInput>) -> Self {
+    pub fn set_content(mut self, input: ::std::option::Option<super::operation::publish_layer_version::Input>) -> Self {
         self.content = input;
         self
     }
     /// <p>The function layer archive.</p>
-    pub fn get_content(&self) -> &::std::option::Option<super::types::LayerVersionContentInput> {
+    pub fn get_content(&self) -> &::std::option::Option<super::operation::publish_layer_version::Input> {
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
-    pub content: ::std::option::Option<super::types::LayerVersionContentOutput>,
+    pub content: ::std::option::Option<super::operation::publish_layer_version::Output>,
     /// <p>The ARN of the layer.</p>
     pub layer_arn: ::std::option::Option<::std::string::String>,
     /// <p>The ARN of the layer version.</p>
@@ -27,7 +27,7 @@
 }
 impl PublishLayerVersionOutput {
     /// <p>Details about the layer version.</p>
-    pub fn content(&self) -> ::std::option::Option<&super::types::LayerVersionContentOutput> {
+    pub fn content(&self) -> ::std::option::Option<&super::operation::publish_layer_version::Output> {
         self.content.as_ref()
     }
     /// <p>The ARN of the layer.</p>
@@ -85,7 +85,7 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
 pub struct PublishLayerVersionOutputBuilder {
-    pub(crate) content: ::std::option::Option<super::types::LayerVersionContentOutput>,
+    pub(crate) content: ::std::option::Option<super::operation::publish_layer_version::Output>,
     pub(crate) layer_arn: ::std::option::Option<::std::string::String>,
     pub(crate) layer_version_arn: ::std::option::Option<::std::string::String>,
     pub(crate) description: ::std::option::Option<::std::string::String>,
@@ -98,17 +98,17 @@
 }
 impl PublishLayerVersionOutputBuilder {
     /// <p>Details about the layer version.</p>
-    pub fn content(mut self, input: super::types::LayerVersionContentOutput) -> Self {
+    pub fn content(mut self, input: super::operation::publish_layer_version::Output) -> Self {
         self.content = ::std::option::Option::Some(input);
         self
     }
     /// <p>Details about the layer version.</p>
-    pub fn set_content(mut self, input: ::std::option::Option<super::types::LayerVersionContentOutput>) -> Self {
+    pub fn set_content(mut self, input: ::std::option::Option<super::operation::publish_layer_version::Output>) -> Self {
         self.content = input;
         self
     }
     /// <p>Details about the layer version.</p>
-    pub fn get_content(&self) -> &::std::option::Option<super::types::LayerVersionContentOutput> {
+    pub fn get_content(&self) -> &::std::option::Option<super::operation::publish_layer_version::Output> {
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
-    pub fn content(mut self, input: super::types::LayerVersionContentInput) -> Self {
+    pub fn content(mut self, input: super::operation::publish_layer_version::Input) -> Self {
         self.inner = self.inner.content(input);
         self
     }
     /// <p>The function layer archive.</p>
-    pub fn set_content(mut self, input: ::std::option::Option<super::types::LayerVersionContentInput>) -> Self {
+    pub fn set_content(mut self, input: ::std::option::Option<super::operation::publish_layer_version::Input>) -> Self {
         self.inner = self.inner.set_content(input);
         self
     }
     /// <p>The function layer archive.</p>
-    pub fn get_content(&self) -> &::std::option::Option<super::types::LayerVersionContentInput> {
+    pub fn get_content(&self) -> &::std::option::Option<super::operation::publish_layer_version::Input> {
         self.inner.get_content()
     }
     ///
```

### `src/operation/remove_layer_version_permission/_remove_layer_version_permission_input.rs`

```diff
--- reference/src/operation/remove_layer_version_permission/_remove_layer_version_permission_input.rs
+++ generated/src/operation/remove_layer_version_permission/_remove_layer_version_permission_input.rs
@@ -115,7 +115,7 @@
     > {
         ::std::result::Result::Ok(super::operation::remove_layer_version_permission::RemoveLayerVersionPermissionInput {
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
@@ -268,8 +268,7 @@
                 let input_2 = input_2
                     .as_ref()
                     .ok_or_else(|| ::aws_smithy_types::error::operation::BuildError::missing_field("version_number", "cannot be empty or unset"))?;
-                let mut version_number_encoder = ::aws_smithy_types::primitive::Encoder::from(*input_2);
-                let version_number = version_number_encoder.encode();
+                let version_number = ::aws_smithy_http::label::fmt_string(input_2, ::aws_smithy_http::label::EncodingStrategy::Default);
                 if version_number.is_empty() {
                     return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
                         "version_number",
@@ -320,10 +319,16 @@
                 ::std::result::Result::Ok(builder.method("DELETE").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_remove_layer_version_permission::ser_remove_layer_version_permission_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/remove_permission.rs`

```diff
--- reference/src/operation/remove_permission.rs
+++ generated/src/operation/remove_permission.rs
@@ -311,10 +311,14 @@
                 ::std::result::Result::Ok(builder.method("DELETE").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_remove_permission::ser_remove_permission_input(&input)?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/send_durable_execution_callback_failure.rs`

```diff
--- reference/src/operation/send_durable_execution_callback_failure.rs
+++ generated/src/operation/send_durable_execution_callback_failure.rs
@@ -280,7 +280,7 @@
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::protocol_serde::shape_send_durable_execution_callback_failure_input::ser_error_http_payload(&input.error)?,
+            super::protocol_serde::shape_send_durable_execution_callback_failure::ser_send_durable_execution_callback_failure_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/send_durable_execution_callback_heartbeat.rs`

```diff
--- reference/src/operation/send_durable_execution_callback_heartbeat.rs
+++ generated/src/operation/send_durable_execution_callback_heartbeat.rs
@@ -276,10 +276,16 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from("");
-
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            super::protocol_serde::shape_send_durable_execution_callback_heartbeat::ser_send_durable_execution_callback_heartbeat_input(&input)?,
+        );
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
```

### `src/operation/send_durable_execution_callback_success.rs`

```diff
--- reference/src/operation/send_durable_execution_callback_success.rs
+++ generated/src/operation/send_durable_execution_callback_success.rs
@@ -276,11 +276,11 @@
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/octet-stream");
+            builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            super::protocol_serde::shape_send_durable_execution_callback_success_input::ser_result_http_payload(input.result)?,
+            super::protocol_serde::shape_send_durable_execution_callback_success::ser_send_durable_execution_callback_success_input(&input)?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
```

### `src/operation/stop_durable_execution.rs`

```diff
--- reference/src/operation/stop_durable_execution.rs
+++ generated/src/operation/stop_durable_execution.rs
@@ -269,8 +269,8 @@
             builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/json");
             builder
         };
-        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_stop_durable_execution_input::ser_error_http_payload(
-            &input.error,
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_stop_durable_execution::ser_stop_durable_execution_input(
+            &input,
         )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
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
+        let body = ::aws_smithy_types::body::SdkBody::from(super::protocol_serde::shape_untag_resource::ser_untag_resource_input(&input)?);
+        if let Some(content_length) = body.content_length() {
+            let content_length = content_length.to_string();
+            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+        }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
 }
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

### `src/protocol_serde/shape_add_layer_version_permission.rs`

```diff
--- reference/src/protocol_serde/shape_add_layer_version_permission.rs
+++ generated/src/protocol_serde/shape_add_layer_version_permission.rs
@@ -134,13 +134,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::add_layer_version_permission::AddLayerVersionPermissionError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::add_layer_version_permission::AddLayerVersionPermissionError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -198,15 +191,15 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "RevisionId" => {
-                    builder = builder.set_revision_id(
+                "Statement" => {
+                    builder = builder.set_statement(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "Statement" => {
-                    builder = builder.set_statement(
+                "RevisionId" => {
+                    builder = builder.set_revision_id(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
```

### `src/protocol_serde/shape_add_layer_version_permission_input.rs`

```diff
--- reference/src/protocol_serde/shape_add_layer_version_permission_input.rs
+++ generated/src/protocol_serde/shape_add_layer_version_permission_input.rs
@@ -3,17 +3,29 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::add_layer_version_permission::AddLayerVersionPermissionInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.action {
-        object.key("Action").string(var_1.as_str());
+    if let Some(var_1) = &input.layer_name {
+        object.key("LayerName").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.version_number {
+        object.key("VersionNumber").number(
+            #[allow(clippy::useless_conversion)]
+            ::aws_smithy_types::Number::NegInt((*var_2).into()),
+        );
+    }
+    if let Some(var_3) = &input.statement_id {
+        object.key("StatementId").string(var_3.as_str());
+    }
+    if let Some(var_4) = &input.action {
+        object.key("Action").string(var_4.as_str());
     }
-    if let Some(var_2) = &input.organization_id {
-        object.key("OrganizationId").string(var_2.as_str());
+    if let Some(var_5) = &input.principal {
+        object.key("Principal").string(var_5.as_str());
     }
-    if let Some(var_3) = &input.principal {
-        object.key("Principal").string(var_3.as_str());
+    if let Some(var_6) = &input.organization_id {
+        object.key("OrganizationId").string(var_6.as_str());
     }
-    if let Some(var_4) = &input.statement_id {
-        object.key("StatementId").string(var_4.as_str());
+    if let Some(var_7) = &input.revision_id {
+        object.key("RevisionId").string(var_7.as_str());
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_add_permission.rs`

```diff
--- reference/src/protocol_serde/shape_add_permission.rs
+++ generated/src/protocol_serde/shape_add_permission.rs
@@ -136,11 +136,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::add_permission::AddPermissionError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::add_permission::AddPermissionError::unhandled("Failed to parse retryAfterSeconds from header `Retry-After")
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
```

### `src/protocol_serde/shape_add_permission_input.rs`

```diff
--- reference/src/protocol_serde/shape_add_permission_input.rs
+++ generated/src/protocol_serde/shape_add_permission_input.rs
@@ -3,35 +3,41 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::add_permission::AddPermissionInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.action {
-        object.key("Action").string(var_1.as_str());
+    if let Some(var_1) = &input.function_name {
+        object.key("FunctionName").string(var_1.as_str());
     }
-    if let Some(var_2) = &input.event_source_token {
-        object.key("EventSourceToken").string(var_2.as_str());
+    if let Some(var_2) = &input.statement_id {
+        object.key("StatementId").string(var_2.as_str());
     }
-    if let Some(var_3) = &input.function_url_auth_type {
-        object.key("FunctionUrlAuthType").string(var_3.as_str());
+    if let Some(var_3) = &input.action {
+        object.key("Action").string(var_3.as_str());
     }
-    if let Some(var_4) = &input.invoked_via_function_url {
-        object.key("InvokedViaFunctionUrl").boolean(*var_4);
+    if let Some(var_4) = &input.principal {
+        object.key("Principal").string(var_4.as_str());
     }
-    if let Some(var_5) = &input.principal {
-        object.key("Principal").string(var_5.as_str());
+    if let Some(var_5) = &input.source_arn {
+        object.key("SourceArn").string(var_5.as_str());
     }
-    if let Some(var_6) = &input.principal_org_id {
-        object.key("PrincipalOrgID").string(var_6.as_str());
+    if let Some(var_6) = &input.function_url_auth_type {
+        object.key("FunctionUrlAuthType").string(var_6.as_str());
     }
-    if let Some(var_7) = &input.revision_id {
-        object.key("RevisionId").string(var_7.as_str());
+    if let Some(var_7) = &input.invoked_via_function_url {
+        object.key("InvokedViaFunctionUrl").boolean(*var_7);
     }
     if let Some(var_8) = &input.source_account {
         object.key("SourceAccount").string(var_8.as_str());
     }
-    if let Some(var_9) = &input.source_arn {
-        object.key("SourceArn").string(var_9.as_str());
+    if let Some(var_9) = &input.event_source_token {
+        object.key("EventSourceToken").string(var_9.as_str());
     }
-    if let Some(var_10) = &input.statement_id {
-        object.key("StatementId").string(var_10.as_str());
+    if let Some(var_10) = &input.qualifier {
+        object.key("Qualifier").string(var_10.as_str());
+    }
+    if let Some(var_11) = &input.revision_id {
+        object.key("RevisionId").string(var_11.as_str());
+    }
+    if let Some(var_12) = &input.principal_org_id {
+        object.key("PrincipalOrgID").string(var_12.as_str());
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_alias_routing_configuration.rs`

```diff
--- reference/src/protocol_serde/shape_alias_routing_configuration.rs
+++ generated/src/protocol_serde/shape_alias_routing_configuration.rs
@@ -1,4 +1,24 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_alias_routing_configuration(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::types::AliasRoutingConfiguration,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    if let Some(var_1) = &input.additional_version_weights {
+        #[allow(unused_mut)]
+        let mut object_2 = object.key("AdditionalVersionWeights").start_object();
+        for (key_3, value_4) in var_1 {
+            {
+                object_2.key(key_3.as_str()).number(
+                    #[allow(clippy::useless_conversion)]
+                    ::aws_smithy_types::Number::Float((*value_4).into()),
+                );
+            }
+        }
+        object_2.finish();
+    }
+    Ok(())
+}
+
 pub(crate) fn de_alias_routing_configuration<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
@@ -42,23 +62,3 @@
         )),
     }
 }
-
-pub fn ser_alias_routing_configuration(
-    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
-    input: &super::types::AliasRoutingConfiguration,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.additional_version_weights {
-        #[allow(unused_mut)]
-        let mut object_2 = object.key("AdditionalVersionWeights").start_object();
-        for (key_3, value_4) in var_1 {
-            {
-                object_2.key(key_3.as_str()).number(
-                    #[allow(clippy::useless_conversion)]
-                    ::aws_smithy_types::Number::Float((*value_4).into()),
-                );
-            }
-        }
-        object_2.finish();
-    }
-    Ok(())
-}
```

### `src/protocol_serde/shape_amazon_managed_kafka_event_source_config.rs`

```diff
--- reference/src/protocol_serde/shape_amazon_managed_kafka_event_source_config.rs
+++ generated/src/protocol_serde/shape_amazon_managed_kafka_event_source_config.rs
@@ -1,4 +1,20 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_amazon_managed_kafka_event_source_config(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::types::AmazonManagedKafkaEventSourceConfig,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    if let Some(var_1) = &input.consumer_group_id {
+        object.key("ConsumerGroupId").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.schema_registry_config {
+        #[allow(unused_mut)]
+        let mut object_3 = object.key("SchemaRegistryConfig").start_object();
+        super::protocol_serde::shape_kafka_schema_registry_config::ser_kafka_schema_registry_config(&mut object_3, var_2)?;
+        object_3.finish();
+    }
+    Ok(())
+}
+
 pub(crate) fn de_amazon_managed_kafka_event_source_config<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
@@ -53,19 +69,3 @@
         )),
     }
 }
-
-pub fn ser_amazon_managed_kafka_event_source_config(
-    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
-    input: &super::types::AmazonManagedKafkaEventSourceConfig,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.consumer_group_id {
-        object.key("ConsumerGroupId").string(var_1.as_str());
-    }
-    if let Some(var_2) = &input.schema_registry_config {
-        #[allow(unused_mut)]
-        let mut object_3 = object.key("SchemaRegistryConfig").start_object();
-        super::protocol_serde::shape_kafka_schema_registry_config::ser_kafka_schema_registry_config(&mut object_3, var_2)?;
-        object_3.finish();
-    }
-    Ok(())
-}
```

### `src/protocol_serde/shape_callback_options.rs`

```diff
--- reference/src/protocol_serde/shape_callback_options.rs
+++ generated/src/protocol_serde/shape_callback_options.rs
@@ -3,13 +3,13 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::types::CallbackOptions,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if input.timeout_seconds != 0 {
+    {
         object.key("TimeoutSeconds").number(
             #[allow(clippy::useless_conversion)]
             ::aws_smithy_types::Number::NegInt((input.timeout_seconds).into()),
         );
     }
-    if input.heartbeat_timeout_seconds != 0 {
+    {
         object.key("HeartbeatTimeoutSeconds").number(
             #[allow(clippy::useless_conversion)]
             ::aws_smithy_types::Number::NegInt((input.heartbeat_timeout_seconds).into()),
```

### `src/protocol_serde/shape_capacity_provider_config.rs`

```diff
--- reference/src/protocol_serde/shape_capacity_provider_config.rs
+++ generated/src/protocol_serde/shape_capacity_provider_config.rs
@@ -1,4 +1,20 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_capacity_provider_config(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::types::CapacityProviderConfig,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    if let Some(var_1) = &input.lambda_managed_instances_capacity_provider_config {
+        #[allow(unused_mut)]
+        let mut object_2 = object.key("LambdaManagedInstancesCapacityProviderConfig").start_object();
+        super::protocol_serde::shape_lambda_managed_instances_capacity_provider_config::ser_lambda_managed_instances_capacity_provider_config(
+            &mut object_2,
+            var_1,
+        )?;
+        object_2.finish();
+    }
+    Ok(())
+}
+
 pub(crate) fn de_capacity_provider_config<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
@@ -22,9 +38,7 @@
                     Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                     Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                         "LambdaManagedInstancesCapacityProviderConfig" => {
-                            builder = builder.set_lambda_managed_instances_capacity_provider_config(
-                                    super::protocol_serde::shape_lambda_managed_instances_capacity_provider_config::de_lambda_managed_instances_capacity_provider_config(tokens, _value, depth + 1)?
-                                );
+                            builder = builder.set_lambda_managed_instances_capacity_provider_config(super::protocol_serde::shape_lambda_managed_instances_capacity_provider_config::de_lambda_managed_instances_capacity_provider_config(tokens, _value, depth + 1)?);
                         }
                         _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                     },
@@ -42,19 +56,3 @@
         )),
     }
 }
-
-pub fn ser_capacity_provider_config(
-    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
-    input: &super::types::CapacityProviderConfig,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.lambda_managed_instances_capacity_provider_config {
-        #[allow(unused_mut)]
-        let mut object_2 = object.key("LambdaManagedInstancesCapacityProviderConfig").start_object();
-        super::protocol_serde::shape_lambda_managed_instances_capacity_provider_config::ser_lambda_managed_instances_capacity_provider_config(
-            &mut object_2,
-            var_1,
-        )?;
-        object_2.finish();
-    }
-    Ok(())
-}
```

### `src/protocol_serde/shape_checkpoint_durable_execution.rs`

```diff
--- reference/src/protocol_serde/shape_checkpoint_durable_execution.rs
+++ generated/src/protocol_serde/shape_checkpoint_durable_execution.rs
@@ -122,13 +122,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::checkpoint_durable_execution::CheckpointDurableExecutionError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::checkpoint_durable_execution::CheckpointDurableExecutionError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
```

### `src/protocol_serde/shape_checkpoint_durable_execution_input.rs`

```diff
--- reference/src/protocol_serde/shape_checkpoint_durable_execution_input.rs
+++ generated/src/protocol_serde/shape_checkpoint_durable_execution_input.rs
@@ -3,11 +3,11 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::checkpoint_durable_execution::CheckpointDurableExecutionInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.checkpoint_token {
-        object.key("CheckpointToken").string(var_1.as_str());
+    if let Some(var_1) = &input.durable_execution_arn {
+        object.key("DurableExecutionArn").string(var_1.as_str());
     }
-    if let Some(var_2) = &input.client_token {
-        object.key("ClientToken").string(var_2.as_str());
+    if let Some(var_2) = &input.checkpoint_token {
+        object.key("CheckpointToken").string(var_2.as_str());
     }
     if let Some(var_3) = &input.updates {
         let mut array_4 = object.key("Updates").start_array();
@@ -21,5 +21,8 @@
         }
         array_4.finish();
     }
+    if let Some(var_7) = &input.client_token {
+        object.key("ClientToken").string(var_7.as_str());
+    }
     Ok(())
 }
```

### `src/protocol_serde/shape_cors.rs`

```diff
--- reference/src/protocol_serde/shape_cors.rs
+++ generated/src/protocol_serde/shape_cors.rs
@@ -1,4 +1,56 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_cors(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::types::Cors,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    if let Some(var_1) = &input.allow_credentials {
+        object.key("AllowCredentials").boolean(*var_1);
+    }
+    if let Some(var_2) = &input.allow_headers {
+        let mut array_3 = object.key("AllowHeaders").start_array();
+        for item_4 in var_2 {
+            {
+                array_3.value().string(item_4.as_str());
+            }
+        }
+        array_3.finish();
+    }
+    if let Some(var_5) = &input.allow_methods {
+        let mut array_6 = object.key("AllowMethods").start_array();
+        for item_7 in var_5 {
+            {
+                array_6.value().string(item_7.as_str());
+            }
+        }
+        array_6.finish();
+    }
+    if let Some(var_8) = &input.allow_origins {
+        let mut array_9 = object.key("AllowOrigins").start_array();
+        for item_10 in var_8 {
+            {
+                array_9.value().string(item_10.as_str());
+            }
+        }
+        array_9.finish();
+    }
+    if let Some(var_11) = &input.expose_headers {
+        let mut array_12 = object.key("ExposeHeaders").start_array();
+        for item_13 in var_11 {
+            {
+                array_12.value().string(item_13.as_str());
+            }
+        }
+        array_12.finish();
+    }
+    if let Some(var_14) = &input.max_age {
+        object.key("MaxAge").number(
+            #[allow(clippy::useless_conversion)]
+            ::aws_smithy_types::Number::NegInt((*var_14).into()),
+        );
+    }
+    Ok(())
+}
+
 pub(crate) fn de_cors<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
@@ -69,55 +121,3 @@
         )),
     }
 }
-
-pub fn ser_cors(
-    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
-    input: &super::types::Cors,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.allow_credentials {
-        object.key("AllowCredentials").boolean(*var_1);
-    }
-    if let Some(var_2) = &input.allow_headers {
-        let mut array_3 = object.key("AllowHeaders").start_array();
-        for item_4 in var_2 {
-            {
-                array_3.value().string(item_4.as_str());
-            }
-        }
-        array_3.finish();
-    }
-    if let Some(var_5) = &input.allow_methods {
-        let mut array_6 = object.key("AllowMethods").start_array();
-        for item_7 in var_5 {
-            {
-                array_6.value().string(item_7.as_str());
-            }
-        }
-        array_6.finish();
-    }
-    if let Some(var_8) = &input.allow_origins {
-        let mut array_9 = object.key("AllowOrigins").start_array();
-        for item_10 in var_8 {
-            {
-                array_9.value().string(item_10.as_str());
-            }
-        }
-        array_9.finish();
-    }
-    if let Some(var_11) = &input.expose_headers {
-        let mut array_12 = object.key("ExposeHeaders").start_array();
-        for item_13 in var_11 {
-            {
-                array_12.value().string(item_13.as_str());
-            }
-        }
-        array_12.finish();
-    }
-    if let Some(var_14) = &input.max_age {
-        object.key("MaxAge").number(
-            #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_14).into()),
-        );
-    }
-    Ok(())
-}
```

### `src/protocol_serde/shape_create_alias.rs`

```diff
--- reference/src/protocol_serde/shape_create_alias.rs
+++ generated/src/protocol_serde/shape_create_alias.rs
@@ -103,11 +103,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::create_alias::CreateAliasError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::create_alias::CreateAliasError::unhandled("Failed to parse retryAfterSeconds from header `Retry-After")
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -168,8 +163,8 @@
                                 .transpose()?,
                         );
                     }
-                    "Description" => {
-                        builder = builder.set_description(
+                    "Name" => {
+                        builder = builder.set_name(
                             ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                 .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                 .transpose()?,
@@ -182,13 +177,18 @@
                                 .transpose()?,
                         );
                     }
-                    "Name" => {
-                        builder = builder.set_name(
+                    "Description" => {
+                        builder = builder.set_description(
                             ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                 .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                 .transpose()?,
                         );
                     }
+                    "RoutingConfig" => {
+                        builder = builder.set_routing_config(
+                            super::protocol_serde::shape_alias_routing_configuration::de_alias_routing_configuration(tokens, _value, depth + 1)?,
+                        );
+                    }
                     "RevisionId" => {
                         builder = builder.set_revision_id(
                             ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -196,11 +196,6 @@
                                 .transpose()?,
                         );
                     }
-                    "RoutingConfig" => {
-                        builder = builder.set_routing_config(
-                            super::protocol_serde::shape_alias_routing_configuration::de_alias_routing_configuration(tokens, _value, depth + 1)?,
-                        );
-                    }
                     _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                 }
             }
```

### `src/protocol_serde/shape_create_alias_input.rs`

```diff
--- reference/src/protocol_serde/shape_create_alias_input.rs
+++ generated/src/protocol_serde/shape_create_alias_input.rs
@@ -3,20 +3,23 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::create_alias::CreateAliasInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.description {
-        object.key("Description").string(var_1.as_str());
+    if let Some(var_1) = &input.function_name {
+        object.key("FunctionName").string(var_1.as_str());
     }
-    if let Some(var_2) = &input.function_version {
-        object.key("FunctionVersion").string(var_2.as_str());
+    if let Some(var_2) = &input.name {
+        object.key("Name").string(var_2.as_str());
     }
-    if let Some(var_3) = &input.name {
-        object.key("Name").string(var_3.as_str());
+    if let Some(var_3) = &input.function_version {
+        object.key("FunctionVersion").string(var_3.as_str());
+    }
+    if let Some(var_4) = &input.description {
+        object.key("Description").string(var_4.as_str());
     }
-    if let Some(var_4) = &input.routing_config {
+    if let Some(var_5) = &input.routing_config {
         #[allow(unused_mut)]
-        let mut object_5 = object.key("RoutingConfig").start_object();
-        super::protocol_serde::shape_alias_routing_configuration::ser_alias_routing_configuration(&mut object_5, var_4)?;
-        object_5.finish();
+        let mut object_6 = object.key("RoutingConfig").start_object();
+        super::protocol_serde::shape_alias_routing_configuration::ser_alias_routing_configuration(&mut object_6, var_5)?;
+        object_6.finish();
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_create_capacity_provider.rs`

```diff
--- reference/src/protocol_serde/shape_create_capacity_provider.rs
+++ generated/src/protocol_serde/shape_create_capacity_provider.rs
@@ -97,13 +97,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::create_capacity_provider::CreateCapacityProviderError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::create_capacity_provider::CreateCapacityProviderError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
```

### `src/protocol_serde/shape_create_capacity_provider_input.rs`

```diff
--- reference/src/protocol_serde/shape_create_capacity_provider_input.rs
+++ generated/src/protocol_serde/shape_create_capacity_provider_input.rs
@@ -6,32 +6,32 @@
     if let Some(var_1) = &input.capacity_provider_name {
         object.key("CapacityProviderName").string(var_1.as_str());
     }
-    if let Some(var_2) = &input.capacity_provider_scaling_config {
+    if let Some(var_2) = &input.vpc_config {
         #[allow(unused_mut)]
-        let mut object_3 = object.key("CapacityProviderScalingConfig").start_object();
-        super::protocol_serde::shape_capacity_provider_scaling_config::ser_capacity_provider_scaling_config(&mut object_3, var_2)?;
+        let mut object_3 = object.key("VpcConfig").start_object();
+        super::protocol_serde::shape_capacity_provider_vpc_config::ser_capacity_provider_vpc_config(&mut object_3, var_2)?;
         object_3.finish();
     }
-    if let Some(var_4) = &input.instance_requirements {
+    if let Some(var_4) = &input.permissions_config {
         #[allow(unused_mut)]
-        let mut object_5 = object.key("InstanceRequirements").start_object();
-        super::protocol_serde::shape_instance_requirements::ser_instance_requirements(&mut object_5, var_4)?;
+        let mut object_5 = object.key("PermissionsConfig").start_object();
+        super::protocol_serde::shape_capacity_provider_permissions_config::ser_capacity_provider_permissions_config(&mut object_5, var_4)?;
         object_5.finish();
     }
-    if let Some(var_6) = &input.kms_key_arn {
-        object.key("KmsKeyArn").string(var_6.as_str());
+    if let Some(var_6) = &input.instance_requirements {
+        #[allow(unused_mut)]
+        let mut object_7 = object.key("InstanceRequirements").start_object();
+        super::protocol_serde::shape_instance_requirements::ser_instance_requirements(&mut object_7, var_6)?;
+        object_7.finish();
     }
-    if let Some(var_7) = &input.permissions_config {
+    if let Some(var_8) = &input.capacity_provider_scaling_config {
         #[allow(unused_mut)]
-        let mut object_8 = object.key("PermissionsConfig").start_object();
-        super::protocol_serde::shape_capacity_provider_permissions_config::ser_capacity_provider_permissions_config(&mut object_8, var_7)?;
-        object_8.finish();
+        let mut object_9 = object.key("CapacityProviderScalingConfig").start_object();
+        super::protocol_serde::shape_capacity_provider_scaling_config::ser_capacity_provider_scaling_config(&mut object_9, var_8)?;
+        object_9.finish();
     }
-    if let Some(var_9) = &input.propagate_tags {
-        #[allow(unused_mut)]
-        let mut object_10 = object.key("PropagateTags").start_object();
-        super::protocol_serde::shape_propagate_tags::ser_propagate_tags(&mut object_10, var_9)?;
-        object_10.finish();
+    if let Some(var_10) = &input.kms_key_arn {
+        object.key("KmsKeyArn").string(var_10.as_str());
     }
     if let Some(var_11) = &input.tags {
         #[allow(unused_mut)]
@@ -43,16 +43,16 @@
         }
         object_12.finish();
     }
-    if let Some(var_15) = &input.telemetry_config {
+    if let Some(var_15) = &input.propagate_tags {
         #[allow(unused_mut)]
-        let mut object_16 = object.key("TelemetryConfig").start_object();
-        super::protocol_serde::shape_capacity_provider_telemetry_config::ser_capacity_provider_telemetry_config(&mut object_16, var_15)?;
+        let mut object_16 = object.key("PropagateTags").start_object();
+        super::protocol_serde::shape_propagate_tags::ser_propagate_tags(&mut object_16, var_15)?;
         object_16.finish();
     }
-    if let Some(var_17) = &input.vpc_config {
+    if let Some(var_17) = &input.telemetry_config {
         #[allow(unused_mut)]
-        let mut object_18 = object.key("VpcConfig").start_object();
-        super::protocol_serde::shape_capacity_provider_vpc_config::ser_capacity_provider_vpc_config(&mut object_18, var_17)?;
+        let mut object_18 = object.key("TelemetryConfig").start_object();
+        super::protocol_serde::shape_capacity_provider_telemetry_config::ser_capacity_provider_telemetry_config(&mut object_18, var_17)?;
         object_18.finish();
     }
     Ok(())
```

### `src/protocol_serde/shape_create_code_signing_config_input.rs`

```diff
--- reference/src/protocol_serde/shape_create_code_signing_config_input.rs
+++ generated/src/protocol_serde/shape_create_code_signing_config_input.rs
@@ -3,20 +3,20 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::create_code_signing_config::CreateCodeSigningConfigInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.allowed_publishers {
+    if let Some(var_1) = &input.description {
+        object.key("Description").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.allowed_publishers {
         #[allow(unused_mut)]
-        let mut object_2 = object.key("AllowedPublishers").start_object();
-        super::protocol_serde::shape_allowed_publishers::ser_allowed_publishers(&mut object_2, var_1)?;
-        object_2.finish();
+        let mut object_3 = object.key("AllowedPublishers").start_object();
+        super::protocol_serde::shape_allowed_publishers::ser_allowed_publishers(&mut object_3, var_2)?;
+        object_3.finish();
     }
-    if let Some(var_3) = &input.code_signing_policies {
+    if let Some(var_4) = &input.code_signing_policies {
         #[allow(unused_mut)]
-        let mut object_4 = object.key("CodeSigningPolicies").start_object();
-        super::protocol_serde::shape_code_signing_policies::ser_code_signing_policies(&mut object_4, var_3)?;
-        object_4.finish();
-    }
-    if let Some(var_5) = &input.description {
-        object.key("Description").string(var_5.as_str());
+        let mut object_5 = object.key("CodeSigningPolicies").start_object();
+        super::protocol_serde::shape_code_signing_policies::ser_code_signing_policies(&mut object_5, var_4)?;
+        object_5.finish();
     }
     if let Some(var_6) = &input.tags {
         #[allow(unused_mut)]
```

### `src/protocol_serde/shape_create_event_source_mapping.rs`

```diff
--- reference/src/protocol_serde/shape_create_event_source_mapping.rs
+++ generated/src/protocol_serde/shape_create_event_source_mapping.rs
@@ -96,13 +96,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::create_event_source_mapping::CreateEventSourceMappingError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::create_event_source_mapping::CreateEventSourceMappingError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -160,15 +153,26 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "AmazonManagedKafkaEventSourceConfig" => {
-                    builder = builder.set_amazon_managed_kafka_event_source_config(
-                        super::protocol_serde::shape_amazon_managed_kafka_event_source_config::de_amazon_managed_kafka_event_source_config(
-                            tokens,
-                            _value,
-                            depth + 1,
-                        )?,
+                "UUID" => {
+                    builder = builder.set_uuid(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
+                }
+                "StartingPosition" => {
+                    builder = builder.set_starting_position(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| super::types::EventSourcePosition::from(u.as_ref())))
+                            .transpose()?,
                     );
                 }
+                "StartingPositionTimestamp" => {
+                    builder = builder.set_starting_position_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
+                        tokens.next(),
+                        ::aws_smithy_types::date_time::Format::EpochSeconds,
+                    )?);
+                }
                 "BatchSize" => {
                     builder = builder.set_batch_size(
                         ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
@@ -176,30 +180,22 @@
                             .transpose()?,
                     );
                 }
-                "BisectBatchOnFunctionError" => {
-                    builder = builder.set_bisect_batch_on_function_error(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
-                }
-                "DestinationConfig" => {
-                    builder = builder.set_destination_config(super::protocol_serde::shape_destination_config::de_destination_config(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
-                }
-                "DocumentDBEventSourceConfig" => {
-                    builder = builder.set_document_db_event_source_config(
-                        super::protocol_serde::shape_document_db_event_source_config::de_document_db_event_source_config(tokens, _value, depth + 1)?,
+                "MaximumBatchingWindowInSeconds" => {
+                    builder = builder.set_maximum_batching_window_in_seconds(
+                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                            .map(i32::try_from)
+                            .transpose()?,
                     );
                 }
-                "EventSourceArn" => {
-                    builder = builder.set_event_source_arn(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                "ParallelizationFactor" => {
+                    builder = builder.set_parallelization_factor(
+                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                            .map(i32::try_from)
                             .transpose()?,
                     );
                 }
-                "EventSourceMappingArn" => {
-                    builder = builder.set_event_source_mapping_arn(
+                "EventSourceArn" => {
+                    builder = builder.set_event_source_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
@@ -219,20 +215,36 @@
                         depth + 1,
                     )?);
                 }
-                "FunctionArn" => {
-                    builder = builder.set_function_arn(
+                "KMSKeyArn" => {
+                    builder = builder.set_kms_key_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "FunctionResponseTypes" => {
-                    builder = builder.set_function_response_types(
-                        super::protocol_serde::shape_function_response_type_list::de_function_response_type_list(tokens, _value, depth + 1)?,
+                "MetricsConfig" => {
+                    builder = builder.set_metrics_config(
+                        super::protocol_serde::shape_event_source_mapping_metrics_config::de_event_source_mapping_metrics_config(
+                            tokens,
+                            _value,
+                            depth + 1,
+                        )?,
                     );
                 }
-                "KMSKeyArn" => {
-                    builder = builder.set_kms_key_arn(
+                "LoggingConfig" => {
+                    builder = builder.set_logging_config(
+                        super::protocol_serde::shape_event_source_mapping_logging_config::de_event_source_mapping_logging_config(
+                            tokens,
+                            _value,
+                            depth + 1,
+                        )?,
+                    );
+                }
+                "ScalingConfig" => {
+                    builder = builder.set_scaling_config(super::protocol_serde::shape_scaling_config::de_scaling_config(tokens, _value, depth + 1)?);
+                }
+                "FunctionArn" => {
+                    builder = builder.set_function_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
@@ -251,22 +263,43 @@
                             .transpose()?,
                     );
                 }
-                "LoggingConfig" => {
-                    builder = builder.set_logging_config(
-                        super::protocol_serde::shape_event_source_mapping_logging_config::de_event_source_mapping_logging_config(
-                            tokens,
-                            _value,
-                            depth + 1,
-                        )?,
+                "State" => {
+                    builder = builder.set_state(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
                     );
                 }
-                "MaximumBatchingWindowInSeconds" => {
-                    builder = builder.set_maximum_batching_window_in_seconds(
-                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
-                            .map(i32::try_from)
+                "StateTransitionReason" => {
+                    builder = builder.set_state_transition_reason(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
+                "DestinationConfig" => {
+                    builder = builder.set_destination_config(super::protocol_serde::shape_destination_config::de_destination_config(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
+                "Topics" => {
+                    builder = builder.set_topics(super::protocol_serde::shape_topics::de_topics(tokens, _value, depth + 1)?);
+                }
+                "Queues" => {
+                    builder = builder.set_queues(super::protocol_serde::shape_queues::de_queues(tokens, _value, depth + 1)?);
+                }
+                "SourceAccessConfigurations" => {
+                    builder = builder.set_source_access_configurations(
+                        super::protocol_serde::shape_source_access_configurations::de_source_access_configurations(tokens, _value, depth + 1)?,
+                    );
+                }
+                "SelfManagedEventSource" => {
+                    builder = builder.set_self_managed_event_source(
+                        super::protocol_serde::shape_self_managed_event_source::de_self_managed_event_source(tokens, _value, depth + 1)?,
+                    );
+                }
                 "MaximumRecordAgeInSeconds" => {
                     builder = builder.set_maximum_record_age_in_seconds(
                         ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
@@ -274,6 +307,9 @@
                             .transpose()?,
                     );
                 }
+                "BisectBatchOnFunctionError" => {
+                    builder = builder.set_bisect_batch_on_function_error(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
+                }
                 "MaximumRetryAttempts" => {
                     builder = builder.set_maximum_retry_attempts(
                         ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
@@ -281,36 +317,25 @@
                             .transpose()?,
                     );
                 }
-                "MetricsConfig" => {
-                    builder = builder.set_metrics_config(
-                        super::protocol_serde::shape_event_source_mapping_metrics_config::de_event_source_mapping_metrics_config(
-                            tokens,
-                            _value,
-                            depth + 1,
-                        )?,
-                    );
-                }
-                "ParallelizationFactor" => {
-                    builder = builder.set_parallelization_factor(
+                "TumblingWindowInSeconds" => {
+                    builder = builder.set_tumbling_window_in_seconds(
                         ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                             .map(i32::try_from)
                             .transpose()?,
                     );
                 }
-                "ProvisionedPollerConfig" => {
-                    builder = builder.set_provisioned_poller_config(
-                        super::protocol_serde::shape_provisioned_poller_config::de_provisioned_poller_config(tokens, _value, depth + 1)?,
+                "FunctionResponseTypes" => {
+                    builder = builder.set_function_response_types(
+                        super::protocol_serde::shape_function_response_type_list::de_function_response_type_list(tokens, _value, depth + 1)?,
                     );
                 }
-                "Queues" => {
-                    builder = builder.set_queues(super::protocol_serde::shape_queues::de_queues(tokens, _value, depth + 1)?);
-                }
-                "ScalingConfig" => {
-                    builder = builder.set_scaling_config(super::protocol_serde::shape_scaling_config::de_scaling_config(tokens, _value, depth + 1)?);
-                }
-                "SelfManagedEventSource" => {
-                    builder = builder.set_self_managed_event_source(
-                        super::protocol_serde::shape_self_managed_event_source::de_self_managed_event_source(tokens, _value, depth + 1)?,
+                "AmazonManagedKafkaEventSourceConfig" => {
+                    builder = builder.set_amazon_managed_kafka_event_source_config(
+                        super::protocol_serde::shape_amazon_managed_kafka_event_source_config::de_amazon_managed_kafka_event_source_config(
+                            tokens,
+                            _value,
+                            depth + 1,
+                        )?,
                     );
                 }
                 "SelfManagedKafkaEventSourceConfig" => {
@@ -322,53 +347,21 @@
                         )?,
                     );
                 }
-                "SourceAccessConfigurations" => {
-                    builder = builder.set_source_access_configurations(
-                        super::protocol_serde::shape_source_access_configurations::de_source_access_configurations(tokens, _value, depth + 1)?,
-                    );
-                }
-                "StartingPosition" => {
-                    builder = builder.set_starting_position(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| super::types::EventSourcePosition::from(u.as_ref())))
-                            .transpose()?,
-                    );
-                }
-                "StartingPositionTimestamp" => {
-                    builder = builder.set_starting_position_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
-                        tokens.next(),
-                        ::aws_smithy_types::date_time::Format::EpochSeconds,
-                    )?);
-                }
-                "State" => {
-                    builder = builder.set_state(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
+                "DocumentDBEventSourceConfig" => {
+                    builder = builder.set_document_db_event_source_config(
+                        super::protocol_serde::shape_document_db_event_source_config::de_document_db_event_source_config(tokens, _value, depth + 1)?,
                     );
                 }
-                "StateTransitionReason" => {
-                    builder = builder.set_state_transition_reason(
+                "EventSourceMappingArn" => {
+                    builder = builder.set_event_source_mapping_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "Topics" => {
-                    builder = builder.set_topics(super::protocol_serde::shape_topics::de_topics(tokens, _value, depth + 1)?);
-                }
-                "TumblingWindowInSeconds" => {
-                    builder = builder.set_tumbling_window_in_seconds(
-                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
-                            .map(i32::try_from)
-                            .transpose()?,
-                    );
-                }
-                "UUID" => {
-                    builder = builder.set_uuid(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
+                "ProvisionedPollerConfig" => {
+                    builder = builder.set_provisioned_poller_config(
+                        super::protocol_serde::shape_provisioned_poller_config::de_provisioned_poller_config(tokens, _value, depth + 1)?,
                     );
                 }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
```

### `src/protocol_serde/shape_create_event_source_mapping_input.rs`

```diff
--- reference/src/protocol_serde/shape_create_event_source_mapping_input.rs
+++ generated/src/protocol_serde/shape_create_event_source_mapping_input.rs
@@ -3,77 +3,82 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::create_event_source_mapping::CreateEventSourceMappingInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.amazon_managed_kafka_event_source_config {
-        #[allow(unused_mut)]
-        let mut object_2 = object.key("AmazonManagedKafkaEventSourceConfig").start_object();
-        super::protocol_serde::shape_amazon_managed_kafka_event_source_config::ser_amazon_managed_kafka_event_source_config(&mut object_2, var_1)?;
-        object_2.finish();
+    if let Some(var_1) = &input.event_source_arn {
+        object.key("EventSourceArn").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.function_name {
+        object.key("FunctionName").string(var_2.as_str());
+    }
+    if let Some(var_3) = &input.enabled {
+        object.key("Enabled").boolean(*var_3);
     }
-    if let Some(var_3) = &input.batch_size {
+    if let Some(var_4) = &input.batch_size {
         object.key("BatchSize").number(
             #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_3).into()),
+            ::aws_smithy_types::Number::NegInt((*var_4).into()),
         );
     }
-    if let Some(var_4) = &input.bisect_batch_on_function_error {
-        object.key("BisectBatchOnFunctionError").boolean(*var_4);
-    }
-    if let Some(var_5) = &input.destination_config {
+    if let Some(var_5) = &input.filter_criteria {
         #[allow(unused_mut)]
-        let mut object_6 = object.key("DestinationConfig").start_object();
-        super::protocol_serde::shape_destination_config::ser_destination_config(&mut object_6, var_5)?;
+        let mut object_6 = object.key("FilterCriteria").start_object();
+        super::protocol_serde::shape_filter_criteria::ser_filter_criteria(&mut object_6, var_5)?;
         object_6.finish();
     }
-    if let Some(var_7) = &input.document_db_event_source_config {
+    if let Some(var_7) = &input.kms_key_arn {
+        object.key("KMSKeyArn").string(var_7.as_str());
+    }
+    if let Some(var_8) = &input.metrics_config {
         #[allow(unused_mut)]
-        let mut object_8 = object.key("DocumentDBEventSourceConfig").start_object();
-        super::protocol_serde::shape_document_db_event_source_config::ser_document_db_event_source_config(&mut object_8, var_7)?;
-        object_8.finish();
+        let mut object_9 = object.key("MetricsConfig").start_object();
+        super::protocol_serde::shape_event_source_mapping_metrics_config::ser_event_source_mapping_metrics_config(&mut object_9, var_8)?;
+        object_9.finish();
     }
-    if let Some(var_9) = &input.enabled {
-        object.key("Enabled").boolean(*var_9);
+    if let Some(var_10) = &input.logging_config {
+        #[allow(unused_mut)]
+        let mut object_11 = object.key("LoggingConfig").start_object();
+        super::protocol_serde::shape_event_source_mapping_logging_config::ser_event_source_mapping_logging_config(&mut object_11, var_10)?;
+        object_11.finish();
     }
-    if let Some(var_10) = &input.event_source_arn {
-        object.key("EventSourceArn").string(var_10.as_str());
-    }
-    if let Some(var_11) = &input.filter_criteria {
+    if let Some(var_12) = &input.scaling_config {
         #[allow(unused_mut)]
-        let mut object_12 = object.key("FilterCriteria").start_object();
-        super::protocol_serde::shape_filter_criteria::ser_filter_criteria(&mut object_12, var_11)?;
-        object_12.finish();
+        let mut object_13 = object.key("ScalingConfig").start_object();
+        super::protocol_serde::shape_scaling_config::ser_scaling_config(&mut object_13, var_12)?;
+        object_13.finish();
     }
-    if let Some(var_13) = &input.function_name {
-        object.key("FunctionName").string(var_13.as_str());
+    if let Some(var_14) = &input.maximum_batching_window_in_seconds {
+        object.key("MaximumBatchingWindowInSeconds").number(
+            #[allow(clippy::useless_conversion)]
+            ::aws_smithy_types::Number::NegInt((*var_14).into()),
+        );
     }
-    if let Some(var_14) = &input.function_response_types {
-        let mut array_15 = object.key("FunctionResponseTypes").start_array();
-        for item_16 in var_14 {
-            {
-                array_15.value().string(item_16.as_str());
-            }
-        }
-        array_15.finish();
+    if let Some(var_15) = &input.parallelization_factor {
+        object.key("ParallelizationFactor").number(
+            #[allow(clippy::useless_conversion)]
+            ::aws_smithy_types::Number::NegInt((*var_15).into()),
+        );
     }
-    if let Some(var_17) = &input.kms_key_arn {
-        object.key("KMSKeyArn").string(var_17.as_str());
+    if let Some(var_16) = &input.starting_position {
+        object.key("StartingPosition").string(var_16.as_str());
     }
-    if let Some(var_18) = &input.logging_config {
+    if let Some(var_17) = &input.starting_position_timestamp {
+        object
+            .key("StartingPositionTimestamp")
+            .date_time(var_17, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
+    }
+    if let Some(var_18) = &input.destination_config {
         #[allow(unused_mut)]
-        let mut object_19 = object.key("LoggingConfig").start_object();
-        super::protocol_serde::shape_event_source_mapping_logging_config::ser_event_source_mapping_logging_config(&mut object_19, var_18)?;
+        let mut object_19 = object.key("DestinationConfig").start_object();
+        super::protocol_serde::shape_destination_config::ser_destination_config(&mut object_19, var_18)?;
         object_19.finish();
     }
-    if let Some(var_20) = &input.maximum_batching_window_in_seconds {
-        object.key("MaximumBatchingWindowInSeconds").number(
+    if let Some(var_20) = &input.maximum_record_age_in_seconds {
+        object.key("MaximumRecordAgeInSeconds").number(
             #[allow(clippy::useless_conversion)]
             ::aws_smithy_types::Number::NegInt((*var_20).into()),
         );
     }
-    if let Some(var_21) = &input.maximum_record_age_in_seconds {
-        object.key("MaximumRecordAgeInSeconds").number(
-            #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_21).into()),
-        );
+    if let Some(var_21) = &input.bisect_batch_on_function_error {
+        object.key("BisectBatchOnFunctionError").boolean(*var_21);
     }
     if let Some(var_22) = &input.maximum_retry_attempts {
         object.key("MaximumRetryAttempts").number(
@@ -81,26 +86,24 @@
             ::aws_smithy_types::Number::NegInt((*var_22).into()),
         );
     }
-    if let Some(var_23) = &input.metrics_config {
+    if let Some(var_23) = &input.tags {
         #[allow(unused_mut)]
-        let mut object_24 = object.key("MetricsConfig").start_object();
-        super::protocol_serde::shape_event_source_mapping_metrics_config::ser_event_source_mapping_metrics_config(&mut object_24, var_23)?;
+        let mut object_24 = object.key("Tags").start_object();
+        for (key_25, value_26) in var_23 {
+            {
+                object_24.key(key_25.as_str()).string(value_26.as_str());
+            }
+        }
         object_24.finish();
     }
-    if let Some(var_25) = &input.parallelization_factor {
-        object.key("ParallelizationFactor").number(
+    if let Some(var_27) = &input.tumbling_window_in_seconds {
+        object.key("TumblingWindowInSeconds").number(
             #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_25).into()),
+            ::aws_smithy_types::Number::NegInt((*var_27).into()),
         );
     }
-    if let Some(var_26) = &input.provisioned_poller_config {
-        #[allow(unused_mut)]
-        let mut object_27 = object.key("ProvisionedPollerConfig").start_object();
-        super::protocol_serde::shape_provisioned_poller_config::ser_provisioned_poller_config(&mut object_27, var_26)?;
-        object_27.finish();
-    }
-    if let Some(var_28) = &input.queues {
-        let mut array_29 = object.key("Queues").start_array();
+    if let Some(var_28) = &input.topics {
+        let mut array_29 = object.key("Topics").start_array();
         for item_30 in var_28 {
             {
                 array_29.value().string(item_30.as_str());
@@ -108,68 +111,65 @@
         }
         array_29.finish();
     }
-    if let Some(var_31) = &input.scaling_config {
-        #[allow(unused_mut)]
-        let mut object_32 = object.key("ScalingConfig").start_object();
-        super::protocol_serde::shape_scaling_config::ser_scaling_config(&mut object_32, var_31)?;
-        object_32.finish();
+    if let Some(var_31) = &input.queues {
+        let mut array_32 = object.key("Queues").start_array();
+        for item_33 in var_31 {
+            {
+                array_32.value().string(item_33.as_str());
+            }
+        }
+        array_32.finish();
     }
-    if let Some(var_33) = &input.self_managed_event_source {
-        #[allow(unused_mut)]
-        let mut object_34 = object.key("SelfManagedEventSource").start_object();
-        super::protocol_serde::shape_self_managed_event_source::ser_self_managed_event_source(&mut object_34, var_33)?;
-        object_34.finish();
-    }
-    if let Some(var_35) = &input.self_managed_kafka_event_source_config {
-        #[allow(unused_mut)]
-        let mut object_36 = object.key("SelfManagedKafkaEventSourceConfig").start_object();
-        super::protocol_serde::shape_self_managed_kafka_event_source_config::ser_self_managed_kafka_event_source_config(&mut object_36, var_35)?;
-        object_36.finish();
-    }
-    if let Some(var_37) = &input.source_access_configurations {
-        let mut array_38 = object.key("SourceAccessConfigurations").start_array();
-        for item_39 in var_37 {
+    if let Some(var_34) = &input.source_access_configurations {
+        let mut array_35 = object.key("SourceAccessConfigurations").start_array();
+        for item_36 in var_34 {
             {
                 #[allow(unused_mut)]
-                let mut object_40 = array_38.value().start_object();
-                super::protocol_serde::shape_source_access_configuration::ser_source_access_configuration(&mut object_40, item_39)?;
-                object_40.finish();
+                let mut object_37 = array_35.value().start_object();
+                super::protocol_serde::shape_source_access_configuration::ser_source_access_configuration(&mut object_37, item_36)?;
+                object_37.finish();
             }
         }
-        array_38.finish();
-    }
-    if let Some(var_41) = &input.starting_position {
-        object.key("StartingPosition").string(var_41.as_str());
+        array_35.finish();
     }
-    if let Some(var_42) = &input.starting_position_timestamp {
-        object
-            .key("StartingPositionTimestamp")
-            .date_time(var_42, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
+    if let Some(var_38) = &input.self_managed_event_source {
+        #[allow(unused_mut)]
+        let mut object_39 = object.key("SelfManagedEventSource").start_object();
+        super::protocol_serde::shape_self_managed_event_source::ser_self_managed_event_source(&mut object_39, var_38)?;
+        object_39.finish();
     }
-    if let Some(var_43) = &input.tags {
-        #[allow(unused_mut)]
-        let mut object_44 = object.key("Tags").start_object();
-        for (key_45, value_46) in var_43 {
+    if let Some(var_40) = &input.function_response_types {
+        let mut array_41 = object.key("FunctionResponseTypes").start_array();
+        for item_42 in var_40 {
             {
-                object_44.key(key_45.as_str()).string(value_46.as_str());
+                array_41.value().string(item_42.as_str());
             }
         }
+        array_41.finish();
+    }
+    if let Some(var_43) = &input.amazon_managed_kafka_event_source_config {
+        #[allow(unused_mut)]
+        let mut object_44 = object.key("AmazonManagedKafkaEventSourceConfig").start_object();
+        super::protocol_serde::shape_amazon_managed_kafka_event_source_config::ser_amazon_managed_kafka_event_source_config(&mut object_44, var_43)?;
         object_44.finish();
     }
-    if let Some(var_47) = &input.topics {
-        let mut array_48 = object.key("Topics").start_array();
-        for item_49 in var_47 {
-            {
-                array_48.value().string(item_49.as_str());
-            }
-        }
-        array_48.finish();
+    if let Some(var_45) = &input.self_managed_kafka_event_source_config {
+        #[allow(unused_mut)]
+        let mut object_46 = object.key("SelfManagedKafkaEventSourceConfig").start_object();
+        super::protocol_serde::shape_self_managed_kafka_event_source_config::ser_self_managed_kafka_event_source_config(&mut object_46, var_45)?;
+        object_46.finish();
     }
-    if let Some(var_50) = &input.tumbling_window_in_seconds {
-        object.key("TumblingWindowInSeconds").number(
-            #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_50).into()),
-        );
+    if let Some(var_47) = &input.document_db_event_source_config {
+        #[allow(unused_mut)]
+        let mut object_48 = object.key("DocumentDBEventSourceConfig").start_object();
+        super::protocol_serde::shape_document_db_event_source_config::ser_document_db_event_source_config(&mut object_48, var_47)?;
+        object_48.finish();
+    }
+    if let Some(var_49) = &input.provisioned_poller_config {
+        #[allow(unused_mut)]
+        let mut object_50 = object.key("ProvisionedPollerConfig").start_object();
+        super::protocol_serde::shape_provisioned_poller_config::ser_provisioned_poller_config(&mut object_50, var_49)?;
+        object_50.finish();
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_create_function.rs`

```diff
--- reference/src/protocol_serde/shape_create_function.rs
+++ generated/src/protocol_serde/shape_create_function.rs
@@ -173,13 +173,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::create_function::CreateFunctionError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::create_function::CreateFunctionError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -234,20 +227,36 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "Architectures" => {
-                    builder = builder.set_architectures(super::protocol_serde::shape_architectures_list::de_architectures_list(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
+                "FunctionName" => {
+                    builder = builder.set_function_name(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
+                }
+                "FunctionArn" => {
+                    builder = builder.set_function_arn(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
+                }
+                "Runtime" => {
+                    builder = builder.set_runtime(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| super::types::Runtime::from(u.as_ref())))
+                            .transpose()?,
+                    );
                 }
-                "CapacityProviderConfig" => {
-                    builder = builder.set_capacity_provider_config(
-                        super::protocol_serde::shape_capacity_provider_config::de_capacity_provider_config(tokens, _value, depth + 1)?,
+                "Role" => {
+                    builder = builder.set_role(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
                     );
                 }
-                "CodeSha256" => {
-                    builder = builder.set_code_sha256(
+                "Handler" => {
+                    builder = builder.set_handler(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
@@ -260,93 +269,125 @@
                             .transpose()?,
                     );
                 }
-                "ConfigSha256" => {
-                    builder = builder.set_config_sha256(
+                "Description" => {
+                    builder = builder.set_description(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "DeadLetterConfig" => {
-                    builder = builder.set_dead_letter_config(super::protocol_serde::shape_dead_letter_config::de_dead_letter_config(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
+                "Timeout" => {
+                    builder = builder.set_timeout(
+                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                            .map(i32::try_from)
+                            .transpose()?,
+                    );
                 }
-                "Description" => {
-                    builder = builder.set_description(
+                "MemorySize" => {
+                    builder = builder.set_memory_size(
+                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                            .map(i32::try_from)
+                            .transpose()?,
+                    );
+                }
+                "LastModified" => {
+                    builder = builder.set_last_modified(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "DurableConfig" => {
-                    builder = builder.set_durable_config(super::protocol_serde::shape_durable_config::de_durable_config(tokens, _value, depth + 1)?);
+                "CodeSha256" => {
+                    builder = builder.set_code_sha256(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
                 }
-                "Environment" => {
-                    builder = builder.set_environment(super::protocol_serde::shape_environment_response::de_environment_response(
+                "Version" => {
+                    builder = builder.set_version(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
+                }
+                "VpcConfig" => {
+                    builder = builder.set_vpc_config(super::protocol_serde::shape_vpc_config_response::de_vpc_config_response(
                         tokens,
                         _value,
                         depth + 1,
                     )?);
                 }
-                "EphemeralStorage" => {
-                    builder = builder.set_ephemeral_storage(super::protocol_serde::shape_ephemeral_storage::de_ephemeral_storage(
+                "DeadLetterConfig" => {
+                    builder = builder.set_dead_letter_config(super::protocol_serde::shape_dead_letter_config::de_dead_letter_config(
                         tokens,
                         _value,
                         depth + 1,
                     )?);
                 }
-                "FileSystemConfigs" => {
-                    builder = builder.set_file_system_configs(super::protocol_serde::shape_file_system_config_list::de_file_system_config_list(
+                "Environment" => {
+                    builder = builder.set_environment(super::protocol_serde::shape_environment_response::de_environment_response(
                         tokens,
                         _value,
                         depth + 1,
                     )?);
                 }
-                "FunctionArn" => {
-                    builder = builder.set_function_arn(
+                "KMSKeyArn" => {
+                    builder = builder.set_kms_key_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "FunctionName" => {
-                    builder = builder.set_function_name(
+                "TracingConfig" => {
+                    builder = builder.set_tracing_config(super::protocol_serde::shape_tracing_config_response::de_tracing_config_response(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
+                "MasterArn" => {
+                    builder = builder.set_master_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "Handler" => {
-                    builder = builder.set_handler(
+                "RevisionId" => {
+                    builder = builder.set_revision_id(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "ImageConfigResponse" => {
-                    builder = builder.set_image_config_response(super::protocol_serde::shape_image_config_response::de_image_config_response(
+                "Layers" => {
+                    builder = builder.set_layers(super::protocol_serde::shape_layers_reference_list::de_layers_reference_list(
                         tokens,
                         _value,
                         depth + 1,
                     )?);
                 }
-                "KMSKeyArn" => {
-                    builder = builder.set_kms_key_arn(
+                "State" => {
+                    builder = builder.set_state(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .map(|s| s.to_unescaped().map(|u| super::types::State::from(u.as_ref())))
                             .transpose()?,
                     );
                 }
-                "LastModified" => {
-                    builder = builder.set_last_modified(
+                "StateReason" => {
+                    builder = builder.set_state_reason(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
+                "StateReasonCode" => {
+                    builder = builder.set_state_reason_code(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| super::types::StateReasonCode::from(u.as_ref())))
+                            .transpose()?,
+                    );
+                }
                 "LastUpdateStatus" => {
                     builder = builder.set_last_update_status(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -368,27 +409,24 @@
                             .transpose()?,
                     );
                 }
-                "Layers" => {
-                    builder = builder.set_layers(super::protocol_serde::shape_layers_reference_list::de_layers_reference_list(
+                "FileSystemConfigs" => {
+                    builder = builder.set_file_system_configs(super::protocol_serde::shape_file_system_config_list::de_file_system_config_list(
                         tokens,
                         _value,
                         depth + 1,
                     )?);
                 }
-                "LoggingConfig" => {
-                    builder = builder.set_logging_config(super::protocol_serde::shape_logging_config::de_logging_config(tokens, _value, depth + 1)?);
-                }
-                "MasterArn" => {
-                    builder = builder.set_master_arn(
+                "SigningProfileVersionArn" => {
+                    builder = builder.set_signing_profile_version_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "MemorySize" => {
-                    builder = builder.set_memory_size(
-                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
-                            .map(i32::try_from)
+                "SigningJobArn" => {
+                    builder = builder.set_signing_job_arn(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
@@ -399,48 +437,27 @@
                             .transpose()?,
                     );
                 }
-                "RevisionId" => {
-                    builder = builder.set_revision_id(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
+                "ImageConfigResponse" => {
+                    builder = builder.set_image_config_response(super::protocol_serde::shape_image_config_response::de_image_config_response(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
                 }
-                "Role" => {
-                    builder = builder.set_role(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
+                "Architectures" => {
+                    builder = builder.set_architectures(super::protocol_serde::shape_architectures_list::de_architectures_list(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
                 }
-                "Runtime" => {
-                    builder = builder.set_runtime(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| super::types::Runtime::from(u.as_ref())))
-                            .transpose()?,
-                    );
-                }
-                "RuntimeVersionConfig" => {
-                    builder = builder.set_runtime_version_config(super::protocol_serde::shape_runtime_version_config::de_runtime_version_config(
+                "EphemeralStorage" => {
+                    builder = builder.set_ephemeral_storage(super::protocol_serde::shape_ephemeral_storage::de_ephemeral_storage(
                         tokens,
                         _value,
                         depth + 1,
                     )?);
                 }
-                "SigningJobArn" => {
-                    builder = builder.set_signing_job_arn(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
-                }
-                "SigningProfileVersionArn" => {
-                    builder = builder.set_signing_profile_version_arn(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
-                }
                 "SnapStart" => {
                     builder = builder.set_snap_start(super::protocol_serde::shape_snap_start_response::de_snap_start_response(
                         tokens,
@@ -448,57 +465,33 @@
                         depth + 1,
                     )?);
                 }
-                "State" => {
-                    builder = builder.set_state(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| super::types::State::from(u.as_ref())))
-                            .transpose()?,
-                    );
-                }
-                "StateReason" => {
-                    builder = builder.set_state_reason(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
+                "RuntimeVersionConfig" => {
+                    builder = builder.set_runtime_version_config(super::protocol_serde::shape_runtime_version_config::de_runtime_version_config(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
                 }
-                "StateReasonCode" => {
-                    builder = builder.set_state_reason_code(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| super::types::StateReasonCode::from(u.as_ref())))
-                            .transpose()?,
-                    );
+                "LoggingConfig" => {
+                    builder = builder.set_logging_config(super::protocol_serde::shape_logging_config::de_logging_config(tokens, _value, depth + 1)?);
                 }
                 "TenancyConfig" => {
                     builder = builder.set_tenancy_config(super::protocol_serde::shape_tenancy_config::de_tenancy_config(tokens, _value, depth + 1)?);
                 }
-                "Timeout" => {
-                    builder = builder.set_timeout(
-                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
-                            .map(i32::try_from)
-                            .transpose()?,
+                "CapacityProviderConfig" => {
+                    builder = builder.set_capacity_provider_config(
+                        super::protocol_serde::shape_capacity_provider_config::de_capacity_provider_config(tokens, _value, depth + 1)?,
                     );
                 }
-                "TracingConfig" => {
-                    builder = builder.set_tracing_config(super::protocol_serde::shape_tracing_config_response::de_tracing_config_response(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
-                }
-                "Version" => {
-                    builder = builder.set_version(
+                "ConfigSha256" => {
+                    builder = builder.set_config_sha256(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "VpcConfig" => {
-                    builder = builder.set_vpc_config(super::protocol_serde::shape_vpc_config_response::de_vpc_config_response(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
+                "DurableConfig" => {
+                    builder = builder.set_durable_config(super::protocol_serde::shape_durable_config::de_durable_config(tokens, _value, depth + 1)?);
                 }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
```

### `src/protocol_serde/shape_create_function_input.rs`

```diff
--- reference/src/protocol_serde/shape_create_function_input.rs
+++ generated/src/protocol_serde/shape_create_function_input.rs
@@ -3,158 +3,158 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::create_function::CreateFunctionInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.architectures {
-        let mut array_2 = object.key("Architectures").start_array();
-        for item_3 in var_1 {
-            {
-                array_2.value().string(item_3.as_str());
-            }
-        }
-        array_2.finish();
+    if let Some(var_1) = &input.function_name {
+        object.key("FunctionName").string(var_1.as_str());
     }
-    if let Some(var_4) = &input.capacity_provider_config {
+    if let Some(var_2) = &input.runtime {
+        object.key("Runtime").string(var_2.as_str());
+    }
+    if let Some(var_3) = &input.role {
+        object.key("Role").string(var_3.as_str());
+    }
+    if let Some(var_4) = &input.handler {
+        object.key("Handler").string(var_4.as_str());
+    }
+    if let Some(var_5) = &input.code {
         #[allow(unused_mut)]
-        let mut object_5 = object.key("CapacityProviderConfig").start_object();
-        super::protocol_serde::shape_capacity_provider_config::ser_capacity_provider_config(&mut object_5, var_4)?;
-        object_5.finish();
+        let mut object_6 = object.key("Code").start_object();
+        super::protocol_serde::shape_function_code::ser_function_code(&mut object_6, var_5)?;
+        object_6.finish();
     }
-    if let Some(var_6) = &input.code {
+    if let Some(var_7) = &input.description {
+        object.key("Description").string(var_7.as_str());
+    }
+    if let Some(var_8) = &input.timeout {
+        object.key("Timeout").number(
+            #[allow(clippy::useless_conversion)]
+            ::aws_smithy_types::Number::NegInt((*var_8).into()),
+        );
+    }
+    if let Some(var_9) = &input.memory_size {
+        object.key("MemorySize").number(
+            #[allow(clippy::useless_conversion)]
+            ::aws_smithy_types::Number::NegInt((*var_9).into()),
+        );
+    }
+    if let Some(var_10) = &input.publish {
+        object.key("Publish").boolean(*var_10);
+    }
+    if let Some(var_11) = &input.publish_to {
+        object.key("PublishTo").string(var_11.as_str());
+    }
+    if let Some(var_12) = &input.vpc_config {
         #[allow(unused_mut)]
-        let mut object_7 = object.key("Code").start_object();
-        super::protocol_serde::shape_function_code::ser_function_code(&mut object_7, var_6)?;
-        object_7.finish();
+        let mut object_13 = object.key("VpcConfig").start_object();
+        super::protocol_serde::shape_vpc_config::ser_vpc_config(&mut object_13, var_12)?;
+        object_13.finish();
+    }
+    if let Some(var_14) = &input.package_type {
+        object.key("PackageType").string(var_14.as_str());
     }
-    if let Some(var_8) = &input.code_signing_config_arn {
-        object.key("CodeSigningConfigArn").string(var_8.as_str());
+    if let Some(var_15) = &input.dead_letter_config {
+        #[allow(unused_mut)]
+        let mut object_16 = object.key("DeadLetterConfig").start_object();
+        super::protocol_serde::shape_dead_letter_config::ser_dead_letter_config(&mut object_16, var_15)?;
+        object_16.finish();
     }
-    if let Some(var_9) = &input.dead_letter_config {
+    if let Some(var_17) = &input.environment {
         #[allow(unused_mut)]
-        let mut object_10 = object.key("DeadLetterConfig").start_object();
-        super::protocol_serde::shape_dead_letter_config::ser_dead_letter_config(&mut object_10, var_9)?;
-        object_10.finish();
+        let mut object_18 = object.key("Environment").start_object();
+        super::protocol_serde::shape_environment::ser_environment(&mut object_18, var_17)?;
+        object_18.finish();
     }
-    if let Some(var_11) = &input.description {
-        object.key("Description").string(var_11.as_str());
+    if let Some(var_19) = &input.kms_key_arn {
+        object.key("KMSKeyArn").string(var_19.as_str());
     }
-    if let Some(var_12) = &input.durable_config {
+    if let Some(var_20) = &input.tracing_config {
         #[allow(unused_mut)]
-        let mut object_13 = object.key("DurableConfig").start_object();
-        super::protocol_serde::shape_durable_config::ser_durable_config(&mut object_13, var_12)?;
-        object_13.finish();
+        let mut object_21 = object.key("TracingConfig").start_object();
+        super::protocol_serde::shape_tracing_config::ser_tracing_config(&mut object_21, var_20)?;
+        object_21.finish();
     }
-    if let Some(var_14) = &input.environment {
+    if let Some(var_22) = &input.tags {
         #[allow(unused_mut)]
-        let mut object_15 = object.key("Environment").start_object();
-        super::protocol_serde::shape_environment::ser_environment(&mut object_15, var_14)?;
-        object_15.finish();
+        let mut object_23 = object.key("Tags").start_object();
+        for (key_24, value_25) in var_22 {
+            {
+                object_23.key(key_24.as_str()).string(value_25.as_str());
+            }
+        }
+        object_23.finish();
     }
-    if let Some(var_16) = &input.ephemeral_storage {
-        #[allow(unused_mut)]
-        let mut object_17 = object.key("EphemeralStorage").start_object();
-        super::protocol_serde::shape_ephemeral_storage::ser_ephemeral_storage(&mut object_17, var_16)?;
-        object_17.finish();
+    if let Some(var_26) = &input.layers {
+        let mut array_27 = object.key("Layers").start_array();
+        for item_28 in var_26 {
+            {
+                array_27.value().string(item_28.as_str());
+            }
+        }
+        array_27.finish();
     }
-    if let Some(var_18) = &input.file_system_configs {
-        let mut array_19 = object.key("FileSystemConfigs").start_array();
-        for item_20 in var_18 {
+    if let Some(var_29) = &input.file_system_configs {
+        let mut array_30 = object.key("FileSystemConfigs").start_array();
+        for item_31 in var_29 {
             {
                 #[allow(unused_mut)]
-                let mut object_21 = array_19.value().start_object();
-                super::protocol_serde::shape_file_system_config::ser_file_system_config(&mut object_21, item_20)?;
-                object_21.finish();
+                let mut object_32 = array_30.value().start_object();
+                super::protocol_serde::shape_file_system_config::ser_file_system_config(&mut object_32, item_31)?;
+                object_32.finish();
             }
         }
-        array_19.finish();
-    }
-    if let Some(var_22) = &input.function_name {
-        object.key("FunctionName").string(var_22.as_str());
+        array_30.finish();
     }
-    if let Some(var_23) = &input.handler {
-        object.key("Handler").string(var_23.as_str());
+    if let Some(var_33) = &input.code_signing_config_arn {
+        object.key("CodeSigningConfigArn").string(var_33.as_str());
     }
-    if let Some(var_24) = &input.image_config {
+    if let Some(var_34) = &input.image_config {
         #[allow(unused_mut)]
-        let mut object_25 = object.key("ImageConfig").start_object();
-        super::protocol_serde::shape_image_config::ser_image_config(&mut object_25, var_24)?;
-        object_25.finish();
-    }
-    if let Some(var_26) = &input.kms_key_arn {
-        object.key("KMSKeyArn").string(var_26.as_str());
+        let mut object_35 = object.key("ImageConfig").start_object();
+        super::protocol_serde::shape_image_config::ser_image_config(&mut object_35, var_34)?;
+        object_35.finish();
     }
-    if let Some(var_27) = &input.layers {
-        let mut array_28 = object.key("Layers").start_array();
-        for item_29 in var_27 {
+    if let Some(var_36) = &input.architectures {
+        let mut array_37 = object.key("Architectures").start_array();
+        for item_38 in var_36 {
             {
-                array_28.value().string(item_29.as_str());
+                array_37.value().string(item_38.as_str());
             }
         }
-        array_28.finish();
+        array_37.finish();
     }
-    if let Some(var_30) = &input.logging_config {
+    if let Some(var_39) = &input.ephemeral_storage {
         #[allow(unused_mut)]
-        let mut object_31 = object.key("LoggingConfig").start_object();
-        super::protocol_serde::shape_logging_config::ser_logging_config(&mut object_31, var_30)?;
-        object_31.finish();
+        let mut object_40 = object.key("EphemeralStorage").start_object();
+        super::protocol_serde::shape_ephemeral_storage::ser_ephemeral_storage(&mut object_40, var_39)?;
+        object_40.finish();
     }
-    if let Some(var_32) = &input.memory_size {
-        object.key("MemorySize").number(
-            #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_32).into()),
-        );
-    }
-    if let Some(var_33) = &input.package_type {
-        object.key("PackageType").string(var_33.as_str());
-    }
-    if let Some(var_34) = &input.publish {
-        object.key("Publish").boolean(*var_34);
-    }
-    if let Some(var_35) = &input.publish_to {
-        object.key("PublishTo").string(var_35.as_str());
-    }
-    if let Some(var_36) = &input.role {
-        object.key("Role").string(var_36.as_str());
-    }
-    if let Some(var_37) = &input.runtime {
-        object.key("Runtime").string(var_37.as_str());
-    }
-    if let Some(var_38) = &input.snap_start {
+    if let Some(var_41) = &input.snap_start {
         #[allow(unused_mut)]
-        let mut object_39 = object.key("SnapStart").start_object();
-        super::protocol_serde::shape_snap_start::ser_snap_start(&mut object_39, var_38)?;
-        object_39.finish();
+        let mut object_42 = object.key("SnapStart").start_object();
+        super::protocol_serde::shape_snap_start::ser_snap_start(&mut object_42, var_41)?;
+        object_42.finish();
     }
-    if let Some(var_40) = &input.tags {
+    if let Some(var_43) = &input.logging_config {
         #[allow(unused_mut)]
-        let mut object_41 = object.key("Tags").start_object();
-        for (key_42, value_43) in var_40 {
-            {
-                object_41.key(key_42.as_str()).string(value_43.as_str());
-            }
-        }
-        object_41.finish();
+        let mut object_44 = object.key("LoggingConfig").start_object();
+        super::protocol_serde::shape_logging_config::ser_logging_config(&mut object_44, var_43)?;
+        object_44.finish();
     }
-    if let Some(var_44) = &input.tenancy_config {
+    if let Some(var_45) = &input.tenancy_config {
         #[allow(unused_mut)]
-        let mut object_45 = object.key("TenancyConfig").start_object();
-        super::protocol_serde::shape_tenancy_config::ser_tenancy_config(&mut object_45, var_44)?;
-        object_45.finish();
+        let mut object_46 = object.key("TenancyConfig").start_object();
+        super::protocol_serde::shape_tenancy_config::ser_tenancy_config(&mut object_46, var_45)?;
+        object_46.finish();
     }
-    if let Some(var_46) = &input.timeout {
-        object.key("Timeout").number(
-            #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_46).into()),
-        );
-    }
-    if let Some(var_47) = &input.tracing_config {
+    if let Some(var_47) = &input.capacity_provider_config {
         #[allow(unused_mut)]
-        let mut object_48 = object.key("TracingConfig").start_object();
-        super::protocol_serde::shape_tracing_config::ser_tracing_config(&mut object_48, var_47)?;
+        let mut object_48 = object.key("CapacityProviderConfig").start_object();
+        super::protocol_serde::shape_capacity_provider_config::ser_capacity_provider_config(&mut object_48, var_47)?;
         object_48.finish();
     }
-    if let Some(var_49) = &input.vpc_config {
+    if let Some(var_49) = &input.durable_config {
         #[allow(unused_mut)]
-        let mut object_50 = object.key("VpcConfig").start_object();
-        super::protocol_serde::shape_vpc_config::ser_vpc_config(&mut object_50, var_49)?;
+        let mut object_50 = object.key("DurableConfig").start_object();
+        super::protocol_serde::shape_durable_config::ser_durable_config(&mut object_50, var_49)?;
         object_50.finish();
     }
     Ok(())
```

### `src/protocol_serde/shape_create_function_url_config.rs`

```diff
--- reference/src/protocol_serde/shape_create_function_url_config.rs
+++ generated/src/protocol_serde/shape_create_function_url_config.rs
@@ -96,13 +96,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::create_function_url_config::CreateFunctionUrlConfigError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::create_function_url_config::CreateFunctionUrlConfigError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -162,32 +155,32 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "AuthType" => {
-                    builder = builder.set_auth_type(
+                "FunctionUrl" => {
+                    builder = builder.set_function_url(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| super::types::FunctionUrlAuthType::from(u.as_ref())))
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "Cors" => {
-                    builder = builder.set_cors(super::protocol_serde::shape_cors::de_cors(tokens, _value, depth + 1)?);
-                }
-                "CreationTime" => {
-                    builder = builder.set_creation_time(
+                "FunctionArn" => {
+                    builder = builder.set_function_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "FunctionArn" => {
-                    builder = builder.set_function_arn(
+                "AuthType" => {
+                    builder = builder.set_auth_type(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .map(|s| s.to_unescaped().map(|u| super::types::FunctionUrlAuthType::from(u.as_ref())))
                             .transpose()?,
                     );
                 }
-                "FunctionUrl" => {
-                    builder = builder.set_function_url(
+                "Cors" => {
+                    builder = builder.set_cors(super::protocol_serde::shape_cors::de_cors(tokens, _value, depth + 1)?);
+                }
+                "CreationTime" => {
+                    builder = builder.set_creation_time(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
```

### `src/protocol_serde/shape_create_function_url_config_input.rs`

```diff
--- reference/src/protocol_serde/shape_create_function_url_config_input.rs
+++ generated/src/protocol_serde/shape_create_function_url_config_input.rs
@@ -3,17 +3,23 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::create_function_url_config::CreateFunctionUrlConfigInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.auth_type {
-        object.key("AuthType").string(var_1.as_str());
+    if let Some(var_1) = &input.function_name {
+        object.key("FunctionName").string(var_1.as_str());
     }
-    if let Some(var_2) = &input.cors {
+    if let Some(var_2) = &input.qualifier {
+        object.key("Qualifier").string(var_2.as_str());
+    }
+    if let Some(var_3) = &input.auth_type {
+        object.key("AuthType").string(var_3.as_str());
+    }
+    if let Some(var_4) = &input.cors {
         #[allow(unused_mut)]
-        let mut object_3 = object.key("Cors").start_object();
-        super::protocol_serde::shape_cors::ser_cors(&mut object_3, var_2)?;
-        object_3.finish();
+        let mut object_5 = object.key("Cors").start_object();
+        super::protocol_serde::shape_cors::ser_cors(&mut object_5, var_4)?;
+        object_5.finish();
     }
-    if let Some(var_4) = &input.invoke_mode {
-        object.key("InvokeMode").string(var_4.as_str());
+    if let Some(var_6) = &input.invoke_mode {
+        object.key("InvokeMode").string(var_6.as_str());
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_dead_letter_config.rs`

```diff
--- reference/src/protocol_serde/shape_dead_letter_config.rs
+++ generated/src/protocol_serde/shape_dead_letter_config.rs
@@ -1,4 +1,14 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_dead_letter_config(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::types::DeadLetterConfig,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    if let Some(var_1) = &input.target_arn {
+        object.key("TargetArn").string(var_1.as_str());
+    }
+    Ok(())
+}
+
 pub(crate) fn de_dead_letter_config<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
@@ -44,13 +54,3 @@
         )),
     }
 }
-
-pub fn ser_dead_letter_config(
-    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
-    input: &super::types::DeadLetterConfig,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.target_arn {
-        object.key("TargetArn").string(var_1.as_str());
-    }
-    Ok(())
-}
```

### `src/protocol_serde/shape_delete_alias.rs`

```diff
--- reference/src/protocol_serde/shape_delete_alias.rs
+++ generated/src/protocol_serde/shape_delete_alias.rs
@@ -87,11 +87,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::delete_alias::DeleteAliasError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::delete_alias::DeleteAliasError::unhandled("Failed to parse retryAfterSeconds from header `Retry-After")
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -117,3 +112,44 @@
         output.build()
     })
 }
+
+pub fn ser_delete_alias_input(
+    input: &super::operation::delete_alias::DeleteAliasInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_delete_alias_input::ser_delete_alias_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
+pub(crate) fn de_delete_alias(
+    _value: &[u8],
+    mut builder: super::operation::delete_alias::builders::DeleteAliasOutputBuilder,
+) -> ::std::result::Result<super::operation::delete_alias::builders::DeleteAliasOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
+{
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_delete_capacity_provider.rs`

```diff
--- reference/src/protocol_serde/shape_delete_capacity_provider.rs
+++ generated/src/protocol_serde/shape_delete_capacity_provider.rs
@@ -96,13 +96,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::delete_capacity_provider::DeleteCapacityProviderError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::delete_capacity_provider::DeleteCapacityProviderError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -134,6 +127,16 @@
     })
 }

+pub fn ser_delete_capacity_provider_input(
+    input: &super::operation::delete_capacity_provider::DeleteCapacityProviderInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_delete_capacity_provider_input::ser_delete_capacity_provider_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_delete_capacity_provider(
     _value: &[u8],
     mut builder: super::operation::delete_capacity_provider::builders::DeleteCapacityProviderOutputBuilder,
```

### `src/protocol_serde/shape_delete_code_signing_config.rs`

```diff
--- reference/src/protocol_serde/shape_delete_code_signing_config.rs
+++ generated/src/protocol_serde/shape_delete_code_signing_config.rs
@@ -109,3 +109,46 @@
         output.build()
     })
 }
+
+pub fn ser_delete_code_signing_config_input(
+    input: &super::operation::delete_code_signing_config::DeleteCodeSigningConfigInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_delete_code_signing_config_input::ser_delete_code_signing_config_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
+pub(crate) fn de_delete_code_signing_config(
+    _value: &[u8],
+    mut builder: super::operation::delete_code_signing_config::builders::DeleteCodeSigningConfigOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::delete_code_signing_config::builders::DeleteCodeSigningConfigOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_delete_event_source_mapping.rs`

```diff
--- reference/src/protocol_serde/shape_delete_event_source_mapping.rs
+++ generated/src/protocol_serde/shape_delete_event_source_mapping.rs
@@ -111,13 +111,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::delete_event_source_mapping::DeleteEventSourceMappingError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::delete_event_source_mapping::DeleteEventSourceMappingError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -149,6 +142,16 @@
     })
 }

+pub fn ser_delete_event_source_mapping_input(
+    input: &super::operation::delete_event_source_mapping::DeleteEventSourceMappingInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_delete_event_source_mapping_input::ser_delete_event_source_mapping_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_delete_event_source_mapping(
     _value: &[u8],
     mut builder: super::operation::delete_event_source_mapping::builders::DeleteEventSourceMappingOutputBuilder,
@@ -165,15 +168,26 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "AmazonManagedKafkaEventSourceConfig" => {
-                    builder = builder.set_amazon_managed_kafka_event_source_config(
-                        super::protocol_serde::shape_amazon_managed_kafka_event_source_config::de_amazon_managed_kafka_event_source_config(
-                            tokens,
-                            _value,
-                            depth + 1,
-                        )?,
+                "UUID" => {
+                    builder = builder.set_uuid(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
+                }
+                "StartingPosition" => {
+                    builder = builder.set_starting_position(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| super::types::EventSourcePosition::from(u.as_ref())))
+                            .transpose()?,
                     );
                 }
+                "StartingPositionTimestamp" => {
+                    builder = builder.set_starting_position_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
+                        tokens.next(),
+                        ::aws_smithy_types::date_time::Format::EpochSeconds,
+                    )?);
+                }
                 "BatchSize" => {
                     builder = builder.set_batch_size(
                         ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
@@ -181,19 +195,18 @@
                             .transpose()?,
                     );
                 }
-                "BisectBatchOnFunctionError" => {
-                    builder = builder.set_bisect_batch_on_function_error(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
+                "MaximumBatchingWindowInSeconds" => {
+                    builder = builder.set_maximum_batching_window_in_seconds(
+                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                            .map(i32::try_from)
+                            .transpose()?,
+                    );
                 }
-                "DestinationConfig" => {
-                    builder = builder.set_destination_config(super::protocol_serde::shape_destination_config::de_destination_config(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
-                }
-                "DocumentDBEventSourceConfig" => {
-                    builder = builder.set_document_db_event_source_config(
-                        super::protocol_serde::shape_document_db_event_source_config::de_document_db_event_source_config(tokens, _value, depth + 1)?,
+                "ParallelizationFactor" => {
+                    builder = builder.set_parallelization_factor(
+                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                            .map(i32::try_from)
+                            .transpose()?,
                     );
                 }
                 "EventSourceArn" => {
@@ -203,13 +216,6 @@
                             .transpose()?,
                     );
                 }
-                "EventSourceMappingArn" => {
-                    builder = builder.set_event_source_mapping_arn(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
-                }
                 "FilterCriteria" => {
                     builder = builder.set_filter_criteria(super::protocol_serde::shape_filter_criteria::de_filter_criteria(
                         tokens,
@@ -224,20 +230,36 @@
                         depth + 1,
                     )?);
                 }
-                "FunctionArn" => {
-                    builder = builder.set_function_arn(
+                "KMSKeyArn" => {
+                    builder = builder.set_kms_key_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "FunctionResponseTypes" => {
-                    builder = builder.set_function_response_types(
-                        super::protocol_serde::shape_function_response_type_list::de_function_response_type_list(tokens, _value, depth + 1)?,
+                "MetricsConfig" => {
+                    builder = builder.set_metrics_config(
+                        super::protocol_serde::shape_event_source_mapping_metrics_config::de_event_source_mapping_metrics_config(
+                            tokens,
+                            _value,
+                            depth + 1,
+                        )?,
+                    );
+                }
+                "LoggingConfig" => {
+                    builder = builder.set_logging_config(
+                        super::protocol_serde::shape_event_source_mapping_logging_config::de_event_source_mapping_logging_config(
+                            tokens,
+                            _value,
+                            depth + 1,
+                        )?,
                     );
                 }
-                "KMSKeyArn" => {
-                    builder = builder.set_kms_key_arn(
+                "ScalingConfig" => {
+                    builder = builder.set_scaling_config(super::protocol_serde::shape_scaling_config::de_scaling_config(tokens, _value, depth + 1)?);
+                }
+                "FunctionArn" => {
+                    builder = builder.set_function_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
@@ -256,22 +278,43 @@
                             .transpose()?,
                     );
                 }
-                "LoggingConfig" => {
-                    builder = builder.set_logging_config(
-                        super::protocol_serde::shape_event_source_mapping_logging_config::de_event_source_mapping_logging_config(
-                            tokens,
-                            _value,
-                            depth + 1,
-                        )?,
+                "State" => {
+                    builder = builder.set_state(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
                     );
                 }
-                "MaximumBatchingWindowInSeconds" => {
-                    builder = builder.set_maximum_batching_window_in_seconds(
-                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
-                            .map(i32::try_from)
+                "StateTransitionReason" => {
+                    builder = builder.set_state_transition_reason(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
+                "DestinationConfig" => {
+                    builder = builder.set_destination_config(super::protocol_serde::shape_destination_config::de_destination_config(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
+                "Topics" => {
+                    builder = builder.set_topics(super::protocol_serde::shape_topics::de_topics(tokens, _value, depth + 1)?);
+                }
+                "Queues" => {
+                    builder = builder.set_queues(super::protocol_serde::shape_queues::de_queues(tokens, _value, depth + 1)?);
+                }
+                "SourceAccessConfigurations" => {
+                    builder = builder.set_source_access_configurations(
+                        super::protocol_serde::shape_source_access_configurations::de_source_access_configurations(tokens, _value, depth + 1)?,
+                    );
+                }
+                "SelfManagedEventSource" => {
+                    builder = builder.set_self_managed_event_source(
+                        super::protocol_serde::shape_self_managed_event_source::de_self_managed_event_source(tokens, _value, depth + 1)?,
+                    );
+                }
                 "MaximumRecordAgeInSeconds" => {
                     builder = builder.set_maximum_record_age_in_seconds(
                         ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
@@ -279,6 +322,9 @@
                             .transpose()?,
                     );
                 }
+                "BisectBatchOnFunctionError" => {
+                    builder = builder.set_bisect_batch_on_function_error(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
+                }
                 "MaximumRetryAttempts" => {
                     builder = builder.set_maximum_retry_attempts(
                         ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
@@ -286,36 +332,25 @@
                             .transpose()?,
                     );
                 }
-                "MetricsConfig" => {
-                    builder = builder.set_metrics_config(
-                        super::protocol_serde::shape_event_source_mapping_metrics_config::de_event_source_mapping_metrics_config(
-                            tokens,
-                            _value,
-                            depth + 1,
-                        )?,
-                    );
-                }
-                "ParallelizationFactor" => {
-                    builder = builder.set_parallelization_factor(
+                "TumblingWindowInSeconds" => {
+                    builder = builder.set_tumbling_window_in_seconds(
                         ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                             .map(i32::try_from)
                             .transpose()?,
                     );
                 }
-                "ProvisionedPollerConfig" => {
-                    builder = builder.set_provisioned_poller_config(
-                        super::protocol_serde::shape_provisioned_poller_config::de_provisioned_poller_config(tokens, _value, depth + 1)?,
+                "FunctionResponseTypes" => {
+                    builder = builder.set_function_response_types(
+                        super::protocol_serde::shape_function_response_type_list::de_function_response_type_list(tokens, _value, depth + 1)?,
                     );
                 }
-                "Queues" => {
-                    builder = builder.set_queues(super::protocol_serde::shape_queues::de_queues(tokens, _value, depth + 1)?);
-                }
-                "ScalingConfig" => {
-                    builder = builder.set_scaling_config(super::protocol_serde::shape_scaling_config::de_scaling_config(tokens, _value, depth + 1)?);
-                }
-                "SelfManagedEventSource" => {
-                    builder = builder.set_self_managed_event_source(
-                        super::protocol_serde::shape_self_managed_event_source::de_self_managed_event_source(tokens, _value, depth + 1)?,
+                "AmazonManagedKafkaEventSourceConfig" => {
+                    builder = builder.set_amazon_managed_kafka_event_source_config(
+                        super::protocol_serde::shape_amazon_managed_kafka_event_source_config::de_amazon_managed_kafka_event_source_config(
+                            tokens,
+                            _value,
+                            depth + 1,
+                        )?,
                     );
                 }
                 "SelfManagedKafkaEventSourceConfig" => {
@@ -327,53 +362,21 @@
                         )?,
                     );
                 }
-                "SourceAccessConfigurations" => {
-                    builder = builder.set_source_access_configurations(
-                        super::protocol_serde::shape_source_access_configurations::de_source_access_configurations(tokens, _value, depth + 1)?,
-                    );
-                }
-                "StartingPosition" => {
-                    builder = builder.set_starting_position(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| super::types::EventSourcePosition::from(u.as_ref())))
-                            .transpose()?,
-                    );
-                }
-                "StartingPositionTimestamp" => {
-                    builder = builder.set_starting_position_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
-                        tokens.next(),
-                        ::aws_smithy_types::date_time::Format::EpochSeconds,
-                    )?);
-                }
-                "State" => {
-                    builder = builder.set_state(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
+                "DocumentDBEventSourceConfig" => {
+                    builder = builder.set_document_db_event_source_config(
+                        super::protocol_serde::shape_document_db_event_source_config::de_document_db_event_source_config(tokens, _value, depth + 1)?,
                     );
                 }
-                "StateTransitionReason" => {
-                    builder = builder.set_state_transition_reason(
+                "EventSourceMappingArn" => {
+                    builder = builder.set_event_source_mapping_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "Topics" => {
-                    builder = builder.set_topics(super::protocol_serde::shape_topics::de_topics(tokens, _value, depth + 1)?);
-                }
-                "TumblingWindowInSeconds" => {
-                    builder = builder.set_tumbling_window_in_seconds(
-                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
-                            .map(i32::try_from)
-                            .transpose()?,
-                    );
-                }
-                "UUID" => {
-                    builder = builder.set_uuid(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
+                "ProvisionedPollerConfig" => {
+                    builder = builder.set_provisioned_poller_config(
+                        super::protocol_serde::shape_provisioned_poller_config::de_provisioned_poller_config(tokens, _value, depth + 1)?,
                     );
                 }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
```

### `src/protocol_serde/shape_delete_function.rs`

```diff
--- reference/src/protocol_serde/shape_delete_function.rs
+++ generated/src/protocol_serde/shape_delete_function.rs
@@ -87,13 +87,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::delete_function::DeleteFunctionError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::delete_function::DeleteFunctionError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -115,8 +108,59 @@
     Ok({
         #[allow(unused_mut)]
         let mut output = super::operation::delete_function::builders::DeleteFunctionOutputBuilder::default();
-        output = output.set_status_code(Some(_response_status as _));
+        output = super::protocol_serde::shape_delete_function::de_delete_function(_response_body, output)
+            .map_err(super::operation::delete_function::DeleteFunctionError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
         output.build()
     })
 }
+
+pub fn ser_delete_function_input(
+    input: &super::operation::delete_function::DeleteFunctionInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_delete_function_input::ser_delete_function_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
+pub(crate) fn de_delete_function(
+    _value: &[u8],
+    mut builder: super::operation::delete_function::builders::DeleteFunctionOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::delete_function::builders::DeleteFunctionOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                "StatusCode" => {
+                    builder = builder.set_status_code(
+                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                            .map(i32::try_from)
+                            .transpose()?,
+                    );
+                }
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_delete_function_code_signing_config.rs`

```diff
--- reference/src/protocol_serde/shape_delete_function_code_signing_config.rs
+++ generated/src/protocol_serde/shape_delete_function_code_signing_config.rs
@@ -121,13 +121,6 @@
                     output =
                         super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                             .map_err(super::operation::delete_function_code_signing_config::DeleteFunctionCodeSigningConfigError::unhandled)?;
-                    output = output.set_retry_after_seconds(
-                        super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                            super::operation::delete_function_code_signing_config::DeleteFunctionCodeSigningConfigError::unhandled(
-                                "Failed to parse retryAfterSeconds from header `Retry-After",
-                            )
-                        })?,
-                    );
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -157,3 +150,46 @@
         output.build()
     })
 }
+
+pub fn ser_delete_function_code_signing_config_input(
+    input: &super::operation::delete_function_code_signing_config::DeleteFunctionCodeSigningConfigInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_delete_function_code_signing_config_input::ser_delete_function_code_signing_config_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
+pub(crate) fn de_delete_function_code_signing_config(
+    _value: &[u8],
+    mut builder: super::operation::delete_function_code_signing_config::builders::DeleteFunctionCodeSigningConfigOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::delete_function_code_signing_config::builders::DeleteFunctionCodeSigningConfigOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_delete_function_concurrency.rs`

```diff
--- reference/src/protocol_serde/shape_delete_function_concurrency.rs
+++ generated/src/protocol_serde/shape_delete_function_concurrency.rs
@@ -96,13 +96,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::delete_function_concurrency::DeleteFunctionConcurrencyError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::delete_function_concurrency::DeleteFunctionConcurrencyError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -131,3 +124,46 @@
         output.build()
     })
 }
+
+pub fn ser_delete_function_concurrency_input(
+    input: &super::operation::delete_function_concurrency::DeleteFunctionConcurrencyInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_delete_function_concurrency_input::ser_delete_function_concurrency_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
+pub(crate) fn de_delete_function_concurrency(
+    _value: &[u8],
+    mut builder: super::operation::delete_function_concurrency::builders::DeleteFunctionConcurrencyOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::delete_function_concurrency::builders::DeleteFunctionConcurrencyOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_delete_function_event_invoke_config.rs`

```diff
--- reference/src/protocol_serde/shape_delete_function_event_invoke_config.rs
+++ generated/src/protocol_serde/shape_delete_function_event_invoke_config.rs
@@ -100,13 +100,6 @@
                     output =
                         super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                             .map_err(super::operation::delete_function_event_invoke_config::DeleteFunctionEventInvokeConfigError::unhandled)?;
-                    output = output.set_retry_after_seconds(
-                        super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                            super::operation::delete_function_event_invoke_config::DeleteFunctionEventInvokeConfigError::unhandled(
-                                "Failed to parse retryAfterSeconds from header `Retry-After",
-                            )
-                        })?,
-                    );
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -136,3 +129,46 @@
         output.build()
     })
 }
+
+pub fn ser_delete_function_event_invoke_config_input(
+    input: &super::operation::delete_function_event_invoke_config::DeleteFunctionEventInvokeConfigInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_delete_function_event_invoke_config_input::ser_delete_function_event_invoke_config_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
+pub(crate) fn de_delete_function_event_invoke_config(
+    _value: &[u8],
+    mut builder: super::operation::delete_function_event_invoke_config::builders::DeleteFunctionEventInvokeConfigOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::delete_function_event_invoke_config::builders::DeleteFunctionEventInvokeConfigOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_delete_function_url_config.rs`

```diff
--- reference/src/protocol_serde/shape_delete_function_url_config.rs
+++ generated/src/protocol_serde/shape_delete_function_url_config.rs
@@ -96,13 +96,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::delete_function_url_config::DeleteFunctionUrlConfigError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::delete_function_url_config::DeleteFunctionUrlConfigError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -131,3 +124,46 @@
         output.build()
     })
 }
+
+pub fn ser_delete_function_url_config_input(
+    input: &super::operation::delete_function_url_config::DeleteFunctionUrlConfigInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_delete_function_url_config_input::ser_delete_function_url_config_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
+pub(crate) fn de_delete_function_url_config(
+    _value: &[u8],
+    mut builder: super::operation::delete_function_url_config::builders::DeleteFunctionUrlConfigOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::delete_function_url_config::builders::DeleteFunctionUrlConfigOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_delete_layer_version.rs`

```diff
--- reference/src/protocol_serde/shape_delete_layer_version.rs
+++ generated/src/protocol_serde/shape_delete_layer_version.rs
@@ -75,13 +75,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::delete_layer_version::DeleteLayerVersionError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::delete_layer_version::DeleteLayerVersionError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -110,3 +103,46 @@
         output.build()
     })
 }
+
+pub fn ser_delete_layer_version_input(
+    input: &super::operation::delete_layer_version::DeleteLayerVersionInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_delete_layer_version_input::ser_delete_layer_version_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
+pub(crate) fn de_delete_layer_version(
+    _value: &[u8],
+    mut builder: super::operation::delete_layer_version::builders::DeleteLayerVersionOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::delete_layer_version::builders::DeleteLayerVersionOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_delete_provisioned_concurrency_config.rs`

```diff
--- reference/src/protocol_serde/shape_delete_provisioned_concurrency_config.rs
+++ generated/src/protocol_serde/shape_delete_provisioned_concurrency_config.rs
@@ -100,13 +100,6 @@
                     output =
                         super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                             .map_err(super::operation::delete_provisioned_concurrency_config::DeleteProvisionedConcurrencyConfigError::unhandled)?;
-                    output = output.set_retry_after_seconds(
-                        super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                            super::operation::delete_provisioned_concurrency_config::DeleteProvisionedConcurrencyConfigError::unhandled(
-                                "Failed to parse retryAfterSeconds from header `Retry-After",
-                            )
-                        })?,
-                    );
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -137,3 +130,49 @@
         output.build()
     })
 }
+
+pub fn ser_delete_provisioned_concurrency_config_input(
+    input: &super::operation::delete_provisioned_concurrency_config::DeleteProvisionedConcurrencyConfigInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_delete_provisioned_concurrency_config_input::ser_delete_provisioned_concurrency_config_input_input(
+        &mut object,
+        input,
+    )?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
+pub(crate) fn de_delete_provisioned_concurrency_config(
+    _value: &[u8],
+    mut builder: super::operation::delete_provisioned_concurrency_config::builders::DeleteProvisionedConcurrencyConfigOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::delete_provisioned_concurrency_config::builders::DeleteProvisionedConcurrencyConfigOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_delete_resource_policy.rs`

```diff
--- reference/src/protocol_serde/shape_delete_resource_policy.rs
+++ generated/src/protocol_serde/shape_delete_resource_policy.rs
@@ -106,13 +106,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::delete_resource_policy::DeleteResourcePolicyError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::delete_resource_policy::DeleteResourcePolicyError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -141,3 +134,46 @@
         output.build()
     })
 }
+
+pub fn ser_delete_resource_policy_input(
+    input: &super::operation::delete_resource_policy::DeleteResourcePolicyInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_delete_resource_policy_input::ser_delete_resource_policy_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
+pub(crate) fn de_delete_resource_policy(
+    _value: &[u8],
+    mut builder: super::operation::delete_resource_policy::builders::DeleteResourcePolicyOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::delete_resource_policy::builders::DeleteResourcePolicyOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_destination_config.rs`

```diff
--- reference/src/protocol_serde/shape_destination_config.rs
+++ generated/src/protocol_serde/shape_destination_config.rs
@@ -1,4 +1,23 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_destination_config(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::types::DestinationConfig,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    if let Some(var_1) = &input.on_success {
+        #[allow(unused_mut)]
+        let mut object_2 = object.key("OnSuccess").start_object();
+        super::protocol_serde::shape_on_success::ser_on_success(&mut object_2, var_1)?;
+        object_2.finish();
+    }
+    if let Some(var_3) = &input.on_failure {
+        #[allow(unused_mut)]
+        let mut object_4 = object.key("OnFailure").start_object();
+        super::protocol_serde::shape_on_failure::ser_on_failure(&mut object_4, var_3)?;
+        object_4.finish();
+    }
+    Ok(())
+}
+
 pub(crate) fn de_destination_config<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
@@ -43,22 +62,3 @@
         )),
     }
 }
-
-pub fn ser_destination_config(
-    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
-    input: &super::types::DestinationConfig,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.on_success {
-        #[allow(unused_mut)]
-        let mut object_2 = object.key("OnSuccess").start_object();
-        super::protocol_serde::shape_on_success::ser_on_success(&mut object_2, var_1)?;
-        object_2.finish();
-    }
-    if let Some(var_3) = &input.on_failure {
-        #[allow(unused_mut)]
-        let mut object_4 = object.key("OnFailure").start_object();
-        super::protocol_serde::shape_on_failure::ser_on_failure(&mut object_4, var_3)?;
-        object_4.finish();
-    }
-    Ok(())
-}
```

### `src/protocol_serde/shape_document_db_event_source_config.rs`

```diff
--- reference/src/protocol_serde/shape_document_db_event_source_config.rs
+++ generated/src/protocol_serde/shape_document_db_event_source_config.rs
@@ -1,4 +1,20 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_document_db_event_source_config(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::types::DocumentDbEventSourceConfig,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    if let Some(var_1) = &input.database_name {
+        object.key("DatabaseName").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.collection_name {
+        object.key("CollectionName").string(var_2.as_str());
+    }
+    if let Some(var_3) = &input.full_document {
+        object.key("FullDocument").string(var_3.as_str());
+    }
+    Ok(())
+}
+
 pub(crate) fn de_document_db_event_source_config<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
@@ -58,19 +74,3 @@
         )),
     }
 }
-
-pub fn ser_document_db_event_source_config(
-    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
-    input: &super::types::DocumentDbEventSourceConfig,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.database_name {
-        object.key("DatabaseName").string(var_1.as_str());
-    }
-    if let Some(var_2) = &input.collection_name {
-        object.key("CollectionName").string(var_2.as_str());
-    }
-    if let Some(var_3) = &input.full_document {
-        object.key("FullDocument").string(var_3.as_str());
-    }
-    Ok(())
-}
```

### `src/protocol_serde/shape_durable_config.rs`

```diff
--- reference/src/protocol_serde/shape_durable_config.rs
+++ generated/src/protocol_serde/shape_durable_config.rs
@@ -1,4 +1,26 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_durable_config(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::types::DurableConfig,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    if let Some(var_1) = &input.kms_key_arn {
+        object.key("KMSKeyArn").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.retention_period_in_days {
+        object.key("RetentionPeriodInDays").number(
+            #[allow(clippy::useless_conversion)]
+            ::aws_smithy_types::Number::NegInt((*var_2).into()),
+        );
+    }
+    if let Some(var_3) = &input.execution_timeout {
+        object.key("ExecutionTimeout").number(
+            #[allow(clippy::useless_conversion)]
+            ::aws_smithy_types::Number::NegInt((*var_3).into()),
+        );
+    }
+    Ok(())
+}
+
 pub(crate) fn de_durable_config<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
@@ -58,25 +80,3 @@
         )),
     }
 }
-
-pub fn ser_durable_config(
-    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
-    input: &super::types::DurableConfig,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.kms_key_arn {
-        object.key("KMSKeyArn").string(var_1.as_str());
-    }
-    if let Some(var_2) = &input.retention_period_in_days {
-        object.key("RetentionPeriodInDays").number(
-            #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_2).into()),
-        );
-    }
-    if let Some(var_3) = &input.execution_timeout {
-        object.key("ExecutionTimeout").number(
-            #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_3).into()),
-        );
-    }
-    Ok(())
-}
```

### `src/protocol_serde/shape_endpoints.rs`

```diff
--- reference/src/protocol_serde/shape_endpoints.rs
+++ generated/src/protocol_serde/shape_endpoints.rs
@@ -23,7 +23,7 @@
                 match tokens.next().transpose()? {
                     Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                     Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
-                        let key = key.to_unescaped().map(|u| super::types::EndPointType::from(u.as_ref()))?;
+                        let key = key.to_unescaped().map(|u| u.into_owned())?;
                         let value = super::protocol_serde::shape_endpoint_lists::de_endpoint_lists(tokens, _value, depth + 1)?;
                         match value {
                             Some(value) => {
```

### `src/protocol_serde/shape_ephemeral_storage.rs`

```diff
--- reference/src/protocol_serde/shape_ephemeral_storage.rs
+++ generated/src/protocol_serde/shape_ephemeral_storage.rs
@@ -1,4 +1,17 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_ephemeral_storage(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::types::EphemeralStorage,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    {
+        object.key("Size").number(
+            #[allow(clippy::useless_conversion)]
+            ::aws_smithy_types::Number::NegInt((input.size).into()),
+        );
+    }
+    Ok(())
+}
+
 pub(crate) fn de_ephemeral_storage<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
@@ -46,16 +59,3 @@
         )),
     }
 }
-
-pub fn ser_ephemeral_storage(
-    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
-    input: &super::types::EphemeralStorage,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    {
-        object.key("Size").number(
-            #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((input.size).into()),
-        );
-    }
-    Ok(())
-}
```

### `src/protocol_serde/shape_error_object.rs`

```diff
--- reference/src/protocol_serde/shape_error_object.rs
+++ generated/src/protocol_serde/shape_error_object.rs
@@ -1,4 +1,29 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_error_object(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::types::ErrorObject,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    if let Some(var_1) = &input.error_message {
+        object.key("ErrorMessage").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.error_type {
+        object.key("ErrorType").string(var_2.as_str());
+    }
+    if let Some(var_3) = &input.error_data {
+        object.key("ErrorData").string(var_3.as_str());
+    }
+    if let Some(var_4) = &input.stack_trace {
+        let mut array_5 = object.key("StackTrace").start_array();
+        for item_6 in var_4 {
+            {
+                array_5.value().string(item_6.as_str());
+            }
+        }
+        array_5.finish();
+    }
+    Ok(())
+}
+
 pub(crate) fn de_error_object<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
@@ -65,28 +90,3 @@
         )),
     }
 }
-
-pub fn ser_error_object(
-    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
-    input: &super::types::ErrorObject,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.error_message {
-        object.key("ErrorMessage").string(var_1.as_str());
-    }
-    if let Some(var_2) = &input.error_type {
-        object.key("ErrorType").string(var_2.as_str());
-    }
-    if let Some(var_3) = &input.error_data {
-        object.key("ErrorData").string(var_3.as_str());
-    }
-    if let Some(var_4) = &input.stack_trace {
-        let mut array_5 = object.key("StackTrace").start_array();
-        for item_6 in var_4 {
-            {
-                array_5.value().string(item_6.as_str());
-            }
-        }
-        array_5.finish();
-    }
-    Ok(())
-}
```

### `src/protocol_serde/shape_event_source_mapping_configuration.rs`

```diff
--- reference/src/protocol_serde/shape_event_source_mapping_configuration.rs
+++ generated/src/protocol_serde/shape_event_source_mapping_configuration.rs
@@ -20,231 +20,231 @@
             loop {
                 match tokens.next().transpose()? {
                     Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                        "UUID" => {
-                            builder = builder.set_uuid(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                    .transpose()?,
-                            );
-                        }
-                        "StartingPosition" => {
-                            builder = builder.set_starting_position(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                    .map(|s| s.to_unescaped().map(|u| super::types::EventSourcePosition::from(u.as_ref())))
-                                    .transpose()?,
-                            );
-                        }
-                        "StartingPositionTimestamp" => {
-                            builder = builder.set_starting_position_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
-                                tokens.next(),
-                                ::aws_smithy_types::date_time::Format::EpochSeconds,
-                            )?);
-                        }
-                        "BatchSize" => {
-                            builder = builder.set_batch_size(
-                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
-                                    .map(i32::try_from)
-                                    .transpose()?,
-                            );
-                        }
-                        "MaximumBatchingWindowInSeconds" => {
-                            builder = builder.set_maximum_batching_window_in_seconds(
-                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
-                                    .map(i32::try_from)
-                                    .transpose()?,
-                            );
-                        }
-                        "ParallelizationFactor" => {
-                            builder = builder.set_parallelization_factor(
-                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
-                                    .map(i32::try_from)
-                                    .transpose()?,
-                            );
-                        }
-                        "EventSourceArn" => {
-                            builder = builder.set_event_source_arn(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                    .transpose()?,
-                            );
-                        }
-                        "FilterCriteria" => {
-                            builder = builder.set_filter_criteria(super::protocol_serde::shape_filter_criteria::de_filter_criteria(
-                                tokens,
-                                _value,
-                                depth + 1,
-                            )?);
-                        }
-                        "FilterCriteriaError" => {
-                            builder = builder.set_filter_criteria_error(
-                                super::protocol_serde::shape_filter_criteria_error::de_filter_criteria_error(tokens, _value, depth + 1)?,
-                            );
-                        }
-                        "KMSKeyArn" => {
-                            builder = builder.set_kms_key_arn(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                    .transpose()?,
-                            );
-                        }
-                        "MetricsConfig" => {
-                            builder = builder.set_metrics_config(
-                                super::protocol_serde::shape_event_source_mapping_metrics_config::de_event_source_mapping_metrics_config(
+                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
+                        match key.to_unescaped()?.as_ref() {
+                            "UUID" => {
+                                builder = builder.set_uuid(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                        .transpose()?,
+                                );
+                            }
+                            "StartingPosition" => {
+                                builder = builder.set_starting_position(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                        .map(|s| s.to_unescaped().map(|u| super::types::EventSourcePosition::from(u.as_ref())))
+                                        .transpose()?,
+                                );
+                            }
+                            "StartingPositionTimestamp" => {
+                                builder = builder.set_starting_position_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
+                                    tokens.next(),
+                                    ::aws_smithy_types::date_time::Format::EpochSeconds,
+                                )?);
+                            }
+                            "BatchSize" => {
+                                builder = builder.set_batch_size(
+                                    ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                                        .map(i32::try_from)
+                                        .transpose()?,
+                                );
+                            }
+                            "MaximumBatchingWindowInSeconds" => {
+                                builder = builder.set_maximum_batching_window_in_seconds(
+                                    ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                                        .map(i32::try_from)
+                                        .transpose()?,
+                                );
+                            }
+                            "ParallelizationFactor" => {
+                                builder = builder.set_parallelization_factor(
+                                    ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                                        .map(i32::try_from)
+                                        .transpose()?,
+                                );
+                            }
+                            "EventSourceArn" => {
+                                builder = builder.set_event_source_arn(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                        .transpose()?,
+                                );
+                            }
+                            "FilterCriteria" => {
+                                builder = builder.set_filter_criteria(super::protocol_serde::shape_filter_criteria::de_filter_criteria(
                                     tokens,
                                     _value,
                                     depth + 1,
-                                )?,
-                            );
-                        }
-                        "LoggingConfig" => {
-                            builder = builder.set_logging_config(
-                                super::protocol_serde::shape_event_source_mapping_logging_config::de_event_source_mapping_logging_config(
+                                )?);
+                            }
+                            "FilterCriteriaError" => {
+                                builder = builder.set_filter_criteria_error(
+                                    super::protocol_serde::shape_filter_criteria_error::de_filter_criteria_error(tokens, _value, depth + 1)?,
+                                );
+                            }
+                            "KMSKeyArn" => {
+                                builder = builder.set_kms_key_arn(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                        .transpose()?,
+                                );
+                            }
+                            "MetricsConfig" => {
+                                builder = builder.set_metrics_config(
+                                    super::protocol_serde::shape_event_source_mapping_metrics_config::de_event_source_mapping_metrics_config(
+                                        tokens,
+                                        _value,
+                                        depth + 1,
+                                    )?,
+                                );
+                            }
+                            "LoggingConfig" => {
+                                builder = builder.set_logging_config(
+                                    super::protocol_serde::shape_event_source_mapping_logging_config::de_event_source_mapping_logging_config(
+                                        tokens,
+                                        _value,
+                                        depth + 1,
+                                    )?,
+                                );
+                            }
+                            "ScalingConfig" => {
+                                builder = builder.set_scaling_config(super::protocol_serde::shape_scaling_config::de_scaling_config(
                                     tokens,
                                     _value,
                                     depth + 1,
-                                )?,
-                            );
-                        }
-                        "ScalingConfig" => {
-                            builder = builder.set_scaling_config(super::protocol_serde::shape_scaling_config::de_scaling_config(
-                                tokens,
-                                _value,
-                                depth + 1,
-                            )?);
-                        }
-                        "FunctionArn" => {
-                            builder = builder.set_function_arn(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                    .transpose()?,
-                            );
-                        }
-                        "LastModified" => {
-                            builder = builder.set_last_modified(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
-                                tokens.next(),
-                                ::aws_smithy_types::date_time::Format::EpochSeconds,
-                            )?);
-                        }
-                        "LastProcessingResult" => {
-                            builder = builder.set_last_processing_result(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                    .transpose()?,
-                            );
-                        }
-                        "State" => {
-                            builder = builder.set_state(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                    .transpose()?,
-                            );
-                        }
-                        "StateTransitionReason" => {
-                            builder = builder.set_state_transition_reason(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                    .transpose()?,
-                            );
-                        }
-                        "DestinationConfig" => {
-                            builder = builder.set_destination_config(super::protocol_serde::shape_destination_config::de_destination_config(
-                                tokens,
-                                _value,
-                                depth + 1,
-                            )?);
-                        }
-                        "Topics" => {
-                            builder = builder.set_topics(super::protocol_serde::shape_topics::de_topics(tokens, _value, depth + 1)?);
-                        }
-                        "Queues" => {
-                            builder = builder.set_queues(super::protocol_serde::shape_queues::de_queues(tokens, _value, depth + 1)?);
-                        }
-                        "SourceAccessConfigurations" => {
-                            builder = builder.set_source_access_configurations(
-                                super::protocol_serde::shape_source_access_configurations::de_source_access_configurations(
-                                    tokens,
-                                    _value,
-                                    depth + 1,
-                                )?,
-                            );
-                        }
-                        "SelfManagedEventSource" => {
-                            builder = builder.set_self_managed_event_source(
-                                super::protocol_serde::shape_self_managed_event_source::de_self_managed_event_source(tokens, _value, depth + 1)?,
-                            );
-                        }
-                        "MaximumRecordAgeInSeconds" => {
-                            builder = builder.set_maximum_record_age_in_seconds(
-                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
-                                    .map(i32::try_from)
-                                    .transpose()?,
-                            );
-                        }
-                        "BisectBatchOnFunctionError" => {
-                            builder = builder
-                                .set_bisect_batch_on_function_error(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
-                        }
-                        "MaximumRetryAttempts" => {
-                            builder = builder.set_maximum_retry_attempts(
-                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
-                                    .map(i32::try_from)
-                                    .transpose()?,
-                            );
-                        }
-                        "TumblingWindowInSeconds" => {
-                            builder = builder.set_tumbling_window_in_seconds(
-                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
-                                    .map(i32::try_from)
-                                    .transpose()?,
-                            );
-                        }
-                        "FunctionResponseTypes" => {
-                            builder = builder.set_function_response_types(
-                                super::protocol_serde::shape_function_response_type_list::de_function_response_type_list(tokens, _value, depth + 1)?,
-                            );
-                        }
-                        "AmazonManagedKafkaEventSourceConfig" => {
-                            builder = builder.set_amazon_managed_kafka_event_source_config(
-                                super::protocol_serde::shape_amazon_managed_kafka_event_source_config::de_amazon_managed_kafka_event_source_config(
-                                    tokens,
-                                    _value,
-                                    depth + 1,
-                                )?,
-                            );
-                        }
-                        "SelfManagedKafkaEventSourceConfig" => {
-                            builder = builder.set_self_managed_kafka_event_source_config(
-                                super::protocol_serde::shape_self_managed_kafka_event_source_config::de_self_managed_kafka_event_source_config(
-                                    tokens,
-                                    _value,
-                                    depth + 1,
-                                )?,
-                            );
-                        }
-                        "DocumentDBEventSourceConfig" => {
-                            builder = builder.set_document_db_event_source_config(
-                                super::protocol_serde::shape_document_db_event_source_config::de_document_db_event_source_config(
+                                )?);
+                            }
+                            "FunctionArn" => {
+                                builder = builder.set_function_arn(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                        .transpose()?,
+                                );
+                            }
+                            "LastModified" => {
+                                builder = builder.set_last_modified(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
+                                    tokens.next(),
+                                    ::aws_smithy_types::date_time::Format::EpochSeconds,
+                                )?);
+                            }
+                            "LastProcessingResult" => {
+                                builder = builder.set_last_processing_result(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                        .transpose()?,
+                                );
+                            }
+                            "State" => {
+                                builder = builder.set_state(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                        .transpose()?,
+                                );
+                            }
+                            "StateTransitionReason" => {
+                                builder = builder.set_state_transition_reason(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                        .transpose()?,
+                                );
+                            }
+                            "DestinationConfig" => {
+                                builder = builder.set_destination_config(super::protocol_serde::shape_destination_config::de_destination_config(
                                     tokens,
                                     _value,
                                     depth + 1,
-                                )?,
-                            );
-                        }
-                        "EventSourceMappingArn" => {
-                            builder = builder.set_event_source_mapping_arn(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                    .transpose()?,
-                            );
-                        }
-                        "ProvisionedPollerConfig" => {
-                            builder = builder.set_provisioned_poller_config(
-                                super::protocol_serde::shape_provisioned_poller_config::de_provisioned_poller_config(tokens, _value, depth + 1)?,
-                            );
+                                )?);
+                            }
+                            "Topics" => {
+                                builder = builder.set_topics(super::protocol_serde::shape_topics::de_topics(tokens, _value, depth + 1)?);
+                            }
+                            "Queues" => {
+                                builder = builder.set_queues(super::protocol_serde::shape_queues::de_queues(tokens, _value, depth + 1)?);
+                            }
+                            "SourceAccessConfigurations" => {
+                                builder = builder.set_source_access_configurations(
+                                    super::protocol_serde::shape_source_access_configurations::de_source_access_configurations(
+                                        tokens,
+                                        _value,
+                                        depth + 1,
+                                    )?,
+                                );
+                            }
+                            "SelfManagedEventSource" => {
+                                builder = builder.set_self_managed_event_source(
+                                    super::protocol_serde::shape_self_managed_event_source::de_self_managed_event_source(tokens, _value, depth + 1)?,
+                                );
+                            }
+                            "MaximumRecordAgeInSeconds" => {
+                                builder = builder.set_maximum_record_age_in_seconds(
+                                    ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                                        .map(i32::try_from)
+                                        .transpose()?,
+                                );
+                            }
+                            "BisectBatchOnFunctionError" => {
+                                builder = builder
+                                    .set_bisect_batch_on_function_error(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
+                            }
+                            "MaximumRetryAttempts" => {
+                                builder = builder.set_maximum_retry_attempts(
+                                    ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                                        .map(i32::try_from)
+                                        .transpose()?,
+                                );
+                            }
+                            "TumblingWindowInSeconds" => {
+                                builder = builder.set_tumbling_window_in_seconds(
+                                    ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                                        .map(i32::try_from)
+                                        .transpose()?,
+                                );
+                            }
+                            "FunctionResponseTypes" => {
+                                builder = builder.set_function_response_types(
+                                    super::protocol_serde::shape_function_response_type_list::de_function_response_type_list(
+                                        tokens,
+                                        _value,
+                                        depth + 1,
+                                    )?,
+                                );
+                            }
+                            "AmazonManagedKafkaEventSourceConfig" => {
+                                builder = builder.set_amazon_managed_kafka_event_source_config(super::protocol_serde::shape_amazon_managed_kafka_event_source_config::de_amazon_managed_kafka_event_source_config(tokens, _value, depth + 1)?);
+                            }
+                            "SelfManagedKafkaEventSourceConfig" => {
+                                builder = builder.set_self_managed_kafka_event_source_config(
+                                    super::protocol_serde::shape_self_managed_kafka_event_source_config::de_self_managed_kafka_event_source_config(
+                                        tokens,
+                                        _value,
+                                        depth + 1,
+                                    )?,
+                                );
+                            }
+                            "DocumentDBEventSourceConfig" => {
+                                builder = builder.set_document_db_event_source_config(
+                                    super::protocol_serde::shape_document_db_event_source_config::de_document_db_event_source_config(
+                                        tokens,
+                                        _value,
+                                        depth + 1,
+                                    )?,
+                                );
+                            }
+                            "EventSourceMappingArn" => {
+                                builder = builder.set_event_source_mapping_arn(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                        .transpose()?,
+                                );
+                            }
+                            "ProvisionedPollerConfig" => {
+                                builder = builder.set_provisioned_poller_config(
+                                    super::protocol_serde::shape_provisioned_poller_config::de_provisioned_poller_config(tokens, _value, depth + 1)?,
+                                );
+                            }
+                            _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                         }
-                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
-                    },
+                    }
                     other => {
                         return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                             "expected object key or end object, found: {other:?}"
```

### `src/protocol_serde/shape_event_source_mapping_logging_config.rs`

```diff
--- reference/src/protocol_serde/shape_event_source_mapping_logging_config.rs
+++ generated/src/protocol_serde/shape_event_source_mapping_logging_config.rs
@@ -1,4 +1,14 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_event_source_mapping_logging_config(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::types::EventSourceMappingLoggingConfig,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    if let Some(var_1) = &input.system_log_level {
+        object.key("SystemLogLevel").string(var_1.as_str());
+    }
+    Ok(())
+}
+
 pub(crate) fn de_event_source_mapping_logging_config<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
@@ -44,13 +54,3 @@
         )),
     }
 }
-
-pub fn ser_event_source_mapping_logging_config(
-    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
-    input: &super::types::EventSourceMappingLoggingConfig,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.system_log_level {
-        object.key("SystemLogLevel").string(var_1.as_str());
-    }
-    Ok(())
-}
```

### `src/protocol_serde/shape_event_source_mapping_metrics_config.rs`

```diff
--- reference/src/protocol_serde/shape_event_source_mapping_metrics_config.rs
+++ generated/src/protocol_serde/shape_event_source_mapping_metrics_config.rs
@@ -1,4 +1,20 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_event_source_mapping_metrics_config(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::types::EventSourceMappingMetricsConfig,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    if let Some(var_1) = &input.metrics {
+        let mut array_2 = object.key("Metrics").start_array();
+        for item_3 in var_1 {
+            {
+                array_2.value().string(item_3.as_str());
+            }
+        }
+        array_2.finish();
+    }
+    Ok(())
+}
+
 pub(crate) fn de_event_source_mapping_metrics_config<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
@@ -46,19 +62,3 @@
         )),
     }
 }
-
-pub fn ser_event_source_mapping_metrics_config(
-    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
-    input: &super::types::EventSourceMappingMetricsConfig,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.metrics {
-        let mut array_2 = object.key("Metrics").start_array();
-        for item_3 in var_1 {
-            {
-                array_2.value().string(item_3.as_str());
-            }
-        }
-        array_2.finish();
-    }
-    Ok(())
-}
```

### `src/protocol_serde/shape_filter_criteria.rs`

```diff
--- reference/src/protocol_serde/shape_filter_criteria.rs
+++ generated/src/protocol_serde/shape_filter_criteria.rs
@@ -1,4 +1,23 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_filter_criteria(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::types::FilterCriteria,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    if let Some(var_1) = &input.filters {
+        let mut array_2 = object.key("Filters").start_array();
+        for item_3 in var_1 {
+            {
+                #[allow(unused_mut)]
+                let mut object_4 = array_2.value().start_object();
+                super::protocol_serde::shape_filter::ser_filter(&mut object_4, item_3)?;
+                object_4.finish();
+            }
+        }
+        array_2.finish();
+    }
+    Ok(())
+}
+
 pub(crate) fn de_filter_criteria<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
@@ -40,22 +59,3 @@
         )),
     }
 }
-
-pub fn ser_filter_criteria(
-    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
-    input: &super::types::FilterCriteria,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.filters {
-        let mut array_2 = object.key("Filters").start_array();
-        for item_3 in var_1 {
-            {
-                #[allow(unused_mut)]
-                let mut object_4 = array_2.value().start_object();
-                super::protocol_serde::shape_filter::ser_filter(&mut object_4, item_3)?;
-                object_4.finish();
-            }
-        }
-        array_2.finish();
-    }
-    Ok(())
-}
```

### `src/protocol_serde/shape_function_scaling_config.rs`

```diff
--- reference/src/protocol_serde/shape_function_scaling_config.rs
+++ generated/src/protocol_serde/shape_function_scaling_config.rs
@@ -1,4 +1,23 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_function_scaling_config(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::types::FunctionScalingConfig,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    if let Some(var_1) = &input.min_execution_environments {
+        object.key("MinExecutionEnvironments").number(
+            #[allow(clippy::useless_conversion)]
+            ::aws_smithy_types::Number::NegInt((*var_1).into()),
+        );
+    }
+    if let Some(var_2) = &input.max_execution_environments {
+        object.key("MaxExecutionEnvironments").number(
+            #[allow(clippy::useless_conversion)]
+            ::aws_smithy_types::Number::NegInt((*var_2).into()),
+        );
+    }
+    Ok(())
+}
+
 pub(crate) fn de_function_scaling_config<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
@@ -51,22 +70,3 @@
         )),
     }
 }
-
-pub fn ser_function_scaling_config(
-    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
-    input: &super::types::FunctionScalingConfig,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.min_execution_environments {
-        object.key("MinExecutionEnvironments").number(
-            #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_1).into()),
-        );
-    }
-    if let Some(var_2) = &input.max_execution_environments {
-        object.key("MaxExecutionEnvironments").number(
-            #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_2).into()),
-        );
-    }
-    Ok(())
-}
```

### `src/protocol_serde/shape_function_versions_by_capacity_provider_list.rs`

```diff
--- reference/src/protocol_serde/shape_function_versions_by_capacity_provider_list.rs
+++ generated/src/protocol_serde/shape_function_versions_by_capacity_provider_list.rs
@@ -26,9 +26,7 @@
                         break;
                     }
                     _ => {
-                        let value =
-                            super::protocol_serde::shape_function_versions_by_capacity_provider_list_item::de_function_versions_by_capacity_provider_list_item(tokens, _value, depth + 1)?
-                        ;
+                        let value = super::protocol_serde::shape_function_versions_by_capacity_provider_list_item::de_function_versions_by_capacity_provider_list_item(tokens, _value, depth + 1)?;
                         if let Some(value) = value {
                             items.push(value);
                         } else {
```

### `src/protocol_serde/shape_get_account_settings.rs`

```diff
--- reference/src/protocol_serde/shape_get_account_settings.rs
+++ generated/src/protocol_serde/shape_get_account_settings.rs
@@ -42,13 +42,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_account_settings::GetAccountSettingsError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::get_account_settings::GetAccountSettingsError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -80,6 +73,12 @@
     })
 }

+pub fn ser_get_account_settings_input(
+    _input: &super::operation::get_account_settings::GetAccountSettingsInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    Ok(::aws_smithy_types::body::SdkBody::from("{}"))
+}
+
 pub(crate) fn de_get_account_settings(
     _value: &[u8],
     mut builder: super::operation::get_account_settings::builders::GetAccountSettingsOutputBuilder,
```

### `src/protocol_serde/shape_get_alias.rs`

```diff
--- reference/src/protocol_serde/shape_get_alias.rs
+++ generated/src/protocol_serde/shape_get_alias.rs
@@ -72,11 +72,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_alias::GetAliasError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::get_alias::GetAliasError::unhandled("Failed to parse retryAfterSeconds from header `Retry-After")
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -105,6 +100,16 @@
     })
 }

+pub fn ser_get_alias_input(
+    input: &super::operation::get_alias::GetAliasInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_alias_input::ser_get_alias_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_get_alias(
     _value: &[u8],
     mut builder: super::operation::get_alias::builders::GetAliasOutputBuilder,
@@ -126,8 +131,8 @@
                                 .transpose()?,
                         );
                     }
-                    "Description" => {
-                        builder = builder.set_description(
+                    "Name" => {
+                        builder = builder.set_name(
                             ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                 .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                 .transpose()?,
@@ -140,13 +145,18 @@
                                 .transpose()?,
                         );
                     }
-                    "Name" => {
-                        builder = builder.set_name(
+                    "Description" => {
+                        builder = builder.set_description(
                             ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                 .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                 .transpose()?,
                         );
                     }
+                    "RoutingConfig" => {
+                        builder = builder.set_routing_config(
+                            super::protocol_serde::shape_alias_routing_configuration::de_alias_routing_configuration(tokens, _value, depth + 1)?,
+                        );
+                    }
                     "RevisionId" => {
                         builder = builder.set_revision_id(
                             ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -154,11 +164,6 @@
                                 .transpose()?,
                         );
                     }
-                    "RoutingConfig" => {
-                        builder = builder.set_routing_config(
-                            super::protocol_serde::shape_alias_routing_configuration::de_alias_routing_configuration(tokens, _value, depth + 1)?,
-                        );
-                    }
                     _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                 }
             }
```

### `src/protocol_serde/shape_get_capacity_provider.rs`

```diff
--- reference/src/protocol_serde/shape_get_capacity_provider.rs
+++ generated/src/protocol_serde/shape_get_capacity_provider.rs
@@ -75,13 +75,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_capacity_provider::GetCapacityProviderError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::get_capacity_provider::GetCapacityProviderError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -113,6 +106,16 @@
     })
 }

+pub fn ser_get_capacity_provider_input(
+    input: &super::operation::get_capacity_provider::GetCapacityProviderInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_capacity_provider_input::ser_get_capacity_provider_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_get_capacity_provider(
     _value: &[u8],
     mut builder: super::operation::get_capacity_provider::builders::GetCapacityProviderOutputBuilder,
```

### `src/protocol_serde/shape_get_code_signing_config.rs`

```diff
--- reference/src/protocol_serde/shape_get_code_signing_config.rs
+++ generated/src/protocol_serde/shape_get_code_signing_config.rs
@@ -91,6 +91,16 @@
     })
 }

+pub fn ser_get_code_signing_config_input(
+    input: &super::operation::get_code_signing_config::GetCodeSigningConfigInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_code_signing_config_input::ser_get_code_signing_config_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_get_code_signing_config(
     _value: &[u8],
     mut builder: super::operation::get_code_signing_config::builders::GetCodeSigningConfigOutputBuilder,
```

### `src/protocol_serde/shape_get_durable_execution.rs`

```diff
--- reference/src/protocol_serde/shape_get_durable_execution.rs
+++ generated/src/protocol_serde/shape_get_durable_execution.rs
@@ -135,13 +135,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_durable_execution::GetDurableExecutionError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::get_durable_execution::GetDurableExecutionError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -175,6 +168,16 @@
     })
 }

+pub fn ser_get_durable_execution_input(
+    input: &super::operation::get_durable_execution::GetDurableExecutionInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_durable_execution_input::ser_get_durable_execution_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_get_durable_execution(
     _value: &[u8],
     mut builder: super::operation::get_durable_execution::builders::GetDurableExecutionOutputBuilder,
@@ -191,9 +194,6 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "DurableConfig" => {
-                    builder = builder.set_durable_config(super::protocol_serde::shape_durable_config::de_durable_config(tokens, _value, depth + 1)?);
-                }
                 "DurableExecutionArn" => {
                     builder = builder.set_durable_execution_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -208,18 +208,6 @@
                             .transpose()?,
                     );
                 }
-                "EndTimestamp" => {
-                    builder = builder.set_end_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
-                        tokens.next(),
-                        ::aws_smithy_types::date_time::Format::EpochSeconds,
-                    )?);
-                }
-                "Error" => {
-                    builder = builder.set_error(super::protocol_serde::shape_error_object::de_error_object(tokens, _value, depth + 1)?);
-                }
-                "ExecutionDataIncluded" => {
-                    builder = builder.set_execution_data_included(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
-                }
                 "FunctionArn" => {
                     builder = builder.set_function_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -241,6 +229,9 @@
                             .transpose()?,
                     );
                 }
+                "Error" => {
+                    builder = builder.set_error(super::protocol_serde::shape_error_object::de_error_object(tokens, _value, depth + 1)?);
+                }
                 "StartTimestamp" => {
                     builder = builder.set_start_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                         tokens.next(),
@@ -254,8 +245,11 @@
                             .transpose()?,
                     );
                 }
-                "TraceHeader" => {
-                    builder = builder.set_trace_header(super::protocol_serde::shape_trace_header::de_trace_header(tokens, _value, depth + 1)?);
+                "EndTimestamp" => {
+                    builder = builder.set_end_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
+                        tokens.next(),
+                        ::aws_smithy_types::date_time::Format::EpochSeconds,
+                    )?);
                 }
                 "Version" => {
                     builder = builder.set_version(
@@ -264,6 +258,15 @@
                             .transpose()?,
                     );
                 }
+                "TraceHeader" => {
+                    builder = builder.set_trace_header(super::protocol_serde::shape_trace_header::de_trace_header(tokens, _value, depth + 1)?);
+                }
+                "ExecutionDataIncluded" => {
+                    builder = builder.set_execution_data_included(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
+                }
+                "DurableConfig" => {
+                    builder = builder.set_durable_config(super::protocol_serde::shape_durable_config::de_durable_config(tokens, _value, depth + 1)?);
+                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_get_durable_execution_history.rs`

```diff
--- reference/src/protocol_serde/shape_get_durable_execution_history.rs
+++ generated/src/protocol_serde/shape_get_durable_execution_history.rs
@@ -137,13 +137,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_durable_execution_history::GetDurableExecutionHistoryError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::get_durable_execution_history::GetDurableExecutionHistoryError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -177,6 +170,16 @@
     })
 }

+pub fn ser_get_durable_execution_history_input(
+    input: &super::operation::get_durable_execution_history::GetDurableExecutionHistoryInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_durable_execution_history_input::ser_get_durable_execution_history_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_get_durable_execution_history(
     _value: &[u8],
     mut builder: super::operation::get_durable_execution_history::builders::GetDurableExecutionHistoryOutputBuilder,
```

### `src/protocol_serde/shape_get_durable_execution_state.rs`

```diff
--- reference/src/protocol_serde/shape_get_durable_execution_state.rs
+++ generated/src/protocol_serde/shape_get_durable_execution_state.rs
@@ -126,13 +126,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_durable_execution_state::GetDurableExecutionStateError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::get_durable_execution_state::GetDurableExecutionStateError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -166,6 +159,16 @@
     })
 }

+pub fn ser_get_durable_execution_state_input(
+    input: &super::operation::get_durable_execution_state::GetDurableExecutionStateInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_durable_execution_state_input::ser_get_durable_execution_state_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_get_durable_execution_state(
     _value: &[u8],
     mut builder: super::operation::get_durable_execution_state::builders::GetDurableExecutionStateOutputBuilder,
@@ -182,6 +185,9 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                "Operations" => {
+                    builder = builder.set_operations(super::protocol_serde::shape_operations::de_operations(tokens, _value, depth + 1)?);
+                }
                 "NextMarker" => {
                     builder = builder.set_next_marker(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -189,9 +195,6 @@
                             .transpose()?,
                     );
                 }
-                "Operations" => {
-                    builder = builder.set_operations(super::protocol_serde::shape_operations::de_operations(tokens, _value, depth + 1)?);
-                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_get_event_source_mapping.rs`

```diff
--- reference/src/protocol_serde/shape_get_event_source_mapping.rs
+++ generated/src/protocol_serde/shape_get_event_source_mapping.rs
@@ -75,13 +75,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_event_source_mapping::GetEventSourceMappingError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::get_event_source_mapping::GetEventSourceMappingError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -113,6 +106,16 @@
     })
 }

+pub fn ser_get_event_source_mapping_input(
+    input: &super::operation::get_event_source_mapping::GetEventSourceMappingInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_event_source_mapping_input::ser_get_event_source_mapping_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_get_event_source_mapping(
     _value: &[u8],
     mut builder: super::operation::get_event_source_mapping::builders::GetEventSourceMappingOutputBuilder,
@@ -129,15 +132,26 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "AmazonManagedKafkaEventSourceConfig" => {
-                    builder = builder.set_amazon_managed_kafka_event_source_config(
-                        super::protocol_serde::shape_amazon_managed_kafka_event_source_config::de_amazon_managed_kafka_event_source_config(
-                            tokens,
-                            _value,
-                            depth + 1,
-                        )?,
+                "UUID" => {
+                    builder = builder.set_uuid(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
+                }
+                "StartingPosition" => {
+                    builder = builder.set_starting_position(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| super::types::EventSourcePosition::from(u.as_ref())))
+                            .transpose()?,
                     );
                 }
+                "StartingPositionTimestamp" => {
+                    builder = builder.set_starting_position_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
+                        tokens.next(),
+                        ::aws_smithy_types::date_time::Format::EpochSeconds,
+                    )?);
+                }
                 "BatchSize" => {
                     builder = builder.set_batch_size(
                         ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
@@ -145,19 +159,18 @@
                             .transpose()?,
                     );
                 }
-                "BisectBatchOnFunctionError" => {
-                    builder = builder.set_bisect_batch_on_function_error(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
+                "MaximumBatchingWindowInSeconds" => {
+                    builder = builder.set_maximum_batching_window_in_seconds(
+                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                            .map(i32::try_from)
+                            .transpose()?,
+                    );
                 }
-                "DestinationConfig" => {
-                    builder = builder.set_destination_config(super::protocol_serde::shape_destination_config::de_destination_config(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
-                }
-                "DocumentDBEventSourceConfig" => {
-                    builder = builder.set_document_db_event_source_config(
-                        super::protocol_serde::shape_document_db_event_source_config::de_document_db_event_source_config(tokens, _value, depth + 1)?,
+                "ParallelizationFactor" => {
+                    builder = builder.set_parallelization_factor(
+                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                            .map(i32::try_from)
+                            .transpose()?,
                     );
                 }
                 "EventSourceArn" => {
@@ -167,13 +180,6 @@
                             .transpose()?,
                     );
                 }
-                "EventSourceMappingArn" => {
-                    builder = builder.set_event_source_mapping_arn(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
-                }
                 "FilterCriteria" => {
                     builder = builder.set_filter_criteria(super::protocol_serde::shape_filter_criteria::de_filter_criteria(
                         tokens,
@@ -188,20 +194,36 @@
                         depth + 1,
                     )?);
                 }
-                "FunctionArn" => {
-                    builder = builder.set_function_arn(
+                "KMSKeyArn" => {
+                    builder = builder.set_kms_key_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "FunctionResponseTypes" => {
-                    builder = builder.set_function_response_types(
-                        super::protocol_serde::shape_function_response_type_list::de_function_response_type_list(tokens, _value, depth + 1)?,
+                "MetricsConfig" => {
+                    builder = builder.set_metrics_config(
+                        super::protocol_serde::shape_event_source_mapping_metrics_config::de_event_source_mapping_metrics_config(
+                            tokens,
+                            _value,
+                            depth + 1,
+                        )?,
+                    );
+                }
+                "LoggingConfig" => {
+                    builder = builder.set_logging_config(
+                        super::protocol_serde::shape_event_source_mapping_logging_config::de_event_source_mapping_logging_config(
+                            tokens,
+                            _value,
+                            depth + 1,
+                        )?,
                     );
                 }
-                "KMSKeyArn" => {
-                    builder = builder.set_kms_key_arn(
+                "ScalingConfig" => {
+                    builder = builder.set_scaling_config(super::protocol_serde::shape_scaling_config::de_scaling_config(tokens, _value, depth + 1)?);
+                }
+                "FunctionArn" => {
+                    builder = builder.set_function_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
@@ -220,22 +242,43 @@
                             .transpose()?,
                     );
                 }
-                "LoggingConfig" => {
-                    builder = builder.set_logging_config(
-                        super::protocol_serde::shape_event_source_mapping_logging_config::de_event_source_mapping_logging_config(
-                            tokens,
-                            _value,
-                            depth + 1,
-                        )?,
+                "State" => {
+                    builder = builder.set_state(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
                     );
                 }
-                "MaximumBatchingWindowInSeconds" => {
-                    builder = builder.set_maximum_batching_window_in_seconds(
-                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
-                            .map(i32::try_from)
+                "StateTransitionReason" => {
+                    builder = builder.set_state_transition_reason(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
+                "DestinationConfig" => {
+                    builder = builder.set_destination_config(super::protocol_serde::shape_destination_config::de_destination_config(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
+                "Topics" => {
+                    builder = builder.set_topics(super::protocol_serde::shape_topics::de_topics(tokens, _value, depth + 1)?);
+                }
+                "Queues" => {
+                    builder = builder.set_queues(super::protocol_serde::shape_queues::de_queues(tokens, _value, depth + 1)?);
+                }
+                "SourceAccessConfigurations" => {
+                    builder = builder.set_source_access_configurations(
+                        super::protocol_serde::shape_source_access_configurations::de_source_access_configurations(tokens, _value, depth + 1)?,
+                    );
+                }
+                "SelfManagedEventSource" => {
+                    builder = builder.set_self_managed_event_source(
+                        super::protocol_serde::shape_self_managed_event_source::de_self_managed_event_source(tokens, _value, depth + 1)?,
+                    );
+                }
                 "MaximumRecordAgeInSeconds" => {
                     builder = builder.set_maximum_record_age_in_seconds(
                         ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
@@ -243,6 +286,9 @@
                             .transpose()?,
                     );
                 }
+                "BisectBatchOnFunctionError" => {
+                    builder = builder.set_bisect_batch_on_function_error(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
+                }
                 "MaximumRetryAttempts" => {
                     builder = builder.set_maximum_retry_attempts(
                         ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
@@ -250,36 +296,25 @@
                             .transpose()?,
                     );
                 }
-                "MetricsConfig" => {
-                    builder = builder.set_metrics_config(
-                        super::protocol_serde::shape_event_source_mapping_metrics_config::de_event_source_mapping_metrics_config(
-                            tokens,
-                            _value,
-                            depth + 1,
-                        )?,
-                    );
-                }
-                "ParallelizationFactor" => {
-                    builder = builder.set_parallelization_factor(
+                "TumblingWindowInSeconds" => {
+                    builder = builder.set_tumbling_window_in_seconds(
                         ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                             .map(i32::try_from)
                             .transpose()?,
                     );
                 }
-                "ProvisionedPollerConfig" => {
-                    builder = builder.set_provisioned_poller_config(
-                        super::protocol_serde::shape_provisioned_poller_config::de_provisioned_poller_config(tokens, _value, depth + 1)?,
+                "FunctionResponseTypes" => {
+                    builder = builder.set_function_response_types(
+                        super::protocol_serde::shape_function_response_type_list::de_function_response_type_list(tokens, _value, depth + 1)?,
                     );
                 }
-                "Queues" => {
-                    builder = builder.set_queues(super::protocol_serde::shape_queues::de_queues(tokens, _value, depth + 1)?);
-                }
-                "ScalingConfig" => {
-                    builder = builder.set_scaling_config(super::protocol_serde::shape_scaling_config::de_scaling_config(tokens, _value, depth + 1)?);
-                }
-                "SelfManagedEventSource" => {
-                    builder = builder.set_self_managed_event_source(
-                        super::protocol_serde::shape_self_managed_event_source::de_self_managed_event_source(tokens, _value, depth + 1)?,
+                "AmazonManagedKafkaEventSourceConfig" => {
+                    builder = builder.set_amazon_managed_kafka_event_source_config(
+                        super::protocol_serde::shape_amazon_managed_kafka_event_source_config::de_amazon_managed_kafka_event_source_config(
+                            tokens,
+                            _value,
+                            depth + 1,
+                        )?,
                     );
                 }
                 "SelfManagedKafkaEventSourceConfig" => {
@@ -291,53 +326,21 @@
                         )?,
                     );
                 }
-                "SourceAccessConfigurations" => {
-                    builder = builder.set_source_access_configurations(
-                        super::protocol_serde::shape_source_access_configurations::de_source_access_configurations(tokens, _value, depth + 1)?,
-                    );
-                }
-                "StartingPosition" => {
-                    builder = builder.set_starting_position(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| super::types::EventSourcePosition::from(u.as_ref())))
-                            .transpose()?,
-                    );
-                }
-                "StartingPositionTimestamp" => {
-                    builder = builder.set_starting_position_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
-                        tokens.next(),
-                        ::aws_smithy_types::date_time::Format::EpochSeconds,
-                    )?);
-                }
-                "State" => {
-                    builder = builder.set_state(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
+                "DocumentDBEventSourceConfig" => {
+                    builder = builder.set_document_db_event_source_config(
+                        super::protocol_serde::shape_document_db_event_source_config::de_document_db_event_source_config(tokens, _value, depth + 1)?,
                     );
                 }
-                "StateTransitionReason" => {
-                    builder = builder.set_state_transition_reason(
+                "EventSourceMappingArn" => {
+                    builder = builder.set_event_source_mapping_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "Topics" => {
-                    builder = builder.set_topics(super::protocol_serde::shape_topics::de_topics(tokens, _value, depth + 1)?);
-                }
-                "TumblingWindowInSeconds" => {
-                    builder = builder.set_tumbling_window_in_seconds(
-                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
-                            .map(i32::try_from)
-                            .transpose()?,
-                    );
-                }
-                "UUID" => {
-                    builder = builder.set_uuid(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
+                "ProvisionedPollerConfig" => {
+                    builder = builder.set_provisioned_poller_config(
+                        super::protocol_serde::shape_provisioned_poller_config::de_provisioned_poller_config(tokens, _value, depth + 1)?,
                     );
                 }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
```

### `src/protocol_serde/shape_get_function.rs`

```diff
--- reference/src/protocol_serde/shape_get_function.rs
+++ generated/src/protocol_serde/shape_get_function.rs
@@ -72,11 +72,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_function::GetFunctionError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::get_function::GetFunctionError::unhandled("Failed to parse retryAfterSeconds from header `Retry-After")
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -105,6 +100,16 @@
     })
 }

+pub fn ser_get_function_input(
+    input: &super::operation::get_function::GetFunctionInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_function_input::ser_get_function_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_get_function(
     _value: &[u8],
     mut builder: super::operation::get_function::builders::GetFunctionOutputBuilder,
@@ -119,18 +124,15 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "Code" => {
-                    builder = builder.set_code(super::protocol_serde::shape_function_code_location::de_function_code_location(
+                "Configuration" => {
+                    builder = builder.set_configuration(super::protocol_serde::shape_function_configuration::de_function_configuration(
                         tokens,
                         _value,
                         depth + 1,
                     )?);
                 }
-                "Concurrency" => {
-                    builder = builder.set_concurrency(super::protocol_serde::shape_concurrency::de_concurrency(tokens, _value, depth + 1)?);
-                }
-                "Configuration" => {
-                    builder = builder.set_configuration(super::protocol_serde::shape_function_configuration::de_function_configuration(
+                "Code" => {
+                    builder = builder.set_code(super::protocol_serde::shape_function_code_location::de_function_code_location(
                         tokens,
                         _value,
                         depth + 1,
@@ -142,6 +144,9 @@
                 "TagsError" => {
                     builder = builder.set_tags_error(super::protocol_serde::shape_tags_error::de_tags_error(tokens, _value, depth + 1)?);
                 }
+                "Concurrency" => {
+                    builder = builder.set_concurrency(super::protocol_serde::shape_concurrency::de_concurrency(tokens, _value, depth + 1)?);
+                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_get_function_code_signing_config.rs`

```diff
--- reference/src/protocol_serde/shape_get_function_code_signing_config.rs
+++ generated/src/protocol_serde/shape_get_function_code_signing_config.rs
@@ -103,13 +103,6 @@
                     output =
                         super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                             .map_err(super::operation::get_function_code_signing_config::GetFunctionCodeSigningConfigError::unhandled)?;
-                    output = output.set_retry_after_seconds(
-                        super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                            super::operation::get_function_code_signing_config::GetFunctionCodeSigningConfigError::unhandled(
-                                "Failed to parse retryAfterSeconds from header `Retry-After",
-                            )
-                        })?,
-                    );
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -144,6 +137,16 @@
     })
 }

+pub fn ser_get_function_code_signing_config_input(
+    input: &super::operation::get_function_code_signing_config::GetFunctionCodeSigningConfigInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_function_code_signing_config_input::ser_get_function_code_signing_config_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_get_function_code_signing_config(
     _value: &[u8],
     mut builder: super::operation::get_function_code_signing_config::builders::GetFunctionCodeSigningConfigOutputBuilder,
```

### `src/protocol_serde/shape_get_function_concurrency.rs`

```diff
--- reference/src/protocol_serde/shape_get_function_concurrency.rs
+++ generated/src/protocol_serde/shape_get_function_concurrency.rs
@@ -81,13 +81,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_function_concurrency::GetFunctionConcurrencyError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::get_function_concurrency::GetFunctionConcurrencyError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -119,6 +112,16 @@
     })
 }

+pub fn ser_get_function_concurrency_input(
+    input: &super::operation::get_function_concurrency::GetFunctionConcurrencyInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_function_concurrency_input::ser_get_function_concurrency_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_get_function_concurrency(
     _value: &[u8],
     mut builder: super::operation::get_function_concurrency::builders::GetFunctionConcurrencyOutputBuilder,
```

### `src/protocol_serde/shape_get_function_configuration.rs`

```diff
--- reference/src/protocol_serde/shape_get_function_configuration.rs
+++ generated/src/protocol_serde/shape_get_function_configuration.rs
@@ -81,13 +81,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_function_configuration::GetFunctionConfigurationError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::get_function_configuration::GetFunctionConfigurationError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -119,6 +112,16 @@
     })
 }

+pub fn ser_get_function_configuration_input(
+    input: &super::operation::get_function_configuration::GetFunctionConfigurationInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_function_configuration_input::ser_get_function_configuration_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_get_function_configuration(
     _value: &[u8],
     mut builder: super::operation::get_function_configuration::builders::GetFunctionConfigurationOutputBuilder,
@@ -135,20 +138,36 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "Architectures" => {
-                    builder = builder.set_architectures(super::protocol_serde::shape_architectures_list::de_architectures_list(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
+                "FunctionName" => {
+                    builder = builder.set_function_name(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
+                }
+                "FunctionArn" => {
+                    builder = builder.set_function_arn(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
+                }
+                "Runtime" => {
+                    builder = builder.set_runtime(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| super::types::Runtime::from(u.as_ref())))
+                            .transpose()?,
+                    );
                 }
-                "CapacityProviderConfig" => {
-                    builder = builder.set_capacity_provider_config(
-                        super::protocol_serde::shape_capacity_provider_config::de_capacity_provider_config(tokens, _value, depth + 1)?,
+                "Role" => {
+                    builder = builder.set_role(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
                     );
                 }
-                "CodeSha256" => {
-                    builder = builder.set_code_sha256(
+                "Handler" => {
+                    builder = builder.set_handler(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
@@ -161,93 +180,125 @@
                             .transpose()?,
                     );
                 }
-                "ConfigSha256" => {
-                    builder = builder.set_config_sha256(
+                "Description" => {
+                    builder = builder.set_description(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "DeadLetterConfig" => {
-                    builder = builder.set_dead_letter_config(super::protocol_serde::shape_dead_letter_config::de_dead_letter_config(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
+                "Timeout" => {
+                    builder = builder.set_timeout(
+                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                            .map(i32::try_from)
+                            .transpose()?,
+                    );
+                }
+                "MemorySize" => {
+                    builder = builder.set_memory_size(
+                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                            .map(i32::try_from)
+                            .transpose()?,
+                    );
+                }
+                "LastModified" => {
+                    builder = builder.set_last_modified(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
                 }
-                "Description" => {
-                    builder = builder.set_description(
+                "CodeSha256" => {
+                    builder = builder.set_code_sha256(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "DurableConfig" => {
-                    builder = builder.set_durable_config(super::protocol_serde::shape_durable_config::de_durable_config(tokens, _value, depth + 1)?);
+                "Version" => {
+                    builder = builder.set_version(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
                 }
-                "Environment" => {
-                    builder = builder.set_environment(super::protocol_serde::shape_environment_response::de_environment_response(
+                "VpcConfig" => {
+                    builder = builder.set_vpc_config(super::protocol_serde::shape_vpc_config_response::de_vpc_config_response(
                         tokens,
                         _value,
                         depth + 1,
                     )?);
                 }
-                "EphemeralStorage" => {
-                    builder = builder.set_ephemeral_storage(super::protocol_serde::shape_ephemeral_storage::de_ephemeral_storage(
+                "DeadLetterConfig" => {
+                    builder = builder.set_dead_letter_config(super::protocol_serde::shape_dead_letter_config::de_dead_letter_config(
                         tokens,
                         _value,
                         depth + 1,
                     )?);
                 }
-                "FileSystemConfigs" => {
-                    builder = builder.set_file_system_configs(super::protocol_serde::shape_file_system_config_list::de_file_system_config_list(
+                "Environment" => {
+                    builder = builder.set_environment(super::protocol_serde::shape_environment_response::de_environment_response(
                         tokens,
                         _value,
                         depth + 1,
                     )?);
                 }
-                "FunctionArn" => {
-                    builder = builder.set_function_arn(
+                "KMSKeyArn" => {
+                    builder = builder.set_kms_key_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "FunctionName" => {
-                    builder = builder.set_function_name(
+                "TracingConfig" => {
+                    builder = builder.set_tracing_config(super::protocol_serde::shape_tracing_config_response::de_tracing_config_response(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
+                "MasterArn" => {
+                    builder = builder.set_master_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "Handler" => {
-                    builder = builder.set_handler(
+                "RevisionId" => {
+                    builder = builder.set_revision_id(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "ImageConfigResponse" => {
-                    builder = builder.set_image_config_response(super::protocol_serde::shape_image_config_response::de_image_config_response(
+                "Layers" => {
+                    builder = builder.set_layers(super::protocol_serde::shape_layers_reference_list::de_layers_reference_list(
                         tokens,
                         _value,
                         depth + 1,
                     )?);
                 }
-                "KMSKeyArn" => {
-                    builder = builder.set_kms_key_arn(
+                "State" => {
+                    builder = builder.set_state(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .map(|s| s.to_unescaped().map(|u| super::types::State::from(u.as_ref())))
                             .transpose()?,
                     );
                 }
-                "LastModified" => {
-                    builder = builder.set_last_modified(
+                "StateReason" => {
+                    builder = builder.set_state_reason(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
+                "StateReasonCode" => {
+                    builder = builder.set_state_reason_code(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| super::types::StateReasonCode::from(u.as_ref())))
+                            .transpose()?,
+                    );
+                }
                 "LastUpdateStatus" => {
                     builder = builder.set_last_update_status(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -269,27 +320,24 @@
                             .transpose()?,
                     );
                 }
-                "Layers" => {
-                    builder = builder.set_layers(super::protocol_serde::shape_layers_reference_list::de_layers_reference_list(
+                "FileSystemConfigs" => {
+                    builder = builder.set_file_system_configs(super::protocol_serde::shape_file_system_config_list::de_file_system_config_list(
                         tokens,
                         _value,
                         depth + 1,
                     )?);
                 }
-                "LoggingConfig" => {
-                    builder = builder.set_logging_config(super::protocol_serde::shape_logging_config::de_logging_config(tokens, _value, depth + 1)?);
-                }
-                "MasterArn" => {
-                    builder = builder.set_master_arn(
+                "SigningProfileVersionArn" => {
+                    builder = builder.set_signing_profile_version_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "MemorySize" => {
-                    builder = builder.set_memory_size(
-                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
-                            .map(i32::try_from)
+                "SigningJobArn" => {
+                    builder = builder.set_signing_job_arn(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
@@ -300,47 +348,26 @@
                             .transpose()?,
                     );
                 }
-                "RevisionId" => {
-                    builder = builder.set_revision_id(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
-                }
-                "Role" => {
-                    builder = builder.set_role(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
-                }
-                "Runtime" => {
-                    builder = builder.set_runtime(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| super::types::Runtime::from(u.as_ref())))
-                            .transpose()?,
-                    );
-                }
-                "RuntimeVersionConfig" => {
-                    builder = builder.set_runtime_version_config(super::protocol_serde::shape_runtime_version_config::de_runtime_version_config(
+                "ImageConfigResponse" => {
+                    builder = builder.set_image_config_response(super::protocol_serde::shape_image_config_response::de_image_config_response(
                         tokens,
                         _value,
                         depth + 1,
                     )?);
                 }
-                "SigningJobArn" => {
-                    builder = builder.set_signing_job_arn(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
+                "Architectures" => {
+                    builder = builder.set_architectures(super::protocol_serde::shape_architectures_list::de_architectures_list(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
                 }
-                "SigningProfileVersionArn" => {
-                    builder = builder.set_signing_profile_version_arn(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
+                "EphemeralStorage" => {
+                    builder = builder.set_ephemeral_storage(super::protocol_serde::shape_ephemeral_storage::de_ephemeral_storage(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
                 }
                 "SnapStart" => {
                     builder = builder.set_snap_start(super::protocol_serde::shape_snap_start_response::de_snap_start_response(
@@ -349,57 +376,33 @@
                         depth + 1,
                     )?);
                 }
-                "State" => {
-                    builder = builder.set_state(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| super::types::State::from(u.as_ref())))
-                            .transpose()?,
-                    );
-                }
-                "StateReason" => {
-                    builder = builder.set_state_reason(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
+                "RuntimeVersionConfig" => {
+                    builder = builder.set_runtime_version_config(super::protocol_serde::shape_runtime_version_config::de_runtime_version_config(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
                 }
-                "StateReasonCode" => {
-                    builder = builder.set_state_reason_code(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| super::types::StateReasonCode::from(u.as_ref())))
-                            .transpose()?,
-                    );
+                "LoggingConfig" => {
+                    builder = builder.set_logging_config(super::protocol_serde::shape_logging_config::de_logging_config(tokens, _value, depth + 1)?);
                 }
                 "TenancyConfig" => {
                     builder = builder.set_tenancy_config(super::protocol_serde::shape_tenancy_config::de_tenancy_config(tokens, _value, depth + 1)?);
                 }
-                "Timeout" => {
-                    builder = builder.set_timeout(
-                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
-                            .map(i32::try_from)
-                            .transpose()?,
+                "CapacityProviderConfig" => {
+                    builder = builder.set_capacity_provider_config(
+                        super::protocol_serde::shape_capacity_provider_config::de_capacity_provider_config(tokens, _value, depth + 1)?,
                     );
                 }
-                "TracingConfig" => {
-                    builder = builder.set_tracing_config(super::protocol_serde::shape_tracing_config_response::de_tracing_config_response(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
-                }
-                "Version" => {
-                    builder = builder.set_version(
+                "ConfigSha256" => {
+                    builder = builder.set_config_sha256(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "VpcConfig" => {
-                    builder = builder.set_vpc_config(super::protocol_serde::shape_vpc_config_response::de_vpc_config_response(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
+                "DurableConfig" => {
+                    builder = builder.set_durable_config(super::protocol_serde::shape_durable_config::de_durable_config(tokens, _value, depth + 1)?);
                 }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
```

### `src/protocol_serde/shape_get_function_event_invoke_config.rs`

```diff
--- reference/src/protocol_serde/shape_get_function_event_invoke_config.rs
+++ generated/src/protocol_serde/shape_get_function_event_invoke_config.rs
@@ -82,13 +82,6 @@
                     output =
                         super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                             .map_err(super::operation::get_function_event_invoke_config::GetFunctionEventInvokeConfigError::unhandled)?;
-                    output = output.set_retry_after_seconds(
-                        super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                            super::operation::get_function_event_invoke_config::GetFunctionEventInvokeConfigError::unhandled(
-                                "Failed to parse retryAfterSeconds from header `Retry-After",
-                            )
-                        })?,
-                    );
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -121,6 +114,16 @@
     })
 }

+pub fn ser_get_function_event_invoke_config_input(
+    input: &super::operation::get_function_event_invoke_config::GetFunctionEventInvokeConfigInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_function_event_invoke_config_input::ser_get_function_event_invoke_config_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_get_function_event_invoke_config(
     _value: &[u8],
     mut builder: super::operation::get_function_event_invoke_config::builders::GetFunctionEventInvokeConfigOutputBuilder,
@@ -137,11 +140,10 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "DestinationConfig" => {
-                    builder = builder.set_destination_config(super::protocol_serde::shape_destination_config::de_destination_config(
-                        tokens,
-                        _value,
-                        depth + 1,
+                "LastModified" => {
+                    builder = builder.set_last_modified(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
+                        tokens.next(),
+                        ::aws_smithy_types::date_time::Format::EpochSeconds,
                     )?);
                 }
                 "FunctionArn" => {
@@ -151,26 +153,27 @@
                             .transpose()?,
                     );
                 }
-                "LastModified" => {
-                    builder = builder.set_last_modified(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
-                        tokens.next(),
-                        ::aws_smithy_types::date_time::Format::EpochSeconds,
-                    )?);
-                }
-                "MaximumEventAgeInSeconds" => {
-                    builder = builder.set_maximum_event_age_in_seconds(
+                "MaximumRetryAttempts" => {
+                    builder = builder.set_maximum_retry_attempts(
                         ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                             .map(i32::try_from)
                             .transpose()?,
                     );
                 }
-                "MaximumRetryAttempts" => {
-                    builder = builder.set_maximum_retry_attempts(
+                "MaximumEventAgeInSeconds" => {
+                    builder = builder.set_maximum_event_age_in_seconds(
                         ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                             .map(i32::try_from)
                             .transpose()?,
                     );
                 }
+                "DestinationConfig" => {
+                    builder = builder.set_destination_config(super::protocol_serde::shape_destination_config::de_destination_config(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_get_function_recursion_config.rs`

```diff
--- reference/src/protocol_serde/shape_get_function_recursion_config.rs
+++ generated/src/protocol_serde/shape_get_function_recursion_config.rs
@@ -77,13 +77,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_function_recursion_config::GetFunctionRecursionConfigError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::get_function_recursion_config::GetFunctionRecursionConfigError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -115,6 +108,16 @@
     })
 }

+pub fn ser_get_function_recursion_config_input(
+    input: &super::operation::get_function_recursion_config::GetFunctionRecursionConfigInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_function_recursion_config_input::ser_get_function_recursion_config_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_get_function_recursion_config(
     _value: &[u8],
     mut builder: super::operation::get_function_recursion_config::builders::GetFunctionRecursionConfigOutputBuilder,
```

### `src/protocol_serde/shape_get_function_scaling_config.rs`

```diff
--- reference/src/protocol_serde/shape_get_function_scaling_config.rs
+++ generated/src/protocol_serde/shape_get_function_scaling_config.rs
@@ -81,13 +81,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_function_scaling_config::GetFunctionScalingConfigError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::get_function_scaling_config::GetFunctionScalingConfigError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -119,6 +112,16 @@
     })
 }

+pub fn ser_get_function_scaling_config_input(
+    input: &super::operation::get_function_scaling_config::GetFunctionScalingConfigInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_function_scaling_config_input::ser_get_function_scaling_config_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_get_function_scaling_config(
     _value: &[u8],
     mut builder: super::operation::get_function_scaling_config::builders::GetFunctionScalingConfigOutputBuilder,
@@ -135,11 +138,6 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "AppliedFunctionScalingConfig" => {
-                    builder = builder.set_applied_function_scaling_config(
-                        super::protocol_serde::shape_function_scaling_config::de_function_scaling_config(tokens, _value, depth + 1)?,
-                    );
-                }
                 "FunctionArn" => {
                     builder = builder.set_function_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -147,6 +145,11 @@
                             .transpose()?,
                     );
                 }
+                "AppliedFunctionScalingConfig" => {
+                    builder = builder.set_applied_function_scaling_config(
+                        super::protocol_serde::shape_function_scaling_config::de_function_scaling_config(tokens, _value, depth + 1)?,
+                    );
+                }
                 "RequestedFunctionScalingConfig" => {
                     builder = builder.set_requested_function_scaling_config(
                         super::protocol_serde::shape_function_scaling_config::de_function_scaling_config(tokens, _value, depth + 1)?,
```

### `src/protocol_serde/shape_get_function_url_config.rs`

```diff
--- reference/src/protocol_serde/shape_get_function_url_config.rs
+++ generated/src/protocol_serde/shape_get_function_url_config.rs
@@ -75,13 +75,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_function_url_config::GetFunctionUrlConfigError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::get_function_url_config::GetFunctionUrlConfigError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -115,6 +108,16 @@
     })
 }

+pub fn ser_get_function_url_config_input(
+    input: &super::operation::get_function_url_config::GetFunctionUrlConfigInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_function_url_config_input::ser_get_function_url_config_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_get_function_url_config(
     _value: &[u8],
     mut builder: super::operation::get_function_url_config::builders::GetFunctionUrlConfigOutputBuilder,
@@ -131,6 +134,20 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                "FunctionUrl" => {
+                    builder = builder.set_function_url(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
+                }
+                "FunctionArn" => {
+                    builder = builder.set_function_arn(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
+                }
                 "AuthType" => {
                     builder = builder.set_auth_type(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -148,15 +165,8 @@
                             .transpose()?,
                     );
                 }
-                "FunctionArn" => {
-                    builder = builder.set_function_arn(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
-                }
-                "FunctionUrl" => {
-                    builder = builder.set_function_url(
+                "LastModifiedTime" => {
+                    builder = builder.set_last_modified_time(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
@@ -169,13 +179,6 @@
                             .transpose()?,
                     );
                 }
-                "LastModifiedTime" => {
-                    builder = builder.set_last_modified_time(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
-                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_get_layer_version.rs`

```diff
--- reference/src/protocol_serde/shape_get_layer_version.rs
+++ generated/src/protocol_serde/shape_get_layer_version.rs
@@ -72,13 +72,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_layer_version::GetLayerVersionError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::get_layer_version::GetLayerVersionError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -107,6 +100,16 @@
     })
 }

+pub fn ser_get_layer_version_input(
+    input: &super::operation::get_layer_version::GetLayerVersionInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_layer_version_input::ser_get_layer_version_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_get_layer_version(
     _value: &[u8],
     mut builder: super::operation::get_layer_version::builders::GetLayerVersionOutputBuilder,
@@ -123,53 +126,34 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "CompatibleArchitectures" => {
-                    builder = builder.set_compatible_architectures(
-                        super::protocol_serde::shape_compatible_architectures::de_compatible_architectures(tokens, _value, depth + 1)?,
-                    );
-                }
-                "CompatibleRuntimes" => {
-                    builder = builder.set_compatible_runtimes(super::protocol_serde::shape_compatible_runtimes::de_compatible_runtimes(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
-                }
                 "Content" => {
                     builder = builder.set_content(
                         super::protocol_serde::shape_layer_version_content_output::de_layer_version_content_output(tokens, _value, depth + 1)?,
                     );
                 }
-                "CreatedDate" => {
-                    builder = builder.set_created_date(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
-                }
-                "Description" => {
-                    builder = builder.set_description(
+                "LayerArn" => {
+                    builder = builder.set_layer_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "LayerArn" => {
-                    builder = builder.set_layer_arn(
+                "LayerVersionArn" => {
+                    builder = builder.set_layer_version_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "LayerVersionArn" => {
-                    builder = builder.set_layer_version_arn(
+                "Description" => {
+                    builder = builder.set_description(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "LicenseInfo" => {
-                    builder = builder.set_license_info(
+                "CreatedDate" => {
+                    builder = builder.set_created_date(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
@@ -182,6 +166,25 @@
                             .transpose()?,
                     );
                 }
+                "CompatibleArchitectures" => {
+                    builder = builder.set_compatible_architectures(
+                        super::protocol_serde::shape_compatible_architectures::de_compatible_architectures(tokens, _value, depth + 1)?,
+                    );
+                }
+                "CompatibleRuntimes" => {
+                    builder = builder.set_compatible_runtimes(super::protocol_serde::shape_compatible_runtimes::de_compatible_runtimes(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
+                "LicenseInfo" => {
+                    builder = builder.set_license_info(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
+                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_get_layer_version_by_arn.rs`

```diff
--- reference/src/protocol_serde/shape_get_layer_version_by_arn.rs
+++ generated/src/protocol_serde/shape_get_layer_version_by_arn.rs
@@ -75,13 +75,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_layer_version_by_arn::GetLayerVersionByArnError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::get_layer_version_by_arn::GetLayerVersionByArnError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -113,6 +106,16 @@
     })
 }

+pub fn ser_get_layer_version_by_arn_input(
+    input: &super::operation::get_layer_version_by_arn::GetLayerVersionByArnInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_layer_version_by_arn_input::ser_get_layer_version_by_arn_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_get_layer_version_by_arn(
     _value: &[u8],
     mut builder: super::operation::get_layer_version_by_arn::builders::GetLayerVersionByArnOutputBuilder,
@@ -129,53 +132,34 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "CompatibleArchitectures" => {
-                    builder = builder.set_compatible_architectures(
-                        super::protocol_serde::shape_compatible_architectures::de_compatible_architectures(tokens, _value, depth + 1)?,
-                    );
-                }
-                "CompatibleRuntimes" => {
-                    builder = builder.set_compatible_runtimes(super::protocol_serde::shape_compatible_runtimes::de_compatible_runtimes(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
-                }
                 "Content" => {
                     builder = builder.set_content(
                         super::protocol_serde::shape_layer_version_content_output::de_layer_version_content_output(tokens, _value, depth + 1)?,
                     );
                 }
-                "CreatedDate" => {
-                    builder = builder.set_created_date(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
-                }
-                "Description" => {
-                    builder = builder.set_description(
+                "LayerArn" => {
+                    builder = builder.set_layer_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "LayerArn" => {
-                    builder = builder.set_layer_arn(
+                "LayerVersionArn" => {
+                    builder = builder.set_layer_version_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "LayerVersionArn" => {
-                    builder = builder.set_layer_version_arn(
+                "Description" => {
+                    builder = builder.set_description(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "LicenseInfo" => {
-                    builder = builder.set_license_info(
+                "CreatedDate" => {
+                    builder = builder.set_created_date(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
@@ -188,6 +172,25 @@
                             .transpose()?,
                     );
                 }
+                "CompatibleArchitectures" => {
+                    builder = builder.set_compatible_architectures(
+                        super::protocol_serde::shape_compatible_architectures::de_compatible_architectures(tokens, _value, depth + 1)?,
+                    );
+                }
+                "CompatibleRuntimes" => {
+                    builder = builder.set_compatible_runtimes(super::protocol_serde::shape_compatible_runtimes::de_compatible_runtimes(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
+                "LicenseInfo" => {
+                    builder = builder.set_license_info(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
+                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_get_layer_version_policy.rs`

```diff
--- reference/src/protocol_serde/shape_get_layer_version_policy.rs
+++ generated/src/protocol_serde/shape_get_layer_version_policy.rs
@@ -75,13 +75,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_layer_version_policy::GetLayerVersionPolicyError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::get_layer_version_policy::GetLayerVersionPolicyError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -113,6 +106,16 @@
     })
 }

+pub fn ser_get_layer_version_policy_input(
+    input: &super::operation::get_layer_version_policy::GetLayerVersionPolicyInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_layer_version_policy_input::ser_get_layer_version_policy_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_get_layer_version_policy(
     _value: &[u8],
     mut builder: super::operation::get_layer_version_policy::builders::GetLayerVersionPolicyOutputBuilder,
```

### `src/protocol_serde/shape_get_policy.rs`

```diff
--- reference/src/protocol_serde/shape_get_policy.rs
+++ generated/src/protocol_serde/shape_get_policy.rs
@@ -72,11 +72,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_policy::GetPolicyError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::get_policy::GetPolicyError::unhandled("Failed to parse retryAfterSeconds from header `Retry-After")
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -105,6 +100,16 @@
     })
 }

+pub fn ser_get_policy_input(
+    input: &super::operation::get_policy::GetPolicyInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_policy_input::ser_get_policy_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_get_policy(
     _value: &[u8],
     mut builder: super::operation::get_policy::builders::GetPolicyOutputBuilder,
```

### `src/protocol_serde/shape_get_provisioned_concurrency_config.rs`

```diff
--- reference/src/protocol_serde/shape_get_provisioned_concurrency_config.rs
+++ generated/src/protocol_serde/shape_get_provisioned_concurrency_config.rs
@@ -100,13 +100,6 @@
                     output =
                         super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                             .map_err(super::operation::get_provisioned_concurrency_config::GetProvisionedConcurrencyConfigError::unhandled)?;
-                    output = output.set_retry_after_seconds(
-                        super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                            super::operation::get_provisioned_concurrency_config::GetProvisionedConcurrencyConfigError::unhandled(
-                                "Failed to parse retryAfterSeconds from header `Retry-After",
-                            )
-                        })?,
-                    );
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -139,6 +132,16 @@
     })
 }

+pub fn ser_get_provisioned_concurrency_config_input(
+    input: &super::operation::get_provisioned_concurrency_config::GetProvisionedConcurrencyConfigInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_provisioned_concurrency_config_input::ser_get_provisioned_concurrency_config_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_get_provisioned_concurrency_config(
     _value: &[u8],
     mut builder: super::operation::get_provisioned_concurrency_config::builders::GetProvisionedConcurrencyConfigOutputBuilder,
@@ -155,8 +158,8 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "AllocatedProvisionedConcurrentExecutions" => {
-                    builder = builder.set_allocated_provisioned_concurrent_executions(
+                "RequestedProvisionedConcurrentExecutions" => {
+                    builder = builder.set_requested_provisioned_concurrent_executions(
                         ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                             .map(i32::try_from)
                             .transpose()?,
@@ -169,15 +172,8 @@
                             .transpose()?,
                     );
                 }
-                "LastModified" => {
-                    builder = builder.set_last_modified(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
-                }
-                "RequestedProvisionedConcurrentExecutions" => {
-                    builder = builder.set_requested_provisioned_concurrent_executions(
+                "AllocatedProvisionedConcurrentExecutions" => {
+                    builder = builder.set_allocated_provisioned_concurrent_executions(
                         ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                             .map(i32::try_from)
                             .transpose()?,
@@ -197,6 +193,13 @@
                             .transpose()?,
                     );
                 }
+                "LastModified" => {
+                    builder = builder.set_last_modified(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
+                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_get_resource_policy.rs`

```diff
--- reference/src/protocol_serde/shape_get_resource_policy.rs
+++ generated/src/protocol_serde/shape_get_resource_policy.rs
@@ -73,13 +73,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_resource_policy::GetResourcePolicyError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::get_resource_policy::GetResourcePolicyError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -109,6 +102,16 @@
     })
 }

+pub fn ser_get_resource_policy_input(
+    input: &super::operation::get_resource_policy::GetResourcePolicyInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_resource_policy_input::ser_get_resource_policy_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_get_resource_policy(
     _value: &[u8],
     mut builder: super::operation::get_resource_policy::builders::GetResourcePolicyOutputBuilder,
```

### `src/protocol_serde/shape_get_runtime_management_config.rs`

```diff
--- reference/src/protocol_serde/shape_get_runtime_management_config.rs
+++ generated/src/protocol_serde/shape_get_runtime_management_config.rs
@@ -77,13 +77,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::get_runtime_management_config::GetRuntimeManagementConfigError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::get_runtime_management_config::GetRuntimeManagementConfigError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -115,6 +108,16 @@
     })
 }

+pub fn ser_get_runtime_management_config_input(
+    input: &super::operation::get_runtime_management_config::GetRuntimeManagementConfigInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_get_runtime_management_config_input::ser_get_runtime_management_config_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_get_runtime_management_config(
     _value: &[u8],
     mut builder: super::operation::get_runtime_management_config::builders::GetRuntimeManagementConfigOutputBuilder,
@@ -131,6 +134,13 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                "UpdateRuntimeOn" => {
+                    builder = builder.set_update_runtime_on(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| super::types::UpdateRuntimeOn::from(u.as_ref())))
+                            .transpose()?,
+                    );
+                }
                 "FunctionArn" => {
                     builder = builder.set_function_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -145,13 +155,6 @@
                             .transpose()?,
                     );
                 }
-                "UpdateRuntimeOn" => {
-                    builder = builder.set_update_runtime_on(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| super::types::UpdateRuntimeOn::from(u.as_ref())))
-                            .transpose()?,
-                    );
-                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_invoke.rs`

```diff
--- reference/src/protocol_serde/shape_invoke.rs
+++ generated/src/protocol_serde/shape_invoke.rs
@@ -675,11 +675,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::invoke::InvokeError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::invoke::InvokeError::unhandled("Failed to parse retryAfterSeconds from header `Retry-After")
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -719,89 +714,86 @@
     Ok({
         #[allow(unused_mut)]
         let mut output = super::operation::invoke::builders::InvokeOutputBuilder::default();
-        output = output.set_durable_execution_arn(
-            super::protocol_serde::shape_invoke_output::de_durable_execution_arn_header(_response_headers).map_err(|_| {
-                super::operation::invoke::InvokeError::unhandled("Failed to parse DurableExecutionArn from header `X-Amz-Durable-Execution-Arn")
-            })?,
-        );
-        output = output.set_executed_version(
-            super::protocol_serde::shape_invoke_output::de_executed_version_header(_response_headers).map_err(|_| {
-                super::operation::invoke::InvokeError::unhandled("Failed to parse ExecutedVersion from header `X-Amz-Executed-Version")
-            })?,
-        );
-        output = output.set_function_error(
-            super::protocol_serde::shape_invoke_output::de_function_error_header(_response_headers)
-                .map_err(|_| super::operation::invoke::InvokeError::unhandled("Failed to parse FunctionError from header `X-Amz-Function-Error"))?,
-        );
-        output = output.set_log_result(
-            super::protocol_serde::shape_invoke_output::de_log_result_header(_response_headers)
-                .map_err(|_| super::operation::invoke::InvokeError::unhandled("Failed to parse LogResult from header `X-Amz-Log-Result"))?,
-        );
-        output = output.set_payload(super::protocol_serde::shape_invoke_output::de_payload_payload(_response_body)?);
-        output = output.set_status_code(Some(_response_status as _));
+        output = super::protocol_serde::shape_invoke::de_invoke(_response_body, output).map_err(super::operation::invoke::InvokeError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
         output.build()
     })
 }

-pub fn ser_invoke_headers(
+pub fn ser_invoke_input(
     input: &super::operation::invoke::InvokeInput,
-    mut builder: ::http_1x::request::Builder,
-) -> std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
-    if let ::std::option::Option::Some(inner_1) = &input.invocation_type {
-        let formatted_2 = inner_1.as_str();
-        let header_value = formatted_2;
-        let header_value: ::http_1x::HeaderValue = header_value.parse().map_err(|err| {
-            ::aws_smithy_types::error::operation::BuildError::invalid_field(
-                "invocation_type",
-                format!("`{}` cannot be used as a header value: {}", &header_value, err),
-            )
-        })?;
-        builder = builder.header("X-Amz-Invocation-Type", header_value);
-    }
-    if let ::std::option::Option::Some(inner_3) = &input.log_type {
-        let formatted_4 = inner_3.as_str();
-        let header_value = formatted_4;
-        let header_value: ::http_1x::HeaderValue = header_value.parse().map_err(|err| {
-            ::aws_smithy_types::error::operation::BuildError::invalid_field(
-                "log_type",
-                format!("`{}` cannot be used as a header value: {}", &header_value, err),
-            )
-        })?;
-        builder = builder.header("X-Amz-Log-Type", header_value);
-    }
-    if let ::std::option::Option::Some(inner_5) = &input.client_context {
-        let formatted_6 = inner_5.as_str();
-        let header_value = formatted_6;
-        let header_value: ::http_1x::HeaderValue = header_value.parse().map_err(|err| {
-            ::aws_smithy_types::error::operation::BuildError::invalid_field(
-                "client_context",
-                format!("`{}` cannot be used as a header value: {}", &header_value, err),
-            )
-        })?;
-        builder = builder.header("X-Amz-Client-Context", header_value);
-    }
-    if let ::std::option::Option::Some(inner_7) = &input.durable_execution_name {
-        let formatted_8 = inner_7.as_str();
-        let header_value = formatted_8;
-        let header_value: ::http_1x::HeaderValue = header_value.parse().map_err(|err| {
-            ::aws_smithy_types::error::operation::BuildError::invalid_field(
-                "durable_execution_name",
-                format!("`{}` cannot be used as a header value: {}", &header_value, err),
-            )
-        })?;
-        builder = builder.header("X-Amz-Durable-Execution-Name", header_value);
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_invoke_input::ser_invoke_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
+pub(crate) fn de_invoke(
+    _value: &[u8],
+    mut builder: super::operation::invoke::builders::InvokeOutputBuilder,
+) -> ::std::result::Result<super::operation::invoke::builders::InvokeOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                "StatusCode" => {
+                    builder = builder.set_status_code(
+                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                            .map(i32::try_from)
+                            .transpose()?,
+                    );
+                }
+                "FunctionError" => {
+                    builder = builder.set_function_error(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
+                }
+                "LogResult" => {
+                    builder = builder.set_log_result(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
+                }
+                "Payload" => {
+                    builder = builder.set_payload(::aws_smithy_json::deserialize::token::expect_blob_or_null(tokens.next())?);
+                }
+                "ExecutedVersion" => {
+                    builder = builder.set_executed_version(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
+                }
+                "DurableExecutionArn" => {
+                    builder = builder.set_durable_execution_arn(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
+                }
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
     }
-    if let ::std::option::Option::Some(inner_9) = &input.tenant_id {
-        let formatted_10 = inner_9.as_str();
-        let header_value = formatted_10;
-        let header_value: ::http_1x::HeaderValue = header_value.parse().map_err(|err| {
-            ::aws_smithy_types::error::operation::BuildError::invalid_field(
-                "tenant_id",
-                format!("`{}` cannot be used as a header value: {}", &header_value, err),
-            )
-        })?;
-        builder = builder.header("X-Amz-Tenant-Id", header_value);
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
     }
     Ok(builder)
 }
```

### `src/protocol_serde/shape_invoke_async.rs`

```diff
--- reference/src/protocol_serde/shape_invoke_async.rs
+++ generated/src/protocol_serde/shape_invoke_async.rs
@@ -489,8 +489,57 @@
     Ok({
         #[allow(unused_mut)]
         let mut output = super::operation::invoke_async::builders::InvokeAsyncOutputBuilder::default();
-        output = output.set_status(Some(_response_status as _));
+        output = super::protocol_serde::shape_invoke_async::de_invoke_async(_response_body, output)
+            .map_err(super::operation::invoke_async::InvokeAsyncError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
         output.build()
     })
 }
+
+pub fn ser_invoke_async_input(
+    input: &super::operation::invoke_async::InvokeAsyncInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_invoke_async_input::ser_invoke_async_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
+pub(crate) fn de_invoke_async(
+    _value: &[u8],
+    mut builder: super::operation::invoke_async::builders::InvokeAsyncOutputBuilder,
+) -> ::std::result::Result<super::operation::invoke_async::builders::InvokeAsyncOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
+{
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                "Status" => {
+                    builder = builder.set_status(
+                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                            .map(i32::try_from)
+                            .transpose()?,
+                    );
+                }
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_invoke_async_input.rs`

```diff
--- reference/src/protocol_serde/shape_invoke_async_input.rs
+++ generated/src/protocol_serde/shape_invoke_async_input.rs
@@ -1,6 +1,13 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-pub fn ser_invoke_args_http_payload(
-    payload: ::aws_smithy_types::byte_stream::ByteStream,
-) -> ::std::result::Result<::aws_smithy_types::byte_stream::ByteStream, ::aws_smithy_types::error::operation::BuildError> {
-    Ok(payload)
+pub fn ser_invoke_async_input_input(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &crate::operation::invoke_async::InvokeAsyncInput,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    if let Some(var_1) = &input.function_name {
+        object.key("FunctionName").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.invoke_args {
+        object.key("InvokeArgs").string_unchecked(&::aws_smithy_types::base64::encode(var_2));
+    }
+    Ok(())
 }
```

### `src/protocol_serde/shape_invoke_input.rs`

```diff
--- reference/src/protocol_serde/shape_invoke_input.rs
+++ generated/src/protocol_serde/shape_invoke_input.rs
@@ -1,10 +1,31 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-pub fn ser_payload_http_payload(
-    payload: ::std::option::Option<::aws_smithy_types::Blob>,
-) -> ::std::result::Result<::bytes::Bytes, ::aws_smithy_types::error::operation::BuildError> {
-    let payload = match payload {
-        Some(t) => t,
-        None => return Ok(::bytes::Bytes::new()),
-    };
-    Ok(::aws_smithy_types::Blob::from(payload).into_bytes())
+pub fn ser_invoke_input_input(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &crate::operation::invoke::InvokeInput,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    if let Some(var_1) = &input.function_name {
+        object.key("FunctionName").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.invocation_type {
+        object.key("InvocationType").string(var_2.as_str());
+    }
+    if let Some(var_3) = &input.log_type {
+        object.key("LogType").string(var_3.as_str());
+    }
+    if let Some(var_4) = &input.client_context {
+        object.key("ClientContext").string(var_4.as_str());
+    }
+    if let Some(var_5) = &input.durable_execution_name {
+        object.key("DurableExecutionName").string(var_5.as_str());
+    }
+    if let Some(var_6) = &input.payload {
+        object.key("Payload").string_unchecked(&::aws_smithy_types::base64::encode(var_6));
+    }
+    if let Some(var_7) = &input.qualifier {
+        object.key("Qualifier").string(var_7.as_str());
+    }
+    if let Some(var_8) = &input.tenant_id {
+        object.key("TenantId").string(var_8.as_str());
+    }
+    Ok(())
 }
```

### `src/protocol_serde/shape_invoke_with_response_stream.rs`

```diff
--- reference/src/protocol_serde/shape_invoke_with_response_stream.rs
+++ generated/src/protocol_serde/shape_invoke_with_response_stream.rs
@@ -1,48 +1,5 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 #[allow(clippy::unnecessary_wraps)]
-pub fn de_invoke_with_response_stream_http_response(
-    response: &mut ::aws_smithy_runtime_api::http::Response,
-) -> std::result::Result<
-    super::operation::invoke_with_response_stream::InvokeWithResponseStreamOutput,
-    super::operation::invoke_with_response_stream::InvokeWithResponseStreamError,
-> {
-    let mut _response_body = ::aws_smithy_types::body::SdkBody::taken();
-    std::mem::swap(&mut _response_body, response.body_mut());
-    let _response_body = &mut _response_body;
-
-    let _response_status = response.status().as_u16();
-    let _response_headers = response.headers();
-    Ok({
-        #[allow(unused_mut)]
-        let mut output = super::operation::invoke_with_response_stream::builders::InvokeWithResponseStreamOutputBuilder::default();
-        output = output.set_event_stream(Some(
-            super::protocol_serde::shape_invoke_with_response_stream_output::de_event_stream_payload(_response_body)?,
-        ));
-        output = output.set_executed_version(
-            super::protocol_serde::shape_invoke_with_response_stream_output::de_executed_version_header(_response_headers).map_err(|_| {
-                super::operation::invoke_with_response_stream::InvokeWithResponseStreamError::unhandled(
-                    "Failed to parse ExecutedVersion from header `X-Amz-Executed-Version",
-                )
-            })?,
-        );
-        output = output.set_response_stream_content_type(
-            super::protocol_serde::shape_invoke_with_response_stream_output::de_response_stream_content_type_header(_response_headers).map_err(
-                |_| {
-                    super::operation::invoke_with_response_stream::InvokeWithResponseStreamError::unhandled(
-                        "Failed to parse ResponseStreamContentType from header `Content-Type",
-                    )
-                },
-            )?,
-        );
-        output = output.set_status_code(Some(_response_status as _));
-        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
-        output
-            .build()
-            .map_err(super::operation::invoke_with_response_stream::InvokeWithResponseStreamError::unhandled)?
-    })
-}
-
-#[allow(clippy::unnecessary_wraps)]
 pub fn de_invoke_with_response_stream_http_error(
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
@@ -649,13 +606,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::invoke_with_response_stream::InvokeWithResponseStreamError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::invoke_with_response_stream::InvokeWithResponseStreamError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -688,53 +638,94 @@
     })
 }

-pub fn ser_invoke_with_response_stream_headers(
+#[allow(clippy::unnecessary_wraps)]
+pub fn de_invoke_with_response_stream_http_response(
+    _response_status: u16,
+    _response_headers: &::aws_smithy_runtime_api::http::Headers,
+    _response_body: &[u8],
+) -> std::result::Result<
+    super::operation::invoke_with_response_stream::InvokeWithResponseStreamOutput,
+    super::operation::invoke_with_response_stream::InvokeWithResponseStreamError,
+> {
+    Ok({
+        #[allow(unused_mut)]
+        let mut output = super::operation::invoke_with_response_stream::builders::InvokeWithResponseStreamOutputBuilder::default();
+        output = super::protocol_serde::shape_invoke_with_response_stream::de_invoke_with_response_stream(_response_body, output)
+            .map_err(super::operation::invoke_with_response_stream::InvokeWithResponseStreamError::unhandled)?;
+        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
+        output.build()
+    })
+}
+
+pub fn ser_invoke_with_response_stream_input(
     input: &super::operation::invoke_with_response_stream::InvokeWithResponseStreamInput,
-    mut builder: ::http_1x::request::Builder,
-) -> std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
-    if let ::std::option::Option::Some(inner_1) = &input.log_type {
-        let formatted_2 = inner_1.as_str();
-        let header_value = formatted_2;
-        let header_value: ::http_1x::HeaderValue = header_value.parse().map_err(|err| {
-            ::aws_smithy_types::error::operation::BuildError::invalid_field(
-                "log_type",
-                format!("`{}` cannot be used as a header value: {}", &header_value, err),
-            )
-        })?;
-        builder = builder.header("X-Amz-Log-Type", header_value);
-    }
-    if let ::std::option::Option::Some(inner_3) = &input.client_context {
-        let formatted_4 = inner_3.as_str();
-        let header_value = formatted_4;
-        let header_value: ::http_1x::HeaderValue = header_value.parse().map_err(|err| {
-            ::aws_smithy_types::error::operation::BuildError::invalid_field(
-                "client_context",
-                format!("`{}` cannot be used as a header value: {}", &header_value, err),
-            )
-        })?;
-        builder = builder.header("X-Amz-Client-Context", header_value);
-    }
-    if let ::std::option::Option::Some(inner_5) = &input.tenant_id {
-        let formatted_6 = inner_5.as_str();
-        let header_value = formatted_6;
-        let header_value: ::http_1x::HeaderValue = header_value.parse().map_err(|err| {
-            ::aws_smithy_types::error::operation::BuildError::invalid_field(
-                "tenant_id",
-                format!("`{}` cannot be used as a header value: {}", &header_value, err),
-            )
-        })?;
-        builder = builder.header("X-Amz-Tenant-Id", header_value);
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_invoke_with_response_stream_input::ser_invoke_with_response_stream_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
+pub(crate) fn de_invoke_with_response_stream(
+    _value: &[u8],
+    mut builder: super::operation::invoke_with_response_stream::builders::InvokeWithResponseStreamOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::invoke_with_response_stream::builders::InvokeWithResponseStreamOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                "StatusCode" => {
+                    builder = builder.set_status_code(
+                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                            .map(i32::try_from)
+                            .transpose()?,
+                    );
+                }
+                "ExecutedVersion" => {
+                    builder = builder.set_executed_version(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
+                }
+                "EventStream" => {
+                    builder = builder.set_event_stream(
+                        super::protocol_serde::shape_invoke_with_response_stream_response_event::de_invoke_with_response_stream_response_event(
+                            tokens,
+                            _value,
+                            depth + 1,
+                        )?,
+                    );
+                }
+                "ResponseStreamContentType" => {
+                    builder = builder.set_response_stream_content_type(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
+                }
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
     }
-    if let ::std::option::Option::Some(inner_7) = &input.invocation_type {
-        let formatted_8 = inner_7.as_str();
-        let header_value = formatted_8;
-        let header_value: ::http_1x::HeaderValue = header_value.parse().map_err(|err| {
-            ::aws_smithy_types::error::operation::BuildError::invalid_field(
-                "invocation_type",
-                format!("`{}` cannot be used as a header value: {}", &header_value, err),
-            )
-        })?;
-        builder = builder.header("X-Amz-Invocation-Type", header_value);
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
     }
     Ok(builder)
 }
```

### `src/protocol_serde/shape_invoke_with_response_stream_complete_event.rs`

```diff
--- reference/src/protocol_serde/shape_invoke_with_response_stream_complete_event.rs
+++ generated/src/protocol_serde/shape_invoke_with_response_stream_complete_event.rs
@@ -1,25 +1,4 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-pub(crate) fn de_invoke_with_response_stream_complete_event_payload(
-    _value: &[u8],
-) -> ::std::result::Result<super::types::InvokeWithResponseStreamCompleteEvent, ::aws_smithy_json::deserialize::error::DeserializeError> {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
-    let tokens = &mut tokens_owned;
-    #[allow(unused_variables)]
-    let depth = 0u32;
-    let result = super::protocol_serde::shape_invoke_with_response_stream_complete_event::de_invoke_with_response_stream_complete_event(
-        tokens,
-        _value,
-        depth + 1,
-    )?
-    .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("expected payload member value"));
-    if tokens.next().is_some() {
-        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-            "found more JSON tokens after completing parsing",
-        ));
-    }
-    result
-}
-
 pub(crate) fn de_invoke_with_response_stream_complete_event<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
```

### `src/protocol_serde/shape_invoke_with_response_stream_input.rs`

```diff
--- reference/src/protocol_serde/shape_invoke_with_response_stream_input.rs
+++ generated/src/protocol_serde/shape_invoke_with_response_stream_input.rs
@@ -1,10 +1,28 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-pub fn ser_payload_http_payload(
-    payload: ::std::option::Option<::aws_smithy_types::Blob>,
-) -> ::std::result::Result<::bytes::Bytes, ::aws_smithy_types::error::operation::BuildError> {
-    let payload = match payload {
-        Some(t) => t,
-        None => return Ok(::bytes::Bytes::new()),
-    };
-    Ok(::aws_smithy_types::Blob::from(payload).into_bytes())
+pub fn ser_invoke_with_response_stream_input_input(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &crate::operation::invoke_with_response_stream::InvokeWithResponseStreamInput,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    if let Some(var_1) = &input.function_name {
+        object.key("FunctionName").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.log_type {
+        object.key("LogType").string(var_2.as_str());
+    }
+    if let Some(var_3) = &input.client_context {
+        object.key("ClientContext").string(var_3.as_str());
+    }
+    if let Some(var_4) = &input.qualifier {
+        object.key("Qualifier").string(var_4.as_str());
+    }
+    if let Some(var_5) = &input.payload {
+        object.key("Payload").string_unchecked(&::aws_smithy_types::base64::encode(var_5));
+    }
+    if let Some(var_6) = &input.tenant_id {
+        object.key("TenantId").string(var_6.as_str());
+    }
+    if let Some(var_7) = &input.invocation_type {
+        object.key("InvocationType").string(var_7.as_str());
+    }
+    Ok(())
 }
```

### `src/protocol_serde/shape_kafka_schema_registry_config.rs`

```diff
--- reference/src/protocol_serde/shape_kafka_schema_registry_config.rs
+++ generated/src/protocol_serde/shape_kafka_schema_registry_config.rs
@@ -1,4 +1,41 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_kafka_schema_registry_config(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::types::KafkaSchemaRegistryConfig,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    if let Some(var_1) = &input.schema_registry_uri {
+        object.key("SchemaRegistryURI").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.event_record_format {
+        object.key("EventRecordFormat").string(var_2.as_str());
+    }
+    if let Some(var_3) = &input.access_configs {
+        let mut array_4 = object.key("AccessConfigs").start_array();
+        for item_5 in var_3 {
+            {
+                #[allow(unused_mut)]
+                let mut object_6 = array_4.value().start_object();
+                super::protocol_serde::shape_kafka_schema_registry_access_config::ser_kafka_schema_registry_access_config(&mut object_6, item_5)?;
+                object_6.finish();
+            }
+        }
+        array_4.finish();
+    }
+    if let Some(var_7) = &input.schema_validation_configs {
+        let mut array_8 = object.key("SchemaValidationConfigs").start_array();
+        for item_9 in var_7 {
+            {
+                #[allow(unused_mut)]
+                let mut object_10 = array_8.value().start_object();
+                super::protocol_serde::shape_kafka_schema_validation_config::ser_kafka_schema_validation_config(&mut object_10, item_9)?;
+                object_10.finish();
+            }
+        }
+        array_8.finish();
+    }
+    Ok(())
+}
+
 pub(crate) fn de_kafka_schema_registry_config<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
@@ -37,9 +74,7 @@
                                 );
                             }
                             "AccessConfigs" => {
-                                builder = builder.set_access_configs(
-                                    super::protocol_serde::shape_kafka_schema_registry_access_config_list::de_kafka_schema_registry_access_config_list(tokens, _value, depth + 1)?
-                                );
+                                builder = builder.set_access_configs(super::protocol_serde::shape_kafka_schema_registry_access_config_list::de_kafka_schema_registry_access_config_list(tokens, _value, depth + 1)?);
                             }
                             "SchemaValidationConfigs" => {
                                 builder = builder.set_schema_validation_configs(
@@ -67,40 +102,3 @@
         )),
     }
 }
-
-pub fn ser_kafka_schema_registry_config(
-    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
-    input: &super::types::KafkaSchemaRegistryConfig,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.schema_registry_uri {
-        object.key("SchemaRegistryURI").string(var_1.as_str());
-    }
-    if let Some(var_2) = &input.event_record_format {
-        object.key("EventRecordFormat").string(var_2.as_str());
-    }
-    if let Some(var_3) = &input.access_configs {
-        let mut array_4 = object.key("AccessConfigs").start_array();
-        for item_5 in var_3 {
-            {
-                #[allow(unused_mut)]
-                let mut object_6 = array_4.value().start_object();
-                super::protocol_serde::shape_kafka_schema_registry_access_config::ser_kafka_schema_registry_access_config(&mut object_6, item_5)?;
-                object_6.finish();
-            }
-        }
-        array_4.finish();
-    }
-    if let Some(var_7) = &input.schema_validation_configs {
-        let mut array_8 = object.key("SchemaValidationConfigs").start_array();
-        for item_9 in var_7 {
-            {
-                #[allow(unused_mut)]
-                let mut object_10 = array_8.value().start_object();
-                super::protocol_serde::shape_kafka_schema_validation_config::ser_kafka_schema_validation_config(&mut object_10, item_9)?;
-                object_10.finish();
-            }
-        }
-        array_8.finish();
-    }
-    Ok(())
-}
```

### `src/protocol_serde/shape_lambda_managed_instances_capacity_provider_config.rs`

```diff
--- reference/src/protocol_serde/shape_lambda_managed_instances_capacity_provider_config.rs
+++ generated/src/protocol_serde/shape_lambda_managed_instances_capacity_provider_config.rs
@@ -1,4 +1,26 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_lambda_managed_instances_capacity_provider_config(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::types::LambdaManagedInstancesCapacityProviderConfig,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    {
+        object.key("CapacityProviderArn").string(input.capacity_provider_arn.as_str());
+    }
+    if let Some(var_1) = &input.per_execution_environment_max_concurrency {
+        object.key("PerExecutionEnvironmentMaxConcurrency").number(
+            #[allow(clippy::useless_conversion)]
+            ::aws_smithy_types::Number::NegInt((*var_1).into()),
+        );
+    }
+    if let Some(var_2) = &input.execution_environment_memory_gi_b_per_v_cpu {
+        object.key("ExecutionEnvironmentMemoryGiBPerVCpu").number(
+            #[allow(clippy::useless_conversion)]
+            ::aws_smithy_types::Number::Float((*var_2).into()),
+        );
+    }
+    Ok(())
+}
+
 pub(crate) fn de_lambda_managed_instances_capacity_provider_config<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
@@ -36,7 +58,7 @@
                             );
                         }
                         "ExecutionEnvironmentMemoryGiBPerVCpu" => {
-                            builder = builder.set_execution_environment_memory_gib_per_v_cpu(
+                            builder = builder.set_execution_environment_memory_gi_b_per_v_cpu(
                                 ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?.map(|v| v.to_f64_lossy()),
                             );
                         }
@@ -60,25 +82,3 @@
         )),
     }
 }
-
-pub fn ser_lambda_managed_instances_capacity_provider_config(
-    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
-    input: &super::types::LambdaManagedInstancesCapacityProviderConfig,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    {
-        object.key("CapacityProviderArn").string(input.capacity_provider_arn.as_str());
-    }
-    if let Some(var_1) = &input.per_execution_environment_max_concurrency {
-        object.key("PerExecutionEnvironmentMaxConcurrency").number(
-            #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_1).into()),
-        );
-    }
-    if let Some(var_2) = &input.execution_environment_memory_gib_per_v_cpu {
-        object.key("ExecutionEnvironmentMemoryGiBPerVCpu").number(
-            #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::Float((*var_2).into()),
-        );
-    }
-    Ok(())
-}
```

### `src/protocol_serde/shape_list_aliases.rs`

```diff
--- reference/src/protocol_serde/shape_list_aliases.rs
+++ generated/src/protocol_serde/shape_list_aliases.rs
@@ -72,11 +72,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_aliases::ListAliasesError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::list_aliases::ListAliasesError::unhandled("Failed to parse retryAfterSeconds from header `Retry-After")
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -105,6 +100,16 @@
     })
 }

+pub fn ser_list_aliases_input(
+    input: &super::operation::list_aliases::ListAliasesInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_list_aliases_input::ser_list_aliases_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_list_aliases(
     _value: &[u8],
     mut builder: super::operation::list_aliases::builders::ListAliasesOutputBuilder,
@@ -119,9 +124,6 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "Aliases" => {
-                    builder = builder.set_aliases(super::protocol_serde::shape_alias_list::de_alias_list(tokens, _value, depth + 1)?);
-                }
                 "NextMarker" => {
                     builder = builder.set_next_marker(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -129,6 +131,9 @@
                             .transpose()?,
                     );
                 }
+                "Aliases" => {
+                    builder = builder.set_aliases(super::protocol_serde::shape_alias_list::de_alias_list(tokens, _value, depth + 1)?);
+                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_list_capacity_providers.rs`

```diff
--- reference/src/protocol_serde/shape_list_capacity_providers.rs
+++ generated/src/protocol_serde/shape_list_capacity_providers.rs
@@ -60,13 +60,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_capacity_providers::ListCapacityProvidersError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::list_capacity_providers::ListCapacityProvidersError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -100,6 +93,16 @@
     })
 }

+pub fn ser_list_capacity_providers_input(
+    input: &super::operation::list_capacity_providers::ListCapacityProvidersInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_list_capacity_providers_input::ser_list_capacity_providers_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_list_capacity_providers(
     _value: &[u8],
     mut builder: super::operation::list_capacity_providers::builders::ListCapacityProvidersOutputBuilder,
```

### `src/protocol_serde/shape_list_code_signing_configs.rs`

```diff
--- reference/src/protocol_serde/shape_list_code_signing_configs.rs
+++ generated/src/protocol_serde/shape_list_code_signing_configs.rs
@@ -82,6 +82,16 @@
     })
 }

+pub fn ser_list_code_signing_configs_input(
+    input: &super::operation::list_code_signing_configs::ListCodeSigningConfigsInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_list_code_signing_configs_input::ser_list_code_signing_configs_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_list_code_signing_configs(
     _value: &[u8],
     mut builder: super::operation::list_code_signing_configs::builders::ListCodeSigningConfigsOutputBuilder,
@@ -99,11 +109,6 @@
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                 match key.to_unescaped()?.as_ref() {
-                    "CodeSigningConfigs" => {
-                        builder = builder.set_code_signing_configs(
-                            super::protocol_serde::shape_code_signing_config_list::de_code_signing_config_list(tokens, _value, depth + 1)?,
-                        );
-                    }
                     "NextMarker" => {
                         builder = builder.set_next_marker(
                             ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -111,6 +116,11 @@
                                 .transpose()?,
                         );
                     }
+                    "CodeSigningConfigs" => {
+                        builder = builder.set_code_signing_configs(
+                            super::protocol_serde::shape_code_signing_config_list::de_code_signing_config_list(tokens, _value, depth + 1)?,
+                        );
+                    }
                     _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                 }
             }
```

### `src/protocol_serde/shape_list_durable_executions_by_function.rs`

```diff
--- reference/src/protocol_serde/shape_list_durable_executions_by_function.rs
+++ generated/src/protocol_serde/shape_list_durable_executions_by_function.rs
@@ -82,13 +82,6 @@
                     output =
                         super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                             .map_err(super::operation::list_durable_executions_by_function::ListDurableExecutionsByFunctionError::unhandled)?;
-                    output = output.set_retry_after_seconds(
-                        super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                            super::operation::list_durable_executions_by_function::ListDurableExecutionsByFunctionError::unhandled(
-                                "Failed to parse retryAfterSeconds from header `Retry-After",
-                            )
-                        })?,
-                    );
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -121,6 +114,16 @@
     })
 }

+pub fn ser_list_durable_executions_by_function_input(
+    input: &super::operation::list_durable_executions_by_function::ListDurableExecutionsByFunctionInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_list_durable_executions_by_function_input::ser_list_durable_executions_by_function_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_list_durable_executions_by_function(
     _value: &[u8],
     mut builder: super::operation::list_durable_executions_by_function::builders::ListDurableExecutionsByFunctionOutputBuilder,
```

### `src/protocol_serde/shape_list_event_source_mappings.rs`

```diff
--- reference/src/protocol_serde/shape_list_event_source_mappings.rs
+++ generated/src/protocol_serde/shape_list_event_source_mappings.rs
@@ -81,13 +81,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_event_source_mappings::ListEventSourceMappingsError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::list_event_source_mappings::ListEventSourceMappingsError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -119,6 +112,16 @@
     })
 }

+pub fn ser_list_event_source_mappings_input(
+    input: &super::operation::list_event_source_mappings::ListEventSourceMappingsInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_list_event_source_mappings_input::ser_list_event_source_mappings_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_list_event_source_mappings(
     _value: &[u8],
     mut builder: super::operation::list_event_source_mappings::builders::ListEventSourceMappingsOutputBuilder,
@@ -135,11 +138,6 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "EventSourceMappings" => {
-                    builder = builder.set_event_source_mappings(
-                        super::protocol_serde::shape_event_source_mappings_list::de_event_source_mappings_list(tokens, _value, depth + 1)?,
-                    );
-                }
                 "NextMarker" => {
                     builder = builder.set_next_marker(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -147,6 +145,11 @@
                             .transpose()?,
                     );
                 }
+                "EventSourceMappings" => {
+                    builder = builder.set_event_source_mappings(
+                        super::protocol_serde::shape_event_source_mappings_list::de_event_source_mappings_list(tokens, _value, depth + 1)?,
+                    );
+                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_list_function_event_invoke_configs.rs`

```diff
--- reference/src/protocol_serde/shape_list_function_event_invoke_configs.rs
+++ generated/src/protocol_serde/shape_list_function_event_invoke_configs.rs
@@ -82,13 +82,6 @@
                     output =
                         super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                             .map_err(super::operation::list_function_event_invoke_configs::ListFunctionEventInvokeConfigsError::unhandled)?;
-                    output = output.set_retry_after_seconds(
-                        super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                            super::operation::list_function_event_invoke_configs::ListFunctionEventInvokeConfigsError::unhandled(
-                                "Failed to parse retryAfterSeconds from header `Retry-After",
-                            )
-                        })?,
-                    );
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -121,6 +114,16 @@
     })
 }

+pub fn ser_list_function_event_invoke_configs_input(
+    input: &super::operation::list_function_event_invoke_configs::ListFunctionEventInvokeConfigsInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_list_function_event_invoke_configs_input::ser_list_function_event_invoke_configs_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_list_function_event_invoke_configs(
     _value: &[u8],
     mut builder: super::operation::list_function_event_invoke_configs::builders::ListFunctionEventInvokeConfigsOutputBuilder,
```

### `src/protocol_serde/shape_list_function_url_configs.rs`

```diff
--- reference/src/protocol_serde/shape_list_function_url_configs.rs
+++ generated/src/protocol_serde/shape_list_function_url_configs.rs
@@ -81,13 +81,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_function_url_configs::ListFunctionUrlConfigsError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::list_function_url_configs::ListFunctionUrlConfigsError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -121,6 +114,16 @@
     })
 }

+pub fn ser_list_function_url_configs_input(
+    input: &super::operation::list_function_url_configs::ListFunctionUrlConfigsInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_list_function_url_configs_input::ser_list_function_url_configs_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_list_function_url_configs(
     _value: &[u8],
     mut builder: super::operation::list_function_url_configs::builders::ListFunctionUrlConfigsOutputBuilder,
```

### `src/protocol_serde/shape_list_function_versions_by_capacity_provider.rs`

```diff
--- reference/src/protocol_serde/shape_list_function_versions_by_capacity_provider.rs
+++ generated/src/protocol_serde/shape_list_function_versions_by_capacity_provider.rs
@@ -97,13 +97,6 @@
                             .map_err(
                                 super::operation::list_function_versions_by_capacity_provider::ListFunctionVersionsByCapacityProviderError::unhandled,
                             )?;
-                    output = output.set_retry_after_seconds(
-                        super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                            super::operation::list_function_versions_by_capacity_provider::ListFunctionVersionsByCapacityProviderError::unhandled(
-                                "Failed to parse retryAfterSeconds from header `Retry-After",
-                            )
-                        })?,
-                    );
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -142,6 +135,19 @@
     })
 }

+pub fn ser_list_function_versions_by_capacity_provider_input(
+    input: &super::operation::list_function_versions_by_capacity_provider::ListFunctionVersionsByCapacityProviderInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_list_function_versions_by_capacity_provider_input::ser_list_function_versions_by_capacity_provider_input_input(
+        &mut object,
+        input,
+    )?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_list_function_versions_by_capacity_provider(
     _value: &[u8],
     mut builder: super::operation::list_function_versions_by_capacity_provider::builders::ListFunctionVersionsByCapacityProviderOutputBuilder,
```

### `src/protocol_serde/shape_list_functions.rs`

```diff
--- reference/src/protocol_serde/shape_list_functions.rs
+++ generated/src/protocol_serde/shape_list_functions.rs
@@ -57,11 +57,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_functions::ListFunctionsError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::list_functions::ListFunctionsError::unhandled("Failed to parse retryAfterSeconds from header `Retry-After")
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -90,6 +85,16 @@
     })
 }

+pub fn ser_list_functions_input(
+    input: &super::operation::list_functions::ListFunctionsInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_list_functions_input::ser_list_functions_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_list_functions(
     _value: &[u8],
     mut builder: super::operation::list_functions::builders::ListFunctionsOutputBuilder,
@@ -106,9 +111,6 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "Functions" => {
-                    builder = builder.set_functions(super::protocol_serde::shape_function_list::de_function_list(tokens, _value, depth + 1)?);
-                }
                 "NextMarker" => {
                     builder = builder.set_next_marker(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -116,6 +118,9 @@
                             .transpose()?,
                     );
                 }
+                "Functions" => {
+                    builder = builder.set_functions(super::protocol_serde::shape_function_list::de_function_list(tokens, _value, depth + 1)?);
+                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_list_functions_by_code_signing_config.rs`

```diff
--- reference/src/protocol_serde/shape_list_functions_by_code_signing_config.rs
+++ generated/src/protocol_serde/shape_list_functions_by_code_signing_config.rs
@@ -96,6 +96,19 @@
     })
 }

+pub fn ser_list_functions_by_code_signing_config_input(
+    input: &super::operation::list_functions_by_code_signing_config::ListFunctionsByCodeSigningConfigInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_list_functions_by_code_signing_config_input::ser_list_functions_by_code_signing_config_input_input(
+        &mut object,
+        input,
+    )?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_list_functions_by_code_signing_config(
     _value: &[u8],
     mut builder: super::operation::list_functions_by_code_signing_config::builders::ListFunctionsByCodeSigningConfigOutputBuilder,
@@ -112,13 +125,6 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "FunctionArns" => {
-                    builder = builder.set_function_arns(super::protocol_serde::shape_function_arn_list::de_function_arn_list(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
-                }
                 "NextMarker" => {
                     builder = builder.set_next_marker(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -126,6 +132,13 @@
                             .transpose()?,
                     );
                 }
+                "FunctionArns" => {
+                    builder = builder.set_function_arns(super::protocol_serde::shape_function_arn_list::de_function_arn_list(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_list_layer_versions.rs`

```diff
--- reference/src/protocol_serde/shape_list_layer_versions.rs
+++ generated/src/protocol_serde/shape_list_layer_versions.rs
@@ -73,13 +73,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_layer_versions::ListLayerVersionsError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::list_layer_versions::ListLayerVersionsError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -109,6 +102,16 @@
     })
 }

+pub fn ser_list_layer_versions_input(
+    input: &super::operation::list_layer_versions::ListLayerVersionsInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_list_layer_versions_input::ser_list_layer_versions_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_list_layer_versions(
     _value: &[u8],
     mut builder: super::operation::list_layer_versions::builders::ListLayerVersionsOutputBuilder,
@@ -125,13 +128,6 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "LayerVersions" => {
-                    builder = builder.set_layer_versions(super::protocol_serde::shape_layer_versions_list::de_layer_versions_list(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
-                }
                 "NextMarker" => {
                     builder = builder.set_next_marker(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -139,6 +135,13 @@
                             .transpose()?,
                     );
                 }
+                "LayerVersions" => {
+                    builder = builder.set_layer_versions(super::protocol_serde::shape_layer_versions_list::de_layer_versions_list(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_list_layers.rs`

```diff
--- reference/src/protocol_serde/shape_list_layers.rs
+++ generated/src/protocol_serde/shape_list_layers.rs
@@ -57,11 +57,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_layers::ListLayersError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::list_layers::ListLayersError::unhandled("Failed to parse retryAfterSeconds from header `Retry-After")
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -90,6 +85,16 @@
     })
 }

+pub fn ser_list_layers_input(
+    input: &super::operation::list_layers::ListLayersInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_list_layers_input::ser_list_layers_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_list_layers(
     _value: &[u8],
     mut builder: super::operation::list_layers::builders::ListLayersOutputBuilder,
@@ -104,9 +109,6 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "Layers" => {
-                    builder = builder.set_layers(super::protocol_serde::shape_layers_list::de_layers_list(tokens, _value, depth + 1)?);
-                }
                 "NextMarker" => {
                     builder = builder.set_next_marker(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -114,6 +116,9 @@
                             .transpose()?,
                     );
                 }
+                "Layers" => {
+                    builder = builder.set_layers(super::protocol_serde::shape_layers_list::de_layers_list(tokens, _value, depth + 1)?);
+                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_list_provisioned_concurrency_configs.rs`

```diff
--- reference/src/protocol_serde/shape_list_provisioned_concurrency_configs.rs
+++ generated/src/protocol_serde/shape_list_provisioned_concurrency_configs.rs
@@ -82,13 +82,6 @@
                     output =
                         super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                             .map_err(super::operation::list_provisioned_concurrency_configs::ListProvisionedConcurrencyConfigsError::unhandled)?;
-                    output = output.set_retry_after_seconds(
-                        super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                            super::operation::list_provisioned_concurrency_configs::ListProvisionedConcurrencyConfigsError::unhandled(
-                                "Failed to parse retryAfterSeconds from header `Retry-After",
-                            )
-                        })?,
-                    );
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -121,6 +114,19 @@
     })
 }

+pub fn ser_list_provisioned_concurrency_configs_input(
+    input: &super::operation::list_provisioned_concurrency_configs::ListProvisionedConcurrencyConfigsInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_list_provisioned_concurrency_configs_input::ser_list_provisioned_concurrency_configs_input_input(
+        &mut object,
+        input,
+    )?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_list_provisioned_concurrency_configs(
     _value: &[u8],
     mut builder: super::operation::list_provisioned_concurrency_configs::builders::ListProvisionedConcurrencyConfigsOutputBuilder,
@@ -137,13 +143,6 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "NextMarker" => {
-                    builder = builder.set_next_marker(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
-                }
                 "ProvisionedConcurrencyConfigs" => {
                     builder = builder.set_provisioned_concurrency_configs(
                         super::protocol_serde::shape_provisioned_concurrency_config_list::de_provisioned_concurrency_config_list(
@@ -153,6 +152,13 @@
                         )?,
                     );
                 }
+                "NextMarker" => {
+                    builder = builder.set_next_marker(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
+                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_list_tags.rs`

```diff
--- reference/src/protocol_serde/shape_list_tags.rs
+++ generated/src/protocol_serde/shape_list_tags.rs
@@ -72,11 +72,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_tags::ListTagsError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::list_tags::ListTagsError::unhandled("Failed to parse retryAfterSeconds from header `Retry-After")
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -105,6 +100,16 @@
     })
 }

+pub fn ser_list_tags_input(
+    input: &super::operation::list_tags::ListTagsInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_list_tags_input::ser_list_tags_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_list_tags(
     _value: &[u8],
     mut builder: super::operation::list_tags::builders::ListTagsOutputBuilder,
```

### `src/protocol_serde/shape_list_versions_by_function.rs`

```diff
--- reference/src/protocol_serde/shape_list_versions_by_function.rs
+++ generated/src/protocol_serde/shape_list_versions_by_function.rs
@@ -81,13 +81,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::list_versions_by_function::ListVersionsByFunctionError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::list_versions_by_function::ListVersionsByFunctionError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -119,6 +112,16 @@
     })
 }

+pub fn ser_list_versions_by_function_input(
+    input: &super::operation::list_versions_by_function::ListVersionsByFunctionInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_list_versions_by_function_input::ser_list_versions_by_function_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_list_versions_by_function(
     _value: &[u8],
     mut builder: super::operation::list_versions_by_function::builders::ListVersionsByFunctionOutputBuilder,
```

### `src/protocol_serde/shape_logging_config.rs`

```diff
--- reference/src/protocol_serde/shape_logging_config.rs
+++ generated/src/protocol_serde/shape_logging_config.rs
@@ -1,4 +1,23 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_logging_config(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::types::LoggingConfig,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    if let Some(var_1) = &input.log_format {
+        object.key("LogFormat").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.application_log_level {
+        object.key("ApplicationLogLevel").string(var_2.as_str());
+    }
+    if let Some(var_3) = &input.system_log_level {
+        object.key("SystemLogLevel").string(var_3.as_str());
+    }
+    if let Some(var_4) = &input.log_group {
+        object.key("LogGroup").string(var_4.as_str());
+    }
+    Ok(())
+}
+
 pub(crate) fn de_logging_config<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
@@ -65,22 +84,3 @@
         )),
     }
 }
-
-pub fn ser_logging_config(
-    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
-    input: &super::types::LoggingConfig,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.log_format {
-        object.key("LogFormat").string(var_1.as_str());
-    }
-    if let Some(var_2) = &input.application_log_level {
-        object.key("ApplicationLogLevel").string(var_2.as_str());
-    }
-    if let Some(var_3) = &input.system_log_level {
-        object.key("SystemLogLevel").string(var_3.as_str());
-    }
-    if let Some(var_4) = &input.log_group {
-        object.key("LogGroup").string(var_4.as_str());
-    }
-    Ok(())
-}
```

### `src/protocol_serde/shape_on_failure.rs`

```diff
--- reference/src/protocol_serde/shape_on_failure.rs
+++ generated/src/protocol_serde/shape_on_failure.rs
@@ -1,4 +1,14 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_on_failure(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::types::OnFailure,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    if let Some(var_1) = &input.destination {
+        object.key("Destination").string(var_1.as_str());
+    }
+    Ok(())
+}
+
 pub(crate) fn de_on_failure<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
@@ -44,13 +54,3 @@
         )),
     }
 }
-
-pub fn ser_on_failure(
-    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
-    input: &super::types::OnFailure,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.destination {
-        object.key("Destination").string(var_1.as_str());
-    }
-    Ok(())
-}
```

### `src/protocol_serde/shape_on_success.rs`

```diff
--- reference/src/protocol_serde/shape_on_success.rs
+++ generated/src/protocol_serde/shape_on_success.rs
@@ -1,4 +1,14 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_on_success(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::types::OnSuccess,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    if let Some(var_1) = &input.destination {
+        object.key("Destination").string(var_1.as_str());
+    }
+    Ok(())
+}
+
 pub(crate) fn de_on_success<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
@@ -44,13 +54,3 @@
         )),
     }
 }
-
-pub fn ser_on_success(
-    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
-    input: &super::types::OnSuccess,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.destination {
-        object.key("Destination").string(var_1.as_str());
-    }
-    Ok(())
-}
```

### `src/protocol_serde/shape_provisioned_poller_config.rs`

```diff
--- reference/src/protocol_serde/shape_provisioned_poller_config.rs
+++ generated/src/protocol_serde/shape_provisioned_poller_config.rs
@@ -1,4 +1,26 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_provisioned_poller_config(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::types::ProvisionedPollerConfig,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    if let Some(var_1) = &input.minimum_pollers {
+        object.key("MinimumPollers").number(
+            #[allow(clippy::useless_conversion)]
+            ::aws_smithy_types::Number::NegInt((*var_1).into()),
+        );
+    }
+    if let Some(var_2) = &input.maximum_pollers {
+        object.key("MaximumPollers").number(
+            #[allow(clippy::useless_conversion)]
+            ::aws_smithy_types::Number::NegInt((*var_2).into()),
+        );
+    }
+    if let Some(var_3) = &input.poller_group_name {
+        object.key("PollerGroupName").string(var_3.as_str());
+    }
+    Ok(())
+}
+
 pub(crate) fn de_provisioned_poller_config<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
@@ -58,25 +80,3 @@
         )),
     }
 }
-
-pub fn ser_provisioned_poller_config(
-    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
-    input: &super::types::ProvisionedPollerConfig,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.minimum_pollers {
-        object.key("MinimumPollers").number(
-            #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_1).into()),
-        );
-    }
-    if let Some(var_2) = &input.maximum_pollers {
-        object.key("MaximumPollers").number(
-            #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_2).into()),
-        );
-    }
-    if let Some(var_3) = &input.poller_group_name {
-        object.key("PollerGroupName").string(var_3.as_str());
-    }
-    Ok(())
-}
```

### `src/protocol_serde/shape_publish_layer_version.rs`

```diff
--- reference/src/protocol_serde/shape_publish_layer_version.rs
+++ generated/src/protocol_serde/shape_publish_layer_version.rs
@@ -91,13 +91,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::publish_layer_version::PublishLayerVersionError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::publish_layer_version::PublishLayerVersionError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -155,53 +148,34 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "CompatibleArchitectures" => {
-                    builder = builder.set_compatible_architectures(
-                        super::protocol_serde::shape_compatible_architectures::de_compatible_architectures(tokens, _value, depth + 1)?,
-                    );
-                }
-                "CompatibleRuntimes" => {
-                    builder = builder.set_compatible_runtimes(super::protocol_serde::shape_compatible_runtimes::de_compatible_runtimes(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
-                }
                 "Content" => {
                     builder = builder.set_content(
                         super::protocol_serde::shape_layer_version_content_output::de_layer_version_content_output(tokens, _value, depth + 1)?,
                     );
                 }
-                "CreatedDate" => {
-                    builder = builder.set_created_date(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
-                }
-                "Description" => {
-                    builder = builder.set_description(
+                "LayerArn" => {
+                    builder = builder.set_layer_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "LayerArn" => {
-                    builder = builder.set_layer_arn(
+                "LayerVersionArn" => {
+                    builder = builder.set_layer_version_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "LayerVersionArn" => {
-                    builder = builder.set_layer_version_arn(
+                "Description" => {
+                    builder = builder.set_description(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "LicenseInfo" => {
-                    builder = builder.set_license_info(
+                "CreatedDate" => {
+                    builder = builder.set_created_date(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
@@ -214,6 +188,25 @@
                             .transpose()?,
                     );
                 }
+                "CompatibleArchitectures" => {
+                    builder = builder.set_compatible_architectures(
+                        super::protocol_serde::shape_compatible_architectures::de_compatible_architectures(tokens, _value, depth + 1)?,
+                    );
+                }
+                "CompatibleRuntimes" => {
+                    builder = builder.set_compatible_runtimes(super::protocol_serde::shape_compatible_runtimes::de_compatible_runtimes(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
+                "LicenseInfo" => {
+                    builder = builder.set_license_info(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
+                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_publish_layer_version_input.rs`

```diff
--- reference/src/protocol_serde/shape_publish_layer_version_input.rs
+++ generated/src/protocol_serde/shape_publish_layer_version_input.rs
@@ -3,35 +3,38 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::publish_layer_version::PublishLayerVersionInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.compatible_architectures {
-        let mut array_2 = object.key("CompatibleArchitectures").start_array();
-        for item_3 in var_1 {
+    if let Some(var_1) = &input.layer_name {
+        object.key("LayerName").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.description {
+        object.key("Description").string(var_2.as_str());
+    }
+    if let Some(var_3) = &input.content {
+        #[allow(unused_mut)]
+        let mut object_4 = object.key("Content").start_object();
+        super::protocol_serde::shape_layer_version_content_input::ser_layer_version_content_input(&mut object_4, var_3)?;
+        object_4.finish();
+    }
+    if let Some(var_5) = &input.compatible_architectures {
+        let mut array_6 = object.key("CompatibleArchitectures").start_array();
+        for item_7 in var_5 {
             {
-                array_2.value().string(item_3.as_str());
+                array_6.value().string(item_7.as_str());
             }
         }
-        array_2.finish();
+        array_6.finish();
     }
-    if let Some(var_4) = &input.compatible_runtimes {
-        let mut array_5 = object.key("CompatibleRuntimes").start_array();
-        for item_6 in var_4 {
+    if let Some(var_8) = &input.compatible_runtimes {
+        let mut array_9 = object.key("CompatibleRuntimes").start_array();
+        for item_10 in var_8 {
             {
-                array_5.value().string(item_6.as_str());
+                array_9.value().string(item_10.as_str());
             }
         }
-        array_5.finish();
+        array_9.finish();
     }
-    if let Some(var_7) = &input.content {
-        #[allow(unused_mut)]
-        let mut object_8 = object.key("Content").start_object();
-        super::protocol_serde::shape_layer_version_content_input::ser_layer_version_content_input(&mut object_8, var_7)?;
-        object_8.finish();
-    }
-    if let Some(var_9) = &input.description {
-        object.key("Description").string(var_9.as_str());
-    }
-    if let Some(var_10) = &input.license_info {
-        object.key("LicenseInfo").string(var_10.as_str());
+    if let Some(var_11) = &input.license_info {
+        object.key("LicenseInfo").string(var_11.as_str());
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_publish_version.rs`

```diff
--- reference/src/protocol_serde/shape_publish_version.rs
+++ generated/src/protocol_serde/shape_publish_version.rs
@@ -135,13 +135,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::publish_version::PublishVersionError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::publish_version::PublishVersionError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -196,20 +189,36 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "Architectures" => {
-                    builder = builder.set_architectures(super::protocol_serde::shape_architectures_list::de_architectures_list(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
+                "FunctionName" => {
+                    builder = builder.set_function_name(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
+                }
+                "FunctionArn" => {
+                    builder = builder.set_function_arn(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
+                }
+                "Runtime" => {
+                    builder = builder.set_runtime(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| super::types::Runtime::from(u.as_ref())))
+                            .transpose()?,
+                    );
                 }
-                "CapacityProviderConfig" => {
-                    builder = builder.set_capacity_provider_config(
-                        super::protocol_serde::shape_capacity_provider_config::de_capacity_provider_config(tokens, _value, depth + 1)?,
+                "Role" => {
+                    builder = builder.set_role(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
                     );
                 }
-                "CodeSha256" => {
-                    builder = builder.set_code_sha256(
+                "Handler" => {
+                    builder = builder.set_handler(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
@@ -222,93 +231,125 @@
                             .transpose()?,
                     );
                 }
-                "ConfigSha256" => {
-                    builder = builder.set_config_sha256(
+                "Description" => {
+                    builder = builder.set_description(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "DeadLetterConfig" => {
-                    builder = builder.set_dead_letter_config(super::protocol_serde::shape_dead_letter_config::de_dead_letter_config(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
+                "Timeout" => {
+                    builder = builder.set_timeout(
+                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                            .map(i32::try_from)
+                            .transpose()?,
+                    );
                 }
-                "Description" => {
-                    builder = builder.set_description(
+                "MemorySize" => {
+                    builder = builder.set_memory_size(
+                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                            .map(i32::try_from)
+                            .transpose()?,
+                    );
+                }
+                "LastModified" => {
+                    builder = builder.set_last_modified(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "DurableConfig" => {
-                    builder = builder.set_durable_config(super::protocol_serde::shape_durable_config::de_durable_config(tokens, _value, depth + 1)?);
+                "CodeSha256" => {
+                    builder = builder.set_code_sha256(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
                 }
-                "Environment" => {
-                    builder = builder.set_environment(super::protocol_serde::shape_environment_response::de_environment_response(
+                "Version" => {
+                    builder = builder.set_version(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
+                }
+                "VpcConfig" => {
+                    builder = builder.set_vpc_config(super::protocol_serde::shape_vpc_config_response::de_vpc_config_response(
                         tokens,
                         _value,
                         depth + 1,
                     )?);
                 }
-                "EphemeralStorage" => {
-                    builder = builder.set_ephemeral_storage(super::protocol_serde::shape_ephemeral_storage::de_ephemeral_storage(
+                "DeadLetterConfig" => {
+                    builder = builder.set_dead_letter_config(super::protocol_serde::shape_dead_letter_config::de_dead_letter_config(
                         tokens,
                         _value,
                         depth + 1,
                     )?);
                 }
-                "FileSystemConfigs" => {
-                    builder = builder.set_file_system_configs(super::protocol_serde::shape_file_system_config_list::de_file_system_config_list(
+                "Environment" => {
+                    builder = builder.set_environment(super::protocol_serde::shape_environment_response::de_environment_response(
                         tokens,
                         _value,
                         depth + 1,
                     )?);
                 }
-                "FunctionArn" => {
-                    builder = builder.set_function_arn(
+                "KMSKeyArn" => {
+                    builder = builder.set_kms_key_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "FunctionName" => {
-                    builder = builder.set_function_name(
+                "TracingConfig" => {
+                    builder = builder.set_tracing_config(super::protocol_serde::shape_tracing_config_response::de_tracing_config_response(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
+                "MasterArn" => {
+                    builder = builder.set_master_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "Handler" => {
-                    builder = builder.set_handler(
+                "RevisionId" => {
+                    builder = builder.set_revision_id(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "ImageConfigResponse" => {
-                    builder = builder.set_image_config_response(super::protocol_serde::shape_image_config_response::de_image_config_response(
+                "Layers" => {
+                    builder = builder.set_layers(super::protocol_serde::shape_layers_reference_list::de_layers_reference_list(
                         tokens,
                         _value,
                         depth + 1,
                     )?);
                 }
-                "KMSKeyArn" => {
-                    builder = builder.set_kms_key_arn(
+                "State" => {
+                    builder = builder.set_state(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .map(|s| s.to_unescaped().map(|u| super::types::State::from(u.as_ref())))
                             .transpose()?,
                     );
                 }
-                "LastModified" => {
-                    builder = builder.set_last_modified(
+                "StateReason" => {
+                    builder = builder.set_state_reason(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
+                "StateReasonCode" => {
+                    builder = builder.set_state_reason_code(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| super::types::StateReasonCode::from(u.as_ref())))
+                            .transpose()?,
+                    );
+                }
                 "LastUpdateStatus" => {
                     builder = builder.set_last_update_status(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -330,27 +371,24 @@
                             .transpose()?,
                     );
                 }
-                "Layers" => {
-                    builder = builder.set_layers(super::protocol_serde::shape_layers_reference_list::de_layers_reference_list(
+                "FileSystemConfigs" => {
+                    builder = builder.set_file_system_configs(super::protocol_serde::shape_file_system_config_list::de_file_system_config_list(
                         tokens,
                         _value,
                         depth + 1,
                     )?);
                 }
-                "LoggingConfig" => {
-                    builder = builder.set_logging_config(super::protocol_serde::shape_logging_config::de_logging_config(tokens, _value, depth + 1)?);
-                }
-                "MasterArn" => {
-                    builder = builder.set_master_arn(
+                "SigningProfileVersionArn" => {
+                    builder = builder.set_signing_profile_version_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "MemorySize" => {
-                    builder = builder.set_memory_size(
-                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
-                            .map(i32::try_from)
+                "SigningJobArn" => {
+                    builder = builder.set_signing_job_arn(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
@@ -361,48 +399,27 @@
                             .transpose()?,
                     );
                 }
-                "RevisionId" => {
-                    builder = builder.set_revision_id(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
+                "ImageConfigResponse" => {
+                    builder = builder.set_image_config_response(super::protocol_serde::shape_image_config_response::de_image_config_response(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
                 }
-                "Role" => {
-                    builder = builder.set_role(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
+                "Architectures" => {
+                    builder = builder.set_architectures(super::protocol_serde::shape_architectures_list::de_architectures_list(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
                 }
-                "Runtime" => {
-                    builder = builder.set_runtime(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| super::types::Runtime::from(u.as_ref())))
-                            .transpose()?,
-                    );
-                }
-                "RuntimeVersionConfig" => {
-                    builder = builder.set_runtime_version_config(super::protocol_serde::shape_runtime_version_config::de_runtime_version_config(
+                "EphemeralStorage" => {
+                    builder = builder.set_ephemeral_storage(super::protocol_serde::shape_ephemeral_storage::de_ephemeral_storage(
                         tokens,
                         _value,
                         depth + 1,
                     )?);
                 }
-                "SigningJobArn" => {
-                    builder = builder.set_signing_job_arn(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
-                }
-                "SigningProfileVersionArn" => {
-                    builder = builder.set_signing_profile_version_arn(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
-                }
                 "SnapStart" => {
                     builder = builder.set_snap_start(super::protocol_serde::shape_snap_start_response::de_snap_start_response(
                         tokens,
@@ -410,57 +427,33 @@
                         depth + 1,
                     )?);
                 }
-                "State" => {
-                    builder = builder.set_state(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| super::types::State::from(u.as_ref())))
-                            .transpose()?,
-                    );
-                }
-                "StateReason" => {
-                    builder = builder.set_state_reason(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
+                "RuntimeVersionConfig" => {
+                    builder = builder.set_runtime_version_config(super::protocol_serde::shape_runtime_version_config::de_runtime_version_config(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
                 }
-                "StateReasonCode" => {
-                    builder = builder.set_state_reason_code(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| super::types::StateReasonCode::from(u.as_ref())))
-                            .transpose()?,
-                    );
+                "LoggingConfig" => {
+                    builder = builder.set_logging_config(super::protocol_serde::shape_logging_config::de_logging_config(tokens, _value, depth + 1)?);
                 }
                 "TenancyConfig" => {
                     builder = builder.set_tenancy_config(super::protocol_serde::shape_tenancy_config::de_tenancy_config(tokens, _value, depth + 1)?);
                 }
-                "Timeout" => {
-                    builder = builder.set_timeout(
-                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
-                            .map(i32::try_from)
-                            .transpose()?,
+                "CapacityProviderConfig" => {
+                    builder = builder.set_capacity_provider_config(
+                        super::protocol_serde::shape_capacity_provider_config::de_capacity_provider_config(tokens, _value, depth + 1)?,
                     );
                 }
-                "TracingConfig" => {
-                    builder = builder.set_tracing_config(super::protocol_serde::shape_tracing_config_response::de_tracing_config_response(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
-                }
-                "Version" => {
-                    builder = builder.set_version(
+                "ConfigSha256" => {
+                    builder = builder.set_config_sha256(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "VpcConfig" => {
-                    builder = builder.set_vpc_config(super::protocol_serde::shape_vpc_config_response::de_vpc_config_response(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
+                "DurableConfig" => {
+                    builder = builder.set_durable_config(super::protocol_serde::shape_durable_config::de_durable_config(tokens, _value, depth + 1)?);
                 }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
```

### `src/protocol_serde/shape_publish_version_input.rs`

```diff
--- reference/src/protocol_serde/shape_publish_version_input.rs
+++ generated/src/protocol_serde/shape_publish_version_input.rs
@@ -3,17 +3,20 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::publish_version::PublishVersionInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.code_sha256 {
-        object.key("CodeSha256").string(var_1.as_str());
+    if let Some(var_1) = &input.function_name {
+        object.key("FunctionName").string(var_1.as_str());
     }
-    if let Some(var_2) = &input.description {
-        object.key("Description").string(var_2.as_str());
+    if let Some(var_2) = &input.code_sha256 {
+        object.key("CodeSha256").string(var_2.as_str());
     }
-    if let Some(var_3) = &input.publish_to {
-        object.key("PublishTo").string(var_3.as_str());
+    if let Some(var_3) = &input.description {
+        object.key("Description").string(var_3.as_str());
     }
     if let Some(var_4) = &input.revision_id {
         object.key("RevisionId").string(var_4.as_str());
     }
+    if let Some(var_5) = &input.publish_to {
+        object.key("PublishTo").string(var_5.as_str());
+    }
     Ok(())
 }
```

### `src/protocol_serde/shape_put_function_code_signing_config.rs`

```diff
--- reference/src/protocol_serde/shape_put_function_code_signing_config.rs
+++ generated/src/protocol_serde/shape_put_function_code_signing_config.rs
@@ -121,13 +121,6 @@
                     output =
                         super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                             .map_err(super::operation::put_function_code_signing_config::PutFunctionCodeSigningConfigError::unhandled)?;
-                    output = output.set_retry_after_seconds(
-                        super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                            super::operation::put_function_code_signing_config::PutFunctionCodeSigningConfigError::unhandled(
-                                "Failed to parse retryAfterSeconds from header `Retry-After",
-                            )
-                        })?,
-                    );
                     let output = output.meta(generic);
                     output.build()
                 };
```

### `src/protocol_serde/shape_put_function_code_signing_config_input.rs`

```diff
--- reference/src/protocol_serde/shape_put_function_code_signing_config_input.rs
+++ generated/src/protocol_serde/shape_put_function_code_signing_config_input.rs
@@ -6,5 +6,8 @@
     if let Some(var_1) = &input.code_signing_config_arn {
         object.key("CodeSigningConfigArn").string(var_1.as_str());
     }
+    if let Some(var_2) = &input.function_name {
+        object.key("FunctionName").string(var_2.as_str());
+    }
     Ok(())
 }
```

### `src/protocol_serde/shape_put_function_concurrency.rs`

```diff
--- reference/src/protocol_serde/shape_put_function_concurrency.rs
+++ generated/src/protocol_serde/shape_put_function_concurrency.rs
@@ -96,13 +96,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::put_function_concurrency::PutFunctionConcurrencyError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::put_function_concurrency::PutFunctionConcurrencyError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
```

### `src/protocol_serde/shape_put_function_concurrency_input.rs`

```diff
--- reference/src/protocol_serde/shape_put_function_concurrency_input.rs
+++ generated/src/protocol_serde/shape_put_function_concurrency_input.rs
@@ -3,10 +3,13 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::put_function_concurrency::PutFunctionConcurrencyInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.reserved_concurrent_executions {
+    if let Some(var_1) = &input.function_name {
+        object.key("FunctionName").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.reserved_concurrent_executions {
         object.key("ReservedConcurrentExecutions").number(
             #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_1).into()),
+            ::aws_smithy_types::Number::NegInt((*var_2).into()),
         );
     }
     Ok(())
```

### `src/protocol_serde/shape_put_function_event_invoke_config.rs`

```diff
--- reference/src/protocol_serde/shape_put_function_event_invoke_config.rs
+++ generated/src/protocol_serde/shape_put_function_event_invoke_config.rs
@@ -100,13 +100,6 @@
                     output =
                         super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                             .map_err(super::operation::put_function_event_invoke_config::PutFunctionEventInvokeConfigError::unhandled)?;
-                    output = output.set_retry_after_seconds(
-                        super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                            super::operation::put_function_event_invoke_config::PutFunctionEventInvokeConfigError::unhandled(
-                                "Failed to parse retryAfterSeconds from header `Retry-After",
-                            )
-                        })?,
-                    );
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -165,11 +158,10 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "DestinationConfig" => {
-                    builder = builder.set_destination_config(super::protocol_serde::shape_destination_config::de_destination_config(
-                        tokens,
-                        _value,
-                        depth + 1,
+                "LastModified" => {
+                    builder = builder.set_last_modified(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
+                        tokens.next(),
+                        ::aws_smithy_types::date_time::Format::EpochSeconds,
                     )?);
                 }
                 "FunctionArn" => {
@@ -179,26 +171,27 @@
                             .transpose()?,
                     );
                 }
-                "LastModified" => {
-                    builder = builder.set_last_modified(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
-                        tokens.next(),
-                        ::aws_smithy_types::date_time::Format::EpochSeconds,
-                    )?);
-                }
-                "MaximumEventAgeInSeconds" => {
-                    builder = builder.set_maximum_event_age_in_seconds(
+                "MaximumRetryAttempts" => {
+                    builder = builder.set_maximum_retry_attempts(
                         ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                             .map(i32::try_from)
                             .transpose()?,
                     );
                 }
-                "MaximumRetryAttempts" => {
-                    builder = builder.set_maximum_retry_attempts(
+                "MaximumEventAgeInSeconds" => {
+                    builder = builder.set_maximum_event_age_in_seconds(
                         ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                             .map(i32::try_from)
                             .transpose()?,
                     );
                 }
+                "DestinationConfig" => {
+                    builder = builder.set_destination_config(super::protocol_serde::shape_destination_config::de_destination_config(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_put_function_event_invoke_config_input.rs`

```diff
--- reference/src/protocol_serde/shape_put_function_event_invoke_config_input.rs
+++ generated/src/protocol_serde/shape_put_function_event_invoke_config_input.rs
@@ -3,23 +3,29 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::put_function_event_invoke_config::PutFunctionEventInvokeConfigInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.destination_config {
-        #[allow(unused_mut)]
-        let mut object_2 = object.key("DestinationConfig").start_object();
-        super::protocol_serde::shape_destination_config::ser_destination_config(&mut object_2, var_1)?;
-        object_2.finish();
+    if let Some(var_1) = &input.function_name {
+        object.key("FunctionName").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.qualifier {
+        object.key("Qualifier").string(var_2.as_str());
     }
-    if let Some(var_3) = &input.maximum_event_age_in_seconds {
-        object.key("MaximumEventAgeInSeconds").number(
+    if let Some(var_3) = &input.maximum_retry_attempts {
+        object.key("MaximumRetryAttempts").number(
             #[allow(clippy::useless_conversion)]
             ::aws_smithy_types::Number::NegInt((*var_3).into()),
         );
     }
-    if let Some(var_4) = &input.maximum_retry_attempts {
-        object.key("MaximumRetryAttempts").number(
+    if let Some(var_4) = &input.maximum_event_age_in_seconds {
+        object.key("MaximumEventAgeInSeconds").number(
             #[allow(clippy::useless_conversion)]
             ::aws_smithy_types::Number::NegInt((*var_4).into()),
         );
     }
+    if let Some(var_5) = &input.destination_config {
+        #[allow(unused_mut)]
+        let mut object_6 = object.key("DestinationConfig").start_object();
+        super::protocol_serde::shape_destination_config::ser_destination_config(&mut object_6, var_5)?;
+        object_6.finish();
+    }
     Ok(())
 }
```

### `src/protocol_serde/shape_put_function_recursion_config.rs`

```diff
--- reference/src/protocol_serde/shape_put_function_recursion_config.rs
+++ generated/src/protocol_serde/shape_put_function_recursion_config.rs
@@ -92,13 +92,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::put_function_recursion_config::PutFunctionRecursionConfigError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::put_function_recursion_config::PutFunctionRecursionConfigError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
```

### `src/protocol_serde/shape_put_function_recursion_config_input.rs`

```diff
--- reference/src/protocol_serde/shape_put_function_recursion_config_input.rs
+++ generated/src/protocol_serde/shape_put_function_recursion_config_input.rs
@@ -3,8 +3,11 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::put_function_recursion_config::PutFunctionRecursionConfigInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.recursive_loop {
-        object.key("RecursiveLoop").string(var_1.as_str());
+    if let Some(var_1) = &input.function_name {
+        object.key("FunctionName").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.recursive_loop {
+        object.key("RecursiveLoop").string(var_2.as_str());
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_put_function_scaling_config.rs`

```diff
--- reference/src/protocol_serde/shape_put_function_scaling_config.rs
+++ generated/src/protocol_serde/shape_put_function_scaling_config.rs
@@ -96,13 +96,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::put_function_scaling_config::PutFunctionScalingConfigError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::put_function_scaling_config::PutFunctionScalingConfigError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
```

### `src/protocol_serde/shape_put_function_scaling_config_input.rs`

```diff
--- reference/src/protocol_serde/shape_put_function_scaling_config_input.rs
+++ generated/src/protocol_serde/shape_put_function_scaling_config_input.rs
@@ -3,11 +3,17 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::put_function_scaling_config::PutFunctionScalingConfigInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.function_scaling_config {
+    if let Some(var_1) = &input.function_name {
+        object.key("FunctionName").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.qualifier {
+        object.key("Qualifier").string(var_2.as_str());
+    }
+    if let Some(var_3) = &input.function_scaling_config {
         #[allow(unused_mut)]
-        let mut object_2 = object.key("FunctionScalingConfig").start_object();
-        super::protocol_serde::shape_function_scaling_config::ser_function_scaling_config(&mut object_2, var_1)?;
-        object_2.finish();
+        let mut object_4 = object.key("FunctionScalingConfig").start_object();
+        super::protocol_serde::shape_function_scaling_config::ser_function_scaling_config(&mut object_4, var_3)?;
+        object_4.finish();
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_put_provisioned_concurrency_config.rs`

```diff
--- reference/src/protocol_serde/shape_put_provisioned_concurrency_config.rs
+++ generated/src/protocol_serde/shape_put_provisioned_concurrency_config.rs
@@ -100,13 +100,6 @@
                     output =
                         super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                             .map_err(super::operation::put_provisioned_concurrency_config::PutProvisionedConcurrencyConfigError::unhandled)?;
-                    output = output.set_retry_after_seconds(
-                        super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                            super::operation::put_provisioned_concurrency_config::PutProvisionedConcurrencyConfigError::unhandled(
-                                "Failed to parse retryAfterSeconds from header `Retry-After",
-                            )
-                        })?,
-                    );
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -165,29 +158,22 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "AllocatedProvisionedConcurrentExecutions" => {
-                    builder = builder.set_allocated_provisioned_concurrent_executions(
+                "RequestedProvisionedConcurrentExecutions" => {
+                    builder = builder.set_requested_provisioned_concurrent_executions(
                         ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                             .map(i32::try_from)
                             .transpose()?,
                     );
                 }
-                "AvailableProvisionedConcurrentExecutions" => {
-                    builder = builder.set_available_provisioned_concurrent_executions(
+                "AllocatedProvisionedConcurrentExecutions" => {
+                    builder = builder.set_allocated_provisioned_concurrent_executions(
                         ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                             .map(i32::try_from)
                             .transpose()?,
                     );
                 }
-                "LastModified" => {
-                    builder = builder.set_last_modified(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
-                }
-                "RequestedProvisionedConcurrentExecutions" => {
-                    builder = builder.set_requested_provisioned_concurrent_executions(
+                "AvailableProvisionedConcurrentExecutions" => {
+                    builder = builder.set_available_provisioned_concurrent_executions(
                         ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                             .map(i32::try_from)
                             .transpose()?,
@@ -207,6 +193,13 @@
                             .transpose()?,
                     );
                 }
+                "LastModified" => {
+                    builder = builder.set_last_modified(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
+                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_put_provisioned_concurrency_config_input.rs`

```diff
--- reference/src/protocol_serde/shape_put_provisioned_concurrency_config_input.rs
+++ generated/src/protocol_serde/shape_put_provisioned_concurrency_config_input.rs
@@ -3,10 +3,16 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::put_provisioned_concurrency_config::PutProvisionedConcurrencyConfigInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.provisioned_concurrent_executions {
+    if let Some(var_1) = &input.function_name {
+        object.key("FunctionName").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.qualifier {
+        object.key("Qualifier").string(var_2.as_str());
+    }
+    if let Some(var_3) = &input.provisioned_concurrent_executions {
         object.key("ProvisionedConcurrentExecutions").number(
             #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_1).into()),
+            ::aws_smithy_types::Number::NegInt((*var_3).into()),
         );
     }
     Ok(())
```

### `src/protocol_serde/shape_put_resource_policy.rs`

```diff
--- reference/src/protocol_serde/shape_put_resource_policy.rs
+++ generated/src/protocol_serde/shape_put_resource_policy.rs
@@ -137,13 +137,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::put_resource_policy::PutResourcePolicyError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::put_resource_policy::PutResourcePolicyError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
```

### `src/protocol_serde/shape_put_resource_policy_input.rs`

```diff
--- reference/src/protocol_serde/shape_put_resource_policy_input.rs
+++ generated/src/protocol_serde/shape_put_resource_policy_input.rs
@@ -3,11 +3,14 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::put_resource_policy::PutResourcePolicyInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.policy {
-        object.key("Policy").string(var_1.as_str());
+    if let Some(var_1) = &input.resource_arn {
+        object.key("ResourceArn").string(var_1.as_str());
     }
-    if let Some(var_2) = &input.revision_id {
-        object.key("RevisionId").string(var_2.as_str());
+    if let Some(var_2) = &input.policy {
+        object.key("Policy").string(var_2.as_str());
+    }
+    if let Some(var_3) = &input.revision_id {
+        object.key("RevisionId").string(var_3.as_str());
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_put_runtime_management_config.rs`

```diff
--- reference/src/protocol_serde/shape_put_runtime_management_config.rs
+++ generated/src/protocol_serde/shape_put_runtime_management_config.rs
@@ -92,13 +92,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::put_runtime_management_config::PutRuntimeManagementConfigError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::put_runtime_management_config::PutRuntimeManagementConfigError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -158,6 +151,13 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                "UpdateRuntimeOn" => {
+                    builder = builder.set_update_runtime_on(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| super::types::UpdateRuntimeOn::from(u.as_ref())))
+                            .transpose()?,
+                    );
+                }
                 "FunctionArn" => {
                     builder = builder.set_function_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -172,13 +172,6 @@
                             .transpose()?,
                     );
                 }
-                "UpdateRuntimeOn" => {
-                    builder = builder.set_update_runtime_on(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| super::types::UpdateRuntimeOn::from(u.as_ref())))
-                            .transpose()?,
-                    );
-                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_put_runtime_management_config_input.rs`

```diff
--- reference/src/protocol_serde/shape_put_runtime_management_config_input.rs
+++ generated/src/protocol_serde/shape_put_runtime_management_config_input.rs
@@ -3,11 +3,17 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::put_runtime_management_config::PutRuntimeManagementConfigInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.runtime_version_arn {
-        object.key("RuntimeVersionArn").string(var_1.as_str());
+    if let Some(var_1) = &input.function_name {
+        object.key("FunctionName").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.qualifier {
+        object.key("Qualifier").string(var_2.as_str());
+    }
+    if let Some(var_3) = &input.update_runtime_on {
+        object.key("UpdateRuntimeOn").string(var_3.as_str());
     }
-    if let Some(var_2) = &input.update_runtime_on {
-        object.key("UpdateRuntimeOn").string(var_2.as_str());
+    if let Some(var_4) = &input.runtime_version_arn {
+        object.key("RuntimeVersionArn").string(var_4.as_str());
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_remove_layer_version_permission.rs`

```diff
--- reference/src/protocol_serde/shape_remove_layer_version_permission.rs
+++ generated/src/protocol_serde/shape_remove_layer_version_permission.rs
@@ -100,13 +100,6 @@
                     output =
                         super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                             .map_err(super::operation::remove_layer_version_permission::RemoveLayerVersionPermissionError::unhandled)?;
-                    output = output.set_retry_after_seconds(
-                        super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                            super::operation::remove_layer_version_permission::RemoveLayerVersionPermissionError::unhandled(
-                                "Failed to parse retryAfterSeconds from header `Retry-After",
-                            )
-                        })?,
-                    );
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -136,3 +129,46 @@
         output.build()
     })
 }
+
+pub fn ser_remove_layer_version_permission_input(
+    input: &super::operation::remove_layer_version_permission::RemoveLayerVersionPermissionInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_remove_layer_version_permission_input::ser_remove_layer_version_permission_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
+pub(crate) fn de_remove_layer_version_permission(
+    _value: &[u8],
+    mut builder: super::operation::remove_layer_version_permission::builders::RemoveLayerVersionPermissionOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::remove_layer_version_permission::builders::RemoveLayerVersionPermissionOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_remove_permission.rs`

```diff
--- reference/src/protocol_serde/shape_remove_permission.rs
+++ generated/src/protocol_serde/shape_remove_permission.rs
@@ -103,13 +103,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::remove_permission::RemovePermissionError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::remove_permission::RemovePermissionError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -135,3 +128,46 @@
         output.build()
     })
 }
+
+pub fn ser_remove_permission_input(
+    input: &super::operation::remove_permission::RemovePermissionInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_remove_permission_input::ser_remove_permission_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
+pub(crate) fn de_remove_permission(
+    _value: &[u8],
+    mut builder: super::operation::remove_permission::builders::RemovePermissionOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::remove_permission::builders::RemovePermissionOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_scaling_config.rs`

```diff
--- reference/src/protocol_serde/shape_scaling_config.rs
+++ generated/src/protocol_serde/shape_scaling_config.rs
@@ -1,4 +1,17 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_scaling_config(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::types::ScalingConfig,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    if let Some(var_1) = &input.maximum_concurrency {
+        object.key("MaximumConcurrency").number(
+            #[allow(clippy::useless_conversion)]
+            ::aws_smithy_types::Number::NegInt((*var_1).into()),
+        );
+    }
+    Ok(())
+}
+
 pub(crate) fn de_scaling_config<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
@@ -44,16 +57,3 @@
         )),
     }
 }
-
-pub fn ser_scaling_config(
-    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
-    input: &super::types::ScalingConfig,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.maximum_concurrency {
-        object.key("MaximumConcurrency").number(
-            #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_1).into()),
-        );
-    }
-    Ok(())
-}
```

### `src/protocol_serde/shape_self_managed_event_source.rs`

```diff
--- reference/src/protocol_serde/shape_self_managed_event_source.rs
+++ generated/src/protocol_serde/shape_self_managed_event_source.rs
@@ -1,4 +1,27 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_self_managed_event_source(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::types::SelfManagedEventSource,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    if let Some(var_1) = &input.endpoints {
+        #[allow(unused_mut)]
+        let mut object_2 = object.key("Endpoints").start_object();
+        for (key_3, value_4) in var_1 {
+            {
+                let mut array_5 = object_2.key(key_3.as_str()).start_array();
+                for item_6 in value_4 {
+                    {
+                        array_5.value().string(item_6.as_str());
+                    }
+                }
+                array_5.finish();
+            }
+        }
+        object_2.finish();
+    }
+    Ok(())
+}
+
 pub(crate) fn de_self_managed_event_source<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
@@ -40,26 +63,3 @@
         )),
     }
 }
-
-pub fn ser_self_managed_event_source(
-    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
-    input: &super::types::SelfManagedEventSource,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.endpoints {
-        #[allow(unused_mut)]
-        let mut object_2 = object.key("Endpoints").start_object();
-        for (key_3, value_4) in var_1 {
-            {
-                let mut array_5 = object_2.key(key_3.as_str()).start_array();
-                for item_6 in value_4 {
-                    {
-                        array_5.value().string(item_6.as_str());
-                    }
-                }
-                array_5.finish();
-            }
-        }
-        object_2.finish();
-    }
-    Ok(())
-}
```

### `src/protocol_serde/shape_self_managed_kafka_event_source_config.rs`

```diff
--- reference/src/protocol_serde/shape_self_managed_kafka_event_source_config.rs
+++ generated/src/protocol_serde/shape_self_managed_kafka_event_source_config.rs
@@ -1,4 +1,20 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_self_managed_kafka_event_source_config(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::types::SelfManagedKafkaEventSourceConfig,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    if let Some(var_1) = &input.consumer_group_id {
+        object.key("ConsumerGroupId").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.schema_registry_config {
+        #[allow(unused_mut)]
+        let mut object_3 = object.key("SchemaRegistryConfig").start_object();
+        super::protocol_serde::shape_kafka_schema_registry_config::ser_kafka_schema_registry_config(&mut object_3, var_2)?;
+        object_3.finish();
+    }
+    Ok(())
+}
+
 pub(crate) fn de_self_managed_kafka_event_source_config<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
@@ -53,19 +69,3 @@
         )),
     }
 }
-
-pub fn ser_self_managed_kafka_event_source_config(
-    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
-    input: &super::types::SelfManagedKafkaEventSourceConfig,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.consumer_group_id {
-        object.key("ConsumerGroupId").string(var_1.as_str());
-    }
-    if let Some(var_2) = &input.schema_registry_config {
-        #[allow(unused_mut)]
-        let mut object_3 = object.key("SchemaRegistryConfig").start_object();
-        super::protocol_serde::shape_kafka_schema_registry_config::ser_kafka_schema_registry_config(&mut object_3, var_2)?;
-        object_3.finish();
-    }
-    Ok(())
-}
```

### `src/protocol_serde/shape_send_durable_execution_callback_failure.rs`

```diff
--- reference/src/protocol_serde/shape_send_durable_execution_callback_failure.rs
+++ generated/src/protocol_serde/shape_send_durable_execution_callback_failure.rs
@@ -177,13 +177,6 @@
                             .map_err(
                                 super::operation::send_durable_execution_callback_failure::SendDurableExecutionCallbackFailureError::unhandled,
                             )?;
-                    output = output.set_retry_after_seconds(
-                        super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                            super::operation::send_durable_execution_callback_failure::SendDurableExecutionCallbackFailureError::unhandled(
-                                "Failed to parse retryAfterSeconds from header `Retry-After",
-                            )
-                        })?,
-                    );
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -214,3 +207,49 @@
         output.build()
     })
 }
+
+pub fn ser_send_durable_execution_callback_failure_input(
+    input: &super::operation::send_durable_execution_callback_failure::SendDurableExecutionCallbackFailureInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_send_durable_execution_callback_failure_input::ser_send_durable_execution_callback_failure_input_input(
+        &mut object,
+        input,
+    )?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
+pub(crate) fn de_send_durable_execution_callback_failure(
+    _value: &[u8],
+    mut builder: super::operation::send_durable_execution_callback_failure::builders::SendDurableExecutionCallbackFailureOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::send_durable_execution_callback_failure::builders::SendDurableExecutionCallbackFailureOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_send_durable_execution_callback_failure_input.rs`

```diff
--- reference/src/protocol_serde/shape_send_durable_execution_callback_failure_input.rs
+++ generated/src/protocol_serde/shape_send_durable_execution_callback_failure_input.rs
@@ -1,20 +1,16 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-pub fn ser_error_http_payload(
-    payload: &::std::option::Option<super::types::ErrorObject>,
-) -> ::std::result::Result<::std::vec::Vec<u8>, ::aws_smithy_types::error::operation::BuildError> {
-    let payload = match payload.as_ref() {
-        Some(t) => t,
-        None => return Ok(super::protocol_serde::rest_json_unset_struct_payload()),
-    };
-    Ok(super::protocol_serde::shape_send_durable_execution_callback_failure_input::ser_error_payload(payload)?)
-}
-
-pub fn ser_error_payload(
-    input: &super::types::ErrorObject,
-) -> std::result::Result<::std::vec::Vec<u8>, ::aws_smithy_types::error::operation::SerializationError> {
-    let mut out = String::new();
-    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
-    super::protocol_serde::shape_error_object::ser_error_object(&mut object, input)?;
-    object.finish();
-    Ok(out.into_bytes())
+pub fn ser_send_durable_execution_callback_failure_input_input(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::operation::send_durable_execution_callback_failure::SendDurableExecutionCallbackFailureInput,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    if let Some(var_1) = &input.callback_id {
+        object.key("CallbackId").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.error {
+        #[allow(unused_mut)]
+        let mut object_3 = object.key("Error").start_object();
+        super::protocol_serde::shape_error_object::ser_error_object(&mut object_3, var_2)?;
+        object_3.finish();
+    }
+    Ok(())
 }
```

### `src/protocol_serde/shape_send_durable_execution_callback_heartbeat.rs`

```diff
--- reference/src/protocol_serde/shape_send_durable_execution_callback_heartbeat.rs
+++ generated/src/protocol_serde/shape_send_durable_execution_callback_heartbeat.rs
@@ -110,13 +110,6 @@
                             .map_err(
                                 super::operation::send_durable_execution_callback_heartbeat::SendDurableExecutionCallbackHeartbeatError::unhandled,
                             )?;
-                    output = output.set_retry_after_seconds(
-                        super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                            super::operation::send_durable_execution_callback_heartbeat::SendDurableExecutionCallbackHeartbeatError::unhandled(
-                                "Failed to parse retryAfterSeconds from header `Retry-After",
-                            )
-                        })?,
-                    );
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -147,3 +140,49 @@
         output.build()
     })
 }
+
+pub fn ser_send_durable_execution_callback_heartbeat_input(
+    input: &super::operation::send_durable_execution_callback_heartbeat::SendDurableExecutionCallbackHeartbeatInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_send_durable_execution_callback_heartbeat_input::ser_send_durable_execution_callback_heartbeat_input_input(
+        &mut object,
+        input,
+    )?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
+pub(crate) fn de_send_durable_execution_callback_heartbeat(
+    _value: &[u8],
+    mut builder: super::operation::send_durable_execution_callback_heartbeat::builders::SendDurableExecutionCallbackHeartbeatOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::send_durable_execution_callback_heartbeat::builders::SendDurableExecutionCallbackHeartbeatOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_send_durable_execution_callback_success.rs`

```diff
--- reference/src/protocol_serde/shape_send_durable_execution_callback_success.rs
+++ generated/src/protocol_serde/shape_send_durable_execution_callback_success.rs
@@ -177,13 +177,6 @@
                             .map_err(
                                 super::operation::send_durable_execution_callback_success::SendDurableExecutionCallbackSuccessError::unhandled,
                             )?;
-                    output = output.set_retry_after_seconds(
-                        super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                            super::operation::send_durable_execution_callback_success::SendDurableExecutionCallbackSuccessError::unhandled(
-                                "Failed to parse retryAfterSeconds from header `Retry-After",
-                            )
-                        })?,
-                    );
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -214,3 +207,49 @@
         output.build()
     })
 }
+
+pub fn ser_send_durable_execution_callback_success_input(
+    input: &super::operation::send_durable_execution_callback_success::SendDurableExecutionCallbackSuccessInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_send_durable_execution_callback_success_input::ser_send_durable_execution_callback_success_input_input(
+        &mut object,
+        input,
+    )?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
+pub(crate) fn de_send_durable_execution_callback_success(
+    _value: &[u8],
+    mut builder: super::operation::send_durable_execution_callback_success::builders::SendDurableExecutionCallbackSuccessOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::send_durable_execution_callback_success::builders::SendDurableExecutionCallbackSuccessOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_send_durable_execution_callback_success_input.rs`

```diff
--- reference/src/protocol_serde/shape_send_durable_execution_callback_success_input.rs
+++ generated/src/protocol_serde/shape_send_durable_execution_callback_success_input.rs
@@ -1,10 +1,13 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-pub fn ser_result_http_payload(
-    payload: ::std::option::Option<::aws_smithy_types::Blob>,
-) -> ::std::result::Result<::bytes::Bytes, ::aws_smithy_types::error::operation::BuildError> {
-    let payload = match payload {
-        Some(t) => t,
-        None => return Ok(::bytes::Bytes::new()),
-    };
-    Ok(::aws_smithy_types::Blob::from(payload).into_bytes())
+pub fn ser_send_durable_execution_callback_success_input_input(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &crate::operation::send_durable_execution_callback_success::SendDurableExecutionCallbackSuccessInput,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    if let Some(var_1) = &input.callback_id {
+        object.key("CallbackId").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.result {
+        object.key("Result").string_unchecked(&::aws_smithy_types::base64::encode(var_2));
+    }
+    Ok(())
 }
```

### `src/protocol_serde/shape_stop_durable_execution.rs`

```diff
--- reference/src/protocol_serde/shape_stop_durable_execution.rs
+++ generated/src/protocol_serde/shape_stop_durable_execution.rs
@@ -135,13 +135,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::stop_durable_execution::StopDurableExecutionError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::stop_durable_execution::StopDurableExecutionError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -175,6 +168,16 @@
     })
 }

+pub fn ser_stop_durable_execution_input(
+    input: &super::operation::stop_durable_execution::StopDurableExecutionInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_stop_durable_execution_input::ser_stop_durable_execution_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
 pub(crate) fn de_stop_durable_execution(
     _value: &[u8],
     mut builder: super::operation::stop_durable_execution::builders::StopDurableExecutionOutputBuilder,
```

### `src/protocol_serde/shape_stop_durable_execution_input.rs`

```diff
--- reference/src/protocol_serde/shape_stop_durable_execution_input.rs
+++ generated/src/protocol_serde/shape_stop_durable_execution_input.rs
@@ -1,20 +1,16 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-pub fn ser_error_http_payload(
-    payload: &::std::option::Option<super::types::ErrorObject>,
-) -> ::std::result::Result<::std::vec::Vec<u8>, ::aws_smithy_types::error::operation::BuildError> {
-    let payload = match payload.as_ref() {
-        Some(t) => t,
-        None => return Ok(super::protocol_serde::rest_json_unset_struct_payload()),
-    };
-    Ok(super::protocol_serde::shape_stop_durable_execution_input::ser_error_payload(payload)?)
-}
-
-pub fn ser_error_payload(
-    input: &super::types::ErrorObject,
-) -> std::result::Result<::std::vec::Vec<u8>, ::aws_smithy_types::error::operation::SerializationError> {
-    let mut out = String::new();
-    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
-    super::protocol_serde::shape_error_object::ser_error_object(&mut object, input)?;
-    object.finish();
-    Ok(out.into_bytes())
+pub fn ser_stop_durable_execution_input_input(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::operation::stop_durable_execution::StopDurableExecutionInput,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    if let Some(var_1) = &input.durable_execution_arn {
+        object.key("DurableExecutionArn").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.error {
+        #[allow(unused_mut)]
+        let mut object_3 = object.key("Error").start_object();
+        super::protocol_serde::shape_error_object::ser_error_object(&mut object_3, var_2)?;
+        object_3.finish();
+    }
+    Ok(())
 }
```

### `src/protocol_serde/shape_tag_resource.rs`

```diff
--- reference/src/protocol_serde/shape_tag_resource.rs
+++ generated/src/protocol_serde/shape_tag_resource.rs
@@ -87,11 +87,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::tag_resource::TagResourceError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::tag_resource::TagResourceError::unhandled("Failed to parse retryAfterSeconds from header `Retry-After")
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -127,3 +122,34 @@
     object.finish();
     Ok(::aws_smithy_types::body::SdkBody::from(out))
 }
+
+pub(crate) fn de_tag_resource(
+    _value: &[u8],
+    mut builder: super::operation::tag_resource::builders::TagResourceOutputBuilder,
+) -> ::std::result::Result<super::operation::tag_resource::builders::TagResourceOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
+{
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_tag_resource_input.rs`

```diff
--- reference/src/protocol_serde/shape_tag_resource_input.rs
+++ generated/src/protocol_serde/shape_tag_resource_input.rs
@@ -3,15 +3,18 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::tag_resource::TagResourceInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.tags {
+    if let Some(var_1) = &input.resource {
+        object.key("Resource").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.tags {
         #[allow(unused_mut)]
-        let mut object_2 = object.key("Tags").start_object();
-        for (key_3, value_4) in var_1 {
+        let mut object_3 = object.key("Tags").start_object();
+        for (key_4, value_5) in var_2 {
             {
-                object_2.key(key_3.as_str()).string(value_4.as_str());
+                object_3.key(key_4.as_str()).string(value_5.as_str());
             }
         }
-        object_2.finish();
+        object_3.finish();
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_tenancy_config.rs`

```diff
--- reference/src/protocol_serde/shape_tenancy_config.rs
+++ generated/src/protocol_serde/shape_tenancy_config.rs
@@ -1,4 +1,14 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_tenancy_config(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::types::TenancyConfig,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    {
+        object.key("TenantIsolationMode").string(input.tenant_isolation_mode.as_str());
+    }
+    Ok(())
+}
+
 pub(crate) fn de_tenancy_config<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
@@ -46,13 +56,3 @@
         )),
     }
 }
-
-pub fn ser_tenancy_config(
-    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
-    input: &super::types::TenancyConfig,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    {
-        object.key("TenantIsolationMode").string(input.tenant_isolation_mode.as_str());
-    }
-    Ok(())
-}
```

### `src/protocol_serde/shape_too_many_requests_exception.rs`

```diff
--- reference/src/protocol_serde/shape_too_many_requests_exception.rs
+++ generated/src/protocol_serde/shape_too_many_requests_exception.rs
@@ -56,10 +56,3 @@
     }
     Ok(builder)
 }
-
-pub(crate) fn de_retry_after_seconds_header(
-    header_map: &::aws_smithy_runtime_api::http::Headers,
-) -> ::std::result::Result<::std::option::Option<::std::string::String>, ::aws_smithy_http::header::ParseError> {
-    let headers = header_map.get_all("Retry-After");
-    ::aws_smithy_http::header::one_or_none(headers)
-}
```

### `src/protocol_serde/shape_untag_resource.rs`

```diff
--- reference/src/protocol_serde/shape_untag_resource.rs
+++ generated/src/protocol_serde/shape_untag_resource.rs
@@ -87,11 +87,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::untag_resource::UntagResourceError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::untag_resource::UntagResourceError::unhandled("Failed to parse retryAfterSeconds from header `Retry-After")
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -117,3 +112,46 @@
         output.build()
     })
 }
+
+pub fn ser_untag_resource_input(
+    input: &super::operation::untag_resource::UntagResourceInput,
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+    let mut out = String::new();
+    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
+    super::protocol_serde::shape_untag_resource_input::ser_untag_resource_input_input(&mut object, input)?;
+    object.finish();
+    Ok(::aws_smithy_types::body::SdkBody::from(out))
+}
+
+pub(crate) fn de_untag_resource(
+    _value: &[u8],
+    mut builder: super::operation::untag_resource::builders::UntagResourceOutputBuilder,
+) -> ::std::result::Result<
+    super::operation::untag_resource::builders::UntagResourceOutputBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::protocol_serde::or_empty_doc(_value)).peekable();
+    let tokens = &mut tokens_owned;
+    #[allow(unused_variables)]
+    let depth = 0u32;
+    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
+    loop {
+        match tokens.next().transpose()? {
+            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+            },
+            other => {
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                    "expected object key or end object, found: {other:?}"
+                )))
+            }
+        }
+    }
+    if tokens.next().is_some() {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "found more JSON tokens after completing parsing",
+        ));
+    }
+    Ok(builder)
+}
```

### `src/protocol_serde/shape_update_alias.rs`

```diff
--- reference/src/protocol_serde/shape_update_alias.rs
+++ generated/src/protocol_serde/shape_update_alias.rs
@@ -103,11 +103,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::update_alias::UpdateAliasError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::update_alias::UpdateAliasError::unhandled("Failed to parse retryAfterSeconds from header `Retry-After")
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -168,8 +163,8 @@
                                 .transpose()?,
                         );
                     }
-                    "Description" => {
-                        builder = builder.set_description(
+                    "Name" => {
+                        builder = builder.set_name(
                             ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                 .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                 .transpose()?,
@@ -182,13 +177,18 @@
                                 .transpose()?,
                         );
                     }
-                    "Name" => {
-                        builder = builder.set_name(
+                    "Description" => {
+                        builder = builder.set_description(
                             ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                 .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                 .transpose()?,
                         );
                     }
+                    "RoutingConfig" => {
+                        builder = builder.set_routing_config(
+                            super::protocol_serde::shape_alias_routing_configuration::de_alias_routing_configuration(tokens, _value, depth + 1)?,
+                        );
+                    }
                     "RevisionId" => {
                         builder = builder.set_revision_id(
                             ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -196,11 +196,6 @@
                                 .transpose()?,
                         );
                     }
-                    "RoutingConfig" => {
-                        builder = builder.set_routing_config(
-                            super::protocol_serde::shape_alias_routing_configuration::de_alias_routing_configuration(tokens, _value, depth + 1)?,
-                        );
-                    }
                     _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                 }
             }
```

### `src/protocol_serde/shape_update_alias_input.rs`

```diff
--- reference/src/protocol_serde/shape_update_alias_input.rs
+++ generated/src/protocol_serde/shape_update_alias_input.rs
@@ -3,20 +3,26 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::update_alias::UpdateAliasInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.description {
-        object.key("Description").string(var_1.as_str());
+    if let Some(var_1) = &input.function_name {
+        object.key("FunctionName").string(var_1.as_str());
     }
-    if let Some(var_2) = &input.function_version {
-        object.key("FunctionVersion").string(var_2.as_str());
+    if let Some(var_2) = &input.name {
+        object.key("Name").string(var_2.as_str());
+    }
+    if let Some(var_3) = &input.function_version {
+        object.key("FunctionVersion").string(var_3.as_str());
     }
-    if let Some(var_3) = &input.revision_id {
-        object.key("RevisionId").string(var_3.as_str());
+    if let Some(var_4) = &input.description {
+        object.key("Description").string(var_4.as_str());
     }
-    if let Some(var_4) = &input.routing_config {
+    if let Some(var_5) = &input.routing_config {
         #[allow(unused_mut)]
-        let mut object_5 = object.key("RoutingConfig").start_object();
-        super::protocol_serde::shape_alias_routing_configuration::ser_alias_routing_configuration(&mut object_5, var_4)?;
-        object_5.finish();
+        let mut object_6 = object.key("RoutingConfig").start_object();
+        super::protocol_serde::shape_alias_routing_configuration::ser_alias_routing_configuration(&mut object_6, var_5)?;
+        object_6.finish();
+    }
+    if let Some(var_7) = &input.revision_id {
+        object.key("RevisionId").string(var_7.as_str());
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_update_capacity_provider.rs`

```diff
--- reference/src/protocol_serde/shape_update_capacity_provider.rs
+++ generated/src/protocol_serde/shape_update_capacity_provider.rs
@@ -96,13 +96,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::update_capacity_provider::UpdateCapacityProviderError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::update_capacity_provider::UpdateCapacityProviderError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
```

### `src/protocol_serde/shape_update_capacity_provider_input.rs`

```diff
--- reference/src/protocol_serde/shape_update_capacity_provider_input.rs
+++ generated/src/protocol_serde/shape_update_capacity_provider_input.rs
@@ -3,23 +3,26 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::update_capacity_provider::UpdateCapacityProviderInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.capacity_provider_scaling_config {
+    if let Some(var_1) = &input.capacity_provider_name {
+        object.key("CapacityProviderName").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.capacity_provider_scaling_config {
         #[allow(unused_mut)]
-        let mut object_2 = object.key("CapacityProviderScalingConfig").start_object();
-        super::protocol_serde::shape_capacity_provider_scaling_config::ser_capacity_provider_scaling_config(&mut object_2, var_1)?;
-        object_2.finish();
+        let mut object_3 = object.key("CapacityProviderScalingConfig").start_object();
+        super::protocol_serde::shape_capacity_provider_scaling_config::ser_capacity_provider_scaling_config(&mut object_3, var_2)?;
+        object_3.finish();
     }
-    if let Some(var_3) = &input.propagate_tags {
+    if let Some(var_4) = &input.propagate_tags {
         #[allow(unused_mut)]
-        let mut object_4 = object.key("PropagateTags").start_object();
-        super::protocol_serde::shape_propagate_tags::ser_propagate_tags(&mut object_4, var_3)?;
-        object_4.finish();
+        let mut object_5 = object.key("PropagateTags").start_object();
+        super::protocol_serde::shape_propagate_tags::ser_propagate_tags(&mut object_5, var_4)?;
+        object_5.finish();
     }
-    if let Some(var_5) = &input.telemetry_config {
+    if let Some(var_6) = &input.telemetry_config {
         #[allow(unused_mut)]
-        let mut object_6 = object.key("TelemetryConfig").start_object();
-        super::protocol_serde::shape_capacity_provider_telemetry_config::ser_capacity_provider_telemetry_config(&mut object_6, var_5)?;
-        object_6.finish();
+        let mut object_7 = object.key("TelemetryConfig").start_object();
+        super::protocol_serde::shape_capacity_provider_telemetry_config::ser_capacity_provider_telemetry_config(&mut object_7, var_6)?;
+        object_7.finish();
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_update_code_signing_config_input.rs`

```diff
--- reference/src/protocol_serde/shape_update_code_signing_config_input.rs
+++ generated/src/protocol_serde/shape_update_code_signing_config_input.rs
@@ -3,20 +3,23 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::update_code_signing_config::UpdateCodeSigningConfigInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.allowed_publishers {
-        #[allow(unused_mut)]
-        let mut object_2 = object.key("AllowedPublishers").start_object();
-        super::protocol_serde::shape_allowed_publishers::ser_allowed_publishers(&mut object_2, var_1)?;
-        object_2.finish();
+    if let Some(var_1) = &input.code_signing_config_arn {
+        object.key("CodeSigningConfigArn").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.description {
+        object.key("Description").string(var_2.as_str());
     }
-    if let Some(var_3) = &input.code_signing_policies {
+    if let Some(var_3) = &input.allowed_publishers {
         #[allow(unused_mut)]
-        let mut object_4 = object.key("CodeSigningPolicies").start_object();
-        super::protocol_serde::shape_code_signing_policies::ser_code_signing_policies(&mut object_4, var_3)?;
+        let mut object_4 = object.key("AllowedPublishers").start_object();
+        super::protocol_serde::shape_allowed_publishers::ser_allowed_publishers(&mut object_4, var_3)?;
         object_4.finish();
     }
-    if let Some(var_5) = &input.description {
-        object.key("Description").string(var_5.as_str());
+    if let Some(var_5) = &input.code_signing_policies {
+        #[allow(unused_mut)]
+        let mut object_6 = object.key("CodeSigningPolicies").start_object();
+        super::protocol_serde::shape_code_signing_policies::ser_code_signing_policies(&mut object_6, var_5)?;
+        object_6.finish();
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_update_event_source_mapping.rs`

```diff
--- reference/src/protocol_serde/shape_update_event_source_mapping.rs
+++ generated/src/protocol_serde/shape_update_event_source_mapping.rs
@@ -111,13 +111,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::update_event_source_mapping::UpdateEventSourceMappingError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::update_event_source_mapping::UpdateEventSourceMappingError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -175,15 +168,26 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "AmazonManagedKafkaEventSourceConfig" => {
-                    builder = builder.set_amazon_managed_kafka_event_source_config(
-                        super::protocol_serde::shape_amazon_managed_kafka_event_source_config::de_amazon_managed_kafka_event_source_config(
-                            tokens,
-                            _value,
-                            depth + 1,
-                        )?,
+                "UUID" => {
+                    builder = builder.set_uuid(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
+                }
+                "StartingPosition" => {
+                    builder = builder.set_starting_position(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| super::types::EventSourcePosition::from(u.as_ref())))
+                            .transpose()?,
                     );
                 }
+                "StartingPositionTimestamp" => {
+                    builder = builder.set_starting_position_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
+                        tokens.next(),
+                        ::aws_smithy_types::date_time::Format::EpochSeconds,
+                    )?);
+                }
                 "BatchSize" => {
                     builder = builder.set_batch_size(
                         ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
@@ -191,30 +195,22 @@
                             .transpose()?,
                     );
                 }
-                "BisectBatchOnFunctionError" => {
-                    builder = builder.set_bisect_batch_on_function_error(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
-                }
-                "DestinationConfig" => {
-                    builder = builder.set_destination_config(super::protocol_serde::shape_destination_config::de_destination_config(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
-                }
-                "DocumentDBEventSourceConfig" => {
-                    builder = builder.set_document_db_event_source_config(
-                        super::protocol_serde::shape_document_db_event_source_config::de_document_db_event_source_config(tokens, _value, depth + 1)?,
+                "MaximumBatchingWindowInSeconds" => {
+                    builder = builder.set_maximum_batching_window_in_seconds(
+                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                            .map(i32::try_from)
+                            .transpose()?,
                     );
                 }
-                "EventSourceArn" => {
-                    builder = builder.set_event_source_arn(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                "ParallelizationFactor" => {
+                    builder = builder.set_parallelization_factor(
+                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                            .map(i32::try_from)
                             .transpose()?,
                     );
                 }
-                "EventSourceMappingArn" => {
-                    builder = builder.set_event_source_mapping_arn(
+                "EventSourceArn" => {
+                    builder = builder.set_event_source_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
@@ -234,20 +230,36 @@
                         depth + 1,
                     )?);
                 }
-                "FunctionArn" => {
-                    builder = builder.set_function_arn(
+                "KMSKeyArn" => {
+                    builder = builder.set_kms_key_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "FunctionResponseTypes" => {
-                    builder = builder.set_function_response_types(
-                        super::protocol_serde::shape_function_response_type_list::de_function_response_type_list(tokens, _value, depth + 1)?,
+                "MetricsConfig" => {
+                    builder = builder.set_metrics_config(
+                        super::protocol_serde::shape_event_source_mapping_metrics_config::de_event_source_mapping_metrics_config(
+                            tokens,
+                            _value,
+                            depth + 1,
+                        )?,
                     );
                 }
-                "KMSKeyArn" => {
-                    builder = builder.set_kms_key_arn(
+                "LoggingConfig" => {
+                    builder = builder.set_logging_config(
+                        super::protocol_serde::shape_event_source_mapping_logging_config::de_event_source_mapping_logging_config(
+                            tokens,
+                            _value,
+                            depth + 1,
+                        )?,
+                    );
+                }
+                "ScalingConfig" => {
+                    builder = builder.set_scaling_config(super::protocol_serde::shape_scaling_config::de_scaling_config(tokens, _value, depth + 1)?);
+                }
+                "FunctionArn" => {
+                    builder = builder.set_function_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
@@ -266,22 +278,43 @@
                             .transpose()?,
                     );
                 }
-                "LoggingConfig" => {
-                    builder = builder.set_logging_config(
-                        super::protocol_serde::shape_event_source_mapping_logging_config::de_event_source_mapping_logging_config(
-                            tokens,
-                            _value,
-                            depth + 1,
-                        )?,
+                "State" => {
+                    builder = builder.set_state(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
                     );
                 }
-                "MaximumBatchingWindowInSeconds" => {
-                    builder = builder.set_maximum_batching_window_in_seconds(
-                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
-                            .map(i32::try_from)
+                "StateTransitionReason" => {
+                    builder = builder.set_state_transition_reason(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
+                "DestinationConfig" => {
+                    builder = builder.set_destination_config(super::protocol_serde::shape_destination_config::de_destination_config(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
+                "Topics" => {
+                    builder = builder.set_topics(super::protocol_serde::shape_topics::de_topics(tokens, _value, depth + 1)?);
+                }
+                "Queues" => {
+                    builder = builder.set_queues(super::protocol_serde::shape_queues::de_queues(tokens, _value, depth + 1)?);
+                }
+                "SourceAccessConfigurations" => {
+                    builder = builder.set_source_access_configurations(
+                        super::protocol_serde::shape_source_access_configurations::de_source_access_configurations(tokens, _value, depth + 1)?,
+                    );
+                }
+                "SelfManagedEventSource" => {
+                    builder = builder.set_self_managed_event_source(
+                        super::protocol_serde::shape_self_managed_event_source::de_self_managed_event_source(tokens, _value, depth + 1)?,
+                    );
+                }
                 "MaximumRecordAgeInSeconds" => {
                     builder = builder.set_maximum_record_age_in_seconds(
                         ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
@@ -289,6 +322,9 @@
                             .transpose()?,
                     );
                 }
+                "BisectBatchOnFunctionError" => {
+                    builder = builder.set_bisect_batch_on_function_error(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
+                }
                 "MaximumRetryAttempts" => {
                     builder = builder.set_maximum_retry_attempts(
                         ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
@@ -296,36 +332,25 @@
                             .transpose()?,
                     );
                 }
-                "MetricsConfig" => {
-                    builder = builder.set_metrics_config(
-                        super::protocol_serde::shape_event_source_mapping_metrics_config::de_event_source_mapping_metrics_config(
-                            tokens,
-                            _value,
-                            depth + 1,
-                        )?,
-                    );
-                }
-                "ParallelizationFactor" => {
-                    builder = builder.set_parallelization_factor(
+                "TumblingWindowInSeconds" => {
+                    builder = builder.set_tumbling_window_in_seconds(
                         ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                             .map(i32::try_from)
                             .transpose()?,
                     );
                 }
-                "ProvisionedPollerConfig" => {
-                    builder = builder.set_provisioned_poller_config(
-                        super::protocol_serde::shape_provisioned_poller_config::de_provisioned_poller_config(tokens, _value, depth + 1)?,
+                "FunctionResponseTypes" => {
+                    builder = builder.set_function_response_types(
+                        super::protocol_serde::shape_function_response_type_list::de_function_response_type_list(tokens, _value, depth + 1)?,
                     );
                 }
-                "Queues" => {
-                    builder = builder.set_queues(super::protocol_serde::shape_queues::de_queues(tokens, _value, depth + 1)?);
-                }
-                "ScalingConfig" => {
-                    builder = builder.set_scaling_config(super::protocol_serde::shape_scaling_config::de_scaling_config(tokens, _value, depth + 1)?);
-                }
-                "SelfManagedEventSource" => {
-                    builder = builder.set_self_managed_event_source(
-                        super::protocol_serde::shape_self_managed_event_source::de_self_managed_event_source(tokens, _value, depth + 1)?,
+                "AmazonManagedKafkaEventSourceConfig" => {
+                    builder = builder.set_amazon_managed_kafka_event_source_config(
+                        super::protocol_serde::shape_amazon_managed_kafka_event_source_config::de_amazon_managed_kafka_event_source_config(
+                            tokens,
+                            _value,
+                            depth + 1,
+                        )?,
                     );
                 }
                 "SelfManagedKafkaEventSourceConfig" => {
@@ -337,53 +362,21 @@
                         )?,
                     );
                 }
-                "SourceAccessConfigurations" => {
-                    builder = builder.set_source_access_configurations(
-                        super::protocol_serde::shape_source_access_configurations::de_source_access_configurations(tokens, _value, depth + 1)?,
-                    );
-                }
-                "StartingPosition" => {
-                    builder = builder.set_starting_position(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| super::types::EventSourcePosition::from(u.as_ref())))
-                            .transpose()?,
-                    );
-                }
-                "StartingPositionTimestamp" => {
-                    builder = builder.set_starting_position_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
-                        tokens.next(),
-                        ::aws_smithy_types::date_time::Format::EpochSeconds,
-                    )?);
-                }
-                "State" => {
-                    builder = builder.set_state(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
+                "DocumentDBEventSourceConfig" => {
+                    builder = builder.set_document_db_event_source_config(
+                        super::protocol_serde::shape_document_db_event_source_config::de_document_db_event_source_config(tokens, _value, depth + 1)?,
                     );
                 }
-                "StateTransitionReason" => {
-                    builder = builder.set_state_transition_reason(
+                "EventSourceMappingArn" => {
+                    builder = builder.set_event_source_mapping_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "Topics" => {
-                    builder = builder.set_topics(super::protocol_serde::shape_topics::de_topics(tokens, _value, depth + 1)?);
-                }
-                "TumblingWindowInSeconds" => {
-                    builder = builder.set_tumbling_window_in_seconds(
-                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
-                            .map(i32::try_from)
-                            .transpose()?,
-                    );
-                }
-                "UUID" => {
-                    builder = builder.set_uuid(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
+                "ProvisionedPollerConfig" => {
+                    builder = builder.set_provisioned_poller_config(
+                        super::protocol_serde::shape_provisioned_poller_config::de_provisioned_poller_config(tokens, _value, depth + 1)?,
                     );
                 }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
```

### `src/protocol_serde/shape_update_event_source_mapping_input.rs`

```diff
--- reference/src/protocol_serde/shape_update_event_source_mapping_input.rs
+++ generated/src/protocol_serde/shape_update_event_source_mapping_input.rs
@@ -3,128 +3,131 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::update_event_source_mapping::UpdateEventSourceMappingInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.amazon_managed_kafka_event_source_config {
-        #[allow(unused_mut)]
-        let mut object_2 = object.key("AmazonManagedKafkaEventSourceConfig").start_object();
-        super::protocol_serde::shape_amazon_managed_kafka_event_source_config::ser_amazon_managed_kafka_event_source_config(&mut object_2, var_1)?;
-        object_2.finish();
+    if let Some(var_1) = &input.uuid {
+        object.key("UUID").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.function_name {
+        object.key("FunctionName").string(var_2.as_str());
+    }
+    if let Some(var_3) = &input.enabled {
+        object.key("Enabled").boolean(*var_3);
     }
-    if let Some(var_3) = &input.batch_size {
+    if let Some(var_4) = &input.batch_size {
         object.key("BatchSize").number(
             #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_3).into()),
+            ::aws_smithy_types::Number::NegInt((*var_4).into()),
         );
     }
-    if let Some(var_4) = &input.bisect_batch_on_function_error {
-        object.key("BisectBatchOnFunctionError").boolean(*var_4);
-    }
-    if let Some(var_5) = &input.destination_config {
+    if let Some(var_5) = &input.filter_criteria {
         #[allow(unused_mut)]
-        let mut object_6 = object.key("DestinationConfig").start_object();
-        super::protocol_serde::shape_destination_config::ser_destination_config(&mut object_6, var_5)?;
+        let mut object_6 = object.key("FilterCriteria").start_object();
+        super::protocol_serde::shape_filter_criteria::ser_filter_criteria(&mut object_6, var_5)?;
         object_6.finish();
     }
-    if let Some(var_7) = &input.document_db_event_source_config {
+    if let Some(var_7) = &input.kms_key_arn {
+        object.key("KMSKeyArn").string(var_7.as_str());
+    }
+    if let Some(var_8) = &input.metrics_config {
         #[allow(unused_mut)]
-        let mut object_8 = object.key("DocumentDBEventSourceConfig").start_object();
-        super::protocol_serde::shape_document_db_event_source_config::ser_document_db_event_source_config(&mut object_8, var_7)?;
-        object_8.finish();
-    }
-    if let Some(var_9) = &input.enabled {
-        object.key("Enabled").boolean(*var_9);
+        let mut object_9 = object.key("MetricsConfig").start_object();
+        super::protocol_serde::shape_event_source_mapping_metrics_config::ser_event_source_mapping_metrics_config(&mut object_9, var_8)?;
+        object_9.finish();
     }
-    if let Some(var_10) = &input.filter_criteria {
+    if let Some(var_10) = &input.logging_config {
         #[allow(unused_mut)]
-        let mut object_11 = object.key("FilterCriteria").start_object();
-        super::protocol_serde::shape_filter_criteria::ser_filter_criteria(&mut object_11, var_10)?;
+        let mut object_11 = object.key("LoggingConfig").start_object();
+        super::protocol_serde::shape_event_source_mapping_logging_config::ser_event_source_mapping_logging_config(&mut object_11, var_10)?;
         object_11.finish();
     }
-    if let Some(var_12) = &input.function_name {
-        object.key("FunctionName").string(var_12.as_str());
+    if let Some(var_12) = &input.scaling_config {
+        #[allow(unused_mut)]
+        let mut object_13 = object.key("ScalingConfig").start_object();
+        super::protocol_serde::shape_scaling_config::ser_scaling_config(&mut object_13, var_12)?;
+        object_13.finish();
     }
-    if let Some(var_13) = &input.function_response_types {
-        let mut array_14 = object.key("FunctionResponseTypes").start_array();
-        for item_15 in var_13 {
-            {
-                array_14.value().string(item_15.as_str());
-            }
-        }
-        array_14.finish();
+    if let Some(var_14) = &input.maximum_batching_window_in_seconds {
+        object.key("MaximumBatchingWindowInSeconds").number(
+            #[allow(clippy::useless_conversion)]
+            ::aws_smithy_types::Number::NegInt((*var_14).into()),
+        );
     }
-    if let Some(var_16) = &input.kms_key_arn {
-        object.key("KMSKeyArn").string(var_16.as_str());
+    if let Some(var_15) = &input.parallelization_factor {
+        object.key("ParallelizationFactor").number(
+            #[allow(clippy::useless_conversion)]
+            ::aws_smithy_types::Number::NegInt((*var_15).into()),
+        );
     }
-    if let Some(var_17) = &input.logging_config {
+    if let Some(var_16) = &input.destination_config {
         #[allow(unused_mut)]
-        let mut object_18 = object.key("LoggingConfig").start_object();
-        super::protocol_serde::shape_event_source_mapping_logging_config::ser_event_source_mapping_logging_config(&mut object_18, var_17)?;
-        object_18.finish();
+        let mut object_17 = object.key("DestinationConfig").start_object();
+        super::protocol_serde::shape_destination_config::ser_destination_config(&mut object_17, var_16)?;
+        object_17.finish();
     }
-    if let Some(var_19) = &input.maximum_batching_window_in_seconds {
-        object.key("MaximumBatchingWindowInSeconds").number(
+    if let Some(var_18) = &input.maximum_record_age_in_seconds {
+        object.key("MaximumRecordAgeInSeconds").number(
             #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_19).into()),
+            ::aws_smithy_types::Number::NegInt((*var_18).into()),
         );
     }
-    if let Some(var_20) = &input.maximum_record_age_in_seconds {
-        object.key("MaximumRecordAgeInSeconds").number(
+    if let Some(var_19) = &input.bisect_batch_on_function_error {
+        object.key("BisectBatchOnFunctionError").boolean(*var_19);
+    }
+    if let Some(var_20) = &input.maximum_retry_attempts {
+        object.key("MaximumRetryAttempts").number(
             #[allow(clippy::useless_conversion)]
             ::aws_smithy_types::Number::NegInt((*var_20).into()),
         );
     }
-    if let Some(var_21) = &input.maximum_retry_attempts {
-        object.key("MaximumRetryAttempts").number(
+    if let Some(var_21) = &input.tumbling_window_in_seconds {
+        object.key("TumblingWindowInSeconds").number(
             #[allow(clippy::useless_conversion)]
             ::aws_smithy_types::Number::NegInt((*var_21).into()),
         );
     }
-    if let Some(var_22) = &input.metrics_config {
-        #[allow(unused_mut)]
-        let mut object_23 = object.key("MetricsConfig").start_object();
-        super::protocol_serde::shape_event_source_mapping_metrics_config::ser_event_source_mapping_metrics_config(&mut object_23, var_22)?;
-        object_23.finish();
+    if let Some(var_22) = &input.source_access_configurations {
+        let mut array_23 = object.key("SourceAccessConfigurations").start_array();
+        for item_24 in var_22 {
+            {
+                #[allow(unused_mut)]
+                let mut object_25 = array_23.value().start_object();
+                super::protocol_serde::shape_source_access_configuration::ser_source_access_configuration(&mut object_25, item_24)?;
+                object_25.finish();
+            }
+        }
+        array_23.finish();
     }
-    if let Some(var_24) = &input.parallelization_factor {
-        object.key("ParallelizationFactor").number(
-            #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_24).into()),
-        );
+    if let Some(var_26) = &input.function_response_types {
+        let mut array_27 = object.key("FunctionResponseTypes").start_array();
+        for item_28 in var_26 {
+            {
+                array_27.value().string(item_28.as_str());
+            }
+        }
+        array_27.finish();
     }
-    if let Some(var_25) = &input.provisioned_poller_config {
+    if let Some(var_29) = &input.amazon_managed_kafka_event_source_config {
         #[allow(unused_mut)]
-        let mut object_26 = object.key("ProvisionedPollerConfig").start_object();
-        super::protocol_serde::shape_provisioned_poller_config::ser_provisioned_poller_config(&mut object_26, var_25)?;
-        object_26.finish();
+        let mut object_30 = object.key("AmazonManagedKafkaEventSourceConfig").start_object();
+        super::protocol_serde::shape_amazon_managed_kafka_event_source_config::ser_amazon_managed_kafka_event_source_config(&mut object_30, var_29)?;
+        object_30.finish();
     }
-    if let Some(var_27) = &input.scaling_config {
+    if let Some(var_31) = &input.self_managed_kafka_event_source_config {
         #[allow(unused_mut)]
-        let mut object_28 = object.key("ScalingConfig").start_object();
-        super::protocol_serde::shape_scaling_config::ser_scaling_config(&mut object_28, var_27)?;
-        object_28.finish();
+        let mut object_32 = object.key("SelfManagedKafkaEventSourceConfig").start_object();
+        super::protocol_serde::shape_self_managed_kafka_event_source_config::ser_self_managed_kafka_event_source_config(&mut object_32, var_31)?;
+        object_32.finish();
     }
-    if let Some(var_29) = &input.self_managed_kafka_event_source_config {
+    if let Some(var_33) = &input.document_db_event_source_config {
         #[allow(unused_mut)]
-        let mut object_30 = object.key("SelfManagedKafkaEventSourceConfig").start_object();
-        super::protocol_serde::shape_self_managed_kafka_event_source_config::ser_self_managed_kafka_event_source_config(&mut object_30, var_29)?;
-        object_30.finish();
-    }
-    if let Some(var_31) = &input.source_access_configurations {
-        let mut array_32 = object.key("SourceAccessConfigurations").start_array();
-        for item_33 in var_31 {
-            {
-                #[allow(unused_mut)]
-                let mut object_34 = array_32.value().start_object();
-                super::protocol_serde::shape_source_access_configuration::ser_source_access_configuration(&mut object_34, item_33)?;
-                object_34.finish();
-            }
-        }
-        array_32.finish();
+        let mut object_34 = object.key("DocumentDBEventSourceConfig").start_object();
+        super::protocol_serde::shape_document_db_event_source_config::ser_document_db_event_source_config(&mut object_34, var_33)?;
+        object_34.finish();
     }
-    if let Some(var_35) = &input.tumbling_window_in_seconds {
-        object.key("TumblingWindowInSeconds").number(
-            #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_35).into()),
-        );
+    if let Some(var_35) = &input.provisioned_poller_config {
+        #[allow(unused_mut)]
+        let mut object_36 = object.key("ProvisionedPollerConfig").start_object();
+        super::protocol_serde::shape_provisioned_poller_config::ser_provisioned_poller_config(&mut object_36, var_35)?;
+        object_36.finish();
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_update_function_code.rs`

```diff
--- reference/src/protocol_serde/shape_update_function_code.rs
+++ generated/src/protocol_serde/shape_update_function_code.rs
@@ -179,13 +179,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::update_function_code::UpdateFunctionCodeError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::update_function_code::UpdateFunctionCodeError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -243,20 +236,36 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "Architectures" => {
-                    builder = builder.set_architectures(super::protocol_serde::shape_architectures_list::de_architectures_list(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
+                "FunctionName" => {
+                    builder = builder.set_function_name(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
+                }
+                "FunctionArn" => {
+                    builder = builder.set_function_arn(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
+                }
+                "Runtime" => {
+                    builder = builder.set_runtime(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| super::types::Runtime::from(u.as_ref())))
+                            .transpose()?,
+                    );
                 }
-                "CapacityProviderConfig" => {
-                    builder = builder.set_capacity_provider_config(
-                        super::protocol_serde::shape_capacity_provider_config::de_capacity_provider_config(tokens, _value, depth + 1)?,
+                "Role" => {
+                    builder = builder.set_role(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
                     );
                 }
-                "CodeSha256" => {
-                    builder = builder.set_code_sha256(
+                "Handler" => {
+                    builder = builder.set_handler(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
@@ -269,93 +278,125 @@
                             .transpose()?,
                     );
                 }
-                "ConfigSha256" => {
-                    builder = builder.set_config_sha256(
+                "Description" => {
+                    builder = builder.set_description(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "DeadLetterConfig" => {
-                    builder = builder.set_dead_letter_config(super::protocol_serde::shape_dead_letter_config::de_dead_letter_config(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
+                "Timeout" => {
+                    builder = builder.set_timeout(
+                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                            .map(i32::try_from)
+                            .transpose()?,
+                    );
                 }
-                "Description" => {
-                    builder = builder.set_description(
+                "MemorySize" => {
+                    builder = builder.set_memory_size(
+                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                            .map(i32::try_from)
+                            .transpose()?,
+                    );
+                }
+                "LastModified" => {
+                    builder = builder.set_last_modified(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "DurableConfig" => {
-                    builder = builder.set_durable_config(super::protocol_serde::shape_durable_config::de_durable_config(tokens, _value, depth + 1)?);
+                "CodeSha256" => {
+                    builder = builder.set_code_sha256(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
                 }
-                "Environment" => {
-                    builder = builder.set_environment(super::protocol_serde::shape_environment_response::de_environment_response(
+                "Version" => {
+                    builder = builder.set_version(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
+                }
+                "VpcConfig" => {
+                    builder = builder.set_vpc_config(super::protocol_serde::shape_vpc_config_response::de_vpc_config_response(
                         tokens,
                         _value,
                         depth + 1,
                     )?);
                 }
-                "EphemeralStorage" => {
-                    builder = builder.set_ephemeral_storage(super::protocol_serde::shape_ephemeral_storage::de_ephemeral_storage(
+                "DeadLetterConfig" => {
+                    builder = builder.set_dead_letter_config(super::protocol_serde::shape_dead_letter_config::de_dead_letter_config(
                         tokens,
                         _value,
                         depth + 1,
                     )?);
                 }
-                "FileSystemConfigs" => {
-                    builder = builder.set_file_system_configs(super::protocol_serde::shape_file_system_config_list::de_file_system_config_list(
+                "Environment" => {
+                    builder = builder.set_environment(super::protocol_serde::shape_environment_response::de_environment_response(
                         tokens,
                         _value,
                         depth + 1,
                     )?);
                 }
-                "FunctionArn" => {
-                    builder = builder.set_function_arn(
+                "KMSKeyArn" => {
+                    builder = builder.set_kms_key_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "FunctionName" => {
-                    builder = builder.set_function_name(
+                "TracingConfig" => {
+                    builder = builder.set_tracing_config(super::protocol_serde::shape_tracing_config_response::de_tracing_config_response(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
+                "MasterArn" => {
+                    builder = builder.set_master_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "Handler" => {
-                    builder = builder.set_handler(
+                "RevisionId" => {
+                    builder = builder.set_revision_id(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "ImageConfigResponse" => {
-                    builder = builder.set_image_config_response(super::protocol_serde::shape_image_config_response::de_image_config_response(
+                "Layers" => {
+                    builder = builder.set_layers(super::protocol_serde::shape_layers_reference_list::de_layers_reference_list(
                         tokens,
                         _value,
                         depth + 1,
                     )?);
                 }
-                "KMSKeyArn" => {
-                    builder = builder.set_kms_key_arn(
+                "State" => {
+                    builder = builder.set_state(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .map(|s| s.to_unescaped().map(|u| super::types::State::from(u.as_ref())))
                             .transpose()?,
                     );
                 }
-                "LastModified" => {
-                    builder = builder.set_last_modified(
+                "StateReason" => {
+                    builder = builder.set_state_reason(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
+                "StateReasonCode" => {
+                    builder = builder.set_state_reason_code(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| super::types::StateReasonCode::from(u.as_ref())))
+                            .transpose()?,
+                    );
+                }
                 "LastUpdateStatus" => {
                     builder = builder.set_last_update_status(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -377,27 +418,24 @@
                             .transpose()?,
                     );
                 }
-                "Layers" => {
-                    builder = builder.set_layers(super::protocol_serde::shape_layers_reference_list::de_layers_reference_list(
+                "FileSystemConfigs" => {
+                    builder = builder.set_file_system_configs(super::protocol_serde::shape_file_system_config_list::de_file_system_config_list(
                         tokens,
                         _value,
                         depth + 1,
                     )?);
                 }
-                "LoggingConfig" => {
-                    builder = builder.set_logging_config(super::protocol_serde::shape_logging_config::de_logging_config(tokens, _value, depth + 1)?);
-                }
-                "MasterArn" => {
-                    builder = builder.set_master_arn(
+                "SigningProfileVersionArn" => {
+                    builder = builder.set_signing_profile_version_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "MemorySize" => {
-                    builder = builder.set_memory_size(
-                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
-                            .map(i32::try_from)
+                "SigningJobArn" => {
+                    builder = builder.set_signing_job_arn(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
@@ -408,48 +446,27 @@
                             .transpose()?,
                     );
                 }
-                "RevisionId" => {
-                    builder = builder.set_revision_id(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
+                "ImageConfigResponse" => {
+                    builder = builder.set_image_config_response(super::protocol_serde::shape_image_config_response::de_image_config_response(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
                 }
-                "Role" => {
-                    builder = builder.set_role(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
+                "Architectures" => {
+                    builder = builder.set_architectures(super::protocol_serde::shape_architectures_list::de_architectures_list(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
                 }
-                "Runtime" => {
-                    builder = builder.set_runtime(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| super::types::Runtime::from(u.as_ref())))
-                            .transpose()?,
-                    );
-                }
-                "RuntimeVersionConfig" => {
-                    builder = builder.set_runtime_version_config(super::protocol_serde::shape_runtime_version_config::de_runtime_version_config(
+                "EphemeralStorage" => {
+                    builder = builder.set_ephemeral_storage(super::protocol_serde::shape_ephemeral_storage::de_ephemeral_storage(
                         tokens,
                         _value,
                         depth + 1,
                     )?);
                 }
-                "SigningJobArn" => {
-                    builder = builder.set_signing_job_arn(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
-                }
-                "SigningProfileVersionArn" => {
-                    builder = builder.set_signing_profile_version_arn(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
-                }
                 "SnapStart" => {
                     builder = builder.set_snap_start(super::protocol_serde::shape_snap_start_response::de_snap_start_response(
                         tokens,
@@ -457,57 +474,33 @@
                         depth + 1,
                     )?);
                 }
-                "State" => {
-                    builder = builder.set_state(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| super::types::State::from(u.as_ref())))
-                            .transpose()?,
-                    );
-                }
-                "StateReason" => {
-                    builder = builder.set_state_reason(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
+                "RuntimeVersionConfig" => {
+                    builder = builder.set_runtime_version_config(super::protocol_serde::shape_runtime_version_config::de_runtime_version_config(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
                 }
-                "StateReasonCode" => {
-                    builder = builder.set_state_reason_code(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| super::types::StateReasonCode::from(u.as_ref())))
-                            .transpose()?,
-                    );
+                "LoggingConfig" => {
+                    builder = builder.set_logging_config(super::protocol_serde::shape_logging_config::de_logging_config(tokens, _value, depth + 1)?);
                 }
                 "TenancyConfig" => {
                     builder = builder.set_tenancy_config(super::protocol_serde::shape_tenancy_config::de_tenancy_config(tokens, _value, depth + 1)?);
                 }
-                "Timeout" => {
-                    builder = builder.set_timeout(
-                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
-                            .map(i32::try_from)
-                            .transpose()?,
+                "CapacityProviderConfig" => {
+                    builder = builder.set_capacity_provider_config(
+                        super::protocol_serde::shape_capacity_provider_config::de_capacity_provider_config(tokens, _value, depth + 1)?,
                     );
                 }
-                "TracingConfig" => {
-                    builder = builder.set_tracing_config(super::protocol_serde::shape_tracing_config_response::de_tracing_config_response(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
-                }
-                "Version" => {
-                    builder = builder.set_version(
+                "ConfigSha256" => {
+                    builder = builder.set_config_sha256(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "VpcConfig" => {
-                    builder = builder.set_vpc_config(super::protocol_serde::shape_vpc_config_response::de_vpc_config_response(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
+                "DurableConfig" => {
+                    builder = builder.set_durable_config(super::protocol_serde::shape_durable_config::de_durable_config(tokens, _value, depth + 1)?);
                 }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
```

### `src/protocol_serde/shape_update_function_code_input.rs`

```diff
--- reference/src/protocol_serde/shape_update_function_code_input.rs
+++ generated/src/protocol_serde/shape_update_function_code_input.rs
@@ -3,47 +3,50 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::update_function_code::UpdateFunctionCodeInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.architectures {
-        let mut array_2 = object.key("Architectures").start_array();
-        for item_3 in var_1 {
-            {
-                array_2.value().string(item_3.as_str());
-            }
-        }
-        array_2.finish();
+    if let Some(var_1) = &input.function_name {
+        object.key("FunctionName").string(var_1.as_str());
     }
-    if let Some(var_4) = &input.dry_run {
-        object.key("DryRun").boolean(*var_4);
+    if let Some(var_2) = &input.zip_file {
+        object.key("ZipFile").string_unchecked(&::aws_smithy_types::base64::encode(var_2));
     }
-    if let Some(var_5) = &input.image_uri {
-        object.key("ImageUri").string(var_5.as_str());
+    if let Some(var_3) = &input.s3_bucket {
+        object.key("S3Bucket").string(var_3.as_str());
     }
-    if let Some(var_6) = &input.publish {
-        object.key("Publish").boolean(*var_6);
+    if let Some(var_4) = &input.s3_key {
+        object.key("S3Key").string(var_4.as_str());
     }
-    if let Some(var_7) = &input.publish_to {
-        object.key("PublishTo").string(var_7.as_str());
+    if let Some(var_5) = &input.s3_object_version {
+        object.key("S3ObjectVersion").string(var_5.as_str());
     }
-    if let Some(var_8) = &input.revision_id {
-        object.key("RevisionId").string(var_8.as_str());
+    if let Some(var_6) = &input.s3_object_storage_mode {
+        object.key("S3ObjectStorageMode").string(var_6.as_str());
     }
-    if let Some(var_9) = &input.s3_bucket {
-        object.key("S3Bucket").string(var_9.as_str());
+    if let Some(var_7) = &input.image_uri {
+        object.key("ImageUri").string(var_7.as_str());
     }
-    if let Some(var_10) = &input.s3_key {
-        object.key("S3Key").string(var_10.as_str());
+    if let Some(var_8) = &input.architectures {
+        let mut array_9 = object.key("Architectures").start_array();
+        for item_10 in var_8 {
+            {
+                array_9.value().string(item_10.as_str());
+            }
+        }
+        array_9.finish();
     }
-    if let Some(var_11) = &input.s3_object_storage_mode {
-        object.key("S3ObjectStorageMode").string(var_11.as_str());
+    if let Some(var_11) = &input.publish {
+        object.key("Publish").boolean(*var_11);
     }
-    if let Some(var_12) = &input.s3_object_version {
-        object.key("S3ObjectVersion").string(var_12.as_str());
+    if let Some(var_12) = &input.publish_to {
+        object.key("PublishTo").string(var_12.as_str());
     }
-    if let Some(var_13) = &input.source_kms_key_arn {
-        object.key("SourceKMSKeyArn").string(var_13.as_str());
+    if let Some(var_13) = &input.dry_run {
+        object.key("DryRun").boolean(*var_13);
     }
-    if let Some(var_14) = &input.zip_file {
-        object.key("ZipFile").string_unchecked(&::aws_smithy_types::base64::encode(var_14));
+    if let Some(var_14) = &input.revision_id {
+        object.key("RevisionId").string(var_14.as_str());
+    }
+    if let Some(var_15) = &input.source_kms_key_arn {
+        object.key("SourceKMSKeyArn").string(var_15.as_str());
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_update_function_configuration.rs`

```diff
--- reference/src/protocol_serde/shape_update_function_configuration.rs
+++ generated/src/protocol_serde/shape_update_function_configuration.rs
@@ -177,13 +177,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::update_function_configuration::UpdateFunctionConfigurationError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::update_function_configuration::UpdateFunctionConfigurationError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -241,20 +234,36 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "Architectures" => {
-                    builder = builder.set_architectures(super::protocol_serde::shape_architectures_list::de_architectures_list(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
+                "FunctionName" => {
+                    builder = builder.set_function_name(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
+                }
+                "FunctionArn" => {
+                    builder = builder.set_function_arn(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
+                }
+                "Runtime" => {
+                    builder = builder.set_runtime(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| super::types::Runtime::from(u.as_ref())))
+                            .transpose()?,
+                    );
                 }
-                "CapacityProviderConfig" => {
-                    builder = builder.set_capacity_provider_config(
-                        super::protocol_serde::shape_capacity_provider_config::de_capacity_provider_config(tokens, _value, depth + 1)?,
+                "Role" => {
+                    builder = builder.set_role(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
                     );
                 }
-                "CodeSha256" => {
-                    builder = builder.set_code_sha256(
+                "Handler" => {
+                    builder = builder.set_handler(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
@@ -267,93 +276,125 @@
                             .transpose()?,
                     );
                 }
-                "ConfigSha256" => {
-                    builder = builder.set_config_sha256(
+                "Description" => {
+                    builder = builder.set_description(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "DeadLetterConfig" => {
-                    builder = builder.set_dead_letter_config(super::protocol_serde::shape_dead_letter_config::de_dead_letter_config(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
+                "Timeout" => {
+                    builder = builder.set_timeout(
+                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                            .map(i32::try_from)
+                            .transpose()?,
+                    );
                 }
-                "Description" => {
-                    builder = builder.set_description(
+                "MemorySize" => {
+                    builder = builder.set_memory_size(
+                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                            .map(i32::try_from)
+                            .transpose()?,
+                    );
+                }
+                "LastModified" => {
+                    builder = builder.set_last_modified(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "DurableConfig" => {
-                    builder = builder.set_durable_config(super::protocol_serde::shape_durable_config::de_durable_config(tokens, _value, depth + 1)?);
+                "CodeSha256" => {
+                    builder = builder.set_code_sha256(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
                 }
-                "Environment" => {
-                    builder = builder.set_environment(super::protocol_serde::shape_environment_response::de_environment_response(
+                "Version" => {
+                    builder = builder.set_version(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
+                }
+                "VpcConfig" => {
+                    builder = builder.set_vpc_config(super::protocol_serde::shape_vpc_config_response::de_vpc_config_response(
                         tokens,
                         _value,
                         depth + 1,
                     )?);
                 }
-                "EphemeralStorage" => {
-                    builder = builder.set_ephemeral_storage(super::protocol_serde::shape_ephemeral_storage::de_ephemeral_storage(
+                "DeadLetterConfig" => {
+                    builder = builder.set_dead_letter_config(super::protocol_serde::shape_dead_letter_config::de_dead_letter_config(
                         tokens,
                         _value,
                         depth + 1,
                     )?);
                 }
-                "FileSystemConfigs" => {
-                    builder = builder.set_file_system_configs(super::protocol_serde::shape_file_system_config_list::de_file_system_config_list(
+                "Environment" => {
+                    builder = builder.set_environment(super::protocol_serde::shape_environment_response::de_environment_response(
                         tokens,
                         _value,
                         depth + 1,
                     )?);
                 }
-                "FunctionArn" => {
-                    builder = builder.set_function_arn(
+                "KMSKeyArn" => {
+                    builder = builder.set_kms_key_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "FunctionName" => {
-                    builder = builder.set_function_name(
+                "TracingConfig" => {
+                    builder = builder.set_tracing_config(super::protocol_serde::shape_tracing_config_response::de_tracing_config_response(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
+                "MasterArn" => {
+                    builder = builder.set_master_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "Handler" => {
-                    builder = builder.set_handler(
+                "RevisionId" => {
+                    builder = builder.set_revision_id(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "ImageConfigResponse" => {
-                    builder = builder.set_image_config_response(super::protocol_serde::shape_image_config_response::de_image_config_response(
+                "Layers" => {
+                    builder = builder.set_layers(super::protocol_serde::shape_layers_reference_list::de_layers_reference_list(
                         tokens,
                         _value,
                         depth + 1,
                     )?);
                 }
-                "KMSKeyArn" => {
-                    builder = builder.set_kms_key_arn(
+                "State" => {
+                    builder = builder.set_state(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .map(|s| s.to_unescaped().map(|u| super::types::State::from(u.as_ref())))
                             .transpose()?,
                     );
                 }
-                "LastModified" => {
-                    builder = builder.set_last_modified(
+                "StateReason" => {
+                    builder = builder.set_state_reason(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
+                "StateReasonCode" => {
+                    builder = builder.set_state_reason_code(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| super::types::StateReasonCode::from(u.as_ref())))
+                            .transpose()?,
+                    );
+                }
                 "LastUpdateStatus" => {
                     builder = builder.set_last_update_status(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -375,27 +416,24 @@
                             .transpose()?,
                     );
                 }
-                "Layers" => {
-                    builder = builder.set_layers(super::protocol_serde::shape_layers_reference_list::de_layers_reference_list(
+                "FileSystemConfigs" => {
+                    builder = builder.set_file_system_configs(super::protocol_serde::shape_file_system_config_list::de_file_system_config_list(
                         tokens,
                         _value,
                         depth + 1,
                     )?);
                 }
-                "LoggingConfig" => {
-                    builder = builder.set_logging_config(super::protocol_serde::shape_logging_config::de_logging_config(tokens, _value, depth + 1)?);
-                }
-                "MasterArn" => {
-                    builder = builder.set_master_arn(
+                "SigningProfileVersionArn" => {
+                    builder = builder.set_signing_profile_version_arn(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "MemorySize" => {
-                    builder = builder.set_memory_size(
-                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
-                            .map(i32::try_from)
+                "SigningJobArn" => {
+                    builder = builder.set_signing_job_arn(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
@@ -406,48 +444,27 @@
                             .transpose()?,
                     );
                 }
-                "RevisionId" => {
-                    builder = builder.set_revision_id(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
+                "ImageConfigResponse" => {
+                    builder = builder.set_image_config_response(super::protocol_serde::shape_image_config_response::de_image_config_response(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
                 }
-                "Role" => {
-                    builder = builder.set_role(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
+                "Architectures" => {
+                    builder = builder.set_architectures(super::protocol_serde::shape_architectures_list::de_architectures_list(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
                 }
-                "Runtime" => {
-                    builder = builder.set_runtime(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| super::types::Runtime::from(u.as_ref())))
-                            .transpose()?,
-                    );
-                }
-                "RuntimeVersionConfig" => {
-                    builder = builder.set_runtime_version_config(super::protocol_serde::shape_runtime_version_config::de_runtime_version_config(
+                "EphemeralStorage" => {
+                    builder = builder.set_ephemeral_storage(super::protocol_serde::shape_ephemeral_storage::de_ephemeral_storage(
                         tokens,
                         _value,
                         depth + 1,
                     )?);
                 }
-                "SigningJobArn" => {
-                    builder = builder.set_signing_job_arn(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
-                }
-                "SigningProfileVersionArn" => {
-                    builder = builder.set_signing_profile_version_arn(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
-                }
                 "SnapStart" => {
                     builder = builder.set_snap_start(super::protocol_serde::shape_snap_start_response::de_snap_start_response(
                         tokens,
@@ -455,57 +472,33 @@
                         depth + 1,
                     )?);
                 }
-                "State" => {
-                    builder = builder.set_state(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| super::types::State::from(u.as_ref())))
-                            .transpose()?,
-                    );
-                }
-                "StateReason" => {
-                    builder = builder.set_state_reason(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
+                "RuntimeVersionConfig" => {
+                    builder = builder.set_runtime_version_config(super::protocol_serde::shape_runtime_version_config::de_runtime_version_config(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
                 }
-                "StateReasonCode" => {
-                    builder = builder.set_state_reason_code(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| super::types::StateReasonCode::from(u.as_ref())))
-                            .transpose()?,
-                    );
+                "LoggingConfig" => {
+                    builder = builder.set_logging_config(super::protocol_serde::shape_logging_config::de_logging_config(tokens, _value, depth + 1)?);
                 }
                 "TenancyConfig" => {
                     builder = builder.set_tenancy_config(super::protocol_serde::shape_tenancy_config::de_tenancy_config(tokens, _value, depth + 1)?);
                 }
-                "Timeout" => {
-                    builder = builder.set_timeout(
-                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
-                            .map(i32::try_from)
-                            .transpose()?,
+                "CapacityProviderConfig" => {
+                    builder = builder.set_capacity_provider_config(
+                        super::protocol_serde::shape_capacity_provider_config::de_capacity_provider_config(tokens, _value, depth + 1)?,
                     );
                 }
-                "TracingConfig" => {
-                    builder = builder.set_tracing_config(super::protocol_serde::shape_tracing_config_response::de_tracing_config_response(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
-                }
-                "Version" => {
-                    builder = builder.set_version(
+                "ConfigSha256" => {
+                    builder = builder.set_config_sha256(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
                     );
                 }
-                "VpcConfig" => {
-                    builder = builder.set_vpc_config(super::protocol_serde::shape_vpc_config_response::de_vpc_config_response(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
+                "DurableConfig" => {
+                    builder = builder.set_durable_config(super::protocol_serde::shape_durable_config::de_durable_config(tokens, _value, depth + 1)?);
                 }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
```

### `src/protocol_serde/shape_update_function_configuration_input.rs`

```diff
--- reference/src/protocol_serde/shape_update_function_configuration_input.rs
+++ generated/src/protocol_serde/shape_update_function_configuration_input.rs
@@ -3,93 +3,96 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::update_function_configuration::UpdateFunctionConfigurationInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.capacity_provider_config {
+    if let Some(var_1) = &input.function_name {
+        object.key("FunctionName").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.role {
+        object.key("Role").string(var_2.as_str());
+    }
+    if let Some(var_3) = &input.handler {
+        object.key("Handler").string(var_3.as_str());
+    }
+    if let Some(var_4) = &input.description {
+        object.key("Description").string(var_4.as_str());
+    }
+    if let Some(var_5) = &input.timeout {
+        object.key("Timeout").number(
+            #[allow(clippy::useless_conversion)]
+            ::aws_smithy_types::Number::NegInt((*var_5).into()),
+        );
+    }
+    if let Some(var_6) = &input.memory_size {
+        object.key("MemorySize").number(
+            #[allow(clippy::useless_conversion)]
+            ::aws_smithy_types::Number::NegInt((*var_6).into()),
+        );
+    }
+    if let Some(var_7) = &input.vpc_config {
         #[allow(unused_mut)]
-        let mut object_2 = object.key("CapacityProviderConfig").start_object();
-        super::protocol_serde::shape_capacity_provider_config::ser_capacity_provider_config(&mut object_2, var_1)?;
-        object_2.finish();
+        let mut object_8 = object.key("VpcConfig").start_object();
+        super::protocol_serde::shape_vpc_config::ser_vpc_config(&mut object_8, var_7)?;
+        object_8.finish();
     }
-    if let Some(var_3) = &input.dead_letter_config {
+    if let Some(var_9) = &input.environment {
         #[allow(unused_mut)]
-        let mut object_4 = object.key("DeadLetterConfig").start_object();
-        super::protocol_serde::shape_dead_letter_config::ser_dead_letter_config(&mut object_4, var_3)?;
-        object_4.finish();
+        let mut object_10 = object.key("Environment").start_object();
+        super::protocol_serde::shape_environment::ser_environment(&mut object_10, var_9)?;
+        object_10.finish();
     }
-    if let Some(var_5) = &input.description {
-        object.key("Description").string(var_5.as_str());
+    if let Some(var_11) = &input.runtime {
+        object.key("Runtime").string(var_11.as_str());
     }
-    if let Some(var_6) = &input.durable_config {
+    if let Some(var_12) = &input.dead_letter_config {
         #[allow(unused_mut)]
-        let mut object_7 = object.key("DurableConfig").start_object();
-        super::protocol_serde::shape_durable_config::ser_durable_config(&mut object_7, var_6)?;
-        object_7.finish();
+        let mut object_13 = object.key("DeadLetterConfig").start_object();
+        super::protocol_serde::shape_dead_letter_config::ser_dead_letter_config(&mut object_13, var_12)?;
+        object_13.finish();
+    }
+    if let Some(var_14) = &input.kms_key_arn {
+        object.key("KMSKeyArn").string(var_14.as_str());
     }
-    if let Some(var_8) = &input.environment {
+    if let Some(var_15) = &input.tracing_config {
         #[allow(unused_mut)]
-        let mut object_9 = object.key("Environment").start_object();
-        super::protocol_serde::shape_environment::ser_environment(&mut object_9, var_8)?;
-        object_9.finish();
+        let mut object_16 = object.key("TracingConfig").start_object();
+        super::protocol_serde::shape_tracing_config::ser_tracing_config(&mut object_16, var_15)?;
+        object_16.finish();
     }
-    if let Some(var_10) = &input.ephemeral_storage {
-        #[allow(unused_mut)]
-        let mut object_11 = object.key("EphemeralStorage").start_object();
-        super::protocol_serde::shape_ephemeral_storage::ser_ephemeral_storage(&mut object_11, var_10)?;
-        object_11.finish();
+    if let Some(var_17) = &input.revision_id {
+        object.key("RevisionId").string(var_17.as_str());
     }
-    if let Some(var_12) = &input.file_system_configs {
-        let mut array_13 = object.key("FileSystemConfigs").start_array();
-        for item_14 in var_12 {
+    if let Some(var_18) = &input.layers {
+        let mut array_19 = object.key("Layers").start_array();
+        for item_20 in var_18 {
             {
-                #[allow(unused_mut)]
-                let mut object_15 = array_13.value().start_object();
-                super::protocol_serde::shape_file_system_config::ser_file_system_config(&mut object_15, item_14)?;
-                object_15.finish();
+                array_19.value().string(item_20.as_str());
             }
         }
-        array_13.finish();
+        array_19.finish();
     }
-    if let Some(var_16) = &input.handler {
-        object.key("Handler").string(var_16.as_str());
-    }
-    if let Some(var_17) = &input.image_config {
-        #[allow(unused_mut)]
-        let mut object_18 = object.key("ImageConfig").start_object();
-        super::protocol_serde::shape_image_config::ser_image_config(&mut object_18, var_17)?;
-        object_18.finish();
-    }
-    if let Some(var_19) = &input.kms_key_arn {
-        object.key("KMSKeyArn").string(var_19.as_str());
-    }
-    if let Some(var_20) = &input.layers {
-        let mut array_21 = object.key("Layers").start_array();
-        for item_22 in var_20 {
+    if let Some(var_21) = &input.file_system_configs {
+        let mut array_22 = object.key("FileSystemConfigs").start_array();
+        for item_23 in var_21 {
             {
-                array_21.value().string(item_22.as_str());
+                #[allow(unused_mut)]
+                let mut object_24 = array_22.value().start_object();
+                super::protocol_serde::shape_file_system_config::ser_file_system_config(&mut object_24, item_23)?;
+                object_24.finish();
             }
         }
-        array_21.finish();
+        array_22.finish();
     }
-    if let Some(var_23) = &input.logging_config {
+    if let Some(var_25) = &input.image_config {
         #[allow(unused_mut)]
-        let mut object_24 = object.key("LoggingConfig").start_object();
-        super::protocol_serde::shape_logging_config::ser_logging_config(&mut object_24, var_23)?;
-        object_24.finish();
+        let mut object_26 = object.key("ImageConfig").start_object();
+        super::protocol_serde::shape_image_config::ser_image_config(&mut object_26, var_25)?;
+        object_26.finish();
     }
-    if let Some(var_25) = &input.memory_size {
-        object.key("MemorySize").number(
-            #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_25).into()),
-        );
-    }
-    if let Some(var_26) = &input.revision_id {
-        object.key("RevisionId").string(var_26.as_str());
-    }
-    if let Some(var_27) = &input.role {
-        object.key("Role").string(var_27.as_str());
+    if let Some(var_27) = &input.ephemeral_storage {
+        #[allow(unused_mut)]
+        let mut object_28 = object.key("EphemeralStorage").start_object();
+        super::protocol_serde::shape_ephemeral_storage::ser_ephemeral_storage(&mut object_28, var_27)?;
+        object_28.finish();
     }
-    if let Some(var_28) = &input.runtime {
-        object.key("Runtime").string(var_28.as_str());
-    }
     if let Some(var_29) = &input.snap_start {
         #[allow(unused_mut)]
         let mut object_30 = object.key("SnapStart").start_object();
@@ -96,23 +99,23 @@
         super::protocol_serde::shape_snap_start::ser_snap_start(&mut object_30, var_29)?;
         object_30.finish();
     }
-    if let Some(var_31) = &input.timeout {
-        object.key("Timeout").number(
-            #[allow(clippy::useless_conversion)]
-            ::aws_smithy_types::Number::NegInt((*var_31).into()),
-        );
+    if let Some(var_31) = &input.logging_config {
+        #[allow(unused_mut)]
+        let mut object_32 = object.key("LoggingConfig").start_object();
+        super::protocol_serde::shape_logging_config::ser_logging_config(&mut object_32, var_31)?;
+        object_32.finish();
     }
-    if let Some(var_32) = &input.tracing_config {
+    if let Some(var_33) = &input.capacity_provider_config {
         #[allow(unused_mut)]
-        let mut object_33 = object.key("TracingConfig").start_object();
-        super::protocol_serde::shape_tracing_config::ser_tracing_config(&mut object_33, var_32)?;
-        object_33.finish();
+        let mut object_34 = object.key("CapacityProviderConfig").start_object();
+        super::protocol_serde::shape_capacity_provider_config::ser_capacity_provider_config(&mut object_34, var_33)?;
+        object_34.finish();
     }
-    if let Some(var_34) = &input.vpc_config {
+    if let Some(var_35) = &input.durable_config {
         #[allow(unused_mut)]
-        let mut object_35 = object.key("VpcConfig").start_object();
-        super::protocol_serde::shape_vpc_config::ser_vpc_config(&mut object_35, var_34)?;
-        object_35.finish();
+        let mut object_36 = object.key("DurableConfig").start_object();
+        super::protocol_serde::shape_durable_config::ser_durable_config(&mut object_36, var_35)?;
+        object_36.finish();
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_update_function_event_invoke_config.rs`

```diff
--- reference/src/protocol_serde/shape_update_function_event_invoke_config.rs
+++ generated/src/protocol_serde/shape_update_function_event_invoke_config.rs
@@ -100,13 +100,6 @@
                     output =
                         super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                             .map_err(super::operation::update_function_event_invoke_config::UpdateFunctionEventInvokeConfigError::unhandled)?;
-                    output = output.set_retry_after_seconds(
-                        super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                            super::operation::update_function_event_invoke_config::UpdateFunctionEventInvokeConfigError::unhandled(
-                                "Failed to parse retryAfterSeconds from header `Retry-After",
-                            )
-                        })?,
-                    );
                     let output = output.meta(generic);
                     output.build()
                 };
@@ -165,11 +158,10 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "DestinationConfig" => {
-                    builder = builder.set_destination_config(super::protocol_serde::shape_destination_config::de_destination_config(
-                        tokens,
-                        _value,
-                        depth + 1,
+                "LastModified" => {
+                    builder = builder.set_last_modified(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
+                        tokens.next(),
+                        ::aws_smithy_types::date_time::Format::EpochSeconds,
                     )?);
                 }
                 "FunctionArn" => {
@@ -179,26 +171,27 @@
                             .transpose()?,
                     );
                 }
-                "LastModified" => {
-                    builder = builder.set_last_modified(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
-                        tokens.next(),
-                        ::aws_smithy_types::date_time::Format::EpochSeconds,
-                    )?);
-                }
-                "MaximumEventAgeInSeconds" => {
-                    builder = builder.set_maximum_event_age_in_seconds(
+                "MaximumRetryAttempts" => {
+                    builder = builder.set_maximum_retry_attempts(
                         ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                             .map(i32::try_from)
                             .transpose()?,
                     );
                 }
-                "MaximumRetryAttempts" => {
-                    builder = builder.set_maximum_retry_attempts(
+                "MaximumEventAgeInSeconds" => {
+                    builder = builder.set_maximum_event_age_in_seconds(
                         ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                             .map(i32::try_from)
                             .transpose()?,
                     );
                 }
+                "DestinationConfig" => {
+                    builder = builder.set_destination_config(super::protocol_serde::shape_destination_config::de_destination_config(
+                        tokens,
+                        _value,
+                        depth + 1,
+                    )?);
+                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_update_function_event_invoke_config_input.rs`

```diff
--- reference/src/protocol_serde/shape_update_function_event_invoke_config_input.rs
+++ generated/src/protocol_serde/shape_update_function_event_invoke_config_input.rs
@@ -3,23 +3,29 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::update_function_event_invoke_config::UpdateFunctionEventInvokeConfigInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.destination_config {
-        #[allow(unused_mut)]
-        let mut object_2 = object.key("DestinationConfig").start_object();
-        super::protocol_serde::shape_destination_config::ser_destination_config(&mut object_2, var_1)?;
-        object_2.finish();
+    if let Some(var_1) = &input.function_name {
+        object.key("FunctionName").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.qualifier {
+        object.key("Qualifier").string(var_2.as_str());
     }
-    if let Some(var_3) = &input.maximum_event_age_in_seconds {
-        object.key("MaximumEventAgeInSeconds").number(
+    if let Some(var_3) = &input.maximum_retry_attempts {
+        object.key("MaximumRetryAttempts").number(
             #[allow(clippy::useless_conversion)]
             ::aws_smithy_types::Number::NegInt((*var_3).into()),
         );
     }
-    if let Some(var_4) = &input.maximum_retry_attempts {
-        object.key("MaximumRetryAttempts").number(
+    if let Some(var_4) = &input.maximum_event_age_in_seconds {
+        object.key("MaximumEventAgeInSeconds").number(
             #[allow(clippy::useless_conversion)]
             ::aws_smithy_types::Number::NegInt((*var_4).into()),
         );
     }
+    if let Some(var_5) = &input.destination_config {
+        #[allow(unused_mut)]
+        let mut object_6 = object.key("DestinationConfig").start_object();
+        super::protocol_serde::shape_destination_config::ser_destination_config(&mut object_6, var_5)?;
+        object_6.finish();
+    }
     Ok(())
 }
```

### `src/protocol_serde/shape_update_function_url_config.rs`

```diff
--- reference/src/protocol_serde/shape_update_function_url_config.rs
+++ generated/src/protocol_serde/shape_update_function_url_config.rs
@@ -96,13 +96,6 @@
                 let mut output = super::types::error::builders::TooManyRequestsExceptionBuilder::default();
                 output = super::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(super::operation::update_function_url_config::UpdateFunctionUrlConfigError::unhandled)?;
-                output = output.set_retry_after_seconds(
-                    super::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
-                        super::operation::update_function_url_config::UpdateFunctionUrlConfigError::unhandled(
-                            "Failed to parse retryAfterSeconds from header `Retry-After",
-                        )
-                    })?,
-                );
                 let output = output.meta(generic);
                 output.build()
             };
@@ -162,6 +155,20 @@
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                "FunctionUrl" => {
+                    builder = builder.set_function_url(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
+                }
+                "FunctionArn" => {
+                    builder = builder.set_function_arn(
+                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                            .transpose()?,
+                    );
+                }
                 "AuthType" => {
                     builder = builder.set_auth_type(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
@@ -179,15 +186,8 @@
                             .transpose()?,
                     );
                 }
-                "FunctionArn" => {
-                    builder = builder.set_function_arn(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
-                }
-                "FunctionUrl" => {
-                    builder = builder.set_function_url(
+                "LastModifiedTime" => {
+                    builder = builder.set_last_modified_time(
                         ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
@@ -200,13 +200,6 @@
                             .transpose()?,
                     );
                 }
-                "LastModifiedTime" => {
-                    builder = builder.set_last_modified_time(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
-                }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
```

### `src/protocol_serde/shape_update_function_url_config_input.rs`

```diff
--- reference/src/protocol_serde/shape_update_function_url_config_input.rs
+++ generated/src/protocol_serde/shape_update_function_url_config_input.rs
@@ -3,17 +3,23 @@
     object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::operation::update_function_url_config::UpdateFunctionUrlConfigInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.auth_type {
-        object.key("AuthType").string(var_1.as_str());
+    if let Some(var_1) = &input.function_name {
+        object.key("FunctionName").string(var_1.as_str());
     }
-    if let Some(var_2) = &input.cors {
+    if let Some(var_2) = &input.qualifier {
+        object.key("Qualifier").string(var_2.as_str());
+    }
+    if let Some(var_3) = &input.auth_type {
+        object.key("AuthType").string(var_3.as_str());
+    }
+    if let Some(var_4) = &input.cors {
         #[allow(unused_mut)]
-        let mut object_3 = object.key("Cors").start_object();
-        super::protocol_serde::shape_cors::ser_cors(&mut object_3, var_2)?;
-        object_3.finish();
+        let mut object_5 = object.key("Cors").start_object();
+        super::protocol_serde::shape_cors::ser_cors(&mut object_5, var_4)?;
+        object_5.finish();
     }
-    if let Some(var_4) = &input.invoke_mode {
-        object.key("InvokeMode").string(var_4.as_str());
+    if let Some(var_6) = &input.invoke_mode {
+        object.key("InvokeMode").string(var_6.as_str());
     }
     Ok(())
 }
```

### `src/protocol_serde.rs`

```diff
--- reference/src/protocol_serde.rs
+++ generated/src/protocol_serde.rs
@@ -115,14 +115,8 @@

 pub(crate) mod shape_invoke_async;

-pub(crate) mod shape_invoke_async_input;
-
-pub(crate) mod shape_invoke_input;
-
 pub(crate) mod shape_invoke_with_response_stream;

-pub(crate) mod shape_invoke_with_response_stream_input;
-
 pub(crate) mod shape_list_aliases;

 pub(crate) mod shape_list_capacity_providers;
@@ -179,22 +173,12 @@

 pub(crate) mod shape_send_durable_execution_callback_failure;

-pub fn rest_json_unset_struct_payload() -> ::std::vec::Vec<u8> {
-    b"{}"[..].into()
-}
-
-pub(crate) mod shape_send_durable_execution_callback_failure_input;
-
 pub(crate) mod shape_send_durable_execution_callback_heartbeat;

 pub(crate) mod shape_send_durable_execution_callback_success;

-pub(crate) mod shape_send_durable_execution_callback_success_input;
-
 pub(crate) mod shape_stop_durable_execution;

-pub(crate) mod shape_stop_durable_execution_input;
-
 pub(crate) mod shape_tag_resource;

 pub(crate) mod shape_untag_resource;
@@ -259,6 +243,30 @@

 pub(crate) mod shape_create_function_url_config_input;

+pub(crate) mod shape_delete_alias_input;
+
+pub(crate) mod shape_delete_capacity_provider_input;
+
+pub(crate) mod shape_delete_code_signing_config_input;
+
+pub(crate) mod shape_delete_event_source_mapping_input;
+
+pub(crate) mod shape_delete_function_code_signing_config_input;
+
+pub(crate) mod shape_delete_function_concurrency_input;
+
+pub(crate) mod shape_delete_function_event_invoke_config_input;
+
+pub(crate) mod shape_delete_function_input;
+
+pub(crate) mod shape_delete_function_url_config_input;
+
+pub(crate) mod shape_delete_layer_version_input;
+
+pub(crate) mod shape_delete_provisioned_concurrency_config_input;
+
+pub(crate) mod shape_delete_resource_policy_input;
+
 pub(crate) mod shape_durable_execution_already_started_exception;

 pub(crate) mod shape_ec2_access_denied_exception;
@@ -281,6 +289,50 @@

 pub(crate) mod shape_function_versions_per_capacity_provider_limit_exceeded_exception;

+pub(crate) mod shape_get_alias_input;
+
+pub(crate) mod shape_get_capacity_provider_input;
+
+pub(crate) mod shape_get_code_signing_config_input;
+
+pub(crate) mod shape_get_durable_execution_history_input;
+
+pub(crate) mod shape_get_durable_execution_input;
+
+pub(crate) mod shape_get_durable_execution_state_input;
+
+pub(crate) mod shape_get_event_source_mapping_input;
+
+pub(crate) mod shape_get_function_code_signing_config_input;
+
+pub(crate) mod shape_get_function_concurrency_input;
+
+pub(crate) mod shape_get_function_configuration_input;
+
+pub(crate) mod shape_get_function_event_invoke_config_input;
+
+pub(crate) mod shape_get_function_input;
+
+pub(crate) mod shape_get_function_recursion_config_input;
+
+pub(crate) mod shape_get_function_scaling_config_input;
+
+pub(crate) mod shape_get_function_url_config_input;
+
+pub(crate) mod shape_get_layer_version_by_arn_input;
+
+pub(crate) mod shape_get_layer_version_input;
+
+pub(crate) mod shape_get_layer_version_policy_input;
+
+pub(crate) mod shape_get_policy_input;
+
+pub(crate) mod shape_get_provisioned_concurrency_config_input;
+
+pub(crate) mod shape_get_resource_policy_input;
+
+pub(crate) mod shape_get_runtime_management_config_input;
+
 pub(crate) mod shape_invalid_code_signature_exception;

 pub(crate) mod shape_invalid_parameter_value_exception;
@@ -295,10 +347,12 @@

 pub(crate) mod shape_invalid_zip_file_exception;

-pub(crate) mod shape_invoke_output;
+pub(crate) mod shape_invoke_async_input;

-pub(crate) mod shape_invoke_with_response_stream_output;
+pub(crate) mod shape_invoke_input;

+pub(crate) mod shape_invoke_with_response_stream_input;
+
 pub(crate) mod shape_kms_access_denied_exception;

 pub(crate) mod shape_kms_disabled_exception;
@@ -307,6 +361,36 @@

 pub(crate) mod shape_kms_not_found_exception;

+pub(crate) mod shape_list_aliases_input;
+
+pub(crate) mod shape_list_capacity_providers_input;
+
+pub(crate) mod shape_list_code_signing_configs_input;
+
+pub(crate) mod shape_list_durable_executions_by_function_input;
+
+pub(crate) mod shape_list_event_source_mappings_input;
+
+pub(crate) mod shape_list_function_event_invoke_configs_input;
+
+pub(crate) mod shape_list_function_url_configs_input;
+
+pub(crate) mod shape_list_function_versions_by_capacity_provider_input;
+
+pub(crate) mod shape_list_functions_by_code_signing_config_input;
+
+pub(crate) mod shape_list_functions_input;
+
+pub(crate) mod shape_list_layer_versions_input;
+
+pub(crate) mod shape_list_layers_input;
+
+pub(crate) mod shape_list_provisioned_concurrency_configs_input;
+
+pub(crate) mod shape_list_tags_input;
+
+pub(crate) mod shape_list_versions_by_function_input;
+
 pub(crate) mod shape_mode_not_supported_exception;

 pub(crate) mod shape_no_published_version_exception;
@@ -341,6 +425,10 @@

 pub(crate) mod shape_recursive_invocation_exception;

+pub(crate) mod shape_remove_layer_version_permission_input;
+
+pub(crate) mod shape_remove_permission_input;
+
 pub(crate) mod shape_request_too_large_exception;

 pub(crate) mod shape_resource_conflict_exception;
@@ -357,6 +445,12 @@

 pub(crate) mod shape_s3_files_mount_timeout_exception;

+pub(crate) mod shape_send_durable_execution_callback_failure_input;
+
+pub(crate) mod shape_send_durable_execution_callback_heartbeat_input;
+
+pub(crate) mod shape_send_durable_execution_callback_success_input;
+
 pub(crate) mod shape_serialized_request_entity_too_large_exception;

 pub(crate) mod shape_service_exception;
@@ -371,6 +465,8 @@

 pub(crate) mod shape_snap_start_timeout_exception;

+pub(crate) mod shape_stop_durable_execution_input;
+
 pub(crate) mod shape_subnet_ip_address_limit_reached_exception;

 pub(crate) mod shape_tag_resource_input;
@@ -379,6 +475,8 @@

 pub(crate) mod shape_unsupported_media_type_exception;

+pub(crate) mod shape_untag_resource_input;
+
 pub(crate) mod shape_update_alias_input;

 pub(crate) mod shape_update_capacity_provider_input;
@@ -395,12 +493,6 @@

 pub(crate) mod shape_update_function_url_config_input;

-pub fn parse_event_stream_error_metadata(
-    payload: &::bytes::Bytes,
-) -> ::std::result::Result<::aws_smithy_types::error::metadata::Builder, ::aws_smithy_json::deserialize::error::DeserializeError> {
-    super::json_errors::parse_error_metadata(payload, &::aws_smithy_runtime_api::http::Headers::new())
-}
-
 pub(crate) mod shape_account_limit;

 pub(crate) mod shape_account_usage;
@@ -505,6 +597,8 @@

 pub(crate) mod shape_instance_requirements;

+pub(crate) mod shape_invoke_with_response_stream_response_event;
+
 pub(crate) mod shape_layer_version_content_input;

 pub(crate) mod shape_layer_version_content_output;
@@ -609,6 +703,8 @@

 pub(crate) mod shape_image_config_error;

+pub(crate) mod shape_invoke_response_stream_update;
+
 pub(crate) mod shape_invoke_with_response_stream_complete_event;

 pub(crate) mod shape_kafka_schema_registry_config;
@@ -677,8 +773,6 @@

 pub(crate) mod shape_context_failed_details;

-pub(crate) mod shape_context_started_details;
-
 pub(crate) mod shape_context_succeeded_details;

 pub(crate) mod shape_endpoint_lists;
@@ -713,8 +807,6 @@

 pub(crate) mod shape_step_failed_details;

-pub(crate) mod shape_step_started_details;
-
 pub(crate) mod shape_step_succeeded_details;

 pub(crate) mod shape_string_list;
```

### `src/serde_util.rs`

```diff
--- reference/src/serde_util.rs
+++ generated/src/serde_util.rs
@@ -295,6 +295,27 @@
     builder
 }

+pub(crate) fn capacity_provider_vpc_config_correct_errors(
+    mut builder: super::types::builders::CapacityProviderVpcConfigBuilder,
+) -> super::types::builders::CapacityProviderVpcConfigBuilder {
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
+    mut builder: super::types::builders::CapacityProviderPermissionsConfigBuilder,
+) -> super::types::builders::CapacityProviderPermissionsConfigBuilder {
+    if builder.capacity_provider_operator_role_arn.is_none() {
+        builder.capacity_provider_operator_role_arn = Some(Default::default())
+    }
+    builder
+}
+
 pub(crate) fn code_signing_config_correct_errors(
     mut builder: super::types::builders::CodeSigningConfigBuilder,
 ) -> super::types::builders::CodeSigningConfigBuilder {
@@ -322,6 +343,15 @@
     builder
 }

+pub(crate) fn allowed_publishers_correct_errors(
+    mut builder: super::types::builders::AllowedPublishersBuilder,
+) -> super::types::builders::AllowedPublishersBuilder {
+    if builder.signing_profile_version_arns.is_none() {
+        builder.signing_profile_version_arns = Some(Default::default())
+    }
+    builder
+}
+
 pub(crate) fn capacity_provider_config_correct_errors(
     mut builder: super::types::builders::CapacityProviderConfigBuilder,
 ) -> super::types::builders::CapacityProviderConfigBuilder {
@@ -364,36 +394,6 @@
     builder
 }

-pub(crate) fn capacity_provider_vpc_config_correct_errors(
-    mut builder: super::types::builders::CapacityProviderVpcConfigBuilder,
-) -> super::types::builders::CapacityProviderVpcConfigBuilder {
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
-    mut builder: super::types::builders::CapacityProviderPermissionsConfigBuilder,
-) -> super::types::builders::CapacityProviderPermissionsConfigBuilder {
-    if builder.capacity_provider_operator_role_arn.is_none() {
-        builder.capacity_provider_operator_role_arn = Some(Default::default())
-    }
-    builder
-}
-
-pub(crate) fn allowed_publishers_correct_errors(
-    mut builder: super::types::builders::AllowedPublishersBuilder,
-) -> super::types::builders::AllowedPublishersBuilder {
-    if builder.signing_profile_version_arns.is_none() {
-        builder.signing_profile_version_arns = Some(Default::default())
-    }
-    builder
-}
-
 pub(crate) fn execution_correct_errors(mut builder: super::types::builders::ExecutionBuilder) -> super::types::builders::ExecutionBuilder {
     if builder.durable_execution_arn.is_none() {
         builder.durable_execution_arn = Some(Default::default())
@@ -687,7 +687,7 @@
     if builder.retry_details.is_none() {
         builder.retry_details = {
             let builder = super::types::builders::RetryDetailsBuilder::default();
-            Some(builder.build())
+            builder.build().ok()
         }
     }
     builder
@@ -705,7 +705,7 @@
     if builder.retry_details.is_none() {
         builder.retry_details = {
             let builder = super::types::builders::RetryDetailsBuilder::default();
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
     pub system_log_level: ::std::option::Option<super::types::SystemLogLevel>,
-    /// <p>The name of the Amazon CloudWatch log group the capacity provider sends logs to. By default, Lambda capacity providers send logs to a default log group named <code>/aws/lambda/capacity-provider/&lt;capacity provider name&gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
+    /// <p>The name of the Amazon CloudWatch log group the capacity provider sends logs to. By default, Lambda capacity providers send logs to a default log group named <code>/aws/lambda/capacity-provider/&amp;lt;capacity provider name&amp;gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
     pub log_group: ::std::option::Option<::std::string::String>,
 }
 impl CapacityProviderLoggingConfig {
@@ -14,7 +14,7 @@
     pub fn system_log_level(&self) -> ::std::option::Option<&super::types::SystemLogLevel> {
         self.system_log_level.as_ref()
     }
-    /// <p>The name of the Amazon CloudWatch log group the capacity provider sends logs to. By default, Lambda capacity providers send logs to a default log group named <code>/aws/lambda/capacity-provider/&lt;capacity provider name&gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
+    /// <p>The name of the Amazon CloudWatch log group the capacity provider sends logs to. By default, Lambda capacity providers send logs to a default log group named <code>/aws/lambda/capacity-provider/&amp;lt;capacity provider name&amp;gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
     pub fn log_group(&self) -> ::std::option::Option<&str> {
         self.log_group.as_deref()
     }
@@ -48,17 +48,17 @@
     pub fn get_system_log_level(&self) -> &::std::option::Option<super::types::SystemLogLevel> {
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
         super::types::Event {
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
     pub application_log_level: ::std::option::Option<super::types::ApplicationLogLevel>,
     /// <p>Set this property to filter the system logs for your function that Lambda sends to CloudWatch. Lambda only sends system logs at the selected level of detail and lower, where <code>DEBUG</code> is the highest level and <code>WARN</code> is the lowest.</p>
     pub system_log_level: ::std::option::Option<super::types::SystemLogLevel>,
-    /// <p>The name of the Amazon CloudWatch log group the function sends logs to. By default, Lambda functions send logs to a default log group named <code>/aws/lambda/&lt;function name&gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
+    /// <p>The name of the Amazon CloudWatch log group the function sends logs to. By default, Lambda functions send logs to a default log group named <code>/aws/lambda/&amp;lt;function name&amp;gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
     pub log_group: ::std::option::Option<::std::string::String>,
 }
 impl LoggingConfig {
@@ -26,7 +26,7 @@
     pub fn system_log_level(&self) -> ::std::option::Option<&super::types::SystemLogLevel> {
         self.system_log_level.as_ref()
     }
-    /// <p>The name of the Amazon CloudWatch log group the function sends logs to. By default, Lambda functions send logs to a default log group named <code>/aws/lambda/&lt;function name&gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
+    /// <p>The name of the Amazon CloudWatch log group the function sends logs to. By default, Lambda functions send logs to a default log group named <code>/aws/lambda/&amp;lt;function name&amp;gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
     pub fn log_group(&self) -> ::std::option::Option<&str> {
         self.log_group.as_deref()
     }
@@ -90,17 +90,17 @@
     pub fn get_system_log_level(&self) -> &::std::option::Option<super::types::SystemLogLevel> {
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

 pub use super::types::_error_object::ErrorObjectBuilder;

+pub use super::types::_checkpoint_updated_execution_state::CheckpointUpdatedExecutionStateBuilder;
+
+pub use super::types::_alias_routing_configuration::AliasRoutingConfigurationBuilder;
+
 pub use super::types::_capacity_provider_vpc_config::CapacityProviderVpcConfigBuilder;

 pub use super::types::_capacity_provider_permissions_config::CapacityProviderPermissionsConfigBuilder;
@@ -27,12 +31,6 @@

 pub use super::types::_code_signing_config::CodeSigningConfigBuilder;

-pub use super::types::_trace_header::TraceHeaderBuilder;
-
-pub use super::types::_durable_config::DurableConfigBuilder;
-
-pub use super::types::_checkpoint_updated_execution_state::CheckpointUpdatedExecutionStateBuilder;
-
 pub use super::types::_filter_criteria::FilterCriteriaBuilder;

 pub use super::types::_event_source_mapping_metrics_config::EventSourceMappingMetricsConfigBuilder;
@@ -75,6 +73,8 @@

 pub use super::types::_capacity_provider_config::CapacityProviderConfigBuilder;

+pub use super::types::_durable_config::DurableConfigBuilder;
+
 pub use super::types::_vpc_config_response::VpcConfigResponseBuilder;

 pub use super::types::_environment_response::EnvironmentResponseBuilder;
@@ -89,6 +89,8 @@

 pub use super::types::_cors::CorsBuilder;

+pub use super::types::_trace_header::TraceHeaderBuilder;
+
 pub use super::types::_function_configuration::FunctionConfigurationBuilder;

 pub use super::types::_function_code_location::FunctionCodeLocationBuilder;
@@ -99,8 +101,6 @@

 pub use super::types::_function_scaling_config::FunctionScalingConfigBuilder;

-pub use super::types::_alias_routing_configuration::AliasRoutingConfigurationBuilder;
-
 pub use super::types::_layer_version_content_output::LayerVersionContentOutputBuilder;

 pub use super::types::_layer_version_content_input::LayerVersionContentInputBuilder;
@@ -111,17 +111,9 @@

 pub use super::types::_function_event_invoke_config::FunctionEventInvokeConfigBuilder;

-pub use super::types::_capacity_provider_logging_config::CapacityProviderLoggingConfigBuilder;
-
-pub use super::types::_function_versions_by_capacity_provider_list_item::FunctionVersionsByCapacityProviderListItemBuilder;
-
 pub use super::types::_operation_update::OperationUpdateBuilder;

-pub use super::types::_event::EventBuilder;
-
-pub use super::types::_operation::OperationBuilder;
-
-pub use super::types::_event_source_mapping_configuration::EventSourceMappingConfigurationBuilder;
+pub use super::types::_capacity_provider_logging_config::CapacityProviderLoggingConfigBuilder;

 pub use super::types::_source_access_configuration::SourceAccessConfigurationBuilder;

@@ -139,7 +131,9 @@

 pub use super::types::_runtime_version_error::RuntimeVersionErrorBuilder;

-pub use super::types::_provisioned_concurrency_config_list_item::ProvisionedConcurrencyConfigListItemBuilder;
+pub use super::types::_event::EventBuilder;
+
+pub use super::types::_operation::OperationBuilder;

 pub use super::types::_resolved_s3_object::ResolvedS3ObjectBuilder;

@@ -149,18 +143,22 @@

 pub use super::types::_invoke_with_response_stream_complete_event::InvokeWithResponseStreamCompleteEventBuilder;

+pub use super::types::_alias_configuration::AliasConfigurationBuilder;
+
 pub use super::types::_execution::ExecutionBuilder;

+pub use super::types::_event_source_mapping_configuration::EventSourceMappingConfigurationBuilder;
+
 pub use super::types::_function_url_config::FunctionUrlConfigBuilder;

-pub use super::types::_alias_configuration::AliasConfigurationBuilder;
-
-pub use super::types::_layers_list_item::LayersListItemBuilder;
+pub use super::types::_function_versions_by_capacity_provider_list_item::FunctionVersionsByCapacityProviderListItemBuilder;

 pub use super::types::_layer_versions_list_item::LayerVersionsListItemBuilder;

-pub use super::types::_target_tracking_scaling_policy::TargetTrackingScalingPolicyBuilder;
+pub use super::types::_layers_list_item::LayersListItemBuilder;

+pub use super::types::_provisioned_concurrency_config_list_item::ProvisionedConcurrencyConfigListItemBuilder;
+
 pub use super::types::_context_options::ContextOptionsBuilder;

 pub use super::types::_step_options::StepOptionsBuilder;
@@ -171,6 +169,10 @@

 pub use super::types::_chained_invoke_options::ChainedInvokeOptionsBuilder;

+pub use super::types::_target_tracking_scaling_policy::TargetTrackingScalingPolicyBuilder;
+
+pub use super::types::_filter::FilterBuilder;
+
 pub use super::types::_execution_started_details::ExecutionStartedDetailsBuilder;

 pub use super::types::_execution_succeeded_details::ExecutionSucceededDetailsBuilder;
@@ -231,7 +233,9 @@

 pub use super::types::_chained_invoke_details::ChainedInvokeDetailsBuilder;

-pub use super::types::_filter::FilterBuilder;
+pub use super::types::_kafka_schema_registry_access_config::KafkaSchemaRegistryAccessConfigBuilder;
+
+pub use super::types::_kafka_schema_validation_config::KafkaSchemaValidationConfigBuilder;

 pub use super::types::_event_input::EventInputBuilder;

@@ -240,7 +244,3 @@
 pub use super::types::_event_error::EventErrorBuilder;

 pub use super::types::_retry_details::RetryDetailsBuilder;
-
-pub use super::types::_kafka_schema_registry_access_config::KafkaSchemaRegistryAccessConfigBuilder;
-
-pub use super::types::_kafka_schema_validation_config::KafkaSchemaValidationConfigBuilder;
```

### `src/types/error/builders.rs`

```diff
--- reference/src/types/error/builders.rs
+++ generated/src/types/error/builders.rs
@@ -1,6 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub use super::types::error::_invalid_parameter_value_exception::InvalidParameterValueExceptionBuilder;

+pub use super::types::error::_policy_length_exceeded_exception::PolicyLengthExceededExceptionBuilder;
+
+pub use super::types::error::_precondition_failed_exception::PreconditionFailedExceptionBuilder;
+
 pub use super::types::error::_resource_conflict_exception::ResourceConflictExceptionBuilder;

 pub use super::types::error::_resource_not_found_exception::ResourceNotFoundExceptionBuilder;
@@ -9,14 +13,8 @@

 pub use super::types::error::_too_many_requests_exception::TooManyRequestsExceptionBuilder;

-pub use super::types::error::_precondition_failed_exception::PreconditionFailedExceptionBuilder;
-
-pub use super::types::error::_policy_length_exceeded_exception::PolicyLengthExceededExceptionBuilder;
-
 pub use super::types::error::_public_policy_exception::PublicPolicyExceptionBuilder;

-pub use super::types::error::_callback_timeout_exception::CallbackTimeoutExceptionBuilder;
-
 pub use super::types::error::_kms_access_denied_exception::KmsAccessDeniedExceptionBuilder;

 pub use super::types::error::_kms_disabled_exception::KmsDisabledExceptionBuilder;
@@ -25,10 +23,10 @@

 pub use super::types::error::_kms_not_found_exception::KmsNotFoundExceptionBuilder;

+pub use super::types::error::_alias_limit_exceeded_exception::AliasLimitExceededExceptionBuilder;
+
 pub use super::types::error::_capacity_provider_limit_exceeded_exception::CapacityProviderLimitExceededExceptionBuilder;

-pub use super::types::error::_resource_in_use_exception::ResourceInUseExceptionBuilder;
-
 pub use super::types::error::_code_signing_config_not_found_exception::CodeSigningConfigNotFoundExceptionBuilder;

 pub use super::types::error::_code_storage_exceeded_exception::CodeStorageExceededExceptionBuilder;
@@ -39,6 +37,10 @@

 pub use super::types::error::_invalid_code_signature_exception::InvalidCodeSignatureExceptionBuilder;

+pub use super::types::error::_resource_in_use_exception::ResourceInUseExceptionBuilder;
+
+pub use super::types::error::_provisioned_concurrency_config_not_found_exception::ProvisionedConcurrencyConfigNotFoundExceptionBuilder;
+
 pub use super::types::error::_code_artifact_user_deleted_exception::CodeArtifactUserDeletedExceptionBuilder;

 pub use super::types::error::_code_artifact_user_failed_exception::CodeArtifactUserFailedExceptionBuilder;
@@ -107,6 +109,4 @@

 pub use super::types::error::_unsupported_media_type_exception::UnsupportedMediaTypeExceptionBuilder;

-pub use super::types::error::_alias_limit_exceeded_exception::AliasLimitExceededExceptionBuilder;
-
-pub use super::types::error::_provisioned_concurrency_config_not_found_exception::ProvisionedConcurrencyConfigNotFoundExceptionBuilder;
+pub use super::types::error::_callback_timeout_exception::CallbackTimeoutExceptionBuilder;
```

### `src/types/error.rs`

```diff
--- reference/src/types/error.rs
+++ generated/src/types/error.rs
@@ -1,6 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub use super::types::error::_invalid_parameter_value_exception::InvalidParameterValueException;

+pub use super::types::error::_policy_length_exceeded_exception::PolicyLengthExceededException;
+
+pub use super::types::error::_precondition_failed_exception::PreconditionFailedException;
+
 pub use super::types::error::_resource_conflict_exception::ResourceConflictException;

 pub use super::types::error::_resource_not_found_exception::ResourceNotFoundException;
@@ -9,14 +13,8 @@

 pub use super::types::error::_too_many_requests_exception::TooManyRequestsException;

-pub use super::types::error::_precondition_failed_exception::PreconditionFailedException;
-
-pub use super::types::error::_policy_length_exceeded_exception::PolicyLengthExceededException;
-
 pub use super::types::error::_public_policy_exception::PublicPolicyException;

-pub use super::types::error::_callback_timeout_exception::CallbackTimeoutException;
-
 pub use super::types::error::_kms_access_denied_exception::KmsAccessDeniedException;

 pub use super::types::error::_kms_disabled_exception::KmsDisabledException;
@@ -25,9 +23,9 @@

 pub use super::types::error::_kms_not_found_exception::KmsNotFoundException;

-pub use super::types::error::_capacity_provider_limit_exceeded_exception::CapacityProviderLimitExceededException;
+pub use super::types::error::_alias_limit_exceeded_exception::AliasLimitExceededException;

-pub use super::types::error::_resource_in_use_exception::ResourceInUseException;
+pub use super::types::error::_capacity_provider_limit_exceeded_exception::CapacityProviderLimitExceededException;

 pub use super::types::error::_code_signing_config_not_found_exception::CodeSigningConfigNotFoundException;

@@ -39,6 +37,10 @@

 pub use super::types::error::_invalid_code_signature_exception::InvalidCodeSignatureException;

+pub use super::types::error::_resource_in_use_exception::ResourceInUseException;
+
+pub use super::types::error::_provisioned_concurrency_config_not_found_exception::ProvisionedConcurrencyConfigNotFoundException;
+
 pub use super::types::error::_code_artifact_user_deleted_exception::CodeArtifactUserDeletedException;

 pub use super::types::error::_code_artifact_user_failed_exception::CodeArtifactUserFailedException;
@@ -107,9 +109,7 @@

 pub use super::types::error::_unsupported_media_type_exception::UnsupportedMediaTypeException;

-pub use super::types::error::_alias_limit_exceeded_exception::AliasLimitExceededException;
-
-pub use super::types::error::_provisioned_concurrency_config_not_found_exception::ProvisionedConcurrencyConfigNotFoundException;
+pub use super::types::error::_callback_timeout_exception::CallbackTimeoutException;

 /// Error type for the `InvokeWithResponseStreamResponseEventError` operation.
 #[non_exhaustive]
@@ -198,6 +198,11 @@
         })
     }
 }
+impl super::s3_request_id::RequestIdExt for super::types::error::InvokeWithResponseStreamResponseEventError {
+    fn extended_request_id(&self) -> Option<&str> {
+        self.meta().extended_request_id()
+    }
+}
 impl ::aws_types::request_id::RequestId for super::types::error::InvokeWithResponseStreamResponseEventError {
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
```

### `src/types.rs`

```diff
--- reference/src/types.rs
+++ generated/src/types.rs
@@ -9,7 +9,11 @@

 pub use super::types::_error_object::ErrorObject;

-pub use super::types::_capacity_provider_state::CapacityProviderState;
+pub use super::types::_function_url_auth_type::FunctionUrlAuthType;
+
+pub use super::types::_checkpoint_updated_execution_state::CheckpointUpdatedExecutionState;
+
+pub use super::types::_alias_routing_configuration::AliasRoutingConfiguration;

 pub use super::types::_capacity_provider_vpc_config::CapacityProviderVpcConfig;

@@ -31,14 +35,6 @@

 pub use super::types::_code_signing_config::CodeSigningConfig;

-pub use super::types::_execution_status::ExecutionStatus;
-
-pub use super::types::_trace_header::TraceHeader;
-
-pub use super::types::_durable_config::DurableConfig;
-
-pub use super::types::_checkpoint_updated_execution_state::CheckpointUpdatedExecutionState;
-
 pub use super::types::_filter_criteria::FilterCriteria;

 pub use super::types::_event_source_mapping_metrics_config::EventSourceMappingMetricsConfig;
@@ -61,8 +57,6 @@

 pub use super::types::_filter_criteria_error::FilterCriteriaError;

-pub use super::types::_function_version::FunctionVersion;
-
 pub use super::types::_runtime::Runtime;

 pub use super::types::_function_code::FunctionCode;
@@ -91,6 +85,8 @@

 pub use super::types::_capacity_provider_config::CapacityProviderConfig;

+pub use super::types::_durable_config::DurableConfig;
+
 pub use super::types::_vpc_config_response::VpcConfigResponse;

 pub use super::types::_environment_response::EnvironmentResponse;
@@ -111,14 +107,14 @@

 pub use super::types::_runtime_version_config::RuntimeVersionConfig;

-pub use super::types::_s3_object_storage_mode::S3ObjectStorageMode;
-
-pub use super::types::_function_url_auth_type::FunctionUrlAuthType;
-
 pub use super::types::_cors::Cors;

 pub use super::types::_invoke_mode::InvokeMode;

+pub use super::types::_execution_status::ExecutionStatus;
+
+pub use super::types::_trace_header::TraceHeader;
+
 pub use super::types::_function_configuration::FunctionConfiguration;

 pub use super::types::_function_code_location::FunctionCodeLocation;
@@ -131,6 +127,10 @@

 pub use super::types::_function_scaling_config::FunctionScalingConfig;

+pub use super::types::_layer_version_content_output::LayerVersionContentOutput;
+
+pub use super::types::_provisioned_concurrency_status_enum::ProvisionedConcurrencyStatusEnum;
+
 pub use super::types::_update_runtime_on::UpdateRuntimeOn;

 pub use super::types::_invocation_type::InvocationType;
@@ -141,15 +141,15 @@

 pub use super::types::_invoke_with_response_stream_response_event::InvokeWithResponseStreamResponseEvent;

-pub use super::types::_alias_routing_configuration::AliasRoutingConfiguration;
+pub use super::types::_capacity_provider_state::CapacityProviderState;
+
+pub use super::types::_function_version::FunctionVersion;

 pub use super::types::_architecture::Architecture;

-pub use super::types::_layer_version_content_output::LayerVersionContentOutput;
-
 pub use super::types::_layer_version_content_input::LayerVersionContentInput;

-pub use super::types::_provisioned_concurrency_status_enum::ProvisionedConcurrencyStatusEnum;
+pub use super::types::_s3_object_storage_mode::S3ObjectStorageMode;

 pub use super::types::_on_success::OnSuccess;

@@ -157,6 +157,8 @@

 pub use super::types::_function_event_invoke_config::FunctionEventInvokeConfig;

+pub use super::types::_operation_update::OperationUpdate;
+
 pub use super::types::_capacity_provider_scaling_mode::CapacityProviderScalingMode;

 pub use super::types::_propagate_tags_mode::PropagateTagsMode;
@@ -163,18 +165,8 @@

 pub use super::types::_capacity_provider_logging_config::CapacityProviderLoggingConfig;

-pub use super::types::_function_versions_by_capacity_provider_list_item::FunctionVersionsByCapacityProviderListItem;
-
 pub use super::types::_code_signing_policy::CodeSigningPolicy;

-pub use super::types::_operation_update::OperationUpdate;
-
-pub use super::types::_event::Event;
-
-pub use super::types::_operation::Operation;
-
-pub use super::types::_event_source_mapping_configuration::EventSourceMappingConfiguration;
-
 pub use super::types::_event_source_mapping_system_log_level::EventSourceMappingSystemLogLevel;

 pub use super::types::_source_access_configuration::SourceAccessConfiguration;
@@ -211,7 +203,9 @@

 pub use super::types::_runtime_version_error::RuntimeVersionError;

-pub use super::types::_provisioned_concurrency_config_list_item::ProvisionedConcurrencyConfigListItem;
+pub use super::types::_event::Event;
+
+pub use super::types::_operation::Operation;

 pub use super::types::_resolved_s3_object::ResolvedS3Object;

@@ -221,17 +215,21 @@

 pub use super::types::_invoke_with_response_stream_complete_event::InvokeWithResponseStreamCompleteEvent;

+pub use super::types::_alias_configuration::AliasConfiguration;
+
 pub use super::types::_execution::Execution;

+pub use super::types::_event_source_mapping_configuration::EventSourceMappingConfiguration;
+
 pub use super::types::_function_url_config::FunctionUrlConfig;

-pub use super::types::_alias_configuration::AliasConfiguration;
+pub use super::types::_function_versions_by_capacity_provider_list_item::FunctionVersionsByCapacityProviderListItem;
+
+pub use super::types::_layer_versions_list_item::LayerVersionsListItem;

 pub use super::types::_layers_list_item::LayersListItem;

-pub use super::types::_layer_versions_list_item::LayerVersionsListItem;
-
-pub use super::types::_target_tracking_scaling_policy::TargetTrackingScalingPolicy;
+pub use super::types::_provisioned_concurrency_config_list_item::ProvisionedConcurrencyConfigListItem;

 pub use super::types::_operation_type::OperationType;

@@ -247,6 +245,18 @@

 pub use super::types::_chained_invoke_options::ChainedInvokeOptions;

+pub use super::types::_target_tracking_scaling_policy::TargetTrackingScalingPolicy;
+
+pub use super::types::_filter::Filter;
+
+pub use super::types::_event_source_mapping_metric::EventSourceMappingMetric;
+
+pub use super::types::_source_access_type::SourceAccessType;
+
+pub use super::types::_end_point_type::EndPointType;
+
+pub use super::types::_schema_registry_event_record_format::SchemaRegistryEventRecordFormat;
+
 pub use super::types::_event_type::EventType;

 pub use super::types::_execution_started_details::ExecutionStartedDetails;
@@ -311,17 +321,11 @@

 pub use super::types::_chained_invoke_details::ChainedInvokeDetails;

-pub use super::types::_filter::Filter;
-
-pub use super::types::_event_source_mapping_metric::EventSourceMappingMetric;
-
-pub use super::types::_source_access_type::SourceAccessType;
+pub use super::types::_capacity_provider_predefined_metric_type::CapacityProviderPredefinedMetricType;

-pub use super::types::_end_point_type::EndPointType;
+pub use super::types::_kafka_schema_registry_access_config::KafkaSchemaRegistryAccessConfig;

-pub use super::types::_schema_registry_event_record_format::SchemaRegistryEventRecordFormat;
-
-pub use super::types::_capacity_provider_predefined_metric_type::CapacityProviderPredefinedMetricType;
+pub use super::types::_kafka_schema_validation_config::KafkaSchemaValidationConfig;

 pub use super::types::_event_input::EventInput;

@@ -331,10 +335,6 @@

 pub use super::types::_retry_details::RetryDetails;

-pub use super::types::_kafka_schema_registry_access_config::KafkaSchemaRegistryAccessConfig;
-
-pub use super::types::_kafka_schema_validation_config::KafkaSchemaValidationConfig;
-
 pub use super::types::_kafka_schema_registry_auth_type::KafkaSchemaRegistryAuthType;

 pub use super::types::_kafka_schema_validation_attribute::KafkaSchemaValidationAttribute;
```

### Missing reference files

- `src/protocol_serde/shape_context_started_details.rs`
- `src/protocol_serde/shape_invoke_output.rs`
- `src/protocol_serde/shape_invoke_with_response_stream_output.rs`
- `src/protocol_serde/shape_step_started_details.rs`

### Unexpected generated files

- `src/protocol_serde/shape_delete_alias_input.rs`
- `src/protocol_serde/shape_delete_capacity_provider_input.rs`
- `src/protocol_serde/shape_delete_code_signing_config_input.rs`
- `src/protocol_serde/shape_delete_event_source_mapping_input.rs`
- `src/protocol_serde/shape_delete_function_code_signing_config_input.rs`
- `src/protocol_serde/shape_delete_function_concurrency_input.rs`
- `src/protocol_serde/shape_delete_function_event_invoke_config_input.rs`
- `src/protocol_serde/shape_delete_function_input.rs`
- `src/protocol_serde/shape_delete_function_url_config_input.rs`
- `src/protocol_serde/shape_delete_layer_version_input.rs`
- `src/protocol_serde/shape_delete_provisioned_concurrency_config_input.rs`
- `src/protocol_serde/shape_delete_resource_policy_input.rs`
- `src/protocol_serde/shape_get_alias_input.rs`
- `src/protocol_serde/shape_get_capacity_provider_input.rs`
- `src/protocol_serde/shape_get_code_signing_config_input.rs`
- `src/protocol_serde/shape_get_durable_execution_history_input.rs`
- `src/protocol_serde/shape_get_durable_execution_input.rs`
- `src/protocol_serde/shape_get_durable_execution_state_input.rs`
- `src/protocol_serde/shape_get_event_source_mapping_input.rs`
- `src/protocol_serde/shape_get_function_code_signing_config_input.rs`
- `src/protocol_serde/shape_get_function_concurrency_input.rs`
- `src/protocol_serde/shape_get_function_configuration_input.rs`
- `src/protocol_serde/shape_get_function_event_invoke_config_input.rs`
- `src/protocol_serde/shape_get_function_input.rs`
- `src/protocol_serde/shape_get_function_recursion_config_input.rs`
- `src/protocol_serde/shape_get_function_scaling_config_input.rs`
- `src/protocol_serde/shape_get_function_url_config_input.rs`
- `src/protocol_serde/shape_get_layer_version_by_arn_input.rs`
- `src/protocol_serde/shape_get_layer_version_input.rs`
- `src/protocol_serde/shape_get_layer_version_policy_input.rs`
- `src/protocol_serde/shape_get_policy_input.rs`
- `src/protocol_serde/shape_get_provisioned_concurrency_config_input.rs`
- `src/protocol_serde/shape_get_resource_policy_input.rs`
- `src/protocol_serde/shape_get_runtime_management_config_input.rs`
- `src/protocol_serde/shape_invoke_response_stream_update.rs`
- `src/protocol_serde/shape_invoke_with_response_stream_response_event.rs`
- `src/protocol_serde/shape_list_aliases_input.rs`
- `src/protocol_serde/shape_list_capacity_providers_input.rs`
- `src/protocol_serde/shape_list_code_signing_configs_input.rs`
- `src/protocol_serde/shape_list_durable_executions_by_function_input.rs`
- `src/protocol_serde/shape_list_event_source_mappings_input.rs`
- `src/protocol_serde/shape_list_function_event_invoke_configs_input.rs`
- `src/protocol_serde/shape_list_function_url_configs_input.rs`
- `src/protocol_serde/shape_list_function_versions_by_capacity_provider_input.rs`
- `src/protocol_serde/shape_list_functions_by_code_signing_config_input.rs`
- `src/protocol_serde/shape_list_functions_input.rs`
- `src/protocol_serde/shape_list_layer_versions_input.rs`
- `src/protocol_serde/shape_list_layers_input.rs`
- `src/protocol_serde/shape_list_provisioned_concurrency_configs_input.rs`
- `src/protocol_serde/shape_list_tags_input.rs`
- `src/protocol_serde/shape_list_versions_by_function_input.rs`
- `src/protocol_serde/shape_remove_layer_version_permission_input.rs`
- `src/protocol_serde/shape_remove_permission_input.rs`
- `src/protocol_serde/shape_send_durable_execution_callback_heartbeat_input.rs`
- `src/protocol_serde/shape_untag_resource_input.rs`

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
- `src/operation/create_function/_create_function_input.rs`
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
- `src/operation/remove_layer_version_permission/_remove_layer_version_permission_input.rs`
- `src/operation/remove_layer_version_permission.rs`
- `src/operation/remove_permission.rs`
- `src/operation/send_durable_execution_callback_failure.rs`
- `src/operation/send_durable_execution_callback_heartbeat.rs`
- `src/operation/send_durable_execution_callback_success.rs`
- `src/operation/stop_durable_execution.rs`
- `src/operation/untag_resource.rs`
- `src/operation/update_function_code/_update_function_code_input.rs`
- `src/protocol_serde/shape_add_layer_version_permission.rs`
- `src/protocol_serde/shape_add_layer_version_permission_input.rs`
- `src/protocol_serde/shape_add_permission.rs`
- `src/protocol_serde/shape_add_permission_input.rs`
- `src/protocol_serde/shape_alias_routing_configuration.rs`
- `src/protocol_serde/shape_amazon_managed_kafka_event_source_config.rs`
- `src/protocol_serde/shape_callback_options.rs`
- `src/protocol_serde/shape_capacity_provider_config.rs`
- `src/protocol_serde/shape_checkpoint_durable_execution.rs`
- `src/protocol_serde/shape_checkpoint_durable_execution_input.rs`
- `src/protocol_serde/shape_cors.rs`
- `src/protocol_serde/shape_create_alias.rs`
- `src/protocol_serde/shape_create_alias_input.rs`
- `src/protocol_serde/shape_create_capacity_provider.rs`
- `src/protocol_serde/shape_create_capacity_provider_input.rs`
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
- `src/protocol_serde/shape_endpoints.rs`
- `src/protocol_serde/shape_ephemeral_storage.rs`
- `src/protocol_serde/shape_error_object.rs`
- `src/protocol_serde/shape_event_source_mapping_configuration.rs`
- `src/protocol_serde/shape_event_source_mapping_logging_config.rs`
- `src/protocol_serde/shape_event_source_mapping_metrics_config.rs`
- `src/protocol_serde/shape_filter_criteria.rs`
- `src/protocol_serde/shape_function_scaling_config.rs`
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
- `src/protocol_serde/shape_invoke.rs`
- `src/protocol_serde/shape_invoke_async.rs`
- `src/protocol_serde/shape_invoke_async_input.rs`
- `src/protocol_serde/shape_invoke_input.rs`
- `src/protocol_serde/shape_invoke_with_response_stream.rs`
- `src/protocol_serde/shape_invoke_with_response_stream_complete_event.rs`
- `src/protocol_serde/shape_invoke_with_response_stream_input.rs`
- `src/protocol_serde/shape_kafka_schema_registry_config.rs`
- `src/protocol_serde/shape_lambda_managed_instances_capacity_provider_config.rs`
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
- `src/protocol_serde/shape_on_failure.rs`
- `src/protocol_serde/shape_on_success.rs`
- `src/protocol_serde/shape_provisioned_poller_config.rs`
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
- `src/protocol_serde/shape_remove_layer_version_permission.rs`
- `src/protocol_serde/shape_remove_permission.rs`
- `src/protocol_serde/shape_scaling_config.rs`
- `src/protocol_serde/shape_self_managed_event_source.rs`
- `src/protocol_serde/shape_self_managed_kafka_event_source_config.rs`
- `src/protocol_serde/shape_send_durable_execution_callback_failure.rs`
- `src/protocol_serde/shape_send_durable_execution_callback_failure_input.rs`
- `src/protocol_serde/shape_send_durable_execution_callback_heartbeat.rs`
- `src/protocol_serde/shape_send_durable_execution_callback_success.rs`
- `src/protocol_serde/shape_send_durable_execution_callback_success_input.rs`
- `src/protocol_serde/shape_stop_durable_execution.rs`
- `src/protocol_serde/shape_stop_durable_execution_input.rs`
- `src/protocol_serde/shape_tag_resource.rs`
- `src/protocol_serde/shape_tag_resource_input.rs`
- `src/protocol_serde/shape_tenancy_config.rs`
- `src/protocol_serde/shape_too_many_requests_exception.rs`
- `src/protocol_serde/shape_untag_resource.rs`
- `src/protocol_serde/shape_update_alias.rs`
- `src/protocol_serde/shape_update_alias_input.rs`
- `src/protocol_serde/shape_update_capacity_provider.rs`
- `src/protocol_serde/shape_update_capacity_provider_input.rs`
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
- `src/protocol_serde.rs`
- `src/serde_util.rs`
- `src/types/_capacity_provider_logging_config.rs`
- `src/types/_event.rs`
- `src/types/_lambda_managed_instances_capacity_provider_config.rs`
- `src/types/_logging_config.rs`
- `src/types/_operation_update.rs`
- `src/types/_propagate_tags_mode.rs`
- `src/types/_s3_object_storage_mode.rs`
- `src/types/builders.rs`
- `src/types/error/builders.rs`
- `src/types/error.rs`
- `src/types.rs`
