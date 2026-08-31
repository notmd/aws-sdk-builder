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
- `src/json_errors.rs`
- `src/lens.rs`
- `src/meta.rs`
- `src/observability_feature.rs`
- `src/operation.rs`
- `src/operation/get_role_credentials.rs`
- `src/operation/get_role_credentials/_get_role_credentials_input.rs`
- `src/operation/get_role_credentials/builders.rs`
- `src/operation/list_account_roles.rs`
- `src/operation/list_account_roles/_list_account_roles_input.rs`
- `src/operation/list_account_roles/builders.rs`
- `src/operation/list_account_roles/paginator.rs`
- `src/operation/list_accounts.rs`
- `src/operation/list_accounts/_list_accounts_input.rs`
- `src/operation/list_accounts/_list_accounts_output.rs`
- `src/operation/list_accounts/builders.rs`
- `src/operation/list_accounts/paginator.rs`
- `src/operation/logout.rs`
- `src/operation/logout/_logout_input.rs`
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
- `src/types/builders.rs`
- `src/types/error.rs`
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
@@ -117,10 +117,14 @@
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

### `src/client/get_role_credentials.rs`

```diff
diff --git a/src/client/get_role_credentials.rs b/src/client/get_role_credentials.rs
--- a/src/client/get_role_credentials.rs
+++ b/src/client/get_role_credentials.rs
@@ -9,7 +9,9 @@
     /// - On success, responds with [`GetRoleCredentialsOutput`](crate::operation::get_role_credentials::GetRoleCredentialsOutput) with field(s):
     ///   - [`role_credentials(Option<RoleCredentials>)`](crate::operation::get_role_credentials::GetRoleCredentialsOutput::role_credentials): <p>The credentials for the role that is assigned to the user.</p>
     /// - On failure, responds with [`SdkError<GetRoleCredentialsError>`](crate::operation::get_role_credentials::GetRoleCredentialsError)
-    pub fn get_role_credentials(&self) -> crate::operation::get_role_credentials::builders::GetRoleCredentialsFluentBuilder {
+    pub fn get_role_credentials(
+        &self,
+    ) -> crate::operation::get_role_credentials::builders::GetRoleCredentialsFluentBuilder {
         crate::operation::get_role_credentials::builders::GetRoleCredentialsFluentBuilder::new(self.handle.clone())
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
@@ -193,18 +209,42 @@
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
         builder.set_region(config_bag.load::<crate::config::Region>().cloned());
         builder
     }
@@ -220,7 +260,10 @@
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
@@ -253,7 +296,10 @@
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
@@ -296,7 +342,9 @@
     /// # }
     /// ```
     pub fn http_client(mut self, http_client: impl crate::config::HttpClient + 'static) -> Self {
-        self.set_http_client(::std::option::Option::Some(crate::config::IntoShared::into_shared(http_client)));
+        self.set_http_client(::std::option::Option::Some(crate::config::IntoShared::into_shared(
+            http_client,
+        )));
         self
     }

@@ -441,7 +489,10 @@
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
@@ -476,7 +527,10 @@
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
@@ -485,7 +539,10 @@
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
@@ -505,7 +562,9 @@
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
@@ -592,7 +651,10 @@
     /// let config = aws_sdk_sso::Config::builder().endpoint_resolver(resolver).build();
     /// let client = aws_sdk_sso::Client::from_conf(config);
     /// ```
-    pub fn endpoint_resolver(mut self, endpoint_resolver: impl crate::config::endpoint::ResolveEndpoint + 'static) -> Self {
+    pub fn endpoint_resolver(
+        mut self,
+        endpoint_resolver: impl crate::config::endpoint::ResolveEndpoint + 'static,
+    ) -> Self {
         self.set_endpoint_resolver(::std::option::Option::Some(endpoint_resolver.into_shared_resolver()));
         self
     }
@@ -650,7 +712,10 @@
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
@@ -674,7 +739,9 @@
     /// let config = Config::builder().sleep_impl(sleep_impl).build();
     /// ```
     pub fn sleep_impl(mut self, sleep_impl: impl crate::config::AsyncSleep + 'static) -> Self {
-        self.set_sleep_impl(Some(::aws_smithy_runtime_api::shared::IntoShared::into_shared(sleep_impl)));
+        self.set_sleep_impl(Some(::aws_smithy_runtime_api::shared::IntoShared::into_shared(
+            sleep_impl,
+        )));
         self
     }

@@ -750,9 +817,14 @@
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
@@ -921,8 +993,12 @@
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
@@ -984,19 +1060,25 @@
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
@@ -1091,9 +1173,9 @@
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

@@ -1110,9 +1192,12 @@
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
@@ -1153,7 +1238,10 @@
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
@@ -1160,7 +1248,10 @@
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
@@ -1191,7 +1282,8 @@
     /// will be prefixed onto this URL. To fully override the endpoint resolver, use
     /// [`Builder::endpoint_resolver`].
     pub fn set_endpoint_url(&mut self, endpoint_url: Option<::std::string::String>) -> &mut Self {
-        self.config.store_or_unset(endpoint_url.map(::aws_types::endpoint_config::EndpointUrl));
+        self.config
+            .store_or_unset(endpoint_url.map(::aws_types::endpoint_config::EndpointUrl));
         self
     }
     /// When true, use the dual-stack endpoint. If the configured endpoint does not support dual-stack, dispatching the request MAY return an error.
@@ -1201,7 +1293,8 @@
     }
     /// When true, use the dual-stack endpoint. If the configured endpoint does not support dual-stack, dispatching the request MAY return an error.
     pub fn set_use_dual_stack(&mut self, use_dual_stack: Option<bool>) -> &mut Self {
-        self.config.store_or_unset(use_dual_stack.map(::aws_types::endpoint_config::UseDualStack));
+        self.config
+            .store_or_unset(use_dual_stack.map(::aws_types::endpoint_config::UseDualStack));
         self
     }
     /// When true, send this request to the FIPS-compliant regional endpoint. If the configured endpoint does not have a FIPS compliant endpoint, dispatching the request will return an error.
@@ -1211,7 +1304,8 @@
     }
     /// When true, send this request to the FIPS-compliant regional endpoint. If the configured endpoint does not have a FIPS compliant endpoint, dispatching the request will return an error.
     pub fn set_use_fips(&mut self, use_fips: Option<bool>) -> &mut Self {
-        self.config.store_or_unset(use_fips.map(::aws_types::endpoint_config::UseFips));
+        self.config
+            .store_or_unset(use_fips.map(::aws_types::endpoint_config::UseFips));
         self
     }
     /// Sets the AWS region to use when making requests.
@@ -1235,14 +1329,20 @@
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
@@ -1342,10 +1442,13 @@
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
@@ -1414,7 +1517,8 @@
             cfg.store_put(::aws_smithy_runtime::client::orchestrator::AuthSchemeAndEndpointOrchestrationV2);
             ::std::option::Option::Some(cfg.freeze())
         };
-        let mut runtime_components = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ServiceRuntimePlugin");
+        let mut runtime_components =
+            ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ServiceRuntimePlugin");
         runtime_components.set_auth_scheme_option_resolver(::std::option::Option::Some({
             use crate::config::auth::ResolveAuthScheme;
             crate::config::auth::DefaultAuthSchemeResolver::default().into_shared_resolver()
@@ -1423,32 +1527,49 @@
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

@@ -1491,14 +1612,22 @@
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

@@ -1511,7 +1640,9 @@
         // This is skipped if the override already sets its own identity cache.
         if components.has_identity_resolvers() && components.identity_cache().is_none() {
             components.set_identity_cache(::std::option::Option::Some(
-                ::aws_smithy_runtime::client::identity::IdentityCache::lazy().max_partitions(1).build(),
+                ::aws_smithy_runtime::client::identity::IdentityCache::lazy()
+                    .max_partitions(1)
+                    .build(),
             ));
         }

@@ -1599,7 +1730,11 @@
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
@@ -1610,13 +1745,16 @@

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
@@ -104,7 +107,8 @@
             None => &self.service_defaults,
         };

-        let _fut = ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionsFuture::ready(Ok(modeled_auth_options.clone()));
+        let _fut =
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionsFuture::ready(Ok(modeled_auth_options.clone()));

         _fut
     }
@@ -139,7 +143,10 @@
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
@@ -149,7 +156,9 @@
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
@@ -480,7 +482,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso-fips.cn-north-1.api.amazonwebservices.com.cn");
+        let endpoint =
+            endpoint.expect("Expected valid endpoint: https://portal.sso-fips.cn-north-1.api.amazonwebservices.com.cn");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -520,7 +523,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.cn-north-1.api.amazonwebservices.com.cn");
+        let endpoint =
+            endpoint.expect("Expected valid endpoint: https://portal.sso.cn-north-1.api.amazonwebservices.com.cn");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -744,7 +748,9 @@
         let endpoint = endpoint.expect("Expected valid endpoint: https://example.com");
         assert_eq!(
             endpoint,
-            ::aws_smithy_types::endpoint::Endpoint::builder().url("https://example.com").build()
+            ::aws_smithy_types::endpoint::Endpoint::builder()
+                .url("https://example.com")
+                .build()
         );
     }

@@ -762,7 +768,9 @@
         let endpoint = endpoint.expect("Expected valid endpoint: https://example.com");
         assert_eq!(
             endpoint,
-            ::aws_smithy_types::endpoint::Endpoint::builder().url("https://example.com").build()
+            ::aws_smithy_types::endpoint::Endpoint::builder()
+                .url("https://example.com")
+                .build()
         );
     }

@@ -779,7 +787,10 @@
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
@@ -804,7 +815,9 @@
     /// Missing region
     #[test]
     fn test_40() {
-        let params = crate::config::endpoint::Params::builder().build().expect("invalid params");
+        let params = crate::config::endpoint::Params::builder()
+            .build()
+            .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
         let error = endpoint.expect_err("expected error: Invalid Configuration: Missing Region [Missing region]");
@@ -815,7 +828,10 @@
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
@@ -840,7 +856,9 @@
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
@@ -884,7 +902,8 @@
     fn resolve_endpoint<'a>(
         &'a self,
         params: &'a crate::config::endpoint::Params,
-    ) -> ::std::result::Result<::aws_smithy_types::endpoint::Endpoint, ::aws_smithy_runtime_api::box_error::BoxError> {
+    ) -> ::std::result::Result<::aws_smithy_types::endpoint::Endpoint, ::aws_smithy_runtime_api::box_error::BoxError>
+    {
         let mut _diagnostic_collector = crate::endpoint_lib::diagnostic::DiagnosticCollector::new();
         #[allow(unused_mut)]
         let mut context = ConditionContext::default();
@@ -900,22 +919,36 @@
             match current_ref {
                 ref_val if ref_val >= 100_000_000 => {
                     return match (ref_val - 100_000_000) as usize {
-                        0 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
-                            "No endpoint rule matched",
-                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
-                        1 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
-                            "Invalid Configuration: FIPS and custom endpoint are not supported".to_string(),
-                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
-                        2 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
-                            "Invalid Configuration: Dualstack and custom endpoint are not supported".to_string(),
-                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        0 => ::std::result::Result::Err(Box::new(
+                            ::aws_smithy_http::endpoint::ResolveEndpointError::message("No endpoint rule matched"),
+                        )
+                            as ::aws_smithy_runtime_api::box_error::BoxError),
+                        1 => ::std::result::Result::Err(Box::new(
+                            ::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                                "Invalid Configuration: FIPS and custom endpoint are not supported".to_string(),
+                            ),
+                        )
+                            as ::aws_smithy_runtime_api::box_error::BoxError),
+                        2 => ::std::result::Result::Err(Box::new(
+                            ::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                                "Invalid Configuration: Dualstack and custom endpoint are not supported".to_string(),
+                            ),
+                        )
+                            as ::aws_smithy_runtime_api::box_error::BoxError),
                         3 => {
                             let endpoint = params.endpoint.as_deref().unwrap_or_default();
-                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url(endpoint.to_owned()).build())
+                            ::std::result::Result::Ok(
+                                ::aws_smithy_types::endpoint::Endpoint::builder()
+                                    .url(endpoint.to_owned())
+                                    .build(),
+                            )
                         }
                         4 => {
                             let region = params.region.as_deref().unwrap_or_default();
-                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            let partition_result = context
+                                .partition_result
+                                .as_ref()
+                                .expect("Guaranteed to have a value by earlier checks.");
                             ::std::result::Result::Ok(
                                 ::aws_smithy_types::endpoint::Endpoint::builder()
                                     .url({
@@ -931,9 +964,13 @@
                                     .build(),
                             )
                         }
-                        5 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
-                            "FIPS and DualStack are enabled, but this partition does not support one or both".to_string(),
-                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        5 => ::std::result::Result::Err(Box::new(
+                            ::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                                "FIPS and DualStack are enabled, but this partition does not support one or both"
+                                    .to_string(),
+                            ),
+                        )
+                            as ::aws_smithy_runtime_api::box_error::BoxError),
                         6 => {
                             let region = params.region.as_deref().unwrap_or_default();
                             ::std::result::Result::Ok(
@@ -951,7 +988,10 @@
                         }
                         7 => {
                             let region = params.region.as_deref().unwrap_or_default();
-                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            let partition_result = context
+                                .partition_result
+                                .as_ref()
+                                .expect("Guaranteed to have a value by earlier checks.");
                             ::std::result::Result::Ok(
                                 ::aws_smithy_types::endpoint::Endpoint::builder()
                                     .url({
@@ -967,12 +1007,18 @@
                                     .build(),
                             )
                         }
-                        8 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
-                            "FIPS is enabled but this partition does not support FIPS".to_string(),
-                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        8 => ::std::result::Result::Err(Box::new(
+                            ::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                                "FIPS is enabled but this partition does not support FIPS".to_string(),
+                            ),
+                        )
+                            as ::aws_smithy_runtime_api::box_error::BoxError),
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
@@ -988,12 +1034,18 @@
                                     .build(),
                             )
                         }
-                        10 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
-                            "DualStack is enabled but this partition does not support DualStack".to_string(),
-                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        10 => ::std::result::Result::Err(Box::new(
+                            ::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                                "DualStack is enabled but this partition does not support DualStack".to_string(),
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
@@ -1009,19 +1061,23 @@
                                     .build(),
                             )
                         }
-                        12 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
-                            "Invalid Configuration: Missing Region".to_string(),
-                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
-                        _ => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
-                            "No endpoint rule matched",
-                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        12 => ::std::result::Result::Err(Box::new(
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
@@ -1070,7 +1126,11 @@
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
@@ -1078,18 +1138,24 @@
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
@@ -1222,7 +1288,9 @@
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
@@ -1353,7 +1421,9 @@
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
@@ -48,19 +98,54 @@
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
             ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
             _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
@@ -70,10 +155,13 @@
         }
     }
 }
+#[cfg(feature = "op_get_role_credentials")]
 impl From<crate::operation::get_role_credentials::GetRoleCredentialsError> for Error {
     fn from(err: crate::operation::get_role_credentials::GetRoleCredentialsError) -> Self {
         match err {
-            crate::operation::get_role_credentials::GetRoleCredentialsError::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
+            crate::operation::get_role_credentials::GetRoleCredentialsError::InvalidRequestException(inner) => {
+                Error::InvalidRequestException(inner)
+            }
             crate::operation::get_role_credentials::GetRoleCredentialsError::ResourceNotFoundException(inner) => {
                 Error::ResourceNotFoundException(inner)
             }
@@ -80,16 +168,32 @@
             crate::operation::get_role_credentials::GetRoleCredentialsError::TooManyRequestsException(inner) => {
                 Error::TooManyRequestsException(inner)
             }
-            crate::operation::get_role_credentials::GetRoleCredentialsError::UnauthorizedException(inner) => Error::UnauthorizedException(inner),
-            crate::operation::get_role_credentials::GetRoleCredentialsError::Unhandled(inner) => Error::Unhandled(inner),
+            crate::operation::get_role_credentials::GetRoleCredentialsError::UnauthorizedException(inner) => {
+                Error::UnauthorizedException(inner)
+            }
+            crate::operation::get_role_credentials::GetRoleCredentialsError::Unhandled(inner) => {
+                Error::Unhandled(inner)
+            }
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
             ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
             _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
@@ -99,22 +203,35 @@
         }
     }
 }
+#[cfg(feature = "op_list_account_roles")]
 impl From<crate::operation::list_account_roles::ListAccountRolesError> for Error {
     fn from(err: crate::operation::list_account_roles::ListAccountRolesError) -> Self {
         match err {
-            crate::operation::list_account_roles::ListAccountRolesError::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
-            crate::operation::list_account_roles::ListAccountRolesError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
-            crate::operation::list_account_roles::ListAccountRolesError::TooManyRequestsException(inner) => Error::TooManyRequestsException(inner),
-            crate::operation::list_account_roles::ListAccountRolesError::UnauthorizedException(inner) => Error::UnauthorizedException(inner),
+            crate::operation::list_account_roles::ListAccountRolesError::InvalidRequestException(inner) => {
+                Error::InvalidRequestException(inner)
+            }
+            crate::operation::list_account_roles::ListAccountRolesError::ResourceNotFoundException(inner) => {
+                Error::ResourceNotFoundException(inner)
+            }
+            crate::operation::list_account_roles::ListAccountRolesError::TooManyRequestsException(inner) => {
+                Error::TooManyRequestsException(inner)
+            }
+            crate::operation::list_account_roles::ListAccountRolesError::UnauthorizedException(inner) => {
+                Error::UnauthorizedException(inner)
+            }
             crate::operation::list_account_roles::ListAccountRolesError::Unhandled(inner) => Error::Unhandled(inner),
         }
     }
 }
-impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_accounts::ListAccountsError, R>> for Error
+#[cfg(feature = "op_list_accounts")]
+impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_accounts::ListAccountsError, R>>
+    for Error
 where
     R: Send + Sync + std::fmt::Debug + 'static,
 {
-    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_accounts::ListAccountsError, R>) -> Self {
+    fn from(
+        err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_accounts::ListAccountsError, R>,
+    ) -> Self {
         match err {
             ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
             _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
@@ -124,17 +241,27 @@
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
+            crate::operation::list_accounts::ListAccountsError::InvalidRequestException(inner) => {
+                Error::InvalidRequestException(inner)
+            }
+            crate::operation::list_accounts::ListAccountsError::ResourceNotFoundException(inner) => {
+                Error::ResourceNotFoundException(inner)
+            }
+            crate::operation::list_accounts::ListAccountsError::TooManyRequestsException(inner) => {
+                Error::TooManyRequestsException(inner)
+            }
+            crate::operation::list_accounts::ListAccountsError::UnauthorizedException(inner) => {
+                Error::UnauthorizedException(inner)
+            }
             crate::operation::list_accounts::ListAccountsError::Unhandled(inner) => Error::Unhandled(inner),
         }
     }
 }
+#[cfg(feature = "op_logout")]
 impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::logout::LogoutError, R>> for Error
 where
     R: Send + Sync + std::fmt::Debug + 'static,
@@ -149,11 +276,16 @@
         }
     }
 }
+#[cfg(feature = "op_logout")]
 impl From<crate::operation::logout::LogoutError> for Error {
     fn from(err: crate::operation::logout::LogoutError) -> Self {
         match err {
-            crate::operation::logout::LogoutError::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
-            crate::operation::logout::LogoutError::TooManyRequestsException(inner) => Error::TooManyRequestsException(inner),
+            crate::operation::logout::LogoutError::InvalidRequestException(inner) => {
+                Error::InvalidRequestException(inner)
+            }
+            crate::operation::logout::LogoutError::TooManyRequestsException(inner) => {
+                Error::TooManyRequestsException(inner)
+            }
             crate::operation::logout::LogoutError::UnauthorizedException(inner) => Error::UnauthorizedException(inner),
             crate::operation::logout::LogoutError::Unhandled(inner) => Error::Unhandled(inner),
         }
@@ -162,9 +294,32 @@
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
@@ -173,9 +328,32 @@
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
@@ -58,10 +58,15 @@
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

 pub fn parse_error_metadata(payload: &[u8], headers: &Headers) -> Result<ErrorMetadataBuilder, DeserializeError> {
@@ -68,7 +73,11 @@
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
@@ -112,7 +121,9 @@
     fn code_takes_priority() {
         assert_eq!(
             Some(Cow::Borrowed("BarError")),
-            parse_error_body(br#"{ "code": "BarError", "__type": "FooError" }"#).unwrap().code
+            parse_error_body(br#"{ "code": "BarError", "__type": "FooError" }"#)
+                .unwrap()
+                .code
         );
     }

@@ -129,7 +140,9 @@
     #[test]
     fn sanitize_namespace_and_url() {
         assert_eq!(
-            sanitize_error_code("aws.protocoltests.restjson#FooError:http://internal.amazon.com/coral/com.amazon.coral.validate/"),
+            sanitize_error_code(
+                "aws.protocoltests.restjson#FooError:http://internal.amazon.com/coral/com.amazon.coral.validate/"
+            ),
             "FooError"
         );
     }
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
@@ -91,19 +101,26 @@
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
             GetRoleCredentialsRequestSerializer,
         ));
-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            GetRoleCredentialsResponseDeserializer,
-        ));
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
+        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
+            "GetRoleCredentials",
+            "SSO",
+        ));

         ::std::option::Option::Some(cfg.freeze())
     }
@@ -113,25 +130,37 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetRoleCredentials")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new(
+            "GetRoleCredentials",
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 GetRoleCredentialsTelemetryInputCaptureInterceptor,
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
                 GetRoleCredentialsEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+            ),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 crate::operation::get_role_credentials::GetRoleCredentialsError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::get_role_credentials::GetRoleCredentialsError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::get_role_credentials::GetRoleCredentialsError,
-            >::new());
+            >::new(),
+        )
+        .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+            crate::operation::get_role_credentials::GetRoleCredentialsError,
+        >::new());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -202,7 +231,9 @@
         let parse_result = if !success && status != 200 || force_error {
             crate::protocol_serde::shape_get_role_credentials::de_get_role_credentials_http_error(status, headers, body)
         } else {
-            crate::protocol_serde::shape_get_role_credentials::de_get_role_credentials_http_response(status, headers, body)
+            crate::protocol_serde::shape_get_role_credentials::de_get_role_credentials_http_response(
+                status, headers, body,
+            )
         };
         crate::protocol_serde::type_erase_result(parse_result)
     }
@@ -210,12 +241,20 @@
 #[derive(Debug)]
 struct GetRoleCredentialsRequestSerializer;
 impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for GetRoleCredentialsRequestSerializer {
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
             .downcast::<crate::operation::get_role_credentials::GetRoleCredentialsInput>()
             .expect("correct type");
@@ -239,25 +278,35 @@
             ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
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
@@ -266,11 +315,14 @@
             fn update_http_builder(
                 input: &crate::operation::get_role_credentials::GetRoleCredentialsInput,
                 builder: ::http_1x::request::Builder,
-            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError>
+            {
                 let mut uri = ::std::string::String::new();
                 uri_base(input, &mut uri)?;
                 uri_query(input, &mut uri)?;
-                let builder = crate::protocol_serde::shape_get_role_credentials::ser_get_role_credentials_headers(input, builder)?;
+                let builder = crate::protocol_serde::shape_get_role_credentials::ser_get_role_credentials_headers(
+                    input, builder,
+                )?;
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
@@ -309,13 +361,21 @@
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
@@ -336,18 +396,22 @@
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
+            ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
+        >,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
             source: err.into(),
@@ -411,7 +475,9 @@
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
@@ -431,10 +497,18 @@
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
```

### `src/operation/get_role_credentials/_get_role_credentials_input.rs`

```diff
diff --git a/src/operation/get_role_credentials/_get_role_credentials_input.rs b/src/operation/get_role_credentials/_get_role_credentials_input.rs
--- a/src/operation/get_role_credentials/_get_role_credentials_input.rs
+++ b/src/operation/get_role_credentials/_get_role_credentials_input.rs
@@ -97,8 +97,10 @@
     /// Consumes the builder and constructs a [`GetRoleCredentialsInput`](crate::operation::get_role_credentials::GetRoleCredentialsInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<crate::operation::get_role_credentials::GetRoleCredentialsInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        crate::operation::get_role_credentials::GetRoleCredentialsInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(crate::operation::get_role_credentials::GetRoleCredentialsInput {
             role_name: self.role_name,
             account_id: self.account_id,
```

### `src/operation/get_role_credentials/builders.rs`

```diff
diff --git a/src/operation/get_role_credentials/builders.rs b/src/operation/get_role_credentials/builders.rs
--- a/src/operation/get_role_credentials/builders.rs
+++ b/src/operation/get_role_credentials/builders.rs
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
@@ -91,19 +101,26 @@
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
             ListAccountRolesRequestSerializer,
         ));
-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            ListAccountRolesResponseDeserializer,
-        ));
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
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
+                crate::config::auth::Params::builder()
+                    .operation_name("ListAccountRoles")
+                    .build()
+                    .expect("required fields set"),
+            ),
+        );
+
+        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
+            "ListAccountRoles",
+            "SSO",
         ));

-        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("ListAccountRoles", "SSO"));
-
         ::std::option::Option::Some(cfg.freeze())
     }

@@ -112,25 +129,37 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ListAccountRoles")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new(
+            "ListAccountRoles",
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ListAccountRolesTelemetryInputCaptureInterceptor,
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
                 ListAccountRolesEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::list_account_roles::ListAccountRolesError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+            ),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 crate::operation::list_account_roles::ListAccountRolesError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::list_account_roles::ListAccountRolesError,
-            >::new());
+            >::new(),
+        )
+        .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+            crate::operation::list_account_roles::ListAccountRolesError,
+        >::new());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -209,12 +238,20 @@
 #[derive(Debug)]
 struct ListAccountRolesRequestSerializer;
 impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for ListAccountRolesRequestSerializer {
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
             .downcast::<crate::operation::list_account_roles::ListAccountRolesInput>()
             .expect("correct type");
@@ -244,18 +281,26 @@
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
@@ -264,11 +309,13 @@
             fn update_http_builder(
                 input: &crate::operation::list_account_roles::ListAccountRolesInput,
                 builder: ::http_1x::request::Builder,
-            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError>
+            {
                 let mut uri = ::std::string::String::new();
                 uri_base(input, &mut uri)?;
                 uri_query(input, &mut uri)?;
-                let builder = crate::protocol_serde::shape_list_account_roles::ser_list_account_roles_headers(input, builder)?;
+                let builder =
+                    crate::protocol_serde::shape_list_account_roles::ser_list_account_roles_headers(input, builder)?;
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
@@ -307,13 +354,21 @@
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
@@ -334,18 +389,22 @@
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
+            ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
+        >,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
             source: err.into(),
@@ -409,7 +468,9 @@
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
@@ -429,10 +490,18 @@
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
```

### `src/operation/list_account_roles/_list_account_roles_input.rs`

```diff
diff --git a/src/operation/list_account_roles/_list_account_roles_input.rs b/src/operation/list_account_roles/_list_account_roles_input.rs
--- a/src/operation/list_account_roles/_list_account_roles_input.rs
+++ b/src/operation/list_account_roles/_list_account_roles_input.rs
@@ -118,7 +118,10 @@
     /// Consumes the builder and constructs a [`ListAccountRolesInput`](crate::operation::list_account_roles::ListAccountRolesInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<crate::operation::list_account_roles::ListAccountRolesInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        crate::operation::list_account_roles::ListAccountRolesInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(crate::operation::list_account_roles::ListAccountRolesInput {
             next_token: self.next_token,
             max_results: self.max_results,
```

### `src/operation/list_account_roles/builders.rs`

```diff
diff --git a/src/operation/list_account_roles/builders.rs b/src/operation/list_account_roles/builders.rs
--- a/src/operation/list_account_roles/builders.rs
+++ b/src/operation/list_account_roles/builders.rs
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

### `src/operation/list_account_roles/paginator.rs`

```diff
diff --git a/src/operation/list_account_roles/paginator.rs b/src/operation/list_account_roles/paginator.rs
--- a/src/operation/list_account_roles/paginator.rs
+++ b/src/operation/list_account_roles/paginator.rs
@@ -71,8 +71,8 @@
             ::std::option::Option::None,
         )
         .with_operation_plugin(crate::sdk_feature_tracker::paginator::PaginatorFeatureTrackerRuntimePlugin::new());
-        ::aws_smithy_async::future::pagination_stream::PaginationStream::new(::aws_smithy_async::future::pagination_stream::fn_stream::FnStream::new(
-            move |tx| {
+        ::aws_smithy_async::future::pagination_stream::PaginationStream::new(
+            ::aws_smithy_async::future::pagination_stream::fn_stream::FnStream::new(move |tx| {
                 ::std::boxed::Box::pin(async move {
                     // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                     let mut input = match builder
@@ -86,7 +86,11 @@
                         }
                     };
                     loop {
-                        let resp = crate::operation::list_account_roles::ListAccountRoles::orchestrate(&runtime_plugins, input.clone()).await;
+                        let resp = crate::operation::list_account_roles::ListAccountRoles::orchestrate(
+                            &runtime_plugins,
+                            input.clone(),
+                        )
+                        .await;
                         // If the input member is None or it was an error
                         let done = match resp {
                             ::std::result::Result::Ok(ref resp) => {
@@ -111,8 +115,8 @@
                         }
                     }
                 })
-            },
-        ))
+            }),
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
@@ -91,19 +101,24 @@
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
             ListAccountsRequestSerializer,
         ));
-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            ListAccountsResponseDeserializer,
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(ListAccountsResponseDeserializer),
+        );
+
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
+                crate::config::auth::Params::builder()
+                    .operation_name("ListAccounts")
+                    .build()
+                    .expect("required fields set"),
+            ),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
-            crate::config::auth::Params::builder()
-                .operation_name("ListAccounts")
-                .build()
-                .expect("required fields set"),
+        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
+            "ListAccounts",
+            "SSO",
         ));

-        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("ListAccounts", "SSO"));
-
         ::std::option::Option::Some(cfg.freeze())
     }

@@ -112,25 +127,37 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ListAccounts")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new(
+            "ListAccounts",
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ListAccountsTelemetryInputCaptureInterceptor,
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
                 ListAccountsEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::list_accounts::ListAccountsError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+            ),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 crate::operation::list_accounts::ListAccountsError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::list_accounts::ListAccountsError,
-            >::new());
+            >::new(),
+        )
+        .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+            crate::operation::list_accounts::ListAccountsError,
+        >::new());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -204,12 +231,20 @@
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
@@ -239,7 +274,10 @@
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
@@ -248,7 +286,8 @@
             fn update_http_builder(
                 input: &crate::operation::list_accounts::ListAccountsInput,
                 builder: ::http_1x::request::Builder,
-            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError>
+            {
                 let mut uri = ::std::string::String::new();
                 uri_base(input, &mut uri)?;
                 uri_query(input, &mut uri)?;
@@ -291,13 +330,21 @@
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
@@ -318,18 +365,22 @@
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
+            ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
+        >,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
             source: err.into(),
@@ -393,7 +444,9 @@
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
@@ -413,10 +466,18 @@
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
@@ -68,7 +68,10 @@
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
```

### `src/operation/list_accounts/builders.rs`

```diff
diff --git a/src/operation/list_accounts/builders.rs b/src/operation/list_accounts/builders.rs
--- a/src/operation/list_accounts/builders.rs
+++ b/src/operation/list_accounts/builders.rs
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

### `src/operation/list_accounts/paginator.rs`

```diff
diff --git a/src/operation/list_accounts/paginator.rs b/src/operation/list_accounts/paginator.rs
--- a/src/operation/list_accounts/paginator.rs
+++ b/src/operation/list_accounts/paginator.rs
@@ -71,8 +71,8 @@
             ::std::option::Option::None,
         )
         .with_operation_plugin(crate::sdk_feature_tracker::paginator::PaginatorFeatureTrackerRuntimePlugin::new());
-        ::aws_smithy_async::future::pagination_stream::PaginationStream::new(::aws_smithy_async::future::pagination_stream::fn_stream::FnStream::new(
-            move |tx| {
+        ::aws_smithy_async::future::pagination_stream::PaginationStream::new(
+            ::aws_smithy_async::future::pagination_stream::fn_stream::FnStream::new(move |tx| {
                 ::std::boxed::Box::pin(async move {
                     // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                     let mut input = match builder
@@ -86,7 +86,9 @@
                         }
                     };
                     loop {
-                        let resp = crate::operation::list_accounts::ListAccounts::orchestrate(&runtime_plugins, input.clone()).await;
+                        let resp =
+                            crate::operation::list_accounts::ListAccounts::orchestrate(&runtime_plugins, input.clone())
+                                .await;
                         // If the input member is None or it was an error
                         let done = match resp {
                             ::std::result::Result::Ok(ref resp) => {
@@ -111,8 +113,8 @@
                         }
                     }
                 })
-            },
-        ))
+            }),
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
@@ -83,19 +101,23 @@
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
             LogoutRequestSerializer,
         ));
-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            LogoutResponseDeserializer,
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(LogoutResponseDeserializer),
+        );
+
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
+                crate::config::auth::Params::builder()
+                    .operation_name("Logout")
+                    .build()
+                    .expect("required fields set"),
+            ),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
-            crate::config::auth::Params::builder()
-                .operation_name("Logout")
-                .build()
-                .expect("required fields set"),
+        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
+            "Logout", "SSO",
         ));

-        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("Logout", "SSO"));
-
         ::std::option::Option::Some(cfg.freeze())
     }

@@ -150,13 +172,23 @@
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
@@ -175,7 +207,8 @@
             fn update_http_builder(
                 input: &crate::operation::logout::LogoutInput,
                 builder: ::http_1x::request::Builder,
-            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError>
+            {
                 let mut uri = ::std::string::String::new();
                 uri_base(input, &mut uri)?;
                 let builder = crate::protocol_serde::shape_logout::ser_logout_headers(input, builder)?;
@@ -208,19 +241,30 @@
         >,
         cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
     ) -> ::std::result::Result<(), ::aws_smithy_runtime_api::box_error::BoxError> {
-        let _input = context.input().downcast_ref::<LogoutInput>().ok_or("failed to downcast to LogoutInput")?;
+        let _input = context
+            .input()
+            .downcast_ref::<LogoutInput>()
+            .ok_or("failed to downcast to LogoutInput")?;

         let params = crate::config::endpoint::Params::builder()
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
@@ -239,18 +283,22 @@
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
+            ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
+        >,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
             source: err.into(),
@@ -307,7 +355,9 @@
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
@@ -327,9 +377,15 @@
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
```

### `src/operation/logout/_logout_input.rs`

```diff
diff --git a/src/operation/logout/_logout_input.rs b/src/operation/logout/_logout_input.rs
--- a/src/operation/logout/_logout_input.rs
+++ b/src/operation/logout/_logout_input.rs
@@ -49,7 +49,10 @@
         &self.access_token
     }
     /// Consumes the builder and constructs a [`LogoutInput`](crate::operation::logout::LogoutInput).
-    pub fn build(self) -> ::std::result::Result<crate::operation::logout::LogoutInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<crate::operation::logout::LogoutInput, ::aws_smithy_types::error::operation::BuildError>
+    {
         ::std::result::Result::Ok(crate::operation::logout::LogoutInput {
             access_token: self.access_token,
         })
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
@@ -15,22 +23,41 @@
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
@@ -39,20 +66,48 @@
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
@@ -5,7 +5,12 @@
     depth: u32,
 ) -> ::std::result::Result<Option<crate::types::AccountInfo>, ::aws_smithy_json::deserialize::error::DeserializeError>
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
         return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
@@ -20,34 +25,36 @@
             loop {
                 match tokens.next().transpose()? {
                     Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                        "accountId" => {
-                            builder = builder.set_account_id(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                    .transpose()?,
-                            );
-                        }
-                        "accountName" => {
-                            builder = builder.set_account_name(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                    .transpose()?,
-                            );
-                        }
-                        "emailAddress" => {
-                            builder = builder.set_email_address(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                    .transpose()?,
-                            );
+                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
+                        match key.to_unescaped()?.as_ref() {
+                            "accountId" => {
+                                builder = builder.set_account_id(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                        .transpose()?,
+                                );
+                            }
+                            "accountName" => {
+                                builder = builder.set_account_name(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                        .transpose()?,
+                                );
+                            }
+                            "emailAddress" => {
+                                builder = builder.set_email_address(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                        .transpose()?,
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
+                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                            format!("expected object key or end object, found: {other:?}"),
+                        ))
                     }
                 }
             }
```

### `src/protocol_serde/shape_account_list_type.rs`

```diff
diff --git a/src/protocol_serde/shape_account_list_type.rs b/src/protocol_serde/shape_account_list_type.rs
--- a/src/protocol_serde/shape_account_list_type.rs
+++ b/src/protocol_serde/shape_account_list_type.rs
@@ -3,9 +3,17 @@
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
         return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
@@ -23,7 +31,8 @@
                         break;
                     }
                     _ => {
-                        let value = crate::protocol_serde::shape_account_info::de_account_info(tokens, _value, depth + 1)?;
+                        let value =
+                            crate::protocol_serde::shape_account_info::de_account_info(tokens, _value, depth + 1)?;
                         if let Some(value) = value {
                             items.push(value);
                         } else {
```

### `src/protocol_serde/shape_get_role_credentials.rs`

```diff
diff --git a/src/protocol_serde/shape_get_role_credentials.rs b/src/protocol_serde/shape_get_role_credentials.rs
--- a/src/protocol_serde/shape_get_role_credentials.rs
+++ b/src/protocol_serde/shape_get_role_credentials.rs
@@ -9,8 +9,9 @@
     crate::operation::get_role_credentials::GetRoleCredentialsError,
 > {
     #[allow(unused_mut)]
-    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(crate::operation::get_role_credentials::GetRoleCredentialsError::unhandled)?;
+    let mut generic_builder =
+        crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
+            .map_err(crate::operation::get_role_credentials::GetRoleCredentialsError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
@@ -20,66 +21,81 @@

     let _error_message = generic.message().map(|msg| msg.to_owned());
     Err(match error_code {
-        "InvalidRequestException" => crate::operation::get_role_credentials::GetRoleCredentialsError::InvalidRequestException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "InvalidRequestException" => {
+            crate::operation::get_role_credentials::GetRoleCredentialsError::InvalidRequestException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::InvalidRequestExceptionBuilder::default();
-                output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(_response_body, output)
-                    .map_err(crate::operation::get_role_credentials::GetRoleCredentialsError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "ResourceNotFoundException" => crate::operation::get_role_credentials::GetRoleCredentialsError::ResourceNotFoundException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::InvalidRequestExceptionBuilder::default();
+                    output =
+                        crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(
+                            _response_body,
+                            output,
+                        )
+                        .map_err(crate::operation::get_role_credentials::GetRoleCredentialsError::unhandled)?;
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
+            crate::operation::get_role_credentials::GetRoleCredentialsError::ResourceNotFoundException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
-                output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                     .map_err(crate::operation::get_role_credentials::GetRoleCredentialsError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "TooManyRequestsException" => crate::operation::get_role_credentials::GetRoleCredentialsError::TooManyRequestsException({
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
+            crate::operation::get_role_credentials::GetRoleCredentialsError::TooManyRequestsException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
-                output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(crate::operation::get_role_credentials::GetRoleCredentialsError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "UnauthorizedException" => crate::operation::get_role_credentials::GetRoleCredentialsError::UnauthorizedException({
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
+            crate::operation::get_role_credentials::GetRoleCredentialsError::UnauthorizedException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::UnauthorizedExceptionBuilder::default();
-                output = crate::protocol_serde::shape_unauthorized_exception::de_unauthorized_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::UnauthorizedExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_unauthorized_exception::de_unauthorized_exception_json_err(
+                        _response_body,
+                        output,
+                    )
                     .map_err(crate::operation::get_role_credentials::GetRoleCredentialsError::unhandled)?;
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
         _ => crate::operation::get_role_credentials::GetRoleCredentialsError::generic(generic),
     })
 }
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
@@ -128,7 +147,8 @@
     crate::operation::get_role_credentials::builders::GetRoleCredentialsOutputBuilder,
     ::aws_smithy_json::deserialize::error::DeserializeError,
 > {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+    let mut tokens_owned =
+        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -138,18 +158,16 @@
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                 "roleCredentials" => {
-                    builder = builder.set_role_credentials(crate::protocol_serde::shape_role_credentials::de_role_credentials(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
+                    builder = builder.set_role_credentials(
+                        crate::protocol_serde::shape_role_credentials::de_role_credentials(tokens, _value, depth + 1)?,
+                    );
                 }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
-                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
-                    "expected object key or end object, found: {other:?}"
-                )))
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                    format!("expected object key or end object, found: {other:?}"),
+                ))
             }
         }
     }
```

### `src/protocol_serde/shape_invalid_request_exception.rs`

```diff
diff --git a/src/protocol_serde/shape_invalid_request_exception.rs b/src/protocol_serde/shape_invalid_request_exception.rs
--- a/src/protocol_serde/shape_invalid_request_exception.rs
+++ b/src/protocol_serde/shape_invalid_request_exception.rs
@@ -2,8 +2,12 @@
 pub(crate) fn de_invalid_request_exception_json_err(
     _value: &[u8],
     mut builder: crate::types::error::builders::InvalidRequestExceptionBuilder,
-) -> ::std::result::Result<crate::types::error::builders::InvalidRequestExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+) -> ::std::result::Result<
+    crate::types::error::builders::InvalidRequestExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned =
+        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -22,9 +26,9 @@
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
-                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
-                    "expected object key or end object, found: {other:?}"
-                )))
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                    format!("expected object key or end object, found: {other:?}"),
+                ))
             }
         }
     }
```

### `src/protocol_serde/shape_list_account_roles.rs`

```diff
diff --git a/src/protocol_serde/shape_list_account_roles.rs b/src/protocol_serde/shape_list_account_roles.rs
--- a/src/protocol_serde/shape_list_account_roles.rs
+++ b/src/protocol_serde/shape_list_account_roles.rs
@@ -4,79 +4,102 @@
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
+    let mut generic_builder =
+        crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
+            .map_err(crate::operation::list_account_roles::ListAccountRolesError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(crate::operation::list_account_roles::ListAccountRolesError::unhandled(generic)),
+        None => {
+            return Err(crate::operation::list_account_roles::ListAccountRolesError::unhandled(
+                generic,
+            ))
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
-                    .map_err(crate::operation::list_account_roles::ListAccountRolesError::unhandled)?;
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
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::InvalidRequestExceptionBuilder::default();
+                    output =
+                        crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(
+                            _response_body,
+                            output,
+                        )
+                        .map_err(crate::operation::list_account_roles::ListAccountRolesError::unhandled)?;
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
+            crate::operation::list_account_roles::ListAccountRolesError::ResourceNotFoundException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
-                output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
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
+            crate::operation::list_account_roles::ListAccountRolesError::TooManyRequestsException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
-                output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
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
+                    let mut output = crate::types::error::builders::UnauthorizedExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_unauthorized_exception::de_unauthorized_exception_json_err(
+                        _response_body,
+                        output,
+                    )
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
@@ -86,7 +109,10 @@
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
         let mut output = crate::operation::list_account_roles::builders::ListAccountRolesOutputBuilder::default();
@@ -107,7 +133,10 @@
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
@@ -122,7 +151,8 @@
     crate::operation::list_account_roles::builders::ListAccountRolesOutputBuilder,
     ::aws_smithy_json::deserialize::error::DeserializeError,
 > {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+    let mut tokens_owned =
+        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -130,23 +160,27 @@
     loop {
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "nextToken" => {
-                    builder = builder.set_next_token(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
-                }
-                "roleList" => {
-                    builder = builder.set_role_list(crate::protocol_serde::shape_role_list_type::de_role_list_type(tokens, _value, depth + 1)?);
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
+                match key.to_unescaped()?.as_ref() {
+                    "nextToken" => {
+                        builder = builder.set_next_token(
+                            ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                .transpose()?,
+                        );
+                    }
+                    "roleList" => {
+                        builder = builder.set_role_list(
+                            crate::protocol_serde::shape_role_list_type::de_role_list_type(tokens, _value, depth + 1)?,
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
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                    format!("expected object key or end object, found: {other:?}"),
+                ))
             }
         }
     }
```

### `src/protocol_serde/shape_list_accounts.rs`

```diff
diff --git a/src/protocol_serde/shape_list_accounts.rs b/src/protocol_serde/shape_list_accounts.rs
--- a/src/protocol_serde/shape_list_accounts.rs
+++ b/src/protocol_serde/shape_list_accounts.rs
@@ -4,10 +4,14 @@
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
+    let mut generic_builder =
+        crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
+            .map_err(crate::operation::list_accounts::ListAccountsError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
@@ -22,8 +26,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = crate::types::error::builders::InvalidRequestExceptionBuilder::default();
-                output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(_response_body, output)
-                    .map_err(crate::operation::list_accounts::ListAccountsError::unhandled)?;
+                output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(crate::operation::list_accounts::ListAccountsError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -47,28 +54,33 @@
             }
             tmp
         }),
-        "TooManyRequestsException" => crate::operation::list_accounts::ListAccountsError::TooManyRequestsException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "TooManyRequestsException" => {
+            crate::operation::list_accounts::ListAccountsError::TooManyRequestsException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
-                output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
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
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
         "UnauthorizedException" => crate::operation::list_accounts::ListAccountsError::UnauthorizedException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = crate::types::error::builders::UnauthorizedExceptionBuilder::default();
-                output = crate::protocol_serde::shape_unauthorized_exception::de_unauthorized_exception_json_err(_response_body, output)
-                    .map_err(crate::operation::list_accounts::ListAccountsError::unhandled)?;
+                output = crate::protocol_serde::shape_unauthorized_exception::de_unauthorized_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(crate::operation::list_accounts::ListAccountsError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -86,7 +98,10 @@
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
         let mut output = crate::operation::list_accounts::builders::ListAccountsOutputBuilder::default();
@@ -107,7 +122,10 @@
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
@@ -122,7 +140,8 @@
     crate::operation::list_accounts::builders::ListAccountsOutputBuilder,
     ::aws_smithy_json::deserialize::error::DeserializeError,
 > {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+    let mut tokens_owned =
+        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -132,11 +151,12 @@
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                 "accountList" => {
-                    builder = builder.set_account_list(crate::protocol_serde::shape_account_list_type::de_account_list_type(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
+                    builder =
+                        builder.set_account_list(crate::protocol_serde::shape_account_list_type::de_account_list_type(
+                            tokens,
+                            _value,
+                            depth + 1,
+                        )?);
                 }
                 "nextToken" => {
                     builder = builder.set_next_token(
@@ -148,9 +168,9 @@
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
-                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
-                    "expected object key or end object, found: {other:?}"
-                )))
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                    format!("expected object key or end object, found: {other:?}"),
+                ))
             }
         }
     }
```

### `src/protocol_serde/shape_logout.rs`

```diff
diff --git a/src/protocol_serde/shape_logout.rs b/src/protocol_serde/shape_logout.rs
--- a/src/protocol_serde/shape_logout.rs
+++ b/src/protocol_serde/shape_logout.rs
@@ -6,8 +6,9 @@
     _response_body: &[u8],
 ) -> std::result::Result<crate::operation::logout::LogoutOutput, crate::operation::logout::LogoutError> {
     #[allow(unused_mut)]
-    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(crate::operation::logout::LogoutError::unhandled)?;
+    let mut generic_builder =
+        crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
+            .map_err(crate::operation::logout::LogoutError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
@@ -22,8 +23,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = crate::types::error::builders::InvalidRequestExceptionBuilder::default();
-                output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(_response_body, output)
-                    .map_err(crate::operation::logout::LogoutError::unhandled)?;
+                output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(crate::operation::logout::LogoutError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -32,28 +36,33 @@
             }
             tmp
         }),
-        "TooManyRequestsException" => crate::operation::logout::LogoutError::TooManyRequestsException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "TooManyRequestsException" => {
+            crate::operation::logout::LogoutError::TooManyRequestsException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
-                output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
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
                 let mut output = crate::types::error::builders::UnauthorizedExceptionBuilder::default();
-                output = crate::protocol_serde::shape_unauthorized_exception::de_unauthorized_exception_json_err(_response_body, output)
-                    .map_err(crate::operation::logout::LogoutError::unhandled)?;
+                output = crate::protocol_serde::shape_unauthorized_exception::de_unauthorized_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(crate::operation::logout::LogoutError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -90,7 +99,10 @@
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
@@ -2,8 +2,12 @@
 pub(crate) fn de_resource_not_found_exception_json_err(
     _value: &[u8],
     mut builder: crate::types::error::builders::ResourceNotFoundExceptionBuilder,
-) -> ::std::result::Result<crate::types::error::builders::ResourceNotFoundExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+) -> ::std::result::Result<
+    crate::types::error::builders::ResourceNotFoundExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned =
+        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -22,9 +26,9 @@
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
-                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
-                    "expected object key or end object, found: {other:?}"
-                )))
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                    format!("expected object key or end object, found: {other:?}"),
+                ))
             }
         }
     }
```

### `src/protocol_serde/shape_role_credentials.rs`

```diff
diff --git a/src/protocol_serde/shape_role_credentials.rs b/src/protocol_serde/shape_role_credentials.rs
--- a/src/protocol_serde/shape_role_credentials.rs
+++ b/src/protocol_serde/shape_role_credentials.rs
@@ -5,7 +5,12 @@
     depth: u32,
 ) -> ::std::result::Result<Option<crate::types::RoleCredentials>, ::aws_smithy_json::deserialize::error::DeserializeError>
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
         return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
@@ -20,41 +25,43 @@
             loop {
                 match tokens.next().transpose()? {
                     Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                        "accessKeyId" => {
-                            builder = builder.set_access_key_id(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                    .transpose()?,
-                            );
-                        }
-                        "secretAccessKey" => {
-                            builder = builder.set_secret_access_key(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                    .transpose()?,
-                            );
-                        }
-                        "sessionToken" => {
-                            builder = builder.set_session_token(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                    .transpose()?,
-                            );
-                        }
-                        "expiration" => {
-                            builder = builder.set_expiration(
-                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
-                                    .map(i64::try_from)
-                                    .transpose()?,
-                            );
+                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
+                        match key.to_unescaped()?.as_ref() {
+                            "accessKeyId" => {
+                                builder = builder.set_access_key_id(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                        .transpose()?,
+                                );
+                            }
+                            "secretAccessKey" => {
+                                builder = builder.set_secret_access_key(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                        .transpose()?,
+                                );
+                            }
+                            "sessionToken" => {
+                                builder = builder.set_session_token(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                        .transpose()?,
+                                );
+                            }
+                            "expiration" => {
+                                builder = builder.set_expiration(
+                                    ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                                        .map(i64::try_from)
+                                        .transpose()?,
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
+                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                            format!("expected object key or end object, found: {other:?}"),
+                        ))
                     }
                 }
             }
```

### `src/protocol_serde/shape_role_info.rs`

```diff
diff --git a/src/protocol_serde/shape_role_info.rs b/src/protocol_serde/shape_role_info.rs
--- a/src/protocol_serde/shape_role_info.rs
+++ b/src/protocol_serde/shape_role_info.rs
@@ -5,7 +5,12 @@
     depth: u32,
 ) -> ::std::result::Result<Option<crate::types::RoleInfo>, ::aws_smithy_json::deserialize::error::DeserializeError>
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
         return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
@@ -20,27 +25,29 @@
             loop {
                 match tokens.next().transpose()? {
                     Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                        "roleName" => {
-                            builder = builder.set_role_name(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                    .transpose()?,
-                            );
-                        }
-                        "accountId" => {
-                            builder = builder.set_account_id(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                    .transpose()?,
-                            );
+                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
+                        match key.to_unescaped()?.as_ref() {
+                            "roleName" => {
+                                builder = builder.set_role_name(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                        .transpose()?,
+                                );
+                            }
+                            "accountId" => {
+                                builder = builder.set_account_id(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                        .transpose()?,
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
+                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                            format!("expected object key or end object, found: {other:?}"),
+                        ))
                     }
                 }
             }
```

### `src/protocol_serde/shape_role_list_type.rs`

```diff
diff --git a/src/protocol_serde/shape_role_list_type.rs b/src/protocol_serde/shape_role_list_type.rs
--- a/src/protocol_serde/shape_role_list_type.rs
+++ b/src/protocol_serde/shape_role_list_type.rs
@@ -3,9 +3,17 @@
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
         return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
```

### `src/protocol_serde/shape_too_many_requests_exception.rs`

```diff
diff --git a/src/protocol_serde/shape_too_many_requests_exception.rs b/src/protocol_serde/shape_too_many_requests_exception.rs
--- a/src/protocol_serde/shape_too_many_requests_exception.rs
+++ b/src/protocol_serde/shape_too_many_requests_exception.rs
@@ -2,8 +2,12 @@
 pub(crate) fn de_too_many_requests_exception_json_err(
     _value: &[u8],
     mut builder: crate::types::error::builders::TooManyRequestsExceptionBuilder,
-) -> ::std::result::Result<crate::types::error::builders::TooManyRequestsExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+) -> ::std::result::Result<
+    crate::types::error::builders::TooManyRequestsExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned =
+        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -22,9 +26,9 @@
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
-                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
-                    "expected object key or end object, found: {other:?}"
-                )))
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                    format!("expected object key or end object, found: {other:?}"),
+                ))
             }
         }
     }
```

### `src/protocol_serde/shape_unauthorized_exception.rs`

```diff
diff --git a/src/protocol_serde/shape_unauthorized_exception.rs b/src/protocol_serde/shape_unauthorized_exception.rs
--- a/src/protocol_serde/shape_unauthorized_exception.rs
+++ b/src/protocol_serde/shape_unauthorized_exception.rs
@@ -2,8 +2,12 @@
 pub(crate) fn de_unauthorized_exception_json_err(
     _value: &[u8],
     mut builder: crate::types::error::builders::UnauthorizedExceptionBuilder,
-) -> ::std::result::Result<crate::types::error::builders::UnauthorizedExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+) -> ::std::result::Result<
+    crate::types::error::builders::UnauthorizedExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned =
+        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -22,9 +26,9 @@
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
-                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
-                    "expected object key or end object, found: {other:?}"
-                )))
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                    format!("expected object key or end object, found: {other:?}"),
+                ))
             }
         }
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
@@ -117,10 +117,14 @@
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
diff --git a/src/client/get_role_credentials.rs b/src/client/get_role_credentials.rs
--- a/src/client/get_role_credentials.rs
+++ b/src/client/get_role_credentials.rs
@@ -9,7 +9,9 @@
     /// - On success, responds with [`GetRoleCredentialsOutput`](crate::operation::get_role_credentials::GetRoleCredentialsOutput) with field(s):
     ///   - [`role_credentials(Option<RoleCredentials>)`](crate::operation::get_role_credentials::GetRoleCredentialsOutput::role_credentials): <p>The credentials for the role that is assigned to the user.</p>
     /// - On failure, responds with [`SdkError<GetRoleCredentialsError>`](crate::operation::get_role_credentials::GetRoleCredentialsError)
-    pub fn get_role_credentials(&self) -> crate::operation::get_role_credentials::builders::GetRoleCredentialsFluentBuilder {
+    pub fn get_role_credentials(
+        &self,
+    ) -> crate::operation::get_role_credentials::builders::GetRoleCredentialsFluentBuilder {
         crate::operation::get_role_credentials::builders::GetRoleCredentialsFluentBuilder::new(self.handle.clone())
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
@@ -193,18 +209,42 @@
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
         builder.set_region(config_bag.load::<crate::config::Region>().cloned());
         builder
     }
@@ -220,7 +260,10 @@
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
@@ -253,7 +296,10 @@
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
@@ -296,7 +342,9 @@
     /// # }
     /// ```
     pub fn http_client(mut self, http_client: impl crate::config::HttpClient + 'static) -> Self {
-        self.set_http_client(::std::option::Option::Some(crate::config::IntoShared::into_shared(http_client)));
+        self.set_http_client(::std::option::Option::Some(crate::config::IntoShared::into_shared(
+            http_client,
+        )));
         self
     }

@@ -441,7 +489,10 @@
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
@@ -476,7 +527,10 @@
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
@@ -485,7 +539,10 @@
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
@@ -505,7 +562,9 @@
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
@@ -592,7 +651,10 @@
     /// let config = aws_sdk_sso::Config::builder().endpoint_resolver(resolver).build();
     /// let client = aws_sdk_sso::Client::from_conf(config);
     /// ```
-    pub fn endpoint_resolver(mut self, endpoint_resolver: impl crate::config::endpoint::ResolveEndpoint + 'static) -> Self {
+    pub fn endpoint_resolver(
+        mut self,
+        endpoint_resolver: impl crate::config::endpoint::ResolveEndpoint + 'static,
+    ) -> Self {
         self.set_endpoint_resolver(::std::option::Option::Some(endpoint_resolver.into_shared_resolver()));
         self
     }
@@ -650,7 +712,10 @@
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
@@ -674,7 +739,9 @@
     /// let config = Config::builder().sleep_impl(sleep_impl).build();
     /// ```
     pub fn sleep_impl(mut self, sleep_impl: impl crate::config::AsyncSleep + 'static) -> Self {
-        self.set_sleep_impl(Some(::aws_smithy_runtime_api::shared::IntoShared::into_shared(sleep_impl)));
+        self.set_sleep_impl(Some(::aws_smithy_runtime_api::shared::IntoShared::into_shared(
+            sleep_impl,
+        )));
         self
     }

@@ -750,9 +817,14 @@
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
@@ -921,8 +993,12 @@
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
@@ -984,19 +1060,25 @@
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
@@ -1091,9 +1173,9 @@
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

@@ -1110,9 +1192,12 @@
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
@@ -1153,7 +1238,10 @@
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
@@ -1160,7 +1248,10 @@
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
@@ -1191,7 +1282,8 @@
     /// will be prefixed onto this URL. To fully override the endpoint resolver, use
     /// [`Builder::endpoint_resolver`].
     pub fn set_endpoint_url(&mut self, endpoint_url: Option<::std::string::String>) -> &mut Self {
-        self.config.store_or_unset(endpoint_url.map(::aws_types::endpoint_config::EndpointUrl));
+        self.config
+            .store_or_unset(endpoint_url.map(::aws_types::endpoint_config::EndpointUrl));
         self
     }
     /// When true, use the dual-stack endpoint. If the configured endpoint does not support dual-stack, dispatching the request MAY return an error.
@@ -1201,7 +1293,8 @@
     }
     /// When true, use the dual-stack endpoint. If the configured endpoint does not support dual-stack, dispatching the request MAY return an error.
     pub fn set_use_dual_stack(&mut self, use_dual_stack: Option<bool>) -> &mut Self {
-        self.config.store_or_unset(use_dual_stack.map(::aws_types::endpoint_config::UseDualStack));
+        self.config
+            .store_or_unset(use_dual_stack.map(::aws_types::endpoint_config::UseDualStack));
         self
     }
     /// When true, send this request to the FIPS-compliant regional endpoint. If the configured endpoint does not have a FIPS compliant endpoint, dispatching the request will return an error.
@@ -1211,7 +1304,8 @@
     }
     /// When true, send this request to the FIPS-compliant regional endpoint. If the configured endpoint does not have a FIPS compliant endpoint, dispatching the request will return an error.
     pub fn set_use_fips(&mut self, use_fips: Option<bool>) -> &mut Self {
-        self.config.store_or_unset(use_fips.map(::aws_types::endpoint_config::UseFips));
+        self.config
+            .store_or_unset(use_fips.map(::aws_types::endpoint_config::UseFips));
         self
     }
     /// Sets the AWS region to use when making requests.
@@ -1235,14 +1329,20 @@
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
@@ -1342,10 +1442,13 @@
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
@@ -1414,7 +1517,8 @@
             cfg.store_put(::aws_smithy_runtime::client::orchestrator::AuthSchemeAndEndpointOrchestrationV2);
             ::std::option::Option::Some(cfg.freeze())
         };
-        let mut runtime_components = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ServiceRuntimePlugin");
+        let mut runtime_components =
+            ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ServiceRuntimePlugin");
         runtime_components.set_auth_scheme_option_resolver(::std::option::Option::Some({
             use crate::config::auth::ResolveAuthScheme;
             crate::config::auth::DefaultAuthSchemeResolver::default().into_shared_resolver()
@@ -1423,32 +1527,49 @@
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

@@ -1491,14 +1612,22 @@
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

@@ -1511,7 +1640,9 @@
         // This is skipped if the override already sets its own identity cache.
         if components.has_identity_resolvers() && components.identity_cache().is_none() {
             components.set_identity_cache(::std::option::Option::Some(
-                ::aws_smithy_runtime::client::identity::IdentityCache::lazy().max_partitions(1).build(),
+                ::aws_smithy_runtime::client::identity::IdentityCache::lazy()
+                    .max_partitions(1)
+                    .build(),
             ));
         }

@@ -1599,7 +1730,11 @@
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
@@ -1610,13 +1745,16 @@

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
@@ -104,7 +107,8 @@
             None => &self.service_defaults,
         };

-        let _fut = ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionsFuture::ready(Ok(modeled_auth_options.clone()));
+        let _fut =
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionsFuture::ready(Ok(modeled_auth_options.clone()));

         _fut
     }
@@ -139,7 +143,10 @@
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
@@ -149,7 +156,9 @@
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
@@ -480,7 +482,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso-fips.cn-north-1.api.amazonwebservices.com.cn");
+        let endpoint =
+            endpoint.expect("Expected valid endpoint: https://portal.sso-fips.cn-north-1.api.amazonwebservices.com.cn");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -520,7 +523,8 @@
             .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
-        let endpoint = endpoint.expect("Expected valid endpoint: https://portal.sso.cn-north-1.api.amazonwebservices.com.cn");
+        let endpoint =
+            endpoint.expect("Expected valid endpoint: https://portal.sso.cn-north-1.api.amazonwebservices.com.cn");
         assert_eq!(
             endpoint,
             ::aws_smithy_types::endpoint::Endpoint::builder()
@@ -744,7 +748,9 @@
         let endpoint = endpoint.expect("Expected valid endpoint: https://example.com");
         assert_eq!(
             endpoint,
-            ::aws_smithy_types::endpoint::Endpoint::builder().url("https://example.com").build()
+            ::aws_smithy_types::endpoint::Endpoint::builder()
+                .url("https://example.com")
+                .build()
         );
     }

@@ -762,7 +768,9 @@
         let endpoint = endpoint.expect("Expected valid endpoint: https://example.com");
         assert_eq!(
             endpoint,
-            ::aws_smithy_types::endpoint::Endpoint::builder().url("https://example.com").build()
+            ::aws_smithy_types::endpoint::Endpoint::builder()
+                .url("https://example.com")
+                .build()
         );
     }

@@ -779,7 +787,10 @@
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
@@ -804,7 +815,9 @@
     /// Missing region
     #[test]
     fn test_40() {
-        let params = crate::config::endpoint::Params::builder().build().expect("invalid params");
+        let params = crate::config::endpoint::Params::builder()
+            .build()
+            .expect("invalid params");
         let resolver = crate::config::endpoint::DefaultResolver::new();
         let endpoint = resolver.resolve_endpoint(&params);
         let error = endpoint.expect_err("expected error: Invalid Configuration: Missing Region [Missing region]");
@@ -815,7 +828,10 @@
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
@@ -840,7 +856,9 @@
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
@@ -884,7 +902,8 @@
     fn resolve_endpoint<'a>(
         &'a self,
         params: &'a crate::config::endpoint::Params,
-    ) -> ::std::result::Result<::aws_smithy_types::endpoint::Endpoint, ::aws_smithy_runtime_api::box_error::BoxError> {
+    ) -> ::std::result::Result<::aws_smithy_types::endpoint::Endpoint, ::aws_smithy_runtime_api::box_error::BoxError>
+    {
         let mut _diagnostic_collector = crate::endpoint_lib::diagnostic::DiagnosticCollector::new();
         #[allow(unused_mut)]
         let mut context = ConditionContext::default();
@@ -900,22 +919,36 @@
             match current_ref {
                 ref_val if ref_val >= 100_000_000 => {
                     return match (ref_val - 100_000_000) as usize {
-                        0 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
-                            "No endpoint rule matched",
-                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
-                        1 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
-                            "Invalid Configuration: FIPS and custom endpoint are not supported".to_string(),
-                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
-                        2 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
-                            "Invalid Configuration: Dualstack and custom endpoint are not supported".to_string(),
-                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        0 => ::std::result::Result::Err(Box::new(
+                            ::aws_smithy_http::endpoint::ResolveEndpointError::message("No endpoint rule matched"),
+                        )
+                            as ::aws_smithy_runtime_api::box_error::BoxError),
+                        1 => ::std::result::Result::Err(Box::new(
+                            ::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                                "Invalid Configuration: FIPS and custom endpoint are not supported".to_string(),
+                            ),
+                        )
+                            as ::aws_smithy_runtime_api::box_error::BoxError),
+                        2 => ::std::result::Result::Err(Box::new(
+                            ::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                                "Invalid Configuration: Dualstack and custom endpoint are not supported".to_string(),
+                            ),
+                        )
+                            as ::aws_smithy_runtime_api::box_error::BoxError),
                         3 => {
                             let endpoint = params.endpoint.as_deref().unwrap_or_default();
-                            ::std::result::Result::Ok(::aws_smithy_types::endpoint::Endpoint::builder().url(endpoint.to_owned()).build())
+                            ::std::result::Result::Ok(
+                                ::aws_smithy_types::endpoint::Endpoint::builder()
+                                    .url(endpoint.to_owned())
+                                    .build(),
+                            )
                         }
                         4 => {
                             let region = params.region.as_deref().unwrap_or_default();
-                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            let partition_result = context
+                                .partition_result
+                                .as_ref()
+                                .expect("Guaranteed to have a value by earlier checks.");
                             ::std::result::Result::Ok(
                                 ::aws_smithy_types::endpoint::Endpoint::builder()
                                     .url({
@@ -931,9 +964,13 @@
                                     .build(),
                             )
                         }
-                        5 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
-                            "FIPS and DualStack are enabled, but this partition does not support one or both".to_string(),
-                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        5 => ::std::result::Result::Err(Box::new(
+                            ::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                                "FIPS and DualStack are enabled, but this partition does not support one or both"
+                                    .to_string(),
+                            ),
+                        )
+                            as ::aws_smithy_runtime_api::box_error::BoxError),
                         6 => {
                             let region = params.region.as_deref().unwrap_or_default();
                             ::std::result::Result::Ok(
@@ -951,7 +988,10 @@
                         }
                         7 => {
                             let region = params.region.as_deref().unwrap_or_default();
-                            let partition_result = context.partition_result.as_ref().expect("Guaranteed to have a value by earlier checks.");
+                            let partition_result = context
+                                .partition_result
+                                .as_ref()
+                                .expect("Guaranteed to have a value by earlier checks.");
                             ::std::result::Result::Ok(
                                 ::aws_smithy_types::endpoint::Endpoint::builder()
                                     .url({
@@ -967,12 +1007,18 @@
                                     .build(),
                             )
                         }
-                        8 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
-                            "FIPS is enabled but this partition does not support FIPS".to_string(),
-                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        8 => ::std::result::Result::Err(Box::new(
+                            ::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                                "FIPS is enabled but this partition does not support FIPS".to_string(),
+                            ),
+                        )
+                            as ::aws_smithy_runtime_api::box_error::BoxError),
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
@@ -988,12 +1034,18 @@
                                     .build(),
                             )
                         }
-                        10 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
-                            "DualStack is enabled but this partition does not support DualStack".to_string(),
-                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        10 => ::std::result::Result::Err(Box::new(
+                            ::aws_smithy_http::endpoint::ResolveEndpointError::message(
+                                "DualStack is enabled but this partition does not support DualStack".to_string(),
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
@@ -1009,19 +1061,23 @@
                                     .build(),
                             )
                         }
-                        12 => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
-                            "Invalid Configuration: Missing Region".to_string(),
-                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
-                        _ => ::std::result::Result::Err(Box::new(::aws_smithy_http::endpoint::ResolveEndpointError::message(
-                            "No endpoint rule matched",
-                        )) as ::aws_smithy_runtime_api::box_error::BoxError),
+                        12 => ::std::result::Result::Err(Box::new(
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
@@ -1070,7 +1126,11 @@
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
@@ -1078,18 +1138,24 @@
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
@@ -1222,7 +1288,9 @@
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
@@ -1353,7 +1421,9 @@
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
@@ -48,19 +98,54 @@
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
             ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
             _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
@@ -70,10 +155,13 @@
         }
     }
 }
+#[cfg(feature = "op_get_role_credentials")]
 impl From<crate::operation::get_role_credentials::GetRoleCredentialsError> for Error {
     fn from(err: crate::operation::get_role_credentials::GetRoleCredentialsError) -> Self {
         match err {
-            crate::operation::get_role_credentials::GetRoleCredentialsError::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
+            crate::operation::get_role_credentials::GetRoleCredentialsError::InvalidRequestException(inner) => {
+                Error::InvalidRequestException(inner)
+            }
             crate::operation::get_role_credentials::GetRoleCredentialsError::ResourceNotFoundException(inner) => {
                 Error::ResourceNotFoundException(inner)
             }
@@ -80,16 +168,32 @@
             crate::operation::get_role_credentials::GetRoleCredentialsError::TooManyRequestsException(inner) => {
                 Error::TooManyRequestsException(inner)
             }
-            crate::operation::get_role_credentials::GetRoleCredentialsError::UnauthorizedException(inner) => Error::UnauthorizedException(inner),
-            crate::operation::get_role_credentials::GetRoleCredentialsError::Unhandled(inner) => Error::Unhandled(inner),
+            crate::operation::get_role_credentials::GetRoleCredentialsError::UnauthorizedException(inner) => {
+                Error::UnauthorizedException(inner)
+            }
+            crate::operation::get_role_credentials::GetRoleCredentialsError::Unhandled(inner) => {
+                Error::Unhandled(inner)
+            }
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
             ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
             _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
@@ -99,22 +203,35 @@
         }
     }
 }
+#[cfg(feature = "op_list_account_roles")]
 impl From<crate::operation::list_account_roles::ListAccountRolesError> for Error {
     fn from(err: crate::operation::list_account_roles::ListAccountRolesError) -> Self {
         match err {
-            crate::operation::list_account_roles::ListAccountRolesError::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
-            crate::operation::list_account_roles::ListAccountRolesError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
-            crate::operation::list_account_roles::ListAccountRolesError::TooManyRequestsException(inner) => Error::TooManyRequestsException(inner),
-            crate::operation::list_account_roles::ListAccountRolesError::UnauthorizedException(inner) => Error::UnauthorizedException(inner),
+            crate::operation::list_account_roles::ListAccountRolesError::InvalidRequestException(inner) => {
+                Error::InvalidRequestException(inner)
+            }
+            crate::operation::list_account_roles::ListAccountRolesError::ResourceNotFoundException(inner) => {
+                Error::ResourceNotFoundException(inner)
+            }
+            crate::operation::list_account_roles::ListAccountRolesError::TooManyRequestsException(inner) => {
+                Error::TooManyRequestsException(inner)
+            }
+            crate::operation::list_account_roles::ListAccountRolesError::UnauthorizedException(inner) => {
+                Error::UnauthorizedException(inner)
+            }
             crate::operation::list_account_roles::ListAccountRolesError::Unhandled(inner) => Error::Unhandled(inner),
         }
     }
 }
-impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_accounts::ListAccountsError, R>> for Error
+#[cfg(feature = "op_list_accounts")]
+impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_accounts::ListAccountsError, R>>
+    for Error
 where
     R: Send + Sync + std::fmt::Debug + 'static,
 {
-    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_accounts::ListAccountsError, R>) -> Self {
+    fn from(
+        err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_accounts::ListAccountsError, R>,
+    ) -> Self {
         match err {
             ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
             _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
@@ -124,17 +241,27 @@
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
+            crate::operation::list_accounts::ListAccountsError::InvalidRequestException(inner) => {
+                Error::InvalidRequestException(inner)
+            }
+            crate::operation::list_accounts::ListAccountsError::ResourceNotFoundException(inner) => {
+                Error::ResourceNotFoundException(inner)
+            }
+            crate::operation::list_accounts::ListAccountsError::TooManyRequestsException(inner) => {
+                Error::TooManyRequestsException(inner)
+            }
+            crate::operation::list_accounts::ListAccountsError::UnauthorizedException(inner) => {
+                Error::UnauthorizedException(inner)
+            }
             crate::operation::list_accounts::ListAccountsError::Unhandled(inner) => Error::Unhandled(inner),
         }
     }
 }
+#[cfg(feature = "op_logout")]
 impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::logout::LogoutError, R>> for Error
 where
     R: Send + Sync + std::fmt::Debug + 'static,
@@ -149,11 +276,16 @@
         }
     }
 }
+#[cfg(feature = "op_logout")]
 impl From<crate::operation::logout::LogoutError> for Error {
     fn from(err: crate::operation::logout::LogoutError) -> Self {
         match err {
-            crate::operation::logout::LogoutError::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
-            crate::operation::logout::LogoutError::TooManyRequestsException(inner) => Error::TooManyRequestsException(inner),
+            crate::operation::logout::LogoutError::InvalidRequestException(inner) => {
+                Error::InvalidRequestException(inner)
+            }
+            crate::operation::logout::LogoutError::TooManyRequestsException(inner) => {
+                Error::TooManyRequestsException(inner)
+            }
             crate::operation::logout::LogoutError::UnauthorizedException(inner) => Error::UnauthorizedException(inner),
             crate::operation::logout::LogoutError::Unhandled(inner) => Error::Unhandled(inner),
         }
@@ -162,9 +294,32 @@
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
@@ -173,9 +328,32 @@
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
@@ -58,10 +58,15 @@
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

 pub fn parse_error_metadata(payload: &[u8], headers: &Headers) -> Result<ErrorMetadataBuilder, DeserializeError> {
@@ -68,7 +73,11 @@
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
@@ -112,7 +121,9 @@
     fn code_takes_priority() {
         assert_eq!(
             Some(Cow::Borrowed("BarError")),
-            parse_error_body(br#"{ "code": "BarError", "__type": "FooError" }"#).unwrap().code
+            parse_error_body(br#"{ "code": "BarError", "__type": "FooError" }"#)
+                .unwrap()
+                .code
         );
     }

@@ -129,7 +140,9 @@
     #[test]
     fn sanitize_namespace_and_url() {
         assert_eq!(
-            sanitize_error_code("aws.protocoltests.restjson#FooError:http://internal.amazon.com/coral/com.amazon.coral.validate/"),
+            sanitize_error_code(
+                "aws.protocoltests.restjson#FooError:http://internal.amazon.com/coral/com.amazon.coral.validate/"
+            ),
             "FooError"
         );
     }
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
@@ -91,19 +101,26 @@
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
             GetRoleCredentialsRequestSerializer,
         ));
-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            GetRoleCredentialsResponseDeserializer,
-        ));
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
+        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
+            "GetRoleCredentials",
+            "SSO",
+        ));

         ::std::option::Option::Some(cfg.freeze())
     }
@@ -113,25 +130,37 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetRoleCredentials")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new(
+            "GetRoleCredentials",
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 GetRoleCredentialsTelemetryInputCaptureInterceptor,
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
                 GetRoleCredentialsEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
+            ),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 crate::operation::get_role_credentials::GetRoleCredentialsError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::get_role_credentials::GetRoleCredentialsError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
-                crate::operation::get_role_credentials::GetRoleCredentialsError,
-            >::new());
+            >::new(),
+        )
+        .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+            crate::operation::get_role_credentials::GetRoleCredentialsError,
+        >::new());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -202,7 +231,9 @@
         let parse_result = if !success && status != 200 || force_error {
             crate::protocol_serde::shape_get_role_credentials::de_get_role_credentials_http_error(status, headers, body)
         } else {
-            crate::protocol_serde::shape_get_role_credentials::de_get_role_credentials_http_response(status, headers, body)
+            crate::protocol_serde::shape_get_role_credentials::de_get_role_credentials_http_response(
+                status, headers, body,
+            )
         };
         crate::protocol_serde::type_erase_result(parse_result)
     }
@@ -210,12 +241,20 @@
 #[derive(Debug)]
 struct GetRoleCredentialsRequestSerializer;
 impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for GetRoleCredentialsRequestSerializer {
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
             .downcast::<crate::operation::get_role_credentials::GetRoleCredentialsInput>()
             .expect("correct type");
@@ -239,25 +278,35 @@
             ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
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
@@ -266,11 +315,14 @@
             fn update_http_builder(
                 input: &crate::operation::get_role_credentials::GetRoleCredentialsInput,
                 builder: ::http_1x::request::Builder,
-            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError>
+            {
                 let mut uri = ::std::string::String::new();
                 uri_base(input, &mut uri)?;
                 uri_query(input, &mut uri)?;
-                let builder = crate::protocol_serde::shape_get_role_credentials::ser_get_role_credentials_headers(input, builder)?;
+                let builder = crate::protocol_serde::shape_get_role_credentials::ser_get_role_credentials_headers(
+                    input, builder,
+                )?;
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
@@ -309,13 +361,21 @@
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
@@ -336,18 +396,22 @@
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
+            ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
+        >,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
             source: err.into(),
@@ -411,7 +475,9 @@
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
@@ -431,10 +497,18 @@
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
diff --git a/src/operation/get_role_credentials/_get_role_credentials_input.rs b/src/operation/get_role_credentials/_get_role_credentials_input.rs
--- a/src/operation/get_role_credentials/_get_role_credentials_input.rs
+++ b/src/operation/get_role_credentials/_get_role_credentials_input.rs
@@ -97,8 +97,10 @@
     /// Consumes the builder and constructs a [`GetRoleCredentialsInput`](crate::operation::get_role_credentials::GetRoleCredentialsInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<crate::operation::get_role_credentials::GetRoleCredentialsInput, ::aws_smithy_types::error::operation::BuildError>
-    {
+    ) -> ::std::result::Result<
+        crate::operation::get_role_credentials::GetRoleCredentialsInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(crate::operation::get_role_credentials::GetRoleCredentialsInput {
             role_name: self.role_name,
             account_id: self.account_id,
diff --git a/src/operation/get_role_credentials/builders.rs b/src/operation/get_role_credentials/builders.rs
--- a/src/operation/get_role_credentials/builders.rs
+++ b/src/operation/get_role_credentials/builders.rs
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
@@ -91,19 +101,26 @@
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
             ListAccountRolesRequestSerializer,
         ));
-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            ListAccountRolesResponseDeserializer,
-        ));
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
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
+                crate::config::auth::Params::builder()
+                    .operation_name("ListAccountRoles")
+                    .build()
+                    .expect("required fields set"),
+            ),
+        );
+
+        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
+            "ListAccountRoles",
+            "SSO",
         ));

-        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("ListAccountRoles", "SSO"));
-
         ::std::option::Option::Some(cfg.freeze())
     }

@@ -112,25 +129,37 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ListAccountRoles")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new(
+            "ListAccountRoles",
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ListAccountRolesTelemetryInputCaptureInterceptor,
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
                 ListAccountRolesEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::list_account_roles::ListAccountRolesError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+            ),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 crate::operation::list_account_roles::ListAccountRolesError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::list_account_roles::ListAccountRolesError,
-            >::new());
+            >::new(),
+        )
+        .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+            crate::operation::list_account_roles::ListAccountRolesError,
+        >::new());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -209,12 +238,20 @@
 #[derive(Debug)]
 struct ListAccountRolesRequestSerializer;
 impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for ListAccountRolesRequestSerializer {
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
             .downcast::<crate::operation::list_account_roles::ListAccountRolesInput>()
             .expect("correct type");
@@ -244,18 +281,26 @@
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
@@ -264,11 +309,13 @@
             fn update_http_builder(
                 input: &crate::operation::list_account_roles::ListAccountRolesInput,
                 builder: ::http_1x::request::Builder,
-            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError>
+            {
                 let mut uri = ::std::string::String::new();
                 uri_base(input, &mut uri)?;
                 uri_query(input, &mut uri)?;
-                let builder = crate::protocol_serde::shape_list_account_roles::ser_list_account_roles_headers(input, builder)?;
+                let builder =
+                    crate::protocol_serde::shape_list_account_roles::ser_list_account_roles_headers(input, builder)?;
                 ::std::result::Result::Ok(builder.method("GET").uri(uri))
             }
             let mut builder = update_http_builder(&input, ::http_1x::request::Builder::new())?;
@@ -307,13 +354,21 @@
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
@@ -334,18 +389,22 @@
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
+            ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
+        >,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
             source: err.into(),
@@ -409,7 +468,9 @@
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
@@ -429,10 +490,18 @@
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
diff --git a/src/operation/list_account_roles/_list_account_roles_input.rs b/src/operation/list_account_roles/_list_account_roles_input.rs
--- a/src/operation/list_account_roles/_list_account_roles_input.rs
+++ b/src/operation/list_account_roles/_list_account_roles_input.rs
@@ -118,7 +118,10 @@
     /// Consumes the builder and constructs a [`ListAccountRolesInput`](crate::operation::list_account_roles::ListAccountRolesInput).
     pub fn build(
         self,
-    ) -> ::std::result::Result<crate::operation::list_account_roles::ListAccountRolesInput, ::aws_smithy_types::error::operation::BuildError> {
+    ) -> ::std::result::Result<
+        crate::operation::list_account_roles::ListAccountRolesInput,
+        ::aws_smithy_types::error::operation::BuildError,
+    > {
         ::std::result::Result::Ok(crate::operation::list_account_roles::ListAccountRolesInput {
             next_token: self.next_token,
             max_results: self.max_results,
diff --git a/src/operation/list_account_roles/builders.rs b/src/operation/list_account_roles/builders.rs
--- a/src/operation/list_account_roles/builders.rs
+++ b/src/operation/list_account_roles/builders.rs
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
diff --git a/src/operation/list_account_roles/paginator.rs b/src/operation/list_account_roles/paginator.rs
--- a/src/operation/list_account_roles/paginator.rs
+++ b/src/operation/list_account_roles/paginator.rs
@@ -71,8 +71,8 @@
             ::std::option::Option::None,
         )
         .with_operation_plugin(crate::sdk_feature_tracker::paginator::PaginatorFeatureTrackerRuntimePlugin::new());
-        ::aws_smithy_async::future::pagination_stream::PaginationStream::new(::aws_smithy_async::future::pagination_stream::fn_stream::FnStream::new(
-            move |tx| {
+        ::aws_smithy_async::future::pagination_stream::PaginationStream::new(
+            ::aws_smithy_async::future::pagination_stream::fn_stream::FnStream::new(move |tx| {
                 ::std::boxed::Box::pin(async move {
                     // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                     let mut input = match builder
@@ -86,7 +86,11 @@
                         }
                     };
                     loop {
-                        let resp = crate::operation::list_account_roles::ListAccountRoles::orchestrate(&runtime_plugins, input.clone()).await;
+                        let resp = crate::operation::list_account_roles::ListAccountRoles::orchestrate(
+                            &runtime_plugins,
+                            input.clone(),
+                        )
+                        .await;
                         // If the input member is None or it was an error
                         let done = match resp {
                             ::std::result::Result::Ok(ref resp) => {
@@ -111,8 +115,8 @@
                         }
                     }
                 })
-            },
-        ))
+            }),
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
@@ -91,19 +101,24 @@
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
             ListAccountsRequestSerializer,
         ));
-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            ListAccountsResponseDeserializer,
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(ListAccountsResponseDeserializer),
+        );
+
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
+                crate::config::auth::Params::builder()
+                    .operation_name("ListAccounts")
+                    .build()
+                    .expect("required fields set"),
+            ),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
-            crate::config::auth::Params::builder()
-                .operation_name("ListAccounts")
-                .build()
-                .expect("required fields set"),
+        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
+            "ListAccounts",
+            "SSO",
         ));

-        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("ListAccounts", "SSO"));
-
         ::std::option::Option::Some(cfg.freeze())
     }

@@ -112,25 +127,37 @@
         _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
     ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
         #[allow(unused_mut)]
-        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ListAccounts")
-            .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
+        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new(
+            "ListAccounts",
+        )
+        .with_interceptor(
+            ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::permanent(
                 ListAccountsTelemetryInputCaptureInterceptor,
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
                 ListAccountsEndpointParamsInterceptor,
-            ))
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
-                crate::operation::list_accounts::ListAccountsError,
-            >::new())
-            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
+            ),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                 crate::operation::list_accounts::ListAccountsError,
-            >::new())
-            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+            >::new(),
+        )
+        .with_retry_classifier(
+            ::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                 crate::operation::list_accounts::ListAccountsError,
-            >::new());
+            >::new(),
+        )
+        .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
+            crate::operation::list_accounts::ListAccountsError,
+        >::new());

         ::std::borrow::Cow::Owned(rcb)
     }
@@ -204,12 +231,20 @@
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
@@ -239,7 +274,10 @@
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
@@ -248,7 +286,8 @@
             fn update_http_builder(
                 input: &crate::operation::list_accounts::ListAccountsInput,
                 builder: ::http_1x::request::Builder,
-            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError>
+            {
                 let mut uri = ::std::string::String::new();
                 uri_base(input, &mut uri)?;
                 uri_query(input, &mut uri)?;
@@ -291,13 +330,21 @@
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
@@ -318,18 +365,22 @@
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
+            ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
+        >,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
             source: err.into(),
@@ -393,7 +444,9 @@
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
@@ -413,10 +466,18 @@
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
@@ -68,7 +68,10 @@
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
diff --git a/src/operation/list_accounts/builders.rs b/src/operation/list_accounts/builders.rs
--- a/src/operation/list_accounts/builders.rs
+++ b/src/operation/list_accounts/builders.rs
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
diff --git a/src/operation/list_accounts/paginator.rs b/src/operation/list_accounts/paginator.rs
--- a/src/operation/list_accounts/paginator.rs
+++ b/src/operation/list_accounts/paginator.rs
@@ -71,8 +71,8 @@
             ::std::option::Option::None,
         )
         .with_operation_plugin(crate::sdk_feature_tracker::paginator::PaginatorFeatureTrackerRuntimePlugin::new());
-        ::aws_smithy_async::future::pagination_stream::PaginationStream::new(::aws_smithy_async::future::pagination_stream::fn_stream::FnStream::new(
-            move |tx| {
+        ::aws_smithy_async::future::pagination_stream::PaginationStream::new(
+            ::aws_smithy_async::future::pagination_stream::fn_stream::FnStream::new(move |tx| {
                 ::std::boxed::Box::pin(async move {
                     // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                     let mut input = match builder
@@ -86,7 +86,9 @@
                         }
                     };
                     loop {
-                        let resp = crate::operation::list_accounts::ListAccounts::orchestrate(&runtime_plugins, input.clone()).await;
+                        let resp =
+                            crate::operation::list_accounts::ListAccounts::orchestrate(&runtime_plugins, input.clone())
+                                .await;
                         // If the input member is None or it was an error
                         let done = match resp {
                             ::std::result::Result::Ok(ref resp) => {
@@ -111,8 +113,8 @@
                         }
                     }
                 })
-            },
-        ))
+            }),
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
@@ -83,19 +101,23 @@
         cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
             LogoutRequestSerializer,
         ));
-        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
-            LogoutResponseDeserializer,
-        ));
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(LogoutResponseDeserializer),
+        );
+
+        cfg.store_put(
+            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
+                crate::config::auth::Params::builder()
+                    .operation_name("Logout")
+                    .build()
+                    .expect("required fields set"),
+            ),
+        );

-        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
-            crate::config::auth::Params::builder()
-                .operation_name("Logout")
-                .build()
-                .expect("required fields set"),
+        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
+            "Logout", "SSO",
         ));

-        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new("Logout", "SSO"));
-
         ::std::option::Option::Some(cfg.freeze())
     }

@@ -150,13 +172,23 @@
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
@@ -175,7 +207,8 @@
             fn update_http_builder(
                 input: &crate::operation::logout::LogoutInput,
                 builder: ::http_1x::request::Builder,
-            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
+            ) -> ::std::result::Result<::http_1x::request::Builder, ::aws_smithy_types::error::operation::BuildError>
+            {
                 let mut uri = ::std::string::String::new();
                 uri_base(input, &mut uri)?;
                 let builder = crate::protocol_serde::shape_logout::ser_logout_headers(input, builder)?;
@@ -208,19 +241,30 @@
         >,
         cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
     ) -> ::std::result::Result<(), ::aws_smithy_runtime_api::box_error::BoxError> {
-        let _input = context.input().downcast_ref::<LogoutInput>().ok_or("failed to downcast to LogoutInput")?;
+        let _input = context
+            .input()
+            .downcast_ref::<LogoutInput>()
+            .ok_or("failed to downcast to LogoutInput")?;

         let params = crate::config::endpoint::Params::builder()
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
@@ -239,18 +283,22 @@
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
+            ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
+        >,
     ) -> Self {
         Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
             source: err.into(),
@@ -307,7 +355,9 @@
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
@@ -327,9 +377,15 @@
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
diff --git a/src/operation/logout/_logout_input.rs b/src/operation/logout/_logout_input.rs
--- a/src/operation/logout/_logout_input.rs
+++ b/src/operation/logout/_logout_input.rs
@@ -49,7 +49,10 @@
         &self.access_token
     }
     /// Consumes the builder and constructs a [`LogoutInput`](crate::operation::logout::LogoutInput).
-    pub fn build(self) -> ::std::result::Result<crate::operation::logout::LogoutInput, ::aws_smithy_types::error::operation::BuildError> {
+    pub fn build(
+        self,
+    ) -> ::std::result::Result<crate::operation::logout::LogoutInput, ::aws_smithy_types::error::operation::BuildError>
+    {
         ::std::result::Result::Ok(crate::operation::logout::LogoutInput {
             access_token: self.access_token,
         })
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
@@ -15,22 +23,41 @@
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
@@ -39,20 +66,48 @@
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
@@ -5,7 +5,12 @@
     depth: u32,
 ) -> ::std::result::Result<Option<crate::types::AccountInfo>, ::aws_smithy_json::deserialize::error::DeserializeError>
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
         return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
@@ -20,34 +25,36 @@
             loop {
                 match tokens.next().transpose()? {
                     Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                        "accountId" => {
-                            builder = builder.set_account_id(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                    .transpose()?,
-                            );
-                        }
-                        "accountName" => {
-                            builder = builder.set_account_name(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                    .transpose()?,
-                            );
-                        }
-                        "emailAddress" => {
-                            builder = builder.set_email_address(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                    .transpose()?,
-                            );
+                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
+                        match key.to_unescaped()?.as_ref() {
+                            "accountId" => {
+                                builder = builder.set_account_id(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                        .transpose()?,
+                                );
+                            }
+                            "accountName" => {
+                                builder = builder.set_account_name(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                        .transpose()?,
+                                );
+                            }
+                            "emailAddress" => {
+                                builder = builder.set_email_address(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                        .transpose()?,
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
+                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                            format!("expected object key or end object, found: {other:?}"),
+                        ))
                     }
                 }
             }
diff --git a/src/protocol_serde/shape_account_list_type.rs b/src/protocol_serde/shape_account_list_type.rs
--- a/src/protocol_serde/shape_account_list_type.rs
+++ b/src/protocol_serde/shape_account_list_type.rs
@@ -3,9 +3,17 @@
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
         return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
@@ -23,7 +31,8 @@
                         break;
                     }
                     _ => {
-                        let value = crate::protocol_serde::shape_account_info::de_account_info(tokens, _value, depth + 1)?;
+                        let value =
+                            crate::protocol_serde::shape_account_info::de_account_info(tokens, _value, depth + 1)?;
                         if let Some(value) = value {
                             items.push(value);
                         } else {
diff --git a/src/protocol_serde/shape_get_role_credentials.rs b/src/protocol_serde/shape_get_role_credentials.rs
--- a/src/protocol_serde/shape_get_role_credentials.rs
+++ b/src/protocol_serde/shape_get_role_credentials.rs
@@ -9,8 +9,9 @@
     crate::operation::get_role_credentials::GetRoleCredentialsError,
 > {
     #[allow(unused_mut)]
-    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(crate::operation::get_role_credentials::GetRoleCredentialsError::unhandled)?;
+    let mut generic_builder =
+        crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
+            .map_err(crate::operation::get_role_credentials::GetRoleCredentialsError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
@@ -20,66 +21,81 @@

     let _error_message = generic.message().map(|msg| msg.to_owned());
     Err(match error_code {
-        "InvalidRequestException" => crate::operation::get_role_credentials::GetRoleCredentialsError::InvalidRequestException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "InvalidRequestException" => {
+            crate::operation::get_role_credentials::GetRoleCredentialsError::InvalidRequestException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::InvalidRequestExceptionBuilder::default();
-                output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(_response_body, output)
-                    .map_err(crate::operation::get_role_credentials::GetRoleCredentialsError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "ResourceNotFoundException" => crate::operation::get_role_credentials::GetRoleCredentialsError::ResourceNotFoundException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::InvalidRequestExceptionBuilder::default();
+                    output =
+                        crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(
+                            _response_body,
+                            output,
+                        )
+                        .map_err(crate::operation::get_role_credentials::GetRoleCredentialsError::unhandled)?;
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
+            crate::operation::get_role_credentials::GetRoleCredentialsError::ResourceNotFoundException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
-                output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                     .map_err(crate::operation::get_role_credentials::GetRoleCredentialsError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "TooManyRequestsException" => crate::operation::get_role_credentials::GetRoleCredentialsError::TooManyRequestsException({
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
+            crate::operation::get_role_credentials::GetRoleCredentialsError::TooManyRequestsException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
-                output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                     .map_err(crate::operation::get_role_credentials::GetRoleCredentialsError::unhandled)?;
-                let output = output.meta(generic);
-                output.build()
-            };
-            if tmp.message.is_none() {
-                tmp.message = _error_message;
-            }
-            tmp
-        }),
-        "UnauthorizedException" => crate::operation::get_role_credentials::GetRoleCredentialsError::UnauthorizedException({
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
+            crate::operation::get_role_credentials::GetRoleCredentialsError::UnauthorizedException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::UnauthorizedExceptionBuilder::default();
-                output = crate::protocol_serde::shape_unauthorized_exception::de_unauthorized_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::UnauthorizedExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_unauthorized_exception::de_unauthorized_exception_json_err(
+                        _response_body,
+                        output,
+                    )
                     .map_err(crate::operation::get_role_credentials::GetRoleCredentialsError::unhandled)?;
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
         _ => crate::operation::get_role_credentials::GetRoleCredentialsError::generic(generic),
     })
 }
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
@@ -128,7 +147,8 @@
     crate::operation::get_role_credentials::builders::GetRoleCredentialsOutputBuilder,
     ::aws_smithy_json::deserialize::error::DeserializeError,
 > {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+    let mut tokens_owned =
+        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -138,18 +158,16 @@
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                 "roleCredentials" => {
-                    builder = builder.set_role_credentials(crate::protocol_serde::shape_role_credentials::de_role_credentials(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
+                    builder = builder.set_role_credentials(
+                        crate::protocol_serde::shape_role_credentials::de_role_credentials(tokens, _value, depth + 1)?,
+                    );
                 }
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
-                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
-                    "expected object key or end object, found: {other:?}"
-                )))
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                    format!("expected object key or end object, found: {other:?}"),
+                ))
             }
         }
     }
diff --git a/src/protocol_serde/shape_invalid_request_exception.rs b/src/protocol_serde/shape_invalid_request_exception.rs
--- a/src/protocol_serde/shape_invalid_request_exception.rs
+++ b/src/protocol_serde/shape_invalid_request_exception.rs
@@ -2,8 +2,12 @@
 pub(crate) fn de_invalid_request_exception_json_err(
     _value: &[u8],
     mut builder: crate::types::error::builders::InvalidRequestExceptionBuilder,
-) -> ::std::result::Result<crate::types::error::builders::InvalidRequestExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+) -> ::std::result::Result<
+    crate::types::error::builders::InvalidRequestExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned =
+        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -22,9 +26,9 @@
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
-                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
-                    "expected object key or end object, found: {other:?}"
-                )))
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                    format!("expected object key or end object, found: {other:?}"),
+                ))
             }
         }
     }
diff --git a/src/protocol_serde/shape_list_account_roles.rs b/src/protocol_serde/shape_list_account_roles.rs
--- a/src/protocol_serde/shape_list_account_roles.rs
+++ b/src/protocol_serde/shape_list_account_roles.rs
@@ -4,79 +4,102 @@
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
+    let mut generic_builder =
+        crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
+            .map_err(crate::operation::list_account_roles::ListAccountRolesError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
         Some(code) => code,
-        None => return Err(crate::operation::list_account_roles::ListAccountRolesError::unhandled(generic)),
+        None => {
+            return Err(crate::operation::list_account_roles::ListAccountRolesError::unhandled(
+                generic,
+            ))
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
-                    .map_err(crate::operation::list_account_roles::ListAccountRolesError::unhandled)?;
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
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::InvalidRequestExceptionBuilder::default();
+                    output =
+                        crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(
+                            _response_body,
+                            output,
+                        )
+                        .map_err(crate::operation::list_account_roles::ListAccountRolesError::unhandled)?;
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
+            crate::operation::list_account_roles::ListAccountRolesError::ResourceNotFoundException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
-                output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
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
+            crate::operation::list_account_roles::ListAccountRolesError::TooManyRequestsException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
-                output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
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
+                    let mut output = crate::types::error::builders::UnauthorizedExceptionBuilder::default();
+                    output = crate::protocol_serde::shape_unauthorized_exception::de_unauthorized_exception_json_err(
+                        _response_body,
+                        output,
+                    )
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
@@ -86,7 +109,10 @@
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
         let mut output = crate::operation::list_account_roles::builders::ListAccountRolesOutputBuilder::default();
@@ -107,7 +133,10 @@
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
@@ -122,7 +151,8 @@
     crate::operation::list_account_roles::builders::ListAccountRolesOutputBuilder,
     ::aws_smithy_json::deserialize::error::DeserializeError,
 > {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+    let mut tokens_owned =
+        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -130,23 +160,27 @@
     loop {
         match tokens.next().transpose()? {
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                "nextToken" => {
-                    builder = builder.set_next_token(
-                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                            .transpose()?,
-                    );
-                }
-                "roleList" => {
-                    builder = builder.set_role_list(crate::protocol_serde::shape_role_list_type::de_role_list_type(tokens, _value, depth + 1)?);
+            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
+                match key.to_unescaped()?.as_ref() {
+                    "nextToken" => {
+                        builder = builder.set_next_token(
+                            ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                .transpose()?,
+                        );
+                    }
+                    "roleList" => {
+                        builder = builder.set_role_list(
+                            crate::protocol_serde::shape_role_list_type::de_role_list_type(tokens, _value, depth + 1)?,
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
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                    format!("expected object key or end object, found: {other:?}"),
+                ))
             }
         }
     }
diff --git a/src/protocol_serde/shape_list_accounts.rs b/src/protocol_serde/shape_list_accounts.rs
--- a/src/protocol_serde/shape_list_accounts.rs
+++ b/src/protocol_serde/shape_list_accounts.rs
@@ -4,10 +4,14 @@
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
+    let mut generic_builder =
+        crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
+            .map_err(crate::operation::list_accounts::ListAccountsError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
@@ -22,8 +26,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = crate::types::error::builders::InvalidRequestExceptionBuilder::default();
-                output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(_response_body, output)
-                    .map_err(crate::operation::list_accounts::ListAccountsError::unhandled)?;
+                output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(crate::operation::list_accounts::ListAccountsError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -47,28 +54,33 @@
             }
             tmp
         }),
-        "TooManyRequestsException" => crate::operation::list_accounts::ListAccountsError::TooManyRequestsException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "TooManyRequestsException" => {
+            crate::operation::list_accounts::ListAccountsError::TooManyRequestsException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
-                output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
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
+                    let output = output.meta(generic);
+                    output.build()
+                };
+                if tmp.message.is_none() {
+                    tmp.message = _error_message;
+                }
+                tmp
+            })
+        }
         "UnauthorizedException" => crate::operation::list_accounts::ListAccountsError::UnauthorizedException({
             #[allow(unused_mut)]
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = crate::types::error::builders::UnauthorizedExceptionBuilder::default();
-                output = crate::protocol_serde::shape_unauthorized_exception::de_unauthorized_exception_json_err(_response_body, output)
-                    .map_err(crate::operation::list_accounts::ListAccountsError::unhandled)?;
+                output = crate::protocol_serde::shape_unauthorized_exception::de_unauthorized_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(crate::operation::list_accounts::ListAccountsError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -86,7 +98,10 @@
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
         let mut output = crate::operation::list_accounts::builders::ListAccountsOutputBuilder::default();
@@ -107,7 +122,10 @@
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
@@ -122,7 +140,8 @@
     crate::operation::list_accounts::builders::ListAccountsOutputBuilder,
     ::aws_smithy_json::deserialize::error::DeserializeError,
 > {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+    let mut tokens_owned =
+        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -132,11 +151,12 @@
             Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
             Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                 "accountList" => {
-                    builder = builder.set_account_list(crate::protocol_serde::shape_account_list_type::de_account_list_type(
-                        tokens,
-                        _value,
-                        depth + 1,
-                    )?);
+                    builder =
+                        builder.set_account_list(crate::protocol_serde::shape_account_list_type::de_account_list_type(
+                            tokens,
+                            _value,
+                            depth + 1,
+                        )?);
                 }
                 "nextToken" => {
                     builder = builder.set_next_token(
@@ -148,9 +168,9 @@
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
-                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
-                    "expected object key or end object, found: {other:?}"
-                )))
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                    format!("expected object key or end object, found: {other:?}"),
+                ))
             }
         }
     }
diff --git a/src/protocol_serde/shape_logout.rs b/src/protocol_serde/shape_logout.rs
--- a/src/protocol_serde/shape_logout.rs
+++ b/src/protocol_serde/shape_logout.rs
@@ -6,8 +6,9 @@
     _response_body: &[u8],
 ) -> std::result::Result<crate::operation::logout::LogoutOutput, crate::operation::logout::LogoutError> {
     #[allow(unused_mut)]
-    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
-        .map_err(crate::operation::logout::LogoutError::unhandled)?;
+    let mut generic_builder =
+        crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
+            .map_err(crate::operation::logout::LogoutError::unhandled)?;
     generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
     let generic = generic_builder.build();
     let error_code = match generic.code() {
@@ -22,8 +23,11 @@
             let mut tmp = {
                 #[allow(unused_mut)]
                 let mut output = crate::types::error::builders::InvalidRequestExceptionBuilder::default();
-                output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(_response_body, output)
-                    .map_err(crate::operation::logout::LogoutError::unhandled)?;
+                output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(crate::operation::logout::LogoutError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -32,28 +36,33 @@
             }
             tmp
         }),
-        "TooManyRequestsException" => crate::operation::logout::LogoutError::TooManyRequestsException({
-            #[allow(unused_mut)]
-            let mut tmp = {
+        "TooManyRequestsException" => {
+            crate::operation::logout::LogoutError::TooManyRequestsException({
                 #[allow(unused_mut)]
-                let mut output = crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
-                output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
+                let mut tmp = {
+                    #[allow(unused_mut)]
+                    let mut output = crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
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
                 let mut output = crate::types::error::builders::UnauthorizedExceptionBuilder::default();
-                output = crate::protocol_serde::shape_unauthorized_exception::de_unauthorized_exception_json_err(_response_body, output)
-                    .map_err(crate::operation::logout::LogoutError::unhandled)?;
+                output = crate::protocol_serde::shape_unauthorized_exception::de_unauthorized_exception_json_err(
+                    _response_body,
+                    output,
+                )
+                .map_err(crate::operation::logout::LogoutError::unhandled)?;
                 let output = output.meta(generic);
                 output.build()
             };
@@ -90,7 +99,10 @@
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
@@ -2,8 +2,12 @@
 pub(crate) fn de_resource_not_found_exception_json_err(
     _value: &[u8],
     mut builder: crate::types::error::builders::ResourceNotFoundExceptionBuilder,
-) -> ::std::result::Result<crate::types::error::builders::ResourceNotFoundExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+) -> ::std::result::Result<
+    crate::types::error::builders::ResourceNotFoundExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned =
+        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -22,9 +26,9 @@
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
-                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
-                    "expected object key or end object, found: {other:?}"
-                )))
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                    format!("expected object key or end object, found: {other:?}"),
+                ))
             }
         }
     }
diff --git a/src/protocol_serde/shape_role_credentials.rs b/src/protocol_serde/shape_role_credentials.rs
--- a/src/protocol_serde/shape_role_credentials.rs
+++ b/src/protocol_serde/shape_role_credentials.rs
@@ -5,7 +5,12 @@
     depth: u32,
 ) -> ::std::result::Result<Option<crate::types::RoleCredentials>, ::aws_smithy_json::deserialize::error::DeserializeError>
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
         return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
@@ -20,41 +25,43 @@
             loop {
                 match tokens.next().transpose()? {
                     Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                        "accessKeyId" => {
-                            builder = builder.set_access_key_id(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                    .transpose()?,
-                            );
-                        }
-                        "secretAccessKey" => {
-                            builder = builder.set_secret_access_key(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                    .transpose()?,
-                            );
-                        }
-                        "sessionToken" => {
-                            builder = builder.set_session_token(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                    .transpose()?,
-                            );
-                        }
-                        "expiration" => {
-                            builder = builder.set_expiration(
-                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
-                                    .map(i64::try_from)
-                                    .transpose()?,
-                            );
+                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
+                        match key.to_unescaped()?.as_ref() {
+                            "accessKeyId" => {
+                                builder = builder.set_access_key_id(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                        .transpose()?,
+                                );
+                            }
+                            "secretAccessKey" => {
+                                builder = builder.set_secret_access_key(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                        .transpose()?,
+                                );
+                            }
+                            "sessionToken" => {
+                                builder = builder.set_session_token(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                        .transpose()?,
+                                );
+                            }
+                            "expiration" => {
+                                builder = builder.set_expiration(
+                                    ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
+                                        .map(i64::try_from)
+                                        .transpose()?,
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
+                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                            format!("expected object key or end object, found: {other:?}"),
+                        ))
                     }
                 }
             }
diff --git a/src/protocol_serde/shape_role_info.rs b/src/protocol_serde/shape_role_info.rs
--- a/src/protocol_serde/shape_role_info.rs
+++ b/src/protocol_serde/shape_role_info.rs
@@ -5,7 +5,12 @@
     depth: u32,
 ) -> ::std::result::Result<Option<crate::types::RoleInfo>, ::aws_smithy_json::deserialize::error::DeserializeError>
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
         return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
@@ -20,27 +25,29 @@
             loop {
                 match tokens.next().transpose()? {
                     Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
-                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
-                        "roleName" => {
-                            builder = builder.set_role_name(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                    .transpose()?,
-                            );
-                        }
-                        "accountId" => {
-                            builder = builder.set_account_id(
-                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
-                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
-                                    .transpose()?,
-                            );
+                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
+                        match key.to_unescaped()?.as_ref() {
+                            "roleName" => {
+                                builder = builder.set_role_name(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                        .transpose()?,
+                                );
+                            }
+                            "accountId" => {
+                                builder = builder.set_account_id(
+                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
+                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
+                                        .transpose()?,
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
+                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                            format!("expected object key or end object, found: {other:?}"),
+                        ))
                     }
                 }
             }
diff --git a/src/protocol_serde/shape_role_list_type.rs b/src/protocol_serde/shape_role_list_type.rs
--- a/src/protocol_serde/shape_role_list_type.rs
+++ b/src/protocol_serde/shape_role_list_type.rs
@@ -3,9 +3,17 @@
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
         return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
diff --git a/src/protocol_serde/shape_too_many_requests_exception.rs b/src/protocol_serde/shape_too_many_requests_exception.rs
--- a/src/protocol_serde/shape_too_many_requests_exception.rs
+++ b/src/protocol_serde/shape_too_many_requests_exception.rs
@@ -2,8 +2,12 @@
 pub(crate) fn de_too_many_requests_exception_json_err(
     _value: &[u8],
     mut builder: crate::types::error::builders::TooManyRequestsExceptionBuilder,
-) -> ::std::result::Result<crate::types::error::builders::TooManyRequestsExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+) -> ::std::result::Result<
+    crate::types::error::builders::TooManyRequestsExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned =
+        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -22,9 +26,9 @@
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
-                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
-                    "expected object key or end object, found: {other:?}"
-                )))
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                    format!("expected object key or end object, found: {other:?}"),
+                ))
             }
         }
     }
diff --git a/src/protocol_serde/shape_unauthorized_exception.rs b/src/protocol_serde/shape_unauthorized_exception.rs
--- a/src/protocol_serde/shape_unauthorized_exception.rs
+++ b/src/protocol_serde/shape_unauthorized_exception.rs
@@ -2,8 +2,12 @@
 pub(crate) fn de_unauthorized_exception_json_err(
     _value: &[u8],
     mut builder: crate::types::error::builders::UnauthorizedExceptionBuilder,
-) -> ::std::result::Result<crate::types::error::builders::UnauthorizedExceptionBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
-    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
+) -> ::std::result::Result<
+    crate::types::error::builders::UnauthorizedExceptionBuilder,
+    ::aws_smithy_json::deserialize::error::DeserializeError,
+> {
+    let mut tokens_owned =
+        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(_value)).peekable();
     let tokens = &mut tokens_owned;
     #[allow(unused_variables)]
     let depth = 0u32;
@@ -22,9 +26,9 @@
                 _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
             },
             other => {
-                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
-                    "expected object key or end object, found: {other:?}"
-                )))
+                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
+                    format!("expected object key or end object, found: {other:?}"),
+                ))
             }
         }
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
