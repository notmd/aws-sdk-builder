# AWS SDK modularizer diff

- repository: `https://github.com/awslabs/aws-sdk-rust`
- revision: `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db`
- service: `sdk/sso`
- model: `aws-models/sso.json`
- package: `aws-sdk-sso`
- library: `aws_sdk_sso`
- tests exclusion: `tests/**` is excluded from the comparison.

## Operations

- `op_get_role_credentials`: `GetRoleCredentials`
- `op_list_account_roles`: `ListAccountRoles`
- `op_list_accounts`: `ListAccounts`
- `op_logout`: `Logout`

## Changed files

- `Cargo.toml`
- `src/client.rs`
- `src/client/customize.rs`
- `src/client/customize/internal.rs`
- `src/client/get_role_credentials.rs`
- `src/client/list_account_roles.rs`
- `src/client/list_accounts.rs`
- `src/config.rs`
- `src/config/auth.rs`
- `src/config/endpoint.rs`
- `src/endpoint_lib.rs`
- `src/endpoint_lib/bdd_interpreter.rs`
- `src/endpoint_lib/diagnostic.rs`
- `src/endpoint_lib/host.rs`
- `src/endpoint_lib/partition.rs`
- `src/error.rs`
- `src/error/sealed_unhandled.rs`
- `src/error_meta.rs`
- `src/json_errors.rs`
- `src/lens.rs`
- `src/meta.rs`
- `src/observability_feature.rs`
- `src/operation.rs`
- `src/operation/get_role_credentials.rs`
- `src/operation/get_role_credentials/_get_role_credentials_input.rs`
- `src/operation/get_role_credentials/_get_role_credentials_output.rs`
- `src/operation/get_role_credentials/builders.rs`
- `src/operation/list_account_roles.rs`
- `src/operation/list_account_roles/_list_account_roles_input.rs`
- `src/operation/list_account_roles/_list_account_roles_output.rs`
- `src/operation/list_account_roles/builders.rs`
- `src/operation/list_account_roles/paginator.rs`
- `src/operation/list_accounts.rs`
- `src/operation/list_accounts/_list_accounts_input.rs`
- `src/operation/list_accounts/_list_accounts_output.rs`
- `src/operation/list_accounts/builders.rs`
- `src/operation/list_accounts/paginator.rs`
- `src/operation/logout.rs`
- `src/operation/logout/_logout_input.rs`
- `src/operation/logout/_logout_output.rs`
- `src/operation/logout/builders.rs`
- `src/protocol_serde.rs`
- `src/protocol_serde/shape_account_info.rs`
- `src/protocol_serde/shape_account_list_type.rs`
- `src/protocol_serde/shape_get_role_credentials.rs`
- `src/protocol_serde/shape_invalid_request_exception.rs`
- `src/protocol_serde/shape_list_account_roles.rs`
- `src/protocol_serde/shape_list_accounts.rs`
- `src/protocol_serde/shape_logout.rs`
- `src/protocol_serde/shape_resource_not_found_exception.rs`
- `src/protocol_serde/shape_role_credentials.rs`
- `src/protocol_serde/shape_role_info.rs`
- `src/protocol_serde/shape_role_list_type.rs`
- `src/protocol_serde/shape_too_many_requests_exception.rs`
- `src/protocol_serde/shape_unauthorized_exception.rs`
- `src/sdk_feature_tracker.rs`
- `src/serialization_settings.rs`
- `src/types.rs`
- `src/types/_account_info.rs`
- `src/types/_role_credentials.rs`
- `src/types/_role_info.rs`
- `src/types/builders.rs`
- `src/types/error.rs`
- `src/types/error/_invalid_request_exception.rs`
- `src/types/error/_resource_not_found_exception.rs`
- `src/types/error/_too_many_requests_exception.rs`
- `src/types/error/_unauthorized_exception.rs`
- `src/types/error/builders.rs`

## File diffs

### `Cargo.toml`

```diff
diff --git a/Cargo.toml b/Cargo.toml
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -19,50 +19,39 @@
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

 [dependencies.aws-types]
-path = "../aws-types"
 version = "1.5.0"

 [dependencies.bytes]
@@ -84,7 +73,6 @@
 [dependencies.tracing]
 version = "0.1"
 [dev-dependencies.aws-credential-types]
-path = "../aws-credential-types"
 features = ["test-util"]
 version = "1.3.0"

@@ -103,3 +91,10 @@
 test-util = ["aws-credential-types/test-util", "aws-smithy-runtime/legacy-test-util", "aws-smithy-runtime/test-util"]
 gated-tests = []
 default = ["rustls", "default-https-client", "rt-tokio"]
+op_get_role_credentials = []
+op_list_account_roles = []
+op_list_accounts = []
+op_logout = []
+
+[lib]
+name = "aws_sdk_sso"
```

### `src/client.rs`

```diff
diff --git a/src/client.rs b/src/client.rs
--- a/src/client.rs
+++ b/src/client.rs
@@ -64,7 +64,9 @@
         &self.handle.conf
     }

-    fn validate_config(handle: &Handle) -> ::std::result::Result<(), ::aws_smithy_runtime_api::box_error::BoxError> {
+    fn validate_config(
+        handle: &Handle,
+    ) -> ::std::result::Result<(), ::aws_smithy_runtime_api::box_error::BoxError> {
         let mut cfg = ::aws_smithy_types::config_bag::ConfigBag::base();
         handle
             .runtime_plugins
@@ -117,10 +119,14 @@
 /// ```
 pub mod customize;

+#[cfg(feature = "op_get_role_credentials")]
 mod get_role_credentials;

+#[cfg(feature = "op_list_account_roles")]
 mod list_account_roles;

+#[cfg(feature = "op_list_accounts")]
 mod list_accounts;

+#[cfg(feature = "op_logout")]
 mod logout;
```

### `src/client/customize.rs`

```diff
diff --git a/src/client/customize.rs b/src/client/customize.rs
--- a/src/client/customize.rs
+++ b/src/client/customize.rs
@@ -24,7 +24,10 @@
         }
     }

-    pub(crate) fn execute<U>(self, f: impl ::std::ops::FnOnce(B, crate::config::Builder) -> U) -> U {
+    pub(crate) fn execute<U>(
+        self,
+        f: impl ::std::ops::FnOnce(B, crate::config::Builder) -> U,
+    ) -> U {
         let mut config_override = self.config_override.unwrap_or_default();
         self.interceptors.into_iter().for_each(|interceptor| {
             config_override.push_interceptor(interceptor);
@@ -41,17 +44,27 @@
     /// `map_request`, and `mutate_request` (the last two are implemented via interceptors under the hood).
     /// The order in which those user-specified operation interceptors are invoked should not be relied upon
     /// as it is an implementation detail.
-    pub fn interceptor(mut self, interceptor: impl ::aws_smithy_runtime_api::client::interceptors::Intercept + 'static) -> Self {
-        self.interceptors
-            .push(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::new(interceptor));
+    pub fn interceptor(
+        mut self,
+        interceptor: impl ::aws_smithy_runtime_api::client::interceptors::Intercept + 'static,
+    ) -> Self {
+        self.interceptors.push(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::new(interceptor),
+        );
         self
     }

     /// Adds a runtime plugin.
     #[allow(unused)]
-    pub(crate) fn runtime_plugin(mut self, runtime_plugin: impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin + 'static) -> Self {
-        self.runtime_plugins
-            .push(::aws_smithy_runtime_api::client::runtime_plugin::SharedRuntimePlugin::new(runtime_plugin));
+    pub(crate) fn runtime_plugin(
+        mut self,
+        runtime_plugin: impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin + 'static,
+    ) -> Self {
+        self.runtime_plugins.push(
+            ::aws_smithy_runtime_api::client::runtime_plugin::SharedRuntimePlugin::new(
+                runtime_plugin,
+            ),
+        );
         self
     }

@@ -60,16 +73,19 @@
     where
         F: ::std::ops::Fn(
                 ::aws_smithy_runtime_api::client::orchestrator::HttpRequest,
-            ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, MapE>
-            + ::std::marker::Send
+            ) -> ::std::result::Result<
+                ::aws_smithy_runtime_api::client::orchestrator::HttpRequest,
+                MapE,
+            > + ::std::marker::Send
             + ::std::marker::Sync
             + 'static,
         MapE: ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static,
     {
-        self.interceptors
-            .push(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::new(
+        self.interceptors.push(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::new(
                 ::aws_smithy_runtime::client::interceptors::MapRequestInterceptor::new(f),
-            ));
+            ),
+        );
         self
     }

@@ -76,12 +92,16 @@
     /// Convenience for `map_request` where infallible direct mutation of request is acceptable.
     pub fn mutate_request<F>(mut self, f: F) -> Self
     where
-        F: ::std::ops::Fn(&mut ::aws_smithy_runtime_api::client::orchestrator::HttpRequest) + ::std::marker::Send + ::std::marker::Sync + 'static,
+        F: ::std::ops::Fn(&mut ::aws_smithy_runtime_api::client::orchestrator::HttpRequest)
+            + ::std::marker::Send
+            + ::std::marker::Sync
+            + 'static,
     {
-        self.interceptors
-            .push(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::new(
+        self.interceptors.push(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::new(
                 ::aws_smithy_runtime::client::interceptors::MutateRequestInterceptor::new(f),
-            ));
+            ),
+        );
         self
     }

@@ -96,7 +116,10 @@
     /// | field_1: None,     | field_1: Some(v2),     | field_1: Some(v2), |
     /// | field_2: Some(v1), | field_2: Some(v2),     | field_2: Some(v2), |
     /// | field_3: Some(v1), | field_3: None,         | field_3: Some(v1), |
-    pub fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
+    pub fn config_override(
+        mut self,
+        config_override: impl ::std::convert::Into<crate::config::Builder>,
+    ) -> Self {
         self.config_override = Some(config_override.into());
         self
     }
```

### `src/client/customize/internal.rs`

```diff
diff --git a/src/client/customize/internal.rs b/src/client/customize/internal.rs
--- a/src/client/customize/internal.rs
+++ b/src/client/customize/internal.rs
@@ -1,8 +1,14 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-pub type BoxFuture<T> = ::std::pin::Pin<::std::boxed::Box<dyn ::std::future::Future<Output = T> + ::std::marker::Send>>;
+pub type BoxFuture<T> =
+    ::std::pin::Pin<::std::boxed::Box<dyn ::std::future::Future<Output = T> + ::std::marker::Send>>;

-pub type SendResult<T, E> =
-    ::std::result::Result<T, ::aws_smithy_runtime_api::client::result::SdkError<E, ::aws_smithy_runtime_api::client::orchestrator::HttpResponse>>;
+pub type SendResult<T, E> = ::std::result::Result<
+    T,
+    ::aws_smithy_runtime_api::client::result::SdkError<
+        E,
+        ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
+    >,
+>;

 pub trait CustomizableSend<T, E>: ::std::marker::Send + ::std::marker::Sync {
     // Takes an owned `self` as the implementation will internally call methods that take `self`.
```

### `src/client/get_role_credentials.rs`

```diff
diff --git a/src/client/get_role_credentials.rs b/src/client/get_role_credentials.rs
--- a/src/client/get_role_credentials.rs
+++ b/src/client/get_role_credentials.rs
@@ -9,7 +9,11 @@
     /// - On success, responds with [`GetRoleCredentialsOutput`](crate::operation::get_role_credentials::GetRoleCredentialsOutput) with field(s):
     ///   - [`role_credentials(Option<RoleCredentials>)`](crate::operation::get_role_credentials::GetRoleCredentialsOutput::role_credentials): <p>The credentials for the role that is assigned to the user.</p>
     /// - On failure, responds with [`SdkError<GetRoleCredentialsError>`](crate::operation::get_role_credentials::GetRoleCredentialsError)
-    pub fn get_role_credentials(&self) -> crate::operation::get_role_credentials::builders::GetRoleCredentialsFluentBuilder {
-        crate::operation::get_role_credentials::builders::GetRoleCredentialsFluentBuilder::new(self.handle.clone())
+    pub fn get_role_credentials(
+        &self,
+    ) -> crate::operation::get_role_credentials::builders::GetRoleCredentialsFluentBuilder {
+        crate::operation::get_role_credentials::builders::GetRoleCredentialsFluentBuilder::new(
+            self.handle.clone(),
+        )
     }
 }
```

### `src/client/list_account_roles.rs`

```diff
diff --git a/src/client/list_account_roles.rs b/src/client/list_account_roles.rs
--- a/src/client/list_account_roles.rs
+++ b/src/client/list_account_roles.rs
@@ -12,7 +12,11 @@
     ///   - [`next_token(Option<String>)`](crate::operation::list_account_roles::ListAccountRolesOutput::next_token): <p>The page token client that is used to retrieve the list of accounts.</p>
     ///   - [`role_list(Option<Vec::<RoleInfo>>)`](crate::operation::list_account_roles::ListAccountRolesOutput::role_list): <p>A paginated response with the list of roles and the next token if more results are available.</p>
     /// - On failure, responds with [`SdkError<ListAccountRolesError>`](crate::operation::list_account_roles::ListAccountRolesError)
-    pub fn list_account_roles(&self) -> crate::operation::list_account_roles::builders::ListAccountRolesFluentBuilder {
-        crate::operation::list_account_roles::builders::ListAccountRolesFluentBuilder::new(self.handle.clone())
+    pub fn list_account_roles(
+        &self,
+    ) -> crate::operation::list_account_roles::builders::ListAccountRolesFluentBuilder {
+        crate::operation::list_account_roles::builders::ListAccountRolesFluentBuilder::new(
+            self.handle.clone(),
+        )
     }
 }
```

### `src/client/list_accounts.rs`

```diff
diff --git a/src/client/list_accounts.rs b/src/client/list_accounts.rs
--- a/src/client/list_accounts.rs
+++ b/src/client/list_accounts.rs
@@ -11,7 +11,11 @@
     ///   - [`next_token(Option<String>)`](crate::operation::list_accounts::ListAccountsOutput::next_token): <p>The page token client that is used to retrieve the list of accounts.</p>
     ///   - [`account_list(Option<Vec::<AccountInfo>>)`](crate::operation::list_accounts::ListAccountsOutput::account_list): <p>A paginated response with the list of account information and the next token if more results are available.</p>
     /// - On failure, responds with [`SdkError<ListAccountsError>`](crate::operation::list_accounts::ListAccountsError)
-    pub fn list_accounts(&self) -> crate::operation::list_accounts::builders::ListAccountsFluentBuilder {
-        crate::operation::list_accounts::builders::ListAccountsFluentBuilder::new(self.handle.clone())
+    pub fn list_accounts(
+        &self,
+    ) -> crate::operation::list_accounts::builders::ListAccountsFluentBuilder {
+        crate::operation::list_accounts::builders::ListAccountsFluentBuilder::new(
+            self.handle.clone(),
+        )
     }
 }
```

### `src/config.rs`

```diff
diff --git a/src/config.rs b/src/config.rs
--- a/src/config.rs
+++ b/src/config.rs
@@ -47,8 +47,11 @@
         }
     }
     /// Return a reference to the stalled stream protection configuration contained in this config, if any.
-    pub fn stalled_stream_protection(&self) -> ::std::option::Option<&crate::config::StalledStreamProtectionConfig> {
-        self.config.load::<crate::config::StalledStreamProtectionConfig>()
+    pub fn stalled_stream_protection(
+        &self,
+    ) -> ::std::option::Option<&crate::config::StalledStreamProtectionConfig> {
+        self.config
+            .load::<crate::config::StalledStreamProtectionConfig>()
     }
     /// Return the [`SharedHttpClient`](crate::config::SharedHttpClient) to use when making requests, if any.
     pub fn http_client(&self) -> Option<crate::config::SharedHttpClient> {
@@ -55,21 +58,33 @@
         self.runtime_components.http_client()
     }
     /// Return the auth schemes configured on this service config
-    pub fn auth_schemes(&self) -> impl Iterator<Item = ::aws_smithy_runtime_api::client::auth::SharedAuthScheme> + '_ {
+    pub fn auth_schemes(
+        &self,
+    ) -> impl Iterator<Item = ::aws_smithy_runtime_api::client::auth::SharedAuthScheme> + '_ {
         self.runtime_components.auth_schemes()
     }

     /// Return the auth scheme resolver configured on this service config
-    pub fn auth_scheme_resolver(&self) -> ::std::option::Option<::aws_smithy_runtime_api::client::auth::SharedAuthSchemeOptionResolver> {
+    pub fn auth_scheme_resolver(
+        &self,
+    ) -> ::std::option::Option<::aws_smithy_runtime_api::client::auth::SharedAuthSchemeOptionResolver>
+    {
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
-    pub fn endpoint_resolver(&self) -> ::aws_smithy_runtime_api::client::endpoint::SharedEndpointResolver {
-        self.runtime_components.endpoint_resolver().expect("resolver defaulted if not set")
+    pub fn endpoint_resolver(
+        &self,
+    ) -> ::aws_smithy_runtime_api::client::endpoint::SharedEndpointResolver {
+        self.runtime_components
+            .endpoint_resolver()
+            .expect("resolver defaulted if not set")
     }
     /// Return a reference to the retry configuration contained in this config, if any.
     pub fn retry_config(&self) -> ::std::option::Option<&::aws_smithy_types::retry::RetryConfig> {
@@ -82,8 +97,11 @@
     }

     /// Return a reference to the timeout configuration contained in this config, if any.
-    pub fn timeout_config(&self) -> ::std::option::Option<&::aws_smithy_types::timeout::TimeoutConfig> {
-        self.config.load::<::aws_smithy_types::timeout::TimeoutConfig>()
+    pub fn timeout_config(
+        &self,
+    ) -> ::std::option::Option<&::aws_smithy_types::timeout::TimeoutConfig> {
+        self.config
+            .load::<::aws_smithy_types::timeout::TimeoutConfig>()
     }

     /// Returns a reference to the retry partition contained in this config, if any.
@@ -90,8 +108,11 @@
     ///
     /// WARNING: This method is unstable and may be removed at any time. Do not rely on this
     /// method for anything!
-    pub fn retry_partition(&self) -> ::std::option::Option<&::aws_smithy_runtime::client::retries::RetryPartition> {
-        self.config.load::<::aws_smithy_runtime::client::retries::RetryPartition>()
+    pub fn retry_partition(
+        &self,
+    ) -> ::std::option::Option<&::aws_smithy_runtime::client::retries::RetryPartition> {
+        self.config
+            .load::<::aws_smithy_runtime::client::retries::RetryPartition>()
     }
     /// Returns the configured identity cache for auth.
     pub fn identity_cache(&self) -> ::std::option::Option<crate::config::SharedIdentityCache> {
@@ -106,7 +127,11 @@
         self.runtime_components.time_source()
     }
     /// Returns retry classifiers currently registered by the user.
-    pub fn retry_classifiers(&self) -> impl Iterator<Item = ::aws_smithy_runtime_api::client::retries::classifiers::SharedRetryClassifier> + '_ {
+    pub fn retry_classifiers(
+        &self,
+    ) -> impl Iterator<
+        Item = ::aws_smithy_runtime_api::client::retries::classifiers::SharedRetryClassifier,
+    > + '_ {
         self.runtime_components.retry_classifiers()
     }
     /// Returns the name of the app that is using the client, if it was provided.
@@ -122,11 +147,15 @@
     /// being used with the client, rendered into the user agent as `lib/{name}/{version}`.
     /// Entries are returned in first-seen (insertion) order, matching the order they are
     /// rendered into the user agent.
-    pub fn framework_metadata(&self) -> ::std::vec::Vec<&::aws_types::sdk_ua_metadata::FrameworkMetadata> {
+    pub fn framework_metadata(
+        &self,
+    ) -> ::std::vec::Vec<&::aws_types::sdk_ua_metadata::FrameworkMetadata> {
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
@@ -133,8 +162,12 @@
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
@@ -193,18 +226,52 @@
     #[allow(unused)]
     pub(crate) fn from_config_bag(config_bag: &::aws_smithy_types::config_bag::ConfigBag) -> Self {
         let mut builder = Self::new();
-        builder.set_stalled_stream_protection(config_bag.load::<crate::config::StalledStreamProtectionConfig>().cloned());
-        builder.set_auth_scheme_preference(config_bag.load::<::aws_smithy_runtime_api::client::auth::AuthSchemePreference>().cloned());
-        builder.set_retry_config(config_bag.load::<::aws_smithy_types::retry::RetryConfig>().cloned());
-        builder.set_timeout_config(config_bag.load::<::aws_smithy_types::timeout::TimeoutConfig>().cloned());
-        builder.set_retry_partition(config_bag.load::<::aws_smithy_runtime::client::retries::RetryPartition>().cloned());
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
+        builder.set_retry_config(
+            config_bag
+                .load::<::aws_smithy_types::retry::RetryConfig>()
+                .cloned(),
+        );
+        builder.set_timeout_config(
+            config_bag
+                .load::<::aws_smithy_types::timeout::TimeoutConfig>()
+                .cloned(),
+        );
+        builder.set_retry_partition(
+            config_bag
+                .load::<::aws_smithy_runtime::client::retries::RetryPartition>()
+                .cloned(),
+        );
         builder.set_app_name(config_bag.load::<::aws_types::app_name::AppName>().cloned());
-        for framework_metadata in config_bag.load::<::aws_types::sdk_ua_metadata::FrameworkMetadata>() {
+        for framework_metadata in
+            config_bag.load::<::aws_types::sdk_ua_metadata::FrameworkMetadata>()
+        {
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
         builder.set_region(config_bag.load::<crate::config::Region>().cloned());
         builder
     }
@@ -220,7 +287,10 @@
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
@@ -253,8 +323,13 @@
     }
     /// Set the [`StalledStreamProtectionConfig`](crate::config::StalledStreamProtectionConfig)
     /// to configure protection for stalled streams.
-    pub fn stalled_stream_protection(mut self, stalled_stream_protection_config: crate::config::StalledStreamProtectionConfig) -> Self {
-        self.set_stalled_stream_protection(::std::option::Option::Some(stalled_stream_protection_config));
+    pub fn stalled_stream_protection(
+        mut self,
+        stalled_stream_protection_config: crate::config::StalledStreamProtectionConfig,
+    ) -> Self {
+        self.set_stalled_stream_protection(::std::option::Option::Some(
+            stalled_stream_protection_config,
+        ));
         self
     }
     /// Set the [`StalledStreamProtectionConfig`](crate::config::StalledStreamProtectionConfig)
@@ -261,7 +336,9 @@
     /// to configure protection for stalled streams.
     pub fn set_stalled_stream_protection(
         &mut self,
-        stalled_stream_protection_config: ::std::option::Option<crate::config::StalledStreamProtectionConfig>,
+        stalled_stream_protection_config: ::std::option::Option<
+            crate::config::StalledStreamProtectionConfig,
+        >,
     ) -> &mut Self {
         self.config.store_or_unset(stalled_stream_protection_config);
         self
@@ -296,7 +373,9 @@
     /// # }
     /// ```
     pub fn http_client(mut self, http_client: impl crate::config::HttpClient + 'static) -> Self {
-        self.set_http_client(::std::option::Option::Some(crate::config::IntoShared::into_shared(http_client)));
+        self.set_http_client(::std::option::Option::Some(
+            crate::config::IntoShared::into_shared(http_client),
+        ));
         self
     }

@@ -329,7 +408,10 @@
     /// # }
     /// # }
     /// ```
-    pub fn set_http_client(&mut self, http_client: Option<crate::config::SharedHttpClient>) -> &mut Self {
+    pub fn set_http_client(
+        &mut self,
+        http_client: Option<crate::config::SharedHttpClient>,
+    ) -> &mut Self {
         self.runtime_components.set_http_client(http_client);
         self
     }
@@ -441,7 +523,10 @@
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
@@ -476,7 +561,10 @@
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
@@ -485,9 +573,14 @@
     ///
     /// # Examples
     /// See an example for [`Self::auth_scheme_resolver`].
-    pub fn set_auth_scheme_resolver(&mut self, auth_scheme_resolver: impl crate::config::auth::ResolveAuthScheme + 'static) -> &mut Self {
+    pub fn set_auth_scheme_resolver(
+        &mut self,
+        auth_scheme_resolver: impl crate::config::auth::ResolveAuthScheme + 'static,
+    ) -> &mut Self {
         self.runtime_components
-            .set_auth_scheme_option_resolver(::std::option::Option::Some(auth_scheme_resolver.into_shared_resolver()));
+            .set_auth_scheme_option_resolver(::std::option::Option::Some(
+                auth_scheme_resolver.into_shared_resolver(),
+            ));
         self
     }

@@ -505,7 +598,9 @@
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
@@ -531,7 +626,9 @@

     pub fn auth_scheme_preference(
         mut self,
-        preference: impl ::std::convert::Into<::aws_smithy_runtime_api::client::auth::AuthSchemePreference>,
+        preference: impl ::std::convert::Into<
+            ::aws_smithy_runtime_api::client::auth::AuthSchemePreference,
+        >,
     ) -> Self {
         self.set_auth_scheme_preference(::std::option::Option::Some(preference.into()));
         self
@@ -560,7 +657,9 @@

     pub fn set_auth_scheme_preference(
         &mut self,
-        preference: ::std::option::Option<::aws_smithy_runtime_api::client::auth::AuthSchemePreference>,
+        preference: ::std::option::Option<
+            ::aws_smithy_runtime_api::client::auth::AuthSchemePreference,
+        >,
     ) -> &mut Self {
         self.config.store_or_unset(preference);
         self
@@ -592,8 +691,13 @@
     /// let config = aws_sdk_sso::Config::builder().endpoint_resolver(resolver).build();
     /// let client = aws_sdk_sso::Client::from_conf(config);
     /// ```
-    pub fn endpoint_resolver(mut self, endpoint_resolver: impl crate::config::endpoint::ResolveEndpoint + 'static) -> Self {
-        self.set_endpoint_resolver(::std::option::Option::Some(endpoint_resolver.into_shared_resolver()));
+    pub fn endpoint_resolver(
+        mut self,
+        endpoint_resolver: impl crate::config::endpoint::ResolveEndpoint + 'static,
+    ) -> Self {
+        self.set_endpoint_resolver(::std::option::Option::Some(
+            endpoint_resolver.into_shared_resolver(),
+        ));
         self
     }

@@ -605,9 +709,12 @@
     ///
     pub fn set_endpoint_resolver(
         &mut self,
-        endpoint_resolver: ::std::option::Option<::aws_smithy_runtime_api::client::endpoint::SharedEndpointResolver>,
+        endpoint_resolver: ::std::option::Option<
+            ::aws_smithy_runtime_api::client::endpoint::SharedEndpointResolver,
+        >,
     ) -> &mut Self {
-        self.runtime_components.set_endpoint_resolver(endpoint_resolver);
+        self.runtime_components
+            .set_endpoint_resolver(endpoint_resolver);
         self
     }
     /// Set the retry_config for the builder
@@ -650,7 +757,10 @@
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
@@ -674,7 +784,9 @@
     /// let config = Config::builder().sleep_impl(sleep_impl).build();
     /// ```
     pub fn sleep_impl(mut self, sleep_impl: impl crate::config::AsyncSleep + 'static) -> Self {
-        self.set_sleep_impl(Some(::aws_smithy_runtime_api::shared::IntoShared::into_shared(sleep_impl)));
+        self.set_sleep_impl(Some(
+            ::aws_smithy_runtime_api::shared::IntoShared::into_shared(sleep_impl),
+        ));
         self
     }

@@ -703,7 +815,10 @@
     /// set_never_ending_sleep_impl(&mut builder);
     /// let config = builder.build();
     /// ```
-    pub fn set_sleep_impl(&mut self, sleep_impl: ::std::option::Option<crate::config::SharedAsyncSleep>) -> &mut Self {
+    pub fn set_sleep_impl(
+        &mut self,
+        sleep_impl: ::std::option::Option<crate::config::SharedAsyncSleep>,
+    ) -> &mut Self {
         self.runtime_components.set_sleep_impl(sleep_impl);
         self
     }
@@ -721,7 +836,10 @@
     ///     .build();
     /// let config = Config::builder().timeout_config(timeout_config).build();
     /// ```
-    pub fn timeout_config(mut self, timeout_config: ::aws_smithy_types::timeout::TimeoutConfig) -> Self {
+    pub fn timeout_config(
+        mut self,
+        timeout_config: ::aws_smithy_types::timeout::TimeoutConfig,
+    ) -> Self {
         self.set_timeout_config(Some(timeout_config));
         self
     }
@@ -750,11 +868,19 @@
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

-        if let Some(base) = self.config.load::<::aws_smithy_types::timeout::TimeoutConfig>() {
+        if let Some(base) = self
+            .config
+            .load::<::aws_smithy_types::timeout::TimeoutConfig>()
+        {
             timeout_config.take_defaults_from(base);
         }
         self.config.store_put(timeout_config);
@@ -822,7 +948,10 @@
     ///     .retry_config(RetryConfig::adaptive())
     ///     .build();
     /// ```
-    pub fn retry_partition(mut self, retry_partition: ::aws_smithy_runtime::client::retries::RetryPartition) -> Self {
+    pub fn retry_partition(
+        mut self,
+        retry_partition: ::aws_smithy_runtime::client::retries::RetryPartition,
+    ) -> Self {
         self.set_retry_partition(Some(retry_partition));
         self
     }
@@ -829,7 +958,9 @@
     /// Like [`Self::retry_partition`], but takes a mutable reference to the builder and an optional `RetryPartition`
     pub fn set_retry_partition(
         &mut self,
-        retry_partition: ::std::option::Option<::aws_smithy_runtime::client::retries::RetryPartition>,
+        retry_partition: ::std::option::Option<
+            ::aws_smithy_runtime::client::retries::RetryPartition,
+        >,
     ) -> &mut Self {
         retry_partition.map(|r| self.config.store_put(r));
         self
@@ -875,7 +1006,10 @@
     /// let client = aws_sdk_sso::Client::from_conf(config);
     /// ```
     ///
-    pub fn identity_cache(mut self, identity_cache: impl crate::config::ResolveCachedIdentity + 'static) -> Self {
+    pub fn identity_cache(
+        mut self,
+        identity_cache: impl crate::config::ResolveCachedIdentity + 'static,
+    ) -> Self {
         self.set_identity_cache(identity_cache);
         self
     }
@@ -921,8 +1055,12 @@
     /// let client = aws_sdk_sso::Client::from_conf(config);
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
@@ -984,19 +1122,29 @@
     }

     /// Set [`SharedInterceptor`](crate::config::SharedInterceptor)s for the builder.
-    pub fn set_interceptors(&mut self, interceptors: impl IntoIterator<Item = crate::config::SharedInterceptor>) -> &mut Self {
-        self.runtime_components.set_interceptors(interceptors.into_iter());
+    pub fn set_interceptors(
+        &mut self,
+        interceptors: impl IntoIterator<Item = crate::config::SharedInterceptor>,
+    ) -> &mut Self {
+        self.runtime_components
+            .set_interceptors(interceptors.into_iter());
         self
     }
     /// Sets the time source used for this service
-    pub fn time_source(mut self, time_source: impl ::aws_smithy_async::time::TimeSource + 'static) -> Self {
-        self.set_time_source(::std::option::Option::Some(::aws_smithy_runtime_api::shared::IntoShared::into_shared(
-            time_source,
-        )));
+    pub fn time_source(
+        mut self,
+        time_source: impl ::aws_smithy_async::time::TimeSource + 'static,
+    ) -> Self {
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
@@ -1089,11 +1237,14 @@
     /// ```
     pub fn retry_classifier(
         mut self,
-        retry_classifier: impl ::aws_smithy_runtime_api::client::retries::classifiers::ClassifyRetry + 'static,
+        retry_classifier: impl ::aws_smithy_runtime_api::client::retries::classifiers::ClassifyRetry
+            + 'static,
     ) -> Self {
-        self.push_retry_classifier(::aws_smithy_runtime_api::client::retries::classifiers::SharedRetryClassifier::new(
-            retry_classifier,
-        ));
+        self.push_retry_classifier(
+            ::aws_smithy_runtime_api::client::retries::classifiers::SharedRetryClassifier::new(
+                retry_classifier,
+            ),
+        );
         self
     }

@@ -1102,7 +1253,8 @@
         &mut self,
         retry_classifier: ::aws_smithy_runtime_api::client::retries::classifiers::SharedRetryClassifier,
     ) -> &mut Self {
-        self.runtime_components.push_retry_classifier(retry_classifier);
+        self.runtime_components
+            .push_retry_classifier(retry_classifier);
         self
     }

@@ -1110,9 +1262,12 @@
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
@@ -1127,7 +1282,10 @@
     ///
     /// This _optional_ name is used to identify the application in the user agent that
     /// gets sent along with requests.
-    pub fn set_app_name(&mut self, app_name: ::std::option::Option<::aws_types::app_name::AppName>) -> &mut Self {
+    pub fn set_app_name(
+        &mut self,
+        app_name: ::std::option::Option<::aws_types::app_name::AppName>,
+    ) -> &mut Self {
         self.config.store_or_unset(app_name);
         self
     }
@@ -1142,7 +1300,10 @@
     /// Entries are de-duplicated on `(name, version)`, rendered in first-seen order, and
     /// the total number of unique entries included in the user agent is capped (currently
     /// at 10); additional entries beyond the cap are dropped with a warning.
-    pub fn framework_metadata(mut self, framework_metadata: ::aws_types::sdk_ua_metadata::FrameworkMetadata) -> Self {
+    pub fn framework_metadata(
+        mut self,
+        framework_metadata: ::aws_types::sdk_ua_metadata::FrameworkMetadata,
+    ) -> Self {
         self.push_framework_metadata(framework_metadata);
         self
     }
@@ -1153,7 +1314,10 @@
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
@@ -1160,7 +1324,10 @@
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
@@ -1191,7 +1358,8 @@
     /// will be prefixed onto this URL. To fully override the endpoint resolver, use
     /// [`Builder::endpoint_resolver`].
     pub fn set_endpoint_url(&mut self, endpoint_url: Option<::std::string::String>) -> &mut Self {
-        self.config.store_or_unset(endpoint_url.map(::aws_types::endpoint_config::EndpointUrl));
+        self.config
+            .store_or_unset(endpoint_url.map(::aws_types::endpoint_config::EndpointUrl));
         self
     }
     /// When true, use the dual-stack endpoint. If the configured endpoint does not support dual-stack, dispatching the request MAY return an error.
@@ -1201,7 +1369,8 @@
     }
     /// When true, use the dual-stack endpoint. If the configured endpoint does not support dual-stack, dispatching the request MAY return an error.
     pub fn set_use_dual_stack(&mut self, use_dual_stack: Option<bool>) -> &mut Self {
-        self.config.store_or_unset(use_dual_stack.map(::aws_types::endpoint_config::UseDualStack));
+        self.config
+            .store_or_unset(use_dual_stack.map(::aws_types::endpoint_config::UseDualStack));
         self
     }
     /// When true, send this request to the FIPS-compliant regional endpoint. If the configured endpoint does not have a FIPS compliant endpoint, dispatching the request will return an error.
@@ -1211,7 +1380,8 @@
     }
     /// When true, send this request to the FIPS-compliant regional endpoint. If the configured endpoint does not have a FIPS compliant endpoint, dispatching the request will return an error.
     pub fn set_use_fips(&mut self, use_fips: Option<bool>) -> &mut Self {
-        self.config.store_or_unset(use_fips.map(::aws_types::endpoint_config::UseFips));
+        self.config
+            .store_or_unset(use_fips.map(::aws_types::endpoint_config::UseFips));
         self
     }
     /// Sets the AWS region to use when making requests.
@@ -1225,24 +1395,36 @@
     ///     .region(Region::new("us-east-1"))
     ///     .build();
     /// ```
-    pub fn region(mut self, region: impl ::std::convert::Into<::std::option::Option<crate::config::Region>>) -> Self {
+    pub fn region(
+        mut self,
+        region: impl ::std::convert::Into<::std::option::Option<crate::config::Region>>,
+    ) -> Self {
         self.set_region(region.into());
         self
     }
     /// Sets the AWS region to use when making requests.
-    pub fn set_region(&mut self, region: ::std::option::Option<crate::config::Region>) -> &mut Self {
+    pub fn set_region(
+        &mut self,
+        region: ::std::option::Option<crate::config::Region>,
+    ) -> &mut Self {
         self.config.store_or_unset(region);
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
             self.runtime_components
                 .set_identity_resolver(::aws_runtime::auth::sigv4::SCHEME_ID, credentials_provider);
@@ -1314,7 +1496,10 @@
     /// let client = aws_sdk_sso::Client::from_conf(config);
     /// ```
     ///
-    pub fn set_behavior_version(&mut self, behavior_version: Option<crate::config::BehaviorVersion>) -> &mut Self {
+    pub fn set_behavior_version(
+        &mut self,
+        behavior_version: Option<crate::config::BehaviorVersion>,
+    ) -> &mut Self {
         self.behavior_version = behavior_version;
         self
     }
@@ -1328,13 +1513,19 @@
     }
     /// Adds a runtime plugin to the config.
     #[allow(unused)]
-    pub(crate) fn runtime_plugin(mut self, plugin: impl crate::config::RuntimePlugin + 'static) -> Self {
+    pub(crate) fn runtime_plugin(
+        mut self,
+        plugin: impl crate::config::RuntimePlugin + 'static,
+    ) -> Self {
         self.push_runtime_plugin(crate::config::SharedRuntimePlugin::new(plugin));
         self
     }
     /// Adds a runtime plugin to the config.
     #[allow(unused)]
-    pub(crate) fn push_runtime_plugin(&mut self, plugin: crate::config::SharedRuntimePlugin) -> &mut Self {
+    pub(crate) fn push_runtime_plugin(
+        &mut self,
+        plugin: crate::config::SharedRuntimePlugin,
+    ) -> &mut Self {
         self.runtime_plugins.push(plugin);
         self
     }
@@ -1342,14 +1533,20 @@
     #[allow(unused_mut)]
     /// Apply test defaults to the builder. NOTE: Consider migrating to use `apply_test_defaults_v2` instead.
     pub fn apply_test_defaults(&mut self) -> &mut Self {
-        self.set_time_source(::std::option::Option::Some(::aws_smithy_async::time::SharedTimeSource::new(
-            ::aws_smithy_async::time::StaticTimeSource::new(::std::time::UNIX_EPOCH + ::std::time::Duration::from_secs(1234567890)),
-        )));
-        self.config.store_put(::aws_runtime::user_agent::AwsUserAgent::for_tests());
+        self.set_time_source(::std::option::Option::Some(
+            ::aws_smithy_async::time::SharedTimeSource::new(
+                ::aws_smithy_async::time::StaticTimeSource::new(
+                    ::std::time::UNIX_EPOCH + ::std::time::Duration::from_secs(1234567890),
+                ),
+            ),
+        ));
+        self.config
+            .store_put(::aws_runtime::user_agent::AwsUserAgent::for_tests());
         self.set_credentials_provider(Some(crate::config::SharedCredentialsProvider::new(
             ::aws_credential_types::Credentials::for_tests(),
         )));
-        self.behavior_version = ::std::option::Option::Some(crate::config::BehaviorVersion::latest());
+        self.behavior_version =
+            ::std::option::Option::Some(crate::config::BehaviorVersion::latest());
         self
     }
     #[cfg(any(feature = "test-util", test))]
@@ -1365,7 +1562,9 @@
     pub fn apply_test_defaults_v2(&mut self) -> &mut Self {
         self.apply_test_defaults();
         if self.config.load::<crate::config::Region>().is_none() {
-            self.set_region(::std::option::Option::Some(crate::config::Region::new("us-east-1")));
+            self.set_region(::std::option::Option::Some(crate::config::Region::new(
+                "us-east-1",
+            )));
         }
         self
     }
@@ -1382,7 +1581,9 @@
         let mut layer = self.config;
         if self.runtime_components.time_source().is_none() {
             self.runtime_components
-                .set_time_source(::std::option::Option::Some(::std::default::Default::default()));
+                .set_time_source(::std::option::Option::Some(
+                    ::std::default::Default::default(),
+                ));
         }
         layer.store_put(crate::meta::API_METADATA.clone());
         layer.store_put(::aws_types::SigningName::from_static("awsssoportal"));
@@ -1404,7 +1605,8 @@
 #[derive(::std::fmt::Debug)]
 pub(crate) struct ServiceRuntimePlugin {
     config: ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer>,
-    runtime_components: ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
+    runtime_components:
+        ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
 }

 impl ServiceRuntimePlugin {
@@ -1411,10 +1613,15 @@
     pub fn new(_service_config: crate::config::Config) -> Self {
         let config = {
             let mut cfg = ::aws_smithy_types::config_bag::Layer::new("SWBPortalService");
-            cfg.store_put(::aws_smithy_runtime::client::orchestrator::AuthSchemeAndEndpointOrchestrationV2);
+            cfg.store_put(
+                ::aws_smithy_runtime::client::orchestrator::AuthSchemeAndEndpointOrchestrationV2,
+            );
             ::std::option::Option::Some(cfg.freeze())
         };
-        let mut runtime_components = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ServiceRuntimePlugin");
+        let mut runtime_components =
+            ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new(
+                "ServiceRuntimePlugin",
+            );
         runtime_components.set_auth_scheme_option_resolver(::std::option::Option::Some({
             use crate::config::auth::ResolveAuthScheme;
             crate::config::auth::DefaultAuthSchemeResolver::default().into_shared_resolver()
@@ -1426,29 +1633,48 @@
         runtime_components.push_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
             ::aws_smithy_runtime::client::http::connection_poisoning::ConnectionPoisoningInterceptor::new(),
         ));
-        runtime_components.push_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::HttpStatusCodeClassifier::default());
-        runtime_components.push_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-            crate::sdk_feature_tracker::retry_mode::RetryModeFeatureTrackerInterceptor::new(),
-        ));
-        runtime_components.push_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-            ::aws_runtime::service_clock_skew::ServiceClockSkewInterceptor::new(),
-        ));
-        runtime_components.push_interceptor(::aws_runtime::request_info::RequestInfoInterceptor::new());
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
+        runtime_components
+            .push_interceptor(::aws_runtime::request_info::RequestInfoInterceptor::new());
         runtime_components.push_interceptor(::aws_runtime::user_agent::UserAgentInterceptor::new());
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
+        runtime_components
+            .push_interceptor(::aws_runtime::invocation_id::InvocationIdInterceptor::new());
+        runtime_components.push_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                ::aws_runtime::recursion_detection::RecursionDetectionInterceptor::new(),
+            ),
+        );
+        runtime_components.push_auth_scheme(
+            ::aws_smithy_runtime_api::client::auth::SharedAuthScheme::new(
+                ::aws_runtime::auth::sigv4::SigV4AuthScheme::new(),
+            ),
+        );
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

@@ -1464,7 +1690,10 @@
     fn runtime_components(
         &self,
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
-    ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
+    ) -> ::std::borrow::Cow<
+        '_,
+        ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
+    > {
         ::std::borrow::Cow::Borrowed(&self.runtime_components)
     }
 }
@@ -1478,7 +1707,8 @@
 #[derive(Debug)]
 pub(crate) struct ConfigOverrideRuntimePlugin {
     pub(crate) config: ::aws_smithy_types::config_bag::FrozenLayer,
-    pub(crate) components: ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
+    pub(crate) components:
+        ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
 }

 impl ConfigOverrideRuntimePlugin {
@@ -1491,14 +1721,22 @@
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

@@ -1511,7 +1749,9 @@
         // This is skipped if the override already sets its own identity cache.
         if components.has_identity_resolvers() && components.identity_cache().is_none() {
             components.set_identity_cache(::std::option::Option::Some(
-                ::aws_smithy_runtime::client::identity::IdentityCache::lazy().max_partitions(1).build(),
+                ::aws_smithy_runtime::client::identity::IdentityCache::lazy()
+                    .max_partitions(1)
+                    .build(),
             ));
         }

@@ -1524,7 +1764,9 @@
     }
 }

-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for ConfigOverrideRuntimePlugin {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin
+    for ConfigOverrideRuntimePlugin
+{
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         Some(self.config.clone())
     }
@@ -1532,7 +1774,10 @@
     fn runtime_components(
         &self,
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
-    ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
+    ) -> ::std::borrow::Cow<
+        '_,
+        ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
+    > {
         ::std::borrow::Cow::Borrowed(&self.components)
     }
 }
@@ -1557,8 +1802,12 @@
                 input
                     .service_config()
                     .and_then(|conf| {
-                        conf.load_config(service_config_key("SSO", "AWS_ENDPOINT_URL", "endpoint_url"))
-                            .map(|it| it.parse().unwrap())
+                        conf.load_config(service_config_key(
+                            "SSO",
+                            "AWS_ENDPOINT_URL",
+                            "endpoint_url",
+                        ))
+                        .map(|it| it.parse().unwrap())
                     })
                     .or_else(|| input.endpoint_url().map(|s| s.to_string())),
             );
@@ -1599,7 +1848,11 @@
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
@@ -1610,13 +1863,16 @@

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
@@ -10,11 +10,15 @@
     ) -> ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionsFuture<'a>;

     /// Convert this service-specific resolver into a `SharedAuthSchemeOptionResolver`
-    fn into_shared_resolver(self) -> ::aws_smithy_runtime_api::client::auth::SharedAuthSchemeOptionResolver
+    fn into_shared_resolver(
+        self,
+    ) -> ::aws_smithy_runtime_api::client::auth::SharedAuthSchemeOptionResolver
     where
         Self: ::std::marker::Sized + 'static,
     {
-        ::aws_smithy_runtime_api::client::auth::SharedAuthSchemeOptionResolver::new(DowncastParams(self))
+        ::aws_smithy_runtime_api::client::auth::SharedAuthSchemeOptionResolver::new(DowncastParams(
+            self,
+        ))
     }
 }

@@ -31,10 +35,15 @@
         runtime_components: &'a ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponents,
     ) -> ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionsFuture<'a> {
         match params.get::<crate::config::auth::Params>() {
-            ::std::option::Option::Some(concrete_params) => self.0.resolve_auth_scheme(concrete_params, cfg, runtime_components),
-            ::std::option::Option::None => ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionsFuture::ready(::std::result::Result::Err(
-                "params of expected type was not present".into(),
-            )),
+            ::std::option::Option::Some(concrete_params) => {
+                self.0
+                    .resolve_auth_scheme(concrete_params, cfg, runtime_components)
+            }
+            ::std::option::Option::None => {
+                ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionsFuture::ready(
+                    ::std::result::Result::Err("params of expected type was not present".into()),
+                )
+            }
         }
     }
 }
@@ -44,7 +53,10 @@
 #[allow(dead_code)]
 pub struct DefaultAuthSchemeResolver {
     service_defaults: Vec<::aws_smithy_runtime_api::client::auth::AuthSchemeOption>,
-    operation_overrides: ::std::collections::HashMap<&'static str, Vec<::aws_smithy_runtime_api::client::auth::AuthSchemeOption>>,
+    operation_overrides: ::std::collections::HashMap<
+        &'static str,
+        Vec<::aws_smithy_runtime_api::client::auth::AuthSchemeOption>,
+    >,
 }

 // TODO(https://github.com/smithy-lang/smithy-rs/issues/4177): Remove `allow(...)` once the issue is addressed.
@@ -55,34 +67,44 @@
 impl Default for DefaultAuthSchemeResolver {
     fn default() -> Self {
         Self {
-            service_defaults: vec![::aws_smithy_runtime_api::client::auth::AuthSchemeOption::builder()
-                .scheme_id(::aws_runtime::auth::sigv4::SCHEME_ID)
-                .build()
-                .expect("required fields set")],
+            service_defaults: vec![
+                ::aws_smithy_runtime_api::client::auth::AuthSchemeOption::builder()
+                    .scheme_id(::aws_runtime::auth::sigv4::SCHEME_ID)
+                    .build()
+                    .expect("required fields set"),
+            ],
             operation_overrides: [
                 (
                     "GetRoleCredentials",
-                    vec![::aws_smithy_runtime_api::client::auth::AuthSchemeOption::from(
-                        ::aws_smithy_runtime::client::auth::no_auth::NO_AUTH_SCHEME_ID,
-                    )],
+                    vec![
+                        ::aws_smithy_runtime_api::client::auth::AuthSchemeOption::from(
+                            ::aws_smithy_runtime::client::auth::no_auth::NO_AUTH_SCHEME_ID,
+                        ),
+                    ],
                 ),
                 (
                     "ListAccountRoles",
-                    vec![::aws_smithy_runtime_api::client::auth::AuthSchemeOption::from(
-                        ::aws_smithy_runtime::client::auth::no_auth::NO_AUTH_SCHEME_ID,
-                    )],
+                    vec![
+                        ::aws_smithy_runtime_api::client::auth::AuthSchemeOption::from(
+                            ::aws_smithy_runtime::client::auth::no_auth::NO_AUTH_SCHEME_ID,
+                        ),
+                    ],
                 ),
                 (
                     "ListAccounts",
-                    vec![::aws_smithy_runtime_api::client::auth::AuthSchemeOption::from(
-                        ::aws_smithy_runtime::client::auth::no_auth::NO_AUTH_SCHEME_ID,
-                    )],
+                    vec![
+                        ::aws_smithy_runtime_api::client::auth::AuthSchemeOption::from(
+                            ::aws_smithy_runtime::client::auth::no_auth::NO_AUTH_SCHEME_ID,
+                        ),
+                    ],
                 ),
                 (
                     "Logout",
-                    vec![::aws_smithy_runtime_api::client::auth::AuthSchemeOption::from(
-                        ::aws_smithy_runtime::client::auth::no_auth::NO_AUTH_SCHEME_ID,
-                    )],
+                    vec![
+                        ::aws_smithy_runtime_api::client::auth::AuthSchemeOption::from(
+                            ::aws_smithy_runtime::client::auth::no_auth::NO_AUTH_SCHEME_ID,
+                        ),
+                    ],
                 ),
             ]
             .into(),
@@ -104,7 +126,9 @@
             None => &self.service_defaults,
         };

-        let _fut = ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionsFuture::ready(Ok(modeled_auth_options.clone()));
+        let _fut = ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionsFuture::ready(Ok(
+            modeled_auth_options.clone(),
+        ));

         _fut
     }
@@ -127,7 +151,9 @@
     }
 }

-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(
+    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
+)]
 /// Builder for [`Params`]
 pub struct ParamsBuilder {
     operation_name: ::std::option::Option<::std::borrow::Cow<'static, str>>,
@@ -134,12 +160,18 @@
 }
 impl ParamsBuilder {
     /// Set the operation name for the builder
-    pub fn operation_name(self, operation_name: impl Into<::std::borrow::Cow<'static, str>>) -> Self {
+    pub fn operation_name(
+        self,
+        operation_name: impl Into<::std::borrow::Cow<'static, str>>,
+    ) -> Self {
         self.set_operation_name(::std::option::Option::Some(operation_name.into()))
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
@@ -147,9 +179,13 @@
     ///
     /// Return [`BuildError`] if any of the required fields are unset.
     ///
-    pub fn build(self) -> ::std::result::Result<crate::config::auth::Params, crate::config::auth::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<crate::config::auth::Params, crate::config::auth::BuildError> {
         ::std::result::Result::Ok(crate::config::auth::Params {
-            operation_name: self.operation_name.ok_or_else(|| BuildError::missing("operation_name"))?,
+            operation_name: self
+                .operation_name
+                .ok_or_else(|| BuildError::missing("operation_name"))?,
         })
     }
 }
@@ -162,7 +198,9 @@

 impl BuildError {
     fn missing(field: &'static str) -> Self {
-        Self { field: field.into() }
+        Self {
+            field: field.into(),
+        }
     }
 }

```

### `src/config/endpoint.rs`

```diff
diff --git a/src/config/endpoint.rs b/src/config/endpoint.rs
--- a/src/config/endpoint.rs
+++ b/src/config/endpoint.rs
@@ -8,7 +8,9 @@
 pub(crate) struct EndpointOverrideFeatureTrackerInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for EndpointOverrideFeatureTrackerInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept
+    for EndpointOverrideFeatureTrackerInterceptor
+{
     fn name(&self) -> &'static str {
         "EndpointOverrideFeatureTrackerInterceptor"
     }
@@ -18,7 +20,10 @@
         _context: &::aws_smithy_runtime_api::client::interceptors::context::BeforeSerializationInterceptorContextRef<'_>,
         cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
     ) -> ::std::result::Result<(), ::aws_smithy_runtime_api::box_error::BoxError> {
-        if cfg.load::<::aws_types::endpoint_config::EndpointUrl>().is_some() {
+        if cfg
+            .load::<::aws_types::endpoint_config::EndpointUrl>()
+            .is_some()
+        {
             cfg.interceptor_state()
                 .store_append(::aws_runtime::sdk_feature::AwsSdkFeature::EndpointOverride);
         }
@@ -40,7 +45,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.ap-east-1.amazonaws.com");
+        let endpoint =
+            endpoint.expect("Expected valid endpoint: https://portal.sso.ap-east-1.amazonaws.com");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -60,7 +66,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.ap-northeast-1.amazonaws.com");
+        let endpoint = endpoint
+            .expect("Expected valid endpoint: https://portal.sso.ap-northeast-1.amazonaws.com");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -80,7 +87,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.ap-northeast-2.amazonaws.com");
+        let endpoint = endpoint
+            .expect("Expected valid endpoint: https://portal.sso.ap-northeast-2.amazonaws.com");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -100,7 +108,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.ap-northeast-3.amazonaws.com");
+        let endpoint = endpoint
+            .expect("Expected valid endpoint: https://portal.sso.ap-northeast-3.amazonaws.com");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -120,7 +129,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.ap-south-1.amazonaws.com");
+        let endpoint =
+            endpoint.expect("Expected valid endpoint: https://portal.sso.ap-south-1.amazonaws.com");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -140,7 +150,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.ap-southeast-1.amazonaws.com");
+        let endpoint = endpoint
+            .expect("Expected valid endpoint: https://portal.sso.ap-southeast-1.amazonaws.com");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -160,7 +171,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.ap-southeast-2.amazonaws.com");
+        let endpoint = endpoint
+            .expect("Expected valid endpoint: https://portal.sso.ap-southeast-2.amazonaws.com");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -180,7 +192,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.ca-central-1.amazonaws.com");
+        let endpoint = endpoint
+            .expect("Expected valid endpoint: https://portal.sso.ca-central-1.amazonaws.com");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -200,7 +213,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.eu-central-1.amazonaws.com");
+        let endpoint = endpoint
+            .expect("Expected valid endpoint: https://portal.sso.eu-central-1.amazonaws.com");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -220,7 +234,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.eu-north-1.amazonaws.com");
+        let endpoint =
+            endpoint.expect("Expected valid endpoint: https://portal.sso.eu-north-1.amazonaws.com");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -240,7 +255,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.eu-south-1.amazonaws.com");
+        let endpoint =
+            endpoint.expect("Expected valid endpoint: https://portal.sso.eu-south-1.amazonaws.com");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -260,7 +276,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.eu-west-1.amazonaws.com");
+        let endpoint =
+            endpoint.expect("Expected valid endpoint: https://portal.sso.eu-west-1.amazonaws.com");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -280,7 +297,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.eu-west-2.amazonaws.com");
+        let endpoint =
+            endpoint.expect("Expected valid endpoint: https://portal.sso.eu-west-2.amazonaws.com");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -300,7 +318,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.eu-west-3.amazonaws.com");
+        let endpoint =
+            endpoint.expect("Expected valid endpoint: https://portal.sso.eu-west-3.amazonaws.com");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -320,7 +339,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.me-south-1.amazonaws.com");
+        let endpoint =
+            endpoint.expect("Expected valid endpoint: https://portal.sso.me-south-1.amazonaws.com");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -340,7 +360,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.sa-east-1.amazonaws.com");
+        let endpoint =
+            endpoint.expect("Expected valid endpoint: https://portal.sso.sa-east-1.amazonaws.com");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -360,7 +381,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.us-east-1.amazonaws.com");
+        let endpoint =
+            endpoint.expect("Expected valid endpoint: https://portal.sso.us-east-1.amazonaws.com");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -380,7 +402,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.us-east-2.amazonaws.com");
+        let endpoint =
+            endpoint.expect("Expected valid endpoint: https://portal.sso.us-east-2.amazonaws.com");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -400,7 +423,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.us-west-2.amazonaws.com");
+        let endpoint =
+            endpoint.expect("Expected valid endpoint: https://portal.sso.us-west-2.amazonaws.com");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -420,7 +444,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso-fips.us-east-1.api.aws");
+        let endpoint =
+            endpoint.expect("Expected valid endpoint: https://portal.sso-fips.us-east-1.api.aws");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -440,7 +465,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso-fips.us-east-1.amazonaws.com");
+        let endpoint = endpoint
+            .expect("Expected valid endpoint: https://portal.sso-fips.us-east-1.amazonaws.com");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -460,7 +486,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.us-east-1.api.aws");
+        let endpoint =
+            endpoint.expect("Expected valid endpoint: https://portal.sso.us-east-1.api.aws");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -500,7 +527,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso-fips.cn-north-1.amazonaws.com.cn");
+        let endpoint = endpoint
+            .expect("Expected valid endpoint: https://portal.sso-fips.cn-north-1.amazonaws.com.cn");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -520,7 +548,9 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.cn-north-1.api.amazonwebservices.com.cn");
+        let endpoint = endpoint.expect(
+            "Expected valid endpoint: https://portal.sso.cn-north-1.api.amazonwebservices.com.cn",
+        );
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -540,7 +570,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.cn-north-1.amazonaws.com.cn");
+        let endpoint = endpoint
+            .expect("Expected valid endpoint: https://portal.sso.cn-north-1.amazonaws.com.cn");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -560,7 +591,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.us-gov-east-1.amazonaws.com");
+        let endpoint = endpoint
+            .expect("Expected valid endpoint: https://portal.sso.us-gov-east-1.amazonaws.com");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -580,7 +612,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.us-gov-west-1.amazonaws.com");
+        let endpoint = endpoint
+            .expect("Expected valid endpoint: https://portal.sso.us-gov-west-1.amazonaws.com");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -600,7 +633,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso-fips.us-gov-east-1.api.aws");
+        let endpoint = endpoint
+            .expect("Expected valid endpoint: https://portal.sso-fips.us-gov-east-1.api.aws");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -620,7 +654,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.us-gov-east-1.amazonaws.com");
+        let endpoint = endpoint
+            .expect("Expected valid endpoint: https://portal.sso.us-gov-east-1.amazonaws.com");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -640,7 +675,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.us-gov-east-1.api.aws");
+        let endpoint =
+            endpoint.expect("Expected valid endpoint: https://portal.sso.us-gov-east-1.api.aws");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -660,7 +696,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso-fips.us-iso-east-1.c2s.ic.gov");
+        let endpoint = endpoint
+            .expect("Expected valid endpoint: https://portal.sso-fips.us-iso-east-1.c2s.ic.gov");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -680,7 +717,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.us-iso-east-1.c2s.ic.gov");
+        let endpoint =
+            endpoint.expect("Expected valid endpoint: https://portal.sso.us-iso-east-1.c2s.ic.gov");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -700,7 +738,9 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso-fips.us-isob-east-1.sc2s.sgov.gov");
+        let endpoint = endpoint.expect(
+            "Expected valid endpoint: https://portal.sso-fips.us-isob-east-1.sc2s.sgov.gov",
+        );
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -720,7 +760,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.us-isob-east-1.sc2s.sgov.gov");
+        let endpoint = endpoint
+            .expect("Expected valid endpoint: https://portal.sso.us-isob-east-1.sc2s.sgov.gov");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -744,7 +785,9 @@
         let endpoint = endpoint.expect("Expected valid endpoint: https://example.com");
         assert_eq!(
             endpoint,
-            ::aws_smithy_types::endpoint::Endpoint::builder().url("https://example.com").build()
+            ::aws_smithy_types::endpoint::Endpoint::builder()
+                .url("https://example.com")
+                .build()
         );
     }

@@ -762,7 +805,9 @@
         let endpoint = endpoint.expect("Expected valid endpoint: https://example.com");
         assert_eq!(
             endpoint,
-            ::aws_smithy_types::endpoint::Endpoint::builder().url("https://example.com").build()
+            ::aws_smithy_types::endpoint::Endpoint::builder()
+                .url("https://example.com")
+                .build()
         );
     }

@@ -779,7 +824,10 @@
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
@@ -804,11 +852,17 @@
     /// Missing region
     #[test]
     fn test_40() {
-        let params = crate::config::endpoint::Params::builder().build().expect("invalid params");
+        let params = crate::config::endpoint::Params::builder()
+            .build()
+            .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let error = endpoint.expect_err("expected error: Invalid Configuration: Missing Region [Missing region]");
-        assert_eq!(format!("{}", error), "Invalid Configuration: Missing Region")
+        let error = endpoint
+            .expect_err("expected error: Invalid Configuration: Missing Region [Missing region]");
+        assert_eq!(
+            format!("{}", error),
+            "Invalid Configuration: Missing Region"
+        )
     }
 }

@@ -815,16 +869,23 @@
 /// Endpoint resolver trait specific to AWS Single Sign-On
 pub trait ResolveEndpoint: ::std::marker::Send + ::std::marker::Sync + ::std::fmt::Debug {
     /// Resolve an endpoint with the given parameters
-    fn resolve_endpoint<'a>(&'a self, params: &'a crate::config::endpoint::Params) -> ::aws_smithy_runtime_api::client::endpoint::EndpointFuture<'a>;
+    fn resolve_endpoint<'a>(
+        &'a self,
+        params: &'a crate::config::endpoint::Params,
+    ) -> ::aws_smithy_runtime_api::client::endpoint::EndpointFuture<'a>;

     /// Convert this service-specific resolver into a `SharedEndpointResolver`
     ///
     /// The resulting resolver will downcast `EndpointResolverParams` into `crate::config::endpoint::Params`.
-    fn into_shared_resolver(self) -> ::aws_smithy_runtime_api::client::endpoint::SharedEndpointResolver
+    fn into_shared_resolver(
+        self,
+    ) -> ::aws_smithy_runtime_api::client::endpoint::SharedEndpointResolver
     where
         Self: Sized + 'static,
     {
-        ::aws_smithy_runtime_api::client::endpoint::SharedEndpointResolver::new(DowncastParams(self))
+        ::aws_smithy_runtime_api::client::endpoint::SharedEndpointResolver::new(DowncastParams(
+            self,
+        ))
     }
 }

@@ -840,7 +901,9 @@
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
@@ -850,7 +913,9 @@
 /// The default endpoint resolver.
 pub struct DefaultResolver {
     partition_resolver: &'static crate::endpoint_lib::partition::PartitionResolver,
-    endpoint_cache: ::arc_swap::ArcSwap<::std::option::Option<(Params, ::aws_smithy_types::endpoint::Endpoint)>>,
+    endpoint_cache: ::arc_swap::ArcSwap<
+        ::std::option::Option<(Params, ::aws_smithy_types::endpoint::Endpoint)>,
+    >,
 }

 impl Default for DefaultResolver {
@@ -884,7 +949,10 @@
     fn resolve_endpoint<'a>(
         &'a self,
         params: &'a crate::config::endpoint::Params,
-    ) -> ::std::result::Result<::aws_smithy_types::endpoint::Endpoint, ::aws_smithy_runtime_api::box_error::BoxError> {
+    ) -> ::std::result::Result<
+        ::aws_smithy_types::endpoint::Endpoint,
+        ::aws_smithy_runtime_api::box_error::BoxError,
+    > {
         let mut _diagnostic_collector = crate::endpoint_lib::diagnostic::DiagnosticCollector::new();
         #[allow(unused_mut)]
         let mut context = ConditionContext::default();
@@ -1018,10 +1086,12 @@
                     };
                 }
                 1 | -1 => {
-                    return ::std::result::Result::Err(
-                        Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message("No endpoint rule matched"))
-                            as ::aws_smithy_runtime_api::box_error::BoxError,
+                    return ::std::result::Result::Err(Box::new(
+                        ::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                            "No endpoint rule matched",
+                        ),
                     )
+                        as ::aws_smithy_runtime_api::box_error::BoxError)
                 }
                 ref_val => {
                     let is_complement = ref_val < 0;
@@ -1070,7 +1140,11 @@
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
@@ -1078,18 +1152,26 @@
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
+                return ::aws_smithy_runtime_api::client::endpoint::EndpointFuture::ready(
+                    ::std::result::Result::Ok(cached_endpoint.clone()),
+                );
             }
         }
         drop(cached);
         let result = self.resolve_endpoint(params);
         if let ::std::result::Result::Ok(ref endpoint) = result {
-            self.endpoint_cache.store(::std::sync::Arc::new(Some((params.clone(), endpoint.clone()))));
+            self.endpoint_cache.store(::std::sync::Arc::new(Some((
+                params.clone(),
+                endpoint.clone(),
+            ))));
         }
         ::aws_smithy_runtime_api::client::endpoint::EndpointFuture::ready(result)
     }
@@ -1213,7 +1295,9 @@
 }

 /// Builder for [`Params`]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(
+    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
+)]
 pub struct ParamsBuilder {
     region: ::std::option::Option<::std::string::String>,
     use_dual_stack: ::std::option::Option<bool>,
@@ -1222,7 +1306,12 @@
 }
 impl ParamsBuilder {
     /// Consume this builder, creating [`Params`].
-    pub fn build(self) -> ::std::result::Result<crate::config::endpoint::Params, crate::config::endpoint::InvalidParams> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<
+        crate::config::endpoint::Params,
+        crate::config::endpoint::InvalidParams,
+    > {
         if let Some(region) = &self.region {
             if !crate::endpoint_lib::host::is_valid_host_label(
                 region.as_ref() as &str,
@@ -1239,10 +1328,9 @@
             #[allow(clippy::unnecessary_lazy_evaluations)]
             crate::config::endpoint::Params {
                 region: self.region,
-                use_dual_stack: self
-                    .use_dual_stack
-                    .or_else(|| Some(false))
-                    .ok_or_else(|| crate::config::endpoint::InvalidParams::missing("use_dual_stack"))?,
+                use_dual_stack: self.use_dual_stack.or_else(|| Some(false)).ok_or_else(|| {
+                    crate::config::endpoint::InvalidParams::missing("use_dual_stack")
+                })?,
                 use_fips: self
                     .use_fips
                     .or_else(|| Some(false))
@@ -1352,8 +1440,12 @@
 impl std::fmt::Display for InvalidParams {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
         match self.kind {
-            InvalidParamsErrorKind::MissingField => write!(f, "a required field was missing: `{}`", self.field),
-            InvalidParamsErrorKind::InvalidValue { message } => write!(f, "invalid value for field: `{}` - {}", self.field, message),
+            InvalidParamsErrorKind::MissingField => {
+                write!(f, "a required field was missing: `{}`", self.field)
+            }
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
@@ -1,12 +1,18 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 // Loading the partition JSON is expensive since it involves many regex compilations,
 // so cache the result so that it only need to be paid for the first constructed client.
-pub(crate) static DEFAULT_PARTITION_RESOLVER: std::sync::LazyLock<crate::endpoint_lib::partition::PartitionResolver> = std::sync::LazyLock::new(
+pub(crate) static DEFAULT_PARTITION_RESOLVER: std::sync::LazyLock<
+    crate::endpoint_lib::partition::PartitionResolver,
+> = std::sync::LazyLock::new(
     || match std::env::var("SMITHY_CLIENT_SDK_CUSTOM_PARTITION") {
         Ok(partitions) => {
             ::tracing::debug!("loading custom partitions located at {partitions}");
-            let partition_dot_json = std::fs::read_to_string(partitions).expect("should be able to read a custom partition JSON");
-            crate::endpoint_lib::partition::PartitionResolver::new_from_json(partition_dot_json.as_bytes()).expect("valid JSON")
+            let partition_dot_json = std::fs::read_to_string(partitions)
+                .expect("should be able to read a custom partition JSON");
+            crate::endpoint_lib::partition::PartitionResolver::new_from_json(
+                partition_dot_json.as_bytes(),
+            )
+            .expect("valid JSON")
         }
         _ => {
             ::tracing::debug!("loading default partitions");
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
@@ -66,9 +71,14 @@
                 let node = nodes.get(node_index)?;
                 let condition_index = node.condition_index as usize;
                 let condition = conditions.get(condition_index)?;
-                let condition_result = condition_evaluator(condition, params, context, diagnostic_collector);
+                let condition_result =
+                    condition_evaluator(condition, params, context, diagnostic_collector);
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

### `src/endpoint_lib/diagnostic.rs`

```diff
diff --git a/src/endpoint_lib/diagnostic.rs b/src/endpoint_lib/diagnostic.rs
--- a/src/endpoint_lib/diagnostic.rs
+++ b/src/endpoint_lib/diagnostic.rs
@@ -24,7 +24,10 @@

     #[allow(unused)]
     /// Capture a result, returning Some(t) when the input was `Ok` and `None` otherwise
-    pub(crate) fn capture<T, E: Into<Box<dyn Error + Send + Sync>>>(&mut self, err: Result<T, E>) -> Option<T> {
+    pub(crate) fn capture<T, E: Into<Box<dyn Error + Send + Sync>>>(
+        &mut self,
+        err: Result<T, E>,
+    ) -> Option<T> {
         match err {
             Ok(res) => Some(res),
             Err(e) => {
```

### `src/endpoint_lib/host.rs`

```diff
diff --git a/src/endpoint_lib/host.rs b/src/endpoint_lib/host.rs
--- a/src/endpoint_lib/host.rs
+++ b/src/endpoint_lib/host.rs
@@ -6,7 +6,11 @@

 use crate::endpoint_lib::diagnostic::DiagnosticCollector;

-pub(crate) fn is_valid_host_label(label: &str, allow_dots: bool, e: &mut DiagnosticCollector) -> bool {
+pub(crate) fn is_valid_host_label(
+    label: &str,
+    allow_dots: bool,
+    e: &mut DiagnosticCollector,
+) -> bool {
     let bytes = label.as_bytes();
     if allow_dots {
         let mut start = 0;
@@ -63,7 +67,10 @@
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

@@ -94,7 +98,9 @@
         self.partitions.push(partition);
     }

-    pub(crate) fn new_from_json(partition_dot_json: &[u8]) -> Result<PartitionResolver, DeserializeError> {
+    pub(crate) fn new_from_json(
+        partition_dot_json: &[u8],
+    ) -> Result<PartitionResolver, DeserializeError> {
         deserialize_partitions(partition_dot_json)
     }

@@ -112,26 +118,41 @@
     /// 4. After matching the identifier to a partition using one of the previous steps, the partition function should return a
     ///    typed data structure containing the fields in `outputs` in the matched partition. **Important:** If a specific region
     ///    was matched, the properties associated with that region **MUST** be merged with the `outputs` field.
-    pub(crate) fn resolve_partition(&self, region: &str, e: &mut DiagnosticCollector) -> Option<Partition<'_>> {
-        let mut explicit_match_partition = self.partitions.iter().flat_map(|part| part.explicit_match(region));
-        let mut regex_match_partition = self.partitions.iter().flat_map(|part| part.regex_match(region));
+    pub(crate) fn resolve_partition(
+        &self,
+        region: &str,
+        e: &mut DiagnosticCollector,
+    ) -> Option<Partition<'_>> {
+        let mut explicit_match_partition = self
+            .partitions
+            .iter()
+            .flat_map(|part| part.explicit_match(region));
+        let mut regex_match_partition = self
+            .partitions
+            .iter()
+            .flat_map(|part| part.regex_match(region));

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
             dns_suffix: merge!(base, region_override, dns_suffix),
             dual_stack_dns_suffix: merge!(base, region_override, dual_stack_dns_suffix),
-            supports_fips: region_override.supports_fips.unwrap_or(base.outputs.supports_fips),
-            supports_dual_stack: region_override.supports_dual_stack.unwrap_or(base.outputs.supports_dual_stack),
+            supports_fips: region_override
+                .supports_fips
+                .unwrap_or(base.outputs.supports_fips),
+            supports_dual_stack: region_override
+                .supports_dual_stack
+                .unwrap_or(base.outputs.supports_dual_stack),
             implicit_global_region: merge!(base, region_override, implicit_global_region),
         })
     }
@@ -171,11 +192,19 @@
 }

 impl PartitionMetadata {
-    fn explicit_match(&self, region: &str) -> Option<(&PartitionMetadata, Option<&PartitionOutputOverride>)> {
-        self.regions.get(region).map(|output_override| (self, Some(output_override)))
+    fn explicit_match(
+        &self,
+        region: &str,
+    ) -> Option<(&PartitionMetadata, Option<&PartitionOutputOverride>)> {
+        self.regions
+            .get(region)
+            .map(|output_override| (self, Some(output_override)))
     }

-    fn regex_match(&self, region: &str) -> Option<(&PartitionMetadata, Option<&PartitionOutputOverride>)> {
+    fn regex_match(
+        &self,
+        region: &str,
+    ) -> Option<(&PartitionMetadata, Option<&PartitionOutputOverride>)> {
         if self.region_regex.is_match(region) {
             Some((self, None))
         } else {
@@ -205,14 +234,22 @@
 }

 impl PartitionOutputOverride {
-    pub(crate) fn into_partition_output(self) -> Result<PartitionOutput, Box<dyn std::error::Error>> {
+    pub(crate) fn into_partition_output(
+        self,
+    ) -> Result<PartitionOutput, Box<dyn std::error::Error>> {
         Ok(PartitionOutput {
             name: self.name.ok_or("missing name")?,
             dns_suffix: self.dns_suffix.ok_or("missing dnsSuffix")?,
-            dual_stack_dns_suffix: self.dual_stack_dns_suffix.ok_or("missing dual_stackDnsSuffix")?,
+            dual_stack_dns_suffix: self
+                .dual_stack_dns_suffix
+                .ok_or("missing dual_stackDnsSuffix")?,
             supports_fips: self.supports_fips.ok_or("missing supports fips")?,
-            supports_dual_stack: self.supports_dual_stack.ok_or("missing supportsDualstack")?,
-            implicit_global_region: self.implicit_global_region.ok_or("missing implicitGlobalRegion")?,
+            supports_dual_stack: self
+                .supports_dual_stack
+                .ok_or("missing supportsDualstack")?,
+            implicit_global_region: self
+                .implicit_global_region
+                .ok_or("missing implicitGlobalRegion")?,
         })
     }
 }
@@ -221,14 +258,20 @@
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
     use std::collections::HashMap;

-    pub(crate) fn deserialize_partitions(value: &[u8]) -> Result<PartitionResolver, DeserializeError> {
+    pub(crate) fn deserialize_partitions(
+        value: &[u8],
+    ) -> Result<PartitionResolver, DeserializeError> {
         let mut tokens_owned = json_token_iter(value).peekable();
         let tokens = &mut tokens_owned;
         expect_start_object(tokens.next())?;
@@ -238,20 +281,30 @@
                 Some(Token::EndObject { .. }) => break,
                 Some(Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                     "partitions" => {
-                        resolver = Some(PartitionResolver::from_partitions(deser_partitions(tokens)?));
+                        resolver = Some(PartitionResolver::from_partitions(deser_partitions(
+                            tokens,
+                        )?));
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

-    fn deser_partitions<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Vec<PartitionMetadata>, DeserializeError>
+    fn deser_partitions<'a, I>(
+        tokens: &mut std::iter::Peekable<I>,
+    ) -> Result<Vec<PartitionMetadata>, DeserializeError>
     where
         I: Iterator<Item = Result<Token<'a>, DeserializeError>>,
     {
@@ -275,7 +328,9 @@
         }
     }

-    pub(crate) fn deser_partition<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<PartitionMetadata, DeserializeError>
+    pub(crate) fn deser_partition<'a, I>(
+        tokens: &mut std::iter::Peekable<I>,
+    ) -> Result<PartitionMetadata, DeserializeError>
     where
         I: Iterator<Item = Result<Token<'a>, DeserializeError>>,
     {
@@ -303,7 +358,11 @@
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
@@ -332,7 +391,11 @@
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
@@ -342,7 +405,9 @@
     }

     /// Convert a token to `Str` (a potentially static String)
-    fn token_to_str(token: Option<Result<Token, DeserializeError>>) -> Result<Option<super::Str>, DeserializeError> {
+    fn token_to_str(
+        token: Option<Result<Token, DeserializeError>>,
+    ) -> Result<Option<super::Str>, DeserializeError> {
         Ok(expect_string_or_null(token)?
             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
             .transpose()?
@@ -349,7 +414,9 @@
             .map(Cow::Owned))
     }

-    fn deser_outputs<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<PartitionOutputOverride>, DeserializeError>
+    fn deser_outputs<'a, I>(
+        tokens: &mut std::iter::Peekable<I>,
+    ) -> Result<Option<PartitionOutputOverride>, DeserializeError>
     where
         I: Iterator<Item = Result<Token<'a>, DeserializeError>>,
     {
@@ -381,7 +448,11 @@
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
@@ -394,7 +465,9 @@
 #[cfg(test)]
 mod test {
     use crate::endpoint_lib::diagnostic::DiagnosticCollector;
-    use crate::endpoint_lib::partition::{Partition, PartitionMetadata, PartitionOutput, PartitionOutputOverride, PartitionResolver};
+    use crate::endpoint_lib::partition::{
+        Partition, PartitionMetadata, PartitionOutput, PartitionOutputOverride, PartitionResolver,
+    };
     use regex_lite::Regex;
     use std::collections::HashMap;

@@ -508,11 +581,18 @@
     }
   ]
 }"#;
-        let resolver = super::deser::deserialize_partitions(partitions.as_bytes()).expect("valid resolver");
+        let resolver =
+            super::deser::deserialize_partitions(partitions.as_bytes()).expect("valid resolver");
         assert_eq!(resolve(&resolver, "cn-north-1").name, "aws-cn");
-        assert_eq!(resolve(&resolver, "cn-north-1").dns_suffix, "amazonaws.com.cn");
+        assert_eq!(
+            resolve(&resolver, "cn-north-1").dns_suffix,
+            "amazonaws.com.cn"
+        );
         assert_eq!(resolver.partitions.len(), 5);
-        assert_eq!(resolve(&resolver, "af-south-1").implicit_global_region, "us-east-1");
+        assert_eq!(
+            resolve(&resolver, "af-south-1").implicit_global_region,
+            "us-east-1"
+        );
     }

     #[test]
@@ -551,7 +631,10 @@
         assert_eq!(resolve(&resolver, "us-east-1").name, "aws");
         assert_eq!(resolve(&resolver, "other-west-2").name, "other");
         // mars-east-1 hits aws through the default fallback
-        assert_eq!(resolve(&resolver, "mars-east-1").dns_suffix, "amazonaws.com");
+        assert_eq!(
+            resolve(&resolver, "mars-east-1").dns_suffix,
+            "amazonaws.com"
+        );
         // mars-east-2 hits aws through the region override
         assert_eq!(resolve(&resolver, "mars-east-2").dns_suffix, "mars.aws");
     }
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
@@ -3,32 +3,82 @@
 #[non_exhaustive]
 #[derive(::std::fmt::Debug)]
 pub enum Error {
+    #[cfg(any(
+        feature = "op_get_role_credentials",
+        feature = "op_list_account_roles",
+        feature = "op_list_accounts",
+        feature = "op_logout"
+    ))]
     /// <p>Indicates that a problem occurred with the input to the request. For example, a required parameter might be missing or out of range.</p>
     InvalidRequestException(crate::types::error::InvalidRequestException),
+    #[cfg(any(
+        feature = "op_get_role_credentials",
+        feature = "op_list_account_roles",
+        feature = "op_list_accounts"
+    ))]
     /// <p>The specified resource doesn't exist.</p>
     ResourceNotFoundException(crate::types::error::ResourceNotFoundException),
+    #[cfg(any(
+        feature = "op_get_role_credentials",
+        feature = "op_list_account_roles",
+        feature = "op_list_accounts",
+        feature = "op_logout"
+    ))]
     /// <p>Indicates that the request is being made too frequently and is more than what the server can handle.</p>
     TooManyRequestsException(crate::types::error::TooManyRequestsException),
+    #[cfg(any(
+        feature = "op_get_role_credentials",
+        feature = "op_list_account_roles",
+        feature = "op_list_accounts",
+        feature = "op_logout"
+    ))]
     /// <p>Indicates that the request is not authorized. This can happen due to an invalid access token in the request.</p>
     UnauthorizedException(crate::types::error::UnauthorizedException),
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
+                feature = "op_get_role_credentials",
+                feature = "op_list_account_roles",
+                feature = "op_list_accounts",
+                feature = "op_logout"
+            ))]
             Error::InvalidRequestException(inner) => inner.fmt(f),
+            #[cfg(any(
+                feature = "op_get_role_credentials",
+                feature = "op_list_account_roles",
+                feature = "op_list_accounts"
+            ))]
             Error::ResourceNotFoundException(inner) => inner.fmt(f),
+            #[cfg(any(
+                feature = "op_get_role_credentials",
+                feature = "op_list_account_roles",
+                feature = "op_list_accounts",
+                feature = "op_logout"
+            ))]
             Error::TooManyRequestsException(inner) => inner.fmt(f),
+            #[cfg(any(
+                feature = "op_get_role_credentials",
+                feature = "op_list_account_roles",
+                feature = "op_list_accounts",
+                feature = "op_logout"
+            ))]
             Error::UnauthorizedException(inner) => inner.fmt(f),
             Error::Unhandled(_) => {
-                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
+                if let ::std::option::Option::Some(code) =
+                    ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
+                {
                     write!(f, "unhandled error ({code})")
                 } else {
                     f.write_str("unhandled error")
@@ -48,21 +98,58 @@
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for Error {
     fn meta(&self) -> &::aws_smithy_types::error::metadata::ErrorMetadata {
         match self {
+            #[cfg(any(
+                feature = "op_get_role_credentials",
+                feature = "op_list_account_roles",
+                feature = "op_list_accounts",
+                feature = "op_logout"
+            ))]
             Self::InvalidRequestException(inner) => inner.meta(),
+            #[cfg(any(
+                feature = "op_get_role_credentials",
+                feature = "op_list_account_roles",
+                feature = "op_list_accounts"
+            ))]
             Self::ResourceNotFoundException(inner) => inner.meta(),
+            #[cfg(any(
+                feature = "op_get_role_credentials",
+                feature = "op_list_account_roles",
+                feature = "op_list_accounts",
+                feature = "op_logout"
+            ))]
             Self::TooManyRequestsException(inner) => inner.meta(),
+            #[cfg(any(
+                feature = "op_get_role_credentials",
+                feature = "op_list_account_roles",
+                feature = "op_list_accounts",
+                feature = "op_logout"
+            ))]
             Self::UnauthorizedException(inner) => inner.meta(),
             Self::Unhandled(inner) => &inner.meta,
         }
     }
 }
-impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_role_credentials::GetRoleCredentialsError, R>> for Error
+#[cfg(feature = "op_get_role_credentials")]
+impl<R>
+    From<
+        ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::get_role_credentials::GetRoleCredentialsError,
+            R,
+        >,
+    > for Error
 where
     R: Send + Sync + std::fmt::Debug + 'static,
 {
-    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_role_credentials::GetRoleCredentialsError, R>) -> Self {
+    fn from(
+        err: ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::get_role_credentials::GetRoleCredentialsError,
+            R,
+        >,
+    ) -> Self {
         match err {
-            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
+            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => {
+                Self::from(context.into_err())
+            }
             _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                 meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                 source: err.into(),
@@ -70,6 +157,7 @@
         }
     }
 }
+#[cfg(feature = "op_get_role_credentials")]
 impl From<crate::operation::get_role_credentials::GetRoleCredentialsError> for Error {
     fn from(err: crate::operation::get_role_credentials::GetRoleCredentialsError) -> Self {
         match err {
@@ -85,13 +173,27 @@
         }
     }
 }
-impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_account_roles::ListAccountRolesError, R>> for Error
+#[cfg(feature = "op_list_account_roles")]
+impl<R>
+    From<
+        ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::list_account_roles::ListAccountRolesError,
+            R,
+        >,
+    > for Error
 where
     R: Send + Sync + std::fmt::Debug + 'static,
 {
-    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_account_roles::ListAccountRolesError, R>) -> Self {
+    fn from(
+        err: ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::list_account_roles::ListAccountRolesError,
+            R,
+        >,
+    ) -> Self {
         match err {
-            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
+            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => {
+                Self::from(context.into_err())
+            }
             _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                 meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                 source: err.into(),
@@ -99,6 +201,7 @@
         }
     }
 }
+#[cfg(feature = "op_list_account_roles")]
 impl From<crate::operation::list_account_roles::ListAccountRolesError> for Error {
     fn from(err: crate::operation::list_account_roles::ListAccountRolesError) -> Self {
         match err {
@@ -110,13 +213,27 @@
         }
     }
 }
-impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_accounts::ListAccountsError, R>> for Error
+#[cfg(feature = "op_list_accounts")]
+impl<R>
+    From<
+        ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::list_accounts::ListAccountsError,
+            R,
+        >,
+    > for Error
 where
     R: Send + Sync + std::fmt::Debug + 'static,
 {
-    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_accounts::ListAccountsError, R>) -> Self {
+    fn from(
+        err: ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::list_accounts::ListAccountsError,
+            R,
+        >,
+    ) -> Self {
         match err {
-            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
+            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => {
+                Self::from(context.into_err())
+            }
             _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                 meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                 source: err.into(),
@@ -124,24 +241,49 @@
         }
     }
 }
+#[cfg(feature = "op_list_accounts")]
 impl From<crate::operation::list_accounts::ListAccountsError> for Error {
     fn from(err: crate::operation::list_accounts::ListAccountsError) -> Self {
         match err {
-            crate::operation::list_accounts::ListAccountsError::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
-            crate::operation::list_accounts::ListAccountsError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
-            crate::operation::list_accounts::ListAccountsError::TooManyRequestsException(inner) => Error::TooManyRequestsException(inner),
-            crate::operation::list_accounts::ListAccountsError::UnauthorizedException(inner) => Error::UnauthorizedException(inner),
-            crate::operation::list_accounts::ListAccountsError::Unhandled(inner) => Error::Unhandled(inner),
+            crate::operation::list_accounts::ListAccountsError::InvalidRequestException(inner) => {
+                Error::InvalidRequestException(inner)
+            }
+            crate::operation::list_accounts::ListAccountsError::ResourceNotFoundException(
+                inner,
+            ) => Error::ResourceNotFoundException(inner),
+            crate::operation::list_accounts::ListAccountsError::TooManyRequestsException(inner) => {
+                Error::TooManyRequestsException(inner)
+            }
+            crate::operation::list_accounts::ListAccountsError::UnauthorizedException(inner) => {
+                Error::UnauthorizedException(inner)
+            }
+            crate::operation::list_accounts::ListAccountsError::Unhandled(inner) => {
+                Error::Unhandled(inner)
+            }
         }
     }
 }
-impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::logout::LogoutError, R>> for Error
+#[cfg(feature = "op_logout")]
+impl<R>
+    From<
+        ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::logout::LogoutError,
+            R,
+        >,
+    > for Error
 where
     R: Send + Sync + std::fmt::Debug + 'static,
 {
-    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::logout::LogoutError, R>) -> Self {
+    fn from(
+        err: ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::logout::LogoutError,
+            R,
+        >,
+    ) -> Self {
         match err {
-            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
+            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => {
+                Self::from(context.into_err())
+            }
             _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                 meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                 source: err.into(),
@@ -149,12 +291,19 @@
         }
     }
 }
+#[cfg(feature = "op_logout")]
 impl From<crate::operation::logout::LogoutError> for Error {
     fn from(err: crate::operation::logout::LogoutError) -> Self {
         match err {
-            crate::operation::logout::LogoutError::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
-            crate::operation::logout::LogoutError::TooManyRequestsException(inner) => Error::TooManyRequestsException(inner),
-            crate::operation::logout::LogoutError::UnauthorizedException(inner) => Error::UnauthorizedException(inner),
+            crate::operation::logout::LogoutError::InvalidRequestException(inner) => {
+                Error::InvalidRequestException(inner)
+            }
+            crate::operation::logout::LogoutError::TooManyRequestsException(inner) => {
+                Error::TooManyRequestsException(inner)
+            }
+            crate::operation::logout::LogoutError::UnauthorizedException(inner) => {
+                Error::UnauthorizedException(inner)
+            }
             crate::operation::logout::LogoutError::Unhandled(inner) => Error::Unhandled(inner),
         }
     }
@@ -162,9 +311,32 @@
 impl ::std::error::Error for Error {
     fn source(&self) -> std::option::Option<&(dyn ::std::error::Error + 'static)> {
         match self {
+            #[cfg(any(
+                feature = "op_get_role_credentials",
+                feature = "op_list_account_roles",
+                feature = "op_list_accounts",
+                feature = "op_logout"
+            ))]
             Error::InvalidRequestException(inner) => inner.source(),
+            #[cfg(any(
+                feature = "op_get_role_credentials",
+                feature = "op_list_account_roles",
+                feature = "op_list_accounts"
+            ))]
             Error::ResourceNotFoundException(inner) => inner.source(),
+            #[cfg(any(
+                feature = "op_get_role_credentials",
+                feature = "op_list_account_roles",
+                feature = "op_list_accounts",
+                feature = "op_logout"
+            ))]
             Error::TooManyRequestsException(inner) => inner.source(),
+            #[cfg(any(
+                feature = "op_get_role_credentials",
+                feature = "op_list_account_roles",
+                feature = "op_list_accounts",
+                feature = "op_logout"
+            ))]
             Error::UnauthorizedException(inner) => inner.source(),
             Error::Unhandled(inner) => ::std::option::Option::Some(&*inner.source),
         }
@@ -173,9 +345,32 @@
 impl ::aws_types::request_id::RequestId for Error {
     fn request_id(&self) -> Option<&str> {
         match self {
+            #[cfg(any(
+                feature = "op_get_role_credentials",
+                feature = "op_list_account_roles",
+                feature = "op_list_accounts",
+                feature = "op_logout"
+            ))]
             Self::InvalidRequestException(e) => e.request_id(),
+            #[cfg(any(
+                feature = "op_get_role_credentials",
+                feature = "op_list_account_roles",
+                feature = "op_list_accounts"
+            ))]
             Self::ResourceNotFoundException(e) => e.request_id(),
+            #[cfg(any(
+                feature = "op_get_role_credentials",
+                feature = "op_list_account_roles",
+                feature = "op_list_accounts",
+                feature = "op_logout"
+            ))]
             Self::TooManyRequestsException(e) => e.request_id(),
+            #[cfg(any(
+                feature = "op_get_role_credentials",
+                feature = "op_list_account_roles",
+                feature = "op_list_accounts",
+                feature = "op_logout"
+            ))]
             Self::UnauthorizedException(e) => e.request_id(),
             Self::Unhandled(e) => e.meta.request_id(),
         }
```

### `src/json_errors.rs`

```diff
diff --git a/src/json_errors.rs b/src/json_errors.rs
--- a/src/json_errors.rs
+++ b/src/json_errors.rs
@@ -48,27 +48,45 @@
                         match key.as_escaped_str() {
                             "code" => code = Some(value.to_unescaped()?),
                             "__type" => typ = Some(value.to_unescaped()?),
-                            "message" | "Message" | "errorMessage" => message = Some(value.to_unescaped()?),
+                            "message" | "Message" | "errorMessage" => {
+                                message = Some(value.to_unescaped()?)
+                            }
                             _ => {}
                         }
                     }
                     skip_value(&mut tokens)?;
                 }
-                _ => return Err(DeserializeError::custom("expected object key or end object")),
+                _ => {
+                    return Err(DeserializeError::custom(
+                        "expected object key or end object",
+                    ))
+                }
             }
         }
         if tokens.next().is_some() {
-            return Err(DeserializeError::custom("found more JSON tokens after completing parsing"));
+            return Err(DeserializeError::custom(
+                "found more JSON tokens after completing parsing",
+            ));
         }
     }
-    Ok(ErrorBody { code: code.or(typ), message })
+    Ok(ErrorBody {
+        code: code.or(typ),
+        message,
+    })
 }

-pub fn parse_error_metadata(payload: &[u8], headers: &Headers) -> Result<ErrorMetadataBuilder, DeserializeError> {
+pub fn parse_error_metadata(
+    payload: &[u8],
+    headers: &Headers,
+) -> Result<ErrorMetadataBuilder, DeserializeError> {
     let ErrorBody { code, message } = parse_error_body(payload)?;

     let mut err_builder = ErrorMetadata::builder();
-    if let Some(code) = headers.get("x-amzn-errortype").or(code.as_deref()).map(sanitize_error_code) {
+    if let Some(code) = headers
+        .get("x-amzn-errortype")
+        .or(code.as_deref())
+        .map(sanitize_error_code)
+    {
         err_builder = err_builder.code(code);
     }
     if let Some(message) = message {
@@ -88,7 +106,9 @@
     fn error_metadata() {
         let response = HttpResponse::try_from(
             http_1x::Response::builder()
-                .body(SdkBody::from(r#"{ "__type": "FooError", "message": "Go to foo" }"#))
+                .body(SdkBody::from(
+                    r#"{ "__type": "FooError", "message": "Go to foo" }"#,
+                ))
                 .unwrap(),
         )
         .unwrap();
@@ -96,7 +116,10 @@
             parse_error_metadata(response.body().bytes().unwrap(), response.headers())
                 .unwrap()
                 .build(),
-            ErrorMetadata::builder().code("FooError").message("Go to foo").build()
+            ErrorMetadata::builder()
+                .code("FooError")
+                .message("Go to foo")
+                .build()
         )
     }

@@ -104,7 +127,9 @@
     fn error_type() {
         assert_eq!(
             Some(Cow::Borrowed("FooError")),
-            parse_error_body(br#"{ "__type": "FooError" }"#).unwrap().code
+            parse_error_body(br#"{ "__type": "FooError" }"#)
+                .unwrap()
+                .code
         );
     }

@@ -112,7 +137,9 @@
     fn code_takes_priority() {
         assert_eq!(
             Some(Cow::Borrowed("BarError")),
-            parse_error_body(br#"{ "code": "BarError", "__type": "FooError" }"#).unwrap().code
+            parse_error_body(br#"{ "code": "BarError", "__type": "FooError" }"#)
+                .unwrap()
+                .code
         );
     }

@@ -142,7 +169,9 @@
     #[test]
     fn sanitize_url() {
         assert_eq!(
-            sanitize_error_code("FooError:http://internal.amazon.com/coral/com.amazon.coral.validate/"),
+            sanitize_error_code(
+                "FooError:http://internal.amazon.com/coral/com.amazon.coral.validate/"
+            ),
             "FooError"
         );
     }
@@ -149,7 +178,10 @@

     #[test]
     fn sanitize_namespace() {
-        assert_eq!(sanitize_error_code("aws.protocoltests.restjson#FooError"), "FooError");
+        assert_eq!(
+            sanitize_error_code("aws.protocoltests.restjson#FooError"),
+            "FooError"
+        );
     }

     // services like lambda use an alternate `Message` instead of `message`
```

### `src/lens.rs`

```diff
diff --git a/src/lens.rs b/src/lens.rs
--- a/src/lens.rs
+++ b/src/lens.rs
@@ -1,4 +1,5 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+#[cfg(feature = "op_list_account_roles")]
 pub(crate) fn reflens_list_account_roles_output_output_next_token(
     input: &crate::operation::list_account_roles::ListAccountRolesOutput,
 ) -> ::std::option::Option<&::std::string::String> {
@@ -9,6 +10,7 @@
     ::std::option::Option::Some(input)
 }

+#[cfg(feature = "op_list_accounts")]
 pub(crate) fn reflens_list_accounts_output_output_next_token(
     input: &crate::operation::list_accounts::ListAccountsOutput,
 ) -> ::std::option::Option<&::std::string::String> {
@@ -19,6 +21,7 @@
     ::std::option::Option::Some(input)
 }

+#[cfg(feature = "op_list_account_roles")]
 pub(crate) fn lens_list_account_roles_output_output_role_list(
     input: crate::operation::list_account_roles::ListAccountRolesOutput,
 ) -> ::std::option::Option<::std::vec::Vec<crate::types::RoleInfo>> {
@@ -26,6 +29,7 @@
     ::std::option::Option::Some(input)
 }

+#[cfg(feature = "op_list_accounts")]
 pub(crate) fn lens_list_accounts_output_output_account_list(
     input: crate::operation::list_accounts::ListAccountsOutput,
 ) -> ::std::option::Option<::std::vec::Vec<crate::types::AccountInfo>> {
```

### `src/meta.rs`

```diff
diff --git a/src/meta.rs b/src/meta.rs
--- a/src/meta.rs
+++ b/src/meta.rs
@@ -1,5 +1,6 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-pub(crate) static API_METADATA: ::aws_runtime::user_agent::ApiMetadata = ::aws_runtime::user_agent::ApiMetadata::new("sso", crate::meta::PKG_VERSION);
+pub(crate) static API_METADATA: ::aws_runtime::user_agent::ApiMetadata =
+    ::aws_runtime::user_agent::ApiMetadata::new("sso", crate::meta::PKG_VERSION);

 /// Crate version number.
 pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
```

### `src/observability_feature.rs`

```diff
diff --git a/src/observability_feature.rs b/src/observability_feature.rs
--- a/src/observability_feature.rs
+++ b/src/observability_feature.rs
@@ -7,7 +7,9 @@
 use aws_smithy_runtime::client::sdk_feature::SmithySdkFeature;
 use aws_smithy_runtime_api::{
     box_error::BoxError,
-    client::interceptors::{context::BeforeSerializationInterceptorContextRef, dyn_dispatch_hint, Intercept},
+    client::interceptors::{
+        context::BeforeSerializationInterceptorContextRef, dyn_dispatch_hint, Intercept,
+    },
 };
 use aws_smithy_types::config_bag::ConfigBag;

@@ -21,7 +23,11 @@
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
@@ -28,7 +34,8 @@

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
@@ -1,14 +1,18 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub use ::aws_types::request_id::RequestId;

+#[cfg(feature = "op_get_role_credentials")]
 /// Types for the `GetRoleCredentials` operation.
 pub mod get_role_credentials;

+#[cfg(feature = "op_list_account_roles")]
 /// Types for the `ListAccountRoles` operation.
 pub mod list_account_roles;

+#[cfg(feature = "op_list_accounts")]
 /// Types for the `ListAccounts` operation.
 pub mod list_accounts;

+#[cfg(feature = "op_logout")]
 /// Types for the `Logout` operation.
 pub mod logout;
```

### `src/operation/get_role_credentials.rs`

```diff
diff --git a/src/operation/get_role_credentials.rs b/src/operation/get_role_credentials.rs
--- a/src/operation/get_role_credentials.rs
+++ b/src/operation/get_role_credentials.rs
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
-        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("SSO", "GetRoleCredentials", input, runtime_plugins, stop_point)
-            // Create a parent span for the entire operation. Includes a random, internal-only,
-            // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
-            .instrument(::tracing::debug_span!(
-                "SSO.GetRoleCredentials",
-                "rpc.service" = "SSO",
-                "rpc.method" = "GetRoleCredentials",
-                "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
-                "rpc.system" = "aws-api",
-            ))
-            .await
+        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point(
+            "SSO",
+            "GetRoleCredentials",
+            input,
+            runtime_plugins,
+            stop_point,
+        )
+        // Create a parent span for the entire operation. Includes a random, internal-only,
+        // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
+        .instrument(::tracing::debug_span!(
+            "SSO.GetRoleCredentials",
+            "rpc.service" = "SSO",
+            "rpc.method" = "GetRoleCredentials",
+            "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
+            "rpc.system" = "aws-api",
+        ))
+        .await
     }

     pub(crate) fn operation_runtime_plugins(
@@ -75,11 +85,13 @@
             for plugin in config_override.runtime_plugins.iter().cloned() {
                 runtime_plugins = runtime_plugins.with_operation_plugin(plugin);
             }
-            runtime_plugins = runtime_plugins.with_operation_plugin(crate::config::ConfigOverrideRuntimePlugin::new(
-                config_override,
-                client_config.config.clone(),
-                &client_config.runtime_components,
-            ));
+            runtime_plugins = runtime_plugins.with_operation_plugin(
+                crate::config::ConfigOverrideRuntimePlugin::new(
+                    config_override,
+                    client_config.config.clone(),
+                    &client_config.runtime_components,
+                ),
+            );
         }
         runtime_plugins
     }
@@ -88,22 +100,33 @@
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("GetRoleCredentials");

-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            GetRoleCredentialsRequestSerializer,
-        ));
-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            GetRoleCredentialsResponseDeserializer,
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
+                GetRoleCredentialsRequestSerializer,
+            ),
+        );
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
+                GetRoleCredentialsResponseDeserializer,
+            ),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
-            crate::config::auth::Params::builder()
-                .operation_name("GetRoleCredentials")
-                .build()
-                .expect("required fields set"),
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
+                crate::config::auth::Params::builder()
+                    .operation_name("GetRoleCredentials")
+                    .build()
+                    .expect("required fields set"),
+            ),
+        );

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::SensitiveOutput);
-        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("GetRoleCredentials", "SSO"));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
+                "GetRoleCredentials",
+                "SSO",
+            ),
+        );

         ::std::option::Option::Some(cfg.freeze())
     }
@@ -111,7 +134,10 @@
     fn runtime_components(
         &self,
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
-    ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
+    ) -> ::std::borrow::Cow<
+        '_,
+        ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
+    > {
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetRoleCredentials")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
@@ -141,7 +167,9 @@
 struct GetRoleCredentialsTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for GetRoleCredentialsTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept
+    for GetRoleCredentialsTelemetryInputCaptureInterceptor
+{
     fn name(&self) -> &'static str {
         "GetRoleCredentialsTelemetryInputCaptureInterceptor"
     }
@@ -164,7 +192,9 @@
             return ::std::result::Result::Ok(());
         };

-        let ::std::option::Option::Some(input) = context.input().downcast_ref::<GetRoleCredentialsInput>() else {
+        let ::std::option::Option::Some(input) =
+            context.input().downcast_ref::<GetRoleCredentialsInput>()
+        else {
             // A mismatched input is not this interceptor's concern; skip quietly.
             return ::std::result::Result::Ok(());
         };
@@ -187,7 +217,9 @@
 }
 #[derive(Debug)]
 struct GetRoleCredentialsResponseDeserializer;
-impl ::aws_smithy_runtime_api::client::ser_de::DeserializeResponse for GetRoleCredentialsResponseDeserializer {
+impl ::aws_smithy_runtime_api::client::ser_de::DeserializeResponse
+    for GetRoleCredentialsResponseDeserializer
+{
     fn deserialize_nonstreaming_with_config(
         &self,
         response: &::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
@@ -200,9 +232,13 @@
         let mut force_error = false;
         ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));
         let parse_result = if !success && status != 200 || force_error {
-            crate::protocol_serde::shape_get_role_credentials::de_get_role_credentials_http_error(status, headers, body)
+            crate::protocol_serde::shape_get_role_credentials::de_get_role_credentials_http_error(
+                status, headers, body,
+            )
         } else {
-            crate::protocol_serde::shape_get_role_credentials::de_get_role_credentials_http_response(status, headers, body)
+            crate::protocol_serde::shape_get_role_credentials::de_get_role_credentials_http_response(
+                status, headers, body,
+            )
         };
         crate::protocol_serde::type_erase_result(parse_result)
     }
@@ -209,13 +245,23 @@
 }
 #[derive(Debug)]
 struct GetRoleCredentialsRequestSerializer;
-impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for GetRoleCredentialsRequestSerializer {
-    #[allow(unused_mut, clippy::let_and_return, clippy::needless_borrow, clippy::useless_conversion)]
+impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest
+    for GetRoleCredentialsRequestSerializer
+{
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
             .downcast::<crate::operation::get_role_credentials::GetRoleCredentialsInput>()
             .expect("correct type");
@@ -228,36 +274,49 @@
             fn uri_base(
                 _input: &crate::operation::get_role_credentials::GetRoleCredentialsInput,
                 output: &mut ::std::string::String,
-            ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError>
+            {
                 use ::std::fmt::Write as _;
-                ::std::write!(output, "/federation/credentials").expect("formatting should succeed");
+                ::std::write!(output, "/federation/credentials")
+                    .expect("formatting should succeed");
                 ::std::result::Result::Ok(())
             }
             fn uri_query(
                 _input: &crate::operation::get_role_credentials::GetRoleCredentialsInput,
                 mut output: &mut ::std::string::String,
-            ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError>
+            {
                 let mut query = ::aws_smithy_http::query::Writer::new(output);
                 let inner_1 = &_input.role_name;
-                let inner_1 = inner_1
-                    .as_ref()
-                    .ok_or_else(|| ::aws_smithy_types::error::operation::BuildError::missing_field("role_name", "cannot be empty or unset"))?;
-                if inner_1.is_empty() {
-                    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
+                let inner_1 = inner_1.as_ref().ok_or_else(|| {
+                    ::aws_smithy_types::error::operation::BuildError::missing_field(
                         "role_name",
                         "cannot be empty or unset",
-                    ));
+                    )
+                })?;
+                if inner_1.is_empty() {
+                    return ::std::result::Result::Err(
+                        ::aws_smithy_types::error::operation::BuildError::missing_field(
+                            "role_name",
+                            "cannot be empty or unset",
+                        ),
+                    );
                 }
                 query.push_kv("role_name", &::aws_smithy_http::query::fmt_string(inner_1));
                 let inner_2 = &_input.account_id;
-                let inner_2 = inner_2
-                    .as_ref()
-                    .ok_or_else(|| ::aws_smithy_types::error::operation::BuildError::missing_field("account_id", "cannot be empty or unset"))?;
-                if inner_2.is_empty() {
-                    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
+                let inner_2 = inner_2.as_ref().ok_or_else(|| {
+                    ::aws_smithy_types::error::operation::BuildError::missing_field(
                         "account_id",
                         "cannot be empty or unset",
-                    ));
+                    )
+                })?;
+                if inner_2.is_empty() {
+                    return ::std::result::Result::Err(
+                        ::aws_smithy_types::error::operation::BuildError::missing_field(
+                            "account_id",
+                            "cannot be empty or unset",
+                        ),
+                    );
                 }
                 query.push_kv("account_id", &::aws_smithy_http::query::fmt_string(inner_2));
                 ::std::result::Result::Ok(())
@@ -266,7 +325,10 @@
             fn update_http_builder(
                 input: &crate::operation::get_role_credentials::GetRoleCredentialsInput,
                 builder: ::http_1x::request::Builder,
-            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<
+                ::http_1x::request::Builder,
+                ::aws_smithy_types::error::operation::BuildError,
+            > {
                 let mut uri = ::std::string::String::new();
                 uri_base(input, &mut uri)?;
                 uri_query(input, &mut uri)?;
@@ -278,7 +340,13 @@
         };
         let body = ::aws_smithy_types::body::SdkBody::from("");

-        ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
+        ::std::result::Result::Ok(
+            request_builder
+                .body(body)
+                .expect("valid request")
+                .try_into()
+                .unwrap(),
+        )
     }
 }
 #[derive(Debug)]
@@ -285,7 +353,9 @@
 struct GetRoleCredentialsEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for GetRoleCredentialsEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept
+    for GetRoleCredentialsEndpointParamsInterceptor
+{
     fn name(&self) -> &'static str {
         "GetRoleCredentialsEndpointParamsInterceptor"
     }
@@ -306,16 +376,32 @@
             .ok_or("failed to downcast to GetRoleCredentialsInput")?;

         let params = crate::config::endpoint::Params::builder()
-            .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
-            .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
+            .set_region(
+                cfg.load::<::aws_types::region::Region>()
+                    .map(|r| r.as_ref().to_owned()),
+            )
+            .set_use_dual_stack(
+                cfg.load::<::aws_types::endpoint_config::UseDualStack>()
+                    .map(|ty| ty.0),
+            )
+            .set_use_fips(
+                cfg.load::<::aws_types::endpoint_config::UseFips>()
+                    .map(|ty| ty.0),
+            )
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
-        cfg.interceptor_state()
-            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params));
+        cfg.interceptor_state().store_put(
+            ::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params),
+        );
         ::std::result::Result::Ok(())
     }
 }
@@ -336,18 +422,24 @@
     /// <p>Indicates that the request is not authorized. This can happen due to an invalid access token in the request.</p>
     UnauthorizedException(crate::types::error::UnauthorizedException),
     /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
-    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
+    #[deprecated(
+        note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
     variable wildcard pattern and check `.code()`:
      \
     &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
      \
-    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-GetRoleCredentialsError) for what information is available for the error.")]
+    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-GetRoleCredentialsError) for what information is available for the error."
+    )]
     Unhandled(crate::error::sealed_unhandled::Unhandled),
 }
 impl GetRoleCredentialsError {
     /// Creates the `GetRoleCredentialsError::Unhandled` variant from any error type.
     pub fn unhandled(
-        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
+        err: impl ::std::convert::Into<
+            ::std::boxed::Box<
+                dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static,
+            >,
+        >,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
             source: err.into(),
@@ -368,10 +460,18 @@
     ///
     pub fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::InvalidRequestException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::ResourceNotFoundException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::TooManyRequestsException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::UnauthorizedException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::InvalidRequestException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
+            Self::ResourceNotFoundException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
+            Self::TooManyRequestsException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
+            Self::UnauthorizedException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
             Self::Unhandled(e) => &e.meta,
         }
     }
@@ -411,7 +511,9 @@
             Self::TooManyRequestsException(_inner) => _inner.fmt(f),
             Self::UnauthorizedException(_inner) => _inner.fmt(f),
             Self::Unhandled(_inner) => {
-                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
+                if let ::std::option::Option::Some(code) =
+                    ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
+                {
                     write!(f, "unhandled error ({code})")
                 } else {
                     f.write_str("unhandled error")
@@ -431,10 +533,18 @@
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for GetRoleCredentialsError {
     fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::InvalidRequestException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::ResourceNotFoundException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::TooManyRequestsException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::UnauthorizedException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::InvalidRequestException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::ResourceNotFoundException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::TooManyRequestsException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::UnauthorizedException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
             Self::Unhandled(_inner) => &_inner.meta,
         }
     }
@@ -441,7 +551,9 @@
 }
 impl ::aws_smithy_runtime_api::client::result::CreateUnhandledError for GetRoleCredentialsError {
     fn create_unhandled_error(
-        source: ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
+        source: ::std::boxed::Box<
+            dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static,
+        >,
         meta: ::std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
@@ -450,7 +562,9 @@
         })
     }
 }
-impl ::aws_types::request_id::RequestId for crate::operation::get_role_credentials::GetRoleCredentialsError {
+impl ::aws_types::request_id::RequestId
+    for crate::operation::get_role_credentials::GetRoleCredentialsError
+{
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
     }
```

### `src/operation/get_role_credentials/_get_role_credentials_input.rs`

```diff
diff --git a/src/operation/get_role_credentials/_get_role_credentials_input.rs b/src/operation/get_role_credentials/_get_role_credentials_input.rs
--- a/src/operation/get_role_credentials/_get_role_credentials_input.rs
+++ b/src/operation/get_role_credentials/_get_role_credentials_input.rs
@@ -35,7 +35,8 @@
 }
 impl GetRoleCredentialsInput {
     /// Creates a new builder-style object to manufacture [`GetRoleCredentialsInput`](crate::operation::get_role_credentials::GetRoleCredentialsInput).
-    pub fn builder() -> crate::operation::get_role_credentials::builders::GetRoleCredentialsInputBuilder {
+    pub fn builder(
+    ) -> crate::operation::get_role_credentials::builders::GetRoleCredentialsInputBuilder {
         crate::operation::get_role_credentials::builders::GetRoleCredentialsInputBuilder::default()
     }
 }
@@ -97,13 +98,17 @@
     /// Consumes the builder and constructs a [`GetRoleCredentialsInput`](crate::operation::get_role_credentials::GetRoleCredentialsInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<crate::operation::get_role_credentials::GetRoleCredentialsInput, ::aws_smithy_types::error::operation::BuildError>
-    {
-        ::std::result::Result::Ok(crate::operation::get_role_credentials::GetRoleCredentialsInput {
-            role_name: self.role_name,
-            account_id: self.account_id,
-            access_token: self.access_token,
-        })
+    ) -> ::std::result::Result<
+        crate::operation::get_role_credentials::GetRoleCredentialsInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
+        ::std::result::Result::Ok(
+            crate::operation::get_role_credentials::GetRoleCredentialsInput {
+                role_name: self.role_name,
+                account_id: self.account_id,
+                access_token: self.access_token,
+            },
+        )
     }
 }
 impl ::std::fmt::Debug for GetRoleCredentialsInputBuilder {
```

### `src/operation/get_role_credentials/_get_role_credentials_output.rs`

```diff
diff --git a/src/operation/get_role_credentials/_get_role_credentials_output.rs b/src/operation/get_role_credentials/_get_role_credentials_output.rs
--- a/src/operation/get_role_credentials/_get_role_credentials_output.rs
+++ b/src/operation/get_role_credentials/_get_role_credentials_output.rs
@@ -28,7 +28,8 @@
 }
 impl GetRoleCredentialsOutput {
     /// Creates a new builder-style object to manufacture [`GetRoleCredentialsOutput`](crate::operation::get_role_credentials::GetRoleCredentialsOutput).
-    pub fn builder() -> crate::operation::get_role_credentials::builders::GetRoleCredentialsOutputBuilder {
+    pub fn builder(
+    ) -> crate::operation::get_role_credentials::builders::GetRoleCredentialsOutputBuilder {
         crate::operation::get_role_credentials::builders::GetRoleCredentialsOutputBuilder::default()
     }
 }
@@ -47,7 +48,10 @@
         self
     }
     /// <p>The credentials for the role that is assigned to the user.</p>
-    pub fn set_role_credentials(mut self, input: ::std::option::Option<crate::types::RoleCredentials>) -> Self {
+    pub fn set_role_credentials(
+        mut self,
+        input: ::std::option::Option<crate::types::RoleCredentials>,
+    ) -> Self {
         self.role_credentials = input;
         self
     }
```

### `src/operation/get_role_credentials/builders.rs`

```diff
diff --git a/src/operation/get_role_credentials/builders.rs b/src/operation/get_role_credentials/builders.rs
--- a/src/operation/get_role_credentials/builders.rs
+++ b/src/operation/get_role_credentials/builders.rs
@@ -57,7 +57,9 @@
         }
     }
     /// Access the GetRoleCredentials as a reference.
-    pub fn as_input(&self) -> &crate::operation::get_role_credentials::builders::GetRoleCredentialsInputBuilder {
+    pub fn as_input(
+        &self,
+    ) -> &crate::operation::get_role_credentials::builders::GetRoleCredentialsInputBuilder {
         &self.inner
     }
     /// Sends the request and returns the response.
@@ -81,12 +83,17 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = crate::operation::get_role_credentials::GetRoleCredentials::operation_runtime_plugins(
-            self.handle.runtime_plugins.clone(),
-            &self.handle.conf,
-            self.config_override,
-        );
-        crate::operation::get_role_credentials::GetRoleCredentials::orchestrate(&runtime_plugins, input).await
+        let runtime_plugins =
+            crate::operation::get_role_credentials::GetRoleCredentials::operation_runtime_plugins(
+                self.handle.runtime_plugins.clone(),
+                &self.handle.conf,
+                self.config_override,
+            );
+        crate::operation::get_role_credentials::GetRoleCredentials::orchestrate(
+            &runtime_plugins,
+            input,
+        )
+        .await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -99,12 +106,18 @@
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

### `src/operation/list_account_roles.rs`

```diff
diff --git a/src/operation/list_account_roles.rs b/src/operation/list_account_roles.rs
--- a/src/operation/list_account_roles.rs
+++ b/src/operation/list_account_roles.rs
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
-        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("SSO", "ListAccountRoles", input, runtime_plugins, stop_point)
-            // Create a parent span for the entire operation. Includes a random, internal-only,
-            // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
-            .instrument(::tracing::debug_span!(
-                "SSO.ListAccountRoles",
-                "rpc.service" = "SSO",
-                "rpc.method" = "ListAccountRoles",
-                "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
-                "rpc.system" = "aws-api",
-            ))
-            .await
+        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point(
+            "SSO",
+            "ListAccountRoles",
+            input,
+            runtime_plugins,
+            stop_point,
+        )
+        // Create a parent span for the entire operation. Includes a random, internal-only,
+        // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
+        .instrument(::tracing::debug_span!(
+            "SSO.ListAccountRoles",
+            "rpc.service" = "SSO",
+            "rpc.method" = "ListAccountRoles",
+            "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
+            "rpc.system" = "aws-api",
+        ))
+        .await
     }

     pub(crate) fn operation_runtime_plugins(
@@ -75,11 +85,13 @@
             for plugin in config_override.runtime_plugins.iter().cloned() {
                 runtime_plugins = runtime_plugins.with_operation_plugin(plugin);
             }
-            runtime_plugins = runtime_plugins.with_operation_plugin(crate::config::ConfigOverrideRuntimePlugin::new(
-                config_override,
-                client_config.config.clone(),
-                &client_config.runtime_components,
-            ));
+            runtime_plugins = runtime_plugins.with_operation_plugin(
+                crate::config::ConfigOverrideRuntimePlugin::new(
+                    config_override,
+                    client_config.config.clone(),
+                    &client_config.runtime_components,
+                ),
+            );
         }
         runtime_plugins
     }
@@ -88,21 +100,32 @@
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("ListAccountRoles");

-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            ListAccountRolesRequestSerializer,
-        ));
-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            ListAccountRolesResponseDeserializer,
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
+                ListAccountRolesRequestSerializer,
+            ),
+        );
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
+                ListAccountRolesResponseDeserializer,
+            ),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
-            crate::config::auth::Params::builder()
-                .operation_name("ListAccountRoles")
-                .build()
-                .expect("required fields set"),
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
+                crate::config::auth::Params::builder()
+                    .operation_name("ListAccountRoles")
+                    .build()
+                    .expect("required fields set"),
+            ),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("ListAccountRoles", "SSO"));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
+                "ListAccountRoles",
+                "SSO",
+            ),
+        );

         ::std::option::Option::Some(cfg.freeze())
     }
@@ -110,7 +133,10 @@
     fn runtime_components(
         &self,
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
-    ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
+    ) -> ::std::borrow::Cow<
+        '_,
+        ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
+    > {
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ListAccountRoles")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
@@ -140,7 +166,9 @@
 struct ListAccountRolesTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for ListAccountRolesTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept
+    for ListAccountRolesTelemetryInputCaptureInterceptor
+{
     fn name(&self) -> &'static str {
         "ListAccountRolesTelemetryInputCaptureInterceptor"
     }
@@ -163,7 +191,9 @@
             return ::std::result::Result::Ok(());
         };

-        let ::std::option::Option::Some(input) = context.input().downcast_ref::<ListAccountRolesInput>() else {
+        let ::std::option::Option::Some(input) =
+            context.input().downcast_ref::<ListAccountRolesInput>()
+        else {
             // A mismatched input is not this interceptor's concern; skip quietly.
             return ::std::result::Result::Ok(());
         };
@@ -186,7 +216,9 @@
 }
 #[derive(Debug)]
 struct ListAccountRolesResponseDeserializer;
-impl ::aws_smithy_runtime_api::client::ser_de::DeserializeResponse for ListAccountRolesResponseDeserializer {
+impl ::aws_smithy_runtime_api::client::ser_de::DeserializeResponse
+    for ListAccountRolesResponseDeserializer
+{
     fn deserialize_nonstreaming_with_config(
         &self,
         response: &::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
@@ -199,9 +231,13 @@
         let mut force_error = false;
         ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));
         let parse_result = if !success && status != 200 || force_error {
-            crate::protocol_serde::shape_list_account_roles::de_list_account_roles_http_error(status, headers, body)
+            crate::protocol_serde::shape_list_account_roles::de_list_account_roles_http_error(
+                status, headers, body,
+            )
         } else {
-            crate::protocol_serde::shape_list_account_roles::de_list_account_roles_http_response(status, headers, body)
+            crate::protocol_serde::shape_list_account_roles::de_list_account_roles_http_response(
+                status, headers, body,
+            )
         };
         crate::protocol_serde::type_erase_result(parse_result)
     }
@@ -208,13 +244,23 @@
 }
 #[derive(Debug)]
 struct ListAccountRolesRequestSerializer;
-impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for ListAccountRolesRequestSerializer {
-    #[allow(unused_mut, clippy::let_and_return, clippy::needless_borrow, clippy::useless_conversion)]
+impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest
+    for ListAccountRolesRequestSerializer
+{
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
             .downcast::<crate::operation::list_account_roles::ListAccountRolesInput>()
             .expect("correct type");
@@ -227,7 +273,8 @@
             fn uri_base(
                 _input: &crate::operation::list_account_roles::ListAccountRolesInput,
                 output: &mut ::std::string::String,
-            ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError>
+            {
                 use ::std::fmt::Write as _;
                 ::std::write!(output, "/assignment/roles").expect("formatting should succeed");
                 ::std::result::Result::Ok(())
@@ -235,7 +282,8 @@
             fn uri_query(
                 _input: &crate::operation::list_account_roles::ListAccountRolesInput,
                 mut output: &mut ::std::string::String,
-            ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError>
+            {
                 let mut query = ::aws_smithy_http::query::Writer::new(output);
                 if let ::std::option::Option::Some(inner_1) = &_input.next_token {
                     {
@@ -244,18 +292,26 @@
                 }
                 if let ::std::option::Option::Some(inner_2) = &_input.max_results {
                     {
-                        query.push_kv("max_result", ::aws_smithy_types::primitive::Encoder::from(*inner_2).encode());
+                        query.push_kv(
+                            "max_result",
+                            ::aws_smithy_types::primitive::Encoder::from(*inner_2).encode(),
+                        );
                     }
                 }
                 let inner_3 = &_input.account_id;
-                let inner_3 = inner_3
-                    .as_ref()
-                    .ok_or_else(|| ::aws_smithy_types::error::operation::BuildError::missing_field("account_id", "cannot be empty or unset"))?;
-                if inner_3.is_empty() {
-                    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
+                let inner_3 = inner_3.as_ref().ok_or_else(|| {
+                    ::aws_smithy_types::error::operation::BuildError::missing_field(
                         "account_id",
                         "cannot be empty or unset",
-                    ));
+                    )
+                })?;
+                if inner_3.is_empty() {
+                    return ::std::result::Result::Err(
+                        ::aws_smithy_types::error::operation::BuildError::missing_field(
+                            "account_id",
+                            "cannot be empty or unset",
+                        ),
+                    );
                 }
                 query.push_kv("account_id", &::aws_smithy_http::query::fmt_string(inner_3));
                 ::std::result::Result::Ok(())
@@ -264,7 +320,10 @@
             fn update_http_builder(
                 input: &crate::operation::list_account_roles::ListAccountRolesInput,
                 builder: ::http_1x::request::Builder,
-            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<
+                ::http_1x::request::Builder,
+                ::aws_smithy_types::error::operation::BuildError,
+            > {
                 let mut uri = ::std::string::String::new();
                 uri_base(input, &mut uri)?;
                 uri_query(input, &mut uri)?;
@@ -276,7 +335,13 @@
         };
         let body = ::aws_smithy_types::body::SdkBody::from("");

-        ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
+        ::std::result::Result::Ok(
+            request_builder
+                .body(body)
+                .expect("valid request")
+                .try_into()
+                .unwrap(),
+        )
     }
 }
 #[derive(Debug)]
@@ -283,7 +348,9 @@
 struct ListAccountRolesEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for ListAccountRolesEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept
+    for ListAccountRolesEndpointParamsInterceptor
+{
     fn name(&self) -> &'static str {
         "ListAccountRolesEndpointParamsInterceptor"
     }
@@ -304,16 +371,32 @@
             .ok_or("failed to downcast to ListAccountRolesInput")?;

         let params = crate::config::endpoint::Params::builder()
-            .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
-            .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
+            .set_region(
+                cfg.load::<::aws_types::region::Region>()
+                    .map(|r| r.as_ref().to_owned()),
+            )
+            .set_use_dual_stack(
+                cfg.load::<::aws_types::endpoint_config::UseDualStack>()
+                    .map(|ty| ty.0),
+            )
+            .set_use_fips(
+                cfg.load::<::aws_types::endpoint_config::UseFips>()
+                    .map(|ty| ty.0),
+            )
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
-        cfg.interceptor_state()
-            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params));
+        cfg.interceptor_state().store_put(
+            ::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params),
+        );
         ::std::result::Result::Ok(())
     }
 }
@@ -334,18 +417,24 @@
     /// <p>Indicates that the request is not authorized. This can happen due to an invalid access token in the request.</p>
     UnauthorizedException(crate::types::error::UnauthorizedException),
     /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
-    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
+    #[deprecated(
+        note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
     variable wildcard pattern and check `.code()`:
      \
     &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
      \
-    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-ListAccountRolesError) for what information is available for the error.")]
+    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-ListAccountRolesError) for what information is available for the error."
+    )]
     Unhandled(crate::error::sealed_unhandled::Unhandled),
 }
 impl ListAccountRolesError {
     /// Creates the `ListAccountRolesError::Unhandled` variant from any error type.
     pub fn unhandled(
-        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
+        err: impl ::std::convert::Into<
+            ::std::boxed::Box<
+                dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static,
+            >,
+        >,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
             source: err.into(),
@@ -366,10 +455,18 @@
     ///
     pub fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::InvalidRequestException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::ResourceNotFoundException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::TooManyRequestsException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::UnauthorizedException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::InvalidRequestException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
+            Self::ResourceNotFoundException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
+            Self::TooManyRequestsException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
+            Self::UnauthorizedException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
             Self::Unhandled(e) => &e.meta,
         }
     }
@@ -409,7 +506,9 @@
             Self::TooManyRequestsException(_inner) => _inner.fmt(f),
             Self::UnauthorizedException(_inner) => _inner.fmt(f),
             Self::Unhandled(_inner) => {
-                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
+                if let ::std::option::Option::Some(code) =
+                    ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
+                {
                     write!(f, "unhandled error ({code})")
                 } else {
                     f.write_str("unhandled error")
@@ -429,10 +528,18 @@
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for ListAccountRolesError {
     fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::InvalidRequestException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::ResourceNotFoundException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::TooManyRequestsException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::UnauthorizedException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::InvalidRequestException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::ResourceNotFoundException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::TooManyRequestsException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::UnauthorizedException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
             Self::Unhandled(_inner) => &_inner.meta,
         }
     }
@@ -439,7 +546,9 @@
 }
 impl ::aws_smithy_runtime_api::client::result::CreateUnhandledError for ListAccountRolesError {
     fn create_unhandled_error(
-        source: ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
+        source: ::std::boxed::Box<
+            dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static,
+        >,
         meta: ::std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
@@ -448,7 +557,9 @@
         })
     }
 }
-impl ::aws_types::request_id::RequestId for crate::operation::list_account_roles::ListAccountRolesError {
+impl ::aws_types::request_id::RequestId
+    for crate::operation::list_account_roles::ListAccountRolesError
+{
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
     }
```

### `src/operation/list_account_roles/_list_account_roles_input.rs`

```diff
diff --git a/src/operation/list_account_roles/_list_account_roles_input.rs b/src/operation/list_account_roles/_list_account_roles_input.rs
--- a/src/operation/list_account_roles/_list_account_roles_input.rs
+++ b/src/operation/list_account_roles/_list_account_roles_input.rs
@@ -42,7 +42,8 @@
 }
 impl ListAccountRolesInput {
     /// Creates a new builder-style object to manufacture [`ListAccountRolesInput`](crate::operation::list_account_roles::ListAccountRolesInput).
-    pub fn builder() -> crate::operation::list_account_roles::builders::ListAccountRolesInputBuilder {
+    pub fn builder() -> crate::operation::list_account_roles::builders::ListAccountRolesInputBuilder
+    {
         crate::operation::list_account_roles::builders::ListAccountRolesInputBuilder::default()
     }
 }
@@ -118,13 +119,18 @@
     /// Consumes the builder and constructs a [`ListAccountRolesInput`](crate::operation::list_account_roles::ListAccountRolesInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<crate::operation::list_account_roles::ListAccountRolesInput, ::aws_smithy_types::error::operation::BuildError> {
-        ::std::result::Result::Ok(crate::operation::list_account_roles::ListAccountRolesInput {
-            next_token: self.next_token,
-            max_results: self.max_results,
-            access_token: self.access_token,
-            account_id: self.account_id,
-        })
+    ) -> ::std::result::Result<
+        crate::operation::list_account_roles::ListAccountRolesInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
+        ::std::result::Result::Ok(
+            crate::operation::list_account_roles::ListAccountRolesInput {
+                next_token: self.next_token,
+                max_results: self.max_results,
+                access_token: self.access_token,
+                account_id: self.account_id,
+            },
+        )
     }
 }
 impl ::std::fmt::Debug for ListAccountRolesInputBuilder {
```

### `src/operation/list_account_roles/_list_account_roles_output.rs`

```diff
diff --git a/src/operation/list_account_roles/_list_account_roles_output.rs b/src/operation/list_account_roles/_list_account_roles_output.rs
--- a/src/operation/list_account_roles/_list_account_roles_output.rs
+++ b/src/operation/list_account_roles/_list_account_roles_output.rs
@@ -28,13 +28,16 @@
 }
 impl ListAccountRolesOutput {
     /// Creates a new builder-style object to manufacture [`ListAccountRolesOutput`](crate::operation::list_account_roles::ListAccountRolesOutput).
-    pub fn builder() -> crate::operation::list_account_roles::builders::ListAccountRolesOutputBuilder {
+    pub fn builder() -> crate::operation::list_account_roles::builders::ListAccountRolesOutputBuilder
+    {
         crate::operation::list_account_roles::builders::ListAccountRolesOutputBuilder::default()
     }
 }

 /// A builder for [`ListAccountRolesOutput`](crate::operation::list_account_roles::ListAccountRolesOutput).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(
+    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
+)]
 #[non_exhaustive]
 pub struct ListAccountRolesOutputBuilder {
     pub(crate) next_token: ::std::option::Option<::std::string::String>,
@@ -68,7 +71,10 @@
         self
     }
     /// <p>A paginated response with the list of roles and the next token if more results are available.</p>
-    pub fn set_role_list(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::RoleInfo>>) -> Self {
+    pub fn set_role_list(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<crate::types::RoleInfo>>,
+    ) -> Self {
         self.role_list = input;
         self
     }
```

### `src/operation/list_account_roles/builders.rs`

```diff
diff --git a/src/operation/list_account_roles/builders.rs b/src/operation/list_account_roles/builders.rs
--- a/src/operation/list_account_roles/builders.rs
+++ b/src/operation/list_account_roles/builders.rs
@@ -57,7 +57,9 @@
         }
     }
     /// Access the ListAccountRoles as a reference.
-    pub fn as_input(&self) -> &crate::operation::list_account_roles::builders::ListAccountRolesInputBuilder {
+    pub fn as_input(
+        &self,
+    ) -> &crate::operation::list_account_roles::builders::ListAccountRolesInputBuilder {
         &self.inner
     }
     /// Sends the request and returns the response.
@@ -81,12 +83,14 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = crate::operation::list_account_roles::ListAccountRoles::operation_runtime_plugins(
-            self.handle.runtime_plugins.clone(),
-            &self.handle.conf,
-            self.config_override,
-        );
-        crate::operation::list_account_roles::ListAccountRoles::orchestrate(&runtime_plugins, input).await
+        let runtime_plugins =
+            crate::operation::list_account_roles::ListAccountRoles::operation_runtime_plugins(
+                self.handle.runtime_plugins.clone(),
+                &self.handle.conf,
+                self.config_override,
+            );
+        crate::operation::list_account_roles::ListAccountRoles::orchestrate(&runtime_plugins, input)
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
@@ -111,8 +121,13 @@
     /// Create a paginator for this request
     ///
     /// Paginators are used by calling [`send().await`](crate::operation::list_account_roles::paginator::ListAccountRolesPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
-    pub fn into_paginator(self) -> crate::operation::list_account_roles::paginator::ListAccountRolesPaginator {
-        crate::operation::list_account_roles::paginator::ListAccountRolesPaginator::new(self.handle, self.inner)
+    pub fn into_paginator(
+        self,
+    ) -> crate::operation::list_account_roles::paginator::ListAccountRolesPaginator {
+        crate::operation::list_account_roles::paginator::ListAccountRolesPaginator::new(
+            self.handle,
+            self.inner,
+        )
     }
     /// <p>The page token from the previous response output when you request subsequent pages.</p>
     pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
```

### `src/operation/list_account_roles/paginator.rs`

```diff
diff --git a/src/operation/list_account_roles/paginator.rs b/src/operation/list_account_roles/paginator.rs
--- a/src/operation/list_account_roles/paginator.rs
+++ b/src/operation/list_account_roles/paginator.rs
@@ -31,7 +31,9 @@
     ///
     /// This paginator automatically flattens results using `role_list`. Queries to the underlying service
     /// are dispatched lazily.
-    pub fn items(self) -> crate::operation::list_account_roles::paginator::ListAccountRolesPaginatorItems {
+    pub fn items(
+        self,
+    ) -> crate::operation::list_account_roles::paginator::ListAccountRolesPaginatorItems {
         crate::operation::list_account_roles::paginator::ListAccountRolesPaginatorItems(self)
     }

@@ -65,20 +67,22 @@
         // Move individual fields out of self for the borrow checker
         let builder = self.builder;
         let handle = self.handle;
-        let runtime_plugins = crate::operation::list_account_roles::ListAccountRoles::operation_runtime_plugins(
-            handle.runtime_plugins.clone(),
-            &handle.conf,
-            ::std::option::Option::None,
-        )
-        .with_operation_plugin(crate::sdk_feature_tracker::paginator::PaginatorFeatureTrackerRuntimePlugin::new());
-        ::aws_smithy_async::future::pagination_stream::PaginationStream::new(::aws_smithy_async::future::pagination_stream::fn_stream::FnStream::new(
-            move |tx| {
+        let runtime_plugins =
+            crate::operation::list_account_roles::ListAccountRoles::operation_runtime_plugins(
+                handle.runtime_plugins.clone(),
+                &handle.conf,
+                ::std::option::Option::None,
+            )
+            .with_operation_plugin(
+                crate::sdk_feature_tracker::paginator::PaginatorFeatureTrackerRuntimePlugin::new(),
+            );
+        ::aws_smithy_async::future::pagination_stream::PaginationStream::new(
+            ::aws_smithy_async::future::pagination_stream::fn_stream::FnStream::new(move |tx| {
                 ::std::boxed::Box::pin(async move {
                     // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
-                    let mut input = match builder
-                        .build()
-                        .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)
-                    {
+                    let mut input = match builder.build().map_err(
+                        ::aws_smithy_runtime_api::client::result::SdkError::construction_failure,
+                    ) {
                         ::std::result::Result::Ok(input) => input,
                         ::std::result::Result::Err(e) => {
                             let _ = tx.send(::std::result::Result::Err(e)).await;
@@ -86,14 +90,23 @@
                         }
                     };
                     loop {
-                        let resp = crate::operation::list_account_roles::ListAccountRoles::orchestrate(&runtime_plugins, input.clone()).await;
+                        let resp =
+                            crate::operation::list_account_roles::ListAccountRoles::orchestrate(
+                                &runtime_plugins,
+                                input.clone(),
+                            )
+                            .await;
                         // If the input member is None or it was an error
                         let done = match resp {
                             ::std::result::Result::Ok(ref resp) => {
                                 let new_token = crate::lens::reflens_list_account_roles_output_output_next_token(resp);
                                 // Pagination is exhausted when the next token is an empty string
-                                let is_empty = new_token.map(|token| token.is_empty()).unwrap_or(true);
-                                if !is_empty && new_token == input.next_token.as_ref() && self.stop_on_duplicate_token {
+                                let is_empty =
+                                    new_token.map(|token| token.is_empty()).unwrap_or(true);
+                                if !is_empty
+                                    && new_token == input.next_token.as_ref()
+                                    && self.stop_on_duplicate_token
+                                {
                                     true
                                 } else {
                                     input.next_token = new_token.cloned();
@@ -111,8 +124,8 @@
                         }
                     }
                 })
-            },
-        ))
+            }),
+        )
     }
 }

@@ -139,10 +152,12 @@
             >,
         >,
     > {
-        ::aws_smithy_async::future::pagination_stream::TryFlatMap::new(self.0.send()).flat_map(|page| {
-            crate::lens::lens_list_account_roles_output_output_role_list(page)
-                .unwrap_or_default()
-                .into_iter()
-        })
+        ::aws_smithy_async::future::pagination_stream::TryFlatMap::new(self.0.send()).flat_map(
+            |page| {
+                crate::lens::lens_list_account_roles_output_output_role_list(page)
+                    .unwrap_or_default()
+                    .into_iter()
+            },
+        )
     }
 }
```

### `src/operation/list_accounts.rs`

```diff
diff --git a/src/operation/list_accounts.rs b/src/operation/list_accounts.rs
--- a/src/operation/list_accounts.rs
+++ b/src/operation/list_accounts.rs
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
-        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("SSO", "ListAccounts", input, runtime_plugins, stop_point)
-            // Create a parent span for the entire operation. Includes a random, internal-only,
-            // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
-            .instrument(::tracing::debug_span!(
-                "SSO.ListAccounts",
-                "rpc.service" = "SSO",
-                "rpc.method" = "ListAccounts",
-                "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
-                "rpc.system" = "aws-api",
-            ))
-            .await
+        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point(
+            "SSO",
+            "ListAccounts",
+            input,
+            runtime_plugins,
+            stop_point,
+        )
+        // Create a parent span for the entire operation. Includes a random, internal-only,
+        // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
+        .instrument(::tracing::debug_span!(
+            "SSO.ListAccounts",
+            "rpc.service" = "SSO",
+            "rpc.method" = "ListAccounts",
+            "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
+            "rpc.system" = "aws-api",
+        ))
+        .await
     }

     pub(crate) fn operation_runtime_plugins(
@@ -75,11 +85,13 @@
             for plugin in config_override.runtime_plugins.iter().cloned() {
                 runtime_plugins = runtime_plugins.with_operation_plugin(plugin);
             }
-            runtime_plugins = runtime_plugins.with_operation_plugin(crate::config::ConfigOverrideRuntimePlugin::new(
-                config_override,
-                client_config.config.clone(),
-                &client_config.runtime_components,
-            ));
+            runtime_plugins = runtime_plugins.with_operation_plugin(
+                crate::config::ConfigOverrideRuntimePlugin::new(
+                    config_override,
+                    client_config.config.clone(),
+                    &client_config.runtime_components,
+                ),
+            );
         }
         runtime_plugins
     }
@@ -88,21 +100,29 @@
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("ListAccounts");

-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            ListAccountsRequestSerializer,
-        ));
-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            ListAccountsResponseDeserializer,
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
+                ListAccountsRequestSerializer,
+            ),
+        );
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
+                ListAccountsResponseDeserializer,
+            ),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
-            crate::config::auth::Params::builder()
-                .operation_name("ListAccounts")
-                .build()
-                .expect("required fields set"),
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
+                crate::config::auth::Params::builder()
+                    .operation_name("ListAccounts")
+                    .build()
+                    .expect("required fields set"),
+            ),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("ListAccounts", "SSO"));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::orchestrator::Metadata::new("ListAccounts", "SSO"),
+        );

         ::std::option::Option::Some(cfg.freeze())
     }
@@ -110,7 +130,10 @@
     fn runtime_components(
         &self,
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
-    ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
+    ) -> ::std::borrow::Cow<
+        '_,
+        ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
+    > {
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ListAccounts")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
@@ -140,7 +163,9 @@
 struct ListAccountsTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for ListAccountsTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept
+    for ListAccountsTelemetryInputCaptureInterceptor
+{
     fn name(&self) -> &'static str {
         "ListAccountsTelemetryInputCaptureInterceptor"
     }
@@ -163,7 +188,9 @@
             return ::std::result::Result::Ok(());
         };

-        let ::std::option::Option::Some(input) = context.input().downcast_ref::<ListAccountsInput>() else {
+        let ::std::option::Option::Some(input) =
+            context.input().downcast_ref::<ListAccountsInput>()
+        else {
             // A mismatched input is not this interceptor's concern; skip quietly.
             return ::std::result::Result::Ok(());
         };
@@ -181,7 +208,9 @@
 }
 #[derive(Debug)]
 struct ListAccountsResponseDeserializer;
-impl ::aws_smithy_runtime_api::client::ser_de::DeserializeResponse for ListAccountsResponseDeserializer {
+impl ::aws_smithy_runtime_api::client::ser_de::DeserializeResponse
+    for ListAccountsResponseDeserializer
+{
     fn deserialize_nonstreaming_with_config(
         &self,
         response: &::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
@@ -194,9 +223,13 @@
         let mut force_error = false;
         ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));
         let parse_result = if !success && status != 200 || force_error {
-            crate::protocol_serde::shape_list_accounts::de_list_accounts_http_error(status, headers, body)
+            crate::protocol_serde::shape_list_accounts::de_list_accounts_http_error(
+                status, headers, body,
+            )
         } else {
-            crate::protocol_serde::shape_list_accounts::de_list_accounts_http_response(status, headers, body)
+            crate::protocol_serde::shape_list_accounts::de_list_accounts_http_response(
+                status, headers, body,
+            )
         };
         crate::protocol_serde::type_erase_result(parse_result)
     }
@@ -204,12 +237,20 @@
 #[derive(Debug)]
 struct ListAccountsRequestSerializer;
 impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for ListAccountsRequestSerializer {
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
             .downcast::<crate::operation::list_accounts::ListAccountsInput>()
             .expect("correct type");
@@ -222,7 +263,8 @@
             fn uri_base(
                 _input: &crate::operation::list_accounts::ListAccountsInput,
                 output: &mut ::std::string::String,
-            ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError>
+            {
                 use ::std::fmt::Write as _;
                 ::std::write!(output, "/assignment/accounts").expect("formatting should succeed");
                 ::std::result::Result::Ok(())
@@ -230,7 +272,8 @@
             fn uri_query(
                 _input: &crate::operation::list_accounts::ListAccountsInput,
                 mut output: &mut ::std::string::String,
-            ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError>
+            {
                 let mut query = ::aws_smithy_http::query::Writer::new(output);
                 if let ::std::option::Option::Some(inner_1) = &_input.next_token {
                     {
@@ -239,7 +282,10 @@
                 }
                 if let ::std::option::Option::Some(inner_2) = &_input.max_results {
                     {
-                        query.push_kv("max_result", ::aws_smithy_types::primitive::Encoder::from(*inner_2).encode());
+                        query.push_kv(
+                            "max_result",
+                            ::aws_smithy_types::primitive::Encoder::from(*inner_2).encode(),
+                        );
                     }
                 }
                 ::std::result::Result::Ok(())
@@ -248,11 +294,17 @@
             fn update_http_builder(
                 input: &crate::operation::list_accounts::ListAccountsInput,
                 builder: ::http_1x::request::Builder,
-            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<
+                ::http_1x::request::Builder,
+                ::aws_smithy_types::error::operation::BuildError,
+            > {
                 let mut uri = ::std::string::String::new();
                 uri_base(input, &mut uri)?;
                 uri_query(input, &mut uri)?;
-                let builder = crate::protocol_serde::shape_list_accounts::ser_list_accounts_headers(input, builder)?;
+                let builder =
+                    crate::protocol_serde::shape_list_accounts::ser_list_accounts_headers(
+                        input, builder,
+                    )?;
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
@@ -260,7 +312,13 @@
         };
         let body = ::aws_smithy_types::body::SdkBody::from("");

-        ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
+        ::std::result::Result::Ok(
+            request_builder
+                .body(body)
+                .expect("valid request")
+                .try_into()
+                .unwrap(),
+        )
     }
 }
 #[derive(Debug)]
@@ -267,7 +325,9 @@
 struct ListAccountsEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for ListAccountsEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept
+    for ListAccountsEndpointParamsInterceptor
+{
     fn name(&self) -> &'static str {
         "ListAccountsEndpointParamsInterceptor"
     }
@@ -288,16 +348,32 @@
             .ok_or("failed to downcast to ListAccountsInput")?;

         let params = crate::config::endpoint::Params::builder()
-            .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
-            .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
+            .set_region(
+                cfg.load::<::aws_types::region::Region>()
+                    .map(|r| r.as_ref().to_owned()),
+            )
+            .set_use_dual_stack(
+                cfg.load::<::aws_types::endpoint_config::UseDualStack>()
+                    .map(|ty| ty.0),
+            )
+            .set_use_fips(
+                cfg.load::<::aws_types::endpoint_config::UseFips>()
+                    .map(|ty| ty.0),
+            )
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
-        cfg.interceptor_state()
-            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params));
+        cfg.interceptor_state().store_put(
+            ::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params),
+        );
         ::std::result::Result::Ok(())
     }
 }
@@ -318,18 +394,24 @@
     /// <p>Indicates that the request is not authorized. This can happen due to an invalid access token in the request.</p>
     UnauthorizedException(crate::types::error::UnauthorizedException),
     /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
-    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
+    #[deprecated(
+        note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
     variable wildcard pattern and check `.code()`:
      \
     &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
      \
-    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-ListAccountsError) for what information is available for the error.")]
+    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-ListAccountsError) for what information is available for the error."
+    )]
     Unhandled(crate::error::sealed_unhandled::Unhandled),
 }
 impl ListAccountsError {
     /// Creates the `ListAccountsError::Unhandled` variant from any error type.
     pub fn unhandled(
-        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
+        err: impl ::std::convert::Into<
+            ::std::boxed::Box<
+                dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static,
+            >,
+        >,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
             source: err.into(),
@@ -350,10 +432,18 @@
     ///
     pub fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::InvalidRequestException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::ResourceNotFoundException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::TooManyRequestsException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::UnauthorizedException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::InvalidRequestException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
+            Self::ResourceNotFoundException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
+            Self::TooManyRequestsException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
+            Self::UnauthorizedException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
             Self::Unhandled(e) => &e.meta,
         }
     }
@@ -393,7 +483,9 @@
             Self::TooManyRequestsException(_inner) => _inner.fmt(f),
             Self::UnauthorizedException(_inner) => _inner.fmt(f),
             Self::Unhandled(_inner) => {
-                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
+                if let ::std::option::Option::Some(code) =
+                    ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
+                {
                     write!(f, "unhandled error ({code})")
                 } else {
                     f.write_str("unhandled error")
@@ -413,10 +505,18 @@
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for ListAccountsError {
     fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::InvalidRequestException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::ResourceNotFoundException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::TooManyRequestsException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::UnauthorizedException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::InvalidRequestException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::ResourceNotFoundException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::TooManyRequestsException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::UnauthorizedException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
             Self::Unhandled(_inner) => &_inner.meta,
         }
     }
@@ -423,7 +523,9 @@
 }
 impl ::aws_smithy_runtime_api::client::result::CreateUnhandledError for ListAccountsError {
     fn create_unhandled_error(
-        source: ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
+        source: ::std::boxed::Box<
+            dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static,
+        >,
         meta: ::std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
```

### `src/operation/list_accounts/_list_accounts_input.rs`

```diff
diff --git a/src/operation/list_accounts/_list_accounts_input.rs b/src/operation/list_accounts/_list_accounts_input.rs
--- a/src/operation/list_accounts/_list_accounts_input.rs
+++ b/src/operation/list_accounts/_list_accounts_input.rs
@@ -95,7 +95,10 @@
     /// Consumes the builder and constructs a [`ListAccountsInput`](crate::operation::list_accounts::ListAccountsInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<crate::operation::list_accounts::ListAccountsInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        crate::operation::list_accounts::ListAccountsInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(crate::operation::list_accounts::ListAccountsInput {
             next_token: self.next_token,
             max_results: self.max_results,
```

### `src/operation/list_accounts/_list_accounts_output.rs`

```diff
diff --git a/src/operation/list_accounts/_list_accounts_output.rs b/src/operation/list_accounts/_list_accounts_output.rs
--- a/src/operation/list_accounts/_list_accounts_output.rs
+++ b/src/operation/list_accounts/_list_accounts_output.rs
@@ -34,7 +34,9 @@
 }

 /// A builder for [`ListAccountsOutput`](crate::operation::list_accounts::ListAccountsOutput).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(
+    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
+)]
 #[non_exhaustive]
 pub struct ListAccountsOutputBuilder {
     pub(crate) next_token: ::std::option::Option<::std::string::String>,
@@ -68,12 +70,17 @@
         self
     }
     /// <p>A paginated response with the list of account information and the next token if more results are available.</p>
-    pub fn set_account_list(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::AccountInfo>>) -> Self {
+    pub fn set_account_list(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<crate::types::AccountInfo>>,
+    ) -> Self {
         self.account_list = input;
         self
     }
     /// <p>A paginated response with the list of account information and the next token if more results are available.</p>
-    pub fn get_account_list(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::AccountInfo>> {
+    pub fn get_account_list(
+        &self,
+    ) -> &::std::option::Option<::std::vec::Vec<crate::types::AccountInfo>> {
         &self.account_list
     }
     pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
```

### `src/operation/list_accounts/builders.rs`

```diff
diff --git a/src/operation/list_accounts/builders.rs b/src/operation/list_accounts/builders.rs
--- a/src/operation/list_accounts/builders.rs
+++ b/src/operation/list_accounts/builders.rs
@@ -81,11 +81,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = crate::operation::list_accounts::ListAccounts::operation_runtime_plugins(
-            self.handle.runtime_plugins.clone(),
-            &self.handle.conf,
-            self.config_override,
-        );
+        let runtime_plugins =
+            crate::operation::list_accounts::ListAccounts::operation_runtime_plugins(
+                self.handle.runtime_plugins.clone(),
+                &self.handle.conf,
+                self.config_override,
+            );
         crate::operation::list_accounts::ListAccounts::orchestrate(&runtime_plugins, input).await
     }

@@ -99,12 +100,18 @@
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
@@ -111,8 +118,13 @@
     /// Create a paginator for this request
     ///
     /// Paginators are used by calling [`send().await`](crate::operation::list_accounts::paginator::ListAccountsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
-    pub fn into_paginator(self) -> crate::operation::list_accounts::paginator::ListAccountsPaginator {
-        crate::operation::list_accounts::paginator::ListAccountsPaginator::new(self.handle, self.inner)
+    pub fn into_paginator(
+        self,
+    ) -> crate::operation::list_accounts::paginator::ListAccountsPaginator {
+        crate::operation::list_accounts::paginator::ListAccountsPaginator::new(
+            self.handle,
+            self.inner,
+        )
     }
     /// <p>(Optional) When requesting subsequent pages, this is the page token from the previous response output.</p>
     pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
```

### `src/operation/list_accounts/paginator.rs`

```diff
diff --git a/src/operation/list_accounts/paginator.rs b/src/operation/list_accounts/paginator.rs
--- a/src/operation/list_accounts/paginator.rs
+++ b/src/operation/list_accounts/paginator.rs
@@ -65,20 +65,22 @@
         // Move individual fields out of self for the borrow checker
         let builder = self.builder;
         let handle = self.handle;
-        let runtime_plugins = crate::operation::list_accounts::ListAccounts::operation_runtime_plugins(
-            handle.runtime_plugins.clone(),
-            &handle.conf,
-            ::std::option::Option::None,
-        )
-        .with_operation_plugin(crate::sdk_feature_tracker::paginator::PaginatorFeatureTrackerRuntimePlugin::new());
-        ::aws_smithy_async::future::pagination_stream::PaginationStream::new(::aws_smithy_async::future::pagination_stream::fn_stream::FnStream::new(
-            move |tx| {
+        let runtime_plugins =
+            crate::operation::list_accounts::ListAccounts::operation_runtime_plugins(
+                handle.runtime_plugins.clone(),
+                &handle.conf,
+                ::std::option::Option::None,
+            )
+            .with_operation_plugin(
+                crate::sdk_feature_tracker::paginator::PaginatorFeatureTrackerRuntimePlugin::new(),
+            );
+        ::aws_smithy_async::future::pagination_stream::PaginationStream::new(
+            ::aws_smithy_async::future::pagination_stream::fn_stream::FnStream::new(move |tx| {
                 ::std::boxed::Box::pin(async move {
                     // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
-                    let mut input = match builder
-                        .build()
-                        .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)
-                    {
+                    let mut input = match builder.build().map_err(
+                        ::aws_smithy_runtime_api::client::result::SdkError::construction_failure,
+                    ) {
                         ::std::result::Result::Ok(input) => input,
                         ::std::result::Result::Err(e) => {
                             let _ = tx.send(::std::result::Result::Err(e)).await;
@@ -86,14 +88,25 @@
                         }
                     };
                     loop {
-                        let resp = crate::operation::list_accounts::ListAccounts::orchestrate(&runtime_plugins, input.clone()).await;
+                        let resp = crate::operation::list_accounts::ListAccounts::orchestrate(
+                            &runtime_plugins,
+                            input.clone(),
+                        )
+                        .await;
                         // If the input member is None or it was an error
                         let done = match resp {
                             ::std::result::Result::Ok(ref resp) => {
-                                let new_token = crate::lens::reflens_list_accounts_output_output_next_token(resp);
+                                let new_token =
+                                    crate::lens::reflens_list_accounts_output_output_next_token(
+                                        resp,
+                                    );
                                 // Pagination is exhausted when the next token is an empty string
-                                let is_empty = new_token.map(|token| token.is_empty()).unwrap_or(true);
-                                if !is_empty && new_token == input.next_token.as_ref() && self.stop_on_duplicate_token {
+                                let is_empty =
+                                    new_token.map(|token| token.is_empty()).unwrap_or(true);
+                                if !is_empty
+                                    && new_token == input.next_token.as_ref()
+                                    && self.stop_on_duplicate_token
+                                {
                                     true
                                 } else {
                                     input.next_token = new_token.cloned();
@@ -111,8 +124,8 @@
                         }
                     }
                 })
-            },
-        ))
+            }),
+        )
     }
 }

@@ -139,10 +152,12 @@
             >,
         >,
     > {
-        ::aws_smithy_async::future::pagination_stream::TryFlatMap::new(self.0.send()).flat_map(|page| {
-            crate::lens::lens_list_accounts_output_output_account_list(page)
-                .unwrap_or_default()
-                .into_iter()
-        })
+        ::aws_smithy_async::future::pagination_stream::TryFlatMap::new(self.0.send()).flat_map(
+            |page| {
+                crate::lens::lens_list_accounts_output_output_account_list(page)
+                    .unwrap_or_default()
+                    .into_iter()
+            },
+        )
     }
 }
```

### `src/operation/logout.rs`

```diff
diff --git a/src/operation/logout.rs b/src/operation/logout.rs
--- a/src/operation/logout.rs
+++ b/src/operation/logout.rs
@@ -18,16 +18,28 @@
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
-        let map_err =
-            |err: ::aws_smithy_runtime_api::client::result::SdkError<
-                ::aws_smithy_runtime_api::client::interceptors::context::Error,
-                ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
-            >| { err.map_service_error(|err| err.downcast::<crate::operation::logout::LogoutError>().expect("correct error type")) };
-        let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
-            .await
-            .map_err(map_err)?;
+        let map_err = |err: ::aws_smithy_runtime_api::client::result::SdkError<
+            ::aws_smithy_runtime_api::client::interceptors::context::Error,
+            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
+        >| {
+            err.map_service_error(|err| {
+                err.downcast::<crate::operation::logout::LogoutError>()
+                    .expect("correct error type")
+            })
+        };
+        let context = Self::orchestrate_with_stop_point(
+            runtime_plugins,
+            input,
+            ::aws_smithy_runtime::client::orchestrator::StopPoint::None,
+        )
+        .await
+        .map_err(map_err)?;
         let output = context.finalize().map_err(map_err)?;
-        ::std::result::Result::Ok(output.downcast::<crate::operation::logout::LogoutOutput>().expect("correct output type"))
+        ::std::result::Result::Ok(
+            output
+                .downcast::<crate::operation::logout::LogoutOutput>()
+                .expect("correct output type"),
+        )
     }

     pub(crate) async fn orchestrate_with_stop_point(
@@ -43,17 +55,23 @@
     > {
         let input = ::aws_smithy_runtime_api::client::interceptors::context::Input::erase(input);
         use ::tracing::Instrument;
-        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("SSO", "Logout", input, runtime_plugins, stop_point)
-            // Create a parent span for the entire operation. Includes a random, internal-only,
-            // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
-            .instrument(::tracing::debug_span!(
-                "SSO.Logout",
-                "rpc.service" = "SSO",
-                "rpc.method" = "Logout",
-                "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
-                "rpc.system" = "aws-api",
-            ))
-            .await
+        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point(
+            "SSO",
+            "Logout",
+            input,
+            runtime_plugins,
+            stop_point,
+        )
+        // Create a parent span for the entire operation. Includes a random, internal-only,
+        // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
+        .instrument(::tracing::debug_span!(
+            "SSO.Logout",
+            "rpc.service" = "SSO",
+            "rpc.method" = "Logout",
+            "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
+            "rpc.system" = "aws-api",
+        ))
+        .await
     }

     pub(crate) fn operation_runtime_plugins(
@@ -67,11 +85,13 @@
             for plugin in config_override.runtime_plugins.iter().cloned() {
                 runtime_plugins = runtime_plugins.with_operation_plugin(plugin);
             }
-            runtime_plugins = runtime_plugins.with_operation_plugin(crate::config::ConfigOverrideRuntimePlugin::new(
-                config_override,
-                client_config.config.clone(),
-                &client_config.runtime_components,
-            ));
+            runtime_plugins = runtime_plugins.with_operation_plugin(
+                crate::config::ConfigOverrideRuntimePlugin::new(
+                    config_override,
+                    client_config.config.clone(),
+                    &client_config.runtime_components,
+                ),
+            );
         }
         runtime_plugins
     }
@@ -80,21 +100,29 @@
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("Logout");

-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            LogoutRequestSerializer,
-        ));
-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            LogoutResponseDeserializer,
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
+                LogoutRequestSerializer,
+            ),
+        );
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
+                LogoutResponseDeserializer,
+            ),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
-            crate::config::auth::Params::builder()
-                .operation_name("Logout")
-                .build()
-                .expect("required fields set"),
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
+                crate::config::auth::Params::builder()
+                    .operation_name("Logout")
+                    .build()
+                    .expect("required fields set"),
+            ),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("Logout", "SSO"));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::orchestrator::Metadata::new("Logout", "SSO"),
+        );

         ::std::option::Option::Some(cfg.freeze())
     }
@@ -102,7 +130,10 @@
     fn runtime_components(
         &self,
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
-    ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
+    ) -> ::std::borrow::Cow<
+        '_,
+        ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
+    > {
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("Logout")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
@@ -150,13 +181,23 @@
 #[derive(Debug)]
 struct LogoutRequestSerializer;
 impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for LogoutRequestSerializer {
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
-        let input = input.downcast::<crate::operation::logout::LogoutInput>().expect("correct type");
+    ) -> ::std::result::Result<
+        ::aws_smithy_runtime_api::client::orchestrator::HttpRequest,
+        ::aws_smithy_runtime_api::box_error::BoxError,
+    > {
+        let input = input
+            .downcast::<crate::operation::logout::LogoutInput>()
+            .expect("correct type");
         let _header_serialization_settings = _cfg
             .load::<crate::serialization_settings::HeaderSerializationSettings>()
             .cloned()
@@ -166,7 +207,8 @@
             fn uri_base(
                 _input: &crate::operation::logout::LogoutInput,
                 output: &mut ::std::string::String,
-            ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError>
+            {
                 use ::std::fmt::Write as _;
                 ::std::write!(output, "/logout").expect("formatting should succeed");
                 ::std::result::Result::Ok(())
@@ -175,10 +217,14 @@
             fn update_http_builder(
                 input: &crate::operation::logout::LogoutInput,
                 builder: ::http_1x::request::Builder,
-            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<
+                ::http_1x::request::Builder,
+                ::aws_smithy_types::error::operation::BuildError,
+            > {
                 let mut uri = ::std::string::String::new();
                 uri_base(input, &mut uri)?;
-                let builder = crate::protocol_serde::shape_logout::ser_logout_headers(input, builder)?;
+                let builder =
+                    crate::protocol_serde::shape_logout::ser_logout_headers(input, builder)?;
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
@@ -186,7 +232,13 @@
         };
         let body = ::aws_smithy_types::body::SdkBody::from("");

-        ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
+        ::std::result::Result::Ok(
+            request_builder
+                .body(body)
+                .expect("valid request")
+                .try_into()
+                .unwrap(),
+        )
     }
 }
 #[derive(Debug)]
@@ -208,19 +260,38 @@
         >,
         cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
     ) -> ::std::result::Result<(), ::aws_smithy_runtime_api::box_error::BoxError> {
-        let _input = context.input().downcast_ref::<LogoutInput>().ok_or("failed to downcast to LogoutInput")?;
+        let _input = context
+            .input()
+            .downcast_ref::<LogoutInput>()
+            .ok_or("failed to downcast to LogoutInput")?;

         let params = crate::config::endpoint::Params::builder()
-            .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
-            .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
+            .set_region(
+                cfg.load::<::aws_types::region::Region>()
+                    .map(|r| r.as_ref().to_owned()),
+            )
+            .set_use_dual_stack(
+                cfg.load::<::aws_types::endpoint_config::UseDualStack>()
+                    .map(|ty| ty.0),
+            )
+            .set_use_fips(
+                cfg.load::<::aws_types::endpoint_config::UseFips>()
+                    .map(|ty| ty.0),
+            )
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
-        cfg.interceptor_state()
-            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params));
+        cfg.interceptor_state().store_put(
+            ::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params),
+        );
         ::std::result::Result::Ok(())
     }
 }
@@ -239,18 +310,24 @@
     /// <p>Indicates that the request is not authorized. This can happen due to an invalid access token in the request.</p>
     UnauthorizedException(crate::types::error::UnauthorizedException),
     /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
-    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
+    #[deprecated(
+        note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
     variable wildcard pattern and check `.code()`:
      \
     &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
      \
-    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-LogoutError) for what information is available for the error.")]
+    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-LogoutError) for what information is available for the error."
+    )]
     Unhandled(crate::error::sealed_unhandled::Unhandled),
 }
 impl LogoutError {
     /// Creates the `LogoutError::Unhandled` variant from any error type.
     pub fn unhandled(
-        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
+        err: impl ::std::convert::Into<
+            ::std::boxed::Box<
+                dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static,
+            >,
+        >,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
             source: err.into(),
@@ -271,9 +348,15 @@
     ///
     pub fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::InvalidRequestException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::TooManyRequestsException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::UnauthorizedException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::InvalidRequestException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
+            Self::TooManyRequestsException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
+            Self::UnauthorizedException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
             Self::Unhandled(e) => &e.meta,
         }
     }
@@ -307,7 +390,9 @@
             Self::TooManyRequestsException(_inner) => _inner.fmt(f),
             Self::UnauthorizedException(_inner) => _inner.fmt(f),
             Self::Unhandled(_inner) => {
-                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
+                if let ::std::option::Option::Some(code) =
+                    ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
+                {
                     write!(f, "unhandled error ({code})")
                 } else {
                     f.write_str("unhandled error")
@@ -327,9 +412,15 @@
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for LogoutError {
     fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::InvalidRequestException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::TooManyRequestsException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::UnauthorizedException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::InvalidRequestException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::TooManyRequestsException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::UnauthorizedException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
             Self::Unhandled(_inner) => &_inner.meta,
         }
     }
@@ -336,7 +427,9 @@
 }
 impl ::aws_smithy_runtime_api::client::result::CreateUnhandledError for LogoutError {
     fn create_unhandled_error(
-        source: ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
+        source: ::std::boxed::Box<
+            dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static,
+        >,
         meta: ::std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
```

### `src/operation/logout/_logout_input.rs`

```diff
diff --git a/src/operation/logout/_logout_input.rs b/src/operation/logout/_logout_input.rs
--- a/src/operation/logout/_logout_input.rs
+++ b/src/operation/logout/_logout_input.rs
@@ -49,7 +49,12 @@
         &self.access_token
     }
     /// Consumes the builder and constructs a [`LogoutInput`](crate::operation::logout::LogoutInput).
-    pub fn build(self) -> ::std::result::Result<crate::operation::logout::LogoutInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<
+        crate::operation::logout::LogoutInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(crate::operation::logout::LogoutInput {
             access_token: self.access_token,
         })
```

### `src/operation/logout/_logout_output.rs`

```diff
diff --git a/src/operation/logout/_logout_output.rs b/src/operation/logout/_logout_output.rs
--- a/src/operation/logout/_logout_output.rs
+++ b/src/operation/logout/_logout_output.rs
@@ -18,7 +18,9 @@
 }

 /// A builder for [`LogoutOutput`](crate::operation::logout::LogoutOutput).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(
+    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
+)]
 #[non_exhaustive]
 pub struct LogoutOutputBuilder {
     _request_id: Option<String>,
```

### `src/operation/logout/builders.rs`

```diff
diff --git a/src/operation/logout/builders.rs b/src/operation/logout/builders.rs
--- a/src/operation/logout/builders.rs
+++ b/src/operation/logout/builders.rs
@@ -32,14 +32,20 @@
     inner: crate::operation::logout::builders::LogoutInputBuilder,
     config_override: ::std::option::Option<crate::config::Builder>,
 }
-impl crate::client::customize::internal::CustomizableSend<crate::operation::logout::LogoutOutput, crate::operation::logout::LogoutError>
-    for LogoutFluentBuilder
+impl
+    crate::client::customize::internal::CustomizableSend<
+        crate::operation::logout::LogoutOutput,
+        crate::operation::logout::LogoutError,
+    > for LogoutFluentBuilder
 {
     fn send(
         self,
         config_override: crate::config::Builder,
     ) -> crate::client::customize::internal::BoxFuture<
-        crate::client::customize::internal::SendResult<crate::operation::logout::LogoutOutput, crate::operation::logout::LogoutError>,
+        crate::client::customize::internal::SendResult<
+            crate::operation::logout::LogoutOutput,
+            crate::operation::logout::LogoutError,
+        >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
@@ -78,8 +84,11 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins =
-            crate::operation::logout::Logout::operation_runtime_plugins(self.handle.runtime_plugins.clone(), &self.handle.conf, self.config_override);
+        let runtime_plugins = crate::operation::logout::Logout::operation_runtime_plugins(
+            self.handle.runtime_plugins.clone(),
+            &self.handle.conf,
+            self.config_override,
+        );
         crate::operation::logout::Logout::orchestrate(&runtime_plugins, input).await
     }

@@ -86,15 +95,25 @@
     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
     pub fn customize(
         self,
-    ) -> crate::client::customize::CustomizableOperation<crate::operation::logout::LogoutOutput, crate::operation::logout::LogoutError, Self> {
+    ) -> crate::client::customize::CustomizableOperation<
+        crate::operation::logout::LogoutOutput,
+        crate::operation::logout::LogoutError,
+        Self,
+    > {
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
@@ -1,9 +1,17 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+#[cfg(any(
+    feature = "op_get_role_credentials",
+    feature = "op_list_account_roles",
+    feature = "op_list_accounts",
+    feature = "op_logout"
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
@@ -10,27 +18,50 @@
     E: ::std::error::Error + std::fmt::Debug + ::std::marker::Send + ::std::marker::Sync + 'static,
 {
     result
-        .map(|output| ::aws_smithy_runtime_api::client::interceptors::context::Output::erase(output))
-        .map_err(|error| ::aws_smithy_runtime_api::client::interceptors::context::Error::erase(error))
+        .map(|output| {
+            ::aws_smithy_runtime_api::client::interceptors::context::Output::erase(output)
+        })
+        .map_err(|error| {
+            ::aws_smithy_runtime_api::client::interceptors::context::Error::erase(error)
+        })
         .map_err(::std::convert::Into::into)
 }

+#[cfg(any(
+    feature = "op_get_role_credentials",
+    feature = "op_list_account_roles",
+    feature = "op_list_accounts",
+    feature = "op_logout"
+))]
 pub fn parse_http_error_metadata(
     _response_status: u16,
     response_headers: &::aws_smithy_runtime_api::http::Headers,
     response_body: &[u8],
-) -> ::std::result::Result<::aws_smithy_types::error::metadata::Builder, ::aws_smithy_json::deserialize::error::DeserializeError> {
+) -> ::std::result::Result<
+    ::aws_smithy_types::error::metadata::Builder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     crate::json_errors::parse_error_metadata(response_body, response_headers)
 }

+#[cfg(feature = "op_get_role_credentials")]
 pub(crate) mod shape_get_role_credentials;

+#[cfg(feature = "op_list_account_roles")]
 pub(crate) mod shape_list_account_roles;

+#[cfg(feature = "op_list_accounts")]
 pub(crate) mod shape_list_accounts;

+#[cfg(feature = "op_logout")]
 pub(crate) mod shape_logout;

+#[cfg(any(
+    feature = "op_get_role_credentials",
+    feature = "op_list_account_roles",
+    feature = "op_list_accounts",
+    feature = "op_logout"
+))]
 pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
     if data.is_empty() {
         b"{}"
@@ -39,20 +70,48 @@
     }
 }

+#[cfg(any(
+    feature = "op_get_role_credentials",
+    feature = "op_list_account_roles",
+    feature = "op_list_accounts",
+    feature = "op_logout"
+))]
 pub(crate) mod shape_invalid_request_exception;

+#[cfg(any(
+    feature = "op_get_role_credentials",
+    feature = "op_list_account_roles",
+    feature = "op_list_accounts"
+))]
 pub(crate) mod shape_resource_not_found_exception;

+#[cfg(any(
+    feature = "op_get_role_credentials",
+    feature = "op_list_account_roles",
+    feature = "op_list_accounts",
+    feature = "op_logout"
+))]
 pub(crate) mod shape_too_many_requests_exception;

+#[cfg(any(
+    feature = "op_get_role_credentials",
+    feature = "op_list_account_roles",
+    feature = "op_list_accounts",
+    feature = "op_logout"
+))]
 pub(crate) mod shape_unauthorized_exception;

+#[cfg(feature = "op_list_accounts")]
 pub(crate) mod shape_account_list_type;

+#[cfg(feature = "op_get_role_credentials")]
 pub(crate) mod shape_role_credentials;

+#[cfg(feature = "op_list_account_roles")]
 pub(crate) mod shape_role_list_type;

+#[cfg(feature = "op_list_accounts")]
 pub(crate) mod shape_account_info;

+#[cfg(feature = "op_list_account_roles")]
 pub(crate) mod shape_role_info;
```

### `src/protocol_serde/shape_account_info.rs`

```diff
diff --git a/src/protocol_serde/shape_account_info.rs b/src/protocol_serde/shape_account_info.rs
--- a/src/protocol_serde/shape_account_info.rs
+++ b/src/protocol_serde/shape_account_info.rs
@@ -3,14 +3,24 @@
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
     depth: u32,
-) -> ::std::result::Result<Option<crate::types::AccountInfo>, ::aws_smithy_json::deserialize::error::DeserializeError>
+) -> ::std::result::Result<
+    Option<crate::types::AccountInfo>,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+>
 where
-    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
+    I: Iterator<
+        Item = Result<
+            ::aws_smithy_json::deserialize::Token<'a>,
+            ::aws_smithy_json::deserialize::error::DeserializeError,
+        >,
+    >,
 {
     if depth >= 128u32 {
-        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-            "maximum nesting depth exceeded",
-        ));
+        return Err(
+            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                "maximum nesting depth exceeded",
+            ),
+        );
     }
     match tokens.next().transpose()? {
         Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
@@ -20,41 +30,53 @@
             loop {
                 match tokens.next().transpose()? {
                     Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                        "accountId" => {
-                            builder = builder.set_account_id(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
+                        match key.to_unescaped()?.as_ref() {
+                            "accountId" => {
+                                builder = builder.set_account_id(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
+                                        tokens.next(),
+                                    )?
                                     .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                     .transpose()?,
-                            );
-                        }
-                        "accountName" => {
-                            builder = builder.set_account_name(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                );
+                            }
+                            "accountName" => {
+                                builder = builder.set_account_name(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
+                                        tokens.next(),
+                                    )?
                                     .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                     .transpose()?,
-                            );
-                        }
-                        "emailAddress" => {
-                            builder = builder.set_email_address(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                );
+                            }
+                            "emailAddress" => {
+                                builder = builder.set_email_address(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
+                                        tokens.next(),
+                                    )?
                                     .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                     .transpose()?,
-                            );
+                                );
+                            }
+                            _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                         }
-                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
-                    },
+                    }
                     other => {
-                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
-                            "expected object key or end object, found: {other:?}"
-                        )))
+                        return Err(
+                            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                                format!("expected object key or end object, found: {other:?}"),
+                            ),
+                        )
                     }
                 }
             }
             Ok(Some(builder.build()))
         }
-        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-            "expected start object or null",
-        )),
+        _ => Err(
+            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                "expected start object or null",
+            ),
+        ),
     }
 }
```

### `src/protocol_serde/shape_account_list_type.rs`

```diff
diff --git a/src/protocol_serde/shape_account_list_type.rs b/src/protocol_serde/shape_account_list_type.rs
--- a/src/protocol_serde/shape_account_list_type.rs
+++ b/src/protocol_serde/shape_account_list_type.rs
@@ -3,14 +3,24 @@
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
     depth: u32,
-) -> ::std::result::Result<Option<::std::vec::Vec<crate::types::AccountInfo>>, ::aws_smithy_json::deserialize::error::DeserializeError>
+) -> ::std::result::Result<
+    Option<::std::vec::Vec<crate::types::AccountInfo>>,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+>
 where
-    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
+    I: Iterator<
+        Item = Result<
+            ::aws_smithy_json::deserialize::Token<'a>,
+            ::aws_smithy_json::deserialize::error::DeserializeError,
+        >,
+    >,
 {
     if depth >= 128u32 {
-        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-            "maximum nesting depth exceeded",
-        ));
+        return Err(
+            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                "maximum nesting depth exceeded",
+            ),
+        );
     }
     match tokens.next().transpose()? {
         Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
@@ -23,13 +33,19 @@
                         break;
                     }
                     _ => {
-                        let value = crate::protocol_serde::shape_account_info::de_account_info(tokens, _value, depth + 1)?;
+                        let value = crate::protocol_serde::shape_account_info::de_account_info(
+                            tokens,
+                            _value,
+                            depth + 1,
+                        )?;
                         if let Some(value) = value {
                             items.push(value);
                         } else {
-                            return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-                                "dense list cannot contain null values",
-                            ));
+                            return Err(
+                                ::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                                    "dense list cannot contain null values",
+                                ),
+                            );
                         }
                     }
                 }
@@ -36,8 +52,10 @@
             }
             Ok(Some(items))
         }
-        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-            "expected start array or null",
-        )),
+        _ => Err(
+            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                "expected start array or null",
+            ),
+        ),
     }
 }
```

### `src/protocol_serde/shape_get_role_credentials.rs`

```diff
diff --git a/src/protocol_serde/shape_get_role_credentials.rs b/src/protocol_serde/shape_get_role_credentials.rs
--- a/src/protocol_serde/shape_get_role_credentials.rs
+++ b/src/protocol_serde/shape_get_role_credentials.rs
@@ -9,13 +9,21 @@
     crate::operation::get_role_credentials::GetRoleCredentialsError,
 > {
     #[allow(unused_mut)]
-    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(crate::operation::get_role_credentials::GetRoleCredentialsError::unhandled)?;
+    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(
+        _response_status,
+        _response_headers,
+        _response_body,
+    )
+    .map_err(crate::operation::get_role_credentials::GetRoleCredentialsError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(crate::operation::get_role_credentials::GetRoleCredentialsError::unhandled(generic)),
+        None => {
+            return Err(
+                crate::operation::get_role_credentials::GetRoleCredentialsError::unhandled(generic),
+            )
+        }
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
@@ -96,9 +104,14 @@
     Ok({
         #[allow(unused_mut)]
         let mut output = crate::operation::get_role_credentials::builders::GetRoleCredentialsOutputBuilder::default();
-        output = crate::protocol_serde::shape_get_role_credentials::de_get_role_credentials(_response_body, output)
-            .map_err(crate::operation::get_role_credentials::GetRoleCredentialsError::unhandled)?;
-        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
+        output = crate::protocol_serde::shape_get_role_credentials::de_get_role_credentials(
+            _response_body,
+            output,
+        )
+        .map_err(crate::operation::get_role_credentials::GetRoleCredentialsError::unhandled)?;
+        output._set_request_id(
+            ::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string),
+        );
         output.build()
     })
 }
@@ -106,7 +119,10 @@
 pub fn ser_get_role_credentials_headers(
     input: &crate::operation::get_role_credentials::GetRoleCredentialsInput,
     mut builder: ::http_1x::request::Builder,
-) -> std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+) -> std::result::Result<
+    ::http_1x::request::Builder,
+    ::aws_smithy_types::error::operation::BuildError,
+> {
     if let ::std::option::Option::Some(inner_1) = &input.access_token {
         let formatted_2 = inner_1.as_str();
         let header_value = formatted_2;
@@ -113,7 +129,10 @@
         let header_value: ::http_1x::HeaderValue = header_value.parse().map_err(|err| {
             ::aws_smithy_types::error::operation::BuildError::invalid_field(
                 "access_token",
-                format!("`{}` cannot be used as a header value: {}", &"*** Sensitive Data Redacted ***", err),
+                format!(
+                    "`{}` cannot be used as a header value: {}",
+                    &"*** Sensitive Data Redacted ***", err
+                ),
             )
         })?;
         builder = builder.header("x-amz-sso_bearer_token", header_value);
@@ -128,7 +147,10 @@
     crate::operation::get_role_credentials::builders::GetRoleCredentialsOutputBuilder,
     ::aws_smithy_json::deserialize::error::DeserializeError,
 > {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(
+        crate::protocol_serde::or_empty_doc(_value),
+    )
+    .peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -136,27 +158,35 @@
     loop {
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "roleCredentials" => {
-                    builder = builder.set_role_credentials(crate::protocol_serde::shape_role_credentials::de_role_credentials(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
+                match key.to_unescaped()?.as_ref() {
+                    "roleCredentials" => {
+                        builder = builder.set_role_credentials(
+                            crate::protocol_serde::shape_role_credentials::de_role_credentials(
+                                tokens,
+                                _value,
+                                depth + 1,
+                            )?,
+                        );
+                    }
+                    _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                 }
-                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
-            },
+            }
             other => {
-                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
-                    "expected object key or end object, found: {other:?}"
-                )))
+                return Err(
+                    ::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                        "expected object key or end object, found: {other:?}"
+                    )),
+                )
             }
         }
     }
     if tokens.next().is_some() {
-        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-            "found more JSON tokens after completing parsing",
-        ));
+        return Err(
+            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                "found more JSON tokens after completing parsing",
+            ),
+        );
     }
     Ok(builder)
 }
```

### `src/protocol_serde/shape_invalid_request_exception.rs`

```diff
diff --git a/src/protocol_serde/shape_invalid_request_exception.rs b/src/protocol_serde/shape_invalid_request_exception.rs
--- a/src/protocol_serde/shape_invalid_request_exception.rs
+++ b/src/protocol_serde/shape_invalid_request_exception.rs
@@ -2,8 +2,14 @@
 pub(crate) fn de_invalid_request_exception_json_err(
     _value: &[u8],
     mut builder: crate::types::error::builders::InvalidRequestExceptionBuilder,
-) -> ::std::result::Result<crate::types::error::builders::InvalidRequestExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+) -> ::std::result::Result<
+    crate::types::error::builders::InvalidRequestExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(
+        crate::protocol_serde::or_empty_doc(_value),
+    )
+    .peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -11,27 +17,35 @@
     loop {
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "message" => {
-                    builder = builder.set_message(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
+                match key.to_unescaped()?.as_ref() {
+                    "message" => {
+                        builder = builder.set_message(
+                            ::aws_smithy_json::deserialize::token::expect_string_or_null(
+                                tokens.next(),
+                            )?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
-                    );
+                        );
+                    }
+                    _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                 }
-                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
-            },
+            }
             other => {
-                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
-                    "expected object key or end object, found: {other:?}"
-                )))
+                return Err(
+                    ::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                        "expected object key or end object, found: {other:?}"
+                    )),
+                )
             }
         }
     }
     if tokens.next().is_some() {
-        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-            "found more JSON tokens after completing parsing",
-        ));
+        return Err(
+            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                "found more JSON tokens after completing parsing",
+            ),
+        );
     }
     Ok(builder)
 }
```

### `src/protocol_serde/shape_list_account_roles.rs`

```diff
diff --git a/src/protocol_serde/shape_list_account_roles.rs b/src/protocol_serde/shape_list_account_roles.rs
--- a/src/protocol_serde/shape_list_account_roles.rs
+++ b/src/protocol_serde/shape_list_account_roles.rs
@@ -4,79 +4,103 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<crate::operation::list_account_roles::ListAccountRolesOutput, crate::operation::list_account_roles::ListAccountRolesError> {
+) -> std::result::Result<
+    crate::operation::list_account_roles::ListAccountRolesOutput,
+    crate::operation::list_account_roles::ListAccountRolesError,
+> {
     #[allow(unused_mut)]
-    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(crate::operation::list_account_roles::ListAccountRolesError::unhandled)?;
+    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(
+        _response_status,
+        _response_headers,
+        _response_body,
+    )
+    .map_err(crate::operation::list_account_roles::ListAccountRolesError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(crate::operation::list_account_roles::ListAccountRolesError::unhandled(generic)),
+        None => {
+            return Err(
+                crate::operation::list_account_roles::ListAccountRolesError::unhandled(generic),
+            )
+        }
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
     Err(match error_code {
-        "InvalidRequestException" => crate::operation::list_account_roles::ListAccountRolesError::InvalidRequestException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "InvalidRequestException" => {
+            crate::operation::list_account_roles::ListAccountRolesError::InvalidRequestException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::InvalidRequestExceptionBuilder::default();
-                output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output =
+                        crate::types::error::builders::InvalidRequestExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(_response_body, output)
                     .map_err(crate::operation::list_account_roles::ListAccountRolesError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "ResourceNotFoundException" => crate::operation::list_account_roles::ListAccountRolesError::ResourceNotFoundException({
-            #[allow(unused_mut)]
-            let mut tmp = {
-                #[allow(unused_mut)]
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
+        "ResourceNotFoundException" => {
+            crate::operation::list_account_roles::ListAccountRolesError::ResourceNotFoundException(
+                {
+                    #[allow(unused_mut)]
+                    let mut tmp = {
+                        #[allow(unused_mut)]
                 let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
-                output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
+                        output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                     .map_err(crate::operation::list_account_roles::ListAccountRolesError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "TooManyRequestsException" => crate::operation::list_account_roles::ListAccountRolesError::TooManyRequestsException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+                        let output = output.meta(generic);
+                        output.build()
+                    };
+                    if tmp.message.is_none() {
+                        tmp.message = _error_message;
+                    }
+                    tmp
+                },
+            )
+        }
+        "TooManyRequestsException" => {
+            crate::operation::list_account_roles::ListAccountRolesError::TooManyRequestsException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
-                output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output =
+                        crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(crate::operation::list_account_roles::ListAccountRolesError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "UnauthorizedException" => crate::operation::list_account_roles::ListAccountRolesError::UnauthorizedException({
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
+        "UnauthorizedException" => {
+            crate::operation::list_account_roles::ListAccountRolesError::UnauthorizedException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::UnauthorizedExceptionBuilder::default();
-                output = crate::protocol_serde::shape_unauthorized_exception::de_unauthorized_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output =
+                        crate::types::error::builders::UnauthorizedExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_unauthorized_exception::de_unauthorized_exception_json_err(_response_body, output)
                     .map_err(crate::operation::list_account_roles::ListAccountRolesError::unhandled)?;
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
         _ => crate::operation::list_account_roles::ListAccountRolesError::generic(generic),
     })
 }
@@ -86,13 +110,23 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<crate::operation::list_account_roles::ListAccountRolesOutput, crate::operation::list_account_roles::ListAccountRolesError> {
+) -> std::result::Result<
+    crate::operation::list_account_roles::ListAccountRolesOutput,
+    crate::operation::list_account_roles::ListAccountRolesError,
+> {
     Ok({
         #[allow(unused_mut)]
-        let mut output = crate::operation::list_account_roles::builders::ListAccountRolesOutputBuilder::default();
-        output = crate::protocol_serde::shape_list_account_roles::de_list_account_roles(_response_body, output)
-            .map_err(crate::operation::list_account_roles::ListAccountRolesError::unhandled)?;
-        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
+        let mut output =
+            crate::operation::list_account_roles::builders::ListAccountRolesOutputBuilder::default(
+            );
+        output = crate::protocol_serde::shape_list_account_roles::de_list_account_roles(
+            _response_body,
+            output,
+        )
+        .map_err(crate::operation::list_account_roles::ListAccountRolesError::unhandled)?;
+        output._set_request_id(
+            ::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string),
+        );
         output.build()
     })
 }
@@ -100,7 +134,10 @@
 pub fn ser_list_account_roles_headers(
     input: &crate::operation::list_account_roles::ListAccountRolesInput,
     mut builder: ::http_1x::request::Builder,
-) -> std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+) -> std::result::Result<
+    ::http_1x::request::Builder,
+    ::aws_smithy_types::error::operation::BuildError,
+> {
     if let ::std::option::Option::Some(inner_1) = &input.access_token {
         let formatted_2 = inner_1.as_str();
         let header_value = formatted_2;
@@ -107,7 +144,10 @@
         let header_value: ::http_1x::HeaderValue = header_value.parse().map_err(|err| {
             ::aws_smithy_types::error::operation::BuildError::invalid_field(
                 "access_token",
-                format!("`{}` cannot be used as a header value: {}", &"*** Sensitive Data Redacted ***", err),
+                format!(
+                    "`{}` cannot be used as a header value: {}",
+                    &"*** Sensitive Data Redacted ***", err
+                ),
             )
         })?;
         builder = builder.header("x-amz-sso_bearer_token", header_value);
@@ -122,7 +162,10 @@
     crate::operation::list_account_roles::builders::ListAccountRolesOutputBuilder,
     ::aws_smithy_json::deserialize::error::DeserializeError,
 > {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(
+        crate::protocol_serde::or_empty_doc(_value),
+    )
+    .peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -130,30 +173,44 @@
     loop {
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "nextToken" => {
-                    builder = builder.set_next_token(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
+                match key.to_unescaped()?.as_ref() {
+                    "nextToken" => {
+                        builder = builder.set_next_token(
+                            ::aws_smithy_json::deserialize::token::expect_string_or_null(
+                                tokens.next(),
+                            )?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
-                    );
+                        );
+                    }
+                    "roleList" => {
+                        builder = builder.set_role_list(
+                            crate::protocol_serde::shape_role_list_type::de_role_list_type(
+                                tokens,
+                                _value,
+                                depth + 1,
+                            )?,
+                        );
+                    }
+                    _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                 }
-                "roleList" => {
-                    builder = builder.set_role_list(crate::protocol_serde::shape_role_list_type::de_role_list_type(tokens, _value, depth + 1)?);
-                }
-                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
-            },
+            }
             other => {
-                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
-                    "expected object key or end object, found: {other:?}"
-                )))
+                return Err(
+                    ::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                        "expected object key or end object, found: {other:?}"
+                    )),
+                )
             }
         }
     }
     if tokens.next().is_some() {
-        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-            "found more JSON tokens after completing parsing",
-        ));
+        return Err(
+            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                "found more JSON tokens after completing parsing",
+            ),
+        );
     }
     Ok(builder)
 }
```

### `src/protocol_serde/shape_list_accounts.rs`

```diff
diff --git a/src/protocol_serde/shape_list_accounts.rs b/src/protocol_serde/shape_list_accounts.rs
--- a/src/protocol_serde/shape_list_accounts.rs
+++ b/src/protocol_serde/shape_list_accounts.rs
@@ -4,10 +4,17 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<crate::operation::list_accounts::ListAccountsOutput, crate::operation::list_accounts::ListAccountsError> {
+) -> std::result::Result<
+    crate::operation::list_accounts::ListAccountsOutput,
+    crate::operation::list_accounts::ListAccountsError,
+> {
     #[allow(unused_mut)]
-    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(crate::operation::list_accounts::ListAccountsError::unhandled)?;
+    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(
+        _response_status,
+        _response_headers,
+        _response_body,
+    )
+    .map_err(crate::operation::list_accounts::ListAccountsError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
@@ -17,66 +24,78 @@

     let _error_message = generic.message().map(|msg| msg.to_owned());
     Err(match error_code {
-        "InvalidRequestException" => crate::operation::list_accounts::ListAccountsError::InvalidRequestException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "InvalidRequestException" => {
+            crate::operation::list_accounts::ListAccountsError::InvalidRequestException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::InvalidRequestExceptionBuilder::default();
-                output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output =
+                        crate::types::error::builders::InvalidRequestExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(_response_body, output)
                     .map_err(crate::operation::list_accounts::ListAccountsError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "ResourceNotFoundException" => crate::operation::list_accounts::ListAccountsError::ResourceNotFoundException({
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
+        "ResourceNotFoundException" => {
+            crate::operation::list_accounts::ListAccountsError::ResourceNotFoundException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
-                output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output =
+                        crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                     .map_err(crate::operation::list_accounts::ListAccountsError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "TooManyRequestsException" => crate::operation::list_accounts::ListAccountsError::TooManyRequestsException({
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
+        "TooManyRequestsException" => {
+            crate::operation::list_accounts::ListAccountsError::TooManyRequestsException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
-                output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output =
+                        crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(crate::operation::list_accounts::ListAccountsError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "UnauthorizedException" => crate::operation::list_accounts::ListAccountsError::UnauthorizedException({
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
+        "UnauthorizedException" => {
+            crate::operation::list_accounts::ListAccountsError::UnauthorizedException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::UnauthorizedExceptionBuilder::default();
-                output = crate::protocol_serde::shape_unauthorized_exception::de_unauthorized_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output =
+                        crate::types::error::builders::UnauthorizedExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_unauthorized_exception::de_unauthorized_exception_json_err(_response_body, output)
                     .map_err(crate::operation::list_accounts::ListAccountsError::unhandled)?;
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
         _ => crate::operation::list_accounts::ListAccountsError::generic(generic),
     })
 }
@@ -86,13 +105,20 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<crate::operation::list_accounts::ListAccountsOutput, crate::operation::list_accounts::ListAccountsError> {
+) -> std::result::Result<
+    crate::operation::list_accounts::ListAccountsOutput,
+    crate::operation::list_accounts::ListAccountsError,
+> {
     Ok({
         #[allow(unused_mut)]
-        let mut output = crate::operation::list_accounts::builders::ListAccountsOutputBuilder::default();
-        output = crate::protocol_serde::shape_list_accounts::de_list_accounts(_response_body, output)
-            .map_err(crate::operation::list_accounts::ListAccountsError::unhandled)?;
-        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
+        let mut output =
+            crate::operation::list_accounts::builders::ListAccountsOutputBuilder::default();
+        output =
+            crate::protocol_serde::shape_list_accounts::de_list_accounts(_response_body, output)
+                .map_err(crate::operation::list_accounts::ListAccountsError::unhandled)?;
+        output._set_request_id(
+            ::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string),
+        );
         output.build()
     })
 }
@@ -100,7 +126,10 @@
 pub fn ser_list_accounts_headers(
     input: &crate::operation::list_accounts::ListAccountsInput,
     mut builder: ::http_1x::request::Builder,
-) -> std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+) -> std::result::Result<
+    ::http_1x::request::Builder,
+    ::aws_smithy_types::error::operation::BuildError,
+> {
     if let ::std::option::Option::Some(inner_1) = &input.access_token {
         let formatted_2 = inner_1.as_str();
         let header_value = formatted_2;
@@ -107,7 +136,10 @@
         let header_value: ::http_1x::HeaderValue = header_value.parse().map_err(|err| {
             ::aws_smithy_types::error::operation::BuildError::invalid_field(
                 "access_token",
-                format!("`{}` cannot be used as a header value: {}", &"*** Sensitive Data Redacted ***", err),
+                format!(
+                    "`{}` cannot be used as a header value: {}",
+                    &"*** Sensitive Data Redacted ***", err
+                ),
             )
         })?;
         builder = builder.header("x-amz-sso_bearer_token", header_value);
@@ -122,7 +154,10 @@
     crate::operation::list_accounts::builders::ListAccountsOutputBuilder,
     ::aws_smithy_json::deserialize::error::DeserializeError,
 > {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(
+        crate::protocol_serde::or_empty_doc(_value),
+    )
+    .peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -130,34 +165,44 @@
     loop {
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "accountList" => {
-                    builder = builder.set_account_list(crate::protocol_serde::shape_account_list_type::de_account_list_type(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
-                }
-                "nextToken" => {
-                    builder = builder.set_next_token(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
+                match key.to_unescaped()?.as_ref() {
+                    "accountList" => {
+                        builder = builder.set_account_list(
+                            crate::protocol_serde::shape_account_list_type::de_account_list_type(
+                                tokens,
+                                _value,
+                                depth + 1,
+                            )?,
+                        );
+                    }
+                    "nextToken" => {
+                        builder = builder.set_next_token(
+                            ::aws_smithy_json::deserialize::token::expect_string_or_null(
+                                tokens.next(),
+                            )?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
-                    );
+                        );
+                    }
+                    _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                 }
-                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
-            },
+            }
             other => {
-                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
-                    "expected object key or end object, found: {other:?}"
-                )))
+                return Err(
+                    ::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                        "expected object key or end object, found: {other:?}"
+                    )),
+                )
             }
         }
     }
     if tokens.next().is_some() {
-        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-            "found more JSON tokens after completing parsing",
-        ));
+        return Err(
+            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                "found more JSON tokens after completing parsing",
+            ),
+        );
     }
     Ok(builder)
 }
```

### `src/protocol_serde/shape_logout.rs`

```diff
diff --git a/src/protocol_serde/shape_logout.rs b/src/protocol_serde/shape_logout.rs
--- a/src/protocol_serde/shape_logout.rs
+++ b/src/protocol_serde/shape_logout.rs
@@ -4,10 +4,17 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<crate::operation::logout::LogoutOutput, crate::operation::logout::LogoutError> {
+) -> std::result::Result<
+    crate::operation::logout::LogoutOutput,
+    crate::operation::logout::LogoutError,
+> {
     #[allow(unused_mut)]
-    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(crate::operation::logout::LogoutError::unhandled)?;
+    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(
+        _response_status,
+        _response_headers,
+        _response_body,
+    )
+    .map_err(crate::operation::logout::LogoutError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
@@ -17,41 +24,48 @@

     let _error_message = generic.message().map(|msg| msg.to_owned());
     Err(match error_code {
-        "InvalidRequestException" => crate::operation::logout::LogoutError::InvalidRequestException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "InvalidRequestException" => {
+            crate::operation::logout::LogoutError::InvalidRequestException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::InvalidRequestExceptionBuilder::default();
-                output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output =
+                        crate::types::error::builders::InvalidRequestExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(_response_body, output)
                     .map_err(crate::operation::logout::LogoutError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "TooManyRequestsException" => crate::operation::logout::LogoutError::TooManyRequestsException({
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
+        "TooManyRequestsException" => {
+            crate::operation::logout::LogoutError::TooManyRequestsException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
-                output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output =
+                        crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(crate::operation::logout::LogoutError::unhandled)?;
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
         "UnauthorizedException" => crate::operation::logout::LogoutError::UnauthorizedException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::UnauthorizedExceptionBuilder::default();
+                let mut output =
+                    crate::types::error::builders::UnauthorizedExceptionBuilder::default();
                 output = crate::protocol_serde::shape_unauthorized_exception::de_unauthorized_exception_json_err(_response_body, output)
                     .map_err(crate::operation::logout::LogoutError::unhandled)?;
                 let output = output.meta(generic);
@@ -71,11 +85,16 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<crate::operation::logout::LogoutOutput, crate::operation::logout::LogoutError> {
+) -> std::result::Result<
+    crate::operation::logout::LogoutOutput,
+    crate::operation::logout::LogoutError,
+> {
     Ok({
         #[allow(unused_mut)]
         let mut output = crate::operation::logout::builders::LogoutOutputBuilder::default();
-        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
+        output._set_request_id(
+            ::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string),
+        );
         output.build()
     })
 }
@@ -83,7 +102,10 @@
 pub fn ser_logout_headers(
     input: &crate::operation::logout::LogoutInput,
     mut builder: ::http_1x::request::Builder,
-) -> std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+) -> std::result::Result<
+    ::http_1x::request::Builder,
+    ::aws_smithy_types::error::operation::BuildError,
+> {
     if let ::std::option::Option::Some(inner_1) = &input.access_token {
         let formatted_2 = inner_1.as_str();
         let header_value = formatted_2;
@@ -90,7 +112,10 @@
         let header_value: ::http_1x::HeaderValue = header_value.parse().map_err(|err| {
             ::aws_smithy_types::error::operation::BuildError::invalid_field(
                 "access_token",
-                format!("`{}` cannot be used as a header value: {}", &"*** Sensitive Data Redacted ***", err),
+                format!(
+                    "`{}` cannot be used as a header value: {}",
+                    &"*** Sensitive Data Redacted ***", err
+                ),
             )
         })?;
         builder = builder.header("x-amz-sso_bearer_token", header_value);
```

### `src/protocol_serde/shape_resource_not_found_exception.rs`

```diff
diff --git a/src/protocol_serde/shape_resource_not_found_exception.rs b/src/protocol_serde/shape_resource_not_found_exception.rs
--- a/src/protocol_serde/shape_resource_not_found_exception.rs
+++ b/src/protocol_serde/shape_resource_not_found_exception.rs
@@ -2,8 +2,14 @@
 pub(crate) fn de_resource_not_found_exception_json_err(
     _value: &[u8],
     mut builder: crate::types::error::builders::ResourceNotFoundExceptionBuilder,
-) -> ::std::result::Result<crate::types::error::builders::ResourceNotFoundExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+) -> ::std::result::Result<
+    crate::types::error::builders::ResourceNotFoundExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(
+        crate::protocol_serde::or_empty_doc(_value),
+    )
+    .peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -11,27 +17,35 @@
     loop {
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "message" => {
-                    builder = builder.set_message(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
+                match key.to_unescaped()?.as_ref() {
+                    "message" => {
+                        builder = builder.set_message(
+                            ::aws_smithy_json::deserialize::token::expect_string_or_null(
+                                tokens.next(),
+                            )?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
-                    );
+                        );
+                    }
+                    _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                 }
-                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
-            },
+            }
             other => {
-                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
-                    "expected object key or end object, found: {other:?}"
-                )))
+                return Err(
+                    ::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                        "expected object key or end object, found: {other:?}"
+                    )),
+                )
             }
         }
     }
     if tokens.next().is_some() {
-        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-            "found more JSON tokens after completing parsing",
-        ));
+        return Err(
+            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                "found more JSON tokens after completing parsing",
+            ),
+        );
     }
     Ok(builder)
 }
```

### `src/protocol_serde/shape_role_credentials.rs`

```diff
diff --git a/src/protocol_serde/shape_role_credentials.rs b/src/protocol_serde/shape_role_credentials.rs
--- a/src/protocol_serde/shape_role_credentials.rs
+++ b/src/protocol_serde/shape_role_credentials.rs
@@ -3,14 +3,24 @@
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
     depth: u32,
-) -> ::std::result::Result<Option<crate::types::RoleCredentials>, ::aws_smithy_json::deserialize::error::DeserializeError>
+) -> ::std::result::Result<
+    Option<crate::types::RoleCredentials>,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+>
 where
-    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
+    I: Iterator<
+        Item = Result<
+            ::aws_smithy_json::deserialize::Token<'a>,
+            ::aws_smithy_json::deserialize::error::DeserializeError,
+        >,
+    >,
 {
     if depth >= 128u32 {
-        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-            "maximum nesting depth exceeded",
-        ));
+        return Err(
+            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                "maximum nesting depth exceeded",
+            ),
+        );
     }
     match tokens.next().transpose()? {
         Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
@@ -20,48 +30,62 @@
             loop {
                 match tokens.next().transpose()? {
                     Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                        "accessKeyId" => {
-                            builder = builder.set_access_key_id(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
+                        match key.to_unescaped()?.as_ref() {
+                            "accessKeyId" => {
+                                builder = builder.set_access_key_id(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
+                                        tokens.next(),
+                                    )?
                                     .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                     .transpose()?,
-                            );
-                        }
-                        "secretAccessKey" => {
-                            builder = builder.set_secret_access_key(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                );
+                            }
+                            "secretAccessKey" => {
+                                builder = builder.set_secret_access_key(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
+                                        tokens.next(),
+                                    )?
                                     .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                     .transpose()?,
-                            );
-                        }
-                        "sessionToken" => {
-                            builder = builder.set_session_token(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                );
+                            }
+                            "sessionToken" => {
+                                builder = builder.set_session_token(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
+                                        tokens.next(),
+                                    )?
                                     .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                     .transpose()?,
-                            );
-                        }
-                        "expiration" => {
-                            builder = builder.set_expiration(
-                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                                );
+                            }
+                            "expiration" => {
+                                builder = builder.set_expiration(
+                                    ::aws_smithy_json::deserialize::token::expect_number_or_null(
+                                        tokens.next(),
+                                    )?
                                     .map(i64::try_from)
                                     .transpose()?,
-                            );
+                                );
+                            }
+                            _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                         }
-                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
-                    },
+                    }
                     other => {
-                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
-                            "expected object key or end object, found: {other:?}"
-                        )))
+                        return Err(
+                            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                                format!("expected object key or end object, found: {other:?}"),
+                            ),
+                        )
                     }
                 }
             }
             Ok(Some(builder.build()))
         }
-        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-            "expected start object or null",
-        )),
+        _ => Err(
+            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                "expected start object or null",
+            ),
+        ),
     }
 }
```

### `src/protocol_serde/shape_role_info.rs`

```diff
diff --git a/src/protocol_serde/shape_role_info.rs b/src/protocol_serde/shape_role_info.rs
--- a/src/protocol_serde/shape_role_info.rs
+++ b/src/protocol_serde/shape_role_info.rs
@@ -3,14 +3,24 @@
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
     depth: u32,
-) -> ::std::result::Result<Option<crate::types::RoleInfo>, ::aws_smithy_json::deserialize::error::DeserializeError>
+) -> ::std::result::Result<
+    Option<crate::types::RoleInfo>,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+>
 where
-    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
+    I: Iterator<
+        Item = Result<
+            ::aws_smithy_json::deserialize::Token<'a>,
+            ::aws_smithy_json::deserialize::error::DeserializeError,
+        >,
+    >,
 {
     if depth >= 128u32 {
-        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-            "maximum nesting depth exceeded",
-        ));
+        return Err(
+            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                "maximum nesting depth exceeded",
+            ),
+        );
     }
     match tokens.next().transpose()? {
         Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
@@ -20,34 +30,44 @@
             loop {
                 match tokens.next().transpose()? {
                     Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                        "roleName" => {
-                            builder = builder.set_role_name(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
+                        match key.to_unescaped()?.as_ref() {
+                            "roleName" => {
+                                builder = builder.set_role_name(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
+                                        tokens.next(),
+                                    )?
                                     .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                     .transpose()?,
-                            );
-                        }
-                        "accountId" => {
-                            builder = builder.set_account_id(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                );
+                            }
+                            "accountId" => {
+                                builder = builder.set_account_id(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
+                                        tokens.next(),
+                                    )?
                                     .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                     .transpose()?,
-                            );
+                                );
+                            }
+                            _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                         }
-                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
-                    },
+                    }
                     other => {
-                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
-                            "expected object key or end object, found: {other:?}"
-                        )))
+                        return Err(
+                            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                                format!("expected object key or end object, found: {other:?}"),
+                            ),
+                        )
                     }
                 }
             }
             Ok(Some(builder.build()))
         }
-        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-            "expected start object or null",
-        )),
+        _ => Err(
+            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                "expected start object or null",
+            ),
+        ),
     }
 }
```

### `src/protocol_serde/shape_role_list_type.rs`

```diff
diff --git a/src/protocol_serde/shape_role_list_type.rs b/src/protocol_serde/shape_role_list_type.rs
--- a/src/protocol_serde/shape_role_list_type.rs
+++ b/src/protocol_serde/shape_role_list_type.rs
@@ -3,14 +3,24 @@
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
     depth: u32,
-) -> ::std::result::Result<Option<::std::vec::Vec<crate::types::RoleInfo>>, ::aws_smithy_json::deserialize::error::DeserializeError>
+) -> ::std::result::Result<
+    Option<::std::vec::Vec<crate::types::RoleInfo>>,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+>
 where
-    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
+    I: Iterator<
+        Item = Result<
+            ::aws_smithy_json::deserialize::Token<'a>,
+            ::aws_smithy_json::deserialize::error::DeserializeError,
+        >,
+    >,
 {
     if depth >= 128u32 {
-        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-            "maximum nesting depth exceeded",
-        ));
+        return Err(
+            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                "maximum nesting depth exceeded",
+            ),
+        );
     }
     match tokens.next().transpose()? {
         Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
@@ -23,13 +33,19 @@
                         break;
                     }
                     _ => {
-                        let value = crate::protocol_serde::shape_role_info::de_role_info(tokens, _value, depth + 1)?;
+                        let value = crate::protocol_serde::shape_role_info::de_role_info(
+                            tokens,
+                            _value,
+                            depth + 1,
+                        )?;
                         if let Some(value) = value {
                             items.push(value);
                         } else {
-                            return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-                                "dense list cannot contain null values",
-                            ));
+                            return Err(
+                                ::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                                    "dense list cannot contain null values",
+                                ),
+                            );
                         }
                     }
                 }
@@ -36,8 +52,10 @@
             }
             Ok(Some(items))
         }
-        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-            "expected start array or null",
-        )),
+        _ => Err(
+            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                "expected start array or null",
+            ),
+        ),
     }
 }
```

### `src/protocol_serde/shape_too_many_requests_exception.rs`

```diff
diff --git a/src/protocol_serde/shape_too_many_requests_exception.rs b/src/protocol_serde/shape_too_many_requests_exception.rs
--- a/src/protocol_serde/shape_too_many_requests_exception.rs
+++ b/src/protocol_serde/shape_too_many_requests_exception.rs
@@ -2,8 +2,14 @@
 pub(crate) fn de_too_many_requests_exception_json_err(
     _value: &[u8],
     mut builder: crate::types::error::builders::TooManyRequestsExceptionBuilder,
-) -> ::std::result::Result<crate::types::error::builders::TooManyRequestsExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+) -> ::std::result::Result<
+    crate::types::error::builders::TooManyRequestsExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(
+        crate::protocol_serde::or_empty_doc(_value),
+    )
+    .peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -11,27 +17,35 @@
     loop {
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "message" => {
-                    builder = builder.set_message(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
+                match key.to_unescaped()?.as_ref() {
+                    "message" => {
+                        builder = builder.set_message(
+                            ::aws_smithy_json::deserialize::token::expect_string_or_null(
+                                tokens.next(),
+                            )?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
-                    );
+                        );
+                    }
+                    _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                 }
-                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
-            },
+            }
             other => {
-                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
-                    "expected object key or end object, found: {other:?}"
-                )))
+                return Err(
+                    ::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                        "expected object key or end object, found: {other:?}"
+                    )),
+                )
             }
         }
     }
     if tokens.next().is_some() {
-        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-            "found more JSON tokens after completing parsing",
-        ));
+        return Err(
+            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                "found more JSON tokens after completing parsing",
+            ),
+        );
     }
     Ok(builder)
 }
```

### `src/protocol_serde/shape_unauthorized_exception.rs`

```diff
diff --git a/src/protocol_serde/shape_unauthorized_exception.rs b/src/protocol_serde/shape_unauthorized_exception.rs
--- a/src/protocol_serde/shape_unauthorized_exception.rs
+++ b/src/protocol_serde/shape_unauthorized_exception.rs
@@ -2,8 +2,14 @@
 pub(crate) fn de_unauthorized_exception_json_err(
     _value: &[u8],
     mut builder: crate::types::error::builders::UnauthorizedExceptionBuilder,
-) -> ::std::result::Result<crate::types::error::builders::UnauthorizedExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+) -> ::std::result::Result<
+    crate::types::error::builders::UnauthorizedExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(
+        crate::protocol_serde::or_empty_doc(_value),
+    )
+    .peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -11,27 +17,35 @@
     loop {
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "message" => {
-                    builder = builder.set_message(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
+                match key.to_unescaped()?.as_ref() {
+                    "message" => {
+                        builder = builder.set_message(
+                            ::aws_smithy_json::deserialize::token::expect_string_or_null(
+                                tokens.next(),
+                            )?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
-                    );
+                        );
+                    }
+                    _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                 }
-                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
-            },
+            }
             other => {
-                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
-                    "expected object key or end object, found: {other:?}"
-                )))
+                return Err(
+                    ::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                        "expected object key or end object, found: {other:?}"
+                    )),
+                )
             }
         }
     }
     if tokens.next().is_some() {
-        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-            "found more JSON tokens after completing parsing",
-        ));
+        return Err(
+            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                "found more JSON tokens after completing parsing",
+            ),
+        );
     }
     Ok(builder)
 }
```

### `src/sdk_feature_tracker.rs`

```diff
diff --git a/src/sdk_feature_tracker.rs b/src/sdk_feature_tracker.rs
--- a/src/sdk_feature_tracker.rs
+++ b/src/sdk_feature_tracker.rs
@@ -46,8 +46,12 @@
     use aws_smithy_runtime::client::sdk_feature::SmithySdkFeature;
     use aws_smithy_runtime_api::box_error::BoxError;
     use aws_smithy_runtime_api::client::interceptors::context::BeforeSerializationInterceptorContextMut;
-    use aws_smithy_runtime_api::client::interceptors::{dyn_dispatch_hint, Intercept, SharedInterceptor};
-    use aws_smithy_runtime_api::client::runtime_components::{RuntimeComponents, RuntimeComponentsBuilder};
+    use aws_smithy_runtime_api::client::interceptors::{
+        dyn_dispatch_hint, Intercept, SharedInterceptor,
+    };
+    use aws_smithy_runtime_api::client::runtime_components::{
+        RuntimeComponents, RuntimeComponentsBuilder,
+    };
     use aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin;
     use aws_smithy_types::config_bag::ConfigBag;
     use std::borrow::Cow;
@@ -73,7 +77,8 @@
             _runtime_components: &RuntimeComponents,
             cfg: &mut ConfigBag,
         ) -> Result<(), BoxError> {
-            cfg.interceptor_state().store_append::<SmithySdkFeature>(SmithySdkFeature::Paginator);
+            cfg.interceptor_state()
+                .store_append::<SmithySdkFeature>(SmithySdkFeature::Paginator);
             Ok(())
         }
     }
@@ -86,14 +91,21 @@
     impl PaginatorFeatureTrackerRuntimePlugin {
         pub(crate) fn new() -> Self {
             Self {
-                runtime_components: RuntimeComponentsBuilder::new("PaginatorFeatureTrackerRuntimePlugin")
-                    .with_interceptor(SharedInterceptor::permanent(PaginatorFeatureTrackerInterceptor::new())),
+                runtime_components: RuntimeComponentsBuilder::new(
+                    "PaginatorFeatureTrackerRuntimePlugin",
+                )
+                .with_interceptor(SharedInterceptor::permanent(
+                    PaginatorFeatureTrackerInterceptor::new(),
+                )),
             }
         }
     }

     impl RuntimePlugin for PaginatorFeatureTrackerRuntimePlugin {
-        fn runtime_components(&self, _: &RuntimeComponentsBuilder) -> Cow<'_, RuntimeComponentsBuilder> {
+        fn runtime_components(
+            &self,
+            _: &RuntimeComponentsBuilder,
+        ) -> Cow<'_, RuntimeComponentsBuilder> {
             Cow::Borrowed(&self.runtime_components)
         }
     }
@@ -104,8 +116,12 @@
     use aws_smithy_runtime::client::sdk_feature::SmithySdkFeature;
     use aws_smithy_runtime_api::box_error::BoxError;
     use aws_smithy_runtime_api::client::interceptors::context::BeforeSerializationInterceptorContextMut;
-    use aws_smithy_runtime_api::client::interceptors::{dyn_dispatch_hint, Intercept, SharedInterceptor};
-    use aws_smithy_runtime_api::client::runtime_components::{RuntimeComponents, RuntimeComponentsBuilder};
+    use aws_smithy_runtime_api::client::interceptors::{
+        dyn_dispatch_hint, Intercept, SharedInterceptor,
+    };
+    use aws_smithy_runtime_api::client::runtime_components::{
+        RuntimeComponents, RuntimeComponentsBuilder,
+    };
     use aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin;
     use aws_smithy_types::config_bag::ConfigBag;
     use std::borrow::Cow;
@@ -131,7 +147,8 @@
             _runtime_components: &RuntimeComponents,
             cfg: &mut ConfigBag,
         ) -> Result<(), BoxError> {
-            cfg.interceptor_state().store_append::<SmithySdkFeature>(SmithySdkFeature::Waiter);
+            cfg.interceptor_state()
+                .store_append::<SmithySdkFeature>(SmithySdkFeature::Waiter);
             Ok(())
         }
     }
@@ -144,14 +161,21 @@
     impl WaiterFeatureTrackerRuntimePlugin {
         pub(crate) fn new() -> Self {
             Self {
-                runtime_components: RuntimeComponentsBuilder::new("WaiterFeatureTrackerRuntimePlugin")
-                    .with_interceptor(SharedInterceptor::permanent(WaiterFeatureTrackerInterceptor::new())),
+                runtime_components: RuntimeComponentsBuilder::new(
+                    "WaiterFeatureTrackerRuntimePlugin",
+                )
+                .with_interceptor(SharedInterceptor::permanent(
+                    WaiterFeatureTrackerInterceptor::new(),
+                )),
             }
         }
     }

     impl RuntimePlugin for WaiterFeatureTrackerRuntimePlugin {
-        fn runtime_components(&self, _: &RuntimeComponentsBuilder) -> Cow<'_, RuntimeComponentsBuilder> {
+        fn runtime_components(
+            &self,
+            _: &RuntimeComponentsBuilder,
+        ) -> Cow<'_, RuntimeComponentsBuilder> {
             Cow::Borrowed(&self.runtime_components)
         }
     }
@@ -194,7 +218,10 @@
                     RetryMode::Adaptive => SmithySdkFeature::RetryModeAdaptive,
                     _ => unreachable!("retry mode must be standard or adaptive"),
                 })
-                .map(|feature| cfg.interceptor_state().store_append::<SmithySdkFeature>(feature));
+                .map(|feature| {
+                    cfg.interceptor_state()
+                        .store_append::<SmithySdkFeature>(feature)
+                });

             Ok(())
         }
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
@@ -1,14 +1,20 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+#[cfg(feature = "op_get_role_credentials")]
 pub use crate::types::_role_credentials::RoleCredentials;

+#[cfg(feature = "op_list_account_roles")]
 pub use crate::types::_role_info::RoleInfo;

+#[cfg(feature = "op_list_accounts")]
 pub use crate::types::_account_info::AccountInfo;

+#[cfg(feature = "op_list_accounts")]
 mod _account_info;

+#[cfg(feature = "op_get_role_credentials")]
 mod _role_credentials;

+#[cfg(feature = "op_list_account_roles")]
 mod _role_info;

 /// Builders
```

### `src/types/_account_info.rs`

```diff
diff --git a/src/types/_account_info.rs b/src/types/_account_info.rs
--- a/src/types/_account_info.rs
+++ b/src/types/_account_info.rs
@@ -33,7 +33,9 @@
 }

 /// A builder for [`AccountInfo`](crate::types::AccountInfo).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(
+    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
+)]
 #[non_exhaustive]
 pub struct AccountInfoBuilder {
     pub(crate) account_id: ::std::option::Option<::std::string::String>,
@@ -70,12 +72,18 @@
         &self.account_name
     }
     /// <p>The email address of the AWS account that is assigned to the user.</p>
-    pub fn email_address(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
+    pub fn email_address(
+        mut self,
+        input: impl ::std::convert::Into<::std::string::String>,
+    ) -> Self {
         self.email_address = ::std::option::Option::Some(input.into());
         self
     }
     /// <p>The email address of the AWS account that is assigned to the user.</p>
-    pub fn set_email_address(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
+    pub fn set_email_address(
+        mut self,
+        input: ::std::option::Option<::std::string::String>,
+    ) -> Self {
         self.email_address = input;
         self
     }
```

### `src/types/_role_credentials.rs`

```diff
diff --git a/src/types/_role_credentials.rs b/src/types/_role_credentials.rs
--- a/src/types/_role_credentials.rs
+++ b/src/types/_role_credentials.rs
@@ -59,12 +59,18 @@
 }
 impl RoleCredentialsBuilder {
     /// <p>The identifier used for the temporary security credentials. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_use-resources.html">Using Temporary Security Credentials to Request Access to AWS Resources</a> in the <i>AWS IAM User Guide</i>.</p>
-    pub fn access_key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
+    pub fn access_key_id(
+        mut self,
+        input: impl ::std::convert::Into<::std::string::String>,
+    ) -> Self {
         self.access_key_id = ::std::option::Option::Some(input.into());
         self
     }
     /// <p>The identifier used for the temporary security credentials. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_use-resources.html">Using Temporary Security Credentials to Request Access to AWS Resources</a> in the <i>AWS IAM User Guide</i>.</p>
-    pub fn set_access_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
+    pub fn set_access_key_id(
+        mut self,
+        input: ::std::option::Option<::std::string::String>,
+    ) -> Self {
         self.access_key_id = input;
         self
     }
@@ -73,12 +79,18 @@
         &self.access_key_id
     }
     /// <p>The key that is used to sign the request. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_use-resources.html">Using Temporary Security Credentials to Request Access to AWS Resources</a> in the <i>AWS IAM User Guide</i>.</p>
-    pub fn secret_access_key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
+    pub fn secret_access_key(
+        mut self,
+        input: impl ::std::convert::Into<::std::string::String>,
+    ) -> Self {
         self.secret_access_key = ::std::option::Option::Some(input.into());
         self
     }
     /// <p>The key that is used to sign the request. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_use-resources.html">Using Temporary Security Credentials to Request Access to AWS Resources</a> in the <i>AWS IAM User Guide</i>.</p>
-    pub fn set_secret_access_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
+    pub fn set_secret_access_key(
+        mut self,
+        input: ::std::option::Option<::std::string::String>,
+    ) -> Self {
         self.secret_access_key = input;
         self
     }
@@ -87,12 +99,18 @@
         &self.secret_access_key
     }
     /// <p>The token used for temporary credentials. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_use-resources.html">Using Temporary Security Credentials to Request Access to AWS Resources</a> in the <i>AWS IAM User Guide</i>.</p>
-    pub fn session_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
+    pub fn session_token(
+        mut self,
+        input: impl ::std::convert::Into<::std::string::String>,
+    ) -> Self {
         self.session_token = ::std::option::Option::Some(input.into());
         self
     }
     /// <p>The token used for temporary credentials. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_use-resources.html">Using Temporary Security Credentials to Request Access to AWS Resources</a> in the <i>AWS IAM User Guide</i>.</p>
-    pub fn set_session_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
+    pub fn set_session_token(
+        mut self,
+        input: ::std::option::Option<::std::string::String>,
+    ) -> Self {
         self.session_token = input;
         self
     }
```

### `src/types/_role_info.rs`

```diff
diff --git a/src/types/_role_info.rs b/src/types/_role_info.rs
--- a/src/types/_role_info.rs
+++ b/src/types/_role_info.rs
@@ -27,7 +27,9 @@
 }

 /// A builder for [`RoleInfo`](crate::types::RoleInfo).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(
+    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
+)]
 #[non_exhaustive]
 pub struct RoleInfoBuilder {
     pub(crate) role_name: ::std::option::Option<::std::string::String>,
```

### `src/types/builders.rs`

```diff
diff --git a/src/types/builders.rs b/src/types/builders.rs
--- a/src/types/builders.rs
+++ b/src/types/builders.rs
@@ -1,6 +1,9 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+#[cfg(feature = "op_get_role_credentials")]
 pub use crate::types::_role_credentials::RoleCredentialsBuilder;

+#[cfg(feature = "op_list_account_roles")]
 pub use crate::types::_role_info::RoleInfoBuilder;

+#[cfg(feature = "op_list_accounts")]
 pub use crate::types::_account_info::AccountInfoBuilder;
```

### `src/types/error.rs`

```diff
diff --git a/src/types/error.rs b/src/types/error.rs
--- a/src/types/error.rs
+++ b/src/types/error.rs
@@ -1,18 +1,64 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+#[cfg(any(
+    feature = "op_get_role_credentials",
+    feature = "op_list_account_roles",
+    feature = "op_list_accounts",
+    feature = "op_logout"
+))]
 pub use crate::types::error::_invalid_request_exception::InvalidRequestException;

+#[cfg(any(
+    feature = "op_get_role_credentials",
+    feature = "op_list_account_roles",
+    feature = "op_list_accounts"
+))]
 pub use crate::types::error::_resource_not_found_exception::ResourceNotFoundException;

+#[cfg(any(
+    feature = "op_get_role_credentials",
+    feature = "op_list_account_roles",
+    feature = "op_list_accounts",
+    feature = "op_logout"
+))]
 pub use crate::types::error::_too_many_requests_exception::TooManyRequestsException;

+#[cfg(any(
+    feature = "op_get_role_credentials",
+    feature = "op_list_account_roles",
+    feature = "op_list_accounts",
+    feature = "op_logout"
+))]
 pub use crate::types::error::_unauthorized_exception::UnauthorizedException;

+#[cfg(any(
+    feature = "op_get_role_credentials",
+    feature = "op_list_account_roles",
+    feature = "op_list_accounts",
+    feature = "op_logout"
+))]
 mod _invalid_request_exception;

+#[cfg(any(
+    feature = "op_get_role_credentials",
+    feature = "op_list_account_roles",
+    feature = "op_list_accounts"
+))]
 mod _resource_not_found_exception;

+#[cfg(any(
+    feature = "op_get_role_credentials",
+    feature = "op_list_account_roles",
+    feature = "op_list_accounts",
+    feature = "op_logout"
+))]
 mod _too_many_requests_exception;

+#[cfg(any(
+    feature = "op_get_role_credentials",
+    feature = "op_list_account_roles",
+    feature = "op_list_accounts",
+    feature = "op_logout"
+))]
 mod _unauthorized_exception;

 /// Builders
```

### `src/types/error/_invalid_request_exception.rs`

```diff
diff --git a/src/types/error/_invalid_request_exception.rs b/src/types/error/_invalid_request_exception.rs
--- a/src/types/error/_invalid_request_exception.rs
+++ b/src/types/error/_invalid_request_exception.rs
@@ -45,7 +45,9 @@
 }

 /// A builder for [`InvalidRequestException`](crate::types::error::InvalidRequestException).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(
+    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
+)]
 #[non_exhaustive]
 pub struct InvalidRequestExceptionBuilder {
     pub(crate) message: ::std::option::Option<::std::string::String>,
@@ -73,7 +75,10 @@
     }

     /// Sets error metadata
-    pub fn set_meta(&mut self, meta: std::option::Option<::aws_smithy_types::error::ErrorMetadata>) -> &mut Self {
+    pub fn set_meta(
+        &mut self,
+        meta: std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
+    ) -> &mut Self {
         self.meta = meta;
         self
     }
```

### `src/types/error/_resource_not_found_exception.rs`

```diff
diff --git a/src/types/error/_resource_not_found_exception.rs b/src/types/error/_resource_not_found_exception.rs
--- a/src/types/error/_resource_not_found_exception.rs
+++ b/src/types/error/_resource_not_found_exception.rs
@@ -45,7 +45,9 @@
 }

 /// A builder for [`ResourceNotFoundException`](crate::types::error::ResourceNotFoundException).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(
+    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
+)]
 #[non_exhaustive]
 pub struct ResourceNotFoundExceptionBuilder {
     pub(crate) message: ::std::option::Option<::std::string::String>,
@@ -73,7 +75,10 @@
     }

     /// Sets error metadata
-    pub fn set_meta(&mut self, meta: std::option::Option<::aws_smithy_types::error::ErrorMetadata>) -> &mut Self {
+    pub fn set_meta(
+        &mut self,
+        meta: std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
+    ) -> &mut Self {
         self.meta = meta;
         self
     }
```

### `src/types/error/_too_many_requests_exception.rs`

```diff
diff --git a/src/types/error/_too_many_requests_exception.rs b/src/types/error/_too_many_requests_exception.rs
--- a/src/types/error/_too_many_requests_exception.rs
+++ b/src/types/error/_too_many_requests_exception.rs
@@ -45,7 +45,9 @@
 }

 /// A builder for [`TooManyRequestsException`](crate::types::error::TooManyRequestsException).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(
+    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
+)]
 #[non_exhaustive]
 pub struct TooManyRequestsExceptionBuilder {
     pub(crate) message: ::std::option::Option<::std::string::String>,
@@ -73,7 +75,10 @@
     }

     /// Sets error metadata
-    pub fn set_meta(&mut self, meta: std::option::Option<::aws_smithy_types::error::ErrorMetadata>) -> &mut Self {
+    pub fn set_meta(
+        &mut self,
+        meta: std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
+    ) -> &mut Self {
         self.meta = meta;
         self
     }
```

### `src/types/error/_unauthorized_exception.rs`

```diff
diff --git a/src/types/error/_unauthorized_exception.rs b/src/types/error/_unauthorized_exception.rs
--- a/src/types/error/_unauthorized_exception.rs
+++ b/src/types/error/_unauthorized_exception.rs
@@ -45,7 +45,9 @@
 }

 /// A builder for [`UnauthorizedException`](crate::types::error::UnauthorizedException).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(
+    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
+)]
 #[non_exhaustive]
 pub struct UnauthorizedExceptionBuilder {
     pub(crate) message: ::std::option::Option<::std::string::String>,
@@ -73,7 +75,10 @@
     }

     /// Sets error metadata
-    pub fn set_meta(&mut self, meta: std::option::Option<::aws_smithy_types::error::ErrorMetadata>) -> &mut Self {
+    pub fn set_meta(
+        &mut self,
+        meta: std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
+    ) -> &mut Self {
         self.meta = meta;
         self
     }
```

### `src/types/error/builders.rs`

```diff
diff --git a/src/types/error/builders.rs b/src/types/error/builders.rs
--- a/src/types/error/builders.rs
+++ b/src/types/error/builders.rs
@@ -1,8 +1,31 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+#[cfg(any(
+    feature = "op_get_role_credentials",
+    feature = "op_list_account_roles",
+    feature = "op_list_accounts",
+    feature = "op_logout"
+))]
 pub use crate::types::error::_invalid_request_exception::InvalidRequestExceptionBuilder;

+#[cfg(any(
+    feature = "op_get_role_credentials",
+    feature = "op_list_account_roles",
+    feature = "op_list_accounts"
+))]
 pub use crate::types::error::_resource_not_found_exception::ResourceNotFoundExceptionBuilder;

+#[cfg(any(
+    feature = "op_get_role_credentials",
+    feature = "op_list_account_roles",
+    feature = "op_list_accounts",
+    feature = "op_logout"
+))]
 pub use crate::types::error::_too_many_requests_exception::TooManyRequestsExceptionBuilder;

+#[cfg(any(
+    feature = "op_get_role_credentials",
+    feature = "op_list_account_roles",
+    feature = "op_list_accounts",
+    feature = "op_logout"
+))]
 pub use crate::types::error::_unauthorized_exception::UnauthorizedExceptionBuilder;
```

## Unified diff

```diff
diff --git a/Cargo.toml b/Cargo.toml
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -19,50 +19,39 @@
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

 [dependencies.aws-types]
-path = "../aws-types"
 version = "1.5.0"

 [dependencies.bytes]
@@ -84,7 +73,6 @@
 [dependencies.tracing]
 version = "0.1"
 [dev-dependencies.aws-credential-types]
-path = "../aws-credential-types"
 features = ["test-util"]
 version = "1.3.0"

@@ -103,3 +91,10 @@
 test-util = ["aws-credential-types/test-util", "aws-smithy-runtime/legacy-test-util", "aws-smithy-runtime/test-util"]
 gated-tests = []
 default = ["rustls", "default-https-client", "rt-tokio"]
+op_get_role_credentials = []
+op_list_account_roles = []
+op_list_accounts = []
+op_logout = []
+
+[lib]
+name = "aws_sdk_sso"
diff --git a/src/client.rs b/src/client.rs
--- a/src/client.rs
+++ b/src/client.rs
@@ -64,7 +64,9 @@
         &self.handle.conf
     }

-    fn validate_config(handle: &Handle) -> ::std::result::Result<(), ::aws_smithy_runtime_api::box_error::BoxError> {
+    fn validate_config(
+        handle: &Handle,
+    ) -> ::std::result::Result<(), ::aws_smithy_runtime_api::box_error::BoxError> {
         let mut cfg = ::aws_smithy_types::config_bag::ConfigBag::base();
         handle
             .runtime_plugins
@@ -117,10 +119,14 @@
 /// ```
 pub mod customize;

+#[cfg(feature = "op_get_role_credentials")]
 mod get_role_credentials;

+#[cfg(feature = "op_list_account_roles")]
 mod list_account_roles;

+#[cfg(feature = "op_list_accounts")]
 mod list_accounts;

+#[cfg(feature = "op_logout")]
 mod logout;
diff --git a/src/client/customize.rs b/src/client/customize.rs
--- a/src/client/customize.rs
+++ b/src/client/customize.rs
@@ -24,7 +24,10 @@
         }
     }

-    pub(crate) fn execute<U>(self, f: impl ::std::ops::FnOnce(B, crate::config::Builder) -> U) -> U {
+    pub(crate) fn execute<U>(
+        self,
+        f: impl ::std::ops::FnOnce(B, crate::config::Builder) -> U,
+    ) -> U {
         let mut config_override = self.config_override.unwrap_or_default();
         self.interceptors.into_iter().for_each(|interceptor| {
             config_override.push_interceptor(interceptor);
@@ -41,17 +44,27 @@
     /// `map_request`, and `mutate_request` (the last two are implemented via interceptors under the hood).
     /// The order in which those user-specified operation interceptors are invoked should not be relied upon
     /// as it is an implementation detail.
-    pub fn interceptor(mut self, interceptor: impl ::aws_smithy_runtime_api::client::interceptors::Intercept + 'static) -> Self {
-        self.interceptors
-            .push(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::new(interceptor));
+    pub fn interceptor(
+        mut self,
+        interceptor: impl ::aws_smithy_runtime_api::client::interceptors::Intercept + 'static,
+    ) -> Self {
+        self.interceptors.push(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::new(interceptor),
+        );
         self
     }

     /// Adds a runtime plugin.
     #[allow(unused)]
-    pub(crate) fn runtime_plugin(mut self, runtime_plugin: impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin + 'static) -> Self {
-        self.runtime_plugins
-            .push(::aws_smithy_runtime_api::client::runtime_plugin::SharedRuntimePlugin::new(runtime_plugin));
+    pub(crate) fn runtime_plugin(
+        mut self,
+        runtime_plugin: impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin + 'static,
+    ) -> Self {
+        self.runtime_plugins.push(
+            ::aws_smithy_runtime_api::client::runtime_plugin::SharedRuntimePlugin::new(
+                runtime_plugin,
+            ),
+        );
         self
     }

@@ -60,16 +73,19 @@
     where
         F: ::std::ops::Fn(
                 ::aws_smithy_runtime_api::client::orchestrator::HttpRequest,
-            ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, MapE>
-            + ::std::marker::Send
+            ) -> ::std::result::Result<
+                ::aws_smithy_runtime_api::client::orchestrator::HttpRequest,
+                MapE,
+            > + ::std::marker::Send
             + ::std::marker::Sync
             + 'static,
         MapE: ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static,
     {
-        self.interceptors
-            .push(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::new(
+        self.interceptors.push(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::new(
                 ::aws_smithy_runtime::client::interceptors::MapRequestInterceptor::new(f),
-            ));
+            ),
+        );
         self
     }

@@ -76,12 +92,16 @@
     /// Convenience for `map_request` where infallible direct mutation of request is acceptable.
     pub fn mutate_request<F>(mut self, f: F) -> Self
     where
-        F: ::std::ops::Fn(&mut ::aws_smithy_runtime_api::client::orchestrator::HttpRequest) + ::std::marker::Send + ::std::marker::Sync + 'static,
+        F: ::std::ops::Fn(&mut ::aws_smithy_runtime_api::client::orchestrator::HttpRequest)
+            + ::std::marker::Send
+            + ::std::marker::Sync
+            + 'static,
     {
-        self.interceptors
-            .push(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::new(
+        self.interceptors.push(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::new(
                 ::aws_smithy_runtime::client::interceptors::MutateRequestInterceptor::new(f),
-            ));
+            ),
+        );
         self
     }

@@ -96,7 +116,10 @@
     /// | field_1: None,     | field_1: Some(v2),     | field_1: Some(v2), |
     /// | field_2: Some(v1), | field_2: Some(v2),     | field_2: Some(v2), |
     /// | field_3: Some(v1), | field_3: None,         | field_3: Some(v1), |
-    pub fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
+    pub fn config_override(
+        mut self,
+        config_override: impl ::std::convert::Into<crate::config::Builder>,
+    ) -> Self {
         self.config_override = Some(config_override.into());
         self
     }
diff --git a/src/client/customize/internal.rs b/src/client/customize/internal.rs
--- a/src/client/customize/internal.rs
+++ b/src/client/customize/internal.rs
@@ -1,8 +1,14 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-pub type BoxFuture<T> = ::std::pin::Pin<::std::boxed::Box<dyn ::std::future::Future<Output = T> + ::std::marker::Send>>;
+pub type BoxFuture<T> =
+    ::std::pin::Pin<::std::boxed::Box<dyn ::std::future::Future<Output = T> + ::std::marker::Send>>;

-pub type SendResult<T, E> =
-    ::std::result::Result<T, ::aws_smithy_runtime_api::client::result::SdkError<E, ::aws_smithy_runtime_api::client::orchestrator::HttpResponse>>;
+pub type SendResult<T, E> = ::std::result::Result<
+    T,
+    ::aws_smithy_runtime_api::client::result::SdkError<
+        E,
+        ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
+    >,
+>;

 pub trait CustomizableSend<T, E>: ::std::marker::Send + ::std::marker::Sync {
     // Takes an owned `self` as the implementation will internally call methods that take `self`.
diff --git a/src/client/get_role_credentials.rs b/src/client/get_role_credentials.rs
--- a/src/client/get_role_credentials.rs
+++ b/src/client/get_role_credentials.rs
@@ -9,7 +9,11 @@
     /// - On success, responds with [`GetRoleCredentialsOutput`](crate::operation::get_role_credentials::GetRoleCredentialsOutput) with field(s):
     ///   - [`role_credentials(Option<RoleCredentials>)`](crate::operation::get_role_credentials::GetRoleCredentialsOutput::role_credentials): <p>The credentials for the role that is assigned to the user.</p>
     /// - On failure, responds with [`SdkError<GetRoleCredentialsError>`](crate::operation::get_role_credentials::GetRoleCredentialsError)
-    pub fn get_role_credentials(&self) -> crate::operation::get_role_credentials::builders::GetRoleCredentialsFluentBuilder {
-        crate::operation::get_role_credentials::builders::GetRoleCredentialsFluentBuilder::new(self.handle.clone())
+    pub fn get_role_credentials(
+        &self,
+    ) -> crate::operation::get_role_credentials::builders::GetRoleCredentialsFluentBuilder {
+        crate::operation::get_role_credentials::builders::GetRoleCredentialsFluentBuilder::new(
+            self.handle.clone(),
+        )
     }
 }
diff --git a/src/client/list_account_roles.rs b/src/client/list_account_roles.rs
--- a/src/client/list_account_roles.rs
+++ b/src/client/list_account_roles.rs
@@ -12,7 +12,11 @@
     ///   - [`next_token(Option<String>)`](crate::operation::list_account_roles::ListAccountRolesOutput::next_token): <p>The page token client that is used to retrieve the list of accounts.</p>
     ///   - [`role_list(Option<Vec::<RoleInfo>>)`](crate::operation::list_account_roles::ListAccountRolesOutput::role_list): <p>A paginated response with the list of roles and the next token if more results are available.</p>
     /// - On failure, responds with [`SdkError<ListAccountRolesError>`](crate::operation::list_account_roles::ListAccountRolesError)
-    pub fn list_account_roles(&self) -> crate::operation::list_account_roles::builders::ListAccountRolesFluentBuilder {
-        crate::operation::list_account_roles::builders::ListAccountRolesFluentBuilder::new(self.handle.clone())
+    pub fn list_account_roles(
+        &self,
+    ) -> crate::operation::list_account_roles::builders::ListAccountRolesFluentBuilder {
+        crate::operation::list_account_roles::builders::ListAccountRolesFluentBuilder::new(
+            self.handle.clone(),
+        )
     }
 }
diff --git a/src/client/list_accounts.rs b/src/client/list_accounts.rs
--- a/src/client/list_accounts.rs
+++ b/src/client/list_accounts.rs
@@ -11,7 +11,11 @@
     ///   - [`next_token(Option<String>)`](crate::operation::list_accounts::ListAccountsOutput::next_token): <p>The page token client that is used to retrieve the list of accounts.</p>
     ///   - [`account_list(Option<Vec::<AccountInfo>>)`](crate::operation::list_accounts::ListAccountsOutput::account_list): <p>A paginated response with the list of account information and the next token if more results are available.</p>
     /// - On failure, responds with [`SdkError<ListAccountsError>`](crate::operation::list_accounts::ListAccountsError)
-    pub fn list_accounts(&self) -> crate::operation::list_accounts::builders::ListAccountsFluentBuilder {
-        crate::operation::list_accounts::builders::ListAccountsFluentBuilder::new(self.handle.clone())
+    pub fn list_accounts(
+        &self,
+    ) -> crate::operation::list_accounts::builders::ListAccountsFluentBuilder {
+        crate::operation::list_accounts::builders::ListAccountsFluentBuilder::new(
+            self.handle.clone(),
+        )
     }
 }
diff --git a/src/config.rs b/src/config.rs
--- a/src/config.rs
+++ b/src/config.rs
@@ -47,8 +47,11 @@
         }
     }
     /// Return a reference to the stalled stream protection configuration contained in this config, if any.
-    pub fn stalled_stream_protection(&self) -> ::std::option::Option<&crate::config::StalledStreamProtectionConfig> {
-        self.config.load::<crate::config::StalledStreamProtectionConfig>()
+    pub fn stalled_stream_protection(
+        &self,
+    ) -> ::std::option::Option<&crate::config::StalledStreamProtectionConfig> {
+        self.config
+            .load::<crate::config::StalledStreamProtectionConfig>()
     }
     /// Return the [`SharedHttpClient`](crate::config::SharedHttpClient) to use when making requests, if any.
     pub fn http_client(&self) -> Option<crate::config::SharedHttpClient> {
@@ -55,21 +58,33 @@
         self.runtime_components.http_client()
     }
     /// Return the auth schemes configured on this service config
-    pub fn auth_schemes(&self) -> impl Iterator<Item = ::aws_smithy_runtime_api::client::auth::SharedAuthScheme> + '_ {
+    pub fn auth_schemes(
+        &self,
+    ) -> impl Iterator<Item = ::aws_smithy_runtime_api::client::auth::SharedAuthScheme> + '_ {
         self.runtime_components.auth_schemes()
     }

     /// Return the auth scheme resolver configured on this service config
-    pub fn auth_scheme_resolver(&self) -> ::std::option::Option<::aws_smithy_runtime_api::client::auth::SharedAuthSchemeOptionResolver> {
+    pub fn auth_scheme_resolver(
+        &self,
+    ) -> ::std::option::Option<::aws_smithy_runtime_api::client::auth::SharedAuthSchemeOptionResolver>
+    {
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
-    pub fn endpoint_resolver(&self) -> ::aws_smithy_runtime_api::client::endpoint::SharedEndpointResolver {
-        self.runtime_components.endpoint_resolver().expect("resolver defaulted if not set")
+    pub fn endpoint_resolver(
+        &self,
+    ) -> ::aws_smithy_runtime_api::client::endpoint::SharedEndpointResolver {
+        self.runtime_components
+            .endpoint_resolver()
+            .expect("resolver defaulted if not set")
     }
     /// Return a reference to the retry configuration contained in this config, if any.
     pub fn retry_config(&self) -> ::std::option::Option<&::aws_smithy_types::retry::RetryConfig> {
@@ -82,8 +97,11 @@
     }

     /// Return a reference to the timeout configuration contained in this config, if any.
-    pub fn timeout_config(&self) -> ::std::option::Option<&::aws_smithy_types::timeout::TimeoutConfig> {
-        self.config.load::<::aws_smithy_types::timeout::TimeoutConfig>()
+    pub fn timeout_config(
+        &self,
+    ) -> ::std::option::Option<&::aws_smithy_types::timeout::TimeoutConfig> {
+        self.config
+            .load::<::aws_smithy_types::timeout::TimeoutConfig>()
     }

     /// Returns a reference to the retry partition contained in this config, if any.
@@ -90,8 +108,11 @@
     ///
     /// WARNING: This method is unstable and may be removed at any time. Do not rely on this
     /// method for anything!
-    pub fn retry_partition(&self) -> ::std::option::Option<&::aws_smithy_runtime::client::retries::RetryPartition> {
-        self.config.load::<::aws_smithy_runtime::client::retries::RetryPartition>()
+    pub fn retry_partition(
+        &self,
+    ) -> ::std::option::Option<&::aws_smithy_runtime::client::retries::RetryPartition> {
+        self.config
+            .load::<::aws_smithy_runtime::client::retries::RetryPartition>()
     }
     /// Returns the configured identity cache for auth.
     pub fn identity_cache(&self) -> ::std::option::Option<crate::config::SharedIdentityCache> {
@@ -106,7 +127,11 @@
         self.runtime_components.time_source()
     }
     /// Returns retry classifiers currently registered by the user.
-    pub fn retry_classifiers(&self) -> impl Iterator<Item = ::aws_smithy_runtime_api::client::retries::classifiers::SharedRetryClassifier> + '_ {
+    pub fn retry_classifiers(
+        &self,
+    ) -> impl Iterator<
+        Item = ::aws_smithy_runtime_api::client::retries::classifiers::SharedRetryClassifier,
+    > + '_ {
         self.runtime_components.retry_classifiers()
     }
     /// Returns the name of the app that is using the client, if it was provided.
@@ -122,11 +147,15 @@
     /// being used with the client, rendered into the user agent as `lib/{name}/{version}`.
     /// Entries are returned in first-seen (insertion) order, matching the order they are
     /// rendered into the user agent.
-    pub fn framework_metadata(&self) -> ::std::vec::Vec<&::aws_types::sdk_ua_metadata::FrameworkMetadata> {
+    pub fn framework_metadata(
+        &self,
+    ) -> ::std::vec::Vec<&::aws_types::sdk_ua_metadata::FrameworkMetadata> {
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
@@ -133,8 +162,12 @@
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
@@ -193,18 +226,52 @@
     #[allow(unused)]
     pub(crate) fn from_config_bag(config_bag: &::aws_smithy_types::config_bag::ConfigBag) -> Self {
         let mut builder = Self::new();
-        builder.set_stalled_stream_protection(config_bag.load::<crate::config::StalledStreamProtectionConfig>().cloned());
-        builder.set_auth_scheme_preference(config_bag.load::<::aws_smithy_runtime_api::client::auth::AuthSchemePreference>().cloned());
-        builder.set_retry_config(config_bag.load::<::aws_smithy_types::retry::RetryConfig>().cloned());
-        builder.set_timeout_config(config_bag.load::<::aws_smithy_types::timeout::TimeoutConfig>().cloned());
-        builder.set_retry_partition(config_bag.load::<::aws_smithy_runtime::client::retries::RetryPartition>().cloned());
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
+        builder.set_retry_config(
+            config_bag
+                .load::<::aws_smithy_types::retry::RetryConfig>()
+                .cloned(),
+        );
+        builder.set_timeout_config(
+            config_bag
+                .load::<::aws_smithy_types::timeout::TimeoutConfig>()
+                .cloned(),
+        );
+        builder.set_retry_partition(
+            config_bag
+                .load::<::aws_smithy_runtime::client::retries::RetryPartition>()
+                .cloned(),
+        );
         builder.set_app_name(config_bag.load::<::aws_types::app_name::AppName>().cloned());
-        for framework_metadata in config_bag.load::<::aws_types::sdk_ua_metadata::FrameworkMetadata>() {
+        for framework_metadata in
+            config_bag.load::<::aws_types::sdk_ua_metadata::FrameworkMetadata>()
+        {
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
         builder.set_region(config_bag.load::<crate::config::Region>().cloned());
         builder
     }
@@ -220,7 +287,10 @@
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
@@ -253,8 +323,13 @@
     }
     /// Set the [`StalledStreamProtectionConfig`](crate::config::StalledStreamProtectionConfig)
     /// to configure protection for stalled streams.
-    pub fn stalled_stream_protection(mut self, stalled_stream_protection_config: crate::config::StalledStreamProtectionConfig) -> Self {
-        self.set_stalled_stream_protection(::std::option::Option::Some(stalled_stream_protection_config));
+    pub fn stalled_stream_protection(
+        mut self,
+        stalled_stream_protection_config: crate::config::StalledStreamProtectionConfig,
+    ) -> Self {
+        self.set_stalled_stream_protection(::std::option::Option::Some(
+            stalled_stream_protection_config,
+        ));
         self
     }
     /// Set the [`StalledStreamProtectionConfig`](crate::config::StalledStreamProtectionConfig)
@@ -261,7 +336,9 @@
     /// to configure protection for stalled streams.
     pub fn set_stalled_stream_protection(
         &mut self,
-        stalled_stream_protection_config: ::std::option::Option<crate::config::StalledStreamProtectionConfig>,
+        stalled_stream_protection_config: ::std::option::Option<
+            crate::config::StalledStreamProtectionConfig,
+        >,
     ) -> &mut Self {
         self.config.store_or_unset(stalled_stream_protection_config);
         self
@@ -296,7 +373,9 @@
     /// # }
     /// ```
     pub fn http_client(mut self, http_client: impl crate::config::HttpClient + 'static) -> Self {
-        self.set_http_client(::std::option::Option::Some(crate::config::IntoShared::into_shared(http_client)));
+        self.set_http_client(::std::option::Option::Some(
+            crate::config::IntoShared::into_shared(http_client),
+        ));
         self
     }

@@ -329,7 +408,10 @@
     /// # }
     /// # }
     /// ```
-    pub fn set_http_client(&mut self, http_client: Option<crate::config::SharedHttpClient>) -> &mut Self {
+    pub fn set_http_client(
+        &mut self,
+        http_client: Option<crate::config::SharedHttpClient>,
+    ) -> &mut Self {
         self.runtime_components.set_http_client(http_client);
         self
     }
@@ -441,7 +523,10 @@
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
@@ -476,7 +561,10 @@
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
@@ -485,9 +573,14 @@
     ///
     /// # Examples
     /// See an example for [`Self::auth_scheme_resolver`].
-    pub fn set_auth_scheme_resolver(&mut self, auth_scheme_resolver: impl crate::config::auth::ResolveAuthScheme + 'static) -> &mut Self {
+    pub fn set_auth_scheme_resolver(
+        &mut self,
+        auth_scheme_resolver: impl crate::config::auth::ResolveAuthScheme + 'static,
+    ) -> &mut Self {
         self.runtime_components
-            .set_auth_scheme_option_resolver(::std::option::Option::Some(auth_scheme_resolver.into_shared_resolver()));
+            .set_auth_scheme_option_resolver(::std::option::Option::Some(
+                auth_scheme_resolver.into_shared_resolver(),
+            ));
         self
     }

@@ -505,7 +598,9 @@
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
@@ -531,7 +626,9 @@

     pub fn auth_scheme_preference(
         mut self,
-        preference: impl ::std::convert::Into<::aws_smithy_runtime_api::client::auth::AuthSchemePreference>,
+        preference: impl ::std::convert::Into<
+            ::aws_smithy_runtime_api::client::auth::AuthSchemePreference,
+        >,
     ) -> Self {
         self.set_auth_scheme_preference(::std::option::Option::Some(preference.into()));
         self
@@ -560,7 +657,9 @@

     pub fn set_auth_scheme_preference(
         &mut self,
-        preference: ::std::option::Option<::aws_smithy_runtime_api::client::auth::AuthSchemePreference>,
+        preference: ::std::option::Option<
+            ::aws_smithy_runtime_api::client::auth::AuthSchemePreference,
+        >,
     ) -> &mut Self {
         self.config.store_or_unset(preference);
         self
@@ -592,8 +691,13 @@
     /// let config = aws_sdk_sso::Config::builder().endpoint_resolver(resolver).build();
     /// let client = aws_sdk_sso::Client::from_conf(config);
     /// ```
-    pub fn endpoint_resolver(mut self, endpoint_resolver: impl crate::config::endpoint::ResolveEndpoint + 'static) -> Self {
-        self.set_endpoint_resolver(::std::option::Option::Some(endpoint_resolver.into_shared_resolver()));
+    pub fn endpoint_resolver(
+        mut self,
+        endpoint_resolver: impl crate::config::endpoint::ResolveEndpoint + 'static,
+    ) -> Self {
+        self.set_endpoint_resolver(::std::option::Option::Some(
+            endpoint_resolver.into_shared_resolver(),
+        ));
         self
     }

@@ -605,9 +709,12 @@
     ///
     pub fn set_endpoint_resolver(
         &mut self,
-        endpoint_resolver: ::std::option::Option<::aws_smithy_runtime_api::client::endpoint::SharedEndpointResolver>,
+        endpoint_resolver: ::std::option::Option<
+            ::aws_smithy_runtime_api::client::endpoint::SharedEndpointResolver,
+        >,
     ) -> &mut Self {
-        self.runtime_components.set_endpoint_resolver(endpoint_resolver);
+        self.runtime_components
+            .set_endpoint_resolver(endpoint_resolver);
         self
     }
     /// Set the retry_config for the builder
@@ -650,7 +757,10 @@
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
@@ -674,7 +784,9 @@
     /// let config = Config::builder().sleep_impl(sleep_impl).build();
     /// ```
     pub fn sleep_impl(mut self, sleep_impl: impl crate::config::AsyncSleep + 'static) -> Self {
-        self.set_sleep_impl(Some(::aws_smithy_runtime_api::shared::IntoShared::into_shared(sleep_impl)));
+        self.set_sleep_impl(Some(
+            ::aws_smithy_runtime_api::shared::IntoShared::into_shared(sleep_impl),
+        ));
         self
     }

@@ -703,7 +815,10 @@
     /// set_never_ending_sleep_impl(&mut builder);
     /// let config = builder.build();
     /// ```
-    pub fn set_sleep_impl(&mut self, sleep_impl: ::std::option::Option<crate::config::SharedAsyncSleep>) -> &mut Self {
+    pub fn set_sleep_impl(
+        &mut self,
+        sleep_impl: ::std::option::Option<crate::config::SharedAsyncSleep>,
+    ) -> &mut Self {
         self.runtime_components.set_sleep_impl(sleep_impl);
         self
     }
@@ -721,7 +836,10 @@
     ///     .build();
     /// let config = Config::builder().timeout_config(timeout_config).build();
     /// ```
-    pub fn timeout_config(mut self, timeout_config: ::aws_smithy_types::timeout::TimeoutConfig) -> Self {
+    pub fn timeout_config(
+        mut self,
+        timeout_config: ::aws_smithy_types::timeout::TimeoutConfig,
+    ) -> Self {
         self.set_timeout_config(Some(timeout_config));
         self
     }
@@ -750,11 +868,19 @@
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

-        if let Some(base) = self.config.load::<::aws_smithy_types::timeout::TimeoutConfig>() {
+        if let Some(base) = self
+            .config
+            .load::<::aws_smithy_types::timeout::TimeoutConfig>()
+        {
             timeout_config.take_defaults_from(base);
         }
         self.config.store_put(timeout_config);
@@ -822,7 +948,10 @@
     ///     .retry_config(RetryConfig::adaptive())
     ///     .build();
     /// ```
-    pub fn retry_partition(mut self, retry_partition: ::aws_smithy_runtime::client::retries::RetryPartition) -> Self {
+    pub fn retry_partition(
+        mut self,
+        retry_partition: ::aws_smithy_runtime::client::retries::RetryPartition,
+    ) -> Self {
         self.set_retry_partition(Some(retry_partition));
         self
     }
@@ -829,7 +958,9 @@
     /// Like [`Self::retry_partition`], but takes a mutable reference to the builder and an optional `RetryPartition`
     pub fn set_retry_partition(
         &mut self,
-        retry_partition: ::std::option::Option<::aws_smithy_runtime::client::retries::RetryPartition>,
+        retry_partition: ::std::option::Option<
+            ::aws_smithy_runtime::client::retries::RetryPartition,
+        >,
     ) -> &mut Self {
         retry_partition.map(|r| self.config.store_put(r));
         self
@@ -875,7 +1006,10 @@
     /// let client = aws_sdk_sso::Client::from_conf(config);
     /// ```
     ///
-    pub fn identity_cache(mut self, identity_cache: impl crate::config::ResolveCachedIdentity + 'static) -> Self {
+    pub fn identity_cache(
+        mut self,
+        identity_cache: impl crate::config::ResolveCachedIdentity + 'static,
+    ) -> Self {
         self.set_identity_cache(identity_cache);
         self
     }
@@ -921,8 +1055,12 @@
     /// let client = aws_sdk_sso::Client::from_conf(config);
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
@@ -984,19 +1122,29 @@
     }

     /// Set [`SharedInterceptor`](crate::config::SharedInterceptor)s for the builder.
-    pub fn set_interceptors(&mut self, interceptors: impl IntoIterator<Item = crate::config::SharedInterceptor>) -> &mut Self {
-        self.runtime_components.set_interceptors(interceptors.into_iter());
+    pub fn set_interceptors(
+        &mut self,
+        interceptors: impl IntoIterator<Item = crate::config::SharedInterceptor>,
+    ) -> &mut Self {
+        self.runtime_components
+            .set_interceptors(interceptors.into_iter());
         self
     }
     /// Sets the time source used for this service
-    pub fn time_source(mut self, time_source: impl ::aws_smithy_async::time::TimeSource + 'static) -> Self {
-        self.set_time_source(::std::option::Option::Some(::aws_smithy_runtime_api::shared::IntoShared::into_shared(
-            time_source,
-        )));
+    pub fn time_source(
+        mut self,
+        time_source: impl ::aws_smithy_async::time::TimeSource + 'static,
+    ) -> Self {
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
@@ -1089,11 +1237,14 @@
     /// ```
     pub fn retry_classifier(
         mut self,
-        retry_classifier: impl ::aws_smithy_runtime_api::client::retries::classifiers::ClassifyRetry + 'static,
+        retry_classifier: impl ::aws_smithy_runtime_api::client::retries::classifiers::ClassifyRetry
+            + 'static,
     ) -> Self {
-        self.push_retry_classifier(::aws_smithy_runtime_api::client::retries::classifiers::SharedRetryClassifier::new(
-            retry_classifier,
-        ));
+        self.push_retry_classifier(
+            ::aws_smithy_runtime_api::client::retries::classifiers::SharedRetryClassifier::new(
+                retry_classifier,
+            ),
+        );
         self
     }

@@ -1102,7 +1253,8 @@
         &mut self,
         retry_classifier: ::aws_smithy_runtime_api::client::retries::classifiers::SharedRetryClassifier,
     ) -> &mut Self {
-        self.runtime_components.push_retry_classifier(retry_classifier);
+        self.runtime_components
+            .push_retry_classifier(retry_classifier);
         self
     }

@@ -1110,9 +1262,12 @@
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
@@ -1127,7 +1282,10 @@
     ///
     /// This _optional_ name is used to identify the application in the user agent that
     /// gets sent along with requests.
-    pub fn set_app_name(&mut self, app_name: ::std::option::Option<::aws_types::app_name::AppName>) -> &mut Self {
+    pub fn set_app_name(
+        &mut self,
+        app_name: ::std::option::Option<::aws_types::app_name::AppName>,
+    ) -> &mut Self {
         self.config.store_or_unset(app_name);
         self
     }
@@ -1142,7 +1300,10 @@
     /// Entries are de-duplicated on `(name, version)`, rendered in first-seen order, and
     /// the total number of unique entries included in the user agent is capped (currently
     /// at 10); additional entries beyond the cap are dropped with a warning.
-    pub fn framework_metadata(mut self, framework_metadata: ::aws_types::sdk_ua_metadata::FrameworkMetadata) -> Self {
+    pub fn framework_metadata(
+        mut self,
+        framework_metadata: ::aws_types::sdk_ua_metadata::FrameworkMetadata,
+    ) -> Self {
         self.push_framework_metadata(framework_metadata);
         self
     }
@@ -1153,7 +1314,10 @@
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
@@ -1160,7 +1324,10 @@
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
@@ -1191,7 +1358,8 @@
     /// will be prefixed onto this URL. To fully override the endpoint resolver, use
     /// [`Builder::endpoint_resolver`].
     pub fn set_endpoint_url(&mut self, endpoint_url: Option<::std::string::String>) -> &mut Self {
-        self.config.store_or_unset(endpoint_url.map(::aws_types::endpoint_config::EndpointUrl));
+        self.config
+            .store_or_unset(endpoint_url.map(::aws_types::endpoint_config::EndpointUrl));
         self
     }
     /// When true, use the dual-stack endpoint. If the configured endpoint does not support dual-stack, dispatching the request MAY return an error.
@@ -1201,7 +1369,8 @@
     }
     /// When true, use the dual-stack endpoint. If the configured endpoint does not support dual-stack, dispatching the request MAY return an error.
     pub fn set_use_dual_stack(&mut self, use_dual_stack: Option<bool>) -> &mut Self {
-        self.config.store_or_unset(use_dual_stack.map(::aws_types::endpoint_config::UseDualStack));
+        self.config
+            .store_or_unset(use_dual_stack.map(::aws_types::endpoint_config::UseDualStack));
         self
     }
     /// When true, send this request to the FIPS-compliant regional endpoint. If the configured endpoint does not have a FIPS compliant endpoint, dispatching the request will return an error.
@@ -1211,7 +1380,8 @@
     }
     /// When true, send this request to the FIPS-compliant regional endpoint. If the configured endpoint does not have a FIPS compliant endpoint, dispatching the request will return an error.
     pub fn set_use_fips(&mut self, use_fips: Option<bool>) -> &mut Self {
-        self.config.store_or_unset(use_fips.map(::aws_types::endpoint_config::UseFips));
+        self.config
+            .store_or_unset(use_fips.map(::aws_types::endpoint_config::UseFips));
         self
     }
     /// Sets the AWS region to use when making requests.
@@ -1225,24 +1395,36 @@
     ///     .region(Region::new("us-east-1"))
     ///     .build();
     /// ```
-    pub fn region(mut self, region: impl ::std::convert::Into<::std::option::Option<crate::config::Region>>) -> Self {
+    pub fn region(
+        mut self,
+        region: impl ::std::convert::Into<::std::option::Option<crate::config::Region>>,
+    ) -> Self {
         self.set_region(region.into());
         self
     }
     /// Sets the AWS region to use when making requests.
-    pub fn set_region(&mut self, region: ::std::option::Option<crate::config::Region>) -> &mut Self {
+    pub fn set_region(
+        &mut self,
+        region: ::std::option::Option<crate::config::Region>,
+    ) -> &mut Self {
         self.config.store_or_unset(region);
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
             self.runtime_components
                 .set_identity_resolver(::aws_runtime::auth::sigv4::SCHEME_ID, credentials_provider);
@@ -1314,7 +1496,10 @@
     /// let client = aws_sdk_sso::Client::from_conf(config);
     /// ```
     ///
-    pub fn set_behavior_version(&mut self, behavior_version: Option<crate::config::BehaviorVersion>) -> &mut Self {
+    pub fn set_behavior_version(
+        &mut self,
+        behavior_version: Option<crate::config::BehaviorVersion>,
+    ) -> &mut Self {
         self.behavior_version = behavior_version;
         self
     }
@@ -1328,13 +1513,19 @@
     }
     /// Adds a runtime plugin to the config.
     #[allow(unused)]
-    pub(crate) fn runtime_plugin(mut self, plugin: impl crate::config::RuntimePlugin + 'static) -> Self {
+    pub(crate) fn runtime_plugin(
+        mut self,
+        plugin: impl crate::config::RuntimePlugin + 'static,
+    ) -> Self {
         self.push_runtime_plugin(crate::config::SharedRuntimePlugin::new(plugin));
         self
     }
     /// Adds a runtime plugin to the config.
     #[allow(unused)]
-    pub(crate) fn push_runtime_plugin(&mut self, plugin: crate::config::SharedRuntimePlugin) -> &mut Self {
+    pub(crate) fn push_runtime_plugin(
+        &mut self,
+        plugin: crate::config::SharedRuntimePlugin,
+    ) -> &mut Self {
         self.runtime_plugins.push(plugin);
         self
     }
@@ -1342,14 +1533,20 @@
     #[allow(unused_mut)]
     /// Apply test defaults to the builder. NOTE: Consider migrating to use `apply_test_defaults_v2` instead.
     pub fn apply_test_defaults(&mut self) -> &mut Self {
-        self.set_time_source(::std::option::Option::Some(::aws_smithy_async::time::SharedTimeSource::new(
-            ::aws_smithy_async::time::StaticTimeSource::new(::std::time::UNIX_EPOCH + ::std::time::Duration::from_secs(1234567890)),
-        )));
-        self.config.store_put(::aws_runtime::user_agent::AwsUserAgent::for_tests());
+        self.set_time_source(::std::option::Option::Some(
+            ::aws_smithy_async::time::SharedTimeSource::new(
+                ::aws_smithy_async::time::StaticTimeSource::new(
+                    ::std::time::UNIX_EPOCH + ::std::time::Duration::from_secs(1234567890),
+                ),
+            ),
+        ));
+        self.config
+            .store_put(::aws_runtime::user_agent::AwsUserAgent::for_tests());
         self.set_credentials_provider(Some(crate::config::SharedCredentialsProvider::new(
             ::aws_credential_types::Credentials::for_tests(),
         )));
-        self.behavior_version = ::std::option::Option::Some(crate::config::BehaviorVersion::latest());
+        self.behavior_version =
+            ::std::option::Option::Some(crate::config::BehaviorVersion::latest());
         self
     }
     #[cfg(any(feature = "test-util", test))]
@@ -1365,7 +1562,9 @@
     pub fn apply_test_defaults_v2(&mut self) -> &mut Self {
         self.apply_test_defaults();
         if self.config.load::<crate::config::Region>().is_none() {
-            self.set_region(::std::option::Option::Some(crate::config::Region::new("us-east-1")));
+            self.set_region(::std::option::Option::Some(crate::config::Region::new(
+                "us-east-1",
+            )));
         }
         self
     }
@@ -1382,7 +1581,9 @@
         let mut layer = self.config;
         if self.runtime_components.time_source().is_none() {
             self.runtime_components
-                .set_time_source(::std::option::Option::Some(::std::default::Default::default()));
+                .set_time_source(::std::option::Option::Some(
+                    ::std::default::Default::default(),
+                ));
         }
         layer.store_put(crate::meta::API_METADATA.clone());
         layer.store_put(::aws_types::SigningName::from_static("awsssoportal"));
@@ -1404,7 +1605,8 @@
 #[derive(::std::fmt::Debug)]
 pub(crate) struct ServiceRuntimePlugin {
     config: ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer>,
-    runtime_components: ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
+    runtime_components:
+        ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
 }

 impl ServiceRuntimePlugin {
@@ -1411,10 +1613,15 @@
     pub fn new(_service_config: crate::config::Config) -> Self {
         let config = {
             let mut cfg = ::aws_smithy_types::config_bag::Layer::new("SWBPortalService");
-            cfg.store_put(::aws_smithy_runtime::client::orchestrator::AuthSchemeAndEndpointOrchestrationV2);
+            cfg.store_put(
+                ::aws_smithy_runtime::client::orchestrator::AuthSchemeAndEndpointOrchestrationV2,
+            );
             ::std::option::Option::Some(cfg.freeze())
         };
-        let mut runtime_components = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ServiceRuntimePlugin");
+        let mut runtime_components =
+            ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new(
+                "ServiceRuntimePlugin",
+            );
         runtime_components.set_auth_scheme_option_resolver(::std::option::Option::Some({
             use crate::config::auth::ResolveAuthScheme;
             crate::config::auth::DefaultAuthSchemeResolver::default().into_shared_resolver()
@@ -1426,29 +1633,48 @@
         runtime_components.push_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
             ::aws_smithy_runtime::client::http::connection_poisoning::ConnectionPoisoningInterceptor::new(),
         ));
-        runtime_components.push_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::HttpStatusCodeClassifier::default());
-        runtime_components.push_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-            crate::sdk_feature_tracker::retry_mode::RetryModeFeatureTrackerInterceptor::new(),
-        ));
-        runtime_components.push_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
-            ::aws_runtime::service_clock_skew::ServiceClockSkewInterceptor::new(),
-        ));
-        runtime_components.push_interceptor(::aws_runtime::request_info::RequestInfoInterceptor::new());
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
+        runtime_components
+            .push_interceptor(::aws_runtime::request_info::RequestInfoInterceptor::new());
         runtime_components.push_interceptor(::aws_runtime::user_agent::UserAgentInterceptor::new());
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
+        runtime_components
+            .push_interceptor(::aws_runtime::invocation_id::InvocationIdInterceptor::new());
+        runtime_components.push_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+                ::aws_runtime::recursion_detection::RecursionDetectionInterceptor::new(),
+            ),
+        );
+        runtime_components.push_auth_scheme(
+            ::aws_smithy_runtime_api::client::auth::SharedAuthScheme::new(
+                ::aws_runtime::auth::sigv4::SigV4AuthScheme::new(),
+            ),
+        );
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

@@ -1464,7 +1690,10 @@
     fn runtime_components(
         &self,
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
-    ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
+    ) -> ::std::borrow::Cow<
+        '_,
+        ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
+    > {
         ::std::borrow::Cow::Borrowed(&self.runtime_components)
     }
 }
@@ -1478,7 +1707,8 @@
 #[derive(Debug)]
 pub(crate) struct ConfigOverrideRuntimePlugin {
     pub(crate) config: ::aws_smithy_types::config_bag::FrozenLayer,
-    pub(crate) components: ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
+    pub(crate) components:
+        ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
 }

 impl ConfigOverrideRuntimePlugin {
@@ -1491,14 +1721,22 @@
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

@@ -1511,7 +1749,9 @@
         // This is skipped if the override already sets its own identity cache.
         if components.has_identity_resolvers() && components.identity_cache().is_none() {
             components.set_identity_cache(::std::option::Option::Some(
-                ::aws_smithy_runtime::client::identity::IdentityCache::lazy().max_partitions(1).build(),
+                ::aws_smithy_runtime::client::identity::IdentityCache::lazy()
+                    .max_partitions(1)
+                    .build(),
             ));
         }

@@ -1524,7 +1764,9 @@
     }
 }

-impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for ConfigOverrideRuntimePlugin {
+impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin
+    for ConfigOverrideRuntimePlugin
+{
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         Some(self.config.clone())
     }
@@ -1532,7 +1774,10 @@
     fn runtime_components(
         &self,
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
-    ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
+    ) -> ::std::borrow::Cow<
+        '_,
+        ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
+    > {
         ::std::borrow::Cow::Borrowed(&self.components)
     }
 }
@@ -1557,8 +1802,12 @@
                 input
                     .service_config()
                     .and_then(|conf| {
-                        conf.load_config(service_config_key("SSO", "AWS_ENDPOINT_URL", "endpoint_url"))
-                            .map(|it| it.parse().unwrap())
+                        conf.load_config(service_config_key(
+                            "SSO",
+                            "AWS_ENDPOINT_URL",
+                            "endpoint_url",
+                        ))
+                        .map(|it| it.parse().unwrap())
                     })
                     .or_else(|| input.endpoint_url().map(|s| s.to_string())),
             );
@@ -1599,7 +1848,11 @@
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
@@ -1610,13 +1863,16 @@

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
@@ -10,11 +10,15 @@
     ) -> ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionsFuture<'a>;

     /// Convert this service-specific resolver into a `SharedAuthSchemeOptionResolver`
-    fn into_shared_resolver(self) -> ::aws_smithy_runtime_api::client::auth::SharedAuthSchemeOptionResolver
+    fn into_shared_resolver(
+        self,
+    ) -> ::aws_smithy_runtime_api::client::auth::SharedAuthSchemeOptionResolver
     where
         Self: ::std::marker::Sized + 'static,
     {
-        ::aws_smithy_runtime_api::client::auth::SharedAuthSchemeOptionResolver::new(DowncastParams(self))
+        ::aws_smithy_runtime_api::client::auth::SharedAuthSchemeOptionResolver::new(DowncastParams(
+            self,
+        ))
     }
 }

@@ -31,10 +35,15 @@
         runtime_components: &'a ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponents,
     ) -> ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionsFuture<'a> {
         match params.get::<crate::config::auth::Params>() {
-            ::std::option::Option::Some(concrete_params) => self.0.resolve_auth_scheme(concrete_params, cfg, runtime_components),
-            ::std::option::Option::None => ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionsFuture::ready(::std::result::Result::Err(
-                "params of expected type was not present".into(),
-            )),
+            ::std::option::Option::Some(concrete_params) => {
+                self.0
+                    .resolve_auth_scheme(concrete_params, cfg, runtime_components)
+            }
+            ::std::option::Option::None => {
+                ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionsFuture::ready(
+                    ::std::result::Result::Err("params of expected type was not present".into()),
+                )
+            }
         }
     }
 }
@@ -44,7 +53,10 @@
 #[allow(dead_code)]
 pub struct DefaultAuthSchemeResolver {
     service_defaults: Vec<::aws_smithy_runtime_api::client::auth::AuthSchemeOption>,
-    operation_overrides: ::std::collections::HashMap<&'static str, Vec<::aws_smithy_runtime_api::client::auth::AuthSchemeOption>>,
+    operation_overrides: ::std::collections::HashMap<
+        &'static str,
+        Vec<::aws_smithy_runtime_api::client::auth::AuthSchemeOption>,
+    >,
 }

 // TODO(https://github.com/smithy-lang/smithy-rs/issues/4177): Remove `allow(...)` once the issue is addressed.
@@ -55,34 +67,44 @@
 impl Default for DefaultAuthSchemeResolver {
     fn default() -> Self {
         Self {
-            service_defaults: vec![::aws_smithy_runtime_api::client::auth::AuthSchemeOption::builder()
-                .scheme_id(::aws_runtime::auth::sigv4::SCHEME_ID)
-                .build()
-                .expect("required fields set")],
+            service_defaults: vec![
+                ::aws_smithy_runtime_api::client::auth::AuthSchemeOption::builder()
+                    .scheme_id(::aws_runtime::auth::sigv4::SCHEME_ID)
+                    .build()
+                    .expect("required fields set"),
+            ],
             operation_overrides: [
                 (
                     "GetRoleCredentials",
-                    vec![::aws_smithy_runtime_api::client::auth::AuthSchemeOption::from(
-                        ::aws_smithy_runtime::client::auth::no_auth::NO_AUTH_SCHEME_ID,
-                    )],
+                    vec![
+                        ::aws_smithy_runtime_api::client::auth::AuthSchemeOption::from(
+                            ::aws_smithy_runtime::client::auth::no_auth::NO_AUTH_SCHEME_ID,
+                        ),
+                    ],
                 ),
                 (
                     "ListAccountRoles",
-                    vec![::aws_smithy_runtime_api::client::auth::AuthSchemeOption::from(
-                        ::aws_smithy_runtime::client::auth::no_auth::NO_AUTH_SCHEME_ID,
-                    )],
+                    vec![
+                        ::aws_smithy_runtime_api::client::auth::AuthSchemeOption::from(
+                            ::aws_smithy_runtime::client::auth::no_auth::NO_AUTH_SCHEME_ID,
+                        ),
+                    ],
                 ),
                 (
                     "ListAccounts",
-                    vec![::aws_smithy_runtime_api::client::auth::AuthSchemeOption::from(
-                        ::aws_smithy_runtime::client::auth::no_auth::NO_AUTH_SCHEME_ID,
-                    )],
+                    vec![
+                        ::aws_smithy_runtime_api::client::auth::AuthSchemeOption::from(
+                            ::aws_smithy_runtime::client::auth::no_auth::NO_AUTH_SCHEME_ID,
+                        ),
+                    ],
                 ),
                 (
                     "Logout",
-                    vec![::aws_smithy_runtime_api::client::auth::AuthSchemeOption::from(
-                        ::aws_smithy_runtime::client::auth::no_auth::NO_AUTH_SCHEME_ID,
-                    )],
+                    vec![
+                        ::aws_smithy_runtime_api::client::auth::AuthSchemeOption::from(
+                            ::aws_smithy_runtime::client::auth::no_auth::NO_AUTH_SCHEME_ID,
+                        ),
+                    ],
                 ),
             ]
             .into(),
@@ -104,7 +126,9 @@
             None => &self.service_defaults,
         };

-        let _fut = ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionsFuture::ready(Ok(modeled_auth_options.clone()));
+        let _fut = ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionsFuture::ready(Ok(
+            modeled_auth_options.clone(),
+        ));

         _fut
     }
@@ -127,7 +151,9 @@
     }
 }

-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(
+    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
+)]
 /// Builder for [`Params`]
 pub struct ParamsBuilder {
     operation_name: ::std::option::Option<::std::borrow::Cow<'static, str>>,
@@ -134,12 +160,18 @@
 }
 impl ParamsBuilder {
     /// Set the operation name for the builder
-    pub fn operation_name(self, operation_name: impl Into<::std::borrow::Cow<'static, str>>) -> Self {
+    pub fn operation_name(
+        self,
+        operation_name: impl Into<::std::borrow::Cow<'static, str>>,
+    ) -> Self {
         self.set_operation_name(::std::option::Option::Some(operation_name.into()))
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
@@ -147,9 +179,13 @@
     ///
     /// Return [`BuildError`] if any of the required fields are unset.
     ///
-    pub fn build(self) -> ::std::result::Result<crate::config::auth::Params, crate::config::auth::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<crate::config::auth::Params, crate::config::auth::BuildError> {
         ::std::result::Result::Ok(crate::config::auth::Params {
-            operation_name: self.operation_name.ok_or_else(|| BuildError::missing("operation_name"))?,
+            operation_name: self
+                .operation_name
+                .ok_or_else(|| BuildError::missing("operation_name"))?,
         })
     }
 }
@@ -162,7 +198,9 @@

 impl BuildError {
     fn missing(field: &'static str) -> Self {
-        Self { field: field.into() }
+        Self {
+            field: field.into(),
+        }
     }
 }

diff --git a/src/config/endpoint.rs b/src/config/endpoint.rs
--- a/src/config/endpoint.rs
+++ b/src/config/endpoint.rs
@@ -8,7 +8,9 @@
 pub(crate) struct EndpointOverrideFeatureTrackerInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for EndpointOverrideFeatureTrackerInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept
+    for EndpointOverrideFeatureTrackerInterceptor
+{
     fn name(&self) -> &'static str {
         "EndpointOverrideFeatureTrackerInterceptor"
     }
@@ -18,7 +20,10 @@
         _context: &::aws_smithy_runtime_api::client::interceptors::context::BeforeSerializationInterceptorContextRef<'_>,
         cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
     ) -> ::std::result::Result<(), ::aws_smithy_runtime_api::box_error::BoxError> {
-        if cfg.load::<::aws_types::endpoint_config::EndpointUrl>().is_some() {
+        if cfg
+            .load::<::aws_types::endpoint_config::EndpointUrl>()
+            .is_some()
+        {
             cfg.interceptor_state()
                 .store_append(::aws_runtime::sdk_feature::AwsSdkFeature::EndpointOverride);
         }
@@ -40,7 +45,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.ap-east-1.amazonaws.com");
+        let endpoint =
+            endpoint.expect("Expected valid endpoint: https://portal.sso.ap-east-1.amazonaws.com");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -60,7 +66,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.ap-northeast-1.amazonaws.com");
+        let endpoint = endpoint
+            .expect("Expected valid endpoint: https://portal.sso.ap-northeast-1.amazonaws.com");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -80,7 +87,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.ap-northeast-2.amazonaws.com");
+        let endpoint = endpoint
+            .expect("Expected valid endpoint: https://portal.sso.ap-northeast-2.amazonaws.com");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -100,7 +108,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.ap-northeast-3.amazonaws.com");
+        let endpoint = endpoint
+            .expect("Expected valid endpoint: https://portal.sso.ap-northeast-3.amazonaws.com");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -120,7 +129,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.ap-south-1.amazonaws.com");
+        let endpoint =
+            endpoint.expect("Expected valid endpoint: https://portal.sso.ap-south-1.amazonaws.com");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -140,7 +150,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.ap-southeast-1.amazonaws.com");
+        let endpoint = endpoint
+            .expect("Expected valid endpoint: https://portal.sso.ap-southeast-1.amazonaws.com");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -160,7 +171,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.ap-southeast-2.amazonaws.com");
+        let endpoint = endpoint
+            .expect("Expected valid endpoint: https://portal.sso.ap-southeast-2.amazonaws.com");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -180,7 +192,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.ca-central-1.amazonaws.com");
+        let endpoint = endpoint
+            .expect("Expected valid endpoint: https://portal.sso.ca-central-1.amazonaws.com");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -200,7 +213,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.eu-central-1.amazonaws.com");
+        let endpoint = endpoint
+            .expect("Expected valid endpoint: https://portal.sso.eu-central-1.amazonaws.com");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -220,7 +234,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.eu-north-1.amazonaws.com");
+        let endpoint =
+            endpoint.expect("Expected valid endpoint: https://portal.sso.eu-north-1.amazonaws.com");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -240,7 +255,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.eu-south-1.amazonaws.com");
+        let endpoint =
+            endpoint.expect("Expected valid endpoint: https://portal.sso.eu-south-1.amazonaws.com");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -260,7 +276,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.eu-west-1.amazonaws.com");
+        let endpoint =
+            endpoint.expect("Expected valid endpoint: https://portal.sso.eu-west-1.amazonaws.com");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -280,7 +297,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.eu-west-2.amazonaws.com");
+        let endpoint =
+            endpoint.expect("Expected valid endpoint: https://portal.sso.eu-west-2.amazonaws.com");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -300,7 +318,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.eu-west-3.amazonaws.com");
+        let endpoint =
+            endpoint.expect("Expected valid endpoint: https://portal.sso.eu-west-3.amazonaws.com");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -320,7 +339,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.me-south-1.amazonaws.com");
+        let endpoint =
+            endpoint.expect("Expected valid endpoint: https://portal.sso.me-south-1.amazonaws.com");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -340,7 +360,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.sa-east-1.amazonaws.com");
+        let endpoint =
+            endpoint.expect("Expected valid endpoint: https://portal.sso.sa-east-1.amazonaws.com");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -360,7 +381,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.us-east-1.amazonaws.com");
+        let endpoint =
+            endpoint.expect("Expected valid endpoint: https://portal.sso.us-east-1.amazonaws.com");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -380,7 +402,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.us-east-2.amazonaws.com");
+        let endpoint =
+            endpoint.expect("Expected valid endpoint: https://portal.sso.us-east-2.amazonaws.com");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -400,7 +423,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.us-west-2.amazonaws.com");
+        let endpoint =
+            endpoint.expect("Expected valid endpoint: https://portal.sso.us-west-2.amazonaws.com");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -420,7 +444,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso-fips.us-east-1.api.aws");
+        let endpoint =
+            endpoint.expect("Expected valid endpoint: https://portal.sso-fips.us-east-1.api.aws");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -440,7 +465,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso-fips.us-east-1.amazonaws.com");
+        let endpoint = endpoint
+            .expect("Expected valid endpoint: https://portal.sso-fips.us-east-1.amazonaws.com");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -460,7 +486,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.us-east-1.api.aws");
+        let endpoint =
+            endpoint.expect("Expected valid endpoint: https://portal.sso.us-east-1.api.aws");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -500,7 +527,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso-fips.cn-north-1.amazonaws.com.cn");
+        let endpoint = endpoint
+            .expect("Expected valid endpoint: https://portal.sso-fips.cn-north-1.amazonaws.com.cn");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -520,7 +548,9 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.cn-north-1.api.amazonwebservices.com.cn");
+        let endpoint = endpoint.expect(
+            "Expected valid endpoint: https://portal.sso.cn-north-1.api.amazonwebservices.com.cn",
+        );
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -540,7 +570,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.cn-north-1.amazonaws.com.cn");
+        let endpoint = endpoint
+            .expect("Expected valid endpoint: https://portal.sso.cn-north-1.amazonaws.com.cn");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -560,7 +591,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.us-gov-east-1.amazonaws.com");
+        let endpoint = endpoint
+            .expect("Expected valid endpoint: https://portal.sso.us-gov-east-1.amazonaws.com");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -580,7 +612,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.us-gov-west-1.amazonaws.com");
+        let endpoint = endpoint
+            .expect("Expected valid endpoint: https://portal.sso.us-gov-west-1.amazonaws.com");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -600,7 +633,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso-fips.us-gov-east-1.api.aws");
+        let endpoint = endpoint
+            .expect("Expected valid endpoint: https://portal.sso-fips.us-gov-east-1.api.aws");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -620,7 +654,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.us-gov-east-1.amazonaws.com");
+        let endpoint = endpoint
+            .expect("Expected valid endpoint: https://portal.sso.us-gov-east-1.amazonaws.com");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -640,7 +675,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.us-gov-east-1.api.aws");
+        let endpoint =
+            endpoint.expect("Expected valid endpoint: https://portal.sso.us-gov-east-1.api.aws");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -660,7 +696,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso-fips.us-iso-east-1.c2s.ic.gov");
+        let endpoint = endpoint
+            .expect("Expected valid endpoint: https://portal.sso-fips.us-iso-east-1.c2s.ic.gov");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -680,7 +717,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.us-iso-east-1.c2s.ic.gov");
+        let endpoint =
+            endpoint.expect("Expected valid endpoint: https://portal.sso.us-iso-east-1.c2s.ic.gov");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -700,7 +738,9 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso-fips.us-isob-east-1.sc2s.sgov.gov");
+        let endpoint = endpoint.expect(
+            "Expected valid endpoint: https://portal.sso-fips.us-isob-east-1.sc2s.sgov.gov",
+        );
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -720,7 +760,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.us-isob-east-1.sc2s.sgov.gov");
+        let endpoint = endpoint
+            .expect("Expected valid endpoint: https://portal.sso.us-isob-east-1.sc2s.sgov.gov");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -744,7 +785,9 @@
         let endpoint = endpoint.expect("Expected valid endpoint: https://example.com");
         assert_eq!(
             endpoint,
-            ::aws_smithy_types::endpoint::Endpoint::builder().url("https://example.com").build()
+            ::aws_smithy_types::endpoint::Endpoint::builder()
+                .url("https://example.com")
+                .build()
         );
     }

@@ -762,7 +805,9 @@
         let endpoint = endpoint.expect("Expected valid endpoint: https://example.com");
         assert_eq!(
             endpoint,
-            ::aws_smithy_types::endpoint::Endpoint::builder().url("https://example.com").build()
+            ::aws_smithy_types::endpoint::Endpoint::builder()
+                .url("https://example.com")
+                .build()
         );
     }

@@ -779,7 +824,10 @@
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
@@ -804,11 +852,17 @@
     /// Missing region
     #[test]
     fn test_40() {
-        let params = crate::config::endpoint::Params::builder().build().expect("invalid params");
+        let params = crate::config::endpoint::Params::builder()
+            .build()
+            .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let error = endpoint.expect_err("expected error: Invalid Configuration: Missing Region [Missing region]");
-        assert_eq!(format!("{}", error), "Invalid Configuration: Missing Region")
+        let error = endpoint
+            .expect_err("expected error: Invalid Configuration: Missing Region [Missing region]");
+        assert_eq!(
+            format!("{}", error),
+            "Invalid Configuration: Missing Region"
+        )
     }
 }

@@ -815,16 +869,23 @@
 /// Endpoint resolver trait specific to AWS Single Sign-On
 pub trait ResolveEndpoint: ::std::marker::Send + ::std::marker::Sync + ::std::fmt::Debug {
     /// Resolve an endpoint with the given parameters
-    fn resolve_endpoint<'a>(&'a self, params: &'a crate::config::endpoint::Params) -> ::aws_smithy_runtime_api::client::endpoint::EndpointFuture<'a>;
+    fn resolve_endpoint<'a>(
+        &'a self,
+        params: &'a crate::config::endpoint::Params,
+    ) -> ::aws_smithy_runtime_api::client::endpoint::EndpointFuture<'a>;

     /// Convert this service-specific resolver into a `SharedEndpointResolver`
     ///
     /// The resulting resolver will downcast `EndpointResolverParams` into `crate::config::endpoint::Params`.
-    fn into_shared_resolver(self) -> ::aws_smithy_runtime_api::client::endpoint::SharedEndpointResolver
+    fn into_shared_resolver(
+        self,
+    ) -> ::aws_smithy_runtime_api::client::endpoint::SharedEndpointResolver
     where
         Self: Sized + 'static,
     {
-        ::aws_smithy_runtime_api::client::endpoint::SharedEndpointResolver::new(DowncastParams(self))
+        ::aws_smithy_runtime_api::client::endpoint::SharedEndpointResolver::new(DowncastParams(
+            self,
+        ))
     }
 }

@@ -840,7 +901,9 @@
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
@@ -850,7 +913,9 @@
 /// The default endpoint resolver.
 pub struct DefaultResolver {
     partition_resolver: &'static crate::endpoint_lib::partition::PartitionResolver,
-    endpoint_cache: ::arc_swap::ArcSwap<::std::option::Option<(Params, ::aws_smithy_types::endpoint::Endpoint)>>,
+    endpoint_cache: ::arc_swap::ArcSwap<
+        ::std::option::Option<(Params, ::aws_smithy_types::endpoint::Endpoint)>,
+    >,
 }

 impl Default for DefaultResolver {
@@ -884,7 +949,10 @@
     fn resolve_endpoint<'a>(
         &'a self,
         params: &'a crate::config::endpoint::Params,
-    ) -> ::std::result::Result<::aws_smithy_types::endpoint::Endpoint, ::aws_smithy_runtime_api::box_error::BoxError> {
+    ) -> ::std::result::Result<
+        ::aws_smithy_types::endpoint::Endpoint,
+        ::aws_smithy_runtime_api::box_error::BoxError,
+    > {
         let mut _diagnostic_collector = crate::endpoint_lib::diagnostic::DiagnosticCollector::new();
         #[allow(unused_mut)]
         let mut context = ConditionContext::default();
@@ -1018,10 +1086,12 @@
                     };
                 }
                 1 | -1 => {
-                    return ::std::result::Result::Err(
-                        Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message("No endpoint rule matched"))
-                            as ::aws_smithy_runtime_api::box_error::BoxError,
+                    return ::std::result::Result::Err(Box::new(
+                        ::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                            "No endpoint rule matched",
+                        ),
                     )
+                        as ::aws_smithy_runtime_api::box_error::BoxError)
                 }
                 ref_val => {
                     let is_complement = ref_val < 0;
@@ -1070,7 +1140,11 @@
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
@@ -1078,18 +1152,26 @@
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
+                return ::aws_smithy_runtime_api::client::endpoint::EndpointFuture::ready(
+                    ::std::result::Result::Ok(cached_endpoint.clone()),
+                );
             }
         }
         drop(cached);
         let result = self.resolve_endpoint(params);
         if let ::std::result::Result::Ok(ref endpoint) = result {
-            self.endpoint_cache.store(::std::sync::Arc::new(Some((params.clone(), endpoint.clone()))));
+            self.endpoint_cache.store(::std::sync::Arc::new(Some((
+                params.clone(),
+                endpoint.clone(),
+            ))));
         }
         ::aws_smithy_runtime_api::client::endpoint::EndpointFuture::ready(result)
     }
@@ -1213,7 +1295,9 @@
 }

 /// Builder for [`Params`]
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(
+    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
+)]
 pub struct ParamsBuilder {
     region: ::std::option::Option<::std::string::String>,
     use_dual_stack: ::std::option::Option<bool>,
@@ -1222,7 +1306,12 @@
 }
 impl ParamsBuilder {
     /// Consume this builder, creating [`Params`].
-    pub fn build(self) -> ::std::result::Result<crate::config::endpoint::Params, crate::config::endpoint::InvalidParams> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<
+        crate::config::endpoint::Params,
+        crate::config::endpoint::InvalidParams,
+    > {
         if let Some(region) = &self.region {
             if !crate::endpoint_lib::host::is_valid_host_label(
                 region.as_ref() as &str,
@@ -1239,10 +1328,9 @@
             #[allow(clippy::unnecessary_lazy_evaluations)]
             crate::config::endpoint::Params {
                 region: self.region,
-                use_dual_stack: self
-                    .use_dual_stack
-                    .or_else(|| Some(false))
-                    .ok_or_else(|| crate::config::endpoint::InvalidParams::missing("use_dual_stack"))?,
+                use_dual_stack: self.use_dual_stack.or_else(|| Some(false)).ok_or_else(|| {
+                    crate::config::endpoint::InvalidParams::missing("use_dual_stack")
+                })?,
                 use_fips: self
                     .use_fips
                     .or_else(|| Some(false))
@@ -1352,8 +1440,12 @@
 impl std::fmt::Display for InvalidParams {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
         match self.kind {
-            InvalidParamsErrorKind::MissingField => write!(f, "a required field was missing: `{}`", self.field),
-            InvalidParamsErrorKind::InvalidValue { message } => write!(f, "invalid value for field: `{}` - {}", self.field, message),
+            InvalidParamsErrorKind::MissingField => {
+                write!(f, "a required field was missing: `{}`", self.field)
+            }
+            InvalidParamsErrorKind::InvalidValue { message } => {
+                write!(f, "invalid value for field: `{}` - {}", self.field, message)
+            }
         }
     }
 }
diff --git a/src/endpoint_lib.rs b/src/endpoint_lib.rs
--- a/src/endpoint_lib.rs
+++ b/src/endpoint_lib.rs
@@ -1,12 +1,18 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 // Loading the partition JSON is expensive since it involves many regex compilations,
 // so cache the result so that it only need to be paid for the first constructed client.
-pub(crate) static DEFAULT_PARTITION_RESOLVER: std::sync::LazyLock<crate::endpoint_lib::partition::PartitionResolver> = std::sync::LazyLock::new(
+pub(crate) static DEFAULT_PARTITION_RESOLVER: std::sync::LazyLock<
+    crate::endpoint_lib::partition::PartitionResolver,
+> = std::sync::LazyLock::new(
     || match std::env::var("SMITHY_CLIENT_SDK_CUSTOM_PARTITION") {
         Ok(partitions) => {
             ::tracing::debug!("loading custom partitions located at {partitions}");
-            let partition_dot_json = std::fs::read_to_string(partitions).expect("should be able to read a custom partition JSON");
-            crate::endpoint_lib::partition::PartitionResolver::new_from_json(partition_dot_json.as_bytes()).expect("valid JSON")
+            let partition_dot_json = std::fs::read_to_string(partitions)
+                .expect("should be able to read a custom partition JSON");
+            crate::endpoint_lib::partition::PartitionResolver::new_from_json(
+                partition_dot_json.as_bytes(),
+            )
+            .expect("valid JSON")
         }
         _ => {
             ::tracing::debug!("loading default partitions");
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
@@ -66,9 +71,14 @@
                 let node = nodes.get(node_index)?;
                 let condition_index = node.condition_index as usize;
                 let condition = conditions.get(condition_index)?;
-                let condition_result = condition_evaluator(condition, params, context, diagnostic_collector);
+                let condition_result =
+                    condition_evaluator(condition, params, context, diagnostic_collector);
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
diff --git a/src/endpoint_lib/diagnostic.rs b/src/endpoint_lib/diagnostic.rs
--- a/src/endpoint_lib/diagnostic.rs
+++ b/src/endpoint_lib/diagnostic.rs
@@ -24,7 +24,10 @@

     #[allow(unused)]
     /// Capture a result, returning Some(t) when the input was `Ok` and `None` otherwise
-    pub(crate) fn capture<T, E: Into<Box<dyn Error + Send + Sync>>>(&mut self, err: Result<T, E>) -> Option<T> {
+    pub(crate) fn capture<T, E: Into<Box<dyn Error + Send + Sync>>>(
+        &mut self,
+        err: Result<T, E>,
+    ) -> Option<T> {
         match err {
             Ok(res) => Some(res),
             Err(e) => {
diff --git a/src/endpoint_lib/host.rs b/src/endpoint_lib/host.rs
--- a/src/endpoint_lib/host.rs
+++ b/src/endpoint_lib/host.rs
@@ -6,7 +6,11 @@

 use crate::endpoint_lib::diagnostic::DiagnosticCollector;

-pub(crate) fn is_valid_host_label(label: &str, allow_dots: bool, e: &mut DiagnosticCollector) -> bool {
+pub(crate) fn is_valid_host_label(
+    label: &str,
+    allow_dots: bool,
+    e: &mut DiagnosticCollector,
+) -> bool {
     let bytes = label.as_bytes();
     if allow_dots {
         let mut start = 0;
@@ -63,7 +67,10 @@
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

@@ -94,7 +98,9 @@
         self.partitions.push(partition);
     }

-    pub(crate) fn new_from_json(partition_dot_json: &[u8]) -> Result<PartitionResolver, DeserializeError> {
+    pub(crate) fn new_from_json(
+        partition_dot_json: &[u8],
+    ) -> Result<PartitionResolver, DeserializeError> {
         deserialize_partitions(partition_dot_json)
     }

@@ -112,26 +118,41 @@
     /// 4. After matching the identifier to a partition using one of the previous steps, the partition function should return a
     ///    typed data structure containing the fields in `outputs` in the matched partition. **Important:** If a specific region
     ///    was matched, the properties associated with that region **MUST** be merged with the `outputs` field.
-    pub(crate) fn resolve_partition(&self, region: &str, e: &mut DiagnosticCollector) -> Option<Partition<'_>> {
-        let mut explicit_match_partition = self.partitions.iter().flat_map(|part| part.explicit_match(region));
-        let mut regex_match_partition = self.partitions.iter().flat_map(|part| part.regex_match(region));
+    pub(crate) fn resolve_partition(
+        &self,
+        region: &str,
+        e: &mut DiagnosticCollector,
+    ) -> Option<Partition<'_>> {
+        let mut explicit_match_partition = self
+            .partitions
+            .iter()
+            .flat_map(|part| part.explicit_match(region));
+        let mut regex_match_partition = self
+            .partitions
+            .iter()
+            .flat_map(|part| part.regex_match(region));

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
             dns_suffix: merge!(base, region_override, dns_suffix),
             dual_stack_dns_suffix: merge!(base, region_override, dual_stack_dns_suffix),
-            supports_fips: region_override.supports_fips.unwrap_or(base.outputs.supports_fips),
-            supports_dual_stack: region_override.supports_dual_stack.unwrap_or(base.outputs.supports_dual_stack),
+            supports_fips: region_override
+                .supports_fips
+                .unwrap_or(base.outputs.supports_fips),
+            supports_dual_stack: region_override
+                .supports_dual_stack
+                .unwrap_or(base.outputs.supports_dual_stack),
             implicit_global_region: merge!(base, region_override, implicit_global_region),
         })
     }
@@ -171,11 +192,19 @@
 }

 impl PartitionMetadata {
-    fn explicit_match(&self, region: &str) -> Option<(&PartitionMetadata, Option<&PartitionOutputOverride>)> {
-        self.regions.get(region).map(|output_override| (self, Some(output_override)))
+    fn explicit_match(
+        &self,
+        region: &str,
+    ) -> Option<(&PartitionMetadata, Option<&PartitionOutputOverride>)> {
+        self.regions
+            .get(region)
+            .map(|output_override| (self, Some(output_override)))
     }

-    fn regex_match(&self, region: &str) -> Option<(&PartitionMetadata, Option<&PartitionOutputOverride>)> {
+    fn regex_match(
+        &self,
+        region: &str,
+    ) -> Option<(&PartitionMetadata, Option<&PartitionOutputOverride>)> {
         if self.region_regex.is_match(region) {
             Some((self, None))
         } else {
@@ -205,14 +234,22 @@
 }

 impl PartitionOutputOverride {
-    pub(crate) fn into_partition_output(self) -> Result<PartitionOutput, Box<dyn std::error::Error>> {
+    pub(crate) fn into_partition_output(
+        self,
+    ) -> Result<PartitionOutput, Box<dyn std::error::Error>> {
         Ok(PartitionOutput {
             name: self.name.ok_or("missing name")?,
             dns_suffix: self.dns_suffix.ok_or("missing dnsSuffix")?,
-            dual_stack_dns_suffix: self.dual_stack_dns_suffix.ok_or("missing dual_stackDnsSuffix")?,
+            dual_stack_dns_suffix: self
+                .dual_stack_dns_suffix
+                .ok_or("missing dual_stackDnsSuffix")?,
             supports_fips: self.supports_fips.ok_or("missing supports fips")?,
-            supports_dual_stack: self.supports_dual_stack.ok_or("missing supportsDualstack")?,
-            implicit_global_region: self.implicit_global_region.ok_or("missing implicitGlobalRegion")?,
+            supports_dual_stack: self
+                .supports_dual_stack
+                .ok_or("missing supportsDualstack")?,
+            implicit_global_region: self
+                .implicit_global_region
+                .ok_or("missing implicitGlobalRegion")?,
         })
     }
 }
@@ -221,14 +258,20 @@
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
     use std::collections::HashMap;

-    pub(crate) fn deserialize_partitions(value: &[u8]) -> Result<PartitionResolver, DeserializeError> {
+    pub(crate) fn deserialize_partitions(
+        value: &[u8],
+    ) -> Result<PartitionResolver, DeserializeError> {
         let mut tokens_owned = json_token_iter(value).peekable();
         let tokens = &mut tokens_owned;
         expect_start_object(tokens.next())?;
@@ -238,20 +281,30 @@
                 Some(Token::EndObject { .. }) => break,
                 Some(Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                     "partitions" => {
-                        resolver = Some(PartitionResolver::from_partitions(deser_partitions(tokens)?));
+                        resolver = Some(PartitionResolver::from_partitions(deser_partitions(
+                            tokens,
+                        )?));
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

-    fn deser_partitions<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Vec<PartitionMetadata>, DeserializeError>
+    fn deser_partitions<'a, I>(
+        tokens: &mut std::iter::Peekable<I>,
+    ) -> Result<Vec<PartitionMetadata>, DeserializeError>
     where
         I: Iterator<Item = Result<Token<'a>, DeserializeError>>,
     {
@@ -275,7 +328,9 @@
         }
     }

-    pub(crate) fn deser_partition<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<PartitionMetadata, DeserializeError>
+    pub(crate) fn deser_partition<'a, I>(
+        tokens: &mut std::iter::Peekable<I>,
+    ) -> Result<PartitionMetadata, DeserializeError>
     where
         I: Iterator<Item = Result<Token<'a>, DeserializeError>>,
     {
@@ -303,7 +358,11 @@
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
@@ -332,7 +391,11 @@
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
@@ -342,7 +405,9 @@
     }

     /// Convert a token to `Str` (a potentially static String)
-    fn token_to_str(token: Option<Result<Token, DeserializeError>>) -> Result<Option<super::Str>, DeserializeError> {
+    fn token_to_str(
+        token: Option<Result<Token, DeserializeError>>,
+    ) -> Result<Option<super::Str>, DeserializeError> {
         Ok(expect_string_or_null(token)?
             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
             .transpose()?
@@ -349,7 +414,9 @@
             .map(Cow::Owned))
     }

-    fn deser_outputs<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<PartitionOutputOverride>, DeserializeError>
+    fn deser_outputs<'a, I>(
+        tokens: &mut std::iter::Peekable<I>,
+    ) -> Result<Option<PartitionOutputOverride>, DeserializeError>
     where
         I: Iterator<Item = Result<Token<'a>, DeserializeError>>,
     {
@@ -381,7 +448,11 @@
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
@@ -394,7 +465,9 @@
 #[cfg(test)]
 mod test {
     use crate::endpoint_lib::diagnostic::DiagnosticCollector;
-    use crate::endpoint_lib::partition::{Partition, PartitionMetadata, PartitionOutput, PartitionOutputOverride, PartitionResolver};
+    use crate::endpoint_lib::partition::{
+        Partition, PartitionMetadata, PartitionOutput, PartitionOutputOverride, PartitionResolver,
+    };
     use regex_lite::Regex;
     use std::collections::HashMap;

@@ -508,11 +581,18 @@
     }
   ]
 }"#;
-        let resolver = super::deser::deserialize_partitions(partitions.as_bytes()).expect("valid resolver");
+        let resolver =
+            super::deser::deserialize_partitions(partitions.as_bytes()).expect("valid resolver");
         assert_eq!(resolve(&resolver, "cn-north-1").name, "aws-cn");
-        assert_eq!(resolve(&resolver, "cn-north-1").dns_suffix, "amazonaws.com.cn");
+        assert_eq!(
+            resolve(&resolver, "cn-north-1").dns_suffix,
+            "amazonaws.com.cn"
+        );
         assert_eq!(resolver.partitions.len(), 5);
-        assert_eq!(resolve(&resolver, "af-south-1").implicit_global_region, "us-east-1");
+        assert_eq!(
+            resolve(&resolver, "af-south-1").implicit_global_region,
+            "us-east-1"
+        );
     }

     #[test]
@@ -551,7 +631,10 @@
         assert_eq!(resolve(&resolver, "us-east-1").name, "aws");
         assert_eq!(resolve(&resolver, "other-west-2").name, "other");
         // mars-east-1 hits aws through the default fallback
-        assert_eq!(resolve(&resolver, "mars-east-1").dns_suffix, "amazonaws.com");
+        assert_eq!(
+            resolve(&resolver, "mars-east-1").dns_suffix,
+            "amazonaws.com"
+        );
         // mars-east-2 hits aws through the region override
         assert_eq!(resolve(&resolver, "mars-east-2").dns_suffix, "mars.aws");
     }
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
@@ -3,32 +3,82 @@
 #[non_exhaustive]
 #[derive(::std::fmt::Debug)]
 pub enum Error {
+    #[cfg(any(
+        feature = "op_get_role_credentials",
+        feature = "op_list_account_roles",
+        feature = "op_list_accounts",
+        feature = "op_logout"
+    ))]
     /// <p>Indicates that a problem occurred with the input to the request. For example, a required parameter might be missing or out of range.</p>
     InvalidRequestException(crate::types::error::InvalidRequestException),
+    #[cfg(any(
+        feature = "op_get_role_credentials",
+        feature = "op_list_account_roles",
+        feature = "op_list_accounts"
+    ))]
     /// <p>The specified resource doesn't exist.</p>
     ResourceNotFoundException(crate::types::error::ResourceNotFoundException),
+    #[cfg(any(
+        feature = "op_get_role_credentials",
+        feature = "op_list_account_roles",
+        feature = "op_list_accounts",
+        feature = "op_logout"
+    ))]
     /// <p>Indicates that the request is being made too frequently and is more than what the server can handle.</p>
     TooManyRequestsException(crate::types::error::TooManyRequestsException),
+    #[cfg(any(
+        feature = "op_get_role_credentials",
+        feature = "op_list_account_roles",
+        feature = "op_list_accounts",
+        feature = "op_logout"
+    ))]
     /// <p>Indicates that the request is not authorized. This can happen due to an invalid access token in the request.</p>
     UnauthorizedException(crate::types::error::UnauthorizedException),
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
+                feature = "op_get_role_credentials",
+                feature = "op_list_account_roles",
+                feature = "op_list_accounts",
+                feature = "op_logout"
+            ))]
             Error::InvalidRequestException(inner) => inner.fmt(f),
+            #[cfg(any(
+                feature = "op_get_role_credentials",
+                feature = "op_list_account_roles",
+                feature = "op_list_accounts"
+            ))]
             Error::ResourceNotFoundException(inner) => inner.fmt(f),
+            #[cfg(any(
+                feature = "op_get_role_credentials",
+                feature = "op_list_account_roles",
+                feature = "op_list_accounts",
+                feature = "op_logout"
+            ))]
             Error::TooManyRequestsException(inner) => inner.fmt(f),
+            #[cfg(any(
+                feature = "op_get_role_credentials",
+                feature = "op_list_account_roles",
+                feature = "op_list_accounts",
+                feature = "op_logout"
+            ))]
             Error::UnauthorizedException(inner) => inner.fmt(f),
             Error::Unhandled(_) => {
-                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
+                if let ::std::option::Option::Some(code) =
+                    ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
+                {
                     write!(f, "unhandled error ({code})")
                 } else {
                     f.write_str("unhandled error")
@@ -48,21 +98,58 @@
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for Error {
     fn meta(&self) -> &::aws_smithy_types::error::metadata::ErrorMetadata {
         match self {
+            #[cfg(any(
+                feature = "op_get_role_credentials",
+                feature = "op_list_account_roles",
+                feature = "op_list_accounts",
+                feature = "op_logout"
+            ))]
             Self::InvalidRequestException(inner) => inner.meta(),
+            #[cfg(any(
+                feature = "op_get_role_credentials",
+                feature = "op_list_account_roles",
+                feature = "op_list_accounts"
+            ))]
             Self::ResourceNotFoundException(inner) => inner.meta(),
+            #[cfg(any(
+                feature = "op_get_role_credentials",
+                feature = "op_list_account_roles",
+                feature = "op_list_accounts",
+                feature = "op_logout"
+            ))]
             Self::TooManyRequestsException(inner) => inner.meta(),
+            #[cfg(any(
+                feature = "op_get_role_credentials",
+                feature = "op_list_account_roles",
+                feature = "op_list_accounts",
+                feature = "op_logout"
+            ))]
             Self::UnauthorizedException(inner) => inner.meta(),
             Self::Unhandled(inner) => &inner.meta,
         }
     }
 }
-impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_role_credentials::GetRoleCredentialsError, R>> for Error
+#[cfg(feature = "op_get_role_credentials")]
+impl<R>
+    From<
+        ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::get_role_credentials::GetRoleCredentialsError,
+            R,
+        >,
+    > for Error
 where
     R: Send + Sync + std::fmt::Debug + 'static,
 {
-    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_role_credentials::GetRoleCredentialsError, R>) -> Self {
+    fn from(
+        err: ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::get_role_credentials::GetRoleCredentialsError,
+            R,
+        >,
+    ) -> Self {
         match err {
-            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
+            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => {
+                Self::from(context.into_err())
+            }
             _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                 meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                 source: err.into(),
@@ -70,6 +157,7 @@
         }
     }
 }
+#[cfg(feature = "op_get_role_credentials")]
 impl From<crate::operation::get_role_credentials::GetRoleCredentialsError> for Error {
     fn from(err: crate::operation::get_role_credentials::GetRoleCredentialsError) -> Self {
         match err {
@@ -85,13 +173,27 @@
         }
     }
 }
-impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_account_roles::ListAccountRolesError, R>> for Error
+#[cfg(feature = "op_list_account_roles")]
+impl<R>
+    From<
+        ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::list_account_roles::ListAccountRolesError,
+            R,
+        >,
+    > for Error
 where
     R: Send + Sync + std::fmt::Debug + 'static,
 {
-    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_account_roles::ListAccountRolesError, R>) -> Self {
+    fn from(
+        err: ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::list_account_roles::ListAccountRolesError,
+            R,
+        >,
+    ) -> Self {
         match err {
-            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
+            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => {
+                Self::from(context.into_err())
+            }
             _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                 meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                 source: err.into(),
@@ -99,6 +201,7 @@
         }
     }
 }
+#[cfg(feature = "op_list_account_roles")]
 impl From<crate::operation::list_account_roles::ListAccountRolesError> for Error {
     fn from(err: crate::operation::list_account_roles::ListAccountRolesError) -> Self {
         match err {
@@ -110,13 +213,27 @@
         }
     }
 }
-impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_accounts::ListAccountsError, R>> for Error
+#[cfg(feature = "op_list_accounts")]
+impl<R>
+    From<
+        ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::list_accounts::ListAccountsError,
+            R,
+        >,
+    > for Error
 where
     R: Send + Sync + std::fmt::Debug + 'static,
 {
-    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_accounts::ListAccountsError, R>) -> Self {
+    fn from(
+        err: ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::list_accounts::ListAccountsError,
+            R,
+        >,
+    ) -> Self {
         match err {
-            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
+            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => {
+                Self::from(context.into_err())
+            }
             _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                 meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                 source: err.into(),
@@ -124,24 +241,49 @@
         }
     }
 }
+#[cfg(feature = "op_list_accounts")]
 impl From<crate::operation::list_accounts::ListAccountsError> for Error {
     fn from(err: crate::operation::list_accounts::ListAccountsError) -> Self {
         match err {
-            crate::operation::list_accounts::ListAccountsError::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
-            crate::operation::list_accounts::ListAccountsError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
-            crate::operation::list_accounts::ListAccountsError::TooManyRequestsException(inner) => Error::TooManyRequestsException(inner),
-            crate::operation::list_accounts::ListAccountsError::UnauthorizedException(inner) => Error::UnauthorizedException(inner),
-            crate::operation::list_accounts::ListAccountsError::Unhandled(inner) => Error::Unhandled(inner),
+            crate::operation::list_accounts::ListAccountsError::InvalidRequestException(inner) => {
+                Error::InvalidRequestException(inner)
+            }
+            crate::operation::list_accounts::ListAccountsError::ResourceNotFoundException(
+                inner,
+            ) => Error::ResourceNotFoundException(inner),
+            crate::operation::list_accounts::ListAccountsError::TooManyRequestsException(inner) => {
+                Error::TooManyRequestsException(inner)
+            }
+            crate::operation::list_accounts::ListAccountsError::UnauthorizedException(inner) => {
+                Error::UnauthorizedException(inner)
+            }
+            crate::operation::list_accounts::ListAccountsError::Unhandled(inner) => {
+                Error::Unhandled(inner)
+            }
         }
     }
 }
-impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::logout::LogoutError, R>> for Error
+#[cfg(feature = "op_logout")]
+impl<R>
+    From<
+        ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::logout::LogoutError,
+            R,
+        >,
+    > for Error
 where
     R: Send + Sync + std::fmt::Debug + 'static,
 {
-    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::logout::LogoutError, R>) -> Self {
+    fn from(
+        err: ::aws_smithy_runtime_api::client::result::SdkError<
+            crate::operation::logout::LogoutError,
+            R,
+        >,
+    ) -> Self {
         match err {
-            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
+            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => {
+                Self::from(context.into_err())
+            }
             _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                 meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                 source: err.into(),
@@ -149,12 +291,19 @@
         }
     }
 }
+#[cfg(feature = "op_logout")]
 impl From<crate::operation::logout::LogoutError> for Error {
     fn from(err: crate::operation::logout::LogoutError) -> Self {
         match err {
-            crate::operation::logout::LogoutError::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
-            crate::operation::logout::LogoutError::TooManyRequestsException(inner) => Error::TooManyRequestsException(inner),
-            crate::operation::logout::LogoutError::UnauthorizedException(inner) => Error::UnauthorizedException(inner),
+            crate::operation::logout::LogoutError::InvalidRequestException(inner) => {
+                Error::InvalidRequestException(inner)
+            }
+            crate::operation::logout::LogoutError::TooManyRequestsException(inner) => {
+                Error::TooManyRequestsException(inner)
+            }
+            crate::operation::logout::LogoutError::UnauthorizedException(inner) => {
+                Error::UnauthorizedException(inner)
+            }
             crate::operation::logout::LogoutError::Unhandled(inner) => Error::Unhandled(inner),
         }
     }
@@ -162,9 +311,32 @@
 impl ::std::error::Error for Error {
     fn source(&self) -> std::option::Option<&(dyn ::std::error::Error + 'static)> {
         match self {
+            #[cfg(any(
+                feature = "op_get_role_credentials",
+                feature = "op_list_account_roles",
+                feature = "op_list_accounts",
+                feature = "op_logout"
+            ))]
             Error::InvalidRequestException(inner) => inner.source(),
+            #[cfg(any(
+                feature = "op_get_role_credentials",
+                feature = "op_list_account_roles",
+                feature = "op_list_accounts"
+            ))]
             Error::ResourceNotFoundException(inner) => inner.source(),
+            #[cfg(any(
+                feature = "op_get_role_credentials",
+                feature = "op_list_account_roles",
+                feature = "op_list_accounts",
+                feature = "op_logout"
+            ))]
             Error::TooManyRequestsException(inner) => inner.source(),
+            #[cfg(any(
+                feature = "op_get_role_credentials",
+                feature = "op_list_account_roles",
+                feature = "op_list_accounts",
+                feature = "op_logout"
+            ))]
             Error::UnauthorizedException(inner) => inner.source(),
             Error::Unhandled(inner) => ::std::option::Option::Some(&*inner.source),
         }
@@ -173,9 +345,32 @@
 impl ::aws_types::request_id::RequestId for Error {
     fn request_id(&self) -> Option<&str> {
         match self {
+            #[cfg(any(
+                feature = "op_get_role_credentials",
+                feature = "op_list_account_roles",
+                feature = "op_list_accounts",
+                feature = "op_logout"
+            ))]
             Self::InvalidRequestException(e) => e.request_id(),
+            #[cfg(any(
+                feature = "op_get_role_credentials",
+                feature = "op_list_account_roles",
+                feature = "op_list_accounts"
+            ))]
             Self::ResourceNotFoundException(e) => e.request_id(),
+            #[cfg(any(
+                feature = "op_get_role_credentials",
+                feature = "op_list_account_roles",
+                feature = "op_list_accounts",
+                feature = "op_logout"
+            ))]
             Self::TooManyRequestsException(e) => e.request_id(),
+            #[cfg(any(
+                feature = "op_get_role_credentials",
+                feature = "op_list_account_roles",
+                feature = "op_list_accounts",
+                feature = "op_logout"
+            ))]
             Self::UnauthorizedException(e) => e.request_id(),
             Self::Unhandled(e) => e.meta.request_id(),
         }
diff --git a/src/json_errors.rs b/src/json_errors.rs
--- a/src/json_errors.rs
+++ b/src/json_errors.rs
@@ -48,27 +48,45 @@
                         match key.as_escaped_str() {
                             "code" => code = Some(value.to_unescaped()?),
                             "__type" => typ = Some(value.to_unescaped()?),
-                            "message" | "Message" | "errorMessage" => message = Some(value.to_unescaped()?),
+                            "message" | "Message" | "errorMessage" => {
+                                message = Some(value.to_unescaped()?)
+                            }
                             _ => {}
                         }
                     }
                     skip_value(&mut tokens)?;
                 }
-                _ => return Err(DeserializeError::custom("expected object key or end object")),
+                _ => {
+                    return Err(DeserializeError::custom(
+                        "expected object key or end object",
+                    ))
+                }
             }
         }
         if tokens.next().is_some() {
-            return Err(DeserializeError::custom("found more JSON tokens after completing parsing"));
+            return Err(DeserializeError::custom(
+                "found more JSON tokens after completing parsing",
+            ));
         }
     }
-    Ok(ErrorBody { code: code.or(typ), message })
+    Ok(ErrorBody {
+        code: code.or(typ),
+        message,
+    })
 }

-pub fn parse_error_metadata(payload: &[u8], headers: &Headers) -> Result<ErrorMetadataBuilder, DeserializeError> {
+pub fn parse_error_metadata(
+    payload: &[u8],
+    headers: &Headers,
+) -> Result<ErrorMetadataBuilder, DeserializeError> {
     let ErrorBody { code, message } = parse_error_body(payload)?;

     let mut err_builder = ErrorMetadata::builder();
-    if let Some(code) = headers.get("x-amzn-errortype").or(code.as_deref()).map(sanitize_error_code) {
+    if let Some(code) = headers
+        .get("x-amzn-errortype")
+        .or(code.as_deref())
+        .map(sanitize_error_code)
+    {
         err_builder = err_builder.code(code);
     }
     if let Some(message) = message {
@@ -88,7 +106,9 @@
     fn error_metadata() {
         let response = HttpResponse::try_from(
             http_1x::Response::builder()
-                .body(SdkBody::from(r#"{ "__type": "FooError", "message": "Go to foo" }"#))
+                .body(SdkBody::from(
+                    r#"{ "__type": "FooError", "message": "Go to foo" }"#,
+                ))
                 .unwrap(),
         )
         .unwrap();
@@ -96,7 +116,10 @@
             parse_error_metadata(response.body().bytes().unwrap(), response.headers())
                 .unwrap()
                 .build(),
-            ErrorMetadata::builder().code("FooError").message("Go to foo").build()
+            ErrorMetadata::builder()
+                .code("FooError")
+                .message("Go to foo")
+                .build()
         )
     }

@@ -104,7 +127,9 @@
     fn error_type() {
         assert_eq!(
             Some(Cow::Borrowed("FooError")),
-            parse_error_body(br#"{ "__type": "FooError" }"#).unwrap().code
+            parse_error_body(br#"{ "__type": "FooError" }"#)
+                .unwrap()
+                .code
         );
     }

@@ -112,7 +137,9 @@
     fn code_takes_priority() {
         assert_eq!(
             Some(Cow::Borrowed("BarError")),
-            parse_error_body(br#"{ "code": "BarError", "__type": "FooError" }"#).unwrap().code
+            parse_error_body(br#"{ "code": "BarError", "__type": "FooError" }"#)
+                .unwrap()
+                .code
         );
     }

@@ -142,7 +169,9 @@
     #[test]
     fn sanitize_url() {
         assert_eq!(
-            sanitize_error_code("FooError:http://internal.amazon.com/coral/com.amazon.coral.validate/"),
+            sanitize_error_code(
+                "FooError:http://internal.amazon.com/coral/com.amazon.coral.validate/"
+            ),
             "FooError"
         );
     }
@@ -149,7 +178,10 @@

     #[test]
     fn sanitize_namespace() {
-        assert_eq!(sanitize_error_code("aws.protocoltests.restjson#FooError"), "FooError");
+        assert_eq!(
+            sanitize_error_code("aws.protocoltests.restjson#FooError"),
+            "FooError"
+        );
     }

     // services like lambda use an alternate `Message` instead of `message`
diff --git a/src/lens.rs b/src/lens.rs
--- a/src/lens.rs
+++ b/src/lens.rs
@@ -1,4 +1,5 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+#[cfg(feature = "op_list_account_roles")]
 pub(crate) fn reflens_list_account_roles_output_output_next_token(
     input: &crate::operation::list_account_roles::ListAccountRolesOutput,
 ) -> ::std::option::Option<&::std::string::String> {
@@ -9,6 +10,7 @@
     ::std::option::Option::Some(input)
 }

+#[cfg(feature = "op_list_accounts")]
 pub(crate) fn reflens_list_accounts_output_output_next_token(
     input: &crate::operation::list_accounts::ListAccountsOutput,
 ) -> ::std::option::Option<&::std::string::String> {
@@ -19,6 +21,7 @@
     ::std::option::Option::Some(input)
 }

+#[cfg(feature = "op_list_account_roles")]
 pub(crate) fn lens_list_account_roles_output_output_role_list(
     input: crate::operation::list_account_roles::ListAccountRolesOutput,
 ) -> ::std::option::Option<::std::vec::Vec<crate::types::RoleInfo>> {
@@ -26,6 +29,7 @@
     ::std::option::Option::Some(input)
 }

+#[cfg(feature = "op_list_accounts")]
 pub(crate) fn lens_list_accounts_output_output_account_list(
     input: crate::operation::list_accounts::ListAccountsOutput,
 ) -> ::std::option::Option<::std::vec::Vec<crate::types::AccountInfo>> {
diff --git a/src/meta.rs b/src/meta.rs
--- a/src/meta.rs
+++ b/src/meta.rs
@@ -1,5 +1,6 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
-pub(crate) static API_METADATA: ::aws_runtime::user_agent::ApiMetadata = ::aws_runtime::user_agent::ApiMetadata::new("sso", crate::meta::PKG_VERSION);
+pub(crate) static API_METADATA: ::aws_runtime::user_agent::ApiMetadata =
+    ::aws_runtime::user_agent::ApiMetadata::new("sso", crate::meta::PKG_VERSION);

 /// Crate version number.
 pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
diff --git a/src/observability_feature.rs b/src/observability_feature.rs
--- a/src/observability_feature.rs
+++ b/src/observability_feature.rs
@@ -7,7 +7,9 @@
 use aws_smithy_runtime::client::sdk_feature::SmithySdkFeature;
 use aws_smithy_runtime_api::{
     box_error::BoxError,
-    client::interceptors::{context::BeforeSerializationInterceptorContextRef, dyn_dispatch_hint, Intercept},
+    client::interceptors::{
+        context::BeforeSerializationInterceptorContextRef, dyn_dispatch_hint, Intercept,
+    },
 };
 use aws_smithy_types::config_bag::ConfigBag;

@@ -21,7 +23,11 @@
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
@@ -28,7 +34,8 @@

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
@@ -1,14 +1,18 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
 pub use ::aws_types::request_id::RequestId;

+#[cfg(feature = "op_get_role_credentials")]
 /// Types for the `GetRoleCredentials` operation.
 pub mod get_role_credentials;

+#[cfg(feature = "op_list_account_roles")]
 /// Types for the `ListAccountRoles` operation.
 pub mod list_account_roles;

+#[cfg(feature = "op_list_accounts")]
 /// Types for the `ListAccounts` operation.
 pub mod list_accounts;

+#[cfg(feature = "op_logout")]
 /// Types for the `Logout` operation.
 pub mod logout;
diff --git a/src/operation/get_role_credentials.rs b/src/operation/get_role_credentials.rs
--- a/src/operation/get_role_credentials.rs
+++ b/src/operation/get_role_credentials.rs
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
-        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("SSO", "GetRoleCredentials", input, runtime_plugins, stop_point)
-            // Create a parent span for the entire operation. Includes a random, internal-only,
-            // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
-            .instrument(::tracing::debug_span!(
-                "SSO.GetRoleCredentials",
-                "rpc.service" = "SSO",
-                "rpc.method" = "GetRoleCredentials",
-                "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
-                "rpc.system" = "aws-api",
-            ))
-            .await
+        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point(
+            "SSO",
+            "GetRoleCredentials",
+            input,
+            runtime_plugins,
+            stop_point,
+        )
+        // Create a parent span for the entire operation. Includes a random, internal-only,
+        // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
+        .instrument(::tracing::debug_span!(
+            "SSO.GetRoleCredentials",
+            "rpc.service" = "SSO",
+            "rpc.method" = "GetRoleCredentials",
+            "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
+            "rpc.system" = "aws-api",
+        ))
+        .await
     }

     pub(crate) fn operation_runtime_plugins(
@@ -75,11 +85,13 @@
             for plugin in config_override.runtime_plugins.iter().cloned() {
                 runtime_plugins = runtime_plugins.with_operation_plugin(plugin);
             }
-            runtime_plugins = runtime_plugins.with_operation_plugin(crate::config::ConfigOverrideRuntimePlugin::new(
-                config_override,
-                client_config.config.clone(),
-                &client_config.runtime_components,
-            ));
+            runtime_plugins = runtime_plugins.with_operation_plugin(
+                crate::config::ConfigOverrideRuntimePlugin::new(
+                    config_override,
+                    client_config.config.clone(),
+                    &client_config.runtime_components,
+                ),
+            );
         }
         runtime_plugins
     }
@@ -88,22 +100,33 @@
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("GetRoleCredentials");

-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            GetRoleCredentialsRequestSerializer,
-        ));
-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            GetRoleCredentialsResponseDeserializer,
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
+                GetRoleCredentialsRequestSerializer,
+            ),
+        );
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
+                GetRoleCredentialsResponseDeserializer,
+            ),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
-            crate::config::auth::Params::builder()
-                .operation_name("GetRoleCredentials")
-                .build()
-                .expect("required fields set"),
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
+                crate::config::auth::Params::builder()
+                    .operation_name("GetRoleCredentials")
+                    .build()
+                    .expect("required fields set"),
+            ),
+        );

         cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::SensitiveOutput);
-        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("GetRoleCredentials", "SSO"));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
+                "GetRoleCredentials",
+                "SSO",
+            ),
+        );

         ::std::option::Option::Some(cfg.freeze())
     }
@@ -111,7 +134,10 @@
     fn runtime_components(
         &self,
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
-    ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
+    ) -> ::std::borrow::Cow<
+        '_,
+        ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
+    > {
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetRoleCredentials")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
@@ -141,7 +167,9 @@
 struct GetRoleCredentialsTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for GetRoleCredentialsTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept
+    for GetRoleCredentialsTelemetryInputCaptureInterceptor
+{
     fn name(&self) -> &'static str {
         "GetRoleCredentialsTelemetryInputCaptureInterceptor"
     }
@@ -164,7 +192,9 @@
             return ::std::result::Result::Ok(());
         };

-        let ::std::option::Option::Some(input) = context.input().downcast_ref::<GetRoleCredentialsInput>() else {
+        let ::std::option::Option::Some(input) =
+            context.input().downcast_ref::<GetRoleCredentialsInput>()
+        else {
             // A mismatched input is not this interceptor's concern; skip quietly.
             return ::std::result::Result::Ok(());
         };
@@ -187,7 +217,9 @@
 }
 #[derive(Debug)]
 struct GetRoleCredentialsResponseDeserializer;
-impl ::aws_smithy_runtime_api::client::ser_de::DeserializeResponse for GetRoleCredentialsResponseDeserializer {
+impl ::aws_smithy_runtime_api::client::ser_de::DeserializeResponse
+    for GetRoleCredentialsResponseDeserializer
+{
     fn deserialize_nonstreaming_with_config(
         &self,
         response: &::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
@@ -200,9 +232,13 @@
         let mut force_error = false;
         ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));
         let parse_result = if !success && status != 200 || force_error {
-            crate::protocol_serde::shape_get_role_credentials::de_get_role_credentials_http_error(status, headers, body)
+            crate::protocol_serde::shape_get_role_credentials::de_get_role_credentials_http_error(
+                status, headers, body,
+            )
         } else {
-            crate::protocol_serde::shape_get_role_credentials::de_get_role_credentials_http_response(status, headers, body)
+            crate::protocol_serde::shape_get_role_credentials::de_get_role_credentials_http_response(
+                status, headers, body,
+            )
         };
         crate::protocol_serde::type_erase_result(parse_result)
     }
@@ -209,13 +245,23 @@
 }
 #[derive(Debug)]
 struct GetRoleCredentialsRequestSerializer;
-impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for GetRoleCredentialsRequestSerializer {
-    #[allow(unused_mut, clippy::let_and_return, clippy::needless_borrow, clippy::useless_conversion)]
+impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest
+    for GetRoleCredentialsRequestSerializer
+{
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
             .downcast::<crate::operation::get_role_credentials::GetRoleCredentialsInput>()
             .expect("correct type");
@@ -228,36 +274,49 @@
             fn uri_base(
                 _input: &crate::operation::get_role_credentials::GetRoleCredentialsInput,
                 output: &mut ::std::string::String,
-            ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError>
+            {
                 use ::std::fmt::Write as _;
-                ::std::write!(output, "/federation/credentials").expect("formatting should succeed");
+                ::std::write!(output, "/federation/credentials")
+                    .expect("formatting should succeed");
                 ::std::result::Result::Ok(())
             }
             fn uri_query(
                 _input: &crate::operation::get_role_credentials::GetRoleCredentialsInput,
                 mut output: &mut ::std::string::String,
-            ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError>
+            {
                 let mut query = ::aws_smithy_http::query::Writer::new(output);
                 let inner_1 = &_input.role_name;
-                let inner_1 = inner_1
-                    .as_ref()
-                    .ok_or_else(|| ::aws_smithy_types::error::operation::BuildError::missing_field("role_name", "cannot be empty or unset"))?;
-                if inner_1.is_empty() {
-                    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
+                let inner_1 = inner_1.as_ref().ok_or_else(|| {
+                    ::aws_smithy_types::error::operation::BuildError::missing_field(
                         "role_name",
                         "cannot be empty or unset",
-                    ));
+                    )
+                })?;
+                if inner_1.is_empty() {
+                    return ::std::result::Result::Err(
+                        ::aws_smithy_types::error::operation::BuildError::missing_field(
+                            "role_name",
+                            "cannot be empty or unset",
+                        ),
+                    );
                 }
                 query.push_kv("role_name", &::aws_smithy_http::query::fmt_string(inner_1));
                 let inner_2 = &_input.account_id;
-                let inner_2 = inner_2
-                    .as_ref()
-                    .ok_or_else(|| ::aws_smithy_types::error::operation::BuildError::missing_field("account_id", "cannot be empty or unset"))?;
-                if inner_2.is_empty() {
-                    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
+                let inner_2 = inner_2.as_ref().ok_or_else(|| {
+                    ::aws_smithy_types::error::operation::BuildError::missing_field(
                         "account_id",
                         "cannot be empty or unset",
-                    ));
+                    )
+                })?;
+                if inner_2.is_empty() {
+                    return ::std::result::Result::Err(
+                        ::aws_smithy_types::error::operation::BuildError::missing_field(
+                            "account_id",
+                            "cannot be empty or unset",
+                        ),
+                    );
                 }
                 query.push_kv("account_id", &::aws_smithy_http::query::fmt_string(inner_2));
                 ::std::result::Result::Ok(())
@@ -266,7 +325,10 @@
             fn update_http_builder(
                 input: &crate::operation::get_role_credentials::GetRoleCredentialsInput,
                 builder: ::http_1x::request::Builder,
-            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<
+                ::http_1x::request::Builder,
+                ::aws_smithy_types::error::operation::BuildError,
+            > {
                 let mut uri = ::std::string::String::new();
                 uri_base(input, &mut uri)?;
                 uri_query(input, &mut uri)?;
@@ -278,7 +340,13 @@
         };
         let body = ::aws_smithy_types::body::SdkBody::from("");

-        ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
+        ::std::result::Result::Ok(
+            request_builder
+                .body(body)
+                .expect("valid request")
+                .try_into()
+                .unwrap(),
+        )
     }
 }
 #[derive(Debug)]
@@ -285,7 +353,9 @@
 struct GetRoleCredentialsEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for GetRoleCredentialsEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept
+    for GetRoleCredentialsEndpointParamsInterceptor
+{
     fn name(&self) -> &'static str {
         "GetRoleCredentialsEndpointParamsInterceptor"
     }
@@ -306,16 +376,32 @@
             .ok_or("failed to downcast to GetRoleCredentialsInput")?;

         let params = crate::config::endpoint::Params::builder()
-            .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
-            .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
+            .set_region(
+                cfg.load::<::aws_types::region::Region>()
+                    .map(|r| r.as_ref().to_owned()),
+            )
+            .set_use_dual_stack(
+                cfg.load::<::aws_types::endpoint_config::UseDualStack>()
+                    .map(|ty| ty.0),
+            )
+            .set_use_fips(
+                cfg.load::<::aws_types::endpoint_config::UseFips>()
+                    .map(|ty| ty.0),
+            )
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
-        cfg.interceptor_state()
-            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params));
+        cfg.interceptor_state().store_put(
+            ::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params),
+        );
         ::std::result::Result::Ok(())
     }
 }
@@ -336,18 +422,24 @@
     /// <p>Indicates that the request is not authorized. This can happen due to an invalid access token in the request.</p>
     UnauthorizedException(crate::types::error::UnauthorizedException),
     /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
-    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
+    #[deprecated(
+        note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
     variable wildcard pattern and check `.code()`:
      \
     &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
      \
-    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-GetRoleCredentialsError) for what information is available for the error.")]
+    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-GetRoleCredentialsError) for what information is available for the error."
+    )]
     Unhandled(crate::error::sealed_unhandled::Unhandled),
 }
 impl GetRoleCredentialsError {
     /// Creates the `GetRoleCredentialsError::Unhandled` variant from any error type.
     pub fn unhandled(
-        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
+        err: impl ::std::convert::Into<
+            ::std::boxed::Box<
+                dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static,
+            >,
+        >,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
             source: err.into(),
@@ -368,10 +460,18 @@
     ///
     pub fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::InvalidRequestException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::ResourceNotFoundException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::TooManyRequestsException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::UnauthorizedException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::InvalidRequestException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
+            Self::ResourceNotFoundException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
+            Self::TooManyRequestsException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
+            Self::UnauthorizedException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
             Self::Unhandled(e) => &e.meta,
         }
     }
@@ -411,7 +511,9 @@
             Self::TooManyRequestsException(_inner) => _inner.fmt(f),
             Self::UnauthorizedException(_inner) => _inner.fmt(f),
             Self::Unhandled(_inner) => {
-                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
+                if let ::std::option::Option::Some(code) =
+                    ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
+                {
                     write!(f, "unhandled error ({code})")
                 } else {
                     f.write_str("unhandled error")
@@ -431,10 +533,18 @@
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for GetRoleCredentialsError {
     fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::InvalidRequestException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::ResourceNotFoundException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::TooManyRequestsException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::UnauthorizedException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::InvalidRequestException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::ResourceNotFoundException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::TooManyRequestsException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::UnauthorizedException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
             Self::Unhandled(_inner) => &_inner.meta,
         }
     }
@@ -441,7 +551,9 @@
 }
 impl ::aws_smithy_runtime_api::client::result::CreateUnhandledError for GetRoleCredentialsError {
     fn create_unhandled_error(
-        source: ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
+        source: ::std::boxed::Box<
+            dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static,
+        >,
         meta: ::std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
@@ -450,7 +562,9 @@
         })
     }
 }
-impl ::aws_types::request_id::RequestId for crate::operation::get_role_credentials::GetRoleCredentialsError {
+impl ::aws_types::request_id::RequestId
+    for crate::operation::get_role_credentials::GetRoleCredentialsError
+{
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
     }
diff --git a/src/operation/get_role_credentials/_get_role_credentials_input.rs b/src/operation/get_role_credentials/_get_role_credentials_input.rs
--- a/src/operation/get_role_credentials/_get_role_credentials_input.rs
+++ b/src/operation/get_role_credentials/_get_role_credentials_input.rs
@@ -35,7 +35,8 @@
 }
 impl GetRoleCredentialsInput {
     /// Creates a new builder-style object to manufacture [`GetRoleCredentialsInput`](crate::operation::get_role_credentials::GetRoleCredentialsInput).
-    pub fn builder() -> crate::operation::get_role_credentials::builders::GetRoleCredentialsInputBuilder {
+    pub fn builder(
+    ) -> crate::operation::get_role_credentials::builders::GetRoleCredentialsInputBuilder {
         crate::operation::get_role_credentials::builders::GetRoleCredentialsInputBuilder::default()
     }
 }
@@ -97,13 +98,17 @@
     /// Consumes the builder and constructs a [`GetRoleCredentialsInput`](crate::operation::get_role_credentials::GetRoleCredentialsInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<crate::operation::get_role_credentials::GetRoleCredentialsInput, ::aws_smithy_types::error::operation::BuildError>
-    {
-        ::std::result::Result::Ok(crate::operation::get_role_credentials::GetRoleCredentialsInput {
-            role_name: self.role_name,
-            account_id: self.account_id,
-            access_token: self.access_token,
-        })
+    ) -> ::std::result::Result<
+        crate::operation::get_role_credentials::GetRoleCredentialsInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
+        ::std::result::Result::Ok(
+            crate::operation::get_role_credentials::GetRoleCredentialsInput {
+                role_name: self.role_name,
+                account_id: self.account_id,
+                access_token: self.access_token,
+            },
+        )
     }
 }
 impl ::std::fmt::Debug for GetRoleCredentialsInputBuilder {
diff --git a/src/operation/get_role_credentials/_get_role_credentials_output.rs b/src/operation/get_role_credentials/_get_role_credentials_output.rs
--- a/src/operation/get_role_credentials/_get_role_credentials_output.rs
+++ b/src/operation/get_role_credentials/_get_role_credentials_output.rs
@@ -28,7 +28,8 @@
 }
 impl GetRoleCredentialsOutput {
     /// Creates a new builder-style object to manufacture [`GetRoleCredentialsOutput`](crate::operation::get_role_credentials::GetRoleCredentialsOutput).
-    pub fn builder() -> crate::operation::get_role_credentials::builders::GetRoleCredentialsOutputBuilder {
+    pub fn builder(
+    ) -> crate::operation::get_role_credentials::builders::GetRoleCredentialsOutputBuilder {
         crate::operation::get_role_credentials::builders::GetRoleCredentialsOutputBuilder::default()
     }
 }
@@ -47,7 +48,10 @@
         self
     }
     /// <p>The credentials for the role that is assigned to the user.</p>
-    pub fn set_role_credentials(mut self, input: ::std::option::Option<crate::types::RoleCredentials>) -> Self {
+    pub fn set_role_credentials(
+        mut self,
+        input: ::std::option::Option<crate::types::RoleCredentials>,
+    ) -> Self {
         self.role_credentials = input;
         self
     }
diff --git a/src/operation/get_role_credentials/builders.rs b/src/operation/get_role_credentials/builders.rs
--- a/src/operation/get_role_credentials/builders.rs
+++ b/src/operation/get_role_credentials/builders.rs
@@ -57,7 +57,9 @@
         }
     }
     /// Access the GetRoleCredentials as a reference.
-    pub fn as_input(&self) -> &crate::operation::get_role_credentials::builders::GetRoleCredentialsInputBuilder {
+    pub fn as_input(
+        &self,
+    ) -> &crate::operation::get_role_credentials::builders::GetRoleCredentialsInputBuilder {
         &self.inner
     }
     /// Sends the request and returns the response.
@@ -81,12 +83,17 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = crate::operation::get_role_credentials::GetRoleCredentials::operation_runtime_plugins(
-            self.handle.runtime_plugins.clone(),
-            &self.handle.conf,
-            self.config_override,
-        );
-        crate::operation::get_role_credentials::GetRoleCredentials::orchestrate(&runtime_plugins, input).await
+        let runtime_plugins =
+            crate::operation::get_role_credentials::GetRoleCredentials::operation_runtime_plugins(
+                self.handle.runtime_plugins.clone(),
+                &self.handle.conf,
+                self.config_override,
+            );
+        crate::operation::get_role_credentials::GetRoleCredentials::orchestrate(
+            &runtime_plugins,
+            input,
+        )
+        .await
     }

     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
@@ -99,12 +106,18 @@
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
diff --git a/src/operation/list_account_roles.rs b/src/operation/list_account_roles.rs
--- a/src/operation/list_account_roles.rs
+++ b/src/operation/list_account_roles.rs
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
-        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("SSO", "ListAccountRoles", input, runtime_plugins, stop_point)
-            // Create a parent span for the entire operation. Includes a random, internal-only,
-            // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
-            .instrument(::tracing::debug_span!(
-                "SSO.ListAccountRoles",
-                "rpc.service" = "SSO",
-                "rpc.method" = "ListAccountRoles",
-                "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
-                "rpc.system" = "aws-api",
-            ))
-            .await
+        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point(
+            "SSO",
+            "ListAccountRoles",
+            input,
+            runtime_plugins,
+            stop_point,
+        )
+        // Create a parent span for the entire operation. Includes a random, internal-only,
+        // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
+        .instrument(::tracing::debug_span!(
+            "SSO.ListAccountRoles",
+            "rpc.service" = "SSO",
+            "rpc.method" = "ListAccountRoles",
+            "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
+            "rpc.system" = "aws-api",
+        ))
+        .await
     }

     pub(crate) fn operation_runtime_plugins(
@@ -75,11 +85,13 @@
             for plugin in config_override.runtime_plugins.iter().cloned() {
                 runtime_plugins = runtime_plugins.with_operation_plugin(plugin);
             }
-            runtime_plugins = runtime_plugins.with_operation_plugin(crate::config::ConfigOverrideRuntimePlugin::new(
-                config_override,
-                client_config.config.clone(),
-                &client_config.runtime_components,
-            ));
+            runtime_plugins = runtime_plugins.with_operation_plugin(
+                crate::config::ConfigOverrideRuntimePlugin::new(
+                    config_override,
+                    client_config.config.clone(),
+                    &client_config.runtime_components,
+                ),
+            );
         }
         runtime_plugins
     }
@@ -88,21 +100,32 @@
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("ListAccountRoles");

-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            ListAccountRolesRequestSerializer,
-        ));
-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            ListAccountRolesResponseDeserializer,
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
+                ListAccountRolesRequestSerializer,
+            ),
+        );
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
+                ListAccountRolesResponseDeserializer,
+            ),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
-            crate::config::auth::Params::builder()
-                .operation_name("ListAccountRoles")
-                .build()
-                .expect("required fields set"),
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
+                crate::config::auth::Params::builder()
+                    .operation_name("ListAccountRoles")
+                    .build()
+                    .expect("required fields set"),
+            ),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("ListAccountRoles", "SSO"));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
+                "ListAccountRoles",
+                "SSO",
+            ),
+        );

         ::std::option::Option::Some(cfg.freeze())
     }
@@ -110,7 +133,10 @@
     fn runtime_components(
         &self,
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
-    ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
+    ) -> ::std::borrow::Cow<
+        '_,
+        ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
+    > {
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ListAccountRoles")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
@@ -140,7 +166,9 @@
 struct ListAccountRolesTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for ListAccountRolesTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept
+    for ListAccountRolesTelemetryInputCaptureInterceptor
+{
     fn name(&self) -> &'static str {
         "ListAccountRolesTelemetryInputCaptureInterceptor"
     }
@@ -163,7 +191,9 @@
             return ::std::result::Result::Ok(());
         };

-        let ::std::option::Option::Some(input) = context.input().downcast_ref::<ListAccountRolesInput>() else {
+        let ::std::option::Option::Some(input) =
+            context.input().downcast_ref::<ListAccountRolesInput>()
+        else {
             // A mismatched input is not this interceptor's concern; skip quietly.
             return ::std::result::Result::Ok(());
         };
@@ -186,7 +216,9 @@
 }
 #[derive(Debug)]
 struct ListAccountRolesResponseDeserializer;
-impl ::aws_smithy_runtime_api::client::ser_de::DeserializeResponse for ListAccountRolesResponseDeserializer {
+impl ::aws_smithy_runtime_api::client::ser_de::DeserializeResponse
+    for ListAccountRolesResponseDeserializer
+{
     fn deserialize_nonstreaming_with_config(
         &self,
         response: &::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
@@ -199,9 +231,13 @@
         let mut force_error = false;
         ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));
         let parse_result = if !success && status != 200 || force_error {
-            crate::protocol_serde::shape_list_account_roles::de_list_account_roles_http_error(status, headers, body)
+            crate::protocol_serde::shape_list_account_roles::de_list_account_roles_http_error(
+                status, headers, body,
+            )
         } else {
-            crate::protocol_serde::shape_list_account_roles::de_list_account_roles_http_response(status, headers, body)
+            crate::protocol_serde::shape_list_account_roles::de_list_account_roles_http_response(
+                status, headers, body,
+            )
         };
         crate::protocol_serde::type_erase_result(parse_result)
     }
@@ -208,13 +244,23 @@
 }
 #[derive(Debug)]
 struct ListAccountRolesRequestSerializer;
-impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for ListAccountRolesRequestSerializer {
-    #[allow(unused_mut, clippy::let_and_return, clippy::needless_borrow, clippy::useless_conversion)]
+impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest
+    for ListAccountRolesRequestSerializer
+{
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
             .downcast::<crate::operation::list_account_roles::ListAccountRolesInput>()
             .expect("correct type");
@@ -227,7 +273,8 @@
             fn uri_base(
                 _input: &crate::operation::list_account_roles::ListAccountRolesInput,
                 output: &mut ::std::string::String,
-            ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError>
+            {
                 use ::std::fmt::Write as _;
                 ::std::write!(output, "/assignment/roles").expect("formatting should succeed");
                 ::std::result::Result::Ok(())
@@ -235,7 +282,8 @@
             fn uri_query(
                 _input: &crate::operation::list_account_roles::ListAccountRolesInput,
                 mut output: &mut ::std::string::String,
-            ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError>
+            {
                 let mut query = ::aws_smithy_http::query::Writer::new(output);
                 if let ::std::option::Option::Some(inner_1) = &_input.next_token {
                     {
@@ -244,18 +292,26 @@
                 }
                 if let ::std::option::Option::Some(inner_2) = &_input.max_results {
                     {
-                        query.push_kv("max_result", ::aws_smithy_types::primitive::Encoder::from(*inner_2).encode());
+                        query.push_kv(
+                            "max_result",
+                            ::aws_smithy_types::primitive::Encoder::from(*inner_2).encode(),
+                        );
                     }
                 }
                 let inner_3 = &_input.account_id;
-                let inner_3 = inner_3
-                    .as_ref()
-                    .ok_or_else(|| ::aws_smithy_types::error::operation::BuildError::missing_field("account_id", "cannot be empty or unset"))?;
-                if inner_3.is_empty() {
-                    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
+                let inner_3 = inner_3.as_ref().ok_or_else(|| {
+                    ::aws_smithy_types::error::operation::BuildError::missing_field(
                         "account_id",
                         "cannot be empty or unset",
-                    ));
+                    )
+                })?;
+                if inner_3.is_empty() {
+                    return ::std::result::Result::Err(
+                        ::aws_smithy_types::error::operation::BuildError::missing_field(
+                            "account_id",
+                            "cannot be empty or unset",
+                        ),
+                    );
                 }
                 query.push_kv("account_id", &::aws_smithy_http::query::fmt_string(inner_3));
                 ::std::result::Result::Ok(())
@@ -264,7 +320,10 @@
             fn update_http_builder(
                 input: &crate::operation::list_account_roles::ListAccountRolesInput,
                 builder: ::http_1x::request::Builder,
-            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<
+                ::http_1x::request::Builder,
+                ::aws_smithy_types::error::operation::BuildError,
+            > {
                 let mut uri = ::std::string::String::new();
                 uri_base(input, &mut uri)?;
                 uri_query(input, &mut uri)?;
@@ -276,7 +335,13 @@
         };
         let body = ::aws_smithy_types::body::SdkBody::from("");

-        ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
+        ::std::result::Result::Ok(
+            request_builder
+                .body(body)
+                .expect("valid request")
+                .try_into()
+                .unwrap(),
+        )
     }
 }
 #[derive(Debug)]
@@ -283,7 +348,9 @@
 struct ListAccountRolesEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for ListAccountRolesEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept
+    for ListAccountRolesEndpointParamsInterceptor
+{
     fn name(&self) -> &'static str {
         "ListAccountRolesEndpointParamsInterceptor"
     }
@@ -304,16 +371,32 @@
             .ok_or("failed to downcast to ListAccountRolesInput")?;

         let params = crate::config::endpoint::Params::builder()
-            .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
-            .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
+            .set_region(
+                cfg.load::<::aws_types::region::Region>()
+                    .map(|r| r.as_ref().to_owned()),
+            )
+            .set_use_dual_stack(
+                cfg.load::<::aws_types::endpoint_config::UseDualStack>()
+                    .map(|ty| ty.0),
+            )
+            .set_use_fips(
+                cfg.load::<::aws_types::endpoint_config::UseFips>()
+                    .map(|ty| ty.0),
+            )
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
-        cfg.interceptor_state()
-            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params));
+        cfg.interceptor_state().store_put(
+            ::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params),
+        );
         ::std::result::Result::Ok(())
     }
 }
@@ -334,18 +417,24 @@
     /// <p>Indicates that the request is not authorized. This can happen due to an invalid access token in the request.</p>
     UnauthorizedException(crate::types::error::UnauthorizedException),
     /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
-    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
+    #[deprecated(
+        note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
     variable wildcard pattern and check `.code()`:
      \
     &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
      \
-    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-ListAccountRolesError) for what information is available for the error.")]
+    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-ListAccountRolesError) for what information is available for the error."
+    )]
     Unhandled(crate::error::sealed_unhandled::Unhandled),
 }
 impl ListAccountRolesError {
     /// Creates the `ListAccountRolesError::Unhandled` variant from any error type.
     pub fn unhandled(
-        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
+        err: impl ::std::convert::Into<
+            ::std::boxed::Box<
+                dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static,
+            >,
+        >,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
             source: err.into(),
@@ -366,10 +455,18 @@
     ///
     pub fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::InvalidRequestException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::ResourceNotFoundException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::TooManyRequestsException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::UnauthorizedException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::InvalidRequestException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
+            Self::ResourceNotFoundException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
+            Self::TooManyRequestsException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
+            Self::UnauthorizedException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
             Self::Unhandled(e) => &e.meta,
         }
     }
@@ -409,7 +506,9 @@
             Self::TooManyRequestsException(_inner) => _inner.fmt(f),
             Self::UnauthorizedException(_inner) => _inner.fmt(f),
             Self::Unhandled(_inner) => {
-                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
+                if let ::std::option::Option::Some(code) =
+                    ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
+                {
                     write!(f, "unhandled error ({code})")
                 } else {
                     f.write_str("unhandled error")
@@ -429,10 +528,18 @@
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for ListAccountRolesError {
     fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::InvalidRequestException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::ResourceNotFoundException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::TooManyRequestsException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::UnauthorizedException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::InvalidRequestException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::ResourceNotFoundException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::TooManyRequestsException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::UnauthorizedException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
             Self::Unhandled(_inner) => &_inner.meta,
         }
     }
@@ -439,7 +546,9 @@
 }
 impl ::aws_smithy_runtime_api::client::result::CreateUnhandledError for ListAccountRolesError {
     fn create_unhandled_error(
-        source: ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
+        source: ::std::boxed::Box<
+            dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static,
+        >,
         meta: ::std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
@@ -448,7 +557,9 @@
         })
     }
 }
-impl ::aws_types::request_id::RequestId for crate::operation::list_account_roles::ListAccountRolesError {
+impl ::aws_types::request_id::RequestId
+    for crate::operation::list_account_roles::ListAccountRolesError
+{
     fn request_id(&self) -> Option<&str> {
         self.meta().request_id()
     }
diff --git a/src/operation/list_account_roles/_list_account_roles_input.rs b/src/operation/list_account_roles/_list_account_roles_input.rs
--- a/src/operation/list_account_roles/_list_account_roles_input.rs
+++ b/src/operation/list_account_roles/_list_account_roles_input.rs
@@ -42,7 +42,8 @@
 }
 impl ListAccountRolesInput {
     /// Creates a new builder-style object to manufacture [`ListAccountRolesInput`](crate::operation::list_account_roles::ListAccountRolesInput).
-    pub fn builder() -> crate::operation::list_account_roles::builders::ListAccountRolesInputBuilder {
+    pub fn builder() -> crate::operation::list_account_roles::builders::ListAccountRolesInputBuilder
+    {
         crate::operation::list_account_roles::builders::ListAccountRolesInputBuilder::default()
     }
 }
@@ -118,13 +119,18 @@
     /// Consumes the builder and constructs a [`ListAccountRolesInput`](crate::operation::list_account_roles::ListAccountRolesInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<crate::operation::list_account_roles::ListAccountRolesInput, ::aws_smithy_types::error::operation::BuildError> {
-        ::std::result::Result::Ok(crate::operation::list_account_roles::ListAccountRolesInput {
-            next_token: self.next_token,
-            max_results: self.max_results,
-            access_token: self.access_token,
-            account_id: self.account_id,
-        })
+    ) -> ::std::result::Result<
+        crate::operation::list_account_roles::ListAccountRolesInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
+        ::std::result::Result::Ok(
+            crate::operation::list_account_roles::ListAccountRolesInput {
+                next_token: self.next_token,
+                max_results: self.max_results,
+                access_token: self.access_token,
+                account_id: self.account_id,
+            },
+        )
     }
 }
 impl ::std::fmt::Debug for ListAccountRolesInputBuilder {
diff --git a/src/operation/list_account_roles/_list_account_roles_output.rs b/src/operation/list_account_roles/_list_account_roles_output.rs
--- a/src/operation/list_account_roles/_list_account_roles_output.rs
+++ b/src/operation/list_account_roles/_list_account_roles_output.rs
@@ -28,13 +28,16 @@
 }
 impl ListAccountRolesOutput {
     /// Creates a new builder-style object to manufacture [`ListAccountRolesOutput`](crate::operation::list_account_roles::ListAccountRolesOutput).
-    pub fn builder() -> crate::operation::list_account_roles::builders::ListAccountRolesOutputBuilder {
+    pub fn builder() -> crate::operation::list_account_roles::builders::ListAccountRolesOutputBuilder
+    {
         crate::operation::list_account_roles::builders::ListAccountRolesOutputBuilder::default()
     }
 }

 /// A builder for [`ListAccountRolesOutput`](crate::operation::list_account_roles::ListAccountRolesOutput).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(
+    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
+)]
 #[non_exhaustive]
 pub struct ListAccountRolesOutputBuilder {
     pub(crate) next_token: ::std::option::Option<::std::string::String>,
@@ -68,7 +71,10 @@
         self
     }
     /// <p>A paginated response with the list of roles and the next token if more results are available.</p>
-    pub fn set_role_list(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::RoleInfo>>) -> Self {
+    pub fn set_role_list(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<crate::types::RoleInfo>>,
+    ) -> Self {
         self.role_list = input;
         self
     }
diff --git a/src/operation/list_account_roles/builders.rs b/src/operation/list_account_roles/builders.rs
--- a/src/operation/list_account_roles/builders.rs
+++ b/src/operation/list_account_roles/builders.rs
@@ -57,7 +57,9 @@
         }
     }
     /// Access the ListAccountRoles as a reference.
-    pub fn as_input(&self) -> &crate::operation::list_account_roles::builders::ListAccountRolesInputBuilder {
+    pub fn as_input(
+        &self,
+    ) -> &crate::operation::list_account_roles::builders::ListAccountRolesInputBuilder {
         &self.inner
     }
     /// Sends the request and returns the response.
@@ -81,12 +83,14 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = crate::operation::list_account_roles::ListAccountRoles::operation_runtime_plugins(
-            self.handle.runtime_plugins.clone(),
-            &self.handle.conf,
-            self.config_override,
-        );
-        crate::operation::list_account_roles::ListAccountRoles::orchestrate(&runtime_plugins, input).await
+        let runtime_plugins =
+            crate::operation::list_account_roles::ListAccountRoles::operation_runtime_plugins(
+                self.handle.runtime_plugins.clone(),
+                &self.handle.conf,
+                self.config_override,
+            );
+        crate::operation::list_account_roles::ListAccountRoles::orchestrate(&runtime_plugins, input)
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
@@ -111,8 +121,13 @@
     /// Create a paginator for this request
     ///
     /// Paginators are used by calling [`send().await`](crate::operation::list_account_roles::paginator::ListAccountRolesPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
-    pub fn into_paginator(self) -> crate::operation::list_account_roles::paginator::ListAccountRolesPaginator {
-        crate::operation::list_account_roles::paginator::ListAccountRolesPaginator::new(self.handle, self.inner)
+    pub fn into_paginator(
+        self,
+    ) -> crate::operation::list_account_roles::paginator::ListAccountRolesPaginator {
+        crate::operation::list_account_roles::paginator::ListAccountRolesPaginator::new(
+            self.handle,
+            self.inner,
+        )
     }
     /// <p>The page token from the previous response output when you request subsequent pages.</p>
     pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
diff --git a/src/operation/list_account_roles/paginator.rs b/src/operation/list_account_roles/paginator.rs
--- a/src/operation/list_account_roles/paginator.rs
+++ b/src/operation/list_account_roles/paginator.rs
@@ -31,7 +31,9 @@
     ///
     /// This paginator automatically flattens results using `role_list`. Queries to the underlying service
     /// are dispatched lazily.
-    pub fn items(self) -> crate::operation::list_account_roles::paginator::ListAccountRolesPaginatorItems {
+    pub fn items(
+        self,
+    ) -> crate::operation::list_account_roles::paginator::ListAccountRolesPaginatorItems {
         crate::operation::list_account_roles::paginator::ListAccountRolesPaginatorItems(self)
     }

@@ -65,20 +67,22 @@
         // Move individual fields out of self for the borrow checker
         let builder = self.builder;
         let handle = self.handle;
-        let runtime_plugins = crate::operation::list_account_roles::ListAccountRoles::operation_runtime_plugins(
-            handle.runtime_plugins.clone(),
-            &handle.conf,
-            ::std::option::Option::None,
-        )
-        .with_operation_plugin(crate::sdk_feature_tracker::paginator::PaginatorFeatureTrackerRuntimePlugin::new());
-        ::aws_smithy_async::future::pagination_stream::PaginationStream::new(::aws_smithy_async::future::pagination_stream::fn_stream::FnStream::new(
-            move |tx| {
+        let runtime_plugins =
+            crate::operation::list_account_roles::ListAccountRoles::operation_runtime_plugins(
+                handle.runtime_plugins.clone(),
+                &handle.conf,
+                ::std::option::Option::None,
+            )
+            .with_operation_plugin(
+                crate::sdk_feature_tracker::paginator::PaginatorFeatureTrackerRuntimePlugin::new(),
+            );
+        ::aws_smithy_async::future::pagination_stream::PaginationStream::new(
+            ::aws_smithy_async::future::pagination_stream::fn_stream::FnStream::new(move |tx| {
                 ::std::boxed::Box::pin(async move {
                     // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
-                    let mut input = match builder
-                        .build()
-                        .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)
-                    {
+                    let mut input = match builder.build().map_err(
+                        ::aws_smithy_runtime_api::client::result::SdkError::construction_failure,
+                    ) {
                         ::std::result::Result::Ok(input) => input,
                         ::std::result::Result::Err(e) => {
                             let _ = tx.send(::std::result::Result::Err(e)).await;
@@ -86,14 +90,23 @@
                         }
                     };
                     loop {
-                        let resp = crate::operation::list_account_roles::ListAccountRoles::orchestrate(&runtime_plugins, input.clone()).await;
+                        let resp =
+                            crate::operation::list_account_roles::ListAccountRoles::orchestrate(
+                                &runtime_plugins,
+                                input.clone(),
+                            )
+                            .await;
                         // If the input member is None or it was an error
                         let done = match resp {
                             ::std::result::Result::Ok(ref resp) => {
                                 let new_token = crate::lens::reflens_list_account_roles_output_output_next_token(resp);
                                 // Pagination is exhausted when the next token is an empty string
-                                let is_empty = new_token.map(|token| token.is_empty()).unwrap_or(true);
-                                if !is_empty && new_token == input.next_token.as_ref() && self.stop_on_duplicate_token {
+                                let is_empty =
+                                    new_token.map(|token| token.is_empty()).unwrap_or(true);
+                                if !is_empty
+                                    && new_token == input.next_token.as_ref()
+                                    && self.stop_on_duplicate_token
+                                {
                                     true
                                 } else {
                                     input.next_token = new_token.cloned();
@@ -111,8 +124,8 @@
                         }
                     }
                 })
-            },
-        ))
+            }),
+        )
     }
 }

@@ -139,10 +152,12 @@
             >,
         >,
     > {
-        ::aws_smithy_async::future::pagination_stream::TryFlatMap::new(self.0.send()).flat_map(|page| {
-            crate::lens::lens_list_account_roles_output_output_role_list(page)
-                .unwrap_or_default()
-                .into_iter()
-        })
+        ::aws_smithy_async::future::pagination_stream::TryFlatMap::new(self.0.send()).flat_map(
+            |page| {
+                crate::lens::lens_list_account_roles_output_output_role_list(page)
+                    .unwrap_or_default()
+                    .into_iter()
+            },
+        )
     }
 }
diff --git a/src/operation/list_accounts.rs b/src/operation/list_accounts.rs
--- a/src/operation/list_accounts.rs
+++ b/src/operation/list_accounts.rs
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
-        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("SSO", "ListAccounts", input, runtime_plugins, stop_point)
-            // Create a parent span for the entire operation. Includes a random, internal-only,
-            // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
-            .instrument(::tracing::debug_span!(
-                "SSO.ListAccounts",
-                "rpc.service" = "SSO",
-                "rpc.method" = "ListAccounts",
-                "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
-                "rpc.system" = "aws-api",
-            ))
-            .await
+        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point(
+            "SSO",
+            "ListAccounts",
+            input,
+            runtime_plugins,
+            stop_point,
+        )
+        // Create a parent span for the entire operation. Includes a random, internal-only,
+        // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
+        .instrument(::tracing::debug_span!(
+            "SSO.ListAccounts",
+            "rpc.service" = "SSO",
+            "rpc.method" = "ListAccounts",
+            "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
+            "rpc.system" = "aws-api",
+        ))
+        .await
     }

     pub(crate) fn operation_runtime_plugins(
@@ -75,11 +85,13 @@
             for plugin in config_override.runtime_plugins.iter().cloned() {
                 runtime_plugins = runtime_plugins.with_operation_plugin(plugin);
             }
-            runtime_plugins = runtime_plugins.with_operation_plugin(crate::config::ConfigOverrideRuntimePlugin::new(
-                config_override,
-                client_config.config.clone(),
-                &client_config.runtime_components,
-            ));
+            runtime_plugins = runtime_plugins.with_operation_plugin(
+                crate::config::ConfigOverrideRuntimePlugin::new(
+                    config_override,
+                    client_config.config.clone(),
+                    &client_config.runtime_components,
+                ),
+            );
         }
         runtime_plugins
     }
@@ -88,21 +100,29 @@
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("ListAccounts");

-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            ListAccountsRequestSerializer,
-        ));
-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            ListAccountsResponseDeserializer,
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
+                ListAccountsRequestSerializer,
+            ),
+        );
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
+                ListAccountsResponseDeserializer,
+            ),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
-            crate::config::auth::Params::builder()
-                .operation_name("ListAccounts")
-                .build()
-                .expect("required fields set"),
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
+                crate::config::auth::Params::builder()
+                    .operation_name("ListAccounts")
+                    .build()
+                    .expect("required fields set"),
+            ),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("ListAccounts", "SSO"));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::orchestrator::Metadata::new("ListAccounts", "SSO"),
+        );

         ::std::option::Option::Some(cfg.freeze())
     }
@@ -110,7 +130,10 @@
     fn runtime_components(
         &self,
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
-    ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
+    ) -> ::std::borrow::Cow<
+        '_,
+        ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
+    > {
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ListAccounts")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
@@ -140,7 +163,9 @@
 struct ListAccountsTelemetryInputCaptureInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for ListAccountsTelemetryInputCaptureInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept
+    for ListAccountsTelemetryInputCaptureInterceptor
+{
     fn name(&self) -> &'static str {
         "ListAccountsTelemetryInputCaptureInterceptor"
     }
@@ -163,7 +188,9 @@
             return ::std::result::Result::Ok(());
         };

-        let ::std::option::Option::Some(input) = context.input().downcast_ref::<ListAccountsInput>() else {
+        let ::std::option::Option::Some(input) =
+            context.input().downcast_ref::<ListAccountsInput>()
+        else {
             // A mismatched input is not this interceptor's concern; skip quietly.
             return ::std::result::Result::Ok(());
         };
@@ -181,7 +208,9 @@
 }
 #[derive(Debug)]
 struct ListAccountsResponseDeserializer;
-impl ::aws_smithy_runtime_api::client::ser_de::DeserializeResponse for ListAccountsResponseDeserializer {
+impl ::aws_smithy_runtime_api::client::ser_de::DeserializeResponse
+    for ListAccountsResponseDeserializer
+{
     fn deserialize_nonstreaming_with_config(
         &self,
         response: &::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
@@ -194,9 +223,13 @@
         let mut force_error = false;
         ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));
         let parse_result = if !success && status != 200 || force_error {
-            crate::protocol_serde::shape_list_accounts::de_list_accounts_http_error(status, headers, body)
+            crate::protocol_serde::shape_list_accounts::de_list_accounts_http_error(
+                status, headers, body,
+            )
         } else {
-            crate::protocol_serde::shape_list_accounts::de_list_accounts_http_response(status, headers, body)
+            crate::protocol_serde::shape_list_accounts::de_list_accounts_http_response(
+                status, headers, body,
+            )
         };
         crate::protocol_serde::type_erase_result(parse_result)
     }
@@ -204,12 +237,20 @@
 #[derive(Debug)]
 struct ListAccountsRequestSerializer;
 impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for ListAccountsRequestSerializer {
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
             .downcast::<crate::operation::list_accounts::ListAccountsInput>()
             .expect("correct type");
@@ -222,7 +263,8 @@
             fn uri_base(
                 _input: &crate::operation::list_accounts::ListAccountsInput,
                 output: &mut ::std::string::String,
-            ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError>
+            {
                 use ::std::fmt::Write as _;
                 ::std::write!(output, "/assignment/accounts").expect("formatting should succeed");
                 ::std::result::Result::Ok(())
@@ -230,7 +272,8 @@
             fn uri_query(
                 _input: &crate::operation::list_accounts::ListAccountsInput,
                 mut output: &mut ::std::string::String,
-            ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError>
+            {
                 let mut query = ::aws_smithy_http::query::Writer::new(output);
                 if let ::std::option::Option::Some(inner_1) = &_input.next_token {
                     {
@@ -239,7 +282,10 @@
                 }
                 if let ::std::option::Option::Some(inner_2) = &_input.max_results {
                     {
-                        query.push_kv("max_result", ::aws_smithy_types::primitive::Encoder::from(*inner_2).encode());
+                        query.push_kv(
+                            "max_result",
+                            ::aws_smithy_types::primitive::Encoder::from(*inner_2).encode(),
+                        );
                     }
                 }
                 ::std::result::Result::Ok(())
@@ -248,11 +294,17 @@
             fn update_http_builder(
                 input: &crate::operation::list_accounts::ListAccountsInput,
                 builder: ::http_1x::request::Builder,
-            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<
+                ::http_1x::request::Builder,
+                ::aws_smithy_types::error::operation::BuildError,
+            > {
                 let mut uri = ::std::string::String::new();
                 uri_base(input, &mut uri)?;
                 uri_query(input, &mut uri)?;
-                let builder = crate::protocol_serde::shape_list_accounts::ser_list_accounts_headers(input, builder)?;
+                let builder =
+                    crate::protocol_serde::shape_list_accounts::ser_list_accounts_headers(
+                        input, builder,
+                    )?;
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
@@ -260,7 +312,13 @@
         };
         let body = ::aws_smithy_types::body::SdkBody::from("");

-        ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
+        ::std::result::Result::Ok(
+            request_builder
+                .body(body)
+                .expect("valid request")
+                .try_into()
+                .unwrap(),
+        )
     }
 }
 #[derive(Debug)]
@@ -267,7 +325,9 @@
 struct ListAccountsEndpointParamsInterceptor;

 #[::aws_smithy_runtime_api::client::interceptors::dyn_dispatch_hint]
-impl ::aws_smithy_runtime_api::client::interceptors::Intercept for ListAccountsEndpointParamsInterceptor {
+impl ::aws_smithy_runtime_api::client::interceptors::Intercept
+    for ListAccountsEndpointParamsInterceptor
+{
     fn name(&self) -> &'static str {
         "ListAccountsEndpointParamsInterceptor"
     }
@@ -288,16 +348,32 @@
             .ok_or("failed to downcast to ListAccountsInput")?;

         let params = crate::config::endpoint::Params::builder()
-            .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
-            .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
+            .set_region(
+                cfg.load::<::aws_types::region::Region>()
+                    .map(|r| r.as_ref().to_owned()),
+            )
+            .set_use_dual_stack(
+                cfg.load::<::aws_types::endpoint_config::UseDualStack>()
+                    .map(|ty| ty.0),
+            )
+            .set_use_fips(
+                cfg.load::<::aws_types::endpoint_config::UseFips>()
+                    .map(|ty| ty.0),
+            )
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
-        cfg.interceptor_state()
-            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params));
+        cfg.interceptor_state().store_put(
+            ::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params),
+        );
         ::std::result::Result::Ok(())
     }
 }
@@ -318,18 +394,24 @@
     /// <p>Indicates that the request is not authorized. This can happen due to an invalid access token in the request.</p>
     UnauthorizedException(crate::types::error::UnauthorizedException),
     /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
-    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
+    #[deprecated(
+        note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
     variable wildcard pattern and check `.code()`:
      \
     &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
      \
-    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-ListAccountsError) for what information is available for the error.")]
+    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-ListAccountsError) for what information is available for the error."
+    )]
     Unhandled(crate::error::sealed_unhandled::Unhandled),
 }
 impl ListAccountsError {
     /// Creates the `ListAccountsError::Unhandled` variant from any error type.
     pub fn unhandled(
-        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
+        err: impl ::std::convert::Into<
+            ::std::boxed::Box<
+                dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static,
+            >,
+        >,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
             source: err.into(),
@@ -350,10 +432,18 @@
     ///
     pub fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::InvalidRequestException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::ResourceNotFoundException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::TooManyRequestsException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::UnauthorizedException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::InvalidRequestException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
+            Self::ResourceNotFoundException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
+            Self::TooManyRequestsException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
+            Self::UnauthorizedException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
             Self::Unhandled(e) => &e.meta,
         }
     }
@@ -393,7 +483,9 @@
             Self::TooManyRequestsException(_inner) => _inner.fmt(f),
             Self::UnauthorizedException(_inner) => _inner.fmt(f),
             Self::Unhandled(_inner) => {
-                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
+                if let ::std::option::Option::Some(code) =
+                    ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
+                {
                     write!(f, "unhandled error ({code})")
                 } else {
                     f.write_str("unhandled error")
@@ -413,10 +505,18 @@
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for ListAccountsError {
     fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::InvalidRequestException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::ResourceNotFoundException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::TooManyRequestsException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::UnauthorizedException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::InvalidRequestException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::ResourceNotFoundException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::TooManyRequestsException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::UnauthorizedException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
             Self::Unhandled(_inner) => &_inner.meta,
         }
     }
@@ -423,7 +523,9 @@
 }
 impl ::aws_smithy_runtime_api::client::result::CreateUnhandledError for ListAccountsError {
     fn create_unhandled_error(
-        source: ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
+        source: ::std::boxed::Box<
+            dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static,
+        >,
         meta: ::std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
diff --git a/src/operation/list_accounts/_list_accounts_input.rs b/src/operation/list_accounts/_list_accounts_input.rs
--- a/src/operation/list_accounts/_list_accounts_input.rs
+++ b/src/operation/list_accounts/_list_accounts_input.rs
@@ -95,7 +95,10 @@
     /// Consumes the builder and constructs a [`ListAccountsInput`](crate::operation::list_accounts::ListAccountsInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<crate::operation::list_accounts::ListAccountsInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        crate::operation::list_accounts::ListAccountsInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(crate::operation::list_accounts::ListAccountsInput {
             next_token: self.next_token,
             max_results: self.max_results,
diff --git a/src/operation/list_accounts/_list_accounts_output.rs b/src/operation/list_accounts/_list_accounts_output.rs
--- a/src/operation/list_accounts/_list_accounts_output.rs
+++ b/src/operation/list_accounts/_list_accounts_output.rs
@@ -34,7 +34,9 @@
 }

 /// A builder for [`ListAccountsOutput`](crate::operation::list_accounts::ListAccountsOutput).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(
+    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
+)]
 #[non_exhaustive]
 pub struct ListAccountsOutputBuilder {
     pub(crate) next_token: ::std::option::Option<::std::string::String>,
@@ -68,12 +70,17 @@
         self
     }
     /// <p>A paginated response with the list of account information and the next token if more results are available.</p>
-    pub fn set_account_list(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::AccountInfo>>) -> Self {
+    pub fn set_account_list(
+        mut self,
+        input: ::std::option::Option<::std::vec::Vec<crate::types::AccountInfo>>,
+    ) -> Self {
         self.account_list = input;
         self
     }
     /// <p>A paginated response with the list of account information and the next token if more results are available.</p>
-    pub fn get_account_list(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::AccountInfo>> {
+    pub fn get_account_list(
+        &self,
+    ) -> &::std::option::Option<::std::vec::Vec<crate::types::AccountInfo>> {
         &self.account_list
     }
     pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
diff --git a/src/operation/list_accounts/builders.rs b/src/operation/list_accounts/builders.rs
--- a/src/operation/list_accounts/builders.rs
+++ b/src/operation/list_accounts/builders.rs
@@ -81,11 +81,12 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins = crate::operation::list_accounts::ListAccounts::operation_runtime_plugins(
-            self.handle.runtime_plugins.clone(),
-            &self.handle.conf,
-            self.config_override,
-        );
+        let runtime_plugins =
+            crate::operation::list_accounts::ListAccounts::operation_runtime_plugins(
+                self.handle.runtime_plugins.clone(),
+                &self.handle.conf,
+                self.config_override,
+            );
         crate::operation::list_accounts::ListAccounts::orchestrate(&runtime_plugins, input).await
     }

@@ -99,12 +100,18 @@
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
@@ -111,8 +118,13 @@
     /// Create a paginator for this request
     ///
     /// Paginators are used by calling [`send().await`](crate::operation::list_accounts::paginator::ListAccountsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
-    pub fn into_paginator(self) -> crate::operation::list_accounts::paginator::ListAccountsPaginator {
-        crate::operation::list_accounts::paginator::ListAccountsPaginator::new(self.handle, self.inner)
+    pub fn into_paginator(
+        self,
+    ) -> crate::operation::list_accounts::paginator::ListAccountsPaginator {
+        crate::operation::list_accounts::paginator::ListAccountsPaginator::new(
+            self.handle,
+            self.inner,
+        )
     }
     /// <p>(Optional) When requesting subsequent pages, this is the page token from the previous response output.</p>
     pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
diff --git a/src/operation/list_accounts/paginator.rs b/src/operation/list_accounts/paginator.rs
--- a/src/operation/list_accounts/paginator.rs
+++ b/src/operation/list_accounts/paginator.rs
@@ -65,20 +65,22 @@
         // Move individual fields out of self for the borrow checker
         let builder = self.builder;
         let handle = self.handle;
-        let runtime_plugins = crate::operation::list_accounts::ListAccounts::operation_runtime_plugins(
-            handle.runtime_plugins.clone(),
-            &handle.conf,
-            ::std::option::Option::None,
-        )
-        .with_operation_plugin(crate::sdk_feature_tracker::paginator::PaginatorFeatureTrackerRuntimePlugin::new());
-        ::aws_smithy_async::future::pagination_stream::PaginationStream::new(::aws_smithy_async::future::pagination_stream::fn_stream::FnStream::new(
-            move |tx| {
+        let runtime_plugins =
+            crate::operation::list_accounts::ListAccounts::operation_runtime_plugins(
+                handle.runtime_plugins.clone(),
+                &handle.conf,
+                ::std::option::Option::None,
+            )
+            .with_operation_plugin(
+                crate::sdk_feature_tracker::paginator::PaginatorFeatureTrackerRuntimePlugin::new(),
+            );
+        ::aws_smithy_async::future::pagination_stream::PaginationStream::new(
+            ::aws_smithy_async::future::pagination_stream::fn_stream::FnStream::new(move |tx| {
                 ::std::boxed::Box::pin(async move {
                     // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
-                    let mut input = match builder
-                        .build()
-                        .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)
-                    {
+                    let mut input = match builder.build().map_err(
+                        ::aws_smithy_runtime_api::client::result::SdkError::construction_failure,
+                    ) {
                         ::std::result::Result::Ok(input) => input,
                         ::std::result::Result::Err(e) => {
                             let _ = tx.send(::std::result::Result::Err(e)).await;
@@ -86,14 +88,25 @@
                         }
                     };
                     loop {
-                        let resp = crate::operation::list_accounts::ListAccounts::orchestrate(&runtime_plugins, input.clone()).await;
+                        let resp = crate::operation::list_accounts::ListAccounts::orchestrate(
+                            &runtime_plugins,
+                            input.clone(),
+                        )
+                        .await;
                         // If the input member is None or it was an error
                         let done = match resp {
                             ::std::result::Result::Ok(ref resp) => {
-                                let new_token = crate::lens::reflens_list_accounts_output_output_next_token(resp);
+                                let new_token =
+                                    crate::lens::reflens_list_accounts_output_output_next_token(
+                                        resp,
+                                    );
                                 // Pagination is exhausted when the next token is an empty string
-                                let is_empty = new_token.map(|token| token.is_empty()).unwrap_or(true);
-                                if !is_empty && new_token == input.next_token.as_ref() && self.stop_on_duplicate_token {
+                                let is_empty =
+                                    new_token.map(|token| token.is_empty()).unwrap_or(true);
+                                if !is_empty
+                                    && new_token == input.next_token.as_ref()
+                                    && self.stop_on_duplicate_token
+                                {
                                     true
                                 } else {
                                     input.next_token = new_token.cloned();
@@ -111,8 +124,8 @@
                         }
                     }
                 })
-            },
-        ))
+            }),
+        )
     }
 }

@@ -139,10 +152,12 @@
             >,
         >,
     > {
-        ::aws_smithy_async::future::pagination_stream::TryFlatMap::new(self.0.send()).flat_map(|page| {
-            crate::lens::lens_list_accounts_output_output_account_list(page)
-                .unwrap_or_default()
-                .into_iter()
-        })
+        ::aws_smithy_async::future::pagination_stream::TryFlatMap::new(self.0.send()).flat_map(
+            |page| {
+                crate::lens::lens_list_accounts_output_output_account_list(page)
+                    .unwrap_or_default()
+                    .into_iter()
+            },
+        )
     }
 }
diff --git a/src/operation/logout.rs b/src/operation/logout.rs
--- a/src/operation/logout.rs
+++ b/src/operation/logout.rs
@@ -18,16 +18,28 @@
             ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
         >,
     > {
-        let map_err =
-            |err: ::aws_smithy_runtime_api::client::result::SdkError<
-                ::aws_smithy_runtime_api::client::interceptors::context::Error,
-                ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
-            >| { err.map_service_error(|err| err.downcast::<crate::operation::logout::LogoutError>().expect("correct error type")) };
-        let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
-            .await
-            .map_err(map_err)?;
+        let map_err = |err: ::aws_smithy_runtime_api::client::result::SdkError<
+            ::aws_smithy_runtime_api::client::interceptors::context::Error,
+            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
+        >| {
+            err.map_service_error(|err| {
+                err.downcast::<crate::operation::logout::LogoutError>()
+                    .expect("correct error type")
+            })
+        };
+        let context = Self::orchestrate_with_stop_point(
+            runtime_plugins,
+            input,
+            ::aws_smithy_runtime::client::orchestrator::StopPoint::None,
+        )
+        .await
+        .map_err(map_err)?;
         let output = context.finalize().map_err(map_err)?;
-        ::std::result::Result::Ok(output.downcast::<crate::operation::logout::LogoutOutput>().expect("correct output type"))
+        ::std::result::Result::Ok(
+            output
+                .downcast::<crate::operation::logout::LogoutOutput>()
+                .expect("correct output type"),
+        )
     }

     pub(crate) async fn orchestrate_with_stop_point(
@@ -43,17 +55,23 @@
     > {
         let input = ::aws_smithy_runtime_api::client::interceptors::context::Input::erase(input);
         use ::tracing::Instrument;
-        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("SSO", "Logout", input, runtime_plugins, stop_point)
-            // Create a parent span for the entire operation. Includes a random, internal-only,
-            // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
-            .instrument(::tracing::debug_span!(
-                "SSO.Logout",
-                "rpc.service" = "SSO",
-                "rpc.method" = "Logout",
-                "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
-                "rpc.system" = "aws-api",
-            ))
-            .await
+        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point(
+            "SSO",
+            "Logout",
+            input,
+            runtime_plugins,
+            stop_point,
+        )
+        // Create a parent span for the entire operation. Includes a random, internal-only,
+        // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
+        .instrument(::tracing::debug_span!(
+            "SSO.Logout",
+            "rpc.service" = "SSO",
+            "rpc.method" = "Logout",
+            "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
+            "rpc.system" = "aws-api",
+        ))
+        .await
     }

     pub(crate) fn operation_runtime_plugins(
@@ -67,11 +85,13 @@
             for plugin in config_override.runtime_plugins.iter().cloned() {
                 runtime_plugins = runtime_plugins.with_operation_plugin(plugin);
             }
-            runtime_plugins = runtime_plugins.with_operation_plugin(crate::config::ConfigOverrideRuntimePlugin::new(
-                config_override,
-                client_config.config.clone(),
-                &client_config.runtime_components,
-            ));
+            runtime_plugins = runtime_plugins.with_operation_plugin(
+                crate::config::ConfigOverrideRuntimePlugin::new(
+                    config_override,
+                    client_config.config.clone(),
+                    &client_config.runtime_components,
+                ),
+            );
         }
         runtime_plugins
     }
@@ -80,21 +100,29 @@
     fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
         let mut cfg = ::aws_smithy_types::config_bag::Layer::new("Logout");

-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
-            LogoutRequestSerializer,
-        ));
-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            LogoutResponseDeserializer,
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
+                LogoutRequestSerializer,
+            ),
+        );
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
+                LogoutResponseDeserializer,
+            ),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
-            crate::config::auth::Params::builder()
-                .operation_name("Logout")
-                .build()
-                .expect("required fields set"),
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
+                crate::config::auth::Params::builder()
+                    .operation_name("Logout")
+                    .build()
+                    .expect("required fields set"),
+            ),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("Logout", "SSO"));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::orchestrator::Metadata::new("Logout", "SSO"),
+        );

         ::std::option::Option::Some(cfg.freeze())
     }
@@ -102,7 +130,10 @@
     fn runtime_components(
         &self,
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
-    ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
+    ) -> ::std::borrow::Cow<
+        '_,
+        ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
+    > {
         #[allow(unused_mut)]
         let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("Logout")
             .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
@@ -150,13 +181,23 @@
 #[derive(Debug)]
 struct LogoutRequestSerializer;
 impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for LogoutRequestSerializer {
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
-        let input = input.downcast::<crate::operation::logout::LogoutInput>().expect("correct type");
+    ) -> ::std::result::Result<
+        ::aws_smithy_runtime_api::client::orchestrator::HttpRequest,
+        ::aws_smithy_runtime_api::box_error::BoxError,
+    > {
+        let input = input
+            .downcast::<crate::operation::logout::LogoutInput>()
+            .expect("correct type");
         let _header_serialization_settings = _cfg
             .load::<crate::serialization_settings::HeaderSerializationSettings>()
             .cloned()
@@ -166,7 +207,8 @@
             fn uri_base(
                 _input: &crate::operation::logout::LogoutInput,
                 output: &mut ::std::string::String,
-            ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError>
+            {
                 use ::std::fmt::Write as _;
                 ::std::write!(output, "/logout").expect("formatting should succeed");
                 ::std::result::Result::Ok(())
@@ -175,10 +217,14 @@
             fn update_http_builder(
                 input: &crate::operation::logout::LogoutInput,
                 builder: ::http_1x::request::Builder,
-            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<
+                ::http_1x::request::Builder,
+                ::aws_smithy_types::error::operation::BuildError,
+            > {
                 let mut uri = ::std::string::String::new();
                 uri_base(input, &mut uri)?;
-                let builder = crate::protocol_serde::shape_logout::ser_logout_headers(input, builder)?;
+                let builder =
+                    crate::protocol_serde::shape_logout::ser_logout_headers(input, builder)?;
                 ::std::result::Result::Ok(builder.method("POST").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
@@ -186,7 +232,13 @@
         };
         let body = ::aws_smithy_types::body::SdkBody::from("");

-        ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
+        ::std::result::Result::Ok(
+            request_builder
+                .body(body)
+                .expect("valid request")
+                .try_into()
+                .unwrap(),
+        )
     }
 }
 #[derive(Debug)]
@@ -208,19 +260,38 @@
         >,
         cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
     ) -> ::std::result::Result<(), ::aws_smithy_runtime_api::box_error::BoxError> {
-        let _input = context.input().downcast_ref::<LogoutInput>().ok_or("failed to downcast to LogoutInput")?;
+        let _input = context
+            .input()
+            .downcast_ref::<LogoutInput>()
+            .ok_or("failed to downcast to LogoutInput")?;

         let params = crate::config::endpoint::Params::builder()
-            .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
-            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
-            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
-            .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
+            .set_region(
+                cfg.load::<::aws_types::region::Region>()
+                    .map(|r| r.as_ref().to_owned()),
+            )
+            .set_use_dual_stack(
+                cfg.load::<::aws_types::endpoint_config::UseDualStack>()
+                    .map(|ty| ty.0),
+            )
+            .set_use_fips(
+                cfg.load::<::aws_types::endpoint_config::UseFips>()
+                    .map(|ty| ty.0),
+            )
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
-        cfg.interceptor_state()
-            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params));
+        cfg.interceptor_state().store_put(
+            ::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params),
+        );
         ::std::result::Result::Ok(())
     }
 }
@@ -239,18 +310,24 @@
     /// <p>Indicates that the request is not authorized. This can happen due to an invalid access token in the request.</p>
     UnauthorizedException(crate::types::error::UnauthorizedException),
     /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
-    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
+    #[deprecated(
+        note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
     variable wildcard pattern and check `.code()`:
      \
     &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
      \
-    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-LogoutError) for what information is available for the error.")]
+    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-LogoutError) for what information is available for the error."
+    )]
     Unhandled(crate::error::sealed_unhandled::Unhandled),
 }
 impl LogoutError {
     /// Creates the `LogoutError::Unhandled` variant from any error type.
     pub fn unhandled(
-        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
+        err: impl ::std::convert::Into<
+            ::std::boxed::Box<
+                dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static,
+            >,
+        >,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
             source: err.into(),
@@ -271,9 +348,15 @@
     ///
     pub fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::InvalidRequestException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::TooManyRequestsException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
-            Self::UnauthorizedException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
+            Self::InvalidRequestException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
+            Self::TooManyRequestsException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
+            Self::UnauthorizedException(e) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e)
+            }
             Self::Unhandled(e) => &e.meta,
         }
     }
@@ -307,7 +390,9 @@
             Self::TooManyRequestsException(_inner) => _inner.fmt(f),
             Self::UnauthorizedException(_inner) => _inner.fmt(f),
             Self::Unhandled(_inner) => {
-                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
+                if let ::std::option::Option::Some(code) =
+                    ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
+                {
                     write!(f, "unhandled error ({code})")
                 } else {
                     f.write_str("unhandled error")
@@ -327,9 +412,15 @@
 impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for LogoutError {
     fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
         match self {
-            Self::InvalidRequestException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::TooManyRequestsException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
-            Self::UnauthorizedException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
+            Self::InvalidRequestException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::TooManyRequestsException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
+            Self::UnauthorizedException(_inner) => {
+                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
+            }
             Self::Unhandled(_inner) => &_inner.meta,
         }
     }
@@ -336,7 +427,9 @@
 }
 impl ::aws_smithy_runtime_api::client::result::CreateUnhandledError for LogoutError {
     fn create_unhandled_error(
-        source: ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
+        source: ::std::boxed::Box<
+            dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static,
+        >,
         meta: ::std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
diff --git a/src/operation/logout/_logout_input.rs b/src/operation/logout/_logout_input.rs
--- a/src/operation/logout/_logout_input.rs
+++ b/src/operation/logout/_logout_input.rs
@@ -49,7 +49,12 @@
         &self.access_token
     }
     /// Consumes the builder and constructs a [`LogoutInput`](crate::operation::logout::LogoutInput).
-    pub fn build(self) -> ::std::result::Result<crate::operation::logout::LogoutInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<
+        crate::operation::logout::LogoutInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(crate::operation::logout::LogoutInput {
             access_token: self.access_token,
         })
diff --git a/src/operation/logout/_logout_output.rs b/src/operation/logout/_logout_output.rs
--- a/src/operation/logout/_logout_output.rs
+++ b/src/operation/logout/_logout_output.rs
@@ -18,7 +18,9 @@
 }

 /// A builder for [`LogoutOutput`](crate::operation::logout::LogoutOutput).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(
+    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
+)]
 #[non_exhaustive]
 pub struct LogoutOutputBuilder {
     _request_id: Option<String>,
diff --git a/src/operation/logout/builders.rs b/src/operation/logout/builders.rs
--- a/src/operation/logout/builders.rs
+++ b/src/operation/logout/builders.rs
@@ -32,14 +32,20 @@
     inner: crate::operation::logout::builders::LogoutInputBuilder,
     config_override: ::std::option::Option<crate::config::Builder>,
 }
-impl crate::client::customize::internal::CustomizableSend<crate::operation::logout::LogoutOutput, crate::operation::logout::LogoutError>
-    for LogoutFluentBuilder
+impl
+    crate::client::customize::internal::CustomizableSend<
+        crate::operation::logout::LogoutOutput,
+        crate::operation::logout::LogoutError,
+    > for LogoutFluentBuilder
 {
     fn send(
         self,
         config_override: crate::config::Builder,
     ) -> crate::client::customize::internal::BoxFuture<
-        crate::client::customize::internal::SendResult<crate::operation::logout::LogoutOutput, crate::operation::logout::LogoutError>,
+        crate::client::customize::internal::SendResult<
+            crate::operation::logout::LogoutOutput,
+            crate::operation::logout::LogoutError,
+        >,
     > {
         ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
     }
@@ -78,8 +84,11 @@
             .inner
             .build()
             .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
-        let runtime_plugins =
-            crate::operation::logout::Logout::operation_runtime_plugins(self.handle.runtime_plugins.clone(), &self.handle.conf, self.config_override);
+        let runtime_plugins = crate::operation::logout::Logout::operation_runtime_plugins(
+            self.handle.runtime_plugins.clone(),
+            &self.handle.conf,
+            self.config_override,
+        );
         crate::operation::logout::Logout::orchestrate(&runtime_plugins, input).await
     }

@@ -86,15 +95,25 @@
     /// Consumes this builder, creating a customizable operation that can be modified before being sent.
     pub fn customize(
         self,
-    ) -> crate::client::customize::CustomizableOperation<crate::operation::logout::LogoutOutput, crate::operation::logout::LogoutError, Self> {
+    ) -> crate::client::customize::CustomizableOperation<
+        crate::operation::logout::LogoutOutput,
+        crate::operation::logout::LogoutError,
+        Self,
+    > {
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
@@ -1,9 +1,17 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+#[cfg(any(
+    feature = "op_get_role_credentials",
+    feature = "op_list_account_roles",
+    feature = "op_list_accounts",
+    feature = "op_logout"
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
@@ -10,27 +18,50 @@
     E: ::std::error::Error + std::fmt::Debug + ::std::marker::Send + ::std::marker::Sync + 'static,
 {
     result
-        .map(|output| ::aws_smithy_runtime_api::client::interceptors::context::Output::erase(output))
-        .map_err(|error| ::aws_smithy_runtime_api::client::interceptors::context::Error::erase(error))
+        .map(|output| {
+            ::aws_smithy_runtime_api::client::interceptors::context::Output::erase(output)
+        })
+        .map_err(|error| {
+            ::aws_smithy_runtime_api::client::interceptors::context::Error::erase(error)
+        })
         .map_err(::std::convert::Into::into)
 }

+#[cfg(any(
+    feature = "op_get_role_credentials",
+    feature = "op_list_account_roles",
+    feature = "op_list_accounts",
+    feature = "op_logout"
+))]
 pub fn parse_http_error_metadata(
     _response_status: u16,
     response_headers: &::aws_smithy_runtime_api::http::Headers,
     response_body: &[u8],
-) -> ::std::result::Result<::aws_smithy_types::error::metadata::Builder, ::aws_smithy_json::deserialize::error::DeserializeError> {
+) -> ::std::result::Result<
+    ::aws_smithy_types::error::metadata::Builder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
     crate::json_errors::parse_error_metadata(response_body, response_headers)
 }

+#[cfg(feature = "op_get_role_credentials")]
 pub(crate) mod shape_get_role_credentials;

+#[cfg(feature = "op_list_account_roles")]
 pub(crate) mod shape_list_account_roles;

+#[cfg(feature = "op_list_accounts")]
 pub(crate) mod shape_list_accounts;

+#[cfg(feature = "op_logout")]
 pub(crate) mod shape_logout;

+#[cfg(any(
+    feature = "op_get_role_credentials",
+    feature = "op_list_account_roles",
+    feature = "op_list_accounts",
+    feature = "op_logout"
+))]
 pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
     if data.is_empty() {
         b"{}"
@@ -39,20 +70,48 @@
     }
 }

+#[cfg(any(
+    feature = "op_get_role_credentials",
+    feature = "op_list_account_roles",
+    feature = "op_list_accounts",
+    feature = "op_logout"
+))]
 pub(crate) mod shape_invalid_request_exception;

+#[cfg(any(
+    feature = "op_get_role_credentials",
+    feature = "op_list_account_roles",
+    feature = "op_list_accounts"
+))]
 pub(crate) mod shape_resource_not_found_exception;

+#[cfg(any(
+    feature = "op_get_role_credentials",
+    feature = "op_list_account_roles",
+    feature = "op_list_accounts",
+    feature = "op_logout"
+))]
 pub(crate) mod shape_too_many_requests_exception;

+#[cfg(any(
+    feature = "op_get_role_credentials",
+    feature = "op_list_account_roles",
+    feature = "op_list_accounts",
+    feature = "op_logout"
+))]
 pub(crate) mod shape_unauthorized_exception;

+#[cfg(feature = "op_list_accounts")]
 pub(crate) mod shape_account_list_type;

+#[cfg(feature = "op_get_role_credentials")]
 pub(crate) mod shape_role_credentials;

+#[cfg(feature = "op_list_account_roles")]
 pub(crate) mod shape_role_list_type;

+#[cfg(feature = "op_list_accounts")]
 pub(crate) mod shape_account_info;

+#[cfg(feature = "op_list_account_roles")]
 pub(crate) mod shape_role_info;
diff --git a/src/protocol_serde/shape_account_info.rs b/src/protocol_serde/shape_account_info.rs
--- a/src/protocol_serde/shape_account_info.rs
+++ b/src/protocol_serde/shape_account_info.rs
@@ -3,14 +3,24 @@
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
     depth: u32,
-) -> ::std::result::Result<Option<crate::types::AccountInfo>, ::aws_smithy_json::deserialize::error::DeserializeError>
+) -> ::std::result::Result<
+    Option<crate::types::AccountInfo>,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+>
 where
-    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
+    I: Iterator<
+        Item = Result<
+            ::aws_smithy_json::deserialize::Token<'a>,
+            ::aws_smithy_json::deserialize::error::DeserializeError,
+        >,
+    >,
 {
     if depth >= 128u32 {
-        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-            "maximum nesting depth exceeded",
-        ));
+        return Err(
+            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                "maximum nesting depth exceeded",
+            ),
+        );
     }
     match tokens.next().transpose()? {
         Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
@@ -20,41 +30,53 @@
             loop {
                 match tokens.next().transpose()? {
                     Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                        "accountId" => {
-                            builder = builder.set_account_id(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
+                        match key.to_unescaped()?.as_ref() {
+                            "accountId" => {
+                                builder = builder.set_account_id(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
+                                        tokens.next(),
+                                    )?
                                     .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                     .transpose()?,
-                            );
-                        }
-                        "accountName" => {
-                            builder = builder.set_account_name(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                );
+                            }
+                            "accountName" => {
+                                builder = builder.set_account_name(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
+                                        tokens.next(),
+                                    )?
                                     .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                     .transpose()?,
-                            );
-                        }
-                        "emailAddress" => {
-                            builder = builder.set_email_address(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                );
+                            }
+                            "emailAddress" => {
+                                builder = builder.set_email_address(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
+                                        tokens.next(),
+                                    )?
                                     .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                     .transpose()?,
-                            );
+                                );
+                            }
+                            _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                         }
-                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
-                    },
+                    }
                     other => {
-                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
-                            "expected object key or end object, found: {other:?}"
-                        )))
+                        return Err(
+                            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                                format!("expected object key or end object, found: {other:?}"),
+                            ),
+                        )
                     }
                 }
             }
             Ok(Some(builder.build()))
         }
-        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-            "expected start object or null",
-        )),
+        _ => Err(
+            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                "expected start object or null",
+            ),
+        ),
     }
 }
diff --git a/src/protocol_serde/shape_account_list_type.rs b/src/protocol_serde/shape_account_list_type.rs
--- a/src/protocol_serde/shape_account_list_type.rs
+++ b/src/protocol_serde/shape_account_list_type.rs
@@ -3,14 +3,24 @@
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
     depth: u32,
-) -> ::std::result::Result<Option<::std::vec::Vec<crate::types::AccountInfo>>, ::aws_smithy_json::deserialize::error::DeserializeError>
+) -> ::std::result::Result<
+    Option<::std::vec::Vec<crate::types::AccountInfo>>,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+>
 where
-    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
+    I: Iterator<
+        Item = Result<
+            ::aws_smithy_json::deserialize::Token<'a>,
+            ::aws_smithy_json::deserialize::error::DeserializeError,
+        >,
+    >,
 {
     if depth >= 128u32 {
-        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-            "maximum nesting depth exceeded",
-        ));
+        return Err(
+            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                "maximum nesting depth exceeded",
+            ),
+        );
     }
     match tokens.next().transpose()? {
         Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
@@ -23,13 +33,19 @@
                         break;
                     }
                     _ => {
-                        let value = crate::protocol_serde::shape_account_info::de_account_info(tokens, _value, depth + 1)?;
+                        let value = crate::protocol_serde::shape_account_info::de_account_info(
+                            tokens,
+                            _value,
+                            depth + 1,
+                        )?;
                         if let Some(value) = value {
                             items.push(value);
                         } else {
-                            return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-                                "dense list cannot contain null values",
-                            ));
+                            return Err(
+                                ::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                                    "dense list cannot contain null values",
+                                ),
+                            );
                         }
                     }
                 }
@@ -36,8 +52,10 @@
             }
             Ok(Some(items))
         }
-        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-            "expected start array or null",
-        )),
+        _ => Err(
+            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                "expected start array or null",
+            ),
+        ),
     }
 }
diff --git a/src/protocol_serde/shape_get_role_credentials.rs b/src/protocol_serde/shape_get_role_credentials.rs
--- a/src/protocol_serde/shape_get_role_credentials.rs
+++ b/src/protocol_serde/shape_get_role_credentials.rs
@@ -9,13 +9,21 @@
     crate::operation::get_role_credentials::GetRoleCredentialsError,
 > {
     #[allow(unused_mut)]
-    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(crate::operation::get_role_credentials::GetRoleCredentialsError::unhandled)?;
+    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(
+        _response_status,
+        _response_headers,
+        _response_body,
+    )
+    .map_err(crate::operation::get_role_credentials::GetRoleCredentialsError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(crate::operation::get_role_credentials::GetRoleCredentialsError::unhandled(generic)),
+        None => {
+            return Err(
+                crate::operation::get_role_credentials::GetRoleCredentialsError::unhandled(generic),
+            )
+        }
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
@@ -96,9 +104,14 @@
     Ok({
         #[allow(unused_mut)]
         let mut output = crate::operation::get_role_credentials::builders::GetRoleCredentialsOutputBuilder::default();
-        output = crate::protocol_serde::shape_get_role_credentials::de_get_role_credentials(_response_body, output)
-            .map_err(crate::operation::get_role_credentials::GetRoleCredentialsError::unhandled)?;
-        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
+        output = crate::protocol_serde::shape_get_role_credentials::de_get_role_credentials(
+            _response_body,
+            output,
+        )
+        .map_err(crate::operation::get_role_credentials::GetRoleCredentialsError::unhandled)?;
+        output._set_request_id(
+            ::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string),
+        );
         output.build()
     })
 }
@@ -106,7 +119,10 @@
 pub fn ser_get_role_credentials_headers(
     input: &crate::operation::get_role_credentials::GetRoleCredentialsInput,
     mut builder: ::http_1x::request::Builder,
-) -> std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+) -> std::result::Result<
+    ::http_1x::request::Builder,
+    ::aws_smithy_types::error::operation::BuildError,
+> {
     if let ::std::option::Option::Some(inner_1) = &input.access_token {
         let formatted_2 = inner_1.as_str();
         let header_value = formatted_2;
@@ -113,7 +129,10 @@
         let header_value: ::http_1x::HeaderValue = header_value.parse().map_err(|err| {
             ::aws_smithy_types::error::operation::BuildError::invalid_field(
                 "access_token",
-                format!("`{}` cannot be used as a header value: {}", &"*** Sensitive Data Redacted ***", err),
+                format!(
+                    "`{}` cannot be used as a header value: {}",
+                    &"*** Sensitive Data Redacted ***", err
+                ),
             )
         })?;
         builder = builder.header("x-amz-sso_bearer_token", header_value);
@@ -128,7 +147,10 @@
     crate::operation::get_role_credentials::builders::GetRoleCredentialsOutputBuilder,
     ::aws_smithy_json::deserialize::error::DeserializeError,
 > {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(
+        crate::protocol_serde::or_empty_doc(_value),
+    )
+    .peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -136,27 +158,35 @@
     loop {
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "roleCredentials" => {
-                    builder = builder.set_role_credentials(crate::protocol_serde::shape_role_credentials::de_role_credentials(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
+                match key.to_unescaped()?.as_ref() {
+                    "roleCredentials" => {
+                        builder = builder.set_role_credentials(
+                            crate::protocol_serde::shape_role_credentials::de_role_credentials(
+                                tokens,
+                                _value,
+                                depth + 1,
+                            )?,
+                        );
+                    }
+                    _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                 }
-                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
-            },
+            }
             other => {
-                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
-                    "expected object key or end object, found: {other:?}"
-                )))
+                return Err(
+                    ::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                        "expected object key or end object, found: {other:?}"
+                    )),
+                )
             }
         }
     }
     if tokens.next().is_some() {
-        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-            "found more JSON tokens after completing parsing",
-        ));
+        return Err(
+            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                "found more JSON tokens after completing parsing",
+            ),
+        );
     }
     Ok(builder)
 }
diff --git a/src/protocol_serde/shape_invalid_request_exception.rs b/src/protocol_serde/shape_invalid_request_exception.rs
--- a/src/protocol_serde/shape_invalid_request_exception.rs
+++ b/src/protocol_serde/shape_invalid_request_exception.rs
@@ -2,8 +2,14 @@
 pub(crate) fn de_invalid_request_exception_json_err(
     _value: &[u8],
     mut builder: crate::types::error::builders::InvalidRequestExceptionBuilder,
-) -> ::std::result::Result<crate::types::error::builders::InvalidRequestExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+) -> ::std::result::Result<
+    crate::types::error::builders::InvalidRequestExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(
+        crate::protocol_serde::or_empty_doc(_value),
+    )
+    .peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -11,27 +17,35 @@
     loop {
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "message" => {
-                    builder = builder.set_message(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
+                match key.to_unescaped()?.as_ref() {
+                    "message" => {
+                        builder = builder.set_message(
+                            ::aws_smithy_json::deserialize::token::expect_string_or_null(
+                                tokens.next(),
+                            )?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
-                    );
+                        );
+                    }
+                    _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                 }
-                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
-            },
+            }
             other => {
-                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
-                    "expected object key or end object, found: {other:?}"
-                )))
+                return Err(
+                    ::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                        "expected object key or end object, found: {other:?}"
+                    )),
+                )
             }
         }
     }
     if tokens.next().is_some() {
-        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-            "found more JSON tokens after completing parsing",
-        ));
+        return Err(
+            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                "found more JSON tokens after completing parsing",
+            ),
+        );
     }
     Ok(builder)
 }
diff --git a/src/protocol_serde/shape_list_account_roles.rs b/src/protocol_serde/shape_list_account_roles.rs
--- a/src/protocol_serde/shape_list_account_roles.rs
+++ b/src/protocol_serde/shape_list_account_roles.rs
@@ -4,79 +4,103 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<crate::operation::list_account_roles::ListAccountRolesOutput, crate::operation::list_account_roles::ListAccountRolesError> {
+) -> std::result::Result<
+    crate::operation::list_account_roles::ListAccountRolesOutput,
+    crate::operation::list_account_roles::ListAccountRolesError,
+> {
     #[allow(unused_mut)]
-    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(crate::operation::list_account_roles::ListAccountRolesError::unhandled)?;
+    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(
+        _response_status,
+        _response_headers,
+        _response_body,
+    )
+    .map_err(crate::operation::list_account_roles::ListAccountRolesError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(crate::operation::list_account_roles::ListAccountRolesError::unhandled(generic)),
+        None => {
+            return Err(
+                crate::operation::list_account_roles::ListAccountRolesError::unhandled(generic),
+            )
+        }
     };

     let _error_message = generic.message().map(|msg| msg.to_owned());
     Err(match error_code {
-        "InvalidRequestException" => crate::operation::list_account_roles::ListAccountRolesError::InvalidRequestException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "InvalidRequestException" => {
+            crate::operation::list_account_roles::ListAccountRolesError::InvalidRequestException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::InvalidRequestExceptionBuilder::default();
-                output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output =
+                        crate::types::error::builders::InvalidRequestExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(_response_body, output)
                     .map_err(crate::operation::list_account_roles::ListAccountRolesError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "ResourceNotFoundException" => crate::operation::list_account_roles::ListAccountRolesError::ResourceNotFoundException({
-            #[allow(unused_mut)]
-            let mut tmp = {
-                #[allow(unused_mut)]
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
+        "ResourceNotFoundException" => {
+            crate::operation::list_account_roles::ListAccountRolesError::ResourceNotFoundException(
+                {
+                    #[allow(unused_mut)]
+                    let mut tmp = {
+                        #[allow(unused_mut)]
                 let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
-                output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
+                        output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                     .map_err(crate::operation::list_account_roles::ListAccountRolesError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "TooManyRequestsException" => crate::operation::list_account_roles::ListAccountRolesError::TooManyRequestsException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+                        let output = output.meta(generic);
+                        output.build()
+                    };
+                    if tmp.message.is_none() {
+                        tmp.message = _error_message;
+                    }
+                    tmp
+                },
+            )
+        }
+        "TooManyRequestsException" => {
+            crate::operation::list_account_roles::ListAccountRolesError::TooManyRequestsException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
-                output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output =
+                        crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(crate::operation::list_account_roles::ListAccountRolesError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "UnauthorizedException" => crate::operation::list_account_roles::ListAccountRolesError::UnauthorizedException({
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
+        "UnauthorizedException" => {
+            crate::operation::list_account_roles::ListAccountRolesError::UnauthorizedException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::UnauthorizedExceptionBuilder::default();
-                output = crate::protocol_serde::shape_unauthorized_exception::de_unauthorized_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output =
+                        crate::types::error::builders::UnauthorizedExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_unauthorized_exception::de_unauthorized_exception_json_err(_response_body, output)
                     .map_err(crate::operation::list_account_roles::ListAccountRolesError::unhandled)?;
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
         _ => crate::operation::list_account_roles::ListAccountRolesError::generic(generic),
     })
 }
@@ -86,13 +110,23 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<crate::operation::list_account_roles::ListAccountRolesOutput, crate::operation::list_account_roles::ListAccountRolesError> {
+) -> std::result::Result<
+    crate::operation::list_account_roles::ListAccountRolesOutput,
+    crate::operation::list_account_roles::ListAccountRolesError,
+> {
     Ok({
         #[allow(unused_mut)]
-        let mut output = crate::operation::list_account_roles::builders::ListAccountRolesOutputBuilder::default();
-        output = crate::protocol_serde::shape_list_account_roles::de_list_account_roles(_response_body, output)
-            .map_err(crate::operation::list_account_roles::ListAccountRolesError::unhandled)?;
-        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
+        let mut output =
+            crate::operation::list_account_roles::builders::ListAccountRolesOutputBuilder::default(
+            );
+        output = crate::protocol_serde::shape_list_account_roles::de_list_account_roles(
+            _response_body,
+            output,
+        )
+        .map_err(crate::operation::list_account_roles::ListAccountRolesError::unhandled)?;
+        output._set_request_id(
+            ::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string),
+        );
         output.build()
     })
 }
@@ -100,7 +134,10 @@
 pub fn ser_list_account_roles_headers(
     input: &crate::operation::list_account_roles::ListAccountRolesInput,
     mut builder: ::http_1x::request::Builder,
-) -> std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+) -> std::result::Result<
+    ::http_1x::request::Builder,
+    ::aws_smithy_types::error::operation::BuildError,
+> {
     if let ::std::option::Option::Some(inner_1) = &input.access_token {
         let formatted_2 = inner_1.as_str();
         let header_value = formatted_2;
@@ -107,7 +144,10 @@
         let header_value: ::http_1x::HeaderValue = header_value.parse().map_err(|err| {
             ::aws_smithy_types::error::operation::BuildError::invalid_field(
                 "access_token",
-                format!("`{}` cannot be used as a header value: {}", &"*** Sensitive Data Redacted ***", err),
+                format!(
+                    "`{}` cannot be used as a header value: {}",
+                    &"*** Sensitive Data Redacted ***", err
+                ),
             )
         })?;
         builder = builder.header("x-amz-sso_bearer_token", header_value);
@@ -122,7 +162,10 @@
     crate::operation::list_account_roles::builders::ListAccountRolesOutputBuilder,
     ::aws_smithy_json::deserialize::error::DeserializeError,
 > {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(
+        crate::protocol_serde::or_empty_doc(_value),
+    )
+    .peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -130,30 +173,44 @@
     loop {
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "nextToken" => {
-                    builder = builder.set_next_token(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
+                match key.to_unescaped()?.as_ref() {
+                    "nextToken" => {
+                        builder = builder.set_next_token(
+                            ::aws_smithy_json::deserialize::token::expect_string_or_null(
+                                tokens.next(),
+                            )?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
-                    );
+                        );
+                    }
+                    "roleList" => {
+                        builder = builder.set_role_list(
+                            crate::protocol_serde::shape_role_list_type::de_role_list_type(
+                                tokens,
+                                _value,
+                                depth + 1,
+                            )?,
+                        );
+                    }
+                    _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                 }
-                "roleList" => {
-                    builder = builder.set_role_list(crate::protocol_serde::shape_role_list_type::de_role_list_type(tokens, _value, depth + 1)?);
-                }
-                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
-            },
+            }
             other => {
-                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
-                    "expected object key or end object, found: {other:?}"
-                )))
+                return Err(
+                    ::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                        "expected object key or end object, found: {other:?}"
+                    )),
+                )
             }
         }
     }
     if tokens.next().is_some() {
-        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-            "found more JSON tokens after completing parsing",
-        ));
+        return Err(
+            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                "found more JSON tokens after completing parsing",
+            ),
+        );
     }
     Ok(builder)
 }
diff --git a/src/protocol_serde/shape_list_accounts.rs b/src/protocol_serde/shape_list_accounts.rs
--- a/src/protocol_serde/shape_list_accounts.rs
+++ b/src/protocol_serde/shape_list_accounts.rs
@@ -4,10 +4,17 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<crate::operation::list_accounts::ListAccountsOutput, crate::operation::list_accounts::ListAccountsError> {
+) -> std::result::Result<
+    crate::operation::list_accounts::ListAccountsOutput,
+    crate::operation::list_accounts::ListAccountsError,
+> {
     #[allow(unused_mut)]
-    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(crate::operation::list_accounts::ListAccountsError::unhandled)?;
+    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(
+        _response_status,
+        _response_headers,
+        _response_body,
+    )
+    .map_err(crate::operation::list_accounts::ListAccountsError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
@@ -17,66 +24,78 @@

     let _error_message = generic.message().map(|msg| msg.to_owned());
     Err(match error_code {
-        "InvalidRequestException" => crate::operation::list_accounts::ListAccountsError::InvalidRequestException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "InvalidRequestException" => {
+            crate::operation::list_accounts::ListAccountsError::InvalidRequestException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::InvalidRequestExceptionBuilder::default();
-                output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output =
+                        crate::types::error::builders::InvalidRequestExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(_response_body, output)
                     .map_err(crate::operation::list_accounts::ListAccountsError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "ResourceNotFoundException" => crate::operation::list_accounts::ListAccountsError::ResourceNotFoundException({
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
+        "ResourceNotFoundException" => {
+            crate::operation::list_accounts::ListAccountsError::ResourceNotFoundException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
-                output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output =
+                        crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                     .map_err(crate::operation::list_accounts::ListAccountsError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "TooManyRequestsException" => crate::operation::list_accounts::ListAccountsError::TooManyRequestsException({
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
+        "TooManyRequestsException" => {
+            crate::operation::list_accounts::ListAccountsError::TooManyRequestsException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
-                output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output =
+                        crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(crate::operation::list_accounts::ListAccountsError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "UnauthorizedException" => crate::operation::list_accounts::ListAccountsError::UnauthorizedException({
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
+        "UnauthorizedException" => {
+            crate::operation::list_accounts::ListAccountsError::UnauthorizedException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::UnauthorizedExceptionBuilder::default();
-                output = crate::protocol_serde::shape_unauthorized_exception::de_unauthorized_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output =
+                        crate::types::error::builders::UnauthorizedExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_unauthorized_exception::de_unauthorized_exception_json_err(_response_body, output)
                     .map_err(crate::operation::list_accounts::ListAccountsError::unhandled)?;
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
         _ => crate::operation::list_accounts::ListAccountsError::generic(generic),
     })
 }
@@ -86,13 +105,20 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<crate::operation::list_accounts::ListAccountsOutput, crate::operation::list_accounts::ListAccountsError> {
+) -> std::result::Result<
+    crate::operation::list_accounts::ListAccountsOutput,
+    crate::operation::list_accounts::ListAccountsError,
+> {
     Ok({
         #[allow(unused_mut)]
-        let mut output = crate::operation::list_accounts::builders::ListAccountsOutputBuilder::default();
-        output = crate::protocol_serde::shape_list_accounts::de_list_accounts(_response_body, output)
-            .map_err(crate::operation::list_accounts::ListAccountsError::unhandled)?;
-        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
+        let mut output =
+            crate::operation::list_accounts::builders::ListAccountsOutputBuilder::default();
+        output =
+            crate::protocol_serde::shape_list_accounts::de_list_accounts(_response_body, output)
+                .map_err(crate::operation::list_accounts::ListAccountsError::unhandled)?;
+        output._set_request_id(
+            ::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string),
+        );
         output.build()
     })
 }
@@ -100,7 +126,10 @@
 pub fn ser_list_accounts_headers(
     input: &crate::operation::list_accounts::ListAccountsInput,
     mut builder: ::http_1x::request::Builder,
-) -> std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+) -> std::result::Result<
+    ::http_1x::request::Builder,
+    ::aws_smithy_types::error::operation::BuildError,
+> {
     if let ::std::option::Option::Some(inner_1) = &input.access_token {
         let formatted_2 = inner_1.as_str();
         let header_value = formatted_2;
@@ -107,7 +136,10 @@
         let header_value: ::http_1x::HeaderValue = header_value.parse().map_err(|err| {
             ::aws_smithy_types::error::operation::BuildError::invalid_field(
                 "access_token",
-                format!("`{}` cannot be used as a header value: {}", &"*** Sensitive Data Redacted ***", err),
+                format!(
+                    "`{}` cannot be used as a header value: {}",
+                    &"*** Sensitive Data Redacted ***", err
+                ),
             )
         })?;
         builder = builder.header("x-amz-sso_bearer_token", header_value);
@@ -122,7 +154,10 @@
     crate::operation::list_accounts::builders::ListAccountsOutputBuilder,
     ::aws_smithy_json::deserialize::error::DeserializeError,
 > {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(
+        crate::protocol_serde::or_empty_doc(_value),
+    )
+    .peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -130,34 +165,44 @@
     loop {
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "accountList" => {
-                    builder = builder.set_account_list(crate::protocol_serde::shape_account_list_type::de_account_list_type(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
-                }
-                "nextToken" => {
-                    builder = builder.set_next_token(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
+                match key.to_unescaped()?.as_ref() {
+                    "accountList" => {
+                        builder = builder.set_account_list(
+                            crate::protocol_serde::shape_account_list_type::de_account_list_type(
+                                tokens,
+                                _value,
+                                depth + 1,
+                            )?,
+                        );
+                    }
+                    "nextToken" => {
+                        builder = builder.set_next_token(
+                            ::aws_smithy_json::deserialize::token::expect_string_or_null(
+                                tokens.next(),
+                            )?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
-                    );
+                        );
+                    }
+                    _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                 }
-                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
-            },
+            }
             other => {
-                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
-                    "expected object key or end object, found: {other:?}"
-                )))
+                return Err(
+                    ::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                        "expected object key or end object, found: {other:?}"
+                    )),
+                )
             }
         }
     }
     if tokens.next().is_some() {
-        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-            "found more JSON tokens after completing parsing",
-        ));
+        return Err(
+            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                "found more JSON tokens after completing parsing",
+            ),
+        );
     }
     Ok(builder)
 }
diff --git a/src/protocol_serde/shape_logout.rs b/src/protocol_serde/shape_logout.rs
--- a/src/protocol_serde/shape_logout.rs
+++ b/src/protocol_serde/shape_logout.rs
@@ -4,10 +4,17 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<crate::operation::logout::LogoutOutput, crate::operation::logout::LogoutError> {
+) -> std::result::Result<
+    crate::operation::logout::LogoutOutput,
+    crate::operation::logout::LogoutError,
+> {
     #[allow(unused_mut)]
-    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(crate::operation::logout::LogoutError::unhandled)?;
+    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(
+        _response_status,
+        _response_headers,
+        _response_body,
+    )
+    .map_err(crate::operation::logout::LogoutError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
@@ -17,41 +24,48 @@

     let _error_message = generic.message().map(|msg| msg.to_owned());
     Err(match error_code {
-        "InvalidRequestException" => crate::operation::logout::LogoutError::InvalidRequestException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "InvalidRequestException" => {
+            crate::operation::logout::LogoutError::InvalidRequestException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::InvalidRequestExceptionBuilder::default();
-                output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output =
+                        crate::types::error::builders::InvalidRequestExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(_response_body, output)
                     .map_err(crate::operation::logout::LogoutError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "TooManyRequestsException" => crate::operation::logout::LogoutError::TooManyRequestsException({
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
+        "TooManyRequestsException" => {
+            crate::operation::logout::LogoutError::TooManyRequestsException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
-                output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output =
+                        crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(crate::operation::logout::LogoutError::unhandled)?;
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
         "UnauthorizedException" => crate::operation::logout::LogoutError::UnauthorizedException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::UnauthorizedExceptionBuilder::default();
+                let mut output =
+                    crate::types::error::builders::UnauthorizedExceptionBuilder::default();
                 output = crate::protocol_serde::shape_unauthorized_exception::de_unauthorized_exception_json_err(_response_body, output)
                     .map_err(crate::operation::logout::LogoutError::unhandled)?;
                 let output = output.meta(generic);
@@ -71,11 +85,16 @@
     _response_status: u16,
     _response_headers: &::aws_smithy_runtime_api::http::Headers,
     _response_body: &[u8],
-) -> std::result::Result<crate::operation::logout::LogoutOutput, crate::operation::logout::LogoutError> {
+) -> std::result::Result<
+    crate::operation::logout::LogoutOutput,
+    crate::operation::logout::LogoutError,
+> {
     Ok({
         #[allow(unused_mut)]
         let mut output = crate::operation::logout::builders::LogoutOutputBuilder::default();
-        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
+        output._set_request_id(
+            ::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string),
+        );
         output.build()
     })
 }
@@ -83,7 +102,10 @@
 pub fn ser_logout_headers(
     input: &crate::operation::logout::LogoutInput,
     mut builder: ::http_1x::request::Builder,
-) -> std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+) -> std::result::Result<
+    ::http_1x::request::Builder,
+    ::aws_smithy_types::error::operation::BuildError,
+> {
     if let ::std::option::Option::Some(inner_1) = &input.access_token {
         let formatted_2 = inner_1.as_str();
         let header_value = formatted_2;
@@ -90,7 +112,10 @@
         let header_value: ::http_1x::HeaderValue = header_value.parse().map_err(|err| {
             ::aws_smithy_types::error::operation::BuildError::invalid_field(
                 "access_token",
-                format!("`{}` cannot be used as a header value: {}", &"*** Sensitive Data Redacted ***", err),
+                format!(
+                    "`{}` cannot be used as a header value: {}",
+                    &"*** Sensitive Data Redacted ***", err
+                ),
             )
         })?;
         builder = builder.header("x-amz-sso_bearer_token", header_value);
diff --git a/src/protocol_serde/shape_resource_not_found_exception.rs b/src/protocol_serde/shape_resource_not_found_exception.rs
--- a/src/protocol_serde/shape_resource_not_found_exception.rs
+++ b/src/protocol_serde/shape_resource_not_found_exception.rs
@@ -2,8 +2,14 @@
 pub(crate) fn de_resource_not_found_exception_json_err(
     _value: &[u8],
     mut builder: crate::types::error::builders::ResourceNotFoundExceptionBuilder,
-) -> ::std::result::Result<crate::types::error::builders::ResourceNotFoundExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+) -> ::std::result::Result<
+    crate::types::error::builders::ResourceNotFoundExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(
+        crate::protocol_serde::or_empty_doc(_value),
+    )
+    .peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -11,27 +17,35 @@
     loop {
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "message" => {
-                    builder = builder.set_message(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
+                match key.to_unescaped()?.as_ref() {
+                    "message" => {
+                        builder = builder.set_message(
+                            ::aws_smithy_json::deserialize::token::expect_string_or_null(
+                                tokens.next(),
+                            )?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
-                    );
+                        );
+                    }
+                    _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                 }
-                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
-            },
+            }
             other => {
-                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
-                    "expected object key or end object, found: {other:?}"
-                )))
+                return Err(
+                    ::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                        "expected object key or end object, found: {other:?}"
+                    )),
+                )
             }
         }
     }
     if tokens.next().is_some() {
-        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-            "found more JSON tokens after completing parsing",
-        ));
+        return Err(
+            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                "found more JSON tokens after completing parsing",
+            ),
+        );
     }
     Ok(builder)
 }
diff --git a/src/protocol_serde/shape_role_credentials.rs b/src/protocol_serde/shape_role_credentials.rs
--- a/src/protocol_serde/shape_role_credentials.rs
+++ b/src/protocol_serde/shape_role_credentials.rs
@@ -3,14 +3,24 @@
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
     depth: u32,
-) -> ::std::result::Result<Option<crate::types::RoleCredentials>, ::aws_smithy_json::deserialize::error::DeserializeError>
+) -> ::std::result::Result<
+    Option<crate::types::RoleCredentials>,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+>
 where
-    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
+    I: Iterator<
+        Item = Result<
+            ::aws_smithy_json::deserialize::Token<'a>,
+            ::aws_smithy_json::deserialize::error::DeserializeError,
+        >,
+    >,
 {
     if depth >= 128u32 {
-        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-            "maximum nesting depth exceeded",
-        ));
+        return Err(
+            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                "maximum nesting depth exceeded",
+            ),
+        );
     }
     match tokens.next().transpose()? {
         Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
@@ -20,48 +30,62 @@
             loop {
                 match tokens.next().transpose()? {
                     Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                        "accessKeyId" => {
-                            builder = builder.set_access_key_id(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
+                        match key.to_unescaped()?.as_ref() {
+                            "accessKeyId" => {
+                                builder = builder.set_access_key_id(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
+                                        tokens.next(),
+                                    )?
                                     .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                     .transpose()?,
-                            );
-                        }
-                        "secretAccessKey" => {
-                            builder = builder.set_secret_access_key(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                );
+                            }
+                            "secretAccessKey" => {
+                                builder = builder.set_secret_access_key(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
+                                        tokens.next(),
+                                    )?
                                     .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                     .transpose()?,
-                            );
-                        }
-                        "sessionToken" => {
-                            builder = builder.set_session_token(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                );
+                            }
+                            "sessionToken" => {
+                                builder = builder.set_session_token(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
+                                        tokens.next(),
+                                    )?
                                     .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                     .transpose()?,
-                            );
-                        }
-                        "expiration" => {
-                            builder = builder.set_expiration(
-                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                                );
+                            }
+                            "expiration" => {
+                                builder = builder.set_expiration(
+                                    ::aws_smithy_json::deserialize::token::expect_number_or_null(
+                                        tokens.next(),
+                                    )?
                                     .map(i64::try_from)
                                     .transpose()?,
-                            );
+                                );
+                            }
+                            _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                         }
-                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
-                    },
+                    }
                     other => {
-                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
-                            "expected object key or end object, found: {other:?}"
-                        )))
+                        return Err(
+                            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                                format!("expected object key or end object, found: {other:?}"),
+                            ),
+                        )
                     }
                 }
             }
             Ok(Some(builder.build()))
         }
-        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-            "expected start object or null",
-        )),
+        _ => Err(
+            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                "expected start object or null",
+            ),
+        ),
     }
 }
diff --git a/src/protocol_serde/shape_role_info.rs b/src/protocol_serde/shape_role_info.rs
--- a/src/protocol_serde/shape_role_info.rs
+++ b/src/protocol_serde/shape_role_info.rs
@@ -3,14 +3,24 @@
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
     depth: u32,
-) -> ::std::result::Result<Option<crate::types::RoleInfo>, ::aws_smithy_json::deserialize::error::DeserializeError>
+) -> ::std::result::Result<
+    Option<crate::types::RoleInfo>,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+>
 where
-    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
+    I: Iterator<
+        Item = Result<
+            ::aws_smithy_json::deserialize::Token<'a>,
+            ::aws_smithy_json::deserialize::error::DeserializeError,
+        >,
+    >,
 {
     if depth >= 128u32 {
-        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-            "maximum nesting depth exceeded",
-        ));
+        return Err(
+            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                "maximum nesting depth exceeded",
+            ),
+        );
     }
     match tokens.next().transpose()? {
         Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
@@ -20,34 +30,44 @@
             loop {
                 match tokens.next().transpose()? {
                     Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                        "roleName" => {
-                            builder = builder.set_role_name(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
+                        match key.to_unescaped()?.as_ref() {
+                            "roleName" => {
+                                builder = builder.set_role_name(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
+                                        tokens.next(),
+                                    )?
                                     .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                     .transpose()?,
-                            );
-                        }
-                        "accountId" => {
-                            builder = builder.set_account_id(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                );
+                            }
+                            "accountId" => {
+                                builder = builder.set_account_id(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
+                                        tokens.next(),
+                                    )?
                                     .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                     .transpose()?,
-                            );
+                                );
+                            }
+                            _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                         }
-                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
-                    },
+                    }
                     other => {
-                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
-                            "expected object key or end object, found: {other:?}"
-                        )))
+                        return Err(
+                            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                                format!("expected object key or end object, found: {other:?}"),
+                            ),
+                        )
                     }
                 }
             }
             Ok(Some(builder.build()))
         }
-        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-            "expected start object or null",
-        )),
+        _ => Err(
+            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                "expected start object or null",
+            ),
+        ),
     }
 }
diff --git a/src/protocol_serde/shape_role_list_type.rs b/src/protocol_serde/shape_role_list_type.rs
--- a/src/protocol_serde/shape_role_list_type.rs
+++ b/src/protocol_serde/shape_role_list_type.rs
@@ -3,14 +3,24 @@
     tokens: &mut ::std::iter::Peekable<I>,
     _value: &'a [u8],
     depth: u32,
-) -> ::std::result::Result<Option<::std::vec::Vec<crate::types::RoleInfo>>, ::aws_smithy_json::deserialize::error::DeserializeError>
+) -> ::std::result::Result<
+    Option<::std::vec::Vec<crate::types::RoleInfo>>,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+>
 where
-    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
+    I: Iterator<
+        Item = Result<
+            ::aws_smithy_json::deserialize::Token<'a>,
+            ::aws_smithy_json::deserialize::error::DeserializeError,
+        >,
+    >,
 {
     if depth >= 128u32 {
-        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-            "maximum nesting depth exceeded",
-        ));
+        return Err(
+            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                "maximum nesting depth exceeded",
+            ),
+        );
     }
     match tokens.next().transpose()? {
         Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
@@ -23,13 +33,19 @@
                         break;
                     }
                     _ => {
-                        let value = crate::protocol_serde::shape_role_info::de_role_info(tokens, _value, depth + 1)?;
+                        let value = crate::protocol_serde::shape_role_info::de_role_info(
+                            tokens,
+                            _value,
+                            depth + 1,
+                        )?;
                         if let Some(value) = value {
                             items.push(value);
                         } else {
-                            return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-                                "dense list cannot contain null values",
-                            ));
+                            return Err(
+                                ::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                                    "dense list cannot contain null values",
+                                ),
+                            );
                         }
                     }
                 }
@@ -36,8 +52,10 @@
             }
             Ok(Some(items))
         }
-        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-            "expected start array or null",
-        )),
+        _ => Err(
+            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                "expected start array or null",
+            ),
+        ),
     }
 }
diff --git a/src/protocol_serde/shape_too_many_requests_exception.rs b/src/protocol_serde/shape_too_many_requests_exception.rs
--- a/src/protocol_serde/shape_too_many_requests_exception.rs
+++ b/src/protocol_serde/shape_too_many_requests_exception.rs
@@ -2,8 +2,14 @@
 pub(crate) fn de_too_many_requests_exception_json_err(
     _value: &[u8],
     mut builder: crate::types::error::builders::TooManyRequestsExceptionBuilder,
-) -> ::std::result::Result<crate::types::error::builders::TooManyRequestsExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+) -> ::std::result::Result<
+    crate::types::error::builders::TooManyRequestsExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(
+        crate::protocol_serde::or_empty_doc(_value),
+    )
+    .peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -11,27 +17,35 @@
     loop {
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "message" => {
-                    builder = builder.set_message(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
+                match key.to_unescaped()?.as_ref() {
+                    "message" => {
+                        builder = builder.set_message(
+                            ::aws_smithy_json::deserialize::token::expect_string_or_null(
+                                tokens.next(),
+                            )?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
-                    );
+                        );
+                    }
+                    _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                 }
-                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
-            },
+            }
             other => {
-                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
-                    "expected object key or end object, found: {other:?}"
-                )))
+                return Err(
+                    ::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                        "expected object key or end object, found: {other:?}"
+                    )),
+                )
             }
         }
     }
     if tokens.next().is_some() {
-        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-            "found more JSON tokens after completing parsing",
-        ));
+        return Err(
+            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                "found more JSON tokens after completing parsing",
+            ),
+        );
     }
     Ok(builder)
 }
diff --git a/src/protocol_serde/shape_unauthorized_exception.rs b/src/protocol_serde/shape_unauthorized_exception.rs
--- a/src/protocol_serde/shape_unauthorized_exception.rs
+++ b/src/protocol_serde/shape_unauthorized_exception.rs
@@ -2,8 +2,14 @@
 pub(crate) fn de_unauthorized_exception_json_err(
     _value: &[u8],
     mut builder: crate::types::error::builders::UnauthorizedExceptionBuilder,
-) -> ::std::result::Result<crate::types::error::builders::UnauthorizedExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+) -> ::std::result::Result<
+    crate::types::error::builders::UnauthorizedExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(
+        crate::protocol_serde::or_empty_doc(_value),
+    )
+    .peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -11,27 +17,35 @@
     loop {
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "message" => {
-                    builder = builder.set_message(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
+                match key.to_unescaped()?.as_ref() {
+                    "message" => {
+                        builder = builder.set_message(
+                            ::aws_smithy_json::deserialize::token::expect_string_or_null(
+                                tokens.next(),
+                            )?
                             .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                             .transpose()?,
-                    );
+                        );
+                    }
+                    _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                 }
-                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
-            },
+            }
             other => {
-                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
-                    "expected object key or end object, found: {other:?}"
-                )))
+                return Err(
+                    ::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
+                        "expected object key or end object, found: {other:?}"
+                    )),
+                )
             }
         }
     }
     if tokens.next().is_some() {
-        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
-            "found more JSON tokens after completing parsing",
-        ));
+        return Err(
+            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                "found more JSON tokens after completing parsing",
+            ),
+        );
     }
     Ok(builder)
 }
diff --git a/src/sdk_feature_tracker.rs b/src/sdk_feature_tracker.rs
--- a/src/sdk_feature_tracker.rs
+++ b/src/sdk_feature_tracker.rs
@@ -46,8 +46,12 @@
     use aws_smithy_runtime::client::sdk_feature::SmithySdkFeature;
     use aws_smithy_runtime_api::box_error::BoxError;
     use aws_smithy_runtime_api::client::interceptors::context::BeforeSerializationInterceptorContextMut;
-    use aws_smithy_runtime_api::client::interceptors::{dyn_dispatch_hint, Intercept, SharedInterceptor};
-    use aws_smithy_runtime_api::client::runtime_components::{RuntimeComponents, RuntimeComponentsBuilder};
+    use aws_smithy_runtime_api::client::interceptors::{
+        dyn_dispatch_hint, Intercept, SharedInterceptor,
+    };
+    use aws_smithy_runtime_api::client::runtime_components::{
+        RuntimeComponents, RuntimeComponentsBuilder,
+    };
     use aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin;
     use aws_smithy_types::config_bag::ConfigBag;
     use std::borrow::Cow;
@@ -73,7 +77,8 @@
             _runtime_components: &RuntimeComponents,
             cfg: &mut ConfigBag,
         ) -> Result<(), BoxError> {
-            cfg.interceptor_state().store_append::<SmithySdkFeature>(SmithySdkFeature::Paginator);
+            cfg.interceptor_state()
+                .store_append::<SmithySdkFeature>(SmithySdkFeature::Paginator);
             Ok(())
         }
     }
@@ -86,14 +91,21 @@
     impl PaginatorFeatureTrackerRuntimePlugin {
         pub(crate) fn new() -> Self {
             Self {
-                runtime_components: RuntimeComponentsBuilder::new("PaginatorFeatureTrackerRuntimePlugin")
-                    .with_interceptor(SharedInterceptor::permanent(PaginatorFeatureTrackerInterceptor::new())),
+                runtime_components: RuntimeComponentsBuilder::new(
+                    "PaginatorFeatureTrackerRuntimePlugin",
+                )
+                .with_interceptor(SharedInterceptor::permanent(
+                    PaginatorFeatureTrackerInterceptor::new(),
+                )),
             }
         }
     }

     impl RuntimePlugin for PaginatorFeatureTrackerRuntimePlugin {
-        fn runtime_components(&self, _: &RuntimeComponentsBuilder) -> Cow<'_, RuntimeComponentsBuilder> {
+        fn runtime_components(
+            &self,
+            _: &RuntimeComponentsBuilder,
+        ) -> Cow<'_, RuntimeComponentsBuilder> {
             Cow::Borrowed(&self.runtime_components)
         }
     }
@@ -104,8 +116,12 @@
     use aws_smithy_runtime::client::sdk_feature::SmithySdkFeature;
     use aws_smithy_runtime_api::box_error::BoxError;
     use aws_smithy_runtime_api::client::interceptors::context::BeforeSerializationInterceptorContextMut;
-    use aws_smithy_runtime_api::client::interceptors::{dyn_dispatch_hint, Intercept, SharedInterceptor};
-    use aws_smithy_runtime_api::client::runtime_components::{RuntimeComponents, RuntimeComponentsBuilder};
+    use aws_smithy_runtime_api::client::interceptors::{
+        dyn_dispatch_hint, Intercept, SharedInterceptor,
+    };
+    use aws_smithy_runtime_api::client::runtime_components::{
+        RuntimeComponents, RuntimeComponentsBuilder,
+    };
     use aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin;
     use aws_smithy_types::config_bag::ConfigBag;
     use std::borrow::Cow;
@@ -131,7 +147,8 @@
             _runtime_components: &RuntimeComponents,
             cfg: &mut ConfigBag,
         ) -> Result<(), BoxError> {
-            cfg.interceptor_state().store_append::<SmithySdkFeature>(SmithySdkFeature::Waiter);
+            cfg.interceptor_state()
+                .store_append::<SmithySdkFeature>(SmithySdkFeature::Waiter);
             Ok(())
         }
     }
@@ -144,14 +161,21 @@
     impl WaiterFeatureTrackerRuntimePlugin {
         pub(crate) fn new() -> Self {
             Self {
-                runtime_components: RuntimeComponentsBuilder::new("WaiterFeatureTrackerRuntimePlugin")
-                    .with_interceptor(SharedInterceptor::permanent(WaiterFeatureTrackerInterceptor::new())),
+                runtime_components: RuntimeComponentsBuilder::new(
+                    "WaiterFeatureTrackerRuntimePlugin",
+                )
+                .with_interceptor(SharedInterceptor::permanent(
+                    WaiterFeatureTrackerInterceptor::new(),
+                )),
             }
         }
     }

     impl RuntimePlugin for WaiterFeatureTrackerRuntimePlugin {
-        fn runtime_components(&self, _: &RuntimeComponentsBuilder) -> Cow<'_, RuntimeComponentsBuilder> {
+        fn runtime_components(
+            &self,
+            _: &RuntimeComponentsBuilder,
+        ) -> Cow<'_, RuntimeComponentsBuilder> {
             Cow::Borrowed(&self.runtime_components)
         }
     }
@@ -194,7 +218,10 @@
                     RetryMode::Adaptive => SmithySdkFeature::RetryModeAdaptive,
                     _ => unreachable!("retry mode must be standard or adaptive"),
                 })
-                .map(|feature| cfg.interceptor_state().store_append::<SmithySdkFeature>(feature));
+                .map(|feature| {
+                    cfg.interceptor_state()
+                        .store_append::<SmithySdkFeature>(feature)
+                });

             Ok(())
         }
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
@@ -1,14 +1,20 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+#[cfg(feature = "op_get_role_credentials")]
 pub use crate::types::_role_credentials::RoleCredentials;

+#[cfg(feature = "op_list_account_roles")]
 pub use crate::types::_role_info::RoleInfo;

+#[cfg(feature = "op_list_accounts")]
 pub use crate::types::_account_info::AccountInfo;

+#[cfg(feature = "op_list_accounts")]
 mod _account_info;

+#[cfg(feature = "op_get_role_credentials")]
 mod _role_credentials;

+#[cfg(feature = "op_list_account_roles")]
 mod _role_info;

 /// Builders
diff --git a/src/types/_account_info.rs b/src/types/_account_info.rs
--- a/src/types/_account_info.rs
+++ b/src/types/_account_info.rs
@@ -33,7 +33,9 @@
 }

 /// A builder for [`AccountInfo`](crate::types::AccountInfo).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(
+    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
+)]
 #[non_exhaustive]
 pub struct AccountInfoBuilder {
     pub(crate) account_id: ::std::option::Option<::std::string::String>,
@@ -70,12 +72,18 @@
         &self.account_name
     }
     /// <p>The email address of the AWS account that is assigned to the user.</p>
-    pub fn email_address(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
+    pub fn email_address(
+        mut self,
+        input: impl ::std::convert::Into<::std::string::String>,
+    ) -> Self {
         self.email_address = ::std::option::Option::Some(input.into());
         self
     }
     /// <p>The email address of the AWS account that is assigned to the user.</p>
-    pub fn set_email_address(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
+    pub fn set_email_address(
+        mut self,
+        input: ::std::option::Option<::std::string::String>,
+    ) -> Self {
         self.email_address = input;
         self
     }
diff --git a/src/types/_role_credentials.rs b/src/types/_role_credentials.rs
--- a/src/types/_role_credentials.rs
+++ b/src/types/_role_credentials.rs
@@ -59,12 +59,18 @@
 }
 impl RoleCredentialsBuilder {
     /// <p>The identifier used for the temporary security credentials. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_use-resources.html">Using Temporary Security Credentials to Request Access to AWS Resources</a> in the <i>AWS IAM User Guide</i>.</p>
-    pub fn access_key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
+    pub fn access_key_id(
+        mut self,
+        input: impl ::std::convert::Into<::std::string::String>,
+    ) -> Self {
         self.access_key_id = ::std::option::Option::Some(input.into());
         self
     }
     /// <p>The identifier used for the temporary security credentials. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_use-resources.html">Using Temporary Security Credentials to Request Access to AWS Resources</a> in the <i>AWS IAM User Guide</i>.</p>
-    pub fn set_access_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
+    pub fn set_access_key_id(
+        mut self,
+        input: ::std::option::Option<::std::string::String>,
+    ) -> Self {
         self.access_key_id = input;
         self
     }
@@ -73,12 +79,18 @@
         &self.access_key_id
     }
     /// <p>The key that is used to sign the request. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_use-resources.html">Using Temporary Security Credentials to Request Access to AWS Resources</a> in the <i>AWS IAM User Guide</i>.</p>
-    pub fn secret_access_key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
+    pub fn secret_access_key(
+        mut self,
+        input: impl ::std::convert::Into<::std::string::String>,
+    ) -> Self {
         self.secret_access_key = ::std::option::Option::Some(input.into());
         self
     }
     /// <p>The key that is used to sign the request. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_use-resources.html">Using Temporary Security Credentials to Request Access to AWS Resources</a> in the <i>AWS IAM User Guide</i>.</p>
-    pub fn set_secret_access_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
+    pub fn set_secret_access_key(
+        mut self,
+        input: ::std::option::Option<::std::string::String>,
+    ) -> Self {
         self.secret_access_key = input;
         self
     }
@@ -87,12 +99,18 @@
         &self.secret_access_key
     }
     /// <p>The token used for temporary credentials. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_use-resources.html">Using Temporary Security Credentials to Request Access to AWS Resources</a> in the <i>AWS IAM User Guide</i>.</p>
-    pub fn session_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
+    pub fn session_token(
+        mut self,
+        input: impl ::std::convert::Into<::std::string::String>,
+    ) -> Self {
         self.session_token = ::std::option::Option::Some(input.into());
         self
     }
     /// <p>The token used for temporary credentials. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_use-resources.html">Using Temporary Security Credentials to Request Access to AWS Resources</a> in the <i>AWS IAM User Guide</i>.</p>
-    pub fn set_session_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
+    pub fn set_session_token(
+        mut self,
+        input: ::std::option::Option<::std::string::String>,
+    ) -> Self {
         self.session_token = input;
         self
     }
diff --git a/src/types/_role_info.rs b/src/types/_role_info.rs
--- a/src/types/_role_info.rs
+++ b/src/types/_role_info.rs
@@ -27,7 +27,9 @@
 }

 /// A builder for [`RoleInfo`](crate::types::RoleInfo).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(
+    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
+)]
 #[non_exhaustive]
 pub struct RoleInfoBuilder {
     pub(crate) role_name: ::std::option::Option<::std::string::String>,
diff --git a/src/types/builders.rs b/src/types/builders.rs
--- a/src/types/builders.rs
+++ b/src/types/builders.rs
@@ -1,6 +1,9 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+#[cfg(feature = "op_get_role_credentials")]
 pub use crate::types::_role_credentials::RoleCredentialsBuilder;

+#[cfg(feature = "op_list_account_roles")]
 pub use crate::types::_role_info::RoleInfoBuilder;

+#[cfg(feature = "op_list_accounts")]
 pub use crate::types::_account_info::AccountInfoBuilder;
diff --git a/src/types/error.rs b/src/types/error.rs
--- a/src/types/error.rs
+++ b/src/types/error.rs
@@ -1,18 +1,64 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+#[cfg(any(
+    feature = "op_get_role_credentials",
+    feature = "op_list_account_roles",
+    feature = "op_list_accounts",
+    feature = "op_logout"
+))]
 pub use crate::types::error::_invalid_request_exception::InvalidRequestException;

+#[cfg(any(
+    feature = "op_get_role_credentials",
+    feature = "op_list_account_roles",
+    feature = "op_list_accounts"
+))]
 pub use crate::types::error::_resource_not_found_exception::ResourceNotFoundException;

+#[cfg(any(
+    feature = "op_get_role_credentials",
+    feature = "op_list_account_roles",
+    feature = "op_list_accounts",
+    feature = "op_logout"
+))]
 pub use crate::types::error::_too_many_requests_exception::TooManyRequestsException;

+#[cfg(any(
+    feature = "op_get_role_credentials",
+    feature = "op_list_account_roles",
+    feature = "op_list_accounts",
+    feature = "op_logout"
+))]
 pub use crate::types::error::_unauthorized_exception::UnauthorizedException;

+#[cfg(any(
+    feature = "op_get_role_credentials",
+    feature = "op_list_account_roles",
+    feature = "op_list_accounts",
+    feature = "op_logout"
+))]
 mod _invalid_request_exception;

+#[cfg(any(
+    feature = "op_get_role_credentials",
+    feature = "op_list_account_roles",
+    feature = "op_list_accounts"
+))]
 mod _resource_not_found_exception;

+#[cfg(any(
+    feature = "op_get_role_credentials",
+    feature = "op_list_account_roles",
+    feature = "op_list_accounts",
+    feature = "op_logout"
+))]
 mod _too_many_requests_exception;

+#[cfg(any(
+    feature = "op_get_role_credentials",
+    feature = "op_list_account_roles",
+    feature = "op_list_accounts",
+    feature = "op_logout"
+))]
 mod _unauthorized_exception;

 /// Builders
diff --git a/src/types/error/_invalid_request_exception.rs b/src/types/error/_invalid_request_exception.rs
--- a/src/types/error/_invalid_request_exception.rs
+++ b/src/types/error/_invalid_request_exception.rs
@@ -45,7 +45,9 @@
 }

 /// A builder for [`InvalidRequestException`](crate::types::error::InvalidRequestException).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(
+    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
+)]
 #[non_exhaustive]
 pub struct InvalidRequestExceptionBuilder {
     pub(crate) message: ::std::option::Option<::std::string::String>,
@@ -73,7 +75,10 @@
     }

     /// Sets error metadata
-    pub fn set_meta(&mut self, meta: std::option::Option<::aws_smithy_types::error::ErrorMetadata>) -> &mut Self {
+    pub fn set_meta(
+        &mut self,
+        meta: std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
+    ) -> &mut Self {
         self.meta = meta;
         self
     }
diff --git a/src/types/error/_resource_not_found_exception.rs b/src/types/error/_resource_not_found_exception.rs
--- a/src/types/error/_resource_not_found_exception.rs
+++ b/src/types/error/_resource_not_found_exception.rs
@@ -45,7 +45,9 @@
 }

 /// A builder for [`ResourceNotFoundException`](crate::types::error::ResourceNotFoundException).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(
+    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
+)]
 #[non_exhaustive]
 pub struct ResourceNotFoundExceptionBuilder {
     pub(crate) message: ::std::option::Option<::std::string::String>,
@@ -73,7 +75,10 @@
     }

     /// Sets error metadata
-    pub fn set_meta(&mut self, meta: std::option::Option<::aws_smithy_types::error::ErrorMetadata>) -> &mut Self {
+    pub fn set_meta(
+        &mut self,
+        meta: std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
+    ) -> &mut Self {
         self.meta = meta;
         self
     }
diff --git a/src/types/error/_too_many_requests_exception.rs b/src/types/error/_too_many_requests_exception.rs
--- a/src/types/error/_too_many_requests_exception.rs
+++ b/src/types/error/_too_many_requests_exception.rs
@@ -45,7 +45,9 @@
 }

 /// A builder for [`TooManyRequestsException`](crate::types::error::TooManyRequestsException).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(
+    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
+)]
 #[non_exhaustive]
 pub struct TooManyRequestsExceptionBuilder {
     pub(crate) message: ::std::option::Option<::std::string::String>,
@@ -73,7 +75,10 @@
     }

     /// Sets error metadata
-    pub fn set_meta(&mut self, meta: std::option::Option<::aws_smithy_types::error::ErrorMetadata>) -> &mut Self {
+    pub fn set_meta(
+        &mut self,
+        meta: std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
+    ) -> &mut Self {
         self.meta = meta;
         self
     }
diff --git a/src/types/error/_unauthorized_exception.rs b/src/types/error/_unauthorized_exception.rs
--- a/src/types/error/_unauthorized_exception.rs
+++ b/src/types/error/_unauthorized_exception.rs
@@ -45,7 +45,9 @@
 }

 /// A builder for [`UnauthorizedException`](crate::types::error::UnauthorizedException).
-#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
+#[derive(
+    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
+)]
 #[non_exhaustive]
 pub struct UnauthorizedExceptionBuilder {
     pub(crate) message: ::std::option::Option<::std::string::String>,
@@ -73,7 +75,10 @@
     }

     /// Sets error metadata
-    pub fn set_meta(&mut self, meta: std::option::Option<::aws_smithy_types::error::ErrorMetadata>) -> &mut Self {
+    pub fn set_meta(
+        &mut self,
+        meta: std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
+    ) -> &mut Self {
         self.meta = meta;
         self
     }
diff --git a/src/types/error/builders.rs b/src/types/error/builders.rs
--- a/src/types/error/builders.rs
+++ b/src/types/error/builders.rs
@@ -1,8 +1,31 @@
 // Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
+#[cfg(any(
+    feature = "op_get_role_credentials",
+    feature = "op_list_account_roles",
+    feature = "op_list_accounts",
+    feature = "op_logout"
+))]
 pub use crate::types::error::_invalid_request_exception::InvalidRequestExceptionBuilder;

+#[cfg(any(
+    feature = "op_get_role_credentials",
+    feature = "op_list_account_roles",
+    feature = "op_list_accounts"
+))]
 pub use crate::types::error::_resource_not_found_exception::ResourceNotFoundExceptionBuilder;

+#[cfg(any(
+    feature = "op_get_role_credentials",
+    feature = "op_list_account_roles",
+    feature = "op_list_accounts",
+    feature = "op_logout"
+))]
 pub use crate::types::error::_too_many_requests_exception::TooManyRequestsExceptionBuilder;

+#[cfg(any(
+    feature = "op_get_role_credentials",
+    feature = "op_list_account_roles",
+    feature = "op_list_accounts",
+    feature = "op_logout"
+))]
 pub use crate::types::error::_unauthorized_exception::UnauthorizedExceptionBuilder;
```


## Customizations

- Add one non-default Cargo feature and matching cfg gates for each model operation.
- Rename the Cargo package/library from manifest data.
- Remove upstream `tests/` from generated output.

## Reproduction

```text
cargo run -p aws-sdk-modularizer -- --manifest services-manifest.json
```
