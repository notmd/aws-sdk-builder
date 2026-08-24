# AWS SDK Conformance Report: sqs

Snapshot: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`

## sqs
**Progress:** `294/294` files compared · `287` matched · `6` mismatches · `1` missing · `0` extra · `97.62%` match (100.00% means fully matched)

### `src/aws_query_compatible_errors.rs`

```diff
--- reference/src/aws_query_compatible_errors.rs
+++ generated/src/aws_query_compatible_errors.rs
@@ -12,8 +12,8 @@
 /// Obtains custom error code and error type from the given `headers`.
 ///
 /// Looks up a value for the `X_AMZN_QUERY_ERROR` header and if found, the value should be in the
-/// form of `<error code>;<error type>`. The function then splits it into two parts and returns
-/// a (error code, error type) as a tuple.
+/// form of `<error code>;<error type>`. The function then splits it into two parts and returns a
+/// (error code, error type) as a tuple.
 ///
 /// Any execution path besides the above happy path will yield a `None`.
 pub fn parse_aws_query_compatible_error(headers: &Headers) -> Option<(&str, &str)> {
```

### `src/operation/change_message_visibility/builders.rs`

```diff
--- reference/src/operation/change_message_visibility/builders.rs
+++ generated/src/operation/change_message_visibility/builders.rs
@@ -35,7 +35,7 @@
 /// </ol>
 /// <p>A message is considered to be <i>stored</i> after it is sent to a queue by a producer, but not yet received from the queue by a consumer (that is, between states 1 and 2). There is no limit to the number of stored messages. A message is considered to be <i>in flight</i> after it is received from a queue by a consumer, but not yet deleted from the queue (that is, between states 2 and 3). There is a limit to the number of in flight messages.</p>
 /// <p>Limits that apply to in flight messages are unrelated to the <i>unlimited</i> number of stored messages.</p>
-/// <p>For most standard queues (depending on queue traffic and message backlog), there can be a maximum of approximately 120,000 in flight messages (received from a queue by a consumer, but not yet deleted from the queue). If you reach this limit, Amazon SQS returns the <code>OverLimit</code> error message. To avoid reaching the limit, you should delete messages from the queue after they're processed. You can also increase the number of queues you use to process your messages. To request a limit increase, <a href="https://console.aws.amazon.com/support/home#/case/create?issueType=service-limit-increase&amp;limitType=service-code-sqs">file a support request</a>.</p>
+/// <p>For most standard queues (depending on queue traffic and message backlog), there can be a maximum of approximately 120,000 in flight messages (received from a queue by a consumer, but not yet deleted from the queue). If you reach this limit, Amazon SQS returns the <code>OverLimit</code> error message. To avoid reaching the limit, you should delete messages from the queue after they're processed. You can also increase the number of queues you use to process your messages. To request a limit increase, <a href="https://console.aws.amazon.com/support/home#/case/create?issueType=service-limit-increase&limitType=service-code-sqs">file a support request</a>.</p>
 /// <p>For FIFO queues, there can be a maximum of 120,000 in flight messages (received from a queue by a consumer, but not yet deleted from the queue). If you reach this limit, Amazon SQS returns no error messages.</p><important>
 /// <p>If you attempt to set the <code>VisibilityTimeout</code> to a value greater than the maximum time left, Amazon SQS returns an error. Amazon SQS doesn't automatically recalculate and increase the timeout to the maximum remaining time.</p>
 /// <p>Unlike with a queue, when you change the visibility timeout for a specific message the timeout value is applied immediately but isn't saved in memory for that message. If you don't delete a message after it is received, the visibility timeout for the message reverts to the original timeout value (not to the value you set using the <code>ChangeMessageVisibility</code> action) the next time the message is received.</p>
```

### `src/operation/delete_message.rs`

```diff
--- reference/src/operation/delete_message.rs
+++ generated/src/operation/delete_message.rs
@@ -319,7 +319,6 @@
     /// <p>The specified ID is invalid.</p>
     InvalidAddress(super::super::types::error::InvalidAddress),
     /// <p>The specified receipt handle isn't valid for the current version.</p>
-    #[deprecated(note = "exception has been included in ReceiptHandleIsInvalid")]
     InvalidIdFormat(super::super::types::error::InvalidIdFormat),
     /// <p>The request was not made over HTTPS or did not use SigV4 for signing.</p>
     InvalidSecurity(super::super::types::error::InvalidSecurity),
```

### `src/operation/receive_message/_receive_message_input.rs`

```diff
--- reference/src/operation/receive_message/_receive_message_input.rs
+++ generated/src/operation/receive_message/_receive_message_input.rs
@@ -167,9 +167,9 @@
     /// <li>
     /// <p><code>SequenceNumber</code> – Returns the value provided by Amazon SQS.</p></li>
     /// </ul>
+    #[deprecated(note = "AttributeNames has been replaced by MessageSystemAttributeNames")]
     ///
     /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.attribute_names.is_none()`.
-    #[deprecated(note = "AttributeNames has been replaced by MessageSystemAttributeNames")]
     pub fn attribute_names(&self) -> &[super::super::super::types::QueueAttributeName] {
         self.attribute_names.as_deref().unwrap_or_default()
     }
```

### `src/operation/receive_message.rs`

```diff
--- reference/src/operation/receive_message.rs
+++ generated/src/operation/receive_message.rs
@@ -130,9 +130,6 @@
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
             ))
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-                super::super::long_polling::LongPollingInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ReceiveMessageEndpointParamsInterceptor,
             ))
             .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
```

### `src/protocol_serde.rs`

```diff
--- reference/src/protocol_serde.rs
+++ generated/src/protocol_serde.rs
@@ -20,12 +20,7 @@
     response_headers: &::aws_smithy_runtime_api::http::Headers,
     response_body: &[u8],
 ) -> ::std::result::Result<::aws_smithy_types::error::metadata::Builder, ::aws_smithy_json::deserialize::error::DeserializeError> {
-    let mut builder = super::json_errors::parse_error_metadata(response_body, response_headers)?;
-    if let Some((error_code, error_type)) = super::aws_query_compatible_errors::parse_aws_query_compatible_error(response_headers) {
-        builder = builder.code(error_code);
-        builder = builder.custom("type", error_type);
-    }
-    Ok(builder)
+    super::json_errors::parse_error_metadata(response_body, response_headers)
 }

 pub(crate) mod shape_add_permission;
@@ -74,8 +69,6 @@

 pub(crate) mod shape_untag_queue;

-pub(crate) mod shape_add_permission_input;
-
 pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
     if data.is_empty() {
         b"{}"
@@ -84,6 +77,8 @@
     }
 }

+pub(crate) mod shape_add_permission_input;
+
 pub(crate) mod shape_batch_entry_ids_not_distinct;

 pub(crate) mod shape_batch_request_too_long;
```

### Missing reference files

- `src/long_polling.rs`
