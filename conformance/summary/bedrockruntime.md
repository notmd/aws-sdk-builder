# AWS SDK Conformance Report: bedrockruntime

Snapshot: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`

## bedrockruntime
**Progress:** `536/536` files compared · `417` matched · `119` mismatches · `0` missing · `0` extra · `77.80%` match (100.00% means fully matched)

### `src/client/invoke_model_with_bidirectional_stream.rs`

```diff
--- reference/src/client/invoke_model_with_bidirectional_stream.rs
+++ generated/src/client/invoke_model_with_bidirectional_stream.rs
@@ -4,7 +4,7 @@
     ///
     /// - The fluent builder is configurable:
     ///   - [`model_id(impl Into<String>)`](crate::operation::invoke_model_with_bidirectional_stream::builders::InvokeModelWithBidirectionalStreamFluentBuilder::model_id) / [`set_model_id(Option<String>)`](crate::operation::invoke_model_with_bidirectional_stream::builders::InvokeModelWithBidirectionalStreamFluentBuilder::set_model_id):<br>required: **true**<br><p>The model ID or ARN of the model ID to use. Currently, only <code>amazon.nova-sonic-v1:0</code> is supported.</p><br>
-    ///   - [`body(EventStreamSender<InvokeModelWithBidirectionalStreamInput, InvokeModelWithBidirectionalStreamInputError>)`](crate::operation::invoke_model_with_bidirectional_stream::builders::InvokeModelWithBidirectionalStreamFluentBuilder::body) / [`set_body(EventStreamSender<InvokeModelWithBidirectionalStreamInput, InvokeModelWithBidirectionalStreamInputError>)`](crate::operation::invoke_model_with_bidirectional_stream::builders::InvokeModelWithBidirectionalStreamFluentBuilder::set_body):<br>required: **true**<br><p>The prompt and inference parameters in the format specified in the <code>BidirectionalInputPayloadPart</code> in the header. You must provide the body in JSON format. To see the format and content of the request and response bodies for different models, refer to <a href="https://docs.aws.amazon.com/bedrock/latest/userguide/model-parameters.html">Inference parameters</a>. For more information, see <a href="https://docs.aws.amazon.com/bedrock/latest/userguide/api-methods-run.html">Run inference</a> in the Bedrock User Guide.</p><br>
+    ///   - [`body(EventReceiver<InvokeModelWithBidirectionalStreamInput, InvokeModelWithBidirectionalStreamInputError>)`](crate::operation::invoke_model_with_bidirectional_stream::builders::InvokeModelWithBidirectionalStreamFluentBuilder::body) / [`set_body(EventReceiver<InvokeModelWithBidirectionalStreamInput, InvokeModelWithBidirectionalStreamInputError>)`](crate::operation::invoke_model_with_bidirectional_stream::builders::InvokeModelWithBidirectionalStreamFluentBuilder::set_body):<br>required: **true**<br><p>The prompt and inference parameters in the format specified in the <code>BidirectionalInputPayloadPart</code> in the header. You must provide the body in JSON format. To see the format and content of the request and response bodies for different models, refer to <a href="https://docs.aws.amazon.com/bedrock/latest/userguide/model-parameters.html">Inference parameters</a>. For more information, see <a href="https://docs.aws.amazon.com/bedrock/latest/userguide/api-methods-run.html">Run inference</a> in the Bedrock User Guide.</p><br>
     /// - On success, responds with [`InvokeModelWithBidirectionalStreamOutput`](crate::operation::invoke_model_with_bidirectional_stream::InvokeModelWithBidirectionalStreamOutput) with field(s):
     ///   - [`body(EventReceiver<InvokeModelWithBidirectionalStreamOutput, InvokeModelWithBidirectionalStreamOutputError>)`](crate::operation::invoke_model_with_bidirectional_stream::InvokeModelWithBidirectionalStreamOutput::body): <p>Streaming response from the model in the format specified by the <code>BidirectionalOutputPayloadPart</code> header.</p>
     /// - On failure, responds with [`SdkError<InvokeModelWithBidirectionalStreamError>`](crate::operation::invoke_model_with_bidirectional_stream::InvokeModelWithBidirectionalStreamError)
```

### `src/lib.rs`

```diff
--- reference/src/lib.rs
+++ generated/src/lib.rs
@@ -193,8 +193,6 @@

 mod event_receiver;

-mod event_stream_serde;
-
 mod idempotency_token;

 mod observability_feature;
@@ -211,6 +209,8 @@

 mod serde_util;

+mod event_stream_serde;
+
 mod json_errors;

 #[doc(inline)]
```

### `src/operation/converse_stream.rs`

```diff
--- reference/src/operation/converse_stream.rs
+++ generated/src/operation/converse_stream.rs
@@ -199,6 +199,7 @@
     ) -> ::std::option::Option<::aws_smithy_runtime_api::client::interceptors::context::OutputOrError> {
         #[allow(unused_mut)]
         let mut force_error = false;
+        ::tracing::debug!(extended_request_id = ?super::super::s3_request_id::RequestIdExt::extended_request_id(response));
         ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));

         // If this is an error, defer to the non-streaming parser
```

### `src/operation/count_tokens/_count_tokens_output.rs`

```diff
--- reference/src/operation/count_tokens/_count_tokens_output.rs
+++ generated/src/operation/count_tokens/_count_tokens_output.rs
@@ -9,8 +9,8 @@
 }
 impl CountTokensOutput {
     /// <p>The number of tokens in the provided input according to the specified model's tokenization rules. This count represents the number of input tokens that would be processed if the same input were sent to the model in an inference request. Use this value to estimate costs and ensure your inputs stay within model token limits.</p>
-    pub fn input_tokens(&self) -> i32 {
-        self.input_tokens
+    pub fn input_tokens(&self) -> &i32 {
+        &self.input_tokens
     }
 }
 impl ::aws_types::request_id::RequestId for CountTokensOutput {
```

### `src/operation/invoke_model_with_bidirectional_stream/_invoke_model_with_bidirectional_stream_input.rs`

```diff
--- reference/src/operation/invoke_model_with_bidirectional_stream/_invoke_model_with_bidirectional_stream_input.rs
+++ generated/src/operation/invoke_model_with_bidirectional_stream/_invoke_model_with_bidirectional_stream_input.rs
@@ -6,10 +6,7 @@
     /// <p>The model ID or ARN of the model ID to use. Currently, only <code>amazon.nova-sonic-v1:0</code> is supported.</p>
     pub model_id: ::std::option::Option<::std::string::String>,
     /// <p>The prompt and inference parameters in the format specified in the <code>BidirectionalInputPayloadPart</code> in the header. You must provide the body in JSON format. To see the format and content of the request and response bodies for different models, refer to <a href="https://docs.aws.amazon.com/bedrock/latest/userguide/model-parameters.html">Inference parameters</a>. For more information, see <a href="https://docs.aws.amazon.com/bedrock/latest/userguide/api-methods-run.html">Run inference</a> in the Bedrock User Guide.</p>
-    pub body: ::aws_smithy_http::event_stream::EventStreamSender<
-        super::super::super::types::InvokeModelWithBidirectionalStreamInput,
-        super::super::super::types::error::InvokeModelWithBidirectionalStreamInputError,
-    >,
+    pub body: ::aws_smithy_types::byte_stream::ByteStream,
 }
 impl InvokeModelWithBidirectionalStreamInput {
     /// <p>The model ID or ARN of the model ID to use. Currently, only <code>amazon.nova-sonic-v1:0</code> is supported.</p>
@@ -17,12 +14,7 @@
         self.model_id.as_deref()
     }
     /// <p>The prompt and inference parameters in the format specified in the <code>BidirectionalInputPayloadPart</code> in the header. You must provide the body in JSON format. To see the format and content of the request and response bodies for different models, refer to <a href="https://docs.aws.amazon.com/bedrock/latest/userguide/model-parameters.html">Inference parameters</a>. For more information, see <a href="https://docs.aws.amazon.com/bedrock/latest/userguide/api-methods-run.html">Run inference</a> in the Bedrock User Guide.</p>
-    pub fn body(
-        &self,
-    ) -> &::aws_smithy_http::event_stream::EventStreamSender<
-        super::super::super::types::InvokeModelWithBidirectionalStreamInput,
-        super::super::super::types::error::InvokeModelWithBidirectionalStreamInputError,
-    > {
+    pub fn body(&self) -> &::aws_smithy_types::byte_stream::ByteStream {
         &self.body
     }
 }
@@ -38,12 +30,7 @@
 #[non_exhaustive]
 pub struct InvokeModelWithBidirectionalStreamInputBuilder {
     pub(crate) model_id: ::std::option::Option<::std::string::String>,
-    pub(crate) body: ::std::option::Option<
-        ::aws_smithy_http::event_stream::EventStreamSender<
-            super::super::super::types::InvokeModelWithBidirectionalStreamInput,
-            super::super::super::types::error::InvokeModelWithBidirectionalStreamInputError,
-        >,
-    >,
+    pub(crate) body: ::std::option::Option<::aws_smithy_types::byte_stream::ByteStream>,
 }
 impl InvokeModelWithBidirectionalStreamInputBuilder {
     /// <p>The model ID or ARN of the model ID to use. Currently, only <code>amazon.nova-sonic-v1:0</code> is supported.</p>
@@ -63,43 +50,20 @@
     }
     /// <p>The prompt and inference parameters in the format specified in the <code>BidirectionalInputPayloadPart</code> in the header. You must provide the body in JSON format. To see the format and content of the request and response bodies for different models, refer to <a href="https://docs.aws.amazon.com/bedrock/latest/userguide/model-parameters.html">Inference parameters</a>. For more information, see <a href="https://docs.aws.amazon.com/bedrock/latest/userguide/api-methods-run.html">Run inference</a> in the Bedrock User Guide.</p>
     /// This field is required.
-    pub fn body(
-        mut self,
-        input: ::aws_smithy_http::event_stream::EventStreamSender<
-            super::super::super::types::InvokeModelWithBidirectionalStreamInput,
-            super::super::super::types::error::InvokeModelWithBidirectionalStreamInputError,
-        >,
-    ) -> Self {
+    pub fn body(mut self, input: ::aws_smithy_types::byte_stream::ByteStream) -> Self {
         self.body = ::std::option::Option::Some(input);
         self
     }
     /// <p>The prompt and inference parameters in the format specified in the <code>BidirectionalInputPayloadPart</code> in the header. You must provide the body in JSON format. To see the format and content of the request and response bodies for different models, refer to <a href="https://docs.aws.amazon.com/bedrock/latest/userguide/model-parameters.html">Inference parameters</a>. For more information, see <a href="https://docs.aws.amazon.com/bedrock/latest/userguide/api-methods-run.html">Run inference</a> in the Bedrock User Guide.</p>
-    pub fn set_body(
-        mut self,
-        input: ::std::option::Option<
-            ::aws_smithy_http::event_stream::EventStreamSender<
-                super::super::super::types::InvokeModelWithBidirectionalStreamInput,
-                super::super::super::types::error::InvokeModelWithBidirectionalStreamInputError,
-            >,
-        >,
-    ) -> Self {
+    pub fn set_body(mut self, input: ::std::option::Option<::aws_smithy_types::byte_stream::ByteStream>) -> Self {
         self.body = input;
         self
     }
     /// <p>The prompt and inference parameters in the format specified in the <code>BidirectionalInputPayloadPart</code> in the header. You must provide the body in JSON format. To see the format and content of the request and response bodies for different models, refer to <a href="https://docs.aws.amazon.com/bedrock/latest/userguide/model-parameters.html">Inference parameters</a>. For more information, see <a href="https://docs.aws.amazon.com/bedrock/latest/userguide/api-methods-run.html">Run inference</a> in the Bedrock User Guide.</p>
-    pub fn get_body(
-        &self,
-    ) -> &::std::option::Option<
-        ::aws_smithy_http::event_stream::EventStreamSender<
-            super::super::super::types::InvokeModelWithBidirectionalStreamInput,
-            super::super::super::types::error::InvokeModelWithBidirectionalStreamInputError,
-        >,
-    > {
+    pub fn get_body(&self) -> &::std::option::Option<::aws_smithy_types::byte_stream::ByteStream> {
         &self.body
     }
     /// Consumes the builder and constructs a [`InvokeModelWithBidirectionalStreamInput`](crate::operation::invoke_model_with_bidirectional_stream::InvokeModelWithBidirectionalStreamInput).
-    /// This method will fail if any of the following fields are not set:
-    /// - [`body`](crate::operation::invoke_model_with_bidirectional_stream::builders::InvokeModelWithBidirectionalStreamInputBuilder::body)
     pub fn build(
         self,
     ) -> ::std::result::Result<
@@ -109,12 +73,7 @@
         ::std::result::Result::Ok(
             super::super::super::operation::invoke_model_with_bidirectional_stream::InvokeModelWithBidirectionalStreamInput {
                 model_id: self.model_id,
-                body: self.body.ok_or_else(|| {
-                    ::aws_smithy_types::error::operation::BuildError::missing_field(
-                        "body",
-                        "body was not specified but it is required when building InvokeModelWithBidirectionalStreamInput",
-                    )
-                })?,
+                body: self.body.unwrap_or_default(),
             },
         )
     }
```

### `src/operation/invoke_model_with_bidirectional_stream/builders.rs`

```diff
--- reference/src/operation/invoke_model_with_bidirectional_stream/builders.rs
+++ generated/src/operation/invoke_model_with_bidirectional_stream/builders.rs
@@ -124,38 +124,17 @@
         self.inner.get_model_id()
     }
     /// <p>The prompt and inference parameters in the format specified in the <code>BidirectionalInputPayloadPart</code> in the header. You must provide the body in JSON format. To see the format and content of the request and response bodies for different models, refer to <a href="https://docs.aws.amazon.com/bedrock/latest/userguide/model-parameters.html">Inference parameters</a>. For more information, see <a href="https://docs.aws.amazon.com/bedrock/latest/userguide/api-methods-run.html">Run inference</a> in the Bedrock User Guide.</p>
-    pub fn body(
-        mut self,
-        input: ::aws_smithy_http::event_stream::EventStreamSender<
-            super::super::super::types::InvokeModelWithBidirectionalStreamInput,
-            super::super::super::types::error::InvokeModelWithBidirectionalStreamInputError,
-        >,
-    ) -> Self {
+    pub fn body(mut self, input: ::aws_smithy_types::byte_stream::ByteStream) -> Self {
         self.inner = self.inner.body(input);
         self
     }
     /// <p>The prompt and inference parameters in the format specified in the <code>BidirectionalInputPayloadPart</code> in the header. You must provide the body in JSON format. To see the format and content of the request and response bodies for different models, refer to <a href="https://docs.aws.amazon.com/bedrock/latest/userguide/model-parameters.html">Inference parameters</a>. For more information, see <a href="https://docs.aws.amazon.com/bedrock/latest/userguide/api-methods-run.html">Run inference</a> in the Bedrock User Guide.</p>
-    pub fn set_body(
-        mut self,
-        input: ::std::option::Option<
-            ::aws_smithy_http::event_stream::EventStreamSender<
-                super::super::super::types::InvokeModelWithBidirectionalStreamInput,
-                super::super::super::types::error::InvokeModelWithBidirectionalStreamInputError,
-            >,
-        >,
-    ) -> Self {
+    pub fn set_body(mut self, input: ::std::option::Option<::aws_smithy_types::byte_stream::ByteStream>) -> Self {
         self.inner = self.inner.set_body(input);
         self
     }
     /// <p>The prompt and inference parameters in the format specified in the <code>BidirectionalInputPayloadPart</code> in the header. You must provide the body in JSON format. To see the format and content of the request and response bodies for different models, refer to <a href="https://docs.aws.amazon.com/bedrock/latest/userguide/model-parameters.html">Inference parameters</a>. For more information, see <a href="https://docs.aws.amazon.com/bedrock/latest/userguide/api-methods-run.html">Run inference</a> in the Bedrock User Guide.</p>
-    pub fn get_body(
-        &self,
-    ) -> &::std::option::Option<
-        ::aws_smithy_http::event_stream::EventStreamSender<
-            super::super::super::types::InvokeModelWithBidirectionalStreamInput,
-            super::super::super::types::error::InvokeModelWithBidirectionalStreamInputError,
-        >,
-    > {
+    pub fn get_body(&self) -> &::std::option::Option<::aws_smithy_types::byte_stream::ByteStream> {
         self.inner.get_body()
     }
 }
```

### `src/operation/invoke_model_with_bidirectional_stream.rs`

```diff
--- reference/src/operation/invoke_model_with_bidirectional_stream.rs
+++ generated/src/operation/invoke_model_with_bidirectional_stream.rs
@@ -117,7 +117,7 @@
         signing_options.double_uri_encode = true;
         signing_options.content_sha256_header = false;
         signing_options.normalize_uri_path = true;
-        signing_options.payload_override = Some(::aws_sigv4::http_request::SignableBody::Bytes(&[]));
+        signing_options.payload_override = None;

         cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
             signing_options,
@@ -205,6 +205,7 @@
     ) -> ::std::option::Option<::aws_smithy_runtime_api::client::interceptors::context::OutputOrError> {
         #[allow(unused_mut)]
         let mut force_error = false;
+        ::tracing::debug!(extended_request_id = ?super::super::s3_request_id::RequestIdExt::extended_request_id(response));
         ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));

         // If this is an error, defer to the non-streaming parser
```

### `src/operation/invoke_model_with_response_stream.rs`

```diff
--- reference/src/operation/invoke_model_with_response_stream.rs
+++ generated/src/operation/invoke_model_with_response_stream.rs
@@ -225,6 +225,7 @@
     ) -> ::std::option::Option<::aws_smithy_runtime_api::client::interceptors::context::OutputOrError> {
         #[allow(unused_mut)]
         let mut force_error = false;
+        ::tracing::debug!(extended_request_id = ?super::super::s3_request_id::RequestIdExt::extended_request_id(response));
         ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));

         // If this is an error, defer to the non-streaming parser
```

### `src/primitives.rs`

```diff
--- reference/src/primitives.rs
+++ generated/src/primitives.rs
@@ -1,4 +1,12 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub use ::aws_smithy_types::body::SdkBody;
+pub use ::aws_smithy_types::byte_stream::error::Error as ByteStreamError;
+pub use ::aws_smithy_types::byte_stream::AggregatedBytes;
+pub use ::aws_smithy_types::byte_stream::ByteStream;
+#[cfg(feature = "rt-tokio")]
+pub use ::aws_smithy_types::byte_stream::FsBuilder;
+#[cfg(feature = "rt-tokio")]
+pub use ::aws_smithy_types::byte_stream::Length;
 pub use ::aws_smithy_types::date_time::Format as DateTimeFormat;
 pub use ::aws_smithy_types::Blob;
 pub use ::aws_smithy_types::DateTime;
```

### `src/protocol_serde/shape_async_invoke_output_data_config.rs`

```diff
--- reference/src/protocol_serde/shape_async_invoke_output_data_config.rs
+++ generated/src/protocol_serde/shape_async_invoke_output_data_config.rs
@@ -1,4 +1,24 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_async_invoke_output_data_config(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::super::types::AsyncInvokeOutputDataConfig,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    match input {
+        super::super::types::AsyncInvokeOutputDataConfig::S3OutputDataConfig(inner) => {
+            #[allow(unused_mut)]
+            let mut object_1 = object.key("s3OutputDataConfig").start_object();
+            super::super::protocol_serde::shape_async_invoke_s3_output_data_config::ser_async_invoke_s3_output_data_config(&mut object_1, inner)?;
+            object_1.finish();
+        }
+        super::super::types::AsyncInvokeOutputDataConfig::Unknown => {
+            return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant(
+                "AsyncInvokeOutputDataConfig",
+            ))
+        }
+    }
+    Ok(())
+}
+
 pub(crate) fn de_async_invoke_output_data_config<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
@@ -19,9 +39,7 @@
             match tokens.next().transpose()? {
                 Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                 Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
-                    if let ::std::option::Option::Some(::std::result::Result::Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) =
-                        tokens.peek()
-                    {
+                    if let Some(Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) = tokens.peek() {
                         let _ = tokens.next().expect("peek returned a token")?;
                         continue;
                     }
@@ -72,23 +90,3 @@
     }
     Ok(variant)
 }
-
-pub fn ser_async_invoke_output_data_config(
-    object_5: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
-    input: &super::super::types::AsyncInvokeOutputDataConfig,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    match input {
-        super::super::types::AsyncInvokeOutputDataConfig::S3OutputDataConfig(inner) => {
-            #[allow(unused_mut)]
-            let mut object_1 = object_5.key("s3OutputDataConfig").start_object();
-            super::super::protocol_serde::shape_async_invoke_s3_output_data_config::ser_async_invoke_s3_output_data_config(&mut object_1, inner)?;
-            object_1.finish();
-        }
-        super::super::types::AsyncInvokeOutputDataConfig::Unknown => {
-            return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant(
-                "AsyncInvokeOutputDataConfig",
-            ))
-        }
-    }
-    Ok(())
-}
```

### `src/protocol_serde/shape_async_invoke_s3_output_data_config.rs`

```diff
--- reference/src/protocol_serde/shape_async_invoke_s3_output_data_config.rs
+++ generated/src/protocol_serde/shape_async_invoke_s3_output_data_config.rs
@@ -1,4 +1,20 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_async_invoke_s3_output_data_config(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::super::types::AsyncInvokeS3OutputDataConfig,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    {
+        object.key("s3Uri").string(input.s3_uri.as_str());
+    }
+    if let Some(var_1) = &input.kms_key_id {
+        object.key("kmsKeyId").string(var_1.as_str());
+    }
+    if let Some(var_2) = &input.bucket_owner {
+        object.key("bucketOwner").string(var_2.as_str());
+    }
+    Ok(())
+}
+
 pub(crate) fn de_async_invoke_s3_output_data_config<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
@@ -62,19 +78,3 @@
         )),
     }
 }
-
-pub fn ser_async_invoke_s3_output_data_config(
-    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
-    input: &super::super::types::AsyncInvokeS3OutputDataConfig,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    {
-        object.key("s3Uri").string(input.s3_uri.as_str());
-    }
-    if let Some(var_1) = &input.kms_key_id {
-        object.key("kmsKeyId").string(var_1.as_str());
-    }
-    if let Some(var_2) = &input.bucket_owner {
-        object.key("bucketOwner").string(var_2.as_str());
-    }
-    Ok(())
-}
```

### `src/protocol_serde/shape_audio_source.rs`

```diff
--- reference/src/protocol_serde/shape_audio_source.rs
+++ generated/src/protocol_serde/shape_audio_source.rs
@@ -1,15 +1,15 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub fn ser_audio_source(
-    object_2: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::AudioSource,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
     match input {
         super::super::types::AudioSource::Bytes(inner) => {
-            object_2.key("bytes").string_unchecked(&::aws_smithy_types::base64::encode(inner));
+            object.key("bytes").string_unchecked(&::aws_smithy_types::base64::encode(inner));
         }
         super::super::types::AudioSource::S3Location(inner) => {
             #[allow(unused_mut)]
-            let mut object_1 = object_2.key("s3Location").start_object();
+            let mut object_1 = object.key("s3Location").start_object();
             super::super::protocol_serde::shape_s3_location::ser_s3_location(&mut object_1, inner)?;
             object_1.finish();
         }
@@ -38,9 +38,7 @@
             match tokens.next().transpose()? {
                 Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                 Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
-                    if let ::std::option::Option::Some(::std::result::Result::Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) =
-                        tokens.peek()
-                    {
+                    if let Some(Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) = tokens.peek() {
                         let _ = tokens.next().expect("peek returned a token")?;
                         continue;
                     }
```

### `src/protocol_serde/shape_bidirectional_output_payload_part.rs`

```diff
--- reference/src/protocol_serde/shape_bidirectional_output_payload_part.rs
+++ generated/src/protocol_serde/shape_bidirectional_output_payload_part.rs
@@ -1,21 +1,4 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-pub(crate) fn de_bidirectional_output_payload_part_payload(
-    _value: &[u8],
-) -> ::std::result::Result<super::super::types::BidirectionalOutputPayloadPart, ::aws_smithy_json::deserialize::error::DeserializeError> {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
-    let tokens = &mut tokens_owned;
-    #[allow(unused_variables)]
-    let depth = 0u32;
-    let result = super::super::protocol_serde::shape_bidirectional_output_payload_part::de_bidirectional_output_payload_part(tokens, _value, depth + 1)?
-        .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("expected payload member value"));
-    if tokens.next().is_some() {
-        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-            "found more JSON tokens after completing parsing",
-        ));
-    }
-    result
-}
-
 pub(crate) fn de_bidirectional_output_payload_part<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
```

### `src/protocol_serde/shape_citation_generated_content.rs`

```diff
--- reference/src/protocol_serde/shape_citation_generated_content.rs
+++ generated/src/protocol_serde/shape_citation_generated_content.rs
@@ -1,11 +1,11 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub fn ser_citation_generated_content(
-    object_4: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::CitationGeneratedContent,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
     match input {
         super::super::types::CitationGeneratedContent::Text(inner) => {
-            object_4.key("text").string(inner.as_str());
+            object.key("text").string(inner.as_str());
         }
         super::super::types::CitationGeneratedContent::Unknown => {
             return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant(
@@ -36,9 +36,7 @@
             match tokens.next().transpose()? {
                 Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                 Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
-                    if let ::std::option::Option::Some(::std::result::Result::Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) =
-                        tokens.peek()
-                    {
+                    if let Some(Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) = tokens.peek() {
                         let _ = tokens.next().expect("peek returned a token")?;
                         continue;
                     }
```

### `src/protocol_serde/shape_citation_location.rs`

```diff
--- reference/src/protocol_serde/shape_citation_location.rs
+++ generated/src/protocol_serde/shape_citation_location.rs
@@ -1,38 +1,38 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub fn ser_citation_location(
-    object_8: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::CitationLocation,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
     match input {
         super::super::types::CitationLocation::Web(inner) => {
             #[allow(unused_mut)]
-            let mut object_1 = object_8.key("web").start_object();
+            let mut object_1 = object.key("web").start_object();
             super::super::protocol_serde::shape_web_location::ser_web_location(&mut object_1, inner)?;
             object_1.finish();
         }
         super::super::types::CitationLocation::DocumentChar(inner) => {
             #[allow(unused_mut)]
-            let mut object_2 = object_8.key("documentChar").start_object();
-            super::super::protocol_serde::shape_document_char_location::ser_document_char_location(&mut object_2, inner)?;
-            object_2.finish();
+            let mut object_1 = object.key("documentChar").start_object();
+            super::super::protocol_serde::shape_document_char_location::ser_document_char_location(&mut object_1, inner)?;
+            object_1.finish();
         }
         super::super::types::CitationLocation::DocumentPage(inner) => {
             #[allow(unused_mut)]
-            let mut object_3 = object_8.key("documentPage").start_object();
-            super::super::protocol_serde::shape_document_page_location::ser_document_page_location(&mut object_3, inner)?;
-            object_3.finish();
+            let mut object_1 = object.key("documentPage").start_object();
+            super::super::protocol_serde::shape_document_page_location::ser_document_page_location(&mut object_1, inner)?;
+            object_1.finish();
         }
         super::super::types::CitationLocation::DocumentChunk(inner) => {
             #[allow(unused_mut)]
-            let mut object_4 = object_8.key("documentChunk").start_object();
-            super::super::protocol_serde::shape_document_chunk_location::ser_document_chunk_location(&mut object_4, inner)?;
-            object_4.finish();
+            let mut object_1 = object.key("documentChunk").start_object();
+            super::super::protocol_serde::shape_document_chunk_location::ser_document_chunk_location(&mut object_1, inner)?;
+            object_1.finish();
         }
         super::super::types::CitationLocation::SearchResultLocation(inner) => {
             #[allow(unused_mut)]
-            let mut object_5 = object_8.key("searchResultLocation").start_object();
-            super::super::protocol_serde::shape_search_result_location::ser_search_result_location(&mut object_5, inner)?;
-            object_5.finish();
+            let mut object_1 = object.key("searchResultLocation").start_object();
+            super::super::protocol_serde::shape_search_result_location::ser_search_result_location(&mut object_1, inner)?;
+            object_1.finish();
         }
         super::super::types::CitationLocation::Unknown => {
             return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant(
@@ -63,9 +63,7 @@
             match tokens.next().transpose()? {
                 Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                 Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
-                    if let ::std::option::Option::Some(::std::result::Result::Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) =
-                        tokens.peek()
-                    {
+                    if let Some(Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) = tokens.peek() {
                         let _ = tokens.next().expect("peek returned a token")?;
                         continue;
                     }
```

### `src/protocol_serde/shape_citation_source_content.rs`

```diff
--- reference/src/protocol_serde/shape_citation_source_content.rs
+++ generated/src/protocol_serde/shape_citation_source_content.rs
@@ -1,11 +1,11 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub fn ser_citation_source_content(
-    object_6: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::CitationSourceContent,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
     match input {
         super::super::types::CitationSourceContent::Text(inner) => {
-            object_6.key("text").string(inner.as_str());
+            object.key("text").string(inner.as_str());
         }
         super::super::types::CitationSourceContent::Unknown => {
             return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant(
@@ -36,9 +36,7 @@
             match tokens.next().transpose()? {
                 Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                 Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
-                    if let ::std::option::Option::Some(::std::result::Result::Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) =
-                        tokens.peek()
-                    {
+                    if let Some(Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) = tokens.peek() {
                         let _ = tokens.next().expect("peek returned a token")?;
                         continue;
                     }
```

### `src/protocol_serde/shape_content_block.rs`

```diff
--- reference/src/protocol_serde/shape_content_block.rs
+++ generated/src/protocol_serde/shape_content_block.rs
@@ -1,89 +1,89 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub fn ser_content_block(
-    object_3: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::ContentBlock,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
     match input {
         super::super::types::ContentBlock::Text(inner) => {
-            object_3.key("text").string(inner.as_str());
+            object.key("text").string(inner.as_str());
         }
         super::super::types::ContentBlock::Image(inner) => {
             #[allow(unused_mut)]
-            let mut object_1 = object_3.key("image").start_object();
+            let mut object_1 = object.key("image").start_object();
             super::super::protocol_serde::shape_image_block::ser_image_block(&mut object_1, inner)?;
             object_1.finish();
         }
         super::super::types::ContentBlock::Document(inner) => {
             #[allow(unused_mut)]
-            let mut object_2 = object_3.key("document").start_object();
-            super::super::protocol_serde::shape_document_block::ser_document_block(&mut object_2, inner)?;
-            object_2.finish();
+            let mut object_1 = object.key("document").start_object();
+            super::super::protocol_serde::shape_document_block::ser_document_block(&mut object_1, inner)?;
+            object_1.finish();
         }
         super::super::types::ContentBlock::Video(inner) => {
             #[allow(unused_mut)]
-            let mut object_3 = object_3.key("video").start_object();
-            super::super::protocol_serde::shape_video_block::ser_video_block(&mut object_3, inner)?;
-            object_3.finish();
+            let mut object_1 = object.key("video").start_object();
+            super::super::protocol_serde::shape_video_block::ser_video_block(&mut object_1, inner)?;
+            object_1.finish();
         }
         super::super::types::ContentBlock::Audio(inner) => {
             #[allow(unused_mut)]
-            let mut object_4 = object_3.key("audio").start_object();
-            super::super::protocol_serde::shape_audio_block::ser_audio_block(&mut object_4, inner)?;
-            object_4.finish();
+            let mut object_1 = object.key("audio").start_object();
+            super::super::protocol_serde::shape_audio_block::ser_audio_block(&mut object_1, inner)?;
+            object_1.finish();
         }
         super::super::types::ContentBlock::ToolUse(inner) => {
             #[allow(unused_mut)]
-            let mut object_5 = object_3.key("toolUse").start_object();
-            super::super::protocol_serde::shape_tool_use_block::ser_tool_use_block(&mut object_5, inner)?;
-            object_5.finish();
+            let mut object_1 = object.key("toolUse").start_object();
+            super::super::protocol_serde::shape_tool_use_block::ser_tool_use_block(&mut object_1, inner)?;
+            object_1.finish();
         }
         super::super::types::ContentBlock::ToolResult(inner) => {
             #[allow(unused_mut)]
-            let mut object_6 = object_3.key("toolResult").start_object();
-            super::super::protocol_serde::shape_tool_result_block::ser_tool_result_block(&mut object_6, inner)?;
-            object_6.finish();
+            let mut object_1 = object.key("toolResult").start_object();
+            super::super::protocol_serde::shape_tool_result_block::ser_tool_result_block(&mut object_1, inner)?;
+            object_1.finish();
         }
         super::super::types::ContentBlock::GuardContent(inner) => {
             #[allow(unused_mut)]
-            let mut object_7 = object_3.key("guardContent").start_object();
-            super::super::protocol_serde::shape_guardrail_converse_content_block::ser_guardrail_converse_content_block(&mut object_7, inner)?;
-            object_7.finish();
+            let mut object_1 = object.key("guardContent").start_object();
+            super::super::protocol_serde::shape_guardrail_converse_content_block::ser_guardrail_converse_content_block(&mut object_1, inner)?;
+            object_1.finish();
         }
         super::super::types::ContentBlock::CachePoint(inner) => {
             #[allow(unused_mut)]
-            let mut object_8 = object_3.key("cachePoint").start_object();
-            super::super::protocol_serde::shape_cache_point_block::ser_cache_point_block(&mut object_8, inner)?;
-            object_8.finish();
+            let mut object_1 = object.key("cachePoint").start_object();
+            super::super::protocol_serde::shape_cache_point_block::ser_cache_point_block(&mut object_1, inner)?;
+            object_1.finish();
         }
         super::super::types::ContentBlock::ReasoningContent(inner) => {
             #[allow(unused_mut)]
-            let mut object_9 = object_3.key("reasoningContent").start_object();
-            super::super::protocol_serde::shape_reasoning_content_block::ser_reasoning_content_block(&mut object_9, inner)?;
-            object_9.finish();
+            let mut object_1 = object.key("reasoningContent").start_object();
+            super::super::protocol_serde::shape_reasoning_content_block::ser_reasoning_content_block(&mut object_1, inner)?;
+            object_1.finish();
         }
         super::super::types::ContentBlock::CitationsContent(inner) => {
             #[allow(unused_mut)]
-            let mut object_10 = object_3.key("citationsContent").start_object();
-            super::super::protocol_serde::shape_citations_content_block::ser_citations_content_block(&mut object_10, inner)?;
-            object_10.finish();
+            let mut object_1 = object.key("citationsContent").start_object();
+            super::super::protocol_serde::shape_citations_content_block::ser_citations_content_block(&mut object_1, inner)?;
+            object_1.finish();
         }
         super::super::types::ContentBlock::SearchResult(inner) => {
             #[allow(unused_mut)]
-            let mut object_11 = object_3.key("searchResult").start_object();
-            super::super::protocol_serde::shape_search_result_block::ser_search_result_block(&mut object_11, inner)?;
-            object_11.finish();
+            let mut object_1 = object.key("searchResult").start_object();
+            super::super::protocol_serde::shape_search_result_block::ser_search_result_block(&mut object_1, inner)?;
+            object_1.finish();
         }
         super::super::types::ContentBlock::ToolAddition(inner) => {
             #[allow(unused_mut)]
-            let mut object_12 = object_3.key("toolAddition").start_object();
-            super::super::protocol_serde::shape_tool_addition_block::ser_tool_addition_block(&mut object_12, inner)?;
-            object_12.finish();
+            let mut object_1 = object.key("toolAddition").start_object();
+            super::super::protocol_serde::shape_tool_addition_block::ser_tool_addition_block(&mut object_1, inner)?;
+            object_1.finish();
         }
         super::super::types::ContentBlock::ToolRemoval(inner) => {
             #[allow(unused_mut)]
-            let mut object_13 = object_3.key("toolRemoval").start_object();
-            super::super::protocol_serde::shape_tool_removal_block::ser_tool_removal_block(&mut object_13, inner)?;
-            object_13.finish();
+            let mut object_1 = object.key("toolRemoval").start_object();
+            super::super::protocol_serde::shape_tool_removal_block::ser_tool_removal_block(&mut object_1, inner)?;
+            object_1.finish();
         }
         super::super::types::ContentBlock::Unknown => return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant("ContentBlock")),
     }
@@ -110,9 +110,7 @@
             match tokens.next().transpose()? {
                 Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                 Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
-                    if let ::std::option::Option::Some(::std::result::Result::Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) =
-                        tokens.peek()
-                    {
+                    if let Some(Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) = tokens.peek() {
                         let _ = tokens.next().expect("peek returned a token")?;
                         continue;
                     }
```

### `src/protocol_serde/shape_content_block_delta.rs`

```diff
--- reference/src/protocol_serde/shape_content_block_delta.rs
+++ generated/src/protocol_serde/shape_content_block_delta.rs
@@ -19,9 +19,7 @@
             match tokens.next().transpose()? {
                 Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                 Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
-                    if let ::std::option::Option::Some(::std::result::Result::Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) =
-                        tokens.peek()
-                    {
+                    if let Some(Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) = tokens.peek() {
                         let _ = tokens.next().expect("peek returned a token")?;
                         continue;
                     }
```

### `src/protocol_serde/shape_content_block_delta_event.rs`

```diff
--- reference/src/protocol_serde/shape_content_block_delta_event.rs
+++ generated/src/protocol_serde/shape_content_block_delta_event.rs
@@ -1,21 +1,4 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-pub(crate) fn de_content_block_delta_event_payload(
-    _value: &[u8],
-) -> ::std::result::Result<super::super::types::ContentBlockDeltaEvent, ::aws_smithy_json::deserialize::error::DeserializeError> {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
-    let tokens = &mut tokens_owned;
-    #[allow(unused_variables)]
-    let depth = 0u32;
-    let result = super::super::protocol_serde::shape_content_block_delta_event::de_content_block_delta_event(tokens, _value, depth + 1)?
-        .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("expected payload member value"));
-    if tokens.next().is_some() {
-        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-            "found more JSON tokens after completing parsing",
-        ));
-    }
-    result
-}
-
 pub(crate) fn de_content_block_delta_event<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
```

### `src/protocol_serde/shape_content_block_start.rs`

```diff
--- reference/src/protocol_serde/shape_content_block_start.rs
+++ generated/src/protocol_serde/shape_content_block_start.rs
@@ -19,9 +19,7 @@
             match tokens.next().transpose()? {
                 Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                 Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
-                    if let ::std::option::Option::Some(::std::result::Result::Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) =
-                        tokens.peek()
-                    {
+                    if let Some(Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) = tokens.peek() {
                         let _ = tokens.next().expect("peek returned a token")?;
                         continue;
                     }
```

### `src/protocol_serde/shape_content_block_start_event.rs`

```diff
--- reference/src/protocol_serde/shape_content_block_start_event.rs
+++ generated/src/protocol_serde/shape_content_block_start_event.rs
@@ -1,21 +1,4 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-pub(crate) fn de_content_block_start_event_payload(
-    _value: &[u8],
-) -> ::std::result::Result<super::super::types::ContentBlockStartEvent, ::aws_smithy_json::deserialize::error::DeserializeError> {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
-    let tokens = &mut tokens_owned;
-    #[allow(unused_variables)]
-    let depth = 0u32;
-    let result = super::super::protocol_serde::shape_content_block_start_event::de_content_block_start_event(tokens, _value, depth + 1)?
-        .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("expected payload member value"));
-    if tokens.next().is_some() {
-        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-            "found more JSON tokens after completing parsing",
-        ));
-    }
-    result
-}
-
 pub(crate) fn de_content_block_start_event<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
```

### `src/protocol_serde/shape_content_block_stop_event.rs`

```diff
--- reference/src/protocol_serde/shape_content_block_stop_event.rs
+++ generated/src/protocol_serde/shape_content_block_stop_event.rs
@@ -1,21 +1,4 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-pub(crate) fn de_content_block_stop_event_payload(
-    _value: &[u8],
-) -> ::std::result::Result<super::super::types::ContentBlockStopEvent, ::aws_smithy_json::deserialize::error::DeserializeError> {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
-    let tokens = &mut tokens_owned;
-    #[allow(unused_variables)]
-    let depth = 0u32;
-    let result = super::super::protocol_serde::shape_content_block_stop_event::de_content_block_stop_event(tokens, _value, depth + 1)?
-        .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("expected payload member value"));
-    if tokens.next().is_some() {
-        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-            "found more JSON tokens after completing parsing",
-        ));
-    }
-    result
-}
-
 pub(crate) fn de_content_block_stop_event<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
```

### `src/protocol_serde/shape_converse.rs`

```diff
--- reference/src/protocol_serde/shape_converse.rs
+++ generated/src/protocol_serde/shape_converse.rs
@@ -200,7 +200,8 @@
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                 match key.to_unescaped()?.as_ref() {
                     "additionalModelResponseFields" => {
-                        builder = builder.set_additional_model_response_fields(Some(::aws_smithy_json::deserialize::token::expect_document(tokens)?));
+                        builder = builder
+                            .set_additional_model_response_fields(Some(::aws_smithy_json::deserialize::token::expect_document(tokens.next())?));
                     }
                     "metrics" => {
                         builder = builder.set_metrics(super::super::protocol_serde::shape_converse_metrics::de_converse_metrics(
```

### `src/protocol_serde/shape_converse_output.rs`

```diff
--- reference/src/protocol_serde/shape_converse_output.rs
+++ generated/src/protocol_serde/shape_converse_output.rs
@@ -19,9 +19,7 @@
             match tokens.next().transpose()? {
                 Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                 Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
-                    if let ::std::option::Option::Some(::std::result::Result::Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) =
-                        tokens.peek()
-                    {
+                    if let Some(Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) = tokens.peek() {
                         let _ = tokens.next().expect("peek returned a token")?;
                         continue;
                     }
```

### `src/protocol_serde/shape_converse_stream_metadata_event.rs`

```diff
--- reference/src/protocol_serde/shape_converse_stream_metadata_event.rs
+++ generated/src/protocol_serde/shape_converse_stream_metadata_event.rs
@@ -1,21 +1,4 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-pub(crate) fn de_converse_stream_metadata_event_payload(
-    _value: &[u8],
-) -> ::std::result::Result<super::super::types::ConverseStreamMetadataEvent, ::aws_smithy_json::deserialize::error::DeserializeError> {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
-    let tokens = &mut tokens_owned;
-    #[allow(unused_variables)]
-    let depth = 0u32;
-    let result = super::super::protocol_serde::shape_converse_stream_metadata_event::de_converse_stream_metadata_event(tokens, _value, depth + 1)?
-        .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("expected payload member value"));
-    if tokens.next().is_some() {
-        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-            "found more JSON tokens after completing parsing",
-        ));
-    }
-    result
-}
-
 pub(crate) fn de_converse_stream_metadata_event<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
```

### `src/protocol_serde/shape_count_tokens_input.rs`

```diff
--- reference/src/protocol_serde/shape_count_tokens_input.rs
+++ generated/src/protocol_serde/shape_count_tokens_input.rs
@@ -1,33 +1,20 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-pub fn ser_count_tokens_input_input(
-    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
-    input: &super::super::operation::count_tokens::CountTokensInput,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    if let Some(var_1) = &input.input {
-        #[allow(unused_mut)]
-        let mut object_2 = object.key("input").start_object();
-        super::super::protocol_serde::shape_count_tokens_input::ser_count_tokens_input(&mut object_2, var_1)?;
-        object_2.finish();
-    }
-    Ok(())
-}
-
 pub fn ser_count_tokens_input(
-    object_2: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::CountTokensInput,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
     match input {
         super::super::types::CountTokensInput::InvokeModel(inner) => {
             #[allow(unused_mut)]
-            let mut object_3 = object_2.key("invokeModel").start_object();
-            super::super::protocol_serde::shape_invoke_model_tokens_request::ser_invoke_model_tokens_request(&mut object_3, inner)?;
-            object_3.finish();
+            let mut object_1 = object.key("invokeModel").start_object();
+            super::super::protocol_serde::shape_invoke_model_tokens_request::ser_invoke_model_tokens_request(&mut object_1, inner)?;
+            object_1.finish();
         }
         super::super::types::CountTokensInput::Converse(inner) => {
             #[allow(unused_mut)]
-            let mut object_4 = object_2.key("converse").start_object();
-            super::super::protocol_serde::shape_converse_tokens_request::ser_converse_tokens_request(&mut object_4, inner)?;
-            object_4.finish();
+            let mut object_1 = object.key("converse").start_object();
+            super::super::protocol_serde::shape_converse_tokens_request::ser_converse_tokens_request(&mut object_1, inner)?;
+            object_1.finish();
         }
         super::super::types::CountTokensInput::Unknown => {
             return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant(
```

### `src/protocol_serde/shape_document_content_block.rs`

```diff
--- reference/src/protocol_serde/shape_document_content_block.rs
+++ generated/src/protocol_serde/shape_document_content_block.rs
@@ -1,11 +1,11 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub fn ser_document_content_block(
-    object_4: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::DocumentContentBlock,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
     match input {
         super::super::types::DocumentContentBlock::Text(inner) => {
-            object_4.key("text").string(inner.as_str());
+            object.key("text").string(inner.as_str());
         }
         super::super::types::DocumentContentBlock::Unknown => {
             return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant(
@@ -36,9 +36,7 @@
             match tokens.next().transpose()? {
                 Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                 Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
-                    if let ::std::option::Option::Some(::std::result::Result::Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) =
-                        tokens.peek()
-                    {
+                    if let Some(Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) = tokens.peek() {
                         let _ = tokens.next().expect("peek returned a token")?;
                         continue;
                     }
```

### `src/protocol_serde/shape_document_source.rs`

```diff
--- reference/src/protocol_serde/shape_document_source.rs
+++ generated/src/protocol_serde/shape_document_source.rs
@@ -1,32 +1,32 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub fn ser_document_source(
-    object_2: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::DocumentSource,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
     match input {
         super::super::types::DocumentSource::Bytes(inner) => {
-            object_2.key("bytes").string_unchecked(&::aws_smithy_types::base64::encode(inner));
+            object.key("bytes").string_unchecked(&::aws_smithy_types::base64::encode(inner));
         }
         super::super::types::DocumentSource::S3Location(inner) => {
             #[allow(unused_mut)]
-            let mut object_1 = object_2.key("s3Location").start_object();
+            let mut object_1 = object.key("s3Location").start_object();
             super::super::protocol_serde::shape_s3_location::ser_s3_location(&mut object_1, inner)?;
             object_1.finish();
         }
         super::super::types::DocumentSource::Text(inner) => {
-            object_2.key("text").string(inner.as_str());
+            object.key("text").string(inner.as_str());
         }
         super::super::types::DocumentSource::Content(inner) => {
-            let mut array_2 = object_2.key("content").start_array();
-            for item_3 in inner {
+            let mut array_1 = object.key("content").start_array();
+            for item_2 in inner {
                 {
                     #[allow(unused_mut)]
-                    let mut object_4 = array_2.value().start_object();
-                    super::super::protocol_serde::shape_document_content_block::ser_document_content_block(&mut object_4, item_3)?;
-                    object_4.finish();
+                    let mut object_3 = array_1.value().start_object();
+                    super::super::protocol_serde::shape_document_content_block::ser_document_content_block(&mut object_3, item_2)?;
+                    object_3.finish();
                 }
             }
-            array_2.finish();
+            array_1.finish();
         }
         super::super::types::DocumentSource::Unknown => {
             return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant(
@@ -57,9 +57,7 @@
             match tokens.next().transpose()? {
                 Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                 Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
-                    if let ::std::option::Option::Some(::std::result::Result::Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) =
-                        tokens.peek()
-                    {
+                    if let Some(Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) = tokens.peek() {
                         let _ = tokens.next().expect("peek returned a token")?;
                         continue;
                     }
```

### `src/protocol_serde/shape_guardrail_automated_reasoning_finding.rs`

```diff
--- reference/src/protocol_serde/shape_guardrail_automated_reasoning_finding.rs
+++ generated/src/protocol_serde/shape_guardrail_automated_reasoning_finding.rs
@@ -19,9 +19,7 @@
             match tokens.next().transpose()? {
                 Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                 Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
-                    if let ::std::option::Option::Some(::std::result::Result::Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) =
-                        tokens.peek()
-                    {
+                    if let Some(Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) = tokens.peek() {
                         let _ = tokens.next().expect("peek returned a token")?;
                         continue;
                     }
@@ -36,53 +34,18 @@
                         ));
                     }
                     variant = match key.as_ref() {
-                            "valid" => {
-                                Some(super::super::types::GuardrailAutomatedReasoningFinding::Valid(
-                                    super::super::protocol_serde::shape_guardrail_automated_reasoning_valid_finding::de_guardrail_automated_reasoning_valid_finding(tokens, _value, depth + 1)?
-                                    .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'valid' cannot be null"))?
-                                ))
-                            }
-                            "invalid" => {
-                                Some(super::super::types::GuardrailAutomatedReasoningFinding::Invalid(
-                                    super::super::protocol_serde::shape_guardrail_automated_reasoning_invalid_finding::de_guardrail_automated_reasoning_invalid_finding(tokens, _value, depth + 1)?
-                                    .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'invalid' cannot be null"))?
-                                ))
-                            }
-                            "satisfiable" => {
-                                Some(super::super::types::GuardrailAutomatedReasoningFinding::Satisfiable(
-                                    super::super::protocol_serde::shape_guardrail_automated_reasoning_satisfiable_finding::de_guardrail_automated_reasoning_satisfiable_finding(tokens, _value, depth + 1)?
-                                    .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'satisfiable' cannot be null"))?
-                                ))
-                            }
-                            "impossible" => {
-                                Some(super::super::types::GuardrailAutomatedReasoningFinding::Impossible(
-                                    super::super::protocol_serde::shape_guardrail_automated_reasoning_impossible_finding::de_guardrail_automated_reasoning_impossible_finding(tokens, _value, depth + 1)?
-                                    .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'impossible' cannot be null"))?
-                                ))
-                            }
-                            "translationAmbiguous" => {
-                                Some(super::super::types::GuardrailAutomatedReasoningFinding::TranslationAmbiguous(
-                                    super::super::protocol_serde::shape_guardrail_automated_reasoning_translation_ambiguous_finding::de_guardrail_automated_reasoning_translation_ambiguous_finding(tokens, _value, depth + 1)?
-                                    .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'translationAmbiguous' cannot be null"))?
-                                ))
-                            }
-                            "tooComplex" => {
-                                Some(super::super::types::GuardrailAutomatedReasoningFinding::TooComplex(
-                                    super::super::protocol_serde::shape_guardrail_automated_reasoning_too_complex_finding::de_guardrail_automated_reasoning_too_complex_finding(tokens, _value, depth + 1)?
-                                    .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'tooComplex' cannot be null"))?
-                                ))
-                            }
-                            "noTranslations" => {
-                                Some(super::super::types::GuardrailAutomatedReasoningFinding::NoTranslations(
-                                    super::super::protocol_serde::shape_guardrail_automated_reasoning_no_translations_finding::de_guardrail_automated_reasoning_no_translations_finding(tokens, _value, depth + 1)?
-                                    .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'noTranslations' cannot be null"))?
-                                ))
-                            }
-                            _ => {
-                                                                              ::aws_smithy_json::deserialize::token::skip_value(tokens)?;
-                                                                              Some(super::super::types::GuardrailAutomatedReasoningFinding::Unknown)
-                                                                            }
-                        };
+                        "valid" => Some(super::super::types::GuardrailAutomatedReasoningFinding::Valid(super::super::protocol_serde::shape_guardrail_automated_reasoning_valid_finding::de_guardrail_automated_reasoning_valid_finding(tokens, _value, depth + 1)?.ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'valid' cannot be null"))?)),
+                        "invalid" => Some(super::super::types::GuardrailAutomatedReasoningFinding::Invalid(super::super::protocol_serde::shape_guardrail_automated_reasoning_invalid_finding::de_guardrail_automated_reasoning_invalid_finding(tokens, _value, depth + 1)?.ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'invalid' cannot be null"))?)),
+                        "satisfiable" => Some(super::super::types::GuardrailAutomatedReasoningFinding::Satisfiable(super::super::protocol_serde::shape_guardrail_automated_reasoning_satisfiable_finding::de_guardrail_automated_reasoning_satisfiable_finding(tokens, _value, depth + 1)?.ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'satisfiable' cannot be null"))?)),
+                        "impossible" => Some(super::super::types::GuardrailAutomatedReasoningFinding::Impossible(super::super::protocol_serde::shape_guardrail_automated_reasoning_impossible_finding::de_guardrail_automated_reasoning_impossible_finding(tokens, _value, depth + 1)?.ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'impossible' cannot be null"))?)),
+                        "translationAmbiguous" => Some(super::super::types::GuardrailAutomatedReasoningFinding::TranslationAmbiguous(super::super::protocol_serde::shape_guardrail_automated_reasoning_translation_ambiguous_finding::de_guardrail_automated_reasoning_translation_ambiguous_finding(tokens, _value, depth + 1)?.ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'translationAmbiguous' cannot be null"))?)),
+                        "tooComplex" => Some(super::super::types::GuardrailAutomatedReasoningFinding::TooComplex(super::super::protocol_serde::shape_guardrail_automated_reasoning_too_complex_finding::de_guardrail_automated_reasoning_too_complex_finding(tokens, _value, depth + 1)?.ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'tooComplex' cannot be null"))?)),
+                        "noTranslations" => Some(super::super::types::GuardrailAutomatedReasoningFinding::NoTranslations(super::super::protocol_serde::shape_guardrail_automated_reasoning_no_translations_finding::de_guardrail_automated_reasoning_no_translations_finding(tokens, _value, depth + 1)?.ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'noTranslations' cannot be null"))?)),
+                        _ => {
+                            ::aws_smithy_json::deserialize::token::skip_value(tokens)?;
+                            Some(super::super::types::GuardrailAutomatedReasoningFinding::Unknown)
+                        }
+                    };
                 }
                 other => {
                     return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
```

### `src/protocol_serde/shape_guardrail_automated_reasoning_no_translations_finding.rs`

```diff
--- reference/src/protocol_serde/shape_guardrail_automated_reasoning_no_translations_finding.rs
+++ generated/src/protocol_serde/shape_guardrail_automated_reasoning_no_translations_finding.rs
@@ -4,7 +4,7 @@
     _value: &'a [u8],
     depth: u32,
 ) -> ::std::result::Result<
-    Option<super::super::types::GuardrailAutomatedReasoningNoTranslationsFinding>,
+    Option<super::super::types::crate::types::GuardrailAutomatedReasoningNoTranslationsFinding>,
     ::aws_smithy_json::deserialize::error::DeserializeError,
 >
 where
@@ -19,7 +19,7 @@
         Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
         Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
             #[allow(unused_mut)]
-            let mut builder = super::super::types::builders::GuardrailAutomatedReasoningNoTranslationsFindingBuilder::default();
+            let mut builder = super::super::types::builders::crate::types::GuardrailAutomatedReasoningNoTranslationsFindingBuilder::default();
             ::aws_smithy_json::deserialize::token::skip_to_end(tokens)?;
             Ok(Some(builder.build()))
         }
```

### `src/protocol_serde/shape_guardrail_automated_reasoning_too_complex_finding.rs`

```diff
--- reference/src/protocol_serde/shape_guardrail_automated_reasoning_too_complex_finding.rs
+++ generated/src/protocol_serde/shape_guardrail_automated_reasoning_too_complex_finding.rs
@@ -3,7 +3,10 @@
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
     depth: u32,
-) -> ::std::result::Result<Option<super::super::types::GuardrailAutomatedReasoningTooComplexFinding>, ::aws_smithy_json::deserialize::error::DeserializeError>
+) -> ::std::result::Result<
+    Option<super::super::types::crate::types::GuardrailAutomatedReasoningTooComplexFinding>,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+>
 where
     I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
 {
@@ -16,7 +19,7 @@
         Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
         Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
             #[allow(unused_mut)]
-            let mut builder = super::super::types::builders::GuardrailAutomatedReasoningTooComplexFindingBuilder::default();
+            let mut builder = super::super::types::builders::crate::types::GuardrailAutomatedReasoningTooComplexFindingBuilder::default();
             ::aws_smithy_json::deserialize::token::skip_to_end(tokens)?;
             Ok(Some(builder.build()))
         }
```

### `src/protocol_serde/shape_guardrail_checks_content_block.rs`

```diff
--- reference/src/protocol_serde/shape_guardrail_checks_content_block.rs
+++ generated/src/protocol_serde/shape_guardrail_checks_content_block.rs
@@ -1,11 +1,11 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub fn ser_guardrail_checks_content_block(
-    object_3: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::GuardrailChecksContentBlock,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
     match input {
         super::super::types::GuardrailChecksContentBlock::Text(inner) => {
-            object_3.key("text").string(inner.as_str());
+            object.key("text").string(inner.as_str());
         }
         super::super::types::GuardrailChecksContentBlock::Unknown => {
             return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant(
```

### `src/protocol_serde/shape_guardrail_content_block.rs`

```diff
--- reference/src/protocol_serde/shape_guardrail_content_block.rs
+++ generated/src/protocol_serde/shape_guardrail_content_block.rs
@@ -1,20 +1,20 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub fn ser_guardrail_content_block(
-    object_4: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::GuardrailContentBlock,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
     match input {
         super::super::types::GuardrailContentBlock::Text(inner) => {
             #[allow(unused_mut)]
-            let mut object_1 = object_4.key("text").start_object();
+            let mut object_1 = object.key("text").start_object();
             super::super::protocol_serde::shape_guardrail_text_block::ser_guardrail_text_block(&mut object_1, inner)?;
             object_1.finish();
         }
         super::super::types::GuardrailContentBlock::Image(inner) => {
             #[allow(unused_mut)]
-            let mut object_2 = object_4.key("image").start_object();
-            super::super::protocol_serde::shape_guardrail_image_block::ser_guardrail_image_block(&mut object_2, inner)?;
-            object_2.finish();
+            let mut object_1 = object.key("image").start_object();
+            super::super::protocol_serde::shape_guardrail_image_block::ser_guardrail_image_block(&mut object_1, inner)?;
+            object_1.finish();
         }
         super::super::types::GuardrailContentBlock::Unknown => {
             return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant(
```

### `src/protocol_serde/shape_guardrail_converse_content_block.rs`

```diff
--- reference/src/protocol_serde/shape_guardrail_converse_content_block.rs
+++ generated/src/protocol_serde/shape_guardrail_converse_content_block.rs
@@ -1,20 +1,20 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub fn ser_guardrail_converse_content_block(
-    object_1: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::GuardrailConverseContentBlock,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
     match input {
         super::super::types::GuardrailConverseContentBlock::Text(inner) => {
             #[allow(unused_mut)]
-            let mut object_1 = object_1.key("text").start_object();
+            let mut object_1 = object.key("text").start_object();
             super::super::protocol_serde::shape_guardrail_converse_text_block::ser_guardrail_converse_text_block(&mut object_1, inner)?;
             object_1.finish();
         }
         super::super::types::GuardrailConverseContentBlock::Image(inner) => {
             #[allow(unused_mut)]
-            let mut object_2 = object_1.key("image").start_object();
-            super::super::protocol_serde::shape_guardrail_converse_image_block::ser_guardrail_converse_image_block(&mut object_2, inner)?;
-            object_2.finish();
+            let mut object_1 = object.key("image").start_object();
+            super::super::protocol_serde::shape_guardrail_converse_image_block::ser_guardrail_converse_image_block(&mut object_1, inner)?;
+            object_1.finish();
         }
         super::super::types::GuardrailConverseContentBlock::Unknown => {
             return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant(
@@ -45,9 +45,7 @@
             match tokens.next().transpose()? {
                 Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                 Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
-                    if let ::std::option::Option::Some(::std::result::Result::Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) =
-                        tokens.peek()
-                    {
+                    if let Some(Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) = tokens.peek() {
                         let _ = tokens.next().expect("peek returned a token")?;
                         continue;
                     }
```

### `src/protocol_serde/shape_guardrail_converse_image_source.rs`

```diff
--- reference/src/protocol_serde/shape_guardrail_converse_image_source.rs
+++ generated/src/protocol_serde/shape_guardrail_converse_image_source.rs
@@ -1,11 +1,11 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub fn ser_guardrail_converse_image_source(
-    object_2: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::GuardrailConverseImageSource,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
     match input {
         super::super::types::GuardrailConverseImageSource::Bytes(inner) => {
-            object_2.key("bytes").string_unchecked(&::aws_smithy_types::base64::encode(inner));
+            object.key("bytes").string_unchecked(&::aws_smithy_types::base64::encode(inner));
         }
         super::super::types::GuardrailConverseImageSource::Unknown => {
             return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant(
@@ -36,9 +36,7 @@
             match tokens.next().transpose()? {
                 Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                 Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
-                    if let ::std::option::Option::Some(::std::result::Result::Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) =
-                        tokens.peek()
-                    {
+                    if let Some(Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) = tokens.peek() {
                         let _ = tokens.next().expect("peek returned a token")?;
                         continue;
                     }
```

### `src/protocol_serde/shape_guardrail_image_source.rs`

```diff
--- reference/src/protocol_serde/shape_guardrail_image_source.rs
+++ generated/src/protocol_serde/shape_guardrail_image_source.rs
@@ -1,11 +1,11 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub fn ser_guardrail_image_source(
-    object_2: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::GuardrailImageSource,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
     match input {
         super::super::types::GuardrailImageSource::Bytes(inner) => {
-            object_2.key("bytes").string_unchecked(&::aws_smithy_types::base64::encode(inner));
+            object.key("bytes").string_unchecked(&::aws_smithy_types::base64::encode(inner));
         }
         super::super::types::GuardrailImageSource::Unknown => {
             return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant(
```

### `src/protocol_serde/shape_image_source.rs`

```diff
--- reference/src/protocol_serde/shape_image_source.rs
+++ generated/src/protocol_serde/shape_image_source.rs
@@ -1,15 +1,15 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub fn ser_image_source(
-    object_2: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::ImageSource,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
     match input {
         super::super::types::ImageSource::Bytes(inner) => {
-            object_2.key("bytes").string_unchecked(&::aws_smithy_types::base64::encode(inner));
+            object.key("bytes").string_unchecked(&::aws_smithy_types::base64::encode(inner));
         }
         super::super::types::ImageSource::S3Location(inner) => {
             #[allow(unused_mut)]
-            let mut object_1 = object_2.key("s3Location").start_object();
+            let mut object_1 = object.key("s3Location").start_object();
             super::super::protocol_serde::shape_s3_location::ser_s3_location(&mut object_1, inner)?;
             object_1.finish();
         }
@@ -38,9 +38,7 @@
             match tokens.next().transpose()? {
                 Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                 Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
-                    if let ::std::option::Option::Some(::std::result::Result::Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) =
-                        tokens.peek()
-                    {
+                    if let Some(Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) = tokens.peek() {
                         let _ = tokens.next().expect("peek returned a token")?;
                         continue;
                     }
```

### `src/protocol_serde/shape_internal_server_exception.rs`

```diff
--- reference/src/protocol_serde/shape_internal_server_exception.rs
+++ generated/src/protocol_serde/shape_internal_server_exception.rs
@@ -1,37 +1,46 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-pub(crate) fn de_internal_server_exception_json_err(
-    _value: &[u8],
-    mut builder: super::super::types::error::builders::InternalServerExceptionBuilder,
-) -> ::std::result::Result<super::super::types::error::builders::InternalServerExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
-    let tokens = &mut tokens_owned;
-    #[allow(unused_variables)]
-    let depth = 0u32;
-    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
-    loop {
-        match tokens.next().transpose()? {
-            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "message" => {
-                    builder = builder.set_message(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
+pub(crate) fn de_internal_server_exception<'a, I>(
+    tokens: &mut ::std::iter::Peekable<I>,
+    _value: &'a [u8],
+    depth: u32,
+) -> ::std::result::Result<Option<super::super::types::InternalServerException>, ::aws_smithy_json::deserialize::error::DeserializeError>
+where
+    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
+{
+    if depth >= 128u32 {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "maximum nesting depth exceeded",
+        ));
+    }
+    match tokens.next().transpose()? {
+        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
+        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
+            #[allow(unused_mut)]
+            let mut builder = super::super::types::builders::InternalServerExceptionBuilder::default();
+            loop {
+                match tokens.next().transpose()? {
+                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                        "message" => {
+                            builder = builder.set_message(
+                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                    .transpose()?,
+                            );
+                        }
+                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+                    },
+                    other => {
+                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                            "expected object key or end object, found: {other:?}"
+                        )))
+                    }
                 }
-                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
-            },
-            other => {
-                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
-                    "expected object key or end object, found: {other:?}"
-                )))
             }
+            Ok(Some(builder.build()))
         }
+        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "expected start object or null",
+        )),
     }
-    if tokens.next().is_some() {
-        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-            "found more JSON tokens after completing parsing",
-        ));
-    }
-    Ok(builder)
 }
```

### `src/protocol_serde/shape_invoke_model.rs`

```diff
--- reference/src/protocol_serde/shape_invoke_model.rs
+++ generated/src/protocol_serde/shape_invoke_model.rs
@@ -207,98 +207,3 @@
             .map_err(super::super::operation::invoke_model::InvokeModelError::unhandled)?
     })
 }
-
-pub fn ser_invoke_model_headers(
-    input: &super::super::operation::invoke_model::InvokeModelInput,
-    mut builder: ::http_1x::request::Builder,
-) -> std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
-    if let ::std::option::Option::Some(inner_1) = &input.content_type {
-        let formatted_2 = inner_1.as_str();
-        let header_value = formatted_2;
-        let header_value: ::http_1x::HeaderValue = header_value.parse().map_err(|err| {
-            ::aws_smithy_types::error::operation::BuildError::invalid_field(
-                "content_type",
-                format!("`{}` cannot be used as a header value: {}", &header_value, err),
-            )
-        })?;
-        builder = builder.header("Content-Type", header_value);
-    }
-    if let ::std::option::Option::Some(inner_3) = &input.accept {
-        let formatted_4 = inner_3.as_str();
-        let header_value = formatted_4;
-        let header_value: ::http_1x::HeaderValue = header_value.parse().map_err(|err| {
-            ::aws_smithy_types::error::operation::BuildError::invalid_field(
-                "accept",
-                format!("`{}` cannot be used as a header value: {}", &header_value, err),
-            )
-        })?;
-        builder = builder.header("Accept", header_value);
-    }
-    if let ::std::option::Option::Some(inner_5) = &input.trace {
-        let formatted_6 = inner_5.as_str();
-        let header_value = formatted_6;
-        let header_value: ::http_1x::HeaderValue = header_value.parse().map_err(|err| {
-            ::aws_smithy_types::error::operation::BuildError::invalid_field(
-                "trace",
-                format!("`{}` cannot be used as a header value: {}", &header_value, err),
-            )
-        })?;
-        builder = builder.header("X-Amzn-Bedrock-Trace", header_value);
-    }
-    if let ::std::option::Option::Some(inner_7) = &input.guardrail_identifier {
-        let formatted_8 = inner_7.as_str();
-        let header_value = formatted_8;
-        let header_value: ::http_1x::HeaderValue = header_value.parse().map_err(|err| {
-            ::aws_smithy_types::error::operation::BuildError::invalid_field(
-                "guardrail_identifier",
-                format!("`{}` cannot be used as a header value: {}", &header_value, err),
-            )
-        })?;
-        builder = builder.header("X-Amzn-Bedrock-GuardrailIdentifier", header_value);
-    }
-    if let ::std::option::Option::Some(inner_9) = &input.guardrail_version {
-        let formatted_10 = inner_9.as_str();
-        let header_value = formatted_10;
-        let header_value: ::http_1x::HeaderValue = header_value.parse().map_err(|err| {
-            ::aws_smithy_types::error::operation::BuildError::invalid_field(
-                "guardrail_version",
-                format!("`{}` cannot be used as a header value: {}", &header_value, err),
-            )
-        })?;
-        builder = builder.header("X-Amzn-Bedrock-GuardrailVersion", header_value);
-    }
-    if let ::std::option::Option::Some(inner_11) = &input.performance_config_latency {
-        let formatted_12 = inner_11.as_str();
-        let header_value = formatted_12;
-        let header_value: ::http_1x::HeaderValue = header_value.parse().map_err(|err| {
-            ::aws_smithy_types::error::operation::BuildError::invalid_field(
-                "performance_config_latency",
-                format!("`{}` cannot be used as a header value: {}", &header_value, err),
-            )
-        })?;
-        builder = builder.header("X-Amzn-Bedrock-PerformanceConfig-Latency", header_value);
-    }
-    if let ::std::option::Option::Some(inner_13) = &input.service_tier {
-        let formatted_14 = inner_13.as_str();
-        let header_value = formatted_14;
-        let header_value: ::http_1x::HeaderValue = header_value.parse().map_err(|err| {
-            ::aws_smithy_types::error::operation::BuildError::invalid_field(
-                "service_tier",
-                format!("`{}` cannot be used as a header value: {}", &header_value, err),
-            )
-        })?;
-        builder = builder.header("X-Amzn-Bedrock-Service-Tier", header_value);
-    }
-    if let ::std::option::Option::Some(inner_15) = &input.request_metadata {
-        let formatted_16 = inner_15.as_str();
-        let header_value = formatted_16;
-        let header_value: ::http_1x::HeaderValue = header_value.parse().map_err(|err| {
-            ::aws_smithy_types::error::operation::BuildError::invalid_field(
-                "request_metadata",
-                format!("`{}` cannot be used as a header value: {}", &"*** Sensitive Data Redacted ***", err),
-            )
-        })?;
-        builder = builder.header("X-Amzn-Bedrock-Request-Metadata", header_value);
-    }
-    Ok(builder)
-}
```

### `src/protocol_serde/shape_invoke_model_tokens_request.rs`

```diff
--- reference/src/protocol_serde/shape_invoke_model_tokens_request.rs
+++ generated/src/protocol_serde/shape_invoke_model_tokens_request.rs
@@ -4,7 +4,7 @@
     input: &super::super::types::InvokeModelTokensRequest,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
     {
-        object.key("body").string_unchecked(&::aws_smithy_types::base64::encode(&input.body));
+        object.key("body").string_unchecked(&::aws_smithy_types::base64::encode(input.body));
     }
     Ok(())
 }
```

### `src/protocol_serde/shape_invoke_model_with_bidirectional_stream_input.rs`

```diff
--- reference/src/protocol_serde/shape_invoke_model_with_bidirectional_stream_input.rs
+++ generated/src/protocol_serde/shape_invoke_model_with_bidirectional_stream_input.rs
@@ -1,7 +1,7 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub fn ser_chunk_payload(
     input: &super::super::types::BidirectionalInputPayloadPart,
-) -> std::result::Result<::std::vec::Vec<u8>, ::aws_smithy_types::error::operation::SerializationError> {
+) -> ::std::result::Result<::std::vec::Vec<u8>, ::aws_smithy_types::error::operation::SerializationError> {
     let mut out = String::new();
     let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
     super::super::protocol_serde::shape_bidirectional_input_payload_part::ser_bidirectional_input_payload_part(&mut object, input)?;
```

### `src/protocol_serde/shape_invoke_model_with_response_stream.rs`

```diff
--- reference/src/protocol_serde/shape_invoke_model_with_response_stream.rs
+++ generated/src/protocol_serde/shape_invoke_model_with_response_stream.rs
@@ -42,12 +42,11 @@
             })?,
         );
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
-        super::super::serde_util::invoke_model_with_response_stream_output_output_correct_errors(output)
+        output
             .build()
             .map_err(super::super::operation::invoke_model_with_response_stream::InvokeModelWithResponseStreamError::unhandled)?
     })
 }
-
 #[allow(clippy::unnecessary_wraps)]
 pub fn de_invoke_model_with_response_stream_http_error(
     _response_status: u16,
@@ -255,98 +254,3 @@
         _ => super::super::operation::invoke_model_with_response_stream::InvokeModelWithResponseStreamError::generic(generic),
     })
 }
-
-pub fn ser_invoke_model_with_response_stream_headers(
-    input: &super::super::operation::invoke_model_with_response_stream::InvokeModelWithResponseStreamInput,
-    mut builder: ::http_1x::request::Builder,
-) -> std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
-    if let ::std::option::Option::Some(inner_1) = &input.content_type {
-        let formatted_2 = inner_1.as_str();
-        let header_value = formatted_2;
-        let header_value: ::http_1x::HeaderValue = header_value.parse().map_err(|err| {
-            ::aws_smithy_types::error::operation::BuildError::invalid_field(
-                "content_type",
-                format!("`{}` cannot be used as a header value: {}", &header_value, err),
-            )
-        })?;
-        builder = builder.header("Content-Type", header_value);
-    }
-    if let ::std::option::Option::Some(inner_3) = &input.accept {
-        let formatted_4 = inner_3.as_str();
-        let header_value = formatted_4;
-        let header_value: ::http_1x::HeaderValue = header_value.parse().map_err(|err| {
-            ::aws_smithy_types::error::operation::BuildError::invalid_field(
-                "accept",
-                format!("`{}` cannot be used as a header value: {}", &header_value, err),
-            )
-        })?;
-        builder = builder.header("X-Amzn-Bedrock-Accept", header_value);
-    }
-    if let ::std::option::Option::Some(inner_5) = &input.trace {
-        let formatted_6 = inner_5.as_str();
-        let header_value = formatted_6;
-        let header_value: ::http_1x::HeaderValue = header_value.parse().map_err(|err| {
-            ::aws_smithy_types::error::operation::BuildError::invalid_field(
-                "trace",
-                format!("`{}` cannot be used as a header value: {}", &header_value, err),
-            )
-        })?;
-        builder = builder.header("X-Amzn-Bedrock-Trace", header_value);
-    }
-    if let ::std::option::Option::Some(inner_7) = &input.guardrail_identifier {
-        let formatted_8 = inner_7.as_str();
-        let header_value = formatted_8;
-        let header_value: ::http_1x::HeaderValue = header_value.parse().map_err(|err| {
-            ::aws_smithy_types::error::operation::BuildError::invalid_field(
-                "guardrail_identifier",
-                format!("`{}` cannot be used as a header value: {}", &header_value, err),
-            )
-        })?;
-        builder = builder.header("X-Amzn-Bedrock-GuardrailIdentifier", header_value);
-    }
-    if let ::std::option::Option::Some(inner_9) = &input.guardrail_version {
-        let formatted_10 = inner_9.as_str();
-        let header_value = formatted_10;
-        let header_value: ::http_1x::HeaderValue = header_value.parse().map_err(|err| {
-            ::aws_smithy_types::error::operation::BuildError::invalid_field(
-                "guardrail_version",
-                format!("`{}` cannot be used as a header value: {}", &header_value, err),
-            )
-        })?;
-        builder = builder.header("X-Amzn-Bedrock-GuardrailVersion", header_value);
-    }
-    if let ::std::option::Option::Some(inner_11) = &input.performance_config_latency {
-        let formatted_12 = inner_11.as_str();
-        let header_value = formatted_12;
-        let header_value: ::http_1x::HeaderValue = header_value.parse().map_err(|err| {
-            ::aws_smithy_types::error::operation::BuildError::invalid_field(
-                "performance_config_latency",
-                format!("`{}` cannot be used as a header value: {}", &header_value, err),
-            )
-        })?;
-        builder = builder.header("X-Amzn-Bedrock-PerformanceConfig-Latency", header_value);
-    }
-    if let ::std::option::Option::Some(inner_13) = &input.service_tier {
-        let formatted_14 = inner_13.as_str();
-        let header_value = formatted_14;
-        let header_value: ::http_1x::HeaderValue = header_value.parse().map_err(|err| {
-            ::aws_smithy_types::error::operation::BuildError::invalid_field(
-                "service_tier",
-                format!("`{}` cannot be used as a header value: {}", &header_value, err),
-            )
-        })?;
-        builder = builder.header("X-Amzn-Bedrock-Service-Tier", header_value);
-    }
-    if let ::std::option::Option::Some(inner_15) = &input.request_metadata {
-        let formatted_16 = inner_15.as_str();
-        let header_value = formatted_16;
-        let header_value: ::http_1x::HeaderValue = header_value.parse().map_err(|err| {
-            ::aws_smithy_types::error::operation::BuildError::invalid_field(
-                "request_metadata",
-                format!("`{}` cannot be used as a header value: {}", &"*** Sensitive Data Redacted ***", err),
-            )
-        })?;
-        builder = builder.header("X-Amzn-Bedrock-Request-Metadata", header_value);
-    }
-    Ok(builder)
-}
```

### `src/protocol_serde/shape_message_start_event.rs`

```diff
--- reference/src/protocol_serde/shape_message_start_event.rs
+++ generated/src/protocol_serde/shape_message_start_event.rs
@@ -1,21 +1,4 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-pub(crate) fn de_message_start_event_payload(
-    _value: &[u8],
-) -> ::std::result::Result<super::super::types::MessageStartEvent, ::aws_smithy_json::deserialize::error::DeserializeError> {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
-    let tokens = &mut tokens_owned;
-    #[allow(unused_variables)]
-    let depth = 0u32;
-    let result = super::super::protocol_serde::shape_message_start_event::de_message_start_event(tokens, _value, depth + 1)?
-        .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("expected payload member value"));
-    if tokens.next().is_some() {
-        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-            "found more JSON tokens after completing parsing",
-        ));
-    }
-    result
-}
-
 pub(crate) fn de_message_start_event<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
```

### `src/protocol_serde/shape_message_stop_event.rs`

```diff
--- reference/src/protocol_serde/shape_message_stop_event.rs
+++ generated/src/protocol_serde/shape_message_stop_event.rs
@@ -1,21 +1,4 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-pub(crate) fn de_message_stop_event_payload(
-    _value: &[u8],
-) -> ::std::result::Result<super::super::types::MessageStopEvent, ::aws_smithy_json::deserialize::error::DeserializeError> {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
-    let tokens = &mut tokens_owned;
-    #[allow(unused_variables)]
-    let depth = 0u32;
-    let result = super::super::protocol_serde::shape_message_stop_event::de_message_stop_event(tokens, _value, depth + 1)?
-        .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("expected payload member value"));
-    if tokens.next().is_some() {
-        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-            "found more JSON tokens after completing parsing",
-        ));
-    }
-    result
-}
-
 pub(crate) fn de_message_stop_event<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
@@ -46,8 +29,8 @@
                             );
                         }
                         "additionalModelResponseFields" => {
-                            builder =
-                                builder.set_additional_model_response_fields(Some(::aws_smithy_json::deserialize::token::expect_document(tokens)?));
+                            builder = builder
+                                .set_additional_model_response_fields(Some(::aws_smithy_json::deserialize::token::expect_document(tokens.next())?));
                         }
                         _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                     },
```

### `src/protocol_serde/shape_model_stream_error_exception.rs`

```diff
--- reference/src/protocol_serde/shape_model_stream_error_exception.rs
+++ generated/src/protocol_serde/shape_model_stream_error_exception.rs
@@ -1,51 +1,60 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-pub(crate) fn de_model_stream_error_exception_json_err(
-    _value: &[u8],
-    mut builder: super::super::types::error::builders::ModelStreamErrorExceptionBuilder,
-) -> ::std::result::Result<super::super::types::error::builders::ModelStreamErrorExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
-    let tokens = &mut tokens_owned;
-    #[allow(unused_variables)]
-    let depth = 0u32;
-    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
-    loop {
-        match tokens.next().transpose()? {
-            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "message" => {
-                    builder = builder.set_message(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
-                }
-                "originalStatusCode" => {
-                    builder = builder.set_original_status_code(
-                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
-                            .map(i32::try_from)
-                            .transpose()?,
-                    );
-                }
-                "originalMessage" => {
-                    builder = builder.set_original_message(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
+pub(crate) fn de_model_stream_error_exception<'a, I>(
+    tokens: &mut ::std::iter::Peekable<I>,
+    _value: &'a [u8],
+    depth: u32,
+) -> ::std::result::Result<Option<super::super::types::ModelStreamErrorException>, ::aws_smithy_json::deserialize::error::DeserializeError>
+where
+    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
+{
+    if depth >= 128u32 {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "maximum nesting depth exceeded",
+        ));
+    }
+    match tokens.next().transpose()? {
+        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
+        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
+            #[allow(unused_mut)]
+            let mut builder = super::super::types::builders::ModelStreamErrorExceptionBuilder::default();
+            loop {
+                match tokens.next().transpose()? {
+                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                        "message" => {
+                            builder = builder.set_message(
+                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                    .transpose()?,
+                            );
+                        }
+                        "originalStatusCode" => {
+                            builder = builder.set_original_status_code(
+                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                                    .map(i32::try_from)
+                                    .transpose()?,
+                            );
+                        }
+                        "originalMessage" => {
+                            builder = builder.set_original_message(
+                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                    .transpose()?,
+                            );
+                        }
+                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+                    },
+                    other => {
+                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                            "expected object key or end object, found: {other:?}"
+                        )))
+                    }
                 }
-                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
-            },
-            other => {
-                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
-                    "expected object key or end object, found: {other:?}"
-                )))
             }
+            Ok(Some(builder.build()))
         }
+        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "expected start object or null",
+        )),
     }
-    if tokens.next().is_some() {
-        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-            "found more JSON tokens after completing parsing",
-        ));
-    }
-    Ok(builder)
 }
```

### `src/protocol_serde/shape_model_timeout_exception.rs`

```diff
--- reference/src/protocol_serde/shape_model_timeout_exception.rs
+++ generated/src/protocol_serde/shape_model_timeout_exception.rs
@@ -1,37 +1,46 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-pub(crate) fn de_model_timeout_exception_json_err(
-    _value: &[u8],
-    mut builder: super::super::types::error::builders::ModelTimeoutExceptionBuilder,
-) -> ::std::result::Result<super::super::types::error::builders::ModelTimeoutExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
-    let tokens = &mut tokens_owned;
-    #[allow(unused_variables)]
-    let depth = 0u32;
-    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
-    loop {
-        match tokens.next().transpose()? {
-            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "message" => {
-                    builder = builder.set_message(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
+pub(crate) fn de_model_timeout_exception<'a, I>(
+    tokens: &mut ::std::iter::Peekable<I>,
+    _value: &'a [u8],
+    depth: u32,
+) -> ::std::result::Result<Option<super::super::types::ModelTimeoutException>, ::aws_smithy_json::deserialize::error::DeserializeError>
+where
+    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
+{
+    if depth >= 128u32 {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "maximum nesting depth exceeded",
+        ));
+    }
+    match tokens.next().transpose()? {
+        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
+        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
+            #[allow(unused_mut)]
+            let mut builder = super::super::types::builders::ModelTimeoutExceptionBuilder::default();
+            loop {
+                match tokens.next().transpose()? {
+                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                        "message" => {
+                            builder = builder.set_message(
+                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                    .transpose()?,
+                            );
+                        }
+                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+                    },
+                    other => {
+                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                            "expected object key or end object, found: {other:?}"
+                        )))
+                    }
                 }
-                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
-            },
-            other => {
-                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
-                    "expected object key or end object, found: {other:?}"
-                )))
             }
+            Ok(Some(builder.build()))
         }
+        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "expected start object or null",
+        )),
     }
-    if tokens.next().is_some() {
-        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-            "found more JSON tokens after completing parsing",
-        ));
-    }
-    Ok(builder)
 }
```

### `src/protocol_serde/shape_output_format_structure.rs`

```diff
--- reference/src/protocol_serde/shape_output_format_structure.rs
+++ generated/src/protocol_serde/shape_output_format_structure.rs
@@ -1,12 +1,12 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub fn ser_output_format_structure(
-    object_2: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::OutputFormatStructure,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
     match input {
         super::super::types::OutputFormatStructure::JsonSchema(inner) => {
             #[allow(unused_mut)]
-            let mut object_1 = object_2.key("jsonSchema").start_object();
+            let mut object_1 = object.key("jsonSchema").start_object();
             super::super::protocol_serde::shape_json_schema_definition::ser_json_schema_definition(&mut object_1, inner)?;
             object_1.finish();
         }
```

### `src/protocol_serde/shape_payload_part.rs`

```diff
--- reference/src/protocol_serde/shape_payload_part.rs
+++ generated/src/protocol_serde/shape_payload_part.rs
@@ -1,21 +1,4 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-pub(crate) fn de_payload_part_payload(
-    _value: &[u8],
-) -> ::std::result::Result<super::super::types::PayloadPart, ::aws_smithy_json::deserialize::error::DeserializeError> {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
-    let tokens = &mut tokens_owned;
-    #[allow(unused_variables)]
-    let depth = 0u32;
-    let result = super::super::protocol_serde::shape_payload_part::de_payload_part(tokens, _value, depth + 1)?
-        .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("expected payload member value"));
-    if tokens.next().is_some() {
-        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-            "found more JSON tokens after completing parsing",
-        ));
-    }
-    result
-}
-
 pub(crate) fn de_payload_part<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
```

### `src/protocol_serde/shape_performance_configuration.rs`

```diff
--- reference/src/protocol_serde/shape_performance_configuration.rs
+++ generated/src/protocol_serde/shape_performance_configuration.rs
@@ -1,4 +1,14 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_performance_configuration(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::super::types::PerformanceConfiguration,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    {
+        object.key("latency").string(input.latency.as_str());
+    }
+    Ok(())
+}
+
 pub(crate) fn de_performance_configuration<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
@@ -44,13 +54,3 @@
         )),
     }
 }
-
-pub fn ser_performance_configuration(
-    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
-    input: &super::super::types::PerformanceConfiguration,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    {
-        object.key("latency").string(input.latency.as_str());
-    }
-    Ok(())
-}
```

### `src/protocol_serde/shape_prompt_variable_values.rs`

```diff
--- reference/src/protocol_serde/shape_prompt_variable_values.rs
+++ generated/src/protocol_serde/shape_prompt_variable_values.rs
@@ -1,11 +1,11 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub fn ser_prompt_variable_values(
-    object_21: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::PromptVariableValues,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
     match input {
         super::super::types::PromptVariableValues::Text(inner) => {
-            object_21.key("text").string(inner.as_str());
+            object.key("text").string(inner.as_str());
         }
         super::super::types::PromptVariableValues::Unknown => {
             return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant(
```

### `src/protocol_serde/shape_reasoning_content_block.rs`

```diff
--- reference/src/protocol_serde/shape_reasoning_content_block.rs
+++ generated/src/protocol_serde/shape_reasoning_content_block.rs
@@ -1,19 +1,17 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub fn ser_reasoning_content_block(
-    object_9: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::ReasoningContentBlock,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
     match input {
         super::super::types::ReasoningContentBlock::ReasoningText(inner) => {
             #[allow(unused_mut)]
-            let mut object_1 = object_9.key("reasoningText").start_object();
+            let mut object_1 = object.key("reasoningText").start_object();
             super::super::protocol_serde::shape_reasoning_text_block::ser_reasoning_text_block(&mut object_1, inner)?;
             object_1.finish();
         }
         super::super::types::ReasoningContentBlock::RedactedContent(inner) => {
-            object_9
-                .key("redactedContent")
-                .string_unchecked(&::aws_smithy_types::base64::encode(inner));
+            object.key("redactedContent").string_unchecked(&::aws_smithy_types::base64::encode(inner));
         }
         super::super::types::ReasoningContentBlock::Unknown => {
             return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant(
@@ -44,9 +42,7 @@
             match tokens.next().transpose()? {
                 Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                 Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
-                    if let ::std::option::Option::Some(::std::result::Result::Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) =
-                        tokens.peek()
-                    {
+                    if let Some(Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) = tokens.peek() {
                         let _ = tokens.next().expect("peek returned a token")?;
                         continue;
                     }
```

### `src/protocol_serde/shape_reasoning_content_block_delta.rs`

```diff
--- reference/src/protocol_serde/shape_reasoning_content_block_delta.rs
+++ generated/src/protocol_serde/shape_reasoning_content_block_delta.rs
@@ -19,9 +19,7 @@
             match tokens.next().transpose()? {
                 Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                 Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
-                    if let ::std::option::Option::Some(::std::result::Result::Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) =
-                        tokens.peek()
-                    {
+                    if let Some(Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) = tokens.peek() {
                         let _ = tokens.next().expect("peek returned a token")?;
                         continue;
                     }
```

### `src/protocol_serde/shape_service_tier.rs`

```diff
--- reference/src/protocol_serde/shape_service_tier.rs
+++ generated/src/protocol_serde/shape_service_tier.rs
@@ -1,4 +1,14 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+pub fn ser_service_tier(
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    input: &super::super::types::ServiceTier,
+) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
+    {
+        object.key("type").string(input.r#type.as_str());
+    }
+    Ok(())
+}
+
 pub(crate) fn de_service_tier<'a, I>(
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
@@ -46,13 +56,3 @@
         )),
     }
 }
-
-pub fn ser_service_tier(
-    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
-    input: &super::super::types::ServiceTier,
-) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
-    {
-        object.key("type").string(input.r#type.as_str());
-    }
-    Ok(())
-}
```

### `src/protocol_serde/shape_service_unavailable_exception.rs`

```diff
--- reference/src/protocol_serde/shape_service_unavailable_exception.rs
+++ generated/src/protocol_serde/shape_service_unavailable_exception.rs
@@ -1,38 +1,46 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-pub(crate) fn de_service_unavailable_exception_json_err(
-    _value: &[u8],
-    mut builder: super::super::types::error::builders::ServiceUnavailableExceptionBuilder,
-) -> ::std::result::Result<super::super::types::error::builders::ServiceUnavailableExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
+pub(crate) fn de_service_unavailable_exception<'a, I>(
+    tokens: &mut ::std::iter::Peekable<I>,
+    _value: &'a [u8],
+    depth: u32,
+) -> ::std::result::Result<Option<super::super::types::ServiceUnavailableException>, ::aws_smithy_json::deserialize::error::DeserializeError>
+where
+    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
 {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
-    let tokens = &mut tokens_owned;
-    #[allow(unused_variables)]
-    let depth = 0u32;
-    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
-    loop {
-        match tokens.next().transpose()? {
-            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "message" => {
-                    builder = builder.set_message(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
+    if depth >= 128u32 {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "maximum nesting depth exceeded",
+        ));
+    }
+    match tokens.next().transpose()? {
+        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
+        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
+            #[allow(unused_mut)]
+            let mut builder = super::super::types::builders::ServiceUnavailableExceptionBuilder::default();
+            loop {
+                match tokens.next().transpose()? {
+                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                        "message" => {
+                            builder = builder.set_message(
+                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                    .transpose()?,
+                            );
+                        }
+                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+                    },
+                    other => {
+                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                            "expected object key or end object, found: {other:?}"
+                        )))
+                    }
                 }
-                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
-            },
-            other => {
-                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
-                    "expected object key or end object, found: {other:?}"
-                )))
             }
+            Ok(Some(builder.build()))
         }
+        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "expected start object or null",
+        )),
     }
-    if tokens.next().is_some() {
-        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-            "found more JSON tokens after completing parsing",
-        ));
-    }
-    Ok(builder)
 }
```

### `src/protocol_serde/shape_system_content_block.rs`

```diff
--- reference/src/protocol_serde/shape_system_content_block.rs
+++ generated/src/protocol_serde/shape_system_content_block.rs
@@ -1,23 +1,23 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub fn ser_system_content_block(
-    object_31: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::SystemContentBlock,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
     match input {
         super::super::types::SystemContentBlock::Text(inner) => {
-            object_31.key("text").string(inner.as_str());
+            object.key("text").string(inner.as_str());
         }
         super::super::types::SystemContentBlock::GuardContent(inner) => {
             #[allow(unused_mut)]
-            let mut object_1 = object_31.key("guardContent").start_object();
+            let mut object_1 = object.key("guardContent").start_object();
             super::super::protocol_serde::shape_guardrail_converse_content_block::ser_guardrail_converse_content_block(&mut object_1, inner)?;
             object_1.finish();
         }
         super::super::types::SystemContentBlock::CachePoint(inner) => {
             #[allow(unused_mut)]
-            let mut object_2 = object_31.key("cachePoint").start_object();
-            super::super::protocol_serde::shape_cache_point_block::ser_cache_point_block(&mut object_2, inner)?;
-            object_2.finish();
+            let mut object_1 = object.key("cachePoint").start_object();
+            super::super::protocol_serde::shape_cache_point_block::ser_cache_point_block(&mut object_1, inner)?;
+            object_1.finish();
         }
         super::super::types::SystemContentBlock::Unknown => {
             return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant(
```

### `src/protocol_serde/shape_throttling_exception.rs`

```diff
--- reference/src/protocol_serde/shape_throttling_exception.rs
+++ generated/src/protocol_serde/shape_throttling_exception.rs
@@ -1,37 +1,46 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-pub(crate) fn de_throttling_exception_json_err(
-    _value: &[u8],
-    mut builder: super::super::types::error::builders::ThrottlingExceptionBuilder,
-) -> ::std::result::Result<super::super::types::error::builders::ThrottlingExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
-    let tokens = &mut tokens_owned;
-    #[allow(unused_variables)]
-    let depth = 0u32;
-    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
-    loop {
-        match tokens.next().transpose()? {
-            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "message" => {
-                    builder = builder.set_message(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
+pub(crate) fn de_throttling_exception<'a, I>(
+    tokens: &mut ::std::iter::Peekable<I>,
+    _value: &'a [u8],
+    depth: u32,
+) -> ::std::result::Result<Option<super::super::types::ThrottlingException>, ::aws_smithy_json::deserialize::error::DeserializeError>
+where
+    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
+{
+    if depth >= 128u32 {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "maximum nesting depth exceeded",
+        ));
+    }
+    match tokens.next().transpose()? {
+        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
+        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
+            #[allow(unused_mut)]
+            let mut builder = super::super::types::builders::ThrottlingExceptionBuilder::default();
+            loop {
+                match tokens.next().transpose()? {
+                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                        "message" => {
+                            builder = builder.set_message(
+                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                    .transpose()?,
+                            );
+                        }
+                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+                    },
+                    other => {
+                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                            "expected object key or end object, found: {other:?}"
+                        )))
+                    }
                 }
-                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
-            },
-            other => {
-                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
-                    "expected object key or end object, found: {other:?}"
-                )))
             }
+            Ok(Some(builder.build()))
         }
+        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "expected start object or null",
+        )),
     }
-    if tokens.next().is_some() {
-        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-            "found more JSON tokens after completing parsing",
-        ));
-    }
-    Ok(builder)
 }
```

### `src/protocol_serde/shape_tool.rs`

```diff
--- reference/src/protocol_serde/shape_tool.rs
+++ generated/src/protocol_serde/shape_tool.rs
@@ -1,26 +1,26 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub fn ser_tool(
-    object_3: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::Tool,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
     match input {
         super::super::types::Tool::ToolSpec(inner) => {
             #[allow(unused_mut)]
-            let mut object_1 = object_3.key("toolSpec").start_object();
+            let mut object_1 = object.key("toolSpec").start_object();
             super::super::protocol_serde::shape_tool_specification::ser_tool_specification(&mut object_1, inner)?;
             object_1.finish();
         }
         super::super::types::Tool::SystemTool(inner) => {
             #[allow(unused_mut)]
-            let mut object_2 = object_3.key("systemTool").start_object();
-            super::super::protocol_serde::shape_system_tool::ser_system_tool(&mut object_2, inner)?;
-            object_2.finish();
+            let mut object_1 = object.key("systemTool").start_object();
+            super::super::protocol_serde::shape_system_tool::ser_system_tool(&mut object_1, inner)?;
+            object_1.finish();
         }
         super::super::types::Tool::CachePoint(inner) => {
             #[allow(unused_mut)]
-            let mut object_3 = object_3.key("cachePoint").start_object();
-            super::super::protocol_serde::shape_cache_point_block::ser_cache_point_block(&mut object_3, inner)?;
-            object_3.finish();
+            let mut object_1 = object.key("cachePoint").start_object();
+            super::super::protocol_serde::shape_cache_point_block::ser_cache_point_block(&mut object_1, inner)?;
+            object_1.finish();
         }
         super::super::types::Tool::Unknown => return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant("Tool")),
     }
```

### `src/protocol_serde/shape_tool_choice.rs`

```diff
--- reference/src/protocol_serde/shape_tool_choice.rs
+++ generated/src/protocol_serde/shape_tool_choice.rs
@@ -1,26 +1,26 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub fn ser_tool_choice(
-    object_5: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::ToolChoice,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
     match input {
         super::super::types::ToolChoice::Auto(inner) => {
             #[allow(unused_mut)]
-            let mut object_1 = object_5.key("auto").start_object();
+            let mut object_1 = object.key("auto").start_object();
             super::super::protocol_serde::shape_auto_tool_choice::ser_auto_tool_choice(&mut object_1, inner)?;
             object_1.finish();
         }
         super::super::types::ToolChoice::Any(inner) => {
             #[allow(unused_mut)]
-            let mut object_2 = object_5.key("any").start_object();
-            super::super::protocol_serde::shape_any_tool_choice::ser_any_tool_choice(&mut object_2, inner)?;
-            object_2.finish();
+            let mut object_1 = object.key("any").start_object();
+            super::super::protocol_serde::shape_any_tool_choice::ser_any_tool_choice(&mut object_1, inner)?;
+            object_1.finish();
         }
         super::super::types::ToolChoice::Tool(inner) => {
             #[allow(unused_mut)]
-            let mut object_3 = object_5.key("tool").start_object();
-            super::super::protocol_serde::shape_specific_tool_choice::ser_specific_tool_choice(&mut object_3, inner)?;
-            object_3.finish();
+            let mut object_1 = object.key("tool").start_object();
+            super::super::protocol_serde::shape_specific_tool_choice::ser_specific_tool_choice(&mut object_1, inner)?;
+            object_1.finish();
         }
         super::super::types::ToolChoice::Unknown => return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant("ToolChoice")),
     }
```

### `src/protocol_serde/shape_tool_input_schema.rs`

```diff
--- reference/src/protocol_serde/shape_tool_input_schema.rs
+++ generated/src/protocol_serde/shape_tool_input_schema.rs
@@ -1,11 +1,11 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub fn ser_tool_input_schema(
-    object_3: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::ToolInputSchema,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
     match input {
         super::super::types::ToolInputSchema::Json(inner) => {
-            object_3.key("json").document(inner);
+            object.key("json").document(inner);
         }
         super::super::types::ToolInputSchema::Unknown => {
             return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant(
```

### `src/protocol_serde/shape_tool_result_block_delta.rs`

```diff
--- reference/src/protocol_serde/shape_tool_result_block_delta.rs
+++ generated/src/protocol_serde/shape_tool_result_block_delta.rs
@@ -19,9 +19,7 @@
             match tokens.next().transpose()? {
                 Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                 Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
-                    if let ::std::option::Option::Some(::std::result::Result::Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) =
-                        tokens.peek()
-                    {
+                    if let Some(Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) = tokens.peek() {
                         let _ = tokens.next().expect("peek returned a token")?;
                         continue;
                     }
@@ -43,7 +41,7 @@
                                 .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'text' cannot be null"))?,
                         )),
                         "json" => Some(super::super::types::ToolResultBlockDelta::Json(
-                            Some(::aws_smithy_json::deserialize::token::expect_document(tokens)?)
+                            Some(::aws_smithy_json::deserialize::token::expect_document(tokens.next())?)
                                 .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'json' cannot be null"))?,
                         )),
                         _ => {
```

### `src/protocol_serde/shape_tool_result_content_block.rs`

```diff
--- reference/src/protocol_serde/shape_tool_result_content_block.rs
+++ generated/src/protocol_serde/shape_tool_result_content_block.rs
@@ -1,38 +1,38 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub fn ser_tool_result_content_block(
-    object_3: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::ToolResultContentBlock,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
     match input {
         super::super::types::ToolResultContentBlock::Json(inner) => {
-            object_3.key("json").document(inner);
+            object.key("json").document(inner);
         }
         super::super::types::ToolResultContentBlock::Text(inner) => {
-            object_3.key("text").string(inner.as_str());
+            object.key("text").string(inner.as_str());
         }
         super::super::types::ToolResultContentBlock::Image(inner) => {
             #[allow(unused_mut)]
-            let mut object_1 = object_3.key("image").start_object();
+            let mut object_1 = object.key("image").start_object();
             super::super::protocol_serde::shape_image_block::ser_image_block(&mut object_1, inner)?;
             object_1.finish();
         }
         super::super::types::ToolResultContentBlock::Document(inner) => {
             #[allow(unused_mut)]
-            let mut object_2 = object_3.key("document").start_object();
-            super::super::protocol_serde::shape_document_block::ser_document_block(&mut object_2, inner)?;
-            object_2.finish();
+            let mut object_1 = object.key("document").start_object();
+            super::super::protocol_serde::shape_document_block::ser_document_block(&mut object_1, inner)?;
+            object_1.finish();
         }
         super::super::types::ToolResultContentBlock::Video(inner) => {
             #[allow(unused_mut)]
-            let mut object_3 = object_3.key("video").start_object();
-            super::super::protocol_serde::shape_video_block::ser_video_block(&mut object_3, inner)?;
-            object_3.finish();
+            let mut object_1 = object.key("video").start_object();
+            super::super::protocol_serde::shape_video_block::ser_video_block(&mut object_1, inner)?;
+            object_1.finish();
         }
         super::super::types::ToolResultContentBlock::SearchResult(inner) => {
             #[allow(unused_mut)]
-            let mut object_4 = object_3.key("searchResult").start_object();
-            super::super::protocol_serde::shape_search_result_block::ser_search_result_block(&mut object_4, inner)?;
-            object_4.finish();
+            let mut object_1 = object.key("searchResult").start_object();
+            super::super::protocol_serde::shape_search_result_block::ser_search_result_block(&mut object_1, inner)?;
+            object_1.finish();
         }
         super::super::types::ToolResultContentBlock::Unknown => {
             return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant(
@@ -63,9 +63,7 @@
             match tokens.next().transpose()? {
                 Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                 Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
-                    if let ::std::option::Option::Some(::std::result::Result::Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) =
-                        tokens.peek()
-                    {
+                    if let Some(Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) = tokens.peek() {
                         let _ = tokens.next().expect("peek returned a token")?;
                         continue;
                     }
@@ -81,7 +79,7 @@
                     }
                     variant = match key.as_ref() {
                         "json" => Some(super::super::types::ToolResultContentBlock::Json(
-                            Some(::aws_smithy_json::deserialize::token::expect_document(tokens)?)
+                            Some(::aws_smithy_json::deserialize::token::expect_document(tokens.next())?)
                                 .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'json' cannot be null"))?,
                         )),
                         "text" => Some(super::super::types::ToolResultContentBlock::Text(
```

### `src/protocol_serde/shape_tool_use_block.rs`

```diff
--- reference/src/protocol_serde/shape_tool_use_block.rs
+++ generated/src/protocol_serde/shape_tool_use_block.rs
@@ -55,7 +55,7 @@
                             );
                         }
                         "input" => {
-                            builder = builder.set_input(Some(::aws_smithy_json::deserialize::token::expect_document(tokens)?));
+                            builder = builder.set_input(Some(::aws_smithy_json::deserialize::token::expect_document(tokens.next())?));
                         }
                         "type" => {
                             builder = builder.set_type(
```

### `src/protocol_serde/shape_validation_exception.rs`

```diff
--- reference/src/protocol_serde/shape_validation_exception.rs
+++ generated/src/protocol_serde/shape_validation_exception.rs
@@ -1,37 +1,46 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-pub(crate) fn de_validation_exception_json_err(
-    _value: &[u8],
-    mut builder: super::super::types::error::builders::ValidationExceptionBuilder,
-) -> ::std::result::Result<super::super::types::error::builders::ValidationExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(super::super::protocol_serde::or_empty_doc(_value)).peekable();
-    let tokens = &mut tokens_owned;
-    #[allow(unused_variables)]
-    let depth = 0u32;
-    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
-    loop {
-        match tokens.next().transpose()? {
-            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "message" => {
-                    builder = builder.set_message(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
+pub(crate) fn de_validation_exception<'a, I>(
+    tokens: &mut ::std::iter::Peekable<I>,
+    _value: &'a [u8],
+    depth: u32,
+) -> ::std::result::Result<Option<super::super::types::ValidationException>, ::aws_smithy_json::deserialize::error::DeserializeError>
+where
+    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
+{
+    if depth >= 128u32 {
+        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "maximum nesting depth exceeded",
+        ));
+    }
+    match tokens.next().transpose()? {
+        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
+        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
+            #[allow(unused_mut)]
+            let mut builder = super::super::types::builders::ValidationExceptionBuilder::default();
+            loop {
+                match tokens.next().transpose()? {
+                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
+                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
+                        "message" => {
+                            builder = builder.set_message(
+                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                    .transpose()?,
+                            );
+                        }
+                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
+                    },
+                    other => {
+                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                            "expected object key or end object, found: {other:?}"
+                        )))
+                    }
                 }
-                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
-            },
-            other => {
-                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
-                    "expected object key or end object, found: {other:?}"
-                )))
             }
+            Ok(Some(builder.build()))
         }
+        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+            "expected start object or null",
+        )),
     }
-    if tokens.next().is_some() {
-        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-            "found more JSON tokens after completing parsing",
-        ));
-    }
-    Ok(builder)
 }
```

### `src/protocol_serde/shape_video_source.rs`

```diff
--- reference/src/protocol_serde/shape_video_source.rs
+++ generated/src/protocol_serde/shape_video_source.rs
@@ -1,15 +1,15 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub fn ser_video_source(
-    object_2: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
+    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
     input: &super::super::types::VideoSource,
 ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
     match input {
         super::super::types::VideoSource::Bytes(inner) => {
-            object_2.key("bytes").string_unchecked(&::aws_smithy_types::base64::encode(inner));
+            object.key("bytes").string_unchecked(&::aws_smithy_types::base64::encode(inner));
         }
         super::super::types::VideoSource::S3Location(inner) => {
             #[allow(unused_mut)]
-            let mut object_1 = object_2.key("s3Location").start_object();
+            let mut object_1 = object.key("s3Location").start_object();
             super::super::protocol_serde::shape_s3_location::ser_s3_location(&mut object_1, inner)?;
             object_1.finish();
         }
@@ -38,9 +38,7 @@
             match tokens.next().transpose()? {
                 Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                 Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
-                    if let ::std::option::Option::Some(::std::result::Result::Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) =
-                        tokens.peek()
-                    {
+                    if let Some(Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) = tokens.peek() {
                         let _ = tokens.next().expect("peek returned a token")?;
                         continue;
                     }
```

### `src/protocol_serde.rs`

```diff
--- reference/src/protocol_serde.rs
+++ generated/src/protocol_serde.rs
@@ -37,14 +37,10 @@

 pub(crate) mod shape_invoke_model;

-pub(crate) mod shape_invoke_model_input;
-
 pub(crate) mod shape_invoke_model_with_bidirectional_stream;

 pub(crate) mod shape_invoke_model_with_response_stream;

-pub(crate) mod shape_invoke_model_with_response_stream_input;
-
 pub(crate) mod shape_list_async_invokes;

 pub(crate) mod shape_start_async_invoke;
@@ -75,12 +71,20 @@

 pub(crate) mod shape_invoke_guardrail_checks_input;

+pub(crate) mod shape_invoke_model_input;
+
 pub(crate) mod shape_invoke_model_output;

+pub fn rest_json_unset_union_payload() -> ::std::vec::Vec<u8> {
+    ::std::vec::Vec::new()
+}
+
 pub(crate) mod shape_invoke_model_with_bidirectional_stream_input;

 pub(crate) mod shape_invoke_model_with_bidirectional_stream_output;

+pub(crate) mod shape_invoke_model_with_response_stream_input;
+
 pub(crate) mod shape_invoke_model_with_response_stream_output;

 pub(crate) mod shape_model_error_exception;
@@ -115,10 +119,20 @@

 pub(crate) mod shape_bidirectional_input_payload_part;

+pub(crate) mod shape_bidirectional_output_payload_part;
+
+pub(crate) mod shape_content_block_delta_event;
+
+pub(crate) mod shape_content_block_start_event;
+
+pub(crate) mod shape_content_block_stop_event;
+
 pub(crate) mod shape_converse_metrics;

 pub(crate) mod shape_converse_output;

+pub(crate) mod shape_converse_stream_metadata_event;
+
 pub(crate) mod shape_converse_trace;

 pub(crate) mod shape_guardrail_assessment_list;
@@ -147,8 +161,14 @@

 pub(crate) mod shape_message;

+pub(crate) mod shape_message_start_event;
+
+pub(crate) mod shape_message_stop_event;
+
 pub(crate) mod shape_output_config;

+pub(crate) mod shape_payload_part;
+
 pub(crate) mod shape_performance_configuration;

 pub(crate) mod shape_prompt_variable_values;
@@ -167,8 +187,6 @@

 pub(crate) mod shape_async_invoke_summary;

-pub(crate) mod shape_bidirectional_output_payload_part;
-
 pub(crate) mod shape_cache_details_list;

 pub(crate) mod shape_cache_point_block;
@@ -175,13 +193,13 @@

 pub(crate) mod shape_content_block;

-pub(crate) mod shape_content_block_delta_event;
+pub(crate) mod shape_content_block_delta;

-pub(crate) mod shape_content_block_start_event;
+pub(crate) mod shape_content_block_start;

-pub(crate) mod shape_content_block_stop_event;
+pub(crate) mod shape_converse_stream_metrics;

-pub(crate) mod shape_converse_stream_metadata_event;
+pub(crate) mod shape_converse_stream_trace;

 pub(crate) mod shape_converse_tokens_request;

@@ -223,14 +241,8 @@

 pub(crate) mod shape_invoke_model_tokens_request;

-pub(crate) mod shape_message_start_event;
-
-pub(crate) mod shape_message_stop_event;
-
 pub(crate) mod shape_output_format;

-pub(crate) mod shape_payload_part;
-
 pub(crate) mod shape_prompt_router_trace;

 pub(crate) mod shape_tool;
@@ -249,6 +261,8 @@

 pub(crate) mod shape_citations_content_block;

+pub(crate) mod shape_citations_delta;
+
 pub(crate) mod shape_content_blocks;

 pub(crate) mod shape_document_block;
@@ -291,6 +305,10 @@

 pub(crate) mod shape_image_block;

+pub(crate) mod shape_image_block_delta;
+
+pub(crate) mod shape_image_block_start;
+
 pub(crate) mod shape_model_outputs;

 pub(crate) mod shape_output_format_structure;
@@ -297,6 +315,8 @@

 pub(crate) mod shape_reasoning_content_block;

+pub(crate) mod shape_reasoning_content_block_delta;
+
 pub(crate) mod shape_search_result_block;

 pub(crate) mod shape_specific_tool_choice;
@@ -309,10 +329,18 @@

 pub(crate) mod shape_tool_result_block;

+pub(crate) mod shape_tool_result_block_start;
+
+pub(crate) mod shape_tool_result_blocks_delta;
+
 pub(crate) mod shape_tool_specification;

 pub(crate) mod shape_tool_use_block;

+pub(crate) mod shape_tool_use_block_delta;
+
+pub(crate) mod shape_tool_use_block_start;
+
 pub(crate) mod shape_video_block;

 pub(crate) mod shape_audio_source;
@@ -321,15 +349,11 @@

 pub(crate) mod shape_citation_generated_content;

-pub(crate) mod shape_citations_config;
-
-pub(crate) mod shape_content_block_delta;
+pub(crate) mod shape_citation_location;

-pub(crate) mod shape_content_block_start;
+pub(crate) mod shape_citation_source_content_list_delta;

-pub(crate) mod shape_converse_stream_metrics;
-
-pub(crate) mod shape_converse_stream_trace;
+pub(crate) mod shape_citations_config;

 pub(crate) mod shape_document_source;

@@ -373,18 +397,24 @@

 pub(crate) mod shape_tool_reference;

+pub(crate) mod shape_tool_result_block_delta;
+
 pub(crate) mod shape_tool_result_content_block;

 pub(crate) mod shape_video_source;

-pub(crate) mod shape_citation_location;
+pub(crate) mod shape_citation_source_content;

-pub(crate) mod shape_citation_source_content;
+pub(crate) mod shape_citation_source_content_delta;
+
+pub(crate) mod shape_document_char_location;

-pub(crate) mod shape_citations_delta;
+pub(crate) mod shape_document_chunk_location;

 pub(crate) mod shape_document_content_block;

+pub(crate) mod shape_document_page_location;
+
 pub(crate) mod shape_guardrail_automated_reasoning_finding;

 pub(crate) mod shape_guardrail_content_filter;
@@ -401,34 +431,16 @@

 pub(crate) mod shape_guardrail_topic;

-pub(crate) mod shape_image_block_delta;
-
-pub(crate) mod shape_image_block_start;
-
-pub(crate) mod shape_reasoning_content_block_delta;
-
 pub(crate) mod shape_s3_location;

-pub(crate) mod shape_tool_result_block_start;
-
-pub(crate) mod shape_tool_result_blocks_delta;
+pub(crate) mod shape_search_result_location;

-pub(crate) mod shape_tool_use_block_delta;
+pub(crate) mod shape_web_location;

-pub(crate) mod shape_tool_use_block_start;
-
 pub(crate) mod shape_citation_generated_content_list;

-pub(crate) mod shape_citation_source_content_list_delta;
-
 pub(crate) mod shape_citations;

-pub(crate) mod shape_document_char_location;
-
-pub(crate) mod shape_document_chunk_location;
-
-pub(crate) mod shape_document_page_location;
-
 pub(crate) mod shape_guardrail_automated_reasoning_impossible_finding;

 pub(crate) mod shape_guardrail_automated_reasoning_invalid_finding;
@@ -445,16 +457,8 @@

 pub(crate) mod shape_search_result_content_blocks;

-pub(crate) mod shape_search_result_location;
-
-pub(crate) mod shape_tool_result_block_delta;
-
 pub(crate) mod shape_tool_result_content_blocks;

-pub(crate) mod shape_web_location;
-
-pub(crate) mod shape_citation_source_content_delta;
-
 pub(crate) mod shape_document_content_blocks;

 pub(crate) mod shape_guardrail_automated_reasoning_difference_scenario_list;
```

### `src/serde_util.rs`

```diff
--- reference/src/serde_util.rs
+++ generated/src/serde_util.rs
@@ -24,7 +24,7 @@
     mut builder: super::operation::converse::builders::ConverseOutputBuilder,
 ) -> super::operation::converse::builders::ConverseOutputBuilder {
     if builder.output.is_none() {
-        builder.output = Some(super::types::ConverseOutput::Unknown)
+        builder.output = Some(Default::default())
     }
     if builder.stop_reason.is_none() {
         builder.stop_reason = "no value was set".parse::<super::types::StopReason>().ok()
@@ -69,7 +69,7 @@
         builder.submit_time = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
     }
     if builder.output_data_config.is_none() {
-        builder.output_data_config = Some(super::types::AsyncInvokeOutputDataConfig::Unknown)
+        builder.output_data_config = Some(Default::default())
     }
     builder
 }
@@ -96,7 +96,7 @@
     mut builder: super::operation::invoke_model::builders::InvokeModelOutputBuilder,
 ) -> super::operation::invoke_model::builders::InvokeModelOutputBuilder {
     if builder.body.is_none() {
-        builder.body = Some(::aws_smithy_types::Blob::new(""))
+        builder.body = Some(Default::default())
     }
     if builder.content_type.is_none() {
         builder.content_type = Some(Default::default())
@@ -104,9 +104,21 @@
     builder
 }

+pub(crate) fn invoke_model_with_bidirectional_stream_output_output_correct_errors(
+    mut builder: super::operation::invoke_model_with_bidirectional_stream::builders::InvokeModelWithBidirectionalStreamOutputBuilder,
+) -> super::operation::invoke_model_with_bidirectional_stream::builders::InvokeModelWithBidirectionalStreamOutputBuilder {
+    if builder.body.is_none() {
+        builder.body = Some(Default::default())
+    }
+    builder
+}
+
 pub(crate) fn invoke_model_with_response_stream_output_output_correct_errors(
     mut builder: super::operation::invoke_model_with_response_stream::builders::InvokeModelWithResponseStreamOutputBuilder,
 ) -> super::operation::invoke_model_with_response_stream::builders::InvokeModelWithResponseStreamOutputBuilder {
+    if builder.body.is_none() {
+        builder.body = Some(Default::default())
+    }
     if builder.content_type.is_none() {
         builder.content_type = Some(Default::default())
     }
@@ -197,11 +209,62 @@
         builder.submit_time = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
     }
     if builder.output_data_config.is_none() {
-        builder.output_data_config = Some(super::types::AsyncInvokeOutputDataConfig::Unknown)
+        builder.output_data_config = Some(Default::default())
+    }
+    builder
+}
+
+pub(crate) fn content_block_delta_event_correct_errors(
+    mut builder: super::types::builders::ContentBlockDeltaEventBuilder,
+) -> super::types::builders::ContentBlockDeltaEventBuilder {
+    if builder.delta.is_none() {
+        builder.delta = Some(Default::default())
+    }
+    if builder.content_block_index.is_none() {
+        builder.content_block_index = Some(Default::default())
     }
     builder
 }

+pub(crate) fn content_block_start_event_correct_errors(
+    mut builder: super::types::builders::ContentBlockStartEventBuilder,
+) -> super::types::builders::ContentBlockStartEventBuilder {
+    if builder.start.is_none() {
+        builder.start = Some(Default::default())
+    }
+    if builder.content_block_index.is_none() {
+        builder.content_block_index = Some(Default::default())
+    }
+    builder
+}
+
+pub(crate) fn content_block_stop_event_correct_errors(
+    mut builder: super::types::builders::ContentBlockStopEventBuilder,
+) -> super::types::builders::ContentBlockStopEventBuilder {
+    if builder.content_block_index.is_none() {
+        builder.content_block_index = Some(Default::default())
+    }
+    builder
+}
+
+pub(crate) fn converse_stream_metadata_event_correct_errors(
+    mut builder: super::types::builders::ConverseStreamMetadataEventBuilder,
+) -> super::types::builders::ConverseStreamMetadataEventBuilder {
+    if builder.usage.is_none() {
+        builder.usage = {
+            let builder = super::types::builders::TokenUsageBuilder::default();
+            super::serde_util::token_usage_correct_errors(builder).build().ok()
+        }
+    }
+    if builder.metrics.is_none() {
+        builder.metrics = {
+            let builder = super::types::builders::ConverseStreamMetricsBuilder::default();
+            super::serde_util::converse_stream_metrics_correct_errors(builder).build().ok()
+        }
+    }
+    builder
+}
+
 pub(crate) fn guardrail_checks_content_filter_result_correct_errors(
     mut builder: super::types::builders::GuardrailChecksContentFilterResultBuilder,
 ) -> super::types::builders::GuardrailChecksContentFilterResultBuilder {
@@ -266,63 +329,39 @@
     builder
 }

-pub(crate) fn cache_detail_correct_errors(mut builder: super::types::builders::CacheDetailBuilder) -> super::types::builders::CacheDetailBuilder {
-    if builder.ttl.is_none() {
-        builder.ttl = "no value was set".parse::<super::types::CacheTtl>().ok()
-    }
-    if builder.input_tokens.is_none() {
-        builder.input_tokens = Some(Default::default())
+pub(crate) fn message_start_event_correct_errors(
+    mut builder: super::types::builders::MessageStartEventBuilder,
+) -> super::types::builders::MessageStartEventBuilder {
+    if builder.role.is_none() {
+        builder.role = "no value was set".parse::<super::types::ConversationRole>().ok()
     }
     builder
 }

-pub(crate) fn content_block_delta_event_correct_errors(
-    mut builder: super::types::builders::ContentBlockDeltaEventBuilder,
-) -> super::types::builders::ContentBlockDeltaEventBuilder {
-    if builder.delta.is_none() {
-        builder.delta = Some(super::types::ContentBlockDelta::Unknown)
-    }
-    if builder.content_block_index.is_none() {
-        builder.content_block_index = Some(Default::default())
+pub(crate) fn message_stop_event_correct_errors(
+    mut builder: super::types::builders::MessageStopEventBuilder,
+) -> super::types::builders::MessageStopEventBuilder {
+    if builder.stop_reason.is_none() {
+        builder.stop_reason = "no value was set".parse::<super::types::StopReason>().ok()
     }
     builder
 }

-pub(crate) fn content_block_start_event_correct_errors(
-    mut builder: super::types::builders::ContentBlockStartEventBuilder,
-) -> super::types::builders::ContentBlockStartEventBuilder {
-    if builder.start.is_none() {
-        builder.start = Some(super::types::ContentBlockStart::Unknown)
-    }
-    if builder.content_block_index.is_none() {
-        builder.content_block_index = Some(Default::default())
+pub(crate) fn cache_detail_correct_errors(mut builder: super::types::builders::CacheDetailBuilder) -> super::types::builders::CacheDetailBuilder {
+    if builder.ttl.is_none() {
+        builder.ttl = "no value was set".parse::<super::types::CacheTtl>().ok()
     }
-    builder
-}
-
-pub(crate) fn content_block_stop_event_correct_errors(
-    mut builder: super::types::builders::ContentBlockStopEventBuilder,
-) -> super::types::builders::ContentBlockStopEventBuilder {
-    if builder.content_block_index.is_none() {
-        builder.content_block_index = Some(Default::default())
+    if builder.input_tokens.is_none() {
+        builder.input_tokens = Some(Default::default())
     }
     builder
 }

-pub(crate) fn converse_stream_metadata_event_correct_errors(
-    mut builder: super::types::builders::ConverseStreamMetadataEventBuilder,
-) -> super::types::builders::ConverseStreamMetadataEventBuilder {
-    if builder.usage.is_none() {
-        builder.usage = {
-            let builder = super::types::builders::TokenUsageBuilder::default();
-            super::serde_util::token_usage_correct_errors(builder).build().ok()
-        }
-    }
-    if builder.metrics.is_none() {
-        builder.metrics = {
-            let builder = super::types::builders::ConverseStreamMetricsBuilder::default();
-            super::serde_util::converse_stream_metrics_correct_errors(builder).build().ok()
-        }
+pub(crate) fn converse_stream_metrics_correct_errors(
+    mut builder: super::types::builders::ConverseStreamMetricsBuilder,
+) -> super::types::builders::ConverseStreamMetricsBuilder {
+    if builder.latency_ms.is_none() {
+        builder.latency_ms = Some(Default::default())
     }
     builder
 }
@@ -369,33 +408,6 @@
     builder
 }

-pub(crate) fn message_start_event_correct_errors(
-    mut builder: super::types::builders::MessageStartEventBuilder,
-) -> super::types::builders::MessageStartEventBuilder {
-    if builder.role.is_none() {
-        builder.role = "no value was set".parse::<super::types::ConversationRole>().ok()
-    }
-    builder
-}
-
-pub(crate) fn message_stop_event_correct_errors(
-    mut builder: super::types::builders::MessageStopEventBuilder,
-) -> super::types::builders::MessageStopEventBuilder {
-    if builder.stop_reason.is_none() {
-        builder.stop_reason = "no value was set".parse::<super::types::StopReason>().ok()
-    }
-    builder
-}
-
-pub(crate) fn converse_stream_metrics_correct_errors(
-    mut builder: super::types::builders::ConverseStreamMetricsBuilder,
-) -> super::types::builders::ConverseStreamMetricsBuilder {
-    if builder.latency_ms.is_none() {
-        builder.latency_ms = Some(Default::default())
-    }
-    builder
-}
-
 pub(crate) fn guardrail_checks_content_filter_result_entry_correct_errors(
     mut builder: super::types::builders::GuardrailChecksContentFilterResultEntryBuilder,
 ) -> super::types::builders::GuardrailChecksContentFilterResultEntryBuilder {
@@ -446,12 +458,51 @@
     builder
 }

+pub(crate) fn image_block_start_correct_errors(
+    mut builder: super::types::builders::ImageBlockStartBuilder,
+) -> super::types::builders::ImageBlockStartBuilder {
+    if builder.format.is_none() {
+        builder.format = "no value was set".parse::<super::types::ImageFormat>().ok()
+    }
+    builder
+}
+
+pub(crate) fn tool_result_block_start_correct_errors(
+    mut builder: super::types::builders::ToolResultBlockStartBuilder,
+) -> super::types::builders::ToolResultBlockStartBuilder {
+    if builder.tool_use_id.is_none() {
+        builder.tool_use_id = Some(Default::default())
+    }
+    builder
+}
+
+pub(crate) fn tool_use_block_delta_correct_errors(
+    mut builder: super::types::builders::ToolUseBlockDeltaBuilder,
+) -> super::types::builders::ToolUseBlockDeltaBuilder {
+    if builder.input.is_none() {
+        builder.input = Some(Default::default())
+    }
+    builder
+}
+
+pub(crate) fn tool_use_block_start_correct_errors(
+    mut builder: super::types::builders::ToolUseBlockStartBuilder,
+) -> super::types::builders::ToolUseBlockStartBuilder {
+    if builder.tool_use_id.is_none() {
+        builder.tool_use_id = Some(Default::default())
+    }
+    if builder.name.is_none() {
+        builder.name = Some(Default::default())
+    }
+    builder
+}
+
 pub(crate) fn audio_block_correct_errors(mut builder: super::types::builders::AudioBlockBuilder) -> super::types::builders::AudioBlockBuilder {
     if builder.format.is_none() {
         builder.format = "no value was set".parse::<super::types::AudioFormat>().ok()
     }
     if builder.source.is_none() {
-        builder.source = Some(super::types::AudioSource::Unknown)
+        builder.source = Some(Default::default())
     }
     builder
 }
@@ -472,7 +523,7 @@
         builder.name = Some(Default::default())
     }
     if builder.source.is_none() {
-        builder.source = Some(super::types::DocumentSource::Unknown)
+        builder.source = Some(Default::default())
     }
     builder
 }
@@ -581,16 +632,7 @@
         builder.format = "no value was set".parse::<super::types::ImageFormat>().ok()
     }
     if builder.source.is_none() {
-        builder.source = Some(super::types::ImageSource::Unknown)
-    }
-    builder
-}
-
-pub(crate) fn image_block_start_correct_errors(
-    mut builder: super::types::builders::ImageBlockStartBuilder,
-) -> super::types::builders::ImageBlockStartBuilder {
-    if builder.format.is_none() {
-        builder.format = "no value was set".parse::<super::types::ImageFormat>().ok()
+        builder.source = Some(Default::default())
     }
     builder
 }
@@ -646,15 +688,6 @@
     builder
 }

-pub(crate) fn tool_result_block_start_correct_errors(
-    mut builder: super::types::builders::ToolResultBlockStartBuilder,
-) -> super::types::builders::ToolResultBlockStartBuilder {
-    if builder.tool_use_id.is_none() {
-        builder.tool_use_id = Some(Default::default())
-    }
-    builder
-}
-
 pub(crate) fn tool_use_block_correct_errors(mut builder: super::types::builders::ToolUseBlockBuilder) -> super::types::builders::ToolUseBlockBuilder {
     if builder.tool_use_id.is_none() {
         builder.tool_use_id = Some(Default::default())
@@ -668,33 +701,12 @@
     builder
 }

-pub(crate) fn tool_use_block_delta_correct_errors(
-    mut builder: super::types::builders::ToolUseBlockDeltaBuilder,
-) -> super::types::builders::ToolUseBlockDeltaBuilder {
-    if builder.input.is_none() {
-        builder.input = Some(Default::default())
-    }
-    builder
-}
-
-pub(crate) fn tool_use_block_start_correct_errors(
-    mut builder: super::types::builders::ToolUseBlockStartBuilder,
-) -> super::types::builders::ToolUseBlockStartBuilder {
-    if builder.tool_use_id.is_none() {
-        builder.tool_use_id = Some(Default::default())
-    }
-    if builder.name.is_none() {
-        builder.name = Some(Default::default())
-    }
-    builder
-}
-
 pub(crate) fn video_block_correct_errors(mut builder: super::types::builders::VideoBlockBuilder) -> super::types::builders::VideoBlockBuilder {
     if builder.format.is_none() {
         builder.format = "no value was set".parse::<super::types::VideoFormat>().ok()
     }
     if builder.source.is_none() {
-        builder.source = Some(super::types::VideoSource::Unknown)
+        builder.source = Some(Default::default())
     }
     builder
 }
@@ -715,7 +727,7 @@
         builder.format = "no value was set".parse::<super::types::GuardrailConverseImageFormat>().ok()
     }
     if builder.source.is_none() {
-        builder.source = Some(super::types::GuardrailConverseImageSource::Unknown)
+        builder.source = Some(Default::default())
     }
     builder
 }
```

### `src/types/_audio_source.rs`

```diff
--- reference/src/types/_audio_source.rs
+++ generated/src/types/_audio_source.rs
@@ -2,7 +2,7 @@

 /// <p>The source of audio data, which can be provided either as raw bytes or a reference to an S3 location.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub enum AudioSource {
     /// <p>Audio data encoded in base64.</p>
     Bytes(::aws_smithy_types::Blob),
@@ -50,8 +50,3 @@
         matches!(self, Self::Unknown)
     }
 }
-impl ::std::fmt::Debug for AudioSource {
-    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
-        ::std::write!(f, "*** Sensitive Data Redacted ***")
-    }
-}
```

### `src/types/_cache_detail.rs`

```diff
--- reference/src/types/_cache_detail.rs
+++ generated/src/types/_cache_detail.rs
@@ -15,8 +15,8 @@
         &self.ttl
     }
     /// <p>Number of tokens written to cache with this TTL (cache creation tokens)</p>
-    pub fn input_tokens(&self) -> i32 {
-        self.input_tokens
+    pub fn input_tokens(&self) -> &i32 {
+        &self.input_tokens
     }
 }
 impl CacheDetail {
```

### `src/types/_cache_ttl.rs`

```diff
--- reference/src/types/_cache_ttl.rs
+++ generated/src/types/_cache_ttl.rs
@@ -12,8 +12,8 @@
 /// ```text
 /// # let cachettl = unimplemented!();
 /// match cachettl {
+///     CacheTtl::FiveMinutes => { /* ... */ },
 ///     CacheTtl::OneHour => { /* ... */ },
-///     CacheTtl::FiveMinutes => { /* ... */ },
 ///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
 ///     _ => { /* ... */ },
 /// }
@@ -43,9 +43,9 @@
 )]
 pub enum CacheTtl {
     #[allow(missing_docs)] // documentation missing in model
-    OneHour,
-    #[allow(missing_docs)] // documentation missing in model
     FiveMinutes,
+    #[allow(missing_docs)] // documentation missing in model
+    OneHour,
     /// `Unknown` contains new variants that have been added since this code was generated.
     #[deprecated(note = "Don't directly match on `Unknown`. See the docs on this enum for the correct way to handle unknown variants.")]
     Unknown(super::super::primitives::sealed_enum_unknown::UnknownVariantValue),
@@ -53,8 +53,8 @@
 impl ::std::convert::From<&str> for CacheTtl {
     fn from(s: &str) -> Self {
         match s {
+            "5m" => CacheTtl::FiveMinutes,
             "1h" => CacheTtl::OneHour,
-            "5m" => CacheTtl::FiveMinutes,
             other => CacheTtl::Unknown(super::super::primitives::sealed_enum_unknown::UnknownVariantValue(other.to_owned())),
         }
     }
@@ -70,14 +70,14 @@
     /// Returns the `&str` value of the enum member.
     pub fn as_str(&self) -> &str {
         match self {
+            CacheTtl::FiveMinutes => "5m",
             CacheTtl::OneHour => "1h",
-            CacheTtl::FiveMinutes => "5m",
             CacheTtl::Unknown(value) => value.as_str(),
         }
     }
     /// Returns all the `&str` representations of the enum members.
     pub const fn values() -> &'static [&'static str] {
-        &["1h", "5m"]
+        &["5m", "1h"]
     }
 }
 impl ::std::convert::AsRef<str> for CacheTtl {
@@ -100,8 +100,8 @@
 impl ::std::fmt::Display for CacheTtl {
     fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
         match self {
-            CacheTtl::OneHour => write!(f, "1h"),
             CacheTtl::FiveMinutes => write!(f, "5m"),
+            CacheTtl::OneHour => write!(f, "1h"),
             CacheTtl::Unknown(value) => write!(f, "{value}"),
         }
     }
```

### `src/types/_citations_config.rs`

```diff
--- reference/src/types/_citations_config.rs
+++ generated/src/types/_citations_config.rs
@@ -9,8 +9,8 @@
 }
 impl CitationsConfig {
     /// <p>Specifies whether citations from the selected document should be used in the model's response. When set to true, the model can generate citations that reference the source documents used to inform the response.</p>
-    pub fn enabled(&self) -> bool {
-        self.enabled
+    pub fn enabled(&self) -> &bool {
+        &self.enabled
     }
 }
 impl CitationsConfig {
```

### `src/types/_content_block.rs`

```diff
--- reference/src/types/_content_block.rs
+++ generated/src/types/_content_block.rs
@@ -2,7 +2,7 @@

 /// <p>A block of content for a message that you pass to, or receive from, a model with the <a href="https://docs.aws.amazon.com/bedrock/latest/APIReference/API_runtime_Converse.html">Converse</a> or <a href="https://docs.aws.amazon.com/bedrock/latest/APIReference/API_runtime_ConverseStream.html">ConverseStream</a> API operations.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub enum ContentBlock {
     /// <p>An audio content block containing audio data in the conversation.</p>
     Audio(super::super::types::AudioBlock),
@@ -233,24 +233,3 @@
         matches!(self, Self::Unknown)
     }
 }
-impl ::std::fmt::Debug for ContentBlock {
-    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
-        match self {
-            ContentBlock::Audio(val) => f.debug_tuple("Audio").field(&val).finish(),
-            ContentBlock::CachePoint(val) => f.debug_tuple("CachePoint").field(&val).finish(),
-            ContentBlock::CitationsContent(val) => f.debug_tuple("CitationsContent").field(&val).finish(),
-            ContentBlock::Document(val) => f.debug_tuple("Document").field(&val).finish(),
-            ContentBlock::GuardContent(val) => f.debug_tuple("GuardContent").field(&val).finish(),
-            ContentBlock::Image(val) => f.debug_tuple("Image").field(&val).finish(),
-            ContentBlock::ReasoningContent(_) => f.debug_tuple("*** Sensitive Data Redacted ***").finish(),
-            ContentBlock::SearchResult(val) => f.debug_tuple("SearchResult").field(&val).finish(),
-            ContentBlock::Text(val) => f.debug_tuple("Text").field(&val).finish(),
-            ContentBlock::ToolAddition(val) => f.debug_tuple("ToolAddition").field(&val).finish(),
-            ContentBlock::ToolRemoval(val) => f.debug_tuple("ToolRemoval").field(&val).finish(),
-            ContentBlock::ToolResult(val) => f.debug_tuple("ToolResult").field(&val).finish(),
-            ContentBlock::ToolUse(val) => f.debug_tuple("ToolUse").field(&val).finish(),
-            ContentBlock::Video(val) => f.debug_tuple("Video").field(&val).finish(),
-            ContentBlock::Unknown => f.debug_tuple("Unknown").finish(),
-        }
-    }
-}
```

### `src/types/_content_block_delta.rs`

```diff
--- reference/src/types/_content_block_delta.rs
+++ generated/src/types/_content_block_delta.rs
@@ -2,7 +2,7 @@

 /// <p>A block of content in a streaming response.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub enum ContentBlockDelta {
     /// <p>Incremental citation information that is streamed as part of the response generation process.</p>
     Citation(super::super::types::CitationsDelta),
@@ -79,7 +79,7 @@
     pub fn is_text(&self) -> bool {
         self.as_text().is_ok()
     }
-    /// Tries to convert the enum instance into [`ToolResult`](crate::types::ContentBlockDelta::ToolResult), extracting the inner [`Vec`](::std::vec::Vec).
+    /// Tries to convert the enum instance into [`ToolResult`](crate::types::ContentBlockDelta::ToolResult), extracting the inner [`Vec::<ToolResultBlockDelta>`](::std::vec::Vec<crate::types::ToolResultBlockDelta>).
     /// Returns `Err(&Self)` if it can't be converted.
     pub fn as_tool_result(&self) -> ::std::result::Result<&::std::vec::Vec<super::super::types::ToolResultBlockDelta>, &Self> {
         if let ContentBlockDelta::ToolResult(val) = &self {
@@ -110,16 +110,3 @@
         matches!(self, Self::Unknown)
     }
 }
-impl ::std::fmt::Debug for ContentBlockDelta {
-    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
-        match self {
-            ContentBlockDelta::Citation(val) => f.debug_tuple("Citation").field(&val).finish(),
-            ContentBlockDelta::Image(val) => f.debug_tuple("Image").field(&val).finish(),
-            ContentBlockDelta::ReasoningContent(_) => f.debug_tuple("*** Sensitive Data Redacted ***").finish(),
-            ContentBlockDelta::Text(val) => f.debug_tuple("Text").field(&val).finish(),
-            ContentBlockDelta::ToolResult(val) => f.debug_tuple("ToolResult").field(&val).finish(),
-            ContentBlockDelta::ToolUse(val) => f.debug_tuple("ToolUse").field(&val).finish(),
-            ContentBlockDelta::Unknown => f.debug_tuple("Unknown").finish(),
-        }
-    }
-}
```

### `src/types/_converse_metrics.rs`

```diff
--- reference/src/types/_converse_metrics.rs
+++ generated/src/types/_converse_metrics.rs
@@ -9,8 +9,8 @@
 }
 impl ConverseMetrics {
     /// <p>The latency of the call to <code>Converse</code>, in milliseconds.</p>
-    pub fn latency_ms(&self) -> i64 {
-        self.latency_ms
+    pub fn latency_ms(&self) -> &i64 {
+        &self.latency_ms
     }
 }
 impl ConverseMetrics {
```

### `src/types/_converse_stream_metrics.rs`

```diff
--- reference/src/types/_converse_stream_metrics.rs
+++ generated/src/types/_converse_stream_metrics.rs
@@ -9,8 +9,8 @@
 }
 impl ConverseStreamMetrics {
     /// <p>The latency for the streaming request, in milliseconds.</p>
-    pub fn latency_ms(&self) -> i64 {
-        self.latency_ms
+    pub fn latency_ms(&self) -> &i64 {
+        &self.latency_ms
     }
 }
 impl ConverseStreamMetrics {
```

### `src/types/_document_block.rs`

```diff
--- reference/src/types/_document_block.rs
+++ generated/src/types/_document_block.rs
@@ -203,9 +203,7 @@
     /// - [`name`](crate::types::builders::DocumentBlockBuilder::name)
     pub fn build(self) -> ::std::result::Result<super::super::types::DocumentBlock, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(super::super::types::DocumentBlock {
-            format: self
-                .format
-                .unwrap_or("txt".parse::<super::super::types::DocumentFormat>().expect("static value validated to member")),
+            format: self.format.unwrap_or_default(),
             name: self.name.ok_or_else(|| {
                 ::aws_smithy_types::error::operation::BuildError::missing_field(
                     "name",
```

### `src/types/_document_char_location.rs`

```diff
--- reference/src/types/_document_char_location.rs
+++ generated/src/types/_document_char_location.rs
@@ -13,16 +13,16 @@
 }
 impl DocumentCharLocation {
     /// <p>The index of the document within the array of documents provided in the request.</p>
-    pub fn document_index(&self) -> ::std::option::Option<i32> {
-        self.document_index
+    pub fn document_index(&self) -> ::std::option::Option<&i32> {
+        self.document_index.as_ref()
     }
     /// <p>The starting character position of the cited content within the document.</p>
-    pub fn start(&self) -> ::std::option::Option<i32> {
-        self.start
+    pub fn start(&self) -> ::std::option::Option<&i32> {
+        self.start.as_ref()
     }
     /// <p>The ending character position of the cited content within the document.</p>
-    pub fn end(&self) -> ::std::option::Option<i32> {
-        self.end
+    pub fn end(&self) -> ::std::option::Option<&i32> {
+        self.end.as_ref()
     }
 }
 impl DocumentCharLocation {
```

### `src/types/_document_chunk_location.rs`

```diff
--- reference/src/types/_document_chunk_location.rs
+++ generated/src/types/_document_chunk_location.rs
@@ -13,16 +13,16 @@
 }
 impl DocumentChunkLocation {
     /// <p>The index of the document within the array of documents provided in the request.</p>
-    pub fn document_index(&self) -> ::std::option::Option<i32> {
-        self.document_index
+    pub fn document_index(&self) -> ::std::option::Option<&i32> {
+        self.document_index.as_ref()
     }
     /// <p>The starting chunk identifier or index of the cited content within the document.</p>
-    pub fn start(&self) -> ::std::option::Option<i32> {
-        self.start
+    pub fn start(&self) -> ::std::option::Option<&i32> {
+        self.start.as_ref()
     }
     /// <p>The ending chunk identifier or index of the cited content within the document.</p>
-    pub fn end(&self) -> ::std::option::Option<i32> {
-        self.end
+    pub fn end(&self) -> ::std::option::Option<&i32> {
+        self.end.as_ref()
     }
 }
 impl DocumentChunkLocation {
```

### `src/types/_document_page_location.rs`

```diff
--- reference/src/types/_document_page_location.rs
+++ generated/src/types/_document_page_location.rs
@@ -13,16 +13,16 @@
 }
 impl DocumentPageLocation {
     /// <p>The index of the document within the array of documents provided in the request.</p>
-    pub fn document_index(&self) -> ::std::option::Option<i32> {
-        self.document_index
+    pub fn document_index(&self) -> ::std::option::Option<&i32> {
+        self.document_index.as_ref()
     }
     /// <p>The starting page number of the cited content within the document.</p>
-    pub fn start(&self) -> ::std::option::Option<i32> {
-        self.start
+    pub fn start(&self) -> ::std::option::Option<&i32> {
+        self.start.as_ref()
     }
     /// <p>The ending page number of the cited content within the document.</p>
-    pub fn end(&self) -> ::std::option::Option<i32> {
-        self.end
+    pub fn end(&self) -> ::std::option::Option<&i32> {
+        self.end.as_ref()
     }
 }
 impl DocumentPageLocation {
```

### `src/types/_document_source.rs`

```diff
--- reference/src/types/_document_source.rs
+++ generated/src/types/_document_source.rs
@@ -36,7 +36,7 @@
     pub fn is_bytes(&self) -> bool {
         self.as_bytes().is_ok()
     }
-    /// Tries to convert the enum instance into [`Content`](crate::types::DocumentSource::Content), extracting the inner [`Vec`](::std::vec::Vec).
+    /// Tries to convert the enum instance into [`Content`](crate::types::DocumentSource::Content), extracting the inner [`Vec::<DocumentContentBlock>`](::std::vec::Vec<crate::types::DocumentContentBlock>).
     /// Returns `Err(&Self)` if it can't be converted.
     pub fn as_content(&self) -> ::std::result::Result<&::std::vec::Vec<super::super::types::DocumentContentBlock>, &Self> {
         if let DocumentSource::Content(val) = &self {
```

### `src/types/_guardrail_checks_content_block.rs`

```diff
--- reference/src/types/_guardrail_checks_content_block.rs
+++ generated/src/types/_guardrail_checks_content_block.rs
@@ -2,7 +2,7 @@

 /// <p>A content block within a message to evaluate.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub enum GuardrailChecksContentBlock {
     /// <p>The text content to evaluate.</p>
     Text(::std::string::String),
@@ -36,11 +36,3 @@
         matches!(self, Self::Unknown)
     }
 }
-impl ::std::fmt::Debug for GuardrailChecksContentBlock {
-    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
-        match self {
-            GuardrailChecksContentBlock::Text(_) => f.debug_tuple("*** Sensitive Data Redacted ***").finish(),
-            GuardrailChecksContentBlock::Unknown => f.debug_tuple("Unknown").finish(),
-        }
-    }
-}
```

### `src/types/_guardrail_checks_content_filter_result_entry.rs`

```diff
--- reference/src/types/_guardrail_checks_content_filter_result_entry.rs
+++ generated/src/types/_guardrail_checks_content_filter_result_entry.rs
@@ -15,8 +15,8 @@
         &self.category
     }
     /// <p>The severity score for the category, ranging from 0.0 to 1.0. Higher values indicate greater severity.</p>
-    pub fn severity_score(&self) -> f64 {
-        self.severity_score
+    pub fn severity_score(&self) -> &f64 {
+        &self.severity_score
     }
 }
 impl GuardrailChecksContentFilterResultEntry {
```

### `src/types/_guardrail_checks_content_filter_usage.rs`

```diff
--- reference/src/types/_guardrail_checks_content_filter_usage.rs
+++ generated/src/types/_guardrail_checks_content_filter_usage.rs
@@ -9,8 +9,8 @@
 }
 impl GuardrailChecksContentFilterUsage {
     /// <p>The number of text units consumed by the content filter check.</p>
-    pub fn text_units(&self) -> i32 {
-        self.text_units
+    pub fn text_units(&self) -> &i32 {
+        &self.text_units
     }
 }
 impl GuardrailChecksContentFilterUsage {
```

### `src/types/_guardrail_checks_prompt_attack_result_entry.rs`

```diff
--- reference/src/types/_guardrail_checks_prompt_attack_result_entry.rs
+++ generated/src/types/_guardrail_checks_prompt_attack_result_entry.rs
@@ -15,8 +15,8 @@
         &self.category
     }
     /// <p>The severity score for the category, ranging from 0.0 to 1.0. Higher values indicate greater severity.</p>
-    pub fn severity_score(&self) -> f64 {
-        self.severity_score
+    pub fn severity_score(&self) -> &f64 {
+        &self.severity_score
     }
 }
 impl GuardrailChecksPromptAttackResultEntry {
```

### `src/types/_guardrail_checks_prompt_attack_usage.rs`

```diff
--- reference/src/types/_guardrail_checks_prompt_attack_usage.rs
+++ generated/src/types/_guardrail_checks_prompt_attack_usage.rs
@@ -9,8 +9,8 @@
 }
 impl GuardrailChecksPromptAttackUsage {
     /// <p>The number of text units consumed by the prompt attack check.</p>
-    pub fn text_units(&self) -> i32 {
-        self.text_units
+    pub fn text_units(&self) -> &i32 {
+        &self.text_units
     }
 }
 impl GuardrailChecksPromptAttackUsage {
```

### `src/types/_guardrail_checks_sensitive_information_result.rs`

```diff
--- reference/src/types/_guardrail_checks_sensitive_information_result.rs
+++ generated/src/types/_guardrail_checks_sensitive_information_result.rs
@@ -16,8 +16,8 @@
         self.results.deref()
     }
     /// <p>Specifies whether the results were truncated because the number of detected entities exceeded the maximum limit.</p>
-    pub fn truncated(&self) -> ::std::option::Option<bool> {
-        self.truncated
+    pub fn truncated(&self) -> ::std::option::Option<&bool> {
+        self.truncated.as_ref()
     }
 }
 impl GuardrailChecksSensitiveInformationResult {
```

### `src/types/_guardrail_checks_sensitive_information_result_entry.rs`

```diff
--- reference/src/types/_guardrail_checks_sensitive_information_result_entry.rs
+++ generated/src/types/_guardrail_checks_sensitive_information_result_entry.rs
@@ -23,24 +23,24 @@
         &self.r#type
     }
     /// <p>The confidence score for the detection, ranging from 0.0 to 1.0. Higher values indicate greater confidence.</p>
-    pub fn confidence_score(&self) -> f64 {
-        self.confidence_score
+    pub fn confidence_score(&self) -> &f64 {
+        &self.confidence_score
     }
     /// <p>The start character offset of the detected entity within the content block.</p>
-    pub fn begin_offset(&self) -> i32 {
-        self.begin_offset
+    pub fn begin_offset(&self) -> &i32 {
+        &self.begin_offset
     }
     /// <p>The end character offset of the detected entity within the content block.</p>
-    pub fn end_offset(&self) -> i32 {
-        self.end_offset
+    pub fn end_offset(&self) -> &i32 {
+        &self.end_offset
     }
     /// <p>The zero-based index of the message in the input messages array where the entity was detected.</p>
-    pub fn message_index(&self) -> i32 {
-        self.message_index
+    pub fn message_index(&self) -> &i32 {
+        &self.message_index
     }
     /// <p>The zero-based index of the content block within the message where the entity was detected.</p>
-    pub fn content_index(&self) -> i32 {
-        self.content_index
+    pub fn content_index(&self) -> &i32 {
+        &self.content_index
     }
 }
 impl GuardrailChecksSensitiveInformationResultEntry {
```

### `src/types/_guardrail_checks_sensitive_information_usage.rs`

```diff
--- reference/src/types/_guardrail_checks_sensitive_information_usage.rs
+++ generated/src/types/_guardrail_checks_sensitive_information_usage.rs
@@ -9,8 +9,8 @@
 }
 impl GuardrailChecksSensitiveInformationUsage {
     /// <p>The number of text units consumed by the sensitive information check.</p>
-    pub fn text_units(&self) -> i32 {
-        self.text_units
+    pub fn text_units(&self) -> &i32 {
+        &self.text_units
     }
 }
 impl GuardrailChecksSensitiveInformationUsage {
```

### `src/types/_guardrail_configuration.rs`

```diff
--- reference/src/types/_guardrail_configuration.rs
+++ generated/src/types/_guardrail_configuration.rs
@@ -90,11 +90,7 @@
         super::super::types::GuardrailConfiguration {
             guardrail_identifier: self.guardrail_identifier.unwrap_or_default(),
             guardrail_version: self.guardrail_version.unwrap_or_default(),
-            trace: self.trace.unwrap_or(
-                "disabled"
-                    .parse::<super::super::types::GuardrailTrace>()
-                    .expect("static value validated to member"),
-            ),
+            trace: self.trace.unwrap_or_default(),
         }
     }
 }
```

### `src/types/_guardrail_content_block.rs`

```diff
--- reference/src/types/_guardrail_content_block.rs
+++ generated/src/types/_guardrail_content_block.rs
@@ -2,7 +2,7 @@

 /// <p>The content block to be evaluated by the guardrail.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub enum GuardrailContentBlock {
     /// <p>Image within guardrail content block to be evaluated by the guardrail.</p>
     Image(super::super::types::GuardrailImageBlock),
@@ -50,12 +50,3 @@
         matches!(self, Self::Unknown)
     }
 }
-impl ::std::fmt::Debug for GuardrailContentBlock {
-    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
-        match self {
-            GuardrailContentBlock::Image(_) => f.debug_tuple("*** Sensitive Data Redacted ***").finish(),
-            GuardrailContentBlock::Text(val) => f.debug_tuple("Text").field(&val).finish(),
-            GuardrailContentBlock::Unknown => f.debug_tuple("Unknown").finish(),
-        }
-    }
-}
```

### `src/types/_guardrail_content_filter.rs`

```diff
--- reference/src/types/_guardrail_content_filter.rs
+++ generated/src/types/_guardrail_content_filter.rs
@@ -33,8 +33,8 @@
         &self.action
     }
     /// <p>Indicates whether content that breaches the guardrail configuration is detected.</p>
-    pub fn detected(&self) -> ::std::option::Option<bool> {
-        self.detected
+    pub fn detected(&self) -> ::std::option::Option<&bool> {
+        self.detected.as_ref()
     }
 }
 impl GuardrailContentFilter {
```

### `src/types/_guardrail_contextual_grounding_filter.rs`

```diff
--- reference/src/types/_guardrail_contextual_grounding_filter.rs
+++ generated/src/types/_guardrail_contextual_grounding_filter.rs
@@ -21,12 +21,12 @@
         &self.r#type
     }
     /// <p>The threshold used by contextual grounding filter to determine whether the content is grounded or not.</p>
-    pub fn threshold(&self) -> f64 {
-        self.threshold
+    pub fn threshold(&self) -> &f64 {
+        &self.threshold
     }
     /// <p>The score generated by contextual grounding filter.</p>
-    pub fn score(&self) -> f64 {
-        self.score
+    pub fn score(&self) -> &f64 {
+        &self.score
     }
     /// <p>The action performed by the guardrails contextual grounding filter.</p>
     pub fn action(&self) -> &super::super::types::GuardrailContextualGroundingPolicyAction {
@@ -33,8 +33,8 @@
         &self.action
     }
     /// <p>Indicates whether content that fails the contextual grounding evaluation (grounding or relevance score less than the corresponding threshold) was detected.</p>
-    pub fn detected(&self) -> ::std::option::Option<bool> {
-        self.detected
+    pub fn detected(&self) -> ::std::option::Option<&bool> {
+        self.detected.as_ref()
     }
 }
 impl GuardrailContextualGroundingFilter {
```

### `src/types/_guardrail_converse_content_block.rs`

```diff
--- reference/src/types/_guardrail_converse_content_block.rs
+++ generated/src/types/_guardrail_converse_content_block.rs
@@ -1,9 +1,9 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

-/// <p></p>
+/// <p/>
 /// <p>A content block for selective guarding with the <a href="https://docs.aws.amazon.com/bedrock/latest/APIReference/API_runtime_Converse.html">Converse</a> or <a href="https://docs.aws.amazon.com/bedrock/latest/APIReference/API_runtime_ConverseStream.html">ConverseStream</a> API operations.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub enum GuardrailConverseContentBlock {
     /// <p>Image within converse content block to be evaluated by the guardrail.</p>
     Image(super::super::types::GuardrailConverseImageBlock),
@@ -51,12 +51,3 @@
         matches!(self, Self::Unknown)
     }
 }
-impl ::std::fmt::Debug for GuardrailConverseContentBlock {
-    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
-        match self {
-            GuardrailConverseContentBlock::Image(_) => f.debug_tuple("*** Sensitive Data Redacted ***").finish(),
-            GuardrailConverseContentBlock::Text(val) => f.debug_tuple("Text").field(&val).finish(),
-            GuardrailConverseContentBlock::Unknown => f.debug_tuple("Unknown").finish(),
-        }
-    }
-}
```

### `src/types/_guardrail_converse_image_source.rs`

```diff
--- reference/src/types/_guardrail_converse_image_source.rs
+++ generated/src/types/_guardrail_converse_image_source.rs
@@ -2,7 +2,7 @@

 /// <p>The image source (image bytes) of the guardrail converse image source.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub enum GuardrailConverseImageSource {
     /// <p>The raw image bytes for the image.</p>
     Bytes(::aws_smithy_types::Blob),
@@ -36,8 +36,3 @@
         matches!(self, Self::Unknown)
     }
 }
-impl ::std::fmt::Debug for GuardrailConverseImageSource {
-    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
-        ::std::write!(f, "*** Sensitive Data Redacted ***")
-    }
-}
```

### `src/types/_guardrail_custom_word.rs`

```diff
--- reference/src/types/_guardrail_custom_word.rs
+++ generated/src/types/_guardrail_custom_word.rs
@@ -22,8 +22,8 @@
         &self.action
     }
     /// <p>Indicates whether custom word content that breaches the guardrail configuration is detected.</p>
-    pub fn detected(&self) -> ::std::option::Option<bool> {
-        self.detected
+    pub fn detected(&self) -> ::std::option::Option<&bool> {
+        self.detected.as_ref()
     }
 }
 impl GuardrailCustomWord {
```

### `src/types/_guardrail_image_source.rs`

```diff
--- reference/src/types/_guardrail_image_source.rs
+++ generated/src/types/_guardrail_image_source.rs
@@ -2,7 +2,7 @@

 /// <p>The image source (image bytes) of the guardrail image source. Object used in independent api.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub enum GuardrailImageSource {
     /// <p>The bytes details of the guardrail image source. Object used in independent api.</p>
     Bytes(::aws_smithy_types::Blob),
@@ -36,8 +36,3 @@
         matches!(self, Self::Unknown)
     }
 }
-impl ::std::fmt::Debug for GuardrailImageSource {
-    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
-        ::std::write!(f, "*** Sensitive Data Redacted ***")
-    }
-}
```

### `src/types/_guardrail_managed_word.rs`

```diff
--- reference/src/types/_guardrail_managed_word.rs
+++ generated/src/types/_guardrail_managed_word.rs
@@ -28,8 +28,8 @@
         &self.action
     }
     /// <p>Indicates whether managed word content that breaches the guardrail configuration is detected.</p>
-    pub fn detected(&self) -> ::std::option::Option<bool> {
-        self.detected
+    pub fn detected(&self) -> ::std::option::Option<&bool> {
+        self.detected.as_ref()
     }
 }
 impl GuardrailManagedWord {
```

### `src/types/_guardrail_ownership.rs`

```diff
--- reference/src/types/_guardrail_ownership.rs
+++ generated/src/types/_guardrail_ownership.rs
@@ -13,7 +13,7 @@
 /// # let guardrailownership = unimplemented!();
 /// match guardrailownership {
 ///     GuardrailOwnership::CrossAccount => { /* ... */ },
-///     GuardrailOwnership::SelfValue => { /* ... */ },
+///     GuardrailOwnership::SelfType => { /* ... */ },
 ///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
 ///     _ => { /* ... */ },
 /// }
@@ -36,8 +36,7 @@
 /// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
 /// - It might inadvertently shadow other intended match arms.
 ///
-///
-/// _Note: `GuardrailOwnership::Self` has been renamed to `::SelfValue`._
+#[allow(missing_docs)] // documentation missing in model
 #[non_exhaustive]
 #[derive(
     ::std::clone::Clone, ::std::cmp::Eq, ::std::cmp::Ord, ::std::cmp::PartialEq, ::std::cmp::PartialOrd, ::std::fmt::Debug, ::std::hash::Hash,
@@ -45,9 +44,8 @@
 pub enum GuardrailOwnership {
     #[allow(missing_docs)] // documentation missing in model
     CrossAccount,
-    ///
-    /// _Note: `::Self` has been renamed to `::SelfValue`._
-    SelfValue,
+    #[allow(missing_docs)] // documentation missing in model
+    SelfType,
     /// `Unknown` contains new variants that have been added since this code was generated.
     #[deprecated(note = "Don't directly match on `Unknown`. See the docs on this enum for the correct way to handle unknown variants.")]
     Unknown(super::super::primitives::sealed_enum_unknown::UnknownVariantValue),
@@ -56,7 +54,7 @@
     fn from(s: &str) -> Self {
         match s {
             "CROSS_ACCOUNT" => GuardrailOwnership::CrossAccount,
-            "SELF" => GuardrailOwnership::SelfValue,
+            "SELF" => GuardrailOwnership::SelfType,
             other => GuardrailOwnership::Unknown(super::super::primitives::sealed_enum_unknown::UnknownVariantValue(other.to_owned())),
         }
     }
@@ -73,7 +71,7 @@
     pub fn as_str(&self) -> &str {
         match self {
             GuardrailOwnership::CrossAccount => "CROSS_ACCOUNT",
-            GuardrailOwnership::SelfValue => "SELF",
+            GuardrailOwnership::SelfType => "SELF",
             GuardrailOwnership::Unknown(value) => value.as_str(),
         }
     }
@@ -103,7 +101,7 @@
     fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
         match self {
             GuardrailOwnership::CrossAccount => write!(f, "CROSS_ACCOUNT"),
-            GuardrailOwnership::SelfValue => write!(f, "SELF"),
+            GuardrailOwnership::SelfType => write!(f, "SELF"),
             GuardrailOwnership::Unknown(value) => write!(f, "{value}"),
         }
     }
```

### `src/types/_guardrail_pii_entity_filter.rs`

```diff
--- reference/src/types/_guardrail_pii_entity_filter.rs
+++ generated/src/types/_guardrail_pii_entity_filter.rs
@@ -28,8 +28,8 @@
         &self.action
     }
     /// <p>Indicates whether personally identifiable information (PII) that breaches the guardrail configuration is detected.</p>
-    pub fn detected(&self) -> ::std::option::Option<bool> {
-        self.detected
+    pub fn detected(&self) -> ::std::option::Option<&bool> {
+        self.detected.as_ref()
     }
 }
 impl GuardrailPiiEntityFilter {
```

### `src/types/_guardrail_regex_filter.rs`

```diff
--- reference/src/types/_guardrail_regex_filter.rs
+++ generated/src/types/_guardrail_regex_filter.rs
@@ -33,8 +33,8 @@
         &self.action
     }
     /// <p>Indicates whether custom regex entities that breach the guardrail configuration are detected.</p>
-    pub fn detected(&self) -> ::std::option::Option<bool> {
-        self.detected
+    pub fn detected(&self) -> ::std::option::Option<&bool> {
+        self.detected.as_ref()
     }
 }
 impl GuardrailRegexFilter {
```

### `src/types/_guardrail_stream_configuration.rs`

```diff
--- reference/src/types/_guardrail_stream_configuration.rs
+++ generated/src/types/_guardrail_stream_configuration.rs
@@ -116,16 +116,8 @@
         super::super::types::GuardrailStreamConfiguration {
             guardrail_identifier: self.guardrail_identifier.unwrap_or_default(),
             guardrail_version: self.guardrail_version.unwrap_or_default(),
-            trace: self.trace.unwrap_or(
-                "disabled"
-                    .parse::<super::super::types::GuardrailTrace>()
-                    .expect("static value validated to member"),
-            ),
-            stream_processing_mode: self.stream_processing_mode.unwrap_or(
-                "sync"
-                    .parse::<super::super::types::GuardrailStreamProcessingMode>()
-                    .expect("static value validated to member"),
-            ),
+            trace: self.trace.unwrap_or_default(),
+            stream_processing_mode: self.stream_processing_mode.unwrap_or_default(),
         }
     }
 }
```

### `src/types/_guardrail_topic.rs`

```diff
--- reference/src/types/_guardrail_topic.rs
+++ generated/src/types/_guardrail_topic.rs
@@ -28,8 +28,8 @@
         &self.action
     }
     /// <p>Indicates whether topic content that breaches the guardrail configuration is detected.</p>
-    pub fn detected(&self) -> ::std::option::Option<bool> {
-        self.detected
+    pub fn detected(&self) -> ::std::option::Option<&bool> {
+        self.detected.as_ref()
     }
 }
 impl GuardrailTopic {
```

### `src/types/_image_source.rs`

```diff
--- reference/src/types/_image_source.rs
+++ generated/src/types/_image_source.rs
@@ -2,7 +2,7 @@

 /// <p>The source for an image.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub enum ImageSource {
     /// <p>The raw image bytes for the image. If you use an AWS SDK, you don't need to encode the image bytes in base64.</p>
     Bytes(::aws_smithy_types::Blob),
@@ -50,8 +50,3 @@
         matches!(self, Self::Unknown)
     }
 }
-impl ::std::fmt::Debug for ImageSource {
-    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
-        ::std::write!(f, "*** Sensitive Data Redacted ***")
-    }
-}
```

### `src/types/_inference_configuration.rs`

```diff
--- reference/src/types/_inference_configuration.rs
+++ generated/src/types/_inference_configuration.rs
@@ -18,18 +18,18 @@
 }
 impl InferenceConfiguration {
     /// <p>The maximum number of tokens to allow in the generated response. The default value is the maximum allowed value for the model that you are using. For more information, see <a href="https://docs.aws.amazon.com/bedrock/latest/userguide/model-parameters.html">Inference parameters for foundation models</a>.</p>
-    pub fn max_tokens(&self) -> ::std::option::Option<i32> {
-        self.max_tokens
+    pub fn max_tokens(&self) -> ::std::option::Option<&i32> {
+        self.max_tokens.as_ref()
     }
     /// <p>The likelihood of the model selecting higher-probability options while generating a response. A lower value makes the model more likely to choose higher-probability options, while a higher value makes the model more likely to choose lower-probability options.</p>
     /// <p>The default value is the default value for the model that you are using. For more information, see <a href="https://docs.aws.amazon.com/bedrock/latest/userguide/model-parameters.html">Inference parameters for foundation models</a>.</p>
-    pub fn temperature(&self) -> ::std::option::Option<f32> {
-        self.temperature
+    pub fn temperature(&self) -> ::std::option::Option<&f32> {
+        self.temperature.as_ref()
     }
     /// <p>The percentage of most-likely candidates that the model considers for the next token. For example, if you choose a value of 0.8 for <code>topP</code>, the model selects from the top 80% of the probability distribution of tokens that could be next in the sequence.</p>
     /// <p>The default value is the default value for the model that you are using. For more information, see <a href="https://docs.aws.amazon.com/bedrock/latest/userguide/model-parameters.html">Inference parameters for foundation models</a>.</p>
-    pub fn top_p(&self) -> ::std::option::Option<f32> {
-        self.top_p
+    pub fn top_p(&self) -> ::std::option::Option<&f32> {
+        self.top_p.as_ref()
     }
     /// <p>A list of stop sequences. A stop sequence is a sequence of characters that causes the model to stop generating the response.</p>
     ///
```

### `src/types/_invoke_model_with_bidirectional_stream_input.rs`

```diff
--- reference/src/types/_invoke_model_with_bidirectional_stream_input.rs
+++ generated/src/types/_invoke_model_with_bidirectional_stream_input.rs
@@ -2,7 +2,7 @@

 /// <p>Payload content, the speech chunk, for the bidirectional input of the invocation step.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub enum InvokeModelWithBidirectionalStreamInput {
     /// <p>The audio chunk that is used as input for the invocation step.</p>
     Chunk(super::super::types::BidirectionalInputPayloadPart),
@@ -36,11 +36,3 @@
         matches!(self, Self::Unknown)
     }
 }
-impl ::std::fmt::Debug for InvokeModelWithBidirectionalStreamInput {
-    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
-        match self {
-            InvokeModelWithBidirectionalStreamInput::Chunk(_) => f.debug_tuple("*** Sensitive Data Redacted ***").finish(),
-            InvokeModelWithBidirectionalStreamInput::Unknown => f.debug_tuple("Unknown").finish(),
-        }
-    }
-}
```

### `src/types/_invoke_model_with_bidirectional_stream_output.rs`

```diff
--- reference/src/types/_invoke_model_with_bidirectional_stream_output.rs
+++ generated/src/types/_invoke_model_with_bidirectional_stream_output.rs
@@ -2,7 +2,7 @@

 /// <p>Output from the bidirectional stream that was used for model invocation.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub enum InvokeModelWithBidirectionalStreamOutput {
     /// <p>The speech chunk that was provided as output from the invocation step.</p>
     Chunk(super::super::types::BidirectionalOutputPayloadPart),
@@ -36,11 +36,3 @@
         matches!(self, Self::Unknown)
     }
 }
-impl ::std::fmt::Debug for InvokeModelWithBidirectionalStreamOutput {
-    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
-        match self {
-            InvokeModelWithBidirectionalStreamOutput::Chunk(_) => f.debug_tuple("*** Sensitive Data Redacted ***").finish(),
-            InvokeModelWithBidirectionalStreamOutput::Unknown => f.debug_tuple("Unknown").finish(),
-        }
-    }
-}
```

### `src/types/_output_format.rs`

```diff
--- reference/src/types/_output_format.rs
+++ generated/src/types/_output_format.rs
@@ -22,7 +22,7 @@
 impl ::std::fmt::Debug for OutputFormat {
     fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
         let mut formatter = f.debug_struct("OutputFormat");
-        formatter.field("r#type", &self.r#type);
+        formatter.field("type", &self.r#type);
         formatter.field("structure", &"*** Sensitive Data Redacted ***");
         formatter.finish()
     }
@@ -90,7 +90,7 @@
 impl ::std::fmt::Debug for OutputFormatBuilder {
     fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
         let mut formatter = f.debug_struct("OutputFormatBuilder");
-        formatter.field("r#type", &self.r#type);
+        formatter.field("type", &self.r#type);
         formatter.field("structure", &"*** Sensitive Data Redacted ***");
         formatter.finish()
     }
```

### `src/types/_output_format_structure.rs`

```diff
--- reference/src/types/_output_format_structure.rs
+++ generated/src/types/_output_format_structure.rs
@@ -2,7 +2,7 @@

 /// <p>The structure that the model's output must adhere to.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub enum OutputFormatStructure {
     /// <p>A JSON schema structure that the model's output must adhere to.</p>
     JsonSchema(super::super::types::JsonSchemaDefinition),
@@ -36,8 +36,3 @@
         matches!(self, Self::Unknown)
     }
 }
-impl ::std::fmt::Debug for OutputFormatStructure {
-    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
-        ::std::write!(f, "*** Sensitive Data Redacted ***")
-    }
-}
```

### `src/types/_performance_configuration.rs`

```diff
--- reference/src/types/_performance_configuration.rs
+++ generated/src/types/_performance_configuration.rs
@@ -44,11 +44,7 @@
     /// Consumes the builder and constructs a [`PerformanceConfiguration`](crate::types::PerformanceConfiguration).
     pub fn build(self) -> super::super::types::PerformanceConfiguration {
         super::super::types::PerformanceConfiguration {
-            latency: self.latency.unwrap_or(
-                "standard"
-                    .parse::<super::super::types::PerformanceConfigLatency>()
-                    .expect("static value validated to member"),
-            ),
+            latency: self.latency.unwrap_or_default(),
         }
     }
 }
```

### `src/types/_reasoning_content_block.rs`

```diff
--- reference/src/types/_reasoning_content_block.rs
+++ generated/src/types/_reasoning_content_block.rs
@@ -2,7 +2,7 @@

 /// <p>Contains content regarding the reasoning that is carried out by the model with respect to the content in the content block. Reasoning refers to a Chain of Thought (CoT) that the model generates to enhance the accuracy of its final response.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub enum ReasoningContentBlock {
     /// <p>The reasoning that the model used to return the output.</p>
     ReasoningText(super::super::types::ReasoningTextBlock),
@@ -50,8 +50,3 @@
         matches!(self, Self::Unknown)
     }
 }
-impl ::std::fmt::Debug for ReasoningContentBlock {
-    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
-        ::std::write!(f, "*** Sensitive Data Redacted ***")
-    }
-}
```

### `src/types/_reasoning_content_block_delta.rs`

```diff
--- reference/src/types/_reasoning_content_block_delta.rs
+++ generated/src/types/_reasoning_content_block_delta.rs
@@ -2,7 +2,7 @@

 /// <p>Contains content regarding the reasoning that is carried out by the model with respect to the content in the content block. Reasoning refers to a Chain of Thought (CoT) that the model generates to enhance the accuracy of its final response.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub enum ReasoningContentBlockDelta {
     /// <p>The content in the reasoning that was encrypted by the model provider for safety reasons. The encryption doesn't affect the quality of responses.</p>
     RedactedContent(::aws_smithy_types::Blob),
@@ -65,8 +65,3 @@
         matches!(self, Self::Unknown)
     }
 }
-impl ::std::fmt::Debug for ReasoningContentBlockDelta {
-    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
-        ::std::write!(f, "*** Sensitive Data Redacted ***")
-    }
-}
```

### `src/types/_response_stream.rs`

```diff
--- reference/src/types/_response_stream.rs
+++ generated/src/types/_response_stream.rs
@@ -2,7 +2,7 @@

 /// <p>Definition of content in the response stream.</p>
 #[non_exhaustive]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
+#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
 pub enum ResponseStream {
     /// <p>Content included in the response.</p>
     Chunk(super::super::types::PayloadPart),
@@ -36,11 +36,3 @@
         matches!(self, Self::Unknown)
     }
 }
-impl ::std::fmt::Debug for ResponseStream {
-    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
-        match self {
-            ResponseStream::Chunk(_) => f.debug_tuple("*** Sensitive Data Redacted ***").finish(),
-            ResponseStream::Unknown => f.debug_tuple("Unknown").finish(),
-        }
-    }
-}
```

### `src/types/_search_result_location.rs`

```diff
--- reference/src/types/_search_result_location.rs
+++ generated/src/types/_search_result_location.rs
@@ -13,16 +13,16 @@
 }
 impl SearchResultLocation {
     /// <p>The index of the search result content block where the cited content is found.</p>
-    pub fn search_result_index(&self) -> ::std::option::Option<i32> {
-        self.search_result_index
+    pub fn search_result_index(&self) -> ::std::option::Option<&i32> {
+        self.search_result_index.as_ref()
     }
     /// <p>The starting position in the content array where the cited content begins.</p>
-    pub fn start(&self) -> ::std::option::Option<i32> {
-        self.start
+    pub fn start(&self) -> ::std::option::Option<&i32> {
+        self.start.as_ref()
     }
     /// <p>The ending position in the content array where the cited content ends.</p>
-    pub fn end(&self) -> ::std::option::Option<i32> {
-        self.end
+    pub fn end(&self) -> ::std::option::Option<&i32> {
+        self.end.as_ref()
     }
 }
 impl SearchResultLocation {
```

### `src/types/_token_usage.rs`

```diff
--- reference/src/types/_token_usage.rs
+++ generated/src/types/_token_usage.rs
@@ -19,24 +19,24 @@
 }
 impl TokenUsage {
     /// <p>The number of tokens sent in the request to the model.</p>
-    pub fn input_tokens(&self) -> i32 {
-        self.input_tokens
+    pub fn input_tokens(&self) -> &i32 {
+        &self.input_tokens
     }
     /// <p>The number of tokens that the model generated for the request.</p>
-    pub fn output_tokens(&self) -> i32 {
-        self.output_tokens
+    pub fn output_tokens(&self) -> &i32 {
+        &self.output_tokens
     }
     /// <p>The total of input tokens and tokens generated by the model.</p>
-    pub fn total_tokens(&self) -> i32 {
-        self.total_tokens
+    pub fn total_tokens(&self) -> &i32 {
+        &self.total_tokens
     }
     /// <p>The number of input tokens read from the cache for the request.</p>
-    pub fn cache_read_input_tokens(&self) -> ::std::option::Option<i32> {
-        self.cache_read_input_tokens
+    pub fn cache_read_input_tokens(&self) -> ::std::option::Option<&i32> {
+        self.cache_read_input_tokens.as_ref()
     }
     /// <p>The number of input tokens written to the cache for the request.</p>
-    pub fn cache_write_input_tokens(&self) -> ::std::option::Option<i32> {
-        self.cache_write_input_tokens
+    pub fn cache_write_input_tokens(&self) -> ::std::option::Option<&i32> {
+        self.cache_write_input_tokens.as_ref()
     }
     /// <p>Detailed breakdown of cache writes by TTL. Empty if no cache creation occurred. Sorted by TTL duration (1h before 5m).</p>
     ///
```

### `src/types/_tool_specification.rs`

```diff
--- reference/src/types/_tool_specification.rs
+++ generated/src/types/_tool_specification.rs
@@ -28,8 +28,8 @@
         self.input_schema.as_ref()
     }
     /// <p>Flag to enable structured output enforcement on a tool usage response.</p>
-    pub fn strict(&self) -> ::std::option::Option<bool> {
-        self.strict
+    pub fn strict(&self) -> ::std::option::Option<&bool> {
+        self.strict.as_ref()
     }
 }
 impl ToolSpecification {
```

### `src/types/builders.rs`

```diff
--- reference/src/types/builders.rs
+++ generated/src/types/builders.rs
@@ -3,12 +3,6 @@

 pub use super::super::types::_guardrail_coverage::GuardrailCoverageBuilder;

-pub use super::super::types::_guardrail_checks_config::GuardrailChecksConfigBuilder;
-
-pub use super::super::types::_guardrail_checks_results::GuardrailChecksResultsBuilder;
-
-pub use super::super::types::_guardrail_checks_usage_results::GuardrailChecksUsageResultsBuilder;
-
 pub use super::super::types::_inference_configuration::InferenceConfigurationBuilder;

 pub use super::super::types::_tool_configuration::ToolConfigurationBuilder;
@@ -29,11 +23,11 @@

 pub use super::super::types::_guardrail_stream_configuration::GuardrailStreamConfigurationBuilder;

-pub use super::super::types::_async_invoke_s3_output_data_config::AsyncInvokeS3OutputDataConfigBuilder;
+pub use super::super::types::_guardrail_checks_config::GuardrailChecksConfigBuilder;

-pub use super::super::types::_async_invoke_summary::AsyncInvokeSummaryBuilder;
+pub use super::super::types::_guardrail_checks_results::GuardrailChecksResultsBuilder;

-pub use super::super::types::_tag::TagBuilder;
+pub use super::super::types::_guardrail_checks_usage_results::GuardrailChecksUsageResultsBuilder;

 pub use super::super::types::_guardrail_output_content::GuardrailOutputContentBuilder;

@@ -43,6 +37,32 @@

 pub use super::super::types::_guardrail_image_coverage::GuardrailImageCoverageBuilder;

+pub use super::super::types::_message::MessageBuilder;
+
+pub use super::super::types::_output_format::OutputFormatBuilder;
+
+pub use super::super::types::_guardrail_trace_assessment::GuardrailTraceAssessmentBuilder;
+
+pub use super::super::types::_prompt_router_trace::PromptRouterTraceBuilder;
+
+pub use super::super::types::_message_start_event::MessageStartEventBuilder;
+
+pub use super::super::types::_content_block_start_event::ContentBlockStartEventBuilder;
+
+pub use super::super::types::_content_block_delta_event::ContentBlockDeltaEventBuilder;
+
+pub use super::super::types::_content_block_stop_event::ContentBlockStopEventBuilder;
+
+pub use super::super::types::_message_stop_event::MessageStopEventBuilder;
+
+pub use super::super::types::_converse_stream_metadata_event::ConverseStreamMetadataEventBuilder;
+
+pub use super::super::types::_invoke_model_tokens_request::InvokeModelTokensRequestBuilder;
+
+pub use super::super::types::_converse_tokens_request::ConverseTokensRequestBuilder;
+
+pub use super::super::types::_async_invoke_s3_output_data_config::AsyncInvokeS3OutputDataConfigBuilder;
+
 pub use super::super::types::_guardrail_checks_message::GuardrailChecksMessageBuilder;

 pub use super::super::types::_guardrail_checks_content_filter_config::GuardrailChecksContentFilterConfigBuilder;
@@ -63,26 +83,6 @@

 pub use super::super::types::_guardrail_checks_sensitive_information_usage::GuardrailChecksSensitiveInformationUsageBuilder;

-pub use super::super::types::_message::MessageBuilder;
-
-pub use super::super::types::_output_format::OutputFormatBuilder;
-
-pub use super::super::types::_guardrail_trace_assessment::GuardrailTraceAssessmentBuilder;
-
-pub use super::super::types::_prompt_router_trace::PromptRouterTraceBuilder;
-
-pub use super::super::types::_message_start_event::MessageStartEventBuilder;
-
-pub use super::super::types::_content_block_start_event::ContentBlockStartEventBuilder;
-
-pub use super::super::types::_content_block_delta_event::ContentBlockDeltaEventBuilder;
-
-pub use super::super::types::_content_block_stop_event::ContentBlockStopEventBuilder;
-
-pub use super::super::types::_message_stop_event::MessageStopEventBuilder;
-
-pub use super::super::types::_converse_stream_metadata_event::ConverseStreamMetadataEventBuilder;
-
 pub use super::super::types::_bidirectional_input_payload_part::BidirectionalInputPayloadPartBuilder;

 pub use super::super::types::_bidirectional_output_payload_part::BidirectionalOutputPayloadPartBuilder;
@@ -89,9 +89,9 @@

 pub use super::super::types::_payload_part::PayloadPartBuilder;

-pub use super::super::types::_invoke_model_tokens_request::InvokeModelTokensRequestBuilder;
+pub use super::super::types::_async_invoke_summary::AsyncInvokeSummaryBuilder;

-pub use super::super::types::_converse_tokens_request::ConverseTokensRequestBuilder;
+pub use super::super::types::_tag::TagBuilder;

 pub use super::super::types::_guardrail_text_block::GuardrailTextBlockBuilder;

@@ -127,18 +127,6 @@

 pub use super::super::types::_converse_stream_trace::ConverseStreamTraceBuilder;

-pub use super::super::types::_guardrail_checks_content_filter_category_config::GuardrailChecksContentFilterCategoryConfigBuilder;
-
-pub use super::super::types::_guardrail_checks_prompt_attack_category_config::GuardrailChecksPromptAttackCategoryConfigBuilder;
-
-pub use super::super::types::_guardrail_checks_sensitive_information_entity_config::GuardrailChecksSensitiveInformationEntityConfigBuilder;
-
-pub use super::super::types::_guardrail_checks_content_filter_result_entry::GuardrailChecksContentFilterResultEntryBuilder;
-
-pub use super::super::types::_guardrail_checks_prompt_attack_result_entry::GuardrailChecksPromptAttackResultEntryBuilder;
-
-pub use super::super::types::_guardrail_checks_sensitive_information_result_entry::GuardrailChecksSensitiveInformationResultEntryBuilder;
-
 pub use super::super::types::_guardrail_converse_text_block::GuardrailConverseTextBlockBuilder;

 pub use super::super::types::_guardrail_converse_image_block::GuardrailConverseImageBlockBuilder;
@@ -161,6 +149,18 @@

 pub use super::super::types::_image_block_delta::ImageBlockDeltaBuilder;

+pub use super::super::types::_guardrail_checks_content_filter_category_config::GuardrailChecksContentFilterCategoryConfigBuilder;
+
+pub use super::super::types::_guardrail_checks_prompt_attack_category_config::GuardrailChecksPromptAttackCategoryConfigBuilder;
+
+pub use super::super::types::_guardrail_checks_sensitive_information_entity_config::GuardrailChecksSensitiveInformationEntityConfigBuilder;
+
+pub use super::super::types::_guardrail_checks_content_filter_result_entry::GuardrailChecksContentFilterResultEntryBuilder;
+
+pub use super::super::types::_guardrail_checks_prompt_attack_result_entry::GuardrailChecksPromptAttackResultEntryBuilder;
+
+pub use super::super::types::_guardrail_checks_sensitive_information_result_entry::GuardrailChecksSensitiveInformationResultEntryBuilder;
+
 pub use super::super::types::_guardrail_topic::GuardrailTopicBuilder;

 pub use super::super::types::_guardrail_content_filter::GuardrailContentFilterBuilder;
```

### `src/types/error/_model_not_ready_exception.rs`

```diff
--- reference/src/types/error/_model_not_ready_exception.rs
+++ generated/src/types/error/_model_not_ready_exception.rs
@@ -11,7 +11,7 @@
 impl ModelNotReadyException {
     /// Returns `Some(ErrorKind)` if the error is retryable. Otherwise, returns `None`.
     pub fn retryable_error_kind(&self) -> ::aws_smithy_types::retry::ErrorKind {
-        ::aws_smithy_types::retry::ErrorKind::ClientError
+        ::aws_smithy_types::retry::ErrorKind::ServerError
     }
     /// Returns the error message.
     pub fn message(&self) -> ::std::option::Option<&str> {
```

### `src/types/error/builders.rs`

```diff
--- reference/src/types/error/builders.rs
+++ generated/src/types/error/builders.rs
@@ -3,12 +3,6 @@

 pub use super::super::super::types::error::_internal_server_exception::InternalServerExceptionBuilder;

-pub use super::super::super::types::error::_throttling_exception::ThrottlingExceptionBuilder;
-
-pub use super::super::super::types::error::_validation_exception::ValidationExceptionBuilder;
-
-pub use super::super::super::types::error::_conflict_exception::ConflictExceptionBuilder;
-
 pub use super::super::super::types::error::_resource_not_found_exception::ResourceNotFoundExceptionBuilder;

 pub use super::super::super::types::error::_service_quota_exceeded_exception::ServiceQuotaExceededExceptionBuilder;
@@ -15,6 +9,10 @@

 pub use super::super::super::types::error::_service_unavailable_exception::ServiceUnavailableExceptionBuilder;

+pub use super::super::super::types::error::_throttling_exception::ThrottlingExceptionBuilder;
+
+pub use super::super::super::types::error::_validation_exception::ValidationExceptionBuilder;
+
 pub use super::super::super::types::error::_model_error_exception::ModelErrorExceptionBuilder;

 pub use super::super::super::types::error::_model_not_ready_exception::ModelNotReadyExceptionBuilder;
@@ -22,3 +20,5 @@
 pub use super::super::super::types::error::_model_timeout_exception::ModelTimeoutExceptionBuilder;

 pub use super::super::super::types::error::_model_stream_error_exception::ModelStreamErrorExceptionBuilder;
+
+pub use super::super::super::types::error::_conflict_exception::ConflictExceptionBuilder;
```

### `src/types/error.rs`

```diff
--- reference/src/types/error.rs
+++ generated/src/types/error.rs
@@ -3,12 +3,6 @@

 pub use super::super::types::error::_internal_server_exception::InternalServerException;

-pub use super::super::types::error::_throttling_exception::ThrottlingException;
-
-pub use super::super::types::error::_validation_exception::ValidationException;
-
-pub use super::super::types::error::_conflict_exception::ConflictException;
-
 pub use super::super::types::error::_resource_not_found_exception::ResourceNotFoundException;

 pub use super::super::types::error::_service_quota_exceeded_exception::ServiceQuotaExceededException;
@@ -15,6 +9,10 @@

 pub use super::super::types::error::_service_unavailable_exception::ServiceUnavailableException;

+pub use super::super::types::error::_throttling_exception::ThrottlingException;
+
+pub use super::super::types::error::_validation_exception::ValidationException;
+
 pub use super::super::types::error::_model_error_exception::ModelErrorException;

 pub use super::super::types::error::_model_not_ready_exception::ModelNotReadyException;
@@ -23,6 +21,8 @@

 pub use super::super::types::error::_model_stream_error_exception::ModelStreamErrorException;

+pub use super::super::types::error::_conflict_exception::ConflictException;
+
 /// Error type for the `ConverseStreamOutputError` operation.
 #[non_exhaustive]
 #[derive(::std::fmt::Debug)]
@@ -78,23 +78,23 @@
             Self::Unhandled(e) => &e.meta,
         }
     }
-    /// Returns `true` if the error kind is `ConverseStreamOutputError::InternalServerException`.
+    /// Returns `true` if the error kind is `InternalServerException::InternalServerException`.
     pub fn is_internal_server_exception(&self) -> bool {
         matches!(self, Self::InternalServerException(_))
     }
-    /// Returns `true` if the error kind is `ConverseStreamOutputError::ModelStreamErrorException`.
+    /// Returns `true` if the error kind is `ModelStreamErrorException::ModelStreamErrorException`.
     pub fn is_model_stream_error_exception(&self) -> bool {
         matches!(self, Self::ModelStreamErrorException(_))
     }
-    /// Returns `true` if the error kind is `ConverseStreamOutputError::ValidationException`.
+    /// Returns `true` if the error kind is `ValidationException::ValidationException`.
     pub fn is_validation_exception(&self) -> bool {
         matches!(self, Self::ValidationException(_))
     }
-    /// Returns `true` if the error kind is `ConverseStreamOutputError::ThrottlingException`.
+    /// Returns `true` if the error kind is `ThrottlingException::ThrottlingException`.
     pub fn is_throttling_exception(&self) -> bool {
         matches!(self, Self::ThrottlingException(_))
     }
-    /// Returns `true` if the error kind is `ConverseStreamOutputError::ServiceUnavailableException`.
+    /// Returns `true` if the error kind is `ServiceUnavailableException::ServiceUnavailableException`.
     pub fn is_service_unavailable_exception(&self) -> bool {
         matches!(self, Self::ServiceUnavailableException(_))
     }
@@ -140,11 +140,11 @@
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for ConverseStreamOutputError {
     fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::InternalServerException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::ModelStreamErrorException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::ValidationException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::ThrottlingException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::ServiceUnavailableException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::InternalServerException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::ModelStreamErrorException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::ValidationException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::ThrottlingException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::ServiceUnavailableException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
             Self::Unhandled(_inner) => &_inner.meta,
         }
     }
@@ -165,7 +165,6 @@
         self.meta().request_id()
     }
 }
-
 /// Error type for the `InvokeModelWithBidirectionalStreamInputError` operation.
 #[non_exhaustive]
 #[derive(::std::fmt::Debug)]
@@ -258,7 +257,6 @@
         self.meta().request_id()
     }
 }
-
 /// Error type for the `InvokeModelWithBidirectionalStreamOutputError` operation.
 #[non_exhaustive]
 #[derive(::std::fmt::Debug)]
@@ -317,27 +315,27 @@
             Self::Unhandled(e) => &e.meta,
         }
     }
-    /// Returns `true` if the error kind is `InvokeModelWithBidirectionalStreamOutputError::InternalServerException`.
+    /// Returns `true` if the error kind is `InternalServerException::InternalServerException`.
     pub fn is_internal_server_exception(&self) -> bool {
         matches!(self, Self::InternalServerException(_))
     }
-    /// Returns `true` if the error kind is `InvokeModelWithBidirectionalStreamOutputError::ModelStreamErrorException`.
+    /// Returns `true` if the error kind is `ModelStreamErrorException::ModelStreamErrorException`.
     pub fn is_model_stream_error_exception(&self) -> bool {
         matches!(self, Self::ModelStreamErrorException(_))
     }
-    /// Returns `true` if the error kind is `InvokeModelWithBidirectionalStreamOutputError::ValidationException`.
+    /// Returns `true` if the error kind is `ValidationException::ValidationException`.
     pub fn is_validation_exception(&self) -> bool {
         matches!(self, Self::ValidationException(_))
     }
-    /// Returns `true` if the error kind is `InvokeModelWithBidirectionalStreamOutputError::ThrottlingException`.
+    /// Returns `true` if the error kind is `ThrottlingException::ThrottlingException`.
     pub fn is_throttling_exception(&self) -> bool {
         matches!(self, Self::ThrottlingException(_))
     }
-    /// Returns `true` if the error kind is `InvokeModelWithBidirectionalStreamOutputError::ModelTimeoutException`.
+    /// Returns `true` if the error kind is `ModelTimeoutException::ModelTimeoutException`.
     pub fn is_model_timeout_exception(&self) -> bool {
         matches!(self, Self::ModelTimeoutException(_))
     }
-    /// Returns `true` if the error kind is `InvokeModelWithBidirectionalStreamOutputError::ServiceUnavailableException`.
+    /// Returns `true` if the error kind is `ServiceUnavailableException::ServiceUnavailableException`.
     pub fn is_service_unavailable_exception(&self) -> bool {
         matches!(self, Self::ServiceUnavailableException(_))
     }
@@ -385,12 +383,12 @@
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for InvokeModelWithBidirectionalStreamOutputError {
     fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::InternalServerException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::ModelStreamErrorException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::ValidationException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::ThrottlingException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::ModelTimeoutException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::ServiceUnavailableException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::InternalServerException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::ModelStreamErrorException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::ValidationException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::ThrottlingException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::ModelTimeoutException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::ServiceUnavailableException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
             Self::Unhandled(_inner) => &_inner.meta,
         }
     }
@@ -411,7 +409,6 @@
         self.meta().request_id()
     }
 }
-
 /// Error type for the `ResponseStreamError` operation.
 #[non_exhaustive]
 #[derive(::std::fmt::Debug)]
@@ -470,27 +467,27 @@
             Self::Unhandled(e) => &e.meta,
         }
     }
-    /// Returns `true` if the error kind is `ResponseStreamError::InternalServerException`.
+    /// Returns `true` if the error kind is `InternalServerException::InternalServerException`.
     pub fn is_internal_server_exception(&self) -> bool {
         matches!(self, Self::InternalServerException(_))
     }
-    /// Returns `true` if the error kind is `ResponseStreamError::ModelStreamErrorException`.
+    /// Returns `true` if the error kind is `ModelStreamErrorException::ModelStreamErrorException`.
     pub fn is_model_stream_error_exception(&self) -> bool {
         matches!(self, Self::ModelStreamErrorException(_))
     }
-    /// Returns `true` if the error kind is `ResponseStreamError::ValidationException`.
+    /// Returns `true` if the error kind is `ValidationException::ValidationException`.
     pub fn is_validation_exception(&self) -> bool {
         matches!(self, Self::ValidationException(_))
     }
-    /// Returns `true` if the error kind is `ResponseStreamError::ThrottlingException`.
+    /// Returns `true` if the error kind is `ThrottlingException::ThrottlingException`.
     pub fn is_throttling_exception(&self) -> bool {
         matches!(self, Self::ThrottlingException(_))
     }
-    /// Returns `true` if the error kind is `ResponseStreamError::ModelTimeoutException`.
+    /// Returns `true` if the error kind is `ModelTimeoutException::ModelTimeoutException`.
     pub fn is_model_timeout_exception(&self) -> bool {
         matches!(self, Self::ModelTimeoutException(_))
     }
-    /// Returns `true` if the error kind is `ResponseStreamError::ServiceUnavailableException`.
+    /// Returns `true` if the error kind is `ServiceUnavailableException::ServiceUnavailableException`.
     pub fn is_service_unavailable_exception(&self) -> bool {
         matches!(self, Self::ServiceUnavailableException(_))
     }
@@ -538,12 +535,12 @@
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for ResponseStreamError {
     fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::InternalServerException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::ModelStreamErrorException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::ValidationException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::ThrottlingException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::ModelTimeoutException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::ServiceUnavailableException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::InternalServerException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::ModelStreamErrorException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::ValidationException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::ThrottlingException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::ModelTimeoutException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::ServiceUnavailableException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
             Self::Unhandled(_inner) => &_inner.meta,
         }
     }
@@ -564,7 +561,6 @@
         self.meta().request_id()
     }
 }
-
 mod _access_denied_exception;

 mod _conflict_exception;
```

### `src/types.rs`

```diff
--- reference/src/types.rs
+++ generated/src/types.rs
@@ -1,12 +1,4 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-pub use super::types::_async_invoke_status::AsyncInvokeStatus;
-
-pub use super::types::_async_invoke_output_data_config::AsyncInvokeOutputDataConfig;
-
-pub use super::types::_sort_async_invocation_by::SortAsyncInvocationBy;
-
-pub use super::types::_sort_order::SortOrder;
-
 pub use super::types::_guardrail_content_source::GuardrailContentSource;

 pub use super::types::_guardrail_output_scope::GuardrailOutputScope;
@@ -17,12 +9,6 @@

 pub use super::types::_guardrail_coverage::GuardrailCoverage;

-pub use super::types::_guardrail_checks_config::GuardrailChecksConfig;
-
-pub use super::types::_guardrail_checks_results::GuardrailChecksResults;
-
-pub use super::types::_guardrail_checks_usage_results::GuardrailChecksUsageResults;
-
 pub use super::types::_inference_configuration::InferenceConfiguration;

 pub use super::types::_tool_configuration::ToolConfiguration;
@@ -49,6 +35,18 @@

 pub use super::types::_converse_stream_output::ConverseStreamOutput;

+pub use super::types::_count_tokens_input::CountTokensInput;
+
+pub use super::types::_async_invoke_status::AsyncInvokeStatus;
+
+pub use super::types::_async_invoke_output_data_config::AsyncInvokeOutputDataConfig;
+
+pub use super::types::_guardrail_checks_config::GuardrailChecksConfig;
+
+pub use super::types::_guardrail_checks_results::GuardrailChecksResults;
+
+pub use super::types::_guardrail_checks_usage_results::GuardrailChecksUsageResults;
+
 pub use super::types::_trace::Trace;

 pub use super::types::_performance_config_latency::PerformanceConfigLatency;
@@ -61,13 +59,9 @@

 pub use super::types::_response_stream::ResponseStream;

-pub use super::types::_count_tokens_input::CountTokensInput;
+pub use super::types::_sort_async_invocation_by::SortAsyncInvocationBy;

-pub use super::types::_async_invoke_s3_output_data_config::AsyncInvokeS3OutputDataConfig;
-
-pub use super::types::_async_invoke_summary::AsyncInvokeSummary;
-
-pub use super::types::_tag::Tag;
+pub use super::types::_sort_order::SortOrder;

 pub use super::types::_guardrail_content_block::GuardrailContentBlock;

@@ -79,26 +73,6 @@

 pub use super::types::_guardrail_image_coverage::GuardrailImageCoverage;

-pub use super::types::_guardrail_checks_message::GuardrailChecksMessage;
-
-pub use super::types::_guardrail_checks_content_filter_config::GuardrailChecksContentFilterConfig;
-
-pub use super::types::_guardrail_checks_prompt_attack_config::GuardrailChecksPromptAttackConfig;
-
-pub use super::types::_guardrail_checks_sensitive_information_config::GuardrailChecksSensitiveInformationConfig;
-
-pub use super::types::_guardrail_checks_content_filter_result::GuardrailChecksContentFilterResult;
-
-pub use super::types::_guardrail_checks_prompt_attack_result::GuardrailChecksPromptAttackResult;
-
-pub use super::types::_guardrail_checks_sensitive_information_result::GuardrailChecksSensitiveInformationResult;
-
-pub use super::types::_guardrail_checks_content_filter_usage::GuardrailChecksContentFilterUsage;
-
-pub use super::types::_guardrail_checks_prompt_attack_usage::GuardrailChecksPromptAttackUsage;
-
-pub use super::types::_guardrail_checks_sensitive_information_usage::GuardrailChecksSensitiveInformationUsage;
-
 pub use super::types::_message::Message;

 pub use super::types::_system_content_block::SystemContentBlock;
@@ -129,6 +103,32 @@

 pub use super::types::_converse_stream_metadata_event::ConverseStreamMetadataEvent;

+pub use super::types::_invoke_model_tokens_request::InvokeModelTokensRequest;
+
+pub use super::types::_converse_tokens_request::ConverseTokensRequest;
+
+pub use super::types::_async_invoke_s3_output_data_config::AsyncInvokeS3OutputDataConfig;
+
+pub use super::types::_guardrail_checks_message::GuardrailChecksMessage;
+
+pub use super::types::_guardrail_checks_content_filter_config::GuardrailChecksContentFilterConfig;
+
+pub use super::types::_guardrail_checks_prompt_attack_config::GuardrailChecksPromptAttackConfig;
+
+pub use super::types::_guardrail_checks_sensitive_information_config::GuardrailChecksSensitiveInformationConfig;
+
+pub use super::types::_guardrail_checks_content_filter_result::GuardrailChecksContentFilterResult;
+
+pub use super::types::_guardrail_checks_prompt_attack_result::GuardrailChecksPromptAttackResult;
+
+pub use super::types::_guardrail_checks_sensitive_information_result::GuardrailChecksSensitiveInformationResult;
+
+pub use super::types::_guardrail_checks_content_filter_usage::GuardrailChecksContentFilterUsage;
+
+pub use super::types::_guardrail_checks_prompt_attack_usage::GuardrailChecksPromptAttackUsage;
+
+pub use super::types::_guardrail_checks_sensitive_information_usage::GuardrailChecksSensitiveInformationUsage;
+
 pub use super::types::_bidirectional_input_payload_part::BidirectionalInputPayloadPart;

 pub use super::types::_bidirectional_output_payload_part::BidirectionalOutputPayloadPart;
@@ -135,9 +135,9 @@

 pub use super::types::_payload_part::PayloadPart;

-pub use super::types::_invoke_model_tokens_request::InvokeModelTokensRequest;
+pub use super::types::_async_invoke_summary::AsyncInvokeSummary;

-pub use super::types::_converse_tokens_request::ConverseTokensRequest;
+pub use super::types::_tag::Tag;

 pub use super::types::_guardrail_text_block::GuardrailTextBlock;

@@ -159,8 +159,6 @@

 pub use super::types::_applied_guardrail_details::AppliedGuardrailDetails;

-pub use super::types::_guardrail_checks_role::GuardrailChecksRole;
-
 pub use super::types::_conversation_role::ConversationRole;

 pub use super::types::_guardrail_converse_content_block::GuardrailConverseContentBlock;
@@ -189,6 +187,8 @@

 pub use super::types::_converse_stream_trace::ConverseStreamTrace;

+pub use super::types::_guardrail_checks_role::GuardrailChecksRole;
+
 pub use super::types::_guardrail_image_format::GuardrailImageFormat;

 pub use super::types::_guardrail_image_source::GuardrailImageSource;
@@ -195,20 +195,6 @@

 pub use super::types::_guardrail_ownership::GuardrailOwnership;

-pub use super::types::_guardrail_checks_content_block::GuardrailChecksContentBlock;
-
-pub use super::types::_guardrail_checks_content_filter_category_config::GuardrailChecksContentFilterCategoryConfig;
-
-pub use super::types::_guardrail_checks_prompt_attack_category_config::GuardrailChecksPromptAttackCategoryConfig;
-
-pub use super::types::_guardrail_checks_sensitive_information_entity_config::GuardrailChecksSensitiveInformationEntityConfig;
-
-pub use super::types::_guardrail_checks_content_filter_result_entry::GuardrailChecksContentFilterResultEntry;
-
-pub use super::types::_guardrail_checks_prompt_attack_result_entry::GuardrailChecksPromptAttackResultEntry;
-
-pub use super::types::_guardrail_checks_sensitive_information_result_entry::GuardrailChecksSensitiveInformationResultEntry;
-
 pub use super::types::_content_block::ContentBlock;

 pub use super::types::_guardrail_converse_text_block::GuardrailConverseTextBlock;
@@ -239,6 +225,20 @@

 pub use super::types::_image_block_delta::ImageBlockDelta;

+pub use super::types::_guardrail_checks_content_block::GuardrailChecksContentBlock;
+
+pub use super::types::_guardrail_checks_content_filter_category_config::GuardrailChecksContentFilterCategoryConfig;
+
+pub use super::types::_guardrail_checks_prompt_attack_category_config::GuardrailChecksPromptAttackCategoryConfig;
+
+pub use super::types::_guardrail_checks_sensitive_information_entity_config::GuardrailChecksSensitiveInformationEntityConfig;
+
+pub use super::types::_guardrail_checks_content_filter_result_entry::GuardrailChecksContentFilterResultEntry;
+
+pub use super::types::_guardrail_checks_prompt_attack_result_entry::GuardrailChecksPromptAttackResultEntry;
+
+pub use super::types::_guardrail_checks_sensitive_information_result_entry::GuardrailChecksSensitiveInformationResultEntry;
+
 pub use super::types::_guardrail_content_qualifier::GuardrailContentQualifier;

 pub use super::types::_guardrail_topic::GuardrailTopic;
@@ -259,12 +259,6 @@

 pub use super::types::_guardrail_origin::GuardrailOrigin;

-pub use super::types::_guardrail_checks_content_filter_category::GuardrailChecksContentFilterCategory;
-
-pub use super::types::_guardrail_checks_prompt_attack_category::GuardrailChecksPromptAttackCategory;
-
-pub use super::types::_guardrail_checks_sensitive_information_entity_type::GuardrailChecksSensitiveInformationEntityType;
-
 pub use super::types::_image_block::ImageBlock;

 pub use super::types::_document_block::DocumentBlock;
@@ -307,6 +301,12 @@

 pub use super::types::_error_block::ErrorBlock;

+pub use super::types::_guardrail_checks_content_filter_category::GuardrailChecksContentFilterCategory;
+
+pub use super::types::_guardrail_checks_prompt_attack_category::GuardrailChecksPromptAttackCategory;
+
+pub use super::types::_guardrail_checks_sensitive_information_entity_type::GuardrailChecksSensitiveInformationEntityType;
+
 pub use super::types::_guardrail_topic_type::GuardrailTopicType;

 pub use super::types::_guardrail_topic_policy_action::GuardrailTopicPolicyAction;
```
