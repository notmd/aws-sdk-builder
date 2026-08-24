# AWS SDK Conformance Report: lambda

Snapshot: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`

## lambda
**Progress:** `1077/1077` files compared · `1022` matched · `54` mismatches · `0` missing · `1` extra · `94.89%` match (100.00% means fully matched)

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
@@ -259,7 +262,9 @@
             builder = _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/octet-stream");
             builder
         };
-        let body = super::super::protocol_serde::shape_invoke_async_input::ser_invoke_args_http_payload(input.invoke_args)?.into_inner();
+        let body = ::aws_smithy_types::body::SdkBody::from(super::super::protocol_serde::shape_invoke_async_input::ser_invoke_args_http_payload(
+            input.invoke_args,
+        )?);
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
             request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
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
```

### `src/protocol_serde/shape_alias_routing_configuration.rs`

```diff
--- reference/src/protocol_serde/shape_alias_routing_configuration.rs
+++ generated/src/protocol_serde/shape_alias_routing_configuration.rs
@@ -1,4 +1,24 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_alias_routing_configuration(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::super::types::AliasRoutingConfiguration,
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
-    input: &super::super::types::AliasRoutingConfiguration,
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
+    input: &super::super::types::AmazonManagedKafkaEventSourceConfig,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    if let Some(var_1) = &input.consumer_group_id {
+        object.key("ConsumerGroupId").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.schema_registry_config {
+        #[allow(unused_mut)]
+        let mut object_3 = object.key("SchemaRegistryConfig").start_object();
+        super::super::protocol_serde::shape_kafka_schema_registry_config::ser_kafka_schema_registry_config(&mut object_3, var_2)?;
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
-    input: &super::super::types::AmazonManagedKafkaEventSourceConfig,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.consumer_group_id {
-        object.key("ConsumerGroupId").string(var_1.as_str());
-    }
-    if let Some(var_2) = &input.schema_registry_config {
-        #[allow(unused_mut)]
-        let mut object_3 = object.key("SchemaRegistryConfig").start_object();
-        super::super::protocol_serde::shape_kafka_schema_registry_config::ser_kafka_schema_registry_config(&mut object_3, var_2)?;
-        object_3.finish();
-    }
-    Ok(())
-}
```

### `src/protocol_serde/shape_capacity_provider_config.rs`

```diff
--- reference/src/protocol_serde/shape_capacity_provider_config.rs
+++ generated/src/protocol_serde/shape_capacity_provider_config.rs
@@ -1,4 +1,20 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_capacity_provider_config(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::super::types::CapacityProviderConfig,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    if let Some(var_1) = &input.lambda_managed_instances_capacity_provider_config {
+        #[allow(unused_mut)]
+        let mut object_2 = object.key("LambdaManagedInstancesCapacityProviderConfig").start_object();
+        super::super::protocol_serde::shape_lambda_managed_instances_capacity_provider_config::ser_lambda_managed_instances_capacity_provider_config(
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
-                                    super::super::protocol_serde::shape_lambda_managed_instances_capacity_provider_config::de_lambda_managed_instances_capacity_provider_config(tokens, _value, depth + 1)?
-                                );
+                            builder = builder.set_lambda_managed_instances_capacity_provider_config(super::super::protocol_serde::shape_lambda_managed_instances_capacity_provider_config::de_lambda_managed_instances_capacity_provider_config(tokens, _value, depth + 1)?);
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
-    input: &super::super::types::CapacityProviderConfig,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.lambda_managed_instances_capacity_provider_config {
-        #[allow(unused_mut)]
-        let mut object_2 = object.key("LambdaManagedInstancesCapacityProviderConfig").start_object();
-        super::super::protocol_serde::shape_lambda_managed_instances_capacity_provider_config::ser_lambda_managed_instances_capacity_provider_config(
-            &mut object_2,
-            var_1,
-        )?;
-        object_2.finish();
-    }
-    Ok(())
-}
```

### `src/protocol_serde/shape_context_started_details.rs`

```diff
--- reference/src/protocol_serde/shape_context_started_details.rs
+++ generated/src/protocol_serde/shape_context_started_details.rs
@@ -3,7 +3,7 @@
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
     depth: u32,
-) -> ::std::result::Result<Option<super::super::types::ContextStartedDetails>, ::aws_smithy_json::deserialize::error::DeserializeError>
+) -> ::std::result::Result<Option<super::super::types::crate::types::ContextStartedDetails>, ::aws_smithy_json::deserialize::error::DeserializeError>
 where
     I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
 {
@@ -16,7 +16,7 @@
         Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
         Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
             #[allow(unused_mut)]
-            let mut builder = super::super::types::builders::ContextStartedDetailsBuilder::default();
+            let mut builder = super::super::types::builders::crate::types::ContextStartedDetailsBuilder::default();
             ::aws_smithy_json::deserialize::token::skip_to_end(tokens)?;
             Ok(Some(builder.build()))
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
+    input: &super::super::types::Cors,
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
-    input: &super::super::types::Cors,
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

### `src/protocol_serde/shape_dead_letter_config.rs`

```diff
--- reference/src/protocol_serde/shape_dead_letter_config.rs
+++ generated/src/protocol_serde/shape_dead_letter_config.rs
@@ -1,4 +1,14 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_dead_letter_config(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::super::types::DeadLetterConfig,
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
-    input: &super::super::types::DeadLetterConfig,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.target_arn {
-        object.key("TargetArn").string(var_1.as_str());
-    }
-    Ok(())
-}
```

### `src/protocol_serde/shape_destination_config.rs`

```diff
--- reference/src/protocol_serde/shape_destination_config.rs
+++ generated/src/protocol_serde/shape_destination_config.rs
@@ -1,4 +1,23 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_destination_config(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::super::types::DestinationConfig,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    if let Some(var_1) = &input.on_success {
+        #[allow(unused_mut)]
+        let mut object_2 = object.key("OnSuccess").start_object();
+        super::super::protocol_serde::shape_on_success::ser_on_success(&mut object_2, var_1)?;
+        object_2.finish();
+    }
+    if let Some(var_3) = &input.on_failure {
+        #[allow(unused_mut)]
+        let mut object_4 = object.key("OnFailure").start_object();
+        super::super::protocol_serde::shape_on_failure::ser_on_failure(&mut object_4, var_3)?;
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
-    input: &super::super::types::DestinationConfig,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.on_success {
-        #[allow(unused_mut)]
-        let mut object_2 = object.key("OnSuccess").start_object();
-        super::super::protocol_serde::shape_on_success::ser_on_success(&mut object_2, var_1)?;
-        object_2.finish();
-    }
-    if let Some(var_3) = &input.on_failure {
-        #[allow(unused_mut)]
-        let mut object_4 = object.key("OnFailure").start_object();
-        super::super::protocol_serde::shape_on_failure::ser_on_failure(&mut object_4, var_3)?;
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
+    input: &super::super::types::DocumentDbEventSourceConfig,
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
-    input: &super::super::types::DocumentDbEventSourceConfig,
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
+    input: &super::super::types::DurableConfig,
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
-    input: &super::super::types::DurableConfig,
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

### `src/protocol_serde/shape_ephemeral_storage.rs`

```diff
--- reference/src/protocol_serde/shape_ephemeral_storage.rs
+++ generated/src/protocol_serde/shape_ephemeral_storage.rs
@@ -1,4 +1,17 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_ephemeral_storage(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::super::types::EphemeralStorage,
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
-    input: &super::super::types::EphemeralStorage,
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
-                                    .map(|s| s.to_unescaped().map(|u| super::super::types::EventSourcePosition::from(u.as_ref())))
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
-                            builder = builder.set_filter_criteria(super::super::protocol_serde::shape_filter_criteria::de_filter_criteria(
-                                tokens,
-                                _value,
-                                depth + 1,
-                            )?);
-                        }
-                        "FilterCriteriaError" => {
-                            builder = builder.set_filter_criteria_error(
-                                super::super::protocol_serde::shape_filter_criteria_error::de_filter_criteria_error(tokens, _value, depth + 1)?,
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
-                                super::super::protocol_serde::shape_event_source_mapping_metrics_config::de_event_source_mapping_metrics_config(
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
+                                        .map(|s| s.to_unescaped().map(|u| super::super::types::EventSourcePosition::from(u.as_ref())))
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
+                                builder = builder.set_filter_criteria(super::super::protocol_serde::shape_filter_criteria::de_filter_criteria(
                                     tokens,
                                     _value,
                                     depth + 1,
-                                )?,
-                            );
-                        }
-                        "LoggingConfig" => {
-                            builder = builder.set_logging_config(
-                                super::super::protocol_serde::shape_event_source_mapping_logging_config::de_event_source_mapping_logging_config(
+                                )?);
+                            }
+                            "FilterCriteriaError" => {
+                                builder = builder.set_filter_criteria_error(
+                                    super::super::protocol_serde::shape_filter_criteria_error::de_filter_criteria_error(tokens, _value, depth + 1)?,
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
+                                    super::super::protocol_serde::shape_event_source_mapping_metrics_config::de_event_source_mapping_metrics_config(
+                                        tokens,
+                                        _value,
+                                        depth + 1,
+                                    )?,
+                                );
+                            }
+                            "LoggingConfig" => {
+                                builder = builder.set_logging_config(
+                                    super::super::protocol_serde::shape_event_source_mapping_logging_config::de_event_source_mapping_logging_config(
+                                        tokens,
+                                        _value,
+                                        depth + 1,
+                                    )?,
+                                );
+                            }
+                            "ScalingConfig" => {
+                                builder = builder.set_scaling_config(super::super::protocol_serde::shape_scaling_config::de_scaling_config(
                                     tokens,
                                     _value,
                                     depth + 1,
-                                )?,
-                            );
-                        }
-                        "ScalingConfig" => {
-                            builder = builder.set_scaling_config(super::super::protocol_serde::shape_scaling_config::de_scaling_config(
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
-                            builder = builder.set_destination_config(super::super::protocol_serde::shape_destination_config::de_destination_config(
-                                tokens,
-                                _value,
-                                depth + 1,
-                            )?);
-                        }
-                        "Topics" => {
-                            builder = builder.set_topics(super::super::protocol_serde::shape_topics::de_topics(tokens, _value, depth + 1)?);
-                        }
-                        "Queues" => {
-                            builder = builder.set_queues(super::super::protocol_serde::shape_queues::de_queues(tokens, _value, depth + 1)?);
-                        }
-                        "SourceAccessConfigurations" => {
-                            builder = builder.set_source_access_configurations(
-                                super::super::protocol_serde::shape_source_access_configurations::de_source_access_configurations(
-                                    tokens,
-                                    _value,
-                                    depth + 1,
-                                )?,
-                            );
-                        }
-                        "SelfManagedEventSource" => {
-                            builder = builder.set_self_managed_event_source(
-                                super::super::protocol_serde::shape_self_managed_event_source::de_self_managed_event_source(tokens, _value, depth + 1)?,
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
-                                super::super::protocol_serde::shape_function_response_type_list::de_function_response_type_list(tokens, _value, depth + 1)?,
-                            );
-                        }
-                        "AmazonManagedKafkaEventSourceConfig" => {
-                            builder = builder.set_amazon_managed_kafka_event_source_config(
-                                super::super::protocol_serde::shape_amazon_managed_kafka_event_source_config::de_amazon_managed_kafka_event_source_config(
-                                    tokens,
-                                    _value,
-                                    depth + 1,
-                                )?,
-                            );
-                        }
-                        "SelfManagedKafkaEventSourceConfig" => {
-                            builder = builder.set_self_managed_kafka_event_source_config(
-                                super::super::protocol_serde::shape_self_managed_kafka_event_source_config::de_self_managed_kafka_event_source_config(
-                                    tokens,
-                                    _value,
-                                    depth + 1,
-                                )?,
-                            );
-                        }
-                        "DocumentDBEventSourceConfig" => {
-                            builder = builder.set_document_db_event_source_config(
-                                super::super::protocol_serde::shape_document_db_event_source_config::de_document_db_event_source_config(
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
+                                builder = builder.set_destination_config(super::super::protocol_serde::shape_destination_config::de_destination_config(
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
-                                super::super::protocol_serde::shape_provisioned_poller_config::de_provisioned_poller_config(tokens, _value, depth + 1)?,
-                            );
+                                )?);
+                            }
+                            "Topics" => {
+                                builder = builder.set_topics(super::super::protocol_serde::shape_topics::de_topics(tokens, _value, depth + 1)?);
+                            }
+                            "Queues" => {
+                                builder = builder.set_queues(super::super::protocol_serde::shape_queues::de_queues(tokens, _value, depth + 1)?);
+                            }
+                            "SourceAccessConfigurations" => {
+                                builder = builder.set_source_access_configurations(
+                                    super::super::protocol_serde::shape_source_access_configurations::de_source_access_configurations(
+                                        tokens,
+                                        _value,
+                                        depth + 1,
+                                    )?,
+                                );
+                            }
+                            "SelfManagedEventSource" => {
+                                builder = builder.set_self_managed_event_source(
+                                    super::super::protocol_serde::shape_self_managed_event_source::de_self_managed_event_source(tokens, _value, depth + 1)?,
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
+                                    super::super::protocol_serde::shape_function_response_type_list::de_function_response_type_list(
+                                        tokens,
+                                        _value,
+                                        depth + 1,
+                                    )?,
+                                );
+                            }
+                            "AmazonManagedKafkaEventSourceConfig" => {
+                                builder = builder.set_amazon_managed_kafka_event_source_config(super::super::protocol_serde::shape_amazon_managed_kafka_event_source_config::de_amazon_managed_kafka_event_source_config(tokens, _value, depth + 1)?);
+                            }
+                            "SelfManagedKafkaEventSourceConfig" => {
+                                builder = builder.set_self_managed_kafka_event_source_config(
+                                    super::super::protocol_serde::shape_self_managed_kafka_event_source_config::de_self_managed_kafka_event_source_config(
+                                        tokens,
+                                        _value,
+                                        depth + 1,
+                                    )?,
+                                );
+                            }
+                            "DocumentDBEventSourceConfig" => {
+                                builder = builder.set_document_db_event_source_config(
+                                    super::super::protocol_serde::shape_document_db_event_source_config::de_document_db_event_source_config(
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
+                                    super::super::protocol_serde::shape_provisioned_poller_config::de_provisioned_poller_config(tokens, _value, depth + 1)?,
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
+    input: &super::super::types::EventSourceMappingLoggingConfig,
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
-    input: &super::super::types::EventSourceMappingLoggingConfig,
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
+    input: &super::super::types::EventSourceMappingMetricsConfig,
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
-    input: &super::super::types::EventSourceMappingMetricsConfig,
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
+    input: &super::super::types::FilterCriteria,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    if let Some(var_1) = &input.filters {
+        let mut array_2 = object.key("Filters").start_array();
+        for item_3 in var_1 {
+            {
+                #[allow(unused_mut)]
+                let mut object_4 = array_2.value().start_object();
+                super::super::protocol_serde::shape_filter::ser_filter(&mut object_4, item_3)?;
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
-    input: &super::super::types::FilterCriteria,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.filters {
-        let mut array_2 = object.key("Filters").start_array();
-        for item_3 in var_1 {
-            {
-                #[allow(unused_mut)]
-                let mut object_4 = array_2.value().start_object();
-                super::super::protocol_serde::shape_filter::ser_filter(&mut object_4, item_3)?;
-                object_4.finish();
-            }
-        }
-        array_2.finish();
-    }
-    Ok(())
-}
```

### `src/protocol_serde/shape_invoke.rs`

```diff
--- reference/src/protocol_serde/shape_invoke.rs
+++ generated/src/protocol_serde/shape_invoke.rs
@@ -743,65 +743,3 @@
         output.build()
     })
 }
-
-pub fn ser_invoke_headers(
-    input: &super::super::operation::invoke::InvokeInput,
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
-    }
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
-    }
-    Ok(builder)
-}
```

### `src/protocol_serde/shape_invoke_async_input.rs`

```diff
--- reference/src/protocol_serde/shape_invoke_async_input.rs
+++ generated/src/protocol_serde/shape_invoke_async_input.rs
@@ -1,6 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub fn ser_invoke_args_http_payload(
-    payload: ::aws_smithy_types::byte_stream::ByteStream,
-) -> ::std::result::Result<::aws_smithy_types::byte_stream::ByteStream, ::aws_smithy_types::error::operation::BuildError> {
-    Ok(payload)
+    payload: ::std::option::Option<::aws_smithy_types::Blob>,
+) -> ::std::result::Result<::bytes::Bytes, ::aws_smithy_types::error::operation::BuildError> {
+    let payload = match payload {
+        Some(t) => t,
+        None => return Ok(::bytes::Bytes::new()),
+    };
+    Ok(::aws_smithy_types::Blob::from(payload).into_bytes())
 }
```

### `src/protocol_serde/shape_invoke_output.rs`

```diff
--- reference/src/protocol_serde/shape_invoke_output.rs
+++ generated/src/protocol_serde/shape_invoke_output.rs
@@ -1,34 +1,33 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-pub(crate) fn de_durable_execution_arn_header(
+pub(crate) fn de_payload_payload(
+    body: &[u8],
+) -> std::result::Result<::std::option::Option<::aws_smithy_types::Blob>, super::super::operation::invoke::InvokeError> {
+    (!body.is_empty()).then(|| Ok(::aws_smithy_types::Blob::new(body))).transpose()
+}
+pub(crate) fn de_function_error_header(
     header_map: &::aws_smithy_runtime_api::http::Headers,
 ) -> ::std::result::Result<::std::option::Option<::std::string::String>, ::aws_smithy_http::header::ParseError> {
-    let headers = header_map.get_all("X-Amz-Durable-Execution-Arn");
+    let headers = header_map.get_all("X-Amz-Function-Error");
     ::aws_smithy_http::header::one_or_none(headers)
 }

-pub(crate) fn de_executed_version_header(
+pub(crate) fn de_log_result_header(
     header_map: &::aws_smithy_runtime_api::http::Headers,
 ) -> ::std::result::Result<::std::option::Option<::std::string::String>, ::aws_smithy_http::header::ParseError> {
-    let headers = header_map.get_all("X-Amz-Executed-Version");
+    let headers = header_map.get_all("X-Amz-Log-Result");
     ::aws_smithy_http::header::one_or_none(headers)
 }

-pub(crate) fn de_function_error_header(
+pub(crate) fn de_executed_version_header(
     header_map: &::aws_smithy_runtime_api::http::Headers,
 ) -> ::std::result::Result<::std::option::Option<::std::string::String>, ::aws_smithy_http::header::ParseError> {
-    let headers = header_map.get_all("X-Amz-Function-Error");
+    let headers = header_map.get_all("X-Amz-Executed-Version");
     ::aws_smithy_http::header::one_or_none(headers)
 }

-pub(crate) fn de_log_result_header(
+pub(crate) fn de_durable_execution_arn_header(
     header_map: &::aws_smithy_runtime_api::http::Headers,
 ) -> ::std::result::Result<::std::option::Option<::std::string::String>, ::aws_smithy_http::header::ParseError> {
-    let headers = header_map.get_all("X-Amz-Log-Result");
+    let headers = header_map.get_all("X-Amz-Durable-Execution-Arn");
     ::aws_smithy_http::header::one_or_none(headers)
 }
-
-pub(crate) fn de_payload_payload(
-    body: &[u8],
-) -> std::result::Result<::std::option::Option<::aws_smithy_types::Blob>, super::super::operation::invoke::InvokeError> {
-    (!body.is_empty()).then(|| Ok(::aws_smithy_types::Blob::new(body))).transpose()
-}
```

### `src/protocol_serde/shape_invoke_with_response_stream.rs`

```diff
--- reference/src/protocol_serde/shape_invoke_with_response_stream.rs
+++ generated/src/protocol_serde/shape_invoke_with_response_stream.rs
@@ -41,7 +41,6 @@
             .map_err(super::super::operation::invoke_with_response_stream::InvokeWithResponseStreamError::unhandled)?
     })
 }
-
 #[allow(clippy::unnecessary_wraps)]
 pub fn de_invoke_with_response_stream_http_error(
     _response_status: u16,
@@ -687,54 +686,3 @@
         _ => super::super::operation::invoke_with_response_stream::InvokeWithResponseStreamError::generic(generic),
     })
 }
-
-pub fn ser_invoke_with_response_stream_headers(
-    input: &super::super::operation::invoke_with_response_stream::InvokeWithResponseStreamInput,
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
-    }
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
-    }
-    Ok(builder)
-}
```

### `src/protocol_serde/shape_kafka_schema_registry_config.rs`

```diff
--- reference/src/protocol_serde/shape_kafka_schema_registry_config.rs
+++ generated/src/protocol_serde/shape_kafka_schema_registry_config.rs
@@ -1,4 +1,41 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_kafka_schema_registry_config(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::super::types::KafkaSchemaRegistryConfig,
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
+                super::super::protocol_serde::shape_kafka_schema_registry_access_config::ser_kafka_schema_registry_access_config(&mut object_6, item_5)?;
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
+                super::super::protocol_serde::shape_kafka_schema_validation_config::ser_kafka_schema_validation_config(&mut object_10, item_9)?;
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
-                                    super::super::protocol_serde::shape_kafka_schema_registry_access_config_list::de_kafka_schema_registry_access_config_list(tokens, _value, depth + 1)?
-                                );
+                                builder = builder.set_access_configs(super::super::protocol_serde::shape_kafka_schema_registry_access_config_list::de_kafka_schema_registry_access_config_list(tokens, _value, depth + 1)?);
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
-    input: &super::super::types::KafkaSchemaRegistryConfig,
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
-                super::super::protocol_serde::shape_kafka_schema_registry_access_config::ser_kafka_schema_registry_access_config(&mut object_6, item_5)?;
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
-                super::super::protocol_serde::shape_kafka_schema_validation_config::ser_kafka_schema_validation_config(&mut object_10, item_9)?;
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
+    input: &super::super::types::LambdaManagedInstancesCapacityProviderConfig,
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
-    input: &super::super::types::LambdaManagedInstancesCapacityProviderConfig,
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

### `src/protocol_serde/shape_logging_config.rs`

```diff
--- reference/src/protocol_serde/shape_logging_config.rs
+++ generated/src/protocol_serde/shape_logging_config.rs
@@ -1,4 +1,23 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_logging_config(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::super::types::LoggingConfig,
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
-    input: &super::super::types::LoggingConfig,
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
+    input: &super::super::types::OnFailure,
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
-    input: &super::super::types::OnFailure,
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
+    input: &super::super::types::OnSuccess,
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
-    input: &super::super::types::OnSuccess,
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
+    input: &super::super::types::ProvisionedPollerConfig,
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
-    input: &super::super::types::ProvisionedPollerConfig,
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

### `src/protocol_serde/shape_scaling_config.rs`

```diff
--- reference/src/protocol_serde/shape_scaling_config.rs
+++ generated/src/protocol_serde/shape_scaling_config.rs
@@ -1,4 +1,17 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_scaling_config(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::super::types::ScalingConfig,
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
-    input: &super::super::types::ScalingConfig,
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
+    input: &super::super::types::SelfManagedEventSource,
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
-    input: &super::super::types::SelfManagedEventSource,
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
+    input: &super::super::types::SelfManagedKafkaEventSourceConfig,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    if let Some(var_1) = &input.consumer_group_id {
+        object.key("ConsumerGroupId").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.schema_registry_config {
+        #[allow(unused_mut)]
+        let mut object_3 = object.key("SchemaRegistryConfig").start_object();
+        super::super::protocol_serde::shape_kafka_schema_registry_config::ser_kafka_schema_registry_config(&mut object_3, var_2)?;
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
-    input: &super::super::types::SelfManagedKafkaEventSourceConfig,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.consumer_group_id {
-        object.key("ConsumerGroupId").string(var_1.as_str());
-    }
-    if let Some(var_2) = &input.schema_registry_config {
-        #[allow(unused_mut)]
-        let mut object_3 = object.key("SchemaRegistryConfig").start_object();
-        super::super::protocol_serde::shape_kafka_schema_registry_config::ser_kafka_schema_registry_config(&mut object_3, var_2)?;
-        object_3.finish();
-    }
-    Ok(())
-}
```

### `src/protocol_serde/shape_send_durable_execution_callback_failure_input.rs`

```diff
--- reference/src/protocol_serde/shape_send_durable_execution_callback_failure_input.rs
+++ generated/src/protocol_serde/shape_send_durable_execution_callback_failure_input.rs
@@ -11,7 +11,7 @@

 pub fn ser_error_payload(
     input: &super::super::types::ErrorObject,
-) -> std::result::Result<::std::vec::Vec<u8>, ::aws_smithy_types::error::operation::SerializationError> {
+) -> ::std::result::Result<::std::vec::Vec<u8>, ::aws_smithy_types::error::operation::SerializationError> {
     let mut out = String::new();
     let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
     super::super::protocol_serde::shape_error_object::ser_error_object(&mut object, input)?;
```

### `src/protocol_serde/shape_step_started_details.rs`

```diff
--- reference/src/protocol_serde/shape_step_started_details.rs
+++ generated/src/protocol_serde/shape_step_started_details.rs
@@ -3,7 +3,7 @@
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
     depth: u32,
-) -> ::std::result::Result<Option<super::super::types::StepStartedDetails>, ::aws_smithy_json::deserialize::error::DeserializeError>
+) -> ::std::result::Result<Option<super::super::types::crate::types::StepStartedDetails>, ::aws_smithy_json::deserialize::error::DeserializeError>
 where
     I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
 {
@@ -16,7 +16,7 @@
         Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
         Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
             #[allow(unused_mut)]
-            let mut builder = super::super::types::builders::StepStartedDetailsBuilder::default();
+            let mut builder = super::super::types::builders::crate::types::StepStartedDetailsBuilder::default();
             ::aws_smithy_json::deserialize::token::skip_to_end(tokens)?;
             Ok(Some(builder.build()))
         }
```

### `src/protocol_serde/shape_stop_durable_execution_input.rs`

```diff
--- reference/src/protocol_serde/shape_stop_durable_execution_input.rs
+++ generated/src/protocol_serde/shape_stop_durable_execution_input.rs
@@ -11,7 +11,7 @@

 pub fn ser_error_payload(
     input: &super::super::types::ErrorObject,
-) -> std::result::Result<::std::vec::Vec<u8>, ::aws_smithy_types::error::operation::SerializationError> {
+) -> ::std::result::Result<::std::vec::Vec<u8>, ::aws_smithy_types::error::operation::SerializationError> {
     let mut out = String::new();
     let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
     super::super::protocol_serde::shape_error_object::ser_error_object(&mut object, input)?;
```

### `src/protocol_serde/shape_tenancy_config.rs`

```diff
--- reference/src/protocol_serde/shape_tenancy_config.rs
+++ generated/src/protocol_serde/shape_tenancy_config.rs
@@ -1,4 +1,14 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_tenancy_config(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::super::types::TenancyConfig,
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
-    input: &super::super::types::TenancyConfig,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    {
-        object.key("TenantIsolationMode").string(input.tenant_isolation_mode.as_str());
-    }
-    Ok(())
-}
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
@@ -295,8 +279,14 @@

 pub(crate) mod shape_invalid_zip_file_exception;

+pub(crate) mod shape_invoke_async_input;
+
+pub(crate) mod shape_invoke_input;
+
 pub(crate) mod shape_invoke_output;

+pub(crate) mod shape_invoke_with_response_stream_input;
+
 pub(crate) mod shape_invoke_with_response_stream_output;

 pub(crate) mod shape_kms_access_denied_exception;
@@ -357,6 +347,14 @@

 pub(crate) mod shape_s3_files_mount_timeout_exception;

+pub fn rest_json_unset_struct_payload() -> ::std::vec::Vec<u8> {
+    b"{}"[..].into()
+}
+
+pub(crate) mod shape_send_durable_execution_callback_failure_input;
+
+pub(crate) mod shape_send_durable_execution_callback_success_input;
+
 pub(crate) mod shape_serialized_request_entity_too_large_exception;

 pub(crate) mod shape_service_exception;
@@ -371,6 +369,8 @@

 pub(crate) mod shape_snap_start_timeout_exception;

+pub(crate) mod shape_stop_durable_execution_input;
+
 pub(crate) mod shape_subnet_ip_address_limit_reached_exception;

 pub(crate) mod shape_tag_resource_input;
@@ -505,6 +505,10 @@

 pub(crate) mod shape_instance_requirements;

+pub(crate) mod shape_invoke_response_stream_update;
+
+pub(crate) mod shape_invoke_with_response_stream_complete_event;
+
 pub(crate) mod shape_layer_version_content_input;

 pub(crate) mod shape_layer_version_content_output;
@@ -609,8 +613,6 @@

 pub(crate) mod shape_image_config_error;

-pub(crate) mod shape_invoke_with_response_stream_complete_event;
-
 pub(crate) mod shape_kafka_schema_registry_config;

 pub(crate) mod shape_lambda_managed_instances_capacity_provider_config;
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
     pub system_log_level: ::std::option::Option<super::super::types::SystemLogLevel>,
-    /// <p>The name of the Amazon CloudWatch log group the capacity provider sends logs to. By default, Lambda capacity providers send logs to a default log group named <code>/aws/lambda/capacity-provider/&lt;capacity provider name&gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
+    /// <p>The name of the Amazon CloudWatch log group the capacity provider sends logs to. By default, Lambda capacity providers send logs to a default log group named <code>/aws/lambda/capacity-provider/&amp;lt;capacity provider name&amp;gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
     pub log_group: ::std::option::Option<::std::string::String>,
 }
 impl CapacityProviderLoggingConfig {
@@ -14,7 +14,7 @@
     pub fn system_log_level(&self) -> ::std::option::Option<&super::super::types::SystemLogLevel> {
         self.system_log_level.as_ref()
     }
-    /// <p>The name of the Amazon CloudWatch log group the capacity provider sends logs to. By default, Lambda capacity providers send logs to a default log group named <code>/aws/lambda/capacity-provider/&lt;capacity provider name&gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
+    /// <p>The name of the Amazon CloudWatch log group the capacity provider sends logs to. By default, Lambda capacity providers send logs to a default log group named <code>/aws/lambda/capacity-provider/&amp;lt;capacity provider name&amp;gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
     pub fn log_group(&self) -> ::std::option::Option<&str> {
         self.log_group.as_deref()
     }
@@ -48,17 +48,17 @@
     pub fn get_system_log_level(&self) -> &::std::option::Option<super::super::types::SystemLogLevel> {
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
         super::super::types::Event {
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
     pub application_log_level: ::std::option::Option<super::super::types::ApplicationLogLevel>,
     /// <p>Set this property to filter the system logs for your function that Lambda sends to CloudWatch. Lambda only sends system logs at the selected level of detail and lower, where <code>DEBUG</code> is the highest level and <code>WARN</code> is the lowest.</p>
     pub system_log_level: ::std::option::Option<super::super::types::SystemLogLevel>,
-    /// <p>The name of the Amazon CloudWatch log group the function sends logs to. By default, Lambda functions send logs to a default log group named <code>/aws/lambda/&lt;function name&gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
+    /// <p>The name of the Amazon CloudWatch log group the function sends logs to. By default, Lambda functions send logs to a default log group named <code>/aws/lambda/&amp;lt;function name&amp;gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
     pub log_group: ::std::option::Option<::std::string::String>,
 }
 impl LoggingConfig {
@@ -26,7 +26,7 @@
     pub fn system_log_level(&self) -> ::std::option::Option<&super::super::types::SystemLogLevel> {
         self.system_log_level.as_ref()
     }
-    /// <p>The name of the Amazon CloudWatch log group the function sends logs to. By default, Lambda functions send logs to a default log group named <code>/aws/lambda/&lt;function name&gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
+    /// <p>The name of the Amazon CloudWatch log group the function sends logs to. By default, Lambda functions send logs to a default log group named <code>/aws/lambda/&amp;lt;function name&amp;gt;</code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
     pub fn log_group(&self) -> ::std::option::Option<&str> {
         self.log_group.as_deref()
     }
@@ -90,17 +90,17 @@
     pub fn get_system_log_level(&self) -> &::std::option::Option<super::super::types::SystemLogLevel> {
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

 pub use super::super::types::_error_object::ErrorObjectBuilder;

+pub use super::super::types::_checkpoint_updated_execution_state::CheckpointUpdatedExecutionStateBuilder;
+
+pub use super::super::types::_alias_routing_configuration::AliasRoutingConfigurationBuilder;
+
 pub use super::super::types::_capacity_provider_vpc_config::CapacityProviderVpcConfigBuilder;

 pub use super::super::types::_capacity_provider_permissions_config::CapacityProviderPermissionsConfigBuilder;
@@ -27,12 +31,6 @@

 pub use super::super::types::_code_signing_config::CodeSigningConfigBuilder;

-pub use super::super::types::_trace_header::TraceHeaderBuilder;
-
-pub use super::super::types::_durable_config::DurableConfigBuilder;
-
-pub use super::super::types::_checkpoint_updated_execution_state::CheckpointUpdatedExecutionStateBuilder;
-
 pub use super::super::types::_filter_criteria::FilterCriteriaBuilder;

 pub use super::super::types::_event_source_mapping_metrics_config::EventSourceMappingMetricsConfigBuilder;
@@ -75,6 +73,8 @@

 pub use super::super::types::_capacity_provider_config::CapacityProviderConfigBuilder;

+pub use super::super::types::_durable_config::DurableConfigBuilder;
+
 pub use super::super::types::_vpc_config_response::VpcConfigResponseBuilder;

 pub use super::super::types::_environment_response::EnvironmentResponseBuilder;
@@ -89,6 +89,8 @@

 pub use super::super::types::_cors::CorsBuilder;

+pub use super::super::types::_trace_header::TraceHeaderBuilder;
+
 pub use super::super::types::_function_configuration::FunctionConfigurationBuilder;

 pub use super::super::types::_function_code_location::FunctionCodeLocationBuilder;
@@ -99,8 +101,6 @@

 pub use super::super::types::_function_scaling_config::FunctionScalingConfigBuilder;

-pub use super::super::types::_alias_routing_configuration::AliasRoutingConfigurationBuilder;
-
 pub use super::super::types::_layer_version_content_output::LayerVersionContentOutputBuilder;

 pub use super::super::types::_layer_version_content_input::LayerVersionContentInputBuilder;
@@ -111,17 +111,9 @@

 pub use super::super::types::_function_event_invoke_config::FunctionEventInvokeConfigBuilder;

-pub use super::super::types::_capacity_provider_logging_config::CapacityProviderLoggingConfigBuilder;
-
-pub use super::super::types::_function_versions_by_capacity_provider_list_item::FunctionVersionsByCapacityProviderListItemBuilder;
-
 pub use super::super::types::_operation_update::OperationUpdateBuilder;

-pub use super::super::types::_event::EventBuilder;
-
-pub use super::super::types::_operation::OperationBuilder;
-
-pub use super::super::types::_event_source_mapping_configuration::EventSourceMappingConfigurationBuilder;
+pub use super::super::types::_capacity_provider_logging_config::CapacityProviderLoggingConfigBuilder;

 pub use super::super::types::_source_access_configuration::SourceAccessConfigurationBuilder;

@@ -139,7 +131,9 @@

 pub use super::super::types::_runtime_version_error::RuntimeVersionErrorBuilder;

-pub use super::super::types::_provisioned_concurrency_config_list_item::ProvisionedConcurrencyConfigListItemBuilder;
+pub use super::super::types::_event::EventBuilder;
+
+pub use super::super::types::_operation::OperationBuilder;

 pub use super::super::types::_resolved_s3_object::ResolvedS3ObjectBuilder;

@@ -149,18 +143,22 @@

 pub use super::super::types::_invoke_with_response_stream_complete_event::InvokeWithResponseStreamCompleteEventBuilder;

+pub use super::super::types::_alias_configuration::AliasConfigurationBuilder;
+
 pub use super::super::types::_execution::ExecutionBuilder;

+pub use super::super::types::_event_source_mapping_configuration::EventSourceMappingConfigurationBuilder;
+
 pub use super::super::types::_function_url_config::FunctionUrlConfigBuilder;

-pub use super::super::types::_alias_configuration::AliasConfigurationBuilder;
-
-pub use super::super::types::_layers_list_item::LayersListItemBuilder;
+pub use super::super::types::_function_versions_by_capacity_provider_list_item::FunctionVersionsByCapacityProviderListItemBuilder;

 pub use super::super::types::_layer_versions_list_item::LayerVersionsListItemBuilder;

-pub use super::super::types::_target_tracking_scaling_policy::TargetTrackingScalingPolicyBuilder;
+pub use super::super::types::_layers_list_item::LayersListItemBuilder;

+pub use super::super::types::_provisioned_concurrency_config_list_item::ProvisionedConcurrencyConfigListItemBuilder;
+
 pub use super::super::types::_context_options::ContextOptionsBuilder;

 pub use super::super::types::_step_options::StepOptionsBuilder;
@@ -171,6 +169,10 @@

 pub use super::super::types::_chained_invoke_options::ChainedInvokeOptionsBuilder;

+pub use super::super::types::_target_tracking_scaling_policy::TargetTrackingScalingPolicyBuilder;
+
+pub use super::super::types::_filter::FilterBuilder;
+
 pub use super::super::types::_execution_started_details::ExecutionStartedDetailsBuilder;

 pub use super::super::types::_execution_succeeded_details::ExecutionSucceededDetailsBuilder;
@@ -231,7 +233,9 @@

 pub use super::super::types::_chained_invoke_details::ChainedInvokeDetailsBuilder;

-pub use super::super::types::_filter::FilterBuilder;
+pub use super::super::types::_kafka_schema_registry_access_config::KafkaSchemaRegistryAccessConfigBuilder;
+
+pub use super::super::types::_kafka_schema_validation_config::KafkaSchemaValidationConfigBuilder;

 pub use super::super::types::_event_input::EventInputBuilder;

@@ -240,7 +244,3 @@
 pub use super::super::types::_event_error::EventErrorBuilder;

 pub use super::super::types::_retry_details::RetryDetailsBuilder;
-
-pub use super::super::types::_kafka_schema_registry_access_config::KafkaSchemaRegistryAccessConfigBuilder;
-
-pub use super::super::types::_kafka_schema_validation_config::KafkaSchemaValidationConfigBuilder;
```

### `src/types/error/builders.rs`

```diff
--- reference/src/types/error/builders.rs
+++ generated/src/types/error/builders.rs
@@ -1,6 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub use super::super::super::types::error::_invalid_parameter_value_exception::InvalidParameterValueExceptionBuilder;

+pub use super::super::super::types::error::_policy_length_exceeded_exception::PolicyLengthExceededExceptionBuilder;
+
+pub use super::super::super::types::error::_precondition_failed_exception::PreconditionFailedExceptionBuilder;
+
 pub use super::super::super::types::error::_resource_conflict_exception::ResourceConflictExceptionBuilder;

 pub use super::super::super::types::error::_resource_not_found_exception::ResourceNotFoundExceptionBuilder;
@@ -9,14 +13,8 @@

 pub use super::super::super::types::error::_too_many_requests_exception::TooManyRequestsExceptionBuilder;

-pub use super::super::super::types::error::_precondition_failed_exception::PreconditionFailedExceptionBuilder;
-
-pub use super::super::super::types::error::_policy_length_exceeded_exception::PolicyLengthExceededExceptionBuilder;
-
 pub use super::super::super::types::error::_public_policy_exception::PublicPolicyExceptionBuilder;

-pub use super::super::super::types::error::_callback_timeout_exception::CallbackTimeoutExceptionBuilder;
-
 pub use super::super::super::types::error::_kms_access_denied_exception::KmsAccessDeniedExceptionBuilder;

 pub use super::super::super::types::error::_kms_disabled_exception::KmsDisabledExceptionBuilder;
@@ -25,10 +23,10 @@

 pub use super::super::super::types::error::_kms_not_found_exception::KmsNotFoundExceptionBuilder;

+pub use super::super::super::types::error::_alias_limit_exceeded_exception::AliasLimitExceededExceptionBuilder;
+
 pub use super::super::super::types::error::_capacity_provider_limit_exceeded_exception::CapacityProviderLimitExceededExceptionBuilder;

-pub use super::super::super::types::error::_resource_in_use_exception::ResourceInUseExceptionBuilder;
-
 pub use super::super::super::types::error::_code_signing_config_not_found_exception::CodeSigningConfigNotFoundExceptionBuilder;

 pub use super::super::super::types::error::_code_storage_exceeded_exception::CodeStorageExceededExceptionBuilder;
@@ -39,6 +37,10 @@

 pub use super::super::super::types::error::_invalid_code_signature_exception::InvalidCodeSignatureExceptionBuilder;

+pub use super::super::super::types::error::_resource_in_use_exception::ResourceInUseExceptionBuilder;
+
+pub use super::super::super::types::error::_provisioned_concurrency_config_not_found_exception::ProvisionedConcurrencyConfigNotFoundExceptionBuilder;
+
 pub use super::super::super::types::error::_code_artifact_user_deleted_exception::CodeArtifactUserDeletedExceptionBuilder;

 pub use super::super::super::types::error::_code_artifact_user_failed_exception::CodeArtifactUserFailedExceptionBuilder;
@@ -107,6 +109,4 @@

 pub use super::super::super::types::error::_unsupported_media_type_exception::UnsupportedMediaTypeExceptionBuilder;

-pub use super::super::super::types::error::_alias_limit_exceeded_exception::AliasLimitExceededExceptionBuilder;
-
-pub use super::super::super::types::error::_provisioned_concurrency_config_not_found_exception::ProvisionedConcurrencyConfigNotFoundExceptionBuilder;
+pub use super::super::super::types::error::_callback_timeout_exception::CallbackTimeoutExceptionBuilder;
```

### `src/types/error.rs`

```diff
--- reference/src/types/error.rs
+++ generated/src/types/error.rs
@@ -1,6 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub use super::super::types::error::_invalid_parameter_value_exception::InvalidParameterValueException;

+pub use super::super::types::error::_policy_length_exceeded_exception::PolicyLengthExceededException;
+
+pub use super::super::types::error::_precondition_failed_exception::PreconditionFailedException;
+
 pub use super::super::types::error::_resource_conflict_exception::ResourceConflictException;

 pub use super::super::types::error::_resource_not_found_exception::ResourceNotFoundException;
@@ -9,14 +13,8 @@

 pub use super::super::types::error::_too_many_requests_exception::TooManyRequestsException;

-pub use super::super::types::error::_precondition_failed_exception::PreconditionFailedException;
-
-pub use super::super::types::error::_policy_length_exceeded_exception::PolicyLengthExceededException;
-
 pub use super::super::types::error::_public_policy_exception::PublicPolicyException;

-pub use super::super::types::error::_callback_timeout_exception::CallbackTimeoutException;
-
 pub use super::super::types::error::_kms_access_denied_exception::KmsAccessDeniedException;

 pub use super::super::types::error::_kms_disabled_exception::KmsDisabledException;
@@ -25,9 +23,9 @@

 pub use super::super::types::error::_kms_not_found_exception::KmsNotFoundException;

-pub use super::super::types::error::_capacity_provider_limit_exceeded_exception::CapacityProviderLimitExceededException;
+pub use super::super::types::error::_alias_limit_exceeded_exception::AliasLimitExceededException;

-pub use super::super::types::error::_resource_in_use_exception::ResourceInUseException;
+pub use super::super::types::error::_capacity_provider_limit_exceeded_exception::CapacityProviderLimitExceededException;

 pub use super::super::types::error::_code_signing_config_not_found_exception::CodeSigningConfigNotFoundException;

@@ -39,6 +37,10 @@

 pub use super::super::types::error::_invalid_code_signature_exception::InvalidCodeSignatureException;

+pub use super::super::types::error::_resource_in_use_exception::ResourceInUseException;
+
+pub use super::super::types::error::_provisioned_concurrency_config_not_found_exception::ProvisionedConcurrencyConfigNotFoundException;
+
 pub use super::super::types::error::_code_artifact_user_deleted_exception::CodeArtifactUserDeletedException;

 pub use super::super::types::error::_code_artifact_user_failed_exception::CodeArtifactUserFailedException;
@@ -107,9 +109,7 @@

 pub use super::super::types::error::_unsupported_media_type_exception::UnsupportedMediaTypeException;

-pub use super::super::types::error::_alias_limit_exceeded_exception::AliasLimitExceededException;
-
-pub use super::super::types::error::_provisioned_concurrency_config_not_found_exception::ProvisionedConcurrencyConfigNotFoundException;
+pub use super::super::types::error::_callback_timeout_exception::CallbackTimeoutException;

 /// Error type for the `InvokeWithResponseStreamResponseEventError` operation.
 #[non_exhaustive]
@@ -203,7 +203,6 @@
         self.meta().request_id()
     }
 }
-
 mod _alias_limit_exceeded_exception;

 mod _callback_timeout_exception;
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

### Unexpected generated files

- `src/protocol_serde/shape_invoke_response_stream_update.rs`
