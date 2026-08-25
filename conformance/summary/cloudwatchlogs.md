# AWS SDK Conformance Report: cloudwatchlogs

Snapshot: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`

## cloudwatchlogs
**Progress:** `1287/1287` files compared · `1282` matched · `5` mismatches · `0` missing · `0` extra · `99.61%` match (100.00% means fully matched)

### `src/operation/put_account_policy/builders.rs`

```diff
--- reference/src/operation/put_account_policy/builders.rs
+++ generated/src/operation/put_account_policy/builders.rs
@@ -89,7 +89,7 @@
 /// <p><b>Field index policy</b></p>
 /// <p>You can use field index policies to create indexes on fields found in log events for a log group or data source name and type combination. Creating field indexes can help lower the scan volume for CloudWatch Logs Insights queries that reference those fields, because these queries attempt to skip the processing of log events that are known to not match the indexed field. Good fields to index are fields that you often need to query for and fields or values that match only a small fraction of the total log events. Common examples of indexes include request ID, session ID, user IDs, or instance IDs. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/CloudWatchLogs-Field-Indexing.html">Create field indexes to improve query performance and reduce costs</a></p>
 /// <p>To find the fields that are in your log group events, use the <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_GetLogGroupFields.html">GetLogGroupFields</a> operation. To find the fields for a data source use the <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_GetLogFields.html">GetLogFields</a> operation.</p>
-/// <p>For example, suppose you have created a field index for <code>requestId</code>. Then, any CloudWatch Logs Insights query on that log group that includes <code>requestId = <i>value</i> </code> or <code>requestId in \[value, value, ...\]</code> will attempt to process only the log events where the indexed field matches the specified value.</p>
+/// <p>For example, suppose you have created a field index for <code>requestId</code>. Then, any CloudWatch Logs Insights query on that log group that includes <code>requestId = <i>value</i> </code> or <code>requestId in \[<i>value</i>, <i>value</i>, ...\]</code> will attempt to process only the log events where the indexed field matches the specified value.</p>
 /// <p>Matches of log events to the names of indexed fields are case-sensitive. For example, an indexed field of <code>RequestId</code> won't match a log event containing <code>requestId</code>.</p>
 /// <p>You can have one account-level field index policy that applies to all log groups in the account. Or you can create as many as 20 account-level field index policies that are each scoped to a subset of log groups using <code>LogGroupNamePrefix</code> with the <code>selectionCriteria</code> parameter. You can have another 20 account-level field index policies using <code>DataSourceName</code> and <code>DataSourceType</code> for the <code>selectionCriteria</code> parameter. If you have multiple account-level index policies with <code>LogGroupNamePrefix</code> selection criteria, no two of them can use the same or overlapping log group name prefixes. For example, if you have one policy filtered to log groups that start with <i>my-log</i>, you can't have another field index policy filtered to <i>my-logpprod</i> or <i>my-logging</i>. Similarly, if you have multiple account-level index policies with <code>DataSourceName</code> and <code>DataSourceType</code> selection criteria, no two of them can use the same data source name and type combination. For example, if you have one policy filtered to the data source name <code>amazon_vpc</code> and data source type <code>flow</code> you cannot create another policy with this combination.</p>
 /// <p>If you create an account-level field index policy in a monitoring account in cross-account observability, the policy is applied only to the monitoring account and not to any source accounts.</p>
```

### `src/operation/put_index_policy/builders.rs`

```diff
--- reference/src/operation/put_index_policy/builders.rs
+++ generated/src/operation/put_index_policy/builders.rs
@@ -26,7 +26,7 @@
 /// <p>You can use field index policies to create <i>field indexes</i> on fields found in log events in the log group. Creating field indexes speeds up and lowers the costs for CloudWatch Logs Insights queries that reference those field indexes, because these queries attempt to skip the processing of log events that are known to not match the indexed field. Good fields to index are fields that you often need to query for and fields or values that match only a small fraction of the total log events. Common examples of indexes include request ID, session ID, userID, and instance IDs. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/CloudWatchLogs-Field-Indexing.html">Create field indexes to improve query performance and reduce costs</a>.</p>
 /// <p>You can configure indexed fields as <i>facets</i> to enable interactive exploration and filtering of your logs in the CloudWatch Logs Insights console. Facets allow you to view value distributions and counts for indexed fields without running queries. When you create a field index, you can optionally set it as a facet to enable this interactive analysis capability. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/CloudWatchLogs-Facets.html">Use facets to group and explore logs</a>.</p>
 /// <p>To find the fields that are in your log group events, use the <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_GetLogGroupFields.html">GetLogGroupFields</a> operation.</p>
-/// <p>For example, suppose you have created a field index for <code>requestId</code>. Then, any CloudWatch Logs Insights query on that log group that includes <code>requestId = <i>value</i> </code> or <code>requestId IN \[value, value, ...\]</code> will process fewer log events to reduce costs, and have improved performance.</p>
+/// <p>For example, suppose you have created a field index for <code>requestId</code>. Then, any CloudWatch Logs Insights query on that log group that includes <code>requestId = <i>value</i> </code> or <code>requestId IN \[<i>value</i>, <i>value</i>, ...\]</code> will process fewer log events to reduce costs, and have improved performance.</p>
 /// <p>CloudWatch Logs provides default field indexes for all log groups in the Standard log class. Default field indexes are automatically available for the following fields:</p>
 /// <ul>
 /// <li>
```

### `src/protocol_serde.rs`

```diff
--- reference/src/protocol_serde.rs
+++ generated/src/protocol_serde.rs
@@ -259,6 +259,8 @@

 pub(crate) mod shape_update_scheduled_query;

+pub(crate) mod shape_associate_kms_key_input;
+
 pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
     if data.is_empty() {
         b"{}"
@@ -269,8 +271,6 @@

 pub(crate) mod shape_access_denied_exception;

-pub(crate) mod shape_associate_kms_key_input;
-
 pub(crate) mod shape_associate_source_to_s3_table_integration_input;

 pub(crate) mod shape_cancel_export_task_input;
@@ -845,6 +845,12 @@

 pub(crate) mod shape_inherited_properties;

+pub(crate) mod shape_input_log_stream_names;
+
+pub(crate) mod shape_live_tail_session_metadata;
+
+pub(crate) mod shape_live_tail_session_results;
+
 pub(crate) mod shape_log_field_type;

 pub(crate) mod shape_log_group_names;
@@ -887,6 +893,8 @@

 pub(crate) mod shape_split_string_entry;

+pub(crate) mod shape_start_live_tail_log_group_identifiers;
+
 pub(crate) mod shape_substitute_string_entry;

 pub(crate) mod shape_table_fields;
@@ -905,11 +913,7 @@

 pub(crate) mod shape_grouping_identifier;

-pub(crate) mod shape_input_log_stream_names;
-
-pub(crate) mod shape_live_tail_session_metadata;
-
-pub(crate) mod shape_live_tail_session_results;
+pub(crate) mod shape_live_tail_session_log_event;

 pub(crate) mod shape_log_event;

@@ -931,8 +935,6 @@

 pub(crate) mod shape_split_string_entries;

-pub(crate) mod shape_start_live_tail_log_group_identifiers;
-
 pub(crate) mod shape_substitute_string_entries;

 pub(crate) mod shape_trim_string_with_keys;
@@ -946,5 +948,3 @@
 pub(crate) mod shape_dimensions;

 pub(crate) mod shape_enumerations;
-
-pub(crate) mod shape_live_tail_session_log_event;
```

### `src/types/_substitute_string_entry.rs`

```diff
--- reference/src/types/_substitute_string_entry.rs
+++ generated/src/types/_substitute_string_entry.rs
@@ -6,7 +6,7 @@
 pub struct SubstituteStringEntry {
     /// <p>The key to modify</p>
     pub source: ::std::string::String,
-    /// <p>The regular expression string to be replaced. Special regex characters such as \[ and \] must be escaped using \\ when using double quotes and with \ when using single quotes. For more information, see Class Pattern on the Oracle web site.</p>
+    /// <p>The regular expression string to be replaced. Special regex characters such as \[ and \] must be escaped using \\ when using double quotes and with \ when using single quotes. For more information, see <a href="https://docs.oracle.com/en/java/javase/17/docs/api/java.base/java/util/regex/Pattern.html"> Class Pattern</a> on the Oracle web site.</p>
     pub from: ::std::string::String,
     /// <p>The string to be substituted for each match of <code>from</code></p>
     pub to: ::std::string::String,
@@ -17,7 +17,7 @@
         use std::ops::Deref;
         self.source.deref()
     }
-    /// <p>The regular expression string to be replaced. Special regex characters such as \[ and \] must be escaped using \\ when using double quotes and with \ when using single quotes. For more information, see Class Pattern on the Oracle web site.</p>
+    /// <p>The regular expression string to be replaced. Special regex characters such as \[ and \] must be escaped using \\ when using double quotes and with \ when using single quotes. For more information, see <a href="https://docs.oracle.com/en/java/javase/17/docs/api/java.base/java/util/regex/Pattern.html"> Class Pattern</a> on the Oracle web site.</p>
     pub fn from(&self) -> &str {
         use std::ops::Deref;
         self.from.deref()
@@ -59,18 +59,18 @@
     pub fn get_source(&self) -> &::std::option::Option<::std::string::String> {
         &self.source
     }
-    /// <p>The regular expression string to be replaced. Special regex characters such as \[ and \] must be escaped using \\ when using double quotes and with \ when using single quotes. For more information, see Class Pattern on the Oracle web site.</p>
+    /// <p>The regular expression string to be replaced. Special regex characters such as \[ and \] must be escaped using \\ when using double quotes and with \ when using single quotes. For more information, see <a href="https://docs.oracle.com/en/java/javase/17/docs/api/java.base/java/util/regex/Pattern.html"> Class Pattern</a> on the Oracle web site.</p>
     /// This field is required.
     pub fn from(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
         self.from = ::std::option::Option::Some(input.into());
         self
     }
-    /// <p>The regular expression string to be replaced. Special regex characters such as \[ and \] must be escaped using \\ when using double quotes and with \ when using single quotes. For more information, see Class Pattern on the Oracle web site.</p>
+    /// <p>The regular expression string to be replaced. Special regex characters such as \[ and \] must be escaped using \\ when using double quotes and with \ when using single quotes. For more information, see <a href="https://docs.oracle.com/en/java/javase/17/docs/api/java.base/java/util/regex/Pattern.html"> Class Pattern</a> on the Oracle web site.</p>
     pub fn set_from(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
         self.from = input;
         self
     }
-    /// <p>The regular expression string to be replaced. Special regex characters such as \[ and \] must be escaped using \\ when using double quotes and with \ when using single quotes. For more information, see Class Pattern on the Oracle web site.</p>
+    /// <p>The regular expression string to be replaced. Special regex characters such as \[ and \] must be escaped using \\ when using double quotes and with \ when using single quotes. For more information, see <a href="https://docs.oracle.com/en/java/javase/17/docs/api/java.base/java/util/regex/Pattern.html"> Class Pattern</a> on the Oracle web site.</p>
     pub fn get_from(&self) -> &::std::option::Option<::std::string::String> {
         &self.from
     }
```

### `src/types/error.rs`

```diff
--- reference/src/types/error.rs
+++ generated/src/types/error.rs
@@ -84,7 +84,7 @@
             Self::Unhandled(e) => &e.meta,
         }
     }
-    /// Returns `true` if the error kind is `GetLogObjectResponseStreamError::InternalStreamingException`.
+    /// Returns `true` if the error kind is `InternalStreamingException::InternalStreamingException`.
     pub fn is_internal_streaming_exception(&self) -> bool {
         matches!(self, Self::InternalStreamingException(_))
     }
@@ -122,7 +122,7 @@
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for GetLogObjectResponseStreamError {
     fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::InternalStreamingException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::InternalStreamingException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
             Self::Unhandled(_inner) => &_inner.meta,
         }
     }
@@ -143,7 +143,6 @@
         self.meta().request_id()
     }
 }
-
 /// Error type for the `StartLiveTailResponseStreamError` operation.
 #[non_exhaustive]
 #[derive(::std::fmt::Debug)]
@@ -190,11 +189,11 @@
             Self::Unhandled(e) => &e.meta,
         }
     }
-    /// Returns `true` if the error kind is `StartLiveTailResponseStreamError::SessionTimeoutException`.
+    /// Returns `true` if the error kind is `SessionTimeoutException::SessionTimeoutException`.
     pub fn is_session_timeout_exception(&self) -> bool {
         matches!(self, Self::SessionTimeoutException(_))
     }
-    /// Returns `true` if the error kind is `StartLiveTailResponseStreamError::SessionStreamingException`.
+    /// Returns `true` if the error kind is `SessionStreamingException::SessionStreamingException`.
     pub fn is_session_streaming_exception(&self) -> bool {
         matches!(self, Self::SessionStreamingException(_))
     }
@@ -234,8 +233,8 @@
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for StartLiveTailResponseStreamError {
     fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::SessionTimeoutException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::SessionStreamingException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::SessionTimeoutException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::SessionStreamingException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
             Self::Unhandled(_inner) => &_inner.meta,
         }
     }
@@ -256,7 +255,6 @@
         self.meta().request_id()
     }
 }
-
 mod _access_denied_exception;

 mod _conflict_exception;
```
