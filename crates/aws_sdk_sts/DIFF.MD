# AWS SDK modularizer diff

- repository: `https://github.com/awslabs/aws-sdk-rust`
- revision: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`
- service: `sdk/sts`
- model: `aws-models/sts.json`
- package: `aws-sdk-sts`
- library: `aws_sdk_sts`
- tests exclusion: `tests/**` is excluded from the comparison.

## Operations

- `op_assume_role`: `AssumeRole`
- `op_assume_role_with_saml`: `AssumeRoleWithSAML`
- `op_assume_role_with_web_identity`: `AssumeRoleWithWebIdentity`
- `op_assume_root`: `AssumeRoot`
- `op_decode_authorization_message`: `DecodeAuthorizationMessage`
- `op_get_access_key_info`: `GetAccessKeyInfo`
- `op_get_caller_identity`: `GetCallerIdentity`
- `op_get_delegated_access_token`: `GetDelegatedAccessToken`
- `op_get_federation_token`: `GetFederationToken`
- `op_get_session_token`: `GetSessionToken`
- `op_get_web_identity_token`: `GetWebIdentityToken`

## Changed files

- `Cargo.toml`
- `src/client.rs`
- `src/client/assume_role_with_saml.rs`
- `src/client/assume_role_with_web_identity.rs`
- `src/client/customize.rs`
- `src/client/customize/internal.rs`
- `src/client/decode_authorization_message.rs`
- `src/client/get_access_key_info.rs`
- `src/client/get_caller_identity.rs`
- `src/client/get_delegated_access_token.rs`
- `src/client/get_federation_token.rs`
- `src/client/get_web_identity_token.rs`
- `src/config.rs`
- `src/config/auth.rs`
- `src/config/endpoint.rs`
- `src/endpoint_lib.rs`
- `src/endpoint_lib/bdd_interpreter.rs`
- `src/endpoint_lib/host.rs`
- `src/endpoint_lib/partition.rs`
- `src/error.rs`
- `src/error/sealed_unhandled.rs`
- `src/error_meta.rs`
- `src/meta.rs`
- `src/observability_feature.rs`
- `src/operation.rs`
- `src/operation/assume_role.rs`
- `src/operation/assume_role/_assume_role_input.rs`
- `src/operation/assume_role/builders.rs`
- `src/operation/assume_role_with_saml.rs`
- `src/operation/assume_role_with_saml/_assume_role_with_saml_input.rs`
- `src/operation/assume_role_with_saml/builders.rs`
- `src/operation/assume_role_with_web_identity.rs`
- `src/operation/assume_role_with_web_identity/_assume_role_with_web_identity_input.rs`
- `src/operation/assume_role_with_web_identity/_assume_role_with_web_identity_output.rs`
- `src/operation/assume_role_with_web_identity/builders.rs`
- `src/operation/assume_root.rs`
- `src/operation/assume_root/_assume_root_input.rs`
- `src/operation/assume_root/builders.rs`
- `src/operation/decode_authorization_message.rs`
- `src/operation/decode_authorization_message/_decode_authorization_message_input.rs`
- `src/operation/decode_authorization_message/_decode_authorization_message_output.rs`
- `src/operation/decode_authorization_message/builders.rs`
- `src/operation/get_access_key_info.rs`
- `src/operation/get_access_key_info/_get_access_key_info_input.rs`
- `src/operation/get_access_key_info/builders.rs`
- `src/operation/get_caller_identity.rs`
- `src/operation/get_caller_identity/_get_caller_identity_input.rs`
- `src/operation/get_caller_identity/builders.rs`
- `src/operation/get_delegated_access_token.rs`
- `src/operation/get_delegated_access_token/_get_delegated_access_token_input.rs`
- `src/operation/get_delegated_access_token/builders.rs`
- `src/operation/get_federation_token.rs`
- `src/operation/get_federation_token/_get_federation_token_input.rs`
- `src/operation/get_federation_token/builders.rs`
- `src/operation/get_session_token.rs`
- `src/operation/get_session_token/_get_session_token_input.rs`
- `src/operation/get_session_token/builders.rs`
- `src/operation/get_web_identity_token.rs`
- `src/operation/get_web_identity_token/_get_web_identity_token_input.rs`
- `src/operation/get_web_identity_token/builders.rs`
- `src/protocol_serde.rs`
- `src/protocol_serde/shape_assume_role.rs`
- `src/protocol_serde/shape_assume_role_input.rs`
- `src/protocol_serde/shape_assume_role_with_saml.rs`
- `src/protocol_serde/shape_assume_role_with_saml_input.rs`
- `src/protocol_serde/shape_assume_role_with_web_identity.rs`
- `src/protocol_serde/shape_assume_role_with_web_identity_input.rs`
- `src/protocol_serde/shape_assume_root.rs`
- `src/protocol_serde/shape_assume_root_input.rs`
- `src/protocol_serde/shape_assumed_role_user.rs`
- `src/protocol_serde/shape_credentials.rs`
- `src/protocol_serde/shape_decode_authorization_message.rs`
- `src/protocol_serde/shape_decode_authorization_message_input.rs`
- `src/protocol_serde/shape_expired_token_exception.rs`
- `src/protocol_serde/shape_expired_trade_in_token_exception.rs`
- `src/protocol_serde/shape_federated_user.rs`
- `src/protocol_serde/shape_get_access_key_info.rs`
- `src/protocol_serde/shape_get_access_key_info_input.rs`
- `src/protocol_serde/shape_get_caller_identity.rs`
- `src/protocol_serde/shape_get_caller_identity_input.rs`
- `src/protocol_serde/shape_get_delegated_access_token.rs`
- `src/protocol_serde/shape_get_delegated_access_token_input.rs`
- `src/protocol_serde/shape_get_federation_token.rs`
- `src/protocol_serde/shape_get_federation_token_input.rs`
- `src/protocol_serde/shape_get_session_token.rs`
- `src/protocol_serde/shape_get_session_token_input.rs`
- `src/protocol_serde/shape_get_web_identity_token.rs`
- `src/protocol_serde/shape_get_web_identity_token_input.rs`
- `src/protocol_serde/shape_idp_communication_error_exception.rs`
- `src/protocol_serde/shape_idp_rejected_claim_exception.rs`
- `src/protocol_serde/shape_invalid_authorization_message_exception.rs`
- `src/protocol_serde/shape_invalid_identity_token_exception.rs`
- `src/protocol_serde/shape_jwt_payload_size_exceeded_exception.rs`
- `src/protocol_serde/shape_malformed_policy_document_exception.rs`
- `src/protocol_serde/shape_outbound_web_identity_federation_disabled_exception.rs`
- `src/protocol_serde/shape_packed_policy_too_large_exception.rs`
- `src/protocol_serde/shape_region_disabled_exception.rs`
- `src/protocol_serde/shape_session_duration_escalation_exception.rs`
- `src/sdk_feature_tracker.rs`
- `src/serde_util.rs`
- `src/serialization_settings.rs`
- `src/types.rs`
- `src/types/_assumed_role_user.rs`
- `src/types/_credentials.rs`
- `src/types/_federated_user.rs`
- `src/types/_tag.rs`
- `src/types/builders.rs`
- `src/types/error.rs`
- `src/types/error/builders.rs`

## File diffs

### `Cargo.toml`

```diff
diff --git a/Cargo.toml b/Cargo.toml
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -19,58 +19,45 @@
 version = "1"

 [dependencies.aws-credential-types]
-path = "../aws-credential-types"
 version = "1.3.0"

 [dependencies.aws-runtime]
-path = "../aws-runtime"
 version = "1.9.1"

 [dependencies.aws-smithy-async]
-path = "../aws-smithy-async"
 version = "1.3.0"

 [dependencies.aws-smithy-http]
-path = "../aws-smithy-http"
 version = "0.64.0"

 [dependencies.aws-smithy-json]
-path = "../aws-smithy-json"
 version = "0.63.0"

 [dependencies.aws-smithy-observability]
-path = "../aws-smithy-observability"
 version = "0.3.0"

 [dependencies.aws-smithy-query]
-path = "../aws-smithy-query"
 version = "0.62.0"

 [dependencies.aws-smithy-runtime]
-path = "../aws-smithy-runtime"
 features = ["client"]
 version = "1.14.0"

 [dependencies.aws-smithy-runtime-api]
-path = "../aws-smithy-runtime-api"
 features = ["client", "http-1x"]
 version = "1.15.0"

 [dependencies.aws-smithy-schema]
-path = "../aws-smithy-schema"
 version = "0.2.0"

 [dependencies.aws-smithy-types]
-path = "../aws-smithy-types"
 features = ["http-body-1-x"]
 version = "1.6.2"

 [dependencies.aws-smithy-xml]
-path = "../aws-smithy-xml"
 version = "0.62.0"

 [dependencies.aws-types]
-path = "../aws-types"
 version = "1.5.0"

 [dependencies.fastrand]
@@ -89,41 +76,33 @@
 [dependencies.tracing]
 version = "0.1"
 [dev-dependencies.aws-credential-types]
-path = "../aws-credential-types"
 features = ["test-util"]
 version = "1.3.0"

 [dev-dependencies.aws-runtime]
-path = "../aws-runtime"
 features = ["test-util"]
 version = "1.9.1"

 [dev-dependencies.aws-smithy-async]
-path = "../aws-smithy-async"
 features = ["test-util"]
 version = "1.3.0"

 [dev-dependencies.aws-smithy-http-client]
-path = "../aws-smithy-http-client"
 features = ["test-util", "wire-mock"]
 version = "1.4.0"

 [dev-dependencies.aws-smithy-protocol-test]
-path = "../aws-smithy-protocol-test"
 version = "0.64.0"

 [dev-dependencies.aws-smithy-runtime]
-path = "../aws-smithy-runtime"
 features = ["test-util"]
 version = "1.14.0"

 [dev-dependencies.aws-smithy-runtime-api]
-path = "../aws-smithy-runtime-api"
 features = ["test-util"]
 version = "1.15.0"

 [dev-dependencies.aws-smithy-types]
-path = "../aws-smithy-types"
 features = ["http-body-1-x", "test-util"]
 version = "1.6.2"

@@ -155,6 +134,20 @@
 test-util = ["aws-credential-types/test-util", "aws-smithy-runtime/legacy-test-util", "aws-smithy-runtime/test-util"]
 gated-tests = []
 default = ["sigv4a", "rustls", "default-https-client", "rt-tokio"]
+op_assume_role = []
+op_assume_role_with_saml = []
+op_assume_role_with_web_identity = []
+op_assume_root = []
+op_decode_authorization_message = []
+op_get_access_key_info = []
+op_get_caller_identity = []
+op_get_delegated_access_token = []
+op_get_federation_token = []
+op_get_session_token = []
+op_get_web_identity_token = []

 [hints]
 mostly-unused = true
+
+[lib]
+name = "aws_sdk_sts"
```

### `src/client.rs`

```diff
diff --git a/src/client.rs b/src/client.rs
--- a/src/client.rs
+++ b/src/client.rs
@@ -90,12 +90,16 @@
     }
 }

+#[cfg(feature = "op_assume_role")]
 mod assume_role;

+#[cfg(feature = "op_assume_role_with_saml")]
 mod assume_role_with_saml;

+#[cfg(feature = "op_assume_role_with_web_identity")]
 mod assume_role_with_web_identity;

+#[cfg(feature = "op_assume_root")]
 mod assume_root;

 /// Operation customization and supporting types.
@@ -125,16 +129,23 @@
 /// ```
 pub mod customize;

+#[cfg(feature = "op_decode_authorization_message")]
 mod decode_authorization_message;

+#[cfg(feature = "op_get_access_key_info")]
 mod get_access_key_info;

+#[cfg(feature = "op_get_caller_identity")]
 mod get_caller_identity;

+#[cfg(feature = "op_get_delegated_access_token")]
 mod get_delegated_access_token;

+#[cfg(feature = "op_get_federation_token")]
 mod get_federation_token;

+#[cfg(feature = "op_get_session_token")]
 mod get_session_token;

+#[cfg(feature = "op_get_web_identity_token")]
 mod get_web_identity_token;
```

### `src/client/assume_role_with_saml.rs`

```diff
diff --git a/src/client/assume_role_with_saml.rs b/src/client/assume_role_with_saml.rs
--- a/src/client/assume_role_with_saml.rs
+++ b/src/client/assume_role_with_saml.rs
@@ -20,7 +20,9 @@
     ///   - [`name_qualifier(Option<String>)`](crate::operation::assume_role_with_saml::AssumeRoleWithSamlOutput::name_qualifier): <p>A hash value based on the concatenation of the following:</p> <ul>  <li>   <p>The <code>Issuer</code> response value.</p></li>  <li>   <p>The Amazon Web Services account ID.</p></li>  <li>   <p>The friendly name (the last part of the ARN) of the SAML provider in IAM.</p></li> </ul> <p>The combination of <code>NameQualifier</code> and <code>Subject</code> can be used to uniquely identify a user.</p> <p>The following pseudocode shows how the hash value is calculated:</p> <p><code>BASE64 ( SHA1 ( "https://example.com/saml" + "123456789012" + "/MySAMLIdP" ) )</code></p>
     ///   - [`source_identity(Option<String>)`](crate::operation::assume_role_with_saml::AssumeRoleWithSamlOutput::source_identity): <p>The value in the <code>SourceIdentity</code> attribute in the SAML assertion. The source identity value persists across <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles.html#iam-term-role-chaining">chained role</a> sessions.</p> <p>You can require users to set a source identity value when they assume a role. You do this by using the <code>sts:SourceIdentity</code> condition key in a role trust policy. That way, actions that are taken with the role are associated with that user. After the source identity is set, the value cannot be changed. It is present in the request for all actions that are taken by the role and persists across <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles.html#id_roles_terms-and-concepts">chained role</a> sessions. You can configure your SAML identity provider to use an attribute associated with your users, like user name or email, as the source identity when calling <code>AssumeRoleWithSAML</code>. You do this by adding an attribute to the SAML assertion. For more information about using source identity, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_control-access_monitor.html">Monitor and control actions taken with assumed roles</a> in the <i>IAM User Guide</i>.</p> <p>The regex used to validate this parameter is a string of characters consisting of upper- and lower-case alphanumeric characters with no spaces. You can also include underscores or any of the following characters: =,.@-</p>
     /// - On failure, responds with [`SdkError<AssumeRoleWithSAMLError>`](crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError)
-    pub fn assume_role_with_saml(&self) -> crate::operation::assume_role_with_saml::builders::AssumeRoleWithSAMLFluentBuilder {
+    pub fn assume_role_with_saml(
+        &self,
+    ) -> crate::operation::assume_role_with_saml::builders::AssumeRoleWithSAMLFluentBuilder {
         crate::operation::assume_role_with_saml::builders::AssumeRoleWithSAMLFluentBuilder::new(self.handle.clone())
     }
 }
```

### `src/client/assume_role_with_web_identity.rs`

```diff
diff --git a/src/client/assume_role_with_web_identity.rs b/src/client/assume_role_with_web_identity.rs
--- a/src/client/assume_role_with_web_identity.rs
+++ b/src/client/assume_role_with_web_identity.rs
@@ -19,7 +19,11 @@
     ///   - [`audience(Option<String>)`](crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityOutput::audience): <p>The intended audience (also known as client ID) of the web identity token. This is traditionally the client identifier issued to the application that requested the web identity token.</p>
     ///   - [`source_identity(Option<String>)`](crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityOutput::source_identity): <p>The value of the source identity that is returned in the JSON web token (JWT) from the identity provider.</p> <p>You can require users to set a source identity value when they assume a role. You do this by using the <code>sts:SourceIdentity</code> condition key in a role trust policy. That way, actions that are taken with the role are associated with that user. After the source identity is set, the value cannot be changed. It is present in the request for all actions that are taken by the role and persists across <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles.html#id_roles_terms-and-concepts">chained role</a> sessions. You can configure your identity provider to use an attribute associated with your users, like user name or email, as the source identity when calling <code>AssumeRoleWithWebIdentity</code>. You do this by adding a claim to the JSON web token. To learn more about OIDC tokens and claims, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-using-tokens-with-identity-providers.html">Using Tokens with User Pools</a> in the <i>Amazon Cognito Developer Guide</i>. For more information about using source identity, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_control-access_monitor.html">Monitor and control actions taken with assumed roles</a> in the <i>IAM User Guide</i>.</p> <p>The regex used to validate this parameter is a string of characters consisting of upper- and lower-case alphanumeric characters with no spaces. You can also include underscores or any of the following characters: =,.@-</p>
     /// - On failure, responds with [`SdkError<AssumeRoleWithWebIdentityError>`](crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError)
-    pub fn assume_role_with_web_identity(&self) -> crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityFluentBuilder {
-        crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityFluentBuilder::new(self.handle.clone())
+    pub fn assume_role_with_web_identity(
+        &self,
+    ) -> crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityFluentBuilder {
+        crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityFluentBuilder::new(
+            self.handle.clone(),
+        )
     }
 }
```

### `src/client/customize.rs`

```diff
diff --git a/src/client/customize.rs b/src/client/customize.rs
--- a/src/client/customize.rs
+++ b/src/client/customize.rs
@@ -41,15 +41,23 @@
     /// `map_request`, and `mutate_request` (the last two are implemented via interceptors under the hood).
     /// The order in which those user-specified operation interceptors are invoked should not be relied upon
     /// as it is an implementation detail.
-    pub fn interceptor(mut self, interceptor: impl ::aws_smithy_runtime_api::client::interceptors::Intercept + 'static) -> Self {
+    pub fn interceptor(
+        mut self,
+        interceptor: impl ::aws_smithy_runtime_api::client::interceptors::Intercept + 'static,
+    ) -> Self {
         self.interceptors
-            .push(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::new(interceptor));
+            .push(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::new(
+                interceptor,
+            ));
         self
     }

     /// Adds a runtime plugin.
     #[allow(unused)]
-    pub(crate) fn runtime_plugin(mut self, runtime_plugin: impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin + 'static) -> Self {
+    pub(crate) fn runtime_plugin(
+        mut self,
+        runtime_plugin: impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin + 'static,
+    ) -> Self {
         self.runtime_plugins
             .push(::aws_smithy_runtime_api::client::runtime_plugin::SharedRuntimePlugin::new(runtime_plugin));
         self
@@ -60,7 +68,8 @@
     where
         F: ::std::ops::Fn(
                 ::aws_smithy_runtime_api::client::orchestrator::HttpRequest,
-            ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, MapE>
+            )
+                -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, MapE>
             + ::std::marker::Send
             + ::std::marker::Sync
             + 'static,
@@ -76,7 +85,10 @@
     /// Convenience for `map_request` where infallible direct mutation of request is acceptable.
     pub fn mutate_request<F>(mut self, f: F) -> Self
     where
-        F: ::std::ops::Fn(&mut ::aws_smithy_runtime_api::client::orchestrator::HttpRequest) + ::std::marker::Send + ::std::marker::Sync + 'static,
+        F: ::std::ops::Fn(&mut ::aws_smithy_runtime_api::client::orchestrator::HttpRequest)
+            + ::std::marker::Send
+            + ::std::marker::Sync
+            + 'static,
     {
         self.interceptors
             .push(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::new(
```

### `src/client/customize/internal.rs`

```diff
diff --git a/src/client/customize/internal.rs b/src/client/customize/internal.rs
--- a/src/client/customize/internal.rs
+++ b/src/client/customize/internal.rs
@@ -1,8 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub type BoxFuture<T> = ::std::pin::Pin<::std::boxed::Box<dyn ::std::future::Future<Output = T> + ::std::marker::Send>>;

-pub type SendResult<T, E> =
-    ::std::result::Result<T, ::aws_smithy_runtime_api::client::result::SdkError<E, ::aws_smithy_runtime_api::client::orchestrator::HttpResponse>>;
+pub type SendResult<T, E> = ::std::result::Result<
+    T,
+    ::aws_smithy_runtime_api::client::result::SdkError<E, ::aws_smithy_runtime_api::client::orchestrator::HttpResponse>,
+>;

 pub trait CustomizableSend<T, E>: ::std::marker::Send + ::std::marker::Sync {
     // Takes an owned `self` as the implementation will internally call methods that take `self`.
```

### `src/client/decode_authorization_message.rs`

```diff
diff --git a/src/client/decode_authorization_message.rs b/src/client/decode_authorization_message.rs
--- a/src/client/decode_authorization_message.rs
+++ b/src/client/decode_authorization_message.rs
@@ -7,7 +7,11 @@
     /// - On success, responds with [`DecodeAuthorizationMessageOutput`](crate::operation::decode_authorization_message::DecodeAuthorizationMessageOutput) with field(s):
     ///   - [`decoded_message(Option<String>)`](crate::operation::decode_authorization_message::DecodeAuthorizationMessageOutput::decoded_message): <p>The API returns a response with the decoded message.</p>
     /// - On failure, responds with [`SdkError<DecodeAuthorizationMessageError>`](crate::operation::decode_authorization_message::DecodeAuthorizationMessageError)
-    pub fn decode_authorization_message(&self) -> crate::operation::decode_authorization_message::builders::DecodeAuthorizationMessageFluentBuilder {
-        crate::operation::decode_authorization_message::builders::DecodeAuthorizationMessageFluentBuilder::new(self.handle.clone())
+    pub fn decode_authorization_message(
+        &self,
+    ) -> crate::operation::decode_authorization_message::builders::DecodeAuthorizationMessageFluentBuilder {
+        crate::operation::decode_authorization_message::builders::DecodeAuthorizationMessageFluentBuilder::new(
+            self.handle.clone(),
+        )
     }
 }
```

### `src/client/get_access_key_info.rs`

```diff
diff --git a/src/client/get_access_key_info.rs b/src/client/get_access_key_info.rs
--- a/src/client/get_access_key_info.rs
+++ b/src/client/get_access_key_info.rs
@@ -7,7 +7,9 @@
     /// - On success, responds with [`GetAccessKeyInfoOutput`](crate::operation::get_access_key_info::GetAccessKeyInfoOutput) with field(s):
     ///   - [`account(Option<String>)`](crate::operation::get_access_key_info::GetAccessKeyInfoOutput::account): <p>The number used to identify the Amazon Web Services account.</p>
     /// - On failure, responds with [`SdkError<GetAccessKeyInfoError>`](crate::operation::get_access_key_info::GetAccessKeyInfoError)
-    pub fn get_access_key_info(&self) -> crate::operation::get_access_key_info::builders::GetAccessKeyInfoFluentBuilder {
+    pub fn get_access_key_info(
+        &self,
+    ) -> crate::operation::get_access_key_info::builders::GetAccessKeyInfoFluentBuilder {
         crate::operation::get_access_key_info::builders::GetAccessKeyInfoFluentBuilder::new(self.handle.clone())
     }
 }
```

### `src/client/get_caller_identity.rs`

```diff
diff --git a/src/client/get_caller_identity.rs b/src/client/get_caller_identity.rs
--- a/src/client/get_caller_identity.rs
+++ b/src/client/get_caller_identity.rs
@@ -8,7 +8,9 @@
     ///   - [`account(Option<String>)`](crate::operation::get_caller_identity::GetCallerIdentityOutput::account): <p>The Amazon Web Services account ID number of the account that owns or contains the calling entity.</p>
     ///   - [`arn(Option<String>)`](crate::operation::get_caller_identity::GetCallerIdentityOutput::arn): <p>The Amazon Web Services ARN associated with the calling entity.</p>
     /// - On failure, responds with [`SdkError<GetCallerIdentityError>`](crate::operation::get_caller_identity::GetCallerIdentityError)
-    pub fn get_caller_identity(&self) -> crate::operation::get_caller_identity::builders::GetCallerIdentityFluentBuilder {
+    pub fn get_caller_identity(
+        &self,
+    ) -> crate::operation::get_caller_identity::builders::GetCallerIdentityFluentBuilder {
         crate::operation::get_caller_identity::builders::GetCallerIdentityFluentBuilder::new(self.handle.clone())
     }
 }
```

### `src/client/get_delegated_access_token.rs`

```diff
diff --git a/src/client/get_delegated_access_token.rs b/src/client/get_delegated_access_token.rs
--- a/src/client/get_delegated_access_token.rs
+++ b/src/client/get_delegated_access_token.rs
@@ -9,7 +9,11 @@
     ///   - [`packed_policy_size(Option<i32>)`](crate::operation::get_delegated_access_token::GetDelegatedAccessTokenOutput::packed_policy_size): <p>The percentage of the maximum policy size that is used by the session policy. The policy size is calculated as the sum of all the session policies and permission boundaries attached to the session. If the packed size exceeds 100%, the request fails.</p>
     ///   - [`assumed_principal(Option<String>)`](crate::operation::get_delegated_access_token::GetDelegatedAccessTokenOutput::assumed_principal): <p>The Amazon Resource Name (ARN) of the principal that was assumed when obtaining the delegated access token. This ARN identifies the IAM entity whose permissions are granted by the temporary credentials.</p>
     /// - On failure, responds with [`SdkError<GetDelegatedAccessTokenError>`](crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError)
-    pub fn get_delegated_access_token(&self) -> crate::operation::get_delegated_access_token::builders::GetDelegatedAccessTokenFluentBuilder {
-        crate::operation::get_delegated_access_token::builders::GetDelegatedAccessTokenFluentBuilder::new(self.handle.clone())
+    pub fn get_delegated_access_token(
+        &self,
+    ) -> crate::operation::get_delegated_access_token::builders::GetDelegatedAccessTokenFluentBuilder {
+        crate::operation::get_delegated_access_token::builders::GetDelegatedAccessTokenFluentBuilder::new(
+            self.handle.clone(),
+        )
     }
 }
```

### `src/client/get_federation_token.rs`

```diff
diff --git a/src/client/get_federation_token.rs b/src/client/get_federation_token.rs
--- a/src/client/get_federation_token.rs
+++ b/src/client/get_federation_token.rs
@@ -13,7 +13,9 @@
     ///   - [`federated_user(Option<FederatedUser>)`](crate::operation::get_federation_token::GetFederationTokenOutput::federated_user): <p>Identifiers for the federated user associated with the credentials (such as <code>arn:aws:sts::123456789012:federated-user/Bob</code> or <code>123456789012:Bob</code>). You can use the federated user's ARN in your resource-based policies, such as an Amazon S3 bucket policy.</p>
     ///   - [`packed_policy_size(Option<i32>)`](crate::operation::get_federation_token::GetFederationTokenOutput::packed_policy_size): <p>A percentage value that indicates the packed size of the session policies and session tags combined passed in the request. The request fails if the packed size is greater than 100 percent, which means the policies and tags exceeded the allowed space.</p>
     /// - On failure, responds with [`SdkError<GetFederationTokenError>`](crate::operation::get_federation_token::GetFederationTokenError)
-    pub fn get_federation_token(&self) -> crate::operation::get_federation_token::builders::GetFederationTokenFluentBuilder {
+    pub fn get_federation_token(
+        &self,
+    ) -> crate::operation::get_federation_token::builders::GetFederationTokenFluentBuilder {
         crate::operation::get_federation_token::builders::GetFederationTokenFluentBuilder::new(self.handle.clone())
     }
 }
```

### `src/client/get_web_identity_token.rs`

```diff
diff --git a/src/client/get_web_identity_token.rs b/src/client/get_web_identity_token.rs
--- a/src/client/get_web_identity_token.rs
+++ b/src/client/get_web_identity_token.rs
@@ -11,7 +11,9 @@
     ///   - [`web_identity_token(Option<String>)`](crate::operation::get_web_identity_token::GetWebIdentityTokenOutput::web_identity_token): <p>A signed JSON Web Token (JWT) that represents the caller's Amazon Web Services identity. The token contains standard JWT claims such as subject, audience, expiration time, and additional identity attributes added by STS as custom claims. You can also add your own custom claims to the token by passing tags as request parameters to the <code>GetWebIdentityToken</code> API. The token is signed using the specified signing algorithm and can be verified using the verification keys available at the issuer's JWKS endpoint.</p>
     ///   - [`expiration(Option<DateTime>)`](crate::operation::get_web_identity_token::GetWebIdentityTokenOutput::expiration): <p>The date and time when the web identity token expires, in UTC. The expiration is determined by adding the <code>DurationSeconds</code> value to the time the token was issued. After this time, the token should no longer be considered valid.</p>
     /// - On failure, responds with [`SdkError<GetWebIdentityTokenError>`](crate::operation::get_web_identity_token::GetWebIdentityTokenError)
-    pub fn get_web_identity_token(&self) -> crate::operation::get_web_identity_token::builders::GetWebIdentityTokenFluentBuilder {
+    pub fn get_web_identity_token(
+        &self,
+    ) -> crate::operation::get_web_identity_token::builders::GetWebIdentityTokenFluentBuilder {
         crate::operation::get_web_identity_token::builders::GetWebIdentityTokenFluentBuilder::new(self.handle.clone())
     }
 }
```

### `src/config.rs`

```diff
diff --git a/src/config.rs b/src/config.rs
--- a/src/config.rs
+++ b/src/config.rs
@@ -60,16 +60,23 @@
     }

     /// Return the auth scheme resolver configured on this service config
-    pub fn auth_scheme_resolver(&self) -> ::std::option::Option<::aws_smithy_runtime_api::client::auth::SharedAuthSchemeOptionResolver> {
+    pub fn auth_scheme_resolver(
+        &self,
+    ) -> ::std::option::Option<::aws_smithy_runtime_api::client::auth::SharedAuthSchemeOptionResolver> {
         self.runtime_components.auth_scheme_option_resolver()
     }
     /// Returns the configured auth scheme preference
-    pub fn auth_scheme_preference(&self) -> ::std::option::Option<&::aws_smithy_runtime_api::client::auth::AuthSchemePreference> {
-        self.config.load::<::aws_smithy_runtime_api::client::auth::AuthSchemePreference>()
+    pub fn auth_scheme_preference(
+        &self,
+    ) -> ::std::option::Option<&::aws_smithy_runtime_api::client::auth::AuthSchemePreference> {
+        self.config
+            .load::<::aws_smithy_runtime_api::client::auth::AuthSchemePreference>()
     }
     /// Returns the endpoint resolver.
     pub fn endpoint_resolver(&self) -> ::aws_smithy_runtime_api::client::endpoint::SharedEndpointResolver {
-        self.runtime_components.endpoint_resolver().expect("resolver defaulted if not set")
+        self.runtime_components
+            .endpoint_resolver()
+            .expect("resolver defaulted if not set")
     }
     /// Return a reference to the retry configuration contained in this config, if any.
     pub fn retry_config(&self) -> ::std::option::Option<&::aws_smithy_types::retry::RetryConfig> {
@@ -91,7 +98,8 @@
     /// WARNING: This method is unstable and may be removed at any time. Do not rely on this
     /// method for anything!
     pub fn retry_partition(&self) -> ::std::option::Option<&::aws_smithy_runtime::client::retries::RetryPartition> {
-        self.config.load::<::aws_smithy_runtime::client::retries::RetryPartition>()
+        self.config
+            .load::<::aws_smithy_runtime::client::retries::RetryPartition>()
     }
     /// Returns the configured identity cache for auth.
     pub fn identity_cache(&self) -> ::std::option::Option<crate::config::SharedIdentityCache> {
@@ -106,7 +114,9 @@
         self.runtime_components.time_source()
     }
     /// Returns retry classifiers currently registered by the user.
-    pub fn retry_classifiers(&self) -> impl Iterator<Item = ::aws_smithy_runtime_api::client::retries::classifiers::SharedRetryClassifier> + '_ {
+    pub fn retry_classifiers(
+        &self,
+    ) -> impl Iterator<Item = ::aws_smithy_runtime_api::client::retries::classifiers::SharedRetryClassifier> + '_ {
         self.runtime_components.retry_classifiers()
     }
     /// Returns the name of the app that is using the client, if it was provided.
@@ -125,8 +135,10 @@
     pub fn framework_metadata(&self) -> ::std::vec::Vec<&::aws_types::sdk_ua_metadata::FrameworkMetadata> {
         // `StoreAppend` loads entries newest-first; reverse to first-seen order so
         // this getter agrees with both the user agent and `SdkConfig::framework_metadata`.
-        let mut entries: ::std::vec::Vec<&::aws_types::sdk_ua_metadata::FrameworkMetadata> =
-            self.config.load::<::aws_types::sdk_ua_metadata::FrameworkMetadata>().collect();
+        let mut entries: ::std::vec::Vec<&::aws_types::sdk_ua_metadata::FrameworkMetadata> = self
+            .config
+            .load::<::aws_types::sdk_ua_metadata::FrameworkMetadata>()
+            .collect();
         entries.reverse();
         entries
     }
@@ -133,8 +145,12 @@
     /// Returns the invocation ID generator if one was given in config.
     ///
     /// The invocation ID generator generates ID values for the `amz-sdk-invocation-id` header. By default, this will be a random UUID. Overriding it may be useful in tests that examine the HTTP request and need to be deterministic.
-    pub fn invocation_id_generator(&self) -> ::std::option::Option<::aws_runtime::invocation_id::SharedInvocationIdGenerator> {
-        self.config.load::<::aws_runtime::invocation_id::SharedInvocationIdGenerator>().cloned()
+    pub fn invocation_id_generator(
+        &self,
+    ) -> ::std::option::Option<::aws_runtime::invocation_id::SharedInvocationIdGenerator> {
+        self.config
+            .load::<::aws_runtime::invocation_id::SharedInvocationIdGenerator>()
+            .cloned()
     }
     /// Creates a new [service config](crate::Config) from a [shared `config`](::aws_types::sdk_config::SdkConfig).
     pub fn new(config: &::aws_types::sdk_config::SdkConfig) -> Self {
@@ -197,18 +213,42 @@
     #[allow(unused)]
     pub(crate) fn from_config_bag(config_bag: &::aws_smithy_types::config_bag::ConfigBag) -> Self {
         let mut builder = Self::new();
-        builder.set_stalled_stream_protection(config_bag.load::<crate::config::StalledStreamProtectionConfig>().cloned());
-        builder.set_auth_scheme_preference(config_bag.load::<::aws_smithy_runtime_api::client::auth::AuthSchemePreference>().cloned());
+        builder.set_stalled_stream_protection(
+            config_bag
+                .load::<crate::config::StalledStreamProtectionConfig>()
+                .cloned(),
+        );
+        builder.set_auth_scheme_preference(
+            config_bag
+                .load::<::aws_smithy_runtime_api::client::auth::AuthSchemePreference>()
+                .cloned(),
+        );
         builder.set_retry_config(config_bag.load::<::aws_smithy_types::retry::RetryConfig>().cloned());
         builder.set_timeout_config(config_bag.load::<::aws_smithy_types::timeout::TimeoutConfig>().cloned());
-        builder.set_retry_partition(config_bag.load::<::aws_smithy_runtime::client::retries::RetryPartition>().cloned());
+        builder.set_retry_partition(
+            config_bag
+                .load::<::aws_smithy_runtime::client::retries::RetryPartition>()
+                .cloned(),
+        );
         builder.set_app_name(config_bag.load::<::aws_types::app_name::AppName>().cloned());
         for framework_metadata in config_bag.load::<::aws_types::sdk_ua_metadata::FrameworkMetadata>() {
             builder.push_framework_metadata(framework_metadata.clone());
         }
-        builder.set_endpoint_url(config_bag.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()));
-        builder.set_use_dual_stack(config_bag.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0));
-        builder.set_use_fips(config_bag.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0));
+        builder.set_endpoint_url(
+            config_bag
+                .load::<::aws_types::endpoint_config::EndpointUrl>()
+                .map(|ty| ty.0.clone()),
+        );
+        builder.set_use_dual_stack(
+            config_bag
+                .load::<::aws_types::endpoint_config::UseDualStack>()
+                .map(|ty| ty.0),
+        );
+        builder.set_use_fips(
+            config_bag
+                .load::<::aws_types::endpoint_config::UseFips>()
+                .map(|ty| ty.0),
+        );
         builder.set_sigv4a_signing_region_set(config_bag.load::<::aws_types::region::SigningRegionSet>().cloned());
         builder.set_region(config_bag.load::<crate::config::Region>().cloned());
         builder
@@ -225,7 +265,10 @@
     /// high-cardinality values (like object keys) fragment the metrics and inflate
     /// cost. Use [`Self::capture_input_attributes`] for values you want to read
     /// in-process without emitting them on the metrics.
-    pub fn emit_input_attributes(mut self, names: impl ::std::iter::IntoIterator<Item = impl ::std::convert::Into<::std::string::String>>) -> Self {
+    pub fn emit_input_attributes(
+        mut self,
+        names: impl ::std::iter::IntoIterator<Item = impl ::std::convert::Into<::std::string::String>>,
+    ) -> Self {
         let mut requested = self
             .config
             .load::<::aws_smithy_types::telemetry::RequestedTelemetryAttributes>()
@@ -258,7 +301,10 @@
     }
     /// Set the [`StalledStreamProtectionConfig`](crate::config::StalledStreamProtectionConfig)
     /// to configure protection for stalled streams.
-    pub fn stalled_stream_protection(mut self, stalled_stream_protection_config: crate::config::StalledStreamProtectionConfig) -> Self {
+    pub fn stalled_stream_protection(
+        mut self,
+        stalled_stream_protection_config: crate::config::StalledStreamProtectionConfig,
+    ) -> Self {
         self.set_stalled_stream_protection(::std::option::Option::Some(stalled_stream_protection_config));
         self
     }
@@ -301,7 +347,9 @@
     /// # }
     /// ```
     pub fn http_client(mut self, http_client: impl crate::config::HttpClient + 'static) -> Self {
-        self.set_http_client(::std::option::Option::Some(crate::config::IntoShared::into_shared(http_client)));
+        self.set_http_client(::std::option::Option::Some(crate::config::IntoShared::into_shared(
+            http_client,
+        )));
         self
     }

@@ -446,7 +494,10 @@
     ///     // other configurations
     ///     .build();
     /// ```
-    pub fn push_auth_scheme(mut self, auth_scheme: impl ::aws_smithy_runtime_api::client::auth::AuthScheme + 'static) -> Self {
+    pub fn push_auth_scheme(
+        mut self,
+        auth_scheme: impl ::aws_smithy_runtime_api::client::auth::AuthScheme + 'static,
+    ) -> Self {
         self.runtime_components.push_auth_scheme(auth_scheme);
         self
     }
@@ -481,7 +532,10 @@
     ///     // other configurations
     ///     .build();
     /// ```
-    pub fn auth_scheme_resolver(mut self, auth_scheme_resolver: impl crate::config::auth::ResolveAuthScheme + 'static) -> Self {
+    pub fn auth_scheme_resolver(
+        mut self,
+        auth_scheme_resolver: impl crate::config::auth::ResolveAuthScheme + 'static,
+    ) -> Self {
         self.set_auth_scheme_resolver(auth_scheme_resolver);
         self
     }
@@ -490,7 +544,10 @@
     ///
     /// # Examples
     /// See an example for [`Self::auth_scheme_resolver`].
-    pub fn set_auth_scheme_resolver(&mut self, auth_scheme_resolver: impl crate::config::auth::ResolveAuthScheme + 'static) -> &mut Self {
+    pub fn set_auth_scheme_resolver(
+        &mut self,
+        auth_scheme_resolver: impl crate::config::auth::ResolveAuthScheme + 'static,
+    ) -> &mut Self {
         self.runtime_components
             .set_auth_scheme_option_resolver(::std::option::Option::Some(auth_scheme_resolver.into_shared_resolver()));
         self
@@ -510,7 +567,9 @@
     /// This adds [NoAuthScheme](aws_smithy_runtime::client::auth::no_auth::NoAuthScheme) as a fallback
     /// and the auth scheme resolver will use it when no other auth schemes are applicable.
     pub fn set_allow_no_auth(&mut self) -> &mut Self {
-        self.push_runtime_plugin(::aws_smithy_runtime::client::auth::no_auth::NoAuthRuntimePluginV2::new().into_shared());
+        self.push_runtime_plugin(
+            ::aws_smithy_runtime::client::auth::no_auth::NoAuthRuntimePluginV2::new().into_shared(),
+        );
         self
     }
     /// Set the auth scheme preference for an auth scheme resolver
@@ -597,7 +656,10 @@
     /// let config = aws_sdk_sts::Config::builder().endpoint_resolver(resolver).build();
     /// let client = aws_sdk_sts::Client::from_conf(config);
     /// ```
-    pub fn endpoint_resolver(mut self, endpoint_resolver: impl crate::config::endpoint::ResolveEndpoint + 'static) -> Self {
+    pub fn endpoint_resolver(
+        mut self,
+        endpoint_resolver: impl crate::config::endpoint::ResolveEndpoint + 'static,
+    ) -> Self {
         self.set_endpoint_resolver(::std::option::Option::Some(endpoint_resolver.into_shared_resolver()));
         self
     }
@@ -655,7 +717,10 @@
     /// disable_retries(&mut builder);
     /// let config = builder.build();
     /// ```
-    pub fn set_retry_config(&mut self, retry_config: ::std::option::Option<::aws_smithy_types::retry::RetryConfig>) -> &mut Self {
+    pub fn set_retry_config(
+        &mut self,
+        retry_config: ::std::option::Option<::aws_smithy_types::retry::RetryConfig>,
+    ) -> &mut Self {
         retry_config.map(|r| self.config.store_put(r));
         self
     }
@@ -679,7 +744,9 @@
     /// let config = Config::builder().sleep_impl(sleep_impl).build();
     /// ```
     pub fn sleep_impl(mut self, sleep_impl: impl crate::config::AsyncSleep + 'static) -> Self {
-        self.set_sleep_impl(Some(::aws_smithy_runtime_api::shared::IntoShared::into_shared(sleep_impl)));
+        self.set_sleep_impl(Some(::aws_smithy_runtime_api::shared::IntoShared::into_shared(
+            sleep_impl,
+        )));
         self
     }

@@ -755,9 +822,14 @@
     /// set_request_timeout(&mut builder);
     /// let config = builder.build();
     /// ```
-    pub fn set_timeout_config(&mut self, timeout_config: ::std::option::Option<::aws_smithy_types::timeout::TimeoutConfig>) -> &mut Self {
+    pub fn set_timeout_config(
+        &mut self,
+        timeout_config: ::std::option::Option<::aws_smithy_types::timeout::TimeoutConfig>,
+    ) -> &mut Self {
         // passing None has no impact.
-        let Some(mut timeout_config) = timeout_config else { return self };
+        let Some(mut timeout_config) = timeout_config else {
+            return self;
+        };

         if let Some(base) = self.config.load::<::aws_smithy_types::timeout::TimeoutConfig>() {
             timeout_config.take_defaults_from(base);
@@ -926,8 +998,12 @@
     /// let client = aws_sdk_sts::Client::from_conf(config);
     /// ```
     ///
-    pub fn set_identity_cache(&mut self, identity_cache: impl crate::config::ResolveCachedIdentity + 'static) -> &mut Self {
-        self.runtime_components.set_identity_cache(::std::option::Option::Some(identity_cache));
+    pub fn set_identity_cache(
+        &mut self,
+        identity_cache: impl crate::config::ResolveCachedIdentity + 'static,
+    ) -> &mut Self {
+        self.runtime_components
+            .set_identity_cache(::std::option::Option::Some(identity_cache));
         self
     }
     /// Add an [interceptor](crate::config::Intercept) that runs at specific stages of the request execution pipeline.
@@ -989,19 +1065,25 @@
     }

     /// Set [`SharedInterceptor`](crate::config::SharedInterceptor)s for the builder.
-    pub fn set_interceptors(&mut self, interceptors: impl IntoIterator<Item = crate::config::SharedInterceptor>) -> &mut Self {
+    pub fn set_interceptors(
+        &mut self,
+        interceptors: impl IntoIterator<Item = crate::config::SharedInterceptor>,
+    ) -> &mut Self {
         self.runtime_components.set_interceptors(interceptors.into_iter());
         self
     }
     /// Sets the time source used for this service
     pub fn time_source(mut self, time_source: impl ::aws_smithy_async::time::TimeSource + 'static) -> Self {
-        self.set_time_source(::std::option::Option::Some(::aws_smithy_runtime_api::shared::IntoShared::into_shared(
-            time_source,
-        )));
+        self.set_time_source(::std::option::Option::Some(
+            ::aws_smithy_runtime_api::shared::IntoShared::into_shared(time_source),
+        ));
         self
     }
     /// Sets the time source used for this service
-    pub fn set_time_source(&mut self, time_source: ::std::option::Option<::aws_smithy_async::time::SharedTimeSource>) -> &mut Self {
+    pub fn set_time_source(
+        &mut self,
+        time_source: ::std::option::Option<::aws_smithy_async::time::SharedTimeSource>,
+    ) -> &mut Self {
         self.runtime_components.set_time_source(time_source);
         self
     }
@@ -1096,9 +1178,9 @@
         mut self,
         retry_classifier: impl ::aws_smithy_runtime_api::client::retries::classifiers::ClassifyRetry + 'static,
     ) -> Self {
-        self.push_retry_classifier(::aws_smithy_runtime_api::client::retries::classifiers::SharedRetryClassifier::new(
-            retry_classifier,
-        ));
+        self.push_retry_classifier(
+            ::aws_smithy_runtime_api::client::retries::classifiers::SharedRetryClassifier::new(retry_classifier),
+        );
         self
     }

@@ -1115,9 +1197,12 @@
     /// were previously set.
     pub fn set_retry_classifiers(
         &mut self,
-        retry_classifiers: impl IntoIterator<Item = ::aws_smithy_runtime_api::client::retries::classifiers::SharedRetryClassifier>,
+        retry_classifiers: impl IntoIterator<
+            Item = ::aws_smithy_runtime_api::client::retries::classifiers::SharedRetryClassifier,
+        >,
     ) -> &mut Self {
-        self.runtime_components.set_retry_classifiers(retry_classifiers.into_iter());
+        self.runtime_components
+            .set_retry_classifiers(retry_classifiers.into_iter());
         self
     }
     /// Sets the name of the app that is using the client.
@@ -1158,7 +1243,10 @@
     /// (as `lib/{name}/{version}`) so that libraries built on top of the AWS SDK can
     /// self-identify in the requests they make. Multiple entries may be added; each call
     /// appends another entry rather than replacing previous ones.
-    pub fn push_framework_metadata(&mut self, framework_metadata: ::aws_types::sdk_ua_metadata::FrameworkMetadata) -> &mut Self {
+    pub fn push_framework_metadata(
+        &mut self,
+        framework_metadata: ::aws_types::sdk_ua_metadata::FrameworkMetadata,
+    ) -> &mut Self {
         self.config.store_append(framework_metadata);
         self
     }
@@ -1165,7 +1253,10 @@
     /// Overrides the default invocation ID generator.
     ///
     /// The invocation ID generator generates ID values for the `amz-sdk-invocation-id` header. By default, this will be a random UUID. Overriding it may be useful in tests that examine the HTTP request and need to be deterministic.
-    pub fn invocation_id_generator(mut self, gen: impl ::aws_runtime::invocation_id::InvocationIdGenerator + 'static) -> Self {
+    pub fn invocation_id_generator(
+        mut self,
+        gen: impl ::aws_runtime::invocation_id::InvocationIdGenerator + 'static,
+    ) -> Self {
         self.set_invocation_id_generator(::std::option::Option::Some(
             ::aws_runtime::invocation_id::SharedInvocationIdGenerator::new(gen),
         ));
@@ -1196,7 +1287,8 @@
     /// will be prefixed onto this URL. To fully override the endpoint resolver, use
     /// [`Builder::endpoint_resolver`].
     pub fn set_endpoint_url(&mut self, endpoint_url: Option<::std::string::String>) -> &mut Self {
-        self.config.store_or_unset(endpoint_url.map(::aws_types::endpoint_config::EndpointUrl));
+        self.config
+            .store_or_unset(endpoint_url.map(::aws_types::endpoint_config::EndpointUrl));
         self
     }
     /// When true, use the dual-stack endpoint. If the configured endpoint does not support dual-stack, dispatching the request MAY return an error.
@@ -1206,7 +1298,8 @@
     }
     /// When true, use the dual-stack endpoint. If the configured endpoint does not support dual-stack, dispatching the request MAY return an error.
     pub fn set_use_dual_stack(&mut self, use_dual_stack: Option<bool>) -> &mut Self {
-        self.config.store_or_unset(use_dual_stack.map(::aws_types::endpoint_config::UseDualStack));
+        self.config
+            .store_or_unset(use_dual_stack.map(::aws_types::endpoint_config::UseDualStack));
         self
     }
     /// When true, send this request to the FIPS-compliant regional endpoint. If the configured endpoint does not have a FIPS compliant endpoint, dispatching the request will return an error.
@@ -1216,7 +1309,8 @@
     }
     /// When true, send this request to the FIPS-compliant regional endpoint. If the configured endpoint does not have a FIPS compliant endpoint, dispatching the request will return an error.
     pub fn set_use_fips(&mut self, use_fips: Option<bool>) -> &mut Self {
-        self.config.store_or_unset(use_fips.map(::aws_types::endpoint_config::UseFips));
+        self.config
+            .store_or_unset(use_fips.map(::aws_types::endpoint_config::UseFips));
         self
     }
     /// Sets the SigV4a signing region set.
@@ -1251,14 +1345,20 @@
         self
     }
     /// Sets the credentials provider for this service
-    pub fn credentials_provider(mut self, credentials_provider: impl crate::config::ProvideCredentials + 'static) -> Self {
-        self.set_credentials_provider(::std::option::Option::Some(crate::config::SharedCredentialsProvider::new(
-            credentials_provider,
-        )));
+    pub fn credentials_provider(
+        mut self,
+        credentials_provider: impl crate::config::ProvideCredentials + 'static,
+    ) -> Self {
+        self.set_credentials_provider(::std::option::Option::Some(
+            crate::config::SharedCredentialsProvider::new(credentials_provider),
+        ));
         self
     }
     /// Sets the credentials provider for this service
-    pub fn set_credentials_provider(&mut self, credentials_provider: ::std::option::Option<crate::config::SharedCredentialsProvider>) -> &mut Self {
+    pub fn set_credentials_provider(
+        &mut self,
+        credentials_provider: ::std::option::Option<crate::config::SharedCredentialsProvider>,
+    ) -> &mut Self {
         if let Some(credentials_provider) = credentials_provider {
             #[cfg(feature = "sigv4a")]
             {
@@ -1363,10 +1463,13 @@
     #[allow(unused_mut)]
     /// Apply test defaults to the builder. NOTE: Consider migrating to use `apply_test_defaults_v2` instead.
     pub fn apply_test_defaults(&mut self) -> &mut Self {
-        self.set_time_source(::std::option::Option::Some(::aws_smithy_async::time::SharedTimeSource::new(
-            ::aws_smithy_async::time::StaticTimeSource::new(::std::time::UNIX_EPOCH + ::std::time::Duration::from_secs(1234567890)),
-        )));
-        self.config.store_put(::aws_runtime::user_agent::AwsUserAgent::for_tests());
+        self.set_time_source(::std::option::Option::Some(
+            ::aws_smithy_async::time::SharedTimeSource::new(::aws_smithy_async::time::StaticTimeSource::new(
+                ::std::time::UNIX_EPOCH + ::std::time::Duration::from_secs(1234567890),
+            )),
+        ));
+        self.config
+            .store_put(::aws_runtime::user_agent::AwsUserAgent::for_tests());
         self.set_credentials_provider(Some(crate::config::SharedCredentialsProvider::new(
             ::aws_credential_types::Credentials::for_tests(),
         )));
@@ -1435,7 +1538,8 @@
             cfg.store_put(::aws_smithy_runtime::client::orchestrator::AuthSchemeAndEndpointOrchestrationV2);
             ::std::option::Option::Some(cfg.freeze())
         };
-        let mut runtime_components = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ServiceRuntimePlugin");
+        let mut runtime_components =
+            ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ServiceRuntimePlugin");
         runtime_components.set_auth_scheme_option_resolver(::std::option::Option::Some({
             use crate::config::auth::ResolveAuthScheme;
             crate::config::auth::DefaultAuthSchemeResolver::default().into_shared_resolver()
@@ -1444,22 +1548,32 @@
             use crate::config::endpoint::ResolveEndpoint;
             crate::config::endpoint::DefaultResolver::new().into_shared_resolver()
         }));
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
+        runtime_components.push_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                ::aws_smithy_runtime::client::http::connection_poisoning::ConnectionPoisoningInterceptor::new(),
+            ),
+        );
+        runtime_components.push_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::HttpStatusCodeClassifier::default(),
+        );
+        runtime_components.push_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                crate::sdk_feature_tracker::retry_mode::RetryModeFeatureTrackerInterceptor::new(),
+            ),
+        );
+        runtime_components.push_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                ::aws_runtime::service_clock_skew::ServiceClockSkewInterceptor::new(),
+            ),
+        );
         runtime_components.push_interceptor(::aws_runtime::request_info::RequestInfoInterceptor::new());
         runtime_components.push_interceptor(::aws_runtime::user_agent::UserAgentInterceptor::new());
         runtime_components.push_interceptor(::aws_runtime::invocation_id::InvocationIdInterceptor::new());
-        runtime_components.push_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-            ::aws_runtime::recursion_detection::RecursionDetectionInterceptor::new(),
-        ));
+        runtime_components.push_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                ::aws_runtime::recursion_detection::RecursionDetectionInterceptor::new(),
+            ),
+        );
         runtime_components.push_auth_scheme(::aws_smithy_runtime_api::client::auth::SharedAuthScheme::new(
             ::aws_runtime::auth::sigv4::SigV4AuthScheme::new(),
         ));
@@ -1469,13 +1583,20 @@
                 ::aws_runtime::auth::sigv4a::SigV4aAuthScheme::new(),
             ));
         }
-        runtime_components.push_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-            crate::config::endpoint::EndpointOverrideFeatureTrackerInterceptor,
-        ));
-        runtime_components.push_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-            crate::observability_feature::ObservabilityFeatureTrackerInterceptor,
-        ));
-        Self { config, runtime_components }
+        runtime_components.push_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                crate::config::endpoint::EndpointOverrideFeatureTrackerInterceptor,
+            ),
+        );
+        runtime_components.push_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                crate::observability_feature::ObservabilityFeatureTrackerInterceptor,
+            ),
+        );
+        Self {
+            config,
+            runtime_components,
+        }
     }
 }

@@ -1518,14 +1639,22 @@
         let mut layer = config_override.config;
         let mut components = config_override.runtime_components;
         #[allow(unused_mut)]
-        let mut resolver =
-            ::aws_smithy_runtime::client::config_override::Resolver::overrid(initial_config, initial_components, &mut layer, &mut components);
+        let mut resolver = ::aws_smithy_runtime::client::config_override::Resolver::overrid(
+            initial_config,
+            initial_components,
+            &mut layer,
+            &mut components,
+        );

         resolver
             .config_mut()
             .load::<::aws_types::region::Region>()
             .cloned()
-            .map(|r| resolver.config_mut().store_put(::aws_types::region::SigningRegion::from(r)));
+            .map(|r| {
+                resolver
+                    .config_mut()
+                    .store_put(::aws_types::region::SigningRegion::from(r))
+            });

         let _ = resolver;

@@ -1538,7 +1667,9 @@
         // This is skipped if the override already sets its own identity cache.
         if components.has_identity_resolvers() && components.identity_cache().is_none() {
             components.set_identity_cache(::std::option::Option::Some(
-                ::aws_smithy_runtime::client::identity::IdentityCache::lazy().max_partitions(1).build(),
+                ::aws_smithy_runtime::client::identity::IdentityCache::lazy()
+                    .max_partitions(1)
+                    .build(),
             ));
         }

@@ -1627,7 +1758,11 @@
 pub use ::aws_types::sdk_ua_metadata::FrameworkMetadata;

 #[allow(dead_code)]
-fn service_config_key<'a>(service_id: &'a str, env: &'a str, profile: &'a str) -> aws_types::service_config::ServiceConfigKey<'a> {
+fn service_config_key<'a>(
+    service_id: &'a str,
+    env: &'a str,
+    profile: &'a str,
+) -> aws_types::service_config::ServiceConfigKey<'a> {
     ::aws_types::service_config::ServiceConfigKey::builder()
         .service_id(service_id)
         .env(env)
@@ -1638,13 +1773,16 @@

 pub use ::aws_smithy_async::rt::sleep::Sleep;

-pub(crate) fn base_client_runtime_plugins(mut config: crate::Config) -> ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins {
+pub(crate) fn base_client_runtime_plugins(
+    mut config: crate::Config,
+) -> ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins {
     let mut configured_plugins = ::std::vec::Vec::new();
     ::std::mem::swap(&mut config.runtime_plugins, &mut configured_plugins);
     #[cfg(feature = "behavior-version-latest")]
     {
         if config.behavior_version.is_none() {
-            config.behavior_version = Some(::aws_smithy_runtime_api::client::behavior_version::BehaviorVersion::latest());
+            config.behavior_version =
+                Some(::aws_smithy_runtime_api::client::behavior_version::BehaviorVersion::latest());
         }
     }

```

### `src/config/auth.rs`

```diff
diff --git a/src/config/auth.rs b/src/config/auth.rs
--- a/src/config/auth.rs
+++ b/src/config/auth.rs
@@ -31,10 +31,12 @@
         runtime_components: &'a ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponents,
     ) -> ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionsFuture<'a> {
         match params.get::<crate::config::auth::Params>() {
-            ::std::option::Option::Some(concrete_params) => self.0.resolve_auth_scheme(concrete_params, cfg, runtime_components),
-            ::std::option::Option::None => ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionsFuture::ready(::std::result::Result::Err(
-                "params of expected type was not present".into(),
-            )),
+            ::std::option::Option::Some(concrete_params) => {
+                self.0.resolve_auth_scheme(concrete_params, cfg, runtime_components)
+            }
+            ::std::option::Option::None => ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionsFuture::ready(
+                ::std::result::Result::Err("params of expected type was not present".into()),
+            ),
         }
     }
 }
@@ -44,7 +46,8 @@
 #[allow(dead_code)]
 pub struct DefaultAuthSchemeResolver {
     service_defaults: Vec<::aws_smithy_runtime_api::client::auth::AuthSchemeOption>,
-    operation_overrides: ::std::collections::HashMap<&'static str, Vec<::aws_smithy_runtime_api::client::auth::AuthSchemeOption>>,
+    operation_overrides:
+        ::std::collections::HashMap<&'static str, Vec<::aws_smithy_runtime_api::client::auth::AuthSchemeOption>>,
 }

 // TODO(https://github.com/smithy-lang/smithy-rs/issues/4177): Remove `allow(...)` once the issue is addressed.
@@ -101,7 +104,8 @@
             None => &self.service_defaults,
         };

-        let _fut = ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionsFuture::ready(Ok(modeled_auth_options.clone()));
+        let _fut =
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionsFuture::ready(Ok(modeled_auth_options.clone()));

         _fut
     }
@@ -136,7 +140,10 @@
     }

     /// Set the operation name for the builder
-    pub fn set_operation_name(mut self, operation_name: ::std::option::Option<::std::borrow::Cow<'static, str>>) -> Self {
+    pub fn set_operation_name(
+        mut self,
+        operation_name: ::std::option::Option<::std::borrow::Cow<'static, str>>,
+    ) -> Self {
         self.operation_name = operation_name;
         self
     }
@@ -146,7 +153,9 @@
     ///
     pub fn build(self) -> ::std::result::Result<crate::config::auth::Params, crate::config::auth::BuildError> {
         ::std::result::Result::Ok(crate::config::auth::Params {
-            operation_name: self.operation_name.ok_or_else(|| BuildError::missing("operation_name"))?,
+            operation_name: self
+                .operation_name
+                .ok_or_else(|| BuildError::missing("operation_name"))?,
         })
     }
 }
```

### `src/config/endpoint.rs`

```diff
diff --git a/src/config/endpoint.rs b/src/config/endpoint.rs
--- a/src/config/endpoint.rs
+++ b/src/config/endpoint.rs
@@ -15,7 +15,9 @@

     fn read_before_execution(
         &self,
-        _context: &::aws_smithy_runtime_api::client::interceptors::context::BeforeSerializationInterceptorContextRef<'_>,
+        _context: &::aws_smithy_runtime_api::client::interceptors::context::BeforeSerializationInterceptorContextRef<
+            '_,
+        >,
         cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
     ) -> ::std::result::Result<(), ::aws_smithy_runtime_api::box_error::BoxError> {
         if cfg.load::<::aws_types::endpoint_config::EndpointUrl>().is_some() {
@@ -665,7 +667,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://sts-fips.cn-north-1.api.amazonwebservices.com.cn");
+        let endpoint =
+            endpoint.expect("Expected valid endpoint: https://sts-fips.cn-north-1.api.amazonwebservices.com.cn");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -949,7 +952,9 @@
         let endpoint = endpoint.expect("Expected valid endpoint: https://example.com");
         assert_eq!(
             endpoint,
-            ::aws_smithy_types::endpoint::Endpoint::builder().url("https://example.com").build()
+            ::aws_smithy_types::endpoint::Endpoint::builder()
+                .url("https://example.com")
+                .build()
         );
     }

@@ -967,7 +972,9 @@
         let endpoint = endpoint.expect("Expected valid endpoint: https://example.com");
         assert_eq!(
             endpoint,
-            ::aws_smithy_types::endpoint::Endpoint::builder().url("https://example.com").build()
+            ::aws_smithy_types::endpoint::Endpoint::builder()
+                .url("https://example.com")
+                .build()
         );
     }

@@ -984,7 +991,10 @@
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
         let error = endpoint.expect_err("expected error: Invalid Configuration: FIPS and custom endpoint are not supported [For custom endpoint with fips enabled and dualstack disabled]");
-        assert_eq!(format!("{}", error), "Invalid Configuration: FIPS and custom endpoint are not supported")
+        assert_eq!(
+            format!("{}", error),
+            "Invalid Configuration: FIPS and custom endpoint are not supported"
+        )
     }

     /// For custom endpoint with fips disabled and dualstack enabled
@@ -1009,7 +1019,9 @@
     /// Missing region
     #[test]
     fn test_50() {
-        let params = crate::config::endpoint::Params::builder().build().expect("invalid params");
+        let params = crate::config::endpoint::Params::builder()
+            .build()
+            .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
         let error = endpoint.expect_err("expected error: Invalid Configuration: Missing Region [Missing region]");
@@ -1474,7 +1486,9 @@
         let endpoint = endpoint.expect("Expected valid endpoint: https://example.com");
         assert_eq!(
             endpoint,
-            ::aws_smithy_types::endpoint::Endpoint::builder().url("https://example.com").build()
+            ::aws_smithy_types::endpoint::Endpoint::builder()
+                .url("https://example.com")
+                .build()
         );
     }

@@ -1493,7 +1507,9 @@
         let endpoint = endpoint.expect("Expected valid endpoint: https://example.com");
         assert_eq!(
             endpoint,
-            ::aws_smithy_types::endpoint::Endpoint::builder().url("https://example.com").build()
+            ::aws_smithy_types::endpoint::Endpoint::builder()
+                .url("https://example.com")
+                .build()
         );
     }
 }
@@ -1501,7 +1517,10 @@
 /// Endpoint resolver trait specific to AWS Security Token Service
 pub trait ResolveEndpoint: ::std::marker::Send + ::std::marker::Sync + ::std::fmt::Debug {
     /// Resolve an endpoint with the given parameters
-    fn resolve_endpoint<'a>(&'a self, params: &'a crate::config::endpoint::Params) -> ::aws_smithy_runtime_api::client::endpoint::EndpointFuture<'a>;
+    fn resolve_endpoint<'a>(
+        &'a self,
+        params: &'a crate::config::endpoint::Params,
+    ) -> ::aws_smithy_runtime_api::client::endpoint::EndpointFuture<'a>;

     /// Convert this service-specific resolver into a `SharedEndpointResolver`
     ///
@@ -1526,7 +1545,9 @@
     ) -> ::aws_smithy_runtime_api::client::endpoint::EndpointFuture<'a> {
         let ep = match params.get::<crate::config::endpoint::Params>() {
             Some(params) => self.0.resolve_endpoint(params),
-            None => ::aws_smithy_runtime_api::client::endpoint::EndpointFuture::ready(Err("params of expected type was not present".into())),
+            None => ::aws_smithy_runtime_api::client::endpoint::EndpointFuture::ready(Err(
+                "params of expected type was not present".into(),
+            )),
         };
         ep
     }
@@ -1570,7 +1591,8 @@
     fn resolve_endpoint<'a>(
         &'a self,
         params: &'a crate::config::endpoint::Params,
-    ) -> ::std::result::Result<::aws_smithy_types::endpoint::Endpoint, ::aws_smithy_runtime_api::box_error::BoxError> {
+    ) -> ::std::result::Result<::aws_smithy_types::endpoint::Endpoint, ::aws_smithy_runtime_api::box_error::BoxError>
+    {
         let mut _diagnostic_collector = crate::endpoint_lib::diagnostic::DiagnosticCollector::new();
         #[allow(unused_mut)]
         let mut context = ConditionContext::default();
@@ -1587,22 +1609,29 @@
             match current_ref {
                 ref_val if ref_val >= 100_000_000 => {
                     return match (ref_val - 100_000_000) as usize {
-                        0 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
-                            "No endpoint rule matched",
-                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        0 => ::std::result::Result::Err(Box::new(
+                            ::aws_smithy_http::endpoint::ResolveEndpointError::message("No endpoint rule matched"),
+                        )
+                            as ::aws_smithy_runtime_api::box_error::BoxError),
                         1 => ::std::result::Result::Ok(
                             ::aws_smithy_types::endpoint::Endpoint::builder()
                                 .url("https://sts.amazonaws.com".to_string())
                                 .auth_scheme(
-                                    ::aws_smithy_types::endpoint::EndpointAuthScheme::with_capacity("sigv4".to_string(), 2)
-                                        .put("signingName", "sts")
-                                        .put("signingRegion", "us-east-1"),
+                                    ::aws_smithy_types::endpoint::EndpointAuthScheme::with_capacity(
+                                        "sigv4".to_string(),
+                                        2,
+                                    )
+                                    .put("signingName", "sts")
+                                    .put("signingRegion", "us-east-1"),
                                 )
                                 .build(),
                         ),
                         2 => {
                             let region = params.region.as_deref().unwrap_or_default();
-                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            let partition_result = context
+                                .partition_result
+                                .as_ref()
+                                .expect("Guaranteed to have a value by earlier checks.");
                             ::std::result::Result::Ok(
                                 ::aws_smithy_types::endpoint::Endpoint::builder()
                                     .url({
@@ -1616,26 +1645,42 @@
                                         out
                                     })
                                     .auth_scheme(
-                                        ::aws_smithy_types::endpoint::EndpointAuthScheme::with_capacity("sigv4".to_string(), 2)
-                                            .put("signingName", "sts")
-                                            .put("signingRegion", region.as_ref()),
+                                        ::aws_smithy_types::endpoint::EndpointAuthScheme::with_capacity(
+                                            "sigv4".to_string(),
+                                            2,
+                                        )
+                                        .put("signingName", "sts")
+                                        .put("signingRegion", region.as_ref()),
                                     )
                                     .build(),
                             )
                         }
-                        3 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
-                            "Invalid Configuration: FIPS and custom endpoint are not supported".to_string(),
-                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
-                        4 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
-                            "Invalid Configuration: Dualstack and custom endpoint are not supported".to_string(),
-                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        3 => ::std::result::Result::Err(Box::new(
+                            ::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                                "Invalid Configuration: FIPS and custom endpoint are not supported".to_string(),
+                            ),
+                        )
+                            as ::aws_smithy_runtime_api::box_error::BoxError),
+                        4 => ::std::result::Result::Err(Box::new(
+                            ::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                                "Invalid Configuration: Dualstack and custom endpoint are not supported".to_string(),
+                            ),
+                        )
+                            as ::aws_smithy_runtime_api::box_error::BoxError),
                         5 => {
                             let endpoint = params.endpoint.as_deref().unwrap_or_default();
-                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url(endpoint.to_owned()).build())
+                            ::std::result::Result::Ok(
+                                ::aws_smithy_types::endpoint::Endpoint::builder()
+                                    .url(endpoint.to_owned())
+                                    .build(),
+                            )
                         }
                         6 => {
                             let region = params.region.as_deref().unwrap_or_default();
-                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            let partition_result = context
+                                .partition_result
+                                .as_ref()
+                                .expect("Guaranteed to have a value by earlier checks.");
                             ::std::result::Result::Ok(
                                 ::aws_smithy_types::endpoint::Endpoint::builder()
                                     .url({
@@ -1651,9 +1696,13 @@
                                     .build(),
                             )
                         }
-                        7 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
-                            "FIPS and DualStack are enabled, but this partition does not support one or both".to_string(),
-                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        7 => ::std::result::Result::Err(Box::new(
+                            ::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                                "FIPS and DualStack are enabled, but this partition does not support one or both"
+                                    .to_string(),
+                            ),
+                        )
+                            as ::aws_smithy_runtime_api::box_error::BoxError),
                         8 => {
                             let region = params.region.as_deref().unwrap_or_default();
                             ::std::result::Result::Ok(
@@ -1671,7 +1720,10 @@
                         }
                         9 => {
                             let region = params.region.as_deref().unwrap_or_default();
-                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            let partition_result = context
+                                .partition_result
+                                .as_ref()
+                                .expect("Guaranteed to have a value by earlier checks.");
                             ::std::result::Result::Ok(
                                 ::aws_smithy_types::endpoint::Endpoint::builder()
                                     .url({
@@ -1687,12 +1739,18 @@
                                     .build(),
                             )
                         }
-                        10 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
-                            "FIPS is enabled but this partition does not support FIPS".to_string(),
-                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        10 => ::std::result::Result::Err(Box::new(
+                            ::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                                "FIPS is enabled but this partition does not support FIPS".to_string(),
+                            ),
+                        )
+                            as ::aws_smithy_runtime_api::box_error::BoxError),
                         11 => {
                             let region = params.region.as_deref().unwrap_or_default();
-                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            let partition_result = context
+                                .partition_result
+                                .as_ref()
+                                .expect("Guaranteed to have a value by earlier checks.");
                             ::std::result::Result::Ok(
                                 ::aws_smithy_types::endpoint::Endpoint::builder()
                                     .url({
@@ -1708,12 +1766,18 @@
                                     .build(),
                             )
                         }
-                        12 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
-                            "DualStack is enabled but this partition does not support DualStack".to_string(),
-                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        12 => ::std::result::Result::Err(Box::new(
+                            ::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                                "DualStack is enabled but this partition does not support DualStack".to_string(),
+                            ),
+                        )
+                            as ::aws_smithy_runtime_api::box_error::BoxError),
                         13 => {
                             let region = params.region.as_deref().unwrap_or_default();
-                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            let partition_result = context
+                                .partition_result
+                                .as_ref()
+                                .expect("Guaranteed to have a value by earlier checks.");
                             ::std::result::Result::Ok(
                                 ::aws_smithy_types::endpoint::Endpoint::builder()
                                     .url({
@@ -1729,19 +1793,23 @@
                                     .build(),
                             )
                         }
-                        14 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
-                            "Invalid Configuration: Missing Region".to_string(),
-                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
-                        _ => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
-                            "No endpoint rule matched",
-                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        14 => ::std::result::Result::Err(Box::new(
+                            ::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                                "Invalid Configuration: Missing Region".to_string(),
+                            ),
+                        )
+                            as ::aws_smithy_runtime_api::box_error::BoxError),
+                        _ => ::std::result::Result::Err(Box::new(
+                            ::aws_smithy_http::endpoint::ResolveEndpointError::message("No endpoint rule matched"),
+                        )
+                            as ::aws_smithy_runtime_api::box_error::BoxError),
                     };
                 }
                 1 | -1 => {
-                    return ::std::result::Result::Err(
-                        Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message("No endpoint rule matched"))
-                            as ::aws_smithy_runtime_api::box_error::BoxError,
+                    return ::std::result::Result::Err(Box::new(
+                        ::aws_smithy_http::endpoint::ResolveEndpointError::message("No endpoint rule matched"),
                     )
+                        as ::aws_smithy_runtime_api::box_error::BoxError)
                 }
                 ref_val => {
                     let is_complement = ref_val < 0;
@@ -1855,7 +1923,11 @@
                         })(&mut _diagnostic_collector),
                         _ => unreachable!("Invalid condition index"),
                     };
-                    current_ref = if is_complement ^ condition_result { node.high_ref } else { node.low_ref };
+                    current_ref = if is_complement ^ condition_result {
+                        node.high_ref
+                    } else {
+                        node.low_ref
+                    };
                 }
             }
         }
@@ -1863,18 +1935,24 @@
 }

 impl crate::config::endpoint::ResolveEndpoint for DefaultResolver {
-    fn resolve_endpoint<'a>(&'a self, params: &'a crate::config::endpoint::Params) -> ::aws_smithy_runtime_api::client::endpoint::EndpointFuture<'a> {
+    fn resolve_endpoint<'a>(
+        &'a self,
+        params: &'a crate::config::endpoint::Params,
+    ) -> ::aws_smithy_runtime_api::client::endpoint::EndpointFuture<'a> {
         // Check single-entry cache (lock-free read via ArcSwap)
         let cached = self.endpoint_cache.load();
         if let Some((cached_params, cached_endpoint)) = cached.as_ref() {
             if cached_params == params {
-                return ::aws_smithy_runtime_api::client::endpoint::EndpointFuture::ready(::std::result::Result::Ok(cached_endpoint.clone()));
+                return ::aws_smithy_runtime_api::client::endpoint::EndpointFuture::ready(::std::result::Result::Ok(
+                    cached_endpoint.clone(),
+                ));
             }
         }
         drop(cached);
         let result = self.resolve_endpoint(params);
         if let ::std::result::Result::Ok(ref endpoint) = result {
-            self.endpoint_cache.store(::std::sync::Arc::new(Some((params.clone(), endpoint.clone()))));
+            self.endpoint_cache
+                .store(::std::sync::Arc::new(Some((params.clone(), endpoint.clone()))));
         }
         ::aws_smithy_runtime_api::client::endpoint::EndpointFuture::ready(result)
     }
@@ -2099,7 +2177,9 @@
 }
 impl ParamsBuilder {
     /// Consume this builder, creating [`Params`].
-    pub fn build(self) -> ::std::result::Result<crate::config::endpoint::Params, crate::config::endpoint::InvalidParams> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<crate::config::endpoint::Params, crate::config::endpoint::InvalidParams> {
         if let Some(region) = &self.region {
             if !crate::endpoint_lib::host::is_valid_host_label(
                 region.as_ref() as &str,
@@ -2251,7 +2331,9 @@
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
         match self.kind {
             InvalidParamsErrorKind::MissingField => write!(f, "a required field was missing: `{}`", self.field),
-            InvalidParamsErrorKind::InvalidValue { message } => write!(f, "invalid value for field: `{}` - {}", self.field, message),
+            InvalidParamsErrorKind::InvalidValue { message } => {
+                write!(f, "invalid value for field: `{}` - {}", self.field, message)
+            }
         }
     }
 }
```

### `src/endpoint_lib.rs`

```diff
diff --git a/src/endpoint_lib.rs b/src/endpoint_lib.rs
--- a/src/endpoint_lib.rs
+++ b/src/endpoint_lib.rs
@@ -1,19 +1,20 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 // Loading the partition JSON is expensive since it involves many regex compilations,
 // so cache the result so that it only need to be paid for the first constructed client.
-pub(crate) static DEFAULT_PARTITION_RESOLVER: std::sync::LazyLock<crate::endpoint_lib::partition::PartitionResolver> = std::sync::LazyLock::new(
-    || match std::env::var("SMITHY_CLIENT_SDK_CUSTOM_PARTITION") {
+pub(crate) static DEFAULT_PARTITION_RESOLVER: std::sync::LazyLock<crate::endpoint_lib::partition::PartitionResolver> =
+    std::sync::LazyLock::new(|| match std::env::var("SMITHY_CLIENT_SDK_CUSTOM_PARTITION") {
         Ok(partitions) => {
             ::tracing::debug!("loading custom partitions located at {partitions}");
-            let partition_dot_json = std::fs::read_to_string(partitions).expect("should be able to read a custom partition JSON");
-            crate::endpoint_lib::partition::PartitionResolver::new_from_json(partition_dot_json.as_bytes()).expect("valid JSON")
+            let partition_dot_json =
+                std::fs::read_to_string(partitions).expect("should be able to read a custom partition JSON");
+            crate::endpoint_lib::partition::PartitionResolver::new_from_json(partition_dot_json.as_bytes())
+                .expect("valid JSON")
         }
         _ => {
             ::tracing::debug!("loading default partitions");
             crate::endpoint_lib::partition::PartitionResolver::new_from_json(b"{\"partitions\":[{\"id\":\"aws\",\"outputs\":{\"dnsSuffix\":\"amazonaws.com\",\"dualStackDnsSuffix\":\"api.aws\",\"implicitGlobalRegion\":\"us-east-1\",\"name\":\"aws\",\"supportsDualStack\":true,\"supportsFIPS\":true},\"regionRegex\":\"^(us|eu|ap|sa|ca|me|af|il|mx)\\\\-\\\\w+\\\\-\\\\d+$\",\"regions\":{\"af-south-1\":{\"description\":\"Africa (Cape Town)\"},\"ap-east-1\":{\"description\":\"Asia Pacific (Hong Kong)\"},\"ap-east-2\":{\"description\":\"Asia Pacific (Taipei)\"},\"ap-northeast-1\":{\"description\":\"Asia Pacific (Tokyo)\"},\"ap-northeast-2\":{\"description\":\"Asia Pacific (Seoul)\"},\"ap-northeast-3\":{\"description\":\"Asia Pacific (Osaka)\"},\"ap-south-1\":{\"description\":\"Asia Pacific (Mumbai)\"},\"ap-south-2\":{\"description\":\"Asia Pacific (Hyderabad)\"},\"ap-southeast-1\":{\"description\":\"Asia Pacific (Singapore)\"},\"ap-southeast-2\":{\"description\":\"Asia Pacific (Sydney)\"},\"ap-southeast-3\":{\"description\":\"Asia Pacific (Jakarta)\"},\"ap-southeast-4\":{\"description\":\"Asia Pacific (Melbourne)\"},\"ap-southeast-5\":{\"description\":\"Asia Pacific (Malaysia)\"},\"ap-southeast-6\":{\"description\":\"Asia Pacific (New Zealand)\"},\"ap-southeast-7\":{\"description\":\"Asia Pacific (Thailand)\"},\"aws-global\":{\"description\":\"aws global region\"},\"ca-central-1\":{\"description\":\"Canada (Central)\"},\"ca-west-1\":{\"description\":\"Canada West (Calgary)\"},\"eu-central-1\":{\"description\":\"Europe (Frankfurt)\"},\"eu-central-2\":{\"description\":\"Europe (Zurich)\"},\"eu-north-1\":{\"description\":\"Europe (Stockholm)\"},\"eu-south-1\":{\"description\":\"Europe (Milan)\"},\"eu-south-2\":{\"description\":\"Europe (Spain)\"},\"eu-west-1\":{\"description\":\"Europe (Ireland)\"},\"eu-west-2\":{\"description\":\"Europe (London)\"},\"eu-west-3\":{\"description\":\"Europe (Paris)\"},\"il-central-1\":{\"description\":\"Israel (Tel Aviv)\"},\"me-central-1\":{\"description\":\"Middle East (UAE)\"},\"me-south-1\":{\"description\":\"Middle East (Bahrain)\"},\"mx-central-1\":{\"description\":\"Mexico (Central)\"},\"sa-east-1\":{\"description\":\"South America (Sao Paulo)\"},\"us-east-1\":{\"description\":\"US East (N. Virginia)\"},\"us-east-2\":{\"description\":\"US East (Ohio)\"},\"us-west-1\":{\"description\":\"US West (N. California)\"},\"us-west-2\":{\"description\":\"US West (Oregon)\"}}},{\"id\":\"aws-cn\",\"outputs\":{\"dnsSuffix\":\"amazonaws.com.cn\",\"dualStackDnsSuffix\":\"api.amazonwebservices.com.cn\",\"implicitGlobalRegion\":\"cn-northwest-1\",\"name\":\"aws-cn\",\"supportsDualStack\":true,\"supportsFIPS\":true},\"regionRegex\":\"^cn\\\\-\\\\w+\\\\-\\\\d+$\",\"regions\":{\"aws-cn-global\":{\"description\":\"aws-cn global region\"},\"cn-north-1\":{\"description\":\"China (Beijing)\"},\"cn-northwest-1\":{\"description\":\"China (Ningxia)\"}}},{\"id\":\"aws-eusc\",\"outputs\":{\"dnsSuffix\":\"amazonaws.eu\",\"dualStackDnsSuffix\":\"api.amazonwebservices.eu\",\"implicitGlobalRegion\":\"eusc-de-east-1\",\"name\":\"aws-eusc\",\"supportsDualStack\":true,\"supportsFIPS\":true},\"regionRegex\":\"^eusc\\\\-(de)\\\\-\\\\w+\\\\-\\\\d+$\",\"regions\":{\"eusc-de-east-1\":{\"description\":\"AWS European Sovereign Cloud (Germany)\"}}},{\"id\":\"aws-iso\",\"outputs\":{\"dnsSuffix\":\"c2s.ic.gov\",\"dualStackDnsSuffix\":\"api.aws.ic.gov\",\"implicitGlobalRegion\":\"us-iso-east-1\",\"name\":\"aws-iso\",\"supportsDualStack\":true,\"supportsFIPS\":true},\"regionRegex\":\"^us\\\\-iso\\\\-\\\\w+\\\\-\\\\d+$\",\"regions\":{\"aws-iso-global\":{\"description\":\"aws-iso global region\"},\"us-iso-east-1\":{\"description\":\"US ISO East\"},\"us-iso-west-1\":{\"description\":\"US ISO WEST\"}}},{\"id\":\"aws-iso-b\",\"outputs\":{\"dnsSuffix\":\"sc2s.sgov.gov\",\"dualStackDnsSuffix\":\"api.aws.scloud\",\"implicitGlobalRegion\":\"us-isob-east-1\",\"name\":\"aws-iso-b\",\"supportsDualStack\":true,\"supportsFIPS\":true},\"regionRegex\":\"^us\\\\-isob\\\\-\\\\w+\\\\-\\\\d+$\",\"regions\":{\"aws-iso-b-global\":{\"description\":\"aws-iso-b global region\"},\"us-isob-east-1\":{\"description\":\"US ISOB East (Ohio)\"},\"us-isob-west-1\":{\"description\":\"US ISOB West\"}}},{\"id\":\"aws-iso-e\",\"outputs\":{\"dnsSuffix\":\"cloud.adc-e.uk\",\"dualStackDnsSuffix\":\"api.cloud-aws.adc-e.uk\",\"implicitGlobalRegion\":\"eu-isoe-west-1\",\"name\":\"aws-iso-e\",\"supportsDualStack\":true,\"supportsFIPS\":true},\"regionRegex\":\"^eu\\\\-isoe\\\\-\\\\w+\\\\-\\\\d+$\",\"regions\":{\"aws-iso-e-global\":{\"description\":\"aws-iso-e global region\"},\"eu-isoe-west-1\":{\"description\":\"EU ISOE West\"}}},{\"id\":\"aws-iso-f\",\"outputs\":{\"dnsSuffix\":\"csp.hci.ic.gov\",\"dualStackDnsSuffix\":\"api.aws.hci.ic.gov\",\"implicitGlobalRegion\":\"us-isof-south-1\",\"name\":\"aws-iso-f\",\"supportsDualStack\":true,\"supportsFIPS\":true},\"regionRegex\":\"^us\\\\-isof\\\\-\\\\w+\\\\-\\\\d+$\",\"regions\":{\"aws-iso-f-global\":{\"description\":\"aws-iso-f global region\"},\"us-isof-east-1\":{\"description\":\"US ISOF EAST\"},\"us-isof-south-1\":{\"description\":\"US ISOF SOUTH\"}}},{\"id\":\"aws-us-gov\",\"outputs\":{\"dnsSuffix\":\"amazonaws.com\",\"dualStackDnsSuffix\":\"api.aws\",\"implicitGlobalRegion\":\"us-gov-west-1\",\"name\":\"aws-us-gov\",\"supportsDualStack\":true,\"supportsFIPS\":true},\"regionRegex\":\"^us\\\\-gov\\\\-\\\\w+\\\\-\\\\d+$\",\"regions\":{\"aws-us-gov-global\":{\"description\":\"aws-us-gov global region\"},\"us-gov-east-1\":{\"description\":\"AWS GovCloud (US-East)\"},\"us-gov-west-1\":{\"description\":\"AWS GovCloud (US-West)\"}}}],\"version\":\"1.1\"}").expect("valid JSON")
         }
-    },
-);
+    });

 pub(crate) mod bdd_interpreter;

```

### `src/endpoint_lib/bdd_interpreter.rs`

```diff
diff --git a/src/endpoint_lib/bdd_interpreter.rs b/src/endpoint_lib/bdd_interpreter.rs
--- a/src/endpoint_lib/bdd_interpreter.rs
+++ b/src/endpoint_lib/bdd_interpreter.rs
@@ -43,7 +43,12 @@
     params: &'a Params,
     context: &mut Context,
     diagnostic_collector: &mut crate::endpoint_lib::diagnostic::DiagnosticCollector,
-    mut condition_evaluator: impl FnMut(&Cond, &'a Params, &mut Context, &mut crate::endpoint_lib::diagnostic::DiagnosticCollector) -> bool,
+    mut condition_evaluator: impl FnMut(
+        &Cond,
+        &'a Params,
+        &mut Context,
+        &mut crate::endpoint_lib::diagnostic::DiagnosticCollector,
+    ) -> bool,
     result_builder: impl FnOnce(usize, &'a Params, &Context) -> R,
 ) -> Option<R> {
     let mut current_ref = root_ref;
@@ -68,7 +73,11 @@
                 let condition = conditions.get(condition_index)?;
                 let condition_result = condition_evaluator(condition, params, context, diagnostic_collector);
                 // Handle complement edges: complement inverts the branch selection
-                current_ref = if is_complement ^ condition_result { node.high_ref } else { node.low_ref };
+                current_ref = if is_complement ^ condition_result {
+                    node.high_ref
+                } else {
+                    node.low_ref
+                };
             }
         }
     }
```

### `src/endpoint_lib/host.rs`

```diff
diff --git a/src/endpoint_lib/host.rs b/src/endpoint_lib/host.rs
--- a/src/endpoint_lib/host.rs
+++ b/src/endpoint_lib/host.rs
@@ -63,7 +63,10 @@
         assert_eq!(is_valid_host_label("a.b.c", true), true);
         assert_eq!(is_valid_host_label("a_b", true), false);
         assert_eq!(is_valid_host_label(&"a".repeat(64), false), false);
-        assert_eq!(is_valid_host_label(&format!("{}.{}", "a".repeat(63), "a".repeat(63)), true), true);
+        assert_eq!(
+            is_valid_host_label(&format!("{}.{}", "a".repeat(63), "a".repeat(63)), true),
+            true
+        );
     }

     #[allow(clippy::bool_assert_comparison)]
```

### `src/endpoint_lib/partition.rs`

```diff
diff --git a/src/endpoint_lib/partition.rs b/src/endpoint_lib/partition.rs
--- a/src/endpoint_lib/partition.rs
+++ b/src/endpoint_lib/partition.rs
@@ -79,7 +79,11 @@
 /// Merge the base output and the override output, dealing with `Cow`s
 macro_rules! merge {
     ($base: expr, $output: expr, $field: ident) => {
-        $output.$field.as_ref().map(|s| s.as_ref()).unwrap_or($base.outputs.$field.as_ref())
+        $output
+            .$field
+            .as_ref()
+            .map(|s| s.as_ref())
+            .unwrap_or($base.outputs.$field.as_ref())
     };
 }

@@ -116,15 +120,16 @@
         let mut explicit_match_partition = self.partitions.iter().flat_map(|part| part.explicit_match(region));
         let mut regex_match_partition = self.partitions.iter().flat_map(|part| part.regex_match(region));

-        let (base, region_override) = explicit_match_partition.next().or_else(|| regex_match_partition.next()).or_else(|| {
-            match self.partitions.iter().find(|p| p.id == "aws") {
+        let (base, region_override) = explicit_match_partition
+            .next()
+            .or_else(|| regex_match_partition.next())
+            .or_else(|| match self.partitions.iter().find(|p| p.id == "aws") {
                 Some(partition) => Some((partition, None)),
                 None => {
                     e.report_error("no AWS partition!");
                     None
                 }
-            }
-        })?;
+            })?;
         let region_override = region_override.as_ref().unwrap_or(&DEFAULT_OVERRIDE);
         Some(Partition {
             name: merge!(base, region_override, name),
@@ -131,7 +136,9 @@
             dns_suffix: merge!(base, region_override, dns_suffix),
             dual_stack_dns_suffix: merge!(base, region_override, dual_stack_dns_suffix),
             supports_fips: region_override.supports_fips.unwrap_or(base.outputs.supports_fips),
-            supports_dual_stack: region_override.supports_dual_stack.unwrap_or(base.outputs.supports_dual_stack),
+            supports_dual_stack: region_override
+                .supports_dual_stack
+                .unwrap_or(base.outputs.supports_dual_stack),
             implicit_global_region: merge!(base, region_override, implicit_global_region),
         })
     }
@@ -172,7 +179,9 @@

 impl PartitionMetadata {
     fn explicit_match(&self, region: &str) -> Option<(&PartitionMetadata, Option<&PartitionOutputOverride>)> {
-        self.regions.get(region).map(|output_override| (self, Some(output_override)))
+        self.regions
+            .get(region)
+            .map(|output_override| (self, Some(output_override)))
     }

     fn regex_match(&self, region: &str) -> Option<(&PartitionMetadata, Option<&PartitionOutputOverride>)> {
@@ -221,8 +230,12 @@
 ///
 /// This code was generated by smithy-rs and then hand edited for clarity
 mod deser {
-    use crate::endpoint_lib::partition::{PartitionMetadata, PartitionMetadataBuilder, PartitionOutputOverride, PartitionResolver};
-    use aws_smithy_json::deserialize::token::{expect_bool_or_null, expect_start_object, expect_string_or_null, skip_value};
+    use crate::endpoint_lib::partition::{
+        PartitionMetadata, PartitionMetadataBuilder, PartitionOutputOverride, PartitionResolver,
+    };
+    use aws_smithy_json::deserialize::token::{
+        expect_bool_or_null, expect_start_object, expect_string_or_null, skip_value,
+    };
     use aws_smithy_json::deserialize::{error::DeserializeError, json_token_iter, Token};
     use regex_lite::Regex;
     use std::borrow::Cow;
@@ -242,11 +255,17 @@
                     }
                     _ => skip_value(tokens)?,
                 },
-                other => return Err(DeserializeError::custom(format!("expected object key or end object, found: {other:?}",))),
+                other => {
+                    return Err(DeserializeError::custom(format!(
+                        "expected object key or end object, found: {other:?}",
+                    )))
+                }
             }
         }
         if tokens.next().is_some() {
-            return Err(DeserializeError::custom("found more JSON tokens after completing parsing"));
+            return Err(DeserializeError::custom(
+                "found more JSON tokens after completing parsing",
+            ));
         }
         resolver.ok_or_else(|| DeserializeError::custom("did not find partitions array"))
     }
@@ -275,7 +294,9 @@
         }
     }

-    pub(crate) fn deser_partition<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<PartitionMetadata, DeserializeError>
+    pub(crate) fn deser_partition<'a, I>(
+        tokens: &mut std::iter::Peekable<I>,
+    ) -> Result<PartitionMetadata, DeserializeError>
     where
         I: Iterator<Item = Result<Token<'a>, DeserializeError>>,
     {
@@ -303,7 +324,11 @@
                             }
                             _ => skip_value(tokens)?,
                         },
-                        other => return Err(DeserializeError::custom(format!("expected object key or end object, found: {other:?}"))),
+                        other => {
+                            return Err(DeserializeError::custom(format!(
+                                "expected object key or end object, found: {other:?}"
+                            )))
+                        }
                     }
                 }
                 Ok(builder.build())
@@ -332,7 +357,11 @@
                                 map.insert(key.into(), value);
                             }
                         }
-                        other => return Err(DeserializeError::custom(format!("expected object key or end object, found: {other:?}"))),
+                        other => {
+                            return Err(DeserializeError::custom(format!(
+                                "expected object key or end object, found: {other:?}"
+                            )))
+                        }
                     }
                 }
                 Ok(map)
@@ -349,7 +378,9 @@
             .map(Cow::Owned))
     }

-    fn deser_outputs<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<PartitionOutputOverride>, DeserializeError>
+    fn deser_outputs<'a, I>(
+        tokens: &mut std::iter::Peekable<I>,
+    ) -> Result<Option<PartitionOutputOverride>, DeserializeError>
     where
         I: Iterator<Item = Result<Token<'a>, DeserializeError>>,
     {
@@ -381,7 +412,11 @@
                             }
                             _ => skip_value(tokens)?,
                         },
-                        other => return Err(DeserializeError::custom(format!("expected object key or end object, found: {other:?}",))),
+                        other => {
+                            return Err(DeserializeError::custom(format!(
+                                "expected object key or end object, found: {other:?}",
+                            )))
+                        }
                     }
                 }
                 Ok(Some(builder))
@@ -394,7 +429,9 @@
 #[cfg(test)]
 mod test {
     use crate::endpoint_lib::diagnostic::DiagnosticCollector;
-    use crate::endpoint_lib::partition::{Partition, PartitionMetadata, PartitionOutput, PartitionOutputOverride, PartitionResolver};
+    use crate::endpoint_lib::partition::{
+        Partition, PartitionMetadata, PartitionOutput, PartitionOutputOverride, PartitionResolver,
+    };
     use regex_lite::Regex;
     use std::collections::HashMap;

```

### `src/error.rs`

```diff
diff --git a/src/error.rs b/src/error.rs
--- a/src/error.rs
+++ b/src/error.rs
@@ -2,7 +2,8 @@
 pub use ::aws_smithy_runtime_api::box_error::BoxError;

 /// Error type returned by the client.
-pub type SdkError<E, R = ::aws_smithy_runtime_api::client::orchestrator::HttpResponse> = ::aws_smithy_runtime_api::client::result::SdkError<E, R>;
+pub type SdkError<E, R = ::aws_smithy_runtime_api::client::orchestrator::HttpResponse> =
+    ::aws_smithy_runtime_api::client::result::SdkError<E, R>;
 pub use ::aws_smithy_runtime_api::client::result::ConnectorError;
 pub use ::aws_smithy_types::error::operation::BuildError;

```

### `src/error/sealed_unhandled.rs`

```diff
diff --git a/src/error/sealed_unhandled.rs b/src/error/sealed_unhandled.rs
--- a/src/error/sealed_unhandled.rs
+++ b/src/error/sealed_unhandled.rs
@@ -7,12 +7,14 @@
 /// on the error type.
 ///
 /// This struct intentionally doesn't yield any useful information itself.
-#[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
+#[deprecated(
+    note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
 variable wildcard pattern and check `.code()`:
  \
 &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
  \
-See [`ProvideErrorMetadata`](::aws_smithy_types::error::metadata::ProvideErrorMetadata) for what information is available for the error.")]
+See [`ProvideErrorMetadata`](::aws_smithy_types::error::metadata::ProvideErrorMetadata) for what information is available for the error."
+)]
 #[derive(Debug)]
 pub struct Unhandled {
     pub(crate) source: ::aws_smithy_runtime_api::box_error::BoxError,
```

### `src/error_meta.rs`

```diff
diff --git a/src/error_meta.rs b/src/error_meta.rs
--- a/src/error_meta.rs
+++ b/src/error_meta.rs
@@ -3,58 +3,134 @@
 #[non_exhaustive]
 #[derive(::std::fmt::Debug)]
 pub enum Error {
+    #[cfg(any(
+        feature = "op_assume_role",
+        feature = "op_assume_role_with_saml",
+        feature = "op_assume_role_with_web_identity",
+        feature = "op_assume_root"
+    ))]
     /// <p>The web identity token that was passed is expired or is not valid. Get a new identity token from the identity provider and then retry the request.</p>
     ExpiredTokenException(crate::types::error::ExpiredTokenException),
+    #[cfg(feature = "op_get_delegated_access_token")]
     /// <p>The trade-in token provided in the request has expired and can no longer be exchanged for credentials. Request a new token and retry the operation.</p>
     ExpiredTradeInTokenException(crate::types::error::ExpiredTradeInTokenException),
+    #[cfg(feature = "op_assume_role_with_web_identity")]
     /// <p>The request could not be fulfilled because the identity provider (IDP) that was asked to verify the incoming identity token could not be reached. This is often a transient error caused by network conditions. Retry the request a limited number of times so that you don't exceed the request rate. If the error persists, the identity provider might be down or not responding.</p>
     IdpCommunicationErrorException(crate::types::error::IdpCommunicationErrorException),
+    #[cfg(any(feature = "op_assume_role_with_saml", feature = "op_assume_role_with_web_identity"))]
     /// <p>The identity provider (IdP) reported that authentication failed. This might be because the claim is invalid.</p>
     /// <p>If this error is returned for the <code>AssumeRoleWithWebIdentity</code> operation, it can also mean that the claim has expired or has been explicitly revoked.</p>
     IdpRejectedClaimException(crate::types::error::IdpRejectedClaimException),
+    #[cfg(feature = "op_decode_authorization_message")]
     /// <p>The error returned if the message passed to <code>DecodeAuthorizationMessage</code> was invalid. This can happen if the token contains invalid characters, such as line breaks, or if the message has expired.</p>
     InvalidAuthorizationMessageException(crate::types::error::InvalidAuthorizationMessageException),
+    #[cfg(any(feature = "op_assume_role_with_saml", feature = "op_assume_role_with_web_identity"))]
     /// <p>The web identity token that was passed could not be validated by Amazon Web Services. Get a new identity token from the identity provider and then retry the request.</p>
     InvalidIdentityTokenException(crate::types::error::InvalidIdentityTokenException),
+    #[cfg(feature = "op_get_web_identity_token")]
     /// <p>The requested token payload size exceeds the maximum allowed size. Reduce the number of request tags included in the <code>GetWebIdentityToken</code> API call to reduce the token payload size.</p>
     JwtPayloadSizeExceededException(crate::types::error::JwtPayloadSizeExceededException),
+    #[cfg(any(
+        feature = "op_assume_role",
+        feature = "op_assume_role_with_saml",
+        feature = "op_assume_role_with_web_identity",
+        feature = "op_get_federation_token"
+    ))]
     /// <p>The request was rejected because the policy document was malformed. The error message describes the specific error.</p>
     MalformedPolicyDocumentException(crate::types::error::MalformedPolicyDocumentException),
+    #[cfg(feature = "op_get_web_identity_token")]
     /// <p>The outbound web identity federation feature is not enabled for this account. To use this feature, you must first enable it through the Amazon Web Services Management Console or API.</p>
     OutboundWebIdentityFederationDisabledException(crate::types::error::OutboundWebIdentityFederationDisabledException),
+    #[cfg(any(
+        feature = "op_assume_role",
+        feature = "op_assume_role_with_saml",
+        feature = "op_assume_role_with_web_identity",
+        feature = "op_get_delegated_access_token",
+        feature = "op_get_federation_token"
+    ))]
     /// <p>The request was rejected because the total packed size of the session policies and session tags combined was too large. An Amazon Web Services conversion compresses the session policy document, session policy ARNs, and session tags into a packed binary format that has a separate limit. The error message indicates by percentage how close the policies and tags are to the upper size limit. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_session-tags.html">Passing Session Tags in STS</a> in the <i>IAM User Guide</i>.</p>
     /// <p>You could receive this error even though you meet other defined session policy and session tag limits. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_iam-quotas.html#reference_iam-limits-entity-length">IAM and STS Entity Character Limits</a> in the <i>IAM User Guide</i>.</p>
     PackedPolicyTooLargeException(crate::types::error::PackedPolicyTooLargeException),
+    #[cfg(any(
+        feature = "op_assume_role",
+        feature = "op_assume_role_with_saml",
+        feature = "op_assume_role_with_web_identity",
+        feature = "op_assume_root",
+        feature = "op_get_delegated_access_token",
+        feature = "op_get_federation_token",
+        feature = "op_get_session_token"
+    ))]
     /// <p>STS is not activated in the requested region for the account that is being asked to generate credentials. The account administrator must use the IAM console to activate STS in that region. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_enable-regions.html#sts-regions-activate-deactivate">Activating and Deactivating STS in an Amazon Web Services Region</a> in the <i>IAM User Guide</i>.</p>
     RegionDisabledException(crate::types::error::RegionDisabledException),
+    #[cfg(feature = "op_get_web_identity_token")]
     /// <p>The requested token duration would extend the session beyond its original expiration time. You cannot use this operation to extend the lifetime of a session beyond what was granted when the session was originally created.</p>
     SessionDurationEscalationException(crate::types::error::SessionDurationEscalationException),
     /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
-    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
+    #[deprecated(
+        note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
     variable wildcard pattern and check `.code()`:
      \
     &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
      \
-    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-Error) for what information is available for the error.")]
+    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-Error) for what information is available for the error."
+    )]
     Unhandled(crate::error::sealed_unhandled::Unhandled),
 }
 impl ::std::fmt::Display for Error {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
         match self {
+            #[cfg(any(
+                feature = "op_assume_role",
+                feature = "op_assume_role_with_saml",
+                feature = "op_assume_role_with_web_identity",
+                feature = "op_assume_root"
+            ))]
             Error::ExpiredTokenException(inner) => inner.fmt(f),
+            #[cfg(feature = "op_get_delegated_access_token")]
             Error::ExpiredTradeInTokenException(inner) => inner.fmt(f),
+            #[cfg(feature = "op_assume_role_with_web_identity")]
             Error::IdpCommunicationErrorException(inner) => inner.fmt(f),
+            #[cfg(any(feature = "op_assume_role_with_saml", feature = "op_assume_role_with_web_identity"))]
             Error::IdpRejectedClaimException(inner) => inner.fmt(f),
+            #[cfg(feature = "op_decode_authorization_message")]
             Error::InvalidAuthorizationMessageException(inner) => inner.fmt(f),
+            #[cfg(any(feature = "op_assume_role_with_saml", feature = "op_assume_role_with_web_identity"))]
             Error::InvalidIdentityTokenException(inner) => inner.fmt(f),
+            #[cfg(feature = "op_get_web_identity_token")]
             Error::JwtPayloadSizeExceededException(inner) => inner.fmt(f),
+            #[cfg(any(
+                feature = "op_assume_role",
+                feature = "op_assume_role_with_saml",
+                feature = "op_assume_role_with_web_identity",
+                feature = "op_get_federation_token"
+            ))]
             Error::MalformedPolicyDocumentException(inner) => inner.fmt(f),
+            #[cfg(feature = "op_get_web_identity_token")]
             Error::OutboundWebIdentityFederationDisabledException(inner) => inner.fmt(f),
+            #[cfg(any(
+                feature = "op_assume_role",
+                feature = "op_assume_role_with_saml",
+                feature = "op_assume_role_with_web_identity",
+                feature = "op_get_delegated_access_token",
+                feature = "op_get_federation_token"
+            ))]
             Error::PackedPolicyTooLargeException(inner) => inner.fmt(f),
+            #[cfg(any(
+                feature = "op_assume_role",
+                feature = "op_assume_role_with_saml",
+                feature = "op_assume_role_with_web_identity",
+                feature = "op_assume_root",
+                feature = "op_get_delegated_access_token",
+                feature = "op_get_federation_token",
+                feature = "op_get_session_token"
+            ))]
             Error::RegionDisabledException(inner) => inner.fmt(f),
+            #[cfg(feature = "op_get_web_identity_token")]
             Error::SessionDurationEscalationException(inner) => inner.fmt(f),
             Error::Unhandled(_) => {
-                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
+                if let ::std::option::Option::Some(code) =
+                    ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
+                {
                     write!(f, "unhandled error ({code})")
                 } else {
                     f.write_str("unhandled error")
@@ -74,27 +150,67 @@
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for Error {
     fn meta(&self) -> &::aws_smithy_types::error::metadata::ErrorMetadata {
         match self {
+            #[cfg(any(
+                feature = "op_assume_role",
+                feature = "op_assume_role_with_saml",
+                feature = "op_assume_role_with_web_identity",
+                feature = "op_assume_root"
+            ))]
             Self::ExpiredTokenException(inner) => inner.meta(),
+            #[cfg(feature = "op_get_delegated_access_token")]
             Self::ExpiredTradeInTokenException(inner) => inner.meta(),
+            #[cfg(feature = "op_assume_role_with_web_identity")]
             Self::IdpCommunicationErrorException(inner) => inner.meta(),
+            #[cfg(any(feature = "op_assume_role_with_saml", feature = "op_assume_role_with_web_identity"))]
             Self::IdpRejectedClaimException(inner) => inner.meta(),
+            #[cfg(feature = "op_decode_authorization_message")]
             Self::InvalidAuthorizationMessageException(inner) => inner.meta(),
+            #[cfg(any(feature = "op_assume_role_with_saml", feature = "op_assume_role_with_web_identity"))]
             Self::InvalidIdentityTokenException(inner) => inner.meta(),
+            #[cfg(feature = "op_get_web_identity_token")]
             Self::JwtPayloadSizeExceededException(inner) => inner.meta(),
+            #[cfg(any(
+                feature = "op_assume_role",
+                feature = "op_assume_role_with_saml",
+                feature = "op_assume_role_with_web_identity",
+                feature = "op_get_federation_token"
+            ))]
             Self::MalformedPolicyDocumentException(inner) => inner.meta(),
+            #[cfg(feature = "op_get_web_identity_token")]
             Self::OutboundWebIdentityFederationDisabledException(inner) => inner.meta(),
+            #[cfg(any(
+                feature = "op_assume_role",
+                feature = "op_assume_role_with_saml",
+                feature = "op_assume_role_with_web_identity",
+                feature = "op_get_delegated_access_token",
+                feature = "op_get_federation_token"
+            ))]
             Self::PackedPolicyTooLargeException(inner) => inner.meta(),
+            #[cfg(any(
+                feature = "op_assume_role",
+                feature = "op_assume_role_with_saml",
+                feature = "op_assume_role_with_web_identity",
+                feature = "op_assume_root",
+                feature = "op_get_delegated_access_token",
+                feature = "op_get_federation_token",
+                feature = "op_get_session_token"
+            ))]
             Self::RegionDisabledException(inner) => inner.meta(),
+            #[cfg(feature = "op_get_web_identity_token")]
             Self::SessionDurationEscalationException(inner) => inner.meta(),
             Self::Unhandled(inner) => &inner.meta,
         }
     }
 }
-impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::assume_role::AssumeRoleError, R>> for Error
+#[cfg(feature = "op_assume_role")]
+impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::assume_role::AssumeRoleError, R>>
+    for Error
 where
     R: Send + Sync + std::fmt::Debug + 'static,
 {
-    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::assume_role::AssumeRoleError, R>) -> Self {
+    fn from(
+        err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::assume_role::AssumeRoleError, R>,
+    ) -> Self {
         match err {
             ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
             _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
@@ -104,22 +220,43 @@
         }
     }
 }
+#[cfg(feature = "op_assume_role")]
 impl From<crate::operation::assume_role::AssumeRoleError> for Error {
     fn from(err: crate::operation::assume_role::AssumeRoleError) -> Self {
         match err {
-            crate::operation::assume_role::AssumeRoleError::ExpiredTokenException(inner) => Error::ExpiredTokenException(inner),
-            crate::operation::assume_role::AssumeRoleError::MalformedPolicyDocumentException(inner) => Error::MalformedPolicyDocumentException(inner),
-            crate::operation::assume_role::AssumeRoleError::PackedPolicyTooLargeException(inner) => Error::PackedPolicyTooLargeException(inner),
-            crate::operation::assume_role::AssumeRoleError::RegionDisabledException(inner) => Error::RegionDisabledException(inner),
+            crate::operation::assume_role::AssumeRoleError::ExpiredTokenException(inner) => {
+                Error::ExpiredTokenException(inner)
+            }
+            crate::operation::assume_role::AssumeRoleError::MalformedPolicyDocumentException(inner) => {
+                Error::MalformedPolicyDocumentException(inner)
+            }
+            crate::operation::assume_role::AssumeRoleError::PackedPolicyTooLargeException(inner) => {
+                Error::PackedPolicyTooLargeException(inner)
+            }
+            crate::operation::assume_role::AssumeRoleError::RegionDisabledException(inner) => {
+                Error::RegionDisabledException(inner)
+            }
             crate::operation::assume_role::AssumeRoleError::Unhandled(inner) => Error::Unhandled(inner),
         }
     }
 }
-impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError, R>> for Error
+#[cfg(feature = "op_assume_role_with_saml")]
+impl<R>
+    From<
+        ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError,
+            R,
+        >,
+    > for Error
 where
     R: Send + Sync + std::fmt::Debug + 'static,
 {
-    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError, R>) -> Self {
+    fn from(
+        err: ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError,
+            R,
+        >,
+    ) -> Self {
         match err {
             ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
             _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
@@ -129,10 +266,13 @@
         }
     }
 }
+#[cfg(feature = "op_assume_role_with_saml")]
 impl From<crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError> for Error {
     fn from(err: crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError) -> Self {
         match err {
-            crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::ExpiredTokenException(inner) => Error::ExpiredTokenException(inner),
+            crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::ExpiredTokenException(inner) => {
+                Error::ExpiredTokenException(inner)
+            }
             crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::IdpRejectedClaimException(inner) => {
                 Error::IdpRejectedClaimException(inner)
             }
@@ -139,24 +279,37 @@
             crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::InvalidIdentityTokenException(inner) => {
                 Error::InvalidIdentityTokenException(inner)
             }
-            crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::MalformedPolicyDocumentException(inner) => {
-                Error::MalformedPolicyDocumentException(inner)
-            }
+            crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::MalformedPolicyDocumentException(
+                inner,
+            ) => Error::MalformedPolicyDocumentException(inner),
             crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::PackedPolicyTooLargeException(inner) => {
                 Error::PackedPolicyTooLargeException(inner)
             }
-            crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::RegionDisabledException(inner) => Error::RegionDisabledException(inner),
-            crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::Unhandled(inner) => Error::Unhandled(inner),
+            crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::RegionDisabledException(inner) => {
+                Error::RegionDisabledException(inner)
+            }
+            crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::Unhandled(inner) => {
+                Error::Unhandled(inner)
+            }
         }
     }
 }
-impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError, R>>
-    for Error
+#[cfg(feature = "op_assume_role_with_web_identity")]
+impl<R>
+    From<
+        ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError,
+            R,
+        >,
+    > for Error
 where
     R: Send + Sync + std::fmt::Debug + 'static,
 {
     fn from(
-        err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError, R>,
+        err: ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError,
+            R,
+        >,
     ) -> Self {
         match err {
             ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
@@ -167,6 +320,7 @@
         }
     }
 }
+#[cfg(feature = "op_assume_role_with_web_identity")]
 impl From<crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError> for Error {
     fn from(err: crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError) -> Self {
         match err {
@@ -195,11 +349,15 @@
         }
     }
 }
-impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::assume_root::AssumeRootError, R>> for Error
+#[cfg(feature = "op_assume_root")]
+impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::assume_root::AssumeRootError, R>>
+    for Error
 where
     R: Send + Sync + std::fmt::Debug + 'static,
 {
-    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::assume_root::AssumeRootError, R>) -> Self {
+    fn from(
+        err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::assume_root::AssumeRootError, R>,
+    ) -> Self {
         match err {
             ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
             _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
@@ -209,22 +367,36 @@
         }
     }
 }
+#[cfg(feature = "op_assume_root")]
 impl From<crate::operation::assume_root::AssumeRootError> for Error {
     fn from(err: crate::operation::assume_root::AssumeRootError) -> Self {
         match err {
-            crate::operation::assume_root::AssumeRootError::ExpiredTokenException(inner) => Error::ExpiredTokenException(inner),
-            crate::operation::assume_root::AssumeRootError::RegionDisabledException(inner) => Error::RegionDisabledException(inner),
+            crate::operation::assume_root::AssumeRootError::ExpiredTokenException(inner) => {
+                Error::ExpiredTokenException(inner)
+            }
+            crate::operation::assume_root::AssumeRootError::RegionDisabledException(inner) => {
+                Error::RegionDisabledException(inner)
+            }
             crate::operation::assume_root::AssumeRootError::Unhandled(inner) => Error::Unhandled(inner),
         }
     }
 }
-impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::decode_authorization_message::DecodeAuthorizationMessageError, R>>
-    for Error
+#[cfg(feature = "op_decode_authorization_message")]
+impl<R>
+    From<
+        ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::decode_authorization_message::DecodeAuthorizationMessageError,
+            R,
+        >,
+    > for Error
 where
     R: Send + Sync + std::fmt::Debug + 'static,
 {
     fn from(
-        err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::decode_authorization_message::DecodeAuthorizationMessageError, R>,
+        err: ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::decode_authorization_message::DecodeAuthorizationMessageError,
+            R,
+        >,
     ) -> Self {
         match err {
             ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
@@ -235,6 +407,7 @@
         }
     }
 }
+#[cfg(feature = "op_decode_authorization_message")]
 impl From<crate::operation::decode_authorization_message::DecodeAuthorizationMessageError> for Error {
     fn from(err: crate::operation::decode_authorization_message::DecodeAuthorizationMessageError) -> Self {
         match err {
@@ -245,11 +418,23 @@
         }
     }
 }
-impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_access_key_info::GetAccessKeyInfoError, R>> for Error
+#[cfg(feature = "op_get_access_key_info")]
+impl<R>
+    From<
+        ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::get_access_key_info::GetAccessKeyInfoError,
+            R,
+        >,
+    > for Error
 where
     R: Send + Sync + std::fmt::Debug + 'static,
 {
-    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_access_key_info::GetAccessKeyInfoError, R>) -> Self {
+    fn from(
+        err: ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::get_access_key_info::GetAccessKeyInfoError,
+            R,
+        >,
+    ) -> Self {
         match err {
             ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
             _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
@@ -259,6 +444,7 @@
         }
     }
 }
+#[cfg(feature = "op_get_access_key_info")]
 impl From<crate::operation::get_access_key_info::GetAccessKeyInfoError> for Error {
     fn from(err: crate::operation::get_access_key_info::GetAccessKeyInfoError) -> Self {
         match err {
@@ -266,11 +452,23 @@
         }
     }
 }
-impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_caller_identity::GetCallerIdentityError, R>> for Error
+#[cfg(feature = "op_get_caller_identity")]
+impl<R>
+    From<
+        ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::get_caller_identity::GetCallerIdentityError,
+            R,
+        >,
+    > for Error
 where
     R: Send + Sync + std::fmt::Debug + 'static,
 {
-    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_caller_identity::GetCallerIdentityError, R>) -> Self {
+    fn from(
+        err: ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::get_caller_identity::GetCallerIdentityError,
+            R,
+        >,
+    ) -> Self {
         match err {
             ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
             _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
@@ -280,6 +478,7 @@
         }
     }
 }
+#[cfg(feature = "op_get_caller_identity")]
 impl From<crate::operation::get_caller_identity::GetCallerIdentityError> for Error {
     fn from(err: crate::operation::get_caller_identity::GetCallerIdentityError) -> Self {
         match err {
@@ -287,13 +486,22 @@
         }
     }
 }
-impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError, R>>
-    for Error
+#[cfg(feature = "op_get_delegated_access_token")]
+impl<R>
+    From<
+        ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError,
+            R,
+        >,
+    > for Error
 where
     R: Send + Sync + std::fmt::Debug + 'static,
 {
     fn from(
-        err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError, R>,
+        err: ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError,
+            R,
+        >,
     ) -> Self {
         match err {
             ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
@@ -304,6 +512,7 @@
         }
     }
 }
+#[cfg(feature = "op_get_delegated_access_token")]
 impl From<crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError> for Error {
     fn from(err: crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError) -> Self {
         match err {
@@ -320,11 +529,23 @@
         }
     }
 }
-impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_federation_token::GetFederationTokenError, R>> for Error
+#[cfg(feature = "op_get_federation_token")]
+impl<R>
+    From<
+        ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::get_federation_token::GetFederationTokenError,
+            R,
+        >,
+    > for Error
 where
     R: Send + Sync + std::fmt::Debug + 'static,
 {
-    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_federation_token::GetFederationTokenError, R>) -> Self {
+    fn from(
+        err: ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::get_federation_token::GetFederationTokenError,
+            R,
+        >,
+    ) -> Self {
         match err {
             ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
             _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
@@ -334,25 +555,42 @@
         }
     }
 }
+#[cfg(feature = "op_get_federation_token")]
 impl From<crate::operation::get_federation_token::GetFederationTokenError> for Error {
     fn from(err: crate::operation::get_federation_token::GetFederationTokenError) -> Self {
         match err {
-            crate::operation::get_federation_token::GetFederationTokenError::MalformedPolicyDocumentException(inner) => {
-                Error::MalformedPolicyDocumentException(inner)
-            }
+            crate::operation::get_federation_token::GetFederationTokenError::MalformedPolicyDocumentException(
+                inner,
+            ) => Error::MalformedPolicyDocumentException(inner),
             crate::operation::get_federation_token::GetFederationTokenError::PackedPolicyTooLargeException(inner) => {
                 Error::PackedPolicyTooLargeException(inner)
             }
-            crate::operation::get_federation_token::GetFederationTokenError::RegionDisabledException(inner) => Error::RegionDisabledException(inner),
-            crate::operation::get_federation_token::GetFederationTokenError::Unhandled(inner) => Error::Unhandled(inner),
+            crate::operation::get_federation_token::GetFederationTokenError::RegionDisabledException(inner) => {
+                Error::RegionDisabledException(inner)
+            }
+            crate::operation::get_federation_token::GetFederationTokenError::Unhandled(inner) => {
+                Error::Unhandled(inner)
+            }
         }
     }
 }
-impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_session_token::GetSessionTokenError, R>> for Error
+#[cfg(feature = "op_get_session_token")]
+impl<R>
+    From<
+        ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::get_session_token::GetSessionTokenError,
+            R,
+        >,
+    > for Error
 where
     R: Send + Sync + std::fmt::Debug + 'static,
 {
-    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_session_token::GetSessionTokenError, R>) -> Self {
+    fn from(
+        err: ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::get_session_token::GetSessionTokenError,
+            R,
+        >,
+    ) -> Self {
         match err {
             ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
             _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
@@ -362,19 +600,34 @@
         }
     }
 }
+#[cfg(feature = "op_get_session_token")]
 impl From<crate::operation::get_session_token::GetSessionTokenError> for Error {
     fn from(err: crate::operation::get_session_token::GetSessionTokenError) -> Self {
         match err {
-            crate::operation::get_session_token::GetSessionTokenError::RegionDisabledException(inner) => Error::RegionDisabledException(inner),
+            crate::operation::get_session_token::GetSessionTokenError::RegionDisabledException(inner) => {
+                Error::RegionDisabledException(inner)
+            }
             crate::operation::get_session_token::GetSessionTokenError::Unhandled(inner) => Error::Unhandled(inner),
         }
     }
 }
-impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_web_identity_token::GetWebIdentityTokenError, R>> for Error
+#[cfg(feature = "op_get_web_identity_token")]
+impl<R>
+    From<
+        ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::get_web_identity_token::GetWebIdentityTokenError,
+            R,
+        >,
+    > for Error
 where
     R: Send + Sync + std::fmt::Debug + 'static,
 {
-    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_web_identity_token::GetWebIdentityTokenError, R>) -> Self {
+    fn from(
+        err: ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::get_web_identity_token::GetWebIdentityTokenError,
+            R,
+        >,
+    ) -> Self {
         match err {
             ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
             _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
@@ -384,6 +637,7 @@
         }
     }
 }
+#[cfg(feature = "op_get_web_identity_token")]
 impl From<crate::operation::get_web_identity_token::GetWebIdentityTokenError> for Error {
     fn from(err: crate::operation::get_web_identity_token::GetWebIdentityTokenError) -> Self {
         match err {
@@ -403,17 +657,53 @@
 impl ::std::error::Error for Error {
     fn source(&self) -> std::option::Option<&(dyn ::std::error::Error + 'static)> {
         match self {
+            #[cfg(any(
+                feature = "op_assume_role",
+                feature = "op_assume_role_with_saml",
+                feature = "op_assume_role_with_web_identity",
+                feature = "op_assume_root"
+            ))]
             Error::ExpiredTokenException(inner) => inner.source(),
+            #[cfg(feature = "op_get_delegated_access_token")]
             Error::ExpiredTradeInTokenException(inner) => inner.source(),
+            #[cfg(feature = "op_assume_role_with_web_identity")]
             Error::IdpCommunicationErrorException(inner) => inner.source(),
+            #[cfg(any(feature = "op_assume_role_with_saml", feature = "op_assume_role_with_web_identity"))]
             Error::IdpRejectedClaimException(inner) => inner.source(),
+            #[cfg(feature = "op_decode_authorization_message")]
             Error::InvalidAuthorizationMessageException(inner) => inner.source(),
+            #[cfg(any(feature = "op_assume_role_with_saml", feature = "op_assume_role_with_web_identity"))]
             Error::InvalidIdentityTokenException(inner) => inner.source(),
+            #[cfg(feature = "op_get_web_identity_token")]
             Error::JwtPayloadSizeExceededException(inner) => inner.source(),
+            #[cfg(any(
+                feature = "op_assume_role",
+                feature = "op_assume_role_with_saml",
+                feature = "op_assume_role_with_web_identity",
+                feature = "op_get_federation_token"
+            ))]
             Error::MalformedPolicyDocumentException(inner) => inner.source(),
+            #[cfg(feature = "op_get_web_identity_token")]
             Error::OutboundWebIdentityFederationDisabledException(inner) => inner.source(),
+            #[cfg(any(
+                feature = "op_assume_role",
+                feature = "op_assume_role_with_saml",
+                feature = "op_assume_role_with_web_identity",
+                feature = "op_get_delegated_access_token",
+                feature = "op_get_federation_token"
+            ))]
             Error::PackedPolicyTooLargeException(inner) => inner.source(),
+            #[cfg(any(
+                feature = "op_assume_role",
+                feature = "op_assume_role_with_saml",
+                feature = "op_assume_role_with_web_identity",
+                feature = "op_assume_root",
+                feature = "op_get_delegated_access_token",
+                feature = "op_get_federation_token",
+                feature = "op_get_session_token"
+            ))]
             Error::RegionDisabledException(inner) => inner.source(),
+            #[cfg(feature = "op_get_web_identity_token")]
             Error::SessionDurationEscalationException(inner) => inner.source(),
             Error::Unhandled(inner) => ::std::option::Option::Some(&*inner.source),
         }
@@ -422,17 +712,53 @@
 impl ::aws_types::request_id::RequestId for Error {
     fn request_id(&self) -> Option<&str> {
         match self {
+            #[cfg(any(
+                feature = "op_assume_role",
+                feature = "op_assume_role_with_saml",
+                feature = "op_assume_role_with_web_identity",
+                feature = "op_assume_root"
+            ))]
             Self::ExpiredTokenException(e) => e.request_id(),
+            #[cfg(feature = "op_get_delegated_access_token")]
             Self::ExpiredTradeInTokenException(e) => e.request_id(),
+            #[cfg(feature = "op_assume_role_with_web_identity")]
             Self::IdpCommunicationErrorException(e) => e.request_id(),
+            #[cfg(any(feature = "op_assume_role_with_saml", feature = "op_assume_role_with_web_identity"))]
             Self::IdpRejectedClaimException(e) => e.request_id(),
+            #[cfg(feature = "op_decode_authorization_message")]
             Self::InvalidAuthorizationMessageException(e) => e.request_id(),
+            #[cfg(any(feature = "op_assume_role_with_saml", feature = "op_assume_role_with_web_identity"))]
             Self::InvalidIdentityTokenException(e) => e.request_id(),
+            #[cfg(feature = "op_get_web_identity_token")]
             Self::JwtPayloadSizeExceededException(e) => e.request_id(),
+            #[cfg(any(
+                feature = "op_assume_role",
+                feature = "op_assume_role_with_saml",
+                feature = "op_assume_role_with_web_identity",
+                feature = "op_get_federation_token"
+            ))]
             Self::MalformedPolicyDocumentException(e) => e.request_id(),
+            #[cfg(feature = "op_get_web_identity_token")]
             Self::OutboundWebIdentityFederationDisabledException(e) => e.request_id(),
+            #[cfg(any(
+                feature = "op_assume_role",
+                feature = "op_assume_role_with_saml",
+                feature = "op_assume_role_with_web_identity",
+                feature = "op_get_delegated_access_token",
+                feature = "op_get_federation_token"
+            ))]
             Self::PackedPolicyTooLargeException(e) => e.request_id(),
+            #[cfg(any(
+                feature = "op_assume_role",
+                feature = "op_assume_role_with_saml",
+                feature = "op_assume_role_with_web_identity",
+                feature = "op_assume_root",
+                feature = "op_get_delegated_access_token",
+                feature = "op_get_federation_token",
+                feature = "op_get_session_token"
+            ))]
             Self::RegionDisabledException(e) => e.request_id(),
+            #[cfg(feature = "op_get_web_identity_token")]
             Self::SessionDurationEscalationException(e) => e.request_id(),
             Self::Unhandled(e) => e.meta.request_id(),
         }
```

### `src/meta.rs`

```diff
diff --git a/src/meta.rs b/src/meta.rs
--- a/src/meta.rs
+++ b/src/meta.rs
@@ -1,5 +1,6 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-pub(crate) static API_METADATA: ::aws_runtime::user_agent::ApiMetadata = ::aws_runtime::user_agent::ApiMetadata::new("sts", crate::meta::PKG_VERSION);
+pub(crate) static API_METADATA: ::aws_runtime::user_agent::ApiMetadata =
+    ::aws_runtime::user_agent::ApiMetadata::new("sts", crate::meta::PKG_VERSION);

 /// Crate version number.
 pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
```

### `src/observability_feature.rs`

```diff
diff --git a/src/observability_feature.rs b/src/observability_feature.rs
--- a/src/observability_feature.rs
+++ b/src/observability_feature.rs
@@ -21,7 +21,11 @@
         "ObservabilityFeatureTrackerInterceptor"
     }

-    fn read_before_execution(&self, _context: &BeforeSerializationInterceptorContextRef<'_>, cfg: &mut ConfigBag) -> Result<(), BoxError> {
+    fn read_before_execution(
+        &self,
+        _context: &BeforeSerializationInterceptorContextRef<'_>,
+        cfg: &mut ConfigBag,
+    ) -> Result<(), BoxError> {
         // Check if an OpenTelemetry meter provider is configured via the global provider
         if let Ok(telemetry_provider) = aws_smithy_observability::global::get_telemetry_provider() {
             let meter_provider = telemetry_provider.meter_provider();
@@ -28,7 +32,8 @@

             // Use provider_name() to detect OpenTelemetry without importing the otel crate.
             if meter_provider.provider_name() == "AwsSmithyObservabilityOtelProvider" {
-                cfg.interceptor_state().store_append(SmithySdkFeature::ObservabilityOtelMetrics);
+                cfg.interceptor_state()
+                    .store_append(SmithySdkFeature::ObservabilityOtelMetrics);
             }
         }

```

### `src/operation.rs`

```diff
diff --git a/src/operation.rs b/src/operation.rs
--- a/src/operation.rs
+++ b/src/operation.rs
@@ -1,35 +1,46 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub use ::aws_types::request_id::RequestId;

+#[cfg(feature = "op_assume_role")]
 /// Types for the `AssumeRole` operation.
 pub mod assume_role;

+#[cfg(feature = "op_assume_role_with_saml")]
 /// Types for the `AssumeRoleWithSAML` operation.
 pub mod assume_role_with_saml;

+#[cfg(feature = "op_assume_role_with_web_identity")]
 /// Types for the `AssumeRoleWithWebIdentity` operation.
 pub mod assume_role_with_web_identity;

+#[cfg(feature = "op_assume_root")]
 /// Types for the `AssumeRoot` operation.
 pub mod assume_root;

+#[cfg(feature = "op_decode_authorization_message")]
 /// Types for the `DecodeAuthorizationMessage` operation.
 pub mod decode_authorization_message;

+#[cfg(feature = "op_get_access_key_info")]
 /// Types for the `GetAccessKeyInfo` operation.
 pub mod get_access_key_info;

+#[cfg(feature = "op_get_caller_identity")]
 /// Types for the `GetCallerIdentity` operation.
 pub mod get_caller_identity;

+#[cfg(feature = "op_get_delegated_access_token")]
 /// Types for the `GetDelegatedAccessToken` operation.
 pub mod get_delegated_access_token;

+#[cfg(feature = "op_get_federation_token")]
 /// Types for the `GetFederationToken` operation.
 pub mod get_federation_token;

+#[cfg(feature = "op_get_session_token")]
 /// Types for the `GetSessionToken` operation.
 pub mod get_session_token;

+#[cfg(feature = "op_get_web_identity_token")]
 /// Types for the `GetWebIdentityToken` operation.
 pub mod get_web_identity_token;
```

### `src/operation/assume_role.rs`

```diff
diff --git a/src/operation/assume_role.rs b/src/operation/assume_role.rs
--- a/src/operation/assume_role.rs
+++ b/src/operation/assume_role.rs
@@ -27,9 +27,13 @@
                     .expect("correct error type")
             })
         };
-        let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
-            .await
-            .map_err(map_err)?;
+        let context = Self::orchestrate_with_stop_point(
+            runtime_plugins,
+            input,
+            ::aws_smithy_runtime::client::orchestrator::StopPoint::None,
+        )
+        .await
+        .map_err(map_err)?;
         let output = context.finalize().map_err(map_err)?;
         ::std::result::Result::Ok(
             output
@@ -51,17 +55,23 @@
     > {
         let input = ::aws_smithy_runtime_api::client::interceptors::context::Input::erase(input);
         use ::tracing::Instrument;
-        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("STS", "AssumeRole", input, runtime_plugins, stop_point)
-            // Create a parent span for the entire operation. Includes a random, internal-only,
-            // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
-            .instrument(::tracing::debug_span!(
-                "STS.AssumeRole",
-                "rpc.service" = "STS",
-                "rpc.method" = "AssumeRole",
-                "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
-                "rpc.system" = "aws-api",
-            ))
-            .await
+        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point(
+            "STS",
+            "AssumeRole",
+            input,
+            runtime_plugins,
+            stop_point,
+        )
+        // Create a parent span for the entire operation. Includes a random, internal-only,
+        // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
+        .instrument(::tracing::debug_span!(
+            "STS.AssumeRole",
+            "rpc.service" = "STS",
+            "rpc.method" = "AssumeRole",
+            "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
+            "rpc.system" = "aws-api",
+        ))
+        .await
     }

     pub(crate) fn operation_runtime_plugins(
@@ -91,19 +101,24 @@
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
             AssumeRoleRequestSerializer,
         ));
-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            AssumeRoleResponseDeserializer,
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(AssumeRoleResponseDeserializer),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
-            crate::config::auth::Params::builder()
-                .operation_name("AssumeRole")
-                .build()
-                .expect("required fields set"),
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
+                crate::config::auth::Params::builder()
+                    .operation_name("AssumeRole")
+                    .build()
+                    .expect("required fields set"),
+            ),
+        );

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::SensitiveOutput);
-        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("AssumeRole", "STS"));
+        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
+            "AssumeRole",
+            "STS",
+        ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
         signing_options.double_uri_encode = true;
         signing_options.content_sha256_header = false;
@@ -251,13 +266,23 @@
 #[derive(Debug)]
 struct AssumeRoleRequestSerializer;
 impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for AssumeRoleRequestSerializer {
-    #[allow(unused_mut, clippy::let_and_return, clippy::needless_borrow, clippy::useless_conversion)]
+    #[allow(
+        unused_mut,
+        clippy::let_and_return,
+        clippy::needless_borrow,
+        clippy::useless_conversion
+    )]
     fn serialize_input(
         &self,
         input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
         _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
-    ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
-        let input = input.downcast::<crate::operation::assume_role::AssumeRoleInput>().expect("correct type");
+    ) -> ::std::result::Result<
+        ::aws_smithy_runtime_api::client::orchestrator::HttpRequest,
+        ::aws_smithy_runtime_api::box_error::BoxError,
+    > {
+        let input = input
+            .downcast::<crate::operation::assume_role::AssumeRoleInput>()
+            .expect("correct type");
         let _header_serialization_settings = _cfg
             .load::<crate::serialization_settings::HeaderSerializationSettings>()
             .cloned()
@@ -276,21 +301,30 @@
             fn update_http_builder(
                 input: &crate::operation::assume_role::AssumeRoleInput,
                 builder: ::http_1x::request::Builder,
-            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError>
+            {
                 let mut uri = ::std::string::String::new();
                 uri_base(input, &mut uri)?;
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(
+                builder,
+                ::http_1x::header::CONTENT_TYPE,
+                "application/x-www-form-urlencoded",
+            );
             builder
         };
-        let body =
-            ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_assume_role_input::ser_assume_role_input_input_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            crate::protocol_serde::shape_assume_role_input::ser_assume_role_input_input_input(&input)?,
+        );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
-            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+            request_builder = _header_serialization_settings.set_default_header(
+                request_builder,
+                ::http_1x::header::CONTENT_LENGTH,
+                &content_length,
+            );
         }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
@@ -323,13 +357,21 @@
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
-            .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
+            .set_endpoint(
+                cfg.load::<::aws_types::endpoint_config::EndpointUrl>()
+                    .map(|ty| ty.0.clone()),
+            )
             .build()
             .map_err(|err| {
-                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new("endpoint params could not be built", err)
+                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new(
+                    "endpoint params could not be built",
+                    err,
+                )
             })?;
         cfg.interceptor_state()
-            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params));
+            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(
+                params,
+            ));
         ::std::result::Result::Ok(())
     }
 }
@@ -351,18 +393,22 @@
     /// <p>STS is not activated in the requested region for the account that is being asked to generate credentials. The account administrator must use the IAM console to activate STS in that region. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_enable-regions.html#sts-regions-activate-deactivate">Activating and Deactivating STS in an Amazon Web Services Region</a> in the <i>IAM User Guide</i>.</p>
     RegionDisabledException(crate::types::error::RegionDisabledException),
     /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
-    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
+    #[deprecated(
+        note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
     variable wildcard pattern and check `.code()`:
      \
     &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
      \
-    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-AssumeRoleError) for what information is available for the error.")]
+    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-AssumeRoleError) for what information is available for the error."
+    )]
     Unhandled(crate::error::sealed_unhandled::Unhandled),
 }
 impl AssumeRoleError {
     /// Creates the `AssumeRoleError::Unhandled` variant from any error type.
     pub fn unhandled(
-        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
+        err: impl ::std::convert::Into<
+            ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
+        >,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
             source: err.into(),
@@ -384,8 +430,12 @@
     pub fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
             Self::ExpiredTokenException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::MalformedPolicyDocumentException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::PackedPolicyTooLargeException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::MalformedPolicyDocumentException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
+            Self::PackedPolicyTooLargeException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
             Self::RegionDisabledException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
             Self::Unhandled(e) => &e.meta,
         }
@@ -426,7 +476,9 @@
             Self::PackedPolicyTooLargeException(_inner) => _inner.fmt(f),
             Self::RegionDisabledException(_inner) => _inner.fmt(f),
             Self::Unhandled(_inner) => {
-                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
+                if let ::std::option::Option::Some(code) =
+                    ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
+                {
                     write!(f, "unhandled error ({code})")
                 } else {
                     f.write_str("unhandled error")
@@ -446,10 +498,18 @@
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for AssumeRoleError {
     fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::ExpiredTokenException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::MalformedPolicyDocumentException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::PackedPolicyTooLargeException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::RegionDisabledException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::ExpiredTokenException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::MalformedPolicyDocumentException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::PackedPolicyTooLargeException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::RegionDisabledException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
             Self::Unhandled(_inner) => &_inner.meta,
         }
     }
```

### `src/operation/assume_role/_assume_role_input.rs`

```diff
diff --git a/src/operation/assume_role/_assume_role_input.rs b/src/operation/assume_role/_assume_role_input.rs
--- a/src/operation/assume_role/_assume_role_input.rs
+++ b/src/operation/assume_role/_assume_role_input.rs
@@ -232,7 +232,10 @@
     /// <p>An Amazon Web Services conversion compresses the passed inline session policy, managed policy ARNs, and session tags into a packed binary format that has a separate limit. Your request can fail for this limit even if your plaintext meets the other requirements. The <code>PackedPolicySize</code> response element indicates by percentage how close the policies and tags for your request are to the upper size limit.</p>
     /// </note>
     /// <p>Passing policies to this operation returns new temporary credentials. The resulting session's permissions are the intersection of the role's identity-based policy and the session policies. You can use the role's temporary credentials in subsequent Amazon Web Services API calls to access resources in the account that owns the role. You cannot use session policies to grant more permissions than those allowed by the identity-based policy of the role that is being assumed. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#policies_session">Session Policies</a> in the <i>IAM User Guide</i>.</p>
-    pub fn set_policy_arns(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::PolicyDescriptorType>>) -> Self {
+    pub fn set_policy_arns(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<crate::types::PolicyDescriptorType>>,
+    ) -> Self {
         self.policy_arns = input;
         self
     }
@@ -353,7 +356,10 @@
     /// <p>A list of keys for session tags that you want to set as transitive. If you set a tag key as transitive, the corresponding key and value passes to subsequent sessions in a role chain. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_session-tags.html#id_session-tags_role-chaining">Chaining Roles with Session Tags</a> in the <i>IAM User Guide</i>.</p>
     /// <p>This parameter is optional. The transitive status of a session tag does not impact its packed binary size.</p>
     /// <p>If you choose not to specify a transitive tag key, then no tags are passed from this session to any subsequent sessions.</p>
-    pub fn set_transitive_tag_keys(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
+    pub fn set_transitive_tag_keys(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
+    ) -> Self {
         self.transitive_tag_keys = input;
         self
     }
@@ -450,7 +456,10 @@
     /// <p>A list of previously acquired trusted context assertions in the format of a JSON array. The trusted context assertion is signed and encrypted by Amazon Web Services STS.</p>
     /// <p>The following is an example of a <code>ProvidedContext</code> value that includes a single trusted context assertion and the ARN of the context provider from which the trusted context assertion was generated.</p>
     /// <p><code>\[{"ProviderArn":"arn:aws:iam::aws:contextProvider/IdentityCenter","ContextAssertion":"trusted-context-assertion"}\]</code></p>
-    pub fn set_provided_contexts(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ProvidedContext>>) -> Self {
+    pub fn set_provided_contexts(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<crate::types::ProvidedContext>>,
+    ) -> Self {
         self.provided_contexts = input;
         self
     }
@@ -461,7 +470,12 @@
         &self.provided_contexts
     }
     /// Consumes the builder and constructs a [`AssumeRoleInput`](crate::operation::assume_role::AssumeRoleInput).
-    pub fn build(self) -> ::std::result::Result<crate::operation::assume_role::AssumeRoleInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<
+        crate::operation::assume_role::AssumeRoleInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(crate::operation::assume_role::AssumeRoleInput {
             role_arn: self.role_arn,
             role_session_name: self.role_session_name,
```

### `src/operation/assume_role/builders.rs`

```diff
diff --git a/src/operation/assume_role/builders.rs b/src/operation/assume_role/builders.rs
--- a/src/operation/assume_role/builders.rs
+++ b/src/operation/assume_role/builders.rs
@@ -122,12 +122,18 @@
     > {
         crate::client::customize::CustomizableOperation::new(self)
     }
-    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
+    pub(crate) fn config_override(
+        mut self,
+        config_override: impl ::std::convert::Into<crate::config::Builder>,
+    ) -> Self {
         self.set_config_override(::std::option::Option::Some(config_override.into()));
         self
     }

-    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
+    pub(crate) fn set_config_override(
+        &mut self,
+        config_override: ::std::option::Option<crate::config::Builder>,
+    ) -> &mut Self {
         self.config_override = config_override;
         self
     }
@@ -187,7 +193,10 @@
     /// <p>An Amazon Web Services conversion compresses the passed inline session policy, managed policy ARNs, and session tags into a packed binary format that has a separate limit. Your request can fail for this limit even if your plaintext meets the other requirements. The <code>PackedPolicySize</code> response element indicates by percentage how close the policies and tags for your request are to the upper size limit.</p>
     /// </note>
     /// <p>Passing policies to this operation returns new temporary credentials. The resulting session's permissions are the intersection of the role's identity-based policy and the session policies. You can use the role's temporary credentials in subsequent Amazon Web Services API calls to access resources in the account that owns the role. You cannot use session policies to grant more permissions than those allowed by the identity-based policy of the role that is being assumed. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#policies_session">Session Policies</a> in the <i>IAM User Guide</i>.</p>
-    pub fn set_policy_arns(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::PolicyDescriptorType>>) -> Self {
+    pub fn set_policy_arns(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<crate::types::PolicyDescriptorType>>,
+    ) -> Self {
         self.inner = self.inner.set_policy_arns(input);
         self
     }
@@ -306,7 +315,10 @@
     /// <p>A list of keys for session tags that you want to set as transitive. If you set a tag key as transitive, the corresponding key and value passes to subsequent sessions in a role chain. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_session-tags.html#id_session-tags_role-chaining">Chaining Roles with Session Tags</a> in the <i>IAM User Guide</i>.</p>
     /// <p>This parameter is optional. The transitive status of a session tag does not impact its packed binary size.</p>
     /// <p>If you choose not to specify a transitive tag key, then no tags are passed from this session to any subsequent sessions.</p>
-    pub fn set_transitive_tag_keys(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
+    pub fn set_transitive_tag_keys(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
+    ) -> Self {
         self.inner = self.inner.set_transitive_tag_keys(input);
         self
     }
@@ -402,7 +414,10 @@
     /// <p>A list of previously acquired trusted context assertions in the format of a JSON array. The trusted context assertion is signed and encrypted by Amazon Web Services STS.</p>
     /// <p>The following is an example of a <code>ProvidedContext</code> value that includes a single trusted context assertion and the ARN of the context provider from which the trusted context assertion was generated.</p>
     /// <p><code>\[{"ProviderArn":"arn:aws:iam::aws:contextProvider/IdentityCenter","ContextAssertion":"trusted-context-assertion"}\]</code></p>
-    pub fn set_provided_contexts(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ProvidedContext>>) -> Self {
+    pub fn set_provided_contexts(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<crate::types::ProvidedContext>>,
+    ) -> Self {
         self.inner = self.inner.set_provided_contexts(input);
         self
     }
```

### `src/operation/assume_role_with_saml.rs`

```diff
diff --git a/src/operation/assume_role_with_saml.rs b/src/operation/assume_role_with_saml.rs
--- a/src/operation/assume_role_with_saml.rs
+++ b/src/operation/assume_role_with_saml.rs
@@ -27,9 +27,13 @@
                     .expect("correct error type")
             })
         };
-        let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
-            .await
-            .map_err(map_err)?;
+        let context = Self::orchestrate_with_stop_point(
+            runtime_plugins,
+            input,
+            ::aws_smithy_runtime::client::orchestrator::StopPoint::None,
+        )
+        .await
+        .map_err(map_err)?;
         let output = context.finalize().map_err(map_err)?;
         ::std::result::Result::Ok(
             output
@@ -51,17 +55,23 @@
     > {
         let input = ::aws_smithy_runtime_api::client::interceptors::context::Input::erase(input);
         use ::tracing::Instrument;
-        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("STS", "AssumeRoleWithSAML", input, runtime_plugins, stop_point)
-            // Create a parent span for the entire operation. Includes a random, internal-only,
-            // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
-            .instrument(::tracing::debug_span!(
-                "STS.AssumeRoleWithSAML",
-                "rpc.service" = "STS",
-                "rpc.method" = "AssumeRoleWithSAML",
-                "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
-                "rpc.system" = "aws-api",
-            ))
-            .await
+        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point(
+            "STS",
+            "AssumeRoleWithSAML",
+            input,
+            runtime_plugins,
+            stop_point,
+        )
+        // Create a parent span for the entire operation. Includes a random, internal-only,
+        // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
+        .instrument(::tracing::debug_span!(
+            "STS.AssumeRoleWithSAML",
+            "rpc.service" = "STS",
+            "rpc.method" = "AssumeRoleWithSAML",
+            "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
+            "rpc.system" = "aws-api",
+        ))
+        .await
     }

     pub(crate) fn operation_runtime_plugins(
@@ -91,19 +101,26 @@
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
             AssumeRoleWithSAMLRequestSerializer,
         ));
-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            AssumeRoleWithSAMLResponseDeserializer,
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
+                AssumeRoleWithSAMLResponseDeserializer,
+            ),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
-            crate::config::auth::Params::builder()
-                .operation_name("AssumeRoleWithSAML")
-                .build()
-                .expect("required fields set"),
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
+                crate::config::auth::Params::builder()
+                    .operation_name("AssumeRoleWithSAML")
+                    .build()
+                    .expect("required fields set"),
+            ),
+        );

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::SensitiveOutput);
-        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("AssumeRoleWithSAML", "STS"));
+        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
+            "AssumeRoleWithSAML",
+            "STS",
+        ));

         ::std::option::Option::Some(cfg.freeze())
     }
@@ -113,17 +130,46 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("AssumeRoleWithSAML")
-                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(AssumeRoleWithSAMLTelemetryInputCaptureInterceptor))
-.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
-.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(AssumeRoleWithSAMLEndpointParamsInterceptor))
-                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError>::new())
-.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError>::new())
-.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError>::builder().transient_errors({
-                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
-                                            transient_errors.push("IDPCommunicationError");
-                                            ::std::borrow::Cow::Owned(transient_errors)
-                                            }).build());
+        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new(
+            "AssumeRoleWithSAML",
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                AssumeRoleWithSAMLTelemetryInputCaptureInterceptor,
+            ),
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
+            ),
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                AssumeRoleWithSAMLEndpointParamsInterceptor,
+            ),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+                crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError,
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+                crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError,
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError,
+            >::builder()
+            .transient_errors({
+                let mut transient_errors: Vec<&'static str> =
+                    ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                transient_errors.push("IDPCommunicationError");
+                ::std::borrow::Cow::Owned(transient_errors)
+            })
+            .build(),
+        );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -197,9 +243,13 @@
         let mut force_error = false;
         ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));
         let parse_result = if !success && status != 200 || force_error {
-            crate::protocol_serde::shape_assume_role_with_saml::de_assume_role_with_saml_http_error(status, headers, body)
+            crate::protocol_serde::shape_assume_role_with_saml::de_assume_role_with_saml_http_error(
+                status, headers, body,
+            )
         } else {
-            crate::protocol_serde::shape_assume_role_with_saml::de_assume_role_with_saml_http_response(status, headers, body)
+            crate::protocol_serde::shape_assume_role_with_saml::de_assume_role_with_saml_http_response(
+                status, headers, body,
+            )
         };
         crate::protocol_serde::type_erase_result(parse_result)
     }
@@ -207,12 +257,20 @@
 #[derive(Debug)]
 struct AssumeRoleWithSAMLRequestSerializer;
 impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for AssumeRoleWithSAMLRequestSerializer {
-    #[allow(unused_mut, clippy::let_and_return, clippy::needless_borrow, clippy::useless_conversion)]
+    #[allow(
+        unused_mut,
+        clippy::let_and_return,
+        clippy::needless_borrow,
+        clippy::useless_conversion
+    )]
     fn serialize_input(
         &self,
         input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
         _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
-    ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
+    ) -> ::std::result::Result<
+        ::aws_smithy_runtime_api::client::orchestrator::HttpRequest,
+        ::aws_smithy_runtime_api::box_error::BoxError,
+    > {
         let input = input
             .downcast::<crate::operation::assume_role_with_saml::AssumeRoleWithSamlInput>()
             .expect("correct type");
@@ -234,22 +292,32 @@
             fn update_http_builder(
                 input: &crate::operation::assume_role_with_saml::AssumeRoleWithSamlInput,
                 builder: ::http_1x::request::Builder,
-            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError>
+            {
                 let mut uri = ::std::string::String::new();
                 uri_base(input, &mut uri)?;
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(
+                builder,
+                ::http_1x::header::CONTENT_TYPE,
+                "application/x-www-form-urlencoded",
+            );
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_assume_role_with_saml_input::ser_assume_role_with_saml_input_input_input(&input)?,
+            crate::protocol_serde::shape_assume_role_with_saml_input::ser_assume_role_with_saml_input_input_input(
+                &input,
+            )?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
-            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+            request_builder = _header_serialization_settings.set_default_header(
+                request_builder,
+                ::http_1x::header::CONTENT_LENGTH,
+                &content_length,
+            );
         }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
@@ -282,13 +350,21 @@
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
-            .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
+            .set_endpoint(
+                cfg.load::<::aws_types::endpoint_config::EndpointUrl>()
+                    .map(|ty| ty.0.clone()),
+            )
             .build()
             .map_err(|err| {
-                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new("endpoint params could not be built", err)
+                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new(
+                    "endpoint params could not be built",
+                    err,
+                )
             })?;
         cfg.interceptor_state()
-            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params));
+            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(
+                params,
+            ));
         ::std::result::Result::Ok(())
     }
 }
@@ -315,18 +391,22 @@
     /// <p>STS is not activated in the requested region for the account that is being asked to generate credentials. The account administrator must use the IAM console to activate STS in that region. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_enable-regions.html#sts-regions-activate-deactivate">Activating and Deactivating STS in an Amazon Web Services Region</a> in the <i>IAM User Guide</i>.</p>
     RegionDisabledException(crate::types::error::RegionDisabledException),
     /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
-    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
+    #[deprecated(
+        note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
     variable wildcard pattern and check `.code()`:
      \
     &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
      \
-    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-AssumeRoleWithSAMLError) for what information is available for the error.")]
+    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-AssumeRoleWithSAMLError) for what information is available for the error."
+    )]
     Unhandled(crate::error::sealed_unhandled::Unhandled),
 }
 impl AssumeRoleWithSAMLError {
     /// Creates the `AssumeRoleWithSAMLError::Unhandled` variant from any error type.
     pub fn unhandled(
-        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
+        err: impl ::std::convert::Into<
+            ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
+        >,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
             source: err.into(),
@@ -349,9 +429,15 @@
         match self {
             Self::ExpiredTokenException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
             Self::IdpRejectedClaimException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::InvalidIdentityTokenException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::MalformedPolicyDocumentException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::PackedPolicyTooLargeException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::InvalidIdentityTokenException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
+            Self::MalformedPolicyDocumentException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
+            Self::PackedPolicyTooLargeException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
             Self::RegionDisabledException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
             Self::Unhandled(e) => &e.meta,
         }
@@ -404,7 +490,9 @@
             Self::PackedPolicyTooLargeException(_inner) => _inner.fmt(f),
             Self::RegionDisabledException(_inner) => _inner.fmt(f),
             Self::Unhandled(_inner) => {
-                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
+                if let ::std::option::Option::Some(code) =
+                    ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
+                {
                     write!(f, "unhandled error ({code})")
                 } else {
                     f.write_str("unhandled error")
@@ -424,12 +512,24 @@
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for AssumeRoleWithSAMLError {
     fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::ExpiredTokenException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::IdpRejectedClaimException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::InvalidIdentityTokenException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::MalformedPolicyDocumentException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::PackedPolicyTooLargeException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::RegionDisabledException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::ExpiredTokenException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::IdpRejectedClaimException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::InvalidIdentityTokenException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::MalformedPolicyDocumentException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::PackedPolicyTooLargeException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::RegionDisabledException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
             Self::Unhandled(_inner) => &_inner.meta,
         }
     }
```

### `src/operation/assume_role_with_saml/_assume_role_with_saml_input.rs`

```diff
diff --git a/src/operation/assume_role_with_saml/_assume_role_with_saml_input.rs b/src/operation/assume_role_with_saml/_assume_role_with_saml_input.rs
--- a/src/operation/assume_role_with_saml/_assume_role_with_saml_input.rs
+++ b/src/operation/assume_role_with_saml/_assume_role_with_saml_input.rs
@@ -169,7 +169,10 @@
     /// <p>An Amazon Web Services conversion compresses the passed inline session policy, managed policy ARNs, and session tags into a packed binary format that has a separate limit. Your request can fail for this limit even if your plaintext meets the other requirements. The <code>PackedPolicySize</code> response element indicates by percentage how close the policies and tags for your request are to the upper size limit.</p>
     /// </note>
     /// <p>Passing policies to this operation returns new temporary credentials. The resulting session's permissions are the intersection of the role's identity-based policy and the session policies. You can use the role's temporary credentials in subsequent Amazon Web Services API calls to access resources in the account that owns the role. You cannot use session policies to grant more permissions than those allowed by the identity-based policy of the role that is being assumed. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#policies_session">Session Policies</a> in the <i>IAM User Guide</i>.</p>
-    pub fn set_policy_arns(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::PolicyDescriptorType>>) -> Self {
+    pub fn set_policy_arns(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<crate::types::PolicyDescriptorType>>,
+    ) -> Self {
         self.policy_arns = input;
         self
     }
@@ -236,8 +239,10 @@
     /// Consumes the builder and constructs a [`AssumeRoleWithSamlInput`](crate::operation::assume_role_with_saml::AssumeRoleWithSamlInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<crate::operation::assume_role_with_saml::AssumeRoleWithSamlInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        crate::operation::assume_role_with_saml::AssumeRoleWithSamlInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(crate::operation::assume_role_with_saml::AssumeRoleWithSamlInput {
             role_arn: self.role_arn,
             principal_arn: self.principal_arn,
```

### `src/operation/assume_role_with_saml/builders.rs`

```diff
diff --git a/src/operation/assume_role_with_saml/builders.rs b/src/operation/assume_role_with_saml/builders.rs
--- a/src/operation/assume_role_with_saml/builders.rs
+++ b/src/operation/assume_role_with_saml/builders.rs
@@ -133,12 +133,18 @@
     > {
         crate::client::customize::CustomizableOperation::new(self)
     }
-    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
+    pub(crate) fn config_override(
+        mut self,
+        config_override: impl ::std::convert::Into<crate::config::Builder>,
+    ) -> Self {
         self.set_config_override(::std::option::Option::Some(config_override.into()));
         self
     }

-    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
+    pub(crate) fn set_config_override(
+        &mut self,
+        config_override: ::std::option::Option<crate::config::Builder>,
+    ) -> &mut Self {
         self.config_override = config_override;
         self
     }
@@ -206,7 +212,10 @@
     /// <p>An Amazon Web Services conversion compresses the passed inline session policy, managed policy ARNs, and session tags into a packed binary format that has a separate limit. Your request can fail for this limit even if your plaintext meets the other requirements. The <code>PackedPolicySize</code> response element indicates by percentage how close the policies and tags for your request are to the upper size limit.</p>
     /// </note>
     /// <p>Passing policies to this operation returns new temporary credentials. The resulting session's permissions are the intersection of the role's identity-based policy and the session policies. You can use the role's temporary credentials in subsequent Amazon Web Services API calls to access resources in the account that owns the role. You cannot use session policies to grant more permissions than those allowed by the identity-based policy of the role that is being assumed. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#policies_session">Session Policies</a> in the <i>IAM User Guide</i>.</p>
-    pub fn set_policy_arns(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::PolicyDescriptorType>>) -> Self {
+    pub fn set_policy_arns(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<crate::types::PolicyDescriptorType>>,
+    ) -> Self {
         self.inner = self.inner.set_policy_arns(input);
         self
     }
```

### `src/operation/assume_role_with_web_identity.rs`

```diff
diff --git a/src/operation/assume_role_with_web_identity.rs b/src/operation/assume_role_with_web_identity.rs
--- a/src/operation/assume_role_with_web_identity.rs
+++ b/src/operation/assume_role_with_web_identity.rs
@@ -27,9 +27,13 @@
                     .expect("correct error type")
             })
         };
-        let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
-            .await
-            .map_err(map_err)?;
+        let context = Self::orchestrate_with_stop_point(
+            runtime_plugins,
+            input,
+            ::aws_smithy_runtime::client::orchestrator::StopPoint::None,
+        )
+        .await
+        .map_err(map_err)?;
         let output = context.finalize().map_err(map_err)?;
         ::std::result::Result::Ok(
             output
@@ -51,17 +55,23 @@
     > {
         let input = ::aws_smithy_runtime_api::client::interceptors::context::Input::erase(input);
         use ::tracing::Instrument;
-        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("STS", "AssumeRoleWithWebIdentity", input, runtime_plugins, stop_point)
-            // Create a parent span for the entire operation. Includes a random, internal-only,
-            // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
-            .instrument(::tracing::debug_span!(
-                "STS.AssumeRoleWithWebIdentity",
-                "rpc.service" = "STS",
-                "rpc.method" = "AssumeRoleWithWebIdentity",
-                "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
-                "rpc.system" = "aws-api",
-            ))
-            .await
+        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point(
+            "STS",
+            "AssumeRoleWithWebIdentity",
+            input,
+            runtime_plugins,
+            stop_point,
+        )
+        // Create a parent span for the entire operation. Includes a random, internal-only,
+        // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
+        .instrument(::tracing::debug_span!(
+            "STS.AssumeRoleWithWebIdentity",
+            "rpc.service" = "STS",
+            "rpc.method" = "AssumeRoleWithWebIdentity",
+            "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
+            "rpc.system" = "aws-api",
+        ))
+        .await
     }

     pub(crate) fn operation_runtime_plugins(
@@ -91,16 +101,20 @@
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
             AssumeRoleWithWebIdentityRequestSerializer,
         ));
-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            AssumeRoleWithWebIdentityResponseDeserializer,
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
+                AssumeRoleWithWebIdentityResponseDeserializer,
+            ),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
-            crate::config::auth::Params::builder()
-                .operation_name("AssumeRoleWithWebIdentity")
-                .build()
-                .expect("required fields set"),
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
+                crate::config::auth::Params::builder()
+                    .operation_name("AssumeRoleWithWebIdentity")
+                    .build()
+                    .expect("required fields set"),
+            ),
+        );

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::SensitiveOutput);
         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
@@ -116,33 +130,46 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("AssumeRoleWithWebIdentity")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new(
+            "AssumeRoleWithWebIdentity",
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 AssumeRoleWithWebIdentityTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+            ),
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+            ),
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 AssumeRoleWithWebIdentityEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+            ),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError,
-            >::new())
-            .with_retry_classifier(
-                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                    crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError,
-                >::builder()
-                .transient_errors({
-                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
-                    transient_errors.push("IDPCommunicationError");
-                    ::std::borrow::Cow::Owned(transient_errors)
-                })
-                .build(),
-            );
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError,
+            >::builder()
+            .transient_errors({
+                let mut transient_errors: Vec<&'static str> =
+                    ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                transient_errors.push("IDPCommunicationError");
+                ::std::borrow::Cow::Owned(transient_errors)
+            })
+            .build(),
+        );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -152,7 +179,9 @@
 struct AssumeRoleWithWebIdentityTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for AssumeRoleWithWebIdentityTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept
+    for AssumeRoleWithWebIdentityTelemetryInputCaptureInterceptor
+{
     fn name(&self) -> &'static str {
         "AssumeRoleWithWebIdentityTelemetryInputCaptureInterceptor"
     }
@@ -175,7 +204,8 @@
             return ::std::result::Result::Ok(());
         };

-        let ::std::option::Option::Some(input) = context.input().downcast_ref::<AssumeRoleWithWebIdentityInput>() else {
+        let ::std::option::Option::Some(input) = context.input().downcast_ref::<AssumeRoleWithWebIdentityInput>()
+        else {
             // A mismatched input is not this interceptor's concern; skip quietly.
             return ::std::result::Result::Ok(());
         };
@@ -221,9 +251,13 @@
         let mut force_error = false;
         ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));
         let parse_result = if !success && status != 200 || force_error {
-            crate::protocol_serde::shape_assume_role_with_web_identity::de_assume_role_with_web_identity_http_error(status, headers, body)
+            crate::protocol_serde::shape_assume_role_with_web_identity::de_assume_role_with_web_identity_http_error(
+                status, headers, body,
+            )
         } else {
-            crate::protocol_serde::shape_assume_role_with_web_identity::de_assume_role_with_web_identity_http_response(status, headers, body)
+            crate::protocol_serde::shape_assume_role_with_web_identity::de_assume_role_with_web_identity_http_response(
+                status, headers, body,
+            )
         };
         crate::protocol_serde::type_erase_result(parse_result)
     }
@@ -231,12 +265,20 @@
 #[derive(Debug)]
 struct AssumeRoleWithWebIdentityRequestSerializer;
 impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for AssumeRoleWithWebIdentityRequestSerializer {
-    #[allow(unused_mut, clippy::let_and_return, clippy::needless_borrow, clippy::useless_conversion)]
+    #[allow(
+        unused_mut,
+        clippy::let_and_return,
+        clippy::needless_borrow,
+        clippy::useless_conversion
+    )]
     fn serialize_input(
         &self,
         input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
         _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
-    ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
+    ) -> ::std::result::Result<
+        ::aws_smithy_runtime_api::client::orchestrator::HttpRequest,
+        ::aws_smithy_runtime_api::box_error::BoxError,
+    > {
         let input = input
             .downcast::<crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityInput>()
             .expect("correct type");
@@ -258,14 +300,18 @@
             fn update_http_builder(
                 input: &crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityInput,
                 builder: ::http_1x::request::Builder,
-            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError>
+            {
                 let mut uri = ::std::string::String::new();
                 uri_base(input, &mut uri)?;
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(
+                builder,
+                ::http_1x::header::CONTENT_TYPE,
+                "application/x-www-form-urlencoded",
+            );
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
@@ -273,7 +319,11 @@
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
-            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+            request_builder = _header_serialization_settings.set_default_header(
+                request_builder,
+                ::http_1x::header::CONTENT_LENGTH,
+                &content_length,
+            );
         }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
@@ -306,13 +356,21 @@
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
-            .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
+            .set_endpoint(
+                cfg.load::<::aws_types::endpoint_config::EndpointUrl>()
+                    .map(|ty| ty.0.clone()),
+            )
             .build()
             .map_err(|err| {
-                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new("endpoint params could not be built", err)
+                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new(
+                    "endpoint params could not be built",
+                    err,
+                )
             })?;
         cfg.interceptor_state()
-            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params));
+            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(
+                params,
+            ));
         ::std::result::Result::Ok(())
     }
 }
@@ -341,18 +399,22 @@
     /// <p>STS is not activated in the requested region for the account that is being asked to generate credentials. The account administrator must use the IAM console to activate STS in that region. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_enable-regions.html#sts-regions-activate-deactivate">Activating and Deactivating STS in an Amazon Web Services Region</a> in the <i>IAM User Guide</i>.</p>
     RegionDisabledException(crate::types::error::RegionDisabledException),
     /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
-    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
+    #[deprecated(
+        note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
     variable wildcard pattern and check `.code()`:
      \
     &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
      \
-    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-AssumeRoleWithWebIdentityError) for what information is available for the error.")]
+    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-AssumeRoleWithWebIdentityError) for what information is available for the error."
+    )]
     Unhandled(crate::error::sealed_unhandled::Unhandled),
 }
 impl AssumeRoleWithWebIdentityError {
     /// Creates the `AssumeRoleWithWebIdentityError::Unhandled` variant from any error type.
     pub fn unhandled(
-        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
+        err: impl ::std::convert::Into<
+            ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
+        >,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
             source: err.into(),
@@ -374,11 +436,19 @@
     pub fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
             Self::ExpiredTokenException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::IdpCommunicationErrorException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::IdpCommunicationErrorException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
             Self::IdpRejectedClaimException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::InvalidIdentityTokenException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::MalformedPolicyDocumentException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::PackedPolicyTooLargeException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::InvalidIdentityTokenException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
+            Self::MalformedPolicyDocumentException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
+            Self::PackedPolicyTooLargeException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
             Self::RegionDisabledException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
             Self::Unhandled(e) => &e.meta,
         }
@@ -437,7 +507,9 @@
             Self::PackedPolicyTooLargeException(_inner) => _inner.fmt(f),
             Self::RegionDisabledException(_inner) => _inner.fmt(f),
             Self::Unhandled(_inner) => {
-                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
+                if let ::std::option::Option::Some(code) =
+                    ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
+                {
                     write!(f, "unhandled error ({code})")
                 } else {
                     f.write_str("unhandled error")
@@ -460,13 +532,27 @@
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for AssumeRoleWithWebIdentityError {
     fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::ExpiredTokenException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::IdpCommunicationErrorException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::IdpRejectedClaimException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::InvalidIdentityTokenException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::MalformedPolicyDocumentException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::PackedPolicyTooLargeException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::RegionDisabledException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::ExpiredTokenException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::IdpCommunicationErrorException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::IdpRejectedClaimException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::InvalidIdentityTokenException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::MalformedPolicyDocumentException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::PackedPolicyTooLargeException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::RegionDisabledException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
             Self::Unhandled(_inner) => &_inner.meta,
         }
     }
@@ -482,7 +568,9 @@
         })
     }
 }
-impl ::aws_types::request_id::RequestId for crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError {
+impl ::aws_types::request_id::RequestId
+    for crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError
+{
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
     }
```

### `src/operation/assume_role_with_web_identity/_assume_role_with_web_identity_input.rs`

```diff
diff --git a/src/operation/assume_role_with_web_identity/_assume_role_with_web_identity_input.rs b/src/operation/assume_role_with_web_identity/_assume_role_with_web_identity_input.rs
--- a/src/operation/assume_role_with_web_identity/_assume_role_with_web_identity_input.rs
+++ b/src/operation/assume_role_with_web_identity/_assume_role_with_web_identity_input.rs
@@ -101,7 +101,8 @@
 }
 impl AssumeRoleWithWebIdentityInput {
     /// Creates a new builder-style object to manufacture [`AssumeRoleWithWebIdentityInput`](crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityInput).
-    pub fn builder() -> crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityInputBuilder {
+    pub fn builder() -> crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityInputBuilder
+    {
         crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityInputBuilder::default()
     }
 }
@@ -216,7 +217,10 @@
     /// <p>An Amazon Web Services conversion compresses the passed inline session policy, managed policy ARNs, and session tags into a packed binary format that has a separate limit. Your request can fail for this limit even if your plaintext meets the other requirements. The <code>PackedPolicySize</code> response element indicates by percentage how close the policies and tags for your request are to the upper size limit.</p>
     /// </note>
     /// <p>Passing policies to this operation returns new temporary credentials. The resulting session's permissions are the intersection of the role's identity-based policy and the session policies. You can use the role's temporary credentials in subsequent Amazon Web Services API calls to access resources in the account that owns the role. You cannot use session policies to grant more permissions than those allowed by the identity-based policy of the role that is being assumed. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#policies_session">Session Policies</a> in the <i>IAM User Guide</i>.</p>
-    pub fn set_policy_arns(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::PolicyDescriptorType>>) -> Self {
+    pub fn set_policy_arns(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<crate::types::PolicyDescriptorType>>,
+    ) -> Self {
         self.policy_arns = input;
         self
     }
@@ -287,15 +291,17 @@
         crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityInput {
-            role_arn: self.role_arn,
-            role_session_name: self.role_session_name,
-            web_identity_token: self.web_identity_token,
-            provider_id: self.provider_id,
-            policy_arns: self.policy_arns,
-            policy: self.policy,
-            duration_seconds: self.duration_seconds,
-        })
+        ::std::result::Result::Ok(
+            crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityInput {
+                role_arn: self.role_arn,
+                role_session_name: self.role_session_name,
+                web_identity_token: self.web_identity_token,
+                provider_id: self.provider_id,
+                policy_arns: self.policy_arns,
+                policy: self.policy,
+                duration_seconds: self.duration_seconds,
+            },
+        )
     }
 }
 impl ::std::fmt::Debug for AssumeRoleWithWebIdentityInputBuilder {
```

### `src/operation/assume_role_with_web_identity/_assume_role_with_web_identity_output.rs`

```diff
diff --git a/src/operation/assume_role_with_web_identity/_assume_role_with_web_identity_output.rs b/src/operation/assume_role_with_web_identity/_assume_role_with_web_identity_output.rs
--- a/src/operation/assume_role_with_web_identity/_assume_role_with_web_identity_output.rs
+++ b/src/operation/assume_role_with_web_identity/_assume_role_with_web_identity_output.rs
@@ -79,7 +79,8 @@
 }
 impl AssumeRoleWithWebIdentityOutput {
     /// Creates a new builder-style object to manufacture [`AssumeRoleWithWebIdentityOutput`](crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityOutput).
-    pub fn builder() -> crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityOutputBuilder {
+    pub fn builder() -> crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityOutputBuilder
+    {
         crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityOutputBuilder::default()
     }
 }
```

### `src/operation/assume_role_with_web_identity/builders.rs`

```diff
diff --git a/src/operation/assume_role_with_web_identity/builders.rs b/src/operation/assume_role_with_web_identity/builders.rs
--- a/src/operation/assume_role_with_web_identity/builders.rs
+++ b/src/operation/assume_role_with_web_identity/builders.rs
@@ -86,7 +86,9 @@
         }
     }
     /// Access the AssumeRoleWithWebIdentity as a reference.
-    pub fn as_input(&self) -> &crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityInputBuilder {
+    pub fn as_input(
+        &self,
+    ) -> &crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityInputBuilder {
         &self.inner
     }
     /// Sends the request and returns the response.
@@ -110,12 +112,14 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentity::operation_runtime_plugins(
-            self.handle.runtime_plugins.clone(),
-            &self.handle.conf,
-            self.config_override,
-        );
-        crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentity::orchestrate(&runtime_plugins, input).await
+        let runtime_plugins =
+            crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentity::operation_runtime_plugins(
+                self.handle.runtime_plugins.clone(),
+                &self.handle.conf,
+                self.config_override,
+            );
+        crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentity::orchestrate(&runtime_plugins, input)
+            .await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -128,12 +132,18 @@
     > {
         crate::client::customize::CustomizableOperation::new(self)
     }
-    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
+    pub(crate) fn config_override(
+        mut self,
+        config_override: impl ::std::convert::Into<crate::config::Builder>,
+    ) -> Self {
         self.set_config_override(::std::option::Option::Some(config_override.into()));
         self
     }

-    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
+    pub(crate) fn set_config_override(
+        &mut self,
+        config_override: ::std::option::Option<crate::config::Builder>,
+    ) -> &mut Self {
         self.config_override = config_override;
         self
     }
@@ -230,7 +240,10 @@
     /// <p>An Amazon Web Services conversion compresses the passed inline session policy, managed policy ARNs, and session tags into a packed binary format that has a separate limit. Your request can fail for this limit even if your plaintext meets the other requirements. The <code>PackedPolicySize</code> response element indicates by percentage how close the policies and tags for your request are to the upper size limit.</p>
     /// </note>
     /// <p>Passing policies to this operation returns new temporary credentials. The resulting session's permissions are the intersection of the role's identity-based policy and the session policies. You can use the role's temporary credentials in subsequent Amazon Web Services API calls to access resources in the account that owns the role. You cannot use session policies to grant more permissions than those allowed by the identity-based policy of the role that is being assumed. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#policies_session">Session Policies</a> in the <i>IAM User Guide</i>.</p>
-    pub fn set_policy_arns(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::PolicyDescriptorType>>) -> Self {
+    pub fn set_policy_arns(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<crate::types::PolicyDescriptorType>>,
+    ) -> Self {
         self.inner = self.inner.set_policy_arns(input);
         self
     }
```

### `src/operation/assume_root.rs`

```diff
diff --git a/src/operation/assume_root.rs b/src/operation/assume_root.rs
--- a/src/operation/assume_root.rs
+++ b/src/operation/assume_root.rs
@@ -27,9 +27,13 @@
                     .expect("correct error type")
             })
         };
-        let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
-            .await
-            .map_err(map_err)?;
+        let context = Self::orchestrate_with_stop_point(
+            runtime_plugins,
+            input,
+            ::aws_smithy_runtime::client::orchestrator::StopPoint::None,
+        )
+        .await
+        .map_err(map_err)?;
         let output = context.finalize().map_err(map_err)?;
         ::std::result::Result::Ok(
             output
@@ -51,17 +55,23 @@
     > {
         let input = ::aws_smithy_runtime_api::client::interceptors::context::Input::erase(input);
         use ::tracing::Instrument;
-        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("STS", "AssumeRoot", input, runtime_plugins, stop_point)
-            // Create a parent span for the entire operation. Includes a random, internal-only,
-            // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
-            .instrument(::tracing::debug_span!(
-                "STS.AssumeRoot",
-                "rpc.service" = "STS",
-                "rpc.method" = "AssumeRoot",
-                "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
-                "rpc.system" = "aws-api",
-            ))
-            .await
+        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point(
+            "STS",
+            "AssumeRoot",
+            input,
+            runtime_plugins,
+            stop_point,
+        )
+        // Create a parent span for the entire operation. Includes a random, internal-only,
+        // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
+        .instrument(::tracing::debug_span!(
+            "STS.AssumeRoot",
+            "rpc.service" = "STS",
+            "rpc.method" = "AssumeRoot",
+            "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
+            "rpc.system" = "aws-api",
+        ))
+        .await
     }

     pub(crate) fn operation_runtime_plugins(
@@ -91,19 +101,24 @@
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
             AssumeRootRequestSerializer,
         ));
-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            AssumeRootResponseDeserializer,
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(AssumeRootResponseDeserializer),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
-            crate::config::auth::Params::builder()
-                .operation_name("AssumeRoot")
-                .build()
-                .expect("required fields set"),
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
+                crate::config::auth::Params::builder()
+                    .operation_name("AssumeRoot")
+                    .build()
+                    .expect("required fields set"),
+            ),
+        );

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::SensitiveOutput);
-        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("AssumeRoot", "STS"));
+        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
+            "AssumeRoot",
+            "STS",
+        ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
         signing_options.double_uri_encode = true;
         signing_options.content_sha256_header = false;
@@ -221,13 +236,23 @@
 #[derive(Debug)]
 struct AssumeRootRequestSerializer;
 impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for AssumeRootRequestSerializer {
-    #[allow(unused_mut, clippy::let_and_return, clippy::needless_borrow, clippy::useless_conversion)]
+    #[allow(
+        unused_mut,
+        clippy::let_and_return,
+        clippy::needless_borrow,
+        clippy::useless_conversion
+    )]
     fn serialize_input(
         &self,
         input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
         _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
-    ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
-        let input = input.downcast::<crate::operation::assume_root::AssumeRootInput>().expect("correct type");
+    ) -> ::std::result::Result<
+        ::aws_smithy_runtime_api::client::orchestrator::HttpRequest,
+        ::aws_smithy_runtime_api::box_error::BoxError,
+    > {
+        let input = input
+            .downcast::<crate::operation::assume_root::AssumeRootInput>()
+            .expect("correct type");
         let _header_serialization_settings = _cfg
             .load::<crate::serialization_settings::HeaderSerializationSettings>()
             .cloned()
@@ -246,21 +271,30 @@
             fn update_http_builder(
                 input: &crate::operation::assume_root::AssumeRootInput,
                 builder: ::http_1x::request::Builder,
-            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError>
+            {
                 let mut uri = ::std::string::String::new();
                 uri_base(input, &mut uri)?;
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(
+                builder,
+                ::http_1x::header::CONTENT_TYPE,
+                "application/x-www-form-urlencoded",
+            );
             builder
         };
-        let body =
-            ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_assume_root_input::ser_assume_root_input_input_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            crate::protocol_serde::shape_assume_root_input::ser_assume_root_input_input_input(&input)?,
+        );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
-            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+            request_builder = _header_serialization_settings.set_default_header(
+                request_builder,
+                ::http_1x::header::CONTENT_LENGTH,
+                &content_length,
+            );
         }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
@@ -293,13 +327,21 @@
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
-            .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
+            .set_endpoint(
+                cfg.load::<::aws_types::endpoint_config::EndpointUrl>()
+                    .map(|ty| ty.0.clone()),
+            )
             .build()
             .map_err(|err| {
-                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new("endpoint params could not be built", err)
+                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new(
+                    "endpoint params could not be built",
+                    err,
+                )
             })?;
         cfg.interceptor_state()
-            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params));
+            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(
+                params,
+            ));
         ::std::result::Result::Ok(())
     }
 }
@@ -316,18 +358,22 @@
     /// <p>STS is not activated in the requested region for the account that is being asked to generate credentials. The account administrator must use the IAM console to activate STS in that region. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_enable-regions.html#sts-regions-activate-deactivate">Activating and Deactivating STS in an Amazon Web Services Region</a> in the <i>IAM User Guide</i>.</p>
     RegionDisabledException(crate::types::error::RegionDisabledException),
     /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
-    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
+    #[deprecated(
+        note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
     variable wildcard pattern and check `.code()`:
      \
     &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
      \
-    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-AssumeRootError) for what information is available for the error.")]
+    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-AssumeRootError) for what information is available for the error."
+    )]
     Unhandled(crate::error::sealed_unhandled::Unhandled),
 }
 impl AssumeRootError {
     /// Creates the `AssumeRootError::Unhandled` variant from any error type.
     pub fn unhandled(
-        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
+        err: impl ::std::convert::Into<
+            ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
+        >,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
             source: err.into(),
@@ -377,7 +423,9 @@
             Self::ExpiredTokenException(_inner) => _inner.fmt(f),
             Self::RegionDisabledException(_inner) => _inner.fmt(f),
             Self::Unhandled(_inner) => {
-                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
+                if let ::std::option::Option::Some(code) =
+                    ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
+                {
                     write!(f, "unhandled error ({code})")
                 } else {
                     f.write_str("unhandled error")
@@ -397,8 +445,12 @@
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for AssumeRootError {
     fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::ExpiredTokenException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::RegionDisabledException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::ExpiredTokenException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::RegionDisabledException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
             Self::Unhandled(_inner) => &_inner.meta,
         }
     }
```

### `src/operation/assume_root/_assume_root_input.rs`

```diff
diff --git a/src/operation/assume_root/_assume_root_input.rs b/src/operation/assume_root/_assume_root_input.rs
--- a/src/operation/assume_root/_assume_root_input.rs
+++ b/src/operation/assume_root/_assume_root_input.rs
@@ -150,7 +150,12 @@
         &self.duration_seconds
     }
     /// Consumes the builder and constructs a [`AssumeRootInput`](crate::operation::assume_root::AssumeRootInput).
-    pub fn build(self) -> ::std::result::Result<crate::operation::assume_root::AssumeRootInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<
+        crate::operation::assume_root::AssumeRootInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(crate::operation::assume_root::AssumeRootInput {
             target_principal: self.target_principal,
             task_policy_arn: self.task_policy_arn,
```

### `src/operation/assume_root/builders.rs`

```diff
diff --git a/src/operation/assume_root/builders.rs b/src/operation/assume_root/builders.rs
--- a/src/operation/assume_root/builders.rs
+++ b/src/operation/assume_root/builders.rs
@@ -104,12 +104,18 @@
     > {
         crate::client::customize::CustomizableOperation::new(self)
     }
-    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
+    pub(crate) fn config_override(
+        mut self,
+        config_override: impl ::std::convert::Into<crate::config::Builder>,
+    ) -> Self {
         self.set_config_override(::std::option::Option::Some(config_override.into()));
         self
     }

-    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
+    pub(crate) fn set_config_override(
+        &mut self,
+        config_override: ::std::option::Option<crate::config::Builder>,
+    ) -> &mut Self {
         self.config_override = config_override;
         self
     }
```

### `src/operation/decode_authorization_message.rs`

```diff
diff --git a/src/operation/decode_authorization_message.rs b/src/operation/decode_authorization_message.rs
--- a/src/operation/decode_authorization_message.rs
+++ b/src/operation/decode_authorization_message.rs
@@ -27,9 +27,13 @@
                     .expect("correct error type")
             })
         };
-        let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
-            .await
-            .map_err(map_err)?;
+        let context = Self::orchestrate_with_stop_point(
+            runtime_plugins,
+            input,
+            ::aws_smithy_runtime::client::orchestrator::StopPoint::None,
+        )
+        .await
+        .map_err(map_err)?;
         let output = context.finalize().map_err(map_err)?;
         ::std::result::Result::Ok(
             output
@@ -51,17 +55,23 @@
     > {
         let input = ::aws_smithy_runtime_api::client::interceptors::context::Input::erase(input);
         use ::tracing::Instrument;
-        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("STS", "DecodeAuthorizationMessage", input, runtime_plugins, stop_point)
-            // Create a parent span for the entire operation. Includes a random, internal-only,
-            // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
-            .instrument(::tracing::debug_span!(
-                "STS.DecodeAuthorizationMessage",
-                "rpc.service" = "STS",
-                "rpc.method" = "DecodeAuthorizationMessage",
-                "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
-                "rpc.system" = "aws-api",
-            ))
-            .await
+        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point(
+            "STS",
+            "DecodeAuthorizationMessage",
+            input,
+            runtime_plugins,
+            stop_point,
+        )
+        // Create a parent span for the entire operation. Includes a random, internal-only,
+        // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
+        .instrument(::tracing::debug_span!(
+            "STS.DecodeAuthorizationMessage",
+            "rpc.service" = "STS",
+            "rpc.method" = "DecodeAuthorizationMessage",
+            "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
+            "rpc.system" = "aws-api",
+        ))
+        .await
     }

     pub(crate) fn operation_runtime_plugins(
@@ -91,16 +101,20 @@
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
             DecodeAuthorizationMessageRequestSerializer,
         ));
-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            DecodeAuthorizationMessageResponseDeserializer,
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
+                DecodeAuthorizationMessageResponseDeserializer,
+            ),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
-            crate::config::auth::Params::builder()
-                .operation_name("DecodeAuthorizationMessage")
-                .build()
-                .expect("required fields set"),
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
+                crate::config::auth::Params::builder()
+                    .operation_name("DecodeAuthorizationMessage")
+                    .build()
+                    .expect("required fields set"),
+            ),
+        );

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
             "DecodeAuthorizationMessage",
@@ -125,33 +139,46 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("DecodeAuthorizationMessage")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new(
+            "DecodeAuthorizationMessage",
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 DecodeAuthorizationMessageTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+            ),
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+            ),
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 DecodeAuthorizationMessageEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+            ),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 crate::operation::decode_authorization_message::DecodeAuthorizationMessageError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+                crate::operation::decode_authorization_message::DecodeAuthorizationMessageError,
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
                 crate::operation::decode_authorization_message::DecodeAuthorizationMessageError,
-            >::new())
-            .with_retry_classifier(
-                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                    crate::operation::decode_authorization_message::DecodeAuthorizationMessageError,
-                >::builder()
-                .transient_errors({
-                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
-                    transient_errors.push("IDPCommunicationError");
-                    ::std::borrow::Cow::Owned(transient_errors)
-                })
-                .build(),
-            );
+            >::builder()
+            .transient_errors({
+                let mut transient_errors: Vec<&'static str> =
+                    ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                transient_errors.push("IDPCommunicationError");
+                ::std::borrow::Cow::Owned(transient_errors)
+            })
+            .build(),
+        );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -161,7 +188,9 @@
 struct DecodeAuthorizationMessageTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for DecodeAuthorizationMessageTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept
+    for DecodeAuthorizationMessageTelemetryInputCaptureInterceptor
+{
     fn name(&self) -> &'static str {
         "DecodeAuthorizationMessageTelemetryInputCaptureInterceptor"
     }
@@ -184,7 +213,8 @@
             return ::std::result::Result::Ok(());
         };

-        let ::std::option::Option::Some(input) = context.input().downcast_ref::<DecodeAuthorizationMessageInput>() else {
+        let ::std::option::Option::Some(input) = context.input().downcast_ref::<DecodeAuthorizationMessageInput>()
+        else {
             // A mismatched input is not this interceptor's concern; skip quietly.
             return ::std::result::Result::Ok(());
         };
@@ -215,9 +245,13 @@
         let mut force_error = false;
         ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));
         let parse_result = if !success && status != 200 || force_error {
-            crate::protocol_serde::shape_decode_authorization_message::de_decode_authorization_message_http_error(status, headers, body)
+            crate::protocol_serde::shape_decode_authorization_message::de_decode_authorization_message_http_error(
+                status, headers, body,
+            )
         } else {
-            crate::protocol_serde::shape_decode_authorization_message::de_decode_authorization_message_http_response(status, headers, body)
+            crate::protocol_serde::shape_decode_authorization_message::de_decode_authorization_message_http_response(
+                status, headers, body,
+            )
         };
         crate::protocol_serde::type_erase_result(parse_result)
     }
@@ -225,12 +259,20 @@
 #[derive(Debug)]
 struct DecodeAuthorizationMessageRequestSerializer;
 impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for DecodeAuthorizationMessageRequestSerializer {
-    #[allow(unused_mut, clippy::let_and_return, clippy::needless_borrow, clippy::useless_conversion)]
+    #[allow(
+        unused_mut,
+        clippy::let_and_return,
+        clippy::needless_borrow,
+        clippy::useless_conversion
+    )]
     fn serialize_input(
         &self,
         input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
         _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
-    ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
+    ) -> ::std::result::Result<
+        ::aws_smithy_runtime_api::client::orchestrator::HttpRequest,
+        ::aws_smithy_runtime_api::box_error::BoxError,
+    > {
         let input = input
             .downcast::<crate::operation::decode_authorization_message::DecodeAuthorizationMessageInput>()
             .expect("correct type");
@@ -252,14 +294,18 @@
             fn update_http_builder(
                 input: &crate::operation::decode_authorization_message::DecodeAuthorizationMessageInput,
                 builder: ::http_1x::request::Builder,
-            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError>
+            {
                 let mut uri = ::std::string::String::new();
                 uri_base(input, &mut uri)?;
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(
+                builder,
+                ::http_1x::header::CONTENT_TYPE,
+                "application/x-www-form-urlencoded",
+            );
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
@@ -267,7 +313,11 @@
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
-            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+            request_builder = _header_serialization_settings.set_default_header(
+                request_builder,
+                ::http_1x::header::CONTENT_LENGTH,
+                &content_length,
+            );
         }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
@@ -300,13 +350,21 @@
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
-            .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
+            .set_endpoint(
+                cfg.load::<::aws_types::endpoint_config::EndpointUrl>()
+                    .map(|ty| ty.0.clone()),
+            )
             .build()
             .map_err(|err| {
-                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new("endpoint params could not be built", err)
+                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new(
+                    "endpoint params could not be built",
+                    err,
+                )
             })?;
         cfg.interceptor_state()
-            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params));
+            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(
+                params,
+            ));
         ::std::result::Result::Ok(())
     }
 }
@@ -321,18 +379,22 @@
     /// <p>The error returned if the message passed to <code>DecodeAuthorizationMessage</code> was invalid. This can happen if the token contains invalid characters, such as line breaks, or if the message has expired.</p>
     InvalidAuthorizationMessageException(crate::types::error::InvalidAuthorizationMessageException),
     /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
-    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
+    #[deprecated(
+        note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
     variable wildcard pattern and check `.code()`:
      \
     &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
      \
-    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-DecodeAuthorizationMessageError) for what information is available for the error.")]
+    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-DecodeAuthorizationMessageError) for what information is available for the error."
+    )]
     Unhandled(crate::error::sealed_unhandled::Unhandled),
 }
 impl DecodeAuthorizationMessageError {
     /// Creates the `DecodeAuthorizationMessageError::Unhandled` variant from any error type.
     pub fn unhandled(
-        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
+        err: impl ::std::convert::Into<
+            ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
+        >,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
             source: err.into(),
@@ -353,7 +415,9 @@
     ///
     pub fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::InvalidAuthorizationMessageException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::InvalidAuthorizationMessageException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
             Self::Unhandled(e) => &e.meta,
         }
     }
@@ -375,7 +439,9 @@
         match self {
             Self::InvalidAuthorizationMessageException(_inner) => _inner.fmt(f),
             Self::Unhandled(_inner) => {
-                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
+                if let ::std::option::Option::Some(code) =
+                    ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
+                {
                     write!(f, "unhandled error ({code})")
                 } else {
                     f.write_str("unhandled error")
@@ -395,7 +461,9 @@
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for DecodeAuthorizationMessageError {
     fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::InvalidAuthorizationMessageException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::InvalidAuthorizationMessageException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
             Self::Unhandled(_inner) => &_inner.meta,
         }
     }
@@ -411,7 +479,9 @@
         })
     }
 }
-impl ::aws_types::request_id::RequestId for crate::operation::decode_authorization_message::DecodeAuthorizationMessageError {
+impl ::aws_types::request_id::RequestId
+    for crate::operation::decode_authorization_message::DecodeAuthorizationMessageError
+{
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
     }
```

### `src/operation/decode_authorization_message/_decode_authorization_message_input.rs`

```diff
diff --git a/src/operation/decode_authorization_message/_decode_authorization_message_input.rs b/src/operation/decode_authorization_message/_decode_authorization_message_input.rs
--- a/src/operation/decode_authorization_message/_decode_authorization_message_input.rs
+++ b/src/operation/decode_authorization_message/_decode_authorization_message_input.rs
@@ -14,7 +14,8 @@
 }
 impl DecodeAuthorizationMessageInput {
     /// Creates a new builder-style object to manufacture [`DecodeAuthorizationMessageInput`](crate::operation::decode_authorization_message::DecodeAuthorizationMessageInput).
-    pub fn builder() -> crate::operation::decode_authorization_message::builders::DecodeAuthorizationMessageInputBuilder {
+    pub fn builder() -> crate::operation::decode_authorization_message::builders::DecodeAuthorizationMessageInputBuilder
+    {
         crate::operation::decode_authorization_message::builders::DecodeAuthorizationMessageInputBuilder::default()
     }
 }
@@ -48,8 +49,10 @@
         crate::operation::decode_authorization_message::DecodeAuthorizationMessageInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(crate::operation::decode_authorization_message::DecodeAuthorizationMessageInput {
-            encoded_message: self.encoded_message,
-        })
+        ::std::result::Result::Ok(
+            crate::operation::decode_authorization_message::DecodeAuthorizationMessageInput {
+                encoded_message: self.encoded_message,
+            },
+        )
     }
 }
```

### `src/operation/decode_authorization_message/_decode_authorization_message_output.rs`

```diff
diff --git a/src/operation/decode_authorization_message/_decode_authorization_message_output.rs b/src/operation/decode_authorization_message/_decode_authorization_message_output.rs
--- a/src/operation/decode_authorization_message/_decode_authorization_message_output.rs
+++ b/src/operation/decode_authorization_message/_decode_authorization_message_output.rs
@@ -21,7 +21,8 @@
 }
 impl DecodeAuthorizationMessageOutput {
     /// Creates a new builder-style object to manufacture [`DecodeAuthorizationMessageOutput`](crate::operation::decode_authorization_message::DecodeAuthorizationMessageOutput).
-    pub fn builder() -> crate::operation::decode_authorization_message::builders::DecodeAuthorizationMessageOutputBuilder {
+    pub fn builder() -> crate::operation::decode_authorization_message::builders::DecodeAuthorizationMessageOutputBuilder
+    {
         crate::operation::decode_authorization_message::builders::DecodeAuthorizationMessageOutputBuilder::default()
     }
 }
```

### `src/operation/decode_authorization_message/builders.rs`

```diff
diff --git a/src/operation/decode_authorization_message/builders.rs b/src/operation/decode_authorization_message/builders.rs
--- a/src/operation/decode_authorization_message/builders.rs
+++ b/src/operation/decode_authorization_message/builders.rs
@@ -74,7 +74,9 @@
         }
     }
     /// Access the DecodeAuthorizationMessage as a reference.
-    pub fn as_input(&self) -> &crate::operation::decode_authorization_message::builders::DecodeAuthorizationMessageInputBuilder {
+    pub fn as_input(
+        &self,
+    ) -> &crate::operation::decode_authorization_message::builders::DecodeAuthorizationMessageInputBuilder {
         &self.inner
     }
     /// Sends the request and returns the response.
@@ -98,12 +100,14 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = crate::operation::decode_authorization_message::DecodeAuthorizationMessage::operation_runtime_plugins(
-            self.handle.runtime_plugins.clone(),
-            &self.handle.conf,
-            self.config_override,
-        );
-        crate::operation::decode_authorization_message::DecodeAuthorizationMessage::orchestrate(&runtime_plugins, input).await
+        let runtime_plugins =
+            crate::operation::decode_authorization_message::DecodeAuthorizationMessage::operation_runtime_plugins(
+                self.handle.runtime_plugins.clone(),
+                &self.handle.conf,
+                self.config_override,
+            );
+        crate::operation::decode_authorization_message::DecodeAuthorizationMessage::orchestrate(&runtime_plugins, input)
+            .await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -116,12 +120,18 @@
     > {
         crate::client::customize::CustomizableOperation::new(self)
     }
-    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
+    pub(crate) fn config_override(
+        mut self,
+        config_override: impl ::std::convert::Into<crate::config::Builder>,
+    ) -> Self {
         self.set_config_override(::std::option::Option::Some(config_override.into()));
         self
     }

-    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
+    pub(crate) fn set_config_override(
+        &mut self,
+        config_override: ::std::option::Option<crate::config::Builder>,
+    ) -> &mut Self {
         self.config_override = config_override;
         self
     }
```

### `src/operation/get_access_key_info.rs`

```diff
diff --git a/src/operation/get_access_key_info.rs b/src/operation/get_access_key_info.rs
--- a/src/operation/get_access_key_info.rs
+++ b/src/operation/get_access_key_info.rs
@@ -27,9 +27,13 @@
                     .expect("correct error type")
             })
         };
-        let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
-            .await
-            .map_err(map_err)?;
+        let context = Self::orchestrate_with_stop_point(
+            runtime_plugins,
+            input,
+            ::aws_smithy_runtime::client::orchestrator::StopPoint::None,
+        )
+        .await
+        .map_err(map_err)?;
         let output = context.finalize().map_err(map_err)?;
         ::std::result::Result::Ok(
             output
@@ -51,17 +55,23 @@
     > {
         let input = ::aws_smithy_runtime_api::client::interceptors::context::Input::erase(input);
         use ::tracing::Instrument;
-        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("STS", "GetAccessKeyInfo", input, runtime_plugins, stop_point)
-            // Create a parent span for the entire operation. Includes a random, internal-only,
-            // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
-            .instrument(::tracing::debug_span!(
-                "STS.GetAccessKeyInfo",
-                "rpc.service" = "STS",
-                "rpc.method" = "GetAccessKeyInfo",
-                "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
-                "rpc.system" = "aws-api",
-            ))
-            .await
+        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point(
+            "STS",
+            "GetAccessKeyInfo",
+            input,
+            runtime_plugins,
+            stop_point,
+        )
+        // Create a parent span for the entire operation. Includes a random, internal-only,
+        // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
+        .instrument(::tracing::debug_span!(
+            "STS.GetAccessKeyInfo",
+            "rpc.service" = "STS",
+            "rpc.method" = "GetAccessKeyInfo",
+            "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
+            "rpc.system" = "aws-api",
+        ))
+        .await
     }

     pub(crate) fn operation_runtime_plugins(
@@ -91,18 +101,25 @@
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
             GetAccessKeyInfoRequestSerializer,
         ));
-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            GetAccessKeyInfoResponseDeserializer,
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
+                GetAccessKeyInfoResponseDeserializer,
+            ),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
-            crate::config::auth::Params::builder()
-                .operation_name("GetAccessKeyInfo")
-                .build()
-                .expect("required fields set"),
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
+                crate::config::auth::Params::builder()
+                    .operation_name("GetAccessKeyInfo")
+                    .build()
+                    .expect("required fields set"),
+            ),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("GetAccessKeyInfo", "STS"));
+        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
+            "GetAccessKeyInfo",
+            "STS",
+        ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
         signing_options.double_uri_encode = true;
         signing_options.content_sha256_header = false;
@@ -122,32 +139,46 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetAccessKeyInfo")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new(
+            "GetAccessKeyInfo",
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 GetAccessKeyInfoTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+            ),
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+            ),
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 GetAccessKeyInfoEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+            ),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 crate::operation::get_access_key_info::GetAccessKeyInfoError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::get_access_key_info::GetAccessKeyInfoError,
-            >::new())
-            .with_retry_classifier(
-                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::get_access_key_info::GetAccessKeyInfoError>::builder(
-                )
-                .transient_errors({
-                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
-                    transient_errors.push("IDPCommunicationError");
-                    ::std::borrow::Cow::Owned(transient_errors)
-                })
-                .build(),
-            );
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                crate::operation::get_access_key_info::GetAccessKeyInfoError,
+            >::builder()
+            .transient_errors({
+                let mut transient_errors: Vec<&'static str> =
+                    ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                transient_errors.push("IDPCommunicationError");
+                ::std::borrow::Cow::Owned(transient_errors)
+            })
+            .build(),
+        );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -213,7 +244,9 @@
         let parse_result = if !success && status != 200 || force_error {
             crate::protocol_serde::shape_get_access_key_info::de_get_access_key_info_http_error(status, headers, body)
         } else {
-            crate::protocol_serde::shape_get_access_key_info::de_get_access_key_info_http_response(status, headers, body)
+            crate::protocol_serde::shape_get_access_key_info::de_get_access_key_info_http_response(
+                status, headers, body,
+            )
         };
         crate::protocol_serde::type_erase_result(parse_result)
     }
@@ -221,12 +254,20 @@
 #[derive(Debug)]
 struct GetAccessKeyInfoRequestSerializer;
 impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for GetAccessKeyInfoRequestSerializer {
-    #[allow(unused_mut, clippy::let_and_return, clippy::needless_borrow, clippy::useless_conversion)]
+    #[allow(
+        unused_mut,
+        clippy::let_and_return,
+        clippy::needless_borrow,
+        clippy::useless_conversion
+    )]
     fn serialize_input(
         &self,
         input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
         _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
-    ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
+    ) -> ::std::result::Result<
+        ::aws_smithy_runtime_api::client::orchestrator::HttpRequest,
+        ::aws_smithy_runtime_api::box_error::BoxError,
+    > {
         let input = input
             .downcast::<crate::operation::get_access_key_info::GetAccessKeyInfoInput>()
             .expect("correct type");
@@ -248,14 +289,18 @@
             fn update_http_builder(
                 input: &crate::operation::get_access_key_info::GetAccessKeyInfoInput,
                 builder: ::http_1x::request::Builder,
-            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError>
+            {
                 let mut uri = ::std::string::String::new();
                 uri_base(input, &mut uri)?;
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(
+                builder,
+                ::http_1x::header::CONTENT_TYPE,
+                "application/x-www-form-urlencoded",
+            );
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
@@ -263,7 +308,11 @@
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
-            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+            request_builder = _header_serialization_settings.set_default_header(
+                request_builder,
+                ::http_1x::header::CONTENT_LENGTH,
+                &content_length,
+            );
         }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
@@ -296,13 +345,21 @@
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
-            .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
+            .set_endpoint(
+                cfg.load::<::aws_types::endpoint_config::EndpointUrl>()
+                    .map(|ty| ty.0.clone()),
+            )
             .build()
             .map_err(|err| {
-                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new("endpoint params could not be built", err)
+                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new(
+                    "endpoint params could not be built",
+                    err,
+                )
             })?;
         cfg.interceptor_state()
-            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params));
+            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(
+                params,
+            ));
         ::std::result::Result::Ok(())
     }
 }
@@ -315,18 +372,22 @@
 #[derive(::std::fmt::Debug)]
 pub enum GetAccessKeyInfoError {
     /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
-    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
+    #[deprecated(
+        note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
     variable wildcard pattern and check `.code()`:
      \
     &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
      \
-    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-GetAccessKeyInfoError) for what information is available for the error.")]
+    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-GetAccessKeyInfoError) for what information is available for the error."
+    )]
     Unhandled(crate::error::sealed_unhandled::Unhandled),
 }
 impl GetAccessKeyInfoError {
     /// Creates the `GetAccessKeyInfoError::Unhandled` variant from any error type.
     pub fn unhandled(
-        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
+        err: impl ::std::convert::Into<
+            ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
+        >,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
             source: err.into(),
@@ -362,7 +423,9 @@
     fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
         match self {
             Self::Unhandled(_inner) => {
-                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
+                if let ::std::option::Option::Some(code) =
+                    ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
+                {
                     write!(f, "unhandled error ({code})")
                 } else {
                     f.write_str("unhandled error")
```

### `src/operation/get_access_key_info/_get_access_key_info_input.rs`

```diff
diff --git a/src/operation/get_access_key_info/_get_access_key_info_input.rs b/src/operation/get_access_key_info/_get_access_key_info_input.rs
--- a/src/operation/get_access_key_info/_get_access_key_info_input.rs
+++ b/src/operation/get_access_key_info/_get_access_key_info_input.rs
@@ -49,7 +49,10 @@
     /// Consumes the builder and constructs a [`GetAccessKeyInfoInput`](crate::operation::get_access_key_info::GetAccessKeyInfoInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<crate::operation::get_access_key_info::GetAccessKeyInfoInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        crate::operation::get_access_key_info::GetAccessKeyInfoInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(crate::operation::get_access_key_info::GetAccessKeyInfoInput {
             access_key_id: self.access_key_id,
         })
```

### `src/operation/get_access_key_info/builders.rs`

```diff
diff --git a/src/operation/get_access_key_info/builders.rs b/src/operation/get_access_key_info/builders.rs
--- a/src/operation/get_access_key_info/builders.rs
+++ b/src/operation/get_access_key_info/builders.rs
@@ -102,12 +102,18 @@
     > {
         crate::client::customize::CustomizableOperation::new(self)
     }
-    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
+    pub(crate) fn config_override(
+        mut self,
+        config_override: impl ::std::convert::Into<crate::config::Builder>,
+    ) -> Self {
         self.set_config_override(::std::option::Option::Some(config_override.into()));
         self
     }

-    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
+    pub(crate) fn set_config_override(
+        &mut self,
+        config_override: ::std::option::Option<crate::config::Builder>,
+    ) -> &mut Self {
         self.config_override = config_override;
         self
     }
```

### `src/operation/get_caller_identity.rs`

```diff
diff --git a/src/operation/get_caller_identity.rs b/src/operation/get_caller_identity.rs
--- a/src/operation/get_caller_identity.rs
+++ b/src/operation/get_caller_identity.rs
@@ -27,9 +27,13 @@
                     .expect("correct error type")
             })
         };
-        let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
-            .await
-            .map_err(map_err)?;
+        let context = Self::orchestrate_with_stop_point(
+            runtime_plugins,
+            input,
+            ::aws_smithy_runtime::client::orchestrator::StopPoint::None,
+        )
+        .await
+        .map_err(map_err)?;
         let output = context.finalize().map_err(map_err)?;
         ::std::result::Result::Ok(
             output
@@ -51,17 +55,23 @@
     > {
         let input = ::aws_smithy_runtime_api::client::interceptors::context::Input::erase(input);
         use ::tracing::Instrument;
-        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("STS", "GetCallerIdentity", input, runtime_plugins, stop_point)
-            // Create a parent span for the entire operation. Includes a random, internal-only,
-            // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
-            .instrument(::tracing::debug_span!(
-                "STS.GetCallerIdentity",
-                "rpc.service" = "STS",
-                "rpc.method" = "GetCallerIdentity",
-                "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
-                "rpc.system" = "aws-api",
-            ))
-            .await
+        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point(
+            "STS",
+            "GetCallerIdentity",
+            input,
+            runtime_plugins,
+            stop_point,
+        )
+        // Create a parent span for the entire operation. Includes a random, internal-only,
+        // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
+        .instrument(::tracing::debug_span!(
+            "STS.GetCallerIdentity",
+            "rpc.service" = "STS",
+            "rpc.method" = "GetCallerIdentity",
+            "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
+            "rpc.system" = "aws-api",
+        ))
+        .await
     }

     pub(crate) fn operation_runtime_plugins(
@@ -91,18 +101,25 @@
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
             GetCallerIdentityRequestSerializer,
         ));
-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            GetCallerIdentityResponseDeserializer,
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
+                GetCallerIdentityResponseDeserializer,
+            ),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
-            crate::config::auth::Params::builder()
-                .operation_name("GetCallerIdentity")
-                .build()
-                .expect("required fields set"),
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
+                crate::config::auth::Params::builder()
+                    .operation_name("GetCallerIdentity")
+                    .build()
+                    .expect("required fields set"),
+            ),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("GetCallerIdentity", "STS"));
+        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
+            "GetCallerIdentity",
+            "STS",
+        ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
         signing_options.double_uri_encode = true;
         signing_options.content_sha256_header = false;
@@ -122,16 +139,41 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetCallerIdentity")
-                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
-.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(GetCallerIdentityEndpointParamsInterceptor))
-                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::get_caller_identity::GetCallerIdentityError>::new())
-.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::get_caller_identity::GetCallerIdentityError>::new())
-.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::get_caller_identity::GetCallerIdentityError>::builder().transient_errors({
-                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
-                                            transient_errors.push("IDPCommunicationError");
-                                            ::std::borrow::Cow::Owned(transient_errors)
-                                            }).build());
+        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new(
+            "GetCallerIdentity",
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
+            ),
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                GetCallerIdentityEndpointParamsInterceptor,
+            ),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+                crate::operation::get_caller_identity::GetCallerIdentityError,
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+                crate::operation::get_caller_identity::GetCallerIdentityError,
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                crate::operation::get_caller_identity::GetCallerIdentityError,
+            >::builder()
+            .transient_errors({
+                let mut transient_errors: Vec<&'static str> =
+                    ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                transient_errors.push("IDPCommunicationError");
+                ::std::borrow::Cow::Owned(transient_errors)
+            })
+            .build(),
+        );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -154,7 +196,9 @@
         let parse_result = if !success && status != 200 || force_error {
             crate::protocol_serde::shape_get_caller_identity::de_get_caller_identity_http_error(status, headers, body)
         } else {
-            crate::protocol_serde::shape_get_caller_identity::de_get_caller_identity_http_response(status, headers, body)
+            crate::protocol_serde::shape_get_caller_identity::de_get_caller_identity_http_response(
+                status, headers, body,
+            )
         };
         crate::protocol_serde::type_erase_result(parse_result)
     }
@@ -162,12 +206,20 @@
 #[derive(Debug)]
 struct GetCallerIdentityRequestSerializer;
 impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for GetCallerIdentityRequestSerializer {
-    #[allow(unused_mut, clippy::let_and_return, clippy::needless_borrow, clippy::useless_conversion)]
+    #[allow(
+        unused_mut,
+        clippy::let_and_return,
+        clippy::needless_borrow,
+        clippy::useless_conversion
+    )]
     fn serialize_input(
         &self,
         input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
         _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
-    ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
+    ) -> ::std::result::Result<
+        ::aws_smithy_runtime_api::client::orchestrator::HttpRequest,
+        ::aws_smithy_runtime_api::box_error::BoxError,
+    > {
         let input = input
             .downcast::<crate::operation::get_caller_identity::GetCallerIdentityInput>()
             .expect("correct type");
@@ -189,14 +241,18 @@
             fn update_http_builder(
                 input: &crate::operation::get_caller_identity::GetCallerIdentityInput,
                 builder: ::http_1x::request::Builder,
-            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError>
+            {
                 let mut uri = ::std::string::String::new();
                 uri_base(input, &mut uri)?;
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(
+                builder,
+                ::http_1x::header::CONTENT_TYPE,
+                "application/x-www-form-urlencoded",
+            );
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
@@ -234,13 +290,21 @@
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
-            .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
+            .set_endpoint(
+                cfg.load::<::aws_types::endpoint_config::EndpointUrl>()
+                    .map(|ty| ty.0.clone()),
+            )
             .build()
             .map_err(|err| {
-                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new("endpoint params could not be built", err)
+                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new(
+                    "endpoint params could not be built",
+                    err,
+                )
             })?;
         cfg.interceptor_state()
-            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params));
+            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(
+                params,
+            ));
         ::std::result::Result::Ok(())
     }
 }
@@ -253,18 +317,22 @@
 #[derive(::std::fmt::Debug)]
 pub enum GetCallerIdentityError {
     /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
-    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
+    #[deprecated(
+        note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
     variable wildcard pattern and check `.code()`:
      \
     &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
      \
-    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-GetCallerIdentityError) for what information is available for the error.")]
+    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-GetCallerIdentityError) for what information is available for the error."
+    )]
     Unhandled(crate::error::sealed_unhandled::Unhandled),
 }
 impl GetCallerIdentityError {
     /// Creates the `GetCallerIdentityError::Unhandled` variant from any error type.
     pub fn unhandled(
-        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
+        err: impl ::std::convert::Into<
+            ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
+        >,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
             source: err.into(),
@@ -300,7 +368,9 @@
     fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
         match self {
             Self::Unhandled(_inner) => {
-                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
+                if let ::std::option::Option::Some(code) =
+                    ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
+                {
                     write!(f, "unhandled error ({code})")
                 } else {
                     f.write_str("unhandled error")
```

### `src/operation/get_caller_identity/_get_caller_identity_input.rs`

```diff
diff --git a/src/operation/get_caller_identity/_get_caller_identity_input.rs b/src/operation/get_caller_identity/_get_caller_identity_input.rs
--- a/src/operation/get_caller_identity/_get_caller_identity_input.rs
+++ b/src/operation/get_caller_identity/_get_caller_identity_input.rs
@@ -18,7 +18,10 @@
     /// Consumes the builder and constructs a [`GetCallerIdentityInput`](crate::operation::get_caller_identity::GetCallerIdentityInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<crate::operation::get_caller_identity::GetCallerIdentityInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        crate::operation::get_caller_identity::GetCallerIdentityInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(crate::operation::get_caller_identity::GetCallerIdentityInput {})
     }
 }
```

### `src/operation/get_caller_identity/builders.rs`

```diff
diff --git a/src/operation/get_caller_identity/builders.rs b/src/operation/get_caller_identity/builders.rs
--- a/src/operation/get_caller_identity/builders.rs
+++ b/src/operation/get_caller_identity/builders.rs
@@ -101,12 +101,18 @@
     > {
         crate::client::customize::CustomizableOperation::new(self)
     }
-    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
+    pub(crate) fn config_override(
+        mut self,
+        config_override: impl ::std::convert::Into<crate::config::Builder>,
+    ) -> Self {
         self.set_config_override(::std::option::Option::Some(config_override.into()));
         self
     }

-    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
+    pub(crate) fn set_config_override(
+        &mut self,
+        config_override: ::std::option::Option<crate::config::Builder>,
+    ) -> &mut Self {
         self.config_override = config_override;
         self
     }
```

### `src/operation/get_delegated_access_token.rs`

```diff
diff --git a/src/operation/get_delegated_access_token.rs b/src/operation/get_delegated_access_token.rs
--- a/src/operation/get_delegated_access_token.rs
+++ b/src/operation/get_delegated_access_token.rs
@@ -27,9 +27,13 @@
                     .expect("correct error type")
             })
         };
-        let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
-            .await
-            .map_err(map_err)?;
+        let context = Self::orchestrate_with_stop_point(
+            runtime_plugins,
+            input,
+            ::aws_smithy_runtime::client::orchestrator::StopPoint::None,
+        )
+        .await
+        .map_err(map_err)?;
         let output = context.finalize().map_err(map_err)?;
         ::std::result::Result::Ok(
             output
@@ -51,17 +55,23 @@
     > {
         let input = ::aws_smithy_runtime_api::client::interceptors::context::Input::erase(input);
         use ::tracing::Instrument;
-        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("STS", "GetDelegatedAccessToken", input, runtime_plugins, stop_point)
-            // Create a parent span for the entire operation. Includes a random, internal-only,
-            // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
-            .instrument(::tracing::debug_span!(
-                "STS.GetDelegatedAccessToken",
-                "rpc.service" = "STS",
-                "rpc.method" = "GetDelegatedAccessToken",
-                "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
-                "rpc.system" = "aws-api",
-            ))
-            .await
+        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point(
+            "STS",
+            "GetDelegatedAccessToken",
+            input,
+            runtime_plugins,
+            stop_point,
+        )
+        // Create a parent span for the entire operation. Includes a random, internal-only,
+        // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
+        .instrument(::tracing::debug_span!(
+            "STS.GetDelegatedAccessToken",
+            "rpc.service" = "STS",
+            "rpc.method" = "GetDelegatedAccessToken",
+            "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
+            "rpc.system" = "aws-api",
+        ))
+        .await
     }

     pub(crate) fn operation_runtime_plugins(
@@ -91,16 +101,20 @@
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
             GetDelegatedAccessTokenRequestSerializer,
         ));
-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            GetDelegatedAccessTokenResponseDeserializer,
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
+                GetDelegatedAccessTokenResponseDeserializer,
+            ),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
-            crate::config::auth::Params::builder()
-                .operation_name("GetDelegatedAccessToken")
-                .build()
-                .expect("required fields set"),
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
+                crate::config::auth::Params::builder()
+                    .operation_name("GetDelegatedAccessToken")
+                    .build()
+                    .expect("required fields set"),
+            ),
+        );

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::SensitiveOutput);
         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
@@ -126,30 +140,41 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetDelegatedAccessToken")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new(
+            "GetDelegatedAccessToken",
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+            ),
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 GetDelegatedAccessTokenEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+            ),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError,
-            >::new())
-            .with_retry_classifier(
-                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                    crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError,
-                >::builder()
-                .transient_errors({
-                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
-                    transient_errors.push("IDPCommunicationError");
-                    ::std::borrow::Cow::Owned(transient_errors)
-                })
-                .build(),
-            );
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError,
+            >::builder()
+            .transient_errors({
+                let mut transient_errors: Vec<&'static str> =
+                    ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                transient_errors.push("IDPCommunicationError");
+                ::std::borrow::Cow::Owned(transient_errors)
+            })
+            .build(),
+        );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -170,9 +195,13 @@
         let mut force_error = false;
         ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));
         let parse_result = if !success && status != 200 || force_error {
-            crate::protocol_serde::shape_get_delegated_access_token::de_get_delegated_access_token_http_error(status, headers, body)
+            crate::protocol_serde::shape_get_delegated_access_token::de_get_delegated_access_token_http_error(
+                status, headers, body,
+            )
         } else {
-            crate::protocol_serde::shape_get_delegated_access_token::de_get_delegated_access_token_http_response(status, headers, body)
+            crate::protocol_serde::shape_get_delegated_access_token::de_get_delegated_access_token_http_response(
+                status, headers, body,
+            )
         };
         crate::protocol_serde::type_erase_result(parse_result)
     }
@@ -180,12 +209,20 @@
 #[derive(Debug)]
 struct GetDelegatedAccessTokenRequestSerializer;
 impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for GetDelegatedAccessTokenRequestSerializer {
-    #[allow(unused_mut, clippy::let_and_return, clippy::needless_borrow, clippy::useless_conversion)]
+    #[allow(
+        unused_mut,
+        clippy::let_and_return,
+        clippy::needless_borrow,
+        clippy::useless_conversion
+    )]
     fn serialize_input(
         &self,
         input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
         _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
-    ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
+    ) -> ::std::result::Result<
+        ::aws_smithy_runtime_api::client::orchestrator::HttpRequest,
+        ::aws_smithy_runtime_api::box_error::BoxError,
+    > {
         let input = input
             .downcast::<crate::operation::get_delegated_access_token::GetDelegatedAccessTokenInput>()
             .expect("correct type");
@@ -207,14 +244,18 @@
             fn update_http_builder(
                 input: &crate::operation::get_delegated_access_token::GetDelegatedAccessTokenInput,
                 builder: ::http_1x::request::Builder,
-            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError>
+            {
                 let mut uri = ::std::string::String::new();
                 uri_base(input, &mut uri)?;
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(
+                builder,
+                ::http_1x::header::CONTENT_TYPE,
+                "application/x-www-form-urlencoded",
+            );
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
@@ -222,7 +263,11 @@
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
-            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+            request_builder = _header_serialization_settings.set_default_header(
+                request_builder,
+                ::http_1x::header::CONTENT_LENGTH,
+                &content_length,
+            );
         }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
@@ -255,13 +300,21 @@
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
-            .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
+            .set_endpoint(
+                cfg.load::<::aws_types::endpoint_config::EndpointUrl>()
+                    .map(|ty| ty.0.clone()),
+            )
             .build()
             .map_err(|err| {
-                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new("endpoint params could not be built", err)
+                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new(
+                    "endpoint params could not be built",
+                    err,
+                )
             })?;
         cfg.interceptor_state()
-            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params));
+            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(
+                params,
+            ));
         ::std::result::Result::Ok(())
     }
 }
@@ -281,18 +334,22 @@
     /// <p>STS is not activated in the requested region for the account that is being asked to generate credentials. The account administrator must use the IAM console to activate STS in that region. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_enable-regions.html#sts-regions-activate-deactivate">Activating and Deactivating STS in an Amazon Web Services Region</a> in the <i>IAM User Guide</i>.</p>
     RegionDisabledException(crate::types::error::RegionDisabledException),
     /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
-    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
+    #[deprecated(
+        note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
     variable wildcard pattern and check `.code()`:
      \
     &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
      \
-    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-GetDelegatedAccessTokenError) for what information is available for the error.")]
+    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-GetDelegatedAccessTokenError) for what information is available for the error."
+    )]
     Unhandled(crate::error::sealed_unhandled::Unhandled),
 }
 impl GetDelegatedAccessTokenError {
     /// Creates the `GetDelegatedAccessTokenError::Unhandled` variant from any error type.
     pub fn unhandled(
-        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
+        err: impl ::std::convert::Into<
+            ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
+        >,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
             source: err.into(),
@@ -314,7 +371,9 @@
     pub fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
             Self::ExpiredTradeInTokenException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::PackedPolicyTooLargeException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::PackedPolicyTooLargeException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
             Self::RegionDisabledException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
             Self::Unhandled(e) => &e.meta,
         }
@@ -349,7 +408,9 @@
             Self::PackedPolicyTooLargeException(_inner) => _inner.fmt(f),
             Self::RegionDisabledException(_inner) => _inner.fmt(f),
             Self::Unhandled(_inner) => {
-                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
+                if let ::std::option::Option::Some(code) =
+                    ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
+                {
                     write!(f, "unhandled error ({code})")
                 } else {
                     f.write_str("unhandled error")
@@ -369,9 +430,15 @@
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for GetDelegatedAccessTokenError {
     fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::ExpiredTradeInTokenException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::PackedPolicyTooLargeException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::RegionDisabledException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::ExpiredTradeInTokenException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::PackedPolicyTooLargeException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::RegionDisabledException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
             Self::Unhandled(_inner) => &_inner.meta,
         }
     }
```

### `src/operation/get_delegated_access_token/_get_delegated_access_token_input.rs`

```diff
diff --git a/src/operation/get_delegated_access_token/_get_delegated_access_token_input.rs b/src/operation/get_delegated_access_token/_get_delegated_access_token_input.rs
--- a/src/operation/get_delegated_access_token/_get_delegated_access_token_input.rs
+++ b/src/operation/get_delegated_access_token/_get_delegated_access_token_input.rs
@@ -55,9 +55,11 @@
         crate::operation::get_delegated_access_token::GetDelegatedAccessTokenInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(crate::operation::get_delegated_access_token::GetDelegatedAccessTokenInput {
-            trade_in_token: self.trade_in_token,
-        })
+        ::std::result::Result::Ok(
+            crate::operation::get_delegated_access_token::GetDelegatedAccessTokenInput {
+                trade_in_token: self.trade_in_token,
+            },
+        )
     }
 }
 impl ::std::fmt::Debug for GetDelegatedAccessTokenInputBuilder {
```

### `src/operation/get_delegated_access_token/builders.rs`

```diff
diff --git a/src/operation/get_delegated_access_token/builders.rs b/src/operation/get_delegated_access_token/builders.rs
--- a/src/operation/get_delegated_access_token/builders.rs
+++ b/src/operation/get_delegated_access_token/builders.rs
@@ -57,7 +57,9 @@
         }
     }
     /// Access the GetDelegatedAccessToken as a reference.
-    pub fn as_input(&self) -> &crate::operation::get_delegated_access_token::builders::GetDelegatedAccessTokenInputBuilder {
+    pub fn as_input(
+        &self,
+    ) -> &crate::operation::get_delegated_access_token::builders::GetDelegatedAccessTokenInputBuilder {
         &self.inner
     }
     /// Sends the request and returns the response.
@@ -81,12 +83,14 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = crate::operation::get_delegated_access_token::GetDelegatedAccessToken::operation_runtime_plugins(
-            self.handle.runtime_plugins.clone(),
-            &self.handle.conf,
-            self.config_override,
-        );
-        crate::operation::get_delegated_access_token::GetDelegatedAccessToken::orchestrate(&runtime_plugins, input).await
+        let runtime_plugins =
+            crate::operation::get_delegated_access_token::GetDelegatedAccessToken::operation_runtime_plugins(
+                self.handle.runtime_plugins.clone(),
+                &self.handle.conf,
+                self.config_override,
+            );
+        crate::operation::get_delegated_access_token::GetDelegatedAccessToken::orchestrate(&runtime_plugins, input)
+            .await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -99,12 +103,18 @@
     > {
         crate::client::customize::CustomizableOperation::new(self)
     }
-    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
+    pub(crate) fn config_override(
+        mut self,
+        config_override: impl ::std::convert::Into<crate::config::Builder>,
+    ) -> Self {
         self.set_config_override(::std::option::Option::Some(config_override.into()));
         self
     }

-    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
+    pub(crate) fn set_config_override(
+        &mut self,
+        config_override: ::std::option::Option<crate::config::Builder>,
+    ) -> &mut Self {
         self.config_override = config_override;
         self
     }
```

### `src/operation/get_federation_token.rs`

```diff
diff --git a/src/operation/get_federation_token.rs b/src/operation/get_federation_token.rs
--- a/src/operation/get_federation_token.rs
+++ b/src/operation/get_federation_token.rs
@@ -27,9 +27,13 @@
                     .expect("correct error type")
             })
         };
-        let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
-            .await
-            .map_err(map_err)?;
+        let context = Self::orchestrate_with_stop_point(
+            runtime_plugins,
+            input,
+            ::aws_smithy_runtime::client::orchestrator::StopPoint::None,
+        )
+        .await
+        .map_err(map_err)?;
         let output = context.finalize().map_err(map_err)?;
         ::std::result::Result::Ok(
             output
@@ -51,17 +55,23 @@
     > {
         let input = ::aws_smithy_runtime_api::client::interceptors::context::Input::erase(input);
         use ::tracing::Instrument;
-        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("STS", "GetFederationToken", input, runtime_plugins, stop_point)
-            // Create a parent span for the entire operation. Includes a random, internal-only,
-            // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
-            .instrument(::tracing::debug_span!(
-                "STS.GetFederationToken",
-                "rpc.service" = "STS",
-                "rpc.method" = "GetFederationToken",
-                "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
-                "rpc.system" = "aws-api",
-            ))
-            .await
+        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point(
+            "STS",
+            "GetFederationToken",
+            input,
+            runtime_plugins,
+            stop_point,
+        )
+        // Create a parent span for the entire operation. Includes a random, internal-only,
+        // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
+        .instrument(::tracing::debug_span!(
+            "STS.GetFederationToken",
+            "rpc.service" = "STS",
+            "rpc.method" = "GetFederationToken",
+            "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
+            "rpc.system" = "aws-api",
+        ))
+        .await
     }

     pub(crate) fn operation_runtime_plugins(
@@ -91,19 +101,26 @@
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
             GetFederationTokenRequestSerializer,
         ));
-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            GetFederationTokenResponseDeserializer,
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
+                GetFederationTokenResponseDeserializer,
+            ),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
-            crate::config::auth::Params::builder()
-                .operation_name("GetFederationToken")
-                .build()
-                .expect("required fields set"),
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
+                crate::config::auth::Params::builder()
+                    .operation_name("GetFederationToken")
+                    .build()
+                    .expect("required fields set"),
+            ),
+        );

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::SensitiveOutput);
-        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("GetFederationToken", "STS"));
+        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
+            "GetFederationToken",
+            "STS",
+        ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
         signing_options.double_uri_encode = true;
         signing_options.content_sha256_header = false;
@@ -123,17 +140,46 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetFederationToken")
-                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(GetFederationTokenTelemetryInputCaptureInterceptor))
-.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
-.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(GetFederationTokenEndpointParamsInterceptor))
-                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::get_federation_token::GetFederationTokenError>::new())
-.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::get_federation_token::GetFederationTokenError>::new())
-.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::get_federation_token::GetFederationTokenError>::builder().transient_errors({
-                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
-                                            transient_errors.push("IDPCommunicationError");
-                                            ::std::borrow::Cow::Owned(transient_errors)
-                                            }).build());
+        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new(
+            "GetFederationToken",
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                GetFederationTokenTelemetryInputCaptureInterceptor,
+            ),
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
+            ),
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                GetFederationTokenEndpointParamsInterceptor,
+            ),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+                crate::operation::get_federation_token::GetFederationTokenError,
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+                crate::operation::get_federation_token::GetFederationTokenError,
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                crate::operation::get_federation_token::GetFederationTokenError,
+            >::builder()
+            .transient_errors({
+                let mut transient_errors: Vec<&'static str> =
+                    ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                transient_errors.push("IDPCommunicationError");
+                ::std::borrow::Cow::Owned(transient_errors)
+            })
+            .build(),
+        );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -204,7 +250,9 @@
         let parse_result = if !success && status != 200 || force_error {
             crate::protocol_serde::shape_get_federation_token::de_get_federation_token_http_error(status, headers, body)
         } else {
-            crate::protocol_serde::shape_get_federation_token::de_get_federation_token_http_response(status, headers, body)
+            crate::protocol_serde::shape_get_federation_token::de_get_federation_token_http_response(
+                status, headers, body,
+            )
         };
         crate::protocol_serde::type_erase_result(parse_result)
     }
@@ -212,12 +260,20 @@
 #[derive(Debug)]
 struct GetFederationTokenRequestSerializer;
 impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for GetFederationTokenRequestSerializer {
-    #[allow(unused_mut, clippy::let_and_return, clippy::needless_borrow, clippy::useless_conversion)]
+    #[allow(
+        unused_mut,
+        clippy::let_and_return,
+        clippy::needless_borrow,
+        clippy::useless_conversion
+    )]
     fn serialize_input(
         &self,
         input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
         _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
-    ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
+    ) -> ::std::result::Result<
+        ::aws_smithy_runtime_api::client::orchestrator::HttpRequest,
+        ::aws_smithy_runtime_api::box_error::BoxError,
+    > {
         let input = input
             .downcast::<crate::operation::get_federation_token::GetFederationTokenInput>()
             .expect("correct type");
@@ -239,22 +295,32 @@
             fn update_http_builder(
                 input: &crate::operation::get_federation_token::GetFederationTokenInput,
                 builder: ::http_1x::request::Builder,
-            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError>
+            {
                 let mut uri = ::std::string::String::new();
                 uri_base(input, &mut uri)?;
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(
+                builder,
+                ::http_1x::header::CONTENT_TYPE,
+                "application/x-www-form-urlencoded",
+            );
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_get_federation_token_input::ser_get_federation_token_input_input_input(&input)?,
+            crate::protocol_serde::shape_get_federation_token_input::ser_get_federation_token_input_input_input(
+                &input,
+            )?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
-            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+            request_builder = _header_serialization_settings.set_default_header(
+                request_builder,
+                ::http_1x::header::CONTENT_LENGTH,
+                &content_length,
+            );
         }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
@@ -287,13 +353,21 @@
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
-            .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
+            .set_endpoint(
+                cfg.load::<::aws_types::endpoint_config::EndpointUrl>()
+                    .map(|ty| ty.0.clone()),
+            )
             .build()
             .map_err(|err| {
-                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new("endpoint params could not be built", err)
+                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new(
+                    "endpoint params could not be built",
+                    err,
+                )
             })?;
         cfg.interceptor_state()
-            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params));
+            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(
+                params,
+            ));
         ::std::result::Result::Ok(())
     }
 }
@@ -313,18 +387,22 @@
     /// <p>STS is not activated in the requested region for the account that is being asked to generate credentials. The account administrator must use the IAM console to activate STS in that region. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_enable-regions.html#sts-regions-activate-deactivate">Activating and Deactivating STS in an Amazon Web Services Region</a> in the <i>IAM User Guide</i>.</p>
     RegionDisabledException(crate::types::error::RegionDisabledException),
     /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
-    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
+    #[deprecated(
+        note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
     variable wildcard pattern and check `.code()`:
      \
     &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
      \
-    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-GetFederationTokenError) for what information is available for the error.")]
+    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-GetFederationTokenError) for what information is available for the error."
+    )]
     Unhandled(crate::error::sealed_unhandled::Unhandled),
 }
 impl GetFederationTokenError {
     /// Creates the `GetFederationTokenError::Unhandled` variant from any error type.
     pub fn unhandled(
-        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
+        err: impl ::std::convert::Into<
+            ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
+        >,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
             source: err.into(),
@@ -345,8 +423,12 @@
     ///
     pub fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::MalformedPolicyDocumentException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::PackedPolicyTooLargeException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::MalformedPolicyDocumentException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
+            Self::PackedPolicyTooLargeException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
             Self::RegionDisabledException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
             Self::Unhandled(e) => &e.meta,
         }
@@ -381,7 +463,9 @@
             Self::PackedPolicyTooLargeException(_inner) => _inner.fmt(f),
             Self::RegionDisabledException(_inner) => _inner.fmt(f),
             Self::Unhandled(_inner) => {
-                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
+                if let ::std::option::Option::Some(code) =
+                    ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
+                {
                     write!(f, "unhandled error ({code})")
                 } else {
                     f.write_str("unhandled error")
@@ -401,9 +485,15 @@
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for GetFederationTokenError {
     fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::MalformedPolicyDocumentException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::PackedPolicyTooLargeException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::RegionDisabledException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::MalformedPolicyDocumentException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::PackedPolicyTooLargeException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::RegionDisabledException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
             Self::Unhandled(_inner) => &_inner.meta,
         }
     }
```

### `src/operation/get_federation_token/_get_federation_token_input.rs`

```diff
diff --git a/src/operation/get_federation_token/_get_federation_token_input.rs b/src/operation/get_federation_token/_get_federation_token_input.rs
--- a/src/operation/get_federation_token/_get_federation_token_input.rs
+++ b/src/operation/get_federation_token/_get_federation_token_input.rs
@@ -173,7 +173,10 @@
     /// <p>The resulting credentials can be used to access a resource that has a resource-based policy. If that policy specifically references the federated user session in the <code>Principal</code> element of the policy, the session has the permissions allowed by the policy. These permissions are granted in addition to the permissions that are granted by the session policies.</p><note>
     /// <p>An Amazon Web Services conversion compresses the passed inline session policy, managed policy ARNs, and session tags into a packed binary format that has a separate limit. Your request can fail for this limit even if your plaintext meets the other requirements. The <code>PackedPolicySize</code> response element indicates by percentage how close the policies and tags for your request are to the upper size limit.</p>
     /// </note>
-    pub fn set_policy_arns(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::PolicyDescriptorType>>) -> Self {
+    pub fn set_policy_arns(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<crate::types::PolicyDescriptorType>>,
+    ) -> Self {
         self.policy_arns = input;
         self
     }
@@ -239,8 +242,10 @@
     /// Consumes the builder and constructs a [`GetFederationTokenInput`](crate::operation::get_federation_token::GetFederationTokenInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<crate::operation::get_federation_token::GetFederationTokenInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        crate::operation::get_federation_token::GetFederationTokenInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(crate::operation::get_federation_token::GetFederationTokenInput {
             name: self.name,
             policy: self.policy,
```

### `src/operation/get_federation_token/builders.rs`

```diff
diff --git a/src/operation/get_federation_token/builders.rs b/src/operation/get_federation_token/builders.rs
--- a/src/operation/get_federation_token/builders.rs
+++ b/src/operation/get_federation_token/builders.rs
@@ -123,12 +123,18 @@
     > {
         crate::client::customize::CustomizableOperation::new(self)
     }
-    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
+    pub(crate) fn config_override(
+        mut self,
+        config_override: impl ::std::convert::Into<crate::config::Builder>,
+    ) -> Self {
         self.set_config_override(::std::option::Option::Some(config_override.into()));
         self
     }

-    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
+    pub(crate) fn set_config_override(
+        &mut self,
+        config_override: ::std::option::Option<crate::config::Builder>,
+    ) -> &mut Self {
         self.config_override = config_override;
         self
     }
@@ -207,7 +213,10 @@
     /// <p>The resulting credentials can be used to access a resource that has a resource-based policy. If that policy specifically references the federated user session in the <code>Principal</code> element of the policy, the session has the permissions allowed by the policy. These permissions are granted in addition to the permissions that are granted by the session policies.</p><note>
     /// <p>An Amazon Web Services conversion compresses the passed inline session policy, managed policy ARNs, and session tags into a packed binary format that has a separate limit. Your request can fail for this limit even if your plaintext meets the other requirements. The <code>PackedPolicySize</code> response element indicates by percentage how close the policies and tags for your request are to the upper size limit.</p>
     /// </note>
-    pub fn set_policy_arns(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::PolicyDescriptorType>>) -> Self {
+    pub fn set_policy_arns(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<crate::types::PolicyDescriptorType>>,
+    ) -> Self {
         self.inner = self.inner.set_policy_arns(input);
         self
     }
```

### `src/operation/get_session_token.rs`

```diff
diff --git a/src/operation/get_session_token.rs b/src/operation/get_session_token.rs
--- a/src/operation/get_session_token.rs
+++ b/src/operation/get_session_token.rs
@@ -27,9 +27,13 @@
                     .expect("correct error type")
             })
         };
-        let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
-            .await
-            .map_err(map_err)?;
+        let context = Self::orchestrate_with_stop_point(
+            runtime_plugins,
+            input,
+            ::aws_smithy_runtime::client::orchestrator::StopPoint::None,
+        )
+        .await
+        .map_err(map_err)?;
         let output = context.finalize().map_err(map_err)?;
         ::std::result::Result::Ok(
             output
@@ -51,17 +55,23 @@
     > {
         let input = ::aws_smithy_runtime_api::client::interceptors::context::Input::erase(input);
         use ::tracing::Instrument;
-        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("STS", "GetSessionToken", input, runtime_plugins, stop_point)
-            // Create a parent span for the entire operation. Includes a random, internal-only,
-            // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
-            .instrument(::tracing::debug_span!(
-                "STS.GetSessionToken",
-                "rpc.service" = "STS",
-                "rpc.method" = "GetSessionToken",
-                "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
-                "rpc.system" = "aws-api",
-            ))
-            .await
+        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point(
+            "STS",
+            "GetSessionToken",
+            input,
+            runtime_plugins,
+            stop_point,
+        )
+        // Create a parent span for the entire operation. Includes a random, internal-only,
+        // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
+        .instrument(::tracing::debug_span!(
+            "STS.GetSessionToken",
+            "rpc.service" = "STS",
+            "rpc.method" = "GetSessionToken",
+            "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
+            "rpc.system" = "aws-api",
+        ))
+        .await
     }

     pub(crate) fn operation_runtime_plugins(
@@ -91,19 +101,26 @@
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
             GetSessionTokenRequestSerializer,
         ));
-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            GetSessionTokenResponseDeserializer,
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
+                GetSessionTokenResponseDeserializer,
+            ),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
-            crate::config::auth::Params::builder()
-                .operation_name("GetSessionToken")
-                .build()
-                .expect("required fields set"),
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
+                crate::config::auth::Params::builder()
+                    .operation_name("GetSessionToken")
+                    .build()
+                    .expect("required fields set"),
+            ),
+        );

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::SensitiveOutput);
-        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("GetSessionToken", "STS"));
+        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
+            "GetSessionToken",
+            "STS",
+        ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
         signing_options.double_uri_encode = true;
         signing_options.content_sha256_header = false;
@@ -123,31 +140,46 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetSessionToken")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new(
+            "GetSessionToken",
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 GetSessionTokenTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+            ),
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+            ),
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 GetSessionTokenEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+            ),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+                crate::operation::get_session_token::GetSessionTokenError,
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::get_session_token::GetSessionTokenError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
                 crate::operation::get_session_token::GetSessionTokenError,
-            >::new())
-            .with_retry_classifier(
-                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::get_session_token::GetSessionTokenError>::builder()
-                    .transient_errors({
-                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
-                        transient_errors.push("IDPCommunicationError");
-                        ::std::borrow::Cow::Owned(transient_errors)
-                    })
-                    .build(),
-            );
+            >::builder()
+            .transient_errors({
+                let mut transient_errors: Vec<&'static str> =
+                    ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                transient_errors.push("IDPCommunicationError");
+                ::std::borrow::Cow::Owned(transient_errors)
+            })
+            .build(),
+        );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -226,12 +258,20 @@
 #[derive(Debug)]
 struct GetSessionTokenRequestSerializer;
 impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for GetSessionTokenRequestSerializer {
-    #[allow(unused_mut, clippy::let_and_return, clippy::needless_borrow, clippy::useless_conversion)]
+    #[allow(
+        unused_mut,
+        clippy::let_and_return,
+        clippy::needless_borrow,
+        clippy::useless_conversion
+    )]
     fn serialize_input(
         &self,
         input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
         _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
-    ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
+    ) -> ::std::result::Result<
+        ::aws_smithy_runtime_api::client::orchestrator::HttpRequest,
+        ::aws_smithy_runtime_api::box_error::BoxError,
+    > {
         let input = input
             .downcast::<crate::operation::get_session_token::GetSessionTokenInput>()
             .expect("correct type");
@@ -253,14 +293,18 @@
             fn update_http_builder(
                 input: &crate::operation::get_session_token::GetSessionTokenInput,
                 builder: ::http_1x::request::Builder,
-            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError>
+            {
                 let mut uri = ::std::string::String::new();
                 uri_base(input, &mut uri)?;
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(
+                builder,
+                ::http_1x::header::CONTENT_TYPE,
+                "application/x-www-form-urlencoded",
+            );
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
@@ -268,7 +312,11 @@
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
-            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+            request_builder = _header_serialization_settings.set_default_header(
+                request_builder,
+                ::http_1x::header::CONTENT_LENGTH,
+                &content_length,
+            );
         }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
@@ -301,13 +349,21 @@
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
-            .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
+            .set_endpoint(
+                cfg.load::<::aws_types::endpoint_config::EndpointUrl>()
+                    .map(|ty| ty.0.clone()),
+            )
             .build()
             .map_err(|err| {
-                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new("endpoint params could not be built", err)
+                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new(
+                    "endpoint params could not be built",
+                    err,
+                )
             })?;
         cfg.interceptor_state()
-            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params));
+            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(
+                params,
+            ));
         ::std::result::Result::Ok(())
     }
 }
@@ -322,18 +378,22 @@
     /// <p>STS is not activated in the requested region for the account that is being asked to generate credentials. The account administrator must use the IAM console to activate STS in that region. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_enable-regions.html#sts-regions-activate-deactivate">Activating and Deactivating STS in an Amazon Web Services Region</a> in the <i>IAM User Guide</i>.</p>
     RegionDisabledException(crate::types::error::RegionDisabledException),
     /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
-    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
+    #[deprecated(
+        note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
     variable wildcard pattern and check `.code()`:
      \
     &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
      \
-    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-GetSessionTokenError) for what information is available for the error.")]
+    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-GetSessionTokenError) for what information is available for the error."
+    )]
     Unhandled(crate::error::sealed_unhandled::Unhandled),
 }
 impl GetSessionTokenError {
     /// Creates the `GetSessionTokenError::Unhandled` variant from any error type.
     pub fn unhandled(
-        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
+        err: impl ::std::convert::Into<
+            ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
+        >,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
             source: err.into(),
@@ -376,7 +436,9 @@
         match self {
             Self::RegionDisabledException(_inner) => _inner.fmt(f),
             Self::Unhandled(_inner) => {
-                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
+                if let ::std::option::Option::Some(code) =
+                    ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
+                {
                     write!(f, "unhandled error ({code})")
                 } else {
                     f.write_str("unhandled error")
@@ -396,7 +458,9 @@
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for GetSessionTokenError {
     fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::RegionDisabledException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::RegionDisabledException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
             Self::Unhandled(_inner) => &_inner.meta,
         }
     }
```

### `src/operation/get_session_token/_get_session_token_input.rs`

```diff
diff --git a/src/operation/get_session_token/_get_session_token_input.rs b/src/operation/get_session_token/_get_session_token_input.rs
--- a/src/operation/get_session_token/_get_session_token_input.rs
+++ b/src/operation/get_session_token/_get_session_token_input.rs
@@ -95,7 +95,10 @@
     /// Consumes the builder and constructs a [`GetSessionTokenInput`](crate::operation::get_session_token::GetSessionTokenInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<crate::operation::get_session_token::GetSessionTokenInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        crate::operation::get_session_token::GetSessionTokenInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(crate::operation::get_session_token::GetSessionTokenInput {
             duration_seconds: self.duration_seconds,
             serial_number: self.serial_number,
```

### `src/operation/get_session_token/builders.rs`

```diff
diff --git a/src/operation/get_session_token/builders.rs b/src/operation/get_session_token/builders.rs
--- a/src/operation/get_session_token/builders.rs
+++ b/src/operation/get_session_token/builders.rs
@@ -116,12 +116,18 @@
     > {
         crate::client::customize::CustomizableOperation::new(self)
     }
-    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
+    pub(crate) fn config_override(
+        mut self,
+        config_override: impl ::std::convert::Into<crate::config::Builder>,
+    ) -> Self {
         self.set_config_override(::std::option::Option::Some(config_override.into()));
         self
     }

-    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
+    pub(crate) fn set_config_override(
+        &mut self,
+        config_override: ::std::option::Option<crate::config::Builder>,
+    ) -> &mut Self {
         self.config_override = config_override;
         self
     }
```

### `src/operation/get_web_identity_token.rs`

```diff
diff --git a/src/operation/get_web_identity_token.rs b/src/operation/get_web_identity_token.rs
--- a/src/operation/get_web_identity_token.rs
+++ b/src/operation/get_web_identity_token.rs
@@ -27,9 +27,13 @@
                     .expect("correct error type")
             })
         };
-        let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
-            .await
-            .map_err(map_err)?;
+        let context = Self::orchestrate_with_stop_point(
+            runtime_plugins,
+            input,
+            ::aws_smithy_runtime::client::orchestrator::StopPoint::None,
+        )
+        .await
+        .map_err(map_err)?;
         let output = context.finalize().map_err(map_err)?;
         ::std::result::Result::Ok(
             output
@@ -51,17 +55,23 @@
     > {
         let input = ::aws_smithy_runtime_api::client::interceptors::context::Input::erase(input);
         use ::tracing::Instrument;
-        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("STS", "GetWebIdentityToken", input, runtime_plugins, stop_point)
-            // Create a parent span for the entire operation. Includes a random, internal-only,
-            // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
-            .instrument(::tracing::debug_span!(
-                "STS.GetWebIdentityToken",
-                "rpc.service" = "STS",
-                "rpc.method" = "GetWebIdentityToken",
-                "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
-                "rpc.system" = "aws-api",
-            ))
-            .await
+        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point(
+            "STS",
+            "GetWebIdentityToken",
+            input,
+            runtime_plugins,
+            stop_point,
+        )
+        // Create a parent span for the entire operation. Includes a random, internal-only,
+        // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
+        .instrument(::tracing::debug_span!(
+            "STS.GetWebIdentityToken",
+            "rpc.service" = "STS",
+            "rpc.method" = "GetWebIdentityToken",
+            "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
+            "rpc.system" = "aws-api",
+        ))
+        .await
     }

     pub(crate) fn operation_runtime_plugins(
@@ -91,16 +101,20 @@
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
             GetWebIdentityTokenRequestSerializer,
         ));
-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            GetWebIdentityTokenResponseDeserializer,
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
+                GetWebIdentityTokenResponseDeserializer,
+            ),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
-            crate::config::auth::Params::builder()
-                .operation_name("GetWebIdentityToken")
-                .build()
-                .expect("required fields set"),
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
+                crate::config::auth::Params::builder()
+                    .operation_name("GetWebIdentityToken")
+                    .build()
+                    .expect("required fields set"),
+            ),
+        );

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::SensitiveOutput);
         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
@@ -126,17 +140,46 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetWebIdentityToken")
-                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(GetWebIdentityTokenTelemetryInputCaptureInterceptor))
-.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
-.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(GetWebIdentityTokenEndpointParamsInterceptor))
-                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::get_web_identity_token::GetWebIdentityTokenError>::new())
-.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::get_web_identity_token::GetWebIdentityTokenError>::new())
-.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::get_web_identity_token::GetWebIdentityTokenError>::builder().transient_errors({
-                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
-                                            transient_errors.push("IDPCommunicationError");
-                                            ::std::borrow::Cow::Owned(transient_errors)
-                                            }).build());
+        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new(
+            "GetWebIdentityToken",
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                GetWebIdentityTokenTelemetryInputCaptureInterceptor,
+            ),
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
+            ),
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                GetWebIdentityTokenEndpointParamsInterceptor,
+            ),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+                crate::operation::get_web_identity_token::GetWebIdentityTokenError,
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+                crate::operation::get_web_identity_token::GetWebIdentityTokenError,
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                crate::operation::get_web_identity_token::GetWebIdentityTokenError,
+            >::builder()
+            .transient_errors({
+                let mut transient_errors: Vec<&'static str> =
+                    ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                transient_errors.push("IDPCommunicationError");
+                ::std::borrow::Cow::Owned(transient_errors)
+            })
+            .build(),
+        );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -200,9 +243,13 @@
         let mut force_error = false;
         ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));
         let parse_result = if !success && status != 200 || force_error {
-            crate::protocol_serde::shape_get_web_identity_token::de_get_web_identity_token_http_error(status, headers, body)
+            crate::protocol_serde::shape_get_web_identity_token::de_get_web_identity_token_http_error(
+                status, headers, body,
+            )
         } else {
-            crate::protocol_serde::shape_get_web_identity_token::de_get_web_identity_token_http_response(status, headers, body)
+            crate::protocol_serde::shape_get_web_identity_token::de_get_web_identity_token_http_response(
+                status, headers, body,
+            )
         };
         crate::protocol_serde::type_erase_result(parse_result)
     }
@@ -210,12 +257,20 @@
 #[derive(Debug)]
 struct GetWebIdentityTokenRequestSerializer;
 impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for GetWebIdentityTokenRequestSerializer {
-    #[allow(unused_mut, clippy::let_and_return, clippy::needless_borrow, clippy::useless_conversion)]
+    #[allow(
+        unused_mut,
+        clippy::let_and_return,
+        clippy::needless_borrow,
+        clippy::useless_conversion
+    )]
     fn serialize_input(
         &self,
         input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
         _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
-    ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
+    ) -> ::std::result::Result<
+        ::aws_smithy_runtime_api::client::orchestrator::HttpRequest,
+        ::aws_smithy_runtime_api::box_error::BoxError,
+    > {
         let input = input
             .downcast::<crate::operation::get_web_identity_token::GetWebIdentityTokenInput>()
             .expect("correct type");
@@ -237,22 +292,32 @@
             fn update_http_builder(
                 input: &crate::operation::get_web_identity_token::GetWebIdentityTokenInput,
                 builder: ::http_1x::request::Builder,
-            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError>
+            {
                 let mut uri = ::std::string::String::new();
                 uri_base(input, &mut uri)?;
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(
+                builder,
+                ::http_1x::header::CONTENT_TYPE,
+                "application/x-www-form-urlencoded",
+            );
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_get_web_identity_token_input::ser_get_web_identity_token_input_input_input(&input)?,
+            crate::protocol_serde::shape_get_web_identity_token_input::ser_get_web_identity_token_input_input_input(
+                &input,
+            )?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
-            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+            request_builder = _header_serialization_settings.set_default_header(
+                request_builder,
+                ::http_1x::header::CONTENT_LENGTH,
+                &content_length,
+            );
         }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
@@ -285,13 +350,21 @@
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
-            .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
+            .set_endpoint(
+                cfg.load::<::aws_types::endpoint_config::EndpointUrl>()
+                    .map(|ty| ty.0.clone()),
+            )
             .build()
             .map_err(|err| {
-                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new("endpoint params could not be built", err)
+                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new(
+                    "endpoint params could not be built",
+                    err,
+                )
             })?;
         cfg.interceptor_state()
-            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params));
+            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(
+                params,
+            ));
         ::std::result::Result::Ok(())
     }
 }
@@ -310,18 +383,22 @@
     /// <p>The requested token duration would extend the session beyond its original expiration time. You cannot use this operation to extend the lifetime of a session beyond what was granted when the session was originally created.</p>
     SessionDurationEscalationException(crate::types::error::SessionDurationEscalationException),
     /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
-    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
+    #[deprecated(
+        note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
     variable wildcard pattern and check `.code()`:
      \
     &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
      \
-    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-GetWebIdentityTokenError) for what information is available for the error.")]
+    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-GetWebIdentityTokenError) for what information is available for the error."
+    )]
     Unhandled(crate::error::sealed_unhandled::Unhandled),
 }
 impl GetWebIdentityTokenError {
     /// Creates the `GetWebIdentityTokenError::Unhandled` variant from any error type.
     pub fn unhandled(
-        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
+        err: impl ::std::convert::Into<
+            ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
+        >,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
             source: err.into(),
@@ -342,9 +419,15 @@
     ///
     pub fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::JwtPayloadSizeExceededException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::OutboundWebIdentityFederationDisabledException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::SessionDurationEscalationException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::JwtPayloadSizeExceededException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
+            Self::OutboundWebIdentityFederationDisabledException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
+            Self::SessionDurationEscalationException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
             Self::Unhandled(e) => &e.meta,
         }
     }
@@ -378,7 +461,9 @@
             Self::OutboundWebIdentityFederationDisabledException(_inner) => _inner.fmt(f),
             Self::SessionDurationEscalationException(_inner) => _inner.fmt(f),
             Self::Unhandled(_inner) => {
-                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
+                if let ::std::option::Option::Some(code) =
+                    ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
+                {
                     write!(f, "unhandled error ({code})")
                 } else {
                     f.write_str("unhandled error")
@@ -398,9 +483,15 @@
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for GetWebIdentityTokenError {
     fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::JwtPayloadSizeExceededException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::OutboundWebIdentityFederationDisabledException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::SessionDurationEscalationException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::JwtPayloadSizeExceededException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::OutboundWebIdentityFederationDisabledException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::SessionDurationEscalationException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
             Self::Unhandled(_inner) => &_inner.meta,
         }
     }
```

### `src/operation/get_web_identity_token/_get_web_identity_token_input.rs`

```diff
diff --git a/src/operation/get_web_identity_token/_get_web_identity_token_input.rs b/src/operation/get_web_identity_token/_get_web_identity_token_input.rs
--- a/src/operation/get_web_identity_token/_get_web_identity_token_input.rs
+++ b/src/operation/get_web_identity_token/_get_web_identity_token_input.rs
@@ -123,8 +123,10 @@
     /// Consumes the builder and constructs a [`GetWebIdentityTokenInput`](crate::operation::get_web_identity_token::GetWebIdentityTokenInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<crate::operation::get_web_identity_token::GetWebIdentityTokenInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        crate::operation::get_web_identity_token::GetWebIdentityTokenInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(crate::operation::get_web_identity_token::GetWebIdentityTokenInput {
             audience: self.audience,
             duration_seconds: self.duration_seconds,
```

### `src/operation/get_web_identity_token/builders.rs`

```diff
diff --git a/src/operation/get_web_identity_token/builders.rs b/src/operation/get_web_identity_token/builders.rs
--- a/src/operation/get_web_identity_token/builders.rs
+++ b/src/operation/get_web_identity_token/builders.rs
@@ -99,12 +99,18 @@
     > {
         crate::client::customize::CustomizableOperation::new(self)
     }
-    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
+    pub(crate) fn config_override(
+        mut self,
+        config_override: impl ::std::convert::Into<crate::config::Builder>,
+    ) -> Self {
         self.set_config_override(::std::option::Option::Some(config_override.into()));
         self
     }

-    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
+    pub(crate) fn set_config_override(
+        &mut self,
+        config_override: ::std::option::Option<crate::config::Builder>,
+    ) -> &mut Self {
         self.config_override = config_override;
         self
     }
```

### `src/protocol_serde.rs`

```diff
diff --git a/src/protocol_serde.rs b/src/protocol_serde.rs
--- a/src/protocol_serde.rs
+++ b/src/protocol_serde.rs
@@ -1,9 +1,24 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_assume_root",
+    feature = "op_decode_authorization_message",
+    feature = "op_get_access_key_info",
+    feature = "op_get_caller_identity",
+    feature = "op_get_delegated_access_token",
+    feature = "op_get_federation_token",
+    feature = "op_get_session_token",
+    feature = "op_get_web_identity_token"
+))]
 pub(crate) fn type_erase_result<O, E>(
     result: ::std::result::Result<O, E>,
 ) -> ::std::result::Result<
     ::aws_smithy_runtime_api::client::interceptors::context::Output,
-    ::aws_smithy_runtime_api::client::orchestrator::OrchestratorError<::aws_smithy_runtime_api::client::interceptors::context::Error>,
+    ::aws_smithy_runtime_api::client::orchestrator::OrchestratorError<
+        ::aws_smithy_runtime_api::client::interceptors::context::Error,
+    >,
 >
 where
     O: ::std::fmt::Debug + ::std::marker::Send + ::std::marker::Sync + 'static,
@@ -15,6 +30,19 @@
         .map_err(::std::convert::Into::into)
 }

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_assume_root",
+    feature = "op_decode_authorization_message",
+    feature = "op_get_access_key_info",
+    feature = "op_get_caller_identity",
+    feature = "op_get_delegated_access_token",
+    feature = "op_get_federation_token",
+    feature = "op_get_session_token",
+    feature = "op_get_web_identity_token"
+))]
 pub fn parse_http_error_metadata(
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
@@ -23,82 +51,168 @@
     crate::rest_xml_wrapped_errors::parse_error_metadata(response_body)
 }

+#[cfg(feature = "op_assume_role")]
 pub(crate) mod shape_assume_role;

+#[cfg(feature = "op_assume_role")]
 pub(crate) mod shape_assume_role_input;

+#[cfg(feature = "op_assume_role_with_saml")]
 pub(crate) mod shape_assume_role_with_saml;

+#[cfg(feature = "op_assume_role_with_saml")]
 pub(crate) mod shape_assume_role_with_saml_input;

+#[cfg(feature = "op_assume_role_with_web_identity")]
 pub(crate) mod shape_assume_role_with_web_identity;

+#[cfg(feature = "op_assume_role_with_web_identity")]
 pub(crate) mod shape_assume_role_with_web_identity_input;

+#[cfg(feature = "op_assume_root")]
 pub(crate) mod shape_assume_root;

+#[cfg(feature = "op_assume_root")]
 pub(crate) mod shape_assume_root_input;

+#[cfg(feature = "op_decode_authorization_message")]
 pub(crate) mod shape_decode_authorization_message;

+#[cfg(feature = "op_decode_authorization_message")]
 pub(crate) mod shape_decode_authorization_message_input;

+#[cfg(feature = "op_get_access_key_info")]
 pub(crate) mod shape_get_access_key_info;

+#[cfg(feature = "op_get_access_key_info")]
 pub(crate) mod shape_get_access_key_info_input;

+#[cfg(feature = "op_get_caller_identity")]
 pub(crate) mod shape_get_caller_identity;

+#[cfg(feature = "op_get_caller_identity")]
 pub(crate) mod shape_get_caller_identity_input;

+#[cfg(feature = "op_get_delegated_access_token")]
 pub(crate) mod shape_get_delegated_access_token;

+#[cfg(feature = "op_get_delegated_access_token")]
 pub(crate) mod shape_get_delegated_access_token_input;

+#[cfg(feature = "op_get_federation_token")]
 pub(crate) mod shape_get_federation_token;

+#[cfg(feature = "op_get_federation_token")]
 pub(crate) mod shape_get_federation_token_input;

+#[cfg(feature = "op_get_session_token")]
 pub(crate) mod shape_get_session_token;

+#[cfg(feature = "op_get_session_token")]
 pub(crate) mod shape_get_session_token_input;

+#[cfg(feature = "op_get_web_identity_token")]
 pub(crate) mod shape_get_web_identity_token;

+#[cfg(feature = "op_get_web_identity_token")]
 pub(crate) mod shape_get_web_identity_token_input;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_assume_root"
+))]
 pub(crate) mod shape_expired_token_exception;

+#[cfg(feature = "op_get_delegated_access_token")]
 pub(crate) mod shape_expired_trade_in_token_exception;

+#[cfg(feature = "op_assume_role_with_web_identity")]
 pub(crate) mod shape_idp_communication_error_exception;

+#[cfg(any(feature = "op_assume_role_with_saml", feature = "op_assume_role_with_web_identity"))]
 pub(crate) mod shape_idp_rejected_claim_exception;

+#[cfg(feature = "op_decode_authorization_message")]
 pub(crate) mod shape_invalid_authorization_message_exception;

+#[cfg(any(feature = "op_assume_role_with_saml", feature = "op_assume_role_with_web_identity"))]
 pub(crate) mod shape_invalid_identity_token_exception;

+#[cfg(feature = "op_get_web_identity_token")]
 pub(crate) mod shape_jwt_payload_size_exceeded_exception;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_get_federation_token"
+))]
 pub(crate) mod shape_malformed_policy_document_exception;

+#[cfg(feature = "op_get_web_identity_token")]
 pub(crate) mod shape_outbound_web_identity_federation_disabled_exception;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_get_delegated_access_token",
+    feature = "op_get_federation_token"
+))]
 pub(crate) mod shape_packed_policy_too_large_exception;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_assume_root",
+    feature = "op_get_federation_token"
+))]
 pub(crate) mod shape_policy_descriptor_type;

+#[cfg(feature = "op_assume_role")]
 pub(crate) mod shape_provided_context;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_assume_root",
+    feature = "op_get_delegated_access_token",
+    feature = "op_get_federation_token",
+    feature = "op_get_session_token"
+))]
 pub(crate) mod shape_region_disabled_exception;

+#[cfg(feature = "op_get_web_identity_token")]
 pub(crate) mod shape_session_duration_escalation_exception;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_get_federation_token",
+    feature = "op_get_web_identity_token"
+))]
 pub(crate) mod shape_tag;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity"
+))]
 pub(crate) mod shape_assumed_role_user;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_assume_root",
+    feature = "op_get_delegated_access_token",
+    feature = "op_get_federation_token",
+    feature = "op_get_session_token"
+))]
 pub(crate) mod shape_credentials;

+#[cfg(feature = "op_get_federation_token")]
 pub(crate) mod shape_federated_user;
```

### `src/protocol_serde/shape_assume_role.rs`

```diff
diff --git a/src/protocol_serde/shape_assume_role.rs b/src/protocol_serde/shape_assume_role.rs
--- a/src/protocol_serde/shape_assume_role.rs
+++ b/src/protocol_serde/shape_assume_role.rs
@@ -4,10 +4,12 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<crate::operation::assume_role::AssumeRoleOutput, crate::operation::assume_role::AssumeRoleError> {
+) -> std::result::Result<crate::operation::assume_role::AssumeRoleOutput, crate::operation::assume_role::AssumeRoleError>
+{
     #[allow(unused_mut)]
-    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(crate::operation::assume_role::AssumeRoleError::unhandled)?;
+    let mut generic_builder =
+        crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
+            .map_err(crate::operation::assume_role::AssumeRoleError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
@@ -22,8 +24,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = crate::types::error::builders::ExpiredTokenExceptionBuilder::default();
-                output = crate::protocol_serde::shape_expired_token_exception::de_expired_token_exception_xml_err(_response_body, output)
-                    .map_err(crate::operation::assume_role::AssumeRoleError::unhandled)?;
+                output = crate::protocol_serde::shape_expired_token_exception::de_expired_token_exception_xml_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(crate::operation::assume_role::AssumeRoleError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -32,24 +37,26 @@
             }
             tmp
         }),
-        "MalformedPolicyDocument" => crate::operation::assume_role::AssumeRoleError::MalformedPolicyDocumentException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "MalformedPolicyDocument" => {
+            crate::operation::assume_role::AssumeRoleError::MalformedPolicyDocumentException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::MalformedPolicyDocumentExceptionBuilder::default();
-                output = crate::protocol_serde::shape_malformed_policy_document_exception::de_malformed_policy_document_exception_xml_err(
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::MalformedPolicyDocumentExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_malformed_policy_document_exception::de_malformed_policy_document_exception_xml_err(
                     _response_body,
                     output,
                 )
                 .map_err(crate::operation::assume_role::AssumeRoleError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
         "PackedPolicyTooLarge" => crate::operation::assume_role::AssumeRoleError::PackedPolicyTooLargeException({
             #[allow(unused_mut)]
             let mut tmp = {
@@ -73,8 +80,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = crate::types::error::builders::RegionDisabledExceptionBuilder::default();
-                output = crate::protocol_serde::shape_region_disabled_exception::de_region_disabled_exception_xml_err(_response_body, output)
-                    .map_err(crate::operation::assume_role::AssumeRoleError::unhandled)?;
+                output = crate::protocol_serde::shape_region_disabled_exception::de_region_disabled_exception_xml_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(crate::operation::assume_role::AssumeRoleError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -92,7 +102,8 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<crate::operation::assume_role::AssumeRoleOutput, crate::operation::assume_role::AssumeRoleError> {
+) -> std::result::Result<crate::operation::assume_role::AssumeRoleOutput, crate::operation::assume_role::AssumeRoleError>
+{
     Ok({
         #[allow(unused_mut)]
         let mut output = crate::operation::assume_role::builders::AssumeRoleOutputBuilder::default();
@@ -107,7 +118,10 @@
 pub fn de_assume_role(
     inp: &[u8],
     mut builder: crate::operation::assume_role::builders::AssumeRoleOutputBuilder,
-) -> std::result::Result<crate::operation::assume_role::builders::AssumeRoleOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
+) -> std::result::Result<
+    crate::operation::assume_role::builders::AssumeRoleOutputBuilder,
+    ::aws_smithy_xml::decode::XmlDecodeError,
+> {
     let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

     #[allow(unused_mut)]
@@ -182,7 +196,9 @@
         }
         }
     } else {
-        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom("expected AssumeRoleResult tag"));
+        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(
+            "expected AssumeRoleResult tag",
+        ));
     };
     Ok(builder)
 }
```

### `src/protocol_serde/shape_assume_role_input.rs`

```diff
diff --git a/src/protocol_serde/shape_assume_role_input.rs b/src/protocol_serde/shape_assume_role_input.rs
--- a/src/protocol_serde/shape_assume_role_input.rs
+++ b/src/protocol_serde/shape_assume_role_input.rs
@@ -1,7 +1,8 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub fn ser_assume_role_input_input_input(
     input: &crate::operation::assume_role::AssumeRoleInput,
-) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError>
+{
     let mut out = String::new();
     #[allow(unused_mut)]
     let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "AssumeRole", "2011-06-15");
```

### `src/protocol_serde/shape_assume_role_with_saml.rs`

```diff
diff --git a/src/protocol_serde/shape_assume_role_with_saml.rs b/src/protocol_serde/shape_assume_role_with_saml.rs
--- a/src/protocol_serde/shape_assume_role_with_saml.rs
+++ b/src/protocol_serde/shape_assume_role_with_saml.rs
@@ -9,8 +9,9 @@
     crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError,
 > {
     #[allow(unused_mut)]
-    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::unhandled)?;
+    let mut generic_builder =
+        crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
+            .map_err(crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
@@ -20,105 +21,124 @@

     let _error_message = generic.message().map(|msg| msg.to_owned());
     Err(match error_code {
-        "ExpiredTokenException" => crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::ExpiredTokenException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "ExpiredTokenException" => {
+            crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::ExpiredTokenException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::ExpiredTokenExceptionBuilder::default();
-                output = crate::protocol_serde::shape_expired_token_exception::de_expired_token_exception_xml_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::ExpiredTokenExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_expired_token_exception::de_expired_token_exception_xml_err(
+                        _response_body,
+                        output,
+                    )
                     .map_err(crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "IDPRejectedClaim" => crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::IdpRejectedClaimException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
+        "IDPRejectedClaim" => {
+            crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::IdpRejectedClaimException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::IdpRejectedClaimExceptionBuilder::default();
-                output = crate::protocol_serde::shape_idp_rejected_claim_exception::de_idp_rejected_claim_exception_xml_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::IdpRejectedClaimExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_idp_rejected_claim_exception::de_idp_rejected_claim_exception_xml_err(_response_body, output)
                     .map_err(crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "InvalidIdentityToken" => crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::InvalidIdentityTokenException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
+        "InvalidIdentityToken" => {
+            crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::InvalidIdentityTokenException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::InvalidIdentityTokenExceptionBuilder::default();
-                output = crate::protocol_serde::shape_invalid_identity_token_exception::de_invalid_identity_token_exception_xml_err(
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::InvalidIdentityTokenExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_invalid_identity_token_exception::de_invalid_identity_token_exception_xml_err(
                     _response_body,
                     output,
                 )
                 .map_err(crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "MalformedPolicyDocument" => crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::MalformedPolicyDocumentException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
+        "MalformedPolicyDocument" => {
+            crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::MalformedPolicyDocumentException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::MalformedPolicyDocumentExceptionBuilder::default();
-                output = crate::protocol_serde::shape_malformed_policy_document_exception::de_malformed_policy_document_exception_xml_err(
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::MalformedPolicyDocumentExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_malformed_policy_document_exception::de_malformed_policy_document_exception_xml_err(
                     _response_body,
                     output,
                 )
                 .map_err(crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "PackedPolicyTooLarge" => crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::PackedPolicyTooLargeException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
+        "PackedPolicyTooLarge" => {
+            crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::PackedPolicyTooLargeException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::PackedPolicyTooLargeExceptionBuilder::default();
-                output = crate::protocol_serde::shape_packed_policy_too_large_exception::de_packed_policy_too_large_exception_xml_err(
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::PackedPolicyTooLargeExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_packed_policy_too_large_exception::de_packed_policy_too_large_exception_xml_err(
                     _response_body,
                     output,
                 )
                 .map_err(crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "RegionDisabledException" => crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::RegionDisabledException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
+        "RegionDisabledException" => {
+            crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::RegionDisabledException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::RegionDisabledExceptionBuilder::default();
-                output = crate::protocol_serde::shape_region_disabled_exception::de_region_disabled_exception_xml_err(_response_body, output)
-                    .map_err(crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::RegionDisabledExceptionBuilder::default();
+                    output =
+                        crate::protocol_serde::shape_region_disabled_exception::de_region_disabled_exception_xml_err(
+                            _response_body,
+                            output,
+                        )
+                        .map_err(crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::unhandled)?;
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
         _ => crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::generic(generic),
     })
 }
@@ -146,8 +166,10 @@
 pub fn de_assume_role_with_saml(
     inp: &[u8],
     mut builder: crate::operation::assume_role_with_saml::builders::AssumeRoleWithSamlOutputBuilder,
-) -> std::result::Result<crate::operation::assume_role_with_saml::builders::AssumeRoleWithSamlOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError>
-{
+) -> std::result::Result<
+    crate::operation::assume_role_with_saml::builders::AssumeRoleWithSamlOutputBuilder,
+    ::aws_smithy_xml::decode::XmlDecodeError,
+> {
     let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

     #[allow(unused_mut)]
@@ -287,7 +309,9 @@
         }
         }
     } else {
-        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom("expected AssumeRoleWithSAMLResult tag"));
+        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(
+            "expected AssumeRoleWithSAMLResult tag",
+        ));
     };
     Ok(builder)
 }
```

### `src/protocol_serde/shape_assume_role_with_saml_input.rs`

```diff
diff --git a/src/protocol_serde/shape_assume_role_with_saml_input.rs b/src/protocol_serde/shape_assume_role_with_saml_input.rs
--- a/src/protocol_serde/shape_assume_role_with_saml_input.rs
+++ b/src/protocol_serde/shape_assume_role_with_saml_input.rs
@@ -1,7 +1,8 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub fn ser_assume_role_with_saml_input_input_input(
     input: &crate::operation::assume_role_with_saml::AssumeRoleWithSamlInput,
-) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError>
+{
     let mut out = String::new();
     #[allow(unused_mut)]
     let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "AssumeRoleWithSAML", "2011-06-15");
```

### `src/protocol_serde/shape_assume_role_with_web_identity.rs`

```diff
diff --git a/src/protocol_serde/shape_assume_role_with_web_identity.rs b/src/protocol_serde/shape_assume_role_with_web_identity.rs
--- a/src/protocol_serde/shape_assume_role_with_web_identity.rs
+++ b/src/protocol_serde/shape_assume_role_with_web_identity.rs
@@ -9,13 +9,18 @@
     crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError,
 > {
     #[allow(unused_mut)]
-    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError::unhandled)?;
+    let mut generic_builder =
+        crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
+            .map_err(crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError::unhandled(generic)),
+        None => {
+            return Err(
+                crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError::unhandled(generic),
+            )
+        }
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
@@ -154,9 +159,14 @@
 > {
     Ok({
         #[allow(unused_mut)]
-        let mut output = crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityOutputBuilder::default();
-        output = crate::protocol_serde::shape_assume_role_with_web_identity::de_assume_role_with_web_identity(_response_body, output)
-            .map_err(crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError::unhandled)?;
+        let mut output =
+            crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityOutputBuilder::default(
+            );
+        output = crate::protocol_serde::shape_assume_role_with_web_identity::de_assume_role_with_web_identity(
+            _response_body,
+            output,
+        )
+        .map_err(crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
         output.build()
     })
```

### `src/protocol_serde/shape_assume_role_with_web_identity_input.rs`

```diff
diff --git a/src/protocol_serde/shape_assume_role_with_web_identity_input.rs b/src/protocol_serde/shape_assume_role_with_web_identity_input.rs
--- a/src/protocol_serde/shape_assume_role_with_web_identity_input.rs
+++ b/src/protocol_serde/shape_assume_role_with_web_identity_input.rs
@@ -1,7 +1,8 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub fn ser_assume_role_with_web_identity_input_input_input(
     input: &crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityInput,
-) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError>
+{
     let mut out = String::new();
     #[allow(unused_mut)]
     let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "AssumeRoleWithWebIdentity", "2011-06-15");
```

### `src/protocol_serde/shape_assume_root.rs`

```diff
diff --git a/src/protocol_serde/shape_assume_root.rs b/src/protocol_serde/shape_assume_root.rs
--- a/src/protocol_serde/shape_assume_root.rs
+++ b/src/protocol_serde/shape_assume_root.rs
@@ -4,10 +4,12 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<crate::operation::assume_root::AssumeRootOutput, crate::operation::assume_root::AssumeRootError> {
+) -> std::result::Result<crate::operation::assume_root::AssumeRootOutput, crate::operation::assume_root::AssumeRootError>
+{
     #[allow(unused_mut)]
-    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(crate::operation::assume_root::AssumeRootError::unhandled)?;
+    let mut generic_builder =
+        crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
+            .map_err(crate::operation::assume_root::AssumeRootError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
@@ -22,8 +24,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = crate::types::error::builders::ExpiredTokenExceptionBuilder::default();
-                output = crate::protocol_serde::shape_expired_token_exception::de_expired_token_exception_xml_err(_response_body, output)
-                    .map_err(crate::operation::assume_root::AssumeRootError::unhandled)?;
+                output = crate::protocol_serde::shape_expired_token_exception::de_expired_token_exception_xml_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(crate::operation::assume_root::AssumeRootError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -37,8 +42,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = crate::types::error::builders::RegionDisabledExceptionBuilder::default();
-                output = crate::protocol_serde::shape_region_disabled_exception::de_region_disabled_exception_xml_err(_response_body, output)
-                    .map_err(crate::operation::assume_root::AssumeRootError::unhandled)?;
+                output = crate::protocol_serde::shape_region_disabled_exception::de_region_disabled_exception_xml_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(crate::operation::assume_root::AssumeRootError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -56,7 +64,8 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<crate::operation::assume_root::AssumeRootOutput, crate::operation::assume_root::AssumeRootError> {
+) -> std::result::Result<crate::operation::assume_root::AssumeRootOutput, crate::operation::assume_root::AssumeRootError>
+{
     Ok({
         #[allow(unused_mut)]
         let mut output = crate::operation::assume_root::builders::AssumeRootOutputBuilder::default();
@@ -71,7 +80,10 @@
 pub fn de_assume_root(
     inp: &[u8],
     mut builder: crate::operation::assume_root::builders::AssumeRootOutputBuilder,
-) -> std::result::Result<crate::operation::assume_root::builders::AssumeRootOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
+) -> std::result::Result<
+    crate::operation::assume_root::builders::AssumeRootOutputBuilder,
+    ::aws_smithy_xml::decode::XmlDecodeError,
+> {
     let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

     #[allow(unused_mut)]
@@ -121,7 +133,9 @@
         }
         }
     } else {
-        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom("expected AssumeRootResult tag"));
+        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(
+            "expected AssumeRootResult tag",
+        ));
     };
     Ok(builder)
 }
```

### `src/protocol_serde/shape_assume_root_input.rs`

```diff
diff --git a/src/protocol_serde/shape_assume_root_input.rs b/src/protocol_serde/shape_assume_root_input.rs
--- a/src/protocol_serde/shape_assume_root_input.rs
+++ b/src/protocol_serde/shape_assume_root_input.rs
@@ -1,7 +1,8 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub fn ser_assume_root_input_input_input(
     input: &crate::operation::assume_root::AssumeRootInput,
-) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError>
+{
     let mut out = String::new();
     #[allow(unused_mut)]
     let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "AssumeRoot", "2011-06-15");
```

### `src/protocol_serde/shape_assumed_role_user.rs`

```diff
diff --git a/src/protocol_serde/shape_assumed_role_user.rs b/src/protocol_serde/shape_assumed_role_user.rs
--- a/src/protocol_serde/shape_assumed_role_user.rs
+++ b/src/protocol_serde/shape_assumed_role_user.rs
@@ -5,7 +5,9 @@
     depth: u32,
 ) -> ::std::result::Result<crate::types::AssumedRoleUser, ::aws_smithy_xml::decode::XmlDecodeError> {
     if depth >= 128u32 {
-        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom("maximum nesting depth exceeded"));
+        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(
+            "maximum nesting depth exceeded",
+        ));
     }
     #[allow(unused_mut)]
     let mut builder = crate::types::AssumedRoleUser::builder();
```

### `src/protocol_serde/shape_credentials.rs`

```diff
diff --git a/src/protocol_serde/shape_credentials.rs b/src/protocol_serde/shape_credentials.rs
--- a/src/protocol_serde/shape_credentials.rs
+++ b/src/protocol_serde/shape_credentials.rs
@@ -5,7 +5,9 @@
     depth: u32,
 ) -> ::std::result::Result<crate::types::Credentials, ::aws_smithy_xml::decode::XmlDecodeError> {
     if depth >= 128u32 {
-        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom("maximum nesting depth exceeded"));
+        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(
+            "maximum nesting depth exceeded",
+        ));
     }
     #[allow(unused_mut)]
     let mut builder = crate::types::Credentials::builder();
```

### `src/protocol_serde/shape_decode_authorization_message.rs`

```diff
diff --git a/src/protocol_serde/shape_decode_authorization_message.rs b/src/protocol_serde/shape_decode_authorization_message.rs
--- a/src/protocol_serde/shape_decode_authorization_message.rs
+++ b/src/protocol_serde/shape_decode_authorization_message.rs
@@ -9,13 +9,18 @@
     crate::operation::decode_authorization_message::DecodeAuthorizationMessageError,
 > {
     #[allow(unused_mut)]
-    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(crate::operation::decode_authorization_message::DecodeAuthorizationMessageError::unhandled)?;
+    let mut generic_builder =
+        crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
+            .map_err(crate::operation::decode_authorization_message::DecodeAuthorizationMessageError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(crate::operation::decode_authorization_message::DecodeAuthorizationMessageError::unhandled(generic)),
+        None => {
+            return Err(
+                crate::operation::decode_authorization_message::DecodeAuthorizationMessageError::unhandled(generic),
+            )
+        }
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
@@ -56,9 +61,14 @@
 > {
     Ok({
         #[allow(unused_mut)]
-        let mut output = crate::operation::decode_authorization_message::builders::DecodeAuthorizationMessageOutputBuilder::default();
-        output = crate::protocol_serde::shape_decode_authorization_message::de_decode_authorization_message(_response_body, output)
-            .map_err(crate::operation::decode_authorization_message::DecodeAuthorizationMessageError::unhandled)?;
+        let mut output =
+            crate::operation::decode_authorization_message::builders::DecodeAuthorizationMessageOutputBuilder::default(
+            );
+        output = crate::protocol_serde::shape_decode_authorization_message::de_decode_authorization_message(
+            _response_body,
+            output,
+        )
+        .map_err(crate::operation::decode_authorization_message::DecodeAuthorizationMessageError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
         output.build()
     })
```

### `src/protocol_serde/shape_decode_authorization_message_input.rs`

```diff
diff --git a/src/protocol_serde/shape_decode_authorization_message_input.rs b/src/protocol_serde/shape_decode_authorization_message_input.rs
--- a/src/protocol_serde/shape_decode_authorization_message_input.rs
+++ b/src/protocol_serde/shape_decode_authorization_message_input.rs
@@ -1,7 +1,8 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub fn ser_decode_authorization_message_input_input_input(
     input: &crate::operation::decode_authorization_message::DecodeAuthorizationMessageInput,
-) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError>
+{
     let mut out = String::new();
     #[allow(unused_mut)]
     let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "DecodeAuthorizationMessage", "2011-06-15");
```

### `src/protocol_serde/shape_expired_token_exception.rs`

```diff
diff --git a/src/protocol_serde/shape_expired_token_exception.rs b/src/protocol_serde/shape_expired_token_exception.rs
--- a/src/protocol_serde/shape_expired_token_exception.rs
+++ b/src/protocol_serde/shape_expired_token_exception.rs
@@ -3,7 +3,10 @@
 pub fn de_expired_token_exception_xml_err(
     inp: &[u8],
     mut builder: crate::types::error::builders::ExpiredTokenExceptionBuilder,
-) -> std::result::Result<crate::types::error::builders::ExpiredTokenExceptionBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
+) -> std::result::Result<
+    crate::types::error::builders::ExpiredTokenExceptionBuilder,
+    ::aws_smithy_xml::decode::XmlDecodeError,
+> {
     if inp.is_empty() {
         return Ok(builder);
     }
```

### `src/protocol_serde/shape_expired_trade_in_token_exception.rs`

```diff
diff --git a/src/protocol_serde/shape_expired_trade_in_token_exception.rs b/src/protocol_serde/shape_expired_trade_in_token_exception.rs
--- a/src/protocol_serde/shape_expired_trade_in_token_exception.rs
+++ b/src/protocol_serde/shape_expired_trade_in_token_exception.rs
@@ -3,7 +3,10 @@
 pub fn de_expired_trade_in_token_exception_xml_err(
     inp: &[u8],
     mut builder: crate::types::error::builders::ExpiredTradeInTokenExceptionBuilder,
-) -> std::result::Result<crate::types::error::builders::ExpiredTradeInTokenExceptionBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
+) -> std::result::Result<
+    crate::types::error::builders::ExpiredTradeInTokenExceptionBuilder,
+    ::aws_smithy_xml::decode::XmlDecodeError,
+> {
     if inp.is_empty() {
         return Ok(builder);
     }
```

### `src/protocol_serde/shape_federated_user.rs`

```diff
diff --git a/src/protocol_serde/shape_federated_user.rs b/src/protocol_serde/shape_federated_user.rs
--- a/src/protocol_serde/shape_federated_user.rs
+++ b/src/protocol_serde/shape_federated_user.rs
@@ -5,7 +5,9 @@
     depth: u32,
 ) -> ::std::result::Result<crate::types::FederatedUser, ::aws_smithy_xml::decode::XmlDecodeError> {
     if depth >= 128u32 {
-        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom("maximum nesting depth exceeded"));
+        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(
+            "maximum nesting depth exceeded",
+        ));
     }
     #[allow(unused_mut)]
     let mut builder = crate::types::FederatedUser::builder();
```

### `src/protocol_serde/shape_get_access_key_info.rs`

```diff
diff --git a/src/protocol_serde/shape_get_access_key_info.rs b/src/protocol_serde/shape_get_access_key_info.rs
--- a/src/protocol_serde/shape_get_access_key_info.rs
+++ b/src/protocol_serde/shape_get_access_key_info.rs
@@ -4,14 +4,19 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<crate::operation::get_access_key_info::GetAccessKeyInfoOutput, crate::operation::get_access_key_info::GetAccessKeyInfoError>
-{
+) -> std::result::Result<
+    crate::operation::get_access_key_info::GetAccessKeyInfoOutput,
+    crate::operation::get_access_key_info::GetAccessKeyInfoError,
+> {
     #[allow(unused_mut)]
-    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(crate::operation::get_access_key_info::GetAccessKeyInfoError::unhandled)?;
+    let mut generic_builder =
+        crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
+            .map_err(crate::operation::get_access_key_info::GetAccessKeyInfoError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
-    Err(crate::operation::get_access_key_info::GetAccessKeyInfoError::generic(generic))
+    Err(crate::operation::get_access_key_info::GetAccessKeyInfoError::generic(
+        generic,
+    ))
 }

 #[allow(clippy::unnecessary_wraps)]
@@ -19,8 +24,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<crate::operation::get_access_key_info::GetAccessKeyInfoOutput, crate::operation::get_access_key_info::GetAccessKeyInfoError>
-{
+) -> std::result::Result<
+    crate::operation::get_access_key_info::GetAccessKeyInfoOutput,
+    crate::operation::get_access_key_info::GetAccessKeyInfoError,
+> {
     Ok({
         #[allow(unused_mut)]
         let mut output = crate::operation::get_access_key_info::builders::GetAccessKeyInfoOutputBuilder::default();
@@ -35,7 +42,10 @@
 pub fn de_get_access_key_info(
     inp: &[u8],
     mut builder: crate::operation::get_access_key_info::builders::GetAccessKeyInfoOutputBuilder,
-) -> std::result::Result<crate::operation::get_access_key_info::builders::GetAccessKeyInfoOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
+) -> std::result::Result<
+    crate::operation::get_access_key_info::builders::GetAccessKeyInfoOutputBuilder,
+    ::aws_smithy_xml::decode::XmlDecodeError,
+> {
     let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

     #[allow(unused_mut)]
@@ -75,7 +85,9 @@
         }
         }
     } else {
-        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom("expected GetAccessKeyInfoResult tag"));
+        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(
+            "expected GetAccessKeyInfoResult tag",
+        ));
     };
     Ok(builder)
 }
```

### `src/protocol_serde/shape_get_access_key_info_input.rs`

```diff
diff --git a/src/protocol_serde/shape_get_access_key_info_input.rs b/src/protocol_serde/shape_get_access_key_info_input.rs
--- a/src/protocol_serde/shape_get_access_key_info_input.rs
+++ b/src/protocol_serde/shape_get_access_key_info_input.rs
@@ -1,7 +1,8 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub fn ser_get_access_key_info_input_input_input(
     input: &crate::operation::get_access_key_info::GetAccessKeyInfoInput,
-) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError>
+{
     let mut out = String::new();
     #[allow(unused_mut)]
     let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "GetAccessKeyInfo", "2011-06-15");
```

### `src/protocol_serde/shape_get_caller_identity.rs`

```diff
diff --git a/src/protocol_serde/shape_get_caller_identity.rs b/src/protocol_serde/shape_get_caller_identity.rs
--- a/src/protocol_serde/shape_get_caller_identity.rs
+++ b/src/protocol_serde/shape_get_caller_identity.rs
@@ -4,14 +4,19 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<crate::operation::get_caller_identity::GetCallerIdentityOutput, crate::operation::get_caller_identity::GetCallerIdentityError>
-{
+) -> std::result::Result<
+    crate::operation::get_caller_identity::GetCallerIdentityOutput,
+    crate::operation::get_caller_identity::GetCallerIdentityError,
+> {
     #[allow(unused_mut)]
-    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(crate::operation::get_caller_identity::GetCallerIdentityError::unhandled)?;
+    let mut generic_builder =
+        crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
+            .map_err(crate::operation::get_caller_identity::GetCallerIdentityError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
-    Err(crate::operation::get_caller_identity::GetCallerIdentityError::generic(generic))
+    Err(crate::operation::get_caller_identity::GetCallerIdentityError::generic(
+        generic,
+    ))
 }

 #[allow(clippy::unnecessary_wraps)]
@@ -19,8 +24,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<crate::operation::get_caller_identity::GetCallerIdentityOutput, crate::operation::get_caller_identity::GetCallerIdentityError>
-{
+) -> std::result::Result<
+    crate::operation::get_caller_identity::GetCallerIdentityOutput,
+    crate::operation::get_caller_identity::GetCallerIdentityError,
+> {
     Ok({
         #[allow(unused_mut)]
         let mut output = crate::operation::get_caller_identity::builders::GetCallerIdentityOutputBuilder::default();
@@ -35,7 +42,10 @@
 pub fn de_get_caller_identity(
     inp: &[u8],
     mut builder: crate::operation::get_caller_identity::builders::GetCallerIdentityOutputBuilder,
-) -> std::result::Result<crate::operation::get_caller_identity::builders::GetCallerIdentityOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
+) -> std::result::Result<
+    crate::operation::get_caller_identity::builders::GetCallerIdentityOutputBuilder,
+    ::aws_smithy_xml::decode::XmlDecodeError,
+> {
     let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

     #[allow(unused_mut)]
@@ -101,7 +111,9 @@
         }
         }
     } else {
-        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom("expected GetCallerIdentityResult tag"));
+        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(
+            "expected GetCallerIdentityResult tag",
+        ));
     };
     Ok(builder)
 }
```

### `src/protocol_serde/shape_get_caller_identity_input.rs`

```diff
diff --git a/src/protocol_serde/shape_get_caller_identity_input.rs b/src/protocol_serde/shape_get_caller_identity_input.rs
--- a/src/protocol_serde/shape_get_caller_identity_input.rs
+++ b/src/protocol_serde/shape_get_caller_identity_input.rs
@@ -1,7 +1,8 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub fn ser_get_caller_identity_input_input_input(
     input: &crate::operation::get_caller_identity::GetCallerIdentityInput,
-) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError>
+{
     let _ = input;
     let mut out = String::new();
     #[allow(unused_mut)]
```

### `src/protocol_serde/shape_get_delegated_access_token.rs`

```diff
diff --git a/src/protocol_serde/shape_get_delegated_access_token.rs b/src/protocol_serde/shape_get_delegated_access_token.rs
--- a/src/protocol_serde/shape_get_delegated_access_token.rs
+++ b/src/protocol_serde/shape_get_delegated_access_token.rs
@@ -9,72 +9,83 @@
     crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError,
 > {
     #[allow(unused_mut)]
-    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError::unhandled)?;
+    let mut generic_builder =
+        crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
+            .map_err(crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
         None => {
-            return Err(crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError::unhandled(
-                generic,
-            ))
+            return Err(crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError::unhandled(generic))
         }
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
     Err(match error_code {
-        "ExpiredTradeInTokenException" => crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError::ExpiredTradeInTokenException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "ExpiredTradeInTokenException" => {
+            crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError::ExpiredTradeInTokenException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::ExpiredTradeInTokenExceptionBuilder::default();
-                output = crate::protocol_serde::shape_expired_trade_in_token_exception::de_expired_trade_in_token_exception_xml_err(
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::ExpiredTradeInTokenExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_expired_trade_in_token_exception::de_expired_trade_in_token_exception_xml_err(
                     _response_body,
                     output,
                 )
                 .map_err(crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "PackedPolicyTooLarge" => crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError::PackedPolicyTooLargeException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
+        "PackedPolicyTooLarge" => {
+            crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError::PackedPolicyTooLargeException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::PackedPolicyTooLargeExceptionBuilder::default();
-                output = crate::protocol_serde::shape_packed_policy_too_large_exception::de_packed_policy_too_large_exception_xml_err(
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::PackedPolicyTooLargeExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_packed_policy_too_large_exception::de_packed_policy_too_large_exception_xml_err(
                     _response_body,
                     output,
                 )
                 .map_err(crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "RegionDisabledException" => crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError::RegionDisabledException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
+        "RegionDisabledException" => {
+            crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError::RegionDisabledException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::RegionDisabledExceptionBuilder::default();
-                output = crate::protocol_serde::shape_region_disabled_exception::de_region_disabled_exception_xml_err(_response_body, output)
-                    .map_err(crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::RegionDisabledExceptionBuilder::default();
+                    output =
+                        crate::protocol_serde::shape_region_disabled_exception::de_region_disabled_exception_xml_err(
+                            _response_body,
+                            output,
+                        )
+                        .map_err(
+                            crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError::unhandled,
+                        )?;
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
         _ => crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError::generic(generic),
     })
 }
@@ -90,9 +101,13 @@
 > {
     Ok({
         #[allow(unused_mut)]
-        let mut output = crate::operation::get_delegated_access_token::builders::GetDelegatedAccessTokenOutputBuilder::default();
-        output = crate::protocol_serde::shape_get_delegated_access_token::de_get_delegated_access_token(_response_body, output)
-            .map_err(crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError::unhandled)?;
+        let mut output =
+            crate::operation::get_delegated_access_token::builders::GetDelegatedAccessTokenOutputBuilder::default();
+        output = crate::protocol_serde::shape_get_delegated_access_token::de_get_delegated_access_token(
+            _response_body,
+            output,
+        )
+        .map_err(crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
         output.build()
     })
```

### `src/protocol_serde/shape_get_delegated_access_token_input.rs`

```diff
diff --git a/src/protocol_serde/shape_get_delegated_access_token_input.rs b/src/protocol_serde/shape_get_delegated_access_token_input.rs
--- a/src/protocol_serde/shape_get_delegated_access_token_input.rs
+++ b/src/protocol_serde/shape_get_delegated_access_token_input.rs
@@ -1,7 +1,8 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub fn ser_get_delegated_access_token_input_input_input(
     input: &crate::operation::get_delegated_access_token::GetDelegatedAccessTokenInput,
-) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError>
+{
     let mut out = String::new();
     #[allow(unused_mut)]
     let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "GetDelegatedAccessToken", "2011-06-15");
```

### `src/protocol_serde/shape_get_federation_token.rs`

```diff
diff --git a/src/protocol_serde/shape_get_federation_token.rs b/src/protocol_serde/shape_get_federation_token.rs
--- a/src/protocol_serde/shape_get_federation_token.rs
+++ b/src/protocol_serde/shape_get_federation_token.rs
@@ -9,8 +9,9 @@
     crate::operation::get_federation_token::GetFederationTokenError,
 > {
     #[allow(unused_mut)]
-    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(crate::operation::get_federation_token::GetFederationTokenError::unhandled)?;
+    let mut generic_builder =
+        crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
+            .map_err(crate::operation::get_federation_token::GetFederationTokenError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
@@ -20,57 +21,67 @@

     let _error_message = generic.message().map(|msg| msg.to_owned());
     Err(match error_code {
-        "MalformedPolicyDocument" => crate::operation::get_federation_token::GetFederationTokenError::MalformedPolicyDocumentException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "MalformedPolicyDocument" => {
+            crate::operation::get_federation_token::GetFederationTokenError::MalformedPolicyDocumentException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::MalformedPolicyDocumentExceptionBuilder::default();
-                output = crate::protocol_serde::shape_malformed_policy_document_exception::de_malformed_policy_document_exception_xml_err(
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::MalformedPolicyDocumentExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_malformed_policy_document_exception::de_malformed_policy_document_exception_xml_err(
                     _response_body,
                     output,
                 )
                 .map_err(crate::operation::get_federation_token::GetFederationTokenError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "PackedPolicyTooLarge" => crate::operation::get_federation_token::GetFederationTokenError::PackedPolicyTooLargeException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
+        "PackedPolicyTooLarge" => {
+            crate::operation::get_federation_token::GetFederationTokenError::PackedPolicyTooLargeException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::PackedPolicyTooLargeExceptionBuilder::default();
-                output = crate::protocol_serde::shape_packed_policy_too_large_exception::de_packed_policy_too_large_exception_xml_err(
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::PackedPolicyTooLargeExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_packed_policy_too_large_exception::de_packed_policy_too_large_exception_xml_err(
                     _response_body,
                     output,
                 )
                 .map_err(crate::operation::get_federation_token::GetFederationTokenError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "RegionDisabledException" => crate::operation::get_federation_token::GetFederationTokenError::RegionDisabledException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
+        "RegionDisabledException" => {
+            crate::operation::get_federation_token::GetFederationTokenError::RegionDisabledException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::RegionDisabledExceptionBuilder::default();
-                output = crate::protocol_serde::shape_region_disabled_exception::de_region_disabled_exception_xml_err(_response_body, output)
-                    .map_err(crate::operation::get_federation_token::GetFederationTokenError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::RegionDisabledExceptionBuilder::default();
+                    output =
+                        crate::protocol_serde::shape_region_disabled_exception::de_region_disabled_exception_xml_err(
+                            _response_body,
+                            output,
+                        )
+                        .map_err(crate::operation::get_federation_token::GetFederationTokenError::unhandled)?;
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
         _ => crate::operation::get_federation_token::GetFederationTokenError::generic(generic),
     })
 }
@@ -98,8 +109,10 @@
 pub fn de_get_federation_token(
     inp: &[u8],
     mut builder: crate::operation::get_federation_token::builders::GetFederationTokenOutputBuilder,
-) -> std::result::Result<crate::operation::get_federation_token::builders::GetFederationTokenOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError>
-{
+) -> std::result::Result<
+    crate::operation::get_federation_token::builders::GetFederationTokenOutputBuilder,
+    ::aws_smithy_xml::decode::XmlDecodeError,
+> {
     let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

     #[allow(unused_mut)]
@@ -161,7 +174,9 @@
         }
         }
     } else {
-        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom("expected GetFederationTokenResult tag"));
+        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(
+            "expected GetFederationTokenResult tag",
+        ));
     };
     Ok(builder)
 }
```

### `src/protocol_serde/shape_get_federation_token_input.rs`

```diff
diff --git a/src/protocol_serde/shape_get_federation_token_input.rs b/src/protocol_serde/shape_get_federation_token_input.rs
--- a/src/protocol_serde/shape_get_federation_token_input.rs
+++ b/src/protocol_serde/shape_get_federation_token_input.rs
@@ -1,7 +1,8 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub fn ser_get_federation_token_input_input_input(
     input: &crate::operation::get_federation_token::GetFederationTokenInput,
-) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError>
+{
     let mut out = String::new();
     #[allow(unused_mut)]
     let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "GetFederationToken", "2011-06-15");
```

### `src/protocol_serde/shape_get_session_token.rs`

```diff
diff --git a/src/protocol_serde/shape_get_session_token.rs b/src/protocol_serde/shape_get_session_token.rs
--- a/src/protocol_serde/shape_get_session_token.rs
+++ b/src/protocol_serde/shape_get_session_token.rs
@@ -4,34 +4,48 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<crate::operation::get_session_token::GetSessionTokenOutput, crate::operation::get_session_token::GetSessionTokenError> {
+) -> std::result::Result<
+    crate::operation::get_session_token::GetSessionTokenOutput,
+    crate::operation::get_session_token::GetSessionTokenError,
+> {
     #[allow(unused_mut)]
-    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(crate::operation::get_session_token::GetSessionTokenError::unhandled)?;
+    let mut generic_builder =
+        crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
+            .map_err(crate::operation::get_session_token::GetSessionTokenError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(crate::operation::get_session_token::GetSessionTokenError::unhandled(generic)),
+        None => {
+            return Err(crate::operation::get_session_token::GetSessionTokenError::unhandled(
+                generic,
+            ))
+        }
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
     Err(match error_code {
-        "RegionDisabledException" => crate::operation::get_session_token::GetSessionTokenError::RegionDisabledException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "RegionDisabledException" => {
+            crate::operation::get_session_token::GetSessionTokenError::RegionDisabledException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::RegionDisabledExceptionBuilder::default();
-                output = crate::protocol_serde::shape_region_disabled_exception::de_region_disabled_exception_xml_err(_response_body, output)
-                    .map_err(crate::operation::get_session_token::GetSessionTokenError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::RegionDisabledExceptionBuilder::default();
+                    output =
+                        crate::protocol_serde::shape_region_disabled_exception::de_region_disabled_exception_xml_err(
+                            _response_body,
+                            output,
+                        )
+                        .map_err(crate::operation::get_session_token::GetSessionTokenError::unhandled)?;
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
         _ => crate::operation::get_session_token::GetSessionTokenError::generic(generic),
     })
 }
@@ -41,7 +55,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<crate::operation::get_session_token::GetSessionTokenOutput, crate::operation::get_session_token::GetSessionTokenError> {
+) -> std::result::Result<
+    crate::operation::get_session_token::GetSessionTokenOutput,
+    crate::operation::get_session_token::GetSessionTokenError,
+> {
     Ok({
         #[allow(unused_mut)]
         let mut output = crate::operation::get_session_token::builders::GetSessionTokenOutputBuilder::default();
@@ -56,7 +73,10 @@
 pub fn de_get_session_token(
     inp: &[u8],
     mut builder: crate::operation::get_session_token::builders::GetSessionTokenOutputBuilder,
-) -> std::result::Result<crate::operation::get_session_token::builders::GetSessionTokenOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
+) -> std::result::Result<
+    crate::operation::get_session_token::builders::GetSessionTokenOutputBuilder,
+    ::aws_smithy_xml::decode::XmlDecodeError,
+> {
     let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

     #[allow(unused_mut)]
@@ -93,7 +113,9 @@
         }
         }
     } else {
-        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom("expected GetSessionTokenResult tag"));
+        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(
+            "expected GetSessionTokenResult tag",
+        ));
     };
     Ok(builder)
 }
```

### `src/protocol_serde/shape_get_session_token_input.rs`

```diff
diff --git a/src/protocol_serde/shape_get_session_token_input.rs b/src/protocol_serde/shape_get_session_token_input.rs
--- a/src/protocol_serde/shape_get_session_token_input.rs
+++ b/src/protocol_serde/shape_get_session_token_input.rs
@@ -1,7 +1,8 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub fn ser_get_session_token_input_input_input(
     input: &crate::operation::get_session_token::GetSessionTokenInput,
-) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError>
+{
     let mut out = String::new();
     #[allow(unused_mut)]
     let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "GetSessionToken", "2011-06-15");
```

### `src/protocol_serde/shape_get_web_identity_token.rs`

```diff
diff --git a/src/protocol_serde/shape_get_web_identity_token.rs b/src/protocol_serde/shape_get_web_identity_token.rs
--- a/src/protocol_serde/shape_get_web_identity_token.rs
+++ b/src/protocol_serde/shape_get_web_identity_token.rs
@@ -9,8 +9,9 @@
     crate::operation::get_web_identity_token::GetWebIdentityTokenError,
 > {
     #[allow(unused_mut)]
-    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(crate::operation::get_web_identity_token::GetWebIdentityTokenError::unhandled)?;
+    let mut generic_builder =
+        crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
+            .map_err(crate::operation::get_web_identity_token::GetWebIdentityTokenError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
@@ -89,7 +90,8 @@
 > {
     Ok({
         #[allow(unused_mut)]
-        let mut output = crate::operation::get_web_identity_token::builders::GetWebIdentityTokenOutputBuilder::default();
+        let mut output =
+            crate::operation::get_web_identity_token::builders::GetWebIdentityTokenOutputBuilder::default();
         output = crate::protocol_serde::shape_get_web_identity_token::de_get_web_identity_token(_response_body, output)
             .map_err(crate::operation::get_web_identity_token::GetWebIdentityTokenError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
@@ -101,8 +103,10 @@
 pub fn de_get_web_identity_token(
     inp: &[u8],
     mut builder: crate::operation::get_web_identity_token::builders::GetWebIdentityTokenOutputBuilder,
-) -> std::result::Result<crate::operation::get_web_identity_token::builders::GetWebIdentityTokenOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError>
-{
+) -> std::result::Result<
+    crate::operation::get_web_identity_token::builders::GetWebIdentityTokenOutputBuilder,
+    ::aws_smithy_xml::decode::XmlDecodeError,
+> {
     let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

     #[allow(unused_mut)]
@@ -156,7 +160,9 @@
         }
         }
     } else {
-        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom("expected GetWebIdentityTokenResult tag"));
+        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(
+            "expected GetWebIdentityTokenResult tag",
+        ));
     };
     Ok(builder)
 }
```

### `src/protocol_serde/shape_get_web_identity_token_input.rs`

```diff
diff --git a/src/protocol_serde/shape_get_web_identity_token_input.rs b/src/protocol_serde/shape_get_web_identity_token_input.rs
--- a/src/protocol_serde/shape_get_web_identity_token_input.rs
+++ b/src/protocol_serde/shape_get_web_identity_token_input.rs
@@ -1,7 +1,8 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub fn ser_get_web_identity_token_input_input_input(
     input: &crate::operation::get_web_identity_token::GetWebIdentityTokenInput,
-) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError>
+{
     let mut out = String::new();
     #[allow(unused_mut)]
     let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "GetWebIdentityToken", "2011-06-15");
```

### `src/protocol_serde/shape_idp_communication_error_exception.rs`

```diff
diff --git a/src/protocol_serde/shape_idp_communication_error_exception.rs b/src/protocol_serde/shape_idp_communication_error_exception.rs
--- a/src/protocol_serde/shape_idp_communication_error_exception.rs
+++ b/src/protocol_serde/shape_idp_communication_error_exception.rs
@@ -3,7 +3,10 @@
 pub fn de_idp_communication_error_exception_xml_err(
     inp: &[u8],
     mut builder: crate::types::error::builders::IdpCommunicationErrorExceptionBuilder,
-) -> std::result::Result<crate::types::error::builders::IdpCommunicationErrorExceptionBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
+) -> std::result::Result<
+    crate::types::error::builders::IdpCommunicationErrorExceptionBuilder,
+    ::aws_smithy_xml::decode::XmlDecodeError,
+> {
     if inp.is_empty() {
         return Ok(builder);
     }
```

### `src/protocol_serde/shape_idp_rejected_claim_exception.rs`

```diff
diff --git a/src/protocol_serde/shape_idp_rejected_claim_exception.rs b/src/protocol_serde/shape_idp_rejected_claim_exception.rs
--- a/src/protocol_serde/shape_idp_rejected_claim_exception.rs
+++ b/src/protocol_serde/shape_idp_rejected_claim_exception.rs
@@ -3,7 +3,10 @@
 pub fn de_idp_rejected_claim_exception_xml_err(
     inp: &[u8],
     mut builder: crate::types::error::builders::IdpRejectedClaimExceptionBuilder,
-) -> std::result::Result<crate::types::error::builders::IdpRejectedClaimExceptionBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
+) -> std::result::Result<
+    crate::types::error::builders::IdpRejectedClaimExceptionBuilder,
+    ::aws_smithy_xml::decode::XmlDecodeError,
+> {
     if inp.is_empty() {
         return Ok(builder);
     }
```

### `src/protocol_serde/shape_invalid_authorization_message_exception.rs`

```diff
diff --git a/src/protocol_serde/shape_invalid_authorization_message_exception.rs b/src/protocol_serde/shape_invalid_authorization_message_exception.rs
--- a/src/protocol_serde/shape_invalid_authorization_message_exception.rs
+++ b/src/protocol_serde/shape_invalid_authorization_message_exception.rs
@@ -3,7 +3,10 @@
 pub fn de_invalid_authorization_message_exception_xml_err(
     inp: &[u8],
     mut builder: crate::types::error::builders::InvalidAuthorizationMessageExceptionBuilder,
-) -> std::result::Result<crate::types::error::builders::InvalidAuthorizationMessageExceptionBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
+) -> std::result::Result<
+    crate::types::error::builders::InvalidAuthorizationMessageExceptionBuilder,
+    ::aws_smithy_xml::decode::XmlDecodeError,
+> {
     if inp.is_empty() {
         return Ok(builder);
     }
```

### `src/protocol_serde/shape_invalid_identity_token_exception.rs`

```diff
diff --git a/src/protocol_serde/shape_invalid_identity_token_exception.rs b/src/protocol_serde/shape_invalid_identity_token_exception.rs
--- a/src/protocol_serde/shape_invalid_identity_token_exception.rs
+++ b/src/protocol_serde/shape_invalid_identity_token_exception.rs
@@ -3,7 +3,10 @@
 pub fn de_invalid_identity_token_exception_xml_err(
     inp: &[u8],
     mut builder: crate::types::error::builders::InvalidIdentityTokenExceptionBuilder,
-) -> std::result::Result<crate::types::error::builders::InvalidIdentityTokenExceptionBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
+) -> std::result::Result<
+    crate::types::error::builders::InvalidIdentityTokenExceptionBuilder,
+    ::aws_smithy_xml::decode::XmlDecodeError,
+> {
     if inp.is_empty() {
         return Ok(builder);
     }
```

### `src/protocol_serde/shape_jwt_payload_size_exceeded_exception.rs`

```diff
diff --git a/src/protocol_serde/shape_jwt_payload_size_exceeded_exception.rs b/src/protocol_serde/shape_jwt_payload_size_exceeded_exception.rs
--- a/src/protocol_serde/shape_jwt_payload_size_exceeded_exception.rs
+++ b/src/protocol_serde/shape_jwt_payload_size_exceeded_exception.rs
@@ -3,7 +3,10 @@
 pub fn de_jwt_payload_size_exceeded_exception_xml_err(
     inp: &[u8],
     mut builder: crate::types::error::builders::JwtPayloadSizeExceededExceptionBuilder,
-) -> std::result::Result<crate::types::error::builders::JwtPayloadSizeExceededExceptionBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
+) -> std::result::Result<
+    crate::types::error::builders::JwtPayloadSizeExceededExceptionBuilder,
+    ::aws_smithy_xml::decode::XmlDecodeError,
+> {
     if inp.is_empty() {
         return Ok(builder);
     }
```

### `src/protocol_serde/shape_malformed_policy_document_exception.rs`

```diff
diff --git a/src/protocol_serde/shape_malformed_policy_document_exception.rs b/src/protocol_serde/shape_malformed_policy_document_exception.rs
--- a/src/protocol_serde/shape_malformed_policy_document_exception.rs
+++ b/src/protocol_serde/shape_malformed_policy_document_exception.rs
@@ -3,7 +3,10 @@
 pub fn de_malformed_policy_document_exception_xml_err(
     inp: &[u8],
     mut builder: crate::types::error::builders::MalformedPolicyDocumentExceptionBuilder,
-) -> std::result::Result<crate::types::error::builders::MalformedPolicyDocumentExceptionBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
+) -> std::result::Result<
+    crate::types::error::builders::MalformedPolicyDocumentExceptionBuilder,
+    ::aws_smithy_xml::decode::XmlDecodeError,
+> {
     if inp.is_empty() {
         return Ok(builder);
     }
```

### `src/protocol_serde/shape_outbound_web_identity_federation_disabled_exception.rs`

```diff
diff --git a/src/protocol_serde/shape_outbound_web_identity_federation_disabled_exception.rs b/src/protocol_serde/shape_outbound_web_identity_federation_disabled_exception.rs
--- a/src/protocol_serde/shape_outbound_web_identity_federation_disabled_exception.rs
+++ b/src/protocol_serde/shape_outbound_web_identity_federation_disabled_exception.rs
@@ -3,8 +3,10 @@
 pub fn de_outbound_web_identity_federation_disabled_exception_xml_err(
     inp: &[u8],
     mut builder: crate::types::error::builders::OutboundWebIdentityFederationDisabledExceptionBuilder,
-) -> std::result::Result<crate::types::error::builders::OutboundWebIdentityFederationDisabledExceptionBuilder, ::aws_smithy_xml::decode::XmlDecodeError>
-{
+) -> std::result::Result<
+    crate::types::error::builders::OutboundWebIdentityFederationDisabledExceptionBuilder,
+    ::aws_smithy_xml::decode::XmlDecodeError,
+> {
     if inp.is_empty() {
         return Ok(builder);
     }
```

### `src/protocol_serde/shape_packed_policy_too_large_exception.rs`

```diff
diff --git a/src/protocol_serde/shape_packed_policy_too_large_exception.rs b/src/protocol_serde/shape_packed_policy_too_large_exception.rs
--- a/src/protocol_serde/shape_packed_policy_too_large_exception.rs
+++ b/src/protocol_serde/shape_packed_policy_too_large_exception.rs
@@ -3,7 +3,10 @@
 pub fn de_packed_policy_too_large_exception_xml_err(
     inp: &[u8],
     mut builder: crate::types::error::builders::PackedPolicyTooLargeExceptionBuilder,
-) -> std::result::Result<crate::types::error::builders::PackedPolicyTooLargeExceptionBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
+) -> std::result::Result<
+    crate::types::error::builders::PackedPolicyTooLargeExceptionBuilder,
+    ::aws_smithy_xml::decode::XmlDecodeError,
+> {
     if inp.is_empty() {
         return Ok(builder);
     }
```

### `src/protocol_serde/shape_region_disabled_exception.rs`

```diff
diff --git a/src/protocol_serde/shape_region_disabled_exception.rs b/src/protocol_serde/shape_region_disabled_exception.rs
--- a/src/protocol_serde/shape_region_disabled_exception.rs
+++ b/src/protocol_serde/shape_region_disabled_exception.rs
@@ -3,7 +3,10 @@
 pub fn de_region_disabled_exception_xml_err(
     inp: &[u8],
     mut builder: crate::types::error::builders::RegionDisabledExceptionBuilder,
-) -> std::result::Result<crate::types::error::builders::RegionDisabledExceptionBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
+) -> std::result::Result<
+    crate::types::error::builders::RegionDisabledExceptionBuilder,
+    ::aws_smithy_xml::decode::XmlDecodeError,
+> {
     if inp.is_empty() {
         return Ok(builder);
     }
```

### `src/protocol_serde/shape_session_duration_escalation_exception.rs`

```diff
diff --git a/src/protocol_serde/shape_session_duration_escalation_exception.rs b/src/protocol_serde/shape_session_duration_escalation_exception.rs
--- a/src/protocol_serde/shape_session_duration_escalation_exception.rs
+++ b/src/protocol_serde/shape_session_duration_escalation_exception.rs
@@ -3,7 +3,10 @@
 pub fn de_session_duration_escalation_exception_xml_err(
     inp: &[u8],
     mut builder: crate::types::error::builders::SessionDurationEscalationExceptionBuilder,
-) -> std::result::Result<crate::types::error::builders::SessionDurationEscalationExceptionBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
+) -> std::result::Result<
+    crate::types::error::builders::SessionDurationEscalationExceptionBuilder,
+    ::aws_smithy_xml::decode::XmlDecodeError,
+> {
     if inp.is_empty() {
         return Ok(builder);
     }
```

### `src/sdk_feature_tracker.rs`

```diff
diff --git a/src/sdk_feature_tracker.rs b/src/sdk_feature_tracker.rs
--- a/src/sdk_feature_tracker.rs
+++ b/src/sdk_feature_tracker.rs
@@ -73,7 +73,8 @@
             _runtime_components: &RuntimeComponents,
             cfg: &mut ConfigBag,
         ) -> Result<(), BoxError> {
-            cfg.interceptor_state().store_append::<SmithySdkFeature>(SmithySdkFeature::Paginator);
+            cfg.interceptor_state()
+                .store_append::<SmithySdkFeature>(SmithySdkFeature::Paginator);
             Ok(())
         }
     }
@@ -131,7 +132,8 @@
             _runtime_components: &RuntimeComponents,
             cfg: &mut ConfigBag,
         ) -> Result<(), BoxError> {
-            cfg.interceptor_state().store_append::<SmithySdkFeature>(SmithySdkFeature::Waiter);
+            cfg.interceptor_state()
+                .store_append::<SmithySdkFeature>(SmithySdkFeature::Waiter);
             Ok(())
         }
     }
```

### `src/serde_util.rs`

```diff
diff --git a/src/serde_util.rs b/src/serde_util.rs
--- a/src/serde_util.rs
+++ b/src/serde_util.rs
@@ -1,4 +1,9 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity"
+))]
 pub(crate) fn assumed_role_user_correct_errors(
     mut builder: crate::types::builders::AssumedRoleUserBuilder,
 ) -> crate::types::builders::AssumedRoleUserBuilder {
@@ -11,7 +16,18 @@
     builder
 }

-pub(crate) fn credentials_correct_errors(mut builder: crate::types::builders::CredentialsBuilder) -> crate::types::builders::CredentialsBuilder {
+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_assume_root",
+    feature = "op_get_delegated_access_token",
+    feature = "op_get_federation_token",
+    feature = "op_get_session_token"
+))]
+pub(crate) fn credentials_correct_errors(
+    mut builder: crate::types::builders::CredentialsBuilder,
+) -> crate::types::builders::CredentialsBuilder {
     if builder.access_key_id.is_none() {
         builder.access_key_id = Some(Default::default())
     }
@@ -27,6 +43,7 @@
     builder
 }

+#[cfg(feature = "op_get_federation_token")]
 pub(crate) fn federated_user_correct_errors(
     mut builder: crate::types::builders::FederatedUserBuilder,
 ) -> crate::types::builders::FederatedUserBuilder {
```

### `src/serialization_settings.rs`

```diff
diff --git a/src/serialization_settings.rs b/src/serialization_settings.rs
--- a/src/serialization_settings.rs
+++ b/src/serialization_settings.rs
@@ -57,7 +57,8 @@

     /// Returns true if the given default header name should be serialized
     fn include_header(&self, header: &HeaderName) -> bool {
-        (!self.omit_default_content_length || header != CONTENT_LENGTH) && (!self.omit_default_content_type || header != CONTENT_TYPE)
+        (!self.omit_default_content_length || header != CONTENT_LENGTH)
+            && (!self.omit_default_content_type || header != CONTENT_TYPE)
     }

     /// Sets a default header on the given request builder if it should be serialized
```

### `src/types.rs`

```diff
diff --git a/src/types.rs b/src/types.rs
--- a/src/types.rs
+++ b/src/types.rs
@@ -1,26 +1,82 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_assume_root",
+    feature = "op_get_delegated_access_token",
+    feature = "op_get_federation_token",
+    feature = "op_get_session_token"
+))]
 pub use crate::types::_credentials::Credentials;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity"
+))]
 pub use crate::types::_assumed_role_user::AssumedRoleUser;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_assume_root",
+    feature = "op_get_federation_token"
+))]
 pub use crate::types::_policy_descriptor_type::PolicyDescriptorType;

+#[cfg(feature = "op_get_federation_token")]
 pub use crate::types::_federated_user::FederatedUser;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_get_federation_token",
+    feature = "op_get_web_identity_token"
+))]
 pub use crate::types::_tag::Tag;

+#[cfg(feature = "op_assume_role")]
 pub use crate::types::_provided_context::ProvidedContext;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity"
+))]
 mod _assumed_role_user;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_assume_root",
+    feature = "op_get_delegated_access_token",
+    feature = "op_get_federation_token",
+    feature = "op_get_session_token"
+))]
 mod _credentials;

+#[cfg(feature = "op_get_federation_token")]
 mod _federated_user;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_assume_root",
+    feature = "op_get_federation_token"
+))]
 mod _policy_descriptor_type;

+#[cfg(feature = "op_assume_role")]
 mod _provided_context;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_get_federation_token",
+    feature = "op_get_web_identity_token"
+))]
 mod _tag;

 /// Builders
```

### `src/types/_assumed_role_user.rs`

```diff
diff --git a/src/types/_assumed_role_user.rs b/src/types/_assumed_role_user.rs
--- a/src/types/_assumed_role_user.rs
+++ b/src/types/_assumed_role_user.rs
@@ -70,7 +70,9 @@
     /// This method will fail if any of the following fields are not set:
     /// - [`assumed_role_id`](crate::types::builders::AssumedRoleUserBuilder::assumed_role_id)
     /// - [`arn`](crate::types::builders::AssumedRoleUserBuilder::arn)
-    pub fn build(self) -> ::std::result::Result<crate::types::AssumedRoleUser, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<crate::types::AssumedRoleUser, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(crate::types::AssumedRoleUser {
             assumed_role_id: self.assumed_role_id.ok_or_else(|| {
                 ::aws_smithy_types::error::operation::BuildError::missing_field(
```

### `src/types/_credentials.rs`

```diff
diff --git a/src/types/_credentials.rs b/src/types/_credentials.rs
--- a/src/types/_credentials.rs
+++ b/src/types/_credentials.rs
@@ -127,7 +127,9 @@
     /// - [`secret_access_key`](crate::types::builders::CredentialsBuilder::secret_access_key)
     /// - [`session_token`](crate::types::builders::CredentialsBuilder::session_token)
     /// - [`expiration`](crate::types::builders::CredentialsBuilder::expiration)
-    pub fn build(self) -> ::std::result::Result<crate::types::Credentials, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<crate::types::Credentials, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(crate::types::Credentials {
             access_key_id: self.access_key_id.ok_or_else(|| {
                 ::aws_smithy_types::error::operation::BuildError::missing_field(
```

### `src/types/_federated_user.rs`

```diff
diff --git a/src/types/_federated_user.rs b/src/types/_federated_user.rs
--- a/src/types/_federated_user.rs
+++ b/src/types/_federated_user.rs
@@ -70,7 +70,9 @@
     /// This method will fail if any of the following fields are not set:
     /// - [`federated_user_id`](crate::types::builders::FederatedUserBuilder::federated_user_id)
     /// - [`arn`](crate::types::builders::FederatedUserBuilder::arn)
-    pub fn build(self) -> ::std::result::Result<crate::types::FederatedUser, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<crate::types::FederatedUser, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(crate::types::FederatedUser {
             federated_user_id: self.federated_user_id.ok_or_else(|| {
                 ::aws_smithy_types::error::operation::BuildError::missing_field(
```

### `src/types/_tag.rs`

```diff
diff --git a/src/types/_tag.rs b/src/types/_tag.rs
--- a/src/types/_tag.rs
+++ b/src/types/_tag.rs
@@ -83,7 +83,10 @@
     pub fn build(self) -> ::std::result::Result<crate::types::Tag, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(crate::types::Tag {
             key: self.key.ok_or_else(|| {
-                ::aws_smithy_types::error::operation::BuildError::missing_field("key", "key was not specified but it is required when building Tag")
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "key",
+                    "key was not specified but it is required when building Tag",
+                )
             })?,
             value: self.value.ok_or_else(|| {
                 ::aws_smithy_types::error::operation::BuildError::missing_field(
```

### `src/types/builders.rs`

```diff
diff --git a/src/types/builders.rs b/src/types/builders.rs
--- a/src/types/builders.rs
+++ b/src/types/builders.rs
@@ -1,12 +1,40 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_assume_root",
+    feature = "op_get_delegated_access_token",
+    feature = "op_get_federation_token",
+    feature = "op_get_session_token"
+))]
 pub use crate::types::_credentials::CredentialsBuilder;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity"
+))]
 pub use crate::types::_assumed_role_user::AssumedRoleUserBuilder;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_assume_root",
+    feature = "op_get_federation_token"
+))]
 pub use crate::types::_policy_descriptor_type::PolicyDescriptorTypeBuilder;

+#[cfg(feature = "op_get_federation_token")]
 pub use crate::types::_federated_user::FederatedUserBuilder;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_get_federation_token",
+    feature = "op_get_web_identity_token"
+))]
 pub use crate::types::_tag::TagBuilder;

+#[cfg(feature = "op_assume_role")]
 pub use crate::types::_provided_context::ProvidedContextBuilder;
```

### `src/types/error.rs`

```diff
diff --git a/src/types/error.rs b/src/types/error.rs
--- a/src/types/error.rs
+++ b/src/types/error.rs
@@ -1,50 +1,122 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_assume_root"
+))]
 pub use crate::types::error::_expired_token_exception::ExpiredTokenException;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_get_federation_token"
+))]
 pub use crate::types::error::_malformed_policy_document_exception::MalformedPolicyDocumentException;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_get_delegated_access_token",
+    feature = "op_get_federation_token"
+))]
 pub use crate::types::error::_packed_policy_too_large_exception::PackedPolicyTooLargeException;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_assume_root",
+    feature = "op_get_delegated_access_token",
+    feature = "op_get_federation_token",
+    feature = "op_get_session_token"
+))]
 pub use crate::types::error::_region_disabled_exception::RegionDisabledException;

+#[cfg(any(feature = "op_assume_role_with_saml", feature = "op_assume_role_with_web_identity"))]
 pub use crate::types::error::_idp_rejected_claim_exception::IdpRejectedClaimException;

+#[cfg(any(feature = "op_assume_role_with_saml", feature = "op_assume_role_with_web_identity"))]
 pub use crate::types::error::_invalid_identity_token_exception::InvalidIdentityTokenException;

+#[cfg(feature = "op_assume_role_with_web_identity")]
 pub use crate::types::error::_idp_communication_error_exception::IdpCommunicationErrorException;

+#[cfg(feature = "op_decode_authorization_message")]
 pub use crate::types::error::_invalid_authorization_message_exception::InvalidAuthorizationMessageException;

+#[cfg(feature = "op_get_delegated_access_token")]
 pub use crate::types::error::_expired_trade_in_token_exception::ExpiredTradeInTokenException;

+#[cfg(feature = "op_get_web_identity_token")]
 pub use crate::types::error::_jwt_payload_size_exceeded_exception::JwtPayloadSizeExceededException;

+#[cfg(feature = "op_get_web_identity_token")]
 pub use crate::types::error::_outbound_web_identity_federation_disabled_exception::OutboundWebIdentityFederationDisabledException;

+#[cfg(feature = "op_get_web_identity_token")]
 pub use crate::types::error::_session_duration_escalation_exception::SessionDurationEscalationException;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_assume_root"
+))]
 mod _expired_token_exception;

+#[cfg(feature = "op_get_delegated_access_token")]
 mod _expired_trade_in_token_exception;

+#[cfg(feature = "op_assume_role_with_web_identity")]
 mod _idp_communication_error_exception;

+#[cfg(any(feature = "op_assume_role_with_saml", feature = "op_assume_role_with_web_identity"))]
 mod _idp_rejected_claim_exception;

+#[cfg(feature = "op_decode_authorization_message")]
 mod _invalid_authorization_message_exception;

+#[cfg(any(feature = "op_assume_role_with_saml", feature = "op_assume_role_with_web_identity"))]
 mod _invalid_identity_token_exception;

+#[cfg(feature = "op_get_web_identity_token")]
 mod _jwt_payload_size_exceeded_exception;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_get_federation_token"
+))]
 mod _malformed_policy_document_exception;

+#[cfg(feature = "op_get_web_identity_token")]
 mod _outbound_web_identity_federation_disabled_exception;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_get_delegated_access_token",
+    feature = "op_get_federation_token"
+))]
 mod _packed_policy_too_large_exception;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_assume_root",
+    feature = "op_get_delegated_access_token",
+    feature = "op_get_federation_token",
+    feature = "op_get_session_token"
+))]
 mod _region_disabled_exception;

+#[cfg(feature = "op_get_web_identity_token")]
 mod _session_duration_escalation_exception;

 /// Builders
```

### `src/types/error/builders.rs`

```diff
diff --git a/src/types/error/builders.rs b/src/types/error/builders.rs
--- a/src/types/error/builders.rs
+++ b/src/types/error/builders.rs
@@ -1,24 +1,60 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_assume_root"
+))]
 pub use crate::types::error::_expired_token_exception::ExpiredTokenExceptionBuilder;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_get_federation_token"
+))]
 pub use crate::types::error::_malformed_policy_document_exception::MalformedPolicyDocumentExceptionBuilder;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_get_delegated_access_token",
+    feature = "op_get_federation_token"
+))]
 pub use crate::types::error::_packed_policy_too_large_exception::PackedPolicyTooLargeExceptionBuilder;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_assume_root",
+    feature = "op_get_delegated_access_token",
+    feature = "op_get_federation_token",
+    feature = "op_get_session_token"
+))]
 pub use crate::types::error::_region_disabled_exception::RegionDisabledExceptionBuilder;

+#[cfg(any(feature = "op_assume_role_with_saml", feature = "op_assume_role_with_web_identity"))]
 pub use crate::types::error::_idp_rejected_claim_exception::IdpRejectedClaimExceptionBuilder;

+#[cfg(any(feature = "op_assume_role_with_saml", feature = "op_assume_role_with_web_identity"))]
 pub use crate::types::error::_invalid_identity_token_exception::InvalidIdentityTokenExceptionBuilder;

+#[cfg(feature = "op_assume_role_with_web_identity")]
 pub use crate::types::error::_idp_communication_error_exception::IdpCommunicationErrorExceptionBuilder;

+#[cfg(feature = "op_decode_authorization_message")]
 pub use crate::types::error::_invalid_authorization_message_exception::InvalidAuthorizationMessageExceptionBuilder;

+#[cfg(feature = "op_get_delegated_access_token")]
 pub use crate::types::error::_expired_trade_in_token_exception::ExpiredTradeInTokenExceptionBuilder;

+#[cfg(feature = "op_get_web_identity_token")]
 pub use crate::types::error::_jwt_payload_size_exceeded_exception::JwtPayloadSizeExceededExceptionBuilder;

+#[cfg(feature = "op_get_web_identity_token")]
 pub use crate::types::error::_outbound_web_identity_federation_disabled_exception::OutboundWebIdentityFederationDisabledExceptionBuilder;

+#[cfg(feature = "op_get_web_identity_token")]
 pub use crate::types::error::_session_duration_escalation_exception::SessionDurationEscalationExceptionBuilder;
```

## Unified diff

```diff
diff --git a/Cargo.toml b/Cargo.toml
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -19,58 +19,45 @@
 version = "1"

 [dependencies.aws-credential-types]
-path = "../aws-credential-types"
 version = "1.3.0"

 [dependencies.aws-runtime]
-path = "../aws-runtime"
 version = "1.9.1"

 [dependencies.aws-smithy-async]
-path = "../aws-smithy-async"
 version = "1.3.0"

 [dependencies.aws-smithy-http]
-path = "../aws-smithy-http"
 version = "0.64.0"

 [dependencies.aws-smithy-json]
-path = "../aws-smithy-json"
 version = "0.63.0"

 [dependencies.aws-smithy-observability]
-path = "../aws-smithy-observability"
 version = "0.3.0"

 [dependencies.aws-smithy-query]
-path = "../aws-smithy-query"
 version = "0.62.0"

 [dependencies.aws-smithy-runtime]
-path = "../aws-smithy-runtime"
 features = ["client"]
 version = "1.14.0"

 [dependencies.aws-smithy-runtime-api]
-path = "../aws-smithy-runtime-api"
 features = ["client", "http-1x"]
 version = "1.15.0"

 [dependencies.aws-smithy-schema]
-path = "../aws-smithy-schema"
 version = "0.2.0"

 [dependencies.aws-smithy-types]
-path = "../aws-smithy-types"
 features = ["http-body-1-x"]
 version = "1.6.2"

 [dependencies.aws-smithy-xml]
-path = "../aws-smithy-xml"
 version = "0.62.0"

 [dependencies.aws-types]
-path = "../aws-types"
 version = "1.5.0"

 [dependencies.fastrand]
@@ -89,41 +76,33 @@
 [dependencies.tracing]
 version = "0.1"
 [dev-dependencies.aws-credential-types]
-path = "../aws-credential-types"
 features = ["test-util"]
 version = "1.3.0"

 [dev-dependencies.aws-runtime]
-path = "../aws-runtime"
 features = ["test-util"]
 version = "1.9.1"

 [dev-dependencies.aws-smithy-async]
-path = "../aws-smithy-async"
 features = ["test-util"]
 version = "1.3.0"

 [dev-dependencies.aws-smithy-http-client]
-path = "../aws-smithy-http-client"
 features = ["test-util", "wire-mock"]
 version = "1.4.0"

 [dev-dependencies.aws-smithy-protocol-test]
-path = "../aws-smithy-protocol-test"
 version = "0.64.0"

 [dev-dependencies.aws-smithy-runtime]
-path = "../aws-smithy-runtime"
 features = ["test-util"]
 version = "1.14.0"

 [dev-dependencies.aws-smithy-runtime-api]
-path = "../aws-smithy-runtime-api"
 features = ["test-util"]
 version = "1.15.0"

 [dev-dependencies.aws-smithy-types]
-path = "../aws-smithy-types"
 features = ["http-body-1-x", "test-util"]
 version = "1.6.2"

@@ -155,6 +134,20 @@
 test-util = ["aws-credential-types/test-util", "aws-smithy-runtime/legacy-test-util", "aws-smithy-runtime/test-util"]
 gated-tests = []
 default = ["sigv4a", "rustls", "default-https-client", "rt-tokio"]
+op_assume_role = []
+op_assume_role_with_saml = []
+op_assume_role_with_web_identity = []
+op_assume_root = []
+op_decode_authorization_message = []
+op_get_access_key_info = []
+op_get_caller_identity = []
+op_get_delegated_access_token = []
+op_get_federation_token = []
+op_get_session_token = []
+op_get_web_identity_token = []

 [hints]
 mostly-unused = true
+
+[lib]
+name = "aws_sdk_sts"
diff --git a/src/client.rs b/src/client.rs
--- a/src/client.rs
+++ b/src/client.rs
@@ -90,12 +90,16 @@
     }
 }

+#[cfg(feature = "op_assume_role")]
 mod assume_role;

+#[cfg(feature = "op_assume_role_with_saml")]
 mod assume_role_with_saml;

+#[cfg(feature = "op_assume_role_with_web_identity")]
 mod assume_role_with_web_identity;

+#[cfg(feature = "op_assume_root")]
 mod assume_root;

 /// Operation customization and supporting types.
@@ -125,16 +129,23 @@
 /// ```
 pub mod customize;

+#[cfg(feature = "op_decode_authorization_message")]
 mod decode_authorization_message;

+#[cfg(feature = "op_get_access_key_info")]
 mod get_access_key_info;

+#[cfg(feature = "op_get_caller_identity")]
 mod get_caller_identity;

+#[cfg(feature = "op_get_delegated_access_token")]
 mod get_delegated_access_token;

+#[cfg(feature = "op_get_federation_token")]
 mod get_federation_token;

+#[cfg(feature = "op_get_session_token")]
 mod get_session_token;

+#[cfg(feature = "op_get_web_identity_token")]
 mod get_web_identity_token;
diff --git a/src/client/assume_role_with_saml.rs b/src/client/assume_role_with_saml.rs
--- a/src/client/assume_role_with_saml.rs
+++ b/src/client/assume_role_with_saml.rs
@@ -20,7 +20,9 @@
     ///   - [`name_qualifier(Option<String>)`](crate::operation::assume_role_with_saml::AssumeRoleWithSamlOutput::name_qualifier): <p>A hash value based on the concatenation of the following:</p> <ul>  <li>   <p>The <code>Issuer</code> response value.</p></li>  <li>   <p>The Amazon Web Services account ID.</p></li>  <li>   <p>The friendly name (the last part of the ARN) of the SAML provider in IAM.</p></li> </ul> <p>The combination of <code>NameQualifier</code> and <code>Subject</code> can be used to uniquely identify a user.</p> <p>The following pseudocode shows how the hash value is calculated:</p> <p><code>BASE64 ( SHA1 ( "https://example.com/saml" + "123456789012" + "/MySAMLIdP" ) )</code></p>
     ///   - [`source_identity(Option<String>)`](crate::operation::assume_role_with_saml::AssumeRoleWithSamlOutput::source_identity): <p>The value in the <code>SourceIdentity</code> attribute in the SAML assertion. The source identity value persists across <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles.html#iam-term-role-chaining">chained role</a> sessions.</p> <p>You can require users to set a source identity value when they assume a role. You do this by using the <code>sts:SourceIdentity</code> condition key in a role trust policy. That way, actions that are taken with the role are associated with that user. After the source identity is set, the value cannot be changed. It is present in the request for all actions that are taken by the role and persists across <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles.html#id_roles_terms-and-concepts">chained role</a> sessions. You can configure your SAML identity provider to use an attribute associated with your users, like user name or email, as the source identity when calling <code>AssumeRoleWithSAML</code>. You do this by adding an attribute to the SAML assertion. For more information about using source identity, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_control-access_monitor.html">Monitor and control actions taken with assumed roles</a> in the <i>IAM User Guide</i>.</p> <p>The regex used to validate this parameter is a string of characters consisting of upper- and lower-case alphanumeric characters with no spaces. You can also include underscores or any of the following characters: =,.@-</p>
     /// - On failure, responds with [`SdkError<AssumeRoleWithSAMLError>`](crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError)
-    pub fn assume_role_with_saml(&self) -> crate::operation::assume_role_with_saml::builders::AssumeRoleWithSAMLFluentBuilder {
+    pub fn assume_role_with_saml(
+        &self,
+    ) -> crate::operation::assume_role_with_saml::builders::AssumeRoleWithSAMLFluentBuilder {
         crate::operation::assume_role_with_saml::builders::AssumeRoleWithSAMLFluentBuilder::new(self.handle.clone())
     }
 }
diff --git a/src/client/assume_role_with_web_identity.rs b/src/client/assume_role_with_web_identity.rs
--- a/src/client/assume_role_with_web_identity.rs
+++ b/src/client/assume_role_with_web_identity.rs
@@ -19,7 +19,11 @@
     ///   - [`audience(Option<String>)`](crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityOutput::audience): <p>The intended audience (also known as client ID) of the web identity token. This is traditionally the client identifier issued to the application that requested the web identity token.</p>
     ///   - [`source_identity(Option<String>)`](crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityOutput::source_identity): <p>The value of the source identity that is returned in the JSON web token (JWT) from the identity provider.</p> <p>You can require users to set a source identity value when they assume a role. You do this by using the <code>sts:SourceIdentity</code> condition key in a role trust policy. That way, actions that are taken with the role are associated with that user. After the source identity is set, the value cannot be changed. It is present in the request for all actions that are taken by the role and persists across <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles.html#id_roles_terms-and-concepts">chained role</a> sessions. You can configure your identity provider to use an attribute associated with your users, like user name or email, as the source identity when calling <code>AssumeRoleWithWebIdentity</code>. You do this by adding a claim to the JSON web token. To learn more about OIDC tokens and claims, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-using-tokens-with-identity-providers.html">Using Tokens with User Pools</a> in the <i>Amazon Cognito Developer Guide</i>. For more information about using source identity, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_control-access_monitor.html">Monitor and control actions taken with assumed roles</a> in the <i>IAM User Guide</i>.</p> <p>The regex used to validate this parameter is a string of characters consisting of upper- and lower-case alphanumeric characters with no spaces. You can also include underscores or any of the following characters: =,.@-</p>
     /// - On failure, responds with [`SdkError<AssumeRoleWithWebIdentityError>`](crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError)
-    pub fn assume_role_with_web_identity(&self) -> crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityFluentBuilder {
-        crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityFluentBuilder::new(self.handle.clone())
+    pub fn assume_role_with_web_identity(
+        &self,
+    ) -> crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityFluentBuilder {
+        crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityFluentBuilder::new(
+            self.handle.clone(),
+        )
     }
 }
diff --git a/src/client/customize.rs b/src/client/customize.rs
--- a/src/client/customize.rs
+++ b/src/client/customize.rs
@@ -41,15 +41,23 @@
     /// `map_request`, and `mutate_request` (the last two are implemented via interceptors under the hood).
     /// The order in which those user-specified operation interceptors are invoked should not be relied upon
     /// as it is an implementation detail.
-    pub fn interceptor(mut self, interceptor: impl ::aws_smithy_runtime_api::client::interceptors::Intercept + 'static) -> Self {
+    pub fn interceptor(
+        mut self,
+        interceptor: impl ::aws_smithy_runtime_api::client::interceptors::Intercept + 'static,
+    ) -> Self {
         self.interceptors
-            .push(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::new(interceptor));
+            .push(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::new(
+                interceptor,
+            ));
         self
     }

     /// Adds a runtime plugin.
     #[allow(unused)]
-    pub(crate) fn runtime_plugin(mut self, runtime_plugin: impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin + 'static) -> Self {
+    pub(crate) fn runtime_plugin(
+        mut self,
+        runtime_plugin: impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin + 'static,
+    ) -> Self {
         self.runtime_plugins
             .push(::aws_smithy_runtime_api::client::runtime_plugin::SharedRuntimePlugin::new(runtime_plugin));
         self
@@ -60,7 +68,8 @@
     where
         F: ::std::ops::Fn(
                 ::aws_smithy_runtime_api::client::orchestrator::HttpRequest,
-            ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, MapE>
+            )
+                -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, MapE>
             + ::std::marker::Send
             + ::std::marker::Sync
             + 'static,
@@ -76,7 +85,10 @@
     /// Convenience for `map_request` where infallible direct mutation of request is acceptable.
     pub fn mutate_request<F>(mut self, f: F) -> Self
     where
-        F: ::std::ops::Fn(&mut ::aws_smithy_runtime_api::client::orchestrator::HttpRequest) + ::std::marker::Send + ::std::marker::Sync + 'static,
+        F: ::std::ops::Fn(&mut ::aws_smithy_runtime_api::client::orchestrator::HttpRequest)
+            + ::std::marker::Send
+            + ::std::marker::Sync
+            + 'static,
     {
         self.interceptors
             .push(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::new(
diff --git a/src/client/customize/internal.rs b/src/client/customize/internal.rs
--- a/src/client/customize/internal.rs
+++ b/src/client/customize/internal.rs
@@ -1,8 +1,10 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub type BoxFuture<T> = ::std::pin::Pin<::std::boxed::Box<dyn ::std::future::Future<Output = T> + ::std::marker::Send>>;

-pub type SendResult<T, E> =
-    ::std::result::Result<T, ::aws_smithy_runtime_api::client::result::SdkError<E, ::aws_smithy_runtime_api::client::orchestrator::HttpResponse>>;
+pub type SendResult<T, E> = ::std::result::Result<
+    T,
+    ::aws_smithy_runtime_api::client::result::SdkError<E, ::aws_smithy_runtime_api::client::orchestrator::HttpResponse>,
+>;

 pub trait CustomizableSend<T, E>: ::std::marker::Send + ::std::marker::Sync {
     // Takes an owned `self` as the implementation will internally call methods that take `self`.
diff --git a/src/client/decode_authorization_message.rs b/src/client/decode_authorization_message.rs
--- a/src/client/decode_authorization_message.rs
+++ b/src/client/decode_authorization_message.rs
@@ -7,7 +7,11 @@
     /// - On success, responds with [`DecodeAuthorizationMessageOutput`](crate::operation::decode_authorization_message::DecodeAuthorizationMessageOutput) with field(s):
     ///   - [`decoded_message(Option<String>)`](crate::operation::decode_authorization_message::DecodeAuthorizationMessageOutput::decoded_message): <p>The API returns a response with the decoded message.</p>
     /// - On failure, responds with [`SdkError<DecodeAuthorizationMessageError>`](crate::operation::decode_authorization_message::DecodeAuthorizationMessageError)
-    pub fn decode_authorization_message(&self) -> crate::operation::decode_authorization_message::builders::DecodeAuthorizationMessageFluentBuilder {
-        crate::operation::decode_authorization_message::builders::DecodeAuthorizationMessageFluentBuilder::new(self.handle.clone())
+    pub fn decode_authorization_message(
+        &self,
+    ) -> crate::operation::decode_authorization_message::builders::DecodeAuthorizationMessageFluentBuilder {
+        crate::operation::decode_authorization_message::builders::DecodeAuthorizationMessageFluentBuilder::new(
+            self.handle.clone(),
+        )
     }
 }
diff --git a/src/client/get_access_key_info.rs b/src/client/get_access_key_info.rs
--- a/src/client/get_access_key_info.rs
+++ b/src/client/get_access_key_info.rs
@@ -7,7 +7,9 @@
     /// - On success, responds with [`GetAccessKeyInfoOutput`](crate::operation::get_access_key_info::GetAccessKeyInfoOutput) with field(s):
     ///   - [`account(Option<String>)`](crate::operation::get_access_key_info::GetAccessKeyInfoOutput::account): <p>The number used to identify the Amazon Web Services account.</p>
     /// - On failure, responds with [`SdkError<GetAccessKeyInfoError>`](crate::operation::get_access_key_info::GetAccessKeyInfoError)
-    pub fn get_access_key_info(&self) -> crate::operation::get_access_key_info::builders::GetAccessKeyInfoFluentBuilder {
+    pub fn get_access_key_info(
+        &self,
+    ) -> crate::operation::get_access_key_info::builders::GetAccessKeyInfoFluentBuilder {
         crate::operation::get_access_key_info::builders::GetAccessKeyInfoFluentBuilder::new(self.handle.clone())
     }
 }
diff --git a/src/client/get_caller_identity.rs b/src/client/get_caller_identity.rs
--- a/src/client/get_caller_identity.rs
+++ b/src/client/get_caller_identity.rs
@@ -8,7 +8,9 @@
     ///   - [`account(Option<String>)`](crate::operation::get_caller_identity::GetCallerIdentityOutput::account): <p>The Amazon Web Services account ID number of the account that owns or contains the calling entity.</p>
     ///   - [`arn(Option<String>)`](crate::operation::get_caller_identity::GetCallerIdentityOutput::arn): <p>The Amazon Web Services ARN associated with the calling entity.</p>
     /// - On failure, responds with [`SdkError<GetCallerIdentityError>`](crate::operation::get_caller_identity::GetCallerIdentityError)
-    pub fn get_caller_identity(&self) -> crate::operation::get_caller_identity::builders::GetCallerIdentityFluentBuilder {
+    pub fn get_caller_identity(
+        &self,
+    ) -> crate::operation::get_caller_identity::builders::GetCallerIdentityFluentBuilder {
         crate::operation::get_caller_identity::builders::GetCallerIdentityFluentBuilder::new(self.handle.clone())
     }
 }
diff --git a/src/client/get_delegated_access_token.rs b/src/client/get_delegated_access_token.rs
--- a/src/client/get_delegated_access_token.rs
+++ b/src/client/get_delegated_access_token.rs
@@ -9,7 +9,11 @@
     ///   - [`packed_policy_size(Option<i32>)`](crate::operation::get_delegated_access_token::GetDelegatedAccessTokenOutput::packed_policy_size): <p>The percentage of the maximum policy size that is used by the session policy. The policy size is calculated as the sum of all the session policies and permission boundaries attached to the session. If the packed size exceeds 100%, the request fails.</p>
     ///   - [`assumed_principal(Option<String>)`](crate::operation::get_delegated_access_token::GetDelegatedAccessTokenOutput::assumed_principal): <p>The Amazon Resource Name (ARN) of the principal that was assumed when obtaining the delegated access token. This ARN identifies the IAM entity whose permissions are granted by the temporary credentials.</p>
     /// - On failure, responds with [`SdkError<GetDelegatedAccessTokenError>`](crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError)
-    pub fn get_delegated_access_token(&self) -> crate::operation::get_delegated_access_token::builders::GetDelegatedAccessTokenFluentBuilder {
-        crate::operation::get_delegated_access_token::builders::GetDelegatedAccessTokenFluentBuilder::new(self.handle.clone())
+    pub fn get_delegated_access_token(
+        &self,
+    ) -> crate::operation::get_delegated_access_token::builders::GetDelegatedAccessTokenFluentBuilder {
+        crate::operation::get_delegated_access_token::builders::GetDelegatedAccessTokenFluentBuilder::new(
+            self.handle.clone(),
+        )
     }
 }
diff --git a/src/client/get_federation_token.rs b/src/client/get_federation_token.rs
--- a/src/client/get_federation_token.rs
+++ b/src/client/get_federation_token.rs
@@ -13,7 +13,9 @@
     ///   - [`federated_user(Option<FederatedUser>)`](crate::operation::get_federation_token::GetFederationTokenOutput::federated_user): <p>Identifiers for the federated user associated with the credentials (such as <code>arn:aws:sts::123456789012:federated-user/Bob</code> or <code>123456789012:Bob</code>). You can use the federated user's ARN in your resource-based policies, such as an Amazon S3 bucket policy.</p>
     ///   - [`packed_policy_size(Option<i32>)`](crate::operation::get_federation_token::GetFederationTokenOutput::packed_policy_size): <p>A percentage value that indicates the packed size of the session policies and session tags combined passed in the request. The request fails if the packed size is greater than 100 percent, which means the policies and tags exceeded the allowed space.</p>
     /// - On failure, responds with [`SdkError<GetFederationTokenError>`](crate::operation::get_federation_token::GetFederationTokenError)
-    pub fn get_federation_token(&self) -> crate::operation::get_federation_token::builders::GetFederationTokenFluentBuilder {
+    pub fn get_federation_token(
+        &self,
+    ) -> crate::operation::get_federation_token::builders::GetFederationTokenFluentBuilder {
         crate::operation::get_federation_token::builders::GetFederationTokenFluentBuilder::new(self.handle.clone())
     }
 }
diff --git a/src/client/get_web_identity_token.rs b/src/client/get_web_identity_token.rs
--- a/src/client/get_web_identity_token.rs
+++ b/src/client/get_web_identity_token.rs
@@ -11,7 +11,9 @@
     ///   - [`web_identity_token(Option<String>)`](crate::operation::get_web_identity_token::GetWebIdentityTokenOutput::web_identity_token): <p>A signed JSON Web Token (JWT) that represents the caller's Amazon Web Services identity. The token contains standard JWT claims such as subject, audience, expiration time, and additional identity attributes added by STS as custom claims. You can also add your own custom claims to the token by passing tags as request parameters to the <code>GetWebIdentityToken</code> API. The token is signed using the specified signing algorithm and can be verified using the verification keys available at the issuer's JWKS endpoint.</p>
     ///   - [`expiration(Option<DateTime>)`](crate::operation::get_web_identity_token::GetWebIdentityTokenOutput::expiration): <p>The date and time when the web identity token expires, in UTC. The expiration is determined by adding the <code>DurationSeconds</code> value to the time the token was issued. After this time, the token should no longer be considered valid.</p>
     /// - On failure, responds with [`SdkError<GetWebIdentityTokenError>`](crate::operation::get_web_identity_token::GetWebIdentityTokenError)
-    pub fn get_web_identity_token(&self) -> crate::operation::get_web_identity_token::builders::GetWebIdentityTokenFluentBuilder {
+    pub fn get_web_identity_token(
+        &self,
+    ) -> crate::operation::get_web_identity_token::builders::GetWebIdentityTokenFluentBuilder {
         crate::operation::get_web_identity_token::builders::GetWebIdentityTokenFluentBuilder::new(self.handle.clone())
     }
 }
diff --git a/src/config.rs b/src/config.rs
--- a/src/config.rs
+++ b/src/config.rs
@@ -60,16 +60,23 @@
     }

     /// Return the auth scheme resolver configured on this service config
-    pub fn auth_scheme_resolver(&self) -> ::std::option::Option<::aws_smithy_runtime_api::client::auth::SharedAuthSchemeOptionResolver> {
+    pub fn auth_scheme_resolver(
+        &self,
+    ) -> ::std::option::Option<::aws_smithy_runtime_api::client::auth::SharedAuthSchemeOptionResolver> {
         self.runtime_components.auth_scheme_option_resolver()
     }
     /// Returns the configured auth scheme preference
-    pub fn auth_scheme_preference(&self) -> ::std::option::Option<&::aws_smithy_runtime_api::client::auth::AuthSchemePreference> {
-        self.config.load::<::aws_smithy_runtime_api::client::auth::AuthSchemePreference>()
+    pub fn auth_scheme_preference(
+        &self,
+    ) -> ::std::option::Option<&::aws_smithy_runtime_api::client::auth::AuthSchemePreference> {
+        self.config
+            .load::<::aws_smithy_runtime_api::client::auth::AuthSchemePreference>()
     }
     /// Returns the endpoint resolver.
     pub fn endpoint_resolver(&self) -> ::aws_smithy_runtime_api::client::endpoint::SharedEndpointResolver {
-        self.runtime_components.endpoint_resolver().expect("resolver defaulted if not set")
+        self.runtime_components
+            .endpoint_resolver()
+            .expect("resolver defaulted if not set")
     }
     /// Return a reference to the retry configuration contained in this config, if any.
     pub fn retry_config(&self) -> ::std::option::Option<&::aws_smithy_types::retry::RetryConfig> {
@@ -91,7 +98,8 @@
     /// WARNING: This method is unstable and may be removed at any time. Do not rely on this
     /// method for anything!
     pub fn retry_partition(&self) -> ::std::option::Option<&::aws_smithy_runtime::client::retries::RetryPartition> {
-        self.config.load::<::aws_smithy_runtime::client::retries::RetryPartition>()
+        self.config
+            .load::<::aws_smithy_runtime::client::retries::RetryPartition>()
     }
     /// Returns the configured identity cache for auth.
     pub fn identity_cache(&self) -> ::std::option::Option<crate::config::SharedIdentityCache> {
@@ -106,7 +114,9 @@
         self.runtime_components.time_source()
     }
     /// Returns retry classifiers currently registered by the user.
-    pub fn retry_classifiers(&self) -> impl Iterator<Item = ::aws_smithy_runtime_api::client::retries::classifiers::SharedRetryClassifier> + '_ {
+    pub fn retry_classifiers(
+        &self,
+    ) -> impl Iterator<Item = ::aws_smithy_runtime_api::client::retries::classifiers::SharedRetryClassifier> + '_ {
         self.runtime_components.retry_classifiers()
     }
     /// Returns the name of the app that is using the client, if it was provided.
@@ -125,8 +135,10 @@
     pub fn framework_metadata(&self) -> ::std::vec::Vec<&::aws_types::sdk_ua_metadata::FrameworkMetadata> {
         // `StoreAppend` loads entries newest-first; reverse to first-seen order so
         // this getter agrees with both the user agent and `SdkConfig::framework_metadata`.
-        let mut entries: ::std::vec::Vec<&::aws_types::sdk_ua_metadata::FrameworkMetadata> =
-            self.config.load::<::aws_types::sdk_ua_metadata::FrameworkMetadata>().collect();
+        let mut entries: ::std::vec::Vec<&::aws_types::sdk_ua_metadata::FrameworkMetadata> = self
+            .config
+            .load::<::aws_types::sdk_ua_metadata::FrameworkMetadata>()
+            .collect();
         entries.reverse();
         entries
     }
@@ -133,8 +145,12 @@
     /// Returns the invocation ID generator if one was given in config.
     ///
     /// The invocation ID generator generates ID values for the `amz-sdk-invocation-id` header. By default, this will be a random UUID. Overriding it may be useful in tests that examine the HTTP request and need to be deterministic.
-    pub fn invocation_id_generator(&self) -> ::std::option::Option<::aws_runtime::invocation_id::SharedInvocationIdGenerator> {
-        self.config.load::<::aws_runtime::invocation_id::SharedInvocationIdGenerator>().cloned()
+    pub fn invocation_id_generator(
+        &self,
+    ) -> ::std::option::Option<::aws_runtime::invocation_id::SharedInvocationIdGenerator> {
+        self.config
+            .load::<::aws_runtime::invocation_id::SharedInvocationIdGenerator>()
+            .cloned()
     }
     /// Creates a new [service config](crate::Config) from a [shared `config`](::aws_types::sdk_config::SdkConfig).
     pub fn new(config: &::aws_types::sdk_config::SdkConfig) -> Self {
@@ -197,18 +213,42 @@
     #[allow(unused)]
     pub(crate) fn from_config_bag(config_bag: &::aws_smithy_types::config_bag::ConfigBag) -> Self {
         let mut builder = Self::new();
-        builder.set_stalled_stream_protection(config_bag.load::<crate::config::StalledStreamProtectionConfig>().cloned());
-        builder.set_auth_scheme_preference(config_bag.load::<::aws_smithy_runtime_api::client::auth::AuthSchemePreference>().cloned());
+        builder.set_stalled_stream_protection(
+            config_bag
+                .load::<crate::config::StalledStreamProtectionConfig>()
+                .cloned(),
+        );
+        builder.set_auth_scheme_preference(
+            config_bag
+                .load::<::aws_smithy_runtime_api::client::auth::AuthSchemePreference>()
+                .cloned(),
+        );
         builder.set_retry_config(config_bag.load::<::aws_smithy_types::retry::RetryConfig>().cloned());
         builder.set_timeout_config(config_bag.load::<::aws_smithy_types::timeout::TimeoutConfig>().cloned());
-        builder.set_retry_partition(config_bag.load::<::aws_smithy_runtime::client::retries::RetryPartition>().cloned());
+        builder.set_retry_partition(
+            config_bag
+                .load::<::aws_smithy_runtime::client::retries::RetryPartition>()
+                .cloned(),
+        );
         builder.set_app_name(config_bag.load::<::aws_types::app_name::AppName>().cloned());
         for framework_metadata in config_bag.load::<::aws_types::sdk_ua_metadata::FrameworkMetadata>() {
             builder.push_framework_metadata(framework_metadata.clone());
         }
-        builder.set_endpoint_url(config_bag.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()));
-        builder.set_use_dual_stack(config_bag.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0));
-        builder.set_use_fips(config_bag.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0));
+        builder.set_endpoint_url(
+            config_bag
+                .load::<::aws_types::endpoint_config::EndpointUrl>()
+                .map(|ty| ty.0.clone()),
+        );
+        builder.set_use_dual_stack(
+            config_bag
+                .load::<::aws_types::endpoint_config::UseDualStack>()
+                .map(|ty| ty.0),
+        );
+        builder.set_use_fips(
+            config_bag
+                .load::<::aws_types::endpoint_config::UseFips>()
+                .map(|ty| ty.0),
+        );
         builder.set_sigv4a_signing_region_set(config_bag.load::<::aws_types::region::SigningRegionSet>().cloned());
         builder.set_region(config_bag.load::<crate::config::Region>().cloned());
         builder
@@ -225,7 +265,10 @@
     /// high-cardinality values (like object keys) fragment the metrics and inflate
     /// cost. Use [`Self::capture_input_attributes`] for values you want to read
     /// in-process without emitting them on the metrics.
-    pub fn emit_input_attributes(mut self, names: impl ::std::iter::IntoIterator<Item = impl ::std::convert::Into<::std::string::String>>) -> Self {
+    pub fn emit_input_attributes(
+        mut self,
+        names: impl ::std::iter::IntoIterator<Item = impl ::std::convert::Into<::std::string::String>>,
+    ) -> Self {
         let mut requested = self
             .config
             .load::<::aws_smithy_types::telemetry::RequestedTelemetryAttributes>()
@@ -258,7 +301,10 @@
     }
     /// Set the [`StalledStreamProtectionConfig`](crate::config::StalledStreamProtectionConfig)
     /// to configure protection for stalled streams.
-    pub fn stalled_stream_protection(mut self, stalled_stream_protection_config: crate::config::StalledStreamProtectionConfig) -> Self {
+    pub fn stalled_stream_protection(
+        mut self,
+        stalled_stream_protection_config: crate::config::StalledStreamProtectionConfig,
+    ) -> Self {
         self.set_stalled_stream_protection(::std::option::Option::Some(stalled_stream_protection_config));
         self
     }
@@ -301,7 +347,9 @@
     /// # }
     /// ```
     pub fn http_client(mut self, http_client: impl crate::config::HttpClient + 'static) -> Self {
-        self.set_http_client(::std::option::Option::Some(crate::config::IntoShared::into_shared(http_client)));
+        self.set_http_client(::std::option::Option::Some(crate::config::IntoShared::into_shared(
+            http_client,
+        )));
         self
     }

@@ -446,7 +494,10 @@
     ///     // other configurations
     ///     .build();
     /// ```
-    pub fn push_auth_scheme(mut self, auth_scheme: impl ::aws_smithy_runtime_api::client::auth::AuthScheme + 'static) -> Self {
+    pub fn push_auth_scheme(
+        mut self,
+        auth_scheme: impl ::aws_smithy_runtime_api::client::auth::AuthScheme + 'static,
+    ) -> Self {
         self.runtime_components.push_auth_scheme(auth_scheme);
         self
     }
@@ -481,7 +532,10 @@
     ///     // other configurations
     ///     .build();
     /// ```
-    pub fn auth_scheme_resolver(mut self, auth_scheme_resolver: impl crate::config::auth::ResolveAuthScheme + 'static) -> Self {
+    pub fn auth_scheme_resolver(
+        mut self,
+        auth_scheme_resolver: impl crate::config::auth::ResolveAuthScheme + 'static,
+    ) -> Self {
         self.set_auth_scheme_resolver(auth_scheme_resolver);
         self
     }
@@ -490,7 +544,10 @@
     ///
     /// # Examples
     /// See an example for [`Self::auth_scheme_resolver`].
-    pub fn set_auth_scheme_resolver(&mut self, auth_scheme_resolver: impl crate::config::auth::ResolveAuthScheme + 'static) -> &mut Self {
+    pub fn set_auth_scheme_resolver(
+        &mut self,
+        auth_scheme_resolver: impl crate::config::auth::ResolveAuthScheme + 'static,
+    ) -> &mut Self {
         self.runtime_components
             .set_auth_scheme_option_resolver(::std::option::Option::Some(auth_scheme_resolver.into_shared_resolver()));
         self
@@ -510,7 +567,9 @@
     /// This adds [NoAuthScheme](aws_smithy_runtime::client::auth::no_auth::NoAuthScheme) as a fallback
     /// and the auth scheme resolver will use it when no other auth schemes are applicable.
     pub fn set_allow_no_auth(&mut self) -> &mut Self {
-        self.push_runtime_plugin(::aws_smithy_runtime::client::auth::no_auth::NoAuthRuntimePluginV2::new().into_shared());
+        self.push_runtime_plugin(
+            ::aws_smithy_runtime::client::auth::no_auth::NoAuthRuntimePluginV2::new().into_shared(),
+        );
         self
     }
     /// Set the auth scheme preference for an auth scheme resolver
@@ -597,7 +656,10 @@
     /// let config = aws_sdk_sts::Config::builder().endpoint_resolver(resolver).build();
     /// let client = aws_sdk_sts::Client::from_conf(config);
     /// ```
-    pub fn endpoint_resolver(mut self, endpoint_resolver: impl crate::config::endpoint::ResolveEndpoint + 'static) -> Self {
+    pub fn endpoint_resolver(
+        mut self,
+        endpoint_resolver: impl crate::config::endpoint::ResolveEndpoint + 'static,
+    ) -> Self {
         self.set_endpoint_resolver(::std::option::Option::Some(endpoint_resolver.into_shared_resolver()));
         self
     }
@@ -655,7 +717,10 @@
     /// disable_retries(&mut builder);
     /// let config = builder.build();
     /// ```
-    pub fn set_retry_config(&mut self, retry_config: ::std::option::Option<::aws_smithy_types::retry::RetryConfig>) -> &mut Self {
+    pub fn set_retry_config(
+        &mut self,
+        retry_config: ::std::option::Option<::aws_smithy_types::retry::RetryConfig>,
+    ) -> &mut Self {
         retry_config.map(|r| self.config.store_put(r));
         self
     }
@@ -679,7 +744,9 @@
     /// let config = Config::builder().sleep_impl(sleep_impl).build();
     /// ```
     pub fn sleep_impl(mut self, sleep_impl: impl crate::config::AsyncSleep + 'static) -> Self {
-        self.set_sleep_impl(Some(::aws_smithy_runtime_api::shared::IntoShared::into_shared(sleep_impl)));
+        self.set_sleep_impl(Some(::aws_smithy_runtime_api::shared::IntoShared::into_shared(
+            sleep_impl,
+        )));
         self
     }

@@ -755,9 +822,14 @@
     /// set_request_timeout(&mut builder);
     /// let config = builder.build();
     /// ```
-    pub fn set_timeout_config(&mut self, timeout_config: ::std::option::Option<::aws_smithy_types::timeout::TimeoutConfig>) -> &mut Self {
+    pub fn set_timeout_config(
+        &mut self,
+        timeout_config: ::std::option::Option<::aws_smithy_types::timeout::TimeoutConfig>,
+    ) -> &mut Self {
         // passing None has no impact.
-        let Some(mut timeout_config) = timeout_config else { return self };
+        let Some(mut timeout_config) = timeout_config else {
+            return self;
+        };

         if let Some(base) = self.config.load::<::aws_smithy_types::timeout::TimeoutConfig>() {
             timeout_config.take_defaults_from(base);
@@ -926,8 +998,12 @@
     /// let client = aws_sdk_sts::Client::from_conf(config);
     /// ```
     ///
-    pub fn set_identity_cache(&mut self, identity_cache: impl crate::config::ResolveCachedIdentity + 'static) -> &mut Self {
-        self.runtime_components.set_identity_cache(::std::option::Option::Some(identity_cache));
+    pub fn set_identity_cache(
+        &mut self,
+        identity_cache: impl crate::config::ResolveCachedIdentity + 'static,
+    ) -> &mut Self {
+        self.runtime_components
+            .set_identity_cache(::std::option::Option::Some(identity_cache));
         self
     }
     /// Add an [interceptor](crate::config::Intercept) that runs at specific stages of the request execution pipeline.
@@ -989,19 +1065,25 @@
     }

     /// Set [`SharedInterceptor`](crate::config::SharedInterceptor)s for the builder.
-    pub fn set_interceptors(&mut self, interceptors: impl IntoIterator<Item = crate::config::SharedInterceptor>) -> &mut Self {
+    pub fn set_interceptors(
+        &mut self,
+        interceptors: impl IntoIterator<Item = crate::config::SharedInterceptor>,
+    ) -> &mut Self {
         self.runtime_components.set_interceptors(interceptors.into_iter());
         self
     }
     /// Sets the time source used for this service
     pub fn time_source(mut self, time_source: impl ::aws_smithy_async::time::TimeSource + 'static) -> Self {
-        self.set_time_source(::std::option::Option::Some(::aws_smithy_runtime_api::shared::IntoShared::into_shared(
-            time_source,
-        )));
+        self.set_time_source(::std::option::Option::Some(
+            ::aws_smithy_runtime_api::shared::IntoShared::into_shared(time_source),
+        ));
         self
     }
     /// Sets the time source used for this service
-    pub fn set_time_source(&mut self, time_source: ::std::option::Option<::aws_smithy_async::time::SharedTimeSource>) -> &mut Self {
+    pub fn set_time_source(
+        &mut self,
+        time_source: ::std::option::Option<::aws_smithy_async::time::SharedTimeSource>,
+    ) -> &mut Self {
         self.runtime_components.set_time_source(time_source);
         self
     }
@@ -1096,9 +1178,9 @@
         mut self,
         retry_classifier: impl ::aws_smithy_runtime_api::client::retries::classifiers::ClassifyRetry + 'static,
     ) -> Self {
-        self.push_retry_classifier(::aws_smithy_runtime_api::client::retries::classifiers::SharedRetryClassifier::new(
-            retry_classifier,
-        ));
+        self.push_retry_classifier(
+            ::aws_smithy_runtime_api::client::retries::classifiers::SharedRetryClassifier::new(retry_classifier),
+        );
         self
     }

@@ -1115,9 +1197,12 @@
     /// were previously set.
     pub fn set_retry_classifiers(
         &mut self,
-        retry_classifiers: impl IntoIterator<Item = ::aws_smithy_runtime_api::client::retries::classifiers::SharedRetryClassifier>,
+        retry_classifiers: impl IntoIterator<
+            Item = ::aws_smithy_runtime_api::client::retries::classifiers::SharedRetryClassifier,
+        >,
     ) -> &mut Self {
-        self.runtime_components.set_retry_classifiers(retry_classifiers.into_iter());
+        self.runtime_components
+            .set_retry_classifiers(retry_classifiers.into_iter());
         self
     }
     /// Sets the name of the app that is using the client.
@@ -1158,7 +1243,10 @@
     /// (as `lib/{name}/{version}`) so that libraries built on top of the AWS SDK can
     /// self-identify in the requests they make. Multiple entries may be added; each call
     /// appends another entry rather than replacing previous ones.
-    pub fn push_framework_metadata(&mut self, framework_metadata: ::aws_types::sdk_ua_metadata::FrameworkMetadata) -> &mut Self {
+    pub fn push_framework_metadata(
+        &mut self,
+        framework_metadata: ::aws_types::sdk_ua_metadata::FrameworkMetadata,
+    ) -> &mut Self {
         self.config.store_append(framework_metadata);
         self
     }
@@ -1165,7 +1253,10 @@
     /// Overrides the default invocation ID generator.
     ///
     /// The invocation ID generator generates ID values for the `amz-sdk-invocation-id` header. By default, this will be a random UUID. Overriding it may be useful in tests that examine the HTTP request and need to be deterministic.
-    pub fn invocation_id_generator(mut self, gen: impl ::aws_runtime::invocation_id::InvocationIdGenerator + 'static) -> Self {
+    pub fn invocation_id_generator(
+        mut self,
+        gen: impl ::aws_runtime::invocation_id::InvocationIdGenerator + 'static,
+    ) -> Self {
         self.set_invocation_id_generator(::std::option::Option::Some(
             ::aws_runtime::invocation_id::SharedInvocationIdGenerator::new(gen),
         ));
@@ -1196,7 +1287,8 @@
     /// will be prefixed onto this URL. To fully override the endpoint resolver, use
     /// [`Builder::endpoint_resolver`].
     pub fn set_endpoint_url(&mut self, endpoint_url: Option<::std::string::String>) -> &mut Self {
-        self.config.store_or_unset(endpoint_url.map(::aws_types::endpoint_config::EndpointUrl));
+        self.config
+            .store_or_unset(endpoint_url.map(::aws_types::endpoint_config::EndpointUrl));
         self
     }
     /// When true, use the dual-stack endpoint. If the configured endpoint does not support dual-stack, dispatching the request MAY return an error.
@@ -1206,7 +1298,8 @@
     }
     /// When true, use the dual-stack endpoint. If the configured endpoint does not support dual-stack, dispatching the request MAY return an error.
     pub fn set_use_dual_stack(&mut self, use_dual_stack: Option<bool>) -> &mut Self {
-        self.config.store_or_unset(use_dual_stack.map(::aws_types::endpoint_config::UseDualStack));
+        self.config
+            .store_or_unset(use_dual_stack.map(::aws_types::endpoint_config::UseDualStack));
         self
     }
     /// When true, send this request to the FIPS-compliant regional endpoint. If the configured endpoint does not have a FIPS compliant endpoint, dispatching the request will return an error.
@@ -1216,7 +1309,8 @@
     }
     /// When true, send this request to the FIPS-compliant regional endpoint. If the configured endpoint does not have a FIPS compliant endpoint, dispatching the request will return an error.
     pub fn set_use_fips(&mut self, use_fips: Option<bool>) -> &mut Self {
-        self.config.store_or_unset(use_fips.map(::aws_types::endpoint_config::UseFips));
+        self.config
+            .store_or_unset(use_fips.map(::aws_types::endpoint_config::UseFips));
         self
     }
     /// Sets the SigV4a signing region set.
@@ -1251,14 +1345,20 @@
         self
     }
     /// Sets the credentials provider for this service
-    pub fn credentials_provider(mut self, credentials_provider: impl crate::config::ProvideCredentials + 'static) -> Self {
-        self.set_credentials_provider(::std::option::Option::Some(crate::config::SharedCredentialsProvider::new(
-            credentials_provider,
-        )));
+    pub fn credentials_provider(
+        mut self,
+        credentials_provider: impl crate::config::ProvideCredentials + 'static,
+    ) -> Self {
+        self.set_credentials_provider(::std::option::Option::Some(
+            crate::config::SharedCredentialsProvider::new(credentials_provider),
+        ));
         self
     }
     /// Sets the credentials provider for this service
-    pub fn set_credentials_provider(&mut self, credentials_provider: ::std::option::Option<crate::config::SharedCredentialsProvider>) -> &mut Self {
+    pub fn set_credentials_provider(
+        &mut self,
+        credentials_provider: ::std::option::Option<crate::config::SharedCredentialsProvider>,
+    ) -> &mut Self {
         if let Some(credentials_provider) = credentials_provider {
             #[cfg(feature = "sigv4a")]
             {
@@ -1363,10 +1463,13 @@
     #[allow(unused_mut)]
     /// Apply test defaults to the builder. NOTE: Consider migrating to use `apply_test_defaults_v2` instead.
     pub fn apply_test_defaults(&mut self) -> &mut Self {
-        self.set_time_source(::std::option::Option::Some(::aws_smithy_async::time::SharedTimeSource::new(
-            ::aws_smithy_async::time::StaticTimeSource::new(::std::time::UNIX_EPOCH + ::std::time::Duration::from_secs(1234567890)),
-        )));
-        self.config.store_put(::aws_runtime::user_agent::AwsUserAgent::for_tests());
+        self.set_time_source(::std::option::Option::Some(
+            ::aws_smithy_async::time::SharedTimeSource::new(::aws_smithy_async::time::StaticTimeSource::new(
+                ::std::time::UNIX_EPOCH + ::std::time::Duration::from_secs(1234567890),
+            )),
+        ));
+        self.config
+            .store_put(::aws_runtime::user_agent::AwsUserAgent::for_tests());
         self.set_credentials_provider(Some(crate::config::SharedCredentialsProvider::new(
             ::aws_credential_types::Credentials::for_tests(),
         )));
@@ -1435,7 +1538,8 @@
             cfg.store_put(::aws_smithy_runtime::client::orchestrator::AuthSchemeAndEndpointOrchestrationV2);
             ::std::option::Option::Some(cfg.freeze())
         };
-        let mut runtime_components = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ServiceRuntimePlugin");
+        let mut runtime_components =
+            ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ServiceRuntimePlugin");
         runtime_components.set_auth_scheme_option_resolver(::std::option::Option::Some({
             use crate::config::auth::ResolveAuthScheme;
             crate::config::auth::DefaultAuthSchemeResolver::default().into_shared_resolver()
@@ -1444,22 +1548,32 @@
             use crate::config::endpoint::ResolveEndpoint;
             crate::config::endpoint::DefaultResolver::new().into_shared_resolver()
         }));
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
+        runtime_components.push_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                ::aws_smithy_runtime::client::http::connection_poisoning::ConnectionPoisoningInterceptor::new(),
+            ),
+        );
+        runtime_components.push_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::HttpStatusCodeClassifier::default(),
+        );
+        runtime_components.push_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                crate::sdk_feature_tracker::retry_mode::RetryModeFeatureTrackerInterceptor::new(),
+            ),
+        );
+        runtime_components.push_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                ::aws_runtime::service_clock_skew::ServiceClockSkewInterceptor::new(),
+            ),
+        );
         runtime_components.push_interceptor(::aws_runtime::request_info::RequestInfoInterceptor::new());
         runtime_components.push_interceptor(::aws_runtime::user_agent::UserAgentInterceptor::new());
         runtime_components.push_interceptor(::aws_runtime::invocation_id::InvocationIdInterceptor::new());
-        runtime_components.push_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-            ::aws_runtime::recursion_detection::RecursionDetectionInterceptor::new(),
-        ));
+        runtime_components.push_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                ::aws_runtime::recursion_detection::RecursionDetectionInterceptor::new(),
+            ),
+        );
         runtime_components.push_auth_scheme(::aws_smithy_runtime_api::client::auth::SharedAuthScheme::new(
             ::aws_runtime::auth::sigv4::SigV4AuthScheme::new(),
         ));
@@ -1469,13 +1583,20 @@
                 ::aws_runtime::auth::sigv4a::SigV4aAuthScheme::new(),
             ));
         }
-        runtime_components.push_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-            crate::config::endpoint::EndpointOverrideFeatureTrackerInterceptor,
-        ));
-        runtime_components.push_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-            crate::observability_feature::ObservabilityFeatureTrackerInterceptor,
-        ));
-        Self { config, runtime_components }
+        runtime_components.push_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                crate::config::endpoint::EndpointOverrideFeatureTrackerInterceptor,
+            ),
+        );
+        runtime_components.push_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                crate::observability_feature::ObservabilityFeatureTrackerInterceptor,
+            ),
+        );
+        Self {
+            config,
+            runtime_components,
+        }
     }
 }

@@ -1518,14 +1639,22 @@
         let mut layer = config_override.config;
         let mut components = config_override.runtime_components;
         #[allow(unused_mut)]
-        let mut resolver =
-            ::aws_smithy_runtime::client::config_override::Resolver::overrid(initial_config, initial_components, &mut layer, &mut components);
+        let mut resolver = ::aws_smithy_runtime::client::config_override::Resolver::overrid(
+            initial_config,
+            initial_components,
+            &mut layer,
+            &mut components,
+        );

         resolver
             .config_mut()
             .load::<::aws_types::region::Region>()
             .cloned()
-            .map(|r| resolver.config_mut().store_put(::aws_types::region::SigningRegion::from(r)));
+            .map(|r| {
+                resolver
+                    .config_mut()
+                    .store_put(::aws_types::region::SigningRegion::from(r))
+            });

         let _ = resolver;

@@ -1538,7 +1667,9 @@
         // This is skipped if the override already sets its own identity cache.
         if components.has_identity_resolvers() && components.identity_cache().is_none() {
             components.set_identity_cache(::std::option::Option::Some(
-                ::aws_smithy_runtime::client::identity::IdentityCache::lazy().max_partitions(1).build(),
+                ::aws_smithy_runtime::client::identity::IdentityCache::lazy()
+                    .max_partitions(1)
+                    .build(),
             ));
         }

@@ -1627,7 +1758,11 @@
 pub use ::aws_types::sdk_ua_metadata::FrameworkMetadata;

 #[allow(dead_code)]
-fn service_config_key<'a>(service_id: &'a str, env: &'a str, profile: &'a str) -> aws_types::service_config::ServiceConfigKey<'a> {
+fn service_config_key<'a>(
+    service_id: &'a str,
+    env: &'a str,
+    profile: &'a str,
+) -> aws_types::service_config::ServiceConfigKey<'a> {
     ::aws_types::service_config::ServiceConfigKey::builder()
         .service_id(service_id)
         .env(env)
@@ -1638,13 +1773,16 @@

 pub use ::aws_smithy_async::rt::sleep::Sleep;

-pub(crate) fn base_client_runtime_plugins(mut config: crate::Config) -> ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins {
+pub(crate) fn base_client_runtime_plugins(
+    mut config: crate::Config,
+) -> ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins {
     let mut configured_plugins = ::std::vec::Vec::new();
     ::std::mem::swap(&mut config.runtime_plugins, &mut configured_plugins);
     #[cfg(feature = "behavior-version-latest")]
     {
         if config.behavior_version.is_none() {
-            config.behavior_version = Some(::aws_smithy_runtime_api::client::behavior_version::BehaviorVersion::latest());
+            config.behavior_version =
+                Some(::aws_smithy_runtime_api::client::behavior_version::BehaviorVersion::latest());
         }
     }

diff --git a/src/config/auth.rs b/src/config/auth.rs
--- a/src/config/auth.rs
+++ b/src/config/auth.rs
@@ -31,10 +31,12 @@
         runtime_components: &'a ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponents,
     ) -> ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionsFuture<'a> {
         match params.get::<crate::config::auth::Params>() {
-            ::std::option::Option::Some(concrete_params) => self.0.resolve_auth_scheme(concrete_params, cfg, runtime_components),
-            ::std::option::Option::None => ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionsFuture::ready(::std::result::Result::Err(
-                "params of expected type was not present".into(),
-            )),
+            ::std::option::Option::Some(concrete_params) => {
+                self.0.resolve_auth_scheme(concrete_params, cfg, runtime_components)
+            }
+            ::std::option::Option::None => ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionsFuture::ready(
+                ::std::result::Result::Err("params of expected type was not present".into()),
+            ),
         }
     }
 }
@@ -44,7 +46,8 @@
 #[allow(dead_code)]
 pub struct DefaultAuthSchemeResolver {
     service_defaults: Vec<::aws_smithy_runtime_api::client::auth::AuthSchemeOption>,
-    operation_overrides: ::std::collections::HashMap<&'static str, Vec<::aws_smithy_runtime_api::client::auth::AuthSchemeOption>>,
+    operation_overrides:
+        ::std::collections::HashMap<&'static str, Vec<::aws_smithy_runtime_api::client::auth::AuthSchemeOption>>,
 }

 // TODO(https://github.com/smithy-lang/smithy-rs/issues/4177): Remove `allow(...)` once the issue is addressed.
@@ -101,7 +104,8 @@
             None => &self.service_defaults,
         };

-        let _fut = ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionsFuture::ready(Ok(modeled_auth_options.clone()));
+        let _fut =
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionsFuture::ready(Ok(modeled_auth_options.clone()));

         _fut
     }
@@ -136,7 +140,10 @@
     }

     /// Set the operation name for the builder
-    pub fn set_operation_name(mut self, operation_name: ::std::option::Option<::std::borrow::Cow<'static, str>>) -> Self {
+    pub fn set_operation_name(
+        mut self,
+        operation_name: ::std::option::Option<::std::borrow::Cow<'static, str>>,
+    ) -> Self {
         self.operation_name = operation_name;
         self
     }
@@ -146,7 +153,9 @@
     ///
     pub fn build(self) -> ::std::result::Result<crate::config::auth::Params, crate::config::auth::BuildError> {
         ::std::result::Result::Ok(crate::config::auth::Params {
-            operation_name: self.operation_name.ok_or_else(|| BuildError::missing("operation_name"))?,
+            operation_name: self
+                .operation_name
+                .ok_or_else(|| BuildError::missing("operation_name"))?,
         })
     }
 }
diff --git a/src/config/endpoint.rs b/src/config/endpoint.rs
--- a/src/config/endpoint.rs
+++ b/src/config/endpoint.rs
@@ -15,7 +15,9 @@

     fn read_before_execution(
         &self,
-        _context: &::aws_smithy_runtime_api::client::interceptors::context::BeforeSerializationInterceptorContextRef<'_>,
+        _context: &::aws_smithy_runtime_api::client::interceptors::context::BeforeSerializationInterceptorContextRef<
+            '_,
+        >,
         cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
     ) -> ::std::result::Result<(), ::aws_smithy_runtime_api::box_error::BoxError> {
         if cfg.load::<::aws_types::endpoint_config::EndpointUrl>().is_some() {
@@ -665,7 +667,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://sts-fips.cn-north-1.api.amazonwebservices.com.cn");
+        let endpoint =
+            endpoint.expect("Expected valid endpoint: https://sts-fips.cn-north-1.api.amazonwebservices.com.cn");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -949,7 +952,9 @@
         let endpoint = endpoint.expect("Expected valid endpoint: https://example.com");
         assert_eq!(
             endpoint,
-            ::aws_smithy_types::endpoint::Endpoint::builder().url("https://example.com").build()
+            ::aws_smithy_types::endpoint::Endpoint::builder()
+                .url("https://example.com")
+                .build()
         );
     }

@@ -967,7 +972,9 @@
         let endpoint = endpoint.expect("Expected valid endpoint: https://example.com");
         assert_eq!(
             endpoint,
-            ::aws_smithy_types::endpoint::Endpoint::builder().url("https://example.com").build()
+            ::aws_smithy_types::endpoint::Endpoint::builder()
+                .url("https://example.com")
+                .build()
         );
     }

@@ -984,7 +991,10 @@
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
         let error = endpoint.expect_err("expected error: Invalid Configuration: FIPS and custom endpoint are not supported [For custom endpoint with fips enabled and dualstack disabled]");
-        assert_eq!(format!("{}", error), "Invalid Configuration: FIPS and custom endpoint are not supported")
+        assert_eq!(
+            format!("{}", error),
+            "Invalid Configuration: FIPS and custom endpoint are not supported"
+        )
     }

     /// For custom endpoint with fips disabled and dualstack enabled
@@ -1009,7 +1019,9 @@
     /// Missing region
     #[test]
     fn test_50() {
-        let params = crate::config::endpoint::Params::builder().build().expect("invalid params");
+        let params = crate::config::endpoint::Params::builder()
+            .build()
+            .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
         let error = endpoint.expect_err("expected error: Invalid Configuration: Missing Region [Missing region]");
@@ -1474,7 +1486,9 @@
         let endpoint = endpoint.expect("Expected valid endpoint: https://example.com");
         assert_eq!(
             endpoint,
-            ::aws_smithy_types::endpoint::Endpoint::builder().url("https://example.com").build()
+            ::aws_smithy_types::endpoint::Endpoint::builder()
+                .url("https://example.com")
+                .build()
         );
     }

@@ -1493,7 +1507,9 @@
         let endpoint = endpoint.expect("Expected valid endpoint: https://example.com");
         assert_eq!(
             endpoint,
-            ::aws_smithy_types::endpoint::Endpoint::builder().url("https://example.com").build()
+            ::aws_smithy_types::endpoint::Endpoint::builder()
+                .url("https://example.com")
+                .build()
         );
     }
 }
@@ -1501,7 +1517,10 @@
 /// Endpoint resolver trait specific to AWS Security Token Service
 pub trait ResolveEndpoint: ::std::marker::Send + ::std::marker::Sync + ::std::fmt::Debug {
     /// Resolve an endpoint with the given parameters
-    fn resolve_endpoint<'a>(&'a self, params: &'a crate::config::endpoint::Params) -> ::aws_smithy_runtime_api::client::endpoint::EndpointFuture<'a>;
+    fn resolve_endpoint<'a>(
+        &'a self,
+        params: &'a crate::config::endpoint::Params,
+    ) -> ::aws_smithy_runtime_api::client::endpoint::EndpointFuture<'a>;

     /// Convert this service-specific resolver into a `SharedEndpointResolver`
     ///
@@ -1526,7 +1545,9 @@
     ) -> ::aws_smithy_runtime_api::client::endpoint::EndpointFuture<'a> {
         let ep = match params.get::<crate::config::endpoint::Params>() {
             Some(params) => self.0.resolve_endpoint(params),
-            None => ::aws_smithy_runtime_api::client::endpoint::EndpointFuture::ready(Err("params of expected type was not present".into())),
+            None => ::aws_smithy_runtime_api::client::endpoint::EndpointFuture::ready(Err(
+                "params of expected type was not present".into(),
+            )),
         };
         ep
     }
@@ -1570,7 +1591,8 @@
     fn resolve_endpoint<'a>(
         &'a self,
         params: &'a crate::config::endpoint::Params,
-    ) -> ::std::result::Result<::aws_smithy_types::endpoint::Endpoint, ::aws_smithy_runtime_api::box_error::BoxError> {
+    ) -> ::std::result::Result<::aws_smithy_types::endpoint::Endpoint, ::aws_smithy_runtime_api::box_error::BoxError>
+    {
         let mut _diagnostic_collector = crate::endpoint_lib::diagnostic::DiagnosticCollector::new();
         #[allow(unused_mut)]
         let mut context = ConditionContext::default();
@@ -1587,22 +1609,29 @@
             match current_ref {
                 ref_val if ref_val >= 100_000_000 => {
                     return match (ref_val - 100_000_000) as usize {
-                        0 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
-                            "No endpoint rule matched",
-                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        0 => ::std::result::Result::Err(Box::new(
+                            ::aws_smithy_http::endpoint::ResolveEndpointError::message("No endpoint rule matched"),
+                        )
+                            as ::aws_smithy_runtime_api::box_error::BoxError),
                         1 => ::std::result::Result::Ok(
                             ::aws_smithy_types::endpoint::Endpoint::builder()
                                 .url("https://sts.amazonaws.com".to_string())
                                 .auth_scheme(
-                                    ::aws_smithy_types::endpoint::EndpointAuthScheme::with_capacity("sigv4".to_string(), 2)
-                                        .put("signingName", "sts")
-                                        .put("signingRegion", "us-east-1"),
+                                    ::aws_smithy_types::endpoint::EndpointAuthScheme::with_capacity(
+                                        "sigv4".to_string(),
+                                        2,
+                                    )
+                                    .put("signingName", "sts")
+                                    .put("signingRegion", "us-east-1"),
                                 )
                                 .build(),
                         ),
                         2 => {
                             let region = params.region.as_deref().unwrap_or_default();
-                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            let partition_result = context
+                                .partition_result
+                                .as_ref()
+                                .expect("Guaranteed to have a value by earlier checks.");
                             ::std::result::Result::Ok(
                                 ::aws_smithy_types::endpoint::Endpoint::builder()
                                     .url({
@@ -1616,26 +1645,42 @@
                                         out
                                     })
                                     .auth_scheme(
-                                        ::aws_smithy_types::endpoint::EndpointAuthScheme::with_capacity("sigv4".to_string(), 2)
-                                            .put("signingName", "sts")
-                                            .put("signingRegion", region.as_ref()),
+                                        ::aws_smithy_types::endpoint::EndpointAuthScheme::with_capacity(
+                                            "sigv4".to_string(),
+                                            2,
+                                        )
+                                        .put("signingName", "sts")
+                                        .put("signingRegion", region.as_ref()),
                                     )
                                     .build(),
                             )
                         }
-                        3 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
-                            "Invalid Configuration: FIPS and custom endpoint are not supported".to_string(),
-                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
-                        4 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
-                            "Invalid Configuration: Dualstack and custom endpoint are not supported".to_string(),
-                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        3 => ::std::result::Result::Err(Box::new(
+                            ::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                                "Invalid Configuration: FIPS and custom endpoint are not supported".to_string(),
+                            ),
+                        )
+                            as ::aws_smithy_runtime_api::box_error::BoxError),
+                        4 => ::std::result::Result::Err(Box::new(
+                            ::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                                "Invalid Configuration: Dualstack and custom endpoint are not supported".to_string(),
+                            ),
+                        )
+                            as ::aws_smithy_runtime_api::box_error::BoxError),
                         5 => {
                             let endpoint = params.endpoint.as_deref().unwrap_or_default();
-                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url(endpoint.to_owned()).build())
+                            ::std::result::Result::Ok(
+                                ::aws_smithy_types::endpoint::Endpoint::builder()
+                                    .url(endpoint.to_owned())
+                                    .build(),
+                            )
                         }
                         6 => {
                             let region = params.region.as_deref().unwrap_or_default();
-                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            let partition_result = context
+                                .partition_result
+                                .as_ref()
+                                .expect("Guaranteed to have a value by earlier checks.");
                             ::std::result::Result::Ok(
                                 ::aws_smithy_types::endpoint::Endpoint::builder()
                                     .url({
@@ -1651,9 +1696,13 @@
                                     .build(),
                             )
                         }
-                        7 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
-                            "FIPS and DualStack are enabled, but this partition does not support one or both".to_string(),
-                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        7 => ::std::result::Result::Err(Box::new(
+                            ::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                                "FIPS and DualStack are enabled, but this partition does not support one or both"
+                                    .to_string(),
+                            ),
+                        )
+                            as ::aws_smithy_runtime_api::box_error::BoxError),
                         8 => {
                             let region = params.region.as_deref().unwrap_or_default();
                             ::std::result::Result::Ok(
@@ -1671,7 +1720,10 @@
                         }
                         9 => {
                             let region = params.region.as_deref().unwrap_or_default();
-                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            let partition_result = context
+                                .partition_result
+                                .as_ref()
+                                .expect("Guaranteed to have a value by earlier checks.");
                             ::std::result::Result::Ok(
                                 ::aws_smithy_types::endpoint::Endpoint::builder()
                                     .url({
@@ -1687,12 +1739,18 @@
                                     .build(),
                             )
                         }
-                        10 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
-                            "FIPS is enabled but this partition does not support FIPS".to_string(),
-                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        10 => ::std::result::Result::Err(Box::new(
+                            ::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                                "FIPS is enabled but this partition does not support FIPS".to_string(),
+                            ),
+                        )
+                            as ::aws_smithy_runtime_api::box_error::BoxError),
                         11 => {
                             let region = params.region.as_deref().unwrap_or_default();
-                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            let partition_result = context
+                                .partition_result
+                                .as_ref()
+                                .expect("Guaranteed to have a value by earlier checks.");
                             ::std::result::Result::Ok(
                                 ::aws_smithy_types::endpoint::Endpoint::builder()
                                     .url({
@@ -1708,12 +1766,18 @@
                                     .build(),
                             )
                         }
-                        12 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
-                            "DualStack is enabled but this partition does not support DualStack".to_string(),
-                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        12 => ::std::result::Result::Err(Box::new(
+                            ::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                                "DualStack is enabled but this partition does not support DualStack".to_string(),
+                            ),
+                        )
+                            as ::aws_smithy_runtime_api::box_error::BoxError),
                         13 => {
                             let region = params.region.as_deref().unwrap_or_default();
-                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            let partition_result = context
+                                .partition_result
+                                .as_ref()
+                                .expect("Guaranteed to have a value by earlier checks.");
                             ::std::result::Result::Ok(
                                 ::aws_smithy_types::endpoint::Endpoint::builder()
                                     .url({
@@ -1729,19 +1793,23 @@
                                     .build(),
                             )
                         }
-                        14 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
-                            "Invalid Configuration: Missing Region".to_string(),
-                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
-                        _ => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
-                            "No endpoint rule matched",
-                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        14 => ::std::result::Result::Err(Box::new(
+                            ::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                                "Invalid Configuration: Missing Region".to_string(),
+                            ),
+                        )
+                            as ::aws_smithy_runtime_api::box_error::BoxError),
+                        _ => ::std::result::Result::Err(Box::new(
+                            ::aws_smithy_http::endpoint::ResolveEndpointError::message("No endpoint rule matched"),
+                        )
+                            as ::aws_smithy_runtime_api::box_error::BoxError),
                     };
                 }
                 1 | -1 => {
-                    return ::std::result::Result::Err(
-                        Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message("No endpoint rule matched"))
-                            as ::aws_smithy_runtime_api::box_error::BoxError,
+                    return ::std::result::Result::Err(Box::new(
+                        ::aws_smithy_http::endpoint::ResolveEndpointError::message("No endpoint rule matched"),
                     )
+                        as ::aws_smithy_runtime_api::box_error::BoxError)
                 }
                 ref_val => {
                     let is_complement = ref_val < 0;
@@ -1855,7 +1923,11 @@
                         })(&mut _diagnostic_collector),
                         _ => unreachable!("Invalid condition index"),
                     };
-                    current_ref = if is_complement ^ condition_result { node.high_ref } else { node.low_ref };
+                    current_ref = if is_complement ^ condition_result {
+                        node.high_ref
+                    } else {
+                        node.low_ref
+                    };
                 }
             }
         }
@@ -1863,18 +1935,24 @@
 }

 impl crate::config::endpoint::ResolveEndpoint for DefaultResolver {
-    fn resolve_endpoint<'a>(&'a self, params: &'a crate::config::endpoint::Params) -> ::aws_smithy_runtime_api::client::endpoint::EndpointFuture<'a> {
+    fn resolve_endpoint<'a>(
+        &'a self,
+        params: &'a crate::config::endpoint::Params,
+    ) -> ::aws_smithy_runtime_api::client::endpoint::EndpointFuture<'a> {
         // Check single-entry cache (lock-free read via ArcSwap)
         let cached = self.endpoint_cache.load();
         if let Some((cached_params, cached_endpoint)) = cached.as_ref() {
             if cached_params == params {
-                return ::aws_smithy_runtime_api::client::endpoint::EndpointFuture::ready(::std::result::Result::Ok(cached_endpoint.clone()));
+                return ::aws_smithy_runtime_api::client::endpoint::EndpointFuture::ready(::std::result::Result::Ok(
+                    cached_endpoint.clone(),
+                ));
             }
         }
         drop(cached);
         let result = self.resolve_endpoint(params);
         if let ::std::result::Result::Ok(ref endpoint) = result {
-            self.endpoint_cache.store(::std::sync::Arc::new(Some((params.clone(), endpoint.clone()))));
+            self.endpoint_cache
+                .store(::std::sync::Arc::new(Some((params.clone(), endpoint.clone()))));
         }
         ::aws_smithy_runtime_api::client::endpoint::EndpointFuture::ready(result)
     }
@@ -2099,7 +2177,9 @@
 }
 impl ParamsBuilder {
     /// Consume this builder, creating [`Params`].
-    pub fn build(self) -> ::std::result::Result<crate::config::endpoint::Params, crate::config::endpoint::InvalidParams> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<crate::config::endpoint::Params, crate::config::endpoint::InvalidParams> {
         if let Some(region) = &self.region {
             if !crate::endpoint_lib::host::is_valid_host_label(
                 region.as_ref() as &str,
@@ -2251,7 +2331,9 @@
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
         match self.kind {
             InvalidParamsErrorKind::MissingField => write!(f, "a required field was missing: `{}`", self.field),
-            InvalidParamsErrorKind::InvalidValue { message } => write!(f, "invalid value for field: `{}` - {}", self.field, message),
+            InvalidParamsErrorKind::InvalidValue { message } => {
+                write!(f, "invalid value for field: `{}` - {}", self.field, message)
+            }
         }
     }
 }
diff --git a/src/endpoint_lib.rs b/src/endpoint_lib.rs
--- a/src/endpoint_lib.rs
+++ b/src/endpoint_lib.rs
@@ -1,19 +1,20 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 // Loading the partition JSON is expensive since it involves many regex compilations,
 // so cache the result so that it only need to be paid for the first constructed client.
-pub(crate) static DEFAULT_PARTITION_RESOLVER: std::sync::LazyLock<crate::endpoint_lib::partition::PartitionResolver> = std::sync::LazyLock::new(
-    || match std::env::var("SMITHY_CLIENT_SDK_CUSTOM_PARTITION") {
+pub(crate) static DEFAULT_PARTITION_RESOLVER: std::sync::LazyLock<crate::endpoint_lib::partition::PartitionResolver> =
+    std::sync::LazyLock::new(|| match std::env::var("SMITHY_CLIENT_SDK_CUSTOM_PARTITION") {
         Ok(partitions) => {
             ::tracing::debug!("loading custom partitions located at {partitions}");
-            let partition_dot_json = std::fs::read_to_string(partitions).expect("should be able to read a custom partition JSON");
-            crate::endpoint_lib::partition::PartitionResolver::new_from_json(partition_dot_json.as_bytes()).expect("valid JSON")
+            let partition_dot_json =
+                std::fs::read_to_string(partitions).expect("should be able to read a custom partition JSON");
+            crate::endpoint_lib::partition::PartitionResolver::new_from_json(partition_dot_json.as_bytes())
+                .expect("valid JSON")
         }
         _ => {
             ::tracing::debug!("loading default partitions");
             crate::endpoint_lib::partition::PartitionResolver::new_from_json(b"{\"partitions\":[{\"id\":\"aws\",\"outputs\":{\"dnsSuffix\":\"amazonaws.com\",\"dualStackDnsSuffix\":\"api.aws\",\"implicitGlobalRegion\":\"us-east-1\",\"name\":\"aws\",\"supportsDualStack\":true,\"supportsFIPS\":true},\"regionRegex\":\"^(us|eu|ap|sa|ca|me|af|il|mx)\\\\-\\\\w+\\\\-\\\\d+$\",\"regions\":{\"af-south-1\":{\"description\":\"Africa (Cape Town)\"},\"ap-east-1\":{\"description\":\"Asia Pacific (Hong Kong)\"},\"ap-east-2\":{\"description\":\"Asia Pacific (Taipei)\"},\"ap-northeast-1\":{\"description\":\"Asia Pacific (Tokyo)\"},\"ap-northeast-2\":{\"description\":\"Asia Pacific (Seoul)\"},\"ap-northeast-3\":{\"description\":\"Asia Pacific (Osaka)\"},\"ap-south-1\":{\"description\":\"Asia Pacific (Mumbai)\"},\"ap-south-2\":{\"description\":\"Asia Pacific (Hyderabad)\"},\"ap-southeast-1\":{\"description\":\"Asia Pacific (Singapore)\"},\"ap-southeast-2\":{\"description\":\"Asia Pacific (Sydney)\"},\"ap-southeast-3\":{\"description\":\"Asia Pacific (Jakarta)\"},\"ap-southeast-4\":{\"description\":\"Asia Pacific (Melbourne)\"},\"ap-southeast-5\":{\"description\":\"Asia Pacific (Malaysia)\"},\"ap-southeast-6\":{\"description\":\"Asia Pacific (New Zealand)\"},\"ap-southeast-7\":{\"description\":\"Asia Pacific (Thailand)\"},\"aws-global\":{\"description\":\"aws global region\"},\"ca-central-1\":{\"description\":\"Canada (Central)\"},\"ca-west-1\":{\"description\":\"Canada West (Calgary)\"},\"eu-central-1\":{\"description\":\"Europe (Frankfurt)\"},\"eu-central-2\":{\"description\":\"Europe (Zurich)\"},\"eu-north-1\":{\"description\":\"Europe (Stockholm)\"},\"eu-south-1\":{\"description\":\"Europe (Milan)\"},\"eu-south-2\":{\"description\":\"Europe (Spain)\"},\"eu-west-1\":{\"description\":\"Europe (Ireland)\"},\"eu-west-2\":{\"description\":\"Europe (London)\"},\"eu-west-3\":{\"description\":\"Europe (Paris)\"},\"il-central-1\":{\"description\":\"Israel (Tel Aviv)\"},\"me-central-1\":{\"description\":\"Middle East (UAE)\"},\"me-south-1\":{\"description\":\"Middle East (Bahrain)\"},\"mx-central-1\":{\"description\":\"Mexico (Central)\"},\"sa-east-1\":{\"description\":\"South America (Sao Paulo)\"},\"us-east-1\":{\"description\":\"US East (N. Virginia)\"},\"us-east-2\":{\"description\":\"US East (Ohio)\"},\"us-west-1\":{\"description\":\"US West (N. California)\"},\"us-west-2\":{\"description\":\"US West (Oregon)\"}}},{\"id\":\"aws-cn\",\"outputs\":{\"dnsSuffix\":\"amazonaws.com.cn\",\"dualStackDnsSuffix\":\"api.amazonwebservices.com.cn\",\"implicitGlobalRegion\":\"cn-northwest-1\",\"name\":\"aws-cn\",\"supportsDualStack\":true,\"supportsFIPS\":true},\"regionRegex\":\"^cn\\\\-\\\\w+\\\\-\\\\d+$\",\"regions\":{\"aws-cn-global\":{\"description\":\"aws-cn global region\"},\"cn-north-1\":{\"description\":\"China (Beijing)\"},\"cn-northwest-1\":{\"description\":\"China (Ningxia)\"}}},{\"id\":\"aws-eusc\",\"outputs\":{\"dnsSuffix\":\"amazonaws.eu\",\"dualStackDnsSuffix\":\"api.amazonwebservices.eu\",\"implicitGlobalRegion\":\"eusc-de-east-1\",\"name\":\"aws-eusc\",\"supportsDualStack\":true,\"supportsFIPS\":true},\"regionRegex\":\"^eusc\\\\-(de)\\\\-\\\\w+\\\\-\\\\d+$\",\"regions\":{\"eusc-de-east-1\":{\"description\":\"AWS European Sovereign Cloud (Germany)\"}}},{\"id\":\"aws-iso\",\"outputs\":{\"dnsSuffix\":\"c2s.ic.gov\",\"dualStackDnsSuffix\":\"api.aws.ic.gov\",\"implicitGlobalRegion\":\"us-iso-east-1\",\"name\":\"aws-iso\",\"supportsDualStack\":true,\"supportsFIPS\":true},\"regionRegex\":\"^us\\\\-iso\\\\-\\\\w+\\\\-\\\\d+$\",\"regions\":{\"aws-iso-global\":{\"description\":\"aws-iso global region\"},\"us-iso-east-1\":{\"description\":\"US ISO East\"},\"us-iso-west-1\":{\"description\":\"US ISO WEST\"}}},{\"id\":\"aws-iso-b\",\"outputs\":{\"dnsSuffix\":\"sc2s.sgov.gov\",\"dualStackDnsSuffix\":\"api.aws.scloud\",\"implicitGlobalRegion\":\"us-isob-east-1\",\"name\":\"aws-iso-b\",\"supportsDualStack\":true,\"supportsFIPS\":true},\"regionRegex\":\"^us\\\\-isob\\\\-\\\\w+\\\\-\\\\d+$\",\"regions\":{\"aws-iso-b-global\":{\"description\":\"aws-iso-b global region\"},\"us-isob-east-1\":{\"description\":\"US ISOB East (Ohio)\"},\"us-isob-west-1\":{\"description\":\"US ISOB West\"}}},{\"id\":\"aws-iso-e\",\"outputs\":{\"dnsSuffix\":\"cloud.adc-e.uk\",\"dualStackDnsSuffix\":\"api.cloud-aws.adc-e.uk\",\"implicitGlobalRegion\":\"eu-isoe-west-1\",\"name\":\"aws-iso-e\",\"supportsDualStack\":true,\"supportsFIPS\":true},\"regionRegex\":\"^eu\\\\-isoe\\\\-\\\\w+\\\\-\\\\d+$\",\"regions\":{\"aws-iso-e-global\":{\"description\":\"aws-iso-e global region\"},\"eu-isoe-west-1\":{\"description\":\"EU ISOE West\"}}},{\"id\":\"aws-iso-f\",\"outputs\":{\"dnsSuffix\":\"csp.hci.ic.gov\",\"dualStackDnsSuffix\":\"api.aws.hci.ic.gov\",\"implicitGlobalRegion\":\"us-isof-south-1\",\"name\":\"aws-iso-f\",\"supportsDualStack\":true,\"supportsFIPS\":true},\"regionRegex\":\"^us\\\\-isof\\\\-\\\\w+\\\\-\\\\d+$\",\"regions\":{\"aws-iso-f-global\":{\"description\":\"aws-iso-f global region\"},\"us-isof-east-1\":{\"description\":\"US ISOF EAST\"},\"us-isof-south-1\":{\"description\":\"US ISOF SOUTH\"}}},{\"id\":\"aws-us-gov\",\"outputs\":{\"dnsSuffix\":\"amazonaws.com\",\"dualStackDnsSuffix\":\"api.aws\",\"implicitGlobalRegion\":\"us-gov-west-1\",\"name\":\"aws-us-gov\",\"supportsDualStack\":true,\"supportsFIPS\":true},\"regionRegex\":\"^us\\\\-gov\\\\-\\\\w+\\\\-\\\\d+$\",\"regions\":{\"aws-us-gov-global\":{\"description\":\"aws-us-gov global region\"},\"us-gov-east-1\":{\"description\":\"AWS GovCloud (US-East)\"},\"us-gov-west-1\":{\"description\":\"AWS GovCloud (US-West)\"}}}],\"version\":\"1.1\"}").expect("valid JSON")
         }
-    },
-);
+    });

 pub(crate) mod bdd_interpreter;

diff --git a/src/endpoint_lib/bdd_interpreter.rs b/src/endpoint_lib/bdd_interpreter.rs
--- a/src/endpoint_lib/bdd_interpreter.rs
+++ b/src/endpoint_lib/bdd_interpreter.rs
@@ -43,7 +43,12 @@
     params: &'a Params,
     context: &mut Context,
     diagnostic_collector: &mut crate::endpoint_lib::diagnostic::DiagnosticCollector,
-    mut condition_evaluator: impl FnMut(&Cond, &'a Params, &mut Context, &mut crate::endpoint_lib::diagnostic::DiagnosticCollector) -> bool,
+    mut condition_evaluator: impl FnMut(
+        &Cond,
+        &'a Params,
+        &mut Context,
+        &mut crate::endpoint_lib::diagnostic::DiagnosticCollector,
+    ) -> bool,
     result_builder: impl FnOnce(usize, &'a Params, &Context) -> R,
 ) -> Option<R> {
     let mut current_ref = root_ref;
@@ -68,7 +73,11 @@
                 let condition = conditions.get(condition_index)?;
                 let condition_result = condition_evaluator(condition, params, context, diagnostic_collector);
                 // Handle complement edges: complement inverts the branch selection
-                current_ref = if is_complement ^ condition_result { node.high_ref } else { node.low_ref };
+                current_ref = if is_complement ^ condition_result {
+                    node.high_ref
+                } else {
+                    node.low_ref
+                };
             }
         }
     }
diff --git a/src/endpoint_lib/host.rs b/src/endpoint_lib/host.rs
--- a/src/endpoint_lib/host.rs
+++ b/src/endpoint_lib/host.rs
@@ -63,7 +63,10 @@
         assert_eq!(is_valid_host_label("a.b.c", true), true);
         assert_eq!(is_valid_host_label("a_b", true), false);
         assert_eq!(is_valid_host_label(&"a".repeat(64), false), false);
-        assert_eq!(is_valid_host_label(&format!("{}.{}", "a".repeat(63), "a".repeat(63)), true), true);
+        assert_eq!(
+            is_valid_host_label(&format!("{}.{}", "a".repeat(63), "a".repeat(63)), true),
+            true
+        );
     }

     #[allow(clippy::bool_assert_comparison)]
diff --git a/src/endpoint_lib/partition.rs b/src/endpoint_lib/partition.rs
--- a/src/endpoint_lib/partition.rs
+++ b/src/endpoint_lib/partition.rs
@@ -79,7 +79,11 @@
 /// Merge the base output and the override output, dealing with `Cow`s
 macro_rules! merge {
     ($base: expr, $output: expr, $field: ident) => {
-        $output.$field.as_ref().map(|s| s.as_ref()).unwrap_or($base.outputs.$field.as_ref())
+        $output
+            .$field
+            .as_ref()
+            .map(|s| s.as_ref())
+            .unwrap_or($base.outputs.$field.as_ref())
     };
 }

@@ -116,15 +120,16 @@
         let mut explicit_match_partition = self.partitions.iter().flat_map(|part| part.explicit_match(region));
         let mut regex_match_partition = self.partitions.iter().flat_map(|part| part.regex_match(region));

-        let (base, region_override) = explicit_match_partition.next().or_else(|| regex_match_partition.next()).or_else(|| {
-            match self.partitions.iter().find(|p| p.id == "aws") {
+        let (base, region_override) = explicit_match_partition
+            .next()
+            .or_else(|| regex_match_partition.next())
+            .or_else(|| match self.partitions.iter().find(|p| p.id == "aws") {
                 Some(partition) => Some((partition, None)),
                 None => {
                     e.report_error("no AWS partition!");
                     None
                 }
-            }
-        })?;
+            })?;
         let region_override = region_override.as_ref().unwrap_or(&DEFAULT_OVERRIDE);
         Some(Partition {
             name: merge!(base, region_override, name),
@@ -131,7 +136,9 @@
             dns_suffix: merge!(base, region_override, dns_suffix),
             dual_stack_dns_suffix: merge!(base, region_override, dual_stack_dns_suffix),
             supports_fips: region_override.supports_fips.unwrap_or(base.outputs.supports_fips),
-            supports_dual_stack: region_override.supports_dual_stack.unwrap_or(base.outputs.supports_dual_stack),
+            supports_dual_stack: region_override
+                .supports_dual_stack
+                .unwrap_or(base.outputs.supports_dual_stack),
             implicit_global_region: merge!(base, region_override, implicit_global_region),
         })
     }
@@ -172,7 +179,9 @@

 impl PartitionMetadata {
     fn explicit_match(&self, region: &str) -> Option<(&PartitionMetadata, Option<&PartitionOutputOverride>)> {
-        self.regions.get(region).map(|output_override| (self, Some(output_override)))
+        self.regions
+            .get(region)
+            .map(|output_override| (self, Some(output_override)))
     }

     fn regex_match(&self, region: &str) -> Option<(&PartitionMetadata, Option<&PartitionOutputOverride>)> {
@@ -221,8 +230,12 @@
 ///
 /// This code was generated by smithy-rs and then hand edited for clarity
 mod deser {
-    use crate::endpoint_lib::partition::{PartitionMetadata, PartitionMetadataBuilder, PartitionOutputOverride, PartitionResolver};
-    use aws_smithy_json::deserialize::token::{expect_bool_or_null, expect_start_object, expect_string_or_null, skip_value};
+    use crate::endpoint_lib::partition::{
+        PartitionMetadata, PartitionMetadataBuilder, PartitionOutputOverride, PartitionResolver,
+    };
+    use aws_smithy_json::deserialize::token::{
+        expect_bool_or_null, expect_start_object, expect_string_or_null, skip_value,
+    };
     use aws_smithy_json::deserialize::{error::DeserializeError, json_token_iter, Token};
     use regex_lite::Regex;
     use std::borrow::Cow;
@@ -242,11 +255,17 @@
                     }
                     _ => skip_value(tokens)?,
                 },
-                other => return Err(DeserializeError::custom(format!("expected object key or end object, found: {other:?}",))),
+                other => {
+                    return Err(DeserializeError::custom(format!(
+                        "expected object key or end object, found: {other:?}",
+                    )))
+                }
             }
         }
         if tokens.next().is_some() {
-            return Err(DeserializeError::custom("found more JSON tokens after completing parsing"));
+            return Err(DeserializeError::custom(
+                "found more JSON tokens after completing parsing",
+            ));
         }
         resolver.ok_or_else(|| DeserializeError::custom("did not find partitions array"))
     }
@@ -275,7 +294,9 @@
         }
     }

-    pub(crate) fn deser_partition<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<PartitionMetadata, DeserializeError>
+    pub(crate) fn deser_partition<'a, I>(
+        tokens: &mut std::iter::Peekable<I>,
+    ) -> Result<PartitionMetadata, DeserializeError>
     where
         I: Iterator<Item = Result<Token<'a>, DeserializeError>>,
     {
@@ -303,7 +324,11 @@
                             }
                             _ => skip_value(tokens)?,
                         },
-                        other => return Err(DeserializeError::custom(format!("expected object key or end object, found: {other:?}"))),
+                        other => {
+                            return Err(DeserializeError::custom(format!(
+                                "expected object key or end object, found: {other:?}"
+                            )))
+                        }
                     }
                 }
                 Ok(builder.build())
@@ -332,7 +357,11 @@
                                 map.insert(key.into(), value);
                             }
                         }
-                        other => return Err(DeserializeError::custom(format!("expected object key or end object, found: {other:?}"))),
+                        other => {
+                            return Err(DeserializeError::custom(format!(
+                                "expected object key or end object, found: {other:?}"
+                            )))
+                        }
                     }
                 }
                 Ok(map)
@@ -349,7 +378,9 @@
             .map(Cow::Owned))
     }

-    fn deser_outputs<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<PartitionOutputOverride>, DeserializeError>
+    fn deser_outputs<'a, I>(
+        tokens: &mut std::iter::Peekable<I>,
+    ) -> Result<Option<PartitionOutputOverride>, DeserializeError>
     where
         I: Iterator<Item = Result<Token<'a>, DeserializeError>>,
     {
@@ -381,7 +412,11 @@
                             }
                             _ => skip_value(tokens)?,
                         },
-                        other => return Err(DeserializeError::custom(format!("expected object key or end object, found: {other:?}",))),
+                        other => {
+                            return Err(DeserializeError::custom(format!(
+                                "expected object key or end object, found: {other:?}",
+                            )))
+                        }
                     }
                 }
                 Ok(Some(builder))
@@ -394,7 +429,9 @@
 #[cfg(test)]
 mod test {
     use crate::endpoint_lib::diagnostic::DiagnosticCollector;
-    use crate::endpoint_lib::partition::{Partition, PartitionMetadata, PartitionOutput, PartitionOutputOverride, PartitionResolver};
+    use crate::endpoint_lib::partition::{
+        Partition, PartitionMetadata, PartitionOutput, PartitionOutputOverride, PartitionResolver,
+    };
     use regex_lite::Regex;
     use std::collections::HashMap;

diff --git a/src/error.rs b/src/error.rs
--- a/src/error.rs
+++ b/src/error.rs
@@ -2,7 +2,8 @@
 pub use ::aws_smithy_runtime_api::box_error::BoxError;

 /// Error type returned by the client.
-pub type SdkError<E, R = ::aws_smithy_runtime_api::client::orchestrator::HttpResponse> = ::aws_smithy_runtime_api::client::result::SdkError<E, R>;
+pub type SdkError<E, R = ::aws_smithy_runtime_api::client::orchestrator::HttpResponse> =
+    ::aws_smithy_runtime_api::client::result::SdkError<E, R>;
 pub use ::aws_smithy_runtime_api::client::result::ConnectorError;
 pub use ::aws_smithy_types::error::operation::BuildError;

diff --git a/src/error/sealed_unhandled.rs b/src/error/sealed_unhandled.rs
--- a/src/error/sealed_unhandled.rs
+++ b/src/error/sealed_unhandled.rs
@@ -7,12 +7,14 @@
 /// on the error type.
 ///
 /// This struct intentionally doesn't yield any useful information itself.
-#[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
+#[deprecated(
+    note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
 variable wildcard pattern and check `.code()`:
  \
 &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
  \
-See [`ProvideErrorMetadata`](::aws_smithy_types::error::metadata::ProvideErrorMetadata) for what information is available for the error.")]
+See [`ProvideErrorMetadata`](::aws_smithy_types::error::metadata::ProvideErrorMetadata) for what information is available for the error."
+)]
 #[derive(Debug)]
 pub struct Unhandled {
     pub(crate) source: ::aws_smithy_runtime_api::box_error::BoxError,
diff --git a/src/error_meta.rs b/src/error_meta.rs
--- a/src/error_meta.rs
+++ b/src/error_meta.rs
@@ -3,58 +3,134 @@
 #[non_exhaustive]
 #[derive(::std::fmt::Debug)]
 pub enum Error {
+    #[cfg(any(
+        feature = "op_assume_role",
+        feature = "op_assume_role_with_saml",
+        feature = "op_assume_role_with_web_identity",
+        feature = "op_assume_root"
+    ))]
     /// <p>The web identity token that was passed is expired or is not valid. Get a new identity token from the identity provider and then retry the request.</p>
     ExpiredTokenException(crate::types::error::ExpiredTokenException),
+    #[cfg(feature = "op_get_delegated_access_token")]
     /// <p>The trade-in token provided in the request has expired and can no longer be exchanged for credentials. Request a new token and retry the operation.</p>
     ExpiredTradeInTokenException(crate::types::error::ExpiredTradeInTokenException),
+    #[cfg(feature = "op_assume_role_with_web_identity")]
     /// <p>The request could not be fulfilled because the identity provider (IDP) that was asked to verify the incoming identity token could not be reached. This is often a transient error caused by network conditions. Retry the request a limited number of times so that you don't exceed the request rate. If the error persists, the identity provider might be down or not responding.</p>
     IdpCommunicationErrorException(crate::types::error::IdpCommunicationErrorException),
+    #[cfg(any(feature = "op_assume_role_with_saml", feature = "op_assume_role_with_web_identity"))]
     /// <p>The identity provider (IdP) reported that authentication failed. This might be because the claim is invalid.</p>
     /// <p>If this error is returned for the <code>AssumeRoleWithWebIdentity</code> operation, it can also mean that the claim has expired or has been explicitly revoked.</p>
     IdpRejectedClaimException(crate::types::error::IdpRejectedClaimException),
+    #[cfg(feature = "op_decode_authorization_message")]
     /// <p>The error returned if the message passed to <code>DecodeAuthorizationMessage</code> was invalid. This can happen if the token contains invalid characters, such as line breaks, or if the message has expired.</p>
     InvalidAuthorizationMessageException(crate::types::error::InvalidAuthorizationMessageException),
+    #[cfg(any(feature = "op_assume_role_with_saml", feature = "op_assume_role_with_web_identity"))]
     /// <p>The web identity token that was passed could not be validated by Amazon Web Services. Get a new identity token from the identity provider and then retry the request.</p>
     InvalidIdentityTokenException(crate::types::error::InvalidIdentityTokenException),
+    #[cfg(feature = "op_get_web_identity_token")]
     /// <p>The requested token payload size exceeds the maximum allowed size. Reduce the number of request tags included in the <code>GetWebIdentityToken</code> API call to reduce the token payload size.</p>
     JwtPayloadSizeExceededException(crate::types::error::JwtPayloadSizeExceededException),
+    #[cfg(any(
+        feature = "op_assume_role",
+        feature = "op_assume_role_with_saml",
+        feature = "op_assume_role_with_web_identity",
+        feature = "op_get_federation_token"
+    ))]
     /// <p>The request was rejected because the policy document was malformed. The error message describes the specific error.</p>
     MalformedPolicyDocumentException(crate::types::error::MalformedPolicyDocumentException),
+    #[cfg(feature = "op_get_web_identity_token")]
     /// <p>The outbound web identity federation feature is not enabled for this account. To use this feature, you must first enable it through the Amazon Web Services Management Console or API.</p>
     OutboundWebIdentityFederationDisabledException(crate::types::error::OutboundWebIdentityFederationDisabledException),
+    #[cfg(any(
+        feature = "op_assume_role",
+        feature = "op_assume_role_with_saml",
+        feature = "op_assume_role_with_web_identity",
+        feature = "op_get_delegated_access_token",
+        feature = "op_get_federation_token"
+    ))]
     /// <p>The request was rejected because the total packed size of the session policies and session tags combined was too large. An Amazon Web Services conversion compresses the session policy document, session policy ARNs, and session tags into a packed binary format that has a separate limit. The error message indicates by percentage how close the policies and tags are to the upper size limit. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_session-tags.html">Passing Session Tags in STS</a> in the <i>IAM User Guide</i>.</p>
     /// <p>You could receive this error even though you meet other defined session policy and session tag limits. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_iam-quotas.html#reference_iam-limits-entity-length">IAM and STS Entity Character Limits</a> in the <i>IAM User Guide</i>.</p>
     PackedPolicyTooLargeException(crate::types::error::PackedPolicyTooLargeException),
+    #[cfg(any(
+        feature = "op_assume_role",
+        feature = "op_assume_role_with_saml",
+        feature = "op_assume_role_with_web_identity",
+        feature = "op_assume_root",
+        feature = "op_get_delegated_access_token",
+        feature = "op_get_federation_token",
+        feature = "op_get_session_token"
+    ))]
     /// <p>STS is not activated in the requested region for the account that is being asked to generate credentials. The account administrator must use the IAM console to activate STS in that region. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_enable-regions.html#sts-regions-activate-deactivate">Activating and Deactivating STS in an Amazon Web Services Region</a> in the <i>IAM User Guide</i>.</p>
     RegionDisabledException(crate::types::error::RegionDisabledException),
+    #[cfg(feature = "op_get_web_identity_token")]
     /// <p>The requested token duration would extend the session beyond its original expiration time. You cannot use this operation to extend the lifetime of a session beyond what was granted when the session was originally created.</p>
     SessionDurationEscalationException(crate::types::error::SessionDurationEscalationException),
     /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
-    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
+    #[deprecated(
+        note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
     variable wildcard pattern and check `.code()`:
      \
     &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
      \
-    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-Error) for what information is available for the error.")]
+    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-Error) for what information is available for the error."
+    )]
     Unhandled(crate::error::sealed_unhandled::Unhandled),
 }
 impl ::std::fmt::Display for Error {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
         match self {
+            #[cfg(any(
+                feature = "op_assume_role",
+                feature = "op_assume_role_with_saml",
+                feature = "op_assume_role_with_web_identity",
+                feature = "op_assume_root"
+            ))]
             Error::ExpiredTokenException(inner) => inner.fmt(f),
+            #[cfg(feature = "op_get_delegated_access_token")]
             Error::ExpiredTradeInTokenException(inner) => inner.fmt(f),
+            #[cfg(feature = "op_assume_role_with_web_identity")]
             Error::IdpCommunicationErrorException(inner) => inner.fmt(f),
+            #[cfg(any(feature = "op_assume_role_with_saml", feature = "op_assume_role_with_web_identity"))]
             Error::IdpRejectedClaimException(inner) => inner.fmt(f),
+            #[cfg(feature = "op_decode_authorization_message")]
             Error::InvalidAuthorizationMessageException(inner) => inner.fmt(f),
+            #[cfg(any(feature = "op_assume_role_with_saml", feature = "op_assume_role_with_web_identity"))]
             Error::InvalidIdentityTokenException(inner) => inner.fmt(f),
+            #[cfg(feature = "op_get_web_identity_token")]
             Error::JwtPayloadSizeExceededException(inner) => inner.fmt(f),
+            #[cfg(any(
+                feature = "op_assume_role",
+                feature = "op_assume_role_with_saml",
+                feature = "op_assume_role_with_web_identity",
+                feature = "op_get_federation_token"
+            ))]
             Error::MalformedPolicyDocumentException(inner) => inner.fmt(f),
+            #[cfg(feature = "op_get_web_identity_token")]
             Error::OutboundWebIdentityFederationDisabledException(inner) => inner.fmt(f),
+            #[cfg(any(
+                feature = "op_assume_role",
+                feature = "op_assume_role_with_saml",
+                feature = "op_assume_role_with_web_identity",
+                feature = "op_get_delegated_access_token",
+                feature = "op_get_federation_token"
+            ))]
             Error::PackedPolicyTooLargeException(inner) => inner.fmt(f),
+            #[cfg(any(
+                feature = "op_assume_role",
+                feature = "op_assume_role_with_saml",
+                feature = "op_assume_role_with_web_identity",
+                feature = "op_assume_root",
+                feature = "op_get_delegated_access_token",
+                feature = "op_get_federation_token",
+                feature = "op_get_session_token"
+            ))]
             Error::RegionDisabledException(inner) => inner.fmt(f),
+            #[cfg(feature = "op_get_web_identity_token")]
             Error::SessionDurationEscalationException(inner) => inner.fmt(f),
             Error::Unhandled(_) => {
-                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
+                if let ::std::option::Option::Some(code) =
+                    ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
+                {
                     write!(f, "unhandled error ({code})")
                 } else {
                     f.write_str("unhandled error")
@@ -74,27 +150,67 @@
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for Error {
     fn meta(&self) -> &::aws_smithy_types::error::metadata::ErrorMetadata {
         match self {
+            #[cfg(any(
+                feature = "op_assume_role",
+                feature = "op_assume_role_with_saml",
+                feature = "op_assume_role_with_web_identity",
+                feature = "op_assume_root"
+            ))]
             Self::ExpiredTokenException(inner) => inner.meta(),
+            #[cfg(feature = "op_get_delegated_access_token")]
             Self::ExpiredTradeInTokenException(inner) => inner.meta(),
+            #[cfg(feature = "op_assume_role_with_web_identity")]
             Self::IdpCommunicationErrorException(inner) => inner.meta(),
+            #[cfg(any(feature = "op_assume_role_with_saml", feature = "op_assume_role_with_web_identity"))]
             Self::IdpRejectedClaimException(inner) => inner.meta(),
+            #[cfg(feature = "op_decode_authorization_message")]
             Self::InvalidAuthorizationMessageException(inner) => inner.meta(),
+            #[cfg(any(feature = "op_assume_role_with_saml", feature = "op_assume_role_with_web_identity"))]
             Self::InvalidIdentityTokenException(inner) => inner.meta(),
+            #[cfg(feature = "op_get_web_identity_token")]
             Self::JwtPayloadSizeExceededException(inner) => inner.meta(),
+            #[cfg(any(
+                feature = "op_assume_role",
+                feature = "op_assume_role_with_saml",
+                feature = "op_assume_role_with_web_identity",
+                feature = "op_get_federation_token"
+            ))]
             Self::MalformedPolicyDocumentException(inner) => inner.meta(),
+            #[cfg(feature = "op_get_web_identity_token")]
             Self::OutboundWebIdentityFederationDisabledException(inner) => inner.meta(),
+            #[cfg(any(
+                feature = "op_assume_role",
+                feature = "op_assume_role_with_saml",
+                feature = "op_assume_role_with_web_identity",
+                feature = "op_get_delegated_access_token",
+                feature = "op_get_federation_token"
+            ))]
             Self::PackedPolicyTooLargeException(inner) => inner.meta(),
+            #[cfg(any(
+                feature = "op_assume_role",
+                feature = "op_assume_role_with_saml",
+                feature = "op_assume_role_with_web_identity",
+                feature = "op_assume_root",
+                feature = "op_get_delegated_access_token",
+                feature = "op_get_federation_token",
+                feature = "op_get_session_token"
+            ))]
             Self::RegionDisabledException(inner) => inner.meta(),
+            #[cfg(feature = "op_get_web_identity_token")]
             Self::SessionDurationEscalationException(inner) => inner.meta(),
             Self::Unhandled(inner) => &inner.meta,
         }
     }
 }
-impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::assume_role::AssumeRoleError, R>> for Error
+#[cfg(feature = "op_assume_role")]
+impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::assume_role::AssumeRoleError, R>>
+    for Error
 where
     R: Send + Sync + std::fmt::Debug + 'static,
 {
-    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::assume_role::AssumeRoleError, R>) -> Self {
+    fn from(
+        err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::assume_role::AssumeRoleError, R>,
+    ) -> Self {
         match err {
             ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
             _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
@@ -104,22 +220,43 @@
         }
     }
 }
+#[cfg(feature = "op_assume_role")]
 impl From<crate::operation::assume_role::AssumeRoleError> for Error {
     fn from(err: crate::operation::assume_role::AssumeRoleError) -> Self {
         match err {
-            crate::operation::assume_role::AssumeRoleError::ExpiredTokenException(inner) => Error::ExpiredTokenException(inner),
-            crate::operation::assume_role::AssumeRoleError::MalformedPolicyDocumentException(inner) => Error::MalformedPolicyDocumentException(inner),
-            crate::operation::assume_role::AssumeRoleError::PackedPolicyTooLargeException(inner) => Error::PackedPolicyTooLargeException(inner),
-            crate::operation::assume_role::AssumeRoleError::RegionDisabledException(inner) => Error::RegionDisabledException(inner),
+            crate::operation::assume_role::AssumeRoleError::ExpiredTokenException(inner) => {
+                Error::ExpiredTokenException(inner)
+            }
+            crate::operation::assume_role::AssumeRoleError::MalformedPolicyDocumentException(inner) => {
+                Error::MalformedPolicyDocumentException(inner)
+            }
+            crate::operation::assume_role::AssumeRoleError::PackedPolicyTooLargeException(inner) => {
+                Error::PackedPolicyTooLargeException(inner)
+            }
+            crate::operation::assume_role::AssumeRoleError::RegionDisabledException(inner) => {
+                Error::RegionDisabledException(inner)
+            }
             crate::operation::assume_role::AssumeRoleError::Unhandled(inner) => Error::Unhandled(inner),
         }
     }
 }
-impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError, R>> for Error
+#[cfg(feature = "op_assume_role_with_saml")]
+impl<R>
+    From<
+        ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError,
+            R,
+        >,
+    > for Error
 where
     R: Send + Sync + std::fmt::Debug + 'static,
 {
-    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError, R>) -> Self {
+    fn from(
+        err: ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError,
+            R,
+        >,
+    ) -> Self {
         match err {
             ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
             _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
@@ -129,10 +266,13 @@
         }
     }
 }
+#[cfg(feature = "op_assume_role_with_saml")]
 impl From<crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError> for Error {
     fn from(err: crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError) -> Self {
         match err {
-            crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::ExpiredTokenException(inner) => Error::ExpiredTokenException(inner),
+            crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::ExpiredTokenException(inner) => {
+                Error::ExpiredTokenException(inner)
+            }
             crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::IdpRejectedClaimException(inner) => {
                 Error::IdpRejectedClaimException(inner)
             }
@@ -139,24 +279,37 @@
             crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::InvalidIdentityTokenException(inner) => {
                 Error::InvalidIdentityTokenException(inner)
             }
-            crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::MalformedPolicyDocumentException(inner) => {
-                Error::MalformedPolicyDocumentException(inner)
-            }
+            crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::MalformedPolicyDocumentException(
+                inner,
+            ) => Error::MalformedPolicyDocumentException(inner),
             crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::PackedPolicyTooLargeException(inner) => {
                 Error::PackedPolicyTooLargeException(inner)
             }
-            crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::RegionDisabledException(inner) => Error::RegionDisabledException(inner),
-            crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::Unhandled(inner) => Error::Unhandled(inner),
+            crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::RegionDisabledException(inner) => {
+                Error::RegionDisabledException(inner)
+            }
+            crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::Unhandled(inner) => {
+                Error::Unhandled(inner)
+            }
         }
     }
 }
-impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError, R>>
-    for Error
+#[cfg(feature = "op_assume_role_with_web_identity")]
+impl<R>
+    From<
+        ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError,
+            R,
+        >,
+    > for Error
 where
     R: Send + Sync + std::fmt::Debug + 'static,
 {
     fn from(
-        err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError, R>,
+        err: ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError,
+            R,
+        >,
     ) -> Self {
         match err {
             ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
@@ -167,6 +320,7 @@
         }
     }
 }
+#[cfg(feature = "op_assume_role_with_web_identity")]
 impl From<crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError> for Error {
     fn from(err: crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError) -> Self {
         match err {
@@ -195,11 +349,15 @@
         }
     }
 }
-impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::assume_root::AssumeRootError, R>> for Error
+#[cfg(feature = "op_assume_root")]
+impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::assume_root::AssumeRootError, R>>
+    for Error
 where
     R: Send + Sync + std::fmt::Debug + 'static,
 {
-    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::assume_root::AssumeRootError, R>) -> Self {
+    fn from(
+        err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::assume_root::AssumeRootError, R>,
+    ) -> Self {
         match err {
             ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
             _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
@@ -209,22 +367,36 @@
         }
     }
 }
+#[cfg(feature = "op_assume_root")]
 impl From<crate::operation::assume_root::AssumeRootError> for Error {
     fn from(err: crate::operation::assume_root::AssumeRootError) -> Self {
         match err {
-            crate::operation::assume_root::AssumeRootError::ExpiredTokenException(inner) => Error::ExpiredTokenException(inner),
-            crate::operation::assume_root::AssumeRootError::RegionDisabledException(inner) => Error::RegionDisabledException(inner),
+            crate::operation::assume_root::AssumeRootError::ExpiredTokenException(inner) => {
+                Error::ExpiredTokenException(inner)
+            }
+            crate::operation::assume_root::AssumeRootError::RegionDisabledException(inner) => {
+                Error::RegionDisabledException(inner)
+            }
             crate::operation::assume_root::AssumeRootError::Unhandled(inner) => Error::Unhandled(inner),
         }
     }
 }
-impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::decode_authorization_message::DecodeAuthorizationMessageError, R>>
-    for Error
+#[cfg(feature = "op_decode_authorization_message")]
+impl<R>
+    From<
+        ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::decode_authorization_message::DecodeAuthorizationMessageError,
+            R,
+        >,
+    > for Error
 where
     R: Send + Sync + std::fmt::Debug + 'static,
 {
     fn from(
-        err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::decode_authorization_message::DecodeAuthorizationMessageError, R>,
+        err: ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::decode_authorization_message::DecodeAuthorizationMessageError,
+            R,
+        >,
     ) -> Self {
         match err {
             ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
@@ -235,6 +407,7 @@
         }
     }
 }
+#[cfg(feature = "op_decode_authorization_message")]
 impl From<crate::operation::decode_authorization_message::DecodeAuthorizationMessageError> for Error {
     fn from(err: crate::operation::decode_authorization_message::DecodeAuthorizationMessageError) -> Self {
         match err {
@@ -245,11 +418,23 @@
         }
     }
 }
-impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_access_key_info::GetAccessKeyInfoError, R>> for Error
+#[cfg(feature = "op_get_access_key_info")]
+impl<R>
+    From<
+        ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::get_access_key_info::GetAccessKeyInfoError,
+            R,
+        >,
+    > for Error
 where
     R: Send + Sync + std::fmt::Debug + 'static,
 {
-    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_access_key_info::GetAccessKeyInfoError, R>) -> Self {
+    fn from(
+        err: ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::get_access_key_info::GetAccessKeyInfoError,
+            R,
+        >,
+    ) -> Self {
         match err {
             ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
             _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
@@ -259,6 +444,7 @@
         }
     }
 }
+#[cfg(feature = "op_get_access_key_info")]
 impl From<crate::operation::get_access_key_info::GetAccessKeyInfoError> for Error {
     fn from(err: crate::operation::get_access_key_info::GetAccessKeyInfoError) -> Self {
         match err {
@@ -266,11 +452,23 @@
         }
     }
 }
-impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_caller_identity::GetCallerIdentityError, R>> for Error
+#[cfg(feature = "op_get_caller_identity")]
+impl<R>
+    From<
+        ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::get_caller_identity::GetCallerIdentityError,
+            R,
+        >,
+    > for Error
 where
     R: Send + Sync + std::fmt::Debug + 'static,
 {
-    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_caller_identity::GetCallerIdentityError, R>) -> Self {
+    fn from(
+        err: ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::get_caller_identity::GetCallerIdentityError,
+            R,
+        >,
+    ) -> Self {
         match err {
             ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
             _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
@@ -280,6 +478,7 @@
         }
     }
 }
+#[cfg(feature = "op_get_caller_identity")]
 impl From<crate::operation::get_caller_identity::GetCallerIdentityError> for Error {
     fn from(err: crate::operation::get_caller_identity::GetCallerIdentityError) -> Self {
         match err {
@@ -287,13 +486,22 @@
         }
     }
 }
-impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError, R>>
-    for Error
+#[cfg(feature = "op_get_delegated_access_token")]
+impl<R>
+    From<
+        ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError,
+            R,
+        >,
+    > for Error
 where
     R: Send + Sync + std::fmt::Debug + 'static,
 {
     fn from(
-        err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError, R>,
+        err: ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError,
+            R,
+        >,
     ) -> Self {
         match err {
             ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
@@ -304,6 +512,7 @@
         }
     }
 }
+#[cfg(feature = "op_get_delegated_access_token")]
 impl From<crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError> for Error {
     fn from(err: crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError) -> Self {
         match err {
@@ -320,11 +529,23 @@
         }
     }
 }
-impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_federation_token::GetFederationTokenError, R>> for Error
+#[cfg(feature = "op_get_federation_token")]
+impl<R>
+    From<
+        ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::get_federation_token::GetFederationTokenError,
+            R,
+        >,
+    > for Error
 where
     R: Send + Sync + std::fmt::Debug + 'static,
 {
-    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_federation_token::GetFederationTokenError, R>) -> Self {
+    fn from(
+        err: ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::get_federation_token::GetFederationTokenError,
+            R,
+        >,
+    ) -> Self {
         match err {
             ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
             _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
@@ -334,25 +555,42 @@
         }
     }
 }
+#[cfg(feature = "op_get_federation_token")]
 impl From<crate::operation::get_federation_token::GetFederationTokenError> for Error {
     fn from(err: crate::operation::get_federation_token::GetFederationTokenError) -> Self {
         match err {
-            crate::operation::get_federation_token::GetFederationTokenError::MalformedPolicyDocumentException(inner) => {
-                Error::MalformedPolicyDocumentException(inner)
-            }
+            crate::operation::get_federation_token::GetFederationTokenError::MalformedPolicyDocumentException(
+                inner,
+            ) => Error::MalformedPolicyDocumentException(inner),
             crate::operation::get_federation_token::GetFederationTokenError::PackedPolicyTooLargeException(inner) => {
                 Error::PackedPolicyTooLargeException(inner)
             }
-            crate::operation::get_federation_token::GetFederationTokenError::RegionDisabledException(inner) => Error::RegionDisabledException(inner),
-            crate::operation::get_federation_token::GetFederationTokenError::Unhandled(inner) => Error::Unhandled(inner),
+            crate::operation::get_federation_token::GetFederationTokenError::RegionDisabledException(inner) => {
+                Error::RegionDisabledException(inner)
+            }
+            crate::operation::get_federation_token::GetFederationTokenError::Unhandled(inner) => {
+                Error::Unhandled(inner)
+            }
         }
     }
 }
-impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_session_token::GetSessionTokenError, R>> for Error
+#[cfg(feature = "op_get_session_token")]
+impl<R>
+    From<
+        ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::get_session_token::GetSessionTokenError,
+            R,
+        >,
+    > for Error
 where
     R: Send + Sync + std::fmt::Debug + 'static,
 {
-    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_session_token::GetSessionTokenError, R>) -> Self {
+    fn from(
+        err: ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::get_session_token::GetSessionTokenError,
+            R,
+        >,
+    ) -> Self {
         match err {
             ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
             _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
@@ -362,19 +600,34 @@
         }
     }
 }
+#[cfg(feature = "op_get_session_token")]
 impl From<crate::operation::get_session_token::GetSessionTokenError> for Error {
     fn from(err: crate::operation::get_session_token::GetSessionTokenError) -> Self {
         match err {
-            crate::operation::get_session_token::GetSessionTokenError::RegionDisabledException(inner) => Error::RegionDisabledException(inner),
+            crate::operation::get_session_token::GetSessionTokenError::RegionDisabledException(inner) => {
+                Error::RegionDisabledException(inner)
+            }
             crate::operation::get_session_token::GetSessionTokenError::Unhandled(inner) => Error::Unhandled(inner),
         }
     }
 }
-impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_web_identity_token::GetWebIdentityTokenError, R>> for Error
+#[cfg(feature = "op_get_web_identity_token")]
+impl<R>
+    From<
+        ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::get_web_identity_token::GetWebIdentityTokenError,
+            R,
+        >,
+    > for Error
 where
     R: Send + Sync + std::fmt::Debug + 'static,
 {
-    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_web_identity_token::GetWebIdentityTokenError, R>) -> Self {
+    fn from(
+        err: ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::get_web_identity_token::GetWebIdentityTokenError,
+            R,
+        >,
+    ) -> Self {
         match err {
             ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
             _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
@@ -384,6 +637,7 @@
         }
     }
 }
+#[cfg(feature = "op_get_web_identity_token")]
 impl From<crate::operation::get_web_identity_token::GetWebIdentityTokenError> for Error {
     fn from(err: crate::operation::get_web_identity_token::GetWebIdentityTokenError) -> Self {
         match err {
@@ -403,17 +657,53 @@
 impl ::std::error::Error for Error {
     fn source(&self) -> std::option::Option<&(dyn ::std::error::Error + 'static)> {
         match self {
+            #[cfg(any(
+                feature = "op_assume_role",
+                feature = "op_assume_role_with_saml",
+                feature = "op_assume_role_with_web_identity",
+                feature = "op_assume_root"
+            ))]
             Error::ExpiredTokenException(inner) => inner.source(),
+            #[cfg(feature = "op_get_delegated_access_token")]
             Error::ExpiredTradeInTokenException(inner) => inner.source(),
+            #[cfg(feature = "op_assume_role_with_web_identity")]
             Error::IdpCommunicationErrorException(inner) => inner.source(),
+            #[cfg(any(feature = "op_assume_role_with_saml", feature = "op_assume_role_with_web_identity"))]
             Error::IdpRejectedClaimException(inner) => inner.source(),
+            #[cfg(feature = "op_decode_authorization_message")]
             Error::InvalidAuthorizationMessageException(inner) => inner.source(),
+            #[cfg(any(feature = "op_assume_role_with_saml", feature = "op_assume_role_with_web_identity"))]
             Error::InvalidIdentityTokenException(inner) => inner.source(),
+            #[cfg(feature = "op_get_web_identity_token")]
             Error::JwtPayloadSizeExceededException(inner) => inner.source(),
+            #[cfg(any(
+                feature = "op_assume_role",
+                feature = "op_assume_role_with_saml",
+                feature = "op_assume_role_with_web_identity",
+                feature = "op_get_federation_token"
+            ))]
             Error::MalformedPolicyDocumentException(inner) => inner.source(),
+            #[cfg(feature = "op_get_web_identity_token")]
             Error::OutboundWebIdentityFederationDisabledException(inner) => inner.source(),
+            #[cfg(any(
+                feature = "op_assume_role",
+                feature = "op_assume_role_with_saml",
+                feature = "op_assume_role_with_web_identity",
+                feature = "op_get_delegated_access_token",
+                feature = "op_get_federation_token"
+            ))]
             Error::PackedPolicyTooLargeException(inner) => inner.source(),
+            #[cfg(any(
+                feature = "op_assume_role",
+                feature = "op_assume_role_with_saml",
+                feature = "op_assume_role_with_web_identity",
+                feature = "op_assume_root",
+                feature = "op_get_delegated_access_token",
+                feature = "op_get_federation_token",
+                feature = "op_get_session_token"
+            ))]
             Error::RegionDisabledException(inner) => inner.source(),
+            #[cfg(feature = "op_get_web_identity_token")]
             Error::SessionDurationEscalationException(inner) => inner.source(),
             Error::Unhandled(inner) => ::std::option::Option::Some(&*inner.source),
         }
@@ -422,17 +712,53 @@
 impl ::aws_types::request_id::RequestId for Error {
     fn request_id(&self) -> Option<&str> {
         match self {
+            #[cfg(any(
+                feature = "op_assume_role",
+                feature = "op_assume_role_with_saml",
+                feature = "op_assume_role_with_web_identity",
+                feature = "op_assume_root"
+            ))]
             Self::ExpiredTokenException(e) => e.request_id(),
+            #[cfg(feature = "op_get_delegated_access_token")]
             Self::ExpiredTradeInTokenException(e) => e.request_id(),
+            #[cfg(feature = "op_assume_role_with_web_identity")]
             Self::IdpCommunicationErrorException(e) => e.request_id(),
+            #[cfg(any(feature = "op_assume_role_with_saml", feature = "op_assume_role_with_web_identity"))]
             Self::IdpRejectedClaimException(e) => e.request_id(),
+            #[cfg(feature = "op_decode_authorization_message")]
             Self::InvalidAuthorizationMessageException(e) => e.request_id(),
+            #[cfg(any(feature = "op_assume_role_with_saml", feature = "op_assume_role_with_web_identity"))]
             Self::InvalidIdentityTokenException(e) => e.request_id(),
+            #[cfg(feature = "op_get_web_identity_token")]
             Self::JwtPayloadSizeExceededException(e) => e.request_id(),
+            #[cfg(any(
+                feature = "op_assume_role",
+                feature = "op_assume_role_with_saml",
+                feature = "op_assume_role_with_web_identity",
+                feature = "op_get_federation_token"
+            ))]
             Self::MalformedPolicyDocumentException(e) => e.request_id(),
+            #[cfg(feature = "op_get_web_identity_token")]
             Self::OutboundWebIdentityFederationDisabledException(e) => e.request_id(),
+            #[cfg(any(
+                feature = "op_assume_role",
+                feature = "op_assume_role_with_saml",
+                feature = "op_assume_role_with_web_identity",
+                feature = "op_get_delegated_access_token",
+                feature = "op_get_federation_token"
+            ))]
             Self::PackedPolicyTooLargeException(e) => e.request_id(),
+            #[cfg(any(
+                feature = "op_assume_role",
+                feature = "op_assume_role_with_saml",
+                feature = "op_assume_role_with_web_identity",
+                feature = "op_assume_root",
+                feature = "op_get_delegated_access_token",
+                feature = "op_get_federation_token",
+                feature = "op_get_session_token"
+            ))]
             Self::RegionDisabledException(e) => e.request_id(),
+            #[cfg(feature = "op_get_web_identity_token")]
             Self::SessionDurationEscalationException(e) => e.request_id(),
             Self::Unhandled(e) => e.meta.request_id(),
         }
diff --git a/src/meta.rs b/src/meta.rs
--- a/src/meta.rs
+++ b/src/meta.rs
@@ -1,5 +1,6 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-pub(crate) static API_METADATA: ::aws_runtime::user_agent::ApiMetadata = ::aws_runtime::user_agent::ApiMetadata::new("sts", crate::meta::PKG_VERSION);
+pub(crate) static API_METADATA: ::aws_runtime::user_agent::ApiMetadata =
+    ::aws_runtime::user_agent::ApiMetadata::new("sts", crate::meta::PKG_VERSION);

 /// Crate version number.
 pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
diff --git a/src/observability_feature.rs b/src/observability_feature.rs
--- a/src/observability_feature.rs
+++ b/src/observability_feature.rs
@@ -21,7 +21,11 @@
         "ObservabilityFeatureTrackerInterceptor"
     }

-    fn read_before_execution(&self, _context: &BeforeSerializationInterceptorContextRef<'_>, cfg: &mut ConfigBag) -> Result<(), BoxError> {
+    fn read_before_execution(
+        &self,
+        _context: &BeforeSerializationInterceptorContextRef<'_>,
+        cfg: &mut ConfigBag,
+    ) -> Result<(), BoxError> {
         // Check if an OpenTelemetry meter provider is configured via the global provider
         if let Ok(telemetry_provider) = aws_smithy_observability::global::get_telemetry_provider() {
             let meter_provider = telemetry_provider.meter_provider();
@@ -28,7 +32,8 @@

             // Use provider_name() to detect OpenTelemetry without importing the otel crate.
             if meter_provider.provider_name() == "AwsSmithyObservabilityOtelProvider" {
-                cfg.interceptor_state().store_append(SmithySdkFeature::ObservabilityOtelMetrics);
+                cfg.interceptor_state()
+                    .store_append(SmithySdkFeature::ObservabilityOtelMetrics);
             }
         }

diff --git a/src/operation.rs b/src/operation.rs
--- a/src/operation.rs
+++ b/src/operation.rs
@@ -1,35 +1,46 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub use ::aws_types::request_id::RequestId;

+#[cfg(feature = "op_assume_role")]
 /// Types for the `AssumeRole` operation.
 pub mod assume_role;

+#[cfg(feature = "op_assume_role_with_saml")]
 /// Types for the `AssumeRoleWithSAML` operation.
 pub mod assume_role_with_saml;

+#[cfg(feature = "op_assume_role_with_web_identity")]
 /// Types for the `AssumeRoleWithWebIdentity` operation.
 pub mod assume_role_with_web_identity;

+#[cfg(feature = "op_assume_root")]
 /// Types for the `AssumeRoot` operation.
 pub mod assume_root;

+#[cfg(feature = "op_decode_authorization_message")]
 /// Types for the `DecodeAuthorizationMessage` operation.
 pub mod decode_authorization_message;

+#[cfg(feature = "op_get_access_key_info")]
 /// Types for the `GetAccessKeyInfo` operation.
 pub mod get_access_key_info;

+#[cfg(feature = "op_get_caller_identity")]
 /// Types for the `GetCallerIdentity` operation.
 pub mod get_caller_identity;

+#[cfg(feature = "op_get_delegated_access_token")]
 /// Types for the `GetDelegatedAccessToken` operation.
 pub mod get_delegated_access_token;

+#[cfg(feature = "op_get_federation_token")]
 /// Types for the `GetFederationToken` operation.
 pub mod get_federation_token;

+#[cfg(feature = "op_get_session_token")]
 /// Types for the `GetSessionToken` operation.
 pub mod get_session_token;

+#[cfg(feature = "op_get_web_identity_token")]
 /// Types for the `GetWebIdentityToken` operation.
 pub mod get_web_identity_token;
diff --git a/src/operation/assume_role.rs b/src/operation/assume_role.rs
--- a/src/operation/assume_role.rs
+++ b/src/operation/assume_role.rs
@@ -27,9 +27,13 @@
                     .expect("correct error type")
             })
         };
-        let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
-            .await
-            .map_err(map_err)?;
+        let context = Self::orchestrate_with_stop_point(
+            runtime_plugins,
+            input,
+            ::aws_smithy_runtime::client::orchestrator::StopPoint::None,
+        )
+        .await
+        .map_err(map_err)?;
         let output = context.finalize().map_err(map_err)?;
         ::std::result::Result::Ok(
             output
@@ -51,17 +55,23 @@
     > {
         let input = ::aws_smithy_runtime_api::client::interceptors::context::Input::erase(input);
         use ::tracing::Instrument;
-        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("STS", "AssumeRole", input, runtime_plugins, stop_point)
-            // Create a parent span for the entire operation. Includes a random, internal-only,
-            // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
-            .instrument(::tracing::debug_span!(
-                "STS.AssumeRole",
-                "rpc.service" = "STS",
-                "rpc.method" = "AssumeRole",
-                "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
-                "rpc.system" = "aws-api",
-            ))
-            .await
+        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point(
+            "STS",
+            "AssumeRole",
+            input,
+            runtime_plugins,
+            stop_point,
+        )
+        // Create a parent span for the entire operation. Includes a random, internal-only,
+        // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
+        .instrument(::tracing::debug_span!(
+            "STS.AssumeRole",
+            "rpc.service" = "STS",
+            "rpc.method" = "AssumeRole",
+            "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
+            "rpc.system" = "aws-api",
+        ))
+        .await
     }

     pub(crate) fn operation_runtime_plugins(
@@ -91,19 +101,24 @@
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
             AssumeRoleRequestSerializer,
         ));
-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            AssumeRoleResponseDeserializer,
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(AssumeRoleResponseDeserializer),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
-            crate::config::auth::Params::builder()
-                .operation_name("AssumeRole")
-                .build()
-                .expect("required fields set"),
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
+                crate::config::auth::Params::builder()
+                    .operation_name("AssumeRole")
+                    .build()
+                    .expect("required fields set"),
+            ),
+        );

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::SensitiveOutput);
-        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("AssumeRole", "STS"));
+        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
+            "AssumeRole",
+            "STS",
+        ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
         signing_options.double_uri_encode = true;
         signing_options.content_sha256_header = false;
@@ -251,13 +266,23 @@
 #[derive(Debug)]
 struct AssumeRoleRequestSerializer;
 impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for AssumeRoleRequestSerializer {
-    #[allow(unused_mut, clippy::let_and_return, clippy::needless_borrow, clippy::useless_conversion)]
+    #[allow(
+        unused_mut,
+        clippy::let_and_return,
+        clippy::needless_borrow,
+        clippy::useless_conversion
+    )]
     fn serialize_input(
         &self,
         input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
         _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
-    ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
-        let input = input.downcast::<crate::operation::assume_role::AssumeRoleInput>().expect("correct type");
+    ) -> ::std::result::Result<
+        ::aws_smithy_runtime_api::client::orchestrator::HttpRequest,
+        ::aws_smithy_runtime_api::box_error::BoxError,
+    > {
+        let input = input
+            .downcast::<crate::operation::assume_role::AssumeRoleInput>()
+            .expect("correct type");
         let _header_serialization_settings = _cfg
             .load::<crate::serialization_settings::HeaderSerializationSettings>()
             .cloned()
@@ -276,21 +301,30 @@
             fn update_http_builder(
                 input: &crate::operation::assume_role::AssumeRoleInput,
                 builder: ::http_1x::request::Builder,
-            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError>
+            {
                 let mut uri = ::std::string::String::new();
                 uri_base(input, &mut uri)?;
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(
+                builder,
+                ::http_1x::header::CONTENT_TYPE,
+                "application/x-www-form-urlencoded",
+            );
             builder
         };
-        let body =
-            ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_assume_role_input::ser_assume_role_input_input_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            crate::protocol_serde::shape_assume_role_input::ser_assume_role_input_input_input(&input)?,
+        );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
-            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+            request_builder = _header_serialization_settings.set_default_header(
+                request_builder,
+                ::http_1x::header::CONTENT_LENGTH,
+                &content_length,
+            );
         }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
@@ -323,13 +357,21 @@
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
-            .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
+            .set_endpoint(
+                cfg.load::<::aws_types::endpoint_config::EndpointUrl>()
+                    .map(|ty| ty.0.clone()),
+            )
             .build()
             .map_err(|err| {
-                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new("endpoint params could not be built", err)
+                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new(
+                    "endpoint params could not be built",
+                    err,
+                )
             })?;
         cfg.interceptor_state()
-            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params));
+            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(
+                params,
+            ));
         ::std::result::Result::Ok(())
     }
 }
@@ -351,18 +393,22 @@
     /// <p>STS is not activated in the requested region for the account that is being asked to generate credentials. The account administrator must use the IAM console to activate STS in that region. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_enable-regions.html#sts-regions-activate-deactivate">Activating and Deactivating STS in an Amazon Web Services Region</a> in the <i>IAM User Guide</i>.</p>
     RegionDisabledException(crate::types::error::RegionDisabledException),
     /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
-    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
+    #[deprecated(
+        note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
     variable wildcard pattern and check `.code()`:
      \
     &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
      \
-    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-AssumeRoleError) for what information is available for the error.")]
+    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-AssumeRoleError) for what information is available for the error."
+    )]
     Unhandled(crate::error::sealed_unhandled::Unhandled),
 }
 impl AssumeRoleError {
     /// Creates the `AssumeRoleError::Unhandled` variant from any error type.
     pub fn unhandled(
-        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
+        err: impl ::std::convert::Into<
+            ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
+        >,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
             source: err.into(),
@@ -384,8 +430,12 @@
     pub fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
             Self::ExpiredTokenException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::MalformedPolicyDocumentException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::PackedPolicyTooLargeException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::MalformedPolicyDocumentException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
+            Self::PackedPolicyTooLargeException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
             Self::RegionDisabledException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
             Self::Unhandled(e) => &e.meta,
         }
@@ -426,7 +476,9 @@
             Self::PackedPolicyTooLargeException(_inner) => _inner.fmt(f),
             Self::RegionDisabledException(_inner) => _inner.fmt(f),
             Self::Unhandled(_inner) => {
-                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
+                if let ::std::option::Option::Some(code) =
+                    ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
+                {
                     write!(f, "unhandled error ({code})")
                 } else {
                     f.write_str("unhandled error")
@@ -446,10 +498,18 @@
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for AssumeRoleError {
     fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::ExpiredTokenException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::MalformedPolicyDocumentException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::PackedPolicyTooLargeException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::RegionDisabledException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::ExpiredTokenException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::MalformedPolicyDocumentException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::PackedPolicyTooLargeException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::RegionDisabledException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
             Self::Unhandled(_inner) => &_inner.meta,
         }
     }
diff --git a/src/operation/assume_role/_assume_role_input.rs b/src/operation/assume_role/_assume_role_input.rs
--- a/src/operation/assume_role/_assume_role_input.rs
+++ b/src/operation/assume_role/_assume_role_input.rs
@@ -232,7 +232,10 @@
     /// <p>An Amazon Web Services conversion compresses the passed inline session policy, managed policy ARNs, and session tags into a packed binary format that has a separate limit. Your request can fail for this limit even if your plaintext meets the other requirements. The <code>PackedPolicySize</code> response element indicates by percentage how close the policies and tags for your request are to the upper size limit.</p>
     /// </note>
     /// <p>Passing policies to this operation returns new temporary credentials. The resulting session's permissions are the intersection of the role's identity-based policy and the session policies. You can use the role's temporary credentials in subsequent Amazon Web Services API calls to access resources in the account that owns the role. You cannot use session policies to grant more permissions than those allowed by the identity-based policy of the role that is being assumed. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#policies_session">Session Policies</a> in the <i>IAM User Guide</i>.</p>
-    pub fn set_policy_arns(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::PolicyDescriptorType>>) -> Self {
+    pub fn set_policy_arns(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<crate::types::PolicyDescriptorType>>,
+    ) -> Self {
         self.policy_arns = input;
         self
     }
@@ -353,7 +356,10 @@
     /// <p>A list of keys for session tags that you want to set as transitive. If you set a tag key as transitive, the corresponding key and value passes to subsequent sessions in a role chain. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_session-tags.html#id_session-tags_role-chaining">Chaining Roles with Session Tags</a> in the <i>IAM User Guide</i>.</p>
     /// <p>This parameter is optional. The transitive status of a session tag does not impact its packed binary size.</p>
     /// <p>If you choose not to specify a transitive tag key, then no tags are passed from this session to any subsequent sessions.</p>
-    pub fn set_transitive_tag_keys(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
+    pub fn set_transitive_tag_keys(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
+    ) -> Self {
         self.transitive_tag_keys = input;
         self
     }
@@ -450,7 +456,10 @@
     /// <p>A list of previously acquired trusted context assertions in the format of a JSON array. The trusted context assertion is signed and encrypted by Amazon Web Services STS.</p>
     /// <p>The following is an example of a <code>ProvidedContext</code> value that includes a single trusted context assertion and the ARN of the context provider from which the trusted context assertion was generated.</p>
     /// <p><code>\[{"ProviderArn":"arn:aws:iam::aws:contextProvider/IdentityCenter","ContextAssertion":"trusted-context-assertion"}\]</code></p>
-    pub fn set_provided_contexts(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ProvidedContext>>) -> Self {
+    pub fn set_provided_contexts(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<crate::types::ProvidedContext>>,
+    ) -> Self {
         self.provided_contexts = input;
         self
     }
@@ -461,7 +470,12 @@
         &self.provided_contexts
     }
     /// Consumes the builder and constructs a [`AssumeRoleInput`](crate::operation::assume_role::AssumeRoleInput).
-    pub fn build(self) -> ::std::result::Result<crate::operation::assume_role::AssumeRoleInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<
+        crate::operation::assume_role::AssumeRoleInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(crate::operation::assume_role::AssumeRoleInput {
             role_arn: self.role_arn,
             role_session_name: self.role_session_name,
diff --git a/src/operation/assume_role/builders.rs b/src/operation/assume_role/builders.rs
--- a/src/operation/assume_role/builders.rs
+++ b/src/operation/assume_role/builders.rs
@@ -122,12 +122,18 @@
     > {
         crate::client::customize::CustomizableOperation::new(self)
     }
-    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
+    pub(crate) fn config_override(
+        mut self,
+        config_override: impl ::std::convert::Into<crate::config::Builder>,
+    ) -> Self {
         self.set_config_override(::std::option::Option::Some(config_override.into()));
         self
     }

-    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
+    pub(crate) fn set_config_override(
+        &mut self,
+        config_override: ::std::option::Option<crate::config::Builder>,
+    ) -> &mut Self {
         self.config_override = config_override;
         self
     }
@@ -187,7 +193,10 @@
     /// <p>An Amazon Web Services conversion compresses the passed inline session policy, managed policy ARNs, and session tags into a packed binary format that has a separate limit. Your request can fail for this limit even if your plaintext meets the other requirements. The <code>PackedPolicySize</code> response element indicates by percentage how close the policies and tags for your request are to the upper size limit.</p>
     /// </note>
     /// <p>Passing policies to this operation returns new temporary credentials. The resulting session's permissions are the intersection of the role's identity-based policy and the session policies. You can use the role's temporary credentials in subsequent Amazon Web Services API calls to access resources in the account that owns the role. You cannot use session policies to grant more permissions than those allowed by the identity-based policy of the role that is being assumed. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#policies_session">Session Policies</a> in the <i>IAM User Guide</i>.</p>
-    pub fn set_policy_arns(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::PolicyDescriptorType>>) -> Self {
+    pub fn set_policy_arns(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<crate::types::PolicyDescriptorType>>,
+    ) -> Self {
         self.inner = self.inner.set_policy_arns(input);
         self
     }
@@ -306,7 +315,10 @@
     /// <p>A list of keys for session tags that you want to set as transitive. If you set a tag key as transitive, the corresponding key and value passes to subsequent sessions in a role chain. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_session-tags.html#id_session-tags_role-chaining">Chaining Roles with Session Tags</a> in the <i>IAM User Guide</i>.</p>
     /// <p>This parameter is optional. The transitive status of a session tag does not impact its packed binary size.</p>
     /// <p>If you choose not to specify a transitive tag key, then no tags are passed from this session to any subsequent sessions.</p>
-    pub fn set_transitive_tag_keys(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
+    pub fn set_transitive_tag_keys(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
+    ) -> Self {
         self.inner = self.inner.set_transitive_tag_keys(input);
         self
     }
@@ -402,7 +414,10 @@
     /// <p>A list of previously acquired trusted context assertions in the format of a JSON array. The trusted context assertion is signed and encrypted by Amazon Web Services STS.</p>
     /// <p>The following is an example of a <code>ProvidedContext</code> value that includes a single trusted context assertion and the ARN of the context provider from which the trusted context assertion was generated.</p>
     /// <p><code>\[{"ProviderArn":"arn:aws:iam::aws:contextProvider/IdentityCenter","ContextAssertion":"trusted-context-assertion"}\]</code></p>
-    pub fn set_provided_contexts(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ProvidedContext>>) -> Self {
+    pub fn set_provided_contexts(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<crate::types::ProvidedContext>>,
+    ) -> Self {
         self.inner = self.inner.set_provided_contexts(input);
         self
     }
diff --git a/src/operation/assume_role_with_saml.rs b/src/operation/assume_role_with_saml.rs
--- a/src/operation/assume_role_with_saml.rs
+++ b/src/operation/assume_role_with_saml.rs
@@ -27,9 +27,13 @@
                     .expect("correct error type")
             })
         };
-        let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
-            .await
-            .map_err(map_err)?;
+        let context = Self::orchestrate_with_stop_point(
+            runtime_plugins,
+            input,
+            ::aws_smithy_runtime::client::orchestrator::StopPoint::None,
+        )
+        .await
+        .map_err(map_err)?;
         let output = context.finalize().map_err(map_err)?;
         ::std::result::Result::Ok(
             output
@@ -51,17 +55,23 @@
     > {
         let input = ::aws_smithy_runtime_api::client::interceptors::context::Input::erase(input);
         use ::tracing::Instrument;
-        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("STS", "AssumeRoleWithSAML", input, runtime_plugins, stop_point)
-            // Create a parent span for the entire operation. Includes a random, internal-only,
-            // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
-            .instrument(::tracing::debug_span!(
-                "STS.AssumeRoleWithSAML",
-                "rpc.service" = "STS",
-                "rpc.method" = "AssumeRoleWithSAML",
-                "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
-                "rpc.system" = "aws-api",
-            ))
-            .await
+        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point(
+            "STS",
+            "AssumeRoleWithSAML",
+            input,
+            runtime_plugins,
+            stop_point,
+        )
+        // Create a parent span for the entire operation. Includes a random, internal-only,
+        // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
+        .instrument(::tracing::debug_span!(
+            "STS.AssumeRoleWithSAML",
+            "rpc.service" = "STS",
+            "rpc.method" = "AssumeRoleWithSAML",
+            "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
+            "rpc.system" = "aws-api",
+        ))
+        .await
     }

     pub(crate) fn operation_runtime_plugins(
@@ -91,19 +101,26 @@
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
             AssumeRoleWithSAMLRequestSerializer,
         ));
-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            AssumeRoleWithSAMLResponseDeserializer,
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
+                AssumeRoleWithSAMLResponseDeserializer,
+            ),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
-            crate::config::auth::Params::builder()
-                .operation_name("AssumeRoleWithSAML")
-                .build()
-                .expect("required fields set"),
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
+                crate::config::auth::Params::builder()
+                    .operation_name("AssumeRoleWithSAML")
+                    .build()
+                    .expect("required fields set"),
+            ),
+        );

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::SensitiveOutput);
-        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("AssumeRoleWithSAML", "STS"));
+        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
+            "AssumeRoleWithSAML",
+            "STS",
+        ));

         ::std::option::Option::Some(cfg.freeze())
     }
@@ -113,17 +130,46 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("AssumeRoleWithSAML")
-                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(AssumeRoleWithSAMLTelemetryInputCaptureInterceptor))
-.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
-.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(AssumeRoleWithSAMLEndpointParamsInterceptor))
-                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError>::new())
-.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError>::new())
-.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError>::builder().transient_errors({
-                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
-                                            transient_errors.push("IDPCommunicationError");
-                                            ::std::borrow::Cow::Owned(transient_errors)
-                                            }).build());
+        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new(
+            "AssumeRoleWithSAML",
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                AssumeRoleWithSAMLTelemetryInputCaptureInterceptor,
+            ),
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
+            ),
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                AssumeRoleWithSAMLEndpointParamsInterceptor,
+            ),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+                crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError,
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+                crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError,
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError,
+            >::builder()
+            .transient_errors({
+                let mut transient_errors: Vec<&'static str> =
+                    ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                transient_errors.push("IDPCommunicationError");
+                ::std::borrow::Cow::Owned(transient_errors)
+            })
+            .build(),
+        );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -197,9 +243,13 @@
         let mut force_error = false;
         ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));
         let parse_result = if !success && status != 200 || force_error {
-            crate::protocol_serde::shape_assume_role_with_saml::de_assume_role_with_saml_http_error(status, headers, body)
+            crate::protocol_serde::shape_assume_role_with_saml::de_assume_role_with_saml_http_error(
+                status, headers, body,
+            )
         } else {
-            crate::protocol_serde::shape_assume_role_with_saml::de_assume_role_with_saml_http_response(status, headers, body)
+            crate::protocol_serde::shape_assume_role_with_saml::de_assume_role_with_saml_http_response(
+                status, headers, body,
+            )
         };
         crate::protocol_serde::type_erase_result(parse_result)
     }
@@ -207,12 +257,20 @@
 #[derive(Debug)]
 struct AssumeRoleWithSAMLRequestSerializer;
 impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for AssumeRoleWithSAMLRequestSerializer {
-    #[allow(unused_mut, clippy::let_and_return, clippy::needless_borrow, clippy::useless_conversion)]
+    #[allow(
+        unused_mut,
+        clippy::let_and_return,
+        clippy::needless_borrow,
+        clippy::useless_conversion
+    )]
     fn serialize_input(
         &self,
         input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
         _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
-    ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
+    ) -> ::std::result::Result<
+        ::aws_smithy_runtime_api::client::orchestrator::HttpRequest,
+        ::aws_smithy_runtime_api::box_error::BoxError,
+    > {
         let input = input
             .downcast::<crate::operation::assume_role_with_saml::AssumeRoleWithSamlInput>()
             .expect("correct type");
@@ -234,22 +292,32 @@
             fn update_http_builder(
                 input: &crate::operation::assume_role_with_saml::AssumeRoleWithSamlInput,
                 builder: ::http_1x::request::Builder,
-            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError>
+            {
                 let mut uri = ::std::string::String::new();
                 uri_base(input, &mut uri)?;
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(
+                builder,
+                ::http_1x::header::CONTENT_TYPE,
+                "application/x-www-form-urlencoded",
+            );
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_assume_role_with_saml_input::ser_assume_role_with_saml_input_input_input(&input)?,
+            crate::protocol_serde::shape_assume_role_with_saml_input::ser_assume_role_with_saml_input_input_input(
+                &input,
+            )?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
-            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+            request_builder = _header_serialization_settings.set_default_header(
+                request_builder,
+                ::http_1x::header::CONTENT_LENGTH,
+                &content_length,
+            );
         }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
@@ -282,13 +350,21 @@
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
-            .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
+            .set_endpoint(
+                cfg.load::<::aws_types::endpoint_config::EndpointUrl>()
+                    .map(|ty| ty.0.clone()),
+            )
             .build()
             .map_err(|err| {
-                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new("endpoint params could not be built", err)
+                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new(
+                    "endpoint params could not be built",
+                    err,
+                )
             })?;
         cfg.interceptor_state()
-            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params));
+            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(
+                params,
+            ));
         ::std::result::Result::Ok(())
     }
 }
@@ -315,18 +391,22 @@
     /// <p>STS is not activated in the requested region for the account that is being asked to generate credentials. The account administrator must use the IAM console to activate STS in that region. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_enable-regions.html#sts-regions-activate-deactivate">Activating and Deactivating STS in an Amazon Web Services Region</a> in the <i>IAM User Guide</i>.</p>
     RegionDisabledException(crate::types::error::RegionDisabledException),
     /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
-    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
+    #[deprecated(
+        note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
     variable wildcard pattern and check `.code()`:
      \
     &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
      \
-    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-AssumeRoleWithSAMLError) for what information is available for the error.")]
+    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-AssumeRoleWithSAMLError) for what information is available for the error."
+    )]
     Unhandled(crate::error::sealed_unhandled::Unhandled),
 }
 impl AssumeRoleWithSAMLError {
     /// Creates the `AssumeRoleWithSAMLError::Unhandled` variant from any error type.
     pub fn unhandled(
-        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
+        err: impl ::std::convert::Into<
+            ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
+        >,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
             source: err.into(),
@@ -349,9 +429,15 @@
         match self {
             Self::ExpiredTokenException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
             Self::IdpRejectedClaimException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::InvalidIdentityTokenException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::MalformedPolicyDocumentException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::PackedPolicyTooLargeException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::InvalidIdentityTokenException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
+            Self::MalformedPolicyDocumentException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
+            Self::PackedPolicyTooLargeException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
             Self::RegionDisabledException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
             Self::Unhandled(e) => &e.meta,
         }
@@ -404,7 +490,9 @@
             Self::PackedPolicyTooLargeException(_inner) => _inner.fmt(f),
             Self::RegionDisabledException(_inner) => _inner.fmt(f),
             Self::Unhandled(_inner) => {
-                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
+                if let ::std::option::Option::Some(code) =
+                    ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
+                {
                     write!(f, "unhandled error ({code})")
                 } else {
                     f.write_str("unhandled error")
@@ -424,12 +512,24 @@
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for AssumeRoleWithSAMLError {
     fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::ExpiredTokenException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::IdpRejectedClaimException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::InvalidIdentityTokenException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::MalformedPolicyDocumentException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::PackedPolicyTooLargeException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::RegionDisabledException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::ExpiredTokenException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::IdpRejectedClaimException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::InvalidIdentityTokenException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::MalformedPolicyDocumentException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::PackedPolicyTooLargeException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::RegionDisabledException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
             Self::Unhandled(_inner) => &_inner.meta,
         }
     }
diff --git a/src/operation/assume_role_with_saml/_assume_role_with_saml_input.rs b/src/operation/assume_role_with_saml/_assume_role_with_saml_input.rs
--- a/src/operation/assume_role_with_saml/_assume_role_with_saml_input.rs
+++ b/src/operation/assume_role_with_saml/_assume_role_with_saml_input.rs
@@ -169,7 +169,10 @@
     /// <p>An Amazon Web Services conversion compresses the passed inline session policy, managed policy ARNs, and session tags into a packed binary format that has a separate limit. Your request can fail for this limit even if your plaintext meets the other requirements. The <code>PackedPolicySize</code> response element indicates by percentage how close the policies and tags for your request are to the upper size limit.</p>
     /// </note>
     /// <p>Passing policies to this operation returns new temporary credentials. The resulting session's permissions are the intersection of the role's identity-based policy and the session policies. You can use the role's temporary credentials in subsequent Amazon Web Services API calls to access resources in the account that owns the role. You cannot use session policies to grant more permissions than those allowed by the identity-based policy of the role that is being assumed. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#policies_session">Session Policies</a> in the <i>IAM User Guide</i>.</p>
-    pub fn set_policy_arns(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::PolicyDescriptorType>>) -> Self {
+    pub fn set_policy_arns(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<crate::types::PolicyDescriptorType>>,
+    ) -> Self {
         self.policy_arns = input;
         self
     }
@@ -236,8 +239,10 @@
     /// Consumes the builder and constructs a [`AssumeRoleWithSamlInput`](crate::operation::assume_role_with_saml::AssumeRoleWithSamlInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<crate::operation::assume_role_with_saml::AssumeRoleWithSamlInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        crate::operation::assume_role_with_saml::AssumeRoleWithSamlInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(crate::operation::assume_role_with_saml::AssumeRoleWithSamlInput {
             role_arn: self.role_arn,
             principal_arn: self.principal_arn,
diff --git a/src/operation/assume_role_with_saml/builders.rs b/src/operation/assume_role_with_saml/builders.rs
--- a/src/operation/assume_role_with_saml/builders.rs
+++ b/src/operation/assume_role_with_saml/builders.rs
@@ -133,12 +133,18 @@
     > {
         crate::client::customize::CustomizableOperation::new(self)
     }
-    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
+    pub(crate) fn config_override(
+        mut self,
+        config_override: impl ::std::convert::Into<crate::config::Builder>,
+    ) -> Self {
         self.set_config_override(::std::option::Option::Some(config_override.into()));
         self
     }

-    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
+    pub(crate) fn set_config_override(
+        &mut self,
+        config_override: ::std::option::Option<crate::config::Builder>,
+    ) -> &mut Self {
         self.config_override = config_override;
         self
     }
@@ -206,7 +212,10 @@
     /// <p>An Amazon Web Services conversion compresses the passed inline session policy, managed policy ARNs, and session tags into a packed binary format that has a separate limit. Your request can fail for this limit even if your plaintext meets the other requirements. The <code>PackedPolicySize</code> response element indicates by percentage how close the policies and tags for your request are to the upper size limit.</p>
     /// </note>
     /// <p>Passing policies to this operation returns new temporary credentials. The resulting session's permissions are the intersection of the role's identity-based policy and the session policies. You can use the role's temporary credentials in subsequent Amazon Web Services API calls to access resources in the account that owns the role. You cannot use session policies to grant more permissions than those allowed by the identity-based policy of the role that is being assumed. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#policies_session">Session Policies</a> in the <i>IAM User Guide</i>.</p>
-    pub fn set_policy_arns(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::PolicyDescriptorType>>) -> Self {
+    pub fn set_policy_arns(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<crate::types::PolicyDescriptorType>>,
+    ) -> Self {
         self.inner = self.inner.set_policy_arns(input);
         self
     }
diff --git a/src/operation/assume_role_with_web_identity.rs b/src/operation/assume_role_with_web_identity.rs
--- a/src/operation/assume_role_with_web_identity.rs
+++ b/src/operation/assume_role_with_web_identity.rs
@@ -27,9 +27,13 @@
                     .expect("correct error type")
             })
         };
-        let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
-            .await
-            .map_err(map_err)?;
+        let context = Self::orchestrate_with_stop_point(
+            runtime_plugins,
+            input,
+            ::aws_smithy_runtime::client::orchestrator::StopPoint::None,
+        )
+        .await
+        .map_err(map_err)?;
         let output = context.finalize().map_err(map_err)?;
         ::std::result::Result::Ok(
             output
@@ -51,17 +55,23 @@
     > {
         let input = ::aws_smithy_runtime_api::client::interceptors::context::Input::erase(input);
         use ::tracing::Instrument;
-        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("STS", "AssumeRoleWithWebIdentity", input, runtime_plugins, stop_point)
-            // Create a parent span for the entire operation. Includes a random, internal-only,
-            // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
-            .instrument(::tracing::debug_span!(
-                "STS.AssumeRoleWithWebIdentity",
-                "rpc.service" = "STS",
-                "rpc.method" = "AssumeRoleWithWebIdentity",
-                "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
-                "rpc.system" = "aws-api",
-            ))
-            .await
+        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point(
+            "STS",
+            "AssumeRoleWithWebIdentity",
+            input,
+            runtime_plugins,
+            stop_point,
+        )
+        // Create a parent span for the entire operation. Includes a random, internal-only,
+        // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
+        .instrument(::tracing::debug_span!(
+            "STS.AssumeRoleWithWebIdentity",
+            "rpc.service" = "STS",
+            "rpc.method" = "AssumeRoleWithWebIdentity",
+            "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
+            "rpc.system" = "aws-api",
+        ))
+        .await
     }

     pub(crate) fn operation_runtime_plugins(
@@ -91,16 +101,20 @@
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
             AssumeRoleWithWebIdentityRequestSerializer,
         ));
-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            AssumeRoleWithWebIdentityResponseDeserializer,
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
+                AssumeRoleWithWebIdentityResponseDeserializer,
+            ),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
-            crate::config::auth::Params::builder()
-                .operation_name("AssumeRoleWithWebIdentity")
-                .build()
-                .expect("required fields set"),
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
+                crate::config::auth::Params::builder()
+                    .operation_name("AssumeRoleWithWebIdentity")
+                    .build()
+                    .expect("required fields set"),
+            ),
+        );

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::SensitiveOutput);
         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
@@ -116,33 +130,46 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("AssumeRoleWithWebIdentity")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new(
+            "AssumeRoleWithWebIdentity",
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 AssumeRoleWithWebIdentityTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+            ),
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+            ),
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 AssumeRoleWithWebIdentityEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+            ),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError,
-            >::new())
-            .with_retry_classifier(
-                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                    crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError,
-                >::builder()
-                .transient_errors({
-                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
-                    transient_errors.push("IDPCommunicationError");
-                    ::std::borrow::Cow::Owned(transient_errors)
-                })
-                .build(),
-            );
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError,
+            >::builder()
+            .transient_errors({
+                let mut transient_errors: Vec<&'static str> =
+                    ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                transient_errors.push("IDPCommunicationError");
+                ::std::borrow::Cow::Owned(transient_errors)
+            })
+            .build(),
+        );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -152,7 +179,9 @@
 struct AssumeRoleWithWebIdentityTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for AssumeRoleWithWebIdentityTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept
+    for AssumeRoleWithWebIdentityTelemetryInputCaptureInterceptor
+{
     fn name(&self) -> &'static str {
         "AssumeRoleWithWebIdentityTelemetryInputCaptureInterceptor"
     }
@@ -175,7 +204,8 @@
             return ::std::result::Result::Ok(());
         };

-        let ::std::option::Option::Some(input) = context.input().downcast_ref::<AssumeRoleWithWebIdentityInput>() else {
+        let ::std::option::Option::Some(input) = context.input().downcast_ref::<AssumeRoleWithWebIdentityInput>()
+        else {
             // A mismatched input is not this interceptor's concern; skip quietly.
             return ::std::result::Result::Ok(());
         };
@@ -221,9 +251,13 @@
         let mut force_error = false;
         ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));
         let parse_result = if !success && status != 200 || force_error {
-            crate::protocol_serde::shape_assume_role_with_web_identity::de_assume_role_with_web_identity_http_error(status, headers, body)
+            crate::protocol_serde::shape_assume_role_with_web_identity::de_assume_role_with_web_identity_http_error(
+                status, headers, body,
+            )
         } else {
-            crate::protocol_serde::shape_assume_role_with_web_identity::de_assume_role_with_web_identity_http_response(status, headers, body)
+            crate::protocol_serde::shape_assume_role_with_web_identity::de_assume_role_with_web_identity_http_response(
+                status, headers, body,
+            )
         };
         crate::protocol_serde::type_erase_result(parse_result)
     }
@@ -231,12 +265,20 @@
 #[derive(Debug)]
 struct AssumeRoleWithWebIdentityRequestSerializer;
 impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for AssumeRoleWithWebIdentityRequestSerializer {
-    #[allow(unused_mut, clippy::let_and_return, clippy::needless_borrow, clippy::useless_conversion)]
+    #[allow(
+        unused_mut,
+        clippy::let_and_return,
+        clippy::needless_borrow,
+        clippy::useless_conversion
+    )]
     fn serialize_input(
         &self,
         input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
         _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
-    ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
+    ) -> ::std::result::Result<
+        ::aws_smithy_runtime_api::client::orchestrator::HttpRequest,
+        ::aws_smithy_runtime_api::box_error::BoxError,
+    > {
         let input = input
             .downcast::<crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityInput>()
             .expect("correct type");
@@ -258,14 +300,18 @@
             fn update_http_builder(
                 input: &crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityInput,
                 builder: ::http_1x::request::Builder,
-            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError>
+            {
                 let mut uri = ::std::string::String::new();
                 uri_base(input, &mut uri)?;
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(
+                builder,
+                ::http_1x::header::CONTENT_TYPE,
+                "application/x-www-form-urlencoded",
+            );
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
@@ -273,7 +319,11 @@
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
-            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+            request_builder = _header_serialization_settings.set_default_header(
+                request_builder,
+                ::http_1x::header::CONTENT_LENGTH,
+                &content_length,
+            );
         }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
@@ -306,13 +356,21 @@
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
-            .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
+            .set_endpoint(
+                cfg.load::<::aws_types::endpoint_config::EndpointUrl>()
+                    .map(|ty| ty.0.clone()),
+            )
             .build()
             .map_err(|err| {
-                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new("endpoint params could not be built", err)
+                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new(
+                    "endpoint params could not be built",
+                    err,
+                )
             })?;
         cfg.interceptor_state()
-            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params));
+            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(
+                params,
+            ));
         ::std::result::Result::Ok(())
     }
 }
@@ -341,18 +399,22 @@
     /// <p>STS is not activated in the requested region for the account that is being asked to generate credentials. The account administrator must use the IAM console to activate STS in that region. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_enable-regions.html#sts-regions-activate-deactivate">Activating and Deactivating STS in an Amazon Web Services Region</a> in the <i>IAM User Guide</i>.</p>
     RegionDisabledException(crate::types::error::RegionDisabledException),
     /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
-    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
+    #[deprecated(
+        note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
     variable wildcard pattern and check `.code()`:
      \
     &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
      \
-    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-AssumeRoleWithWebIdentityError) for what information is available for the error.")]
+    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-AssumeRoleWithWebIdentityError) for what information is available for the error."
+    )]
     Unhandled(crate::error::sealed_unhandled::Unhandled),
 }
 impl AssumeRoleWithWebIdentityError {
     /// Creates the `AssumeRoleWithWebIdentityError::Unhandled` variant from any error type.
     pub fn unhandled(
-        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
+        err: impl ::std::convert::Into<
+            ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
+        >,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
             source: err.into(),
@@ -374,11 +436,19 @@
     pub fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
             Self::ExpiredTokenException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::IdpCommunicationErrorException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::IdpCommunicationErrorException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
             Self::IdpRejectedClaimException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::InvalidIdentityTokenException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::MalformedPolicyDocumentException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::PackedPolicyTooLargeException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::InvalidIdentityTokenException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
+            Self::MalformedPolicyDocumentException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
+            Self::PackedPolicyTooLargeException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
             Self::RegionDisabledException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
             Self::Unhandled(e) => &e.meta,
         }
@@ -437,7 +507,9 @@
             Self::PackedPolicyTooLargeException(_inner) => _inner.fmt(f),
             Self::RegionDisabledException(_inner) => _inner.fmt(f),
             Self::Unhandled(_inner) => {
-                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
+                if let ::std::option::Option::Some(code) =
+                    ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
+                {
                     write!(f, "unhandled error ({code})")
                 } else {
                     f.write_str("unhandled error")
@@ -460,13 +532,27 @@
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for AssumeRoleWithWebIdentityError {
     fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::ExpiredTokenException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::IdpCommunicationErrorException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::IdpRejectedClaimException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::InvalidIdentityTokenException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::MalformedPolicyDocumentException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::PackedPolicyTooLargeException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::RegionDisabledException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::ExpiredTokenException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::IdpCommunicationErrorException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::IdpRejectedClaimException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::InvalidIdentityTokenException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::MalformedPolicyDocumentException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::PackedPolicyTooLargeException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::RegionDisabledException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
             Self::Unhandled(_inner) => &_inner.meta,
         }
     }
@@ -482,7 +568,9 @@
         })
     }
 }
-impl ::aws_types::request_id::RequestId for crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError {
+impl ::aws_types::request_id::RequestId
+    for crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError
+{
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
     }
diff --git a/src/operation/assume_role_with_web_identity/_assume_role_with_web_identity_input.rs b/src/operation/assume_role_with_web_identity/_assume_role_with_web_identity_input.rs
--- a/src/operation/assume_role_with_web_identity/_assume_role_with_web_identity_input.rs
+++ b/src/operation/assume_role_with_web_identity/_assume_role_with_web_identity_input.rs
@@ -101,7 +101,8 @@
 }
 impl AssumeRoleWithWebIdentityInput {
     /// Creates a new builder-style object to manufacture [`AssumeRoleWithWebIdentityInput`](crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityInput).
-    pub fn builder() -> crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityInputBuilder {
+    pub fn builder() -> crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityInputBuilder
+    {
         crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityInputBuilder::default()
     }
 }
@@ -216,7 +217,10 @@
     /// <p>An Amazon Web Services conversion compresses the passed inline session policy, managed policy ARNs, and session tags into a packed binary format that has a separate limit. Your request can fail for this limit even if your plaintext meets the other requirements. The <code>PackedPolicySize</code> response element indicates by percentage how close the policies and tags for your request are to the upper size limit.</p>
     /// </note>
     /// <p>Passing policies to this operation returns new temporary credentials. The resulting session's permissions are the intersection of the role's identity-based policy and the session policies. You can use the role's temporary credentials in subsequent Amazon Web Services API calls to access resources in the account that owns the role. You cannot use session policies to grant more permissions than those allowed by the identity-based policy of the role that is being assumed. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#policies_session">Session Policies</a> in the <i>IAM User Guide</i>.</p>
-    pub fn set_policy_arns(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::PolicyDescriptorType>>) -> Self {
+    pub fn set_policy_arns(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<crate::types::PolicyDescriptorType>>,
+    ) -> Self {
         self.policy_arns = input;
         self
     }
@@ -287,15 +291,17 @@
         crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityInput {
-            role_arn: self.role_arn,
-            role_session_name: self.role_session_name,
-            web_identity_token: self.web_identity_token,
-            provider_id: self.provider_id,
-            policy_arns: self.policy_arns,
-            policy: self.policy,
-            duration_seconds: self.duration_seconds,
-        })
+        ::std::result::Result::Ok(
+            crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityInput {
+                role_arn: self.role_arn,
+                role_session_name: self.role_session_name,
+                web_identity_token: self.web_identity_token,
+                provider_id: self.provider_id,
+                policy_arns: self.policy_arns,
+                policy: self.policy,
+                duration_seconds: self.duration_seconds,
+            },
+        )
     }
 }
 impl ::std::fmt::Debug for AssumeRoleWithWebIdentityInputBuilder {
diff --git a/src/operation/assume_role_with_web_identity/_assume_role_with_web_identity_output.rs b/src/operation/assume_role_with_web_identity/_assume_role_with_web_identity_output.rs
--- a/src/operation/assume_role_with_web_identity/_assume_role_with_web_identity_output.rs
+++ b/src/operation/assume_role_with_web_identity/_assume_role_with_web_identity_output.rs
@@ -79,7 +79,8 @@
 }
 impl AssumeRoleWithWebIdentityOutput {
     /// Creates a new builder-style object to manufacture [`AssumeRoleWithWebIdentityOutput`](crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityOutput).
-    pub fn builder() -> crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityOutputBuilder {
+    pub fn builder() -> crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityOutputBuilder
+    {
         crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityOutputBuilder::default()
     }
 }
diff --git a/src/operation/assume_role_with_web_identity/builders.rs b/src/operation/assume_role_with_web_identity/builders.rs
--- a/src/operation/assume_role_with_web_identity/builders.rs
+++ b/src/operation/assume_role_with_web_identity/builders.rs
@@ -86,7 +86,9 @@
         }
     }
     /// Access the AssumeRoleWithWebIdentity as a reference.
-    pub fn as_input(&self) -> &crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityInputBuilder {
+    pub fn as_input(
+        &self,
+    ) -> &crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityInputBuilder {
         &self.inner
     }
     /// Sends the request and returns the response.
@@ -110,12 +112,14 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentity::operation_runtime_plugins(
-            self.handle.runtime_plugins.clone(),
-            &self.handle.conf,
-            self.config_override,
-        );
-        crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentity::orchestrate(&runtime_plugins, input).await
+        let runtime_plugins =
+            crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentity::operation_runtime_plugins(
+                self.handle.runtime_plugins.clone(),
+                &self.handle.conf,
+                self.config_override,
+            );
+        crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentity::orchestrate(&runtime_plugins, input)
+            .await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -128,12 +132,18 @@
     > {
         crate::client::customize::CustomizableOperation::new(self)
     }
-    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
+    pub(crate) fn config_override(
+        mut self,
+        config_override: impl ::std::convert::Into<crate::config::Builder>,
+    ) -> Self {
         self.set_config_override(::std::option::Option::Some(config_override.into()));
         self
     }

-    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
+    pub(crate) fn set_config_override(
+        &mut self,
+        config_override: ::std::option::Option<crate::config::Builder>,
+    ) -> &mut Self {
         self.config_override = config_override;
         self
     }
@@ -230,7 +240,10 @@
     /// <p>An Amazon Web Services conversion compresses the passed inline session policy, managed policy ARNs, and session tags into a packed binary format that has a separate limit. Your request can fail for this limit even if your plaintext meets the other requirements. The <code>PackedPolicySize</code> response element indicates by percentage how close the policies and tags for your request are to the upper size limit.</p>
     /// </note>
     /// <p>Passing policies to this operation returns new temporary credentials. The resulting session's permissions are the intersection of the role's identity-based policy and the session policies. You can use the role's temporary credentials in subsequent Amazon Web Services API calls to access resources in the account that owns the role. You cannot use session policies to grant more permissions than those allowed by the identity-based policy of the role that is being assumed. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#policies_session">Session Policies</a> in the <i>IAM User Guide</i>.</p>
-    pub fn set_policy_arns(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::PolicyDescriptorType>>) -> Self {
+    pub fn set_policy_arns(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<crate::types::PolicyDescriptorType>>,
+    ) -> Self {
         self.inner = self.inner.set_policy_arns(input);
         self
     }
diff --git a/src/operation/assume_root.rs b/src/operation/assume_root.rs
--- a/src/operation/assume_root.rs
+++ b/src/operation/assume_root.rs
@@ -27,9 +27,13 @@
                     .expect("correct error type")
             })
         };
-        let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
-            .await
-            .map_err(map_err)?;
+        let context = Self::orchestrate_with_stop_point(
+            runtime_plugins,
+            input,
+            ::aws_smithy_runtime::client::orchestrator::StopPoint::None,
+        )
+        .await
+        .map_err(map_err)?;
         let output = context.finalize().map_err(map_err)?;
         ::std::result::Result::Ok(
             output
@@ -51,17 +55,23 @@
     > {
         let input = ::aws_smithy_runtime_api::client::interceptors::context::Input::erase(input);
         use ::tracing::Instrument;
-        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("STS", "AssumeRoot", input, runtime_plugins, stop_point)
-            // Create a parent span for the entire operation. Includes a random, internal-only,
-            // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
-            .instrument(::tracing::debug_span!(
-                "STS.AssumeRoot",
-                "rpc.service" = "STS",
-                "rpc.method" = "AssumeRoot",
-                "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
-                "rpc.system" = "aws-api",
-            ))
-            .await
+        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point(
+            "STS",
+            "AssumeRoot",
+            input,
+            runtime_plugins,
+            stop_point,
+        )
+        // Create a parent span for the entire operation. Includes a random, internal-only,
+        // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
+        .instrument(::tracing::debug_span!(
+            "STS.AssumeRoot",
+            "rpc.service" = "STS",
+            "rpc.method" = "AssumeRoot",
+            "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
+            "rpc.system" = "aws-api",
+        ))
+        .await
     }

     pub(crate) fn operation_runtime_plugins(
@@ -91,19 +101,24 @@
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
             AssumeRootRequestSerializer,
         ));
-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            AssumeRootResponseDeserializer,
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(AssumeRootResponseDeserializer),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
-            crate::config::auth::Params::builder()
-                .operation_name("AssumeRoot")
-                .build()
-                .expect("required fields set"),
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
+                crate::config::auth::Params::builder()
+                    .operation_name("AssumeRoot")
+                    .build()
+                    .expect("required fields set"),
+            ),
+        );

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::SensitiveOutput);
-        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("AssumeRoot", "STS"));
+        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
+            "AssumeRoot",
+            "STS",
+        ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
         signing_options.double_uri_encode = true;
         signing_options.content_sha256_header = false;
@@ -221,13 +236,23 @@
 #[derive(Debug)]
 struct AssumeRootRequestSerializer;
 impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for AssumeRootRequestSerializer {
-    #[allow(unused_mut, clippy::let_and_return, clippy::needless_borrow, clippy::useless_conversion)]
+    #[allow(
+        unused_mut,
+        clippy::let_and_return,
+        clippy::needless_borrow,
+        clippy::useless_conversion
+    )]
     fn serialize_input(
         &self,
         input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
         _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
-    ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
-        let input = input.downcast::<crate::operation::assume_root::AssumeRootInput>().expect("correct type");
+    ) -> ::std::result::Result<
+        ::aws_smithy_runtime_api::client::orchestrator::HttpRequest,
+        ::aws_smithy_runtime_api::box_error::BoxError,
+    > {
+        let input = input
+            .downcast::<crate::operation::assume_root::AssumeRootInput>()
+            .expect("correct type");
         let _header_serialization_settings = _cfg
             .load::<crate::serialization_settings::HeaderSerializationSettings>()
             .cloned()
@@ -246,21 +271,30 @@
             fn update_http_builder(
                 input: &crate::operation::assume_root::AssumeRootInput,
                 builder: ::http_1x::request::Builder,
-            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError>
+            {
                 let mut uri = ::std::string::String::new();
                 uri_base(input, &mut uri)?;
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(
+                builder,
+                ::http_1x::header::CONTENT_TYPE,
+                "application/x-www-form-urlencoded",
+            );
             builder
         };
-        let body =
-            ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_assume_root_input::ser_assume_root_input_input_input(&input)?);
+        let body = ::aws_smithy_types::body::SdkBody::from(
+            crate::protocol_serde::shape_assume_root_input::ser_assume_root_input_input_input(&input)?,
+        );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
-            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+            request_builder = _header_serialization_settings.set_default_header(
+                request_builder,
+                ::http_1x::header::CONTENT_LENGTH,
+                &content_length,
+            );
         }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
@@ -293,13 +327,21 @@
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
-            .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
+            .set_endpoint(
+                cfg.load::<::aws_types::endpoint_config::EndpointUrl>()
+                    .map(|ty| ty.0.clone()),
+            )
             .build()
             .map_err(|err| {
-                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new("endpoint params could not be built", err)
+                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new(
+                    "endpoint params could not be built",
+                    err,
+                )
             })?;
         cfg.interceptor_state()
-            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params));
+            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(
+                params,
+            ));
         ::std::result::Result::Ok(())
     }
 }
@@ -316,18 +358,22 @@
     /// <p>STS is not activated in the requested region for the account that is being asked to generate credentials. The account administrator must use the IAM console to activate STS in that region. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_enable-regions.html#sts-regions-activate-deactivate">Activating and Deactivating STS in an Amazon Web Services Region</a> in the <i>IAM User Guide</i>.</p>
     RegionDisabledException(crate::types::error::RegionDisabledException),
     /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
-    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
+    #[deprecated(
+        note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
     variable wildcard pattern and check `.code()`:
      \
     &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
      \
-    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-AssumeRootError) for what information is available for the error.")]
+    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-AssumeRootError) for what information is available for the error."
+    )]
     Unhandled(crate::error::sealed_unhandled::Unhandled),
 }
 impl AssumeRootError {
     /// Creates the `AssumeRootError::Unhandled` variant from any error type.
     pub fn unhandled(
-        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
+        err: impl ::std::convert::Into<
+            ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
+        >,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
             source: err.into(),
@@ -377,7 +423,9 @@
             Self::ExpiredTokenException(_inner) => _inner.fmt(f),
             Self::RegionDisabledException(_inner) => _inner.fmt(f),
             Self::Unhandled(_inner) => {
-                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
+                if let ::std::option::Option::Some(code) =
+                    ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
+                {
                     write!(f, "unhandled error ({code})")
                 } else {
                     f.write_str("unhandled error")
@@ -397,8 +445,12 @@
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for AssumeRootError {
     fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::ExpiredTokenException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::RegionDisabledException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::ExpiredTokenException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::RegionDisabledException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
             Self::Unhandled(_inner) => &_inner.meta,
         }
     }
diff --git a/src/operation/assume_root/_assume_root_input.rs b/src/operation/assume_root/_assume_root_input.rs
--- a/src/operation/assume_root/_assume_root_input.rs
+++ b/src/operation/assume_root/_assume_root_input.rs
@@ -150,7 +150,12 @@
         &self.duration_seconds
     }
     /// Consumes the builder and constructs a [`AssumeRootInput`](crate::operation::assume_root::AssumeRootInput).
-    pub fn build(self) -> ::std::result::Result<crate::operation::assume_root::AssumeRootInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<
+        crate::operation::assume_root::AssumeRootInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(crate::operation::assume_root::AssumeRootInput {
             target_principal: self.target_principal,
             task_policy_arn: self.task_policy_arn,
diff --git a/src/operation/assume_root/builders.rs b/src/operation/assume_root/builders.rs
--- a/src/operation/assume_root/builders.rs
+++ b/src/operation/assume_root/builders.rs
@@ -104,12 +104,18 @@
     > {
         crate::client::customize::CustomizableOperation::new(self)
     }
-    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
+    pub(crate) fn config_override(
+        mut self,
+        config_override: impl ::std::convert::Into<crate::config::Builder>,
+    ) -> Self {
         self.set_config_override(::std::option::Option::Some(config_override.into()));
         self
     }

-    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
+    pub(crate) fn set_config_override(
+        &mut self,
+        config_override: ::std::option::Option<crate::config::Builder>,
+    ) -> &mut Self {
         self.config_override = config_override;
         self
     }
diff --git a/src/operation/decode_authorization_message.rs b/src/operation/decode_authorization_message.rs
--- a/src/operation/decode_authorization_message.rs
+++ b/src/operation/decode_authorization_message.rs
@@ -27,9 +27,13 @@
                     .expect("correct error type")
             })
         };
-        let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
-            .await
-            .map_err(map_err)?;
+        let context = Self::orchestrate_with_stop_point(
+            runtime_plugins,
+            input,
+            ::aws_smithy_runtime::client::orchestrator::StopPoint::None,
+        )
+        .await
+        .map_err(map_err)?;
         let output = context.finalize().map_err(map_err)?;
         ::std::result::Result::Ok(
             output
@@ -51,17 +55,23 @@
     > {
         let input = ::aws_smithy_runtime_api::client::interceptors::context::Input::erase(input);
         use ::tracing::Instrument;
-        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("STS", "DecodeAuthorizationMessage", input, runtime_plugins, stop_point)
-            // Create a parent span for the entire operation. Includes a random, internal-only,
-            // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
-            .instrument(::tracing::debug_span!(
-                "STS.DecodeAuthorizationMessage",
-                "rpc.service" = "STS",
-                "rpc.method" = "DecodeAuthorizationMessage",
-                "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
-                "rpc.system" = "aws-api",
-            ))
-            .await
+        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point(
+            "STS",
+            "DecodeAuthorizationMessage",
+            input,
+            runtime_plugins,
+            stop_point,
+        )
+        // Create a parent span for the entire operation. Includes a random, internal-only,
+        // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
+        .instrument(::tracing::debug_span!(
+            "STS.DecodeAuthorizationMessage",
+            "rpc.service" = "STS",
+            "rpc.method" = "DecodeAuthorizationMessage",
+            "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
+            "rpc.system" = "aws-api",
+        ))
+        .await
     }

     pub(crate) fn operation_runtime_plugins(
@@ -91,16 +101,20 @@
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
             DecodeAuthorizationMessageRequestSerializer,
         ));
-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            DecodeAuthorizationMessageResponseDeserializer,
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
+                DecodeAuthorizationMessageResponseDeserializer,
+            ),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
-            crate::config::auth::Params::builder()
-                .operation_name("DecodeAuthorizationMessage")
-                .build()
-                .expect("required fields set"),
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
+                crate::config::auth::Params::builder()
+                    .operation_name("DecodeAuthorizationMessage")
+                    .build()
+                    .expect("required fields set"),
+            ),
+        );

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
             "DecodeAuthorizationMessage",
@@ -125,33 +139,46 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("DecodeAuthorizationMessage")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new(
+            "DecodeAuthorizationMessage",
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 DecodeAuthorizationMessageTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+            ),
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+            ),
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 DecodeAuthorizationMessageEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+            ),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 crate::operation::decode_authorization_message::DecodeAuthorizationMessageError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+                crate::operation::decode_authorization_message::DecodeAuthorizationMessageError,
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
                 crate::operation::decode_authorization_message::DecodeAuthorizationMessageError,
-            >::new())
-            .with_retry_classifier(
-                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                    crate::operation::decode_authorization_message::DecodeAuthorizationMessageError,
-                >::builder()
-                .transient_errors({
-                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
-                    transient_errors.push("IDPCommunicationError");
-                    ::std::borrow::Cow::Owned(transient_errors)
-                })
-                .build(),
-            );
+            >::builder()
+            .transient_errors({
+                let mut transient_errors: Vec<&'static str> =
+                    ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                transient_errors.push("IDPCommunicationError");
+                ::std::borrow::Cow::Owned(transient_errors)
+            })
+            .build(),
+        );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -161,7 +188,9 @@
 struct DecodeAuthorizationMessageTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for DecodeAuthorizationMessageTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept
+    for DecodeAuthorizationMessageTelemetryInputCaptureInterceptor
+{
     fn name(&self) -> &'static str {
         "DecodeAuthorizationMessageTelemetryInputCaptureInterceptor"
     }
@@ -184,7 +213,8 @@
             return ::std::result::Result::Ok(());
         };

-        let ::std::option::Option::Some(input) = context.input().downcast_ref::<DecodeAuthorizationMessageInput>() else {
+        let ::std::option::Option::Some(input) = context.input().downcast_ref::<DecodeAuthorizationMessageInput>()
+        else {
             // A mismatched input is not this interceptor's concern; skip quietly.
             return ::std::result::Result::Ok(());
         };
@@ -215,9 +245,13 @@
         let mut force_error = false;
         ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));
         let parse_result = if !success && status != 200 || force_error {
-            crate::protocol_serde::shape_decode_authorization_message::de_decode_authorization_message_http_error(status, headers, body)
+            crate::protocol_serde::shape_decode_authorization_message::de_decode_authorization_message_http_error(
+                status, headers, body,
+            )
         } else {
-            crate::protocol_serde::shape_decode_authorization_message::de_decode_authorization_message_http_response(status, headers, body)
+            crate::protocol_serde::shape_decode_authorization_message::de_decode_authorization_message_http_response(
+                status, headers, body,
+            )
         };
         crate::protocol_serde::type_erase_result(parse_result)
     }
@@ -225,12 +259,20 @@
 #[derive(Debug)]
 struct DecodeAuthorizationMessageRequestSerializer;
 impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for DecodeAuthorizationMessageRequestSerializer {
-    #[allow(unused_mut, clippy::let_and_return, clippy::needless_borrow, clippy::useless_conversion)]
+    #[allow(
+        unused_mut,
+        clippy::let_and_return,
+        clippy::needless_borrow,
+        clippy::useless_conversion
+    )]
     fn serialize_input(
         &self,
         input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
         _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
-    ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
+    ) -> ::std::result::Result<
+        ::aws_smithy_runtime_api::client::orchestrator::HttpRequest,
+        ::aws_smithy_runtime_api::box_error::BoxError,
+    > {
         let input = input
             .downcast::<crate::operation::decode_authorization_message::DecodeAuthorizationMessageInput>()
             .expect("correct type");
@@ -252,14 +294,18 @@
             fn update_http_builder(
                 input: &crate::operation::decode_authorization_message::DecodeAuthorizationMessageInput,
                 builder: ::http_1x::request::Builder,
-            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError>
+            {
                 let mut uri = ::std::string::String::new();
                 uri_base(input, &mut uri)?;
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(
+                builder,
+                ::http_1x::header::CONTENT_TYPE,
+                "application/x-www-form-urlencoded",
+            );
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
@@ -267,7 +313,11 @@
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
-            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+            request_builder = _header_serialization_settings.set_default_header(
+                request_builder,
+                ::http_1x::header::CONTENT_LENGTH,
+                &content_length,
+            );
         }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
@@ -300,13 +350,21 @@
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
-            .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
+            .set_endpoint(
+                cfg.load::<::aws_types::endpoint_config::EndpointUrl>()
+                    .map(|ty| ty.0.clone()),
+            )
             .build()
             .map_err(|err| {
-                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new("endpoint params could not be built", err)
+                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new(
+                    "endpoint params could not be built",
+                    err,
+                )
             })?;
         cfg.interceptor_state()
-            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params));
+            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(
+                params,
+            ));
         ::std::result::Result::Ok(())
     }
 }
@@ -321,18 +379,22 @@
     /// <p>The error returned if the message passed to <code>DecodeAuthorizationMessage</code> was invalid. This can happen if the token contains invalid characters, such as line breaks, or if the message has expired.</p>
     InvalidAuthorizationMessageException(crate::types::error::InvalidAuthorizationMessageException),
     /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
-    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
+    #[deprecated(
+        note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
     variable wildcard pattern and check `.code()`:
      \
     &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
      \
-    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-DecodeAuthorizationMessageError) for what information is available for the error.")]
+    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-DecodeAuthorizationMessageError) for what information is available for the error."
+    )]
     Unhandled(crate::error::sealed_unhandled::Unhandled),
 }
 impl DecodeAuthorizationMessageError {
     /// Creates the `DecodeAuthorizationMessageError::Unhandled` variant from any error type.
     pub fn unhandled(
-        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
+        err: impl ::std::convert::Into<
+            ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
+        >,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
             source: err.into(),
@@ -353,7 +415,9 @@
     ///
     pub fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::InvalidAuthorizationMessageException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::InvalidAuthorizationMessageException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
             Self::Unhandled(e) => &e.meta,
         }
     }
@@ -375,7 +439,9 @@
         match self {
             Self::InvalidAuthorizationMessageException(_inner) => _inner.fmt(f),
             Self::Unhandled(_inner) => {
-                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
+                if let ::std::option::Option::Some(code) =
+                    ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
+                {
                     write!(f, "unhandled error ({code})")
                 } else {
                     f.write_str("unhandled error")
@@ -395,7 +461,9 @@
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for DecodeAuthorizationMessageError {
     fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::InvalidAuthorizationMessageException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::InvalidAuthorizationMessageException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
             Self::Unhandled(_inner) => &_inner.meta,
         }
     }
@@ -411,7 +479,9 @@
         })
     }
 }
-impl ::aws_types::request_id::RequestId for crate::operation::decode_authorization_message::DecodeAuthorizationMessageError {
+impl ::aws_types::request_id::RequestId
+    for crate::operation::decode_authorization_message::DecodeAuthorizationMessageError
+{
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
     }
diff --git a/src/operation/decode_authorization_message/_decode_authorization_message_input.rs b/src/operation/decode_authorization_message/_decode_authorization_message_input.rs
--- a/src/operation/decode_authorization_message/_decode_authorization_message_input.rs
+++ b/src/operation/decode_authorization_message/_decode_authorization_message_input.rs
@@ -14,7 +14,8 @@
 }
 impl DecodeAuthorizationMessageInput {
     /// Creates a new builder-style object to manufacture [`DecodeAuthorizationMessageInput`](crate::operation::decode_authorization_message::DecodeAuthorizationMessageInput).
-    pub fn builder() -> crate::operation::decode_authorization_message::builders::DecodeAuthorizationMessageInputBuilder {
+    pub fn builder() -> crate::operation::decode_authorization_message::builders::DecodeAuthorizationMessageInputBuilder
+    {
         crate::operation::decode_authorization_message::builders::DecodeAuthorizationMessageInputBuilder::default()
     }
 }
@@ -48,8 +49,10 @@
         crate::operation::decode_authorization_message::DecodeAuthorizationMessageInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(crate::operation::decode_authorization_message::DecodeAuthorizationMessageInput {
-            encoded_message: self.encoded_message,
-        })
+        ::std::result::Result::Ok(
+            crate::operation::decode_authorization_message::DecodeAuthorizationMessageInput {
+                encoded_message: self.encoded_message,
+            },
+        )
     }
 }
diff --git a/src/operation/decode_authorization_message/_decode_authorization_message_output.rs b/src/operation/decode_authorization_message/_decode_authorization_message_output.rs
--- a/src/operation/decode_authorization_message/_decode_authorization_message_output.rs
+++ b/src/operation/decode_authorization_message/_decode_authorization_message_output.rs
@@ -21,7 +21,8 @@
 }
 impl DecodeAuthorizationMessageOutput {
     /// Creates a new builder-style object to manufacture [`DecodeAuthorizationMessageOutput`](crate::operation::decode_authorization_message::DecodeAuthorizationMessageOutput).
-    pub fn builder() -> crate::operation::decode_authorization_message::builders::DecodeAuthorizationMessageOutputBuilder {
+    pub fn builder() -> crate::operation::decode_authorization_message::builders::DecodeAuthorizationMessageOutputBuilder
+    {
         crate::operation::decode_authorization_message::builders::DecodeAuthorizationMessageOutputBuilder::default()
     }
 }
diff --git a/src/operation/decode_authorization_message/builders.rs b/src/operation/decode_authorization_message/builders.rs
--- a/src/operation/decode_authorization_message/builders.rs
+++ b/src/operation/decode_authorization_message/builders.rs
@@ -74,7 +74,9 @@
         }
     }
     /// Access the DecodeAuthorizationMessage as a reference.
-    pub fn as_input(&self) -> &crate::operation::decode_authorization_message::builders::DecodeAuthorizationMessageInputBuilder {
+    pub fn as_input(
+        &self,
+    ) -> &crate::operation::decode_authorization_message::builders::DecodeAuthorizationMessageInputBuilder {
         &self.inner
     }
     /// Sends the request and returns the response.
@@ -98,12 +100,14 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = crate::operation::decode_authorization_message::DecodeAuthorizationMessage::operation_runtime_plugins(
-            self.handle.runtime_plugins.clone(),
-            &self.handle.conf,
-            self.config_override,
-        );
-        crate::operation::decode_authorization_message::DecodeAuthorizationMessage::orchestrate(&runtime_plugins, input).await
+        let runtime_plugins =
+            crate::operation::decode_authorization_message::DecodeAuthorizationMessage::operation_runtime_plugins(
+                self.handle.runtime_plugins.clone(),
+                &self.handle.conf,
+                self.config_override,
+            );
+        crate::operation::decode_authorization_message::DecodeAuthorizationMessage::orchestrate(&runtime_plugins, input)
+            .await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -116,12 +120,18 @@
     > {
         crate::client::customize::CustomizableOperation::new(self)
     }
-    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
+    pub(crate) fn config_override(
+        mut self,
+        config_override: impl ::std::convert::Into<crate::config::Builder>,
+    ) -> Self {
         self.set_config_override(::std::option::Option::Some(config_override.into()));
         self
     }

-    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
+    pub(crate) fn set_config_override(
+        &mut self,
+        config_override: ::std::option::Option<crate::config::Builder>,
+    ) -> &mut Self {
         self.config_override = config_override;
         self
     }
diff --git a/src/operation/get_access_key_info.rs b/src/operation/get_access_key_info.rs
--- a/src/operation/get_access_key_info.rs
+++ b/src/operation/get_access_key_info.rs
@@ -27,9 +27,13 @@
                     .expect("correct error type")
             })
         };
-        let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
-            .await
-            .map_err(map_err)?;
+        let context = Self::orchestrate_with_stop_point(
+            runtime_plugins,
+            input,
+            ::aws_smithy_runtime::client::orchestrator::StopPoint::None,
+        )
+        .await
+        .map_err(map_err)?;
         let output = context.finalize().map_err(map_err)?;
         ::std::result::Result::Ok(
             output
@@ -51,17 +55,23 @@
     > {
         let input = ::aws_smithy_runtime_api::client::interceptors::context::Input::erase(input);
         use ::tracing::Instrument;
-        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("STS", "GetAccessKeyInfo", input, runtime_plugins, stop_point)
-            // Create a parent span for the entire operation. Includes a random, internal-only,
-            // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
-            .instrument(::tracing::debug_span!(
-                "STS.GetAccessKeyInfo",
-                "rpc.service" = "STS",
-                "rpc.method" = "GetAccessKeyInfo",
-                "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
-                "rpc.system" = "aws-api",
-            ))
-            .await
+        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point(
+            "STS",
+            "GetAccessKeyInfo",
+            input,
+            runtime_plugins,
+            stop_point,
+        )
+        // Create a parent span for the entire operation. Includes a random, internal-only,
+        // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
+        .instrument(::tracing::debug_span!(
+            "STS.GetAccessKeyInfo",
+            "rpc.service" = "STS",
+            "rpc.method" = "GetAccessKeyInfo",
+            "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
+            "rpc.system" = "aws-api",
+        ))
+        .await
     }

     pub(crate) fn operation_runtime_plugins(
@@ -91,18 +101,25 @@
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
             GetAccessKeyInfoRequestSerializer,
         ));
-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            GetAccessKeyInfoResponseDeserializer,
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
+                GetAccessKeyInfoResponseDeserializer,
+            ),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
-            crate::config::auth::Params::builder()
-                .operation_name("GetAccessKeyInfo")
-                .build()
-                .expect("required fields set"),
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
+                crate::config::auth::Params::builder()
+                    .operation_name("GetAccessKeyInfo")
+                    .build()
+                    .expect("required fields set"),
+            ),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("GetAccessKeyInfo", "STS"));
+        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
+            "GetAccessKeyInfo",
+            "STS",
+        ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
         signing_options.double_uri_encode = true;
         signing_options.content_sha256_header = false;
@@ -122,32 +139,46 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetAccessKeyInfo")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new(
+            "GetAccessKeyInfo",
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 GetAccessKeyInfoTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+            ),
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+            ),
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 GetAccessKeyInfoEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+            ),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 crate::operation::get_access_key_info::GetAccessKeyInfoError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::get_access_key_info::GetAccessKeyInfoError,
-            >::new())
-            .with_retry_classifier(
-                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::get_access_key_info::GetAccessKeyInfoError>::builder(
-                )
-                .transient_errors({
-                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
-                    transient_errors.push("IDPCommunicationError");
-                    ::std::borrow::Cow::Owned(transient_errors)
-                })
-                .build(),
-            );
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                crate::operation::get_access_key_info::GetAccessKeyInfoError,
+            >::builder()
+            .transient_errors({
+                let mut transient_errors: Vec<&'static str> =
+                    ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                transient_errors.push("IDPCommunicationError");
+                ::std::borrow::Cow::Owned(transient_errors)
+            })
+            .build(),
+        );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -213,7 +244,9 @@
         let parse_result = if !success && status != 200 || force_error {
             crate::protocol_serde::shape_get_access_key_info::de_get_access_key_info_http_error(status, headers, body)
         } else {
-            crate::protocol_serde::shape_get_access_key_info::de_get_access_key_info_http_response(status, headers, body)
+            crate::protocol_serde::shape_get_access_key_info::de_get_access_key_info_http_response(
+                status, headers, body,
+            )
         };
         crate::protocol_serde::type_erase_result(parse_result)
     }
@@ -221,12 +254,20 @@
 #[derive(Debug)]
 struct GetAccessKeyInfoRequestSerializer;
 impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for GetAccessKeyInfoRequestSerializer {
-    #[allow(unused_mut, clippy::let_and_return, clippy::needless_borrow, clippy::useless_conversion)]
+    #[allow(
+        unused_mut,
+        clippy::let_and_return,
+        clippy::needless_borrow,
+        clippy::useless_conversion
+    )]
     fn serialize_input(
         &self,
         input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
         _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
-    ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
+    ) -> ::std::result::Result<
+        ::aws_smithy_runtime_api::client::orchestrator::HttpRequest,
+        ::aws_smithy_runtime_api::box_error::BoxError,
+    > {
         let input = input
             .downcast::<crate::operation::get_access_key_info::GetAccessKeyInfoInput>()
             .expect("correct type");
@@ -248,14 +289,18 @@
             fn update_http_builder(
                 input: &crate::operation::get_access_key_info::GetAccessKeyInfoInput,
                 builder: ::http_1x::request::Builder,
-            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError>
+            {
                 let mut uri = ::std::string::String::new();
                 uri_base(input, &mut uri)?;
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(
+                builder,
+                ::http_1x::header::CONTENT_TYPE,
+                "application/x-www-form-urlencoded",
+            );
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
@@ -263,7 +308,11 @@
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
-            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+            request_builder = _header_serialization_settings.set_default_header(
+                request_builder,
+                ::http_1x::header::CONTENT_LENGTH,
+                &content_length,
+            );
         }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
@@ -296,13 +345,21 @@
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
-            .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
+            .set_endpoint(
+                cfg.load::<::aws_types::endpoint_config::EndpointUrl>()
+                    .map(|ty| ty.0.clone()),
+            )
             .build()
             .map_err(|err| {
-                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new("endpoint params could not be built", err)
+                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new(
+                    "endpoint params could not be built",
+                    err,
+                )
             })?;
         cfg.interceptor_state()
-            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params));
+            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(
+                params,
+            ));
         ::std::result::Result::Ok(())
     }
 }
@@ -315,18 +372,22 @@
 #[derive(::std::fmt::Debug)]
 pub enum GetAccessKeyInfoError {
     /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
-    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
+    #[deprecated(
+        note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
     variable wildcard pattern and check `.code()`:
      \
     &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
      \
-    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-GetAccessKeyInfoError) for what information is available for the error.")]
+    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-GetAccessKeyInfoError) for what information is available for the error."
+    )]
     Unhandled(crate::error::sealed_unhandled::Unhandled),
 }
 impl GetAccessKeyInfoError {
     /// Creates the `GetAccessKeyInfoError::Unhandled` variant from any error type.
     pub fn unhandled(
-        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
+        err: impl ::std::convert::Into<
+            ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
+        >,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
             source: err.into(),
@@ -362,7 +423,9 @@
     fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
         match self {
             Self::Unhandled(_inner) => {
-                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
+                if let ::std::option::Option::Some(code) =
+                    ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
+                {
                     write!(f, "unhandled error ({code})")
                 } else {
                     f.write_str("unhandled error")
diff --git a/src/operation/get_access_key_info/_get_access_key_info_input.rs b/src/operation/get_access_key_info/_get_access_key_info_input.rs
--- a/src/operation/get_access_key_info/_get_access_key_info_input.rs
+++ b/src/operation/get_access_key_info/_get_access_key_info_input.rs
@@ -49,7 +49,10 @@
     /// Consumes the builder and constructs a [`GetAccessKeyInfoInput`](crate::operation::get_access_key_info::GetAccessKeyInfoInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<crate::operation::get_access_key_info::GetAccessKeyInfoInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        crate::operation::get_access_key_info::GetAccessKeyInfoInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(crate::operation::get_access_key_info::GetAccessKeyInfoInput {
             access_key_id: self.access_key_id,
         })
diff --git a/src/operation/get_access_key_info/builders.rs b/src/operation/get_access_key_info/builders.rs
--- a/src/operation/get_access_key_info/builders.rs
+++ b/src/operation/get_access_key_info/builders.rs
@@ -102,12 +102,18 @@
     > {
         crate::client::customize::CustomizableOperation::new(self)
     }
-    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
+    pub(crate) fn config_override(
+        mut self,
+        config_override: impl ::std::convert::Into<crate::config::Builder>,
+    ) -> Self {
         self.set_config_override(::std::option::Option::Some(config_override.into()));
         self
     }

-    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
+    pub(crate) fn set_config_override(
+        &mut self,
+        config_override: ::std::option::Option<crate::config::Builder>,
+    ) -> &mut Self {
         self.config_override = config_override;
         self
     }
diff --git a/src/operation/get_caller_identity.rs b/src/operation/get_caller_identity.rs
--- a/src/operation/get_caller_identity.rs
+++ b/src/operation/get_caller_identity.rs
@@ -27,9 +27,13 @@
                     .expect("correct error type")
             })
         };
-        let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
-            .await
-            .map_err(map_err)?;
+        let context = Self::orchestrate_with_stop_point(
+            runtime_plugins,
+            input,
+            ::aws_smithy_runtime::client::orchestrator::StopPoint::None,
+        )
+        .await
+        .map_err(map_err)?;
         let output = context.finalize().map_err(map_err)?;
         ::std::result::Result::Ok(
             output
@@ -51,17 +55,23 @@
     > {
         let input = ::aws_smithy_runtime_api::client::interceptors::context::Input::erase(input);
         use ::tracing::Instrument;
-        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("STS", "GetCallerIdentity", input, runtime_plugins, stop_point)
-            // Create a parent span for the entire operation. Includes a random, internal-only,
-            // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
-            .instrument(::tracing::debug_span!(
-                "STS.GetCallerIdentity",
-                "rpc.service" = "STS",
-                "rpc.method" = "GetCallerIdentity",
-                "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
-                "rpc.system" = "aws-api",
-            ))
-            .await
+        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point(
+            "STS",
+            "GetCallerIdentity",
+            input,
+            runtime_plugins,
+            stop_point,
+        )
+        // Create a parent span for the entire operation. Includes a random, internal-only,
+        // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
+        .instrument(::tracing::debug_span!(
+            "STS.GetCallerIdentity",
+            "rpc.service" = "STS",
+            "rpc.method" = "GetCallerIdentity",
+            "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
+            "rpc.system" = "aws-api",
+        ))
+        .await
     }

     pub(crate) fn operation_runtime_plugins(
@@ -91,18 +101,25 @@
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
             GetCallerIdentityRequestSerializer,
         ));
-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            GetCallerIdentityResponseDeserializer,
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
+                GetCallerIdentityResponseDeserializer,
+            ),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
-            crate::config::auth::Params::builder()
-                .operation_name("GetCallerIdentity")
-                .build()
-                .expect("required fields set"),
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
+                crate::config::auth::Params::builder()
+                    .operation_name("GetCallerIdentity")
+                    .build()
+                    .expect("required fields set"),
+            ),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("GetCallerIdentity", "STS"));
+        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
+            "GetCallerIdentity",
+            "STS",
+        ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
         signing_options.double_uri_encode = true;
         signing_options.content_sha256_header = false;
@@ -122,16 +139,41 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetCallerIdentity")
-                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
-.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(GetCallerIdentityEndpointParamsInterceptor))
-                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::get_caller_identity::GetCallerIdentityError>::new())
-.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::get_caller_identity::GetCallerIdentityError>::new())
-.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::get_caller_identity::GetCallerIdentityError>::builder().transient_errors({
-                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
-                                            transient_errors.push("IDPCommunicationError");
-                                            ::std::borrow::Cow::Owned(transient_errors)
-                                            }).build());
+        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new(
+            "GetCallerIdentity",
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
+            ),
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                GetCallerIdentityEndpointParamsInterceptor,
+            ),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+                crate::operation::get_caller_identity::GetCallerIdentityError,
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+                crate::operation::get_caller_identity::GetCallerIdentityError,
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                crate::operation::get_caller_identity::GetCallerIdentityError,
+            >::builder()
+            .transient_errors({
+                let mut transient_errors: Vec<&'static str> =
+                    ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                transient_errors.push("IDPCommunicationError");
+                ::std::borrow::Cow::Owned(transient_errors)
+            })
+            .build(),
+        );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -154,7 +196,9 @@
         let parse_result = if !success && status != 200 || force_error {
             crate::protocol_serde::shape_get_caller_identity::de_get_caller_identity_http_error(status, headers, body)
         } else {
-            crate::protocol_serde::shape_get_caller_identity::de_get_caller_identity_http_response(status, headers, body)
+            crate::protocol_serde::shape_get_caller_identity::de_get_caller_identity_http_response(
+                status, headers, body,
+            )
         };
         crate::protocol_serde::type_erase_result(parse_result)
     }
@@ -162,12 +206,20 @@
 #[derive(Debug)]
 struct GetCallerIdentityRequestSerializer;
 impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for GetCallerIdentityRequestSerializer {
-    #[allow(unused_mut, clippy::let_and_return, clippy::needless_borrow, clippy::useless_conversion)]
+    #[allow(
+        unused_mut,
+        clippy::let_and_return,
+        clippy::needless_borrow,
+        clippy::useless_conversion
+    )]
     fn serialize_input(
         &self,
         input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
         _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
-    ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
+    ) -> ::std::result::Result<
+        ::aws_smithy_runtime_api::client::orchestrator::HttpRequest,
+        ::aws_smithy_runtime_api::box_error::BoxError,
+    > {
         let input = input
             .downcast::<crate::operation::get_caller_identity::GetCallerIdentityInput>()
             .expect("correct type");
@@ -189,14 +241,18 @@
             fn update_http_builder(
                 input: &crate::operation::get_caller_identity::GetCallerIdentityInput,
                 builder: ::http_1x::request::Builder,
-            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError>
+            {
                 let mut uri = ::std::string::String::new();
                 uri_base(input, &mut uri)?;
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(
+                builder,
+                ::http_1x::header::CONTENT_TYPE,
+                "application/x-www-form-urlencoded",
+            );
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
@@ -234,13 +290,21 @@
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
-            .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
+            .set_endpoint(
+                cfg.load::<::aws_types::endpoint_config::EndpointUrl>()
+                    .map(|ty| ty.0.clone()),
+            )
             .build()
             .map_err(|err| {
-                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new("endpoint params could not be built", err)
+                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new(
+                    "endpoint params could not be built",
+                    err,
+                )
             })?;
         cfg.interceptor_state()
-            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params));
+            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(
+                params,
+            ));
         ::std::result::Result::Ok(())
     }
 }
@@ -253,18 +317,22 @@
 #[derive(::std::fmt::Debug)]
 pub enum GetCallerIdentityError {
     /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
-    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
+    #[deprecated(
+        note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
     variable wildcard pattern and check `.code()`:
      \
     &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
      \
-    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-GetCallerIdentityError) for what information is available for the error.")]
+    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-GetCallerIdentityError) for what information is available for the error."
+    )]
     Unhandled(crate::error::sealed_unhandled::Unhandled),
 }
 impl GetCallerIdentityError {
     /// Creates the `GetCallerIdentityError::Unhandled` variant from any error type.
     pub fn unhandled(
-        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
+        err: impl ::std::convert::Into<
+            ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
+        >,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
             source: err.into(),
@@ -300,7 +368,9 @@
     fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
         match self {
             Self::Unhandled(_inner) => {
-                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
+                if let ::std::option::Option::Some(code) =
+                    ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
+                {
                     write!(f, "unhandled error ({code})")
                 } else {
                     f.write_str("unhandled error")
diff --git a/src/operation/get_caller_identity/_get_caller_identity_input.rs b/src/operation/get_caller_identity/_get_caller_identity_input.rs
--- a/src/operation/get_caller_identity/_get_caller_identity_input.rs
+++ b/src/operation/get_caller_identity/_get_caller_identity_input.rs
@@ -18,7 +18,10 @@
     /// Consumes the builder and constructs a [`GetCallerIdentityInput`](crate::operation::get_caller_identity::GetCallerIdentityInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<crate::operation::get_caller_identity::GetCallerIdentityInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        crate::operation::get_caller_identity::GetCallerIdentityInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(crate::operation::get_caller_identity::GetCallerIdentityInput {})
     }
 }
diff --git a/src/operation/get_caller_identity/builders.rs b/src/operation/get_caller_identity/builders.rs
--- a/src/operation/get_caller_identity/builders.rs
+++ b/src/operation/get_caller_identity/builders.rs
@@ -101,12 +101,18 @@
     > {
         crate::client::customize::CustomizableOperation::new(self)
     }
-    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
+    pub(crate) fn config_override(
+        mut self,
+        config_override: impl ::std::convert::Into<crate::config::Builder>,
+    ) -> Self {
         self.set_config_override(::std::option::Option::Some(config_override.into()));
         self
     }

-    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
+    pub(crate) fn set_config_override(
+        &mut self,
+        config_override: ::std::option::Option<crate::config::Builder>,
+    ) -> &mut Self {
         self.config_override = config_override;
         self
     }
diff --git a/src/operation/get_delegated_access_token.rs b/src/operation/get_delegated_access_token.rs
--- a/src/operation/get_delegated_access_token.rs
+++ b/src/operation/get_delegated_access_token.rs
@@ -27,9 +27,13 @@
                     .expect("correct error type")
             })
         };
-        let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
-            .await
-            .map_err(map_err)?;
+        let context = Self::orchestrate_with_stop_point(
+            runtime_plugins,
+            input,
+            ::aws_smithy_runtime::client::orchestrator::StopPoint::None,
+        )
+        .await
+        .map_err(map_err)?;
         let output = context.finalize().map_err(map_err)?;
         ::std::result::Result::Ok(
             output
@@ -51,17 +55,23 @@
     > {
         let input = ::aws_smithy_runtime_api::client::interceptors::context::Input::erase(input);
         use ::tracing::Instrument;
-        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("STS", "GetDelegatedAccessToken", input, runtime_plugins, stop_point)
-            // Create a parent span for the entire operation. Includes a random, internal-only,
-            // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
-            .instrument(::tracing::debug_span!(
-                "STS.GetDelegatedAccessToken",
-                "rpc.service" = "STS",
-                "rpc.method" = "GetDelegatedAccessToken",
-                "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
-                "rpc.system" = "aws-api",
-            ))
-            .await
+        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point(
+            "STS",
+            "GetDelegatedAccessToken",
+            input,
+            runtime_plugins,
+            stop_point,
+        )
+        // Create a parent span for the entire operation. Includes a random, internal-only,
+        // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
+        .instrument(::tracing::debug_span!(
+            "STS.GetDelegatedAccessToken",
+            "rpc.service" = "STS",
+            "rpc.method" = "GetDelegatedAccessToken",
+            "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
+            "rpc.system" = "aws-api",
+        ))
+        .await
     }

     pub(crate) fn operation_runtime_plugins(
@@ -91,16 +101,20 @@
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
             GetDelegatedAccessTokenRequestSerializer,
         ));
-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            GetDelegatedAccessTokenResponseDeserializer,
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
+                GetDelegatedAccessTokenResponseDeserializer,
+            ),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
-            crate::config::auth::Params::builder()
-                .operation_name("GetDelegatedAccessToken")
-                .build()
-                .expect("required fields set"),
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
+                crate::config::auth::Params::builder()
+                    .operation_name("GetDelegatedAccessToken")
+                    .build()
+                    .expect("required fields set"),
+            ),
+        );

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::SensitiveOutput);
         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
@@ -126,30 +140,41 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetDelegatedAccessToken")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new(
+            "GetDelegatedAccessToken",
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+            ),
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 GetDelegatedAccessTokenEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+            ),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError,
-            >::new())
-            .with_retry_classifier(
-                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                    crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError,
-                >::builder()
-                .transient_errors({
-                    let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
-                    transient_errors.push("IDPCommunicationError");
-                    ::std::borrow::Cow::Owned(transient_errors)
-                })
-                .build(),
-            );
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError,
+            >::builder()
+            .transient_errors({
+                let mut transient_errors: Vec<&'static str> =
+                    ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                transient_errors.push("IDPCommunicationError");
+                ::std::borrow::Cow::Owned(transient_errors)
+            })
+            .build(),
+        );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -170,9 +195,13 @@
         let mut force_error = false;
         ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));
         let parse_result = if !success && status != 200 || force_error {
-            crate::protocol_serde::shape_get_delegated_access_token::de_get_delegated_access_token_http_error(status, headers, body)
+            crate::protocol_serde::shape_get_delegated_access_token::de_get_delegated_access_token_http_error(
+                status, headers, body,
+            )
         } else {
-            crate::protocol_serde::shape_get_delegated_access_token::de_get_delegated_access_token_http_response(status, headers, body)
+            crate::protocol_serde::shape_get_delegated_access_token::de_get_delegated_access_token_http_response(
+                status, headers, body,
+            )
         };
         crate::protocol_serde::type_erase_result(parse_result)
     }
@@ -180,12 +209,20 @@
 #[derive(Debug)]
 struct GetDelegatedAccessTokenRequestSerializer;
 impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for GetDelegatedAccessTokenRequestSerializer {
-    #[allow(unused_mut, clippy::let_and_return, clippy::needless_borrow, clippy::useless_conversion)]
+    #[allow(
+        unused_mut,
+        clippy::let_and_return,
+        clippy::needless_borrow,
+        clippy::useless_conversion
+    )]
     fn serialize_input(
         &self,
         input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
         _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
-    ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
+    ) -> ::std::result::Result<
+        ::aws_smithy_runtime_api::client::orchestrator::HttpRequest,
+        ::aws_smithy_runtime_api::box_error::BoxError,
+    > {
         let input = input
             .downcast::<crate::operation::get_delegated_access_token::GetDelegatedAccessTokenInput>()
             .expect("correct type");
@@ -207,14 +244,18 @@
             fn update_http_builder(
                 input: &crate::operation::get_delegated_access_token::GetDelegatedAccessTokenInput,
                 builder: ::http_1x::request::Builder,
-            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError>
+            {
                 let mut uri = ::std::string::String::new();
                 uri_base(input, &mut uri)?;
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(
+                builder,
+                ::http_1x::header::CONTENT_TYPE,
+                "application/x-www-form-urlencoded",
+            );
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
@@ -222,7 +263,11 @@
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
-            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+            request_builder = _header_serialization_settings.set_default_header(
+                request_builder,
+                ::http_1x::header::CONTENT_LENGTH,
+                &content_length,
+            );
         }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
@@ -255,13 +300,21 @@
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
-            .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
+            .set_endpoint(
+                cfg.load::<::aws_types::endpoint_config::EndpointUrl>()
+                    .map(|ty| ty.0.clone()),
+            )
             .build()
             .map_err(|err| {
-                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new("endpoint params could not be built", err)
+                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new(
+                    "endpoint params could not be built",
+                    err,
+                )
             })?;
         cfg.interceptor_state()
-            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params));
+            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(
+                params,
+            ));
         ::std::result::Result::Ok(())
     }
 }
@@ -281,18 +334,22 @@
     /// <p>STS is not activated in the requested region for the account that is being asked to generate credentials. The account administrator must use the IAM console to activate STS in that region. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_enable-regions.html#sts-regions-activate-deactivate">Activating and Deactivating STS in an Amazon Web Services Region</a> in the <i>IAM User Guide</i>.</p>
     RegionDisabledException(crate::types::error::RegionDisabledException),
     /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
-    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
+    #[deprecated(
+        note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
     variable wildcard pattern and check `.code()`:
      \
     &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
      \
-    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-GetDelegatedAccessTokenError) for what information is available for the error.")]
+    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-GetDelegatedAccessTokenError) for what information is available for the error."
+    )]
     Unhandled(crate::error::sealed_unhandled::Unhandled),
 }
 impl GetDelegatedAccessTokenError {
     /// Creates the `GetDelegatedAccessTokenError::Unhandled` variant from any error type.
     pub fn unhandled(
-        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
+        err: impl ::std::convert::Into<
+            ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
+        >,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
             source: err.into(),
@@ -314,7 +371,9 @@
     pub fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
             Self::ExpiredTradeInTokenException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::PackedPolicyTooLargeException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::PackedPolicyTooLargeException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
             Self::RegionDisabledException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
             Self::Unhandled(e) => &e.meta,
         }
@@ -349,7 +408,9 @@
             Self::PackedPolicyTooLargeException(_inner) => _inner.fmt(f),
             Self::RegionDisabledException(_inner) => _inner.fmt(f),
             Self::Unhandled(_inner) => {
-                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
+                if let ::std::option::Option::Some(code) =
+                    ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
+                {
                     write!(f, "unhandled error ({code})")
                 } else {
                     f.write_str("unhandled error")
@@ -369,9 +430,15 @@
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for GetDelegatedAccessTokenError {
     fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::ExpiredTradeInTokenException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::PackedPolicyTooLargeException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::RegionDisabledException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::ExpiredTradeInTokenException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::PackedPolicyTooLargeException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::RegionDisabledException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
             Self::Unhandled(_inner) => &_inner.meta,
         }
     }
diff --git a/src/operation/get_delegated_access_token/_get_delegated_access_token_input.rs b/src/operation/get_delegated_access_token/_get_delegated_access_token_input.rs
--- a/src/operation/get_delegated_access_token/_get_delegated_access_token_input.rs
+++ b/src/operation/get_delegated_access_token/_get_delegated_access_token_input.rs
@@ -55,9 +55,11 @@
         crate::operation::get_delegated_access_token::GetDelegatedAccessTokenInput,
         ::aws_smithy_types::error::operation::BuildError,
     > {
-        ::std::result::Result::Ok(crate::operation::get_delegated_access_token::GetDelegatedAccessTokenInput {
-            trade_in_token: self.trade_in_token,
-        })
+        ::std::result::Result::Ok(
+            crate::operation::get_delegated_access_token::GetDelegatedAccessTokenInput {
+                trade_in_token: self.trade_in_token,
+            },
+        )
     }
 }
 impl ::std::fmt::Debug for GetDelegatedAccessTokenInputBuilder {
diff --git a/src/operation/get_delegated_access_token/builders.rs b/src/operation/get_delegated_access_token/builders.rs
--- a/src/operation/get_delegated_access_token/builders.rs
+++ b/src/operation/get_delegated_access_token/builders.rs
@@ -57,7 +57,9 @@
         }
     }
     /// Access the GetDelegatedAccessToken as a reference.
-    pub fn as_input(&self) -> &crate::operation::get_delegated_access_token::builders::GetDelegatedAccessTokenInputBuilder {
+    pub fn as_input(
+        &self,
+    ) -> &crate::operation::get_delegated_access_token::builders::GetDelegatedAccessTokenInputBuilder {
         &self.inner
     }
     /// Sends the request and returns the response.
@@ -81,12 +83,14 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = crate::operation::get_delegated_access_token::GetDelegatedAccessToken::operation_runtime_plugins(
-            self.handle.runtime_plugins.clone(),
-            &self.handle.conf,
-            self.config_override,
-        );
-        crate::operation::get_delegated_access_token::GetDelegatedAccessToken::orchestrate(&runtime_plugins, input).await
+        let runtime_plugins =
+            crate::operation::get_delegated_access_token::GetDelegatedAccessToken::operation_runtime_plugins(
+                self.handle.runtime_plugins.clone(),
+                &self.handle.conf,
+                self.config_override,
+            );
+        crate::operation::get_delegated_access_token::GetDelegatedAccessToken::orchestrate(&runtime_plugins, input)
+            .await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -99,12 +103,18 @@
     > {
         crate::client::customize::CustomizableOperation::new(self)
     }
-    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
+    pub(crate) fn config_override(
+        mut self,
+        config_override: impl ::std::convert::Into<crate::config::Builder>,
+    ) -> Self {
         self.set_config_override(::std::option::Option::Some(config_override.into()));
         self
     }

-    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
+    pub(crate) fn set_config_override(
+        &mut self,
+        config_override: ::std::option::Option<crate::config::Builder>,
+    ) -> &mut Self {
         self.config_override = config_override;
         self
     }
diff --git a/src/operation/get_federation_token.rs b/src/operation/get_federation_token.rs
--- a/src/operation/get_federation_token.rs
+++ b/src/operation/get_federation_token.rs
@@ -27,9 +27,13 @@
                     .expect("correct error type")
             })
         };
-        let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
-            .await
-            .map_err(map_err)?;
+        let context = Self::orchestrate_with_stop_point(
+            runtime_plugins,
+            input,
+            ::aws_smithy_runtime::client::orchestrator::StopPoint::None,
+        )
+        .await
+        .map_err(map_err)?;
         let output = context.finalize().map_err(map_err)?;
         ::std::result::Result::Ok(
             output
@@ -51,17 +55,23 @@
     > {
         let input = ::aws_smithy_runtime_api::client::interceptors::context::Input::erase(input);
         use ::tracing::Instrument;
-        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("STS", "GetFederationToken", input, runtime_plugins, stop_point)
-            // Create a parent span for the entire operation. Includes a random, internal-only,
-            // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
-            .instrument(::tracing::debug_span!(
-                "STS.GetFederationToken",
-                "rpc.service" = "STS",
-                "rpc.method" = "GetFederationToken",
-                "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
-                "rpc.system" = "aws-api",
-            ))
-            .await
+        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point(
+            "STS",
+            "GetFederationToken",
+            input,
+            runtime_plugins,
+            stop_point,
+        )
+        // Create a parent span for the entire operation. Includes a random, internal-only,
+        // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
+        .instrument(::tracing::debug_span!(
+            "STS.GetFederationToken",
+            "rpc.service" = "STS",
+            "rpc.method" = "GetFederationToken",
+            "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
+            "rpc.system" = "aws-api",
+        ))
+        .await
     }

     pub(crate) fn operation_runtime_plugins(
@@ -91,19 +101,26 @@
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
             GetFederationTokenRequestSerializer,
         ));
-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            GetFederationTokenResponseDeserializer,
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
+                GetFederationTokenResponseDeserializer,
+            ),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
-            crate::config::auth::Params::builder()
-                .operation_name("GetFederationToken")
-                .build()
-                .expect("required fields set"),
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
+                crate::config::auth::Params::builder()
+                    .operation_name("GetFederationToken")
+                    .build()
+                    .expect("required fields set"),
+            ),
+        );

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::SensitiveOutput);
-        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("GetFederationToken", "STS"));
+        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
+            "GetFederationToken",
+            "STS",
+        ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
         signing_options.double_uri_encode = true;
         signing_options.content_sha256_header = false;
@@ -123,17 +140,46 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetFederationToken")
-                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(GetFederationTokenTelemetryInputCaptureInterceptor))
-.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
-.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(GetFederationTokenEndpointParamsInterceptor))
-                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::get_federation_token::GetFederationTokenError>::new())
-.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::get_federation_token::GetFederationTokenError>::new())
-.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::get_federation_token::GetFederationTokenError>::builder().transient_errors({
-                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
-                                            transient_errors.push("IDPCommunicationError");
-                                            ::std::borrow::Cow::Owned(transient_errors)
-                                            }).build());
+        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new(
+            "GetFederationToken",
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                GetFederationTokenTelemetryInputCaptureInterceptor,
+            ),
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
+            ),
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                GetFederationTokenEndpointParamsInterceptor,
+            ),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+                crate::operation::get_federation_token::GetFederationTokenError,
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+                crate::operation::get_federation_token::GetFederationTokenError,
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                crate::operation::get_federation_token::GetFederationTokenError,
+            >::builder()
+            .transient_errors({
+                let mut transient_errors: Vec<&'static str> =
+                    ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                transient_errors.push("IDPCommunicationError");
+                ::std::borrow::Cow::Owned(transient_errors)
+            })
+            .build(),
+        );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -204,7 +250,9 @@
         let parse_result = if !success && status != 200 || force_error {
             crate::protocol_serde::shape_get_federation_token::de_get_federation_token_http_error(status, headers, body)
         } else {
-            crate::protocol_serde::shape_get_federation_token::de_get_federation_token_http_response(status, headers, body)
+            crate::protocol_serde::shape_get_federation_token::de_get_federation_token_http_response(
+                status, headers, body,
+            )
         };
         crate::protocol_serde::type_erase_result(parse_result)
     }
@@ -212,12 +260,20 @@
 #[derive(Debug)]
 struct GetFederationTokenRequestSerializer;
 impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for GetFederationTokenRequestSerializer {
-    #[allow(unused_mut, clippy::let_and_return, clippy::needless_borrow, clippy::useless_conversion)]
+    #[allow(
+        unused_mut,
+        clippy::let_and_return,
+        clippy::needless_borrow,
+        clippy::useless_conversion
+    )]
     fn serialize_input(
         &self,
         input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
         _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
-    ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
+    ) -> ::std::result::Result<
+        ::aws_smithy_runtime_api::client::orchestrator::HttpRequest,
+        ::aws_smithy_runtime_api::box_error::BoxError,
+    > {
         let input = input
             .downcast::<crate::operation::get_federation_token::GetFederationTokenInput>()
             .expect("correct type");
@@ -239,22 +295,32 @@
             fn update_http_builder(
                 input: &crate::operation::get_federation_token::GetFederationTokenInput,
                 builder: ::http_1x::request::Builder,
-            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError>
+            {
                 let mut uri = ::std::string::String::new();
                 uri_base(input, &mut uri)?;
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(
+                builder,
+                ::http_1x::header::CONTENT_TYPE,
+                "application/x-www-form-urlencoded",
+            );
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_get_federation_token_input::ser_get_federation_token_input_input_input(&input)?,
+            crate::protocol_serde::shape_get_federation_token_input::ser_get_federation_token_input_input_input(
+                &input,
+            )?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
-            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+            request_builder = _header_serialization_settings.set_default_header(
+                request_builder,
+                ::http_1x::header::CONTENT_LENGTH,
+                &content_length,
+            );
         }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
@@ -287,13 +353,21 @@
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
-            .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
+            .set_endpoint(
+                cfg.load::<::aws_types::endpoint_config::EndpointUrl>()
+                    .map(|ty| ty.0.clone()),
+            )
             .build()
             .map_err(|err| {
-                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new("endpoint params could not be built", err)
+                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new(
+                    "endpoint params could not be built",
+                    err,
+                )
             })?;
         cfg.interceptor_state()
-            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params));
+            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(
+                params,
+            ));
         ::std::result::Result::Ok(())
     }
 }
@@ -313,18 +387,22 @@
     /// <p>STS is not activated in the requested region for the account that is being asked to generate credentials. The account administrator must use the IAM console to activate STS in that region. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_enable-regions.html#sts-regions-activate-deactivate">Activating and Deactivating STS in an Amazon Web Services Region</a> in the <i>IAM User Guide</i>.</p>
     RegionDisabledException(crate::types::error::RegionDisabledException),
     /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
-    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
+    #[deprecated(
+        note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
     variable wildcard pattern and check `.code()`:
      \
     &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
      \
-    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-GetFederationTokenError) for what information is available for the error.")]
+    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-GetFederationTokenError) for what information is available for the error."
+    )]
     Unhandled(crate::error::sealed_unhandled::Unhandled),
 }
 impl GetFederationTokenError {
     /// Creates the `GetFederationTokenError::Unhandled` variant from any error type.
     pub fn unhandled(
-        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
+        err: impl ::std::convert::Into<
+            ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
+        >,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
             source: err.into(),
@@ -345,8 +423,12 @@
     ///
     pub fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::MalformedPolicyDocumentException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::PackedPolicyTooLargeException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::MalformedPolicyDocumentException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
+            Self::PackedPolicyTooLargeException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
             Self::RegionDisabledException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
             Self::Unhandled(e) => &e.meta,
         }
@@ -381,7 +463,9 @@
             Self::PackedPolicyTooLargeException(_inner) => _inner.fmt(f),
             Self::RegionDisabledException(_inner) => _inner.fmt(f),
             Self::Unhandled(_inner) => {
-                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
+                if let ::std::option::Option::Some(code) =
+                    ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
+                {
                     write!(f, "unhandled error ({code})")
                 } else {
                     f.write_str("unhandled error")
@@ -401,9 +485,15 @@
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for GetFederationTokenError {
     fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::MalformedPolicyDocumentException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::PackedPolicyTooLargeException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::RegionDisabledException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::MalformedPolicyDocumentException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::PackedPolicyTooLargeException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::RegionDisabledException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
             Self::Unhandled(_inner) => &_inner.meta,
         }
     }
diff --git a/src/operation/get_federation_token/_get_federation_token_input.rs b/src/operation/get_federation_token/_get_federation_token_input.rs
--- a/src/operation/get_federation_token/_get_federation_token_input.rs
+++ b/src/operation/get_federation_token/_get_federation_token_input.rs
@@ -173,7 +173,10 @@
     /// <p>The resulting credentials can be used to access a resource that has a resource-based policy. If that policy specifically references the federated user session in the <code>Principal</code> element of the policy, the session has the permissions allowed by the policy. These permissions are granted in addition to the permissions that are granted by the session policies.</p><note>
     /// <p>An Amazon Web Services conversion compresses the passed inline session policy, managed policy ARNs, and session tags into a packed binary format that has a separate limit. Your request can fail for this limit even if your plaintext meets the other requirements. The <code>PackedPolicySize</code> response element indicates by percentage how close the policies and tags for your request are to the upper size limit.</p>
     /// </note>
-    pub fn set_policy_arns(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::PolicyDescriptorType>>) -> Self {
+    pub fn set_policy_arns(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<crate::types::PolicyDescriptorType>>,
+    ) -> Self {
         self.policy_arns = input;
         self
     }
@@ -239,8 +242,10 @@
     /// Consumes the builder and constructs a [`GetFederationTokenInput`](crate::operation::get_federation_token::GetFederationTokenInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<crate::operation::get_federation_token::GetFederationTokenInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        crate::operation::get_federation_token::GetFederationTokenInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(crate::operation::get_federation_token::GetFederationTokenInput {
             name: self.name,
             policy: self.policy,
diff --git a/src/operation/get_federation_token/builders.rs b/src/operation/get_federation_token/builders.rs
--- a/src/operation/get_federation_token/builders.rs
+++ b/src/operation/get_federation_token/builders.rs
@@ -123,12 +123,18 @@
     > {
         crate::client::customize::CustomizableOperation::new(self)
     }
-    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
+    pub(crate) fn config_override(
+        mut self,
+        config_override: impl ::std::convert::Into<crate::config::Builder>,
+    ) -> Self {
         self.set_config_override(::std::option::Option::Some(config_override.into()));
         self
     }

-    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
+    pub(crate) fn set_config_override(
+        &mut self,
+        config_override: ::std::option::Option<crate::config::Builder>,
+    ) -> &mut Self {
         self.config_override = config_override;
         self
     }
@@ -207,7 +213,10 @@
     /// <p>The resulting credentials can be used to access a resource that has a resource-based policy. If that policy specifically references the federated user session in the <code>Principal</code> element of the policy, the session has the permissions allowed by the policy. These permissions are granted in addition to the permissions that are granted by the session policies.</p><note>
     /// <p>An Amazon Web Services conversion compresses the passed inline session policy, managed policy ARNs, and session tags into a packed binary format that has a separate limit. Your request can fail for this limit even if your plaintext meets the other requirements. The <code>PackedPolicySize</code> response element indicates by percentage how close the policies and tags for your request are to the upper size limit.</p>
     /// </note>
-    pub fn set_policy_arns(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::PolicyDescriptorType>>) -> Self {
+    pub fn set_policy_arns(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<crate::types::PolicyDescriptorType>>,
+    ) -> Self {
         self.inner = self.inner.set_policy_arns(input);
         self
     }
diff --git a/src/operation/get_session_token.rs b/src/operation/get_session_token.rs
--- a/src/operation/get_session_token.rs
+++ b/src/operation/get_session_token.rs
@@ -27,9 +27,13 @@
                     .expect("correct error type")
             })
         };
-        let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
-            .await
-            .map_err(map_err)?;
+        let context = Self::orchestrate_with_stop_point(
+            runtime_plugins,
+            input,
+            ::aws_smithy_runtime::client::orchestrator::StopPoint::None,
+        )
+        .await
+        .map_err(map_err)?;
         let output = context.finalize().map_err(map_err)?;
         ::std::result::Result::Ok(
             output
@@ -51,17 +55,23 @@
     > {
         let input = ::aws_smithy_runtime_api::client::interceptors::context::Input::erase(input);
         use ::tracing::Instrument;
-        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("STS", "GetSessionToken", input, runtime_plugins, stop_point)
-            // Create a parent span for the entire operation. Includes a random, internal-only,
-            // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
-            .instrument(::tracing::debug_span!(
-                "STS.GetSessionToken",
-                "rpc.service" = "STS",
-                "rpc.method" = "GetSessionToken",
-                "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
-                "rpc.system" = "aws-api",
-            ))
-            .await
+        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point(
+            "STS",
+            "GetSessionToken",
+            input,
+            runtime_plugins,
+            stop_point,
+        )
+        // Create a parent span for the entire operation. Includes a random, internal-only,
+        // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
+        .instrument(::tracing::debug_span!(
+            "STS.GetSessionToken",
+            "rpc.service" = "STS",
+            "rpc.method" = "GetSessionToken",
+            "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
+            "rpc.system" = "aws-api",
+        ))
+        .await
     }

     pub(crate) fn operation_runtime_plugins(
@@ -91,19 +101,26 @@
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
             GetSessionTokenRequestSerializer,
         ));
-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            GetSessionTokenResponseDeserializer,
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
+                GetSessionTokenResponseDeserializer,
+            ),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
-            crate::config::auth::Params::builder()
-                .operation_name("GetSessionToken")
-                .build()
-                .expect("required fields set"),
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
+                crate::config::auth::Params::builder()
+                    .operation_name("GetSessionToken")
+                    .build()
+                    .expect("required fields set"),
+            ),
+        );

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::SensitiveOutput);
-        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("GetSessionToken", "STS"));
+        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
+            "GetSessionToken",
+            "STS",
+        ));
         let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
         signing_options.double_uri_encode = true;
         signing_options.content_sha256_header = false;
@@ -123,31 +140,46 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetSessionToken")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new(
+            "GetSessionToken",
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 GetSessionTokenTelemetryInputCaptureInterceptor,
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+            ),
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
-            ))
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+            ),
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 GetSessionTokenEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+            ),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+                crate::operation::get_session_token::GetSessionTokenError,
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::get_session_token::GetSessionTokenError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
                 crate::operation::get_session_token::GetSessionTokenError,
-            >::new())
-            .with_retry_classifier(
-                ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::get_session_token::GetSessionTokenError>::builder()
-                    .transient_errors({
-                        let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
-                        transient_errors.push("IDPCommunicationError");
-                        ::std::borrow::Cow::Owned(transient_errors)
-                    })
-                    .build(),
-            );
+            >::builder()
+            .transient_errors({
+                let mut transient_errors: Vec<&'static str> =
+                    ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                transient_errors.push("IDPCommunicationError");
+                ::std::borrow::Cow::Owned(transient_errors)
+            })
+            .build(),
+        );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -226,12 +258,20 @@
 #[derive(Debug)]
 struct GetSessionTokenRequestSerializer;
 impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for GetSessionTokenRequestSerializer {
-    #[allow(unused_mut, clippy::let_and_return, clippy::needless_borrow, clippy::useless_conversion)]
+    #[allow(
+        unused_mut,
+        clippy::let_and_return,
+        clippy::needless_borrow,
+        clippy::useless_conversion
+    )]
     fn serialize_input(
         &self,
         input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
         _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
-    ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
+    ) -> ::std::result::Result<
+        ::aws_smithy_runtime_api::client::orchestrator::HttpRequest,
+        ::aws_smithy_runtime_api::box_error::BoxError,
+    > {
         let input = input
             .downcast::<crate::operation::get_session_token::GetSessionTokenInput>()
             .expect("correct type");
@@ -253,14 +293,18 @@
             fn update_http_builder(
                 input: &crate::operation::get_session_token::GetSessionTokenInput,
                 builder: ::http_1x::request::Builder,
-            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError>
+            {
                 let mut uri = ::std::string::String::new();
                 uri_base(input, &mut uri)?;
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(
+                builder,
+                ::http_1x::header::CONTENT_TYPE,
+                "application/x-www-form-urlencoded",
+            );
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
@@ -268,7 +312,11 @@
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
-            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+            request_builder = _header_serialization_settings.set_default_header(
+                request_builder,
+                ::http_1x::header::CONTENT_LENGTH,
+                &content_length,
+            );
         }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
@@ -301,13 +349,21 @@
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
-            .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
+            .set_endpoint(
+                cfg.load::<::aws_types::endpoint_config::EndpointUrl>()
+                    .map(|ty| ty.0.clone()),
+            )
             .build()
             .map_err(|err| {
-                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new("endpoint params could not be built", err)
+                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new(
+                    "endpoint params could not be built",
+                    err,
+                )
             })?;
         cfg.interceptor_state()
-            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params));
+            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(
+                params,
+            ));
         ::std::result::Result::Ok(())
     }
 }
@@ -322,18 +378,22 @@
     /// <p>STS is not activated in the requested region for the account that is being asked to generate credentials. The account administrator must use the IAM console to activate STS in that region. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_enable-regions.html#sts-regions-activate-deactivate">Activating and Deactivating STS in an Amazon Web Services Region</a> in the <i>IAM User Guide</i>.</p>
     RegionDisabledException(crate::types::error::RegionDisabledException),
     /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
-    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
+    #[deprecated(
+        note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
     variable wildcard pattern and check `.code()`:
      \
     &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
      \
-    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-GetSessionTokenError) for what information is available for the error.")]
+    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-GetSessionTokenError) for what information is available for the error."
+    )]
     Unhandled(crate::error::sealed_unhandled::Unhandled),
 }
 impl GetSessionTokenError {
     /// Creates the `GetSessionTokenError::Unhandled` variant from any error type.
     pub fn unhandled(
-        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
+        err: impl ::std::convert::Into<
+            ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
+        >,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
             source: err.into(),
@@ -376,7 +436,9 @@
         match self {
             Self::RegionDisabledException(_inner) => _inner.fmt(f),
             Self::Unhandled(_inner) => {
-                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
+                if let ::std::option::Option::Some(code) =
+                    ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
+                {
                     write!(f, "unhandled error ({code})")
                 } else {
                     f.write_str("unhandled error")
@@ -396,7 +458,9 @@
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for GetSessionTokenError {
     fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::RegionDisabledException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::RegionDisabledException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
             Self::Unhandled(_inner) => &_inner.meta,
         }
     }
diff --git a/src/operation/get_session_token/_get_session_token_input.rs b/src/operation/get_session_token/_get_session_token_input.rs
--- a/src/operation/get_session_token/_get_session_token_input.rs
+++ b/src/operation/get_session_token/_get_session_token_input.rs
@@ -95,7 +95,10 @@
     /// Consumes the builder and constructs a [`GetSessionTokenInput`](crate::operation::get_session_token::GetSessionTokenInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<crate::operation::get_session_token::GetSessionTokenInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        crate::operation::get_session_token::GetSessionTokenInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(crate::operation::get_session_token::GetSessionTokenInput {
             duration_seconds: self.duration_seconds,
             serial_number: self.serial_number,
diff --git a/src/operation/get_session_token/builders.rs b/src/operation/get_session_token/builders.rs
--- a/src/operation/get_session_token/builders.rs
+++ b/src/operation/get_session_token/builders.rs
@@ -116,12 +116,18 @@
     > {
         crate::client::customize::CustomizableOperation::new(self)
     }
-    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
+    pub(crate) fn config_override(
+        mut self,
+        config_override: impl ::std::convert::Into<crate::config::Builder>,
+    ) -> Self {
         self.set_config_override(::std::option::Option::Some(config_override.into()));
         self
     }

-    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
+    pub(crate) fn set_config_override(
+        &mut self,
+        config_override: ::std::option::Option<crate::config::Builder>,
+    ) -> &mut Self {
         self.config_override = config_override;
         self
     }
diff --git a/src/operation/get_web_identity_token.rs b/src/operation/get_web_identity_token.rs
--- a/src/operation/get_web_identity_token.rs
+++ b/src/operation/get_web_identity_token.rs
@@ -27,9 +27,13 @@
                     .expect("correct error type")
             })
         };
-        let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
-            .await
-            .map_err(map_err)?;
+        let context = Self::orchestrate_with_stop_point(
+            runtime_plugins,
+            input,
+            ::aws_smithy_runtime::client::orchestrator::StopPoint::None,
+        )
+        .await
+        .map_err(map_err)?;
         let output = context.finalize().map_err(map_err)?;
         ::std::result::Result::Ok(
             output
@@ -51,17 +55,23 @@
     > {
         let input = ::aws_smithy_runtime_api::client::interceptors::context::Input::erase(input);
         use ::tracing::Instrument;
-        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("STS", "GetWebIdentityToken", input, runtime_plugins, stop_point)
-            // Create a parent span for the entire operation. Includes a random, internal-only,
-            // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
-            .instrument(::tracing::debug_span!(
-                "STS.GetWebIdentityToken",
-                "rpc.service" = "STS",
-                "rpc.method" = "GetWebIdentityToken",
-                "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
-                "rpc.system" = "aws-api",
-            ))
-            .await
+        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point(
+            "STS",
+            "GetWebIdentityToken",
+            input,
+            runtime_plugins,
+            stop_point,
+        )
+        // Create a parent span for the entire operation. Includes a random, internal-only,
+        // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
+        .instrument(::tracing::debug_span!(
+            "STS.GetWebIdentityToken",
+            "rpc.service" = "STS",
+            "rpc.method" = "GetWebIdentityToken",
+            "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
+            "rpc.system" = "aws-api",
+        ))
+        .await
     }

     pub(crate) fn operation_runtime_plugins(
@@ -91,16 +101,20 @@
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
             GetWebIdentityTokenRequestSerializer,
         ));
-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            GetWebIdentityTokenResponseDeserializer,
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
+                GetWebIdentityTokenResponseDeserializer,
+            ),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
-            crate::config::auth::Params::builder()
-                .operation_name("GetWebIdentityToken")
-                .build()
-                .expect("required fields set"),
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
+                crate::config::auth::Params::builder()
+                    .operation_name("GetWebIdentityToken")
+                    .build()
+                    .expect("required fields set"),
+            ),
+        );

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::SensitiveOutput);
         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
@@ -126,17 +140,46 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetWebIdentityToken")
-                            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(GetWebIdentityTokenTelemetryInputCaptureInterceptor))
-.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default()))
-.with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(GetWebIdentityTokenEndpointParamsInterceptor))
-                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::get_web_identity_token::GetWebIdentityTokenError>::new())
-.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::get_web_identity_token::GetWebIdentityTokenError>::new())
-.with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<crate::operation::get_web_identity_token::GetWebIdentityTokenError>::builder().transient_errors({
-                                            let mut transient_errors: Vec<&'static str> = ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
-                                            transient_errors.push("IDPCommunicationError");
-                                            ::std::borrow::Cow::Owned(transient_errors)
-                                            }).build());
+        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new(
+            "GetWebIdentityToken",
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                GetWebIdentityTokenTelemetryInputCaptureInterceptor,
+            ),
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
+            ),
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                GetWebIdentityTokenEndpointParamsInterceptor,
+            ),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+                crate::operation::get_web_identity_token::GetWebIdentityTokenError,
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+                crate::operation::get_web_identity_token::GetWebIdentityTokenError,
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+                crate::operation::get_web_identity_token::GetWebIdentityTokenError,
+            >::builder()
+            .transient_errors({
+                let mut transient_errors: Vec<&'static str> =
+                    ::aws_runtime::retries::classifiers::TRANSIENT_ERRORS.into();
+                transient_errors.push("IDPCommunicationError");
+                ::std::borrow::Cow::Owned(transient_errors)
+            })
+            .build(),
+        );

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -200,9 +243,13 @@
         let mut force_error = false;
         ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));
         let parse_result = if !success && status != 200 || force_error {
-            crate::protocol_serde::shape_get_web_identity_token::de_get_web_identity_token_http_error(status, headers, body)
+            crate::protocol_serde::shape_get_web_identity_token::de_get_web_identity_token_http_error(
+                status, headers, body,
+            )
         } else {
-            crate::protocol_serde::shape_get_web_identity_token::de_get_web_identity_token_http_response(status, headers, body)
+            crate::protocol_serde::shape_get_web_identity_token::de_get_web_identity_token_http_response(
+                status, headers, body,
+            )
         };
         crate::protocol_serde::type_erase_result(parse_result)
     }
@@ -210,12 +257,20 @@
 #[derive(Debug)]
 struct GetWebIdentityTokenRequestSerializer;
 impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for GetWebIdentityTokenRequestSerializer {
-    #[allow(unused_mut, clippy::let_and_return, clippy::needless_borrow, clippy::useless_conversion)]
+    #[allow(
+        unused_mut,
+        clippy::let_and_return,
+        clippy::needless_borrow,
+        clippy::useless_conversion
+    )]
     fn serialize_input(
         &self,
         input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
         _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
-    ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
+    ) -> ::std::result::Result<
+        ::aws_smithy_runtime_api::client::orchestrator::HttpRequest,
+        ::aws_smithy_runtime_api::box_error::BoxError,
+    > {
         let input = input
             .downcast::<crate::operation::get_web_identity_token::GetWebIdentityTokenInput>()
             .expect("correct type");
@@ -237,22 +292,32 @@
             fn update_http_builder(
                 input: &crate::operation::get_web_identity_token::GetWebIdentityTokenInput,
                 builder: ::http_1x::request::Builder,
-            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError>
+            {
                 let mut uri = ::std::string::String::new();
                 uri_base(input, &mut uri)?;
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
-            builder =
-                _header_serialization_settings.set_default_header(builder, ::http_1x::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
+            builder = _header_serialization_settings.set_default_header(
+                builder,
+                ::http_1x::header::CONTENT_TYPE,
+                "application/x-www-form-urlencoded",
+            );
             builder
         };
         let body = ::aws_smithy_types::body::SdkBody::from(
-            crate::protocol_serde::shape_get_web_identity_token_input::ser_get_web_identity_token_input_input_input(&input)?,
+            crate::protocol_serde::shape_get_web_identity_token_input::ser_get_web_identity_token_input_input_input(
+                &input,
+            )?,
         );
         if let Some(content_length) = body.content_length() {
             let content_length = content_length.to_string();
-            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http_1x::header::CONTENT_LENGTH, &content_length);
+            request_builder = _header_serialization_settings.set_default_header(
+                request_builder,
+                ::http_1x::header::CONTENT_LENGTH,
+                &content_length,
+            );
         }
         ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
     }
@@ -285,13 +350,21 @@
             .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
             .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
             .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
-            .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
+            .set_endpoint(
+                cfg.load::<::aws_types::endpoint_config::EndpointUrl>()
+                    .map(|ty| ty.0.clone()),
+            )
             .build()
             .map_err(|err| {
-                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new("endpoint params could not be built", err)
+                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new(
+                    "endpoint params could not be built",
+                    err,
+                )
             })?;
         cfg.interceptor_state()
-            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params));
+            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(
+                params,
+            ));
         ::std::result::Result::Ok(())
     }
 }
@@ -310,18 +383,22 @@
     /// <p>The requested token duration would extend the session beyond its original expiration time. You cannot use this operation to extend the lifetime of a session beyond what was granted when the session was originally created.</p>
     SessionDurationEscalationException(crate::types::error::SessionDurationEscalationException),
     /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
-    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
+    #[deprecated(
+        note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
     variable wildcard pattern and check `.code()`:
      \
     &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
      \
-    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-GetWebIdentityTokenError) for what information is available for the error.")]
+    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-GetWebIdentityTokenError) for what information is available for the error."
+    )]
     Unhandled(crate::error::sealed_unhandled::Unhandled),
 }
 impl GetWebIdentityTokenError {
     /// Creates the `GetWebIdentityTokenError::Unhandled` variant from any error type.
     pub fn unhandled(
-        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
+        err: impl ::std::convert::Into<
+            ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
+        >,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
             source: err.into(),
@@ -342,9 +419,15 @@
     ///
     pub fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::JwtPayloadSizeExceededException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::OutboundWebIdentityFederationDisabledException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::SessionDurationEscalationException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::JwtPayloadSizeExceededException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
+            Self::OutboundWebIdentityFederationDisabledException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
+            Self::SessionDurationEscalationException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
             Self::Unhandled(e) => &e.meta,
         }
     }
@@ -378,7 +461,9 @@
             Self::OutboundWebIdentityFederationDisabledException(_inner) => _inner.fmt(f),
             Self::SessionDurationEscalationException(_inner) => _inner.fmt(f),
             Self::Unhandled(_inner) => {
-                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
+                if let ::std::option::Option::Some(code) =
+                    ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
+                {
                     write!(f, "unhandled error ({code})")
                 } else {
                     f.write_str("unhandled error")
@@ -398,9 +483,15 @@
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for GetWebIdentityTokenError {
     fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::JwtPayloadSizeExceededException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::OutboundWebIdentityFederationDisabledException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::SessionDurationEscalationException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::JwtPayloadSizeExceededException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::OutboundWebIdentityFederationDisabledException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::SessionDurationEscalationException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
             Self::Unhandled(_inner) => &_inner.meta,
         }
     }
diff --git a/src/operation/get_web_identity_token/_get_web_identity_token_input.rs b/src/operation/get_web_identity_token/_get_web_identity_token_input.rs
--- a/src/operation/get_web_identity_token/_get_web_identity_token_input.rs
+++ b/src/operation/get_web_identity_token/_get_web_identity_token_input.rs
@@ -123,8 +123,10 @@
     /// Consumes the builder and constructs a [`GetWebIdentityTokenInput`](crate::operation::get_web_identity_token::GetWebIdentityTokenInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<crate::operation::get_web_identity_token::GetWebIdentityTokenInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        crate::operation::get_web_identity_token::GetWebIdentityTokenInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(crate::operation::get_web_identity_token::GetWebIdentityTokenInput {
             audience: self.audience,
             duration_seconds: self.duration_seconds,
diff --git a/src/operation/get_web_identity_token/builders.rs b/src/operation/get_web_identity_token/builders.rs
--- a/src/operation/get_web_identity_token/builders.rs
+++ b/src/operation/get_web_identity_token/builders.rs
@@ -99,12 +99,18 @@
     > {
         crate::client::customize::CustomizableOperation::new(self)
     }
-    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
+    pub(crate) fn config_override(
+        mut self,
+        config_override: impl ::std::convert::Into<crate::config::Builder>,
+    ) -> Self {
         self.set_config_override(::std::option::Option::Some(config_override.into()));
         self
     }

-    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
+    pub(crate) fn set_config_override(
+        &mut self,
+        config_override: ::std::option::Option<crate::config::Builder>,
+    ) -> &mut Self {
         self.config_override = config_override;
         self
     }
diff --git a/src/protocol_serde.rs b/src/protocol_serde.rs
--- a/src/protocol_serde.rs
+++ b/src/protocol_serde.rs
@@ -1,9 +1,24 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_assume_root",
+    feature = "op_decode_authorization_message",
+    feature = "op_get_access_key_info",
+    feature = "op_get_caller_identity",
+    feature = "op_get_delegated_access_token",
+    feature = "op_get_federation_token",
+    feature = "op_get_session_token",
+    feature = "op_get_web_identity_token"
+))]
 pub(crate) fn type_erase_result<O, E>(
     result: ::std::result::Result<O, E>,
 ) -> ::std::result::Result<
     ::aws_smithy_runtime_api::client::interceptors::context::Output,
-    ::aws_smithy_runtime_api::client::orchestrator::OrchestratorError<::aws_smithy_runtime_api::client::interceptors::context::Error>,
+    ::aws_smithy_runtime_api::client::orchestrator::OrchestratorError<
+        ::aws_smithy_runtime_api::client::interceptors::context::Error,
+    >,
 >
 where
     O: ::std::fmt::Debug + ::std::marker::Send + ::std::marker::Sync + 'static,
@@ -15,6 +30,19 @@
         .map_err(::std::convert::Into::into)
 }

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_assume_root",
+    feature = "op_decode_authorization_message",
+    feature = "op_get_access_key_info",
+    feature = "op_get_caller_identity",
+    feature = "op_get_delegated_access_token",
+    feature = "op_get_federation_token",
+    feature = "op_get_session_token",
+    feature = "op_get_web_identity_token"
+))]
 pub fn parse_http_error_metadata(
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
@@ -23,82 +51,168 @@
     crate::rest_xml_wrapped_errors::parse_error_metadata(response_body)
 }

+#[cfg(feature = "op_assume_role")]
 pub(crate) mod shape_assume_role;

+#[cfg(feature = "op_assume_role")]
 pub(crate) mod shape_assume_role_input;

+#[cfg(feature = "op_assume_role_with_saml")]
 pub(crate) mod shape_assume_role_with_saml;

+#[cfg(feature = "op_assume_role_with_saml")]
 pub(crate) mod shape_assume_role_with_saml_input;

+#[cfg(feature = "op_assume_role_with_web_identity")]
 pub(crate) mod shape_assume_role_with_web_identity;

+#[cfg(feature = "op_assume_role_with_web_identity")]
 pub(crate) mod shape_assume_role_with_web_identity_input;

+#[cfg(feature = "op_assume_root")]
 pub(crate) mod shape_assume_root;

+#[cfg(feature = "op_assume_root")]
 pub(crate) mod shape_assume_root_input;

+#[cfg(feature = "op_decode_authorization_message")]
 pub(crate) mod shape_decode_authorization_message;

+#[cfg(feature = "op_decode_authorization_message")]
 pub(crate) mod shape_decode_authorization_message_input;

+#[cfg(feature = "op_get_access_key_info")]
 pub(crate) mod shape_get_access_key_info;

+#[cfg(feature = "op_get_access_key_info")]
 pub(crate) mod shape_get_access_key_info_input;

+#[cfg(feature = "op_get_caller_identity")]
 pub(crate) mod shape_get_caller_identity;

+#[cfg(feature = "op_get_caller_identity")]
 pub(crate) mod shape_get_caller_identity_input;

+#[cfg(feature = "op_get_delegated_access_token")]
 pub(crate) mod shape_get_delegated_access_token;

+#[cfg(feature = "op_get_delegated_access_token")]
 pub(crate) mod shape_get_delegated_access_token_input;

+#[cfg(feature = "op_get_federation_token")]
 pub(crate) mod shape_get_federation_token;

+#[cfg(feature = "op_get_federation_token")]
 pub(crate) mod shape_get_federation_token_input;

+#[cfg(feature = "op_get_session_token")]
 pub(crate) mod shape_get_session_token;

+#[cfg(feature = "op_get_session_token")]
 pub(crate) mod shape_get_session_token_input;

+#[cfg(feature = "op_get_web_identity_token")]
 pub(crate) mod shape_get_web_identity_token;

+#[cfg(feature = "op_get_web_identity_token")]
 pub(crate) mod shape_get_web_identity_token_input;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_assume_root"
+))]
 pub(crate) mod shape_expired_token_exception;

+#[cfg(feature = "op_get_delegated_access_token")]
 pub(crate) mod shape_expired_trade_in_token_exception;

+#[cfg(feature = "op_assume_role_with_web_identity")]
 pub(crate) mod shape_idp_communication_error_exception;

+#[cfg(any(feature = "op_assume_role_with_saml", feature = "op_assume_role_with_web_identity"))]
 pub(crate) mod shape_idp_rejected_claim_exception;

+#[cfg(feature = "op_decode_authorization_message")]
 pub(crate) mod shape_invalid_authorization_message_exception;

+#[cfg(any(feature = "op_assume_role_with_saml", feature = "op_assume_role_with_web_identity"))]
 pub(crate) mod shape_invalid_identity_token_exception;

+#[cfg(feature = "op_get_web_identity_token")]
 pub(crate) mod shape_jwt_payload_size_exceeded_exception;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_get_federation_token"
+))]
 pub(crate) mod shape_malformed_policy_document_exception;

+#[cfg(feature = "op_get_web_identity_token")]
 pub(crate) mod shape_outbound_web_identity_federation_disabled_exception;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_get_delegated_access_token",
+    feature = "op_get_federation_token"
+))]
 pub(crate) mod shape_packed_policy_too_large_exception;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_assume_root",
+    feature = "op_get_federation_token"
+))]
 pub(crate) mod shape_policy_descriptor_type;

+#[cfg(feature = "op_assume_role")]
 pub(crate) mod shape_provided_context;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_assume_root",
+    feature = "op_get_delegated_access_token",
+    feature = "op_get_federation_token",
+    feature = "op_get_session_token"
+))]
 pub(crate) mod shape_region_disabled_exception;

+#[cfg(feature = "op_get_web_identity_token")]
 pub(crate) mod shape_session_duration_escalation_exception;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_get_federation_token",
+    feature = "op_get_web_identity_token"
+))]
 pub(crate) mod shape_tag;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity"
+))]
 pub(crate) mod shape_assumed_role_user;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_assume_root",
+    feature = "op_get_delegated_access_token",
+    feature = "op_get_federation_token",
+    feature = "op_get_session_token"
+))]
 pub(crate) mod shape_credentials;

+#[cfg(feature = "op_get_federation_token")]
 pub(crate) mod shape_federated_user;
diff --git a/src/protocol_serde/shape_assume_role.rs b/src/protocol_serde/shape_assume_role.rs
--- a/src/protocol_serde/shape_assume_role.rs
+++ b/src/protocol_serde/shape_assume_role.rs
@@ -4,10 +4,12 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<crate::operation::assume_role::AssumeRoleOutput, crate::operation::assume_role::AssumeRoleError> {
+) -> std::result::Result<crate::operation::assume_role::AssumeRoleOutput, crate::operation::assume_role::AssumeRoleError>
+{
     #[allow(unused_mut)]
-    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(crate::operation::assume_role::AssumeRoleError::unhandled)?;
+    let mut generic_builder =
+        crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
+            .map_err(crate::operation::assume_role::AssumeRoleError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
@@ -22,8 +24,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = crate::types::error::builders::ExpiredTokenExceptionBuilder::default();
-                output = crate::protocol_serde::shape_expired_token_exception::de_expired_token_exception_xml_err(_response_body, output)
-                    .map_err(crate::operation::assume_role::AssumeRoleError::unhandled)?;
+                output = crate::protocol_serde::shape_expired_token_exception::de_expired_token_exception_xml_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(crate::operation::assume_role::AssumeRoleError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -32,24 +37,26 @@
             }
             tmp
         }),
-        "MalformedPolicyDocument" => crate::operation::assume_role::AssumeRoleError::MalformedPolicyDocumentException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "MalformedPolicyDocument" => {
+            crate::operation::assume_role::AssumeRoleError::MalformedPolicyDocumentException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::MalformedPolicyDocumentExceptionBuilder::default();
-                output = crate::protocol_serde::shape_malformed_policy_document_exception::de_malformed_policy_document_exception_xml_err(
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::MalformedPolicyDocumentExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_malformed_policy_document_exception::de_malformed_policy_document_exception_xml_err(
                     _response_body,
                     output,
                 )
                 .map_err(crate::operation::assume_role::AssumeRoleError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
         "PackedPolicyTooLarge" => crate::operation::assume_role::AssumeRoleError::PackedPolicyTooLargeException({
             #[allow(unused_mut)]
             let mut tmp = {
@@ -73,8 +80,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = crate::types::error::builders::RegionDisabledExceptionBuilder::default();
-                output = crate::protocol_serde::shape_region_disabled_exception::de_region_disabled_exception_xml_err(_response_body, output)
-                    .map_err(crate::operation::assume_role::AssumeRoleError::unhandled)?;
+                output = crate::protocol_serde::shape_region_disabled_exception::de_region_disabled_exception_xml_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(crate::operation::assume_role::AssumeRoleError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -92,7 +102,8 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<crate::operation::assume_role::AssumeRoleOutput, crate::operation::assume_role::AssumeRoleError> {
+) -> std::result::Result<crate::operation::assume_role::AssumeRoleOutput, crate::operation::assume_role::AssumeRoleError>
+{
     Ok({
         #[allow(unused_mut)]
         let mut output = crate::operation::assume_role::builders::AssumeRoleOutputBuilder::default();
@@ -107,7 +118,10 @@
 pub fn de_assume_role(
     inp: &[u8],
     mut builder: crate::operation::assume_role::builders::AssumeRoleOutputBuilder,
-) -> std::result::Result<crate::operation::assume_role::builders::AssumeRoleOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
+) -> std::result::Result<
+    crate::operation::assume_role::builders::AssumeRoleOutputBuilder,
+    ::aws_smithy_xml::decode::XmlDecodeError,
+> {
     let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

     #[allow(unused_mut)]
@@ -182,7 +196,9 @@
         }
         }
     } else {
-        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom("expected AssumeRoleResult tag"));
+        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(
+            "expected AssumeRoleResult tag",
+        ));
     };
     Ok(builder)
 }
diff --git a/src/protocol_serde/shape_assume_role_input.rs b/src/protocol_serde/shape_assume_role_input.rs
--- a/src/protocol_serde/shape_assume_role_input.rs
+++ b/src/protocol_serde/shape_assume_role_input.rs
@@ -1,7 +1,8 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub fn ser_assume_role_input_input_input(
     input: &crate::operation::assume_role::AssumeRoleInput,
-) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError>
+{
     let mut out = String::new();
     #[allow(unused_mut)]
     let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "AssumeRole", "2011-06-15");
diff --git a/src/protocol_serde/shape_assume_role_with_saml.rs b/src/protocol_serde/shape_assume_role_with_saml.rs
--- a/src/protocol_serde/shape_assume_role_with_saml.rs
+++ b/src/protocol_serde/shape_assume_role_with_saml.rs
@@ -9,8 +9,9 @@
     crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError,
 > {
     #[allow(unused_mut)]
-    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::unhandled)?;
+    let mut generic_builder =
+        crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
+            .map_err(crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
@@ -20,105 +21,124 @@

     let _error_message = generic.message().map(|msg| msg.to_owned());
     Err(match error_code {
-        "ExpiredTokenException" => crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::ExpiredTokenException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "ExpiredTokenException" => {
+            crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::ExpiredTokenException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::ExpiredTokenExceptionBuilder::default();
-                output = crate::protocol_serde::shape_expired_token_exception::de_expired_token_exception_xml_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::ExpiredTokenExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_expired_token_exception::de_expired_token_exception_xml_err(
+                        _response_body,
+                        output,
+                    )
                     .map_err(crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "IDPRejectedClaim" => crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::IdpRejectedClaimException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
+        "IDPRejectedClaim" => {
+            crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::IdpRejectedClaimException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::IdpRejectedClaimExceptionBuilder::default();
-                output = crate::protocol_serde::shape_idp_rejected_claim_exception::de_idp_rejected_claim_exception_xml_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::IdpRejectedClaimExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_idp_rejected_claim_exception::de_idp_rejected_claim_exception_xml_err(_response_body, output)
                     .map_err(crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "InvalidIdentityToken" => crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::InvalidIdentityTokenException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
+        "InvalidIdentityToken" => {
+            crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::InvalidIdentityTokenException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::InvalidIdentityTokenExceptionBuilder::default();
-                output = crate::protocol_serde::shape_invalid_identity_token_exception::de_invalid_identity_token_exception_xml_err(
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::InvalidIdentityTokenExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_invalid_identity_token_exception::de_invalid_identity_token_exception_xml_err(
                     _response_body,
                     output,
                 )
                 .map_err(crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "MalformedPolicyDocument" => crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::MalformedPolicyDocumentException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
+        "MalformedPolicyDocument" => {
+            crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::MalformedPolicyDocumentException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::MalformedPolicyDocumentExceptionBuilder::default();
-                output = crate::protocol_serde::shape_malformed_policy_document_exception::de_malformed_policy_document_exception_xml_err(
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::MalformedPolicyDocumentExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_malformed_policy_document_exception::de_malformed_policy_document_exception_xml_err(
                     _response_body,
                     output,
                 )
                 .map_err(crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "PackedPolicyTooLarge" => crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::PackedPolicyTooLargeException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
+        "PackedPolicyTooLarge" => {
+            crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::PackedPolicyTooLargeException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::PackedPolicyTooLargeExceptionBuilder::default();
-                output = crate::protocol_serde::shape_packed_policy_too_large_exception::de_packed_policy_too_large_exception_xml_err(
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::PackedPolicyTooLargeExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_packed_policy_too_large_exception::de_packed_policy_too_large_exception_xml_err(
                     _response_body,
                     output,
                 )
                 .map_err(crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "RegionDisabledException" => crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::RegionDisabledException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
+        "RegionDisabledException" => {
+            crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::RegionDisabledException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::RegionDisabledExceptionBuilder::default();
-                output = crate::protocol_serde::shape_region_disabled_exception::de_region_disabled_exception_xml_err(_response_body, output)
-                    .map_err(crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::RegionDisabledExceptionBuilder::default();
+                    output =
+                        crate::protocol_serde::shape_region_disabled_exception::de_region_disabled_exception_xml_err(
+                            _response_body,
+                            output,
+                        )
+                        .map_err(crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::unhandled)?;
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
         _ => crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::generic(generic),
     })
 }
@@ -146,8 +166,10 @@
 pub fn de_assume_role_with_saml(
     inp: &[u8],
     mut builder: crate::operation::assume_role_with_saml::builders::AssumeRoleWithSamlOutputBuilder,
-) -> std::result::Result<crate::operation::assume_role_with_saml::builders::AssumeRoleWithSamlOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError>
-{
+) -> std::result::Result<
+    crate::operation::assume_role_with_saml::builders::AssumeRoleWithSamlOutputBuilder,
+    ::aws_smithy_xml::decode::XmlDecodeError,
+> {
     let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

     #[allow(unused_mut)]
@@ -287,7 +309,9 @@
         }
         }
     } else {
-        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom("expected AssumeRoleWithSAMLResult tag"));
+        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(
+            "expected AssumeRoleWithSAMLResult tag",
+        ));
     };
     Ok(builder)
 }
diff --git a/src/protocol_serde/shape_assume_role_with_saml_input.rs b/src/protocol_serde/shape_assume_role_with_saml_input.rs
--- a/src/protocol_serde/shape_assume_role_with_saml_input.rs
+++ b/src/protocol_serde/shape_assume_role_with_saml_input.rs
@@ -1,7 +1,8 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub fn ser_assume_role_with_saml_input_input_input(
     input: &crate::operation::assume_role_with_saml::AssumeRoleWithSamlInput,
-) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError>
+{
     let mut out = String::new();
     #[allow(unused_mut)]
     let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "AssumeRoleWithSAML", "2011-06-15");
diff --git a/src/protocol_serde/shape_assume_role_with_web_identity.rs b/src/protocol_serde/shape_assume_role_with_web_identity.rs
--- a/src/protocol_serde/shape_assume_role_with_web_identity.rs
+++ b/src/protocol_serde/shape_assume_role_with_web_identity.rs
@@ -9,13 +9,18 @@
     crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError,
 > {
     #[allow(unused_mut)]
-    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError::unhandled)?;
+    let mut generic_builder =
+        crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
+            .map_err(crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError::unhandled(generic)),
+        None => {
+            return Err(
+                crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError::unhandled(generic),
+            )
+        }
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
@@ -154,9 +159,14 @@
 > {
     Ok({
         #[allow(unused_mut)]
-        let mut output = crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityOutputBuilder::default();
-        output = crate::protocol_serde::shape_assume_role_with_web_identity::de_assume_role_with_web_identity(_response_body, output)
-            .map_err(crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError::unhandled)?;
+        let mut output =
+            crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityOutputBuilder::default(
+            );
+        output = crate::protocol_serde::shape_assume_role_with_web_identity::de_assume_role_with_web_identity(
+            _response_body,
+            output,
+        )
+        .map_err(crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
         output.build()
     })
diff --git a/src/protocol_serde/shape_assume_role_with_web_identity_input.rs b/src/protocol_serde/shape_assume_role_with_web_identity_input.rs
--- a/src/protocol_serde/shape_assume_role_with_web_identity_input.rs
+++ b/src/protocol_serde/shape_assume_role_with_web_identity_input.rs
@@ -1,7 +1,8 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub fn ser_assume_role_with_web_identity_input_input_input(
     input: &crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityInput,
-) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError>
+{
     let mut out = String::new();
     #[allow(unused_mut)]
     let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "AssumeRoleWithWebIdentity", "2011-06-15");
diff --git a/src/protocol_serde/shape_assume_root.rs b/src/protocol_serde/shape_assume_root.rs
--- a/src/protocol_serde/shape_assume_root.rs
+++ b/src/protocol_serde/shape_assume_root.rs
@@ -4,10 +4,12 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<crate::operation::assume_root::AssumeRootOutput, crate::operation::assume_root::AssumeRootError> {
+) -> std::result::Result<crate::operation::assume_root::AssumeRootOutput, crate::operation::assume_root::AssumeRootError>
+{
     #[allow(unused_mut)]
-    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(crate::operation::assume_root::AssumeRootError::unhandled)?;
+    let mut generic_builder =
+        crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
+            .map_err(crate::operation::assume_root::AssumeRootError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
@@ -22,8 +24,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = crate::types::error::builders::ExpiredTokenExceptionBuilder::default();
-                output = crate::protocol_serde::shape_expired_token_exception::de_expired_token_exception_xml_err(_response_body, output)
-                    .map_err(crate::operation::assume_root::AssumeRootError::unhandled)?;
+                output = crate::protocol_serde::shape_expired_token_exception::de_expired_token_exception_xml_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(crate::operation::assume_root::AssumeRootError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -37,8 +42,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = crate::types::error::builders::RegionDisabledExceptionBuilder::default();
-                output = crate::protocol_serde::shape_region_disabled_exception::de_region_disabled_exception_xml_err(_response_body, output)
-                    .map_err(crate::operation::assume_root::AssumeRootError::unhandled)?;
+                output = crate::protocol_serde::shape_region_disabled_exception::de_region_disabled_exception_xml_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(crate::operation::assume_root::AssumeRootError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -56,7 +64,8 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<crate::operation::assume_root::AssumeRootOutput, crate::operation::assume_root::AssumeRootError> {
+) -> std::result::Result<crate::operation::assume_root::AssumeRootOutput, crate::operation::assume_root::AssumeRootError>
+{
     Ok({
         #[allow(unused_mut)]
         let mut output = crate::operation::assume_root::builders::AssumeRootOutputBuilder::default();
@@ -71,7 +80,10 @@
 pub fn de_assume_root(
     inp: &[u8],
     mut builder: crate::operation::assume_root::builders::AssumeRootOutputBuilder,
-) -> std::result::Result<crate::operation::assume_root::builders::AssumeRootOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
+) -> std::result::Result<
+    crate::operation::assume_root::builders::AssumeRootOutputBuilder,
+    ::aws_smithy_xml::decode::XmlDecodeError,
+> {
     let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

     #[allow(unused_mut)]
@@ -121,7 +133,9 @@
         }
         }
     } else {
-        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom("expected AssumeRootResult tag"));
+        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(
+            "expected AssumeRootResult tag",
+        ));
     };
     Ok(builder)
 }
diff --git a/src/protocol_serde/shape_assume_root_input.rs b/src/protocol_serde/shape_assume_root_input.rs
--- a/src/protocol_serde/shape_assume_root_input.rs
+++ b/src/protocol_serde/shape_assume_root_input.rs
@@ -1,7 +1,8 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub fn ser_assume_root_input_input_input(
     input: &crate::operation::assume_root::AssumeRootInput,
-) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError>
+{
     let mut out = String::new();
     #[allow(unused_mut)]
     let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "AssumeRoot", "2011-06-15");
diff --git a/src/protocol_serde/shape_assumed_role_user.rs b/src/protocol_serde/shape_assumed_role_user.rs
--- a/src/protocol_serde/shape_assumed_role_user.rs
+++ b/src/protocol_serde/shape_assumed_role_user.rs
@@ -5,7 +5,9 @@
     depth: u32,
 ) -> ::std::result::Result<crate::types::AssumedRoleUser, ::aws_smithy_xml::decode::XmlDecodeError> {
     if depth >= 128u32 {
-        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom("maximum nesting depth exceeded"));
+        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(
+            "maximum nesting depth exceeded",
+        ));
     }
     #[allow(unused_mut)]
     let mut builder = crate::types::AssumedRoleUser::builder();
diff --git a/src/protocol_serde/shape_credentials.rs b/src/protocol_serde/shape_credentials.rs
--- a/src/protocol_serde/shape_credentials.rs
+++ b/src/protocol_serde/shape_credentials.rs
@@ -5,7 +5,9 @@
     depth: u32,
 ) -> ::std::result::Result<crate::types::Credentials, ::aws_smithy_xml::decode::XmlDecodeError> {
     if depth >= 128u32 {
-        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom("maximum nesting depth exceeded"));
+        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(
+            "maximum nesting depth exceeded",
+        ));
     }
     #[allow(unused_mut)]
     let mut builder = crate::types::Credentials::builder();
diff --git a/src/protocol_serde/shape_decode_authorization_message.rs b/src/protocol_serde/shape_decode_authorization_message.rs
--- a/src/protocol_serde/shape_decode_authorization_message.rs
+++ b/src/protocol_serde/shape_decode_authorization_message.rs
@@ -9,13 +9,18 @@
     crate::operation::decode_authorization_message::DecodeAuthorizationMessageError,
 > {
     #[allow(unused_mut)]
-    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(crate::operation::decode_authorization_message::DecodeAuthorizationMessageError::unhandled)?;
+    let mut generic_builder =
+        crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
+            .map_err(crate::operation::decode_authorization_message::DecodeAuthorizationMessageError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(crate::operation::decode_authorization_message::DecodeAuthorizationMessageError::unhandled(generic)),
+        None => {
+            return Err(
+                crate::operation::decode_authorization_message::DecodeAuthorizationMessageError::unhandled(generic),
+            )
+        }
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
@@ -56,9 +61,14 @@
 > {
     Ok({
         #[allow(unused_mut)]
-        let mut output = crate::operation::decode_authorization_message::builders::DecodeAuthorizationMessageOutputBuilder::default();
-        output = crate::protocol_serde::shape_decode_authorization_message::de_decode_authorization_message(_response_body, output)
-            .map_err(crate::operation::decode_authorization_message::DecodeAuthorizationMessageError::unhandled)?;
+        let mut output =
+            crate::operation::decode_authorization_message::builders::DecodeAuthorizationMessageOutputBuilder::default(
+            );
+        output = crate::protocol_serde::shape_decode_authorization_message::de_decode_authorization_message(
+            _response_body,
+            output,
+        )
+        .map_err(crate::operation::decode_authorization_message::DecodeAuthorizationMessageError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
         output.build()
     })
diff --git a/src/protocol_serde/shape_decode_authorization_message_input.rs b/src/protocol_serde/shape_decode_authorization_message_input.rs
--- a/src/protocol_serde/shape_decode_authorization_message_input.rs
+++ b/src/protocol_serde/shape_decode_authorization_message_input.rs
@@ -1,7 +1,8 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub fn ser_decode_authorization_message_input_input_input(
     input: &crate::operation::decode_authorization_message::DecodeAuthorizationMessageInput,
-) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError>
+{
     let mut out = String::new();
     #[allow(unused_mut)]
     let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "DecodeAuthorizationMessage", "2011-06-15");
diff --git a/src/protocol_serde/shape_expired_token_exception.rs b/src/protocol_serde/shape_expired_token_exception.rs
--- a/src/protocol_serde/shape_expired_token_exception.rs
+++ b/src/protocol_serde/shape_expired_token_exception.rs
@@ -3,7 +3,10 @@
 pub fn de_expired_token_exception_xml_err(
     inp: &[u8],
     mut builder: crate::types::error::builders::ExpiredTokenExceptionBuilder,
-) -> std::result::Result<crate::types::error::builders::ExpiredTokenExceptionBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
+) -> std::result::Result<
+    crate::types::error::builders::ExpiredTokenExceptionBuilder,
+    ::aws_smithy_xml::decode::XmlDecodeError,
+> {
     if inp.is_empty() {
         return Ok(builder);
     }
diff --git a/src/protocol_serde/shape_expired_trade_in_token_exception.rs b/src/protocol_serde/shape_expired_trade_in_token_exception.rs
--- a/src/protocol_serde/shape_expired_trade_in_token_exception.rs
+++ b/src/protocol_serde/shape_expired_trade_in_token_exception.rs
@@ -3,7 +3,10 @@
 pub fn de_expired_trade_in_token_exception_xml_err(
     inp: &[u8],
     mut builder: crate::types::error::builders::ExpiredTradeInTokenExceptionBuilder,
-) -> std::result::Result<crate::types::error::builders::ExpiredTradeInTokenExceptionBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
+) -> std::result::Result<
+    crate::types::error::builders::ExpiredTradeInTokenExceptionBuilder,
+    ::aws_smithy_xml::decode::XmlDecodeError,
+> {
     if inp.is_empty() {
         return Ok(builder);
     }
diff --git a/src/protocol_serde/shape_federated_user.rs b/src/protocol_serde/shape_federated_user.rs
--- a/src/protocol_serde/shape_federated_user.rs
+++ b/src/protocol_serde/shape_federated_user.rs
@@ -5,7 +5,9 @@
     depth: u32,
 ) -> ::std::result::Result<crate::types::FederatedUser, ::aws_smithy_xml::decode::XmlDecodeError> {
     if depth >= 128u32 {
-        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom("maximum nesting depth exceeded"));
+        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(
+            "maximum nesting depth exceeded",
+        ));
     }
     #[allow(unused_mut)]
     let mut builder = crate::types::FederatedUser::builder();
diff --git a/src/protocol_serde/shape_get_access_key_info.rs b/src/protocol_serde/shape_get_access_key_info.rs
--- a/src/protocol_serde/shape_get_access_key_info.rs
+++ b/src/protocol_serde/shape_get_access_key_info.rs
@@ -4,14 +4,19 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<crate::operation::get_access_key_info::GetAccessKeyInfoOutput, crate::operation::get_access_key_info::GetAccessKeyInfoError>
-{
+) -> std::result::Result<
+    crate::operation::get_access_key_info::GetAccessKeyInfoOutput,
+    crate::operation::get_access_key_info::GetAccessKeyInfoError,
+> {
     #[allow(unused_mut)]
-    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(crate::operation::get_access_key_info::GetAccessKeyInfoError::unhandled)?;
+    let mut generic_builder =
+        crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
+            .map_err(crate::operation::get_access_key_info::GetAccessKeyInfoError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
-    Err(crate::operation::get_access_key_info::GetAccessKeyInfoError::generic(generic))
+    Err(crate::operation::get_access_key_info::GetAccessKeyInfoError::generic(
+        generic,
+    ))
 }

 #[allow(clippy::unnecessary_wraps)]
@@ -19,8 +24,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<crate::operation::get_access_key_info::GetAccessKeyInfoOutput, crate::operation::get_access_key_info::GetAccessKeyInfoError>
-{
+) -> std::result::Result<
+    crate::operation::get_access_key_info::GetAccessKeyInfoOutput,
+    crate::operation::get_access_key_info::GetAccessKeyInfoError,
+> {
     Ok({
         #[allow(unused_mut)]
         let mut output = crate::operation::get_access_key_info::builders::GetAccessKeyInfoOutputBuilder::default();
@@ -35,7 +42,10 @@
 pub fn de_get_access_key_info(
     inp: &[u8],
     mut builder: crate::operation::get_access_key_info::builders::GetAccessKeyInfoOutputBuilder,
-) -> std::result::Result<crate::operation::get_access_key_info::builders::GetAccessKeyInfoOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
+) -> std::result::Result<
+    crate::operation::get_access_key_info::builders::GetAccessKeyInfoOutputBuilder,
+    ::aws_smithy_xml::decode::XmlDecodeError,
+> {
     let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

     #[allow(unused_mut)]
@@ -75,7 +85,9 @@
         }
         }
     } else {
-        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom("expected GetAccessKeyInfoResult tag"));
+        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(
+            "expected GetAccessKeyInfoResult tag",
+        ));
     };
     Ok(builder)
 }
diff --git a/src/protocol_serde/shape_get_access_key_info_input.rs b/src/protocol_serde/shape_get_access_key_info_input.rs
--- a/src/protocol_serde/shape_get_access_key_info_input.rs
+++ b/src/protocol_serde/shape_get_access_key_info_input.rs
@@ -1,7 +1,8 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub fn ser_get_access_key_info_input_input_input(
     input: &crate::operation::get_access_key_info::GetAccessKeyInfoInput,
-) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError>
+{
     let mut out = String::new();
     #[allow(unused_mut)]
     let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "GetAccessKeyInfo", "2011-06-15");
diff --git a/src/protocol_serde/shape_get_caller_identity.rs b/src/protocol_serde/shape_get_caller_identity.rs
--- a/src/protocol_serde/shape_get_caller_identity.rs
+++ b/src/protocol_serde/shape_get_caller_identity.rs
@@ -4,14 +4,19 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<crate::operation::get_caller_identity::GetCallerIdentityOutput, crate::operation::get_caller_identity::GetCallerIdentityError>
-{
+) -> std::result::Result<
+    crate::operation::get_caller_identity::GetCallerIdentityOutput,
+    crate::operation::get_caller_identity::GetCallerIdentityError,
+> {
     #[allow(unused_mut)]
-    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(crate::operation::get_caller_identity::GetCallerIdentityError::unhandled)?;
+    let mut generic_builder =
+        crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
+            .map_err(crate::operation::get_caller_identity::GetCallerIdentityError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
-    Err(crate::operation::get_caller_identity::GetCallerIdentityError::generic(generic))
+    Err(crate::operation::get_caller_identity::GetCallerIdentityError::generic(
+        generic,
+    ))
 }

 #[allow(clippy::unnecessary_wraps)]
@@ -19,8 +24,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<crate::operation::get_caller_identity::GetCallerIdentityOutput, crate::operation::get_caller_identity::GetCallerIdentityError>
-{
+) -> std::result::Result<
+    crate::operation::get_caller_identity::GetCallerIdentityOutput,
+    crate::operation::get_caller_identity::GetCallerIdentityError,
+> {
     Ok({
         #[allow(unused_mut)]
         let mut output = crate::operation::get_caller_identity::builders::GetCallerIdentityOutputBuilder::default();
@@ -35,7 +42,10 @@
 pub fn de_get_caller_identity(
     inp: &[u8],
     mut builder: crate::operation::get_caller_identity::builders::GetCallerIdentityOutputBuilder,
-) -> std::result::Result<crate::operation::get_caller_identity::builders::GetCallerIdentityOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
+) -> std::result::Result<
+    crate::operation::get_caller_identity::builders::GetCallerIdentityOutputBuilder,
+    ::aws_smithy_xml::decode::XmlDecodeError,
+> {
     let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

     #[allow(unused_mut)]
@@ -101,7 +111,9 @@
         }
         }
     } else {
-        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom("expected GetCallerIdentityResult tag"));
+        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(
+            "expected GetCallerIdentityResult tag",
+        ));
     };
     Ok(builder)
 }
diff --git a/src/protocol_serde/shape_get_caller_identity_input.rs b/src/protocol_serde/shape_get_caller_identity_input.rs
--- a/src/protocol_serde/shape_get_caller_identity_input.rs
+++ b/src/protocol_serde/shape_get_caller_identity_input.rs
@@ -1,7 +1,8 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub fn ser_get_caller_identity_input_input_input(
     input: &crate::operation::get_caller_identity::GetCallerIdentityInput,
-) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError>
+{
     let _ = input;
     let mut out = String::new();
     #[allow(unused_mut)]
diff --git a/src/protocol_serde/shape_get_delegated_access_token.rs b/src/protocol_serde/shape_get_delegated_access_token.rs
--- a/src/protocol_serde/shape_get_delegated_access_token.rs
+++ b/src/protocol_serde/shape_get_delegated_access_token.rs
@@ -9,72 +9,83 @@
     crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError,
 > {
     #[allow(unused_mut)]
-    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError::unhandled)?;
+    let mut generic_builder =
+        crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
+            .map_err(crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
         None => {
-            return Err(crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError::unhandled(
-                generic,
-            ))
+            return Err(crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError::unhandled(generic))
         }
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
     Err(match error_code {
-        "ExpiredTradeInTokenException" => crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError::ExpiredTradeInTokenException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "ExpiredTradeInTokenException" => {
+            crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError::ExpiredTradeInTokenException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::ExpiredTradeInTokenExceptionBuilder::default();
-                output = crate::protocol_serde::shape_expired_trade_in_token_exception::de_expired_trade_in_token_exception_xml_err(
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::ExpiredTradeInTokenExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_expired_trade_in_token_exception::de_expired_trade_in_token_exception_xml_err(
                     _response_body,
                     output,
                 )
                 .map_err(crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "PackedPolicyTooLarge" => crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError::PackedPolicyTooLargeException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
+        "PackedPolicyTooLarge" => {
+            crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError::PackedPolicyTooLargeException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::PackedPolicyTooLargeExceptionBuilder::default();
-                output = crate::protocol_serde::shape_packed_policy_too_large_exception::de_packed_policy_too_large_exception_xml_err(
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::PackedPolicyTooLargeExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_packed_policy_too_large_exception::de_packed_policy_too_large_exception_xml_err(
                     _response_body,
                     output,
                 )
                 .map_err(crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "RegionDisabledException" => crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError::RegionDisabledException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
+        "RegionDisabledException" => {
+            crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError::RegionDisabledException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::RegionDisabledExceptionBuilder::default();
-                output = crate::protocol_serde::shape_region_disabled_exception::de_region_disabled_exception_xml_err(_response_body, output)
-                    .map_err(crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::RegionDisabledExceptionBuilder::default();
+                    output =
+                        crate::protocol_serde::shape_region_disabled_exception::de_region_disabled_exception_xml_err(
+                            _response_body,
+                            output,
+                        )
+                        .map_err(
+                            crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError::unhandled,
+                        )?;
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
         _ => crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError::generic(generic),
     })
 }
@@ -90,9 +101,13 @@
 > {
     Ok({
         #[allow(unused_mut)]
-        let mut output = crate::operation::get_delegated_access_token::builders::GetDelegatedAccessTokenOutputBuilder::default();
-        output = crate::protocol_serde::shape_get_delegated_access_token::de_get_delegated_access_token(_response_body, output)
-            .map_err(crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError::unhandled)?;
+        let mut output =
+            crate::operation::get_delegated_access_token::builders::GetDelegatedAccessTokenOutputBuilder::default();
+        output = crate::protocol_serde::shape_get_delegated_access_token::de_get_delegated_access_token(
+            _response_body,
+            output,
+        )
+        .map_err(crate::operation::get_delegated_access_token::GetDelegatedAccessTokenError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
         output.build()
     })
diff --git a/src/protocol_serde/shape_get_delegated_access_token_input.rs b/src/protocol_serde/shape_get_delegated_access_token_input.rs
--- a/src/protocol_serde/shape_get_delegated_access_token_input.rs
+++ b/src/protocol_serde/shape_get_delegated_access_token_input.rs
@@ -1,7 +1,8 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub fn ser_get_delegated_access_token_input_input_input(
     input: &crate::operation::get_delegated_access_token::GetDelegatedAccessTokenInput,
-) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError>
+{
     let mut out = String::new();
     #[allow(unused_mut)]
     let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "GetDelegatedAccessToken", "2011-06-15");
diff --git a/src/protocol_serde/shape_get_federation_token.rs b/src/protocol_serde/shape_get_federation_token.rs
--- a/src/protocol_serde/shape_get_federation_token.rs
+++ b/src/protocol_serde/shape_get_federation_token.rs
@@ -9,8 +9,9 @@
     crate::operation::get_federation_token::GetFederationTokenError,
 > {
     #[allow(unused_mut)]
-    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(crate::operation::get_federation_token::GetFederationTokenError::unhandled)?;
+    let mut generic_builder =
+        crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
+            .map_err(crate::operation::get_federation_token::GetFederationTokenError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
@@ -20,57 +21,67 @@

     let _error_message = generic.message().map(|msg| msg.to_owned());
     Err(match error_code {
-        "MalformedPolicyDocument" => crate::operation::get_federation_token::GetFederationTokenError::MalformedPolicyDocumentException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "MalformedPolicyDocument" => {
+            crate::operation::get_federation_token::GetFederationTokenError::MalformedPolicyDocumentException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::MalformedPolicyDocumentExceptionBuilder::default();
-                output = crate::protocol_serde::shape_malformed_policy_document_exception::de_malformed_policy_document_exception_xml_err(
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::MalformedPolicyDocumentExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_malformed_policy_document_exception::de_malformed_policy_document_exception_xml_err(
                     _response_body,
                     output,
                 )
                 .map_err(crate::operation::get_federation_token::GetFederationTokenError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "PackedPolicyTooLarge" => crate::operation::get_federation_token::GetFederationTokenError::PackedPolicyTooLargeException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
+        "PackedPolicyTooLarge" => {
+            crate::operation::get_federation_token::GetFederationTokenError::PackedPolicyTooLargeException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::PackedPolicyTooLargeExceptionBuilder::default();
-                output = crate::protocol_serde::shape_packed_policy_too_large_exception::de_packed_policy_too_large_exception_xml_err(
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::PackedPolicyTooLargeExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_packed_policy_too_large_exception::de_packed_policy_too_large_exception_xml_err(
                     _response_body,
                     output,
                 )
                 .map_err(crate::operation::get_federation_token::GetFederationTokenError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "RegionDisabledException" => crate::operation::get_federation_token::GetFederationTokenError::RegionDisabledException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
+        "RegionDisabledException" => {
+            crate::operation::get_federation_token::GetFederationTokenError::RegionDisabledException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::RegionDisabledExceptionBuilder::default();
-                output = crate::protocol_serde::shape_region_disabled_exception::de_region_disabled_exception_xml_err(_response_body, output)
-                    .map_err(crate::operation::get_federation_token::GetFederationTokenError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::RegionDisabledExceptionBuilder::default();
+                    output =
+                        crate::protocol_serde::shape_region_disabled_exception::de_region_disabled_exception_xml_err(
+                            _response_body,
+                            output,
+                        )
+                        .map_err(crate::operation::get_federation_token::GetFederationTokenError::unhandled)?;
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
         _ => crate::operation::get_federation_token::GetFederationTokenError::generic(generic),
     })
 }
@@ -98,8 +109,10 @@
 pub fn de_get_federation_token(
     inp: &[u8],
     mut builder: crate::operation::get_federation_token::builders::GetFederationTokenOutputBuilder,
-) -> std::result::Result<crate::operation::get_federation_token::builders::GetFederationTokenOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError>
-{
+) -> std::result::Result<
+    crate::operation::get_federation_token::builders::GetFederationTokenOutputBuilder,
+    ::aws_smithy_xml::decode::XmlDecodeError,
+> {
     let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

     #[allow(unused_mut)]
@@ -161,7 +174,9 @@
         }
         }
     } else {
-        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom("expected GetFederationTokenResult tag"));
+        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(
+            "expected GetFederationTokenResult tag",
+        ));
     };
     Ok(builder)
 }
diff --git a/src/protocol_serde/shape_get_federation_token_input.rs b/src/protocol_serde/shape_get_federation_token_input.rs
--- a/src/protocol_serde/shape_get_federation_token_input.rs
+++ b/src/protocol_serde/shape_get_federation_token_input.rs
@@ -1,7 +1,8 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub fn ser_get_federation_token_input_input_input(
     input: &crate::operation::get_federation_token::GetFederationTokenInput,
-) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError>
+{
     let mut out = String::new();
     #[allow(unused_mut)]
     let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "GetFederationToken", "2011-06-15");
diff --git a/src/protocol_serde/shape_get_session_token.rs b/src/protocol_serde/shape_get_session_token.rs
--- a/src/protocol_serde/shape_get_session_token.rs
+++ b/src/protocol_serde/shape_get_session_token.rs
@@ -4,34 +4,48 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<crate::operation::get_session_token::GetSessionTokenOutput, crate::operation::get_session_token::GetSessionTokenError> {
+) -> std::result::Result<
+    crate::operation::get_session_token::GetSessionTokenOutput,
+    crate::operation::get_session_token::GetSessionTokenError,
+> {
     #[allow(unused_mut)]
-    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(crate::operation::get_session_token::GetSessionTokenError::unhandled)?;
+    let mut generic_builder =
+        crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
+            .map_err(crate::operation::get_session_token::GetSessionTokenError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(crate::operation::get_session_token::GetSessionTokenError::unhandled(generic)),
+        None => {
+            return Err(crate::operation::get_session_token::GetSessionTokenError::unhandled(
+                generic,
+            ))
+        }
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
     Err(match error_code {
-        "RegionDisabledException" => crate::operation::get_session_token::GetSessionTokenError::RegionDisabledException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "RegionDisabledException" => {
+            crate::operation::get_session_token::GetSessionTokenError::RegionDisabledException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::RegionDisabledExceptionBuilder::default();
-                output = crate::protocol_serde::shape_region_disabled_exception::de_region_disabled_exception_xml_err(_response_body, output)
-                    .map_err(crate::operation::get_session_token::GetSessionTokenError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::RegionDisabledExceptionBuilder::default();
+                    output =
+                        crate::protocol_serde::shape_region_disabled_exception::de_region_disabled_exception_xml_err(
+                            _response_body,
+                            output,
+                        )
+                        .map_err(crate::operation::get_session_token::GetSessionTokenError::unhandled)?;
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
         _ => crate::operation::get_session_token::GetSessionTokenError::generic(generic),
     })
 }
@@ -41,7 +55,10 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<crate::operation::get_session_token::GetSessionTokenOutput, crate::operation::get_session_token::GetSessionTokenError> {
+) -> std::result::Result<
+    crate::operation::get_session_token::GetSessionTokenOutput,
+    crate::operation::get_session_token::GetSessionTokenError,
+> {
     Ok({
         #[allow(unused_mut)]
         let mut output = crate::operation::get_session_token::builders::GetSessionTokenOutputBuilder::default();
@@ -56,7 +73,10 @@
 pub fn de_get_session_token(
     inp: &[u8],
     mut builder: crate::operation::get_session_token::builders::GetSessionTokenOutputBuilder,
-) -> std::result::Result<crate::operation::get_session_token::builders::GetSessionTokenOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
+) -> std::result::Result<
+    crate::operation::get_session_token::builders::GetSessionTokenOutputBuilder,
+    ::aws_smithy_xml::decode::XmlDecodeError,
+> {
     let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

     #[allow(unused_mut)]
@@ -93,7 +113,9 @@
         }
         }
     } else {
-        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom("expected GetSessionTokenResult tag"));
+        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(
+            "expected GetSessionTokenResult tag",
+        ));
     };
     Ok(builder)
 }
diff --git a/src/protocol_serde/shape_get_session_token_input.rs b/src/protocol_serde/shape_get_session_token_input.rs
--- a/src/protocol_serde/shape_get_session_token_input.rs
+++ b/src/protocol_serde/shape_get_session_token_input.rs
@@ -1,7 +1,8 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub fn ser_get_session_token_input_input_input(
     input: &crate::operation::get_session_token::GetSessionTokenInput,
-) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError>
+{
     let mut out = String::new();
     #[allow(unused_mut)]
     let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "GetSessionToken", "2011-06-15");
diff --git a/src/protocol_serde/shape_get_web_identity_token.rs b/src/protocol_serde/shape_get_web_identity_token.rs
--- a/src/protocol_serde/shape_get_web_identity_token.rs
+++ b/src/protocol_serde/shape_get_web_identity_token.rs
@@ -9,8 +9,9 @@
     crate::operation::get_web_identity_token::GetWebIdentityTokenError,
 > {
     #[allow(unused_mut)]
-    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(crate::operation::get_web_identity_token::GetWebIdentityTokenError::unhandled)?;
+    let mut generic_builder =
+        crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
+            .map_err(crate::operation::get_web_identity_token::GetWebIdentityTokenError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
@@ -89,7 +90,8 @@
 > {
     Ok({
         #[allow(unused_mut)]
-        let mut output = crate::operation::get_web_identity_token::builders::GetWebIdentityTokenOutputBuilder::default();
+        let mut output =
+            crate::operation::get_web_identity_token::builders::GetWebIdentityTokenOutputBuilder::default();
         output = crate::protocol_serde::shape_get_web_identity_token::de_get_web_identity_token(_response_body, output)
             .map_err(crate::operation::get_web_identity_token::GetWebIdentityTokenError::unhandled)?;
         output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
@@ -101,8 +103,10 @@
 pub fn de_get_web_identity_token(
     inp: &[u8],
     mut builder: crate::operation::get_web_identity_token::builders::GetWebIdentityTokenOutputBuilder,
-) -> std::result::Result<crate::operation::get_web_identity_token::builders::GetWebIdentityTokenOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError>
-{
+) -> std::result::Result<
+    crate::operation::get_web_identity_token::builders::GetWebIdentityTokenOutputBuilder,
+    ::aws_smithy_xml::decode::XmlDecodeError,
+> {
     let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

     #[allow(unused_mut)]
@@ -156,7 +160,9 @@
         }
         }
     } else {
-        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom("expected GetWebIdentityTokenResult tag"));
+        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(
+            "expected GetWebIdentityTokenResult tag",
+        ));
     };
     Ok(builder)
 }
diff --git a/src/protocol_serde/shape_get_web_identity_token_input.rs b/src/protocol_serde/shape_get_web_identity_token_input.rs
--- a/src/protocol_serde/shape_get_web_identity_token_input.rs
+++ b/src/protocol_serde/shape_get_web_identity_token_input.rs
@@ -1,7 +1,8 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub fn ser_get_web_identity_token_input_input_input(
     input: &crate::operation::get_web_identity_token::GetWebIdentityTokenInput,
-) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
+) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError>
+{
     let mut out = String::new();
     #[allow(unused_mut)]
     let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "GetWebIdentityToken", "2011-06-15");
diff --git a/src/protocol_serde/shape_idp_communication_error_exception.rs b/src/protocol_serde/shape_idp_communication_error_exception.rs
--- a/src/protocol_serde/shape_idp_communication_error_exception.rs
+++ b/src/protocol_serde/shape_idp_communication_error_exception.rs
@@ -3,7 +3,10 @@
 pub fn de_idp_communication_error_exception_xml_err(
     inp: &[u8],
     mut builder: crate::types::error::builders::IdpCommunicationErrorExceptionBuilder,
-) -> std::result::Result<crate::types::error::builders::IdpCommunicationErrorExceptionBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
+) -> std::result::Result<
+    crate::types::error::builders::IdpCommunicationErrorExceptionBuilder,
+    ::aws_smithy_xml::decode::XmlDecodeError,
+> {
     if inp.is_empty() {
         return Ok(builder);
     }
diff --git a/src/protocol_serde/shape_idp_rejected_claim_exception.rs b/src/protocol_serde/shape_idp_rejected_claim_exception.rs
--- a/src/protocol_serde/shape_idp_rejected_claim_exception.rs
+++ b/src/protocol_serde/shape_idp_rejected_claim_exception.rs
@@ -3,7 +3,10 @@
 pub fn de_idp_rejected_claim_exception_xml_err(
     inp: &[u8],
     mut builder: crate::types::error::builders::IdpRejectedClaimExceptionBuilder,
-) -> std::result::Result<crate::types::error::builders::IdpRejectedClaimExceptionBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
+) -> std::result::Result<
+    crate::types::error::builders::IdpRejectedClaimExceptionBuilder,
+    ::aws_smithy_xml::decode::XmlDecodeError,
+> {
     if inp.is_empty() {
         return Ok(builder);
     }
diff --git a/src/protocol_serde/shape_invalid_authorization_message_exception.rs b/src/protocol_serde/shape_invalid_authorization_message_exception.rs
--- a/src/protocol_serde/shape_invalid_authorization_message_exception.rs
+++ b/src/protocol_serde/shape_invalid_authorization_message_exception.rs
@@ -3,7 +3,10 @@
 pub fn de_invalid_authorization_message_exception_xml_err(
     inp: &[u8],
     mut builder: crate::types::error::builders::InvalidAuthorizationMessageExceptionBuilder,
-) -> std::result::Result<crate::types::error::builders::InvalidAuthorizationMessageExceptionBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
+) -> std::result::Result<
+    crate::types::error::builders::InvalidAuthorizationMessageExceptionBuilder,
+    ::aws_smithy_xml::decode::XmlDecodeError,
+> {
     if inp.is_empty() {
         return Ok(builder);
     }
diff --git a/src/protocol_serde/shape_invalid_identity_token_exception.rs b/src/protocol_serde/shape_invalid_identity_token_exception.rs
--- a/src/protocol_serde/shape_invalid_identity_token_exception.rs
+++ b/src/protocol_serde/shape_invalid_identity_token_exception.rs
@@ -3,7 +3,10 @@
 pub fn de_invalid_identity_token_exception_xml_err(
     inp: &[u8],
     mut builder: crate::types::error::builders::InvalidIdentityTokenExceptionBuilder,
-) -> std::result::Result<crate::types::error::builders::InvalidIdentityTokenExceptionBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
+) -> std::result::Result<
+    crate::types::error::builders::InvalidIdentityTokenExceptionBuilder,
+    ::aws_smithy_xml::decode::XmlDecodeError,
+> {
     if inp.is_empty() {
         return Ok(builder);
     }
diff --git a/src/protocol_serde/shape_jwt_payload_size_exceeded_exception.rs b/src/protocol_serde/shape_jwt_payload_size_exceeded_exception.rs
--- a/src/protocol_serde/shape_jwt_payload_size_exceeded_exception.rs
+++ b/src/protocol_serde/shape_jwt_payload_size_exceeded_exception.rs
@@ -3,7 +3,10 @@
 pub fn de_jwt_payload_size_exceeded_exception_xml_err(
     inp: &[u8],
     mut builder: crate::types::error::builders::JwtPayloadSizeExceededExceptionBuilder,
-) -> std::result::Result<crate::types::error::builders::JwtPayloadSizeExceededExceptionBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
+) -> std::result::Result<
+    crate::types::error::builders::JwtPayloadSizeExceededExceptionBuilder,
+    ::aws_smithy_xml::decode::XmlDecodeError,
+> {
     if inp.is_empty() {
         return Ok(builder);
     }
diff --git a/src/protocol_serde/shape_malformed_policy_document_exception.rs b/src/protocol_serde/shape_malformed_policy_document_exception.rs
--- a/src/protocol_serde/shape_malformed_policy_document_exception.rs
+++ b/src/protocol_serde/shape_malformed_policy_document_exception.rs
@@ -3,7 +3,10 @@
 pub fn de_malformed_policy_document_exception_xml_err(
     inp: &[u8],
     mut builder: crate::types::error::builders::MalformedPolicyDocumentExceptionBuilder,
-) -> std::result::Result<crate::types::error::builders::MalformedPolicyDocumentExceptionBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
+) -> std::result::Result<
+    crate::types::error::builders::MalformedPolicyDocumentExceptionBuilder,
+    ::aws_smithy_xml::decode::XmlDecodeError,
+> {
     if inp.is_empty() {
         return Ok(builder);
     }
diff --git a/src/protocol_serde/shape_outbound_web_identity_federation_disabled_exception.rs b/src/protocol_serde/shape_outbound_web_identity_federation_disabled_exception.rs
--- a/src/protocol_serde/shape_outbound_web_identity_federation_disabled_exception.rs
+++ b/src/protocol_serde/shape_outbound_web_identity_federation_disabled_exception.rs
@@ -3,8 +3,10 @@
 pub fn de_outbound_web_identity_federation_disabled_exception_xml_err(
     inp: &[u8],
     mut builder: crate::types::error::builders::OutboundWebIdentityFederationDisabledExceptionBuilder,
-) -> std::result::Result<crate::types::error::builders::OutboundWebIdentityFederationDisabledExceptionBuilder, ::aws_smithy_xml::decode::XmlDecodeError>
-{
+) -> std::result::Result<
+    crate::types::error::builders::OutboundWebIdentityFederationDisabledExceptionBuilder,
+    ::aws_smithy_xml::decode::XmlDecodeError,
+> {
     if inp.is_empty() {
         return Ok(builder);
     }
diff --git a/src/protocol_serde/shape_packed_policy_too_large_exception.rs b/src/protocol_serde/shape_packed_policy_too_large_exception.rs
--- a/src/protocol_serde/shape_packed_policy_too_large_exception.rs
+++ b/src/protocol_serde/shape_packed_policy_too_large_exception.rs
@@ -3,7 +3,10 @@
 pub fn de_packed_policy_too_large_exception_xml_err(
     inp: &[u8],
     mut builder: crate::types::error::builders::PackedPolicyTooLargeExceptionBuilder,
-) -> std::result::Result<crate::types::error::builders::PackedPolicyTooLargeExceptionBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
+) -> std::result::Result<
+    crate::types::error::builders::PackedPolicyTooLargeExceptionBuilder,
+    ::aws_smithy_xml::decode::XmlDecodeError,
+> {
     if inp.is_empty() {
         return Ok(builder);
     }
diff --git a/src/protocol_serde/shape_region_disabled_exception.rs b/src/protocol_serde/shape_region_disabled_exception.rs
--- a/src/protocol_serde/shape_region_disabled_exception.rs
+++ b/src/protocol_serde/shape_region_disabled_exception.rs
@@ -3,7 +3,10 @@
 pub fn de_region_disabled_exception_xml_err(
     inp: &[u8],
     mut builder: crate::types::error::builders::RegionDisabledExceptionBuilder,
-) -> std::result::Result<crate::types::error::builders::RegionDisabledExceptionBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
+) -> std::result::Result<
+    crate::types::error::builders::RegionDisabledExceptionBuilder,
+    ::aws_smithy_xml::decode::XmlDecodeError,
+> {
     if inp.is_empty() {
         return Ok(builder);
     }
diff --git a/src/protocol_serde/shape_session_duration_escalation_exception.rs b/src/protocol_serde/shape_session_duration_escalation_exception.rs
--- a/src/protocol_serde/shape_session_duration_escalation_exception.rs
+++ b/src/protocol_serde/shape_session_duration_escalation_exception.rs
@@ -3,7 +3,10 @@
 pub fn de_session_duration_escalation_exception_xml_err(
     inp: &[u8],
     mut builder: crate::types::error::builders::SessionDurationEscalationExceptionBuilder,
-) -> std::result::Result<crate::types::error::builders::SessionDurationEscalationExceptionBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
+) -> std::result::Result<
+    crate::types::error::builders::SessionDurationEscalationExceptionBuilder,
+    ::aws_smithy_xml::decode::XmlDecodeError,
+> {
     if inp.is_empty() {
         return Ok(builder);
     }
diff --git a/src/sdk_feature_tracker.rs b/src/sdk_feature_tracker.rs
--- a/src/sdk_feature_tracker.rs
+++ b/src/sdk_feature_tracker.rs
@@ -73,7 +73,8 @@
             _runtime_components: &RuntimeComponents,
             cfg: &mut ConfigBag,
         ) -> Result<(), BoxError> {
-            cfg.interceptor_state().store_append::<SmithySdkFeature>(SmithySdkFeature::Paginator);
+            cfg.interceptor_state()
+                .store_append::<SmithySdkFeature>(SmithySdkFeature::Paginator);
             Ok(())
         }
     }
@@ -131,7 +132,8 @@
             _runtime_components: &RuntimeComponents,
             cfg: &mut ConfigBag,
         ) -> Result<(), BoxError> {
-            cfg.interceptor_state().store_append::<SmithySdkFeature>(SmithySdkFeature::Waiter);
+            cfg.interceptor_state()
+                .store_append::<SmithySdkFeature>(SmithySdkFeature::Waiter);
             Ok(())
         }
     }
diff --git a/src/serde_util.rs b/src/serde_util.rs
--- a/src/serde_util.rs
+++ b/src/serde_util.rs
@@ -1,4 +1,9 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity"
+))]
 pub(crate) fn assumed_role_user_correct_errors(
     mut builder: crate::types::builders::AssumedRoleUserBuilder,
 ) -> crate::types::builders::AssumedRoleUserBuilder {
@@ -11,7 +16,18 @@
     builder
 }

-pub(crate) fn credentials_correct_errors(mut builder: crate::types::builders::CredentialsBuilder) -> crate::types::builders::CredentialsBuilder {
+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_assume_root",
+    feature = "op_get_delegated_access_token",
+    feature = "op_get_federation_token",
+    feature = "op_get_session_token"
+))]
+pub(crate) fn credentials_correct_errors(
+    mut builder: crate::types::builders::CredentialsBuilder,
+) -> crate::types::builders::CredentialsBuilder {
     if builder.access_key_id.is_none() {
         builder.access_key_id = Some(Default::default())
     }
@@ -27,6 +43,7 @@
     builder
 }

+#[cfg(feature = "op_get_federation_token")]
 pub(crate) fn federated_user_correct_errors(
     mut builder: crate::types::builders::FederatedUserBuilder,
 ) -> crate::types::builders::FederatedUserBuilder {
diff --git a/src/serialization_settings.rs b/src/serialization_settings.rs
--- a/src/serialization_settings.rs
+++ b/src/serialization_settings.rs
@@ -57,7 +57,8 @@

     /// Returns true if the given default header name should be serialized
     fn include_header(&self, header: &HeaderName) -> bool {
-        (!self.omit_default_content_length || header != CONTENT_LENGTH) && (!self.omit_default_content_type || header != CONTENT_TYPE)
+        (!self.omit_default_content_length || header != CONTENT_LENGTH)
+            && (!self.omit_default_content_type || header != CONTENT_TYPE)
     }

     /// Sets a default header on the given request builder if it should be serialized
diff --git a/src/types.rs b/src/types.rs
--- a/src/types.rs
+++ b/src/types.rs
@@ -1,26 +1,82 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_assume_root",
+    feature = "op_get_delegated_access_token",
+    feature = "op_get_federation_token",
+    feature = "op_get_session_token"
+))]
 pub use crate::types::_credentials::Credentials;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity"
+))]
 pub use crate::types::_assumed_role_user::AssumedRoleUser;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_assume_root",
+    feature = "op_get_federation_token"
+))]
 pub use crate::types::_policy_descriptor_type::PolicyDescriptorType;

+#[cfg(feature = "op_get_federation_token")]
 pub use crate::types::_federated_user::FederatedUser;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_get_federation_token",
+    feature = "op_get_web_identity_token"
+))]
 pub use crate::types::_tag::Tag;

+#[cfg(feature = "op_assume_role")]
 pub use crate::types::_provided_context::ProvidedContext;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity"
+))]
 mod _assumed_role_user;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_assume_root",
+    feature = "op_get_delegated_access_token",
+    feature = "op_get_federation_token",
+    feature = "op_get_session_token"
+))]
 mod _credentials;

+#[cfg(feature = "op_get_federation_token")]
 mod _federated_user;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_assume_root",
+    feature = "op_get_federation_token"
+))]
 mod _policy_descriptor_type;

+#[cfg(feature = "op_assume_role")]
 mod _provided_context;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_get_federation_token",
+    feature = "op_get_web_identity_token"
+))]
 mod _tag;

 /// Builders
diff --git a/src/types/_assumed_role_user.rs b/src/types/_assumed_role_user.rs
--- a/src/types/_assumed_role_user.rs
+++ b/src/types/_assumed_role_user.rs
@@ -70,7 +70,9 @@
     /// This method will fail if any of the following fields are not set:
     /// - [`assumed_role_id`](crate::types::builders::AssumedRoleUserBuilder::assumed_role_id)
     /// - [`arn`](crate::types::builders::AssumedRoleUserBuilder::arn)
-    pub fn build(self) -> ::std::result::Result<crate::types::AssumedRoleUser, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<crate::types::AssumedRoleUser, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(crate::types::AssumedRoleUser {
             assumed_role_id: self.assumed_role_id.ok_or_else(|| {
                 ::aws_smithy_types::error::operation::BuildError::missing_field(
diff --git a/src/types/_credentials.rs b/src/types/_credentials.rs
--- a/src/types/_credentials.rs
+++ b/src/types/_credentials.rs
@@ -127,7 +127,9 @@
     /// - [`secret_access_key`](crate::types::builders::CredentialsBuilder::secret_access_key)
     /// - [`session_token`](crate::types::builders::CredentialsBuilder::session_token)
     /// - [`expiration`](crate::types::builders::CredentialsBuilder::expiration)
-    pub fn build(self) -> ::std::result::Result<crate::types::Credentials, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<crate::types::Credentials, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(crate::types::Credentials {
             access_key_id: self.access_key_id.ok_or_else(|| {
                 ::aws_smithy_types::error::operation::BuildError::missing_field(
diff --git a/src/types/_federated_user.rs b/src/types/_federated_user.rs
--- a/src/types/_federated_user.rs
+++ b/src/types/_federated_user.rs
@@ -70,7 +70,9 @@
     /// This method will fail if any of the following fields are not set:
     /// - [`federated_user_id`](crate::types::builders::FederatedUserBuilder::federated_user_id)
     /// - [`arn`](crate::types::builders::FederatedUserBuilder::arn)
-    pub fn build(self) -> ::std::result::Result<crate::types::FederatedUser, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<crate::types::FederatedUser, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(crate::types::FederatedUser {
             federated_user_id: self.federated_user_id.ok_or_else(|| {
                 ::aws_smithy_types::error::operation::BuildError::missing_field(
diff --git a/src/types/_tag.rs b/src/types/_tag.rs
--- a/src/types/_tag.rs
+++ b/src/types/_tag.rs
@@ -83,7 +83,10 @@
     pub fn build(self) -> ::std::result::Result<crate::types::Tag, ::aws_smithy_types::error::operation::BuildError> {
         ::std::result::Result::Ok(crate::types::Tag {
             key: self.key.ok_or_else(|| {
-                ::aws_smithy_types::error::operation::BuildError::missing_field("key", "key was not specified but it is required when building Tag")
+                ::aws_smithy_types::error::operation::BuildError::missing_field(
+                    "key",
+                    "key was not specified but it is required when building Tag",
+                )
             })?,
             value: self.value.ok_or_else(|| {
                 ::aws_smithy_types::error::operation::BuildError::missing_field(
diff --git a/src/types/builders.rs b/src/types/builders.rs
--- a/src/types/builders.rs
+++ b/src/types/builders.rs
@@ -1,12 +1,40 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_assume_root",
+    feature = "op_get_delegated_access_token",
+    feature = "op_get_federation_token",
+    feature = "op_get_session_token"
+))]
 pub use crate::types::_credentials::CredentialsBuilder;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity"
+))]
 pub use crate::types::_assumed_role_user::AssumedRoleUserBuilder;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_assume_root",
+    feature = "op_get_federation_token"
+))]
 pub use crate::types::_policy_descriptor_type::PolicyDescriptorTypeBuilder;

+#[cfg(feature = "op_get_federation_token")]
 pub use crate::types::_federated_user::FederatedUserBuilder;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_get_federation_token",
+    feature = "op_get_web_identity_token"
+))]
 pub use crate::types::_tag::TagBuilder;

+#[cfg(feature = "op_assume_role")]
 pub use crate::types::_provided_context::ProvidedContextBuilder;
diff --git a/src/types/error.rs b/src/types/error.rs
--- a/src/types/error.rs
+++ b/src/types/error.rs
@@ -1,50 +1,122 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_assume_root"
+))]
 pub use crate::types::error::_expired_token_exception::ExpiredTokenException;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_get_federation_token"
+))]
 pub use crate::types::error::_malformed_policy_document_exception::MalformedPolicyDocumentException;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_get_delegated_access_token",
+    feature = "op_get_federation_token"
+))]
 pub use crate::types::error::_packed_policy_too_large_exception::PackedPolicyTooLargeException;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_assume_root",
+    feature = "op_get_delegated_access_token",
+    feature = "op_get_federation_token",
+    feature = "op_get_session_token"
+))]
 pub use crate::types::error::_region_disabled_exception::RegionDisabledException;

+#[cfg(any(feature = "op_assume_role_with_saml", feature = "op_assume_role_with_web_identity"))]
 pub use crate::types::error::_idp_rejected_claim_exception::IdpRejectedClaimException;

+#[cfg(any(feature = "op_assume_role_with_saml", feature = "op_assume_role_with_web_identity"))]
 pub use crate::types::error::_invalid_identity_token_exception::InvalidIdentityTokenException;

+#[cfg(feature = "op_assume_role_with_web_identity")]
 pub use crate::types::error::_idp_communication_error_exception::IdpCommunicationErrorException;

+#[cfg(feature = "op_decode_authorization_message")]
 pub use crate::types::error::_invalid_authorization_message_exception::InvalidAuthorizationMessageException;

+#[cfg(feature = "op_get_delegated_access_token")]
 pub use crate::types::error::_expired_trade_in_token_exception::ExpiredTradeInTokenException;

+#[cfg(feature = "op_get_web_identity_token")]
 pub use crate::types::error::_jwt_payload_size_exceeded_exception::JwtPayloadSizeExceededException;

+#[cfg(feature = "op_get_web_identity_token")]
 pub use crate::types::error::_outbound_web_identity_federation_disabled_exception::OutboundWebIdentityFederationDisabledException;

+#[cfg(feature = "op_get_web_identity_token")]
 pub use crate::types::error::_session_duration_escalation_exception::SessionDurationEscalationException;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_assume_root"
+))]
 mod _expired_token_exception;

+#[cfg(feature = "op_get_delegated_access_token")]
 mod _expired_trade_in_token_exception;

+#[cfg(feature = "op_assume_role_with_web_identity")]
 mod _idp_communication_error_exception;

+#[cfg(any(feature = "op_assume_role_with_saml", feature = "op_assume_role_with_web_identity"))]
 mod _idp_rejected_claim_exception;

+#[cfg(feature = "op_decode_authorization_message")]
 mod _invalid_authorization_message_exception;

+#[cfg(any(feature = "op_assume_role_with_saml", feature = "op_assume_role_with_web_identity"))]
 mod _invalid_identity_token_exception;

+#[cfg(feature = "op_get_web_identity_token")]
 mod _jwt_payload_size_exceeded_exception;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_get_federation_token"
+))]
 mod _malformed_policy_document_exception;

+#[cfg(feature = "op_get_web_identity_token")]
 mod _outbound_web_identity_federation_disabled_exception;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_get_delegated_access_token",
+    feature = "op_get_federation_token"
+))]
 mod _packed_policy_too_large_exception;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_assume_root",
+    feature = "op_get_delegated_access_token",
+    feature = "op_get_federation_token",
+    feature = "op_get_session_token"
+))]
 mod _region_disabled_exception;

+#[cfg(feature = "op_get_web_identity_token")]
 mod _session_duration_escalation_exception;

 /// Builders
diff --git a/src/types/error/builders.rs b/src/types/error/builders.rs
--- a/src/types/error/builders.rs
+++ b/src/types/error/builders.rs
@@ -1,24 +1,60 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_assume_root"
+))]
 pub use crate::types::error::_expired_token_exception::ExpiredTokenExceptionBuilder;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_get_federation_token"
+))]
 pub use crate::types::error::_malformed_policy_document_exception::MalformedPolicyDocumentExceptionBuilder;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_get_delegated_access_token",
+    feature = "op_get_federation_token"
+))]
 pub use crate::types::error::_packed_policy_too_large_exception::PackedPolicyTooLargeExceptionBuilder;

+#[cfg(any(
+    feature = "op_assume_role",
+    feature = "op_assume_role_with_saml",
+    feature = "op_assume_role_with_web_identity",
+    feature = "op_assume_root",
+    feature = "op_get_delegated_access_token",
+    feature = "op_get_federation_token",
+    feature = "op_get_session_token"
+))]
 pub use crate::types::error::_region_disabled_exception::RegionDisabledExceptionBuilder;

+#[cfg(any(feature = "op_assume_role_with_saml", feature = "op_assume_role_with_web_identity"))]
 pub use crate::types::error::_idp_rejected_claim_exception::IdpRejectedClaimExceptionBuilder;

+#[cfg(any(feature = "op_assume_role_with_saml", feature = "op_assume_role_with_web_identity"))]
 pub use crate::types::error::_invalid_identity_token_exception::InvalidIdentityTokenExceptionBuilder;

+#[cfg(feature = "op_assume_role_with_web_identity")]
 pub use crate::types::error::_idp_communication_error_exception::IdpCommunicationErrorExceptionBuilder;

+#[cfg(feature = "op_decode_authorization_message")]
 pub use crate::types::error::_invalid_authorization_message_exception::InvalidAuthorizationMessageExceptionBuilder;

+#[cfg(feature = "op_get_delegated_access_token")]
 pub use crate::types::error::_expired_trade_in_token_exception::ExpiredTradeInTokenExceptionBuilder;

+#[cfg(feature = "op_get_web_identity_token")]
 pub use crate::types::error::_jwt_payload_size_exceeded_exception::JwtPayloadSizeExceededExceptionBuilder;

+#[cfg(feature = "op_get_web_identity_token")]
 pub use crate::types::error::_outbound_web_identity_federation_disabled_exception::OutboundWebIdentityFederationDisabledExceptionBuilder;

+#[cfg(feature = "op_get_web_identity_token")]
 pub use crate::types::error::_session_duration_escalation_exception::SessionDurationEscalationExceptionBuilder;
```


## Customizations

- Add one non-default Cargo feature and matching cfg gates for each model operation.
- Rename the Cargo package/library from manifest data.
- Remove upstream `tests/` from generated output.

## Reproduction

```text
cargo run -p aws-sdk-modularizer -- --manifest services-manifest.json
```
