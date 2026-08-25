# AWS SDK Conformance Report: sqs

Snapshot: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`

## sqs
**Progress:** `294/294` files compared · `292` matched · `2` mismatches · `0` missing · `0` extra · `99.32%` match (100.00% means fully matched)

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
