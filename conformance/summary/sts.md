# AWS SDK Conformance Report: sts

Snapshot: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`

## sts
**Progress:** `146/146` files compared · `145` matched · `1` mismatches · `0` missing · `0` extra · `99.32%` match (100.00% means fully matched)

### `src/protocol_serde.rs`

```diff
--- reference/src/protocol_serde.rs
+++ generated/src/protocol_serde.rs
@@ -67,6 +67,12 @@

 pub(crate) mod shape_get_web_identity_token_input;

+pub(crate) mod shape_policy_descriptor_type;
+
+pub(crate) mod shape_provided_context;
+
+pub(crate) mod shape_tag;
+
 pub(crate) mod shape_expired_token_exception;

 pub(crate) mod shape_expired_trade_in_token_exception;
@@ -87,16 +93,10 @@

 pub(crate) mod shape_packed_policy_too_large_exception;

-pub(crate) mod shape_policy_descriptor_type;
-
-pub(crate) mod shape_provided_context;
-
 pub(crate) mod shape_region_disabled_exception;

 pub(crate) mod shape_session_duration_escalation_exception;

-pub(crate) mod shape_tag;
-
 pub(crate) mod shape_assumed_role_user;

 pub(crate) mod shape_credentials;
```
