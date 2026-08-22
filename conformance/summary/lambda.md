# AWS SDK Conformance Report: lambda

Snapshot: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`

## lambda
**Progress:** `1084/1084` files compared · `499` matched · `218` mismatches · `367` missing · `0` extra · `46.03%` match (100.00% means fully matched)

### `src/client/create_function.rs`

```diff
--- reference/src/client/create_function.rs
+++ generated/src/client/create_function.rs
@@ -37,7 +37,7 @@
     ///   - [`runtime(Option<Runtime>)`](crate::operation::create_function::CreateFunctionOutput::runtime): <p>The identifier of the function's <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html"> runtime</a>. Runtime is required if the deployment package is a .zip file archive. Specifying a runtime results in an error if you're deploying a function using a container image.</p> <p>The following list includes deprecated runtimes. Lambda blocks creating new functions and updating existing functions shortly after each runtime is deprecated. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html#runtime-deprecation-levels">Runtime use after deprecation</a>.</p> <p>For a list of all currently supported runtimes, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html#runtimes-supported">Supported runtimes</a>.</p>
     ///   - [`role(Option<String>)`](crate::operation::create_function::CreateFunctionOutput::role): <p>The function's execution role.</p>
     ///   - [`handler(Option<String>)`](crate::operation::create_function::CreateFunctionOutput::handler): <p>The function that Lambda calls to begin running your function.</p>
-    ///   - [`code_size(i64)`](crate::operation::create_function::CreateFunctionOutput::code_size): <p>The size of the function's deployment package, in bytes.</p>
+    ///   - [`code_size(Option<i64>)`](crate::operation::create_function::CreateFunctionOutput::code_size): <p>The size of the function's deployment package, in bytes.</p>
     ///   - [`description(Option<String>)`](crate::operation::create_function::CreateFunctionOutput::description): <p>The function's description.</p>
     ///   - [`timeout(Option<i32>)`](crate::operation::create_function::CreateFunctionOutput::timeout): <p>The amount of time in seconds that Lambda allows a function to run before stopping it.</p>
     ///   - [`memory_size(Option<i32>)`](crate::operation::create_function::CreateFunctionOutput::memory_size): <p>The amount of memory available to the function at runtime.</p>
```

### `src/client/delete_function.rs`

```diff
--- reference/src/client/delete_function.rs
+++ generated/src/client/delete_function.rs
@@ -6,7 +6,7 @@
     ///   - [`function_name(impl Into<String>)`](crate::operation::delete_function::builders::DeleteFunctionFluentBuilder::function_name) / [`set_function_name(Option<String>)`](crate::operation::delete_function::builders::DeleteFunctionFluentBuilder::set_function_name):<br>required: **true**<br><p>The name or ARN of the Lambda function or version.</p> <p class="title"><b>Name formats</b></p> <ul>  <li>   <p><b>Function name</b> – <code>my-function</code> (name-only), <code>my-function:1</code> (with version).</p></li>  <li>   <p><b>Function ARN</b> – <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p></li>  <li>   <p><b>Partial ARN</b> – <code>123456789012:function:my-function</code>.</p></li> </ul> <p>You can append a version number or alias to any of the formats. The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p><br>
     ///   - [`qualifier(impl Into<String>)`](crate::operation::delete_function::builders::DeleteFunctionFluentBuilder::qualifier) / [`set_qualifier(Option<String>)`](crate::operation::delete_function::builders::DeleteFunctionFluentBuilder::set_qualifier):<br>required: **false**<br><p>Specify a version to delete. You can't delete a version that an alias references.</p><br>
     /// - On success, responds with [`DeleteFunctionOutput`](crate::operation::delete_function::DeleteFunctionOutput) with field(s):
-    ///   - [`status_code(i32)`](crate::operation::delete_function::DeleteFunctionOutput::status_code): <p>The HTTP status code returned by the operation.</p>
+    ///   - [`status_code(Option<i32>)`](crate::operation::delete_function::DeleteFunctionOutput::status_code): <p>The HTTP status code returned by the operation.</p>
     /// - On failure, responds with [`SdkError<DeleteFunctionError>`](crate::operation::delete_function::DeleteFunctionError)
     pub fn delete_function(&self) -> crate::operation::delete_function::builders::DeleteFunctionFluentBuilder {
         crate::operation::delete_function::builders::DeleteFunctionFluentBuilder::new(self.handle.clone())
```

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

### `src/client/get_function_configuration.rs`

```diff
--- reference/src/client/get_function_configuration.rs
+++ generated/src/client/get_function_configuration.rs
@@ -11,7 +11,7 @@
     ///   - [`runtime(Option<Runtime>)`](crate::operation::get_function_configuration::GetFunctionConfigurationOutput::runtime): <p>The identifier of the function's <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html"> runtime</a>. Runtime is required if the deployment package is a .zip file archive. Specifying a runtime results in an error if you're deploying a function using a container image.</p> <p>The following list includes deprecated runtimes. Lambda blocks creating new functions and updating existing functions shortly after each runtime is deprecated. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html#runtime-deprecation-levels">Runtime use after deprecation</a>.</p> <p>For a list of all currently supported runtimes, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html#runtimes-supported">Supported runtimes</a>.</p>
     ///   - [`role(Option<String>)`](crate::operation::get_function_configuration::GetFunctionConfigurationOutput::role): <p>The function's execution role.</p>
     ///   - [`handler(Option<String>)`](crate::operation::get_function_configuration::GetFunctionConfigurationOutput::handler): <p>The function that Lambda calls to begin running your function.</p>
-    ///   - [`code_size(i64)`](crate::operation::get_function_configuration::GetFunctionConfigurationOutput::code_size): <p>The size of the function's deployment package, in bytes.</p>
+    ///   - [`code_size(Option<i64>)`](crate::operation::get_function_configuration::GetFunctionConfigurationOutput::code_size): <p>The size of the function's deployment package, in bytes.</p>
     ///   - [`description(Option<String>)`](crate::operation::get_function_configuration::GetFunctionConfigurationOutput::description): <p>The function's description.</p>
     ///   - [`timeout(Option<i32>)`](crate::operation::get_function_configuration::GetFunctionConfigurationOutput::timeout): <p>The amount of time in seconds that Lambda allows a function to run before stopping it.</p>
     ///   - [`memory_size(Option<i32>)`](crate::operation::get_function_configuration::GetFunctionConfigurationOutput::memory_size): <p>The amount of memory available to the function at runtime.</p>
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

### `src/client/get_layer_version.rs`

```diff
--- reference/src/client/get_layer_version.rs
+++ generated/src/client/get_layer_version.rs
@@ -11,7 +11,7 @@
     ///   - [`layer_version_arn(Option<String>)`](crate::operation::get_layer_version::GetLayerVersionOutput::layer_version_arn): <p>The ARN of the layer version.</p>
     ///   - [`description(Option<String>)`](crate::operation::get_layer_version::GetLayerVersionOutput::description): <p>The description of the version.</p>
     ///   - [`created_date(Option<String>)`](crate::operation::get_layer_version::GetLayerVersionOutput::created_date): <p>The date that the layer version was created, in <a href="https://www.w3.org/TR/NOTE-datetime">ISO-8601 format</a> (YYYY-MM-DDThh:mm:ss.sTZD).</p>
-    ///   - [`version(i64)`](crate::operation::get_layer_version::GetLayerVersionOutput::version): <p>The version number.</p>
+    ///   - [`version(Option<i64>)`](crate::operation::get_layer_version::GetLayerVersionOutput::version): <p>The version number.</p>
     ///   - [`compatible_architectures(Option<Vec::<Architecture>>)`](crate::operation::get_layer_version::GetLayerVersionOutput::compatible_architectures): <p>A list of compatible <a href="https://docs.aws.amazon.com/lambda/latest/dg/foundation-arch.html">instruction set architectures</a>.</p>
     ///   - [`compatible_runtimes(Option<Vec::<Runtime>>)`](crate::operation::get_layer_version::GetLayerVersionOutput::compatible_runtimes): <p>The layer's compatible runtimes.</p> <p>The following list includes deprecated runtimes. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html#runtime-deprecation-levels">Runtime use after deprecation</a>.</p> <p>For a list of all currently supported runtimes, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html#runtimes-supported">Supported runtimes</a>.</p>
     ///   - [`license_info(Option<String>)`](crate::operation::get_layer_version::GetLayerVersionOutput::license_info): <p>The layer's software license.</p>
```

### `src/client/get_layer_version_by_arn.rs`

```diff
--- reference/src/client/get_layer_version_by_arn.rs
+++ generated/src/client/get_layer_version_by_arn.rs
@@ -10,7 +10,7 @@
     ///   - [`layer_version_arn(Option<String>)`](crate::operation::get_layer_version_by_arn::GetLayerVersionByArnOutput::layer_version_arn): <p>The ARN of the layer version.</p>
     ///   - [`description(Option<String>)`](crate::operation::get_layer_version_by_arn::GetLayerVersionByArnOutput::description): <p>The description of the version.</p>
     ///   - [`created_date(Option<String>)`](crate::operation::get_layer_version_by_arn::GetLayerVersionByArnOutput::created_date): <p>The date that the layer version was created, in <a href="https://www.w3.org/TR/NOTE-datetime">ISO-8601 format</a> (YYYY-MM-DDThh:mm:ss.sTZD).</p>
-    ///   - [`version(i64)`](crate::operation::get_layer_version_by_arn::GetLayerVersionByArnOutput::version): <p>The version number.</p>
+    ///   - [`version(Option<i64>)`](crate::operation::get_layer_version_by_arn::GetLayerVersionByArnOutput::version): <p>The version number.</p>
     ///   - [`compatible_architectures(Option<Vec::<Architecture>>)`](crate::operation::get_layer_version_by_arn::GetLayerVersionByArnOutput::compatible_architectures): <p>A list of compatible <a href="https://docs.aws.amazon.com/lambda/latest/dg/foundation-arch.html">instruction set architectures</a>.</p>
     ///   - [`compatible_runtimes(Option<Vec::<Runtime>>)`](crate::operation::get_layer_version_by_arn::GetLayerVersionByArnOutput::compatible_runtimes): <p>The layer's compatible runtimes.</p> <p>The following list includes deprecated runtimes. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html#runtime-deprecation-levels">Runtime use after deprecation</a>.</p> <p>For a list of all currently supported runtimes, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html#runtimes-supported">Supported runtimes</a>.</p>
     ///   - [`license_info(Option<String>)`](crate::operation::get_layer_version_by_arn::GetLayerVersionByArnOutput::license_info): <p>The layer's software license.</p>
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

### `src/client/invoke.rs`

```diff
--- reference/src/client/invoke.rs
+++ generated/src/client/invoke.rs
@@ -12,7 +12,7 @@
     ///   - [`qualifier(impl Into<String>)`](crate::operation::invoke::builders::InvokeFluentBuilder::qualifier) / [`set_qualifier(Option<String>)`](crate::operation::invoke::builders::InvokeFluentBuilder::set_qualifier):<br>required: **false**<br><p>Specify a version or alias to invoke a published version of the function.</p><br>
     ///   - [`tenant_id(impl Into<String>)`](crate::operation::invoke::builders::InvokeFluentBuilder::tenant_id) / [`set_tenant_id(Option<String>)`](crate::operation::invoke::builders::InvokeFluentBuilder::set_tenant_id):<br>required: **false**<br><p>The identifier of the tenant in a multi-tenant Lambda function.</p><br>
     /// - On success, responds with [`InvokeOutput`](crate::operation::invoke::InvokeOutput) with field(s):
-    ///   - [`status_code(i32)`](crate::operation::invoke::InvokeOutput::status_code): <p>The HTTP status code is in the 200 range for a successful request. For the <code>RequestResponse</code> invocation type, this status code is 200. For the <code>Event</code> invocation type, this status code is 202. For the <code>DryRun</code> invocation type, the status code is 204.</p>
+    ///   - [`status_code(Option<i32>)`](crate::operation::invoke::InvokeOutput::status_code): <p>The HTTP status code is in the 200 range for a successful request. For the <code>RequestResponse</code> invocation type, this status code is 200. For the <code>Event</code> invocation type, this status code is 202. For the <code>DryRun</code> invocation type, the status code is 204.</p>
     ///   - [`function_error(Option<String>)`](crate::operation::invoke::InvokeOutput::function_error): <p>If present, indicates that an error occurred during function execution. Details about the error are included in the response payload.</p>
     ///   - [`log_result(Option<String>)`](crate::operation::invoke::InvokeOutput::log_result): <p>The last 4 KB of the execution log, which is base64-encoded.</p>
     ///   - [`payload(Option<Blob>)`](crate::operation::invoke::InvokeOutput::payload): <p>The response from the function, or an error object.</p>
```

### `src/client/invoke_async.rs`

```diff
--- reference/src/client/invoke_async.rs
+++ generated/src/client/invoke_async.rs
@@ -4,11 +4,10 @@
     ///
     /// - The fluent builder is configurable:
     ///   - [`function_name(impl Into<String>)`](crate::operation::invoke_async::builders::InvokeAsyncFluentBuilder::function_name) / [`set_function_name(Option<String>)`](crate::operation::invoke_async::builders::InvokeAsyncFluentBuilder::set_function_name):<br>required: **true**<br><p>The name or ARN of the Lambda function.</p> <p class="title"><b>Name formats</b></p> <ul>  <li>   <p><b>Function name</b> – <code>my-function</code>.</p></li>  <li>   <p><b>Function ARN</b> – <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p></li>  <li>   <p><b>Partial ARN</b> – <code>123456789012:function:my-function</code>.</p></li> </ul> <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p><br>
-    ///   - [`invoke_args(ByteStream)`](crate::operation::invoke_async::builders::InvokeAsyncFluentBuilder::invoke_args) / [`set_invoke_args(ByteStream)`](crate::operation::invoke_async::builders::InvokeAsyncFluentBuilder::set_invoke_args):<br>required: **true**<br><p>The JSON that you want to provide to your Lambda function as input.</p><br>
+    ///   - [`invoke_args(Blob)`](crate::operation::invoke_async::builders::InvokeAsyncFluentBuilder::invoke_args) / [`set_invoke_args(Option<Blob>)`](crate::operation::invoke_async::builders::InvokeAsyncFluentBuilder::set_invoke_args):<br>required: **true**<br><p>The JSON that you want to provide to your Lambda function as input.</p><br>
     /// - On success, responds with [`InvokeAsyncOutput`](crate::operation::invoke_async::InvokeAsyncOutput) with field(s):
-    ///   - [`status(i32)`](crate::operation::invoke_async::InvokeAsyncOutput::status): <p>The status code.</p>
+    ///   - [`status(Option<i32>)`](crate::operation::invoke_async::InvokeAsyncOutput::status): <p>The status code.</p>
     /// - On failure, responds with [`SdkError<InvokeAsyncError>`](crate::operation::invoke_async::InvokeAsyncError)
-    #[deprecated]
     pub fn invoke_async(&self) -> crate::operation::invoke_async::builders::InvokeAsyncFluentBuilder {
         crate::operation::invoke_async::builders::InvokeAsyncFluentBuilder::new(self.handle.clone())
     }
```

### `src/client/invoke_with_response_stream.rs`

```diff
--- reference/src/client/invoke_with_response_stream.rs
+++ generated/src/client/invoke_with_response_stream.rs
@@ -11,9 +11,9 @@
     ///   - [`tenant_id(impl Into<String>)`](crate::operation::invoke_with_response_stream::builders::InvokeWithResponseStreamFluentBuilder::tenant_id) / [`set_tenant_id(Option<String>)`](crate::operation::invoke_with_response_stream::builders::InvokeWithResponseStreamFluentBuilder::set_tenant_id):<br>required: **false**<br><p>The identifier of the tenant in a multi-tenant Lambda function.</p><br>
     ///   - [`invocation_type(ResponseStreamingInvocationType)`](crate::operation::invoke_with_response_stream::builders::InvokeWithResponseStreamFluentBuilder::invocation_type) / [`set_invocation_type(Option<ResponseStreamingInvocationType>)`](crate::operation::invoke_with_response_stream::builders::InvokeWithResponseStreamFluentBuilder::set_invocation_type):<br>required: **false**<br><p>Use one of the following options:</p> <ul>  <li>   <p><code>RequestResponse</code> (default) – Invoke the function synchronously. Keep the connection open until the function returns a response or times out. The API operation response includes the function response and additional data.</p></li>  <li>   <p><code>DryRun</code> – Validate parameter values and verify that the IAM user or role has permission to invoke the function.</p></li> </ul><br>
     /// - On success, responds with [`InvokeWithResponseStreamOutput`](crate::operation::invoke_with_response_stream::InvokeWithResponseStreamOutput) with field(s):
-    ///   - [`status_code(i32)`](crate::operation::invoke_with_response_stream::InvokeWithResponseStreamOutput::status_code): <p>For a successful request, the HTTP status code is in the 200 range. For the <code>RequestResponse</code> invocation type, this status code is 200. For the <code>DryRun</code> invocation type, this status code is 204.</p>
+    ///   - [`status_code(Option<i32>)`](crate::operation::invoke_with_response_stream::InvokeWithResponseStreamOutput::status_code): <p>For a successful request, the HTTP status code is in the 200 range. For the <code>RequestResponse</code> invocation type, this status code is 200. For the <code>DryRun</code> invocation type, this status code is 204.</p>
     ///   - [`executed_version(Option<String>)`](crate::operation::invoke_with_response_stream::InvokeWithResponseStreamOutput::executed_version): <p>The version of the function that executed. When you invoke a function with an alias, this indicates which version the alias resolved to.</p>
-    ///   - [`event_stream(EventReceiver<InvokeWithResponseStreamResponseEvent, InvokeWithResponseStreamResponseEventError>)`](crate::operation::invoke_with_response_stream::InvokeWithResponseStreamOutput::event_stream): <p>The stream of response payloads.</p>
+    ///   - [`event_stream(Option<InvokeWithResponseStreamResponseEvent>)`](crate::operation::invoke_with_response_stream::InvokeWithResponseStreamOutput::event_stream): <p>The stream of response payloads.</p>
     ///   - [`response_stream_content_type(Option<String>)`](crate::operation::invoke_with_response_stream::InvokeWithResponseStreamOutput::response_stream_content_type): <p>The type of data the stream is returning.</p>
     /// - On failure, responds with [`SdkError<InvokeWithResponseStreamError>`](crate::operation::invoke_with_response_stream::InvokeWithResponseStreamError)
     pub fn invoke_with_response_stream(&self) -> crate::operation::invoke_with_response_stream::builders::InvokeWithResponseStreamFluentBuilder {
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
@@ -15,7 +15,7 @@
     ///   - [`layer_version_arn(Option<String>)`](crate::operation::publish_layer_version::PublishLayerVersionOutput::layer_version_arn): <p>The ARN of the layer version.</p>
     ///   - [`description(Option<String>)`](crate::operation::publish_layer_version::PublishLayerVersionOutput::description): <p>The description of the version.</p>
     ///   - [`created_date(Option<String>)`](crate::operation::publish_layer_version::PublishLayerVersionOutput::created_date): <p>The date that the layer version was created, in <a href="https://www.w3.org/TR/NOTE-datetime">ISO-8601 format</a> (YYYY-MM-DDThh:mm:ss.sTZD).</p>
-    ///   - [`version(i64)`](crate::operation::publish_layer_version::PublishLayerVersionOutput::version): <p>The version number.</p>
+    ///   - [`version(Option<i64>)`](crate::operation::publish_layer_version::PublishLayerVersionOutput::version): <p>The version number.</p>
     ///   - [`compatible_architectures(Option<Vec::<Architecture>>)`](crate::operation::publish_layer_version::PublishLayerVersionOutput::compatible_architectures): <p>A list of compatible <a href="https://docs.aws.amazon.com/lambda/latest/dg/foundation-arch.html">instruction set architectures</a>.</p>
     ///   - [`compatible_runtimes(Option<Vec::<Runtime>>)`](crate::operation::publish_layer_version::PublishLayerVersionOutput::compatible_runtimes): <p>The layer's compatible runtimes.</p> <p>The following list includes deprecated runtimes. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html#runtime-deprecation-levels">Runtime use after deprecation</a>.</p> <p>For a list of all currently supported runtimes, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html#runtimes-supported">Supported runtimes</a>.</p>
     ///   - [`license_info(Option<String>)`](crate::operation::publish_layer_version::PublishLayerVersionOutput::license_info): <p>The layer's software license.</p>
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
@@ -14,7 +14,7 @@
     ///   - [`runtime(Option<Runtime>)`](crate::operation::publish_version::PublishVersionOutput::runtime): <p>The identifier of the function's <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html"> runtime</a>. Runtime is required if the deployment package is a .zip file archive. Specifying a runtime results in an error if you're deploying a function using a container image.</p> <p>The following list includes deprecated runtimes. Lambda blocks creating new functions and updating existing functions shortly after each runtime is deprecated. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html#runtime-deprecation-levels">Runtime use after deprecation</a>.</p> <p>For a list of all currently supported runtimes, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html#runtimes-supported">Supported runtimes</a>.</p>
     ///   - [`role(Option<String>)`](crate::operation::publish_version::PublishVersionOutput::role): <p>The function's execution role.</p>
     ///   - [`handler(Option<String>)`](crate::operation::publish_version::PublishVersionOutput::handler): <p>The function that Lambda calls to begin running your function.</p>
-    ///   - [`code_size(i64)`](crate::operation::publish_version::PublishVersionOutput::code_size): <p>The size of the function's deployment package, in bytes.</p>
+    ///   - [`code_size(Option<i64>)`](crate::operation::publish_version::PublishVersionOutput::code_size): <p>The size of the function's deployment package, in bytes.</p>
     ///   - [`description(Option<String>)`](crate::operation::publish_version::PublishVersionOutput::description): <p>The function's description.</p>
     ///   - [`timeout(Option<i32>)`](crate::operation::publish_version::PublishVersionOutput::timeout): <p>The amount of time in seconds that Lambda allows a function to run before stopping it.</p>
     ///   - [`memory_size(Option<i32>)`](crate::operation::publish_version::PublishVersionOutput::memory_size): <p>The amount of memory available to the function at runtime.</p>
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
@@ -22,7 +22,7 @@
     ///   - [`runtime(Option<Runtime>)`](crate::operation::update_function_code::UpdateFunctionCodeOutput::runtime): <p>The identifier of the function's <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html"> runtime</a>. Runtime is required if the deployment package is a .zip file archive. Specifying a runtime results in an error if you're deploying a function using a container image.</p> <p>The following list includes deprecated runtimes. Lambda blocks creating new functions and updating existing functions shortly after each runtime is deprecated. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html#runtime-deprecation-levels">Runtime use after deprecation</a>.</p> <p>For a list of all currently supported runtimes, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html#runtimes-supported">Supported runtimes</a>.</p>
     ///   - [`role(Option<String>)`](crate::operation::update_function_code::UpdateFunctionCodeOutput::role): <p>The function's execution role.</p>
     ///   - [`handler(Option<String>)`](crate::operation::update_function_code::UpdateFunctionCodeOutput::handler): <p>The function that Lambda calls to begin running your function.</p>
-    ///   - [`code_size(i64)`](crate::operation::update_function_code::UpdateFunctionCodeOutput::code_size): <p>The size of the function's deployment package, in bytes.</p>
+    ///   - [`code_size(Option<i64>)`](crate::operation::update_function_code::UpdateFunctionCodeOutput::code_size): <p>The size of the function's deployment package, in bytes.</p>
     ///   - [`description(Option<String>)`](crate::operation::update_function_code::UpdateFunctionCodeOutput::description): <p>The function's description.</p>
     ///   - [`timeout(Option<i32>)`](crate::operation::update_function_code::UpdateFunctionCodeOutput::timeout): <p>The amount of time in seconds that Lambda allows a function to run before stopping it.</p>
     ///   - [`memory_size(Option<i32>)`](crate::operation::update_function_code::UpdateFunctionCodeOutput::memory_size): <p>The amount of memory available to the function at runtime.</p>
```

### `src/client/update_function_configuration.rs`

```diff
--- reference/src/client/update_function_configuration.rs
+++ generated/src/client/update_function_configuration.rs
@@ -30,7 +30,7 @@
     ///   - [`runtime(Option<Runtime>)`](crate::operation::update_function_configuration::UpdateFunctionConfigurationOutput::runtime): <p>The identifier of the function's <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html"> runtime</a>. Runtime is required if the deployment package is a .zip file archive. Specifying a runtime results in an error if you're deploying a function using a container image.</p> <p>The following list includes deprecated runtimes. Lambda blocks creating new functions and updating existing functions shortly after each runtime is deprecated. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html#runtime-deprecation-levels">Runtime use after deprecation</a>.</p> <p>For a list of all currently supported runtimes, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html#runtimes-supported">Supported runtimes</a>.</p>
     ///   - [`role(Option<String>)`](crate::operation::update_function_configuration::UpdateFunctionConfigurationOutput::role): <p>The function's execution role.</p>
     ///   - [`handler(Option<String>)`](crate::operation::update_function_configuration::UpdateFunctionConfigurationOutput::handler): <p>The function that Lambda calls to begin running your function.</p>
-    ///   - [`code_size(i64)`](crate::operation::update_function_configuration::UpdateFunctionConfigurationOutput::code_size): <p>The size of the function's deployment package, in bytes.</p>
+    ///   - [`code_size(Option<i64>)`](crate::operation::update_function_configuration::UpdateFunctionConfigurationOutput::code_size): <p>The size of the function's deployment package, in bytes.</p>
     ///   - [`description(Option<String>)`](crate::operation::update_function_configuration::UpdateFunctionConfigurationOutput::description): <p>The function's description.</p>
     ///   - [`timeout(Option<i32>)`](crate::operation::update_function_configuration::UpdateFunctionConfigurationOutput::timeout): <p>The amount of time in seconds that Lambda allows a function to run before stopping it.</p>
     ///   - [`memory_size(Option<i32>)`](crate::operation::update_function_configuration::UpdateFunctionConfigurationOutput::memory_size): <p>The amount of memory available to the function at runtime.</p>
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

### `src/client.rs`

```diff
--- reference/src/client.rs
+++ generated/src/client.rs
@@ -1,394 +1,379 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-#[derive(Debug)]
-pub(crate) struct Handle {
-    pub(crate) conf: crate::Config,
-    #[allow(dead_code)] // unused when a service does not provide any operations
-    pub(crate) runtime_plugins: ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
-}

-/// Client for AWS Lambda
-///
-/// Client for invoking operations on AWS Lambda. Each operation on AWS Lambda is a method on this
-/// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
-/// ## Constructing a `Client`
-///
-/// A [`Config`] is required to construct a client. For most use cases, the [`aws-config`]
-/// crate should be used to automatically resolve this config using
-/// [`aws_config::load_from_env()`], since this will resolve an [`SdkConfig`] which can be shared
-/// across multiple different AWS SDK clients. This config resolution process can be customized
-/// by calling [`aws_config::from_env()`] instead, which returns a [`ConfigLoader`] that uses
-/// the [builder pattern] to customize the default config.
-///
-/// In the simplest case, creating a client looks as follows:
-/// ```rust,no_run
-/// # async fn wrapper() {
-/// let config = aws_config::load_from_env().await;
-/// let client = aws_sdk_lambda::Client::new(&config);
-/// # }
-/// ```
-///
-/// Occasionally, SDKs may have additional service-specific values that can be set on the [`Config`] that
-/// is absent from [`SdkConfig`], or slightly different settings for a specific client may be desired.
-/// The [`Builder`](crate::config::Builder) struct implements `From<&SdkConfig>`, so setting these specific settings can be
-/// done as follows:
-///
-/// ```rust,no_run
-/// # async fn wrapper() {
-/// let sdk_config = ::aws_config::load_from_env().await;
-/// let config = aws_sdk_lambda::config::Builder::from(&sdk_config)
-/// # /*
-///     .some_service_specific_setting("value")
-/// # */
-///     .build();
-/// # }
-/// ```
-///
-/// See the [`aws-config` docs] and [`Config`] for more information on customizing configuration.
-///
-/// _Note:_ Client construction is expensive due to connection thread pool initialization, and should
-/// be done once at application start-up.
-///
-/// [`Config`]: crate::Config
-/// [`ConfigLoader`]: https://docs.rs/aws-config/*/aws_config/struct.ConfigLoader.html
-/// [`SdkConfig`]: https://docs.rs/aws-config/*/aws_config/struct.SdkConfig.html
-/// [`aws-config` docs]: https://docs.rs/aws-config/*
-/// [`aws-config`]: https://crates.io/crates/aws-config
-/// [`aws_config::from_env()`]: https://docs.rs/aws-config/*/aws_config/fn.from_env.html
-/// [`aws_config::load_from_env()`]: https://docs.rs/aws-config/*/aws_config/fn.load_from_env.html
-/// [builder pattern]: https://rust-lang.github.io/api-guidelines/type-safety.html#builders-enable-construction-of-complex-values-c-builder
-/// # Using the `Client`
-///
-/// A client has a function for every operation that can be performed by the service.
-/// For example, the [`AddLayerVersionPermission`](crate::operation::add_layer_version_permission) operation has
-/// a [`Client::add_layer_version_permission`], function which returns a builder for that operation.
-/// The fluent builder ultimately has a `send()` function that returns an async future that
-/// returns a result, as illustrated below:
-///
-/// ```rust,ignore
-/// let result = client.add_layer_version_permission()
-///     .layer_name("example")
-///     .send()
-///     .await;
-/// ```
-///
-/// The underlying HTTP requests that get made by this can be modified with the `customize_operation`
-/// function on the fluent builder. See the [`customize`](crate::client::customize) module for more
-/// information.
-/// # Waiters
-///
-/// This client provides `wait_until` methods behind the [`Waiters`](crate::client::Waiters) trait.
-/// To use them, simply import the trait, and then call one of the `wait_until` methods. This will
-/// return a waiter fluent builder that takes various parameters, which are documented on the builder
-/// type. Once parameters have been provided, the `wait` method can be called to initiate waiting.
-///
-/// For example, if there was a `wait_until_thing` method, it could look like:
-/// ```rust,ignore
-/// let result = client.wait_until_thing()
-///     .thing_id("someId")
-///     .wait(Duration::from_secs(120))
-///     .await;
-/// ```
-#[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct Client {
-    handle: ::std::sync::Arc<Handle>,
-}
+#[allow(dead_code)]
+pub(crate) mod transport {
+    use ::std::collections::BTreeMap;
+    use ::std::fmt;
+    use ::std::io::{Read, Write};
+    use ::std::net::TcpStream;
+
+    #[derive(Clone, Copy, Debug)]
+    pub(crate) enum Method {
+        Get,
+        Put,
+        Post,
+        Delete,
+        Head,
+        Patch,
+    }
+
+    impl Method {
+        fn as_str(self) -> &'static str {
+            match self {
+                Self::Get => "GET",
+                Self::Put => "PUT",
+                Self::Post => "POST",
+                Self::Delete => "DELETE",
+                Self::Head => "HEAD",
+                Self::Patch => "PATCH",
+            }
+        }
+    }
+
+    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
+    pub(crate) struct StatusCode(u16);
+
+    impl StatusCode {
+        pub(crate) const CONFLICT: Self = Self(409);
+        pub(crate) fn is_success(self) -> bool {
+            (200..300).contains(&self.0)
+        }
+    }
+
+    impl fmt::Display for StatusCode {
+        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
+            self.0.fmt(formatter)
+        }
+    }
+
+    #[derive(Clone, Debug)]
+    pub(crate) struct Response {
+        status: StatusCode,
+        headers: BTreeMap<String, String>,
+        body: Vec<u8>,
+    }
+
+    impl Response {
+        pub(crate) fn status(&self) -> StatusCode {
+            self.status
+        }
+        pub(crate) fn header(&self, name: &str) -> Option<&str> {
+            self.headers.get(&name.to_ascii_lowercase()).map(String::as_str)
+        }
+        pub(crate) fn body(&self) -> &[u8] {
+            &self.body
+        }
+        pub(crate) async fn text(&self) -> Result<String, String> {
+            String::from_utf8(self.body.clone()).map_err(|error| error.to_string())
+        }
+    }
+
+    #[derive(Clone, Debug, Default)]
+    pub(crate) struct HttpClient;

-impl Client {
-    /// Creates a new client from the service [`Config`](crate::Config).
-    ///
-    /// # Panics
-    ///
-    /// This method will panic in the following cases:
-    ///
-    /// - Retries or timeouts are enabled without a `sleep_impl` configured.
-    /// - Identity caching is enabled without a `sleep_impl` and `time_source` configured.
-    /// - No `behavior_version` is provided.
-    ///
-    /// The panic message for each of these will have instructions on how to resolve them.
-    #[track_caller]
-    pub fn from_conf(conf: crate::Config) -> Self {
-        let handle = Handle {
-            conf: conf.clone(),
-            runtime_plugins: crate::config::base_client_runtime_plugins(conf),
-        };
-        if let Err(err) = Self::validate_config(&handle) {
-            panic!("Invalid client configuration: {err}");
+    impl HttpClient {
+        pub(crate) fn new() -> Self {
+            Self
         }
-        Self {
-            handle: ::std::sync::Arc::new(handle),
+        pub(crate) async fn request(&self, method: Method, url: &str, headers: &[(&str, &str)], body: &[u8]) -> Result<Response, String> {
+            let (host, port, path) = parse_http_url(url)?;
+            let mut stream = TcpStream::connect((host.as_str(), port)).map_err(|error| format!("failed to connect to {host}:{port}: {error}"))?;
+            let mut request = format!(
+                "{} {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\nContent-Length: {}\r\n",
+                method.as_str(),
+                path,
+                host,
+                body.len()
+            );
+            for (name, value) in headers {
+                request.push_str(name);
+                request.push_str(": ");
+                request.push_str(value);
+                request.push_str("\r\n");
+            }
+            request.push_str("\r\n");
+            let mut request_bytes = request.into_bytes();
+            request_bytes.extend_from_slice(body);
+            stream
+                .write_all(&request_bytes)
+                .map_err(|error| format!("failed to write HTTP request: {error}"))?;
+            let mut bytes = Vec::new();
+            stream
+                .read_to_end(&mut bytes)
+                .map_err(|error| format!("failed to read HTTP response: {error}"))?;
+            parse_response(&bytes)
         }
     }

-    /// Returns the client's configuration.
-    pub fn config(&self) -> &crate::Config {
-        &self.handle.conf
+    fn parse_http_url(url: &str) -> Result<(String, u16, String), String> {
+        let authority_and_path = url
+            .strip_prefix("http://")
+            .ok_or_else(|| format!("only http:// endpoints are supported: {url}"))?;
+        let (authority, path) = authority_and_path
+            .split_once('/')
+            .map_or((authority_and_path, "/"), |(authority, _path)| {
+                (authority, &authority_and_path[authority.len()..])
+            });
+        if authority.is_empty() {
+            return Err(format!("endpoint has no host: {url}"));
+        }
+        let (host, port) = if let Some((host, port)) = authority.rsplit_once(':') {
+            let port = port.parse::<u16>().map_err(|error| format!("invalid endpoint port in {url}: {error}"))?;
+            (host.to_owned(), port)
+        } else {
+            (authority.to_owned(), 80)
+        };
+        Ok((host, port, path.to_owned()))
     }

-    fn validate_config(handle: &Handle) -> ::std::result::Result<(), ::aws_smithy_runtime_api::box_error::BoxError> {
-        let mut cfg = ::aws_smithy_types::config_bag::ConfigBag::base();
-        handle
-            .runtime_plugins
-            .apply_client_configuration(&mut cfg)?
-            .validate_base_client_config(&cfg)?;
-        Ok(())
+    fn parse_response(bytes: &[u8]) -> Result<Response, String> {
+        let header_end = bytes
+            .windows(4)
+            .position(|window| window == b"\r\n\r\n")
+            .ok_or_else(|| "HTTP response did not contain a header terminator".to_owned())?;
+        let header = ::std::str::from_utf8(&bytes[..header_end]).map_err(|error| format!("HTTP response headers were not UTF-8: {error}"))?;
+        let status = header
+            .lines()
+            .next()
+            .and_then(|line| line.split_whitespace().nth(1))
+            .ok_or_else(|| "HTTP response did not contain a status code".to_owned())?
+            .parse::<u16>()
+            .map_err(|error| format!("HTTP response status was invalid: {error}"))?;
+        let mut headers = BTreeMap::new();
+        for line in header.lines().skip(1) {
+            if let Some((name, value)) = line.split_once(':') {
+                headers.insert(name.trim().to_ascii_lowercase(), value.trim().to_owned());
+            }
+        }
+        Ok(Response {
+            status: StatusCode(status),
+            headers,
+            body: bytes[header_end + 4..].to_vec(),
+        })
     }
-}

-///
-/// Waiter functions for the client.
-///
-/// Import this trait to get `wait_until` methods on the client.
-///
-pub trait Waiters {
-    /// Waits for the function's State to be Active. This waiter uses GetFunction API. This should be used after new function creation.
-    fn wait_until_function_active_v2(&self) -> crate::waiters::function_active_v2::FunctionActiveV2FluentBuilder;
-    /// Wait for `function_exists`
-    fn wait_until_function_exists(&self) -> crate::waiters::function_exists::FunctionExistsFluentBuilder;
-    /// Waits for the function's LastUpdateStatus to be Successful. This waiter uses GetFunction API. This should be used after function updates.
-    fn wait_until_function_updated_v2(&self) -> crate::waiters::function_updated_v2::FunctionUpdatedV2FluentBuilder;
-    /// Waits for the function's State to be Active. This waiter uses GetFunctionConfiguration API. This should be used after new function creation.
-    fn wait_until_function_active(&self) -> crate::waiters::function_active::FunctionActiveFluentBuilder;
-    /// Waits for the function's LastUpdateStatus to be Successful. This waiter uses GetFunctionConfiguration API. This should be used after function updates.
-    fn wait_until_function_updated(&self) -> crate::waiters::function_updated::FunctionUpdatedFluentBuilder;
-    /// Waits for the published version's State to be Active. This waiter uses GetFunctionConfiguration API. This should be used after new version is published.
-    fn wait_until_published_version_active(&self) -> crate::waiters::published_version_active::PublishedVersionActiveFluentBuilder;
-}
-impl Waiters for Client {
-    fn wait_until_function_active_v2(&self) -> crate::waiters::function_active_v2::FunctionActiveV2FluentBuilder {
-        crate::waiters::function_active_v2::FunctionActiveV2FluentBuilder::new(self.handle.clone())
+    pub(crate) fn encode_path(value: &str) -> String {
+        value.bytes().fold(String::new(), |mut result, byte| {
+            if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~' | b'/') {
+                result.push(byte as char);
+            } else {
+                result.push('%');
+                result.push(hex(byte >> 4));
+                result.push(hex(byte & 0x0f));
+            }
+            result
+        })
     }
-    fn wait_until_function_exists(&self) -> crate::waiters::function_exists::FunctionExistsFluentBuilder {
-        crate::waiters::function_exists::FunctionExistsFluentBuilder::new(self.handle.clone())
+    fn hex(value: u8) -> char {
+        match value {
+            0..=9 => (b'0' + value) as char,
+            _ => (b'A' + value - 10) as char,
+        }
     }
-    fn wait_until_function_updated_v2(&self) -> crate::waiters::function_updated_v2::FunctionUpdatedV2FluentBuilder {
-        crate::waiters::function_updated_v2::FunctionUpdatedV2FluentBuilder::new(self.handle.clone())
+    pub(crate) fn xml_escape(value: &str) -> String {
+        value
+            .replace('&', "&amp;")
+            .replace('<', "&lt;")
+            .replace('>', "&gt;")
+            .replace('\"', "&quot;")
+            .replace('\'', "&apos;")
     }
-    fn wait_until_function_active(&self) -> crate::waiters::function_active::FunctionActiveFluentBuilder {
-        crate::waiters::function_active::FunctionActiveFluentBuilder::new(self.handle.clone())
+    pub(crate) fn xml_unescape(value: &str) -> String {
+        value
+            .replace("&lt;", "<")
+            .replace("&gt;", ">")
+            .replace("&apos;", "'")
+            .replace("&amp;", "&")
     }
-    fn wait_until_function_updated(&self) -> crate::waiters::function_updated::FunctionUpdatedFluentBuilder {
-        crate::waiters::function_updated::FunctionUpdatedFluentBuilder::new(self.handle.clone())
+    pub(crate) fn xml_first(xml: &str, tag: &str) -> Option<String> {
+        xml_tags(xml, tag).into_iter().next().map(|value| xml_unescape(&value))
     }
-    fn wait_until_published_version_active(&self) -> crate::waiters::published_version_active::PublishedVersionActiveFluentBuilder {
-        crate::waiters::published_version_active::PublishedVersionActiveFluentBuilder::new(self.handle.clone())
+    pub(crate) fn xml_tags(xml: &str, tag: &str) -> Vec<String> {
+        let open = format!("<{tag}>");
+        let close = format!("</{tag}>");
+        let mut values = Vec::new();
+        let mut remaining = xml;
+        while let Some(start) = remaining.find(&open) {
+            let value_start = start + open.len();
+            let Some(end) = remaining[value_start..].find(&close) else { break };
+            values.push(remaining[value_start..value_start + end].to_owned());
+            remaining = &remaining[value_start + end + close.len()..];
+        }
+        values
     }
 }

+// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+
+#[derive(Clone, Debug, Default)]
+pub struct Client {
+    config: Config,
+    http: transport::HttpClient,
+}
 impl Client {
-    /// Creates a new client from an [SDK Config](::aws_types::sdk_config::SdkConfig).
-    ///
-    /// # Panics
-    ///
-    /// - This method will panic if the `sdk_config` is missing an async sleep implementation. If you experience this panic, set
-    ///   the `sleep_impl` on the Config passed into this function to fix it.
-    /// - This method will panic if the `sdk_config` is missing an HTTP connector. If you experience this panic, set the
-    ///   `http_connector` on the Config passed into this function to fix it.
-    /// - This method will panic if no `BehaviorVersion` is provided. If you experience this panic, set `behavior_version` on the Config or enable the `behavior-version-latest` Cargo feature.
-    #[track_caller]
-    pub fn new(sdk_config: &::aws_types::sdk_config::SdkConfig) -> Self {
-        Self::from_conf(sdk_config.into())
+    pub fn new(config: &Config) -> Self {
+        Self {
+            config: config.clone(),
+            http: transport::HttpClient::new(),
+        }
+    }
+    pub fn config(&self) -> &Config {
+        &self.config
     }
+    pub(crate) async fn request(
+        &self,
+        method: transport::Method,
+        path: &str,
+        headers: &[(&str, &str)],
+        body: &[u8],
+    ) -> ::std::result::Result<transport::Response, ::std::string::String> {
+        let url = format!("{}{}", self.config.endpoint_url.trim_end_matches('/'), path);
+        self.http.request(method, &url, headers, body).await
+    }
 }

-mod add_layer_version_permission;
-
-mod add_permission;
-
-mod checkpoint_durable_execution;
-
-mod create_alias;
-
-mod create_capacity_provider;
-
-mod create_code_signing_config;
-
-mod create_event_source_mapping;
-
-mod create_function;
-
-mod create_function_url_config;
-
-/// Operation customization and supporting types.
-///
-/// The underlying HTTP requests made during an operation can be customized
-/// by calling the `customize()` method on the builder returned from a client
-/// operation call. For example, this can be used to add an additional HTTP header:
-///
-/// ```ignore
-/// # async fn wrapper() -> ::std::result::Result<(), aws_sdk_lambda::Error> {
-/// # let client: aws_sdk_lambda::Client = unimplemented!();
-/// use ::http_1x::header::{HeaderName, HeaderValue};
-///
-/// let result = client.add_layer_version_permission()
-///     .customize()
-///     .mutate_request(|req| {
-///         // Add `x-example-header` with value
-///         req.headers_mut()
-///             .insert(
-///                 HeaderName::from_static("x-example-header"),
-///                 HeaderValue::from_static("1"),
-///             );
-///     })
-///     .send()
-///     .await;
-/// # }
-/// ```
-pub mod customize;
-
-mod delete_alias;
-
-mod delete_capacity_provider;
-
-mod delete_code_signing_config;
-
-mod delete_event_source_mapping;
-
-mod delete_function;
-
-mod delete_function_code_signing_config;
-
-mod delete_function_concurrency;
-
-mod delete_function_event_invoke_config;
-
-mod delete_function_url_config;
-
-mod delete_layer_version;
-
-mod delete_provisioned_concurrency_config;
-
-mod delete_resource_policy;
-
-mod get_account_settings;
-
-mod get_alias;
-
-mod get_capacity_provider;
-
-mod get_code_signing_config;
-
-mod get_durable_execution;
-
-mod get_durable_execution_history;
-
-mod get_durable_execution_state;
-
-mod get_event_source_mapping;
-
-mod get_function;
-
-mod get_function_code_signing_config;
-
-mod get_function_concurrency;
-
-mod get_function_configuration;
-
-mod get_function_event_invoke_config;
-
-mod get_function_recursion_config;
-
-mod get_function_scaling_config;
-
-mod get_function_url_config;
-
-mod get_layer_version;
-
-mod get_layer_version_by_arn;
-
-mod get_layer_version_policy;
-
-mod get_policy;
-
-mod get_provisioned_concurrency_config;
-
-mod get_resource_policy;
-
-mod get_runtime_management_config;
-
-mod invoke;
-
-mod invoke_async;
-
-mod invoke_with_response_stream;
-
-mod list_aliases;
-
-mod list_capacity_providers;
-
-mod list_code_signing_configs;
-
-mod list_durable_executions_by_function;
-
-mod list_event_source_mappings;
-
-mod list_function_event_invoke_configs;
-
-mod list_function_url_configs;
-
-mod list_function_versions_by_capacity_provider;
-
-mod list_functions;
-
-mod list_functions_by_code_signing_config;
-
-mod list_layer_versions;
-
-mod list_layers;
-
-mod list_provisioned_concurrency_configs;
-
-mod list_tags;
-
-mod list_versions_by_function;
-
-mod publish_layer_version;
-
-mod publish_version;
-
-mod put_function_code_signing_config;
-
-mod put_function_concurrency;
-
-mod put_function_event_invoke_config;
-
-mod put_function_recursion_config;
-
-mod put_function_scaling_config;
-
-mod put_provisioned_concurrency_config;
-
-mod put_resource_policy;
-
-mod put_runtime_management_config;
-
-mod remove_layer_version_permission;
-
-mod remove_permission;
-
-mod send_durable_execution_callback_failure;
-
-mod send_durable_execution_callback_heartbeat;
-
-mod send_durable_execution_callback_success;
-
-mod stop_durable_execution;
-
-mod tag_resource;
-
-mod untag_resource;
-
-mod update_alias;
-
-mod update_capacity_provider;
-
-mod update_code_signing_config;
-
-mod update_event_source_mapping;
-
-mod update_function_code;
-
-mod update_function_configuration;
-
-mod update_function_event_invoke_config;
-
-mod update_function_url_config;
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/add_layer_version_permission.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/add_permission.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/checkpoint_durable_execution.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/create_alias.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/create_capacity_provider.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/create_code_signing_config.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/create_event_source_mapping.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/create_function.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/create_function_url_config.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/delete_alias.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/delete_capacity_provider.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/delete_code_signing_config.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/delete_event_source_mapping.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/delete_function.rs"));
+include!(concat!(
+    env!("OUT_DIR"),
+    "/generated/lambda/src/client/delete_function_code_signing_config.rs"
+));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/delete_function_concurrency.rs"));
+include!(concat!(
+    env!("OUT_DIR"),
+    "/generated/lambda/src/client/delete_function_event_invoke_config.rs"
+));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/delete_function_url_config.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/delete_layer_version.rs"));
+include!(concat!(
+    env!("OUT_DIR"),
+    "/generated/lambda/src/client/delete_provisioned_concurrency_config.rs"
+));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/delete_resource_policy.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/get_account_settings.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/get_alias.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/get_capacity_provider.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/get_code_signing_config.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/get_durable_execution.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/get_durable_execution_history.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/get_durable_execution_state.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/get_event_source_mapping.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/get_function.rs"));
+include!(concat!(
+    env!("OUT_DIR"),
+    "/generated/lambda/src/client/get_function_code_signing_config.rs"
+));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/get_function_concurrency.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/get_function_configuration.rs"));
+include!(concat!(
+    env!("OUT_DIR"),
+    "/generated/lambda/src/client/get_function_event_invoke_config.rs"
+));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/get_function_recursion_config.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/get_function_scaling_config.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/get_function_url_config.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/get_layer_version.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/get_layer_version_by_arn.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/get_layer_version_policy.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/get_policy.rs"));
+include!(concat!(
+    env!("OUT_DIR"),
+    "/generated/lambda/src/client/get_provisioned_concurrency_config.rs"
+));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/get_resource_policy.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/get_runtime_management_config.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/invoke.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/invoke_async.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/invoke_with_response_stream.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/list_aliases.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/list_capacity_providers.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/list_code_signing_configs.rs"));
+include!(concat!(
+    env!("OUT_DIR"),
+    "/generated/lambda/src/client/list_durable_executions_by_function.rs"
+));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/list_event_source_mappings.rs"));
+include!(concat!(
+    env!("OUT_DIR"),
+    "/generated/lambda/src/client/list_function_event_invoke_configs.rs"
+));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/list_function_url_configs.rs"));
+include!(concat!(
+    env!("OUT_DIR"),
+    "/generated/lambda/src/client/list_function_versions_by_capacity_provider.rs"
+));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/list_functions.rs"));
+include!(concat!(
+    env!("OUT_DIR"),
+    "/generated/lambda/src/client/list_functions_by_code_signing_config.rs"
+));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/list_layer_versions.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/list_layers.rs"));
+include!(concat!(
+    env!("OUT_DIR"),
+    "/generated/lambda/src/client/list_provisioned_concurrency_configs.rs"
+));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/list_tags.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/list_versions_by_function.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/publish_layer_version.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/publish_version.rs"));
+include!(concat!(
+    env!("OUT_DIR"),
+    "/generated/lambda/src/client/put_function_code_signing_config.rs"
+));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/put_function_concurrency.rs"));
+include!(concat!(
+    env!("OUT_DIR"),
+    "/generated/lambda/src/client/put_function_event_invoke_config.rs"
+));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/put_function_recursion_config.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/put_function_scaling_config.rs"));
+include!(concat!(
+    env!("OUT_DIR"),
+    "/generated/lambda/src/client/put_provisioned_concurrency_config.rs"
+));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/put_resource_policy.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/put_runtime_management_config.rs"));
+include!(concat!(
+    env!("OUT_DIR"),
+    "/generated/lambda/src/client/remove_layer_version_permission.rs"
+));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/remove_permission.rs"));
+include!(concat!(
+    env!("OUT_DIR"),
+    "/generated/lambda/src/client/send_durable_execution_callback_failure.rs"
+));
+include!(concat!(
+    env!("OUT_DIR"),
+    "/generated/lambda/src/client/send_durable_execution_callback_heartbeat.rs"
+));
+include!(concat!(
+    env!("OUT_DIR"),
+    "/generated/lambda/src/client/send_durable_execution_callback_success.rs"
+));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/stop_durable_execution.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/tag_resource.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/untag_resource.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/update_alias.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/update_capacity_provider.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/update_code_signing_config.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/update_event_source_mapping.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/update_function_code.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/update_function_configuration.rs"));
+include!(concat!(
+    env!("OUT_DIR"),
+    "/generated/lambda/src/client/update_function_event_invoke_config.rs"
+));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client/update_function_url_config.rs"));
```

### `src/config.rs`

```diff
--- reference/src/config.rs
+++ generated/src/config.rs
@@ -1,1737 +1,45 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-#![allow(clippy::empty_line_after_doc_comments)]
-/// Configuration for a aws_sdk_lambda service client.
-///
-/// Service configuration allows for customization of endpoints, region, credentials providers,
-/// and retry configuration. Generally, it is constructed automatically for you from a shared
-/// configuration loaded by the `aws-config` crate. For example:
-///
-/// ```ignore
-/// // Load a shared config from the environment
-/// let shared_config = aws_config::from_env().load().await;
-/// // The client constructor automatically converts the shared config into the service config
-/// let client = Client::new(&shared_config);
-/// ```
-///
-/// The service config can also be constructed manually using its builder.
-///
-#[derive(::std::clone::Clone, ::std::fmt::Debug)]
+
+#[derive(Clone, Debug)]
 pub struct Config {
-    // Both `config` and `cloneable` are the same config, but the cloneable one
-    // is kept around so that it is possible to convert back into a builder. This can be
-    // optimized in the future.
-    pub(crate) config: crate::config::FrozenLayer,
-    cloneable: ::aws_smithy_types::config_bag::CloneableLayer,
-    pub(crate) runtime_components: crate::config::RuntimeComponentsBuilder,
-    pub(crate) runtime_plugins: ::std::vec::Vec<crate::config::SharedRuntimePlugin>,
-    pub(crate) behavior_version: ::std::option::Option<crate::config::BehaviorVersion>,
+    pub(crate) endpoint_url: ::std::string::String,
 }
-impl Config {
-    ///
-    /// Constructs a config builder.
-    /// <div class="warning">
-    /// Note that a config created from this builder will not have the same safe defaults as one created by
-    /// the <a href="https://crates.io/crates/aws-config" target="_blank">aws-config</a> crate.
-    /// </div>
-    ///
-    pub fn builder() -> Builder {
-        Builder::default()
-    }
-    /// Converts this config back into a builder so that it can be tweaked.
-    pub fn to_builder(&self) -> Builder {
-        Builder {
-            config: self.cloneable.clone(),
-            runtime_components: self.runtime_components.clone(),
-            runtime_plugins: self.runtime_plugins.clone(),
-            behavior_version: self.behavior_version,
-        }
-    }
-    /// Return a reference to the stalled stream protection configuration contained in this config, if any.
-    pub fn stalled_stream_protection(&self) -> ::std::option::Option<&crate::config::StalledStreamProtectionConfig> {
-        self.config.load::<crate::config::StalledStreamProtectionConfig>()
-    }
-    /// Return the [`SharedHttpClient`](crate::config::SharedHttpClient) to use when making requests, if any.
-    pub fn http_client(&self) -> Option<crate::config::SharedHttpClient> {
-        self.runtime_components.http_client()
-    }
-    /// Return the auth schemes configured on this service config
-    pub fn auth_schemes(&self) -> impl Iterator<Item = ::aws_smithy_runtime_api::client::auth::SharedAuthScheme> + '_ {
-        self.runtime_components.auth_schemes()
-    }

-    /// Return the auth scheme resolver configured on this service config
-    pub fn auth_scheme_resolver(&self) -> ::std::option::Option<::aws_smithy_runtime_api::client::auth::SharedAuthSchemeOptionResolver> {
-        self.runtime_components.auth_scheme_option_resolver()
-    }
-    /// Returns the configured auth scheme preference
-    pub fn auth_scheme_preference(&self) -> ::std::option::Option<&::aws_smithy_runtime_api::client::auth::AuthSchemePreference> {
-        self.config.load::<::aws_smithy_runtime_api::client::auth::AuthSchemePreference>()
-    }
-    /// Returns the endpoint resolver.
-    pub fn endpoint_resolver(&self) -> ::aws_smithy_runtime_api::client::endpoint::SharedEndpointResolver {
-        self.runtime_components.endpoint_resolver().expect("resolver defaulted if not set")
-    }
-    /// Return a reference to the retry configuration contained in this config, if any.
-    pub fn retry_config(&self) -> ::std::option::Option<&::aws_smithy_types::retry::RetryConfig> {
-        self.config.load::<::aws_smithy_types::retry::RetryConfig>()
-    }
-
-    /// Return a cloned shared async sleep implementation from this config, if any.
-    pub fn sleep_impl(&self) -> ::std::option::Option<crate::config::SharedAsyncSleep> {
-        self.runtime_components.sleep_impl()
-    }
-
-    /// Return a reference to the timeout configuration contained in this config, if any.
-    pub fn timeout_config(&self) -> ::std::option::Option<&::aws_smithy_types::timeout::TimeoutConfig> {
-        self.config.load::<::aws_smithy_types::timeout::TimeoutConfig>()
-    }
-
-    /// Returns a reference to the retry partition contained in this config, if any.
-    ///
-    /// WARNING: This method is unstable and may be removed at any time. Do not rely on this
-    /// method for anything!
-    pub fn retry_partition(&self) -> ::std::option::Option<&::aws_smithy_runtime::client::retries::RetryPartition> {
-        self.config.load::<::aws_smithy_runtime::client::retries::RetryPartition>()
-    }
-    /// Returns the configured identity cache for auth.
-    pub fn identity_cache(&self) -> ::std::option::Option<crate::config::SharedIdentityCache> {
-        self.runtime_components.identity_cache()
-    }
-    /// Returns interceptors currently registered by the user.
-    pub fn interceptors(&self) -> impl Iterator<Item = crate::config::SharedInterceptor> + '_ {
-        self.runtime_components.interceptors()
-    }
-    /// Return time source used for this service.
-    pub fn time_source(&self) -> ::std::option::Option<::aws_smithy_async::time::SharedTimeSource> {
-        self.runtime_components.time_source()
-    }
-    /// Returns retry classifiers currently registered by the user.
-    pub fn retry_classifiers(&self) -> impl Iterator<Item = ::aws_smithy_runtime_api::client::retries::classifiers::SharedRetryClassifier> + '_ {
-        self.runtime_components.retry_classifiers()
-    }
-    /// Returns the name of the app that is using the client, if it was provided.
-    ///
-    /// This _optional_ name is used to identify the application in the user agent that
-    /// gets sent along with requests.
-    pub fn app_name(&self) -> ::std::option::Option<&::aws_types::app_name::AppName> {
-        self.config.load::<::aws_types::app_name::AppName>()
-    }
-    /// Returns the framework metadata that has been configured, if any.
-    ///
-    /// This _optional_ metadata identifies software frameworks or third-party libraries
-    /// being used with the client, rendered into the user agent as `lib/{name}/{version}`.
-    /// Entries are returned in first-seen (insertion) order, matching the order they are
-    /// rendered into the user agent.
-    pub fn framework_metadata(&self) -> ::std::vec::Vec<&::aws_types::sdk_ua_metadata::FrameworkMetadata> {
-        // `StoreAppend` loads entries newest-first; reverse to first-seen order so
-        // this getter agrees with both the user agent and `SdkConfig::framework_metadata`.
-        let mut entries: ::std::vec::Vec<&::aws_types::sdk_ua_metadata::FrameworkMetadata> =
-            self.config.load::<::aws_types::sdk_ua_metadata::FrameworkMetadata>().collect();
-        entries.reverse();
-        entries
-    }
-    /// Returns the invocation ID generator if one was given in config.
-    ///
-    /// The invocation ID generator generates ID values for the `amz-sdk-invocation-id` header. By default, this will be a random UUID. Overriding it may be useful in tests that examine the HTTP request and need to be deterministic.
-    pub fn invocation_id_generator(&self) -> ::std::option::Option<::aws_runtime::invocation_id::SharedInvocationIdGenerator> {
-        self.config.load::<::aws_runtime::invocation_id::SharedInvocationIdGenerator>().cloned()
-    }
-    /// Creates a new [service config](crate::Config) from a [shared `config`](::aws_types::sdk_config::SdkConfig).
-    pub fn new(config: &::aws_types::sdk_config::SdkConfig) -> Self {
-        Builder::from(config).build()
-    }
-    /// The signature version 4 service signing name to use in the credential scope when signing requests.
-    ///
-    /// The signing service may be overridden by the `Endpoint`, or by specifying a custom
-    /// [`SigningName`](aws_types::SigningName) during operation construction
-    pub fn signing_name(&self) -> &'static str {
-        "lambda"
-    }
-    /// Returns the AWS region, if it was provided.
-    pub fn region(&self) -> ::std::option::Option<&crate::config::Region> {
-        self.config.load::<crate::config::Region>()
-    }
-    /// This function was intended to be removed, and has been broken since release-2023-11-15 as it always returns a `None`. Do not use.
-    #[deprecated(
-        note = "This function was intended to be removed, and has been broken since release-2023-11-15 as it always returns a `None`. Do not use."
-    )]
-    pub fn credentials_provider(&self) -> Option<crate::config::SharedCredentialsProvider> {
-        ::std::option::Option::None
-    }
-}
-/// Builder for creating a `Config`.
-#[derive(::std::clone::Clone, ::std::fmt::Debug)]
-pub struct Builder {
-    pub(crate) config: ::aws_smithy_types::config_bag::CloneableLayer,
-    pub(crate) runtime_components: crate::config::RuntimeComponentsBuilder,
-    pub(crate) runtime_plugins: ::std::vec::Vec<crate::config::SharedRuntimePlugin>,
-    pub(crate) behavior_version: ::std::option::Option<crate::config::BehaviorVersion>,
-}
-impl ::std::default::Default for Builder {
+impl ::std::default::Default for Config {
     fn default() -> Self {
         Self {
-            config: ::std::default::Default::default(),
-            runtime_components: crate::config::RuntimeComponentsBuilder::new("service config"),
-            runtime_plugins: ::std::default::Default::default(),
-            behavior_version: ::std::default::Default::default(),
+            endpoint_url: ::std::env::var("AWS_ENDPOINT_URL").unwrap_or_else(|_| "http://localhost:4566".to_owned()),
         }
     }
 }
-impl Builder {
-    ///
-    /// Constructs a config builder.
-    /// <div class="warning">
-    /// Note that a config created from this builder will not have the same safe defaults as one created by
-    /// the <a href="https://crates.io/crates/aws-config" target="_blank">aws-config</a> crate.
-    /// </div>
-    ///
-    pub fn new() -> Self {
-        Self::default()
-    }
-    /// Constructs a config builder from the given `config_bag`, setting only fields stored in the config bag,
-    /// but not those in runtime components.
-    #[allow(unused)]
-    pub(crate) fn from_config_bag(config_bag: &::aws_smithy_types::config_bag::ConfigBag) -> Self {
-        let mut builder = Self::new();
-        builder.set_stalled_stream_protection(config_bag.load::<crate::config::StalledStreamProtectionConfig>().cloned());
-        builder.set_auth_scheme_preference(config_bag.load::<::aws_smithy_runtime_api::client::auth::AuthSchemePreference>().cloned());
-        builder.set_retry_config(config_bag.load::<::aws_smithy_types::retry::RetryConfig>().cloned());
-        builder.set_timeout_config(config_bag.load::<::aws_smithy_types::timeout::TimeoutConfig>().cloned());
-        builder.set_retry_partition(config_bag.load::<::aws_smithy_runtime::client::retries::RetryPartition>().cloned());
-        builder.set_app_name(config_bag.load::<::aws_types::app_name::AppName>().cloned());
-        for framework_metadata in config_bag.load::<::aws_types::sdk_ua_metadata::FrameworkMetadata>() {
-            builder.push_framework_metadata(framework_metadata.clone());
-        }
-        builder.set_endpoint_url(config_bag.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()));
-        builder.set_use_dual_stack(config_bag.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0));
-        builder.set_use_fips(config_bag.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0));
-        builder.set_region(config_bag.load::<crate::config::Region>().cloned());
-        builder
-    }
-    /// Names operation-input members whose values are captured *and* emitted as
-    /// attributes on the client's built-in metrics (e.g. `["Bucket"]`).
-    ///
-    /// Emitting implies capture, so an emitted member is also readable in-process
-    /// via `CapturedTelemetryAttributes` on the config bag. Names are Smithy input
-    /// member names; only string-valued, non-sensitive members are eligible, and
-    /// naming any other member has no effect. Off by default.
-    ///
-    /// Prefer bounded identifiers here: an emitted member becomes a metric label, so
-    /// high-cardinality values (like object keys) fragment the metrics and inflate
-    /// cost. Use [`Self::capture_input_attributes`] for values you want to read
-    /// in-process without emitting them on the metrics.
-    pub fn emit_input_attributes(mut self, names: impl ::std::iter::IntoIterator<Item = impl ::std::convert::Into<::std::string::String>>) -> Self {
-        let mut requested = self
-            .config
-            .load::<::aws_smithy_types::telemetry::RequestedTelemetryAttributes>()
-            .cloned()
-            .unwrap_or_default();
-        requested.emit(names.into_iter().map(|n| n.into()));
-        self.config.store_put(requested);
-        self
-    }

-    /// Names operation-input members whose values are captured into
-    /// `CapturedTelemetryAttributes` for in-process reads (e.g. from a custom
-    /// interceptor), but are **not** emitted on the built-in metrics.
-    ///
-    /// Use this for values you need during the operation lifecycle but do not want on
-    /// the metric label set (for example, high-cardinality identifiers). Names follow
-    /// the same eligibility rules as [`Self::emit_input_attributes`]. Off by default.
-    pub fn capture_input_attributes(
-        mut self,
-        names: impl ::std::iter::IntoIterator<Item = impl ::std::convert::Into<::std::string::String>>,
-    ) -> Self {
-        let mut requested = self
-            .config
-            .load::<::aws_smithy_types::telemetry::RequestedTelemetryAttributes>()
-            .cloned()
-            .unwrap_or_default();
-        requested.capture_only(names.into_iter().map(|n| n.into()));
-        self.config.store_put(requested);
-        self
-    }
-    /// Set the [`StalledStreamProtectionConfig`](crate::config::StalledStreamProtectionConfig)
-    /// to configure protection for stalled streams.
-    pub fn stalled_stream_protection(mut self, stalled_stream_protection_config: crate::config::StalledStreamProtectionConfig) -> Self {
-        self.set_stalled_stream_protection(::std::option::Option::Some(stalled_stream_protection_config));
-        self
-    }
-    /// Set the [`StalledStreamProtectionConfig`](crate::config::StalledStreamProtectionConfig)
-    /// to configure protection for stalled streams.
-    pub fn set_stalled_stream_protection(
-        &mut self,
-        stalled_stream_protection_config: ::std::option::Option<crate::config::StalledStreamProtectionConfig>,
-    ) -> &mut Self {
-        self.config.store_or_unset(stalled_stream_protection_config);
-        self
-    }
-    /// Sets the idempotency token provider to use for service calls that require tokens.
-    pub fn idempotency_token_provider(
-        mut self,
-        idempotency_token_provider: impl ::std::convert::Into<crate::idempotency_token::IdempotencyTokenProvider>,
-    ) -> Self {
-        self.set_idempotency_token_provider(::std::option::Option::Some(idempotency_token_provider.into()));
-        self
-    }
-    /// Sets the idempotency token provider to use for service calls that require tokens.
-    pub fn set_idempotency_token_provider(
-        &mut self,
-        idempotency_token_provider: ::std::option::Option<crate::idempotency_token::IdempotencyTokenProvider>,
-    ) -> &mut Self {
-        self.config.store_or_unset(idempotency_token_provider);
-        self
-    }
-    /// Sets the HTTP client to use when making requests.
-    ///
-    /// # Examples
-    /// ```no_run
-    /// # #[cfg(test)]
-    /// # mod tests {
-    /// # #[test]
-    /// # fn example() {
-    /// use std::time::Duration;
-    /// use aws_sdk_lambda::config::Config;
-    /// use aws_smithy_runtime::client::http::hyper_014::HyperClientBuilder;
-    ///
-    /// let https_connector = hyper_rustls::HttpsConnectorBuilder::new()
-    ///     .with_webpki_roots()
-    ///     .https_only()
-    ///     .enable_http1()
-    ///     .enable_http2()
-    ///     .build();
-    /// let hyper_client = HyperClientBuilder::new().build(https_connector);
-    ///
-    /// // This connector can then be given to a generated service Config
-    /// let config = my_service_client::Config::builder()
-    ///     .endpoint_url("https://example.com")
-    ///     .http_client(hyper_client)
-    ///     .build();
-    /// let client = my_service_client::Client::from_conf(config);
-    /// # }
-    /// # }
-    /// ```
-    pub fn http_client(mut self, http_client: impl crate::config::HttpClient + 'static) -> Self {
-        self.set_http_client(::std::option::Option::Some(crate::config::IntoShared::into_shared(http_client)));
-        self
-    }
-
-    /// Sets the HTTP client to use when making requests.
-    ///
-    /// # Examples
-    /// ```no_run
-    /// # #[cfg(test)]
-    /// # mod tests {
-    /// # #[test]
-    /// # fn example() {
-    /// use std::time::Duration;
-    /// use aws_sdk_lambda::config::{Builder, Config};
-    /// use aws_smithy_runtime::client::http::hyper_014::HyperClientBuilder;
-    ///
-    /// fn override_http_client(builder: &mut Builder) {
-    ///     let https_connector = hyper_rustls::HttpsConnectorBuilder::new()
-    ///         .with_webpki_roots()
-    ///         .https_only()
-    ///         .enable_http1()
-    ///         .enable_http2()
-    ///         .build();
-    ///     let hyper_client = HyperClientBuilder::new().build(https_connector);
-    ///     builder.set_http_client(Some(hyper_client));
-    /// }
-    ///
-    /// let mut builder = aws_sdk_lambda::Config::builder();
-    /// override_http_client(&mut builder);
-    /// let config = builder.build();
-    /// # }
-    /// # }
-    /// ```
-    pub fn set_http_client(&mut self, http_client: Option<crate::config::SharedHttpClient>) -> &mut Self {
-        self.runtime_components.set_http_client(http_client);
-        self
-    }
-    /// Adds an auth scheme to the builder
-    ///
-    /// If `auth_scheme` has an existing [AuthSchemeId](aws_smithy_runtime_api::client::auth::AuthSchemeId) in the runtime, the current identity
-    /// resolver and signer for that scheme will be replaced by those from `auth_scheme`.
-    ///
-    /// _Important:_ When introducing a custom auth scheme, ensure you override either
-    /// [`Self::auth_scheme_resolver`] or [`Self::set_auth_scheme_resolver`]
-    /// so that the custom auth scheme is included in the list of resolved auth scheme options.
-    /// [The default auth scheme resolver](crate::config::auth::DefaultAuthSchemeResolver) will not recognize your custom auth scheme.
-    ///
-    /// # Examples
-    /// ```no_run
-    /// # use aws_smithy_runtime_api::{
-    /// #     box_error::BoxError,
-    /// #     client::{
-    /// #         auth::{
-    /// #             AuthScheme, AuthSchemeEndpointConfig, AuthSchemeId, AuthSchemeOption,
-    /// #             AuthSchemeOptionsFuture, Sign,
-    /// #         },
-    /// #         identity::{Identity, IdentityFuture, ResolveIdentity, SharedIdentityResolver},
-    /// #         orchestrator::HttpRequest,
-    /// #         runtime_components::{GetIdentityResolver, RuntimeComponents},
-    /// #   },
-    /// #   shared::IntoShared,
-    /// # };
-    /// # use aws_smithy_types::config_bag::ConfigBag;
-    /// // Auth scheme with customer identity resolver and signer
-    /// #[derive(Debug)]
-    /// struct CustomAuthScheme {
-    ///     id: AuthSchemeId,
-    ///     identity_resolver: SharedIdentityResolver,
-    ///     signer: CustomSigner,
-    /// }
-    /// impl Default for CustomAuthScheme {
-    ///     fn default() -> Self {
-    ///         Self {
-    ///             id: AuthSchemeId::new("custom"),
-    ///             identity_resolver: CustomIdentityResolver.into_shared(),
-    ///             signer: CustomSigner,
-    ///         }
-    ///     }
-    /// }
-    /// impl AuthScheme for CustomAuthScheme {
-    ///     fn scheme_id(&self) -> AuthSchemeId {
-    ///         self.id.clone()
-    ///     }
-    ///     fn identity_resolver(
-    ///         &self,
-    ///         _identity_resolvers: &dyn GetIdentityResolver,
-    ///     ) -> Option<SharedIdentityResolver> {
-    ///         Some(self.identity_resolver.clone())
-    ///     }
-    ///     fn signer(&self) -> &dyn Sign {
-    ///         &self.signer
-    ///     }
-    /// }
-    ///
-    /// #[derive(Debug, Default)]
-    /// struct CustomSigner;
-    /// impl Sign for CustomSigner {
-    ///     fn sign_http_request(
-    ///         &self,
-    ///         _request: &mut HttpRequest,
-    ///         _identity: &Identity,
-    ///         _auth_scheme_endpoint_config: AuthSchemeEndpointConfig<'_>,
-    ///         _runtime_components: &RuntimeComponents,
-    ///         _config_bag: &ConfigBag,
-    ///     ) -> Result<(), BoxError> {
-    ///         // --snip--
-    /// #      todo!()
-    ///     }
-    /// }
-    ///
-    /// #[derive(Debug)]
-    /// struct CustomIdentityResolver;
-    /// impl ResolveIdentity for CustomIdentityResolver {
-    ///     fn resolve_identity<'a>(
-    ///         &'a self,
-    ///         _runtime_components: &'a RuntimeComponents,
-    ///         _config_bag: &'a ConfigBag,
-    ///     ) -> IdentityFuture<'a> {
-    ///         // --snip--
-    /// #      todo!()
-    ///     }
-    /// }
-    ///
-    /// // Auth scheme resolver that favors `CustomAuthScheme`
-    /// #[derive(Debug)]
-    /// struct CustomAuthSchemeResolver;
-    /// impl aws_sdk_lambda::config::auth::ResolveAuthScheme for CustomAuthSchemeResolver {
-    ///     fn resolve_auth_scheme<'a>(
-    ///         &'a self,
-    ///         _params: &'a aws_sdk_lambda::config::auth::Params,
-    ///         _cfg: &'a ConfigBag,
-    ///         _runtime_components: &'a RuntimeComponents,
-    ///     ) -> AuthSchemeOptionsFuture<'a> {
-    ///         AuthSchemeOptionsFuture::ready(Ok(vec![AuthSchemeOption::from(AuthSchemeId::new(
-    ///             "custom",
-    ///         ))]))
-    ///     }
-    /// }
-    ///
-    /// let config = aws_sdk_lambda::Config::builder()
-    ///     .push_auth_scheme(CustomAuthScheme::default())
-    ///     .auth_scheme_resolver(CustomAuthSchemeResolver)
-    ///     // other configurations
-    ///     .build();
-    /// ```
-    pub fn push_auth_scheme(mut self, auth_scheme: impl ::aws_smithy_runtime_api::client::auth::AuthScheme + 'static) -> Self {
-        self.runtime_components.push_auth_scheme(auth_scheme);
-        self
-    }
-
-    /// Set the auth scheme resolver for the builder
-    ///
-    /// # Examples
-    /// ```no_run
-    /// # use aws_smithy_runtime_api::{
-    /// #     client::{
-    /// #         auth::AuthSchemeOptionsFuture,
-    /// #         runtime_components::RuntimeComponents,
-    /// #   },
-    /// # };
-    /// # use aws_smithy_types::config_bag::ConfigBag;
-    /// #[derive(Debug)]
-    /// struct CustomAuthSchemeResolver;
-    /// impl aws_sdk_lambda::config::auth::ResolveAuthScheme for CustomAuthSchemeResolver {
-    ///     fn resolve_auth_scheme<'a>(
-    ///         &'a self,
-    ///         _params: &'a aws_sdk_lambda::config::auth::Params,
-    ///         _cfg: &'a ConfigBag,
-    ///         _runtime_components: &'a RuntimeComponents,
-    ///     ) -> AuthSchemeOptionsFuture<'a> {
-    ///         // --snip--
-    /// #      todo!()
-    ///     }
-    /// }
-    ///
-    /// let config = aws_sdk_lambda::Config::builder()
-    ///     .auth_scheme_resolver(CustomAuthSchemeResolver)
-    ///     // other configurations
-    ///     .build();
-    /// ```
-    pub fn auth_scheme_resolver(mut self, auth_scheme_resolver: impl crate::config::auth::ResolveAuthScheme + 'static) -> Self {
-        self.set_auth_scheme_resolver(auth_scheme_resolver);
-        self
-    }
-
-    /// Set the auth scheme resolver for the builder
-    ///
-    /// # Examples
-    /// See an example for [`Self::auth_scheme_resolver`].
-    pub fn set_auth_scheme_resolver(&mut self, auth_scheme_resolver: impl crate::config::auth::ResolveAuthScheme + 'static) -> &mut Self {
-        self.runtime_components
-            .set_auth_scheme_option_resolver(::std::option::Option::Some(auth_scheme_resolver.into_shared_resolver()));
-        self
-    }
-
-    /// Enable no authentication regardless of what authentication mechanisms operations support
-    ///
-    /// This adds [NoAuthScheme](aws_smithy_runtime::client::auth::no_auth::NoAuthScheme) as a fallback
-    /// and the auth scheme resolver will use it when no other auth schemes are applicable.
-    pub fn allow_no_auth(mut self) -> Self {
-        self.set_allow_no_auth();
-        self
-    }
-
-    /// Enable no authentication regardless of what authentication mechanisms operations support
-    ///
-    /// This adds [NoAuthScheme](aws_smithy_runtime::client::auth::no_auth::NoAuthScheme) as a fallback
-    /// and the auth scheme resolver will use it when no other auth schemes are applicable.
-    pub fn set_allow_no_auth(&mut self) -> &mut Self {
-        self.push_runtime_plugin(::aws_smithy_runtime::client::auth::no_auth::NoAuthRuntimePluginV2::new().into_shared());
-        self
-    }
-    /// Set the auth scheme preference for an auth scheme resolver
-    /// (typically the default auth scheme resolver).
-    ///
-    /// Each operation has a predefined order of auth schemes, as determined by the service,
-    /// for auth scheme resolution. By using the auth scheme preference, customers
-    /// can reorder the schemes resolved by the auth scheme resolver.
-    ///
-    /// The preference list is intended as a hint rather than a strict override.
-    /// Any schemes not present in the originally resolved auth schemes will be ignored.
-    ///
-    /// # Examples
-    ///
-    /// ```no_run
-    /// # use aws_smithy_runtime_api::client::auth::AuthSchemeId;
-    /// let config = aws_sdk_lambda::Config::builder()
-    ///     .auth_scheme_preference([AuthSchemeId::from("scheme1"), AuthSchemeId::from("scheme2")])
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_lambda::Client::from_conf(config);
-    /// ```
-
-    pub fn auth_scheme_preference(
-        mut self,
-        preference: impl ::std::convert::Into<::aws_smithy_runtime_api::client::auth::AuthSchemePreference>,
-    ) -> Self {
-        self.set_auth_scheme_preference(::std::option::Option::Some(preference.into()));
-        self
-    }
-
-    /// Set the auth scheme preference for an auth scheme resolver
-    /// (typically the default auth scheme resolver).
-    ///
-    /// Each operation has a predefined order of auth schemes, as determined by the service,
-    /// for auth scheme resolution. By using the auth scheme preference, customers
-    /// can reorder the schemes resolved by the auth scheme resolver.
-    ///
-    /// The preference list is intended as a hint rather than a strict override.
-    /// Any schemes not present in the originally resolved auth schemes will be ignored.
-    ///
-    /// # Examples
-    ///
-    /// ```no_run
-    /// # use aws_smithy_runtime_api::client::auth::AuthSchemeId;
-    /// let config = aws_sdk_lambda::Config::builder()
-    ///     .auth_scheme_preference([AuthSchemeId::from("scheme1"), AuthSchemeId::from("scheme2")])
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_lambda::Client::from_conf(config);
-    /// ```
-
-    pub fn set_auth_scheme_preference(
-        &mut self,
-        preference: ::std::option::Option<::aws_smithy_runtime_api::client::auth::AuthSchemePreference>,
-    ) -> &mut Self {
-        self.config.store_or_unset(preference);
-        self
-    }
-    /// Sets the endpoint resolver to use when making requests.
-    ///
-    ///
-    /// When unset, the client will used a generated endpoint resolver based on the endpoint resolution
-    /// rules for `aws_sdk_lambda`.
-    ///
-    ///
-    /// Note: setting an endpoint resolver will replace any endpoint URL that has been set.
-    /// This method accepts an endpoint resolver [specific to this service](crate::config::endpoint::ResolveEndpoint). If you want to
-    /// provide a shared endpoint resolver, use [`Self::set_endpoint_resolver`].
-    ///
-    /// # Examples
-    /// Create a custom endpoint resolver that resolves a different endpoing per-stage, e.g. staging vs. production.
-    /// ```no_run
-    /// use aws_sdk_lambda::config::endpoint::{ResolveEndpoint, EndpointFuture, Params, Endpoint};
-    /// #[derive(Debug)]
-    /// struct StageResolver { stage: String }
-    /// impl ResolveEndpoint for StageResolver {
-    ///     fn resolve_endpoint(&self, params: &Params) -> EndpointFuture<'_> {
-    ///         let stage = &self.stage;
-    ///         EndpointFuture::ready(Ok(Endpoint::builder().url(format!("{stage}.myservice.com")).build()))
-    ///     }
-    /// }
-    /// let resolver = StageResolver { stage: std::env::var("STAGE").unwrap() };
-    /// let config = aws_sdk_lambda::Config::builder().endpoint_resolver(resolver).build();
-    /// let client = aws_sdk_lambda::Client::from_conf(config);
-    /// ```
-    pub fn endpoint_resolver(mut self, endpoint_resolver: impl crate::config::endpoint::ResolveEndpoint + 'static) -> Self {
-        self.set_endpoint_resolver(::std::option::Option::Some(endpoint_resolver.into_shared_resolver()));
-        self
-    }
-
-    /// Sets the endpoint resolver to use when making requests.
-    ///
-    ///
-    /// When unset, the client will used a generated endpoint resolver based on the endpoint resolution
-    /// rules for `aws_sdk_lambda`.
-    ///
-    pub fn set_endpoint_resolver(
-        &mut self,
-        endpoint_resolver: ::std::option::Option<::aws_smithy_runtime_api::client::endpoint::SharedEndpointResolver>,
-    ) -> &mut Self {
-        self.runtime_components.set_endpoint_resolver(endpoint_resolver);
-        self
-    }
-    /// Set the retry_config for the builder
-    ///
-    /// # Examples
-    /// ```no_run
-    /// use aws_sdk_lambda::config::Config;
-    /// use aws_sdk_lambda::config::retry::RetryConfig;
-    ///
-    /// let retry_config = RetryConfig::standard().with_max_attempts(5);
-    /// let config = Config::builder().retry_config(retry_config).build();
-    /// ```
-    ///
-    /// # Retry token bucket
-    ///
-    /// [`RetryConfig`](::aws_smithy_types::retry::RetryConfig) controls *how many* times to retry and *how long* to back
-    /// off. Retries are **also** gated by a retry token bucket (also called the retry quota) that
-    /// is shared across a [`RetryPartition`](::aws_smithy_runtime::client::retries::RetryPartition). To configure the token bucket — for
-    /// example, to set
-    /// its capacity or to give a workload its own bucket — see [`Self::retry_partition`] and
-    /// [`RetryPartition::custom`](::aws_smithy_runtime::client::retries::RetryPartition::custom).
-    pub fn retry_config(mut self, retry_config: ::aws_smithy_types::retry::RetryConfig) -> Self {
-        self.set_retry_config(Some(retry_config));
-        self
-    }
-
-    /// Set the retry_config for the builder
-    ///
-    /// # Examples
-    /// ```no_run
-    /// use aws_sdk_lambda::config::{Builder, Config};
-    /// use aws_sdk_lambda::config::retry::RetryConfig;
-    ///
-    /// fn disable_retries(builder: &mut Builder) {
-    ///     let retry_config = RetryConfig::standard().with_max_attempts(1);
-    ///     builder.set_retry_config(Some(retry_config));
-    /// }
-    ///
-    /// let mut builder = Config::builder();
-    /// disable_retries(&mut builder);
-    /// let config = builder.build();
-    /// ```
-    pub fn set_retry_config(&mut self, retry_config: ::std::option::Option<::aws_smithy_types::retry::RetryConfig>) -> &mut Self {
-        retry_config.map(|r| self.config.store_put(r));
-        self
-    }
-    /// Set the sleep_impl for the builder
-    ///
-    /// # Examples
-    ///
-    /// ```no_run
-    /// use aws_sdk_lambda::config::{AsyncSleep, Config, SharedAsyncSleep, Sleep};
-    ///
-    /// #[derive(Debug)]
-    /// pub struct ForeverSleep;
-    ///
-    /// impl AsyncSleep for ForeverSleep {
-    ///     fn sleep(&self, duration: std::time::Duration) -> Sleep {
-    ///         Sleep::new(std::future::pending())
-    ///     }
-    /// }
-    ///
-    /// let sleep_impl = SharedAsyncSleep::new(ForeverSleep);
-    /// let config = Config::builder().sleep_impl(sleep_impl).build();
-    /// ```
-    pub fn sleep_impl(mut self, sleep_impl: impl crate::config::AsyncSleep + 'static) -> Self {
-        self.set_sleep_impl(Some(::aws_smithy_runtime_api::shared::IntoShared::into_shared(sleep_impl)));
-        self
-    }
-
-    /// Set the sleep_impl for the builder
-    ///
-    /// # Examples
-    ///
-    /// ```no_run
-    /// use aws_sdk_lambda::config::{AsyncSleep, Builder, Config, SharedAsyncSleep, Sleep};
-    ///
-    /// #[derive(Debug)]
-    /// pub struct ForeverSleep;
-    ///
-    /// impl AsyncSleep for ForeverSleep {
-    ///     fn sleep(&self, duration: std::time::Duration) -> Sleep {
-    ///         Sleep::new(std::future::pending())
-    ///     }
-    /// }
-    ///
-    /// fn set_never_ending_sleep_impl(builder: &mut Builder) {
-    ///     let sleep_impl = SharedAsyncSleep::new(ForeverSleep);
-    ///     builder.set_sleep_impl(Some(sleep_impl));
-    /// }
-    ///
-    /// let mut builder = Config::builder();
-    /// set_never_ending_sleep_impl(&mut builder);
-    /// let config = builder.build();
-    /// ```
-    pub fn set_sleep_impl(&mut self, sleep_impl: ::std::option::Option<crate::config::SharedAsyncSleep>) -> &mut Self {
-        self.runtime_components.set_sleep_impl(sleep_impl);
-        self
-    }
-    /// Set the timeout_config for the builder
-    ///
-    /// # Examples
-    ///
-    /// ```no_run
-    /// # use std::time::Duration;
-    /// use aws_sdk_lambda::config::Config;
-    /// use aws_sdk_lambda::config::timeout::TimeoutConfig;
-    ///
-    /// let timeout_config = TimeoutConfig::builder()
-    ///     .operation_attempt_timeout(Duration::from_secs(1))
-    ///     .build();
-    /// let config = Config::builder().timeout_config(timeout_config).build();
-    /// ```
-    pub fn timeout_config(mut self, timeout_config: ::aws_smithy_types::timeout::TimeoutConfig) -> Self {
-        self.set_timeout_config(Some(timeout_config));
-        self
+pub mod config {
+    #[derive(Clone, Debug, Default)]
+    pub struct Builder {
+        endpoint_url: ::std::option::Option<::std::string::String>,
     }
-
-    /// Set the timeout_config for the builder.
-    ///
-    /// Setting this to `None` has no effect if another source of configuration has set timeouts. If you
-    /// are attempting to disable timeouts, use [`TimeoutConfig::disabled`](::aws_smithy_types::timeout::TimeoutConfig::disabled)
-    ///
-    ///
-    /// # Examples
-    ///
-    /// ```no_run
-    /// # use std::time::Duration;
-    /// use aws_sdk_lambda::config::{Builder, Config};
-    /// use aws_sdk_lambda::config::timeout::TimeoutConfig;
-    ///
-    /// fn set_request_timeout(builder: &mut Builder) {
-    ///     let timeout_config = TimeoutConfig::builder()
-    ///         .operation_attempt_timeout(Duration::from_secs(1))
-    ///         .build();
-    ///     builder.set_timeout_config(Some(timeout_config));
-    /// }
-    ///
-    /// let mut builder = Config::builder();
-    /// set_request_timeout(&mut builder);
-    /// let config = builder.build();
-    /// ```
-    pub fn set_timeout_config(&mut self, timeout_config: ::std::option::Option<::aws_smithy_types::timeout::TimeoutConfig>) -> &mut Self {
-        // passing None has no impact.
-        let Some(mut timeout_config) = timeout_config else { return self };
-
-        if let Some(base) = self.config.load::<::aws_smithy_types::timeout::TimeoutConfig>() {
-            timeout_config.take_defaults_from(base);
+    impl Builder {
+        pub fn endpoint_url(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
+            self.endpoint_url = Some(value.into());
+            self
         }
-        self.config.store_put(timeout_config);
-        self
-    }
-    /// Set the partition for retry-related state. When clients share a retry partition, they will
-    /// also share components such as token buckets and client rate limiters.
-    /// See the [`RetryPartition`](::aws_smithy_runtime::client::retries::RetryPartition) documentation for more details.
-    ///
-    /// # Default Behavior
-    ///
-    /// When no retry partition is explicitly set, the SDK automatically creates a default retry partition named `lambda`
-    /// (or `lambda-<region>` if a region is configured).
-    /// All Lambda clients without an explicit retry partition will share this default partition.
-    ///
-    /// # Notes
-    ///
-    /// - This is an advanced setting. A common reason to set it is to size or isolate the retry
-    ///   token bucket — for example, giving a high-throughput workload its own bucket. Otherwise
-    ///   most users won't need to modify it.
-    /// - A configured client rate limiter has no effect unless [`RetryConfig::adaptive`](::aws_smithy_types::retry::RetryConfig::adaptive) is used.
-    ///
-    /// # Examples
-    ///
-    /// Creating a custom retry partition with a token bucket:
-    /// ```no_run
-    /// use aws_sdk_lambda::config::Config;
-    /// use aws_sdk_lambda::config::retry::{RetryPartition, TokenBucket};
-    ///
-    /// let token_bucket = TokenBucket::new(10);
-    /// let config = Config::builder()
-    ///     .retry_partition(RetryPartition::custom("custom")
-    ///         .token_bucket(token_bucket)
-    ///         .build()
-    ///     )
-    ///     .build();
-    /// ```
-    ///
-    /// Sizing the retry token bucket (for example, for a high-throughput workload), or giving a
-    /// workload its own bucket:
-    /// ```no_run
-    /// use aws_sdk_lambda::config::Config;
-    /// use aws_sdk_lambda::config::retry::{RetryPartition, TokenBucket};
-    ///
-    /// let config = Config::builder()
-    ///     .retry_partition(
-    ///         RetryPartition::custom("high-throughput")
-    ///             .token_bucket(TokenBucket::builder().capacity(5000).build())
-    ///             .build(),
-    ///     )
-    ///     .build();
-    /// ```
-    ///
-    /// Configuring a client rate limiter with adaptive retry mode:
-    /// ```no_run
-    /// use aws_sdk_lambda::config::Config;
-    /// use aws_sdk_lambda::config::retry::{ClientRateLimiter, RetryConfig, RetryPartition};
-    ///
-    /// let client_rate_limiter = ClientRateLimiter::new(10.0);
-    /// let config = Config::builder()
-    ///     .retry_partition(RetryPartition::custom("custom")
-    ///         .client_rate_limiter(client_rate_limiter)
-    ///         .build()
-    ///     )
-    ///     .retry_config(RetryConfig::adaptive())
-    ///     .build();
-    /// ```
-    pub fn retry_partition(mut self, retry_partition: ::aws_smithy_runtime::client::retries::RetryPartition) -> Self {
-        self.set_retry_partition(Some(retry_partition));
-        self
-    }
-    /// Like [`Self::retry_partition`], but takes a mutable reference to the builder and an optional `RetryPartition`
-    pub fn set_retry_partition(
-        &mut self,
-        retry_partition: ::std::option::Option<::aws_smithy_runtime::client::retries::RetryPartition>,
-    ) -> &mut Self {
-        retry_partition.map(|r| self.config.store_put(r));
-        self
-    }
-    /// Set the identity cache for auth.
-    ///
-    /// The identity cache defaults to a lazy caching implementation that will resolve
-    /// an identity when it is requested, and place it in the cache thereafter. Subsequent
-    /// requests will take the value from the cache while it is still valid. Once it expires,
-    /// the next request will result in refreshing the identity.
-    ///
-    /// This configuration allows you to disable or change the default caching mechanism.
-    /// To use a custom caching mechanism, implement the [`ResolveCachedIdentity`](crate::config::ResolveCachedIdentity)
-    /// trait and pass that implementation into this function.
-    ///
-    /// # Examples
-    ///
-    /// Disabling identity caching:
-    /// ```no_run
-    /// use aws_sdk_lambda::config::IdentityCache;
-    ///
-    /// let config = aws_sdk_lambda::Config::builder()
-    ///     .identity_cache(IdentityCache::no_cache())
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_lambda::Client::from_conf(config);
-    /// ```
-    ///
-    /// Customizing lazy caching:
-    /// ```no_run
-    /// use aws_sdk_lambda::config::IdentityCache;
-    /// use std::time::Duration;
-    ///
-    /// let config = aws_sdk_lambda::Config::builder()
-    ///     .identity_cache(
-    ///         IdentityCache::lazy()
-    ///             // change the load timeout to 10 seconds
-    ///             .load_timeout(Duration::from_secs(10))
-    ///             .build()
-    ///     )
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_lambda::Client::from_conf(config);
-    /// ```
-    ///
-    pub fn identity_cache(mut self, identity_cache: impl crate::config::ResolveCachedIdentity + 'static) -> Self {
-        self.set_identity_cache(identity_cache);
-        self
-    }
-
-    /// Set the identity cache for auth.
-    ///
-    /// The identity cache defaults to a lazy caching implementation that will resolve
-    /// an identity when it is requested, and place it in the cache thereafter. Subsequent
-    /// requests will take the value from the cache while it is still valid. Once it expires,
-    /// the next request will result in refreshing the identity.
-    ///
-    /// This configuration allows you to disable or change the default caching mechanism.
-    /// To use a custom caching mechanism, implement the [`ResolveCachedIdentity`](crate::config::ResolveCachedIdentity)
-    /// trait and pass that implementation into this function.
-    ///
-    /// # Examples
-    ///
-    /// Disabling identity caching:
-    /// ```no_run
-    /// use aws_sdk_lambda::config::IdentityCache;
-    ///
-    /// let config = aws_sdk_lambda::Config::builder()
-    ///     .identity_cache(IdentityCache::no_cache())
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_lambda::Client::from_conf(config);
-    /// ```
-    ///
-    /// Customizing lazy caching:
-    /// ```no_run
-    /// use aws_sdk_lambda::config::IdentityCache;
-    /// use std::time::Duration;
-    ///
-    /// let config = aws_sdk_lambda::Config::builder()
-    ///     .identity_cache(
-    ///         IdentityCache::lazy()
-    ///             // change the load timeout to 10 seconds
-    ///             .load_timeout(Duration::from_secs(10))
-    ///             .build()
-    ///     )
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_lambda::Client::from_conf(config);
-    /// ```
-    ///
-    pub fn set_identity_cache(&mut self, identity_cache: impl crate::config::ResolveCachedIdentity + 'static) -> &mut Self {
-        self.runtime_components.set_identity_cache(::std::option::Option::Some(identity_cache));
-        self
-    }
-    /// Add an [interceptor](crate::config::Intercept) that runs at specific stages of the request execution pipeline.
-    ///
-    /// Interceptors targeted at a certain stage are executed according to the pre-defined priority.
-    /// The SDK provides a default set of interceptors. An interceptor configured by this method
-    /// will run after those default interceptors.
-    ///
-    /// # Examples
-    /// ```no_run
-    /// # fn example() {
-    /// use aws_smithy_runtime_api::box_error::BoxError;
-    /// use aws_smithy_runtime_api::client::interceptors::context::BeforeTransmitInterceptorContextMut;
-    /// use aws_smithy_runtime_api::client::interceptors::Intercept;
-    /// use aws_smithy_runtime_api::client::runtime_components::RuntimeComponents;
-    /// use aws_smithy_types::config_bag::ConfigBag;
-    /// use aws_sdk_lambda::config::Config;
-    /// use ::http::uri::Uri;
-    ///
-    /// fn base_url() -> String {
-    ///     // ...
-    ///     # String::new()
-    /// }
-    ///
-    /// #[derive(Debug)]
-    /// pub struct UriModifierInterceptor;
-    /// impl Intercept for UriModifierInterceptor {
-    ///     fn name(&self) -> &'static str {
-    ///         "UriModifierInterceptor"
-    ///     }
-    ///     fn modify_before_signing(
-    ///         &self,
-    ///         context: &mut BeforeTransmitInterceptorContextMut<'_>,
-    ///         _runtime_components: &RuntimeComponents,
-    ///         _cfg: &mut ConfigBag,
-    ///     ) -> Result<(), BoxError> {
-    ///         let request = context.request_mut();
-    ///         let uri = format!("{}{}", base_url(), request.uri());
-    ///         *request.uri_mut() = uri.parse::<Uri>()?.into();
-    ///
-    ///         Ok(())
-    ///     }
-    /// }
-    ///
-    /// let config = Config::builder()
-    ///     .interceptor(UriModifierInterceptor)
-    ///     .build();
-    /// # }
-    /// ```
-    pub fn interceptor(mut self, interceptor: impl crate::config::Intercept + 'static) -> Self {
-        self.push_interceptor(crate::config::SharedInterceptor::new(interceptor));
-        self
-    }
-
-    /// Like [`Self::interceptor`], but takes a [`SharedInterceptor`](crate::config::SharedInterceptor).
-    pub fn push_interceptor(&mut self, interceptor: crate::config::SharedInterceptor) -> &mut Self {
-        self.runtime_components.push_interceptor(interceptor);
-        self
-    }
-
-    /// Set [`SharedInterceptor`](crate::config::SharedInterceptor)s for the builder.
-    pub fn set_interceptors(&mut self, interceptors: impl IntoIterator<Item = crate::config::SharedInterceptor>) -> &mut Self {
-        self.runtime_components.set_interceptors(interceptors.into_iter());
-        self
-    }
-    /// Sets the time source used for this service
-    pub fn time_source(mut self, time_source: impl ::aws_smithy_async::time::TimeSource + 'static) -> Self {
-        self.set_time_source(::std::option::Option::Some(::aws_smithy_runtime_api::shared::IntoShared::into_shared(
-            time_source,
-        )));
-        self
-    }
-    /// Sets the time source used for this service
-    pub fn set_time_source(&mut self, time_source: ::std::option::Option<::aws_smithy_async::time::SharedTimeSource>) -> &mut Self {
-        self.runtime_components.set_time_source(time_source);
-        self
-    }
-    /// Add type implementing [`ClassifyRetry`](::aws_smithy_runtime_api::client::retries::classifiers::ClassifyRetry) that will be used by the
-    /// [`RetryStrategy`](::aws_smithy_runtime_api::client::retries::RetryStrategy) to determine what responses should be retried.
-    ///
-    /// A retry classifier configured by this method will run according to its [priority](::aws_smithy_runtime_api::client::retries::classifiers::RetryClassifierPriority).
-    ///
-    /// # Examples
-    /// ```no_run
-    /// # fn example() {
-    /// use aws_smithy_runtime_api::client::interceptors::context::InterceptorContext;
-    /// use aws_smithy_runtime_api::client::orchestrator::OrchestratorError;
-    /// use aws_smithy_runtime_api::client::retries::classifiers::{
-    ///     ClassifyRetry, RetryAction, RetryClassifierPriority,
-    /// };
-    /// use aws_smithy_types::error::metadata::ProvideErrorMetadata;
-    /// use aws_smithy_types::retry::ErrorKind;
-    /// use std::error::Error as StdError;
-    /// use std::marker::PhantomData;
-    /// use std::fmt;
-    /// use aws_sdk_lambda::config::Config;
-    /// # #[derive(Debug)]
-    /// # struct SomeOperationError {}
-    /// # impl StdError for SomeOperationError {}
-    /// # impl fmt::Display for SomeOperationError {
-    /// #    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { todo!() }
-    /// # }
-    /// # impl ProvideErrorMetadata for SomeOperationError {
-    /// #    fn meta(&self) -> &aws_sdk_lambda::error::ErrorMetadata { todo!() }
-    /// # }
-    ///
-    /// const RETRYABLE_ERROR_CODES: &[&str] = &[
-    ///     // List error codes to be retried here...
-    /// ];
-    ///
-    /// // When classifying at an operation's error type, classifiers require a generic parameter.
-    /// // When classifying the HTTP response alone, no generic is needed.
-    /// #[derive(Debug, Default)]
-    /// pub struct ExampleErrorCodeClassifier<E> {
-    ///     _inner: PhantomData<E>,
-    /// }
-    ///
-    /// impl<E> ExampleErrorCodeClassifier<E> {
-    ///     pub fn new() -> Self {
-    ///         Self {
-    ///             _inner: PhantomData,
-    ///         }
-    ///     }
-    /// }
-    ///
-    /// impl<E> ClassifyRetry for ExampleErrorCodeClassifier<E>
-    /// where
-    ///     // Adding a trait bound for ProvideErrorMetadata allows us to inspect the error code.
-    ///     E: StdError + ProvideErrorMetadata + Send + Sync + 'static,
-    /// {
-    ///     fn classify_retry(&self, ctx: &InterceptorContext) -> RetryAction {
-    ///         // Check for a result
-    ///         let output_or_error = ctx.output_or_error();
-    ///         // Check for an error
-    ///         let error = match output_or_error {
-    ///             Some(Ok(_)) | None => return RetryAction::NoActionIndicated,
-    ///               Some(Err(err)) => err,
-    ///         };
-    ///
-    ///         // Downcast the generic error and extract the code
-    ///         let error_code = OrchestratorError::as_operation_error(error)
-    ///             .and_then(|err| err.downcast_ref::<E>())
-    ///             .and_then(|err| err.code());
-    ///
-    ///         // If this error's code is in our list, return an action that tells the RetryStrategy to retry this request.
-    ///         if let Some(error_code) = error_code {
-    ///             if RETRYABLE_ERROR_CODES.contains(&error_code) {
-    ///                 return RetryAction::transient_error();
-    ///             }
-    ///         }
-    ///
-    ///         // Otherwise, return that no action is indicated i.e. that this classifier doesn't require a retry.
-    ///         // Another classifier may still classify this response as retryable.
-    ///         RetryAction::NoActionIndicated
-    ///     }
-    ///
-    ///     fn name(&self) -> &'static str { "Example Error Code Classifier" }
-    /// }
-    ///
-    /// let config = Config::builder()
-    ///     .retry_classifier(ExampleErrorCodeClassifier::<SomeOperationError>::new())
-    ///     .build();
-    /// # }
-    /// ```
-    pub fn retry_classifier(
-        mut self,
-        retry_classifier: impl ::aws_smithy_runtime_api::client::retries::classifiers::ClassifyRetry + 'static,
-    ) -> Self {
-        self.push_retry_classifier(::aws_smithy_runtime_api::client::retries::classifiers::SharedRetryClassifier::new(
-            retry_classifier,
-        ));
-        self
-    }
-
-    /// Like [`Self::retry_classifier`], but takes a [`SharedRetryClassifier`](::aws_smithy_runtime_api::client::retries::classifiers::SharedRetryClassifier).
-    pub fn push_retry_classifier(
-        &mut self,
-        retry_classifier: ::aws_smithy_runtime_api::client::retries::classifiers::SharedRetryClassifier,
-    ) -> &mut Self {
-        self.runtime_components.push_retry_classifier(retry_classifier);
-        self
-    }
-
-    /// Set [`SharedRetryClassifier`](::aws_smithy_runtime_api::client::retries::classifiers::SharedRetryClassifier)s for the builder, replacing any that
-    /// were previously set.
-    pub fn set_retry_classifiers(
-        &mut self,
-        retry_classifiers: impl IntoIterator<Item = ::aws_smithy_runtime_api::client::retries::classifiers::SharedRetryClassifier>,
-    ) -> &mut Self {
-        self.runtime_components.set_retry_classifiers(retry_classifiers.into_iter());
-        self
-    }
-    /// Sets the name of the app that is using the client.
-    ///
-    /// This _optional_ name is used to identify the application in the user agent that
-    /// gets sent along with requests.
-    pub fn app_name(mut self, app_name: ::aws_types::app_name::AppName) -> Self {
-        self.set_app_name(Some(app_name));
-        self
-    }
-    /// Sets the name of the app that is using the client.
-    ///
-    /// This _optional_ name is used to identify the application in the user agent that
-    /// gets sent along with requests.
-    pub fn set_app_name(&mut self, app_name: ::std::option::Option<::aws_types::app_name::AppName>) -> &mut Self {
-        self.config.store_or_unset(app_name);
-        self
-    }
-    /// Appends framework metadata to the user agent.
-    ///
-    /// This _optional_ metadata identifies a software framework or third-party library
-    /// that is being used with the client. It is rendered into the user agent string
-    /// (as `lib/{name}/{version}`) so that libraries built on top of the AWS SDK can
-    /// self-identify in the requests they make. Multiple entries may be added; each call
-    /// appends another entry rather than replacing previous ones.
-    ///
-    /// Entries are de-duplicated on `(name, version)`, rendered in first-seen order, and
-    /// the total number of unique entries included in the user agent is capped (currently
-    /// at 10); additional entries beyond the cap are dropped with a warning.
-    pub fn framework_metadata(mut self, framework_metadata: ::aws_types::sdk_ua_metadata::FrameworkMetadata) -> Self {
-        self.push_framework_metadata(framework_metadata);
-        self
-    }
-    /// Appends framework metadata to the user agent.
-    ///
-    /// This _optional_ metadata identifies a software framework or third-party library
-    /// that is being used with the client. It is rendered into the user agent string
-    /// (as `lib/{name}/{version}`) so that libraries built on top of the AWS SDK can
-    /// self-identify in the requests they make. Multiple entries may be added; each call
-    /// appends another entry rather than replacing previous ones.
-    pub fn push_framework_metadata(&mut self, framework_metadata: ::aws_types::sdk_ua_metadata::FrameworkMetadata) -> &mut Self {
-        self.config.store_append(framework_metadata);
-        self
-    }
-    /// Overrides the default invocation ID generator.
-    ///
-    /// The invocation ID generator generates ID values for the `amz-sdk-invocation-id` header. By default, this will be a random UUID. Overriding it may be useful in tests that examine the HTTP request and need to be deterministic.
-    pub fn invocation_id_generator(mut self, gen: impl ::aws_runtime::invocation_id::InvocationIdGenerator + 'static) -> Self {
-        self.set_invocation_id_generator(::std::option::Option::Some(
-            ::aws_runtime::invocation_id::SharedInvocationIdGenerator::new(gen),
-        ));
-        self
-    }
-    /// Overrides the default invocation ID generator.
-    ///
-    /// The invocation ID generator generates ID values for the `amz-sdk-invocation-id` header. By default, this will be a random UUID. Overriding it may be useful in tests that examine the HTTP request and need to be deterministic.
-    pub fn set_invocation_id_generator(
-        &mut self,
-        gen: ::std::option::Option<::aws_runtime::invocation_id::SharedInvocationIdGenerator>,
-    ) -> &mut Self {
-        self.config.store_or_unset(gen);
-        self
-    }
-    /// Sets the endpoint URL used to communicate with this service.
-    ///
-    /// Note: this is used in combination with other endpoint rules, e.g. an API that applies a host-label prefix
-    /// will be prefixed onto this URL. To fully override the endpoint resolver, use
-    /// [`Builder::endpoint_resolver`].
-    pub fn endpoint_url(mut self, endpoint_url: impl Into<::std::string::String>) -> Self {
-        self.set_endpoint_url(Some(endpoint_url.into()));
-        self
-    }
-    /// Sets the endpoint URL used to communicate with this service.
-    ///
-    /// Note: this is used in combination with other endpoint rules, e.g. an API that applies a host-label prefix
-    /// will be prefixed onto this URL. To fully override the endpoint resolver, use
-    /// [`Builder::endpoint_resolver`].
-    pub fn set_endpoint_url(&mut self, endpoint_url: Option<::std::string::String>) -> &mut Self {
-        self.config.store_or_unset(endpoint_url.map(::aws_types::endpoint_config::EndpointUrl));
-        self
-    }
-    /// When true, use the dual-stack endpoint. If the configured endpoint does not support dual-stack, dispatching the request MAY return an error.
-    pub fn use_dual_stack(mut self, use_dual_stack: impl Into<bool>) -> Self {
-        self.set_use_dual_stack(Some(use_dual_stack.into()));
-        self
-    }
-    /// When true, use the dual-stack endpoint. If the configured endpoint does not support dual-stack, dispatching the request MAY return an error.
-    pub fn set_use_dual_stack(&mut self, use_dual_stack: Option<bool>) -> &mut Self {
-        self.config.store_or_unset(use_dual_stack.map(::aws_types::endpoint_config::UseDualStack));
-        self
-    }
-    /// When true, send this request to the FIPS-compliant regional endpoint. If the configured endpoint does not have a FIPS compliant endpoint, dispatching the request will return an error.
-    pub fn use_fips(mut self, use_fips: impl Into<bool>) -> Self {
-        self.set_use_fips(Some(use_fips.into()));
-        self
-    }
-    /// When true, send this request to the FIPS-compliant regional endpoint. If the configured endpoint does not have a FIPS compliant endpoint, dispatching the request will return an error.
-    pub fn set_use_fips(&mut self, use_fips: Option<bool>) -> &mut Self {
-        self.config.store_or_unset(use_fips.map(::aws_types::endpoint_config::UseFips));
-        self
-    }
-    /// Sets the AWS region to use when making requests.
-    ///
-    /// # Examples
-    /// ```no_run
-    /// use aws_types::region::Region;
-    /// use aws_sdk_lambda::config::{Builder, Config};
-    ///
-    /// let config = aws_sdk_lambda::Config::builder()
-    ///     .region(Region::new("us-east-1"))
-    ///     .build();
-    /// ```
-    pub fn region(mut self, region: impl ::std::convert::Into<::std::option::Option<crate::config::Region>>) -> Self {
-        self.set_region(region.into());
-        self
-    }
-    /// Sets the AWS region to use when making requests.
-    pub fn set_region(&mut self, region: ::std::option::Option<crate::config::Region>) -> &mut Self {
-        self.config.store_or_unset(region);
-        self
-    }
-    /// Sets the credentials provider for this service
-    pub fn credentials_provider(mut self, credentials_provider: impl crate::config::ProvideCredentials + 'static) -> Self {
-        self.set_credentials_provider(::std::option::Option::Some(crate::config::SharedCredentialsProvider::new(
-            credentials_provider,
-        )));
-        self
-    }
-    /// Sets the credentials provider for this service
-    pub fn set_credentials_provider(&mut self, credentials_provider: ::std::option::Option<crate::config::SharedCredentialsProvider>) -> &mut Self {
-        if let Some(credentials_provider) = credentials_provider {
-            self.runtime_components
-                .set_identity_resolver(::aws_runtime::auth::sigv4::SCHEME_ID, credentials_provider);
+        pub fn build(self) -> super::Config {
+            super::Config {
+                endpoint_url: self.endpoint_url.unwrap_or_else(|| super::Config::default().endpoint_url),
+            }
         }
-        self
-    }
-    /// Sets the [`behavior major version`](crate::config::BehaviorVersion).
-    ///
-    /// Over time, new best-practice behaviors are introduced. However, these behaviors might not be backwards
-    /// compatible. For example, a change which introduces new default timeouts or a new retry-mode for
-    /// all operations might be the ideal behavior but could break existing applications.
-    ///
-    /// # Examples
-    ///
-    /// Set the behavior major version to `latest`. This is equivalent to enabling the `behavior-version-latest` cargo feature.
-    /// ```no_run
-    /// use aws_sdk_lambda::config::BehaviorVersion;
-    ///
-    /// let config = aws_sdk_lambda::Config::builder()
-    ///     .behavior_version(BehaviorVersion::latest())
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_lambda::Client::from_conf(config);
-    /// ```
-    ///
-    /// Customizing behavior major version:
-    /// ```no_run
-    /// use aws_sdk_lambda::config::BehaviorVersion;
-    ///
-    /// let config = aws_sdk_lambda::Config::builder()
-    ///     .behavior_version(BehaviorVersion::v2023_11_09())
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_lambda::Client::from_conf(config);
-    /// ```
-    ///
-    pub fn behavior_version(mut self, behavior_version: crate::config::BehaviorVersion) -> Self {
-        self.set_behavior_version(Some(behavior_version));
-        self
     }
-
-    /// Sets the [`behavior major version`](crate::config::BehaviorVersion).
-    ///
-    /// Over time, new best-practice behaviors are introduced. However, these behaviors might not be backwards
-    /// compatible. For example, a change which introduces new default timeouts or a new retry-mode for
-    /// all operations might be the ideal behavior but could break existing applications.
-    ///
-    /// # Examples
-    ///
-    /// Set the behavior major version to `latest`. This is equivalent to enabling the `behavior-version-latest` cargo feature.
-    /// ```no_run
-    /// use aws_sdk_lambda::config::BehaviorVersion;
-    ///
-    /// let config = aws_sdk_lambda::Config::builder()
-    ///     .behavior_version(BehaviorVersion::latest())
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_lambda::Client::from_conf(config);
-    /// ```
-    ///
-    /// Customizing behavior major version:
-    /// ```no_run
-    /// use aws_sdk_lambda::config::BehaviorVersion;
-    ///
-    /// let config = aws_sdk_lambda::Config::builder()
-    ///     .behavior_version(BehaviorVersion::v2023_11_09())
-    ///     // ...
-    ///     .build();
-    /// let client = aws_sdk_lambda::Client::from_conf(config);
-    /// ```
-    ///
-    pub fn set_behavior_version(&mut self, behavior_version: Option<crate::config::BehaviorVersion>) -> &mut Self {
-        self.behavior_version = behavior_version;
-        self
-    }
-
-    /// Convenience method to set the latest behavior major version
-    ///
-    /// This is equivalent to enabling the `behavior-version-latest` Cargo feature
-    pub fn behavior_version_latest(mut self) -> Self {
-        self.set_behavior_version(Some(crate::config::BehaviorVersion::latest()));
-        self
-    }
-    /// Adds a runtime plugin to the config.
-    #[allow(unused)]
-    pub(crate) fn runtime_plugin(mut self, plugin: impl crate::config::RuntimePlugin + 'static) -> Self {
-        self.push_runtime_plugin(crate::config::SharedRuntimePlugin::new(plugin));
-        self
-    }
-    /// Adds a runtime plugin to the config.
-    #[allow(unused)]
-    pub(crate) fn push_runtime_plugin(&mut self, plugin: crate::config::SharedRuntimePlugin) -> &mut Self {
-        self.runtime_plugins.push(plugin);
-        self
-    }
-    #[cfg(any(feature = "test-util", test))]
-    #[allow(unused_mut)]
-    /// Apply test defaults to the builder. NOTE: Consider migrating to use `apply_test_defaults_v2` instead.
-    pub fn apply_test_defaults(&mut self) -> &mut Self {
-        self.set_idempotency_token_provider(Some("00000000-0000-4000-8000-000000000000".into()));
-        self.set_time_source(::std::option::Option::Some(::aws_smithy_async::time::SharedTimeSource::new(
-            ::aws_smithy_async::time::StaticTimeSource::new(::std::time::UNIX_EPOCH + ::std::time::Duration::from_secs(1234567890)),
-        )));
-        self.config.store_put(::aws_runtime::user_agent::AwsUserAgent::for_tests());
-        self.set_credentials_provider(Some(crate::config::SharedCredentialsProvider::new(
-            ::aws_credential_types::Credentials::for_tests(),
-        )));
-        self.behavior_version = ::std::option::Option::Some(crate::config::BehaviorVersion::latest());
-        self
-    }
-    #[cfg(any(feature = "test-util", test))]
-    #[allow(unused_mut)]
-    /// Apply test defaults to the builder. NOTE: Consider migrating to use `with_test_defaults_v2` instead.
-    pub fn with_test_defaults(mut self) -> Self {
-        self.apply_test_defaults();
-        self
-    }
-    #[cfg(any(feature = "test-util", test))]
-    #[allow(unused_mut)]
-    /// Apply test defaults to the builder. V2 of this function sets additional test defaults such as region configuration (if applicable).
-    pub fn apply_test_defaults_v2(&mut self) -> &mut Self {
-        self.apply_test_defaults();
-        if self.config.load::<crate::config::Region>().is_none() {
-            self.set_region(::std::option::Option::Some(crate::config::Region::new("us-east-1")));
-        }
-        self
-    }
-    #[cfg(any(feature = "test-util", test))]
-    #[allow(unused_mut)]
-    /// Apply test defaults to the builder. V2 of this function sets additional test defaults such as region configuration (if applicable).
-    pub fn with_test_defaults_v2(mut self) -> Self {
-        self.apply_test_defaults_v2();
-        self
-    }
-    /// Builds a [`Config`].
-    #[allow(unused_mut)]
-    pub fn build(mut self) -> Config {
-        let mut layer = self.config;
-        if self.runtime_components.time_source().is_none() {
-            self.runtime_components
-                .set_time_source(::std::option::Option::Some(::std::default::Default::default()));
-        }
-        layer.store_put(crate::meta::API_METADATA.clone());
-        layer.store_put(::aws_types::SigningName::from_static("lambda"));
-        layer
-            .load::<::aws_types::region::Region>()
-            .cloned()
-            .map(|r| layer.store_put(::aws_types::region::SigningRegion::from(r)));
-        Config {
-            config: crate::config::Layer::from(layer.clone())
-                .with_name("aws_sdk_lambda::config::Config")
-                .freeze(),
-            cloneable: layer,
-            runtime_components: self.runtime_components,
-            runtime_plugins: self.runtime_plugins,
-            behavior_version: self.behavior_version,
-        }
-    }
-}
-#[derive(::std::fmt::Debug)]
-pub(crate) struct ServiceRuntimePlugin {
-    config: ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer>,
-    runtime_components: ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
-}
-
-impl ServiceRuntimePlugin {
-    pub fn new(_service_config: crate::config::Config) -> Self {
-        let config = {
-            let mut cfg = ::aws_smithy_types::config_bag::Layer::new("AWSGirApiService");
-            cfg.store_put(crate::idempotency_token::default_provider());
-            cfg.store_put(::aws_smithy_runtime::client::orchestrator::AuthSchemeAndEndpointOrchestrationV2);
-            ::std::option::Option::Some(cfg.freeze())
-        };
-        let mut runtime_components = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ServiceRuntimePlugin");
-        runtime_components.set_auth_scheme_option_resolver(::std::option::Option::Some({
-            use crate::config::auth::ResolveAuthScheme;
-            crate::config::auth::DefaultAuthSchemeResolver::default().into_shared_resolver()
-        }));
-        runtime_components.set_endpoint_resolver(::std::option::Option::Some({
-            use crate::config::endpoint::ResolveEndpoint;
-            crate::config::endpoint::DefaultResolver::new().into_shared_resolver()
-        }));
-        runtime_components.push_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-            ::aws_smithy_runtime::client::http::connection_poisoning::ConnectionPoisoningInterceptor::new(),
-        ));
-        runtime_components.push_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::HttpStatusCodeClassifier::default());
-        runtime_components.push_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-            crate::sdk_feature_tracker::retry_mode::RetryModeFeatureTrackerInterceptor::new(),
-        ));
-        runtime_components.push_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-            ::aws_runtime::service_clock_skew::ServiceClockSkewInterceptor::new(),
-        ));
-        runtime_components.push_interceptor(::aws_runtime::request_info::RequestInfoInterceptor::new());
-        runtime_components.push_interceptor(::aws_runtime::user_agent::UserAgentInterceptor::new());
-        runtime_components.push_interceptor(::aws_runtime::invocation_id::InvocationIdInterceptor::new());
-        runtime_components.push_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-            ::aws_runtime::recursion_detection::RecursionDetectionInterceptor::new(),
-        ));
-        runtime_components.push_auth_scheme(::aws_smithy_runtime_api::client::auth::SharedAuthScheme::new(
-            ::aws_runtime::auth::sigv4::SigV4AuthScheme::new(),
-        ));
-        runtime_components.push_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-            crate::config::endpoint::EndpointOverrideFeatureTrackerInterceptor,
-        ));
-        runtime_components.push_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-            crate::observability_feature::ObservabilityFeatureTrackerInterceptor,
-        ));
-        Self { config, runtime_components }
-    }
-}
-
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for ServiceRuntimePlugin {
-    fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
-        self.config.clone()
-    }
-
-    fn order(&self) -> ::aws_smithy_runtime_api::client::runtime_plugin::Order {
-        ::aws_smithy_runtime_api::client::runtime_plugin::Order::Defaults
-    }
-
-    fn runtime_components(
-        &self,
-        _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
-    ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
-        ::std::borrow::Cow::Borrowed(&self.runtime_components)
-    }
-}
-
-// Cross-operation shared-state singletons
-
-/// A plugin that enables configuration for a single operation invocation
-///
-/// The `config` method will return a `FrozenLayer` by storing values from `config_override`.
-/// In the case of default values requested, they will be obtained from `client_config`.
-#[derive(Debug)]
-pub(crate) struct ConfigOverrideRuntimePlugin {
-    pub(crate) config: ::aws_smithy_types::config_bag::FrozenLayer,
-    pub(crate) components: ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
-}
-
-impl ConfigOverrideRuntimePlugin {
-    #[allow(dead_code)] // unused when a service does not provide any operations
-    pub(crate) fn new(
-        config_override: Builder,
-        initial_config: ::aws_smithy_types::config_bag::FrozenLayer,
-        initial_components: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
-    ) -> Self {
-        let mut layer = config_override.config;
-        let mut components = config_override.runtime_components;
-        #[allow(unused_mut)]
-        let mut resolver =
-            ::aws_smithy_runtime::client::config_override::Resolver::overrid(initial_config, initial_components, &mut layer, &mut components);
-
-        resolver
-            .config_mut()
-            .load::<::aws_types::region::Region>()
-            .cloned()
-            .map(|r| resolver.config_mut().store_put(::aws_types::region::SigningRegion::from(r)));
-
-        let _ = resolver;
-
-        // When the config override supplies an identity resolver for any auth scheme
-        // known to the client or the override itself, we give this operation its own
-        // short-lived identity cache so that new partitions don't accumulate in the
-        // shared client cache. A lazy cache (not `no_cache`) is used so that resolved
-        // identities are served from the short-lived identity cache on retries.
-        //
-        // This is skipped if the override already sets its own identity cache.
-        if components.has_identity_resolvers() && components.identity_cache().is_none() {
-            components.set_identity_cache(::std::option::Option::Some(
-                ::aws_smithy_runtime::client::identity::IdentityCache::lazy().max_partitions(1).build(),
-            ));
-        }
-
-        Self {
-            config: ::aws_smithy_types::config_bag::Layer::from(layer)
-                .with_name("aws_sdk_lambda::config::ConfigOverrideRuntimePlugin")
-                .freeze(),
-            components,
-        }
-    }
-}
-
-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for ConfigOverrideRuntimePlugin {
-    fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
-        Some(self.config.clone())
-    }
-
-    fn runtime_components(
-        &self,
-        _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
-    ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
-        ::std::borrow::Cow::Borrowed(&self.components)
-    }
-}
-
-pub use ::aws_smithy_runtime::client::identity::IdentityCache;
-pub use ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponents;
-pub use ::aws_smithy_types::config_bag::ConfigBag;
-
-pub use ::aws_credential_types::Credentials;
-
-impl From<&::aws_types::sdk_config::SdkConfig> for Builder {
-    fn from(input: &::aws_types::sdk_config::SdkConfig) -> Self {
-        let mut builder = Builder::default();
-        builder.set_credentials_provider(input.credentials_provider());
-        builder = builder.region(input.region().cloned());
-        builder.set_use_fips(input.use_fips());
-        builder.set_use_dual_stack(input.use_dual_stack());
-        if input.get_origin("endpoint_url").is_client_config() {
-            builder.set_endpoint_url(input.endpoint_url().map(|s| s.to_string()));
-        } else {
-            builder.set_endpoint_url(
-                input
-                    .service_config()
-                    .and_then(|conf| {
-                        conf.load_config(service_config_key("Lambda", "AWS_ENDPOINT_URL", "endpoint_url"))
-                            .map(|it| it.parse().unwrap())
-                    })
-                    .or_else(|| input.endpoint_url().map(|s| s.to_string())),
-            );
-        }
-        // resiliency
-        builder.set_retry_config(input.retry_config().cloned());
-        builder.set_timeout_config(input.timeout_config().cloned());
-        builder.set_sleep_impl(input.sleep_impl());
-
-        builder.set_http_client(input.http_client());
-        builder.set_time_source(input.time_source());
-        builder.set_behavior_version(input.behavior_version());
-        builder.set_auth_scheme_preference(input.auth_scheme_preference().cloned());
-        // setting `None` here removes the default
-        if let Some(config) = input.stalled_stream_protection() {
-            builder.set_stalled_stream_protection(Some(config));
-        }
-
-        if let Some(cache) = input.identity_cache() {
-            builder.set_identity_cache(cache);
+    impl From<&super::Config> for Builder {
+        fn from(config: &super::Config) -> Self {
+            Self {
+                endpoint_url: Some(config.endpoint_url.clone()),
+            }
         }
-        builder.set_app_name(input.app_name().cloned());
-        for framework_metadata in input.framework_metadata() {
-            builder.push_framework_metadata(framework_metadata.clone());
-        }
-
-        builder
-    }
-}
-
-impl From<&::aws_types::sdk_config::SdkConfig> for Config {
-    fn from(sdk_config: &::aws_types::sdk_config::SdkConfig) -> Self {
-        Builder::from(sdk_config).build()
     }
 }

-pub use ::aws_types::app_name::AppName;
-pub use ::aws_types::sdk_ua_metadata::FrameworkMetadata;
-
-#[allow(dead_code)]
-fn service_config_key<'a>(service_id: &'a str, env: &'a str, profile: &'a str) -> aws_types::service_config::ServiceConfigKey<'a> {
-    ::aws_types::service_config::ServiceConfigKey::builder()
-        .service_id(service_id)
-        .env(env)
-        .profile(profile)
-        .build()
-        .expect("all field sets explicitly, can't fail")
-}
-
-pub use ::aws_smithy_async::rt::sleep::Sleep;
-
-pub(crate) fn base_client_runtime_plugins(mut config: crate::Config) -> ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins {
-    let mut configured_plugins = ::std::vec::Vec::new();
-    ::std::mem::swap(&mut config.runtime_plugins, &mut configured_plugins);
-    #[cfg(feature = "behavior-version-latest")]
-    {
-        if config.behavior_version.is_none() {
-            config.behavior_version = Some(::aws_smithy_runtime_api::client::behavior_version::BehaviorVersion::latest());
-        }
-    }
-
-    let default_retry_partition = "lambda";
-    let default_retry_partition = match config.region() {
-        Some(region) => ::std::borrow::Cow::from(format!("{default_retry_partition}-{region}")),
-        None => ::std::borrow::Cow::from(default_retry_partition),
-    };
-
-    let scope = "aws-sdk-lambda";
-
-    #[allow(deprecated)]
-                    let mut plugins = ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins::new()
-                        // defaults
-                        .with_client_plugins(::aws_smithy_runtime::client::defaults::default_plugins(
-                            ::aws_smithy_runtime::client::defaults::DefaultPluginParams::new()
-                                .with_retry_partition_name(default_retry_partition)
-                                .with_behavior_version(config.behavior_version.expect("Invalid client configuration: A behavior major version must be set when sending a request or constructing a client. You must set it during client construction or by enabling the `behavior-version-latest` cargo feature."))
-                                .with_is_aws_sdk(true)
-                        ))
-                        // user config
-                        .with_client_plugin(
-                            ::aws_smithy_runtime_api::client::runtime_plugin::StaticRuntimePlugin::new()
-                                .with_config(config.config.clone())
-                                .with_runtime_components(config.runtime_components.clone())
-                        )
-                        // codegen config
-                        .with_client_plugin(crate::config::ServiceRuntimePlugin::new(config.clone()))
-                        .with_client_plugin(::aws_smithy_runtime::client::auth::no_auth::NoAuthRuntimePlugin::new())
-                        .with_client_plugin(
-                            ::aws_smithy_runtime::client::metrics::MetricsRuntimePlugin::builder()
-                                .with_scope(scope)
-                                .with_time_source(config.runtime_components.time_source().unwrap_or_default())
-                                .build()
-                                .expect("All required fields have been set")
-                        );
-
-    for plugin in configured_plugins {
-        plugins = plugins.with_client_plugin(plugin);
+impl Config {
+    pub fn builder() -> config::Builder {
+        config::Builder::default()
     }
-    plugins
 }
-
-pub use ::aws_smithy_types::config_bag::FrozenLayer;
-
-pub use ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder;
-
-pub use ::aws_smithy_runtime_api::client::runtime_plugin::SharedRuntimePlugin;
-
-pub use ::aws_smithy_runtime_api::client::behavior_version::BehaviorVersion;
-
-pub use ::aws_smithy_runtime_api::client::stalled_stream_protection::StalledStreamProtectionConfig;
-
-pub use ::aws_smithy_runtime_api::client::http::SharedHttpClient;
-
-pub use ::aws_smithy_async::rt::sleep::SharedAsyncSleep;
-
-pub use ::aws_smithy_runtime_api::client::identity::SharedIdentityCache;
-
-pub use ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor;
-
-pub use ::aws_types::region::Region;
-
-pub use ::aws_credential_types::provider::SharedCredentialsProvider;
-
-pub use ::aws_smithy_runtime_api::client::http::HttpClient;
-
-pub use ::aws_smithy_runtime_api::shared::IntoShared;
-
-pub use ::aws_smithy_async::rt::sleep::AsyncSleep;
-
-pub use ::aws_smithy_runtime_api::client::identity::ResolveCachedIdentity;
-
-pub use ::aws_smithy_runtime_api::client::interceptors::Intercept;
-
-pub use ::aws_credential_types::provider::ProvideCredentials;
-
-pub use ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin;
-
-pub use ::aws_smithy_types::config_bag::Layer;
-
-/// Types needed to configure endpoint resolution.
-pub mod endpoint;
-
-/// HTTP request and response types.
-pub mod http;
-
-/// Types needed to implement [`Intercept`](crate::config::Intercept).
-pub mod interceptors;
-
-/// Retry configuration.
-///
-/// [`RetryConfig`](crate::config::retry::RetryConfig) sets the number of retry attempts and the backoff between them. Retries are additionally bounded by a retry token bucket (a shared retry quota): [`TokenBucket`](crate::config::retry::TokenBucket) holds the tokens and [`RetryPartition`](crate::config::retry::RetryPartition) determines which clients and operations share one. To size the token bucket or give a workload its own, use [`Builder::retry_partition`](crate::config::Builder::retry_partition).
-pub mod retry;
-
-/// Timeout configuration.
-pub mod timeout;
-
-/// Types needed to configure auth scheme resolution.
-pub mod auth;
```

### `src/lib.rs`

```diff
--- reference/src/lib.rs
+++ generated/src/lib.rs
@@ -1,261 +1,22 @@
-#![allow(deprecated)]
-#![allow(unknown_lints)]
-#![allow(clippy::module_inception)]
-#![allow(clippy::upper_case_acronyms)]
-#![allow(clippy::large_enum_variant)]
-#![allow(clippy::wrong_self_convention)]
-#![allow(clippy::should_implement_trait)]
-#![allow(clippy::disallowed_names)]
-#![allow(clippy::vec_init_then_push)]
-#![allow(clippy::type_complexity)]
-#![allow(clippy::needless_return)]
-#![allow(clippy::derive_partial_eq_without_eq)]
-#![allow(clippy::result_large_err)]
-#![allow(clippy::unnecessary_map_on_constructor)]
-#![allow(clippy::useless_conversion)]
-#![allow(clippy::deprecated_semver)]
-#![allow(rustdoc::bare_urls)]
-#![allow(rustdoc::redundant_explicit_links)]
-#![allow(rustdoc::broken_intra_doc_links)]
-#![allow(rustdoc::invalid_html_tags)]
-#![forbid(unsafe_code)]
-#![warn(missing_docs)]
-#![cfg_attr(docsrs, feature(doc_cfg))]
-//! __Overview__
-//!
-//! Lambda is a compute service that lets you run code without provisioning or managing servers. Lambda runs your code on a high-availability compute infrastructure and performs all of the administration of the compute resources, including server and operating system maintenance, capacity provisioning and automatic scaling, code monitoring and logging. With Lambda, you can run code for virtually any type of application or backend service. For more information about the Lambda service, see [What is Lambda](https://docs.aws.amazon.com/lambda/latest/dg/welcome.html) in the __Lambda Developer Guide__.
-//!
-//! The _Lambda API Reference_ provides information about each of the API methods, including details about the parameters in each API request and response.
-//!
-//! You can use Software Development Kits (SDKs), Integrated Development Environment (IDE) Toolkits, and command line tools to access the API. For installation instructions, see [Tools for Amazon Web Services](http://aws.amazon.com/tools/).
-//!
-//! For a list of Region-specific endpoints that Lambda supports, see [Lambda endpoints and quotas](https://docs.aws.amazon.com/general/latest/gr/lambda-service.html) in the _Amazon Web Services General Reference._.
-//!
-//! When making the API calls, you will need to authenticate your request by providing a signature. Lambda supports signature version 4. For more information, see [Signature Version 4 signing process](https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html) in the _Amazon Web Services General Reference._.
-//!
-//! __CA certificates__
-//!
-//! Because Amazon Web Services SDKs use the CA certificates from your computer, changes to the certificates on the Amazon Web Services servers can cause connection failures when you attempt to use an SDK. You can prevent these failures by keeping your computer's CA certificates and operating system up-to-date. If you encounter this issue in a corporate environment and do not manage your own computer, you might need to ask an administrator to assist with the update process. The following list shows minimum operating system and Java versions:
-//!   - Microsoft Windows versions that have updates from January 2005 or later installed contain at least one of the required CAs in their trust list.
-//!   - Mac OS X 10.4 with Java for Mac OS X 10.4 Release 5 (February 2007), Mac OS X 10.5 (October 2007), and later versions contain at least one of the required CAs in their trust list.
-//!   - Red Hat Enterprise Linux 5 (March 2007), 6, and 7 and CentOS 5, 6, and 7 all contain at least one of the required CAs in their default trusted CA list.
-//!   - Java 1.4.2_12 (May 2006), 5 Update 2 (March 2005), and all later versions, including Java 6 (December 2006), 7, and 8, contain at least one of the required CAs in their default trusted CA list.
-//!
-//! When accessing the Lambda management console or Lambda API endpoints, whether through browsers or programmatically, you will need to ensure your client machines support any of the following CAs:
-//!   - Amazon Root CA 1
-//!   - Starfield Services Root Certificate Authority - G2
-//!   - Starfield Class 2 Certification Authority
-//!
-//! Root certificates from the first two authorities are available from [Amazon trust services](https://www.amazontrust.com/repository/), but keeping your computer up-to-date is the more straightforward solution. To learn more about ACM-provided certificates, see [Amazon Web Services Certificate Manager FAQs.](http://aws.amazon.com/certificate-manager/faqs/#certificates)
-//!
-//! ## Getting Started
-//!
-//! > Examples are available for many services and operations, check out the
-//! > [usage examples](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/rustv1).
-//!
-//! The SDK provides one crate per AWS service. You must add [Tokio](https://crates.io/crates/tokio)
-//! as a dependency within your Rust project to execute asynchronous code. To add `aws-sdk-lambda` to
-//! your project, add the following to your **Cargo.toml** file:
-//!
-//! ```toml
-//! [dependencies]
-//! aws-config = { version = "1.1.7", features = ["behavior-version-latest"] }
-//! aws-sdk-lambda = "1.140.0"
-//! tokio = { version = "1", features = ["full"] }
-//! ```
-//!
-//! Then in code, a client can be created with the following:
-//!
-//! ```rust,no_run
-//! use aws_sdk_lambda as lambda;
-//!
-//! #[::tokio::main]
-//! async fn main() -> Result<(), lambda::Error> {
-//!     let config = aws_config::load_from_env().await;
-//!     let client = aws_sdk_lambda::Client::new(&config);
-//!
-//!     // ... make some calls with the client
-//!
-//!     Ok(())
-//! }
-//! ```
-//!
-//! See the [client documentation](https://docs.rs/aws-sdk-lambda/latest/aws_sdk_lambda/client/struct.Client.html)
-//! for information on what calls can be made, and the inputs and outputs for each of those calls.
-//!
-//! ## Using the SDK
-//!
-//! Until the SDK is released, we will be adding information about using the SDK to the
-//! [Developer Guide](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/welcome.html). Feel free to suggest
-//! additional sections for the guide by opening an issue and describing what you are trying to do.
-//!
-//! ## Getting Help
-//!
-//! * [GitHub discussions](https://github.com/awslabs/aws-sdk-rust/discussions) - For ideas, RFCs & general questions
-//! * [GitHub issues](https://github.com/awslabs/aws-sdk-rust/issues/new/choose) - For bug reports & feature requests
-//! * [Generated Docs (latest version)](https://awslabs.github.io/aws-sdk-rust/)
-//! * [Usage examples](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/rustv1)
-//!
-//!
-//! # Crate Organization
-//!
-//! The entry point for most customers will be [`Client`], which exposes one method for each API
-//! offered by AWS Lambda. The return value of each of these methods is a "fluent builder",
-//! where the different inputs for that API are added by builder-style function call chaining,
-//! followed by calling `send()` to get a [`Future`](std::future::Future) that will result in
-//! either a successful output or a [`SdkError`](crate::error::SdkError).
-//!
-//! Some of these API inputs may be structs or enums to provide more complex structured information.
-//! These structs and enums live in [`types`](crate::types). There are some simpler types for
-//! representing data such as date times or binary blobs that live in [`primitives`](crate::primitives).
-//!
-//! All types required to configure a client via the [`Config`](crate::Config) struct live
-//! in [`config`](crate::config).
-//!
-//! The [`operation`](crate::operation) module has a submodule for every API, and in each submodule
-//! is the input, output, and error type for that API, as well as builders to construct each of those.
-//!
-//! There is a top-level [`Error`](crate::Error) type that encompasses all the errors that the
-//! client can return. Any other error type can be converted to this `Error` type via the
-//! [`From`](std::convert::From) trait.
-//!
-//! The other modules within this crate are not required for normal usage.
-
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-pub use error_meta::Error;

-#[doc(inline)]
-pub use config::Config;
-
-/// Client for calling AWS Lambda.
-/// ## Constructing a `Client`
-///
-/// A [`Config`] is required to construct a client. For most use cases, the [`aws-config`]
-/// crate should be used to automatically resolve this config using
-/// [`aws_config::load_from_env()`], since this will resolve an [`SdkConfig`] which can be shared
-/// across multiple different AWS SDK clients. This config resolution process can be customized
-/// by calling [`aws_config::from_env()`] instead, which returns a [`ConfigLoader`] that uses
-/// the [builder pattern] to customize the default config.
-///
-/// In the simplest case, creating a client looks as follows:
-/// ```rust,no_run
-/// # async fn wrapper() {
-/// let config = aws_config::load_from_env().await;
-/// let client = aws_sdk_lambda::Client::new(&config);
-/// # }
-/// ```
-///
-/// Occasionally, SDKs may have additional service-specific values that can be set on the [`Config`] that
-/// is absent from [`SdkConfig`], or slightly different settings for a specific client may be desired.
-/// The [`Builder`](crate::config::Builder) struct implements `From<&SdkConfig>`, so setting these specific settings can be
-/// done as follows:
-///
-/// ```rust,no_run
-/// # async fn wrapper() {
-/// let sdk_config = ::aws_config::load_from_env().await;
-/// let config = aws_sdk_lambda::config::Builder::from(&sdk_config)
-/// # /*
-///     .some_service_specific_setting("value")
-/// # */
-///     .build();
-/// # }
-/// ```
-///
-/// See the [`aws-config` docs] and [`Config`] for more information on customizing configuration.
-///
-/// _Note:_ Client construction is expensive due to connection thread pool initialization, and should
-/// be done once at application start-up.
-///
-/// [`Config`]: crate::Config
-/// [`ConfigLoader`]: https://docs.rs/aws-config/*/aws_config/struct.ConfigLoader.html
-/// [`SdkConfig`]: https://docs.rs/aws-config/*/aws_config/struct.SdkConfig.html
-/// [`aws-config` docs]: https://docs.rs/aws-config/*
-/// [`aws-config`]: https://crates.io/crates/aws-config
-/// [`aws_config::from_env()`]: https://docs.rs/aws-config/*/aws_config/fn.from_env.html
-/// [`aws_config::load_from_env()`]: https://docs.rs/aws-config/*/aws_config/fn.load_from_env.html
-/// [builder pattern]: https://rust-lang.github.io/api-guidelines/type-safety.html#builders-enable-construction-of-complex-values-c-builder
-/// # Using the `Client`
-///
-/// A client has a function for every operation that can be performed by the service.
-/// For example, the [`AddLayerVersionPermission`](crate::operation::add_layer_version_permission) operation has
-/// a [`Client::add_layer_version_permission`], function which returns a builder for that operation.
-/// The fluent builder ultimately has a `send()` function that returns an async future that
-/// returns a result, as illustrated below:
-///
-/// ```rust,ignore
-/// let result = client.add_layer_version_permission()
-///     .layer_name("example")
-///     .send()
-///     .await;
-/// ```
-///
-/// The underlying HTTP requests that get made by this can be modified with the `customize_operation`
-/// function on the fluent builder. See the [`customize`](crate::client::customize) module for more
-/// information.
-/// # Waiters
-///
-/// This client provides `wait_until` methods behind the [`Waiters`](crate::client::Waiters) trait.
-/// To use them, simply import the trait, and then call one of the `wait_until` methods. This will
-/// return a waiter fluent builder that takes various parameters, which are documented on the builder
-/// type. Once parameters have been provided, the `wait` method can be called to initiate waiting.
-///
-/// For example, if there was a `wait_until_thing` method, it could look like:
-/// ```rust,ignore
-/// let result = client.wait_until_thing()
-///     .thing_id("someId")
-///     .wait(Duration::from_secs(120))
-///     .await;
-/// ```
-pub mod client;
-
-/// Configuration for AWS Lambda.
-pub mod config;
+pub use error_meta::Error;

-/// Common errors and error handling utilities.
-pub mod error;
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/primitives.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/config.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/error.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/meta.rs"));
+pub mod types {
+    include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/types.rs"));
+}
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation.rs"));
+include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/client.rs"));

 mod error_meta;
-
-/// Information about this crate.
-pub mod meta;
-
-/// All operations that this crate can perform.
-pub mod operation;
-
-/// Primitives such as `Blob` or `DateTime` used by other types.
-pub mod primitives;
-
-/// Data structures used by operation inputs/outputs.
-pub mod types;
-
-pub(crate) mod client_idempotency_token;
-
-mod event_receiver;
-
-mod idempotency_token;
-
-mod observability_feature;
-
-pub(crate) mod protocol_serde;
-
-mod sdk_feature_tracker;
-
-mod serialization_settings;
-
-mod endpoint_lib;
-
-mod lens;
-
-mod serde_util;
-
-/// Supporting types for waiters.
-///
-/// Note: to use waiters, import the [`Waiters`](crate::client::Waiters) trait, which adds methods prefixed with `wait_until` to the client.
+mod serde_util {
+    include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/serde_util.rs"));
+}
+mod lens {
+    include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/lens.rs"));
+}
 pub mod waiters;
-
-mod event_stream_serde;
-
-mod json_errors;
-
-#[doc(inline)]
-pub use client::Client;
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

### `src/operation/checkpoint_durable_execution/_checkpoint_durable_execution_input.rs`

```diff
--- reference/src/operation/checkpoint_durable_execution/_checkpoint_durable_execution_input.rs
+++ generated/src/operation/checkpoint_durable_execution/_checkpoint_durable_execution_input.rs
@@ -1,7 +1,7 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 #[allow(missing_docs)] // documentation missing in model
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct CheckpointDurableExecutionInput {
     /// <p>The Amazon Resource Name (ARN) of the durable execution.</p>
     pub durable_execution_arn: ::std::option::Option<::std::string::String>,
@@ -32,6 +32,16 @@
         self.client_token.as_deref()
     }
 }
+impl ::std::fmt::Debug for CheckpointDurableExecutionInput {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("CheckpointDurableExecutionInput");
+        formatter.field("durable_execution_arn", &self.durable_execution_arn);
+        formatter.field("checkpoint_token", &self.checkpoint_token);
+        formatter.field("updates", &"*** Sensitive Data Redacted ***");
+        formatter.field("client_token", &self.client_token);
+        formatter.finish()
+    }
+}
 impl CheckpointDurableExecutionInput {
     /// Creates a new builder-style object to manufacture [`CheckpointDurableExecutionInput`](crate::operation::checkpoint_durable_execution::CheckpointDurableExecutionInput).
     pub fn builder() -> crate::operation::checkpoint_durable_execution::builders::CheckpointDurableExecutionInputBuilder {
@@ -40,7 +50,7 @@
 }

 /// A builder for [`CheckpointDurableExecutionInput`](crate::operation::checkpoint_durable_execution::CheckpointDurableExecutionInput).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
 #[non_exhaustive]
 pub struct CheckpointDurableExecutionInputBuilder {
     pub(crate) durable_execution_arn: ::std::option::Option<::std::string::String>,
@@ -128,3 +138,13 @@
         })
     }
 }
+impl ::std::fmt::Debug for CheckpointDurableExecutionInputBuilder {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("CheckpointDurableExecutionInputBuilder");
+        formatter.field("durable_execution_arn", &self.durable_execution_arn);
+        formatter.field("checkpoint_token", &self.checkpoint_token);
+        formatter.field("updates", &"*** Sensitive Data Redacted ***");
+        formatter.field("client_token", &self.client_token);
+        formatter.finish()
+    }
+}
```

### `src/operation/checkpoint_durable_execution/_checkpoint_durable_execution_output.rs`

```diff
--- reference/src/operation/checkpoint_durable_execution/_checkpoint_durable_execution_output.rs
+++ generated/src/operation/checkpoint_durable_execution/_checkpoint_durable_execution_output.rs
@@ -2,7 +2,7 @@

 /// <p>The response from the CheckpointDurableExecution operation.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct CheckpointDurableExecutionOutput {
     /// <p>A new checkpoint token to use for the next checkpoint operation. This token replaces the one provided in the request and must be used for subsequent checkpoints to maintain proper ordering.</p>
     pub checkpoint_token: ::std::option::Option<::std::string::String>,
@@ -20,6 +20,15 @@
         self.new_execution_state.as_ref()
     }
 }
+impl ::std::fmt::Debug for CheckpointDurableExecutionOutput {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("CheckpointDurableExecutionOutput");
+        formatter.field("checkpoint_token", &self.checkpoint_token);
+        formatter.field("new_execution_state", &"*** Sensitive Data Redacted ***");
+        formatter.field("_request_id", &self._request_id);
+        formatter.finish()
+    }
+}
 impl ::aws_types::request_id::RequestId for CheckpointDurableExecutionOutput {
     fn request_id(&self) -> Option<&str> {
         self._request_id.as_deref()
@@ -33,7 +42,7 @@
 }

 /// A builder for [`CheckpointDurableExecutionOutput`](crate::operation::checkpoint_durable_execution::CheckpointDurableExecutionOutput).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
 #[non_exhaustive]
 pub struct CheckpointDurableExecutionOutputBuilder {
     pub(crate) checkpoint_token: ::std::option::Option<::std::string::String>,
@@ -88,3 +97,12 @@
         }
     }
 }
+impl ::std::fmt::Debug for CheckpointDurableExecutionOutputBuilder {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("CheckpointDurableExecutionOutputBuilder");
+        formatter.field("checkpoint_token", &self.checkpoint_token);
+        formatter.field("new_execution_state", &"*** Sensitive Data Redacted ***");
+        formatter.field("_request_id", &self._request_id);
+        formatter.finish()
+    }
+}
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
@@ -1,7 +1,7 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 #[allow(missing_docs)] // documentation missing in model
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct CreateFunctionInput {
     /// <p>The name or ARN of the Lambda function.</p>
     /// <p class="title"><b>Name formats</b></p>
@@ -226,6 +226,40 @@
         self.durable_config.as_ref()
     }
 }
+impl ::std::fmt::Debug for CreateFunctionInput {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("CreateFunctionInput");
+        formatter.field("function_name", &self.function_name);
+        formatter.field("runtime", &self.runtime);
+        formatter.field("role", &self.role);
+        formatter.field("handler", &self.handler);
+        formatter.field("code", &"*** Sensitive Data Redacted ***");
+        formatter.field("description", &self.description);
+        formatter.field("timeout", &self.timeout);
+        formatter.field("memory_size", &self.memory_size);
+        formatter.field("publish", &self.publish);
+        formatter.field("publish_to", &self.publish_to);
+        formatter.field("vpc_config", &self.vpc_config);
+        formatter.field("package_type", &self.package_type);
+        formatter.field("dead_letter_config", &self.dead_letter_config);
+        formatter.field("environment", &"*** Sensitive Data Redacted ***");
+        formatter.field("kms_key_arn", &self.kms_key_arn);
+        formatter.field("tracing_config", &self.tracing_config);
+        formatter.field("tags", &self.tags);
+        formatter.field("layers", &self.layers);
+        formatter.field("file_system_configs", &self.file_system_configs);
+        formatter.field("code_signing_config_arn", &self.code_signing_config_arn);
+        formatter.field("image_config", &self.image_config);
+        formatter.field("architectures", &self.architectures);
+        formatter.field("ephemeral_storage", &self.ephemeral_storage);
+        formatter.field("snap_start", &self.snap_start);
+        formatter.field("logging_config", &self.logging_config);
+        formatter.field("tenancy_config", &self.tenancy_config);
+        formatter.field("capacity_provider_config", &self.capacity_provider_config);
+        formatter.field("durable_config", &self.durable_config);
+        formatter.finish()
+    }
+}
 impl CreateFunctionInput {
     /// Creates a new builder-style object to manufacture [`CreateFunctionInput`](crate::operation::create_function::CreateFunctionInput).
     pub fn builder() -> crate::operation::create_function::builders::CreateFunctionInputBuilder {
@@ -234,7 +268,7 @@
 }

 /// A builder for [`CreateFunctionInput`](crate::operation::create_function::CreateFunctionInput).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
 #[non_exhaustive]
 pub struct CreateFunctionInputBuilder {
     pub(crate) function_name: ::std::option::Option<::std::string::String>,
@@ -791,3 +825,37 @@
         })
     }
 }
+impl ::std::fmt::Debug for CreateFunctionInputBuilder {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("CreateFunctionInputBuilder");
+        formatter.field("function_name", &self.function_name);
+        formatter.field("runtime", &self.runtime);
+        formatter.field("role", &self.role);
+        formatter.field("handler", &self.handler);
+        formatter.field("code", &"*** Sensitive Data Redacted ***");
+        formatter.field("description", &self.description);
+        formatter.field("timeout", &self.timeout);
+        formatter.field("memory_size", &self.memory_size);
+        formatter.field("publish", &self.publish);
+        formatter.field("publish_to", &self.publish_to);
+        formatter.field("vpc_config", &self.vpc_config);
+        formatter.field("package_type", &self.package_type);
+        formatter.field("dead_letter_config", &self.dead_letter_config);
+        formatter.field("environment", &"*** Sensitive Data Redacted ***");
+        formatter.field("kms_key_arn", &self.kms_key_arn);
+        formatter.field("tracing_config", &self.tracing_config);
+        formatter.field("tags", &self.tags);
+        formatter.field("layers", &self.layers);
+        formatter.field("file_system_configs", &self.file_system_configs);
+        formatter.field("code_signing_config_arn", &self.code_signing_config_arn);
+        formatter.field("image_config", &self.image_config);
+        formatter.field("architectures", &self.architectures);
+        formatter.field("ephemeral_storage", &self.ephemeral_storage);
+        formatter.field("snap_start", &self.snap_start);
+        formatter.field("logging_config", &self.logging_config);
+        formatter.field("tenancy_config", &self.tenancy_config);
+        formatter.field("capacity_provider_config", &self.capacity_provider_config);
+        formatter.field("durable_config", &self.durable_config);
+        formatter.finish()
+    }
+}
```

### `src/operation/create_function/_create_function_output.rs`

```diff
--- reference/src/operation/create_function/_create_function_output.rs
+++ generated/src/operation/create_function/_create_function_output.rs
@@ -2,7 +2,7 @@

 /// <p>Details about a function's configuration.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct CreateFunctionOutput {
     /// <p>The name of the function.</p>
     pub function_name: ::std::option::Option<::std::string::String>,
@@ -17,7 +17,7 @@
     /// <p>The function that Lambda calls to begin running your function.</p>
     pub handler: ::std::option::Option<::std::string::String>,
     /// <p>The size of the function's deployment package, in bytes.</p>
-    pub code_size: i64,
+    pub code_size: ::std::option::Option<i64>,
     /// <p>The function's description.</p>
     pub description: ::std::option::Option<::std::string::String>,
     /// <p>The amount of time in seconds that Lambda allows a function to run before stopping it.</p>
@@ -123,7 +123,7 @@
         self.handler.as_deref()
     }
     /// <p>The size of the function's deployment package, in bytes.</p>
-    pub fn code_size(&self) -> i64 {
+    pub fn code_size(&self) -> ::std::option::Option<i64> {
         self.code_size
     }
     /// <p>The function's description.</p>
@@ -280,6 +280,53 @@
         self.durable_config.as_ref()
     }
 }
+impl ::std::fmt::Debug for CreateFunctionOutput {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("CreateFunctionOutput");
+        formatter.field("function_name", &self.function_name);
+        formatter.field("function_arn", &self.function_arn);
+        formatter.field("runtime", &self.runtime);
+        formatter.field("role", &self.role);
+        formatter.field("handler", &self.handler);
+        formatter.field("code_size", &self.code_size);
+        formatter.field("description", &self.description);
+        formatter.field("timeout", &self.timeout);
+        formatter.field("memory_size", &self.memory_size);
+        formatter.field("last_modified", &self.last_modified);
+        formatter.field("code_sha256", &self.code_sha256);
+        formatter.field("version", &self.version);
+        formatter.field("vpc_config", &self.vpc_config);
+        formatter.field("dead_letter_config", &self.dead_letter_config);
+        formatter.field("environment", &"*** Sensitive Data Redacted ***");
+        formatter.field("kms_key_arn", &self.kms_key_arn);
+        formatter.field("tracing_config", &self.tracing_config);
+        formatter.field("master_arn", &self.master_arn);
+        formatter.field("revision_id", &self.revision_id);
+        formatter.field("layers", &self.layers);
+        formatter.field("state", &self.state);
+        formatter.field("state_reason", &self.state_reason);
+        formatter.field("state_reason_code", &self.state_reason_code);
+        formatter.field("last_update_status", &self.last_update_status);
+        formatter.field("last_update_status_reason", &self.last_update_status_reason);
+        formatter.field("last_update_status_reason_code", &self.last_update_status_reason_code);
+        formatter.field("file_system_configs", &self.file_system_configs);
+        formatter.field("signing_profile_version_arn", &self.signing_profile_version_arn);
+        formatter.field("signing_job_arn", &self.signing_job_arn);
+        formatter.field("package_type", &self.package_type);
+        formatter.field("image_config_response", &"*** Sensitive Data Redacted ***");
+        formatter.field("architectures", &self.architectures);
+        formatter.field("ephemeral_storage", &self.ephemeral_storage);
+        formatter.field("snap_start", &self.snap_start);
+        formatter.field("runtime_version_config", &"*** Sensitive Data Redacted ***");
+        formatter.field("logging_config", &self.logging_config);
+        formatter.field("tenancy_config", &self.tenancy_config);
+        formatter.field("capacity_provider_config", &self.capacity_provider_config);
+        formatter.field("config_sha256", &self.config_sha256);
+        formatter.field("durable_config", &self.durable_config);
+        formatter.field("_request_id", &self._request_id);
+        formatter.finish()
+    }
+}
 impl ::aws_types::request_id::RequestId for CreateFunctionOutput {
     fn request_id(&self) -> Option<&str> {
         self._request_id.as_deref()
@@ -293,7 +340,7 @@
 }

 /// A builder for [`CreateFunctionOutput`](crate::operation::create_function::CreateFunctionOutput).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
 #[non_exhaustive]
 pub struct CreateFunctionOutputBuilder {
     pub(crate) function_name: ::std::option::Option<::std::string::String>,
@@ -973,7 +1020,7 @@
             runtime: self.runtime,
             role: self.role,
             handler: self.handler,
-            code_size: self.code_size.unwrap_or_default(),
+            code_size: self.code_size,
             description: self.description,
             timeout: self.timeout,
             memory_size: self.memory_size,
@@ -1012,3 +1059,50 @@
         }
     }
 }
+impl ::std::fmt::Debug for CreateFunctionOutputBuilder {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("CreateFunctionOutputBuilder");
+        formatter.field("function_name", &self.function_name);
+        formatter.field("function_arn", &self.function_arn);
+        formatter.field("runtime", &self.runtime);
+        formatter.field("role", &self.role);
+        formatter.field("handler", &self.handler);
+        formatter.field("code_size", &self.code_size);
+        formatter.field("description", &self.description);
+        formatter.field("timeout", &self.timeout);
+        formatter.field("memory_size", &self.memory_size);
+        formatter.field("last_modified", &self.last_modified);
+        formatter.field("code_sha256", &self.code_sha256);
+        formatter.field("version", &self.version);
+        formatter.field("vpc_config", &self.vpc_config);
+        formatter.field("dead_letter_config", &self.dead_letter_config);
+        formatter.field("environment", &"*** Sensitive Data Redacted ***");
+        formatter.field("kms_key_arn", &self.kms_key_arn);
+        formatter.field("tracing_config", &self.tracing_config);
+        formatter.field("master_arn", &self.master_arn);
+        formatter.field("revision_id", &self.revision_id);
+        formatter.field("layers", &self.layers);
+        formatter.field("state", &self.state);
+        formatter.field("state_reason", &self.state_reason);
+        formatter.field("state_reason_code", &self.state_reason_code);
+        formatter.field("last_update_status", &self.last_update_status);
+        formatter.field("last_update_status_reason", &self.last_update_status_reason);
+        formatter.field("last_update_status_reason_code", &self.last_update_status_reason_code);
+        formatter.field("file_system_configs", &self.file_system_configs);
+        formatter.field("signing_profile_version_arn", &self.signing_profile_version_arn);
+        formatter.field("signing_job_arn", &self.signing_job_arn);
+        formatter.field("package_type", &self.package_type);
+        formatter.field("image_config_response", &"*** Sensitive Data Redacted ***");
+        formatter.field("architectures", &self.architectures);
+        formatter.field("ephemeral_storage", &self.ephemeral_storage);
+        formatter.field("snap_start", &self.snap_start);
+        formatter.field("runtime_version_config", &"*** Sensitive Data Redacted ***");
+        formatter.field("logging_config", &self.logging_config);
+        formatter.field("tenancy_config", &self.tenancy_config);
+        formatter.field("capacity_provider_config", &self.capacity_provider_config);
+        formatter.field("config_sha256", &self.config_sha256);
+        formatter.field("durable_config", &self.durable_config);
+        formatter.field("_request_id", &self._request_id);
+        formatter.finish()
+    }
+}
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

### `src/operation/delete_function/_delete_function_output.rs`

```diff
--- reference/src/operation/delete_function/_delete_function_output.rs
+++ generated/src/operation/delete_function/_delete_function_output.rs
@@ -4,12 +4,12 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct DeleteFunctionOutput {
     /// <p>The HTTP status code returned by the operation.</p>
-    pub status_code: i32,
+    pub status_code: ::std::option::Option<i32>,
     _request_id: Option<String>,
 }
 impl DeleteFunctionOutput {
     /// <p>The HTTP status code returned by the operation.</p>
-    pub fn status_code(&self) -> i32 {
+    pub fn status_code(&self) -> ::std::option::Option<i32> {
         self.status_code
     }
 }
@@ -59,7 +59,7 @@
     /// Consumes the builder and constructs a [`DeleteFunctionOutput`](crate::operation::delete_function::DeleteFunctionOutput).
     pub fn build(self) -> crate::operation::delete_function::DeleteFunctionOutput {
         crate::operation::delete_function::DeleteFunctionOutput {
-            status_code: self.status_code.unwrap_or_default(),
+            status_code: self.status_code,
             _request_id: self._request_id,
         }
     }
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

### `src/operation/get_durable_execution/_get_durable_execution_output.rs`

```diff
--- reference/src/operation/get_durable_execution/_get_durable_execution_output.rs
+++ generated/src/operation/get_durable_execution/_get_durable_execution_output.rs
@@ -97,7 +97,7 @@
         formatter.field("function_arn", &self.function_arn);
         formatter.field("input_payload", &"*** Sensitive Data Redacted ***");
         formatter.field("result", &"*** Sensitive Data Redacted ***");
-        formatter.field("error", &self.error);
+        formatter.field("error", &"*** Sensitive Data Redacted ***");
         formatter.field("start_timestamp", &self.start_timestamp);
         formatter.field("status", &self.status);
         formatter.field("end_timestamp", &self.end_timestamp);
@@ -399,7 +399,7 @@
         formatter.field("function_arn", &self.function_arn);
         formatter.field("input_payload", &"*** Sensitive Data Redacted ***");
         formatter.field("result", &"*** Sensitive Data Redacted ***");
-        formatter.field("error", &self.error);
+        formatter.field("error", &"*** Sensitive Data Redacted ***");
         formatter.field("start_timestamp", &self.start_timestamp);
         formatter.field("status", &self.status);
         formatter.field("end_timestamp", &self.end_timestamp);
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

### `src/operation/get_durable_execution_history/_get_durable_execution_history_output.rs`

```diff
--- reference/src/operation/get_durable_execution_history/_get_durable_execution_history_output.rs
+++ generated/src/operation/get_durable_execution_history/_get_durable_execution_history_output.rs
@@ -2,7 +2,7 @@

 /// <p>The response from the GetDurableExecutionHistory operation, containing the execution history and events.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct GetDurableExecutionHistoryOutput {
     /// <p>An array of execution history events, ordered chronologically unless <code>ReverseOrder</code> is set to <code>true</code>. Each event represents a significant occurrence during the execution, such as step completion or callback resolution.</p>
     pub events: ::std::vec::Vec<crate::types::Event>,
@@ -21,6 +21,15 @@
         self.next_marker.as_deref()
     }
 }
+impl ::std::fmt::Debug for GetDurableExecutionHistoryOutput {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("GetDurableExecutionHistoryOutput");
+        formatter.field("events", &"*** Sensitive Data Redacted ***");
+        formatter.field("next_marker", &self.next_marker);
+        formatter.field("_request_id", &self._request_id);
+        formatter.finish()
+    }
+}
 impl ::aws_types::request_id::RequestId for GetDurableExecutionHistoryOutput {
     fn request_id(&self) -> Option<&str> {
         self._request_id.as_deref()
@@ -34,7 +43,7 @@
 }

 /// A builder for [`GetDurableExecutionHistoryOutput`](crate::operation::get_durable_execution_history::GetDurableExecutionHistoryOutput).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
 #[non_exhaustive]
 pub struct GetDurableExecutionHistoryOutputBuilder {
     pub(crate) events: ::std::option::Option<::std::vec::Vec<crate::types::Event>>,
@@ -106,3 +115,12 @@
         })
     }
 }
+impl ::std::fmt::Debug for GetDurableExecutionHistoryOutputBuilder {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("GetDurableExecutionHistoryOutputBuilder");
+        formatter.field("events", &"*** Sensitive Data Redacted ***");
+        formatter.field("next_marker", &self.next_marker);
+        formatter.field("_request_id", &self._request_id);
+        formatter.finish()
+    }
+}
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

### `src/operation/get_durable_execution_state/_get_durable_execution_state_output.rs`

```diff
--- reference/src/operation/get_durable_execution_state/_get_durable_execution_state_output.rs
+++ generated/src/operation/get_durable_execution_state/_get_durable_execution_state_output.rs
@@ -2,7 +2,7 @@

 /// <p>The response from the GetDurableExecutionState operation, containing the current execution state for replay.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct GetDurableExecutionStateOutput {
     /// <p>An array of operations that represent the current state of the durable execution. Operations are ordered by their start sequence number in ascending order and include information needed for replay processing.</p>
     pub operations: ::std::vec::Vec<crate::types::Operation>,
@@ -21,6 +21,15 @@
         self.next_marker.as_deref()
     }
 }
+impl ::std::fmt::Debug for GetDurableExecutionStateOutput {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("GetDurableExecutionStateOutput");
+        formatter.field("operations", &"*** Sensitive Data Redacted ***");
+        formatter.field("next_marker", &self.next_marker);
+        formatter.field("_request_id", &self._request_id);
+        formatter.finish()
+    }
+}
 impl ::aws_types::request_id::RequestId for GetDurableExecutionStateOutput {
     fn request_id(&self) -> Option<&str> {
         self._request_id.as_deref()
@@ -34,7 +43,7 @@
 }

 /// A builder for [`GetDurableExecutionStateOutput`](crate::operation::get_durable_execution_state::GetDurableExecutionStateOutput).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
 #[non_exhaustive]
 pub struct GetDurableExecutionStateOutputBuilder {
     pub(crate) operations: ::std::option::Option<::std::vec::Vec<crate::types::Operation>>,
@@ -106,3 +115,12 @@
         })
     }
 }
+impl ::std::fmt::Debug for GetDurableExecutionStateOutputBuilder {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("GetDurableExecutionStateOutputBuilder");
+        formatter.field("operations", &"*** Sensitive Data Redacted ***");
+        formatter.field("next_marker", &self.next_marker);
+        formatter.field("_request_id", &self._request_id);
+        formatter.finish()
+    }
+}
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

### `src/operation/get_function/_get_function_output.rs`

```diff
--- reference/src/operation/get_function/_get_function_output.rs
+++ generated/src/operation/get_function/_get_function_output.rs
@@ -1,7 +1,7 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 #[allow(missing_docs)] // documentation missing in model
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct GetFunctionOutput {
     /// <p>The configuration of the function or version.</p>
     pub configuration: ::std::option::Option<crate::types::FunctionConfiguration>,
@@ -37,6 +37,18 @@
         self.concurrency.as_ref()
     }
 }
+impl ::std::fmt::Debug for GetFunctionOutput {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("GetFunctionOutput");
+        formatter.field("configuration", &"*** Sensitive Data Redacted ***");
+        formatter.field("code", &"*** Sensitive Data Redacted ***");
+        formatter.field("tags", &self.tags);
+        formatter.field("tags_error", &self.tags_error);
+        formatter.field("concurrency", &self.concurrency);
+        formatter.field("_request_id", &self._request_id);
+        formatter.finish()
+    }
+}
 impl ::aws_types::request_id::RequestId for GetFunctionOutput {
     fn request_id(&self) -> Option<&str> {
         self._request_id.as_deref()
@@ -50,7 +62,7 @@
 }

 /// A builder for [`GetFunctionOutput`](crate::operation::get_function::GetFunctionOutput).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
 #[non_exhaustive]
 pub struct GetFunctionOutputBuilder {
     pub(crate) configuration: ::std::option::Option<crate::types::FunctionConfiguration>,
@@ -158,3 +170,15 @@
         }
     }
 }
+impl ::std::fmt::Debug for GetFunctionOutputBuilder {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("GetFunctionOutputBuilder");
+        formatter.field("configuration", &"*** Sensitive Data Redacted ***");
+        formatter.field("code", &"*** Sensitive Data Redacted ***");
+        formatter.field("tags", &self.tags);
+        formatter.field("tags_error", &self.tags_error);
+        formatter.field("concurrency", &self.concurrency);
+        formatter.field("_request_id", &self._request_id);
+        formatter.finish()
+    }
+}
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

### `src/operation/get_function_configuration/_get_function_configuration_output.rs`

```diff
--- reference/src/operation/get_function_configuration/_get_function_configuration_output.rs
+++ generated/src/operation/get_function_configuration/_get_function_configuration_output.rs
@@ -2,7 +2,7 @@

 /// <p>Details about a function's configuration.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct GetFunctionConfigurationOutput {
     /// <p>The name of the function.</p>
     pub function_name: ::std::option::Option<::std::string::String>,
@@ -17,7 +17,7 @@
     /// <p>The function that Lambda calls to begin running your function.</p>
     pub handler: ::std::option::Option<::std::string::String>,
     /// <p>The size of the function's deployment package, in bytes.</p>
-    pub code_size: i64,
+    pub code_size: ::std::option::Option<i64>,
     /// <p>The function's description.</p>
     pub description: ::std::option::Option<::std::string::String>,
     /// <p>The amount of time in seconds that Lambda allows a function to run before stopping it.</p>
@@ -123,7 +123,7 @@
         self.handler.as_deref()
     }
     /// <p>The size of the function's deployment package, in bytes.</p>
-    pub fn code_size(&self) -> i64 {
+    pub fn code_size(&self) -> ::std::option::Option<i64> {
         self.code_size
     }
     /// <p>The function's description.</p>
@@ -280,6 +280,53 @@
         self.durable_config.as_ref()
     }
 }
+impl ::std::fmt::Debug for GetFunctionConfigurationOutput {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("GetFunctionConfigurationOutput");
+        formatter.field("function_name", &self.function_name);
+        formatter.field("function_arn", &self.function_arn);
+        formatter.field("runtime", &self.runtime);
+        formatter.field("role", &self.role);
+        formatter.field("handler", &self.handler);
+        formatter.field("code_size", &self.code_size);
+        formatter.field("description", &self.description);
+        formatter.field("timeout", &self.timeout);
+        formatter.field("memory_size", &self.memory_size);
+        formatter.field("last_modified", &self.last_modified);
+        formatter.field("code_sha256", &self.code_sha256);
+        formatter.field("version", &self.version);
+        formatter.field("vpc_config", &self.vpc_config);
+        formatter.field("dead_letter_config", &self.dead_letter_config);
+        formatter.field("environment", &"*** Sensitive Data Redacted ***");
+        formatter.field("kms_key_arn", &self.kms_key_arn);
+        formatter.field("tracing_config", &self.tracing_config);
+        formatter.field("master_arn", &self.master_arn);
+        formatter.field("revision_id", &self.revision_id);
+        formatter.field("layers", &self.layers);
+        formatter.field("state", &self.state);
+        formatter.field("state_reason", &self.state_reason);
+        formatter.field("state_reason_code", &self.state_reason_code);
+        formatter.field("last_update_status", &self.last_update_status);
+        formatter.field("last_update_status_reason", &self.last_update_status_reason);
+        formatter.field("last_update_status_reason_code", &self.last_update_status_reason_code);
+        formatter.field("file_system_configs", &self.file_system_configs);
+        formatter.field("signing_profile_version_arn", &self.signing_profile_version_arn);
+        formatter.field("signing_job_arn", &self.signing_job_arn);
+        formatter.field("package_type", &self.package_type);
+        formatter.field("image_config_response", &"*** Sensitive Data Redacted ***");
+        formatter.field("architectures", &self.architectures);
+        formatter.field("ephemeral_storage", &self.ephemeral_storage);
+        formatter.field("snap_start", &self.snap_start);
+        formatter.field("runtime_version_config", &"*** Sensitive Data Redacted ***");
+        formatter.field("logging_config", &self.logging_config);
+        formatter.field("tenancy_config", &self.tenancy_config);
+        formatter.field("capacity_provider_config", &self.capacity_provider_config);
+        formatter.field("config_sha256", &self.config_sha256);
+        formatter.field("durable_config", &self.durable_config);
+        formatter.field("_request_id", &self._request_id);
+        formatter.finish()
+    }
+}
 impl ::aws_types::request_id::RequestId for GetFunctionConfigurationOutput {
     fn request_id(&self) -> Option<&str> {
         self._request_id.as_deref()
@@ -293,7 +340,7 @@
 }

 /// A builder for [`GetFunctionConfigurationOutput`](crate::operation::get_function_configuration::GetFunctionConfigurationOutput).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
 #[non_exhaustive]
 pub struct GetFunctionConfigurationOutputBuilder {
     pub(crate) function_name: ::std::option::Option<::std::string::String>,
@@ -973,7 +1020,7 @@
             runtime: self.runtime,
             role: self.role,
             handler: self.handler,
-            code_size: self.code_size.unwrap_or_default(),
+            code_size: self.code_size,
             description: self.description,
             timeout: self.timeout,
             memory_size: self.memory_size,
@@ -1012,3 +1059,50 @@
         }
     }
 }
+impl ::std::fmt::Debug for GetFunctionConfigurationOutputBuilder {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("GetFunctionConfigurationOutputBuilder");
+        formatter.field("function_name", &self.function_name);
+        formatter.field("function_arn", &self.function_arn);
+        formatter.field("runtime", &self.runtime);
+        formatter.field("role", &self.role);
+        formatter.field("handler", &self.handler);
+        formatter.field("code_size", &self.code_size);
+        formatter.field("description", &self.description);
+        formatter.field("timeout", &self.timeout);
+        formatter.field("memory_size", &self.memory_size);
+        formatter.field("last_modified", &self.last_modified);
+        formatter.field("code_sha256", &self.code_sha256);
+        formatter.field("version", &self.version);
+        formatter.field("vpc_config", &self.vpc_config);
+        formatter.field("dead_letter_config", &self.dead_letter_config);
+        formatter.field("environment", &"*** Sensitive Data Redacted ***");
+        formatter.field("kms_key_arn", &self.kms_key_arn);
+        formatter.field("tracing_config", &self.tracing_config);
+        formatter.field("master_arn", &self.master_arn);
+        formatter.field("revision_id", &self.revision_id);
+        formatter.field("layers", &self.layers);
+        formatter.field("state", &self.state);
+        formatter.field("state_reason", &self.state_reason);
+        formatter.field("state_reason_code", &self.state_reason_code);
+        formatter.field("last_update_status", &self.last_update_status);
+        formatter.field("last_update_status_reason", &self.last_update_status_reason);
+        formatter.field("last_update_status_reason_code", &self.last_update_status_reason_code);
+        formatter.field("file_system_configs", &self.file_system_configs);
+        formatter.field("signing_profile_version_arn", &self.signing_profile_version_arn);
+        formatter.field("signing_job_arn", &self.signing_job_arn);
+        formatter.field("package_type", &self.package_type);
+        formatter.field("image_config_response", &"*** Sensitive Data Redacted ***");
+        formatter.field("architectures", &self.architectures);
+        formatter.field("ephemeral_storage", &self.ephemeral_storage);
+        formatter.field("snap_start", &self.snap_start);
+        formatter.field("runtime_version_config", &"*** Sensitive Data Redacted ***");
+        formatter.field("logging_config", &self.logging_config);
+        formatter.field("tenancy_config", &self.tenancy_config);
+        formatter.field("capacity_provider_config", &self.capacity_provider_config);
+        formatter.field("config_sha256", &self.config_sha256);
+        formatter.field("durable_config", &self.durable_config);
+        formatter.field("_request_id", &self._request_id);
+        formatter.finish()
+    }
+}
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
@@ -14,7 +14,7 @@
     /// <p>The date that the layer version was created, in <a href="https://www.w3.org/TR/NOTE-datetime">ISO-8601 format</a> (YYYY-MM-DDThh:mm:ss.sTZD).</p>
     pub created_date: ::std::option::Option<::std::string::String>,
     /// <p>The version number.</p>
-    pub version: i64,
+    pub version: ::std::option::Option<i64>,
     /// <p>A list of compatible <a href="https://docs.aws.amazon.com/lambda/latest/dg/foundation-arch.html">instruction set architectures</a>.</p>
     pub compatible_architectures: ::std::option::Option<::std::vec::Vec<crate::types::Architecture>>,
     /// <p>The layer's compatible runtimes.</p>
@@ -27,7 +27,7 @@
 }
 impl GetLayerVersionOutput {
     /// <p>Details about the layer version.</p>
-    pub fn content(&self) -> ::std::option::Option<&crate::types::LayerVersionContentOutput> {
+    pub fn content(&self) -> ::std::option::Option<&crate::operation::get_layer_version::Output> {
         self.content.as_ref()
     }
     /// <p>The ARN of the layer.</p>
@@ -47,7 +47,7 @@
         self.created_date.as_deref()
     }
     /// <p>The version number.</p>
-    pub fn version(&self) -> i64 {
+    pub fn version(&self) -> ::std::option::Option<i64> {
         self.version
     }
     /// <p>A list of compatible <a href="https://docs.aws.amazon.com/lambda/latest/dg/foundation-arch.html">instruction set architectures</a>.</p>
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
@@ -258,7 +258,7 @@
             layer_version_arn: self.layer_version_arn,
             description: self.description,
             created_date: self.created_date,
-            version: self.version.unwrap_or_default(),
+            version: self.version,
             compatible_architectures: self.compatible_architectures,
             compatible_runtimes: self.compatible_runtimes,
             license_info: self.license_info,
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
@@ -14,7 +14,7 @@
     /// <p>The date that the layer version was created, in <a href="https://www.w3.org/TR/NOTE-datetime">ISO-8601 format</a> (YYYY-MM-DDThh:mm:ss.sTZD).</p>
     pub created_date: ::std::option::Option<::std::string::String>,
     /// <p>The version number.</p>
-    pub version: i64,
+    pub version: ::std::option::Option<i64>,
     /// <p>A list of compatible <a href="https://docs.aws.amazon.com/lambda/latest/dg/foundation-arch.html">instruction set architectures</a>.</p>
     pub compatible_architectures: ::std::option::Option<::std::vec::Vec<crate::types::Architecture>>,
     /// <p>The layer's compatible runtimes.</p>
@@ -27,7 +27,7 @@
 }
 impl GetLayerVersionByArnOutput {
     /// <p>Details about the layer version.</p>
-    pub fn content(&self) -> ::std::option::Option<&crate::types::LayerVersionContentOutput> {
+    pub fn content(&self) -> ::std::option::Option<&crate::operation::get_layer_version_by_arn::Output> {
         self.content.as_ref()
     }
     /// <p>The ARN of the layer.</p>
@@ -47,7 +47,7 @@
         self.created_date.as_deref()
     }
     /// <p>The version number.</p>
-    pub fn version(&self) -> i64 {
+    pub fn version(&self) -> ::std::option::Option<i64> {
         self.version
     }
     /// <p>A list of compatible <a href="https://docs.aws.amazon.com/lambda/latest/dg/foundation-arch.html">instruction set architectures</a>.</p>
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
@@ -258,7 +258,7 @@
             layer_version_arn: self.layer_version_arn,
             description: self.description,
             created_date: self.created_date,
-            version: self.version.unwrap_or_default(),
+            version: self.version,
             compatible_architectures: self.compatible_architectures,
             compatible_runtimes: self.compatible_runtimes,
             license_info: self.license_info,
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

### `src/operation/invoke/_invoke_input.rs`

```diff
--- reference/src/operation/invoke/_invoke_input.rs
+++ generated/src/operation/invoke/_invoke_input.rs
@@ -34,7 +34,7 @@
     pub durable_execution_name: ::std::option::Option<::std::string::String>,
     /// <p>The JSON that you want to provide to your Lambda function as input. The maximum payload size is 6 MB for synchronous invocations and 1 MB for asynchronous invocations.</p>
     /// <p>You can enter the JSON directly. For example, <code>--payload '{ "key": "value" }'</code>. You can also specify a file path. For example, <code>--payload file://payload.json</code>.</p>
-    pub payload: ::std::option::Option<::aws_smithy_types::Blob>,
+    pub payload: ::std::option::Option<::std::vec::Vec<u8>>,
     /// <p>Specify a version or alias to invoke a published version of the function.</p>
     pub qualifier: ::std::option::Option<::std::string::String>,
     /// <p>The identifier of the tenant in a multi-tenant Lambda function.</p>
@@ -82,7 +82,7 @@
     }
     /// <p>The JSON that you want to provide to your Lambda function as input. The maximum payload size is 6 MB for synchronous invocations and 1 MB for asynchronous invocations.</p>
     /// <p>You can enter the JSON directly. For example, <code>--payload '{ "key": "value" }'</code>. You can also specify a file path. For example, <code>--payload file://payload.json</code>.</p>
-    pub fn payload(&self) -> ::std::option::Option<&::aws_smithy_types::Blob> {
+    pub fn payload(&self) -> ::std::option::Option<&::std::vec::Vec<u8>> {
         self.payload.as_ref()
     }
     /// <p>Specify a version or alias to invoke a published version of the function.</p>
@@ -124,7 +124,7 @@
     pub(crate) log_type: ::std::option::Option<crate::types::LogType>,
     pub(crate) client_context: ::std::option::Option<::std::string::String>,
     pub(crate) durable_execution_name: ::std::option::Option<::std::string::String>,
-    pub(crate) payload: ::std::option::Option<::aws_smithy_types::Blob>,
+    pub(crate) payload: ::std::option::Option<::std::vec::Vec<u8>>,
     pub(crate) qualifier: ::std::option::Option<::std::string::String>,
     pub(crate) tenant_id: ::std::option::Option<::std::string::String>,
 }
@@ -259,19 +259,19 @@
     }
     /// <p>The JSON that you want to provide to your Lambda function as input. The maximum payload size is 6 MB for synchronous invocations and 1 MB for asynchronous invocations.</p>
     /// <p>You can enter the JSON directly. For example, <code>--payload '{ "key": "value" }'</code>. You can also specify a file path. For example, <code>--payload file://payload.json</code>.</p>
-    pub fn payload(mut self, input: ::aws_smithy_types::Blob) -> Self {
+    pub fn payload(mut self, input: ::std::vec::Vec<u8>) -> Self {
         self.payload = ::std::option::Option::Some(input);
         self
     }
     /// <p>The JSON that you want to provide to your Lambda function as input. The maximum payload size is 6 MB for synchronous invocations and 1 MB for asynchronous invocations.</p>
     /// <p>You can enter the JSON directly. For example, <code>--payload '{ "key": "value" }'</code>. You can also specify a file path. For example, <code>--payload file://payload.json</code>.</p>
-    pub fn set_payload(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
+    pub fn set_payload(mut self, input: ::std::option::Option<::std::vec::Vec<u8>>) -> Self {
         self.payload = input;
         self
     }
     /// <p>The JSON that you want to provide to your Lambda function as input. The maximum payload size is 6 MB for synchronous invocations and 1 MB for asynchronous invocations.</p>
     /// <p>You can enter the JSON directly. For example, <code>--payload '{ "key": "value" }'</code>. You can also specify a file path. For example, <code>--payload file://payload.json</code>.</p>
-    pub fn get_payload(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
+    pub fn get_payload(&self) -> &::std::option::Option<::std::vec::Vec<u8>> {
         &self.payload
     }
     /// <p>Specify a version or alias to invoke a published version of the function.</p>
```

### `src/operation/invoke/_invoke_output.rs`

```diff
--- reference/src/operation/invoke/_invoke_output.rs
+++ generated/src/operation/invoke/_invoke_output.rs
@@ -4,13 +4,13 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct InvokeOutput {
     /// <p>The HTTP status code is in the 200 range for a successful request. For the <code>RequestResponse</code> invocation type, this status code is 200. For the <code>Event</code> invocation type, this status code is 202. For the <code>DryRun</code> invocation type, the status code is 204.</p>
-    pub status_code: i32,
+    pub status_code: ::std::option::Option<i32>,
     /// <p>If present, indicates that an error occurred during function execution. Details about the error are included in the response payload.</p>
     pub function_error: ::std::option::Option<::std::string::String>,
     /// <p>The last 4 KB of the execution log, which is base64-encoded.</p>
     pub log_result: ::std::option::Option<::std::string::String>,
     /// <p>The response from the function, or an error object.</p>
-    pub payload: ::std::option::Option<::aws_smithy_types::Blob>,
+    pub payload: ::std::option::Option<::std::vec::Vec<u8>>,
     /// <p>The version of the function that executed. When you invoke a function with an alias, this indicates which version the alias resolved to.</p>
     pub executed_version: ::std::option::Option<::std::string::String>,
     /// <p>The ARN of the durable execution that was started. This is returned when invoking a durable function and provides a unique identifier for tracking the execution.</p>
@@ -19,7 +19,7 @@
 }
 impl InvokeOutput {
     /// <p>The HTTP status code is in the 200 range for a successful request. For the <code>RequestResponse</code> invocation type, this status code is 200. For the <code>Event</code> invocation type, this status code is 202. For the <code>DryRun</code> invocation type, the status code is 204.</p>
-    pub fn status_code(&self) -> i32 {
+    pub fn status_code(&self) -> ::std::option::Option<i32> {
         self.status_code
     }
     /// <p>If present, indicates that an error occurred during function execution. Details about the error are included in the response payload.</p>
@@ -31,7 +31,7 @@
         self.log_result.as_deref()
     }
     /// <p>The response from the function, or an error object.</p>
-    pub fn payload(&self) -> ::std::option::Option<&::aws_smithy_types::Blob> {
+    pub fn payload(&self) -> ::std::option::Option<&::std::vec::Vec<u8>> {
         self.payload.as_ref()
     }
     /// <p>The version of the function that executed. When you invoke a function with an alias, this indicates which version the alias resolved to.</p>
@@ -75,7 +75,7 @@
     pub(crate) status_code: ::std::option::Option<i32>,
     pub(crate) function_error: ::std::option::Option<::std::string::String>,
     pub(crate) log_result: ::std::option::Option<::std::string::String>,
-    pub(crate) payload: ::std::option::Option<::aws_smithy_types::Blob>,
+    pub(crate) payload: ::std::option::Option<::std::vec::Vec<u8>>,
     pub(crate) executed_version: ::std::option::Option<::std::string::String>,
     pub(crate) durable_execution_arn: ::std::option::Option<::std::string::String>,
     _request_id: Option<String>,
@@ -124,17 +124,17 @@
         &self.log_result
     }
     /// <p>The response from the function, or an error object.</p>
-    pub fn payload(mut self, input: ::aws_smithy_types::Blob) -> Self {
+    pub fn payload(mut self, input: ::std::vec::Vec<u8>) -> Self {
         self.payload = ::std::option::Option::Some(input);
         self
     }
     /// <p>The response from the function, or an error object.</p>
-    pub fn set_payload(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
+    pub fn set_payload(mut self, input: ::std::option::Option<::std::vec::Vec<u8>>) -> Self {
         self.payload = input;
         self
     }
     /// <p>The response from the function, or an error object.</p>
-    pub fn get_payload(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
+    pub fn get_payload(&self) -> &::std::option::Option<::std::vec::Vec<u8>> {
         &self.payload
     }
     /// <p>The version of the function that executed. When you invoke a function with an alias, this indicates which version the alias resolved to.</p>
@@ -177,7 +177,7 @@
     /// Consumes the builder and constructs a [`InvokeOutput`](crate::operation::invoke::InvokeOutput).
     pub fn build(self) -> crate::operation::invoke::InvokeOutput {
         crate::operation::invoke::InvokeOutput {
-            status_code: self.status_code.unwrap_or_default(),
+            status_code: self.status_code,
             function_error: self.function_error,
             log_result: self.log_result,
             payload: self.payload,
```

### `src/operation/invoke/builders.rs`

```diff
--- reference/src/operation/invoke/builders.rs
+++ generated/src/operation/invoke/builders.rs
@@ -231,19 +231,19 @@
     }
     /// <p>The JSON that you want to provide to your Lambda function as input. The maximum payload size is 6 MB for synchronous invocations and 1 MB for asynchronous invocations.</p>
     /// <p>You can enter the JSON directly. For example, <code>--payload '{ "key": "value" }'</code>. You can also specify a file path. For example, <code>--payload file://payload.json</code>.</p>
-    pub fn payload(mut self, input: ::aws_smithy_types::Blob) -> Self {
+    pub fn payload(mut self, input: ::std::vec::Vec<u8>) -> Self {
         self.inner = self.inner.payload(input);
         self
     }
     /// <p>The JSON that you want to provide to your Lambda function as input. The maximum payload size is 6 MB for synchronous invocations and 1 MB for asynchronous invocations.</p>
     /// <p>You can enter the JSON directly. For example, <code>--payload '{ "key": "value" }'</code>. You can also specify a file path. For example, <code>--payload file://payload.json</code>.</p>
-    pub fn set_payload(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
+    pub fn set_payload(mut self, input: ::std::option::Option<::std::vec::Vec<u8>>) -> Self {
         self.inner = self.inner.set_payload(input);
         self
     }
     /// <p>The JSON that you want to provide to your Lambda function as input. The maximum payload size is 6 MB for synchronous invocations and 1 MB for asynchronous invocations.</p>
     /// <p>You can enter the JSON directly. For example, <code>--payload '{ "key": "value" }'</code>. You can also specify a file path. For example, <code>--payload file://payload.json</code>.</p>
-    pub fn get_payload(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
+    pub fn get_payload(&self) -> &::std::option::Option<::std::vec::Vec<u8>> {
         self.inner.get_payload()
     }
     /// <p>Specify a version or alias to invoke a published version of the function.</p>
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

### `src/operation/invoke_async/_invoke_async_input.rs`

```diff
--- reference/src/operation/invoke_async/_invoke_async_input.rs
+++ generated/src/operation/invoke_async/_invoke_async_input.rs
@@ -1,8 +1,7 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 #[allow(missing_docs)] // documentation missing in model
-#[deprecated]
 #[non_exhaustive]
-#[derive(::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct InvokeAsyncInput {
     /// <p>The name or ARN of the Lambda function.</p>
     /// <p class="title"><b>Name formats</b></p>
@@ -17,7 +16,7 @@
     /// <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
     pub function_name: ::std::option::Option<::std::string::String>,
     /// <p>The JSON that you want to provide to your Lambda function as input.</p>
-    pub invoke_args: ::aws_smithy_types::byte_stream::ByteStream,
+    pub invoke_args: ::std::option::Option<::std::vec::Vec<u8>>,
 }
 impl InvokeAsyncInput {
     /// <p>The name or ARN of the Lambda function.</p>
@@ -35,8 +34,8 @@
         self.function_name.as_deref()
     }
     /// <p>The JSON that you want to provide to your Lambda function as input.</p>
-    pub fn invoke_args(&self) -> &::aws_smithy_types::byte_stream::ByteStream {
-        &self.invoke_args
+    pub fn invoke_args(&self) -> ::std::option::Option<&::std::vec::Vec<u8>> {
+        self.invoke_args.as_ref()
     }
 }
 impl InvokeAsyncInput {
@@ -47,11 +46,11 @@
 }

 /// A builder for [`InvokeAsyncInput`](crate::operation::invoke_async::InvokeAsyncInput).
-#[derive(::std::default::Default, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
 pub struct InvokeAsyncInputBuilder {
     pub(crate) function_name: ::std::option::Option<::std::string::String>,
-    pub(crate) invoke_args: ::std::option::Option<::aws_smithy_types::byte_stream::ByteStream>,
+    pub(crate) invoke_args: ::std::option::Option<::std::vec::Vec<u8>>,
 }
 impl InvokeAsyncInputBuilder {
     /// <p>The name or ARN of the Lambda function.</p>
@@ -101,17 +100,17 @@
     }
     /// <p>The JSON that you want to provide to your Lambda function as input.</p>
     /// This field is required.
-    pub fn invoke_args(mut self, input: ::aws_smithy_types::byte_stream::ByteStream) -> Self {
+    pub fn invoke_args(mut self, input: ::std::vec::Vec<u8>) -> Self {
         self.invoke_args = ::std::option::Option::Some(input);
         self
     }
     /// <p>The JSON that you want to provide to your Lambda function as input.</p>
-    pub fn set_invoke_args(mut self, input: ::std::option::Option<::aws_smithy_types::byte_stream::ByteStream>) -> Self {
+    pub fn set_invoke_args(mut self, input: ::std::option::Option<::std::vec::Vec<u8>>) -> Self {
         self.invoke_args = input;
         self
     }
     /// <p>The JSON that you want to provide to your Lambda function as input.</p>
-    pub fn get_invoke_args(&self) -> &::std::option::Option<::aws_smithy_types::byte_stream::ByteStream> {
+    pub fn get_invoke_args(&self) -> &::std::option::Option<::std::vec::Vec<u8>> {
         &self.invoke_args
     }
     /// Consumes the builder and constructs a [`InvokeAsyncInput`](crate::operation::invoke_async::InvokeAsyncInput).
@@ -118,7 +117,7 @@
     pub fn build(self) -> ::std::result::Result<crate::operation::invoke_async::InvokeAsyncInput, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(crate::operation::invoke_async::InvokeAsyncInput {
             function_name: self.function_name,
-            invoke_args: self.invoke_args.unwrap_or_default(),
+            invoke_args: self.invoke_args,
         })
     }
 }
```

### `src/operation/invoke_async/_invoke_async_output.rs`

```diff
--- reference/src/operation/invoke_async/_invoke_async_output.rs
+++ generated/src/operation/invoke_async/_invoke_async_output.rs
@@ -1,17 +1,16 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

 /// <p>A success response (<code>202 Accepted</code>) indicates that the request is queued for invocation.</p>
-#[deprecated]
 #[non_exhaustive]
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct InvokeAsyncOutput {
     /// <p>The status code.</p>
-    pub status: i32,
+    pub status: ::std::option::Option<i32>,
     _request_id: Option<String>,
 }
 impl InvokeAsyncOutput {
     /// <p>The status code.</p>
-    pub fn status(&self) -> i32 {
+    pub fn status(&self) -> ::std::option::Option<i32> {
         self.status
     }
 }
@@ -61,7 +60,7 @@
     /// Consumes the builder and constructs a [`InvokeAsyncOutput`](crate::operation::invoke_async::InvokeAsyncOutput).
     pub fn build(self) -> crate::operation::invoke_async::InvokeAsyncOutput {
         crate::operation::invoke_async::InvokeAsyncOutput {
-            status: self.status.unwrap_or_default(),
+            status: self.status,
             _request_id: self._request_id,
         }
     }
```

### `src/operation/invoke_async/builders.rs`

```diff
--- reference/src/operation/invoke_async/builders.rs
+++ generated/src/operation/invoke_async/builders.rs
@@ -31,7 +31,7 @@
 /// <p>If you do use the InvokeAsync action, note that it doesn't support the use of X-Ray active tracing. Trace ID is not propagated to the function, even if X-Ray active tracing is turned on.</p>
 /// </note>
 #[deprecated]
-#[derive(::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::fmt::Debug)]
 pub struct InvokeAsyncFluentBuilder {
     handle: ::std::sync::Arc<crate::client::Handle>,
     inner: crate::operation::invoke_async::builders::InvokeAsyncInputBuilder,
@@ -161,17 +161,17 @@
         self.inner.get_function_name()
     }
     /// <p>The JSON that you want to provide to your Lambda function as input.</p>
-    pub fn invoke_args(mut self, input: ::aws_smithy_types::byte_stream::ByteStream) -> Self {
+    pub fn invoke_args(mut self, input: ::std::vec::Vec<u8>) -> Self {
         self.inner = self.inner.invoke_args(input);
         self
     }
     /// <p>The JSON that you want to provide to your Lambda function as input.</p>
-    pub fn set_invoke_args(mut self, input: ::std::option::Option<::aws_smithy_types::byte_stream::ByteStream>) -> Self {
+    pub fn set_invoke_args(mut self, input: ::std::option::Option<::std::vec::Vec<u8>>) -> Self {
         self.inner = self.inner.set_invoke_args(input);
         self
     }
     /// <p>The JSON that you want to provide to your Lambda function as input.</p>
-    pub fn get_invoke_args(&self) -> &::std::option::Option<::aws_smithy_types::byte_stream::ByteStream> {
+    pub fn get_invoke_args(&self) -> &::std::option::Option<::std::vec::Vec<u8>> {
         self.inner.get_invoke_args()
     }
 }
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

### `src/operation/invoke_with_response_stream/_invoke_with_response_stream_input.rs`

```diff
--- reference/src/operation/invoke_with_response_stream/_invoke_with_response_stream_input.rs
+++ generated/src/operation/invoke_with_response_stream/_invoke_with_response_stream_input.rs
@@ -23,7 +23,7 @@
     pub qualifier: ::std::option::Option<::std::string::String>,
     /// <p>The JSON that you want to provide to your Lambda function as input.</p>
     /// <p>You can enter the JSON directly. For example, <code>--payload '{ "key": "value" }'</code>. You can also specify a file path. For example, <code>--payload file://payload.json</code>.</p>
-    pub payload: ::std::option::Option<::aws_smithy_types::Blob>,
+    pub payload: ::std::option::Option<::std::vec::Vec<u8>>,
     /// <p>The identifier of the tenant in a multi-tenant Lambda function.</p>
     pub tenant_id: ::std::option::Option<::std::string::String>,
     /// <p>Use one of the following options:</p>
@@ -64,7 +64,7 @@
     }
     /// <p>The JSON that you want to provide to your Lambda function as input.</p>
     /// <p>You can enter the JSON directly. For example, <code>--payload '{ "key": "value" }'</code>. You can also specify a file path. For example, <code>--payload file://payload.json</code>.</p>
-    pub fn payload(&self) -> ::std::option::Option<&::aws_smithy_types::Blob> {
+    pub fn payload(&self) -> ::std::option::Option<&::std::vec::Vec<u8>> {
         self.payload.as_ref()
     }
     /// <p>The identifier of the tenant in a multi-tenant Lambda function.</p>
@@ -110,7 +110,7 @@
     pub(crate) log_type: ::std::option::Option<crate::types::LogType>,
     pub(crate) client_context: ::std::option::Option<::std::string::String>,
     pub(crate) qualifier: ::std::option::Option<::std::string::String>,
-    pub(crate) payload: ::std::option::Option<::aws_smithy_types::Blob>,
+    pub(crate) payload: ::std::option::Option<::std::vec::Vec<u8>>,
     pub(crate) tenant_id: ::std::option::Option<::std::string::String>,
     pub(crate) invocation_type: ::std::option::Option<crate::types::ResponseStreamingInvocationType>,
 }
@@ -204,19 +204,19 @@
     }
     /// <p>The JSON that you want to provide to your Lambda function as input.</p>
     /// <p>You can enter the JSON directly. For example, <code>--payload '{ "key": "value" }'</code>. You can also specify a file path. For example, <code>--payload file://payload.json</code>.</p>
-    pub fn payload(mut self, input: ::aws_smithy_types::Blob) -> Self {
+    pub fn payload(mut self, input: ::std::vec::Vec<u8>) -> Self {
         self.payload = ::std::option::Option::Some(input);
         self
     }
     /// <p>The JSON that you want to provide to your Lambda function as input.</p>
     /// <p>You can enter the JSON directly. For example, <code>--payload '{ "key": "value" }'</code>. You can also specify a file path. For example, <code>--payload file://payload.json</code>.</p>
-    pub fn set_payload(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
+    pub fn set_payload(mut self, input: ::std::option::Option<::std::vec::Vec<u8>>) -> Self {
         self.payload = input;
         self
     }
     /// <p>The JSON that you want to provide to your Lambda function as input.</p>
     /// <p>You can enter the JSON directly. For example, <code>--payload '{ "key": "value" }'</code>. You can also specify a file path. For example, <code>--payload file://payload.json</code>.</p>
-    pub fn get_payload(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
+    pub fn get_payload(&self) -> &::std::option::Option<::std::vec::Vec<u8>> {
         &self.payload
     }
     /// <p>The identifier of the tenant in a multi-tenant Lambda function.</p>
```

### `src/operation/invoke_with_response_stream/_invoke_with_response_stream_output.rs`

```diff
--- reference/src/operation/invoke_with_response_stream/_invoke_with_response_stream_output.rs
+++ generated/src/operation/invoke_with_response_stream/_invoke_with_response_stream_output.rs
@@ -1,17 +1,14 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 #[allow(missing_docs)] // documentation missing in model
 #[non_exhaustive]
-#[derive(::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct InvokeWithResponseStreamOutput {
     /// <p>For a successful request, the HTTP status code is in the 200 range. For the <code>RequestResponse</code> invocation type, this status code is 200. For the <code>DryRun</code> invocation type, this status code is 204.</p>
-    pub status_code: i32,
+    pub status_code: ::std::option::Option<i32>,
     /// <p>The version of the function that executed. When you invoke a function with an alias, this indicates which version the alias resolved to.</p>
     pub executed_version: ::std::option::Option<::std::string::String>,
     /// <p>The stream of response payloads.</p>
-    pub event_stream: crate::event_receiver::EventReceiver<
-        crate::types::InvokeWithResponseStreamResponseEvent,
-        crate::types::error::InvokeWithResponseStreamResponseEventError,
-    >,
+    pub event_stream: ::std::option::Option<crate::types::InvokeWithResponseStreamResponseEvent>,
     /// <p>The type of data the stream is returning.</p>
     pub response_stream_content_type: ::std::option::Option<::std::string::String>,
     _request_id: Option<String>,
@@ -18,7 +15,7 @@
 }
 impl InvokeWithResponseStreamOutput {
     /// <p>For a successful request, the HTTP status code is in the 200 range. For the <code>RequestResponse</code> invocation type, this status code is 200. For the <code>DryRun</code> invocation type, this status code is 204.</p>
-    pub fn status_code(&self) -> i32 {
+    pub fn status_code(&self) -> ::std::option::Option<i32> {
         self.status_code
     }
     /// <p>The version of the function that executed. When you invoke a function with an alias, this indicates which version the alias resolved to.</p>
@@ -26,13 +23,8 @@
         self.executed_version.as_deref()
     }
     /// <p>The stream of response payloads.</p>
-    pub fn event_stream(
-        &self,
-    ) -> &crate::event_receiver::EventReceiver<
-        crate::types::InvokeWithResponseStreamResponseEvent,
-        crate::types::error::InvokeWithResponseStreamResponseEventError,
-    > {
-        &self.event_stream
+    pub fn event_stream(&self) -> ::std::option::Option<&crate::types::InvokeWithResponseStreamResponseEvent> {
+        self.event_stream.as_ref()
     }
     /// <p>The type of data the stream is returning.</p>
     pub fn response_stream_content_type(&self) -> ::std::option::Option<&str> {
@@ -52,17 +44,12 @@
 }

 /// A builder for [`InvokeWithResponseStreamOutput`](crate::operation::invoke_with_response_stream::InvokeWithResponseStreamOutput).
-#[derive(::std::default::Default, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
 #[non_exhaustive]
 pub struct InvokeWithResponseStreamOutputBuilder {
     pub(crate) status_code: ::std::option::Option<i32>,
     pub(crate) executed_version: ::std::option::Option<::std::string::String>,
-    pub(crate) event_stream: ::std::option::Option<
-        crate::event_receiver::EventReceiver<
-            crate::types::InvokeWithResponseStreamResponseEvent,
-            crate::types::error::InvokeWithResponseStreamResponseEventError,
-        >,
-    >,
+    pub(crate) event_stream: ::std::option::Option<crate::types::InvokeWithResponseStreamResponseEvent>,
     pub(crate) response_stream_content_type: ::std::option::Option<::std::string::String>,
     _request_id: Option<String>,
 }
@@ -96,38 +83,17 @@
         &self.executed_version
     }
     /// <p>The stream of response payloads.</p>
-    pub fn event_stream(
-        mut self,
-        input: crate::event_receiver::EventReceiver<
-            crate::types::InvokeWithResponseStreamResponseEvent,
-            crate::types::error::InvokeWithResponseStreamResponseEventError,
-        >,
-    ) -> Self {
+    pub fn event_stream(mut self, input: crate::types::InvokeWithResponseStreamResponseEvent) -> Self {
         self.event_stream = ::std::option::Option::Some(input);
         self
     }
     /// <p>The stream of response payloads.</p>
-    pub fn set_event_stream(
-        mut self,
-        input: ::std::option::Option<
-            crate::event_receiver::EventReceiver<
-                crate::types::InvokeWithResponseStreamResponseEvent,
-                crate::types::error::InvokeWithResponseStreamResponseEventError,
-            >,
-        >,
-    ) -> Self {
+    pub fn set_event_stream(mut self, input: ::std::option::Option<crate::types::InvokeWithResponseStreamResponseEvent>) -> Self {
         self.event_stream = input;
         self
     }
     /// <p>The stream of response payloads.</p>
-    pub fn get_event_stream(
-        &self,
-    ) -> &::std::option::Option<
-        crate::event_receiver::EventReceiver<
-            crate::types::InvokeWithResponseStreamResponseEvent,
-            crate::types::error::InvokeWithResponseStreamResponseEventError,
-        >,
-    > {
+    pub fn get_event_stream(&self) -> &::std::option::Option<crate::types::InvokeWithResponseStreamResponseEvent> {
         &self.event_stream
     }
     /// <p>The type of data the stream is returning.</p>
@@ -154,25 +120,13 @@
         self
     }
     /// Consumes the builder and constructs a [`InvokeWithResponseStreamOutput`](crate::operation::invoke_with_response_stream::InvokeWithResponseStreamOutput).
-    /// This method will fail if any of the following fields are not set:
-    /// - [`event_stream`](crate::operation::invoke_with_response_stream::builders::InvokeWithResponseStreamOutputBuilder::event_stream)
-    pub fn build(
-        self,
-    ) -> ::std::result::Result<
-        crate::operation::invoke_with_response_stream::InvokeWithResponseStreamOutput,
-        ::aws_smithy_types::error::operation::BuildError,
-    > {
-        ::std::result::Result::Ok(crate::operation::invoke_with_response_stream::InvokeWithResponseStreamOutput {
-            status_code: self.status_code.unwrap_or_default(),
+    pub fn build(self) -> crate::operation::invoke_with_response_stream::InvokeWithResponseStreamOutput {
+        crate::operation::invoke_with_response_stream::InvokeWithResponseStreamOutput {
+            status_code: self.status_code,
             executed_version: self.executed_version,
-            event_stream: self.event_stream.ok_or_else(|| {
-                ::aws_smithy_types::error::operation::BuildError::missing_field(
-                    "event_stream",
-                    "event_stream was not specified but it is required when building InvokeWithResponseStreamOutput",
-                )
-            })?,
+            event_stream: self.event_stream,
             response_stream_content_type: self.response_stream_content_type,
             _request_id: self._request_id,
-        })
+        }
     }
 }
```

### `src/operation/invoke_with_response_stream/builders.rs`

```diff
--- reference/src/operation/invoke_with_response_stream/builders.rs
+++ generated/src/operation/invoke_with_response_stream/builders.rs
@@ -197,19 +197,19 @@
     }
     /// <p>The JSON that you want to provide to your Lambda function as input.</p>
     /// <p>You can enter the JSON directly. For example, <code>--payload '{ "key": "value" }'</code>. You can also specify a file path. For example, <code>--payload file://payload.json</code>.</p>
-    pub fn payload(mut self, input: ::aws_smithy_types::Blob) -> Self {
+    pub fn payload(mut self, input: ::std::vec::Vec<u8>) -> Self {
         self.inner = self.inner.payload(input);
         self
     }
     /// <p>The JSON that you want to provide to your Lambda function as input.</p>
     /// <p>You can enter the JSON directly. For example, <code>--payload '{ "key": "value" }'</code>. You can also specify a file path. For example, <code>--payload file://payload.json</code>.</p>
-    pub fn set_payload(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
+    pub fn set_payload(mut self, input: ::std::option::Option<::std::vec::Vec<u8>>) -> Self {
         self.inner = self.inner.set_payload(input);
         self
     }
     /// <p>The JSON that you want to provide to your Lambda function as input.</p>
     /// <p>You can enter the JSON directly. For example, <code>--payload '{ "key": "value" }'</code>. You can also specify a file path. For example, <code>--payload file://payload.json</code>.</p>
-    pub fn get_payload(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
+    pub fn get_payload(&self) -> &::std::option::Option<::std::vec::Vec<u8>> {
         self.inner.get_payload()
     }
     /// <p>The identifier of the tenant in a multi-tenant Lambda function.</p>
```

### `src/operation/invoke_with_response_stream.rs`

```diff
--- reference/src/operation/invoke_with_response_stream.rs
+++ generated/src/operation/invoke_with_response_stream.rs
@@ -102,15 +102,14 @@
                 .expect("required fields set"),
         ));

-        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::SensitiveOutput);
         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
             "InvokeWithResponseStream",
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
@@ -139,9 +138,17 @@
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
@@ -214,6 +221,7 @@
     ) -> ::std::option::Option<::aws_smithy_runtime_api::client::interceptors::context::OutputOrError> {
         #[allow(unused_mut)]
         let mut force_error = false;
+        ::tracing::debug!(extended_request_id = ?crate::s3_request_id::RequestIdExt::extended_request_id(response));
         ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));

         // If this is an error, defer to the non-streaming parser
@@ -346,8 +354,8 @@

         let params = crate::config::endpoint::Params::builder()
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
+            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
             .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
             .build()
             .map_err(|err| {
@@ -819,6 +827,11 @@
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

### `src/operation/list_functions/_list_functions_output.rs`

```diff
--- reference/src/operation/list_functions/_list_functions_output.rs
+++ generated/src/operation/list_functions/_list_functions_output.rs
@@ -2,7 +2,7 @@

 /// <p>A list of Lambda functions.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct ListFunctionsOutput {
     /// <p>The pagination token that's included if more results are available.</p>
     pub next_marker: ::std::option::Option<::std::string::String>,
@@ -22,6 +22,15 @@
         self.functions.as_deref().unwrap_or_default()
     }
 }
+impl ::std::fmt::Debug for ListFunctionsOutput {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("ListFunctionsOutput");
+        formatter.field("next_marker", &self.next_marker);
+        formatter.field("functions", &"*** Sensitive Data Redacted ***");
+        formatter.field("_request_id", &self._request_id);
+        formatter.finish()
+    }
+}
 impl ::aws_types::request_id::RequestId for ListFunctionsOutput {
     fn request_id(&self) -> Option<&str> {
         self._request_id.as_deref()
@@ -35,7 +44,7 @@
 }

 /// A builder for [`ListFunctionsOutput`](crate::operation::list_functions::ListFunctionsOutput).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
 #[non_exhaustive]
 pub struct ListFunctionsOutputBuilder {
     pub(crate) next_marker: ::std::option::Option<::std::string::String>,
@@ -95,3 +104,12 @@
         }
     }
 }
+impl ::std::fmt::Debug for ListFunctionsOutputBuilder {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("ListFunctionsOutputBuilder");
+        formatter.field("next_marker", &self.next_marker);
+        formatter.field("functions", &"*** Sensitive Data Redacted ***");
+        formatter.field("_request_id", &self._request_id);
+        formatter.finish()
+    }
+}
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

### `src/operation/list_versions_by_function/_list_versions_by_function_output.rs`

```diff
--- reference/src/operation/list_versions_by_function/_list_versions_by_function_output.rs
+++ generated/src/operation/list_versions_by_function/_list_versions_by_function_output.rs
@@ -1,7 +1,7 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 #[allow(missing_docs)] // documentation missing in model
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct ListVersionsByFunctionOutput {
     /// <p>The pagination token that's included if more results are available.</p>
     pub next_marker: ::std::option::Option<::std::string::String>,
@@ -21,6 +21,15 @@
         self.versions.as_deref().unwrap_or_default()
     }
 }
+impl ::std::fmt::Debug for ListVersionsByFunctionOutput {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("ListVersionsByFunctionOutput");
+        formatter.field("next_marker", &self.next_marker);
+        formatter.field("versions", &"*** Sensitive Data Redacted ***");
+        formatter.field("_request_id", &self._request_id);
+        formatter.finish()
+    }
+}
 impl ::aws_types::request_id::RequestId for ListVersionsByFunctionOutput {
     fn request_id(&self) -> Option<&str> {
         self._request_id.as_deref()
@@ -34,7 +43,7 @@
 }

 /// A builder for [`ListVersionsByFunctionOutput`](crate::operation::list_versions_by_function::ListVersionsByFunctionOutput).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
 #[non_exhaustive]
 pub struct ListVersionsByFunctionOutputBuilder {
     pub(crate) next_marker: ::std::option::Option<::std::string::String>,
@@ -94,3 +103,12 @@
         }
     }
 }
+impl ::std::fmt::Debug for ListVersionsByFunctionOutputBuilder {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("ListVersionsByFunctionOutputBuilder");
+        formatter.field("next_marker", &self.next_marker);
+        formatter.field("versions", &"*** Sensitive Data Redacted ***");
+        formatter.field("_request_id", &self._request_id);
+        formatter.finish()
+    }
+}
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
@@ -1,7 +1,7 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 #[allow(missing_docs)] // documentation missing in model
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct PublishLayerVersionInput {
     /// <p>The name or Amazon Resource Name (ARN) of the layer.</p>
     pub layer_name: ::std::option::Option<::std::string::String>,
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
@@ -64,6 +64,18 @@
         self.license_info.as_deref()
     }
 }
+impl ::std::fmt::Debug for PublishLayerVersionInput {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("PublishLayerVersionInput");
+        formatter.field("layer_name", &self.layer_name);
+        formatter.field("description", &self.description);
+        formatter.field("content", &"*** Sensitive Data Redacted ***");
+        formatter.field("compatible_architectures", &self.compatible_architectures);
+        formatter.field("compatible_runtimes", &self.compatible_runtimes);
+        formatter.field("license_info", &self.license_info);
+        formatter.finish()
+    }
+}
 impl PublishLayerVersionInput {
     /// Creates a new builder-style object to manufacture [`PublishLayerVersionInput`](crate::operation::publish_layer_version::PublishLayerVersionInput).
     pub fn builder() -> crate::operation::publish_layer_version::builders::PublishLayerVersionInputBuilder {
@@ -72,12 +84,12 @@
 }

 /// A builder for [`PublishLayerVersionInput`](crate::operation::publish_layer_version::PublishLayerVersionInput).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
 #[non_exhaustive]
 pub struct PublishLayerVersionInputBuilder {
     pub(crate) layer_name: ::std::option::Option<::std::string::String>,
     pub(crate) description: ::std::option::Option<::std::string::String>,
-    pub(crate) content: ::std::option::Option<crate::types::LayerVersionContentInput>,
+    pub(crate) content: ::std::option::Option<crate::operation::publish_layer_version::Input>,
     pub(crate) compatible_architectures: ::std::option::Option<::std::vec::Vec<crate::types::Architecture>>,
     pub(crate) compatible_runtimes: ::std::option::Option<::std::vec::Vec<crate::types::Runtime>>,
     pub(crate) license_info: ::std::option::Option<::std::string::String>,
@@ -114,17 +126,17 @@
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
@@ -223,3 +235,15 @@
         })
     }
 }
+impl ::std::fmt::Debug for PublishLayerVersionInputBuilder {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("PublishLayerVersionInputBuilder");
+        formatter.field("layer_name", &self.layer_name);
+        formatter.field("description", &self.description);
+        formatter.field("content", &"*** Sensitive Data Redacted ***");
+        formatter.field("compatible_architectures", &self.compatible_architectures);
+        formatter.field("compatible_runtimes", &self.compatible_runtimes);
+        formatter.field("license_info", &self.license_info);
+        formatter.finish()
+    }
+}
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
@@ -14,7 +14,7 @@
     /// <p>The date that the layer version was created, in <a href="https://www.w3.org/TR/NOTE-datetime">ISO-8601 format</a> (YYYY-MM-DDThh:mm:ss.sTZD).</p>
     pub created_date: ::std::option::Option<::std::string::String>,
     /// <p>The version number.</p>
-    pub version: i64,
+    pub version: ::std::option::Option<i64>,
     /// <p>A list of compatible <a href="https://docs.aws.amazon.com/lambda/latest/dg/foundation-arch.html">instruction set architectures</a>.</p>
     pub compatible_architectures: ::std::option::Option<::std::vec::Vec<crate::types::Architecture>>,
     /// <p>The layer's compatible runtimes.</p>
@@ -27,7 +27,7 @@
 }
 impl PublishLayerVersionOutput {
     /// <p>Details about the layer version.</p>
-    pub fn content(&self) -> ::std::option::Option<&crate::types::LayerVersionContentOutput> {
+    pub fn content(&self) -> ::std::option::Option<&crate::operation::publish_layer_version::Output> {
         self.content.as_ref()
     }
     /// <p>The ARN of the layer.</p>
@@ -47,7 +47,7 @@
         self.created_date.as_deref()
     }
     /// <p>The version number.</p>
-    pub fn version(&self) -> i64 {
+    pub fn version(&self) -> ::std::option::Option<i64> {
         self.version
     }
     /// <p>A list of compatible <a href="https://docs.aws.amazon.com/lambda/latest/dg/foundation-arch.html">instruction set architectures</a>.</p>
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
@@ -258,7 +258,7 @@
             layer_version_arn: self.layer_version_arn,
             description: self.description,
             created_date: self.created_date,
-            version: self.version.unwrap_or_default(),
+            version: self.version,
             compatible_architectures: self.compatible_architectures,
             compatible_runtimes: self.compatible_runtimes,
             license_info: self.license_info,
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

### `src/operation/publish_version/_publish_version_output.rs`

```diff
--- reference/src/operation/publish_version/_publish_version_output.rs
+++ generated/src/operation/publish_version/_publish_version_output.rs
@@ -2,7 +2,7 @@

 /// <p>Details about a function's configuration.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct PublishVersionOutput {
     /// <p>The name of the function.</p>
     pub function_name: ::std::option::Option<::std::string::String>,
@@ -17,7 +17,7 @@
     /// <p>The function that Lambda calls to begin running your function.</p>
     pub handler: ::std::option::Option<::std::string::String>,
     /// <p>The size of the function's deployment package, in bytes.</p>
-    pub code_size: i64,
+    pub code_size: ::std::option::Option<i64>,
     /// <p>The function's description.</p>
     pub description: ::std::option::Option<::std::string::String>,
     /// <p>The amount of time in seconds that Lambda allows a function to run before stopping it.</p>
@@ -123,7 +123,7 @@
         self.handler.as_deref()
     }
     /// <p>The size of the function's deployment package, in bytes.</p>
-    pub fn code_size(&self) -> i64 {
+    pub fn code_size(&self) -> ::std::option::Option<i64> {
         self.code_size
     }
     /// <p>The function's description.</p>
@@ -280,6 +280,53 @@
         self.durable_config.as_ref()
     }
 }
+impl ::std::fmt::Debug for PublishVersionOutput {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("PublishVersionOutput");
+        formatter.field("function_name", &self.function_name);
+        formatter.field("function_arn", &self.function_arn);
+        formatter.field("runtime", &self.runtime);
+        formatter.field("role", &self.role);
+        formatter.field("handler", &self.handler);
+        formatter.field("code_size", &self.code_size);
+        formatter.field("description", &self.description);
+        formatter.field("timeout", &self.timeout);
+        formatter.field("memory_size", &self.memory_size);
+        formatter.field("last_modified", &self.last_modified);
+        formatter.field("code_sha256", &self.code_sha256);
+        formatter.field("version", &self.version);
+        formatter.field("vpc_config", &self.vpc_config);
+        formatter.field("dead_letter_config", &self.dead_letter_config);
+        formatter.field("environment", &"*** Sensitive Data Redacted ***");
+        formatter.field("kms_key_arn", &self.kms_key_arn);
+        formatter.field("tracing_config", &self.tracing_config);
+        formatter.field("master_arn", &self.master_arn);
+        formatter.field("revision_id", &self.revision_id);
+        formatter.field("layers", &self.layers);
+        formatter.field("state", &self.state);
+        formatter.field("state_reason", &self.state_reason);
+        formatter.field("state_reason_code", &self.state_reason_code);
+        formatter.field("last_update_status", &self.last_update_status);
+        formatter.field("last_update_status_reason", &self.last_update_status_reason);
+        formatter.field("last_update_status_reason_code", &self.last_update_status_reason_code);
+        formatter.field("file_system_configs", &self.file_system_configs);
+        formatter.field("signing_profile_version_arn", &self.signing_profile_version_arn);
+        formatter.field("signing_job_arn", &self.signing_job_arn);
+        formatter.field("package_type", &self.package_type);
+        formatter.field("image_config_response", &"*** Sensitive Data Redacted ***");
+        formatter.field("architectures", &self.architectures);
+        formatter.field("ephemeral_storage", &self.ephemeral_storage);
+        formatter.field("snap_start", &self.snap_start);
+        formatter.field("runtime_version_config", &"*** Sensitive Data Redacted ***");
+        formatter.field("logging_config", &self.logging_config);
+        formatter.field("tenancy_config", &self.tenancy_config);
+        formatter.field("capacity_provider_config", &self.capacity_provider_config);
+        formatter.field("config_sha256", &self.config_sha256);
+        formatter.field("durable_config", &self.durable_config);
+        formatter.field("_request_id", &self._request_id);
+        formatter.finish()
+    }
+}
 impl ::aws_types::request_id::RequestId for PublishVersionOutput {
     fn request_id(&self) -> Option<&str> {
         self._request_id.as_deref()
@@ -293,7 +340,7 @@
 }

 /// A builder for [`PublishVersionOutput`](crate::operation::publish_version::PublishVersionOutput).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
 #[non_exhaustive]
 pub struct PublishVersionOutputBuilder {
     pub(crate) function_name: ::std::option::Option<::std::string::String>,
@@ -973,7 +1020,7 @@
             runtime: self.runtime,
             role: self.role,
             handler: self.handler,
-            code_size: self.code_size.unwrap_or_default(),
+            code_size: self.code_size,
             description: self.description,
             timeout: self.timeout,
             memory_size: self.memory_size,
@@ -1012,3 +1059,50 @@
         }
     }
 }
+impl ::std::fmt::Debug for PublishVersionOutputBuilder {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("PublishVersionOutputBuilder");
+        formatter.field("function_name", &self.function_name);
+        formatter.field("function_arn", &self.function_arn);
+        formatter.field("runtime", &self.runtime);
+        formatter.field("role", &self.role);
+        formatter.field("handler", &self.handler);
+        formatter.field("code_size", &self.code_size);
+        formatter.field("description", &self.description);
+        formatter.field("timeout", &self.timeout);
+        formatter.field("memory_size", &self.memory_size);
+        formatter.field("last_modified", &self.last_modified);
+        formatter.field("code_sha256", &self.code_sha256);
+        formatter.field("version", &self.version);
+        formatter.field("vpc_config", &self.vpc_config);
+        formatter.field("dead_letter_config", &self.dead_letter_config);
+        formatter.field("environment", &"*** Sensitive Data Redacted ***");
+        formatter.field("kms_key_arn", &self.kms_key_arn);
+        formatter.field("tracing_config", &self.tracing_config);
+        formatter.field("master_arn", &self.master_arn);
+        formatter.field("revision_id", &self.revision_id);
+        formatter.field("layers", &self.layers);
+        formatter.field("state", &self.state);
+        formatter.field("state_reason", &self.state_reason);
+        formatter.field("state_reason_code", &self.state_reason_code);
+        formatter.field("last_update_status", &self.last_update_status);
+        formatter.field("last_update_status_reason", &self.last_update_status_reason);
+        formatter.field("last_update_status_reason_code", &self.last_update_status_reason_code);
+        formatter.field("file_system_configs", &self.file_system_configs);
+        formatter.field("signing_profile_version_arn", &self.signing_profile_version_arn);
+        formatter.field("signing_job_arn", &self.signing_job_arn);
+        formatter.field("package_type", &self.package_type);
+        formatter.field("image_config_response", &"*** Sensitive Data Redacted ***");
+        formatter.field("architectures", &self.architectures);
+        formatter.field("ephemeral_storage", &self.ephemeral_storage);
+        formatter.field("snap_start", &self.snap_start);
+        formatter.field("runtime_version_config", &"*** Sensitive Data Redacted ***");
+        formatter.field("logging_config", &self.logging_config);
+        formatter.field("tenancy_config", &self.tenancy_config);
+        formatter.field("capacity_provider_config", &self.capacity_provider_config);
+        formatter.field("config_sha256", &self.config_sha256);
+        formatter.field("durable_config", &self.durable_config);
+        formatter.field("_request_id", &self._request_id);
+        formatter.finish()
+    }
+}
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

### `src/operation/send_durable_execution_callback_failure/_send_durable_execution_callback_failure_input.rs`

```diff
--- reference/src/operation/send_durable_execution_callback_failure/_send_durable_execution_callback_failure_input.rs
+++ generated/src/operation/send_durable_execution_callback_failure/_send_durable_execution_callback_failure_input.rs
@@ -1,7 +1,7 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 #[allow(missing_docs)] // documentation missing in model
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct SendDurableExecutionCallbackFailureInput {
     /// <p>The unique identifier for the callback operation.</p>
     pub callback_id: ::std::option::Option<::std::string::String>,
@@ -18,6 +18,14 @@
         self.error.as_ref()
     }
 }
+impl ::std::fmt::Debug for SendDurableExecutionCallbackFailureInput {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("SendDurableExecutionCallbackFailureInput");
+        formatter.field("callback_id", &self.callback_id);
+        formatter.field("error", &"*** Sensitive Data Redacted ***");
+        formatter.finish()
+    }
+}
 impl SendDurableExecutionCallbackFailureInput {
     /// Creates a new builder-style object to manufacture [`SendDurableExecutionCallbackFailureInput`](crate::operation::send_durable_execution_callback_failure::SendDurableExecutionCallbackFailureInput).
     pub fn builder() -> crate::operation::send_durable_execution_callback_failure::builders::SendDurableExecutionCallbackFailureInputBuilder {
@@ -26,7 +34,7 @@
 }

 /// A builder for [`SendDurableExecutionCallbackFailureInput`](crate::operation::send_durable_execution_callback_failure::SendDurableExecutionCallbackFailureInput).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
 #[non_exhaustive]
 pub struct SendDurableExecutionCallbackFailureInputBuilder {
     pub(crate) callback_id: ::std::option::Option<::std::string::String>,
@@ -77,3 +85,11 @@
         )
     }
 }
+impl ::std::fmt::Debug for SendDurableExecutionCallbackFailureInputBuilder {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("SendDurableExecutionCallbackFailureInputBuilder");
+        formatter.field("callback_id", &self.callback_id);
+        formatter.field("error", &"*** Sensitive Data Redacted ***");
+        formatter.finish()
+    }
+}
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

### `src/operation/send_durable_execution_callback_success/_send_durable_execution_callback_success_input.rs`

```diff
--- reference/src/operation/send_durable_execution_callback_success/_send_durable_execution_callback_success_input.rs
+++ generated/src/operation/send_durable_execution_callback_success/_send_durable_execution_callback_success_input.rs
@@ -6,7 +6,7 @@
     /// <p>The unique identifier for the callback operation.</p>
     pub callback_id: ::std::option::Option<::std::string::String>,
     /// <p>The result data from the successful callback operation. Maximum size is 256 KB.</p>
-    pub result: ::std::option::Option<::aws_smithy_types::Blob>,
+    pub result: ::std::option::Option<::std::vec::Vec<u8>>,
 }
 impl SendDurableExecutionCallbackSuccessInput {
     /// <p>The unique identifier for the callback operation.</p>
@@ -14,7 +14,7 @@
         self.callback_id.as_deref()
     }
     /// <p>The result data from the successful callback operation. Maximum size is 256 KB.</p>
-    pub fn result(&self) -> ::std::option::Option<&::aws_smithy_types::Blob> {
+    pub fn result(&self) -> ::std::option::Option<&::std::vec::Vec<u8>> {
         self.result.as_ref()
     }
 }
@@ -38,7 +38,7 @@
 #[non_exhaustive]
 pub struct SendDurableExecutionCallbackSuccessInputBuilder {
     pub(crate) callback_id: ::std::option::Option<::std::string::String>,
-    pub(crate) result: ::std::option::Option<::aws_smithy_types::Blob>,
+    pub(crate) result: ::std::option::Option<::std::vec::Vec<u8>>,
 }
 impl SendDurableExecutionCallbackSuccessInputBuilder {
     /// <p>The unique identifier for the callback operation.</p>
@@ -57,17 +57,17 @@
         &self.callback_id
     }
     /// <p>The result data from the successful callback operation. Maximum size is 256 KB.</p>
-    pub fn result(mut self, input: ::aws_smithy_types::Blob) -> Self {
+    pub fn result(mut self, input: ::std::vec::Vec<u8>) -> Self {
         self.result = ::std::option::Option::Some(input);
         self
     }
     /// <p>The result data from the successful callback operation. Maximum size is 256 KB.</p>
-    pub fn set_result(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
+    pub fn set_result(mut self, input: ::std::option::Option<::std::vec::Vec<u8>>) -> Self {
         self.result = input;
         self
     }
     /// <p>The result data from the successful callback operation. Maximum size is 256 KB.</p>
-    pub fn get_result(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
+    pub fn get_result(&self) -> &::std::option::Option<::std::vec::Vec<u8>> {
         &self.result
     }
     /// Consumes the builder and constructs a [`SendDurableExecutionCallbackSuccessInput`](crate::operation::send_durable_execution_callback_success::SendDurableExecutionCallbackSuccessInput).
```

### `src/operation/send_durable_execution_callback_success/builders.rs`

```diff
--- reference/src/operation/send_durable_execution_callback_success/builders.rs
+++ generated/src/operation/send_durable_execution_callback_success/builders.rs
@@ -124,17 +124,17 @@
         self.inner.get_callback_id()
     }
     /// <p>The result data from the successful callback operation. Maximum size is 256 KB.</p>
-    pub fn result(mut self, input: ::aws_smithy_types::Blob) -> Self {
+    pub fn result(mut self, input: ::std::vec::Vec<u8>) -> Self {
         self.inner = self.inner.result(input);
         self
     }
     /// <p>The result data from the successful callback operation. Maximum size is 256 KB.</p>
-    pub fn set_result(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
+    pub fn set_result(mut self, input: ::std::option::Option<::std::vec::Vec<u8>>) -> Self {
         self.inner = self.inner.set_result(input);
         self
     }
     /// <p>The result data from the successful callback operation. Maximum size is 256 KB.</p>
-    pub fn get_result(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
+    pub fn get_result(&self) -> &::std::option::Option<::std::vec::Vec<u8>> {
         self.inner.get_result()
     }
 }
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

### `src/operation/stop_durable_execution/_stop_durable_execution_input.rs`

```diff
--- reference/src/operation/stop_durable_execution/_stop_durable_execution_input.rs
+++ generated/src/operation/stop_durable_execution/_stop_durable_execution_input.rs
@@ -1,7 +1,7 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 #[allow(missing_docs)] // documentation missing in model
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct StopDurableExecutionInput {
     /// <p>The Amazon Resource Name (ARN) of the durable execution.</p>
     pub durable_execution_arn: ::std::option::Option<::std::string::String>,
@@ -18,6 +18,14 @@
         self.error.as_ref()
     }
 }
+impl ::std::fmt::Debug for StopDurableExecutionInput {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("StopDurableExecutionInput");
+        formatter.field("durable_execution_arn", &self.durable_execution_arn);
+        formatter.field("error", &"*** Sensitive Data Redacted ***");
+        formatter.finish()
+    }
+}
 impl StopDurableExecutionInput {
     /// Creates a new builder-style object to manufacture [`StopDurableExecutionInput`](crate::operation::stop_durable_execution::StopDurableExecutionInput).
     pub fn builder() -> crate::operation::stop_durable_execution::builders::StopDurableExecutionInputBuilder {
@@ -26,7 +34,7 @@
 }

 /// A builder for [`StopDurableExecutionInput`](crate::operation::stop_durable_execution::StopDurableExecutionInput).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
 #[non_exhaustive]
 pub struct StopDurableExecutionInputBuilder {
     pub(crate) durable_execution_arn: ::std::option::Option<::std::string::String>,
@@ -73,3 +81,11 @@
         })
     }
 }
+impl ::std::fmt::Debug for StopDurableExecutionInputBuilder {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("StopDurableExecutionInputBuilder");
+        formatter.field("durable_execution_arn", &self.durable_execution_arn);
+        formatter.field("error", &"*** Sensitive Data Redacted ***");
+        formatter.finish()
+    }
+}
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
@@ -16,7 +16,7 @@
     /// <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
     pub function_name: ::std::option::Option<::std::string::String>,
     /// <p>The base64-encoded contents of the deployment package. Amazon Web Services SDK and CLI clients handle the encoding for you. Use only with a function defined with a .zip file archive deployment package.</p>
-    pub zip_file: ::std::option::Option<::aws_smithy_types::Blob>,
+    pub zip_file: ::std::option::Option<::std::vec::Vec<u8>>,
     /// <p>An Amazon S3 bucket in the same Amazon Web Services Region as your function. The bucket can be in a different Amazon Web Services account. Use only with a function defined with a .zip file archive deployment package.</p>
     pub s3_bucket: ::std::option::Option<::std::string::String>,
     /// <p>The Amazon S3 key of the deployment package. Use only with a function defined with a .zip file archive deployment package.</p>
@@ -62,7 +62,7 @@
         self.function_name.as_deref()
     }
     /// <p>The base64-encoded contents of the deployment package. Amazon Web Services SDK and CLI clients handle the encoding for you. Use only with a function defined with a .zip file archive deployment package.</p>
-    pub fn zip_file(&self) -> ::std::option::Option<&::aws_smithy_types::Blob> {
+    pub fn zip_file(&self) -> ::std::option::Option<&::std::vec::Vec<u8>> {
         self.zip_file.as_ref()
     }
     /// <p>An Amazon S3 bucket in the same Amazon Web Services Region as your function. The bucket can be in a different Amazon Web Services account. Use only with a function defined with a .zip file archive deployment package.</p>
@@ -149,7 +149,7 @@
 #[non_exhaustive]
 pub struct UpdateFunctionCodeInputBuilder {
     pub(crate) function_name: ::std::option::Option<::std::string::String>,
-    pub(crate) zip_file: ::std::option::Option<::aws_smithy_types::Blob>,
+    pub(crate) zip_file: ::std::option::Option<::std::vec::Vec<u8>>,
     pub(crate) s3_bucket: ::std::option::Option<::std::string::String>,
     pub(crate) s3_key: ::std::option::Option<::std::string::String>,
     pub(crate) s3_object_version: ::std::option::Option<::std::string::String>,
@@ -209,17 +209,17 @@
         &self.function_name
     }
     /// <p>The base64-encoded contents of the deployment package. Amazon Web Services SDK and CLI clients handle the encoding for you. Use only with a function defined with a .zip file archive deployment package.</p>
-    pub fn zip_file(mut self, input: ::aws_smithy_types::Blob) -> Self {
+    pub fn zip_file(mut self, input: ::std::vec::Vec<u8>) -> Self {
         self.zip_file = ::std::option::Option::Some(input);
         self
     }
     /// <p>The base64-encoded contents of the deployment package. Amazon Web Services SDK and CLI clients handle the encoding for you. Use only with a function defined with a .zip file archive deployment package.</p>
-    pub fn set_zip_file(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
+    pub fn set_zip_file(mut self, input: ::std::option::Option<::std::vec::Vec<u8>>) -> Self {
         self.zip_file = input;
         self
     }
     /// <p>The base64-encoded contents of the deployment package. Amazon Web Services SDK and CLI clients handle the encoding for you. Use only with a function defined with a .zip file archive deployment package.</p>
-    pub fn get_zip_file(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
+    pub fn get_zip_file(&self) -> &::std::option::Option<::std::vec::Vec<u8>> {
         &self.zip_file
     }
     /// <p>An Amazon S3 bucket in the same Amazon Web Services Region as your function. The bucket can be in a different Amazon Web Services account. Use only with a function defined with a .zip file archive deployment package.</p>
```

### `src/operation/update_function_code/_update_function_code_output.rs`

```diff
--- reference/src/operation/update_function_code/_update_function_code_output.rs
+++ generated/src/operation/update_function_code/_update_function_code_output.rs
@@ -2,7 +2,7 @@

 /// <p>Details about a function's configuration.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct UpdateFunctionCodeOutput {
     /// <p>The name of the function.</p>
     pub function_name: ::std::option::Option<::std::string::String>,
@@ -17,7 +17,7 @@
     /// <p>The function that Lambda calls to begin running your function.</p>
     pub handler: ::std::option::Option<::std::string::String>,
     /// <p>The size of the function's deployment package, in bytes.</p>
-    pub code_size: i64,
+    pub code_size: ::std::option::Option<i64>,
     /// <p>The function's description.</p>
     pub description: ::std::option::Option<::std::string::String>,
     /// <p>The amount of time in seconds that Lambda allows a function to run before stopping it.</p>
@@ -123,7 +123,7 @@
         self.handler.as_deref()
     }
     /// <p>The size of the function's deployment package, in bytes.</p>
-    pub fn code_size(&self) -> i64 {
+    pub fn code_size(&self) -> ::std::option::Option<i64> {
         self.code_size
     }
     /// <p>The function's description.</p>
@@ -280,6 +280,53 @@
         self.durable_config.as_ref()
     }
 }
+impl ::std::fmt::Debug for UpdateFunctionCodeOutput {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("UpdateFunctionCodeOutput");
+        formatter.field("function_name", &self.function_name);
+        formatter.field("function_arn", &self.function_arn);
+        formatter.field("runtime", &self.runtime);
+        formatter.field("role", &self.role);
+        formatter.field("handler", &self.handler);
+        formatter.field("code_size", &self.code_size);
+        formatter.field("description", &self.description);
+        formatter.field("timeout", &self.timeout);
+        formatter.field("memory_size", &self.memory_size);
+        formatter.field("last_modified", &self.last_modified);
+        formatter.field("code_sha256", &self.code_sha256);
+        formatter.field("version", &self.version);
+        formatter.field("vpc_config", &self.vpc_config);
+        formatter.field("dead_letter_config", &self.dead_letter_config);
+        formatter.field("environment", &"*** Sensitive Data Redacted ***");
+        formatter.field("kms_key_arn", &self.kms_key_arn);
+        formatter.field("tracing_config", &self.tracing_config);
+        formatter.field("master_arn", &self.master_arn);
+        formatter.field("revision_id", &self.revision_id);
+        formatter.field("layers", &self.layers);
+        formatter.field("state", &self.state);
+        formatter.field("state_reason", &self.state_reason);
+        formatter.field("state_reason_code", &self.state_reason_code);
+        formatter.field("last_update_status", &self.last_update_status);
+        formatter.field("last_update_status_reason", &self.last_update_status_reason);
+        formatter.field("last_update_status_reason_code", &self.last_update_status_reason_code);
+        formatter.field("file_system_configs", &self.file_system_configs);
+        formatter.field("signing_profile_version_arn", &self.signing_profile_version_arn);
+        formatter.field("signing_job_arn", &self.signing_job_arn);
+        formatter.field("package_type", &self.package_type);
+        formatter.field("image_config_response", &"*** Sensitive Data Redacted ***");
+        formatter.field("architectures", &self.architectures);
+        formatter.field("ephemeral_storage", &self.ephemeral_storage);
+        formatter.field("snap_start", &self.snap_start);
+        formatter.field("runtime_version_config", &"*** Sensitive Data Redacted ***");
+        formatter.field("logging_config", &self.logging_config);
+        formatter.field("tenancy_config", &self.tenancy_config);
+        formatter.field("capacity_provider_config", &self.capacity_provider_config);
+        formatter.field("config_sha256", &self.config_sha256);
+        formatter.field("durable_config", &self.durable_config);
+        formatter.field("_request_id", &self._request_id);
+        formatter.finish()
+    }
+}
 impl ::aws_types::request_id::RequestId for UpdateFunctionCodeOutput {
     fn request_id(&self) -> Option<&str> {
         self._request_id.as_deref()
@@ -293,7 +340,7 @@
 }

 /// A builder for [`UpdateFunctionCodeOutput`](crate::operation::update_function_code::UpdateFunctionCodeOutput).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
 #[non_exhaustive]
 pub struct UpdateFunctionCodeOutputBuilder {
     pub(crate) function_name: ::std::option::Option<::std::string::String>,
@@ -973,7 +1020,7 @@
             runtime: self.runtime,
             role: self.role,
             handler: self.handler,
-            code_size: self.code_size.unwrap_or_default(),
+            code_size: self.code_size,
             description: self.description,
             timeout: self.timeout,
             memory_size: self.memory_size,
@@ -1012,3 +1059,50 @@
         }
     }
 }
+impl ::std::fmt::Debug for UpdateFunctionCodeOutputBuilder {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("UpdateFunctionCodeOutputBuilder");
+        formatter.field("function_name", &self.function_name);
+        formatter.field("function_arn", &self.function_arn);
+        formatter.field("runtime", &self.runtime);
+        formatter.field("role", &self.role);
+        formatter.field("handler", &self.handler);
+        formatter.field("code_size", &self.code_size);
+        formatter.field("description", &self.description);
+        formatter.field("timeout", &self.timeout);
+        formatter.field("memory_size", &self.memory_size);
+        formatter.field("last_modified", &self.last_modified);
+        formatter.field("code_sha256", &self.code_sha256);
+        formatter.field("version", &self.version);
+        formatter.field("vpc_config", &self.vpc_config);
+        formatter.field("dead_letter_config", &self.dead_letter_config);
+        formatter.field("environment", &"*** Sensitive Data Redacted ***");
+        formatter.field("kms_key_arn", &self.kms_key_arn);
+        formatter.field("tracing_config", &self.tracing_config);
+        formatter.field("master_arn", &self.master_arn);
+        formatter.field("revision_id", &self.revision_id);
+        formatter.field("layers", &self.layers);
+        formatter.field("state", &self.state);
+        formatter.field("state_reason", &self.state_reason);
+        formatter.field("state_reason_code", &self.state_reason_code);
+        formatter.field("last_update_status", &self.last_update_status);
+        formatter.field("last_update_status_reason", &self.last_update_status_reason);
+        formatter.field("last_update_status_reason_code", &self.last_update_status_reason_code);
+        formatter.field("file_system_configs", &self.file_system_configs);
+        formatter.field("signing_profile_version_arn", &self.signing_profile_version_arn);
+        formatter.field("signing_job_arn", &self.signing_job_arn);
+        formatter.field("package_type", &self.package_type);
+        formatter.field("image_config_response", &"*** Sensitive Data Redacted ***");
+        formatter.field("architectures", &self.architectures);
+        formatter.field("ephemeral_storage", &self.ephemeral_storage);
+        formatter.field("snap_start", &self.snap_start);
+        formatter.field("runtime_version_config", &"*** Sensitive Data Redacted ***");
+        formatter.field("logging_config", &self.logging_config);
+        formatter.field("tenancy_config", &self.tenancy_config);
+        formatter.field("capacity_provider_config", &self.capacity_provider_config);
+        formatter.field("config_sha256", &self.config_sha256);
+        formatter.field("durable_config", &self.durable_config);
+        formatter.field("_request_id", &self._request_id);
+        formatter.finish()
+    }
+}
```

### `src/operation/update_function_code/builders.rs`

```diff
--- reference/src/operation/update_function_code/builders.rs
+++ generated/src/operation/update_function_code/builders.rs
@@ -159,17 +159,17 @@
         self.inner.get_function_name()
     }
     /// <p>The base64-encoded contents of the deployment package. Amazon Web Services SDK and CLI clients handle the encoding for you. Use only with a function defined with a .zip file archive deployment package.</p>
-    pub fn zip_file(mut self, input: ::aws_smithy_types::Blob) -> Self {
+    pub fn zip_file(mut self, input: ::std::vec::Vec<u8>) -> Self {
         self.inner = self.inner.zip_file(input);
         self
     }
     /// <p>The base64-encoded contents of the deployment package. Amazon Web Services SDK and CLI clients handle the encoding for you. Use only with a function defined with a .zip file archive deployment package.</p>
-    pub fn set_zip_file(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
+    pub fn set_zip_file(mut self, input: ::std::option::Option<::std::vec::Vec<u8>>) -> Self {
         self.inner = self.inner.set_zip_file(input);
         self
     }
     /// <p>The base64-encoded contents of the deployment package. Amazon Web Services SDK and CLI clients handle the encoding for you. Use only with a function defined with a .zip file archive deployment package.</p>
-    pub fn get_zip_file(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
+    pub fn get_zip_file(&self) -> &::std::option::Option<::std::vec::Vec<u8>> {
         self.inner.get_zip_file()
     }
     /// <p>An Amazon S3 bucket in the same Amazon Web Services Region as your function. The bucket can be in a different Amazon Web Services account. Use only with a function defined with a .zip file archive deployment package.</p>
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

### `src/operation/update_function_configuration/_update_function_configuration_input.rs`

```diff
--- reference/src/operation/update_function_configuration/_update_function_configuration_input.rs
+++ generated/src/operation/update_function_configuration/_update_function_configuration_input.rs
@@ -1,7 +1,7 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 #[allow(missing_docs)] // documentation missing in model
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct UpdateFunctionConfigurationInput {
     /// <p>The name or ARN of the Lambda function.</p>
     /// <p class="title"><b>Name formats</b></p>
@@ -182,6 +182,33 @@
         self.durable_config.as_ref()
     }
 }
+impl ::std::fmt::Debug for UpdateFunctionConfigurationInput {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("UpdateFunctionConfigurationInput");
+        formatter.field("function_name", &self.function_name);
+        formatter.field("role", &self.role);
+        formatter.field("handler", &self.handler);
+        formatter.field("description", &self.description);
+        formatter.field("timeout", &self.timeout);
+        formatter.field("memory_size", &self.memory_size);
+        formatter.field("vpc_config", &self.vpc_config);
+        formatter.field("environment", &"*** Sensitive Data Redacted ***");
+        formatter.field("runtime", &self.runtime);
+        formatter.field("dead_letter_config", &self.dead_letter_config);
+        formatter.field("kms_key_arn", &self.kms_key_arn);
+        formatter.field("tracing_config", &self.tracing_config);
+        formatter.field("revision_id", &self.revision_id);
+        formatter.field("layers", &self.layers);
+        formatter.field("file_system_configs", &self.file_system_configs);
+        formatter.field("image_config", &self.image_config);
+        formatter.field("ephemeral_storage", &self.ephemeral_storage);
+        formatter.field("snap_start", &self.snap_start);
+        formatter.field("logging_config", &self.logging_config);
+        formatter.field("capacity_provider_config", &self.capacity_provider_config);
+        formatter.field("durable_config", &self.durable_config);
+        formatter.finish()
+    }
+}
 impl UpdateFunctionConfigurationInput {
     /// Creates a new builder-style object to manufacture [`UpdateFunctionConfigurationInput`](crate::operation::update_function_configuration::UpdateFunctionConfigurationInput).
     pub fn builder() -> crate::operation::update_function_configuration::builders::UpdateFunctionConfigurationInputBuilder {
@@ -190,7 +217,7 @@
 }

 /// A builder for [`UpdateFunctionConfigurationInput`](crate::operation::update_function_configuration::UpdateFunctionConfigurationInput).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
 #[non_exhaustive]
 pub struct UpdateFunctionConfigurationInputBuilder {
     pub(crate) function_name: ::std::option::Option<::std::string::String>,
@@ -624,3 +651,30 @@
         })
     }
 }
+impl ::std::fmt::Debug for UpdateFunctionConfigurationInputBuilder {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("UpdateFunctionConfigurationInputBuilder");
+        formatter.field("function_name", &self.function_name);
+        formatter.field("role", &self.role);
+        formatter.field("handler", &self.handler);
+        formatter.field("description", &self.description);
+        formatter.field("timeout", &self.timeout);
+        formatter.field("memory_size", &self.memory_size);
+        formatter.field("vpc_config", &self.vpc_config);
+        formatter.field("environment", &"*** Sensitive Data Redacted ***");
+        formatter.field("runtime", &self.runtime);
+        formatter.field("dead_letter_config", &self.dead_letter_config);
+        formatter.field("kms_key_arn", &self.kms_key_arn);
+        formatter.field("tracing_config", &self.tracing_config);
+        formatter.field("revision_id", &self.revision_id);
+        formatter.field("layers", &self.layers);
+        formatter.field("file_system_configs", &self.file_system_configs);
+        formatter.field("image_config", &self.image_config);
+        formatter.field("ephemeral_storage", &self.ephemeral_storage);
+        formatter.field("snap_start", &self.snap_start);
+        formatter.field("logging_config", &self.logging_config);
+        formatter.field("capacity_provider_config", &self.capacity_provider_config);
+        formatter.field("durable_config", &self.durable_config);
+        formatter.finish()
+    }
+}
```

### `src/operation/update_function_configuration/_update_function_configuration_output.rs`

```diff
--- reference/src/operation/update_function_configuration/_update_function_configuration_output.rs
+++ generated/src/operation/update_function_configuration/_update_function_configuration_output.rs
@@ -2,7 +2,7 @@

 /// <p>Details about a function's configuration.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct UpdateFunctionConfigurationOutput {
     /// <p>The name of the function.</p>
     pub function_name: ::std::option::Option<::std::string::String>,
@@ -17,7 +17,7 @@
     /// <p>The function that Lambda calls to begin running your function.</p>
     pub handler: ::std::option::Option<::std::string::String>,
     /// <p>The size of the function's deployment package, in bytes.</p>
-    pub code_size: i64,
+    pub code_size: ::std::option::Option<i64>,
     /// <p>The function's description.</p>
     pub description: ::std::option::Option<::std::string::String>,
     /// <p>The amount of time in seconds that Lambda allows a function to run before stopping it.</p>
@@ -123,7 +123,7 @@
         self.handler.as_deref()
     }
     /// <p>The size of the function's deployment package, in bytes.</p>
-    pub fn code_size(&self) -> i64 {
+    pub fn code_size(&self) -> ::std::option::Option<i64> {
         self.code_size
     }
     /// <p>The function's description.</p>
@@ -280,6 +280,53 @@
         self.durable_config.as_ref()
     }
 }
+impl ::std::fmt::Debug for UpdateFunctionConfigurationOutput {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("UpdateFunctionConfigurationOutput");
+        formatter.field("function_name", &self.function_name);
+        formatter.field("function_arn", &self.function_arn);
+        formatter.field("runtime", &self.runtime);
+        formatter.field("role", &self.role);
+        formatter.field("handler", &self.handler);
+        formatter.field("code_size", &self.code_size);
+        formatter.field("description", &self.description);
+        formatter.field("timeout", &self.timeout);
+        formatter.field("memory_size", &self.memory_size);
+        formatter.field("last_modified", &self.last_modified);
+        formatter.field("code_sha256", &self.code_sha256);
+        formatter.field("version", &self.version);
+        formatter.field("vpc_config", &self.vpc_config);
+        formatter.field("dead_letter_config", &self.dead_letter_config);
+        formatter.field("environment", &"*** Sensitive Data Redacted ***");
+        formatter.field("kms_key_arn", &self.kms_key_arn);
+        formatter.field("tracing_config", &self.tracing_config);
+        formatter.field("master_arn", &self.master_arn);
+        formatter.field("revision_id", &self.revision_id);
+        formatter.field("layers", &self.layers);
+        formatter.field("state", &self.state);
+        formatter.field("state_reason", &self.state_reason);
+        formatter.field("state_reason_code", &self.state_reason_code);
+        formatter.field("last_update_status", &self.last_update_status);
+        formatter.field("last_update_status_reason", &self.last_update_status_reason);
+        formatter.field("last_update_status_reason_code", &self.last_update_status_reason_code);
+        formatter.field("file_system_configs", &self.file_system_configs);
+        formatter.field("signing_profile_version_arn", &self.signing_profile_version_arn);
+        formatter.field("signing_job_arn", &self.signing_job_arn);
+        formatter.field("package_type", &self.package_type);
+        formatter.field("image_config_response", &"*** Sensitive Data Redacted ***");
+        formatter.field("architectures", &self.architectures);
+        formatter.field("ephemeral_storage", &self.ephemeral_storage);
+        formatter.field("snap_start", &self.snap_start);
+        formatter.field("runtime_version_config", &"*** Sensitive Data Redacted ***");
+        formatter.field("logging_config", &self.logging_config);
+        formatter.field("tenancy_config", &self.tenancy_config);
+        formatter.field("capacity_provider_config", &self.capacity_provider_config);
+        formatter.field("config_sha256", &self.config_sha256);
+        formatter.field("durable_config", &self.durable_config);
+        formatter.field("_request_id", &self._request_id);
+        formatter.finish()
+    }
+}
 impl ::aws_types::request_id::RequestId for UpdateFunctionConfigurationOutput {
     fn request_id(&self) -> Option<&str> {
         self._request_id.as_deref()
@@ -293,7 +340,7 @@
 }

 /// A builder for [`UpdateFunctionConfigurationOutput`](crate::operation::update_function_configuration::UpdateFunctionConfigurationOutput).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
 #[non_exhaustive]
 pub struct UpdateFunctionConfigurationOutputBuilder {
     pub(crate) function_name: ::std::option::Option<::std::string::String>,
@@ -973,7 +1020,7 @@
             runtime: self.runtime,
             role: self.role,
             handler: self.handler,
-            code_size: self.code_size.unwrap_or_default(),
+            code_size: self.code_size,
             description: self.description,
             timeout: self.timeout,
             memory_size: self.memory_size,
@@ -1012,3 +1059,50 @@
         }
     }
 }
+impl ::std::fmt::Debug for UpdateFunctionConfigurationOutputBuilder {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("UpdateFunctionConfigurationOutputBuilder");
+        formatter.field("function_name", &self.function_name);
+        formatter.field("function_arn", &self.function_arn);
+        formatter.field("runtime", &self.runtime);
+        formatter.field("role", &self.role);
+        formatter.field("handler", &self.handler);
+        formatter.field("code_size", &self.code_size);
+        formatter.field("description", &self.description);
+        formatter.field("timeout", &self.timeout);
+        formatter.field("memory_size", &self.memory_size);
+        formatter.field("last_modified", &self.last_modified);
+        formatter.field("code_sha256", &self.code_sha256);
+        formatter.field("version", &self.version);
+        formatter.field("vpc_config", &self.vpc_config);
+        formatter.field("dead_letter_config", &self.dead_letter_config);
+        formatter.field("environment", &"*** Sensitive Data Redacted ***");
+        formatter.field("kms_key_arn", &self.kms_key_arn);
+        formatter.field("tracing_config", &self.tracing_config);
+        formatter.field("master_arn", &self.master_arn);
+        formatter.field("revision_id", &self.revision_id);
+        formatter.field("layers", &self.layers);
+        formatter.field("state", &self.state);
+        formatter.field("state_reason", &self.state_reason);
+        formatter.field("state_reason_code", &self.state_reason_code);
+        formatter.field("last_update_status", &self.last_update_status);
+        formatter.field("last_update_status_reason", &self.last_update_status_reason);
+        formatter.field("last_update_status_reason_code", &self.last_update_status_reason_code);
+        formatter.field("file_system_configs", &self.file_system_configs);
+        formatter.field("signing_profile_version_arn", &self.signing_profile_version_arn);
+        formatter.field("signing_job_arn", &self.signing_job_arn);
+        formatter.field("package_type", &self.package_type);
+        formatter.field("image_config_response", &"*** Sensitive Data Redacted ***");
+        formatter.field("architectures", &self.architectures);
+        formatter.field("ephemeral_storage", &self.ephemeral_storage);
+        formatter.field("snap_start", &self.snap_start);
+        formatter.field("runtime_version_config", &"*** Sensitive Data Redacted ***");
+        formatter.field("logging_config", &self.logging_config);
+        formatter.field("tenancy_config", &self.tenancy_config);
+        formatter.field("capacity_provider_config", &self.capacity_provider_config);
+        formatter.field("config_sha256", &self.config_sha256);
+        formatter.field("durable_config", &self.durable_config);
+        formatter.field("_request_id", &self._request_id);
+        formatter.finish()
+    }
+}
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

### `src/operation.rs`

```diff
--- reference/src/operation.rs
+++ generated/src/operation.rs
@@ -1,266 +1,351 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-pub use ::aws_types::request_id::RequestId;
-
-/// Types for the `AddLayerVersionPermission` operation.
-pub mod add_layer_version_permission;
-
-/// Types for the `AddPermission` operation.
-pub mod add_permission;
-
-/// Types for the `CheckpointDurableExecution` operation.
-pub mod checkpoint_durable_execution;

-/// Types for the `CreateAlias` operation.
-pub mod create_alias;
-
-/// Types for the `CreateCapacityProvider` operation.
-pub mod create_capacity_provider;
-
-/// Types for the `CreateCodeSigningConfig` operation.
-pub mod create_code_signing_config;
-
-/// Types for the `CreateEventSourceMapping` operation.
-pub mod create_event_source_mapping;
-
-/// Types for the `CreateFunction` operation.
-pub mod create_function;
-
-/// Types for the `CreateFunctionUrlConfig` operation.
-pub mod create_function_url_config;
-
-/// Types for the `DeleteAlias` operation.
-pub mod delete_alias;
-
-/// Types for the `DeleteCapacityProvider` operation.
-pub mod delete_capacity_provider;
-
-/// Types for the `DeleteCodeSigningConfig` operation.
-pub mod delete_code_signing_config;
-
-/// Types for the `DeleteEventSourceMapping` operation.
-pub mod delete_event_source_mapping;
-
-/// Types for the `DeleteFunction` operation.
-pub mod delete_function;
-
-/// Types for the `DeleteFunctionCodeSigningConfig` operation.
-pub mod delete_function_code_signing_config;
-
-/// Types for the `DeleteFunctionConcurrency` operation.
-pub mod delete_function_concurrency;
-
-/// Types for the `DeleteFunctionEventInvokeConfig` operation.
-pub mod delete_function_event_invoke_config;
-
-/// Types for the `DeleteFunctionUrlConfig` operation.
-pub mod delete_function_url_config;
-
-/// Types for the `DeleteLayerVersion` operation.
-pub mod delete_layer_version;
-
-/// Types for the `DeleteProvisionedConcurrencyConfig` operation.
-pub mod delete_provisioned_concurrency_config;
-
-/// Types for the `DeleteResourcePolicy` operation.
-pub mod delete_resource_policy;
-
-/// Types for the `GetAccountSettings` operation.
-pub mod get_account_settings;
-
-/// Types for the `GetAlias` operation.
-pub mod get_alias;
-
-/// Types for the `GetCapacityProvider` operation.
-pub mod get_capacity_provider;
-
-/// Types for the `GetCodeSigningConfig` operation.
-pub mod get_code_signing_config;
-
-/// Types for the `GetDurableExecution` operation.
-pub mod get_durable_execution;
+pub use ::aws_types::request_id::RequestId;

-/// Types for the `GetDurableExecutionHistory` operation.
-pub mod get_durable_execution_history;
-
-/// Types for the `GetDurableExecutionState` operation.
-pub mod get_durable_execution_state;
-
-/// Types for the `GetEventSourceMapping` operation.
-pub mod get_event_source_mapping;
-
-/// Types for the `GetFunction` operation.
-pub mod get_function;
-
-/// Types for the `GetFunctionCodeSigningConfig` operation.
-pub mod get_function_code_signing_config;
-
-/// Types for the `GetFunctionConcurrency` operation.
-pub mod get_function_concurrency;
-
-/// Types for the `GetFunctionConfiguration` operation.
-pub mod get_function_configuration;
-
-/// Types for the `GetFunctionEventInvokeConfig` operation.
-pub mod get_function_event_invoke_config;
-
-/// Types for the `GetFunctionRecursionConfig` operation.
-pub mod get_function_recursion_config;
-
-/// Types for the `GetFunctionScalingConfig` operation.
-pub mod get_function_scaling_config;
-
-/// Types for the `GetFunctionUrlConfig` operation.
-pub mod get_function_url_config;
-
-/// Types for the `GetLayerVersion` operation.
-pub mod get_layer_version;
-
-/// Types for the `GetLayerVersionByArn` operation.
-pub mod get_layer_version_by_arn;
-
-/// Types for the `GetLayerVersionPolicy` operation.
-pub mod get_layer_version_policy;
-
-/// Types for the `GetPolicy` operation.
-pub mod get_policy;
-
-/// Types for the `GetProvisionedConcurrencyConfig` operation.
-pub mod get_provisioned_concurrency_config;
-
-/// Types for the `GetResourcePolicy` operation.
-pub mod get_resource_policy;
-
-/// Types for the `GetRuntimeManagementConfig` operation.
-pub mod get_runtime_management_config;
-
-/// Types for the `Invoke` operation.
-pub mod invoke;
-
-/// Types for the `InvokeAsync` operation.
-pub mod invoke_async;
-
-/// Types for the `InvokeWithResponseStream` operation.
-pub mod invoke_with_response_stream;
-
-/// Types for the `ListAliases` operation.
-pub mod list_aliases;
-
-/// Types for the `ListCapacityProviders` operation.
-pub mod list_capacity_providers;
-
-/// Types for the `ListCodeSigningConfigs` operation.
-pub mod list_code_signing_configs;
-
-/// Types for the `ListDurableExecutionsByFunction` operation.
-pub mod list_durable_executions_by_function;
-
-/// Types for the `ListEventSourceMappings` operation.
-pub mod list_event_source_mappings;
-
-/// Types for the `ListFunctionEventInvokeConfigs` operation.
-pub mod list_function_event_invoke_configs;
-
-/// Types for the `ListFunctionUrlConfigs` operation.
-pub mod list_function_url_configs;
-
-/// Types for the `ListFunctionVersionsByCapacityProvider` operation.
-pub mod list_function_versions_by_capacity_provider;
-
-/// Types for the `ListFunctions` operation.
-pub mod list_functions;
-
-/// Types for the `ListFunctionsByCodeSigningConfig` operation.
-pub mod list_functions_by_code_signing_config;
-
-/// Types for the `ListLayerVersions` operation.
-pub mod list_layer_versions;
-
-/// Types for the `ListLayers` operation.
-pub mod list_layers;
-
-/// Types for the `ListProvisionedConcurrencyConfigs` operation.
-pub mod list_provisioned_concurrency_configs;
-
-/// Types for the `ListTags` operation.
-pub mod list_tags;
-
-/// Types for the `ListVersionsByFunction` operation.
-pub mod list_versions_by_function;
-
-/// Types for the `PublishLayerVersion` operation.
-pub mod publish_layer_version;
-
-/// Types for the `PublishVersion` operation.
-pub mod publish_version;
-
-/// Types for the `PutFunctionCodeSigningConfig` operation.
-pub mod put_function_code_signing_config;
-
-/// Types for the `PutFunctionConcurrency` operation.
-pub mod put_function_concurrency;
-
-/// Types for the `PutFunctionEventInvokeConfig` operation.
-pub mod put_function_event_invoke_config;
-
-/// Types for the `PutFunctionRecursionConfig` operation.
-pub mod put_function_recursion_config;
-
-/// Types for the `PutFunctionScalingConfig` operation.
-pub mod put_function_scaling_config;
-
-/// Types for the `PutProvisionedConcurrencyConfig` operation.
-pub mod put_provisioned_concurrency_config;
-
-/// Types for the `PutResourcePolicy` operation.
-pub mod put_resource_policy;
-
-/// Types for the `PutRuntimeManagementConfig` operation.
-pub mod put_runtime_management_config;
-
-/// Types for the `RemoveLayerVersionPermission` operation.
-pub mod remove_layer_version_permission;
-
-/// Types for the `RemovePermission` operation.
-pub mod remove_permission;
-
-/// Types for the `SendDurableExecutionCallbackFailure` operation.
-pub mod send_durable_execution_callback_failure;
-
-/// Types for the `SendDurableExecutionCallbackHeartbeat` operation.
-pub mod send_durable_execution_callback_heartbeat;
-
-/// Types for the `SendDurableExecutionCallbackSuccess` operation.
-pub mod send_durable_execution_callback_success;
-
-/// Types for the `StopDurableExecution` operation.
-pub mod stop_durable_execution;
-
-/// Types for the `TagResource` operation.
-pub mod tag_resource;
-
-/// Types for the `UntagResource` operation.
-pub mod untag_resource;
-
-/// Types for the `UpdateAlias` operation.
-pub mod update_alias;
-
-/// Types for the `UpdateCapacityProvider` operation.
-pub mod update_capacity_provider;
-
-/// Types for the `UpdateCodeSigningConfig` operation.
-pub mod update_code_signing_config;
-
-/// Types for the `UpdateEventSourceMapping` operation.
-pub mod update_event_source_mapping;
-
-/// Types for the `UpdateFunctionCode` operation.
-pub mod update_function_code;
-
-/// Types for the `UpdateFunctionConfiguration` operation.
-pub mod update_function_configuration;
-
-/// Types for the `UpdateFunctionEventInvokeConfig` operation.
-pub mod update_function_event_invoke_config;
-
-/// Types for the `UpdateFunctionUrlConfig` operation.
-pub mod update_function_url_config;
+pub mod operation {
+    pub mod add_layer_version_permission {
+        include!(concat!(
+            env!("OUT_DIR"),
+            "/generated/lambda/src/operation/add_layer_version_permission.rs"
+        ));
+    }
+    pub mod add_permission {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/add_permission.rs"));
+    }
+    pub mod checkpoint_durable_execution {
+        include!(concat!(
+            env!("OUT_DIR"),
+            "/generated/lambda/src/operation/checkpoint_durable_execution.rs"
+        ));
+    }
+    pub mod create_alias {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/create_alias.rs"));
+    }
+    pub mod create_capacity_provider {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/create_capacity_provider.rs"));
+    }
+    pub mod create_code_signing_config {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/create_code_signing_config.rs"));
+    }
+    pub mod create_event_source_mapping {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/create_event_source_mapping.rs"));
+    }
+    pub mod create_function {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/create_function.rs"));
+    }
+    pub mod create_function_url_config {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/create_function_url_config.rs"));
+    }
+    pub mod delete_alias {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/delete_alias.rs"));
+    }
+    pub mod delete_capacity_provider {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/delete_capacity_provider.rs"));
+    }
+    pub mod delete_code_signing_config {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/delete_code_signing_config.rs"));
+    }
+    pub mod delete_event_source_mapping {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/delete_event_source_mapping.rs"));
+    }
+    pub mod delete_function {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/delete_function.rs"));
+    }
+    pub mod delete_function_code_signing_config {
+        include!(concat!(
+            env!("OUT_DIR"),
+            "/generated/lambda/src/operation/delete_function_code_signing_config.rs"
+        ));
+    }
+    pub mod delete_function_concurrency {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/delete_function_concurrency.rs"));
+    }
+    pub mod delete_function_event_invoke_config {
+        include!(concat!(
+            env!("OUT_DIR"),
+            "/generated/lambda/src/operation/delete_function_event_invoke_config.rs"
+        ));
+    }
+    pub mod delete_function_url_config {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/delete_function_url_config.rs"));
+    }
+    pub mod delete_layer_version {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/delete_layer_version.rs"));
+    }
+    pub mod delete_provisioned_concurrency_config {
+        include!(concat!(
+            env!("OUT_DIR"),
+            "/generated/lambda/src/operation/delete_provisioned_concurrency_config.rs"
+        ));
+    }
+    pub mod delete_resource_policy {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/delete_resource_policy.rs"));
+    }
+    pub mod get_account_settings {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/get_account_settings.rs"));
+    }
+    pub mod get_alias {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/get_alias.rs"));
+    }
+    pub mod get_capacity_provider {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/get_capacity_provider.rs"));
+    }
+    pub mod get_code_signing_config {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/get_code_signing_config.rs"));
+    }
+    pub mod get_durable_execution {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/get_durable_execution.rs"));
+    }
+    pub mod get_durable_execution_history {
+        include!(concat!(
+            env!("OUT_DIR"),
+            "/generated/lambda/src/operation/get_durable_execution_history.rs"
+        ));
+    }
+    pub mod get_durable_execution_state {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/get_durable_execution_state.rs"));
+    }
+    pub mod get_event_source_mapping {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/get_event_source_mapping.rs"));
+    }
+    pub mod get_function {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/get_function.rs"));
+    }
+    pub mod get_function_code_signing_config {
+        include!(concat!(
+            env!("OUT_DIR"),
+            "/generated/lambda/src/operation/get_function_code_signing_config.rs"
+        ));
+    }
+    pub mod get_function_concurrency {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/get_function_concurrency.rs"));
+    }
+    pub mod get_function_configuration {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/get_function_configuration.rs"));
+    }
+    pub mod get_function_event_invoke_config {
+        include!(concat!(
+            env!("OUT_DIR"),
+            "/generated/lambda/src/operation/get_function_event_invoke_config.rs"
+        ));
+    }
+    pub mod get_function_recursion_config {
+        include!(concat!(
+            env!("OUT_DIR"),
+            "/generated/lambda/src/operation/get_function_recursion_config.rs"
+        ));
+    }
+    pub mod get_function_scaling_config {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/get_function_scaling_config.rs"));
+    }
+    pub mod get_function_url_config {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/get_function_url_config.rs"));
+    }
+    pub mod get_layer_version {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/get_layer_version.rs"));
+    }
+    pub mod get_layer_version_by_arn {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/get_layer_version_by_arn.rs"));
+    }
+    pub mod get_layer_version_policy {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/get_layer_version_policy.rs"));
+    }
+    pub mod get_policy {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/get_policy.rs"));
+    }
+    pub mod get_provisioned_concurrency_config {
+        include!(concat!(
+            env!("OUT_DIR"),
+            "/generated/lambda/src/operation/get_provisioned_concurrency_config.rs"
+        ));
+    }
+    pub mod get_resource_policy {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/get_resource_policy.rs"));
+    }
+    pub mod get_runtime_management_config {
+        include!(concat!(
+            env!("OUT_DIR"),
+            "/generated/lambda/src/operation/get_runtime_management_config.rs"
+        ));
+    }
+    pub mod invoke {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/invoke.rs"));
+    }
+    pub mod invoke_async {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/invoke_async.rs"));
+    }
+    pub mod invoke_with_response_stream {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/invoke_with_response_stream.rs"));
+    }
+    pub mod list_aliases {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/list_aliases.rs"));
+    }
+    pub mod list_capacity_providers {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/list_capacity_providers.rs"));
+    }
+    pub mod list_code_signing_configs {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/list_code_signing_configs.rs"));
+    }
+    pub mod list_durable_executions_by_function {
+        include!(concat!(
+            env!("OUT_DIR"),
+            "/generated/lambda/src/operation/list_durable_executions_by_function.rs"
+        ));
+    }
+    pub mod list_event_source_mappings {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/list_event_source_mappings.rs"));
+    }
+    pub mod list_function_event_invoke_configs {
+        include!(concat!(
+            env!("OUT_DIR"),
+            "/generated/lambda/src/operation/list_function_event_invoke_configs.rs"
+        ));
+    }
+    pub mod list_function_url_configs {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/list_function_url_configs.rs"));
+    }
+    pub mod list_function_versions_by_capacity_provider {
+        include!(concat!(
+            env!("OUT_DIR"),
+            "/generated/lambda/src/operation/list_function_versions_by_capacity_provider.rs"
+        ));
+    }
+    pub mod list_functions {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/list_functions.rs"));
+    }
+    pub mod list_functions_by_code_signing_config {
+        include!(concat!(
+            env!("OUT_DIR"),
+            "/generated/lambda/src/operation/list_functions_by_code_signing_config.rs"
+        ));
+    }
+    pub mod list_layer_versions {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/list_layer_versions.rs"));
+    }
+    pub mod list_layers {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/list_layers.rs"));
+    }
+    pub mod list_provisioned_concurrency_configs {
+        include!(concat!(
+            env!("OUT_DIR"),
+            "/generated/lambda/src/operation/list_provisioned_concurrency_configs.rs"
+        ));
+    }
+    pub mod list_tags {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/list_tags.rs"));
+    }
+    pub mod list_versions_by_function {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/list_versions_by_function.rs"));
+    }
+    pub mod publish_layer_version {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/publish_layer_version.rs"));
+    }
+    pub mod publish_version {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/publish_version.rs"));
+    }
+    pub mod put_function_code_signing_config {
+        include!(concat!(
+            env!("OUT_DIR"),
+            "/generated/lambda/src/operation/put_function_code_signing_config.rs"
+        ));
+    }
+    pub mod put_function_concurrency {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/put_function_concurrency.rs"));
+    }
+    pub mod put_function_event_invoke_config {
+        include!(concat!(
+            env!("OUT_DIR"),
+            "/generated/lambda/src/operation/put_function_event_invoke_config.rs"
+        ));
+    }
+    pub mod put_function_recursion_config {
+        include!(concat!(
+            env!("OUT_DIR"),
+            "/generated/lambda/src/operation/put_function_recursion_config.rs"
+        ));
+    }
+    pub mod put_function_scaling_config {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/put_function_scaling_config.rs"));
+    }
+    pub mod put_provisioned_concurrency_config {
+        include!(concat!(
+            env!("OUT_DIR"),
+            "/generated/lambda/src/operation/put_provisioned_concurrency_config.rs"
+        ));
+    }
+    pub mod put_resource_policy {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/put_resource_policy.rs"));
+    }
+    pub mod put_runtime_management_config {
+        include!(concat!(
+            env!("OUT_DIR"),
+            "/generated/lambda/src/operation/put_runtime_management_config.rs"
+        ));
+    }
+    pub mod remove_layer_version_permission {
+        include!(concat!(
+            env!("OUT_DIR"),
+            "/generated/lambda/src/operation/remove_layer_version_permission.rs"
+        ));
+    }
+    pub mod remove_permission {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/remove_permission.rs"));
+    }
+    pub mod send_durable_execution_callback_failure {
+        include!(concat!(
+            env!("OUT_DIR"),
+            "/generated/lambda/src/operation/send_durable_execution_callback_failure.rs"
+        ));
+    }
+    pub mod send_durable_execution_callback_heartbeat {
+        include!(concat!(
+            env!("OUT_DIR"),
+            "/generated/lambda/src/operation/send_durable_execution_callback_heartbeat.rs"
+        ));
+    }
+    pub mod send_durable_execution_callback_success {
+        include!(concat!(
+            env!("OUT_DIR"),
+            "/generated/lambda/src/operation/send_durable_execution_callback_success.rs"
+        ));
+    }
+    pub mod stop_durable_execution {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/stop_durable_execution.rs"));
+    }
+    pub mod tag_resource {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/tag_resource.rs"));
+    }
+    pub mod untag_resource {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/untag_resource.rs"));
+    }
+    pub mod update_alias {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/update_alias.rs"));
+    }
+    pub mod update_capacity_provider {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/update_capacity_provider.rs"));
+    }
+    pub mod update_code_signing_config {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/update_code_signing_config.rs"));
+    }
+    pub mod update_event_source_mapping {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/update_event_source_mapping.rs"));
+    }
+    pub mod update_function_code {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/update_function_code.rs"));
+    }
+    pub mod update_function_configuration {
+        include!(concat!(
+            env!("OUT_DIR"),
+            "/generated/lambda/src/operation/update_function_configuration.rs"
+        ));
+    }
+    pub mod update_function_event_invoke_config {
+        include!(concat!(
+            env!("OUT_DIR"),
+            "/generated/lambda/src/operation/update_function_event_invoke_config.rs"
+        ));
+    }
+    pub mod update_function_url_config {
+        include!(concat!(env!("OUT_DIR"), "/generated/lambda/src/operation/update_function_url_config.rs"));
+    }
+}
```

### `src/serde_util.rs`

```diff
--- reference/src/serde_util.rs
+++ generated/src/serde_util.rs
@@ -269,6 +269,22 @@
     builder
 }

+pub(crate) fn operation_correct_errors(mut builder: crate::types::builders::OperationBuilder) -> crate::types::builders::OperationBuilder {
+    if builder.id.is_none() {
+        builder.id = Some(Default::default())
+    }
+    if builder.r#type.is_none() {
+        builder.r#type = "no value was set".parse::<crate::types::OperationType>().ok()
+    }
+    if builder.start_timestamp.is_none() {
+        builder.start_timestamp = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
+    }
+    if builder.status.is_none() {
+        builder.status = "no value was set".parse::<crate::types::OperationStatus>().ok()
+    }
+    builder
+}
+
 pub(crate) fn capacity_provider_correct_errors(
     mut builder: crate::types::builders::CapacityProviderBuilder,
 ) -> crate::types::builders::CapacityProviderBuilder {
@@ -295,6 +311,39 @@
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
+pub(crate) fn target_tracking_scaling_policy_correct_errors(
+    mut builder: crate::types::builders::TargetTrackingScalingPolicyBuilder,
+) -> crate::types::builders::TargetTrackingScalingPolicyBuilder {
+    if builder.predefined_metric_type.is_none() {
+        builder.predefined_metric_type = "no value was set".parse::<crate::types::CapacityProviderPredefinedMetricType>().ok()
+    }
+    if builder.target_value.is_none() {
+        builder.target_value = Some(Default::default())
+    }
+    builder
+}
+
 pub(crate) fn code_signing_config_correct_errors(
     mut builder: crate::types::builders::CodeSigningConfigBuilder,
 ) -> crate::types::builders::CodeSigningConfigBuilder {
@@ -322,16 +371,23 @@
     builder
 }

-pub(crate) fn capacity_provider_config_correct_errors(
-    mut builder: crate::types::builders::CapacityProviderConfigBuilder,
-) -> crate::types::builders::CapacityProviderConfigBuilder {
-    if builder.lambda_managed_instances_capacity_provider_config.is_none() {
-        builder.lambda_managed_instances_capacity_provider_config = {
-            let builder = crate::types::builders::LambdaManagedInstancesCapacityProviderConfigBuilder::default();
-            crate::serde_util::lambda_managed_instances_capacity_provider_config_correct_errors(builder)
-                .build()
-                .ok()
-        }
+pub(crate) fn allowed_publishers_correct_errors(
+    mut builder: crate::types::builders::AllowedPublishersBuilder,
+) -> crate::types::builders::AllowedPublishersBuilder {
+    if builder.signing_profile_version_arns.is_none() {
+        builder.signing_profile_version_arns = Some(Default::default())
+    }
+    builder
+}
+
+pub(crate) fn file_system_config_correct_errors(
+    mut builder: crate::types::builders::FileSystemConfigBuilder,
+) -> crate::types::builders::FileSystemConfigBuilder {
+    if builder.arn.is_none() {
+        builder.arn = Some(Default::default())
+    }
+    if builder.local_mount_path.is_none() {
+        builder.local_mount_path = Some(Default::default())
     }
     builder
 }
@@ -345,16 +401,6 @@
     builder
 }

-pub(crate) fn tags_error_correct_errors(mut builder: crate::types::builders::TagsErrorBuilder) -> crate::types::builders::TagsErrorBuilder {
-    if builder.error_code.is_none() {
-        builder.error_code = Some(Default::default())
-    }
-    if builder.message.is_none() {
-        builder.message = Some(Default::default())
-    }
-    builder
-}
-
 pub(crate) fn tenancy_config_correct_errors(
     mut builder: crate::types::builders::TenancyConfigBuilder,
 ) -> crate::types::builders::TenancyConfigBuilder {
@@ -364,128 +410,95 @@
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
+pub(crate) fn capacity_provider_config_correct_errors(
+    mut builder: crate::types::builders::CapacityProviderConfigBuilder,
+) -> crate::types::builders::CapacityProviderConfigBuilder {
+    if builder.lambda_managed_instances_capacity_provider_config.is_none() {
+        builder.lambda_managed_instances_capacity_provider_config = {
+            let builder = crate::types::builders::LambdaManagedInstancesCapacityProviderConfigBuilder::default();
+            crate::serde_util::lambda_managed_instances_capacity_provider_config_correct_errors(builder)
+                .build()
+                .ok()
+        }
     }
     builder
 }

-pub(crate) fn allowed_publishers_correct_errors(
-    mut builder: crate::types::builders::AllowedPublishersBuilder,
-) -> crate::types::builders::AllowedPublishersBuilder {
-    if builder.signing_profile_version_arns.is_none() {
-        builder.signing_profile_version_arns = Some(Default::default())
+pub(crate) fn lambda_managed_instances_capacity_provider_config_correct_errors(
+    mut builder: crate::types::builders::LambdaManagedInstancesCapacityProviderConfigBuilder,
+) -> crate::types::builders::LambdaManagedInstancesCapacityProviderConfigBuilder {
+    if builder.capacity_provider_arn.is_none() {
+        builder.capacity_provider_arn = Some(Default::default())
     }
     builder
 }

-pub(crate) fn execution_correct_errors(mut builder: crate::types::builders::ExecutionBuilder) -> crate::types::builders::ExecutionBuilder {
-    if builder.durable_execution_arn.is_none() {
-        builder.durable_execution_arn = Some(Default::default())
-    }
-    if builder.durable_execution_name.is_none() {
-        builder.durable_execution_name = Some(Default::default())
+pub(crate) fn execution_started_details_correct_errors(
+    mut builder: crate::types::builders::ExecutionStartedDetailsBuilder,
+) -> crate::types::builders::ExecutionStartedDetailsBuilder {
+    if builder.input.is_none() {
+        builder.input = {
+            let builder = crate::types::builders::EventInputBuilder::default();
+            Some(builder.build())
+        }
     }
-    if builder.function_arn.is_none() {
-        builder.function_arn = Some(Default::default())
-    }
-    if builder.status.is_none() {
-        builder.status = "no value was set".parse::<crate::types::ExecutionStatus>().ok()
-    }
-    if builder.start_timestamp.is_none() {
-        builder.start_timestamp = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
+    if builder.execution_timeout.is_none() {
+        builder.execution_timeout = Some(Default::default())
     }
     builder
 }

-pub(crate) fn file_system_config_correct_errors(
-    mut builder: crate::types::builders::FileSystemConfigBuilder,
-) -> crate::types::builders::FileSystemConfigBuilder {
-    if builder.arn.is_none() {
-        builder.arn = Some(Default::default())
-    }
-    if builder.local_mount_path.is_none() {
-        builder.local_mount_path = Some(Default::default())
+pub(crate) fn execution_succeeded_details_correct_errors(
+    mut builder: crate::types::builders::ExecutionSucceededDetailsBuilder,
+) -> crate::types::builders::ExecutionSucceededDetailsBuilder {
+    if builder.result.is_none() {
+        builder.result = {
+            let builder = crate::types::builders::EventResultBuilder::default();
+            Some(builder.build())
+        }
     }
     builder
 }

-pub(crate) fn function_url_config_correct_errors(
-    mut builder: crate::types::builders::FunctionUrlConfigBuilder,
-) -> crate::types::builders::FunctionUrlConfigBuilder {
-    if builder.function_url.is_none() {
-        builder.function_url = Some(Default::default())
-    }
-    if builder.function_arn.is_none() {
-        builder.function_arn = Some(Default::default())
-    }
-    if builder.creation_time.is_none() {
-        builder.creation_time = Some(Default::default())
+pub(crate) fn execution_failed_details_correct_errors(
+    mut builder: crate::types::builders::ExecutionFailedDetailsBuilder,
+) -> crate::types::builders::ExecutionFailedDetailsBuilder {
+    if builder.error.is_none() {
+        builder.error = {
+            let builder = crate::types::builders::EventErrorBuilder::default();
+            Some(builder.build())
+        }
     }
-    if builder.last_modified_time.is_none() {
-        builder.last_modified_time = Some(Default::default())
-    }
-    if builder.auth_type.is_none() {
-        builder.auth_type = "no value was set".parse::<crate::types::FunctionUrlAuthType>().ok()
-    }
     builder
 }

-pub(crate) fn function_versions_by_capacity_provider_list_item_correct_errors(
-    mut builder: crate::types::builders::FunctionVersionsByCapacityProviderListItemBuilder,
-) -> crate::types::builders::FunctionVersionsByCapacityProviderListItemBuilder {
-    if builder.function_arn.is_none() {
-        builder.function_arn = Some(Default::default())
-    }
-    if builder.state.is_none() {
-        builder.state = "no value was set".parse::<crate::types::State>().ok()
+pub(crate) fn execution_stopped_details_correct_errors(
+    mut builder: crate::types::builders::ExecutionStoppedDetailsBuilder,
+) -> crate::types::builders::ExecutionStoppedDetailsBuilder {
+    if builder.error.is_none() {
+        builder.error = {
+            let builder = crate::types::builders::EventErrorBuilder::default();
+            Some(builder.build())
+        }
     }
     builder
 }

-pub(crate) fn lambda_managed_instances_capacity_provider_config_correct_errors(
-    mut builder: crate::types::builders::LambdaManagedInstancesCapacityProviderConfigBuilder,
-) -> crate::types::builders::LambdaManagedInstancesCapacityProviderConfigBuilder {
-    if builder.capacity_provider_arn.is_none() {
-        builder.capacity_provider_arn = Some(Default::default())
+pub(crate) fn context_succeeded_details_correct_errors(
+    mut builder: crate::types::builders::ContextSucceededDetailsBuilder,
+) -> crate::types::builders::ContextSucceededDetailsBuilder {
+    if builder.result.is_none() {
+        builder.result = {
+            let builder = crate::types::builders::EventResultBuilder::default();
+            Some(builder.build())
+        }
     }
     builder
 }

-pub(crate) fn operation_correct_errors(mut builder: crate::types::builders::OperationBuilder) -> crate::types::builders::OperationBuilder {
-    if builder.id.is_none() {
-        builder.id = Some(Default::default())
-    }
-    if builder.r#type.is_none() {
-        builder.r#type = "no value was set".parse::<crate::types::OperationType>().ok()
-    }
-    if builder.start_timestamp.is_none() {
-        builder.start_timestamp = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
-    }
-    if builder.status.is_none() {
-        builder.status = "no value was set".parse::<crate::types::OperationStatus>().ok()
-    }
-    builder
-}
-
-pub(crate) fn callback_failed_details_correct_errors(
-    mut builder: crate::types::builders::CallbackFailedDetailsBuilder,
-) -> crate::types::builders::CallbackFailedDetailsBuilder {
+pub(crate) fn context_failed_details_correct_errors(
+    mut builder: crate::types::builders::ContextFailedDetailsBuilder,
+) -> crate::types::builders::ContextFailedDetailsBuilder {
     if builder.error.is_none() {
         builder.error = {
             let builder = crate::types::builders::EventErrorBuilder::default();
@@ -495,18 +508,21 @@
     builder
 }

-pub(crate) fn callback_started_details_correct_errors(
-    mut builder: crate::types::builders::CallbackStartedDetailsBuilder,
-) -> crate::types::builders::CallbackStartedDetailsBuilder {
-    if builder.callback_id.is_none() {
-        builder.callback_id = Some(Default::default())
+pub(crate) fn wait_started_details_correct_errors(
+    mut builder: crate::types::builders::WaitStartedDetailsBuilder,
+) -> crate::types::builders::WaitStartedDetailsBuilder {
+    if builder.duration.is_none() {
+        builder.duration = Some(Default::default())
+    }
+    if builder.scheduled_end_timestamp.is_none() {
+        builder.scheduled_end_timestamp = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
     }
     builder
 }

-pub(crate) fn callback_succeeded_details_correct_errors(
-    mut builder: crate::types::builders::CallbackSucceededDetailsBuilder,
-) -> crate::types::builders::CallbackSucceededDetailsBuilder {
+pub(crate) fn step_succeeded_details_correct_errors(
+    mut builder: crate::types::builders::StepSucceededDetailsBuilder,
+) -> crate::types::builders::StepSucceededDetailsBuilder {
     if builder.result.is_none() {
         builder.result = {
             let builder = crate::types::builders::EventResultBuilder::default();
@@ -513,12 +529,18 @@
             Some(builder.build())
         }
     }
+    if builder.retry_details.is_none() {
+        builder.retry_details = {
+            let builder = crate::types::builders::RetryDetailsBuilder::default();
+            Some(builder.build())
+        }
+    }
     builder
 }

-pub(crate) fn callback_timed_out_details_correct_errors(
-    mut builder: crate::types::builders::CallbackTimedOutDetailsBuilder,
-) -> crate::types::builders::CallbackTimedOutDetailsBuilder {
+pub(crate) fn step_failed_details_correct_errors(
+    mut builder: crate::types::builders::StepFailedDetailsBuilder,
+) -> crate::types::builders::StepFailedDetailsBuilder {
     if builder.error.is_none() {
         builder.error = {
             let builder = crate::types::builders::EventErrorBuilder::default();
@@ -525,15 +547,9 @@
             Some(builder.build())
         }
     }
-    builder
-}
-
-pub(crate) fn chained_invoke_failed_details_correct_errors(
-    mut builder: crate::types::builders::ChainedInvokeFailedDetailsBuilder,
-) -> crate::types::builders::ChainedInvokeFailedDetailsBuilder {
-    if builder.error.is_none() {
-        builder.error = {
-            let builder = crate::types::builders::EventErrorBuilder::default();
+    if builder.retry_details.is_none() {
+        builder.retry_details = {
+            let builder = crate::types::builders::RetryDetailsBuilder::default();
             Some(builder.build())
         }
     }
@@ -549,18 +565,6 @@
     builder
 }

-pub(crate) fn chained_invoke_stopped_details_correct_errors(
-    mut builder: crate::types::builders::ChainedInvokeStoppedDetailsBuilder,
-) -> crate::types::builders::ChainedInvokeStoppedDetailsBuilder {
-    if builder.error.is_none() {
-        builder.error = {
-            let builder = crate::types::builders::EventErrorBuilder::default();
-            Some(builder.build())
-        }
-    }
-    builder
-}
-
 pub(crate) fn chained_invoke_succeeded_details_correct_errors(
     mut builder: crate::types::builders::ChainedInvokeSucceededDetailsBuilder,
 ) -> crate::types::builders::ChainedInvokeSucceededDetailsBuilder {
@@ -573,9 +577,9 @@
     builder
 }

-pub(crate) fn chained_invoke_timed_out_details_correct_errors(
-    mut builder: crate::types::builders::ChainedInvokeTimedOutDetailsBuilder,
-) -> crate::types::builders::ChainedInvokeTimedOutDetailsBuilder {
+pub(crate) fn chained_invoke_failed_details_correct_errors(
+    mut builder: crate::types::builders::ChainedInvokeFailedDetailsBuilder,
+) -> crate::types::builders::ChainedInvokeFailedDetailsBuilder {
     if builder.error.is_none() {
         builder.error = {
             let builder = crate::types::builders::EventErrorBuilder::default();
@@ -585,9 +589,9 @@
     builder
 }

-pub(crate) fn context_failed_details_correct_errors(
-    mut builder: crate::types::builders::ContextFailedDetailsBuilder,
-) -> crate::types::builders::ContextFailedDetailsBuilder {
+pub(crate) fn chained_invoke_timed_out_details_correct_errors(
+    mut builder: crate::types::builders::ChainedInvokeTimedOutDetailsBuilder,
+) -> crate::types::builders::ChainedInvokeTimedOutDetailsBuilder {
     if builder.error.is_none() {
         builder.error = {
             let builder = crate::types::builders::EventErrorBuilder::default();
@@ -597,12 +601,12 @@
     builder
 }

-pub(crate) fn context_succeeded_details_correct_errors(
-    mut builder: crate::types::builders::ContextSucceededDetailsBuilder,
-) -> crate::types::builders::ContextSucceededDetailsBuilder {
-    if builder.result.is_none() {
-        builder.result = {
-            let builder = crate::types::builders::EventResultBuilder::default();
+pub(crate) fn chained_invoke_stopped_details_correct_errors(
+    mut builder: crate::types::builders::ChainedInvokeStoppedDetailsBuilder,
+) -> crate::types::builders::ChainedInvokeStoppedDetailsBuilder {
+    if builder.error.is_none() {
+        builder.error = {
+            let builder = crate::types::builders::EventErrorBuilder::default();
             Some(builder.build())
         }
     }
@@ -609,36 +613,30 @@
     builder
 }

-pub(crate) fn execution_failed_details_correct_errors(
-    mut builder: crate::types::builders::ExecutionFailedDetailsBuilder,
-) -> crate::types::builders::ExecutionFailedDetailsBuilder {
-    if builder.error.is_none() {
-        builder.error = {
-            let builder = crate::types::builders::EventErrorBuilder::default();
-            Some(builder.build())
-        }
+pub(crate) fn callback_started_details_correct_errors(
+    mut builder: crate::types::builders::CallbackStartedDetailsBuilder,
+) -> crate::types::builders::CallbackStartedDetailsBuilder {
+    if builder.callback_id.is_none() {
+        builder.callback_id = Some(Default::default())
     }
     builder
 }

-pub(crate) fn execution_started_details_correct_errors(
-    mut builder: crate::types::builders::ExecutionStartedDetailsBuilder,
-) -> crate::types::builders::ExecutionStartedDetailsBuilder {
-    if builder.input.is_none() {
-        builder.input = {
-            let builder = crate::types::builders::EventInputBuilder::default();
+pub(crate) fn callback_succeeded_details_correct_errors(
+    mut builder: crate::types::builders::CallbackSucceededDetailsBuilder,
+) -> crate::types::builders::CallbackSucceededDetailsBuilder {
+    if builder.result.is_none() {
+        builder.result = {
+            let builder = crate::types::builders::EventResultBuilder::default();
             Some(builder.build())
         }
     }
-    if builder.execution_timeout.is_none() {
-        builder.execution_timeout = Some(Default::default())
-    }
     builder
 }

-pub(crate) fn execution_stopped_details_correct_errors(
-    mut builder: crate::types::builders::ExecutionStoppedDetailsBuilder,
-) -> crate::types::builders::ExecutionStoppedDetailsBuilder {
+pub(crate) fn callback_failed_details_correct_errors(
+    mut builder: crate::types::builders::CallbackFailedDetailsBuilder,
+) -> crate::types::builders::CallbackFailedDetailsBuilder {
     if builder.error.is_none() {
         builder.error = {
             let builder = crate::types::builders::EventErrorBuilder::default();
@@ -648,12 +646,12 @@
     builder
 }

-pub(crate) fn execution_succeeded_details_correct_errors(
-    mut builder: crate::types::builders::ExecutionSucceededDetailsBuilder,
-) -> crate::types::builders::ExecutionSucceededDetailsBuilder {
-    if builder.result.is_none() {
-        builder.result = {
-            let builder = crate::types::builders::EventResultBuilder::default();
+pub(crate) fn callback_timed_out_details_correct_errors(
+    mut builder: crate::types::builders::CallbackTimedOutDetailsBuilder,
+) -> crate::types::builders::CallbackTimedOutDetailsBuilder {
+    if builder.error.is_none() {
+        builder.error = {
+            let builder = crate::types::builders::EventErrorBuilder::default();
             Some(builder.build())
         }
     }
@@ -675,62 +673,64 @@
     builder
 }

-pub(crate) fn step_failed_details_correct_errors(
-    mut builder: crate::types::builders::StepFailedDetailsBuilder,
-) -> crate::types::builders::StepFailedDetailsBuilder {
-    if builder.error.is_none() {
-        builder.error = {
-            let builder = crate::types::builders::EventErrorBuilder::default();
-            Some(builder.build())
-        }
+pub(crate) fn tags_error_correct_errors(mut builder: crate::types::builders::TagsErrorBuilder) -> crate::types::builders::TagsErrorBuilder {
+    if builder.error_code.is_none() {
+        builder.error_code = Some(Default::default())
     }
-    if builder.retry_details.is_none() {
-        builder.retry_details = {
-            let builder = crate::types::builders::RetryDetailsBuilder::default();
-            Some(builder.build())
-        }
+    if builder.message.is_none() {
+        builder.message = Some(Default::default())
     }
     builder
 }

-pub(crate) fn step_succeeded_details_correct_errors(
-    mut builder: crate::types::builders::StepSucceededDetailsBuilder,
-) -> crate::types::builders::StepSucceededDetailsBuilder {
-    if builder.result.is_none() {
-        builder.result = {
-            let builder = crate::types::builders::EventResultBuilder::default();
-            Some(builder.build())
-        }
+pub(crate) fn execution_correct_errors(mut builder: crate::types::builders::ExecutionBuilder) -> crate::types::builders::ExecutionBuilder {
+    if builder.durable_execution_arn.is_none() {
+        builder.durable_execution_arn = Some(Default::default())
+    }
+    if builder.durable_execution_name.is_none() {
+        builder.durable_execution_name = Some(Default::default())
+    }
+    if builder.function_arn.is_none() {
+        builder.function_arn = Some(Default::default())
     }
-    if builder.retry_details.is_none() {
-        builder.retry_details = {
-            let builder = crate::types::builders::RetryDetailsBuilder::default();
-            Some(builder.build())
-        }
+    if builder.status.is_none() {
+        builder.status = "no value was set".parse::<crate::types::ExecutionStatus>().ok()
+    }
+    if builder.start_timestamp.is_none() {
+        builder.start_timestamp = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
     }
     builder
 }

-pub(crate) fn wait_started_details_correct_errors(
-    mut builder: crate::types::builders::WaitStartedDetailsBuilder,
-) -> crate::types::builders::WaitStartedDetailsBuilder {
-    if builder.duration.is_none() {
-        builder.duration = Some(Default::default())
+pub(crate) fn function_url_config_correct_errors(
+    mut builder: crate::types::builders::FunctionUrlConfigBuilder,
+) -> crate::types::builders::FunctionUrlConfigBuilder {
+    if builder.function_url.is_none() {
+        builder.function_url = Some(Default::default())
     }
-    if builder.scheduled_end_timestamp.is_none() {
-        builder.scheduled_end_timestamp = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
+    if builder.function_arn.is_none() {
+        builder.function_arn = Some(Default::default())
+    }
+    if builder.creation_time.is_none() {
+        builder.creation_time = Some(Default::default())
+    }
+    if builder.last_modified_time.is_none() {
+        builder.last_modified_time = Some(Default::default())
     }
+    if builder.auth_type.is_none() {
+        builder.auth_type = "no value was set".parse::<crate::types::FunctionUrlAuthType>().ok()
+    }
     builder
 }

-pub(crate) fn target_tracking_scaling_policy_correct_errors(
-    mut builder: crate::types::builders::TargetTrackingScalingPolicyBuilder,
-) -> crate::types::builders::TargetTrackingScalingPolicyBuilder {
-    if builder.predefined_metric_type.is_none() {
-        builder.predefined_metric_type = "no value was set".parse::<crate::types::CapacityProviderPredefinedMetricType>().ok()
+pub(crate) fn function_versions_by_capacity_provider_list_item_correct_errors(
+    mut builder: crate::types::builders::FunctionVersionsByCapacityProviderListItemBuilder,
+) -> crate::types::builders::FunctionVersionsByCapacityProviderListItemBuilder {
+    if builder.function_arn.is_none() {
+        builder.function_arn = Some(Default::default())
     }
-    if builder.target_value.is_none() {
-        builder.target_value = Some(Default::default())
+    if builder.state.is_none() {
+        builder.state = "no value was set".parse::<crate::types::State>().ok()
     }
     builder
 }
```

### `src/types/_account_limit.rs`

```diff
--- reference/src/types/_account_limit.rs
+++ generated/src/types/_account_limit.rs
@@ -5,31 +5,31 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct AccountLimit {
     /// <p>The amount of storage space that you can use for all deployment packages and layer archives.</p>
-    pub total_code_size: i64,
+    pub total_code_size: ::std::option::Option<i64>,
     /// <p>The maximum size of a function's deployment package and layers when they're extracted.</p>
-    pub code_size_unzipped: i64,
+    pub code_size_unzipped: ::std::option::Option<i64>,
     /// <p>The maximum size of a deployment package when it's uploaded directly to Lambda. Use Amazon S3 for larger files.</p>
-    pub code_size_zipped: i64,
+    pub code_size_zipped: ::std::option::Option<i64>,
     /// <p>The maximum number of simultaneous function executions.</p>
-    pub concurrent_executions: i32,
+    pub concurrent_executions: ::std::option::Option<i32>,
     /// <p>The maximum number of simultaneous function executions, minus the capacity that's reserved for individual functions with <code>PutFunctionConcurrency</code>.</p>
     pub unreserved_concurrent_executions: ::std::option::Option<i32>,
 }
 impl AccountLimit {
     /// <p>The amount of storage space that you can use for all deployment packages and layer archives.</p>
-    pub fn total_code_size(&self) -> i64 {
+    pub fn total_code_size(&self) -> ::std::option::Option<i64> {
         self.total_code_size
     }
     /// <p>The maximum size of a function's deployment package and layers when they're extracted.</p>
-    pub fn code_size_unzipped(&self) -> i64 {
+    pub fn code_size_unzipped(&self) -> ::std::option::Option<i64> {
         self.code_size_unzipped
     }
     /// <p>The maximum size of a deployment package when it's uploaded directly to Lambda. Use Amazon S3 for larger files.</p>
-    pub fn code_size_zipped(&self) -> i64 {
+    pub fn code_size_zipped(&self) -> ::std::option::Option<i64> {
         self.code_size_zipped
     }
     /// <p>The maximum number of simultaneous function executions.</p>
-    pub fn concurrent_executions(&self) -> i32 {
+    pub fn concurrent_executions(&self) -> ::std::option::Option<i32> {
         self.concurrent_executions
     }
     /// <p>The maximum number of simultaneous function executions, minus the capacity that's reserved for individual functions with <code>PutFunctionConcurrency</code>.</p>
@@ -128,10 +128,10 @@
     /// Consumes the builder and constructs a [`AccountLimit`](crate::types::AccountLimit).
     pub fn build(self) -> crate::types::AccountLimit {
         crate::types::AccountLimit {
-            total_code_size: self.total_code_size.unwrap_or_default(),
-            code_size_unzipped: self.code_size_unzipped.unwrap_or_default(),
-            code_size_zipped: self.code_size_zipped.unwrap_or_default(),
-            concurrent_executions: self.concurrent_executions.unwrap_or_default(),
+            total_code_size: self.total_code_size,
+            code_size_unzipped: self.code_size_unzipped,
+            code_size_zipped: self.code_size_zipped,
+            concurrent_executions: self.concurrent_executions,
             unreserved_concurrent_executions: self.unreserved_concurrent_executions,
         }
     }
```

### `src/types/_account_usage.rs`

```diff
--- reference/src/types/_account_usage.rs
+++ generated/src/types/_account_usage.rs
@@ -5,17 +5,17 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct AccountUsage {
     /// <p>The amount of storage space, in bytes, that's being used by deployment packages and layer archives.</p>
-    pub total_code_size: i64,
+    pub total_code_size: ::std::option::Option<i64>,
     /// <p>The number of Lambda functions.</p>
-    pub function_count: i64,
+    pub function_count: ::std::option::Option<i64>,
 }
 impl AccountUsage {
     /// <p>The amount of storage space, in bytes, that's being used by deployment packages and layer archives.</p>
-    pub fn total_code_size(&self) -> i64 {
+    pub fn total_code_size(&self) -> ::std::option::Option<i64> {
         self.total_code_size
     }
     /// <p>The number of Lambda functions.</p>
-    pub fn function_count(&self) -> i64 {
+    pub fn function_count(&self) -> ::std::option::Option<i64> {
         self.function_count
     }
 }
@@ -65,8 +65,8 @@
     /// Consumes the builder and constructs a [`AccountUsage`](crate::types::AccountUsage).
     pub fn build(self) -> crate::types::AccountUsage {
         crate::types::AccountUsage {
-            total_code_size: self.total_code_size.unwrap_or_default(),
-            function_count: self.function_count.unwrap_or_default(),
+            total_code_size: self.total_code_size,
+            function_count: self.function_count,
         }
     }
 }
```

### `src/types/_callback_details.rs`

```diff
--- reference/src/types/_callback_details.rs
+++ generated/src/types/_callback_details.rs
@@ -30,7 +30,7 @@
         let mut formatter = f.debug_struct("CallbackDetails");
         formatter.field("callback_id", &self.callback_id);
         formatter.field("result", &"*** Sensitive Data Redacted ***");
-        formatter.field("error", &self.error);
+        formatter.field("error", &"*** Sensitive Data Redacted ***");
         formatter.finish()
     }
 }
@@ -106,7 +106,7 @@
         let mut formatter = f.debug_struct("CallbackDetailsBuilder");
         formatter.field("callback_id", &self.callback_id);
         formatter.field("result", &"*** Sensitive Data Redacted ***");
-        formatter.field("error", &self.error);
+        formatter.field("error", &"*** Sensitive Data Redacted ***");
         formatter.finish()
     }
 }
```

### `src/types/_callback_failed_details.rs`

```diff
--- reference/src/types/_callback_failed_details.rs
+++ generated/src/types/_callback_failed_details.rs
@@ -2,7 +2,7 @@

 /// <p>Contains details about a failed callback operation, including error information and the reason for failure.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct CallbackFailedDetails {
     /// <p>An error object that contains details about the failure.</p>
     pub error: ::std::option::Option<crate::types::EventError>,
@@ -13,6 +13,13 @@
         self.error.as_ref()
     }
 }
+impl ::std::fmt::Debug for CallbackFailedDetails {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("CallbackFailedDetails");
+        formatter.field("error", &"*** Sensitive Data Redacted ***");
+        formatter.finish()
+    }
+}
 impl CallbackFailedDetails {
     /// Creates a new builder-style object to manufacture [`CallbackFailedDetails`](crate::types::CallbackFailedDetails).
     pub fn builder() -> crate::types::builders::CallbackFailedDetailsBuilder {
@@ -21,7 +28,7 @@
 }

 /// A builder for [`CallbackFailedDetails`](crate::types::CallbackFailedDetails).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
 #[non_exhaustive]
 pub struct CallbackFailedDetailsBuilder {
     pub(crate) error: ::std::option::Option<crate::types::EventError>,
@@ -47,3 +54,10 @@
         crate::types::CallbackFailedDetails { error: self.error }
     }
 }
+impl ::std::fmt::Debug for CallbackFailedDetailsBuilder {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("CallbackFailedDetailsBuilder");
+        formatter.field("error", &"*** Sensitive Data Redacted ***");
+        formatter.finish()
+    }
+}
```

### `src/types/_callback_options.rs`

```diff
--- reference/src/types/_callback_options.rs
+++ generated/src/types/_callback_options.rs
@@ -5,17 +5,17 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct CallbackOptions {
     /// <p>The timeout for the callback operation in seconds. If not specified or set to 0, the callback has no timeout.</p>
-    pub timeout_seconds: i32,
+    pub timeout_seconds: ::std::option::Option<i32>,
     /// <p>The heartbeat timeout for the callback operation, in seconds. If not specified or set to 0, heartbeat timeout is disabled.</p>
-    pub heartbeat_timeout_seconds: i32,
+    pub heartbeat_timeout_seconds: ::std::option::Option<i32>,
 }
 impl CallbackOptions {
     /// <p>The timeout for the callback operation in seconds. If not specified or set to 0, the callback has no timeout.</p>
-    pub fn timeout_seconds(&self) -> i32 {
+    pub fn timeout_seconds(&self) -> ::std::option::Option<i32> {
         self.timeout_seconds
     }
     /// <p>The heartbeat timeout for the callback operation, in seconds. If not specified or set to 0, heartbeat timeout is disabled.</p>
-    pub fn heartbeat_timeout_seconds(&self) -> i32 {
+    pub fn heartbeat_timeout_seconds(&self) -> ::std::option::Option<i32> {
         self.heartbeat_timeout_seconds
     }
 }
@@ -65,8 +65,8 @@
     /// Consumes the builder and constructs a [`CallbackOptions`](crate::types::CallbackOptions).
     pub fn build(self) -> crate::types::CallbackOptions {
         crate::types::CallbackOptions {
-            timeout_seconds: self.timeout_seconds.unwrap_or_default(),
-            heartbeat_timeout_seconds: self.heartbeat_timeout_seconds.unwrap_or_default(),
+            timeout_seconds: self.timeout_seconds,
+            heartbeat_timeout_seconds: self.heartbeat_timeout_seconds,
         }
     }
 }
```

### `src/types/_callback_succeeded_details.rs`

```diff
--- reference/src/types/_callback_succeeded_details.rs
+++ generated/src/types/_callback_succeeded_details.rs
@@ -2,7 +2,7 @@

 /// <p>Contains details about a successfully completed callback operation, including the result data and completion timestamp.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct CallbackSucceededDetails {
     /// <p>The response payload from the successful operation.</p>
     pub result: ::std::option::Option<crate::types::EventResult>,
@@ -13,6 +13,13 @@
         self.result.as_ref()
     }
 }
+impl ::std::fmt::Debug for CallbackSucceededDetails {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("CallbackSucceededDetails");
+        formatter.field("result", &"*** Sensitive Data Redacted ***");
+        formatter.finish()
+    }
+}
 impl CallbackSucceededDetails {
     /// Creates a new builder-style object to manufacture [`CallbackSucceededDetails`](crate::types::CallbackSucceededDetails).
     pub fn builder() -> crate::types::builders::CallbackSucceededDetailsBuilder {
@@ -21,7 +28,7 @@
 }

 /// A builder for [`CallbackSucceededDetails`](crate::types::CallbackSucceededDetails).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
 #[non_exhaustive]
 pub struct CallbackSucceededDetailsBuilder {
     pub(crate) result: ::std::option::Option<crate::types::EventResult>,
@@ -47,3 +54,10 @@
         crate::types::CallbackSucceededDetails { result: self.result }
     }
 }
+impl ::std::fmt::Debug for CallbackSucceededDetailsBuilder {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("CallbackSucceededDetailsBuilder");
+        formatter.field("result", &"*** Sensitive Data Redacted ***");
+        formatter.finish()
+    }
+}
```

### `src/types/_callback_timed_out_details.rs`

```diff
--- reference/src/types/_callback_timed_out_details.rs
+++ generated/src/types/_callback_timed_out_details.rs
@@ -2,7 +2,7 @@

 /// <p>Contains details about a callback operation that timed out, including timeout duration and any partial results.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct CallbackTimedOutDetails {
     /// <p>Details about the callback timeout.</p>
     pub error: ::std::option::Option<crate::types::EventError>,
@@ -13,6 +13,13 @@
         self.error.as_ref()
     }
 }
+impl ::std::fmt::Debug for CallbackTimedOutDetails {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("CallbackTimedOutDetails");
+        formatter.field("error", &"*** Sensitive Data Redacted ***");
+        formatter.finish()
+    }
+}
 impl CallbackTimedOutDetails {
     /// Creates a new builder-style object to manufacture [`CallbackTimedOutDetails`](crate::types::CallbackTimedOutDetails).
     pub fn builder() -> crate::types::builders::CallbackTimedOutDetailsBuilder {
@@ -21,7 +28,7 @@
 }

 /// A builder for [`CallbackTimedOutDetails`](crate::types::CallbackTimedOutDetails).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
 #[non_exhaustive]
 pub struct CallbackTimedOutDetailsBuilder {
     pub(crate) error: ::std::option::Option<crate::types::EventError>,
@@ -47,3 +54,10 @@
         crate::types::CallbackTimedOutDetails { error: self.error }
     }
 }
+impl ::std::fmt::Debug for CallbackTimedOutDetailsBuilder {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("CallbackTimedOutDetailsBuilder");
+        formatter.field("error", &"*** Sensitive Data Redacted ***");
+        formatter.finish()
+    }
+}
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

### `src/types/_chained_invoke_details.rs`

```diff
--- reference/src/types/_chained_invoke_details.rs
+++ generated/src/types/_chained_invoke_details.rs
@@ -23,7 +23,7 @@
     fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
         let mut formatter = f.debug_struct("ChainedInvokeDetails");
         formatter.field("result", &"*** Sensitive Data Redacted ***");
-        formatter.field("error", &self.error);
+        formatter.field("error", &"*** Sensitive Data Redacted ***");
         formatter.finish()
     }
 }
@@ -82,7 +82,7 @@
     fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
         let mut formatter = f.debug_struct("ChainedInvokeDetailsBuilder");
         formatter.field("result", &"*** Sensitive Data Redacted ***");
-        formatter.field("error", &self.error);
+        formatter.field("error", &"*** Sensitive Data Redacted ***");
         formatter.finish()
     }
 }
```

### `src/types/_chained_invoke_failed_details.rs`

```diff
--- reference/src/types/_chained_invoke_failed_details.rs
+++ generated/src/types/_chained_invoke_failed_details.rs
@@ -2,7 +2,7 @@

 /// <p>Contains details about a failed chained function invocation, including error information and failure reason.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct ChainedInvokeFailedDetails {
     /// <p>Details about the chained invocation failure.</p>
     pub error: ::std::option::Option<crate::types::EventError>,
@@ -13,6 +13,13 @@
         self.error.as_ref()
     }
 }
+impl ::std::fmt::Debug for ChainedInvokeFailedDetails {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("ChainedInvokeFailedDetails");
+        formatter.field("error", &"*** Sensitive Data Redacted ***");
+        formatter.finish()
+    }
+}
 impl ChainedInvokeFailedDetails {
     /// Creates a new builder-style object to manufacture [`ChainedInvokeFailedDetails`](crate::types::ChainedInvokeFailedDetails).
     pub fn builder() -> crate::types::builders::ChainedInvokeFailedDetailsBuilder {
@@ -21,7 +28,7 @@
 }

 /// A builder for [`ChainedInvokeFailedDetails`](crate::types::ChainedInvokeFailedDetails).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
 #[non_exhaustive]
 pub struct ChainedInvokeFailedDetailsBuilder {
     pub(crate) error: ::std::option::Option<crate::types::EventError>,
@@ -47,3 +54,10 @@
         crate::types::ChainedInvokeFailedDetails { error: self.error }
     }
 }
+impl ::std::fmt::Debug for ChainedInvokeFailedDetailsBuilder {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("ChainedInvokeFailedDetailsBuilder");
+        formatter.field("error", &"*** Sensitive Data Redacted ***");
+        formatter.finish()
+    }
+}
```

### `src/types/_chained_invoke_started_details.rs`

```diff
--- reference/src/types/_chained_invoke_started_details.rs
+++ generated/src/types/_chained_invoke_started_details.rs
@@ -2,7 +2,7 @@

 /// <p>Contains details about a chained function invocation that has started execution, including start time and execution context.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct ChainedInvokeStartedDetails {
     /// <p>The name or ARN of the Lambda function being invoked.</p>
     pub function_name: ::std::string::String,
@@ -38,6 +38,17 @@
         self.durable_execution_arn.as_deref()
     }
 }
+impl ::std::fmt::Debug for ChainedInvokeStartedDetails {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("ChainedInvokeStartedDetails");
+        formatter.field("function_name", &self.function_name);
+        formatter.field("tenant_id", &self.tenant_id);
+        formatter.field("input", &"*** Sensitive Data Redacted ***");
+        formatter.field("executed_version", &self.executed_version);
+        formatter.field("durable_execution_arn", &self.durable_execution_arn);
+        formatter.finish()
+    }
+}
 impl ChainedInvokeStartedDetails {
     /// Creates a new builder-style object to manufacture [`ChainedInvokeStartedDetails`](crate::types::ChainedInvokeStartedDetails).
     pub fn builder() -> crate::types::builders::ChainedInvokeStartedDetailsBuilder {
@@ -46,7 +57,7 @@
 }

 /// A builder for [`ChainedInvokeStartedDetails`](crate::types::ChainedInvokeStartedDetails).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
 #[non_exhaustive]
 pub struct ChainedInvokeStartedDetailsBuilder {
     pub(crate) function_name: ::std::option::Option<::std::string::String>,
@@ -145,3 +156,14 @@
         })
     }
 }
+impl ::std::fmt::Debug for ChainedInvokeStartedDetailsBuilder {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("ChainedInvokeStartedDetailsBuilder");
+        formatter.field("function_name", &self.function_name);
+        formatter.field("tenant_id", &self.tenant_id);
+        formatter.field("input", &"*** Sensitive Data Redacted ***");
+        formatter.field("executed_version", &self.executed_version);
+        formatter.field("durable_execution_arn", &self.durable_execution_arn);
+        formatter.finish()
+    }
+}
```

### `src/types/_chained_invoke_stopped_details.rs`

```diff
--- reference/src/types/_chained_invoke_stopped_details.rs
+++ generated/src/types/_chained_invoke_stopped_details.rs
@@ -2,7 +2,7 @@

 /// <p>Details about a chained invocation that was stopped.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct ChainedInvokeStoppedDetails {
     /// <p>Details about why the chained invocation stopped.</p>
     pub error: ::std::option::Option<crate::types::EventError>,
@@ -13,6 +13,13 @@
         self.error.as_ref()
     }
 }
+impl ::std::fmt::Debug for ChainedInvokeStoppedDetails {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("ChainedInvokeStoppedDetails");
+        formatter.field("error", &"*** Sensitive Data Redacted ***");
+        formatter.finish()
+    }
+}
 impl ChainedInvokeStoppedDetails {
     /// Creates a new builder-style object to manufacture [`ChainedInvokeStoppedDetails`](crate::types::ChainedInvokeStoppedDetails).
     pub fn builder() -> crate::types::builders::ChainedInvokeStoppedDetailsBuilder {
@@ -21,7 +28,7 @@
 }

 /// A builder for [`ChainedInvokeStoppedDetails`](crate::types::ChainedInvokeStoppedDetails).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
 #[non_exhaustive]
 pub struct ChainedInvokeStoppedDetailsBuilder {
     pub(crate) error: ::std::option::Option<crate::types::EventError>,
@@ -47,3 +54,10 @@
         crate::types::ChainedInvokeStoppedDetails { error: self.error }
     }
 }
+impl ::std::fmt::Debug for ChainedInvokeStoppedDetailsBuilder {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("ChainedInvokeStoppedDetailsBuilder");
+        formatter.field("error", &"*** Sensitive Data Redacted ***");
+        formatter.finish()
+    }
+}
```

### `src/types/_chained_invoke_succeeded_details.rs`

```diff
--- reference/src/types/_chained_invoke_succeeded_details.rs
+++ generated/src/types/_chained_invoke_succeeded_details.rs
@@ -2,7 +2,7 @@

 /// <p>Details about a chained invocation that succeeded.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct ChainedInvokeSucceededDetails {
     /// <p>The response payload from the successful operation.</p>
     pub result: ::std::option::Option<crate::types::EventResult>,
@@ -13,6 +13,13 @@
         self.result.as_ref()
     }
 }
+impl ::std::fmt::Debug for ChainedInvokeSucceededDetails {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("ChainedInvokeSucceededDetails");
+        formatter.field("result", &"*** Sensitive Data Redacted ***");
+        formatter.finish()
+    }
+}
 impl ChainedInvokeSucceededDetails {
     /// Creates a new builder-style object to manufacture [`ChainedInvokeSucceededDetails`](crate::types::ChainedInvokeSucceededDetails).
     pub fn builder() -> crate::types::builders::ChainedInvokeSucceededDetailsBuilder {
@@ -21,7 +28,7 @@
 }

 /// A builder for [`ChainedInvokeSucceededDetails`](crate::types::ChainedInvokeSucceededDetails).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
 #[non_exhaustive]
 pub struct ChainedInvokeSucceededDetailsBuilder {
     pub(crate) result: ::std::option::Option<crate::types::EventResult>,
@@ -47,3 +54,10 @@
         crate::types::ChainedInvokeSucceededDetails { result: self.result }
     }
 }
+impl ::std::fmt::Debug for ChainedInvokeSucceededDetailsBuilder {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("ChainedInvokeSucceededDetailsBuilder");
+        formatter.field("result", &"*** Sensitive Data Redacted ***");
+        formatter.finish()
+    }
+}
```

### `src/types/_chained_invoke_timed_out_details.rs`

```diff
--- reference/src/types/_chained_invoke_timed_out_details.rs
+++ generated/src/types/_chained_invoke_timed_out_details.rs
@@ -2,7 +2,7 @@

 /// <p>Details about a chained invocation that timed out.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct ChainedInvokeTimedOutDetails {
     /// <p>Details about the chained invocation timeout.</p>
     pub error: ::std::option::Option<crate::types::EventError>,
@@ -13,6 +13,13 @@
         self.error.as_ref()
     }
 }
+impl ::std::fmt::Debug for ChainedInvokeTimedOutDetails {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("ChainedInvokeTimedOutDetails");
+        formatter.field("error", &"*** Sensitive Data Redacted ***");
+        formatter.finish()
+    }
+}
 impl ChainedInvokeTimedOutDetails {
     /// Creates a new builder-style object to manufacture [`ChainedInvokeTimedOutDetails`](crate::types::ChainedInvokeTimedOutDetails).
     pub fn builder() -> crate::types::builders::ChainedInvokeTimedOutDetailsBuilder {
@@ -21,7 +28,7 @@
 }

 /// A builder for [`ChainedInvokeTimedOutDetails`](crate::types::ChainedInvokeTimedOutDetails).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
 #[non_exhaustive]
 pub struct ChainedInvokeTimedOutDetailsBuilder {
     pub(crate) error: ::std::option::Option<crate::types::EventError>,
@@ -47,3 +54,10 @@
         crate::types::ChainedInvokeTimedOutDetails { error: self.error }
     }
 }
+impl ::std::fmt::Debug for ChainedInvokeTimedOutDetailsBuilder {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("ChainedInvokeTimedOutDetailsBuilder");
+        formatter.field("error", &"*** Sensitive Data Redacted ***");
+        formatter.finish()
+    }
+}
```

### `src/types/_checkpoint_updated_execution_state.rs`

```diff
--- reference/src/types/_checkpoint_updated_execution_state.rs
+++ generated/src/types/_checkpoint_updated_execution_state.rs
@@ -2,7 +2,7 @@

 /// <p>Contains operations that have been updated since the last checkpoint, such as completed asynchronous work like timers or callbacks.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct CheckpointUpdatedExecutionState {
     /// <p>A list of operations that have been updated since the last checkpoint.</p>
     pub operations: ::std::option::Option<::std::vec::Vec<crate::types::Operation>>,
@@ -21,6 +21,14 @@
         self.next_marker.as_deref()
     }
 }
+impl ::std::fmt::Debug for CheckpointUpdatedExecutionState {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("CheckpointUpdatedExecutionState");
+        formatter.field("operations", &"*** Sensitive Data Redacted ***");
+        formatter.field("next_marker", &self.next_marker);
+        formatter.finish()
+    }
+}
 impl CheckpointUpdatedExecutionState {
     /// Creates a new builder-style object to manufacture [`CheckpointUpdatedExecutionState`](crate::types::CheckpointUpdatedExecutionState).
     pub fn builder() -> crate::types::builders::CheckpointUpdatedExecutionStateBuilder {
@@ -29,7 +37,7 @@
 }

 /// A builder for [`CheckpointUpdatedExecutionState`](crate::types::CheckpointUpdatedExecutionState).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
 #[non_exhaustive]
 pub struct CheckpointUpdatedExecutionStateBuilder {
     pub(crate) operations: ::std::option::Option<::std::vec::Vec<crate::types::Operation>>,
@@ -78,3 +86,11 @@
         }
     }
 }
+impl ::std::fmt::Debug for CheckpointUpdatedExecutionStateBuilder {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("CheckpointUpdatedExecutionStateBuilder");
+        formatter.field("operations", &"*** Sensitive Data Redacted ***");
+        formatter.field("next_marker", &self.next_marker);
+        formatter.finish()
+    }
+}
```

### `src/types/_concurrency.rs`

```diff
--- reference/src/types/_concurrency.rs
+++ generated/src/types/_concurrency.rs
@@ -1,4 +1,5 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+
 #[allow(missing_docs)] // documentation missing in model
 #[non_exhaustive]
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
```

### `src/types/_context_details.rs`

```diff
--- reference/src/types/_context_details.rs
+++ generated/src/types/_context_details.rs
@@ -30,7 +30,7 @@
         let mut formatter = f.debug_struct("ContextDetails");
         formatter.field("replay_children", &self.replay_children);
         formatter.field("result", &"*** Sensitive Data Redacted ***");
-        formatter.field("error", &self.error);
+        formatter.field("error", &"*** Sensitive Data Redacted ***");
         formatter.finish()
     }
 }
@@ -106,7 +106,7 @@
         let mut formatter = f.debug_struct("ContextDetailsBuilder");
         formatter.field("replay_children", &self.replay_children);
         formatter.field("result", &"*** Sensitive Data Redacted ***");
-        formatter.field("error", &self.error);
+        formatter.field("error", &"*** Sensitive Data Redacted ***");
         formatter.finish()
     }
 }
```

### `src/types/_context_failed_details.rs`

```diff
--- reference/src/types/_context_failed_details.rs
+++ generated/src/types/_context_failed_details.rs
@@ -2,7 +2,7 @@

 /// <p>Details about a context that failed.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct ContextFailedDetails {
     /// <p>Details about the context failure.</p>
     pub error: ::std::option::Option<crate::types::EventError>,
@@ -13,6 +13,13 @@
         self.error.as_ref()
     }
 }
+impl ::std::fmt::Debug for ContextFailedDetails {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("ContextFailedDetails");
+        formatter.field("error", &"*** Sensitive Data Redacted ***");
+        formatter.finish()
+    }
+}
 impl ContextFailedDetails {
     /// Creates a new builder-style object to manufacture [`ContextFailedDetails`](crate::types::ContextFailedDetails).
     pub fn builder() -> crate::types::builders::ContextFailedDetailsBuilder {
@@ -21,7 +28,7 @@
 }

 /// A builder for [`ContextFailedDetails`](crate::types::ContextFailedDetails).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
 #[non_exhaustive]
 pub struct ContextFailedDetailsBuilder {
     pub(crate) error: ::std::option::Option<crate::types::EventError>,
@@ -47,3 +54,10 @@
         crate::types::ContextFailedDetails { error: self.error }
     }
 }
+impl ::std::fmt::Debug for ContextFailedDetailsBuilder {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("ContextFailedDetailsBuilder");
+        formatter.field("error", &"*** Sensitive Data Redacted ***");
+        formatter.finish()
+    }
+}
```

### `src/types/_context_succeeded_details.rs`

```diff
--- reference/src/types/_context_succeeded_details.rs
+++ generated/src/types/_context_succeeded_details.rs
@@ -2,7 +2,7 @@

 /// <p>Details about a context that succeeded.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct ContextSucceededDetails {
     /// <p>The JSON response payload from the successful context.</p>
     pub result: ::std::option::Option<crate::types::EventResult>,
@@ -13,6 +13,13 @@
         self.result.as_ref()
     }
 }
+impl ::std::fmt::Debug for ContextSucceededDetails {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("ContextSucceededDetails");
+        formatter.field("result", &"*** Sensitive Data Redacted ***");
+        formatter.finish()
+    }
+}
 impl ContextSucceededDetails {
     /// Creates a new builder-style object to manufacture [`ContextSucceededDetails`](crate::types::ContextSucceededDetails).
     pub fn builder() -> crate::types::builders::ContextSucceededDetailsBuilder {
@@ -21,7 +28,7 @@
 }

 /// A builder for [`ContextSucceededDetails`](crate::types::ContextSucceededDetails).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
 #[non_exhaustive]
 pub struct ContextSucceededDetailsBuilder {
     pub(crate) result: ::std::option::Option<crate::types::EventResult>,
@@ -47,3 +54,10 @@
         crate::types::ContextSucceededDetails { result: self.result }
     }
 }
+impl ::std::fmt::Debug for ContextSucceededDetailsBuilder {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("ContextSucceededDetailsBuilder");
+        formatter.field("result", &"*** Sensitive Data Redacted ***");
+        formatter.finish()
+    }
+}
```

### `src/types/_environment_response.rs`

```diff
--- reference/src/types/_environment_response.rs
+++ generated/src/types/_environment_response.rs
@@ -23,7 +23,7 @@
     fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
         let mut formatter = f.debug_struct("EnvironmentResponse");
         formatter.field("variables", &"*** Sensitive Data Redacted ***");
-        formatter.field("error", &self.error);
+        formatter.field("error", &"*** Sensitive Data Redacted ***");
         formatter.finish()
     }
 }
@@ -88,7 +88,7 @@
     fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
         let mut formatter = f.debug_struct("EnvironmentResponseBuilder");
         formatter.field("variables", &"*** Sensitive Data Redacted ***");
-        formatter.field("error", &self.error);
+        formatter.field("error", &"*** Sensitive Data Redacted ***");
         formatter.finish()
     }
 }
```

### `src/types/_event.rs`

```diff
--- reference/src/types/_event.rs
+++ generated/src/types/_event.rs
@@ -2,7 +2,7 @@

 /// <p>An event that occurred during the execution of a durable function.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct Event {
     /// <p>The type of event that occurred.</p>
     pub event_type: ::std::option::Option<crate::types::EventType>,
@@ -9,7 +9,7 @@
     /// <p>The subtype of the event, providing additional categorization.</p>
     pub sub_type: ::std::option::Option<::std::string::String>,
     /// <p>The unique identifier for this event. Event IDs increment sequentially.</p>
-    pub event_id: i32,
+    pub event_id: ::std::option::Option<i32>,
     /// <p>The unique identifier for this operation.</p>
     pub id: ::std::option::Option<::std::string::String>,
     /// <p>The customer-provided name for this operation.</p>
@@ -77,7 +77,7 @@
         self.sub_type.as_deref()
     }
     /// <p>The unique identifier for this event. Event IDs increment sequentially.</p>
-    pub fn event_id(&self) -> i32 {
+    pub fn event_id(&self) -> ::std::option::Option<i32> {
         self.event_id
     }
     /// <p>The unique identifier for this operation.</p>
@@ -193,6 +193,43 @@
         self.invocation_completed_details.as_ref()
     }
 }
+impl ::std::fmt::Debug for Event {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("Event");
+        formatter.field("event_type", &self.event_type);
+        formatter.field("sub_type", &self.sub_type);
+        formatter.field("event_id", &self.event_id);
+        formatter.field("id", &self.id);
+        formatter.field("name", &self.name);
+        formatter.field("event_timestamp", &self.event_timestamp);
+        formatter.field("parent_id", &self.parent_id);
+        formatter.field("execution_started_details", &"*** Sensitive Data Redacted ***");
+        formatter.field("execution_succeeded_details", &"*** Sensitive Data Redacted ***");
+        formatter.field("execution_failed_details", &"*** Sensitive Data Redacted ***");
+        formatter.field("execution_timed_out_details", &"*** Sensitive Data Redacted ***");
+        formatter.field("execution_stopped_details", &"*** Sensitive Data Redacted ***");
+        formatter.field("context_started_details", &self.context_started_details);
+        formatter.field("context_succeeded_details", &"*** Sensitive Data Redacted ***");
+        formatter.field("context_failed_details", &"*** Sensitive Data Redacted ***");
+        formatter.field("wait_started_details", &self.wait_started_details);
+        formatter.field("wait_succeeded_details", &self.wait_succeeded_details);
+        formatter.field("wait_cancelled_details", &"*** Sensitive Data Redacted ***");
+        formatter.field("step_started_details", &self.step_started_details);
+        formatter.field("step_succeeded_details", &"*** Sensitive Data Redacted ***");
+        formatter.field("step_failed_details", &"*** Sensitive Data Redacted ***");
+        formatter.field("chained_invoke_started_details", &"*** Sensitive Data Redacted ***");
+        formatter.field("chained_invoke_succeeded_details", &"*** Sensitive Data Redacted ***");
+        formatter.field("chained_invoke_failed_details", &"*** Sensitive Data Redacted ***");
+        formatter.field("chained_invoke_timed_out_details", &"*** Sensitive Data Redacted ***");
+        formatter.field("chained_invoke_stopped_details", &"*** Sensitive Data Redacted ***");
+        formatter.field("callback_started_details", &self.callback_started_details);
+        formatter.field("callback_succeeded_details", &"*** Sensitive Data Redacted ***");
+        formatter.field("callback_failed_details", &"*** Sensitive Data Redacted ***");
+        formatter.field("callback_timed_out_details", &"*** Sensitive Data Redacted ***");
+        formatter.field("invocation_completed_details", &"*** Sensitive Data Redacted ***");
+        formatter.finish()
+    }
+}
 impl Event {
     /// Creates a new builder-style object to manufacture [`Event`](crate::types::Event).
     pub fn builder() -> crate::types::builders::EventBuilder {
@@ -201,7 +238,7 @@
 }

 /// A builder for [`Event`](crate::types::Event).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
 #[non_exhaustive]
 pub struct EventBuilder {
     pub(crate) event_type: ::std::option::Option<crate::types::EventType>,
@@ -676,7 +713,7 @@
         crate::types::Event {
             event_type: self.event_type,
             sub_type: self.sub_type,
-            event_id: self.event_id.unwrap_or(1),
+            event_id: self.event_id,
             id: self.id,
             name: self.name,
             event_timestamp: self.event_timestamp,
@@ -708,3 +745,40 @@
         }
     }
 }
+impl ::std::fmt::Debug for EventBuilder {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("EventBuilder");
+        formatter.field("event_type", &self.event_type);
+        formatter.field("sub_type", &self.sub_type);
+        formatter.field("event_id", &self.event_id);
+        formatter.field("id", &self.id);
+        formatter.field("name", &self.name);
+        formatter.field("event_timestamp", &self.event_timestamp);
+        formatter.field("parent_id", &self.parent_id);
+        formatter.field("execution_started_details", &"*** Sensitive Data Redacted ***");
+        formatter.field("execution_succeeded_details", &"*** Sensitive Data Redacted ***");
+        formatter.field("execution_failed_details", &"*** Sensitive Data Redacted ***");
+        formatter.field("execution_timed_out_details", &"*** Sensitive Data Redacted ***");
+        formatter.field("execution_stopped_details", &"*** Sensitive Data Redacted ***");
+        formatter.field("context_started_details", &self.context_started_details);
+        formatter.field("context_succeeded_details", &"*** Sensitive Data Redacted ***");
+        formatter.field("context_failed_details", &"*** Sensitive Data Redacted ***");
+        formatter.field("wait_started_details", &self.wait_started_details);
+        formatter.field("wait_succeeded_details", &self.wait_succeeded_details);
+        formatter.field("wait_cancelled_details", &"*** Sensitive Data Redacted ***");
+        formatter.field("step_started_details", &self.step_started_details);
+        formatter.field("step_succeeded_details", &"*** Sensitive Data Redacted ***");
+        formatter.field("step_failed_details", &"*** Sensitive Data Redacted ***");
+        formatter.field("chained_invoke_started_details", &"*** Sensitive Data Redacted ***");
+        formatter.field("chained_invoke_succeeded_details", &"*** Sensitive Data Redacted ***");
+        formatter.field("chained_invoke_failed_details", &"*** Sensitive Data Redacted ***");
+        formatter.field("chained_invoke_timed_out_details", &"*** Sensitive Data Redacted ***");
+        formatter.field("chained_invoke_stopped_details", &"*** Sensitive Data Redacted ***");
+        formatter.field("callback_started_details", &self.callback_started_details);
+        formatter.field("callback_succeeded_details", &"*** Sensitive Data Redacted ***");
+        formatter.field("callback_failed_details", &"*** Sensitive Data Redacted ***");
+        formatter.field("callback_timed_out_details", &"*** Sensitive Data Redacted ***");
+        formatter.field("invocation_completed_details", &"*** Sensitive Data Redacted ***");
+        formatter.finish()
+    }
+}
```

### `src/types/_event_error.rs`

```diff
--- reference/src/types/_event_error.rs
+++ generated/src/types/_event_error.rs
@@ -2,7 +2,7 @@

 /// <p>Error information for an event.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct EventError {
     /// <p>The error payload.</p>
     pub payload: ::std::option::Option<crate::types::ErrorObject>,
@@ -19,6 +19,14 @@
         self.truncated
     }
 }
+impl ::std::fmt::Debug for EventError {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("EventError");
+        formatter.field("payload", &"*** Sensitive Data Redacted ***");
+        formatter.field("truncated", &self.truncated);
+        formatter.finish()
+    }
+}
 impl EventError {
     /// Creates a new builder-style object to manufacture [`EventError`](crate::types::EventError).
     pub fn builder() -> crate::types::builders::EventErrorBuilder {
@@ -27,7 +35,7 @@
 }

 /// A builder for [`EventError`](crate::types::EventError).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
 #[non_exhaustive]
 pub struct EventErrorBuilder {
     pub(crate) payload: ::std::option::Option<crate::types::ErrorObject>,
@@ -70,3 +78,11 @@
         }
     }
 }
+impl ::std::fmt::Debug for EventErrorBuilder {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("EventErrorBuilder");
+        formatter.field("payload", &"*** Sensitive Data Redacted ***");
+        formatter.field("truncated", &self.truncated);
+        formatter.finish()
+    }
+}
```

### `src/types/_execution_failed_details.rs`

```diff
--- reference/src/types/_execution_failed_details.rs
+++ generated/src/types/_execution_failed_details.rs
@@ -2,7 +2,7 @@

 /// <p>Details about a failed <a href="https://docs.aws.amazon.com/lambda/latest/dg/durable-functions.html">durable execution</a>.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct ExecutionFailedDetails {
     /// <p>Details about the execution failure.</p>
     pub error: ::std::option::Option<crate::types::EventError>,
@@ -13,6 +13,13 @@
         self.error.as_ref()
     }
 }
+impl ::std::fmt::Debug for ExecutionFailedDetails {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("ExecutionFailedDetails");
+        formatter.field("error", &"*** Sensitive Data Redacted ***");
+        formatter.finish()
+    }
+}
 impl ExecutionFailedDetails {
     /// Creates a new builder-style object to manufacture [`ExecutionFailedDetails`](crate::types::ExecutionFailedDetails).
     pub fn builder() -> crate::types::builders::ExecutionFailedDetailsBuilder {
@@ -21,7 +28,7 @@
 }

 /// A builder for [`ExecutionFailedDetails`](crate::types::ExecutionFailedDetails).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
 #[non_exhaustive]
 pub struct ExecutionFailedDetailsBuilder {
     pub(crate) error: ::std::option::Option<crate::types::EventError>,
@@ -47,3 +54,10 @@
         crate::types::ExecutionFailedDetails { error: self.error }
     }
 }
+impl ::std::fmt::Debug for ExecutionFailedDetailsBuilder {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("ExecutionFailedDetailsBuilder");
+        formatter.field("error", &"*** Sensitive Data Redacted ***");
+        formatter.finish()
+    }
+}
```

### `src/types/_execution_started_details.rs`

```diff
--- reference/src/types/_execution_started_details.rs
+++ generated/src/types/_execution_started_details.rs
@@ -2,7 +2,7 @@

 /// <p>Details about a durable execution that started.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct ExecutionStartedDetails {
     /// <p>The input payload provided for the durable execution.</p>
     pub input: ::std::option::Option<crate::types::EventInput>,
@@ -19,6 +19,14 @@
         self.execution_timeout
     }
 }
+impl ::std::fmt::Debug for ExecutionStartedDetails {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("ExecutionStartedDetails");
+        formatter.field("input", &"*** Sensitive Data Redacted ***");
+        formatter.field("execution_timeout", &self.execution_timeout);
+        formatter.finish()
+    }
+}
 impl ExecutionStartedDetails {
     /// Creates a new builder-style object to manufacture [`ExecutionStartedDetails`](crate::types::ExecutionStartedDetails).
     pub fn builder() -> crate::types::builders::ExecutionStartedDetailsBuilder {
@@ -27,7 +35,7 @@
 }

 /// A builder for [`ExecutionStartedDetails`](crate::types::ExecutionStartedDetails).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
 #[non_exhaustive]
 pub struct ExecutionStartedDetailsBuilder {
     pub(crate) input: ::std::option::Option<crate::types::EventInput>,
@@ -79,3 +87,11 @@
         })
     }
 }
+impl ::std::fmt::Debug for ExecutionStartedDetailsBuilder {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("ExecutionStartedDetailsBuilder");
+        formatter.field("input", &"*** Sensitive Data Redacted ***");
+        formatter.field("execution_timeout", &self.execution_timeout);
+        formatter.finish()
+    }
+}
```

### `src/types/_execution_stopped_details.rs`

```diff
--- reference/src/types/_execution_stopped_details.rs
+++ generated/src/types/_execution_stopped_details.rs
@@ -2,7 +2,7 @@

 /// <p>Details about a <a href="https://docs.aws.amazon.com/lambda/latest/dg/durable-functions.html">durable execution</a> that stopped.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct ExecutionStoppedDetails {
     /// <p>Details about why the execution stopped.</p>
     pub error: ::std::option::Option<crate::types::EventError>,
@@ -13,6 +13,13 @@
         self.error.as_ref()
     }
 }
+impl ::std::fmt::Debug for ExecutionStoppedDetails {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("ExecutionStoppedDetails");
+        formatter.field("error", &"*** Sensitive Data Redacted ***");
+        formatter.finish()
+    }
+}
 impl ExecutionStoppedDetails {
     /// Creates a new builder-style object to manufacture [`ExecutionStoppedDetails`](crate::types::ExecutionStoppedDetails).
     pub fn builder() -> crate::types::builders::ExecutionStoppedDetailsBuilder {
@@ -21,7 +28,7 @@
 }

 /// A builder for [`ExecutionStoppedDetails`](crate::types::ExecutionStoppedDetails).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
 #[non_exhaustive]
 pub struct ExecutionStoppedDetailsBuilder {
     pub(crate) error: ::std::option::Option<crate::types::EventError>,
@@ -47,3 +54,10 @@
         crate::types::ExecutionStoppedDetails { error: self.error }
     }
 }
+impl ::std::fmt::Debug for ExecutionStoppedDetailsBuilder {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("ExecutionStoppedDetailsBuilder");
+        formatter.field("error", &"*** Sensitive Data Redacted ***");
+        formatter.finish()
+    }
+}
```

### `src/types/_execution_succeeded_details.rs`

```diff
--- reference/src/types/_execution_succeeded_details.rs
+++ generated/src/types/_execution_succeeded_details.rs
@@ -2,7 +2,7 @@

 /// <p>Details about a <a href="https://docs.aws.amazon.com/lambda/latest/dg/durable-functions.html">durable execution</a> that succeeded.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct ExecutionSucceededDetails {
     /// <p>The response payload from the successful operation.</p>
     pub result: ::std::option::Option<crate::types::EventResult>,
@@ -13,6 +13,13 @@
         self.result.as_ref()
     }
 }
+impl ::std::fmt::Debug for ExecutionSucceededDetails {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("ExecutionSucceededDetails");
+        formatter.field("result", &"*** Sensitive Data Redacted ***");
+        formatter.finish()
+    }
+}
 impl ExecutionSucceededDetails {
     /// Creates a new builder-style object to manufacture [`ExecutionSucceededDetails`](crate::types::ExecutionSucceededDetails).
     pub fn builder() -> crate::types::builders::ExecutionSucceededDetailsBuilder {
@@ -21,7 +28,7 @@
 }

 /// A builder for [`ExecutionSucceededDetails`](crate::types::ExecutionSucceededDetails).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
 #[non_exhaustive]
 pub struct ExecutionSucceededDetailsBuilder {
     pub(crate) result: ::std::option::Option<crate::types::EventResult>,
@@ -47,3 +54,10 @@
         crate::types::ExecutionSucceededDetails { result: self.result }
     }
 }
+impl ::std::fmt::Debug for ExecutionSucceededDetailsBuilder {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("ExecutionSucceededDetailsBuilder");
+        formatter.field("result", &"*** Sensitive Data Redacted ***");
+        formatter.finish()
+    }
+}
```

### `src/types/_execution_timed_out_details.rs`

```diff
--- reference/src/types/_execution_timed_out_details.rs
+++ generated/src/types/_execution_timed_out_details.rs
@@ -2,7 +2,7 @@

 /// <p>Details about a <a href="https://docs.aws.amazon.com/lambda/latest/dg/durable-functions.html">durable execution</a> that timed out.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct ExecutionTimedOutDetails {
     /// <p>Details about the execution timeout.</p>
     pub error: ::std::option::Option<crate::types::EventError>,
@@ -13,6 +13,13 @@
         self.error.as_ref()
     }
 }
+impl ::std::fmt::Debug for ExecutionTimedOutDetails {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("ExecutionTimedOutDetails");
+        formatter.field("error", &"*** Sensitive Data Redacted ***");
+        formatter.finish()
+    }
+}
 impl ExecutionTimedOutDetails {
     /// Creates a new builder-style object to manufacture [`ExecutionTimedOutDetails`](crate::types::ExecutionTimedOutDetails).
     pub fn builder() -> crate::types::builders::ExecutionTimedOutDetailsBuilder {
@@ -21,7 +28,7 @@
 }

 /// A builder for [`ExecutionTimedOutDetails`](crate::types::ExecutionTimedOutDetails).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
 #[non_exhaustive]
 pub struct ExecutionTimedOutDetailsBuilder {
     pub(crate) error: ::std::option::Option<crate::types::EventError>,
@@ -46,3 +53,10 @@
         crate::types::ExecutionTimedOutDetails { error: self.error }
     }
 }
+impl ::std::fmt::Debug for ExecutionTimedOutDetailsBuilder {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("ExecutionTimedOutDetailsBuilder");
+        formatter.field("error", &"*** Sensitive Data Redacted ***");
+        formatter.finish()
+    }
+}
```

### `src/types/_function_code.rs`

```diff
--- reference/src/types/_function_code.rs
+++ generated/src/types/_function_code.rs
@@ -5,7 +5,7 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct FunctionCode {
     /// <p>The base64-encoded contents of the deployment package. Amazon Web Services SDK and CLI clients handle the encoding for you.</p>
-    pub zip_file: ::std::option::Option<::aws_smithy_types::Blob>,
+    pub zip_file: ::std::option::Option<::std::vec::Vec<u8>>,
     /// <p>An Amazon S3 bucket in the same Amazon Web Services Region as your function. The bucket can be in a different Amazon Web Services account.</p>
     pub s3_bucket: ::std::option::Option<::std::string::String>,
     /// <p>The Amazon S3 key of the deployment package.</p>
@@ -27,7 +27,7 @@
 }
 impl FunctionCode {
     /// <p>The base64-encoded contents of the deployment package. Amazon Web Services SDK and CLI clients handle the encoding for you.</p>
-    pub fn zip_file(&self) -> ::std::option::Option<&::aws_smithy_types::Blob> {
+    pub fn zip_file(&self) -> ::std::option::Option<&::std::vec::Vec<u8>> {
         self.zip_file.as_ref()
     }
     /// <p>An Amazon S3 bucket in the same Amazon Web Services Region as your function. The bucket can be in a different Amazon Web Services account.</p>
@@ -85,7 +85,7 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
 #[non_exhaustive]
 pub struct FunctionCodeBuilder {
-    pub(crate) zip_file: ::std::option::Option<::aws_smithy_types::Blob>,
+    pub(crate) zip_file: ::std::option::Option<::std::vec::Vec<u8>>,
     pub(crate) s3_bucket: ::std::option::Option<::std::string::String>,
     pub(crate) s3_key: ::std::option::Option<::std::string::String>,
     pub(crate) s3_object_version: ::std::option::Option<::std::string::String>,
@@ -95,17 +95,17 @@
 }
 impl FunctionCodeBuilder {
     /// <p>The base64-encoded contents of the deployment package. Amazon Web Services SDK and CLI clients handle the encoding for you.</p>
-    pub fn zip_file(mut self, input: ::aws_smithy_types::Blob) -> Self {
+    pub fn zip_file(mut self, input: ::std::vec::Vec<u8>) -> Self {
         self.zip_file = ::std::option::Option::Some(input);
         self
     }
     /// <p>The base64-encoded contents of the deployment package. Amazon Web Services SDK and CLI clients handle the encoding for you.</p>
-    pub fn set_zip_file(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
+    pub fn set_zip_file(mut self, input: ::std::option::Option<::std::vec::Vec<u8>>) -> Self {
         self.zip_file = input;
         self
     }
     /// <p>The base64-encoded contents of the deployment package. Amazon Web Services SDK and CLI clients handle the encoding for you.</p>
-    pub fn get_zip_file(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
+    pub fn get_zip_file(&self) -> &::std::option::Option<::std::vec::Vec<u8>> {
         &self.zip_file
     }
     /// <p>An Amazon S3 bucket in the same Amazon Web Services Region as your function. The bucket can be in a different Amazon Web Services account.</p>
```

### `src/types/_function_code_location.rs`

```diff
--- reference/src/types/_function_code_location.rs
+++ generated/src/types/_function_code_location.rs
@@ -2,7 +2,7 @@

 /// <p>Details about a function's deployment package.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct FunctionCodeLocation {
     /// <p>The service that's hosting the file.</p>
     pub repository_type: ::std::option::Option<::std::string::String>,
@@ -49,6 +49,19 @@
         self.error.as_ref()
     }
 }
+impl ::std::fmt::Debug for FunctionCodeLocation {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("FunctionCodeLocation");
+        formatter.field("repository_type", &self.repository_type);
+        formatter.field("location", &self.location);
+        formatter.field("image_uri", &self.image_uri);
+        formatter.field("resolved_image_uri", &self.resolved_image_uri);
+        formatter.field("resolved_s3_object", &self.resolved_s3_object);
+        formatter.field("source_kms_key_arn", &self.source_kms_key_arn);
+        formatter.field("error", &"*** Sensitive Data Redacted ***");
+        formatter.finish()
+    }
+}
 impl FunctionCodeLocation {
     /// Creates a new builder-style object to manufacture [`FunctionCodeLocation`](crate::types::FunctionCodeLocation).
     pub fn builder() -> crate::types::builders::FunctionCodeLocationBuilder {
@@ -57,7 +70,7 @@
 }

 /// A builder for [`FunctionCodeLocation`](crate::types::FunctionCodeLocation).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
 #[non_exhaustive]
 pub struct FunctionCodeLocationBuilder {
     pub(crate) repository_type: ::std::option::Option<::std::string::String>,
@@ -180,3 +193,16 @@
         }
     }
 }
+impl ::std::fmt::Debug for FunctionCodeLocationBuilder {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("FunctionCodeLocationBuilder");
+        formatter.field("repository_type", &self.repository_type);
+        formatter.field("location", &self.location);
+        formatter.field("image_uri", &self.image_uri);
+        formatter.field("resolved_image_uri", &self.resolved_image_uri);
+        formatter.field("resolved_s3_object", &self.resolved_s3_object);
+        formatter.field("source_kms_key_arn", &self.source_kms_key_arn);
+        formatter.field("error", &"*** Sensitive Data Redacted ***");
+        formatter.finish()
+    }
+}
```

### `src/types/_function_configuration.rs`

```diff
--- reference/src/types/_function_configuration.rs
+++ generated/src/types/_function_configuration.rs
@@ -2,7 +2,7 @@

 /// <p>Details about a function's configuration.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct FunctionConfiguration {
     /// <p>The name of the function.</p>
     pub function_name: ::std::option::Option<::std::string::String>,
@@ -17,7 +17,7 @@
     /// <p>The function that Lambda calls to begin running your function.</p>
     pub handler: ::std::option::Option<::std::string::String>,
     /// <p>The size of the function's deployment package, in bytes.</p>
-    pub code_size: i64,
+    pub code_size: ::std::option::Option<i64>,
     /// <p>The function's description.</p>
     pub description: ::std::option::Option<::std::string::String>,
     /// <p>The amount of time in seconds that Lambda allows a function to run before stopping it.</p>
@@ -122,7 +122,7 @@
         self.handler.as_deref()
     }
     /// <p>The size of the function's deployment package, in bytes.</p>
-    pub fn code_size(&self) -> i64 {
+    pub fn code_size(&self) -> ::std::option::Option<i64> {
         self.code_size
     }
     /// <p>The function's description.</p>
@@ -279,6 +279,52 @@
         self.durable_config.as_ref()
     }
 }
+impl ::std::fmt::Debug for FunctionConfiguration {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("FunctionConfiguration");
+        formatter.field("function_name", &self.function_name);
+        formatter.field("function_arn", &self.function_arn);
+        formatter.field("runtime", &self.runtime);
+        formatter.field("role", &self.role);
+        formatter.field("handler", &self.handler);
+        formatter.field("code_size", &self.code_size);
+        formatter.field("description", &self.description);
+        formatter.field("timeout", &self.timeout);
+        formatter.field("memory_size", &self.memory_size);
+        formatter.field("last_modified", &self.last_modified);
+        formatter.field("code_sha256", &self.code_sha256);
+        formatter.field("version", &self.version);
+        formatter.field("vpc_config", &self.vpc_config);
+        formatter.field("dead_letter_config", &self.dead_letter_config);
+        formatter.field("environment", &"*** Sensitive Data Redacted ***");
+        formatter.field("kms_key_arn", &self.kms_key_arn);
+        formatter.field("tracing_config", &self.tracing_config);
+        formatter.field("master_arn", &self.master_arn);
+        formatter.field("revision_id", &self.revision_id);
+        formatter.field("layers", &self.layers);
+        formatter.field("state", &self.state);
+        formatter.field("state_reason", &self.state_reason);
+        formatter.field("state_reason_code", &self.state_reason_code);
+        formatter.field("last_update_status", &self.last_update_status);
+        formatter.field("last_update_status_reason", &self.last_update_status_reason);
+        formatter.field("last_update_status_reason_code", &self.last_update_status_reason_code);
+        formatter.field("file_system_configs", &self.file_system_configs);
+        formatter.field("signing_profile_version_arn", &self.signing_profile_version_arn);
+        formatter.field("signing_job_arn", &self.signing_job_arn);
+        formatter.field("package_type", &self.package_type);
+        formatter.field("image_config_response", &"*** Sensitive Data Redacted ***");
+        formatter.field("architectures", &self.architectures);
+        formatter.field("ephemeral_storage", &self.ephemeral_storage);
+        formatter.field("snap_start", &self.snap_start);
+        formatter.field("runtime_version_config", &"*** Sensitive Data Redacted ***");
+        formatter.field("logging_config", &self.logging_config);
+        formatter.field("tenancy_config", &self.tenancy_config);
+        formatter.field("capacity_provider_config", &self.capacity_provider_config);
+        formatter.field("config_sha256", &self.config_sha256);
+        formatter.field("durable_config", &self.durable_config);
+        formatter.finish()
+    }
+}
 impl FunctionConfiguration {
     /// Creates a new builder-style object to manufacture [`FunctionConfiguration`](crate::types::FunctionConfiguration).
     pub fn builder() -> crate::types::builders::FunctionConfigurationBuilder {
@@ -287,7 +333,7 @@
 }

 /// A builder for [`FunctionConfiguration`](crate::types::FunctionConfiguration).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
 #[non_exhaustive]
 pub struct FunctionConfigurationBuilder {
     pub(crate) function_name: ::std::option::Option<::std::string::String>,
@@ -957,7 +1003,7 @@
             runtime: self.runtime,
             role: self.role,
             handler: self.handler,
-            code_size: self.code_size.unwrap_or_default(),
+            code_size: self.code_size,
             description: self.description,
             timeout: self.timeout,
             memory_size: self.memory_size,
@@ -995,3 +1041,49 @@
         }
     }
 }
+impl ::std::fmt::Debug for FunctionConfigurationBuilder {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("FunctionConfigurationBuilder");
+        formatter.field("function_name", &self.function_name);
+        formatter.field("function_arn", &self.function_arn);
+        formatter.field("runtime", &self.runtime);
+        formatter.field("role", &self.role);
+        formatter.field("handler", &self.handler);
+        formatter.field("code_size", &self.code_size);
+        formatter.field("description", &self.description);
+        formatter.field("timeout", &self.timeout);
+        formatter.field("memory_size", &self.memory_size);
+        formatter.field("last_modified", &self.last_modified);
+        formatter.field("code_sha256", &self.code_sha256);
+        formatter.field("version", &self.version);
+        formatter.field("vpc_config", &self.vpc_config);
+        formatter.field("dead_letter_config", &self.dead_letter_config);
+        formatter.field("environment", &"*** Sensitive Data Redacted ***");
+        formatter.field("kms_key_arn", &self.kms_key_arn);
+        formatter.field("tracing_config", &self.tracing_config);
+        formatter.field("master_arn", &self.master_arn);
+        formatter.field("revision_id", &self.revision_id);
+        formatter.field("layers", &self.layers);
+        formatter.field("state", &self.state);
+        formatter.field("state_reason", &self.state_reason);
+        formatter.field("state_reason_code", &self.state_reason_code);
+        formatter.field("last_update_status", &self.last_update_status);
+        formatter.field("last_update_status_reason", &self.last_update_status_reason);
+        formatter.field("last_update_status_reason_code", &self.last_update_status_reason_code);
+        formatter.field("file_system_configs", &self.file_system_configs);
+        formatter.field("signing_profile_version_arn", &self.signing_profile_version_arn);
+        formatter.field("signing_job_arn", &self.signing_job_arn);
+        formatter.field("package_type", &self.package_type);
+        formatter.field("image_config_response", &"*** Sensitive Data Redacted ***");
+        formatter.field("architectures", &self.architectures);
+        formatter.field("ephemeral_storage", &self.ephemeral_storage);
+        formatter.field("snap_start", &self.snap_start);
+        formatter.field("runtime_version_config", &"*** Sensitive Data Redacted ***");
+        formatter.field("logging_config", &self.logging_config);
+        formatter.field("tenancy_config", &self.tenancy_config);
+        formatter.field("capacity_provider_config", &self.capacity_provider_config);
+        formatter.field("config_sha256", &self.config_sha256);
+        formatter.field("durable_config", &self.durable_config);
+        formatter.finish()
+    }
+}
```

### `src/types/_function_event_invoke_config.rs`

```diff
--- reference/src/types/_function_event_invoke_config.rs
+++ generated/src/types/_function_event_invoke_config.rs
@@ -1,4 +1,5 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+
 #[allow(missing_docs)] // documentation missing in model
 #[non_exhaustive]
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
```

### `src/types/_image_config_response.rs`

```diff
--- reference/src/types/_image_config_response.rs
+++ generated/src/types/_image_config_response.rs
@@ -2,7 +2,7 @@

 /// <p>Response to a <code>GetFunctionConfiguration</code> request.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct ImageConfigResponse {
     /// <p>Configuration values that override the container image Dockerfile.</p>
     pub image_config: ::std::option::Option<crate::types::ImageConfig>,
@@ -19,6 +19,14 @@
         self.error.as_ref()
     }
 }
+impl ::std::fmt::Debug for ImageConfigResponse {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("ImageConfigResponse");
+        formatter.field("image_config", &self.image_config);
+        formatter.field("error", &"*** Sensitive Data Redacted ***");
+        formatter.finish()
+    }
+}
 impl ImageConfigResponse {
     /// Creates a new builder-style object to manufacture [`ImageConfigResponse`](crate::types::ImageConfigResponse).
     pub fn builder() -> crate::types::builders::ImageConfigResponseBuilder {
@@ -27,7 +35,7 @@
 }

 /// A builder for [`ImageConfigResponse`](crate::types::ImageConfigResponse).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
 #[non_exhaustive]
 pub struct ImageConfigResponseBuilder {
     pub(crate) image_config: ::std::option::Option<crate::types::ImageConfig>,
@@ -70,3 +78,11 @@
         }
     }
 }
+impl ::std::fmt::Debug for ImageConfigResponseBuilder {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("ImageConfigResponseBuilder");
+        formatter.field("image_config", &self.image_config);
+        formatter.field("error", &"*** Sensitive Data Redacted ***");
+        formatter.finish()
+    }
+}
```

### `src/types/_invocation_completed_details.rs`

```diff
--- reference/src/types/_invocation_completed_details.rs
+++ generated/src/types/_invocation_completed_details.rs
@@ -2,7 +2,7 @@

 /// <p>Details about a function invocation that completed.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct InvocationCompletedDetails {
     /// <p>The date and time when the invocation started, in <a href="https://www.w3.org/TR/NOTE-datetime">ISO-8601 format</a> (YYYY-MM-DDThh:mm:ss.sTZD).</p>
     pub start_timestamp: ::aws_smithy_types::DateTime,
@@ -32,6 +32,16 @@
         self.error.as_ref()
     }
 }
+impl ::std::fmt::Debug for InvocationCompletedDetails {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("InvocationCompletedDetails");
+        formatter.field("start_timestamp", &self.start_timestamp);
+        formatter.field("end_timestamp", &self.end_timestamp);
+        formatter.field("request_id", &self.request_id);
+        formatter.field("error", &"*** Sensitive Data Redacted ***");
+        formatter.finish()
+    }
+}
 impl InvocationCompletedDetails {
     /// Creates a new builder-style object to manufacture [`InvocationCompletedDetails`](crate::types::InvocationCompletedDetails).
     pub fn builder() -> crate::types::builders::InvocationCompletedDetailsBuilder {
@@ -40,7 +50,7 @@
 }

 /// A builder for [`InvocationCompletedDetails`](crate::types::InvocationCompletedDetails).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
 #[non_exhaustive]
 pub struct InvocationCompletedDetailsBuilder {
     pub(crate) start_timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
@@ -137,3 +147,13 @@
         })
     }
 }
+impl ::std::fmt::Debug for InvocationCompletedDetailsBuilder {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("InvocationCompletedDetailsBuilder");
+        formatter.field("start_timestamp", &self.start_timestamp);
+        formatter.field("end_timestamp", &self.end_timestamp);
+        formatter.field("request_id", &self.request_id);
+        formatter.field("error", &"*** Sensitive Data Redacted ***");
+        formatter.finish()
+    }
+}
```

### `src/types/_invoke_response_stream_update.rs`

```diff
--- reference/src/types/_invoke_response_stream_update.rs
+++ generated/src/types/_invoke_response_stream_update.rs
@@ -5,11 +5,11 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct InvokeResponseStreamUpdate {
     /// <p>Data returned by your Lambda function.</p>
-    pub payload: ::std::option::Option<::aws_smithy_types::Blob>,
+    pub payload: ::std::option::Option<::std::vec::Vec<u8>>,
 }
 impl InvokeResponseStreamUpdate {
     /// <p>Data returned by your Lambda function.</p>
-    pub fn payload(&self) -> ::std::option::Option<&::aws_smithy_types::Blob> {
+    pub fn payload(&self) -> ::std::option::Option<&::std::vec::Vec<u8>> {
         self.payload.as_ref()
     }
 }
@@ -31,21 +31,21 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
 #[non_exhaustive]
 pub struct InvokeResponseStreamUpdateBuilder {
-    pub(crate) payload: ::std::option::Option<::aws_smithy_types::Blob>,
+    pub(crate) payload: ::std::option::Option<::std::vec::Vec<u8>>,
 }
 impl InvokeResponseStreamUpdateBuilder {
     /// <p>Data returned by your Lambda function.</p>
-    pub fn payload(mut self, input: ::aws_smithy_types::Blob) -> Self {
+    pub fn payload(mut self, input: ::std::vec::Vec<u8>) -> Self {
         self.payload = ::std::option::Option::Some(input);
         self
     }
     /// <p>Data returned by your Lambda function.</p>
-    pub fn set_payload(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
+    pub fn set_payload(mut self, input: ::std::option::Option<::std::vec::Vec<u8>>) -> Self {
         self.payload = input;
         self
     }
     /// <p>Data returned by your Lambda function.</p>
-    pub fn get_payload(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
+    pub fn get_payload(&self) -> &::std::option::Option<::std::vec::Vec<u8>> {
         &self.payload
     }
     /// Consumes the builder and constructs a [`InvokeResponseStreamUpdate`](crate::types::InvokeResponseStreamUpdate).
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

### `src/types/_layer.rs`

```diff
--- reference/src/types/_layer.rs
+++ generated/src/types/_layer.rs
@@ -7,7 +7,7 @@
     /// <p>The Amazon Resource Name (ARN) of the function layer.</p>
     pub arn: ::std::option::Option<::std::string::String>,
     /// <p>The size of the layer archive in bytes.</p>
-    pub code_size: i64,
+    pub code_size: ::std::option::Option<i64>,
     /// <p>The Amazon Resource Name (ARN) for a signing profile version.</p>
     pub signing_profile_version_arn: ::std::option::Option<::std::string::String>,
     /// <p>The Amazon Resource Name (ARN) of a signing job.</p>
@@ -19,7 +19,7 @@
         self.arn.as_deref()
     }
     /// <p>The size of the layer archive in bytes.</p>
-    pub fn code_size(&self) -> i64 {
+    pub fn code_size(&self) -> ::std::option::Option<i64> {
         self.code_size
     }
     /// <p>The Amazon Resource Name (ARN) for a signing profile version.</p>
@@ -108,7 +108,7 @@
     pub fn build(self) -> crate::types::Layer {
         crate::types::Layer {
             arn: self.arn,
-            code_size: self.code_size.unwrap_or_default(),
+            code_size: self.code_size,
             signing_profile_version_arn: self.signing_profile_version_arn,
             signing_job_arn: self.signing_job_arn,
         }
```

### `src/types/_layer_version_content_input.rs`

```diff
--- reference/src/types/_layer_version_content_input.rs
+++ generated/src/types/_layer_version_content_input.rs
@@ -19,7 +19,7 @@
     /// </ul>
     pub s3_object_storage_mode: ::std::option::Option<crate::types::S3ObjectStorageMode>,
     /// <p>The base64-encoded contents of the layer archive. Amazon Web Services SDK and Amazon Web Services CLI clients handle the encoding for you.</p>
-    pub zip_file: ::std::option::Option<::aws_smithy_types::Blob>,
+    pub zip_file: ::std::option::Option<::std::vec::Vec<u8>>,
 }
 impl LayerVersionContentInput {
     /// <p>The Amazon S3 bucket of the layer archive.</p>
@@ -45,7 +45,7 @@
         self.s3_object_storage_mode.as_ref()
     }
     /// <p>The base64-encoded contents of the layer archive. Amazon Web Services SDK and Amazon Web Services CLI clients handle the encoding for you.</p>
-    pub fn zip_file(&self) -> ::std::option::Option<&::aws_smithy_types::Blob> {
+    pub fn zip_file(&self) -> ::std::option::Option<&::std::vec::Vec<u8>> {
         self.zip_file.as_ref()
     }
 }
@@ -75,7 +75,7 @@
     pub(crate) s3_key: ::std::option::Option<::std::string::String>,
     pub(crate) s3_object_version: ::std::option::Option<::std::string::String>,
     pub(crate) s3_object_storage_mode: ::std::option::Option<crate::types::S3ObjectStorageMode>,
-    pub(crate) zip_file: ::std::option::Option<::aws_smithy_types::Blob>,
+    pub(crate) zip_file: ::std::option::Option<::std::vec::Vec<u8>>,
 }
 impl LayerVersionContentInputBuilder {
     /// <p>The Amazon S3 bucket of the layer archive.</p>
@@ -153,17 +153,17 @@
         &self.s3_object_storage_mode
     }
     /// <p>The base64-encoded contents of the layer archive. Amazon Web Services SDK and Amazon Web Services CLI clients handle the encoding for you.</p>
-    pub fn zip_file(mut self, input: ::aws_smithy_types::Blob) -> Self {
+    pub fn zip_file(mut self, input: ::std::vec::Vec<u8>) -> Self {
         self.zip_file = ::std::option::Option::Some(input);
         self
     }
     /// <p>The base64-encoded contents of the layer archive. Amazon Web Services SDK and Amazon Web Services CLI clients handle the encoding for you.</p>
-    pub fn set_zip_file(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
+    pub fn set_zip_file(mut self, input: ::std::option::Option<::std::vec::Vec<u8>>) -> Self {
         self.zip_file = input;
         self
     }
     /// <p>The base64-encoded contents of the layer archive. Amazon Web Services SDK and Amazon Web Services CLI clients handle the encoding for you.</p>
-    pub fn get_zip_file(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
+    pub fn get_zip_file(&self) -> &::std::option::Option<::std::vec::Vec<u8>> {
         &self.zip_file
     }
     /// Consumes the builder and constructs a [`LayerVersionContentInput`](crate::types::LayerVersionContentInput).
```

### `src/types/_layer_version_content_output.rs`

```diff
--- reference/src/types/_layer_version_content_output.rs
+++ generated/src/types/_layer_version_content_output.rs
@@ -9,7 +9,7 @@
     /// <p>The SHA-256 hash of the layer archive.</p>
     pub code_sha256: ::std::option::Option<::std::string::String>,
     /// <p>The size of the layer archive in bytes.</p>
-    pub code_size: i64,
+    pub code_size: ::std::option::Option<i64>,
     /// <p>The Amazon Resource Name (ARN) for a signing profile version.</p>
     pub signing_profile_version_arn: ::std::option::Option<::std::string::String>,
     /// <p>The Amazon Resource Name (ARN) of a signing job.</p>
@@ -27,7 +27,7 @@
         self.code_sha256.as_deref()
     }
     /// <p>The size of the layer archive in bytes.</p>
-    pub fn code_size(&self) -> i64 {
+    pub fn code_size(&self) -> ::std::option::Option<i64> {
         self.code_size
     }
     /// <p>The Amazon Resource Name (ARN) for a signing profile version.</p>
@@ -151,7 +151,7 @@
         crate::types::LayerVersionContentOutput {
             location: self.location,
             code_sha256: self.code_sha256,
-            code_size: self.code_size.unwrap_or_default(),
+            code_size: self.code_size,
             signing_profile_version_arn: self.signing_profile_version_arn,
             signing_job_arn: self.signing_job_arn,
             resolved_s3_object: self.resolved_s3_object,
```

### `src/types/_layer_versions_list_item.rs`

```diff
--- reference/src/types/_layer_versions_list_item.rs
+++ generated/src/types/_layer_versions_list_item.rs
@@ -7,7 +7,7 @@
     /// <p>The ARN of the layer version.</p>
     pub layer_version_arn: ::std::option::Option<::std::string::String>,
     /// <p>The version number.</p>
-    pub version: i64,
+    pub version: ::std::option::Option<i64>,
     /// <p>The description of the version.</p>
     pub description: ::std::option::Option<::std::string::String>,
     /// <p>The date that the version was created, in ISO 8601 format. For example, <code>2018-11-27T15:10:45.123+0000</code>.</p>
@@ -27,7 +27,7 @@
         self.layer_version_arn.as_deref()
     }
     /// <p>The version number.</p>
-    pub fn version(&self) -> i64 {
+    pub fn version(&self) -> ::std::option::Option<i64> {
         self.version
     }
     /// <p>The description of the version.</p>
@@ -197,7 +197,7 @@
     pub fn build(self) -> crate::types::LayerVersionsListItem {
         crate::types::LayerVersionsListItem {
             layer_version_arn: self.layer_version_arn,
-            version: self.version.unwrap_or_default(),
+            version: self.version,
             description: self.description,
             created_date: self.created_date,
             compatible_architectures: self.compatible_architectures,
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

### `src/types/_operation.rs`

```diff
--- reference/src/types/_operation.rs
+++ generated/src/types/_operation.rs
@@ -2,7 +2,7 @@

 /// <p>Information about an operation within a durable execution.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct Operation {
     /// <p>The unique identifier for this operation.</p>
     pub id: ::std::string::String,
@@ -92,6 +92,26 @@
         self.chained_invoke_details.as_ref()
     }
 }
+impl ::std::fmt::Debug for Operation {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("Operation");
+        formatter.field("id", &self.id);
+        formatter.field("parent_id", &self.parent_id);
+        formatter.field("name", &self.name);
+        formatter.field("type", &self.r#type);
+        formatter.field("sub_type", &self.sub_type);
+        formatter.field("start_timestamp", &self.start_timestamp);
+        formatter.field("end_timestamp", &self.end_timestamp);
+        formatter.field("status", &self.status);
+        formatter.field("execution_details", &"*** Sensitive Data Redacted ***");
+        formatter.field("context_details", &"*** Sensitive Data Redacted ***");
+        formatter.field("step_details", &"*** Sensitive Data Redacted ***");
+        formatter.field("wait_details", &self.wait_details);
+        formatter.field("callback_details", &"*** Sensitive Data Redacted ***");
+        formatter.field("chained_invoke_details", &"*** Sensitive Data Redacted ***");
+        formatter.finish()
+    }
+}
 impl Operation {
     /// Creates a new builder-style object to manufacture [`Operation`](crate::types::Operation).
     pub fn builder() -> crate::types::builders::OperationBuilder {
@@ -100,7 +120,7 @@
 }

 /// A builder for [`Operation`](crate::types::Operation).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
 #[non_exhaustive]
 pub struct OperationBuilder {
     pub(crate) id: ::std::option::Option<::std::string::String>,
@@ -322,7 +342,7 @@
     /// Consumes the builder and constructs a [`Operation`](crate::types::Operation).
     /// This method will fail if any of the following fields are not set:
     /// - [`id`](crate::types::builders::OperationBuilder::id)
-    /// - [`r#type`](crate::types::builders::OperationBuilder::type)
+    /// - [`r#type`](crate::types::builders::OperationBuilder::r#type)
     /// - [`start_timestamp`](crate::types::builders::OperationBuilder::start_timestamp)
     /// - [`status`](crate::types::builders::OperationBuilder::status)
     pub fn build(self) -> ::std::result::Result<crate::types::Operation, ::aws_smithy_types::error::operation::BuildError> {
@@ -364,3 +384,23 @@
         })
     }
 }
+impl ::std::fmt::Debug for OperationBuilder {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("OperationBuilder");
+        formatter.field("id", &self.id);
+        formatter.field("parent_id", &self.parent_id);
+        formatter.field("name", &self.name);
+        formatter.field("type", &self.r#type);
+        formatter.field("sub_type", &self.sub_type);
+        formatter.field("start_timestamp", &self.start_timestamp);
+        formatter.field("end_timestamp", &self.end_timestamp);
+        formatter.field("status", &self.status);
+        formatter.field("execution_details", &"*** Sensitive Data Redacted ***");
+        formatter.field("context_details", &"*** Sensitive Data Redacted ***");
+        formatter.field("step_details", &"*** Sensitive Data Redacted ***");
+        formatter.field("wait_details", &self.wait_details);
+        formatter.field("callback_details", &"*** Sensitive Data Redacted ***");
+        formatter.field("chained_invoke_details", &"*** Sensitive Data Redacted ***");
+        formatter.finish()
+    }
+}
```

### `src/types/_operation_update.rs`

```diff
--- reference/src/types/_operation_update.rs
+++ generated/src/types/_operation_update.rs
@@ -92,11 +92,11 @@
         formatter.field("id", &self.id);
         formatter.field("parent_id", &self.parent_id);
         formatter.field("name", &self.name);
-        formatter.field("r#type", &self.r#type);
+        formatter.field("type", &self.r#type);
         formatter.field("sub_type", &self.sub_type);
         formatter.field("action", &self.action);
         formatter.field("payload", &"*** Sensitive Data Redacted ***");
-        formatter.field("error", &self.error);
+        formatter.field("error", &"*** Sensitive Data Redacted ***");
         formatter.field("context_options", &self.context_options);
         formatter.field("step_options", &self.step_options);
         formatter.field("wait_options", &self.wait_options);
@@ -319,7 +319,7 @@
     /// Consumes the builder and constructs a [`OperationUpdate`](crate::types::OperationUpdate).
     /// This method will fail if any of the following fields are not set:
     /// - [`id`](crate::types::builders::OperationUpdateBuilder::id)
-    /// - [`r#type`](crate::types::builders::OperationUpdateBuilder::type)
+    /// - [`r#type`](crate::types::builders::OperationUpdateBuilder::r#type)
     /// - [`action`](crate::types::builders::OperationUpdateBuilder::action)
     pub fn build(self) -> ::std::result::Result<crate::types::OperationUpdate, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(crate::types::OperationUpdate {
@@ -360,11 +360,11 @@
         formatter.field("id", &self.id);
         formatter.field("parent_id", &self.parent_id);
         formatter.field("name", &self.name);
-        formatter.field("r#type", &self.r#type);
+        formatter.field("type", &self.r#type);
         formatter.field("sub_type", &self.sub_type);
         formatter.field("action", &self.action);
         formatter.field("payload", &"*** Sensitive Data Redacted ***");
-        formatter.field("error", &self.error);
+        formatter.field("error", &"*** Sensitive Data Redacted ***");
         formatter.field("context_options", &self.context_options);
         formatter.field("step_options", &self.step_options);
         formatter.field("wait_options", &self.wait_options);
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

### `src/types/_retry_details.rs`

```diff
--- reference/src/types/_retry_details.rs
+++ generated/src/types/_retry_details.rs
@@ -5,13 +5,13 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub struct RetryDetails {
     /// <p>The current attempt number for this operation.</p>
-    pub current_attempt: i32,
+    pub current_attempt: ::std::option::Option<i32>,
     /// <p>The delay before the next retry attempt, in seconds.</p>
     pub next_attempt_delay_seconds: ::std::option::Option<i32>,
 }
 impl RetryDetails {
     /// <p>The current attempt number for this operation.</p>
-    pub fn current_attempt(&self) -> i32 {
+    pub fn current_attempt(&self) -> ::std::option::Option<i32> {
         self.current_attempt
     }
     /// <p>The delay before the next retry attempt, in seconds.</p>
@@ -65,7 +65,7 @@
     /// Consumes the builder and constructs a [`RetryDetails`](crate::types::RetryDetails).
     pub fn build(self) -> crate::types::RetryDetails {
         crate::types::RetryDetails {
-            current_attempt: self.current_attempt.unwrap_or_default(),
+            current_attempt: self.current_attempt,
             next_attempt_delay_seconds: self.next_attempt_delay_seconds,
         }
     }
```

### `src/types/_runtime_version_config.rs`

```diff
--- reference/src/types/_runtime_version_config.rs
+++ generated/src/types/_runtime_version_config.rs
@@ -2,7 +2,7 @@

 /// <p>The ARN of the runtime and any errors that occured.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct RuntimeVersionConfig {
     /// <p>The ARN of the runtime version you want the function to use.</p>
     pub runtime_version_arn: ::std::option::Option<::std::string::String>,
@@ -19,6 +19,14 @@
         self.error.as_ref()
     }
 }
+impl ::std::fmt::Debug for RuntimeVersionConfig {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("RuntimeVersionConfig");
+        formatter.field("runtime_version_arn", &self.runtime_version_arn);
+        formatter.field("error", &"*** Sensitive Data Redacted ***");
+        formatter.finish()
+    }
+}
 impl RuntimeVersionConfig {
     /// Creates a new builder-style object to manufacture [`RuntimeVersionConfig`](crate::types::RuntimeVersionConfig).
     pub fn builder() -> crate::types::builders::RuntimeVersionConfigBuilder {
@@ -27,7 +35,7 @@
 }

 /// A builder for [`RuntimeVersionConfig`](crate::types::RuntimeVersionConfig).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
 #[non_exhaustive]
 pub struct RuntimeVersionConfigBuilder {
     pub(crate) runtime_version_arn: ::std::option::Option<::std::string::String>,
@@ -70,3 +78,11 @@
         }
     }
 }
+impl ::std::fmt::Debug for RuntimeVersionConfigBuilder {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("RuntimeVersionConfigBuilder");
+        formatter.field("runtime_version_arn", &self.runtime_version_arn);
+        formatter.field("error", &"*** Sensitive Data Redacted ***");
+        formatter.finish()
+    }
+}
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

### `src/types/_step_details.rs`

```diff
--- reference/src/types/_step_details.rs
+++ generated/src/types/_step_details.rs
@@ -5,7 +5,7 @@
 #[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct StepDetails {
     /// <p>The current attempt number for this step.</p>
-    pub attempt: i32,
+    pub attempt: ::std::option::Option<i32>,
     /// <p>The date and time when the next attempt is scheduled, in <a href="https://www.w3.org/TR/NOTE-datetime">ISO-8601 format</a> (YYYY-MM-DDThh:mm:ss.sTZD). Only populated when the step is in a pending state.</p>
     pub next_attempt_timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
     /// <p>The JSON response payload from the step operation.</p>
@@ -15,7 +15,7 @@
 }
 impl StepDetails {
     /// <p>The current attempt number for this step.</p>
-    pub fn attempt(&self) -> i32 {
+    pub fn attempt(&self) -> ::std::option::Option<i32> {
         self.attempt
     }
     /// <p>The date and time when the next attempt is scheduled, in <a href="https://www.w3.org/TR/NOTE-datetime">ISO-8601 format</a> (YYYY-MM-DDThh:mm:ss.sTZD). Only populated when the step is in a pending state.</p>
@@ -37,7 +37,7 @@
         formatter.field("attempt", &self.attempt);
         formatter.field("next_attempt_timestamp", &self.next_attempt_timestamp);
         formatter.field("result", &"*** Sensitive Data Redacted ***");
-        formatter.field("error", &self.error);
+        formatter.field("error", &"*** Sensitive Data Redacted ***");
         formatter.finish()
     }
 }
@@ -117,7 +117,7 @@
     /// Consumes the builder and constructs a [`StepDetails`](crate::types::StepDetails).
     pub fn build(self) -> crate::types::StepDetails {
         crate::types::StepDetails {
-            attempt: self.attempt.unwrap_or_default(),
+            attempt: self.attempt,
             next_attempt_timestamp: self.next_attempt_timestamp,
             result: self.result,
             error: self.error,
@@ -130,7 +130,7 @@
         formatter.field("attempt", &self.attempt);
         formatter.field("next_attempt_timestamp", &self.next_attempt_timestamp);
         formatter.field("result", &"*** Sensitive Data Redacted ***");
-        formatter.field("error", &self.error);
+        formatter.field("error", &"*** Sensitive Data Redacted ***");
         formatter.finish()
     }
 }
```

### `src/types/_step_failed_details.rs`

```diff
--- reference/src/types/_step_failed_details.rs
+++ generated/src/types/_step_failed_details.rs
@@ -2,7 +2,7 @@

 /// <p>Details about a step that failed.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct StepFailedDetails {
     /// <p>Details about the step failure.</p>
     pub error: ::std::option::Option<crate::types::EventError>,
@@ -19,6 +19,14 @@
         self.retry_details.as_ref()
     }
 }
+impl ::std::fmt::Debug for StepFailedDetails {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("StepFailedDetails");
+        formatter.field("error", &"*** Sensitive Data Redacted ***");
+        formatter.field("retry_details", &self.retry_details);
+        formatter.finish()
+    }
+}
 impl StepFailedDetails {
     /// Creates a new builder-style object to manufacture [`StepFailedDetails`](crate::types::StepFailedDetails).
     pub fn builder() -> crate::types::builders::StepFailedDetailsBuilder {
@@ -27,7 +35,7 @@
 }

 /// A builder for [`StepFailedDetails`](crate::types::StepFailedDetails).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
 #[non_exhaustive]
 pub struct StepFailedDetailsBuilder {
     pub(crate) error: ::std::option::Option<crate::types::EventError>,
@@ -72,3 +80,11 @@
         }
     }
 }
+impl ::std::fmt::Debug for StepFailedDetailsBuilder {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("StepFailedDetailsBuilder");
+        formatter.field("error", &"*** Sensitive Data Redacted ***");
+        formatter.field("retry_details", &self.retry_details);
+        formatter.finish()
+    }
+}
```

### `src/types/_step_succeeded_details.rs`

```diff
--- reference/src/types/_step_succeeded_details.rs
+++ generated/src/types/_step_succeeded_details.rs
@@ -2,7 +2,7 @@

 /// <p>Details about a step that succeeded.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct StepSucceededDetails {
     /// <p>The response payload from the successful operation.</p>
     pub result: ::std::option::Option<crate::types::EventResult>,
@@ -19,6 +19,14 @@
         self.retry_details.as_ref()
     }
 }
+impl ::std::fmt::Debug for StepSucceededDetails {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("StepSucceededDetails");
+        formatter.field("result", &"*** Sensitive Data Redacted ***");
+        formatter.field("retry_details", &self.retry_details);
+        formatter.finish()
+    }
+}
 impl StepSucceededDetails {
     /// Creates a new builder-style object to manufacture [`StepSucceededDetails`](crate::types::StepSucceededDetails).
     pub fn builder() -> crate::types::builders::StepSucceededDetailsBuilder {
@@ -27,7 +35,7 @@
 }

 /// A builder for [`StepSucceededDetails`](crate::types::StepSucceededDetails).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
 #[non_exhaustive]
 pub struct StepSucceededDetailsBuilder {
     pub(crate) result: ::std::option::Option<crate::types::EventResult>,
@@ -72,3 +80,11 @@
         }
     }
 }
+impl ::std::fmt::Debug for StepSucceededDetailsBuilder {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("StepSucceededDetailsBuilder");
+        formatter.field("result", &"*** Sensitive Data Redacted ***");
+        formatter.field("retry_details", &self.retry_details);
+        formatter.finish()
+    }
+}
```

### `src/types/_wait_cancelled_details.rs`

```diff
--- reference/src/types/_wait_cancelled_details.rs
+++ generated/src/types/_wait_cancelled_details.rs
@@ -2,7 +2,7 @@

 /// <p>Details about a wait operation that was cancelled.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
 pub struct WaitCancelledDetails {
     /// <p>Details about why the wait operation was cancelled.</p>
     pub error: ::std::option::Option<crate::types::EventError>,
@@ -13,6 +13,13 @@
         self.error.as_ref()
     }
 }
+impl ::std::fmt::Debug for WaitCancelledDetails {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("WaitCancelledDetails");
+        formatter.field("error", &"*** Sensitive Data Redacted ***");
+        formatter.finish()
+    }
+}
 impl WaitCancelledDetails {
     /// Creates a new builder-style object to manufacture [`WaitCancelledDetails`](crate::types::WaitCancelledDetails).
     pub fn builder() -> crate::types::builders::WaitCancelledDetailsBuilder {
@@ -21,7 +28,7 @@
 }

 /// A builder for [`WaitCancelledDetails`](crate::types::WaitCancelledDetails).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
 #[non_exhaustive]
 pub struct WaitCancelledDetailsBuilder {
     pub(crate) error: ::std::option::Option<crate::types::EventError>,
@@ -46,3 +53,10 @@
         crate::types::WaitCancelledDetails { error: self.error }
     }
 }
+impl ::std::fmt::Debug for WaitCancelledDetailsBuilder {
+    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
+        let mut formatter = f.debug_struct("WaitCancelledDetailsBuilder");
+        formatter.field("error", &"*** Sensitive Data Redacted ***");
+        formatter.finish()
+    }
+}
```

### `src/types/builders.rs`

```diff
--- reference/src/types/builders.rs
+++ generated/src/types/builders.rs
@@ -1,11 +1,7 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-pub use crate::types::_account_limit::AccountLimitBuilder;
-
-pub use crate::types::_account_usage::AccountUsageBuilder;
+pub use crate::types::_checkpoint_updated_execution_state::CheckpointUpdatedExecutionStateBuilder;

-pub use crate::types::_destination_config::DestinationConfigBuilder;
-
-pub use crate::types::_error_object::ErrorObjectBuilder;
+pub use crate::types::_alias_routing_configuration::AliasRoutingConfigurationBuilder;

 pub use crate::types::_capacity_provider_vpc_config::CapacityProviderVpcConfigBuilder;

@@ -27,12 +23,6 @@

 pub use crate::types::_code_signing_config::CodeSigningConfigBuilder;

-pub use crate::types::_trace_header::TraceHeaderBuilder;
-
-pub use crate::types::_durable_config::DurableConfigBuilder;
-
-pub use crate::types::_checkpoint_updated_execution_state::CheckpointUpdatedExecutionStateBuilder;
-
 pub use crate::types::_filter_criteria::FilterCriteriaBuilder;

 pub use crate::types::_event_source_mapping_metrics_config::EventSourceMappingMetricsConfigBuilder;
@@ -41,6 +31,8 @@

 pub use crate::types::_scaling_config::ScalingConfigBuilder;

+pub use crate::types::_destination_config::DestinationConfigBuilder;
+
 pub use crate::types::_self_managed_event_source::SelfManagedEventSourceBuilder;

 pub use crate::types::_amazon_managed_kafka_event_source_config::AmazonManagedKafkaEventSourceConfigBuilder;
@@ -75,6 +67,8 @@

 pub use crate::types::_capacity_provider_config::CapacityProviderConfigBuilder;

+pub use crate::types::_durable_config::DurableConfigBuilder;
+
 pub use crate::types::_vpc_config_response::VpcConfigResponseBuilder;

 pub use crate::types::_environment_response::EnvironmentResponseBuilder;
@@ -89,6 +83,14 @@

 pub use crate::types::_cors::CorsBuilder;

+pub use crate::types::_account_limit::AccountLimitBuilder;
+
+pub use crate::types::_account_usage::AccountUsageBuilder;
+
+pub use crate::types::_error_object::ErrorObjectBuilder;
+
+pub use crate::types::_trace_header::TraceHeaderBuilder;
+
 pub use crate::types::_function_configuration::FunctionConfigurationBuilder;

 pub use crate::types::_function_code_location::FunctionCodeLocationBuilder;
@@ -99,29 +101,17 @@

 pub use crate::types::_function_scaling_config::FunctionScalingConfigBuilder;

-pub use crate::types::_alias_routing_configuration::AliasRoutingConfigurationBuilder;
-
 pub use crate::types::_layer_version_content_output::LayerVersionContentOutputBuilder;

 pub use crate::types::_layer_version_content_input::LayerVersionContentInputBuilder;

-pub use crate::types::_on_success::OnSuccessBuilder;
-
-pub use crate::types::_on_failure::OnFailureBuilder;
-
-pub use crate::types::_function_event_invoke_config::FunctionEventInvokeConfigBuilder;
+pub use crate::types::_operation_update::OperationUpdateBuilder;

 pub use crate::types::_capacity_provider_logging_config::CapacityProviderLoggingConfigBuilder;

-pub use crate::types::_function_versions_by_capacity_provider_list_item::FunctionVersionsByCapacityProviderListItemBuilder;
+pub use crate::types::_on_success::OnSuccessBuilder;

-pub use crate::types::_operation_update::OperationUpdateBuilder;
-
-pub use crate::types::_event::EventBuilder;
-
-pub use crate::types::_operation::OperationBuilder;
-
-pub use crate::types::_event_source_mapping_configuration::EventSourceMappingConfigurationBuilder;
+pub use crate::types::_on_failure::OnFailureBuilder;

 pub use crate::types::_source_access_configuration::SourceAccessConfigurationBuilder;

@@ -139,8 +129,10 @@

 pub use crate::types::_runtime_version_error::RuntimeVersionErrorBuilder;

-pub use crate::types::_provisioned_concurrency_config_list_item::ProvisionedConcurrencyConfigListItemBuilder;
+pub use crate::types::_event::EventBuilder;

+pub use crate::types::_operation::OperationBuilder;
+
 pub use crate::types::_resolved_s3_object::ResolvedS3ObjectBuilder;

 pub use crate::types::_function_code_location_error::FunctionCodeLocationErrorBuilder;
@@ -149,18 +141,24 @@

 pub use crate::types::_invoke_with_response_stream_complete_event::InvokeWithResponseStreamCompleteEventBuilder;

+pub use crate::types::_alias_configuration::AliasConfigurationBuilder;
+
 pub use crate::types::_execution::ExecutionBuilder;

-pub use crate::types::_function_url_config::FunctionUrlConfigBuilder;
+pub use crate::types::_event_source_mapping_configuration::EventSourceMappingConfigurationBuilder;

-pub use crate::types::_alias_configuration::AliasConfigurationBuilder;
+pub use crate::types::_function_event_invoke_config::FunctionEventInvokeConfigBuilder;

-pub use crate::types::_layers_list_item::LayersListItemBuilder;
+pub use crate::types::_function_url_config::FunctionUrlConfigBuilder;
+
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
@@ -1,15 +1,11 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub use crate::types::_throttle_reason::ThrottleReason;

-pub use crate::types::_account_limit::AccountLimit;
+pub use crate::types::_function_url_auth_type::FunctionUrlAuthType;

-pub use crate::types::_account_usage::AccountUsage;
-
-pub use crate::types::_destination_config::DestinationConfig;
-
-pub use crate::types::_error_object::ErrorObject;
+pub use crate::types::_checkpoint_updated_execution_state::CheckpointUpdatedExecutionState;

-pub use crate::types::_capacity_provider_state::CapacityProviderState;
+pub use crate::types::_alias_routing_configuration::AliasRoutingConfiguration;

 pub use crate::types::_capacity_provider_vpc_config::CapacityProviderVpcConfig;

@@ -31,14 +27,6 @@

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
@@ -49,6 +37,8 @@

 pub use crate::types::_event_source_position::EventSourcePosition;

+pub use crate::types::_destination_config::DestinationConfig;
+
 pub use crate::types::_self_managed_event_source::SelfManagedEventSource;

 pub use crate::types::_amazon_managed_kafka_event_source_config::AmazonManagedKafkaEventSourceConfig;
@@ -61,8 +51,6 @@

 pub use crate::types::_filter_criteria_error::FilterCriteriaError;

-pub use crate::types::_function_version::FunctionVersion;
-
 pub use crate::types::_runtime::Runtime;

 pub use crate::types::_function_code::FunctionCode;
@@ -91,6 +79,8 @@

 pub use crate::types::_capacity_provider_config::CapacityProviderConfig;

+pub use crate::types::_durable_config::DurableConfig;
+
 pub use crate::types::_vpc_config_response::VpcConfigResponse;

 pub use crate::types::_environment_response::EnvironmentResponse;
@@ -111,14 +101,20 @@

 pub use crate::types::_runtime_version_config::RuntimeVersionConfig;

-pub use crate::types::_s3_object_storage_mode::S3ObjectStorageMode;
-
-pub use crate::types::_function_url_auth_type::FunctionUrlAuthType;
-
 pub use crate::types::_cors::Cors;

 pub use crate::types::_invoke_mode::InvokeMode;

+pub use crate::types::_account_limit::AccountLimit;
+
+pub use crate::types::_account_usage::AccountUsage;
+
+pub use crate::types::_error_object::ErrorObject;
+
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
@@ -141,21 +141,17 @@

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
-
-pub use crate::types::_on_success::OnSuccess;
-
-pub use crate::types::_on_failure::OnFailure;
+pub use crate::types::_s3_object_storage_mode::S3ObjectStorageMode;

-pub use crate::types::_function_event_invoke_config::FunctionEventInvokeConfig;
+pub use crate::types::_operation_update::OperationUpdate;

 pub use crate::types::_capacity_provider_scaling_mode::CapacityProviderScalingMode;

@@ -163,19 +159,13 @@

 pub use crate::types::_capacity_provider_logging_config::CapacityProviderLoggingConfig;

-pub use crate::types::_function_versions_by_capacity_provider_list_item::FunctionVersionsByCapacityProviderListItem;
-
 pub use crate::types::_code_signing_policy::CodeSigningPolicy;

-pub use crate::types::_operation_update::OperationUpdate;
-
-pub use crate::types::_event::Event;
-
-pub use crate::types::_operation::Operation;
+pub use crate::types::_event_source_mapping_system_log_level::EventSourceMappingSystemLogLevel;

-pub use crate::types::_event_source_mapping_configuration::EventSourceMappingConfiguration;
+pub use crate::types::_on_success::OnSuccess;

-pub use crate::types::_event_source_mapping_system_log_level::EventSourceMappingSystemLogLevel;
+pub use crate::types::_on_failure::OnFailure;

 pub use crate::types::_source_access_configuration::SourceAccessConfiguration;

@@ -211,7 +201,9 @@

 pub use crate::types::_runtime_version_error::RuntimeVersionError;

-pub use crate::types::_provisioned_concurrency_config_list_item::ProvisionedConcurrencyConfigListItem;
+pub use crate::types::_event::Event;
+
+pub use crate::types::_operation::Operation;

 pub use crate::types::_resolved_s3_object::ResolvedS3Object;

@@ -221,17 +213,23 @@

 pub use crate::types::_invoke_with_response_stream_complete_event::InvokeWithResponseStreamCompleteEvent;

+pub use crate::types::_alias_configuration::AliasConfiguration;
+
 pub use crate::types::_execution::Execution;

+pub use crate::types::_event_source_mapping_configuration::EventSourceMappingConfiguration;
+
+pub use crate::types::_function_event_invoke_config::FunctionEventInvokeConfig;
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
-
-pub use crate::types::_schema_registry_event_record_format::SchemaRegistryEventRecordFormat;
+pub use crate::types::_kafka_schema_registry_access_config::KafkaSchemaRegistryAccessConfig;

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
- `src/config/auth.rs`
- `src/config/endpoint.rs`
- `src/endpoint_lib/bdd_interpreter.rs`
- `src/endpoint_lib/diagnostic.rs`
- `src/endpoint_lib/host.rs`
- `src/endpoint_lib/partition.rs`
- `src/endpoint_lib.rs`
- `src/event_receiver.rs`
- `src/event_stream_serde.rs`
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

- `src/client/create_function.rs`
- `src/client/delete_function.rs`
- `src/client/delete_resource_policy.rs`
- `src/client/get_function_configuration.rs`
- `src/client/get_function_event_invoke_config.rs`
- `src/client/get_function_recursion_config.rs`
- `src/client/get_layer_version.rs`
- `src/client/get_layer_version_by_arn.rs`
- `src/client/get_resource_policy.rs`
- `src/client/invoke.rs`
- `src/client/invoke_async.rs`
- `src/client/invoke_with_response_stream.rs`
- `src/client/publish_layer_version.rs`
- `src/client/publish_version.rs`
- `src/client/put_function_event_invoke_config.rs`
- `src/client/put_resource_policy.rs`
- `src/client/update_function_code.rs`
- `src/client/update_function_configuration.rs`
- `src/client/update_function_event_invoke_config.rs`
- `src/client.rs`
- `src/config.rs`
- `src/lib.rs`
- `src/operation/add_layer_version_permission.rs`
- `src/operation/add_permission.rs`
- `src/operation/checkpoint_durable_execution/_checkpoint_durable_execution_input.rs`
- `src/operation/checkpoint_durable_execution/_checkpoint_durable_execution_output.rs`
- `src/operation/checkpoint_durable_execution.rs`
- `src/operation/create_alias.rs`
- `src/operation/create_capacity_provider.rs`
- `src/operation/create_code_signing_config.rs`
- `src/operation/create_event_source_mapping.rs`
- `src/operation/create_function/_create_function_input.rs`
- `src/operation/create_function/_create_function_output.rs`
- `src/operation/create_function.rs`
- `src/operation/create_function_url_config.rs`
- `src/operation/delete_alias.rs`
- `src/operation/delete_capacity_provider.rs`
- `src/operation/delete_code_signing_config.rs`
- `src/operation/delete_event_source_mapping.rs`
- `src/operation/delete_function/_delete_function_output.rs`
- `src/operation/delete_function.rs`
- `src/operation/delete_function_code_signing_config.rs`
- `src/operation/delete_function_concurrency.rs`
- `src/operation/delete_function_event_invoke_config.rs`
- `src/operation/delete_function_url_config.rs`
- `src/operation/delete_layer_version.rs`
- `src/operation/delete_provisioned_concurrency_config.rs`
- `src/operation/delete_resource_policy.rs`
- `src/operation/get_account_settings.rs`
- `src/operation/get_alias.rs`
- `src/operation/get_capacity_provider.rs`
- `src/operation/get_code_signing_config.rs`
- `src/operation/get_durable_execution/_get_durable_execution_output.rs`
- `src/operation/get_durable_execution.rs`
- `src/operation/get_durable_execution_history/_get_durable_execution_history_output.rs`
- `src/operation/get_durable_execution_history.rs`
- `src/operation/get_durable_execution_state/_get_durable_execution_state_output.rs`
- `src/operation/get_durable_execution_state.rs`
- `src/operation/get_event_source_mapping.rs`
- `src/operation/get_function/_get_function_output.rs`
- `src/operation/get_function.rs`
- `src/operation/get_function_code_signing_config.rs`
- `src/operation/get_function_concurrency.rs`
- `src/operation/get_function_configuration/_get_function_configuration_output.rs`
- `src/operation/get_function_configuration.rs`
- `src/operation/get_function_event_invoke_config.rs`
- `src/operation/get_function_recursion_config.rs`
- `src/operation/get_function_scaling_config.rs`
- `src/operation/get_function_url_config.rs`
- `src/operation/get_layer_version/_get_layer_version_output.rs`
- `src/operation/get_layer_version.rs`
- `src/operation/get_layer_version_by_arn/_get_layer_version_by_arn_output.rs`
- `src/operation/get_layer_version_by_arn.rs`
- `src/operation/get_layer_version_policy.rs`
- `src/operation/get_policy.rs`
- `src/operation/get_provisioned_concurrency_config.rs`
- `src/operation/get_resource_policy.rs`
- `src/operation/get_runtime_management_config.rs`
- `src/operation/invoke/_invoke_input.rs`
- `src/operation/invoke/_invoke_output.rs`
- `src/operation/invoke/builders.rs`
- `src/operation/invoke.rs`
- `src/operation/invoke_async/_invoke_async_input.rs`
- `src/operation/invoke_async/_invoke_async_output.rs`
- `src/operation/invoke_async/builders.rs`
- `src/operation/invoke_async.rs`
- `src/operation/invoke_with_response_stream/_invoke_with_response_stream_input.rs`
- `src/operation/invoke_with_response_stream/_invoke_with_response_stream_output.rs`
- `src/operation/invoke_with_response_stream/builders.rs`
- `src/operation/invoke_with_response_stream.rs`
- `src/operation/list_aliases.rs`
- `src/operation/list_capacity_providers.rs`
- `src/operation/list_code_signing_configs.rs`
- `src/operation/list_durable_executions_by_function.rs`
- `src/operation/list_event_source_mappings.rs`
- `src/operation/list_function_event_invoke_configs.rs`
- `src/operation/list_function_url_configs.rs`
- `src/operation/list_function_versions_by_capacity_provider.rs`
- `src/operation/list_functions/_list_functions_output.rs`
- `src/operation/list_functions.rs`
- `src/operation/list_functions_by_code_signing_config.rs`
- `src/operation/list_layer_versions.rs`
- `src/operation/list_layers.rs`
- `src/operation/list_provisioned_concurrency_configs.rs`
- `src/operation/list_tags.rs`
- `src/operation/list_versions_by_function/_list_versions_by_function_output.rs`
- `src/operation/list_versions_by_function.rs`
- `src/operation/publish_layer_version/_publish_layer_version_input.rs`
- `src/operation/publish_layer_version/_publish_layer_version_output.rs`
- `src/operation/publish_layer_version/builders.rs`
- `src/operation/publish_layer_version.rs`
- `src/operation/publish_version/_publish_version_output.rs`
- `src/operation/publish_version.rs`
- `src/operation/put_function_code_signing_config.rs`
- `src/operation/put_function_concurrency.rs`
- `src/operation/put_function_event_invoke_config.rs`
- `src/operation/put_function_recursion_config.rs`
- `src/operation/put_function_scaling_config.rs`
- `src/operation/put_provisioned_concurrency_config.rs`
- `src/operation/put_resource_policy.rs`
- `src/operation/put_runtime_management_config.rs`
- `src/operation/remove_layer_version_permission.rs`
- `src/operation/remove_permission.rs`
- `src/operation/send_durable_execution_callback_failure/_send_durable_execution_callback_failure_input.rs`
- `src/operation/send_durable_execution_callback_failure.rs`
- `src/operation/send_durable_execution_callback_heartbeat.rs`
- `src/operation/send_durable_execution_callback_success/_send_durable_execution_callback_success_input.rs`
- `src/operation/send_durable_execution_callback_success/builders.rs`
- `src/operation/send_durable_execution_callback_success.rs`
- `src/operation/stop_durable_execution/_stop_durable_execution_input.rs`
- `src/operation/stop_durable_execution.rs`
- `src/operation/tag_resource.rs`
- `src/operation/untag_resource.rs`
- `src/operation/update_alias.rs`
- `src/operation/update_capacity_provider.rs`
- `src/operation/update_code_signing_config.rs`
- `src/operation/update_event_source_mapping.rs`
- `src/operation/update_function_code/_update_function_code_input.rs`
- `src/operation/update_function_code/_update_function_code_output.rs`
- `src/operation/update_function_code/builders.rs`
- `src/operation/update_function_code.rs`
- `src/operation/update_function_configuration/_update_function_configuration_input.rs`
- `src/operation/update_function_configuration/_update_function_configuration_output.rs`
- `src/operation/update_function_configuration.rs`
- `src/operation/update_function_event_invoke_config.rs`
- `src/operation/update_function_url_config.rs`
- `src/operation.rs`
- `src/serde_util.rs`
- `src/types/_account_limit.rs`
- `src/types/_account_usage.rs`
- `src/types/_callback_details.rs`
- `src/types/_callback_failed_details.rs`
- `src/types/_callback_options.rs`
- `src/types/_callback_succeeded_details.rs`
- `src/types/_callback_timed_out_details.rs`
- `src/types/_capacity_provider_logging_config.rs`
- `src/types/_chained_invoke_details.rs`
- `src/types/_chained_invoke_failed_details.rs`
- `src/types/_chained_invoke_started_details.rs`
- `src/types/_chained_invoke_stopped_details.rs`
- `src/types/_chained_invoke_succeeded_details.rs`
- `src/types/_chained_invoke_timed_out_details.rs`
- `src/types/_checkpoint_updated_execution_state.rs`
- `src/types/_context_details.rs`
- `src/types/_context_failed_details.rs`
- `src/types/_context_succeeded_details.rs`
- `src/types/_environment_response.rs`
- `src/types/_event.rs`
- `src/types/_event_error.rs`
- `src/types/_execution_failed_details.rs`
- `src/types/_execution_started_details.rs`
- `src/types/_execution_stopped_details.rs`
- `src/types/_execution_succeeded_details.rs`
- `src/types/_execution_timed_out_details.rs`
- `src/types/_function_code.rs`
- `src/types/_function_code_location.rs`
- `src/types/_function_configuration.rs`
- `src/types/_image_config_response.rs`
- `src/types/_invocation_completed_details.rs`
- `src/types/_invoke_response_stream_update.rs`
- `src/types/_lambda_managed_instances_capacity_provider_config.rs`
- `src/types/_layer.rs`
- `src/types/_layer_version_content_input.rs`
- `src/types/_layer_version_content_output.rs`
- `src/types/_layer_versions_list_item.rs`
- `src/types/_logging_config.rs`
- `src/types/_operation.rs`
- `src/types/_operation_update.rs`
- `src/types/_propagate_tags_mode.rs`
- `src/types/_retry_details.rs`
- `src/types/_runtime_version_config.rs`
- `src/types/_s3_object_storage_mode.rs`
- `src/types/_step_details.rs`
- `src/types/_step_failed_details.rs`
- `src/types/_step_succeeded_details.rs`
- `src/types/_wait_cancelled_details.rs`
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
