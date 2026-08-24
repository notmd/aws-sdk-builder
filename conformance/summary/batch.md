# AWS SDK Conformance Report: batch

Snapshot: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`

## batch
**Progress:** `762/762` files compared · `761` matched · `1` mismatches · `0` missing · `0` extra · `99.87%` match (100.00% means fully matched)

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
